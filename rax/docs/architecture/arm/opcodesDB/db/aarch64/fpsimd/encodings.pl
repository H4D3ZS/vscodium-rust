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

ENCODING ABS_asisdmisc_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b01011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=ABS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=ABS_advsimd cclass=ABS_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING ABS_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b01011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=ABS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=ABS_advsimd cclass=ABS_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING ADD_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=ADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=ADD_advsimd cclass=ADD_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING ADD_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=ADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=ADD_advsimd cclass=ADD_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING ADDHN_asimddiff_N => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 o1:u=0b0 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=ADDHN',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=ADDHN_advsimd cclass=ADDHN_asimddiff_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING ADDP_asisdpair_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 size:u=0bxx ig2=0b11000 opcode=0b11011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ADDP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=ADDP_advsimd_pair cclass=ADDP_asisdpair_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING ADDP_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10111 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=ADDP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=ADDP_advsimd_vec cclass=ADDP_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING ADDV_asimdall_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b11000 opcode=0b11011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ADDV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=ADDV_advsimd cclass=ADDV_asimdall_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING AESD_B_cryptoaes => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01001110 size=0b00 ig1=0b10100 ig2=0b0010 D:u=0b1 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=AESD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptoaes page=AESD_advsimd cclass=AESD_B_cryptoaes',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING AESE_B_cryptoaes => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01001110 size=0b00 ig1=0b10100 ig2=0b0010 D:u=0b0 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=AESE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptoaes page=AESE_advsimd cclass=AESE_B_cryptoaes',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING AESIMC_B_cryptoaes => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01001110 size=0b00 ig1=0b10100 ig2=0b0011 D:u=0b1 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=AESIMC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptoaes page=AESIMC_advsimd cclass=AESIMC_B_cryptoaes',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING AESMC_B_cryptoaes => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01001110 size=0b00 ig1=0b10100 ig2=0b0011 D:u=0b0 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=AESMC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptoaes page=AESMC_advsimd cclass=AESMC_B_cryptoaes',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING AND_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0b00 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=AND',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=AND_advsimd cclass=AND_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING BCAX_VVV16_crypto4 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b110011100 Op0=0b01 Rm:u=0bxxxxx ig1=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=BCAX',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=crypto4 page=BCAX_advsimd cclass=BCAX_VVV16_crypto4',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING BFCVT_BS_floatdp1 => {
  name       => 'Single-precision to BFloat16',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ptype=0b01 ig2=0b1 opcode=0b000110 ig3=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-bf16 instr-class=float isa=A64 mnemonic=BFCVT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=BFCVT_float cclass=BFCVT_BS_floatdp1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING BFCVTN_asimdmisc_4S => {
  name       => 'Vector single-precision and BFloat16',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size=0b10 ig2=0b10000 opcode=0b10110 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-bf16 advsimd-type=simd datatype=single-and-bf16 instr-class=advsimd isa=A64 mnemonic=BFCVTN',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=BFCVTN_advsimd cclass=BFCVTN_asimdmisc_4S',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING BFDOT_asimdelem_E => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size=0b01 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1111 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=BFDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=BFDOT_advsimd_elt cclass=BFDOT_asimdelem_E',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING BFDOT_asimdsame2_D => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size=0b01 ig2=0b0 Rm:u=0bxxxxx ig3=0b1 opcode=0b1111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BFDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=BFDOT_advsimd_vec cclass=BFDOT_asimdsame2_D',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING BFMLAL_asimdelem_F => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size=0b11 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1111 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=half instr-class=advsimd isa=A64 mnemonic=BFMLAL reguse-datatype=2reg-element-half',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=BFMLAL_advsimd_elt cclass=BFMLAL_asimdelem_F',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING BFMLAL_asimdsame2_F_ => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size=0b11 ig2=0b0 Rm:u=0bxxxxx ig3=0b1 opcode=0b1111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BFMLAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=BFMLAL_advsimd_vec cclass=BFMLAL_asimdsame2_F_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING BFMMLA_asimdsame2_E => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q=0b1 U=0b1 ig1=0b01110 size=0b01 ig2=0b0 Rm:u=0bxxxxx ig3=0b1 opcode=0b1101 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BFMMLA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=BFMMLA_advsimd cclass=BFMMLA_asimdsame2_E',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING BIC_asimdimm_L_hl => {
  name       => '16-bit',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0b1 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0b10x1 o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  bitdiffs   => 'cmode == 0b10x1',
  docvars    => 'asimdimm-datatype=per-halfword asimdimm-immtype=shifted-immediate asimdimm-type=per-halfword-shifted-immediate instr-class=advsimd isa=A64 mnemonic=BIC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=BIC_advsimd_imm cclass=BIC_advsimd_imm_shifted_immediate',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING BIC_asimdimm_L_sl => {
  name       => '32-bit',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0b1 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0b0xx1 o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  bitdiffs   => 'cmode == 0b0xx1',
  docvars    => 'asimdimm-datatype=per-word asimdimm-immtype=shifted-immediate asimdimm-type=per-word-shifted-immediate instr-class=advsimd isa=A64 mnemonic=BIC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=BIC_advsimd_imm cclass=BIC_advsimd_imm_shifted_immediate',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING BIC_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0b01 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BIC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=BIC_advsimd_reg cclass=BIC_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING BIF_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 opc2:u=0b11 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BIF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=BIF_advsimd cclass=BIF_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|INSERTION|BITWISE|CONDITIONALLY',
};

ENCODING BIT_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 opc2:u=0b10 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BIT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=BIT_advsimd cclass=BIT_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|INSERTION|BITWISE|CONDITIONALLY',
};

ENCODING BSL_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 opc2:u=0b01 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BSL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=BSL_advsimd cclass=BSL_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SELECTION',
};

ENCODING CLS_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b00100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CLS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=CLS_advsimd cclass=CLS_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITFIELD',
};

ENCODING CLZ_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b00100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CLZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=CLZ_advsimd cclass=CLZ_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITFIELD',
};

ENCODING CMEQ_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=CMEQ_advsimd_reg cclass=CMEQ_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING CMEQ_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=CMEQ_advsimd_reg cclass=CMEQ_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING CMEQ_asisdmisc_Z => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b10000 ig3=0b0100 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=CMEQ_advsimd_zero cclass=CMEQ_asisdmisc_Z',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING CMEQ_asimdmisc_Z => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b0100 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=CMEQ_advsimd_zero cclass=CMEQ_asimdmisc_Z',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING CMGE_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0011 eq:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=CMGE_advsimd_reg cclass=CMGE_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON|SIGNED',
};

ENCODING CMGE_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0011 eq:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=CMGE_advsimd_reg cclass=CMGE_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON|SIGNED',
};

ENCODING CMGE_asisdmisc_Z => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b10000 ig3=0b0100 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=CMGE_advsimd_zero cclass=CMGE_asisdmisc_Z',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON|SIGNED',
};

ENCODING CMGE_asimdmisc_Z => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b0100 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=CMGE_advsimd_zero cclass=CMGE_asimdmisc_Z',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON|SIGNED',
};

ENCODING CMGT_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0011 eq:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=CMGT_advsimd_reg cclass=CMGT_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON|SIGNED',
};

ENCODING CMGT_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0011 eq:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=CMGT_advsimd_reg cclass=CMGT_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON|SIGNED',
};

ENCODING CMGT_asisdmisc_Z => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b10000 ig3=0b0100 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=CMGT_advsimd_zero cclass=CMGT_asisdmisc_Z',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON|SIGNED',
};

ENCODING CMGT_asimdmisc_Z => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b0100 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=CMGT_advsimd_zero cclass=CMGT_asimdmisc_Z',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON|SIGNED',
};

ENCODING CMHI_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0011 eq:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMHI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=CMHI_advsimd cclass=CMHI_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON|UNSIGNED',
};

ENCODING CMHI_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0011 eq:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMHI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=CMHI_advsimd cclass=CMHI_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON|UNSIGNED',
};

ENCODING CMHS_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0011 eq:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMHS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=CMHS_advsimd cclass=CMHS_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON|UNSIGNED',
};

ENCODING CMHS_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0011 eq:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMHS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=CMHS_advsimd cclass=CMHS_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON|UNSIGNED',
};

ENCODING CMLE_asisdmisc_Z => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b10000 ig3=0b0100 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMLE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=CMLE_advsimd cclass=CMLE_asisdmisc_Z',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON|SIGNED',
};

ENCODING CMLE_asimdmisc_Z => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b0100 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMLE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=CMLE_advsimd cclass=CMLE_asimdmisc_Z',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON|SIGNED',
};

ENCODING CMLT_asisdmisc_Z => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b01010 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMLT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=CMLT_advsimd cclass=CMLT_asisdmisc_Z',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON|SIGNED',
};

ENCODING CMLT_asimdmisc_Z => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b01010 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMLT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=CMLT_advsimd cclass=CMLT_asimdmisc_Z',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON|SIGNED',
};

ENCODING CMTST_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMTST',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=CMTST_advsimd cclass=CMTST_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING CMTST_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMTST',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=CMTST_advsimd cclass=CMTST_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING CNT_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b00101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CNT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=CNT_advsimd cclass=CNT_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITFIELD',
};

ENCODING DUP_asisdone_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 op=0b0 ig1=0b11110000 imm5:u=0bxxxxx ig2=0b0 imm4=0b0000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=DUP vector-xfer-type=scalar-from-element',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdone page=DUP_advsimd_elt cclass=DUP_advsimd_elt_DUP_asisdone_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BROADCASTING',
};

ENCODING DUP_asimdins_DV_v => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx op=0b0 ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4=0b0000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=DUP vector-xfer-type=vector-from-element',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=DUP_advsimd_elt cclass=DUP_asimdins_DV_v',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BROADCASTING',
};

ENCODING DUP_asimdins_DR_r => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx op=0b0 ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4=0b0001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=DUP vector-xfer-type=vector-from-general',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=DUP_advsimd_gen cclass=DUP_asimdins_DR_r',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BROADCASTING',
};

ENCODING EOR3_VVV16_crypto4 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b110011100 Op0=0b00 Rm:u=0bxxxxx ig1=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=EOR3',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=crypto4 page=EOR3_advsimd cclass=EOR3_VVV16_crypto4',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING EOR_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 opc2:u=0b00 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=EOR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=EOR_advsimd cclass=EOR_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING EXT_asimdext_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b101110 op2=0b00 ig2=0b0 Rm:u=0bxxxxx ig3=0b0 imm4:u=0bxxxx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=EXT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdext page=EXT_advsimd cclass=EXT_asimdext_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|EXTRACTION',
};

ENCODING FABD_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 a=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FABD',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FABD_advsimd cclass=FABD_asisdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FABD_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FABD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FABD_advsimd cclass=FABD_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FABD_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FABD reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FABD_advsimd cclass=FABD_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FABD_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FABD reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FABD_advsimd cclass=FABD_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FABS_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a=0b1 ig2=0b111100 opcode=0b01111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FABS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FABS_advsimd cclass=FABS_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FABS_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b01111 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FABS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FABS_advsimd cclass=FABS_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FABS_H_floatdp1 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 ig3=0b0000 opc:u=0b01 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FABS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FABS_float cclass=FABS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|MATH',
};

ENCODING FABS_S_floatdp1 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 ig3=0b0000 opc:u=0b01 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FABS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FABS_float cclass=FABS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|MATH',
};

ENCODING FABS_D_floatdp1 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 ig3=0b0000 opc:u=0b01 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FABS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FABS_float cclass=FABS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|MATH',
};

ENCODING FACGE_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 E:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b1 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FACGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FACGE_advsimd cclass=FACGE_asisdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FACGE_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 E:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FACGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FACGE_advsimd cclass=FACGE_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FACGE_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 E:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b1 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FACGE reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FACGE_advsimd cclass=FACGE_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FACGE_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 E:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FACGE reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FACGE_advsimd cclass=FACGE_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FACGT_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 E:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b1 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FACGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FACGT_advsimd cclass=FACGT_asisdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FACGT_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 E:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FACGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FACGT_advsimd cclass=FACGT_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FACGT_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 E:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b1 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FACGT reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FACGT_advsimd cclass=FACGT_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FACGT_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 E:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FACGT reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FACGT_advsimd cclass=FACGT_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FADD_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FADD reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FADD_advsimd cclass=FADD_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FADD_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FADD reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FADD_advsimd cclass=FADD_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FADD_H_floatdp2 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 Rm:u=0bxxxxx ig3=0b001 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FADD_float cclass=FADD_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING FADD_S_floatdp2 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 Rm:u=0bxxxxx ig3=0b001 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FADD_float cclass=FADD_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING FADD_D_floatdp2 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 Rm:u=0bxxxxx ig3=0b001 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FADD_float cclass=FADD_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING FADDP_asisdpair_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 ig2=0b0 sz:u=0b0 ig3=0b11000 opcode=0b01101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FADDP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FADDP_advsimd_pair cclass=FADDP_asisdpair_only_H',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FADDP_asisdpair_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 ig2=0b0 sz:u=0bx ig3=0b11000 opcode=0b01101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FADDP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FADDP_advsimd_pair cclass=FADDP_asisdpair_only_SD',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FADDP_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FADDP reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FADDP_advsimd_vec cclass=FADDP_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FADDP_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FADDP reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FADDP_advsimd_vec cclass=FADDP_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FCADD_asimdsame2_C => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b11 rot:u=0bx ig5=0b0 ig6=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FCADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=FCADD_advsimd_vec cclass=FCADD_asimdsame2_C',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING FCCMP_H_floatccmp => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 Rm:u=0bxxxxx cond:u=0bxxxx ig3=0b01 Rn:u=0bxxxxx op:u=0b0 nzcv:u=0bxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FCCMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatccmp page=FCCMP_float cclass=FCCMP_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON|CONDITIONALLY',
};

ENCODING FCCMP_S_floatccmp => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 Rm:u=0bxxxxx cond:u=0bxxxx ig3=0b01 Rn:u=0bxxxxx op:u=0b0 nzcv:u=0bxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FCCMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatccmp page=FCCMP_float cclass=FCCMP_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON|CONDITIONALLY',
};

ENCODING FCCMP_D_floatccmp => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 Rm:u=0bxxxxx cond:u=0bxxxx ig3=0b01 Rn:u=0bxxxxx op:u=0b0 nzcv:u=0bxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FCCMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatccmp page=FCCMP_float cclass=FCCMP_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON|CONDITIONALLY',
};

ENCODING FCCMPE_H_floatccmp => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 Rm:u=0bxxxxx cond:u=0bxxxx ig3=0b01 Rn:u=0bxxxxx op:u=0b1 nzcv:u=0bxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FCCMPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatccmp page=FCCMPE_float cclass=FCCMPE_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON|CONDITIONALLY',
};

ENCODING FCCMPE_S_floatccmp => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 Rm:u=0bxxxxx cond:u=0bxxxx ig3=0b01 Rn:u=0bxxxxx op:u=0b1 nzcv:u=0bxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FCCMPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatccmp page=FCCMPE_float cclass=FCCMPE_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON|CONDITIONALLY',
};

ENCODING FCCMPE_D_floatccmp => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 Rm:u=0bxxxxx cond:u=0bxxxx ig3=0b01 Rn:u=0bxxxxx op:u=0b1 nzcv:u=0bxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FCCMPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatccmp page=FCCMPE_float cclass=FCCMPE_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON|CONDITIONALLY',
};

ENCODING FCMEQ_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 E:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FCMEQ_advsimd_reg cclass=FCMEQ_asisdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMEQ_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 E:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FCMEQ_advsimd_reg cclass=FCMEQ_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMEQ_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 E:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMEQ reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FCMEQ_advsimd_reg cclass=FCMEQ_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMEQ_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 E:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMEQ reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FCMEQ_advsimd_reg cclass=FCMEQ_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMEQ_asisdmiscfp16_FZ => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 a=0b1 ig2=0b111100 ig3=0b0110 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCMEQ_advsimd_zero cclass=FCMEQ_asisdmiscfp16_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMEQ_asisdmisc_FZ => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b10000 ig4=0b0110 op:u=0b1 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCMEQ_advsimd_zero cclass=FCMEQ_asisdmisc_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMEQ_asimdmiscfp16_FZ => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a=0b1 ig2=0b111100 ig3=0b0110 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCMEQ_advsimd_zero cclass=FCMEQ_asimdmiscfp16_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMEQ_asimdmisc_FZ => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 ig4=0b0110 op:u=0b1 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCMEQ_advsimd_zero cclass=FCMEQ_asimdmisc_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMGE_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 E:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FCMGE_advsimd_reg cclass=FCMGE_asisdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMGE_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 E:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FCMGE_advsimd_reg cclass=FCMGE_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMGE_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 E:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMGE reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FCMGE_advsimd_reg cclass=FCMGE_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMGE_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 E:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMGE reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FCMGE_advsimd_reg cclass=FCMGE_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMGE_asisdmiscfp16_FZ => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 a=0b1 ig2=0b111100 ig3=0b0110 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCMGE_advsimd_zero cclass=FCMGE_asisdmiscfp16_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMGE_asisdmisc_FZ => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b10000 ig4=0b0110 op:u=0b0 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCMGE_advsimd_zero cclass=FCMGE_asisdmisc_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMGE_asimdmiscfp16_FZ => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a=0b1 ig2=0b111100 ig3=0b0110 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCMGE_advsimd_zero cclass=FCMGE_asimdmiscfp16_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMGE_asimdmisc_FZ => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 ig4=0b0110 op:u=0b0 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCMGE_advsimd_zero cclass=FCMGE_asimdmisc_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMGT_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 E:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FCMGT_advsimd_reg cclass=FCMGT_asisdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMGT_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 E:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FCMGT_advsimd_reg cclass=FCMGT_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMGT_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 E:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMGT reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FCMGT_advsimd_reg cclass=FCMGT_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMGT_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 E:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMGT reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FCMGT_advsimd_reg cclass=FCMGT_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMGT_asisdmiscfp16_FZ => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 a=0b1 ig2=0b111100 ig3=0b0110 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCMGT_advsimd_zero cclass=FCMGT_asisdmiscfp16_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMGT_asisdmisc_FZ => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b10000 ig4=0b0110 op:u=0b0 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCMGT_advsimd_zero cclass=FCMGT_asisdmisc_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMGT_asimdmiscfp16_FZ => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a=0b1 ig2=0b111100 ig3=0b0110 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCMGT_advsimd_zero cclass=FCMGT_asimdmiscfp16_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMGT_asimdmisc_FZ => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 ig4=0b0110 op:u=0b0 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCMGT_advsimd_zero cclass=FCMGT_asimdmisc_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMLA_asimdelem_C_H => {
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01111 size:u=0b01 L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 rot:u=0bxx ig3=0b1 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=FCMLA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FCMLA_advsimd_elt cclass=FCMLA_advsimd_elt_2reg_element',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING FCMLA_asimdelem_C_S => {
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01111 size:u=0b10 L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 rot:u=0bxx ig3=0b1 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=FCMLA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FCMLA_advsimd_elt cclass=FCMLA_advsimd_elt_2reg_element',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING FCMLA_asimdsame2_C => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b10 rot:u=0bxx ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FCMLA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=FCMLA_advsimd_vec cclass=FCMLA_asimdsame2_C',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING FCMLE_asisdmiscfp16_FZ => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 a=0b1 ig2=0b111100 ig3=0b0110 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMLE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCMLE_advsimd cclass=FCMLE_asisdmiscfp16_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMLE_asisdmisc_FZ => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b10000 ig4=0b0110 op:u=0b1 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMLE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCMLE_advsimd cclass=FCMLE_asisdmisc_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMLE_asimdmiscfp16_FZ => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a=0b1 ig2=0b111100 ig3=0b0110 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMLE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCMLE_advsimd cclass=FCMLE_asimdmiscfp16_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMLE_asimdmisc_FZ => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 ig4=0b0110 op:u=0b1 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMLE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCMLE_advsimd cclass=FCMLE_asimdmisc_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMLT_asisdmiscfp16_FZ => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 a=0b1 ig2=0b111100 opcode=0b01110 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMLT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCMLT_advsimd cclass=FCMLT_asisdmiscfp16_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMLT_asisdmisc_FZ => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b01110 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMLT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCMLT_advsimd cclass=FCMLT_asisdmisc_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMLT_asimdmiscfp16_FZ => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 a=0b1 ig2=0b111100 opcode=0b01110 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMLT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCMLT_advsimd cclass=FCMLT_asimdmiscfp16_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMLT_asimdmisc_FZ => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b01110 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMLT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCMLT_advsimd cclass=FCMLT_asimdmisc_FZ',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING FCMP_H_floatcmp => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 Rm:u=0bxxxxx op=0b00 ig3=0b1000 Rn:u=0bxxxxx opc:u=0b00 ig4=0b000',
  bitdiffs   => 'ftype == 0b11 && opc == 0b00',
  docvars    => 'compare-with=cmp-reg datatype=half instr-class=float isa=A64 mnemonic=FCMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatcmp page=FCMP_float cclass=FCMP_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON',
};

ENCODING FCMP_HZ_floatcmp => {
  name       => 'Half-precision, zero',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 Rm:u=0b00000 op=0b00 ig3=0b1000 Rn:u=0bxxxxx opc:u=0b01 ig4=0b000',
  bitdiffs   => 'ftype == 0b11 && Rm == (00000) && opc == 0b01',
  docvars    => 'compare-with=cmp-zero datatype=half instr-class=float isa=A64 mnemonic=FCMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatcmp page=FCMP_float cclass=FCMP_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON',
};

ENCODING FCMP_S_floatcmp => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 Rm:u=0bxxxxx op=0b00 ig3=0b1000 Rn:u=0bxxxxx opc:u=0b00 ig4=0b000',
  bitdiffs   => 'ftype == 0b00 && opc == 0b00',
  docvars    => 'compare-with=cmp-reg datatype=single instr-class=float isa=A64 mnemonic=FCMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatcmp page=FCMP_float cclass=FCMP_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON',
};

ENCODING FCMP_SZ_floatcmp => {
  name       => 'Single-precision, zero',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 Rm:u=0b00000 op=0b00 ig3=0b1000 Rn:u=0bxxxxx opc:u=0b01 ig4=0b000',
  bitdiffs   => 'ftype == 0b00 && Rm == (00000) && opc == 0b01',
  docvars    => 'compare-with=cmp-zero datatype=single instr-class=float isa=A64 mnemonic=FCMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatcmp page=FCMP_float cclass=FCMP_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON',
};

ENCODING FCMP_D_floatcmp => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 Rm:u=0bxxxxx op=0b00 ig3=0b1000 Rn:u=0bxxxxx opc:u=0b00 ig4=0b000',
  bitdiffs   => 'ftype == 0b01 && opc == 0b00',
  docvars    => 'compare-with=cmp-reg datatype=double instr-class=float isa=A64 mnemonic=FCMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatcmp page=FCMP_float cclass=FCMP_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON',
};

ENCODING FCMP_DZ_floatcmp => {
  name       => 'Double-precision, zero',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 Rm:u=0b00000 op=0b00 ig3=0b1000 Rn:u=0bxxxxx opc:u=0b01 ig4=0b000',
  bitdiffs   => 'ftype == 0b01 && Rm == (00000) && opc == 0b01',
  docvars    => 'compare-with=cmp-zero datatype=double instr-class=float isa=A64 mnemonic=FCMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatcmp page=FCMP_float cclass=FCMP_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON',
};

ENCODING FCMPE_H_floatcmp => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 Rm:u=0bxxxxx op=0b00 ig3=0b1000 Rn:u=0bxxxxx opc:u=0b10 ig4=0b000',
  bitdiffs   => 'ftype == 0b11 && opc == 0b10',
  docvars    => 'compare-with=cmp-reg datatype=half instr-class=float isa=A64 mnemonic=FCMPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatcmp page=FCMPE_float cclass=FCMPE_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON',
};

ENCODING FCMPE_HZ_floatcmp => {
  name       => 'Half-precision, zero',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 Rm:u=0b00000 op=0b00 ig3=0b1000 Rn:u=0bxxxxx opc:u=0b11 ig4=0b000',
  bitdiffs   => 'ftype == 0b11 && Rm == (00000) && opc == 0b11',
  docvars    => 'compare-with=cmp-zero datatype=half instr-class=float isa=A64 mnemonic=FCMPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatcmp page=FCMPE_float cclass=FCMPE_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON',
};

ENCODING FCMPE_S_floatcmp => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 Rm:u=0bxxxxx op=0b00 ig3=0b1000 Rn:u=0bxxxxx opc:u=0b10 ig4=0b000',
  bitdiffs   => 'ftype == 0b00 && opc == 0b10',
  docvars    => 'compare-with=cmp-reg datatype=single instr-class=float isa=A64 mnemonic=FCMPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatcmp page=FCMPE_float cclass=FCMPE_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON',
};

ENCODING FCMPE_SZ_floatcmp => {
  name       => 'Single-precision, zero',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 Rm:u=0b00000 op=0b00 ig3=0b1000 Rn:u=0bxxxxx opc:u=0b11 ig4=0b000',
  bitdiffs   => 'ftype == 0b00 && Rm == (00000) && opc == 0b11',
  docvars    => 'compare-with=cmp-zero datatype=single instr-class=float isa=A64 mnemonic=FCMPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatcmp page=FCMPE_float cclass=FCMPE_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON',
};

ENCODING FCMPE_D_floatcmp => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 Rm:u=0bxxxxx op=0b00 ig3=0b1000 Rn:u=0bxxxxx opc:u=0b10 ig4=0b000',
  bitdiffs   => 'ftype == 0b01 && opc == 0b10',
  docvars    => 'compare-with=cmp-reg datatype=double instr-class=float isa=A64 mnemonic=FCMPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatcmp page=FCMPE_float cclass=FCMPE_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON',
};

ENCODING FCMPE_DZ_floatcmp => {
  name       => 'Double-precision, zero',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 Rm:u=0b00000 op=0b00 ig3=0b1000 Rn:u=0bxxxxx opc:u=0b11 ig4=0b000',
  bitdiffs   => 'ftype == 0b01 && Rm == (00000) && opc == 0b11',
  docvars    => 'compare-with=cmp-zero datatype=double instr-class=float isa=A64 mnemonic=FCMPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatcmp page=FCMPE_float cclass=FCMPE_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|COMPARISON',
};

ENCODING FCSEL_H_floatsel => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 Rm:u=0bxxxxx cond:u=0bxxxx ig3=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FCSEL',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatsel page=FCSEL_float cclass=FCSEL_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|SELECTION|CONDITIONALLY',
};

ENCODING FCSEL_S_floatsel => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 Rm:u=0bxxxxx cond:u=0bxxxx ig3=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FCSEL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatsel page=FCSEL_float cclass=FCSEL_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|SELECTION|CONDITIONALLY',
};

ENCODING FCSEL_D_floatsel => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 Rm:u=0bxxxxx cond:u=0bxxxx ig3=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FCSEL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatsel page=FCSEL_float cclass=FCSEL_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|SELECTION|CONDITIONALLY',
};

ENCODING FCVT_SH_floatdp1 => {
  name       => 'Half-precision to single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 ig3=0b0001 opc:u=0b00 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11 && opc == 0b00',
  docvars    => 'convert-type=half-to-single instr-class=float isa=A64 mnemonic=FCVT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FCVT_float cclass=FCVT_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING FCVT_DH_floatdp1 => {
  name       => 'Half-precision to double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 ig3=0b0001 opc:u=0b01 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11 && opc == 0b01',
  docvars    => 'convert-type=half-to-double instr-class=float isa=A64 mnemonic=FCVT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FCVT_float cclass=FCVT_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING FCVT_HS_floatdp1 => {
  name       => 'Single-precision to half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 ig3=0b0001 opc:u=0b11 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00 && opc == 0b11',
  docvars    => 'convert-type=single-to-half instr-class=float isa=A64 mnemonic=FCVT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FCVT_float cclass=FCVT_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING FCVT_DS_floatdp1 => {
  name       => 'Single-precision to double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 ig3=0b0001 opc:u=0b01 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00 && opc == 0b01',
  docvars    => 'convert-type=single-to-double instr-class=float isa=A64 mnemonic=FCVT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FCVT_float cclass=FCVT_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING FCVT_HD_floatdp1 => {
  name       => 'Double-precision to half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 ig3=0b0001 opc:u=0b11 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01 && opc == 0b11',
  docvars    => 'convert-type=double-to-half instr-class=float isa=A64 mnemonic=FCVT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FCVT_float cclass=FCVT_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING FCVT_SD_floatdp1 => {
  name       => 'Double-precision to single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 ig3=0b0001 opc:u=0b00 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01 && opc == 0b00',
  docvars    => 'convert-type=double-to-single instr-class=float isa=A64 mnemonic=FCVT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FCVT_float cclass=FCVT_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING FCVTAS_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size=0b0 ig2=0b111100 opcode=0b11100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTAS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTAS_advsimd cclass=FCVTAS_asisdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTAS_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b11100 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTAS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTAS_advsimd cclass=FCVTAS_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTAS_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a=0b0 ig2=0b111100 opcode=0b11100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTAS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTAS_advsimd cclass=FCVTAS_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTAS_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b11100 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTAS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTAS_advsimd cclass=FCVTAS_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTAS_32H_float2int => {
  name       => 'Half-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b00 opcode:u=0b100 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11',
  docvars    => 'convert-type=half-to-32 instr-class=float isa=A64 mnemonic=FCVTAS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTAS_float cclass=FCVTAS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTAS_64H_float2int => {
  name       => 'Half-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b00 opcode:u=0b100 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11',
  docvars    => 'convert-type=half-to-64 instr-class=float isa=A64 mnemonic=FCVTAS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTAS_float cclass=FCVTAS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTAS_32S_float2int => {
  name       => 'Single-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b00 opcode:u=0b100 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00',
  docvars    => 'convert-type=single-to-32 instr-class=float isa=A64 mnemonic=FCVTAS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTAS_float cclass=FCVTAS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTAS_64S_float2int => {
  name       => 'Single-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b00 opcode:u=0b100 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b00',
  docvars    => 'convert-type=single-to-64 instr-class=float isa=A64 mnemonic=FCVTAS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTAS_float cclass=FCVTAS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTAS_32D_float2int => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b00 opcode:u=0b100 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b01',
  docvars    => 'convert-type=double-to-32 instr-class=float isa=A64 mnemonic=FCVTAS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTAS_float cclass=FCVTAS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTAS_64D_float2int => {
  name       => 'Double-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b00 opcode:u=0b100 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01',
  docvars    => 'convert-type=double-to-64 instr-class=float isa=A64 mnemonic=FCVTAS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTAS_float cclass=FCVTAS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTAU_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size=0b0 ig2=0b111100 opcode=0b11100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTAU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTAU_advsimd cclass=FCVTAU_asisdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTAU_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b11100 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTAU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTAU_advsimd cclass=FCVTAU_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTAU_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a=0b0 ig2=0b111100 opcode=0b11100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTAU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTAU_advsimd cclass=FCVTAU_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTAU_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b11100 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTAU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTAU_advsimd cclass=FCVTAU_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTAU_32H_float2int => {
  name       => 'Half-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b00 opcode:u=0b101 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11',
  docvars    => 'convert-type=half-to-32 instr-class=float isa=A64 mnemonic=FCVTAU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTAU_float cclass=FCVTAU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTAU_64H_float2int => {
  name       => 'Half-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b00 opcode:u=0b101 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11',
  docvars    => 'convert-type=half-to-64 instr-class=float isa=A64 mnemonic=FCVTAU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTAU_float cclass=FCVTAU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTAU_32S_float2int => {
  name       => 'Single-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b00 opcode:u=0b101 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00',
  docvars    => 'convert-type=single-to-32 instr-class=float isa=A64 mnemonic=FCVTAU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTAU_float cclass=FCVTAU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTAU_64S_float2int => {
  name       => 'Single-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b00 opcode:u=0b101 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b00',
  docvars    => 'convert-type=single-to-64 instr-class=float isa=A64 mnemonic=FCVTAU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTAU_float cclass=FCVTAU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTAU_32D_float2int => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b00 opcode:u=0b101 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b01',
  docvars    => 'convert-type=double-to-32 instr-class=float isa=A64 mnemonic=FCVTAU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTAU_float cclass=FCVTAU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTAU_64D_float2int => {
  name       => 'Double-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b00 opcode:u=0b101 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01',
  docvars    => 'convert-type=double-to-64 instr-class=float isa=A64 mnemonic=FCVTAU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTAU_float cclass=FCVTAU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTL_asimdmisc_L => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b10111 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTL_advsimd cclass=FCVTL_asimdmisc_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING FCVTMS_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 o2:u=0b0 ig2=0b111100 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTMS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTMS_advsimd cclass=FCVTMS_asisdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTMS_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTMS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTMS_advsimd cclass=FCVTMS_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTMS_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b0 ig2=0b111100 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTMS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTMS_advsimd cclass=FCVTMS_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTMS_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTMS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTMS_advsimd cclass=FCVTMS_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTMS_32H_float2int => {
  name       => 'Half-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b10 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11',
  docvars    => 'convert-type=half-to-32 instr-class=float isa=A64 mnemonic=FCVTMS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTMS_float cclass=FCVTMS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTMS_64H_float2int => {
  name       => 'Half-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b10 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11',
  docvars    => 'convert-type=half-to-64 instr-class=float isa=A64 mnemonic=FCVTMS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTMS_float cclass=FCVTMS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTMS_32S_float2int => {
  name       => 'Single-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b10 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00',
  docvars    => 'convert-type=single-to-32 instr-class=float isa=A64 mnemonic=FCVTMS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTMS_float cclass=FCVTMS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTMS_64S_float2int => {
  name       => 'Single-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b10 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b00',
  docvars    => 'convert-type=single-to-64 instr-class=float isa=A64 mnemonic=FCVTMS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTMS_float cclass=FCVTMS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTMS_32D_float2int => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b10 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b01',
  docvars    => 'convert-type=double-to-32 instr-class=float isa=A64 mnemonic=FCVTMS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTMS_float cclass=FCVTMS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTMS_64D_float2int => {
  name       => 'Double-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b10 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01',
  docvars    => 'convert-type=double-to-64 instr-class=float isa=A64 mnemonic=FCVTMS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTMS_float cclass=FCVTMS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTMU_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 o2:u=0b0 ig2=0b111100 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTMU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTMU_advsimd cclass=FCVTMU_asisdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTMU_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTMU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTMU_advsimd cclass=FCVTMU_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTMU_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b0 ig2=0b111100 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTMU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTMU_advsimd cclass=FCVTMU_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTMU_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTMU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTMU_advsimd cclass=FCVTMU_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTMU_32H_float2int => {
  name       => 'Half-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b10 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11',
  docvars    => 'convert-type=half-to-32 instr-class=float isa=A64 mnemonic=FCVTMU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTMU_float cclass=FCVTMU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTMU_64H_float2int => {
  name       => 'Half-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b10 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11',
  docvars    => 'convert-type=half-to-64 instr-class=float isa=A64 mnemonic=FCVTMU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTMU_float cclass=FCVTMU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTMU_32S_float2int => {
  name       => 'Single-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b10 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00',
  docvars    => 'convert-type=single-to-32 instr-class=float isa=A64 mnemonic=FCVTMU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTMU_float cclass=FCVTMU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTMU_64S_float2int => {
  name       => 'Single-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b10 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b00',
  docvars    => 'convert-type=single-to-64 instr-class=float isa=A64 mnemonic=FCVTMU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTMU_float cclass=FCVTMU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTMU_32D_float2int => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b10 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b01',
  docvars    => 'convert-type=double-to-32 instr-class=float isa=A64 mnemonic=FCVTMU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTMU_float cclass=FCVTMU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTMU_64D_float2int => {
  name       => 'Double-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b10 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01',
  docvars    => 'convert-type=double-to-64 instr-class=float isa=A64 mnemonic=FCVTMU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTMU_float cclass=FCVTMU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTN_asimdmisc_N => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b10110 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTN',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTN_advsimd cclass=FCVTN_asimdmisc_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING FCVTNS_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 o2:u=0b0 ig2=0b111100 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTNS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTNS_advsimd cclass=FCVTNS_asisdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTNS_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTNS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTNS_advsimd cclass=FCVTNS_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTNS_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b0 ig2=0b111100 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTNS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTNS_advsimd cclass=FCVTNS_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTNS_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTNS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTNS_advsimd cclass=FCVTNS_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTNS_32H_float2int => {
  name       => 'Half-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b00 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11',
  docvars    => 'convert-type=half-to-32 instr-class=float isa=A64 mnemonic=FCVTNS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTNS_float cclass=FCVTNS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTNS_64H_float2int => {
  name       => 'Half-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b00 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11',
  docvars    => 'convert-type=half-to-64 instr-class=float isa=A64 mnemonic=FCVTNS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTNS_float cclass=FCVTNS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTNS_32S_float2int => {
  name       => 'Single-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b00 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00',
  docvars    => 'convert-type=single-to-32 instr-class=float isa=A64 mnemonic=FCVTNS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTNS_float cclass=FCVTNS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTNS_64S_float2int => {
  name       => 'Single-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b00 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b00',
  docvars    => 'convert-type=single-to-64 instr-class=float isa=A64 mnemonic=FCVTNS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTNS_float cclass=FCVTNS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTNS_32D_float2int => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b00 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b01',
  docvars    => 'convert-type=double-to-32 instr-class=float isa=A64 mnemonic=FCVTNS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTNS_float cclass=FCVTNS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTNS_64D_float2int => {
  name       => 'Double-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b00 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01',
  docvars    => 'convert-type=double-to-64 instr-class=float isa=A64 mnemonic=FCVTNS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTNS_float cclass=FCVTNS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTNU_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 o2:u=0b0 ig2=0b111100 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTNU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTNU_advsimd cclass=FCVTNU_asisdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTNU_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTNU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTNU_advsimd cclass=FCVTNU_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTNU_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b0 ig2=0b111100 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTNU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTNU_advsimd cclass=FCVTNU_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTNU_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTNU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTNU_advsimd cclass=FCVTNU_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTNU_32H_float2int => {
  name       => 'Half-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b00 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11',
  docvars    => 'convert-type=half-to-32 instr-class=float isa=A64 mnemonic=FCVTNU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTNU_float cclass=FCVTNU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTNU_64H_float2int => {
  name       => 'Half-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b00 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11',
  docvars    => 'convert-type=half-to-64 instr-class=float isa=A64 mnemonic=FCVTNU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTNU_float cclass=FCVTNU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTNU_32S_float2int => {
  name       => 'Single-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b00 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00',
  docvars    => 'convert-type=single-to-32 instr-class=float isa=A64 mnemonic=FCVTNU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTNU_float cclass=FCVTNU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTNU_64S_float2int => {
  name       => 'Single-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b00 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b00',
  docvars    => 'convert-type=single-to-64 instr-class=float isa=A64 mnemonic=FCVTNU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTNU_float cclass=FCVTNU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTNU_32D_float2int => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b00 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b01',
  docvars    => 'convert-type=double-to-32 instr-class=float isa=A64 mnemonic=FCVTNU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTNU_float cclass=FCVTNU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTNU_64D_float2int => {
  name       => 'Double-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b00 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01',
  docvars    => 'convert-type=double-to-64 instr-class=float isa=A64 mnemonic=FCVTNU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTNU_float cclass=FCVTNU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTPS_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 o2:u=0b1 ig2=0b111100 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTPS_advsimd cclass=FCVTPS_asisdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTPS_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTPS_advsimd cclass=FCVTPS_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTPS_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b1 ig2=0b111100 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTPS_advsimd cclass=FCVTPS_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTPS_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTPS_advsimd cclass=FCVTPS_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTPS_32H_float2int => {
  name       => 'Half-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b01 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11',
  docvars    => 'convert-type=half-to-32 instr-class=float isa=A64 mnemonic=FCVTPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTPS_float cclass=FCVTPS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTPS_64H_float2int => {
  name       => 'Half-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b01 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11',
  docvars    => 'convert-type=half-to-64 instr-class=float isa=A64 mnemonic=FCVTPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTPS_float cclass=FCVTPS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTPS_32S_float2int => {
  name       => 'Single-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b01 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00',
  docvars    => 'convert-type=single-to-32 instr-class=float isa=A64 mnemonic=FCVTPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTPS_float cclass=FCVTPS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTPS_64S_float2int => {
  name       => 'Single-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b01 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b00',
  docvars    => 'convert-type=single-to-64 instr-class=float isa=A64 mnemonic=FCVTPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTPS_float cclass=FCVTPS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTPS_32D_float2int => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b01 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b01',
  docvars    => 'convert-type=double-to-32 instr-class=float isa=A64 mnemonic=FCVTPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTPS_float cclass=FCVTPS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTPS_64D_float2int => {
  name       => 'Double-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b01 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01',
  docvars    => 'convert-type=double-to-64 instr-class=float isa=A64 mnemonic=FCVTPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTPS_float cclass=FCVTPS_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTPU_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 o2:u=0b1 ig2=0b111100 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTPU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTPU_advsimd cclass=FCVTPU_asisdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTPU_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTPU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTPU_advsimd cclass=FCVTPU_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTPU_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b1 ig2=0b111100 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTPU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTPU_advsimd cclass=FCVTPU_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTPU_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTPU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTPU_advsimd cclass=FCVTPU_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTPU_32H_float2int => {
  name       => 'Half-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b01 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11',
  docvars    => 'convert-type=half-to-32 instr-class=float isa=A64 mnemonic=FCVTPU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTPU_float cclass=FCVTPU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTPU_64H_float2int => {
  name       => 'Half-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b01 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11',
  docvars    => 'convert-type=half-to-64 instr-class=float isa=A64 mnemonic=FCVTPU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTPU_float cclass=FCVTPU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTPU_32S_float2int => {
  name       => 'Single-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b01 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00',
  docvars    => 'convert-type=single-to-32 instr-class=float isa=A64 mnemonic=FCVTPU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTPU_float cclass=FCVTPU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTPU_64S_float2int => {
  name       => 'Single-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b01 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b00',
  docvars    => 'convert-type=single-to-64 instr-class=float isa=A64 mnemonic=FCVTPU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTPU_float cclass=FCVTPU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTPU_32D_float2int => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b01 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b01',
  docvars    => 'convert-type=double-to-32 instr-class=float isa=A64 mnemonic=FCVTPU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTPU_float cclass=FCVTPU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTPU_64D_float2int => {
  name       => 'Double-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b01 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01',
  docvars    => 'convert-type=double-to-64 instr-class=float isa=A64 mnemonic=FCVTPU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTPU_float cclass=FCVTPU_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTXN_asisdmisc_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b10110 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=FCVTXN',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTXN_advsimd cclass=FCVTXN_asisdmisc_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING FCVTXN_asimdmisc_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b10110 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTXN',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTXN_advsimd cclass=FCVTXN_asimdmisc_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING FCVTZS_asisdshf_C => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx opcode=0b11111 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=FCVTZS_advsimd_fix cclass=FCVTZS_asisdshf_C',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTZS_asimdshf_C => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b11111 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=FCVTZS_advsimd_fix cclass=FCVTZS_asimdshf_C',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTZS_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 o2:u=0b1 ig2=0b111100 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTZS_advsimd_int cclass=FCVTZS_asisdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTZS_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTZS_advsimd_int cclass=FCVTZS_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTZS_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b1 ig2=0b111100 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTZS_advsimd_int cclass=FCVTZS_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTZS_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTZS_advsimd_int cclass=FCVTZS_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING FCVTZS_32H_float2fix => {
  name       => 'Half-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b0 rmode:u=0b11 opcode:u=0b000 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11',
  docvars    => 'convert-type=half-to-fix32 instr-class=float isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=FCVTZS_float_fix cclass=FCVTZS_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTZS_64H_float2fix => {
  name       => 'Half-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b0 rmode:u=0b11 opcode:u=0b000 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11',
  docvars    => 'convert-type=half-to-fix64 instr-class=float isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=FCVTZS_float_fix cclass=FCVTZS_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTZS_32S_float2fix => {
  name       => 'Single-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b0 rmode:u=0b11 opcode:u=0b000 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00',
  docvars    => 'convert-type=single-to-fix32 instr-class=float isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=FCVTZS_float_fix cclass=FCVTZS_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTZS_64S_float2fix => {
  name       => 'Single-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b0 rmode:u=0b11 opcode:u=0b000 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b00',
  docvars    => 'convert-type=single-to-fix64 instr-class=float isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=FCVTZS_float_fix cclass=FCVTZS_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTZS_32D_float2fix => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b0 rmode:u=0b11 opcode:u=0b000 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b01',
  docvars    => 'convert-type=double-to-fix32 instr-class=float isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=FCVTZS_float_fix cclass=FCVTZS_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTZS_64D_float2fix => {
  name       => 'Double-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b0 rmode:u=0b11 opcode:u=0b000 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01',
  docvars    => 'convert-type=double-to-fix64 instr-class=float isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=FCVTZS_float_fix cclass=FCVTZS_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTZS_32H_float2int => {
  name       => 'Half-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b11 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11',
  docvars    => 'convert-type=half-to-32 instr-class=float isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTZS_float_int cclass=FCVTZS_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTZS_64H_float2int => {
  name       => 'Half-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b11 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11',
  docvars    => 'convert-type=half-to-64 instr-class=float isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTZS_float_int cclass=FCVTZS_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTZS_32S_float2int => {
  name       => 'Single-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b11 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00',
  docvars    => 'convert-type=single-to-32 instr-class=float isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTZS_float_int cclass=FCVTZS_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTZS_64S_float2int => {
  name       => 'Single-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b11 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b00',
  docvars    => 'convert-type=single-to-64 instr-class=float isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTZS_float_int cclass=FCVTZS_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTZS_32D_float2int => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b11 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b01',
  docvars    => 'convert-type=double-to-32 instr-class=float isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTZS_float_int cclass=FCVTZS_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTZS_64D_float2int => {
  name       => 'Double-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b11 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01',
  docvars    => 'convert-type=double-to-64 instr-class=float isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTZS_float_int cclass=FCVTZS_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FCVTZU_asisdshf_C => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx opcode=0b11111 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=FCVTZU_advsimd_fix cclass=FCVTZU_asisdshf_C',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTZU_asimdshf_C => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b11111 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=FCVTZU_advsimd_fix cclass=FCVTZU_asimdshf_C',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTZU_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 o2:u=0b1 ig2=0b111100 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTZU_advsimd_int cclass=FCVTZU_asisdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTZU_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTZU_advsimd_int cclass=FCVTZU_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTZU_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b1 ig2=0b111100 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTZU_advsimd_int cclass=FCVTZU_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTZU_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTZU_advsimd_int cclass=FCVTZU_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING FCVTZU_32H_float2fix => {
  name       => 'Half-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b0 rmode:u=0b11 opcode:u=0b001 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11',
  docvars    => 'convert-type=half-to-fix32 instr-class=float isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=FCVTZU_float_fix cclass=FCVTZU_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTZU_64H_float2fix => {
  name       => 'Half-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b0 rmode:u=0b11 opcode:u=0b001 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11',
  docvars    => 'convert-type=half-to-fix64 instr-class=float isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=FCVTZU_float_fix cclass=FCVTZU_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTZU_32S_float2fix => {
  name       => 'Single-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b0 rmode:u=0b11 opcode:u=0b001 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00',
  docvars    => 'convert-type=single-to-fix32 instr-class=float isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=FCVTZU_float_fix cclass=FCVTZU_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTZU_64S_float2fix => {
  name       => 'Single-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b0 rmode:u=0b11 opcode:u=0b001 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b00',
  docvars    => 'convert-type=single-to-fix64 instr-class=float isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=FCVTZU_float_fix cclass=FCVTZU_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTZU_32D_float2fix => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b0 rmode:u=0b11 opcode:u=0b001 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b01',
  docvars    => 'convert-type=double-to-fix32 instr-class=float isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=FCVTZU_float_fix cclass=FCVTZU_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTZU_64D_float2fix => {
  name       => 'Double-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b0 rmode:u=0b11 opcode:u=0b001 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01',
  docvars    => 'convert-type=double-to-fix64 instr-class=float isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=FCVTZU_float_fix cclass=FCVTZU_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTZU_32H_float2int => {
  name       => 'Half-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b11 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11',
  docvars    => 'convert-type=half-to-32 instr-class=float isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTZU_float_int cclass=FCVTZU_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTZU_64H_float2int => {
  name       => 'Half-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b11 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11',
  docvars    => 'convert-type=half-to-64 instr-class=float isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTZU_float_int cclass=FCVTZU_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTZU_32S_float2int => {
  name       => 'Single-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b11 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00',
  docvars    => 'convert-type=single-to-32 instr-class=float isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTZU_float_int cclass=FCVTZU_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTZU_64S_float2int => {
  name       => 'Single-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b11 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b00',
  docvars    => 'convert-type=single-to-64 instr-class=float isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTZU_float_int cclass=FCVTZU_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTZU_32D_float2int => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b11 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b01',
  docvars    => 'convert-type=double-to-32 instr-class=float isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTZU_float_int cclass=FCVTZU_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FCVTZU_64D_float2int => {
  name       => 'Double-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b11 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01',
  docvars    => 'convert-type=double-to-64 instr-class=float isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTZU_float_int cclass=FCVTZU_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING FDIV_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 a=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FDIV reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FDIV_advsimd cclass=FDIV_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FDIV_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FDIV reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FDIV_advsimd cclass=FDIV_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FDIV_H_floatdp2 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 Rm:u=0bxxxxx opcode=0b0001 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FDIV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FDIV_float cclass=FDIV_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING FDIV_S_floatdp2 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 Rm:u=0bxxxxx opcode=0b0001 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FDIV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FDIV_float cclass=FDIV_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING FDIV_D_floatdp2 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 Rm:u=0bxxxxx opcode=0b0001 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FDIV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FDIV_float cclass=FDIV_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING FJCVTZS_32D_float2int => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b11 opcode:u=0b110 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-32 instr-class=float isa=A64 mnemonic=FJCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FJCVTZS cclass=FJCVTZS_32D_float2int',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING FMADD_H_floatdp3 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11111 ftype:u=0b11 o1:u=0b0 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FMADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp3 page=FMADD_float cclass=FMADD_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|FUSED_OP',
};

ENCODING FMADD_S_floatdp3 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11111 ftype:u=0b00 o1:u=0b0 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FMADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp3 page=FMADD_float cclass=FMADD_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|FUSED_OP',
};

ENCODING FMADD_D_floatdp3 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11111 ftype:u=0b01 o1:u=0b0 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FMADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp3 page=FMADD_float cclass=FMADD_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|FUSED_OP',
};

ENCODING FMAX_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o1:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b110 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMAX reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMAX_advsimd cclass=FMAX_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMAX_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o1:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11110 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMAX reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMAX_advsimd cclass=FMAX_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMAX_H_floatdp2 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b00 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FMAX',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMAX_float cclass=FMAX_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING FMAX_S_floatdp2 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b00 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FMAX',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMAX_float cclass=FMAX_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING FMAX_D_floatdp2 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b00 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FMAX',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMAX_float cclass=FMAX_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING FMAXNM_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 Op3=0b000 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMAXNM reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMAXNM_advsimd cclass=FMAXNM_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMAXNM_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o1:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMAXNM reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMAXNM_advsimd cclass=FMAXNM_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMAXNM_H_floatdp2 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b10 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FMAXNM',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMAXNM_float cclass=FMAXNM_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING FMAXNM_S_floatdp2 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b10 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FMAXNM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMAXNM_float cclass=FMAXNM_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING FMAXNM_D_floatdp2 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b10 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FMAXNM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMAXNM_float cclass=FMAXNM_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING FMAXNMP_asisdpair_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 o1:u=0b0 sz:u=0b0 ig2=0b11000 opcode=0b01100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FMAXNMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FMAXNMP_advsimd_pair cclass=FMAXNMP_asisdpair_only_H',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMAXNMP_asisdpair_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 o1:u=0b0 sz:u=0bx ig2=0b11000 opcode=0b01100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMAXNMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FMAXNMP_advsimd_pair cclass=FMAXNMP_asisdpair_only_SD',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMAXNMP_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 Op3=0b000 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMAXNMP reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMAXNMP_advsimd_vec cclass=FMAXNMP_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMAXNMP_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o1:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMAXNMP reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMAXNMP_advsimd_vec cclass=FMAXNMP_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMAXNMV_asimdall_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 o1:u=0b0 ig2=0b0 ig3=0b11000 opcode=0b01100 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FMAXNMV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=FMAXNMV_advsimd cclass=FMAXNMV_asimdall_only_H',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMAXNMV_asimdall_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 o1:u=0b0 sz:u=0bx ig2=0b11000 opcode=0b01100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMAXNMV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=FMAXNMV_advsimd cclass=FMAXNMV_asimdall_only_SD',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMAXP_asisdpair_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 o1:u=0b0 sz:u=0b0 ig2=0b11000 opcode=0b01111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FMAXP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FMAXP_advsimd_pair cclass=FMAXP_asisdpair_only_H',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMAXP_asisdpair_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 o1:u=0b0 sz:u=0bx ig2=0b11000 opcode=0b01111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMAXP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FMAXP_advsimd_pair cclass=FMAXP_asisdpair_only_SD',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMAXP_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o1:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b110 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMAXP reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMAXP_advsimd_vec cclass=FMAXP_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMAXP_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o1:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11110 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMAXP reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMAXP_advsimd_vec cclass=FMAXP_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMAXV_asimdall_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 o1:u=0b0 ig2=0b0 ig3=0b11000 opcode=0b01111 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FMAXV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=FMAXV_advsimd cclass=FMAXV_asimdall_only_H',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMAXV_asimdall_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 o1:u=0b0 sz:u=0bx ig2=0b11000 opcode=0b01111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMAXV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=FMAXV_advsimd cclass=FMAXV_asimdall_only_SD',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMIN_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o1:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b110 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMIN reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMIN_advsimd cclass=FMIN_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMIN_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o1:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11110 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMIN reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMIN_advsimd cclass=FMIN_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMIN_H_floatdp2 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b01 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FMIN',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMIN_float cclass=FMIN_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING FMIN_S_floatdp2 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b01 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FMIN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMIN_float cclass=FMIN_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING FMIN_D_floatdp2 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b01 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FMIN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMIN_float cclass=FMIN_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING FMINNM_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 Op3=0b000 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMINNM reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMINNM_advsimd cclass=FMINNM_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMINNM_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o1:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMINNM reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMINNM_advsimd cclass=FMINNM_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMINNM_H_floatdp2 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b11 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FMINNM',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMINNM_float cclass=FMINNM_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING FMINNM_S_floatdp2 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b11 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FMINNM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMINNM_float cclass=FMINNM_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING FMINNM_D_floatdp2 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b11 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FMINNM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMINNM_float cclass=FMINNM_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING FMINNMP_asisdpair_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 o1:u=0b1 sz:u=0b0 ig2=0b11000 opcode=0b01100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FMINNMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FMINNMP_advsimd_pair cclass=FMINNMP_asisdpair_only_H',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMINNMP_asisdpair_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 o1:u=0b1 sz:u=0bx ig2=0b11000 opcode=0b01100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMINNMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FMINNMP_advsimd_pair cclass=FMINNMP_asisdpair_only_SD',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMINNMP_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 Op3=0b000 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMINNMP reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMINNMP_advsimd_vec cclass=FMINNMP_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMINNMP_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o1:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMINNMP reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMINNMP_advsimd_vec cclass=FMINNMP_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMINNMV_asimdall_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 o1:u=0b1 ig2=0b0 ig3=0b11000 opcode=0b01100 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FMINNMV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=FMINNMV_advsimd cclass=FMINNMV_asimdall_only_H',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMINNMV_asimdall_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 o1:u=0b1 sz:u=0bx ig2=0b11000 opcode=0b01100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMINNMV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=FMINNMV_advsimd cclass=FMINNMV_asimdall_only_SD',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMINP_asisdpair_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 o1:u=0b1 sz:u=0b0 ig2=0b11000 opcode=0b01111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FMINP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FMINP_advsimd_pair cclass=FMINP_asisdpair_only_H',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMINP_asisdpair_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 o1:u=0b1 sz:u=0bx ig2=0b11000 opcode=0b01111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMINP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FMINP_advsimd_pair cclass=FMINP_asisdpair_only_SD',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMINP_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o1:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b110 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMINP reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMINP_advsimd_vec cclass=FMINP_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMINP_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o1:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11110 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMINP reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMINP_advsimd_vec cclass=FMINP_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMINV_asimdall_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 o1:u=0b1 ig2=0b0 ig3=0b11000 opcode=0b01111 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FMINV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=FMINV_advsimd cclass=FMINV_asimdall_only_H',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMINV_asimdall_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 o1:u=0b1 sz:u=0bx ig2=0b11000 opcode=0b01111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMINV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=FMINV_advsimd cclass=FMINV_asimdall_only_SD',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING FMLA_asisdelem_RH_H => {
  name       => 'Scalar, half-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 size=0b00 L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b0 ig3=0b01 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-reguse=2reg-scalar advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FMLA reguse-datatype=2reg-scalar-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=FMLA_advsimd_elt cclass=FMLA_asisdelem_RH_H',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLA_asisdelem_R_SD => {
  name       => 'Scalar, single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 ig2=0b1 sz:u=0bx L:u=0bx M:u=0bx Rm:u=0bxxxx ig3=0b0 o2:u=0b0 ig4=0b01 H:u=0bx ig5=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-reguse=2reg-scalar advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMLA reguse-datatype=2reg-scalar-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=FMLA_advsimd_elt cclass=FMLA_asisdelem_R_SD',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLA_asimdelem_RH_H => {
  name       => 'Vector, half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size=0b00 L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b0 ig3=0b01 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=half instr-class=advsimd isa=A64 mnemonic=FMLA reguse-datatype=2reg-element-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMLA_advsimd_elt cclass=FMLA_asimdelem_RH_H',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLA_asimdelem_R_SD => {
  name       => 'Vector, single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 ig2=0b1 sz:u=0bx L:u=0bx M:u=0bx Rm:u=0bxxxx ig3=0b0 o2:u=0b0 ig4=0b01 H:u=0bx ig5=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMLA reguse-datatype=2reg-element-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMLA_advsimd_elt cclass=FMLA_asimdelem_R_SD',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLA_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 a:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b001 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMLA reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMLA_advsimd_vec cclass=FMLA_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLA_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 op:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMLA reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMLA_advsimd_vec cclass=FMLA_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLAL_asimdelem_LH => {
  name       => 'FMLAL',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size[1]=0b1 sz:u=0b0 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode[3]=0b0 S:u=0b0 opcode[1:0]=0b00 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=FMLAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMLAL_advsimd_elt cclass=FMLAL_asimdelem_LH',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLAL2_asimdelem_LH => {
  name       => 'FMLAL2',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01111 size[1]=0b1 sz:u=0b0 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode[3]=0b1 S:u=0b0 opcode[1:0]=0b00 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=FMLAL2',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMLAL_advsimd_elt cclass=FMLAL2_asimdelem_LH',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLAL_asimdsame_F => {
  name       => 'FMLAL',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 S:u=0b0 sz:u=0b0 ig2=0b1 Rm:u=0bxxxxx opcode[4]=0b1 opcode[3:0]=0b1101 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMLAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMLAL_advsimd_vec cclass=FMLAL_asimdsame_F',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLAL2_asimdsame_F => {
  name       => 'FMLAL2',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 S:u=0b0 sz:u=0b0 ig2=0b1 Rm:u=0bxxxxx opcode[4]=0b1 opcode[3:0]=0b1001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMLAL2',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMLAL_advsimd_vec cclass=FMLAL2_asimdsame_F',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLS_asisdelem_RH_H => {
  name       => 'Scalar, half-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 size=0b00 L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b1 ig3=0b01 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-reguse=2reg-scalar advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FMLS reguse-datatype=2reg-scalar-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=FMLS_advsimd_elt cclass=FMLS_asisdelem_RH_H',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLS_asisdelem_R_SD => {
  name       => 'Scalar, single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 ig2=0b1 sz:u=0bx L:u=0bx M:u=0bx Rm:u=0bxxxx ig3=0b0 o2:u=0b1 ig4=0b01 H:u=0bx ig5=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-reguse=2reg-scalar advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMLS reguse-datatype=2reg-scalar-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=FMLS_advsimd_elt cclass=FMLS_asisdelem_R_SD',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLS_asimdelem_RH_H => {
  name       => 'Vector, half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size=0b00 L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b1 ig3=0b01 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=half instr-class=advsimd isa=A64 mnemonic=FMLS reguse-datatype=2reg-element-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMLS_advsimd_elt cclass=FMLS_asimdelem_RH_H',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLS_asimdelem_R_SD => {
  name       => 'Vector, single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 ig2=0b1 sz:u=0bx L:u=0bx M:u=0bx Rm:u=0bxxxx ig3=0b0 o2:u=0b1 ig4=0b01 H:u=0bx ig5=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMLS reguse-datatype=2reg-element-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMLS_advsimd_elt cclass=FMLS_asimdelem_R_SD',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLS_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 a:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b001 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMLS reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMLS_advsimd_vec cclass=FMLS_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLS_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 op:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMLS reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMLS_advsimd_vec cclass=FMLS_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLSL_asimdelem_LH => {
  name       => 'FMLSL',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size[1]=0b1 sz:u=0b0 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode[3]=0b0 S:u=0b1 opcode[1:0]=0b00 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=FMLSL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMLSL_advsimd_elt cclass=FMLSL_asimdelem_LH',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLSL2_asimdelem_LH => {
  name       => 'FMLSL2',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01111 size[1]=0b1 sz:u=0b0 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode[3]=0b1 S:u=0b1 opcode[1:0]=0b00 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=FMLSL2',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMLSL_advsimd_elt cclass=FMLSL2_asimdelem_LH',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLSL_asimdsame_F => {
  name       => 'FMLSL',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 S:u=0b1 sz:u=0b0 ig2=0b1 Rm:u=0bxxxxx opcode[4]=0b1 opcode[3:0]=0b1101 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMLSL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMLSL_advsimd_vec cclass=FMLSL_asimdsame_F',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMLSL2_asimdsame_F => {
  name       => 'FMLSL2',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 S:u=0b1 sz:u=0b0 ig2=0b1 Rm:u=0bxxxxx opcode[4]=0b1 opcode[3:0]=0b1001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMLSL2',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMLSL_advsimd_vec cclass=FMLSL2_asimdsame_F',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|FUSED_OP',
};

ENCODING FMOV_asimdimm_H_h => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx op=0b0 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode=0b1111 o2=0b1 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  docvars    => 'asimdimm-datatype=per-half instr-class=advsimd isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=FMOV_advsimd cclass=FMOV_asimdimm_H_h',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING FMOV_asimdimm_S_s => {
  name       => 'Single-precision',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0b0 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0b1111 o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  bitdiffs   => 'op == 0b0',
  docvars    => 'asimdimm-datatype=per-single datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=FMOV_advsimd cclass=single_and_double',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING FMOV_asimdimm_D2_d => {
  name       => 'Double-precision',
  diagram    => 'ig0=0b0 Q:u=0b1 op:u=0b1 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0b1111 o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  bitdiffs   => 'Q == 0b1 && op == 0b1',
  docvars    => 'asimdimm-datatype=per-double datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=FMOV_advsimd cclass=single_and_double',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING FMOV_H_floatdp1 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 ig3=0b0000 opc:u=0b00 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FMOV_float cclass=FMOV_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING FMOV_S_floatdp1 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 ig3=0b0000 opc:u=0b00 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FMOV_float cclass=FMOV_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING FMOV_D_floatdp1 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 ig3=0b0000 opc:u=0b00 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FMOV_float cclass=FMOV_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING FMOV_32H_float2int => {
  name       => 'Half-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b00 opcode:u=0b110 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11 && rmode == 0b00 && opcode == 0b110',
  docvars    => 'convert-type=half-to-32 instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FMOV_float_gen cclass=FMOV_float_gen_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING FMOV_64H_float2int => {
  name       => 'Half-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b00 opcode:u=0b110 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11 && rmode == 0b00 && opcode == 0b110',
  docvars    => 'convert-type=half-to-64 instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FMOV_float_gen cclass=FMOV_float_gen_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING FMOV_H32_float2int => {
  name       => '32-bit to half-precision',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b00 opcode:u=0b111 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11 && rmode == 0b00 && opcode == 0b111',
  docvars    => 'convert-type=32-to-half instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FMOV_float_gen cclass=FMOV_float_gen_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING FMOV_S32_float2int => {
  name       => '32-bit to single-precision',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b00 opcode:u=0b111 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00 && rmode == 0b00 && opcode == 0b111',
  docvars    => 'convert-type=32-to-single instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FMOV_float_gen cclass=FMOV_float_gen_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING FMOV_32S_float2int => {
  name       => 'Single-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b00 opcode:u=0b110 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00 && rmode == 0b00 && opcode == 0b110',
  docvars    => 'convert-type=single-to-32 instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FMOV_float_gen cclass=FMOV_float_gen_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING FMOV_H64_float2int => {
  name       => '64-bit to half-precision',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b00 opcode:u=0b111 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11 && rmode == 0b00 && opcode == 0b111',
  docvars    => 'convert-type=64-to-half instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FMOV_float_gen cclass=FMOV_float_gen_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING FMOV_D64_float2int => {
  name       => '64-bit to double-precision',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b00 opcode:u=0b111 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01 && rmode == 0b00 && opcode == 0b111',
  docvars    => 'convert-type=64-to-double instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FMOV_float_gen cclass=FMOV_float_gen_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING FMOV_V64I_float2int => {
  name       => '64-bit to top half of 128-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b10 ig2=0b1 rmode:u=0b01 opcode:u=0b111 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b10 && rmode == 0b01 && opcode == 0b111',
  docvars    => 'convert-type=64-to-quadhi instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FMOV_float_gen cclass=FMOV_float_gen_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING FMOV_64D_float2int => {
  name       => 'Double-precision to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b00 opcode:u=0b110 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01 && rmode == 0b00 && opcode == 0b110',
  docvars    => 'convert-type=double-to-64 instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FMOV_float_gen cclass=FMOV_float_gen_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING FMOV_64VX_float2int => {
  name       => 'Top half of 128-bit to 64-bit',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b10 ig2=0b1 rmode:u=0b01 opcode:u=0b110 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b10 && rmode == 0b01 && opcode == 0b110',
  docvars    => 'convert-type=quadhi-to-64 instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FMOV_float_gen cclass=FMOV_float_gen_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING FMOV_H_floatimm => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 imm8:u=0bxxxxxxxx ig3=0b100 imm5=0b00000 Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half immediate-type=imm8f instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatimm page=FMOV_float_imm cclass=imm8f',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING FMOV_S_floatimm => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 imm8:u=0bxxxxxxxx ig3=0b100 imm5=0b00000 Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single immediate-type=imm8f instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatimm page=FMOV_float_imm cclass=imm8f',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING FMOV_D_floatimm => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 imm8:u=0bxxxxxxxx ig3=0b100 imm5=0b00000 Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double immediate-type=imm8f instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatimm page=FMOV_float_imm cclass=imm8f',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING FMSUB_H_floatdp3 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11111 ftype:u=0b11 o1:u=0b0 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FMSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp3 page=FMSUB_float cclass=FMSUB_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|FUSED_OP',
};

ENCODING FMSUB_S_floatdp3 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11111 ftype:u=0b00 o1:u=0b0 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FMSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp3 page=FMSUB_float cclass=FMSUB_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|FUSED_OP',
};

ENCODING FMSUB_D_floatdp3 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11111 ftype:u=0b01 o1:u=0b0 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FMSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp3 page=FMSUB_float cclass=FMSUB_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|FUSED_OP',
};

ENCODING FMUL_asisdelem_RH_H => {
  name       => 'Scalar, half-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11111 size=0b00 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1001 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-reguse=2reg-scalar advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FMUL reguse-datatype=2reg-scalar-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=FMUL_advsimd_elt cclass=FMUL_asisdelem_RH_H',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FMUL_asisdelem_R_SD => {
  name       => 'Scalar, single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11111 ig2=0b1 sz:u=0bx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1001 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-reguse=2reg-scalar advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMUL reguse-datatype=2reg-scalar-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=FMUL_advsimd_elt cclass=FMUL_asisdelem_R_SD',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FMUL_asimdelem_RH_H => {
  name       => 'Vector, half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01111 size=0b00 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1001 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=half instr-class=advsimd isa=A64 mnemonic=FMUL reguse-datatype=2reg-element-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMUL_advsimd_elt cclass=FMUL_asimdelem_RH_H',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FMUL_asimdelem_R_SD => {
  name       => 'Vector, single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01111 ig2=0b1 sz:u=0bx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1001 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMUL reguse-datatype=2reg-element-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMUL_advsimd_elt cclass=FMUL_asimdelem_R_SD',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FMUL_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 a=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b011 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMUL reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMUL_advsimd_vec cclass=FMUL_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FMUL_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11011 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMUL reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMUL_advsimd_vec cclass=FMUL_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FMUL_H_floatdp2 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 Rm:u=0bxxxxx op:u=0b0 ig3=0b000 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FMUL',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMUL_float cclass=FMUL_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING FMUL_S_floatdp2 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 Rm:u=0bxxxxx op:u=0b0 ig3=0b000 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FMUL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMUL_float cclass=FMUL_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING FMUL_D_floatdp2 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 Rm:u=0bxxxxx op:u=0b0 ig3=0b000 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FMUL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMUL_float cclass=FMUL_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING FMULX_asisdelem_RH_H => {
  name       => 'Scalar, half-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11111 size=0b00 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1001 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-reguse=2reg-scalar advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FMULX reguse-datatype=2reg-scalar-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=FMULX_advsimd_elt cclass=FMULX_asisdelem_RH_H',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FMULX_asisdelem_R_SD => {
  name       => 'Scalar, single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11111 ig2=0b1 sz:u=0bx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1001 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-reguse=2reg-scalar advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMULX reguse-datatype=2reg-scalar-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=FMULX_advsimd_elt cclass=FMULX_asisdelem_R_SD',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FMULX_asimdelem_RH_H => {
  name       => 'Vector, half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01111 size=0b00 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1001 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=half instr-class=advsimd isa=A64 mnemonic=FMULX reguse-datatype=2reg-element-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMULX_advsimd_elt cclass=FMULX_asimdelem_RH_H',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FMULX_asimdelem_R_SD => {
  name       => 'Vector, single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01111 ig2=0b1 sz:u=0bx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1001 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMULX reguse-datatype=2reg-element-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMULX_advsimd_elt cclass=FMULX_asimdelem_R_SD',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FMULX_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 a=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b011 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FMULX',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FMULX_advsimd_vec cclass=FMULX_asisdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FMULX_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 ig2=0b0 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11011 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMULX',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FMULX_advsimd_vec cclass=FMULX_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FMULX_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 a=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b011 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMULX reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMULX_advsimd_vec cclass=FMULX_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FMULX_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11011 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMULX reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMULX_advsimd_vec cclass=FMULX_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FNEG_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a=0b1 ig2=0b111100 opcode=0b01111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FNEG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FNEG_advsimd cclass=FNEG_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING FNEG_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b01111 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FNEG',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FNEG_advsimd cclass=FNEG_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING FNEG_H_floatdp1 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 ig3=0b0000 opc:u=0b10 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FNEG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FNEG_float cclass=FNEG_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|BITWISE',
};

ENCODING FNEG_S_floatdp1 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 ig3=0b0000 opc:u=0b10 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FNEG',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FNEG_float cclass=FNEG_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|BITWISE',
};

ENCODING FNEG_D_floatdp1 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 ig3=0b0000 opc:u=0b10 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FNEG',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FNEG_float cclass=FNEG_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|BITWISE',
};

ENCODING FNMADD_H_floatdp3 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11111 ftype:u=0b11 o1:u=0b1 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FNMADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp3 page=FNMADD_float cclass=FNMADD_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|FUSED_OP',
};

ENCODING FNMADD_S_floatdp3 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11111 ftype:u=0b00 o1:u=0b1 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FNMADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp3 page=FNMADD_float cclass=FNMADD_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|FUSED_OP',
};

ENCODING FNMADD_D_floatdp3 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11111 ftype:u=0b01 o1:u=0b1 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FNMADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp3 page=FNMADD_float cclass=FNMADD_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|FUSED_OP',
};

ENCODING FNMSUB_H_floatdp3 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11111 ftype:u=0b11 o1:u=0b1 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FNMSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp3 page=FNMSUB_float cclass=FNMSUB_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|FUSED_OP',
};

ENCODING FNMSUB_S_floatdp3 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11111 ftype:u=0b00 o1:u=0b1 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FNMSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp3 page=FNMSUB_float cclass=FNMSUB_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|FUSED_OP',
};

ENCODING FNMSUB_D_floatdp3 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11111 ftype:u=0b01 o1:u=0b1 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FNMSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp3 page=FNMSUB_float cclass=FNMSUB_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|FUSED_OP',
};

ENCODING FNMUL_H_floatdp2 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 Rm:u=0bxxxxx op:u=0b1 ig3=0b000 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FNMUL',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FNMUL_float cclass=FNMUL_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|FUSED_OP',
};

ENCODING FNMUL_S_floatdp2 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 Rm:u=0bxxxxx op:u=0b1 ig3=0b000 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FNMUL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FNMUL_float cclass=FNMUL_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|FUSED_OP',
};

ENCODING FNMUL_D_floatdp2 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 Rm:u=0bxxxxx op:u=0b1 ig3=0b000 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FNMUL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FNMUL_float cclass=FNMUL_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|FUSED_OP',
};

ENCODING FRECPE_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 size=0b1 ig2=0b111100 opcode=0b11101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FRECPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FRECPE_advsimd cclass=FRECPE_asisdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FRECPE_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b11101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRECPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FRECPE_advsimd cclass=FRECPE_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FRECPE_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 a=0b1 ig2=0b111100 opcode=0b11101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRECPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRECPE_advsimd cclass=FRECPE_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FRECPE_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b11101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRECPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRECPE_advsimd cclass=FRECPE_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FRECPS_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 a=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FRECPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FRECPS_advsimd cclass=FRECPS_asisdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FRECPS_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 ig2=0b0 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRECPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FRECPS_advsimd cclass=FRECPS_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FRECPS_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 a=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRECPS reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FRECPS_advsimd cclass=FRECPS_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FRECPS_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRECPS reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FRECPS_advsimd cclass=FRECPS_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FRECPX_asisdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 a=0b1 ig2=0b111100 opcode=0b11111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FRECPX',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FRECPX_advsimd cclass=FRECPX_asisdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FRECPX_asisdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b11111 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRECPX',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FRECPX_advsimd cclass=FRECPX_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FRINT32X_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 ig4=0b1111 op:u=0b0 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINT32X',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINT32X_advsimd cclass=FRINT32X_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINT32X_S_floatdp1 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 ig3=0b0100 op:u=0b01 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FRINT32X',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINT32X_float cclass=FRINT32X_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINT32X_D_floatdp1 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 ig3=0b0100 op:u=0b01 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FRINT32X',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINT32X_float cclass=FRINT32X_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINT32Z_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 ig4=0b1111 op:u=0b0 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINT32Z',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINT32Z_advsimd cclass=FRINT32Z_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINT32Z_S_floatdp1 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 ig3=0b0100 op:u=0b00 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FRINT32Z',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINT32Z_float cclass=FRINT32Z_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINT32Z_D_floatdp1 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 ig3=0b0100 op:u=0b00 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FRINT32Z',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINT32Z_float cclass=FRINT32Z_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINT64X_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 ig4=0b1111 op:u=0b1 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINT64X',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINT64X_advsimd cclass=FRINT64X_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINT64X_S_floatdp1 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 ig3=0b0100 op:u=0b11 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FRINT64X',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINT64X_float cclass=FRINT64X_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINT64X_D_floatdp1 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 ig3=0b0100 op:u=0b11 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FRINT64X',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINT64X_float cclass=FRINT64X_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINT64Z_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 ig4=0b1111 op:u=0b1 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINT64Z',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINT64Z_advsimd cclass=FRINT64Z_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINT64Z_S_floatdp1 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 ig3=0b0100 op:u=0b10 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FRINT64Z',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINT64Z_float cclass=FRINT64Z_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINT64Z_D_floatdp1 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 ig3=0b0100 op:u=0b10 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FRINT64Z',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINT64Z_float cclass=FRINT64Z_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTA_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b0 ig2=0b111100 ig3=0b1100 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRINTA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRINTA_advsimd cclass=FRINTA_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINTA_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1100 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINTA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINTA_advsimd cclass=FRINTA_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINTA_H_floatdp1 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 ig3=0b001 rmode:u=0b100 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FRINTA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTA_float cclass=FRINTA_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTA_S_floatdp1 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 ig3=0b001 rmode:u=0b100 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FRINTA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTA_float cclass=FRINTA_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTA_D_floatdp1 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 ig3=0b001 rmode:u=0b100 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FRINTA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTA_float cclass=FRINTA_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTI_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b1 ig2=0b111100 ig3=0b1100 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRINTI',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRINTI_advsimd cclass=FRINTI_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINTI_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1100 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINTI',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINTI_advsimd cclass=FRINTI_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINTI_H_floatdp1 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 ig3=0b001 rmode:u=0b111 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FRINTI',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTI_float cclass=FRINTI_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTI_S_floatdp1 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 ig3=0b001 rmode:u=0b111 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FRINTI',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTI_float cclass=FRINTI_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTI_D_floatdp1 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 ig3=0b001 rmode:u=0b111 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FRINTI',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTI_float cclass=FRINTI_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTM_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b0 ig2=0b111100 ig3=0b1100 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRINTM',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRINTM_advsimd cclass=FRINTM_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINTM_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1100 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINTM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINTM_advsimd cclass=FRINTM_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINTM_H_floatdp1 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 ig3=0b001 rmode:u=0b010 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FRINTM',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTM_float cclass=FRINTM_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTM_S_floatdp1 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 ig3=0b001 rmode:u=0b010 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FRINTM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTM_float cclass=FRINTM_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTM_D_floatdp1 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 ig3=0b001 rmode:u=0b010 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FRINTM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTM_float cclass=FRINTM_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTN_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b0 ig2=0b111100 ig3=0b1100 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRINTN',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRINTN_advsimd cclass=FRINTN_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINTN_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1100 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINTN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINTN_advsimd cclass=FRINTN_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINTN_H_floatdp1 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 ig3=0b001 rmode:u=0b000 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FRINTN',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTN_float cclass=FRINTN_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTN_S_floatdp1 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 ig3=0b001 rmode:u=0b000 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FRINTN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTN_float cclass=FRINTN_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTN_D_floatdp1 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 ig3=0b001 rmode:u=0b000 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FRINTN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTN_float cclass=FRINTN_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTP_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b1 ig2=0b111100 ig3=0b1100 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRINTP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRINTP_advsimd cclass=FRINTP_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINTP_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1100 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINTP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINTP_advsimd cclass=FRINTP_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINTP_H_floatdp1 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 ig3=0b001 rmode:u=0b001 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FRINTP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTP_float cclass=FRINTP_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTP_S_floatdp1 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 ig3=0b001 rmode:u=0b001 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FRINTP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTP_float cclass=FRINTP_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTP_D_floatdp1 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 ig3=0b001 rmode:u=0b001 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FRINTP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTP_float cclass=FRINTP_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTX_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b0 ig2=0b111100 ig3=0b1100 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRINTX',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRINTX_advsimd cclass=FRINTX_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINTX_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1100 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINTX',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINTX_advsimd cclass=FRINTX_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINTX_H_floatdp1 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 ig3=0b001 rmode:u=0b110 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FRINTX',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTX_float cclass=FRINTX_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTX_S_floatdp1 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 ig3=0b001 rmode:u=0b110 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FRINTX',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTX_float cclass=FRINTX_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTX_D_floatdp1 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 ig3=0b001 rmode:u=0b110 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FRINTX',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTX_float cclass=FRINTX_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTZ_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b1 ig2=0b111100 ig3=0b1100 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRINTZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRINTZ_advsimd cclass=FRINTZ_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINTZ_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1100 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINTZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINTZ_advsimd cclass=FRINTZ_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING FRINTZ_H_floatdp1 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 ig3=0b001 rmode:u=0b011 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FRINTZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTZ_float cclass=FRINTZ_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTZ_S_floatdp1 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 ig3=0b001 rmode:u=0b011 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FRINTZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTZ_float cclass=FRINTZ_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRINTZ_D_floatdp1 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 ig3=0b001 rmode:u=0b011 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FRINTZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTZ_float cclass=FRINTZ_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING FRSQRTE_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 a=0b1 ig2=0b111100 opcode=0b11101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FRSQRTE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FRSQRTE_advsimd cclass=FRSQRTE_asisdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FRSQRTE_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b11101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRSQRTE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FRSQRTE_advsimd cclass=FRSQRTE_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FRSQRTE_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 a=0b1 ig2=0b111100 opcode=0b11101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRSQRTE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRSQRTE_advsimd cclass=FRSQRTE_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FRSQRTE_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b11101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRSQRTE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRSQRTE_advsimd cclass=FRSQRTE_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FRSQRTS_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 a=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FRSQRTS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FRSQRTS_advsimd cclass=FRSQRTS_asisdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FRSQRTS_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRSQRTS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FRSQRTS_advsimd cclass=FRSQRTS_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FRSQRTS_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 a=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRSQRTS reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FRSQRTS_advsimd cclass=FRSQRTS_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FRSQRTS_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRSQRTS reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FRSQRTS_advsimd cclass=FRSQRTS_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FSQRT_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 a=0b1 ig2=0b111100 opcode=0b11111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FSQRT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FSQRT_advsimd cclass=FSQRT_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FSQRT_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b11111 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FSQRT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FSQRT_advsimd cclass=FSQRT_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING FSQRT_H_floatdp1 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 ig3=0b0000 opc:u=0b11 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FSQRT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FSQRT_float cclass=FSQRT_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH',
};

ENCODING FSQRT_S_floatdp1 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 ig3=0b0000 opc:u=0b11 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FSQRT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FSQRT_float cclass=FSQRT_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH',
};

ENCODING FSQRT_D_floatdp1 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 ig3=0b0000 opc:u=0b11 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FSQRT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FSQRT_float cclass=FSQRT_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH',
};

ENCODING FSUB_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FSUB reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FSUB_advsimd cclass=FSUB_asimdsamefp16_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FSUB_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FSUB reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FSUB_advsimd cclass=FSUB_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING FSUB_H_floatdp2 => {
  name       => 'Half-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 Rm:u=0bxxxxx ig3=0b001 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b11',
  docvars    => 'datatype=half instr-class=float isa=A64 mnemonic=FSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FSUB_float cclass=FSUB_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING FSUB_S_floatdp2 => {
  name       => 'Single-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 Rm:u=0bxxxxx ig3=0b001 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b00',
  docvars    => 'datatype=single instr-class=float isa=A64 mnemonic=FSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FSUB_float cclass=FSUB_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING FSUB_D_floatdp2 => {
  name       => 'Double-precision',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 Rm:u=0bxxxxx ig3=0b001 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'ftype == 0b01',
  docvars    => 'datatype=double instr-class=float isa=A64 mnemonic=FSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FSUB_float cclass=FSUB_float_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING INS_asimdins_IV_v => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q=0b1 op=0b1 ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4:u=0bxxxx ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=INS vector-xfer-type=vector-from-element',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=INS_advsimd_elt cclass=INS_advsimd_elt_INS_asimdins_IV_v',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|INSERTION',
};

ENCODING INS_asimdins_IR_r => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q=0b1 op=0b0 ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4=0b0011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=INS vector-xfer-type=vector-from-general',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=INS_advsimd_gen cclass=INS_advsimd_gen_INS_asimdins_IR_r',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|INSERTION',
};

ENCODING LD1_asisdlse_R1_1v => {
  name       => 'One register',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b1 ig2=0b000000 opcode:u=0b0111 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b0111',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 ldstruct-regcount=to-1reg mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=LD1_advsimd_mult cclass=LD1_advsimd_mult_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlse_R2_2v => {
  name       => 'Two registers',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b1 ig2=0b000000 opcode:u=0b1010 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b1010',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 ldstruct-regcount=to-2reg mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=LD1_advsimd_mult cclass=LD1_advsimd_mult_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlse_R3_3v => {
  name       => 'Three registers',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b1 ig2=0b000000 opcode:u=0b0110 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b0110',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 ldstruct-regcount=to-3reg mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=LD1_advsimd_mult cclass=LD1_advsimd_mult_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlse_R4_4v => {
  name       => 'Four registers',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b1 ig2=0b000000 opcode:u=0b0010 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b0010',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 ldstruct-regcount=to-4reg mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=LD1_advsimd_mult cclass=LD1_advsimd_mult_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlsep_I1_i1 => {
  name       => 'One register, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0b11111 opcode:u=0b0111 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b0111',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-multiple-labels=post-index-imm-to-1reg ldstruct-regcount=to-1reg mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD1_advsimd_mult cclass=LD1_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlsep_R1_r1 => {
  name       => 'One register, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b0111 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b0111',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-multiple-labels=post-index-reg-to-1reg ldstruct-regcount=to-1reg mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD1_advsimd_mult cclass=LD1_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlsep_I2_i2 => {
  name       => 'Two registers, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0b11111 opcode:u=0b1010 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b1010',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-multiple-labels=post-index-imm-to-2reg ldstruct-regcount=to-2reg mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD1_advsimd_mult cclass=LD1_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlsep_R2_r2 => {
  name       => 'Two registers, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b1010 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b1010',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-multiple-labels=post-index-reg-to-2reg ldstruct-regcount=to-2reg mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD1_advsimd_mult cclass=LD1_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlsep_I3_i3 => {
  name       => 'Three registers, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0b11111 opcode:u=0b0110 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b0110',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-multiple-labels=post-index-imm-to-3reg ldstruct-regcount=to-3reg mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD1_advsimd_mult cclass=LD1_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlsep_R3_r3 => {
  name       => 'Three registers, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b0110 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b0110',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-multiple-labels=post-index-reg-to-3reg ldstruct-regcount=to-3reg mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD1_advsimd_mult cclass=LD1_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlsep_I4_i4 => {
  name       => 'Four registers, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0b11111 opcode:u=0b0010 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b0010',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-multiple-labels=post-index-imm-to-4reg ldstruct-regcount=to-4reg mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD1_advsimd_mult cclass=LD1_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlsep_R4_r4 => {
  name       => 'Four registers, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b0010 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b0010',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-multiple-labels=post-index-reg-to-4reg ldstruct-regcount=to-4reg mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD1_advsimd_mult cclass=LD1_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlso_B1_1b => {
  name       => '8-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b0 ig2=0b00000 opcode:u=0b000 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b000',
  docvars    => 'as-structure-org=of-bytes as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD1_advsimd_sngl cclass=LD1_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlso_H1_1h => {
  name       => '16-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b0 ig2=0b00000 opcode:u=0b010 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b010 && size == 0bx0',
  docvars    => 'as-structure-org=of-halfwords as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD1_advsimd_sngl cclass=LD1_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlso_S1_1s => {
  name       => '32-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b0 ig2=0b00000 opcode:u=0b100 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b100 && size == 0b00',
  docvars    => 'as-structure-org=of-words as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD1_advsimd_sngl cclass=LD1_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlso_D1_1d => {
  name       => '64-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b0 ig2=0b00000 opcode:u=0b100 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b100 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-org=of-doublewords as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD1_advsimd_sngl cclass=LD1_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlsop_B1_i1b => {
  name       => '8-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0b11111 opcode:u=0b000 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b000',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-bytes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-bytes-post-index-imm mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD1_advsimd_sngl cclass=LD1_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlsop_BX1_r1b => {
  name       => '8-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b000 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b000',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-bytes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-bytes-post-index-reg mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD1_advsimd_sngl cclass=LD1_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlsop_H1_i1h => {
  name       => '16-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0b11111 opcode:u=0b010 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b010 && size == 0bx0',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-halfwords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-halfwords-post-index-imm mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD1_advsimd_sngl cclass=LD1_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlsop_HX1_r1h => {
  name       => '16-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b010 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b010 && size == 0bx0',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-halfwords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-halfwords-post-index-reg mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD1_advsimd_sngl cclass=LD1_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlsop_S1_i1s => {
  name       => '32-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0b11111 opcode:u=0b100 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b100 && size == 0b00',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-words as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-words-post-index-imm mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD1_advsimd_sngl cclass=LD1_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlsop_SX1_r1s => {
  name       => '32-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b100 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b100 && size == 0b00',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-words as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-words-post-index-reg mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD1_advsimd_sngl cclass=LD1_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlsop_D1_i1d => {
  name       => '64-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0b11111 opcode:u=0b100 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b100 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-doublewords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-doublewords-post-index-imm mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD1_advsimd_sngl cclass=LD1_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1_asisdlsop_DX1_r1d => {
  name       => '64-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b100 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b100 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-doublewords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-doublewords-post-index-reg mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD1_advsimd_sngl cclass=LD1_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1R_asisdlso_R1 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b0 ig2=0b00000 opcode:u=0b110 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-org=to-all-lanes as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD1R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD1R_advsimd cclass=LD1R_asisdlso_R1',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1R_asisdlsop_R1_i => {
  name       => 'Immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0b11111 opcode:u=0b110 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=to-all-lanes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=to-all-lanes-post-index-imm mnemonic=LD1R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD1R_advsimd cclass=LD1R_advsimd_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD1R_asisdlsop_RX1_r => {
  name       => 'Register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b110 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=to-all-lanes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=to-all-lanes-post-index-reg mnemonic=LD1R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD1R_advsimd cclass=LD1R_advsimd_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2_asisdlse_R2 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b1 ig2=0b000000 opcode:u=0b1000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=LD2_advsimd_mult cclass=LD2_asisdlse_R2',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2_asisdlsep_I2_i => {
  name       => 'Immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0b11111 opcode:u=0b1000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-multiple-labels=post-index-imm-to-2reg ldstruct-regcount=to-2reg mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD2_advsimd_mult cclass=LD2_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2_asisdlsep_R2_r => {
  name       => 'Register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b1000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-multiple-labels=post-index-reg-to-2reg ldstruct-regcount=to-2reg mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD2_advsimd_mult cclass=LD2_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2_asisdlso_B2_2b => {
  name       => '8-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b1 ig2=0b00000 opcode:u=0b000 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b000',
  docvars    => 'as-structure-org=of-bytes as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD2_advsimd_sngl cclass=LD2_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2_asisdlso_H2_2h => {
  name       => '16-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b1 ig2=0b00000 opcode:u=0b010 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b010 && size == 0bx0',
  docvars    => 'as-structure-org=of-halfwords as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD2_advsimd_sngl cclass=LD2_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2_asisdlso_S2_2s => {
  name       => '32-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b1 ig2=0b00000 opcode:u=0b100 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b100 && size == 0b00',
  docvars    => 'as-structure-org=of-words as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD2_advsimd_sngl cclass=LD2_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2_asisdlso_D2_2d => {
  name       => '64-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b1 ig2=0b00000 opcode:u=0b100 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b100 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-org=of-doublewords as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD2_advsimd_sngl cclass=LD2_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2_asisdlsop_B2_i2b => {
  name       => '8-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0b11111 opcode:u=0b000 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b000',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-bytes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-bytes-post-index-imm mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD2_advsimd_sngl cclass=LD2_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2_asisdlsop_BX2_r2b => {
  name       => '8-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b000 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b000',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-bytes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-bytes-post-index-reg mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD2_advsimd_sngl cclass=LD2_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2_asisdlsop_H2_i2h => {
  name       => '16-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0b11111 opcode:u=0b010 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b010 && size == 0bx0',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-halfwords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-halfwords-post-index-imm mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD2_advsimd_sngl cclass=LD2_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2_asisdlsop_HX2_r2h => {
  name       => '16-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b010 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b010 && size == 0bx0',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-halfwords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-halfwords-post-index-reg mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD2_advsimd_sngl cclass=LD2_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2_asisdlsop_S2_i2s => {
  name       => '32-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0b11111 opcode:u=0b100 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b100 && size == 0b00',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-words as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-words-post-index-imm mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD2_advsimd_sngl cclass=LD2_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2_asisdlsop_SX2_r2s => {
  name       => '32-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b100 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b100 && size == 0b00',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-words as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-words-post-index-reg mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD2_advsimd_sngl cclass=LD2_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2_asisdlsop_D2_i2d => {
  name       => '64-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0b11111 opcode:u=0b100 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b100 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-doublewords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-doublewords-post-index-imm mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD2_advsimd_sngl cclass=LD2_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2_asisdlsop_DX2_r2d => {
  name       => '64-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b100 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b100 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-doublewords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-doublewords-post-index-reg mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD2_advsimd_sngl cclass=LD2_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2R_asisdlso_R2 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b1 ig2=0b00000 opcode:u=0b110 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-org=to-all-lanes as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD2R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD2R_advsimd cclass=LD2R_asisdlso_R2',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2R_asisdlsop_R2_i => {
  name       => 'Immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0b11111 opcode:u=0b110 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=to-all-lanes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=to-all-lanes-post-index-imm mnemonic=LD2R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD2R_advsimd cclass=LD2R_advsimd_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD2R_asisdlsop_RX2_r => {
  name       => 'Register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b110 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=to-all-lanes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=to-all-lanes-post-index-reg mnemonic=LD2R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD2R_advsimd cclass=LD2R_advsimd_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3_asisdlse_R3 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b1 ig2=0b000000 opcode:u=0b0100 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=LD3_advsimd_mult cclass=LD3_asisdlse_R3',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3_asisdlsep_I3_i => {
  name       => 'Immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0b11111 opcode:u=0b0100 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-multiple-labels=post-index-imm-to-3reg ldstruct-regcount=to-3reg mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD3_advsimd_mult cclass=LD3_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3_asisdlsep_R3_r => {
  name       => 'Register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b0100 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-multiple-labels=post-index-reg-to-3reg ldstruct-regcount=to-3reg mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD3_advsimd_mult cclass=LD3_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3_asisdlso_B3_3b => {
  name       => '8-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b0 ig2=0b00000 opcode:u=0b001 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b001',
  docvars    => 'as-structure-org=of-bytes as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD3_advsimd_sngl cclass=LD3_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3_asisdlso_H3_3h => {
  name       => '16-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b0 ig2=0b00000 opcode:u=0b011 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b011 && size == 0bx0',
  docvars    => 'as-structure-org=of-halfwords as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD3_advsimd_sngl cclass=LD3_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3_asisdlso_S3_3s => {
  name       => '32-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b0 ig2=0b00000 opcode:u=0b101 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b101 && size == 0b00',
  docvars    => 'as-structure-org=of-words as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD3_advsimd_sngl cclass=LD3_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3_asisdlso_D3_3d => {
  name       => '64-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b0 ig2=0b00000 opcode:u=0b101 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b101 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-org=of-doublewords as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD3_advsimd_sngl cclass=LD3_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3_asisdlsop_B3_i3b => {
  name       => '8-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0b11111 opcode:u=0b001 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b001',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-bytes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-bytes-post-index-imm mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD3_advsimd_sngl cclass=LD3_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3_asisdlsop_BX3_r3b => {
  name       => '8-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b001 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b001',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-bytes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-bytes-post-index-reg mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD3_advsimd_sngl cclass=LD3_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3_asisdlsop_H3_i3h => {
  name       => '16-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0b11111 opcode:u=0b011 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b011 && size == 0bx0',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-halfwords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-halfwords-post-index-imm mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD3_advsimd_sngl cclass=LD3_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3_asisdlsop_HX3_r3h => {
  name       => '16-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b011 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b011 && size == 0bx0',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-halfwords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-halfwords-post-index-reg mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD3_advsimd_sngl cclass=LD3_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3_asisdlsop_S3_i3s => {
  name       => '32-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0b11111 opcode:u=0b101 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b101 && size == 0b00',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-words as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-words-post-index-imm mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD3_advsimd_sngl cclass=LD3_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3_asisdlsop_SX3_r3s => {
  name       => '32-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b101 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b101 && size == 0b00',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-words as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-words-post-index-reg mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD3_advsimd_sngl cclass=LD3_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3_asisdlsop_D3_i3d => {
  name       => '64-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0b11111 opcode:u=0b101 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b101 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-doublewords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-doublewords-post-index-imm mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD3_advsimd_sngl cclass=LD3_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3_asisdlsop_DX3_r3d => {
  name       => '64-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b101 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b101 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-doublewords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-doublewords-post-index-reg mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD3_advsimd_sngl cclass=LD3_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3R_asisdlso_R3 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b0 ig2=0b00000 opcode:u=0b111 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-org=to-all-lanes as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD3R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD3R_advsimd cclass=LD3R_asisdlso_R3',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3R_asisdlsop_R3_i => {
  name       => 'Immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0b11111 opcode:u=0b111 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=to-all-lanes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=to-all-lanes-post-index-imm mnemonic=LD3R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD3R_advsimd cclass=LD3R_advsimd_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD3R_asisdlsop_RX3_r => {
  name       => 'Register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b111 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=to-all-lanes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=to-all-lanes-post-index-reg mnemonic=LD3R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD3R_advsimd cclass=LD3R_advsimd_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4_asisdlse_R4 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b1 ig2=0b000000 opcode:u=0b0000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=LD4_advsimd_mult cclass=LD4_asisdlse_R4',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4_asisdlsep_I4_i => {
  name       => 'Immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0b11111 opcode:u=0b0000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-multiple-labels=post-index-imm-to-4reg ldstruct-regcount=to-4reg mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD4_advsimd_mult cclass=LD4_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4_asisdlsep_R4_r => {
  name       => 'Register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b0000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-multiple-labels=post-index-reg-to-4reg ldstruct-regcount=to-4reg mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD4_advsimd_mult cclass=LD4_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4_asisdlso_B4_4b => {
  name       => '8-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b1 ig2=0b00000 opcode:u=0b001 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b001',
  docvars    => 'as-structure-org=of-bytes as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD4_advsimd_sngl cclass=LD4_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4_asisdlso_H4_4h => {
  name       => '16-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b1 ig2=0b00000 opcode:u=0b011 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b011 && size == 0bx0',
  docvars    => 'as-structure-org=of-halfwords as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD4_advsimd_sngl cclass=LD4_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4_asisdlso_S4_4s => {
  name       => '32-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b1 ig2=0b00000 opcode:u=0b101 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b101 && size == 0b00',
  docvars    => 'as-structure-org=of-words as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD4_advsimd_sngl cclass=LD4_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4_asisdlso_D4_4d => {
  name       => '64-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b1 ig2=0b00000 opcode:u=0b101 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b101 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-org=of-doublewords as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD4_advsimd_sngl cclass=LD4_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4_asisdlsop_B4_i4b => {
  name       => '8-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0b11111 opcode:u=0b001 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b001',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-bytes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-bytes-post-index-imm mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD4_advsimd_sngl cclass=LD4_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4_asisdlsop_BX4_r4b => {
  name       => '8-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b001 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b001',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-bytes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-bytes-post-index-reg mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD4_advsimd_sngl cclass=LD4_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4_asisdlsop_H4_i4h => {
  name       => '16-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0b11111 opcode:u=0b011 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b011 && size == 0bx0',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-halfwords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-halfwords-post-index-imm mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD4_advsimd_sngl cclass=LD4_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4_asisdlsop_HX4_r4h => {
  name       => '16-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b011 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b011 && size == 0bx0',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-halfwords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-halfwords-post-index-reg mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD4_advsimd_sngl cclass=LD4_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4_asisdlsop_S4_i4s => {
  name       => '32-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0b11111 opcode:u=0b101 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b101 && size == 0b00',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-words as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-words-post-index-imm mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD4_advsimd_sngl cclass=LD4_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4_asisdlsop_SX4_r4s => {
  name       => '32-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b101 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b101 && size == 0b00',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-words as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-words-post-index-reg mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD4_advsimd_sngl cclass=LD4_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4_asisdlsop_D4_i4d => {
  name       => '64-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0b11111 opcode:u=0b101 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b101 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-doublewords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-doublewords-post-index-imm mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD4_advsimd_sngl cclass=LD4_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4_asisdlsop_DX4_r4d => {
  name       => '64-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b101 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b101 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-doublewords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-doublewords-post-index-reg mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD4_advsimd_sngl cclass=LD4_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4R_asisdlso_R4 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b1 ig2=0b00000 opcode:u=0b111 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-org=to-all-lanes as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD4R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD4R_advsimd cclass=LD4R_asisdlso_R4',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4R_asisdlsop_R4_i => {
  name       => 'Immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0b11111 opcode:u=0b111 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=to-all-lanes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=to-all-lanes-post-index-imm mnemonic=LD4R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD4R_advsimd cclass=LD4R_advsimd_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LD4R_asisdlsop_RX4_r => {
  name       => 'Register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b111 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=to-all-lanes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=to-all-lanes-post-index-reg mnemonic=LD4R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD4R_advsimd cclass=LD4R_advsimd_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDNP_S_ldstnapair_offs => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b101 V=0b1 ig1=0b000 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-words atomic-ops=LDNP-pair-words instr-class=fpsimd isa=A64 mnemonic=LDNP offset-type=off7s_s reg-type=pair-words',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstnapair_offs page=LDNP_fpsimd cclass=LDNP_fpsimd_signed_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|HINT|LOAD',
};

ENCODING LDNP_D_ldstnapair_offs => {
  name       => '64-bit',
  diagram    => 'opc:u=0b01 ig0=0b101 V=0b1 ig1=0b000 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b01',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-doublewords atomic-ops=LDNP-pair-doublewords instr-class=fpsimd isa=A64 mnemonic=LDNP offset-type=off7s_s reg-type=pair-doublewords',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstnapair_offs page=LDNP_fpsimd cclass=LDNP_fpsimd_signed_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|HINT|LOAD',
};

ENCODING LDNP_Q_ldstnapair_offs => {
  name       => '128-bit',
  diagram    => 'opc:u=0b10 ig0=0b101 V=0b1 ig1=0b000 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-quadwords atomic-ops=LDNP-pair-quadwords instr-class=fpsimd isa=A64 mnemonic=LDNP offset-type=off7s_s reg-type=pair-quadwords',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstnapair_offs page=LDNP_fpsimd cclass=LDNP_fpsimd_signed_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|HINT|LOAD',
};

ENCODING LDP_S_ldstpair_post => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b101 V=0b1 ig1=0b001 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-pair-words atomic-ops=LDP-pair-words instr-class=fpsimd isa=A64 mnemonic=LDP offset-type=off7s_s reg-type=pair-words',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_post page=LDP_fpsimd cclass=LDP_fpsimd_post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDP_D_ldstpair_post => {
  name       => '64-bit',
  diagram    => 'opc:u=0b01 ig0=0b101 V=0b1 ig1=0b001 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b01',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-pair-doublewords atomic-ops=LDP-pair-doublewords instr-class=fpsimd isa=A64 mnemonic=LDP offset-type=off7s_s reg-type=pair-doublewords',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_post page=LDP_fpsimd cclass=LDP_fpsimd_post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDP_Q_ldstpair_post => {
  name       => '128-bit',
  diagram    => 'opc:u=0b10 ig0=0b101 V=0b1 ig1=0b001 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-pair-quadwords atomic-ops=LDP-pair-quadwords instr-class=fpsimd isa=A64 mnemonic=LDP offset-type=off7s_s reg-type=pair-quadwords',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_post page=LDP_fpsimd cclass=LDP_fpsimd_post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDP_S_ldstpair_pre => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b101 V=0b1 ig1=0b011 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-pair-words atomic-ops=LDP-pair-words instr-class=fpsimd isa=A64 mnemonic=LDP offset-type=off7s_s reg-type=pair-words',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_pre page=LDP_fpsimd cclass=LDP_fpsimd_pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDP_D_ldstpair_pre => {
  name       => '64-bit',
  diagram    => 'opc:u=0b01 ig0=0b101 V=0b1 ig1=0b011 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b01',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-pair-doublewords atomic-ops=LDP-pair-doublewords instr-class=fpsimd isa=A64 mnemonic=LDP offset-type=off7s_s reg-type=pair-doublewords',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_pre page=LDP_fpsimd cclass=LDP_fpsimd_pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDP_Q_ldstpair_pre => {
  name       => '128-bit',
  diagram    => 'opc:u=0b10 ig0=0b101 V=0b1 ig1=0b011 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-pair-quadwords atomic-ops=LDP-pair-quadwords instr-class=fpsimd isa=A64 mnemonic=LDP offset-type=off7s_s reg-type=pair-quadwords',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_pre page=LDP_fpsimd cclass=LDP_fpsimd_pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDP_S_ldstpair_off => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b101 V=0b1 ig1=0b010 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-words atomic-ops=LDP-pair-words instr-class=fpsimd isa=A64 mnemonic=LDP offset-type=off7s_s reg-type=pair-words',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_off page=LDP_fpsimd cclass=LDP_fpsimd_signed_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDP_D_ldstpair_off => {
  name       => '64-bit',
  diagram    => 'opc:u=0b01 ig0=0b101 V=0b1 ig1=0b010 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b01',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-doublewords atomic-ops=LDP-pair-doublewords instr-class=fpsimd isa=A64 mnemonic=LDP offset-type=off7s_s reg-type=pair-doublewords',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_off page=LDP_fpsimd cclass=LDP_fpsimd_signed_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDP_Q_ldstpair_off => {
  name       => '128-bit',
  diagram    => 'opc:u=0b10 ig0=0b101 V=0b1 ig1=0b010 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-quadwords atomic-ops=LDP-pair-quadwords instr-class=fpsimd isa=A64 mnemonic=LDP offset-type=off7s_s reg-type=pair-quadwords',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_off page=LDP_fpsimd cclass=LDP_fpsimd_signed_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_B_ldst_immpost => {
  name       => '8-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b01',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-8-fsreg atomic-ops=LDR-8-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off9s_u reg-type=8-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpost page=LDR_imm_fpsimd cclass=LDR_imm_fpsimd_post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_H_ldst_immpost => {
  name       => '16-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b01 && opc == 0b01',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-16-fsreg atomic-ops=LDR-16-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off9s_u reg-type=16-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpost page=LDR_imm_fpsimd cclass=LDR_imm_fpsimd_post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_S_ldst_immpost => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && opc == 0b01',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-32-fsreg atomic-ops=LDR-32-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off9s_u reg-type=32-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpost page=LDR_imm_fpsimd cclass=LDR_imm_fpsimd_post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_D_ldst_immpost => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && opc == 0b01',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-64-fsreg atomic-ops=LDR-64-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off9s_u reg-type=64-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpost page=LDR_imm_fpsimd cclass=LDR_imm_fpsimd_post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_Q_ldst_immpost => {
  name       => '128-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b11 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b11',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-128-fsreg atomic-ops=LDR-128-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off9s_u reg-type=128-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpost page=LDR_imm_fpsimd cclass=LDR_imm_fpsimd_post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_B_ldst_immpre => {
  name       => '8-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b01',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-8-fsreg atomic-ops=LDR-8-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off9s_u reg-type=8-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpre page=LDR_imm_fpsimd cclass=LDR_imm_fpsimd_pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_H_ldst_immpre => {
  name       => '16-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b01 && opc == 0b01',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-16-fsreg atomic-ops=LDR-16-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off9s_u reg-type=16-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpre page=LDR_imm_fpsimd cclass=LDR_imm_fpsimd_pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_S_ldst_immpre => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && opc == 0b01',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-32-fsreg atomic-ops=LDR-32-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off9s_u reg-type=32-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpre page=LDR_imm_fpsimd cclass=LDR_imm_fpsimd_pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_D_ldst_immpre => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && opc == 0b01',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-64-fsreg atomic-ops=LDR-64-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off9s_u reg-type=64-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpre page=LDR_imm_fpsimd cclass=LDR_imm_fpsimd_pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_Q_ldst_immpre => {
  name       => '128-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b11 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b11',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-128-fsreg atomic-ops=LDR-128-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off9s_u reg-type=128-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpre page=LDR_imm_fpsimd cclass=LDR_imm_fpsimd_pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_B_ldst_pos => {
  name       => '8-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b01 opc:u=0b01 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b01',
  docvars    => 'address-form=unsigned-scaled-offset address-form-reg-type=unsigned-scaled-offset-8-fsreg atomic-ops=LDR-8-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off12u_s reg-type=8-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_pos page=LDR_imm_fpsimd cclass=LDR_imm_fpsimd_unsigned_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_H_ldst_pos => {
  name       => '16-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b1 ig1=0b01 opc:u=0b01 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b01 && opc == 0b01',
  docvars    => 'address-form=unsigned-scaled-offset address-form-reg-type=unsigned-scaled-offset-16-fsreg atomic-ops=LDR-16-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off12u_s reg-type=16-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_pos page=LDR_imm_fpsimd cclass=LDR_imm_fpsimd_unsigned_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_S_ldst_pos => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b1 ig1=0b01 opc:u=0b01 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && opc == 0b01',
  docvars    => 'address-form=unsigned-scaled-offset address-form-reg-type=unsigned-scaled-offset-32-fsreg atomic-ops=LDR-32-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off12u_s reg-type=32-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_pos page=LDR_imm_fpsimd cclass=LDR_imm_fpsimd_unsigned_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_D_ldst_pos => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b1 ig1=0b01 opc:u=0b01 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && opc == 0b01',
  docvars    => 'address-form=unsigned-scaled-offset address-form-reg-type=unsigned-scaled-offset-64-fsreg atomic-ops=LDR-64-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off12u_s reg-type=64-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_pos page=LDR_imm_fpsimd cclass=LDR_imm_fpsimd_unsigned_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_Q_ldst_pos => {
  name       => '128-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b01 opc:u=0b11 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b11',
  docvars    => 'address-form=unsigned-scaled-offset address-form-reg-type=unsigned-scaled-offset-128-fsreg atomic-ops=LDR-128-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off12u_s reg-type=128-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_pos page=LDR_imm_fpsimd cclass=LDR_imm_fpsimd_unsigned_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_S_loadlit => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b011 V=0b1 ig1=0b00 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=literal address-form-reg-type=literal-32-fsreg atomic-ops=LDR-32-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off19s reg-type=32-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=loadlit page=LDR_lit_fpsimd cclass=LDR_lit_fpsimd_literal',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|MISC|LOAD',
};

ENCODING LDR_D_loadlit => {
  name       => '64-bit',
  diagram    => 'opc:u=0b01 ig0=0b011 V=0b1 ig1=0b00 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b01',
  docvars    => 'address-form=literal address-form-reg-type=literal-64-fsreg atomic-ops=LDR-64-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off19s reg-type=64-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=loadlit page=LDR_lit_fpsimd cclass=LDR_lit_fpsimd_literal',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|MISC|LOAD',
};

ENCODING LDR_Q_loadlit => {
  name       => '128-bit',
  diagram    => 'opc:u=0b10 ig0=0b011 V=0b1 ig1=0b00 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=literal address-form-reg-type=literal-128-fsreg atomic-ops=LDR-128-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off19s reg-type=128-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=loadlit page=LDR_lit_fpsimd cclass=LDR_lit_fpsimd_literal',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|MISC|LOAD',
};

ENCODING LDR_B_ldst_regoff => {
  name       => '8-fsreg,LDR-8-fsreg',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b01 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'option!=0b011',
  bitdiffs   => 'size == 0b00 && opc == 0b01 && option != 0b011',
  docvars    => 'atomic-ops=LDR-8-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off-reg reg-type=8-fsreg reg-type-and-use=8-fsreg-ext-reg reguse=ext-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_regoff page=LDR_reg_fpsimd cclass=LDR_reg_fpsimd_fpsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_BL_ldst_regoff => {
  name       => '8-fsreg,LDR-8-fsreg',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b01 ig2=0b1 Rm:u=0bxxxxx option:u=0b011 S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b01 && option == 0b011',
  docvars    => 'atomic-ops=LDR-8-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off-reg reg-type=8-fsreg reg-type-and-use=8-fsreg-shifted-reg reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_regoff page=LDR_reg_fpsimd cclass=LDR_reg_fpsimd_fpsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_H_ldst_regoff => {
  name       => '16-fsreg,LDR-16-fsreg',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b01 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b01 && opc == 0b01',
  docvars    => 'atomic-ops=LDR-16-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off-reg reg-type=16-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_regoff page=LDR_reg_fpsimd cclass=LDR_reg_fpsimd_fpsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_S_ldst_regoff => {
  name       => '32-fsreg,LDR-32-fsreg',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b01 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && opc == 0b01',
  docvars    => 'atomic-ops=LDR-32-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off-reg reg-type=32-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_regoff page=LDR_reg_fpsimd cclass=LDR_reg_fpsimd_fpsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_D_ldst_regoff => {
  name       => '64-fsreg,LDR-64-fsreg',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b01 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && opc == 0b01',
  docvars    => 'atomic-ops=LDR-64-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off-reg reg-type=64-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_regoff page=LDR_reg_fpsimd cclass=LDR_reg_fpsimd_fpsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDR_Q_ldst_regoff => {
  name       => '128-fsreg,LDR-128-fsreg',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b11 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b11',
  docvars    => 'atomic-ops=LDR-128-fsreg instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off-reg reg-type=128-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_regoff page=LDR_reg_fpsimd cclass=LDR_reg_fpsimd_fpsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDUR_B_ldst_unscaled => {
  name       => '8-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b01',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-8-fsreg atomic-ops=LDUR-8-fsreg instr-class=fpsimd isa=A64 mnemonic=LDUR offset-type=off9s_u reg-type=8-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDUR_fpsimd cclass=LDUR_fpsimd_base_plus_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDUR_H_ldst_unscaled => {
  name       => '16-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b01 && opc == 0b01',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-16-fsreg atomic-ops=LDUR-16-fsreg instr-class=fpsimd isa=A64 mnemonic=LDUR offset-type=off9s_u reg-type=16-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDUR_fpsimd cclass=LDUR_fpsimd_base_plus_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDUR_S_ldst_unscaled => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && opc == 0b01',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-32-fsreg atomic-ops=LDUR-32-fsreg instr-class=fpsimd isa=A64 mnemonic=LDUR offset-type=off9s_u reg-type=32-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDUR_fpsimd cclass=LDUR_fpsimd_base_plus_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDUR_D_ldst_unscaled => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && opc == 0b01',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-64-fsreg atomic-ops=LDUR-64-fsreg instr-class=fpsimd isa=A64 mnemonic=LDUR offset-type=off9s_u reg-type=64-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDUR_fpsimd cclass=LDUR_fpsimd_base_plus_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING LDUR_Q_ldst_unscaled => {
  name       => '128-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b11 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b11',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-128-fsreg atomic-ops=LDUR-128-fsreg instr-class=fpsimd isa=A64 mnemonic=LDUR offset-type=off9s_u reg-type=128-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDUR_fpsimd cclass=LDUR_fpsimd_base_plus_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|LOAD',
};

ENCODING MLA_asimdelem_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b0 ig3=0b00 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=MLA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=MLA_advsimd_elt cclass=MLA_asimdelem_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING MLA_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10010 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=MLA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=MLA_advsimd_vec cclass=MLA_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING MLS_asimdelem_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b1 ig3=0b00 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=MLS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=MLS_advsimd_elt cclass=MLS_asimdelem_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING MLS_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10010 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=MLS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=MLS_advsimd_vec cclass=MLS_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING MOVI_asimdimm_N_b => {
  name       => '8-bit',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0b0 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0b1110 o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  bitdiffs   => 'op == 0b0 && cmode == 0b1110',
  docvars    => 'asimdimm-datatype=per-byte asimdimm-immtype=immediate asimdimm-mask=no-byte-mask asimdimm-type=per-byte-immediate instr-class=advsimd isa=A64 mnemonic=MOVI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=MOVI_advsimd cclass=MOVI_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING MOVI_asimdimm_L_hl => {
  name       => '16-bit shifted immediate',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0b0 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0b10x0 o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  bitdiffs   => 'op == 0b0 && cmode == 0b10x0',
  docvars    => 'asimdimm-datatype=per-halfword asimdimm-immtype=shifted-immediate asimdimm-mask=no-byte-mask asimdimm-type=per-halfword-shifted-immediate instr-class=advsimd isa=A64 mnemonic=MOVI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=MOVI_advsimd cclass=MOVI_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING MOVI_asimdimm_L_sl => {
  name       => '32-bit shifted immediate',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0b0 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0b0xx0 o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  bitdiffs   => 'op == 0b0 && cmode == 0b0xx0',
  docvars    => 'asimdimm-datatype=per-word asimdimm-immtype=shifted-immediate asimdimm-mask=no-byte-mask asimdimm-type=per-word-shifted-immediate instr-class=advsimd isa=A64 mnemonic=MOVI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=MOVI_advsimd cclass=MOVI_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING MOVI_asimdimm_M_sm => {
  name       => '32-bit shifting ones',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0b0 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0b110x o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  bitdiffs   => 'op == 0b0 && cmode == 0b110x',
  docvars    => 'asimdimm-datatype=per-word asimdimm-immtype=masked-immediate asimdimm-mask=no-byte-mask asimdimm-type=per-word-masked-immediate instr-class=advsimd isa=A64 mnemonic=MOVI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=MOVI_advsimd cclass=MOVI_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING MOVI_asimdimm_D_ds => {
  name       => '64-bit scalar',
  diagram    => 'ig0=0b0 Q:u=0b0 op:u=0b1 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0b1110 o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  bitdiffs   => 'Q == 0b0 && op == 0b1 && cmode == 0b1110',
  docvars    => 'asimdimm-datatype=doubleword asimdimm-immtype=immediate asimdimm-mask=byte-mask asimdimm-type=doubleword-immediate instr-class=advsimd isa=A64 mnemonic=MOVI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=MOVI_advsimd cclass=MOVI_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING MOVI_asimdimm_D2_d => {
  name       => '64-bit vector',
  diagram    => 'ig0=0b0 Q:u=0b1 op:u=0b1 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0b1110 o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  bitdiffs   => 'Q == 0b1 && op == 0b1 && cmode == 0b1110',
  docvars    => 'asimdimm-datatype=per-doubleword asimdimm-immtype=immediate asimdimm-mask=byte-mask asimdimm-type=per-doubleword-immediate instr-class=advsimd isa=A64 mnemonic=MOVI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=MOVI_advsimd cclass=MOVI_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING MUL_asimdelem_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1000 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=MUL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=MUL_advsimd_elt cclass=MUL_asimdelem_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING MUL_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=MUL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=MUL_advsimd_vec cclass=MUL_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING MVNI_asimdimm_L_hl => {
  name       => '16-bit shifted immediate',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0b1 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0b10x0 o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  bitdiffs   => 'cmode == 0b10x0',
  docvars    => 'asimdimm-datatype=per-halfword asimdimm-immtype=shifted-immediate asimdimm-type=per-halfword-shifted-immediate instr-class=advsimd isa=A64 mnemonic=MVNI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=MVNI_advsimd cclass=MVNI_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING MVNI_asimdimm_L_sl => {
  name       => '32-bit shifted immediate',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0b1 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0b0xx0 o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  bitdiffs   => 'cmode == 0b0xx0',
  docvars    => 'asimdimm-datatype=per-word asimdimm-immtype=shifted-immediate asimdimm-type=per-word-shifted-immediate instr-class=advsimd isa=A64 mnemonic=MVNI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=MVNI_advsimd cclass=MVNI_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING MVNI_asimdimm_M_sm => {
  name       => '32-bit shifting ones',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0b1 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0b110x o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  bitdiffs   => 'cmode == 0b110x',
  docvars    => 'asimdimm-datatype=per-word asimdimm-immtype=masked-immediate asimdimm-mask=no-byte-mask asimdimm-type=per-word-masked-immediate instr-class=advsimd isa=A64 mnemonic=MVNI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=MVNI_advsimd cclass=MVNI_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING NEG_asisdmisc_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b01011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=NEG',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=NEG_advsimd cclass=NEG_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING NEG_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b01011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=NEG',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=NEG_advsimd cclass=NEG_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING NOT_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size=0b00 ig2=0b10000 opcode=0b00101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=NOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=NOT_advsimd cclass=NOT_advsimd_NOT_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING ORN_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0b11 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=ORN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=ORN_advsimd cclass=ORN_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING ORR_asimdimm_L_hl => {
  name       => '16-bit',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0b0 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0b10x1 o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  bitdiffs   => 'cmode == 0b10x1',
  docvars    => 'asimdimm-datatype=per-halfword asimdimm-immtype=shifted-immediate asimdimm-type=per-halfword-shifted-immediate instr-class=advsimd isa=A64 mnemonic=ORR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=ORR_advsimd_imm cclass=ORR_advsimd_imm_shifted_immediate',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING ORR_asimdimm_L_sl => {
  name       => '32-bit',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0b0 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0b0xx1 o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  bitdiffs   => 'cmode == 0b0xx1',
  docvars    => 'asimdimm-datatype=per-word asimdimm-immtype=shifted-immediate asimdimm-type=per-word-shifted-immediate instr-class=advsimd isa=A64 mnemonic=ORR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=ORR_advsimd_imm cclass=ORR_advsimd_imm_shifted_immediate',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING ORR_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0b10 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=ORR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=ORR_advsimd_reg cclass=ORR_advsimd_reg_ORR_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING PMUL_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=PMUL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=PMUL_advsimd cclass=PMUL_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|POLYNOMIAL',
};

ENCODING PMULL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b1110 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=PMULL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=PMULL_advsimd cclass=PMULL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|POLYNOMIAL',
};

ENCODING RADDHN_asimddiff_N => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 o1:u=0b0 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=RADDHN',
  docvars2   => 'rounding=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=RADDHN_advsimd cclass=RADDHN_asimddiff_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING',
};

ENCODING RAX1_VVV2_cryptosha512_3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110011 Rm:u=0bxxxxx ig1=0b1 O=0b0 ig2=0b00 opcode=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=RAX1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_3 page=RAX1_advsimd cclass=RAX1_VVV2_cryptosha512_3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING RBIT_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size=0b01 ig2=0b10000 opcode=0b00101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=RBIT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=RBIT_advsimd cclass=RBIT_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING REV16_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b0000 o0:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=REV16',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=REV16_advsimd cclass=REV16_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING REV32_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b0000 o0:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=REV32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=REV32_advsimd cclass=REV32_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING REV64_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b0000 o0:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=REV64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=REV64_advsimd cclass=REV64_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING RSHRN_asimdshf_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1000 op:u=0b1 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=RSHRN',
  docvars2   => 'rounding=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=RSHRN_advsimd cclass=RSHRN_asimdshf_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING',
};

ENCODING RSUBHN_asimddiff_N => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 o1:u=0b1 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=RSUBHN',
  docvars2   => 'rounding=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=RSUBHN_advsimd cclass=RSUBHN_asimddiff_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING',
};

ENCODING SABA_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0111 ac:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SABA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SABA_advsimd cclass=SABA_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|SIGNED',
};

ENCODING SABAL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b0 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SABAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SABAL_advsimd cclass=SABAL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|SIGNED',
};

ENCODING SABD_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0111 ac:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SABD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SABD_advsimd cclass=SABD_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|SIGNED',
};

ENCODING SABDL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b1 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SABDL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SABDL_advsimd cclass=SABDL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|SIGNED',
};

ENCODING SADALP_asimdmisc_P => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b00 op:u=0b1 ig4=0b10 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SADALP',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SADALP_advsimd cclass=SADALP_asimdmisc_P',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SIGNED',
};

ENCODING SADDL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b00 o1:u=0b0 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SADDL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SADDL_advsimd cclass=SADDL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SIGNED',
};

ENCODING SADDLP_asimdmisc_P => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b00 op:u=0b0 ig4=0b10 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SADDLP',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SADDLP_advsimd cclass=SADDLP_asimdmisc_P',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SIGNED',
};

ENCODING SADDLV_asimdall_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b11000 opcode=0b00011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SADDLV',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=SADDLV_advsimd cclass=SADDLV_asimdall_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SIGNED',
};

ENCODING SADDW_asimddiff_W => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b00 o1:u=0b0 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SADDW',
  docvars2   => 'shape=wide',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SADDW_advsimd cclass=SADDW_asimddiff_W',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SIGNED',
};

ENCODING SCVTF_asisdshf_C => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx opcode=0b11100 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SCVTF_advsimd_fix cclass=SCVTF_asisdshf_C',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING SCVTF_asimdshf_C => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b11100 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SCVTF_advsimd_fix cclass=SCVTF_asimdshf_C',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING SCVTF_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 a=0b0 ig2=0b111100 opcode=0b11101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=SCVTF_advsimd_int cclass=SCVTF_asisdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING SCVTF_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b11101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=SCVTF_advsimd_int cclass=SCVTF_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING SCVTF_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a=0b0 ig2=0b111100 opcode=0b11101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=SCVTF_advsimd_int cclass=SCVTF_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING SCVTF_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b11101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SCVTF_advsimd_int cclass=SCVTF_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|SIGNED',
};

ENCODING SCVTF_H32_float2fix => {
  name       => '32-bit to half-precision',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b0 rmode:u=0b00 opcode:u=0b010 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11',
  docvars    => 'convert-type=fix32-to-half instr-class=float isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=SCVTF_float_fix cclass=SCVTF_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING SCVTF_S32_float2fix => {
  name       => '32-bit to single-precision',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b0 rmode:u=0b00 opcode:u=0b010 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00',
  docvars    => 'convert-type=fix32-to-single instr-class=float isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=SCVTF_float_fix cclass=SCVTF_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING SCVTF_D32_float2fix => {
  name       => '32-bit to double-precision',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b0 rmode:u=0b00 opcode:u=0b010 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b01',
  docvars    => 'convert-type=fix32-to-double instr-class=float isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=SCVTF_float_fix cclass=SCVTF_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING SCVTF_H64_float2fix => {
  name       => '64-bit to half-precision',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b0 rmode:u=0b00 opcode:u=0b010 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11',
  docvars    => 'convert-type=fix64-to-half instr-class=float isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=SCVTF_float_fix cclass=SCVTF_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING SCVTF_S64_float2fix => {
  name       => '64-bit to single-precision',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b0 rmode:u=0b00 opcode:u=0b010 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b00',
  docvars    => 'convert-type=fix64-to-single instr-class=float isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=SCVTF_float_fix cclass=SCVTF_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING SCVTF_D64_float2fix => {
  name       => '64-bit to double-precision',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b0 rmode:u=0b00 opcode:u=0b010 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01',
  docvars    => 'convert-type=fix64-to-double instr-class=float isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=SCVTF_float_fix cclass=SCVTF_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING SCVTF_H32_float2int => {
  name       => '32-bit to half-precision',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b00 opcode:u=0b010 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11',
  docvars    => 'convert-type=32-to-half instr-class=float isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=SCVTF_float_int cclass=SCVTF_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING SCVTF_S32_float2int => {
  name       => '32-bit to single-precision',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b00 opcode:u=0b010 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00',
  docvars    => 'convert-type=32-to-single instr-class=float isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=SCVTF_float_int cclass=SCVTF_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING SCVTF_D32_float2int => {
  name       => '32-bit to double-precision',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b00 opcode:u=0b010 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b01',
  docvars    => 'convert-type=32-to-double instr-class=float isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=SCVTF_float_int cclass=SCVTF_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING SCVTF_H64_float2int => {
  name       => '64-bit to half-precision',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b00 opcode:u=0b010 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11',
  docvars    => 'convert-type=64-to-half instr-class=float isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=SCVTF_float_int cclass=SCVTF_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING SCVTF_S64_float2int => {
  name       => '64-bit to single-precision',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b00 opcode:u=0b010 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b00',
  docvars    => 'convert-type=64-to-single instr-class=float isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=SCVTF_float_int cclass=SCVTF_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING SCVTF_D64_float2int => {
  name       => '64-bit to double-precision',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b00 opcode:u=0b010 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01',
  docvars    => 'convert-type=64-to-double instr-class=float isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=SCVTF_float_int cclass=SCVTF_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|SIGNED',
};

ENCODING SDOT_asimdelem_D => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1110 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SDOT_advsimd_elt cclass=SDOT_asimdelem_D',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|SIGNED',
};

ENCODING SDOT_asimdsame2_D => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 opcode=0b0010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=SDOT_advsimd_vec cclass=SDOT_asimdsame2_D',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|SIGNED',
};

ENCODING SHA1C_QSV_cryptosha3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b0 Rm:u=0bxxxxx ig2=0b0 opcode=0b000 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1C',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha3 page=SHA1C_advsimd cclass=SHA1C_QSV_cryptosha3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA1H_SS_cryptosha2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b10100 opcode=0b00000 ig2=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1H',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha2 page=SHA1H_advsimd cclass=SHA1H_SS_cryptosha2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA1M_QSV_cryptosha3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b0 Rm:u=0bxxxxx ig2=0b0 opcode=0b010 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1M',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha3 page=SHA1M_advsimd cclass=SHA1M_QSV_cryptosha3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA1P_QSV_cryptosha3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b0 Rm:u=0bxxxxx ig2=0b0 opcode=0b001 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1P',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha3 page=SHA1P_advsimd cclass=SHA1P_QSV_cryptosha3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA1SU0_VVV_cryptosha3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b0 Rm:u=0bxxxxx ig2=0b0 opcode=0b011 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1SU0',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha3 page=SHA1SU0_advsimd cclass=SHA1SU0_VVV_cryptosha3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA1SU1_VV_cryptosha2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b10100 opcode=0b00001 ig2=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1SU1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha2 page=SHA1SU1_advsimd cclass=SHA1SU1_VV_cryptosha2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA256H2_QQV_cryptosha3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b0 Rm:u=0bxxxxx ig2=0b0 ig3=0b10 P:u=0b1 ig4=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA256H2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha3 page=SHA256H2_advsimd cclass=SHA256H2_QQV_cryptosha3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA256H_QQV_cryptosha3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b0 Rm:u=0bxxxxx ig2=0b0 ig3=0b10 P:u=0b0 ig4=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA256H',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha3 page=SHA256H_advsimd cclass=SHA256H_QQV_cryptosha3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA256SU0_VV_cryptosha2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b10100 opcode=0b00010 ig2=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA256SU0',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha2 page=SHA256SU0_advsimd cclass=SHA256SU0_VV_cryptosha2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA256SU1_VVV_cryptosha3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b0 Rm:u=0bxxxxx ig2=0b0 opcode=0b110 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA256SU1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha3 page=SHA256SU1_advsimd cclass=SHA256SU1_VVV_cryptosha3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA512H2_QQV_cryptosha512_3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110011 Rm:u=0bxxxxx ig1=0b1 O=0b0 ig2=0b00 opcode=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA512H2',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_3 page=SHA512H2_advsimd cclass=SHA512H2_QQV_cryptosha512_3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA512H_QQV_cryptosha512_3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110011 Rm:u=0bxxxxx ig1=0b1 O=0b0 ig2=0b00 opcode=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA512H',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_3 page=SHA512H_advsimd cclass=SHA512H_QQV_cryptosha512_3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA512SU0_VV2_cryptosha512_2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110110000001000 opcode=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA512SU0',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_2 page=SHA512SU0_advsimd cclass=SHA512SU0_VV2_cryptosha512_2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA512SU1_VVV2_cryptosha512_3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110011 Rm:u=0bxxxxx ig1=0b1 O=0b0 ig2=0b00 opcode=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA512SU1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_3 page=SHA512SU1_advsimd cclass=SHA512SU1_VVV2_cryptosha512_3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHADD_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SHADD',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SHADD_advsimd cclass=SHADD_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING|SIGNED',
};

ENCODING SHL_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx opcode=0b01010 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SHL_advsimd cclass=SHL_asisdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT',
};

ENCODING SHL_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b01010 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SHL_advsimd cclass=SHL_asimdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT',
};

ENCODING SHLL_asimdmisc_S => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b10011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SHLL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SHLL_advsimd cclass=SHLL_asimdmisc_S',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT',
};

ENCODING SHRN_asimdshf_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1000 op:u=0b0 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SHRN',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SHRN_advsimd cclass=SHRN_asimdshf_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT',
};

ENCODING SHSUB_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00100 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SHSUB',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SHSUB_advsimd cclass=SHSUB_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING|SIGNED',
};

ENCODING SLI_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx opcode=0b01010 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SLI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SLI_advsimd cclass=SLI_asisdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|INSERTION',
};

ENCODING SLI_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b01010 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SLI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SLI_advsimd cclass=SLI_asimdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|INSERTION',
};

ENCODING SM3PARTW1_VVV4_cryptosha512_3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110011 Rm:u=0bxxxxx ig1=0b1 O=0b1 ig2=0b00 opcode=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3PARTW1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_3 page=SM3PARTW1_advsimd cclass=SM3PARTW1_VVV4_cryptosha512_3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SM3PARTW2_VVV4_cryptosha512_3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110011 Rm:u=0bxxxxx ig1=0b1 O=0b1 ig2=0b00 opcode=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3PARTW2',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_3 page=SM3PARTW2_advsimd cclass=SM3PARTW2_VVV4_cryptosha512_3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SM3SS1_VVV4_crypto4 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b110011100 Op0=0b10 Rm:u=0bxxxxx ig1=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3SS1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=crypto4 page=SM3SS1_advsimd cclass=SM3SS1_VVV4_crypto4',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SM3TT1A_VVV4_crypto3_imm2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110010 Rm:u=0bxxxxx ig1=0b10 imm2:u=0bxx opcode=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3TT1A',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=crypto3_imm2 page=SM3TT1A_advsimd cclass=SM3TT1A_VVV4_crypto3_imm2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SM3TT1B_VVV4_crypto3_imm2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110010 Rm:u=0bxxxxx ig1=0b10 imm2:u=0bxx opcode=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3TT1B',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=crypto3_imm2 page=SM3TT1B_advsimd cclass=SM3TT1B_VVV4_crypto3_imm2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SM3TT2A_VVV4_crypto3_imm2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110010 Rm:u=0bxxxxx ig1=0b10 imm2:u=0bxx opcode=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3TT2A',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=crypto3_imm2 page=SM3TT2A_advsimd cclass=SM3TT2A_VVV4_crypto3_imm2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SM3TT2B_VVV_crypto3_imm2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110010 Rm:u=0bxxxxx ig1=0b10 imm2:u=0bxx opcode=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3TT2B',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=crypto3_imm2 page=SM3TT2B_advsimd cclass=SM3TT2B_VVV_crypto3_imm2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SM4E_VV4_cryptosha512_2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110110000001000 opcode=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM4E',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_2 page=SM4E_advsimd cclass=SM4E_VV4_cryptosha512_2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SM4EKEY_VVV4_cryptosha512_3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110011 Rm:u=0bxxxxx ig1=0b1 O=0b1 ig2=0b00 opcode=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM4EKEY',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_3 page=SM4EKEY_advsimd cclass=SM4EKEY_VVV4_cryptosha512_3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SMAX_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0110 o1:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMAX',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SMAX_advsimd cclass=SMAX_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX|SIGNED',
};

ENCODING SMAXP_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b1010 o1:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMAXP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SMAXP_advsimd cclass=SMAXP_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX|SIGNED',
};

ENCODING SMAXV_asimdall_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b11000 op:u=0b0 ig3=0b1010 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SMAXV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=SMAXV_advsimd cclass=SMAXV_asimdall_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX|SIGNED',
};

ENCODING SMIN_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0110 o1:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMIN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SMIN_advsimd cclass=SMIN_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX|SIGNED',
};

ENCODING SMINP_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b1010 o1:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMINP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SMINP_advsimd cclass=SMINP_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX|SIGNED',
};

ENCODING SMINV_asimdall_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b11000 op:u=0b1 ig3=0b1010 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SMINV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=SMINV_advsimd cclass=SMINV_asimdall_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX|SIGNED',
};

ENCODING SMLAL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b0 ig3=0b10 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SMLAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SMLAL_advsimd_elt cclass=SMLAL_asimdelem_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SIGNED',
};

ENCODING SMLAL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b10 o1:u=0b0 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMLAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SMLAL_advsimd_vec cclass=SMLAL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SIGNED',
};

ENCODING SMLSL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b1 ig3=0b10 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SMLSL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SMLSL_advsimd_elt cclass=SMLSL_asimdelem_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SIGNED',
};

ENCODING SMLSL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b10 o1:u=0b1 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMLSL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SMLSL_advsimd_vec cclass=SMLSL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SIGNED',
};

ENCODING SMMLA_asimdsame2_G => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q=0b1 U:u=0b0 ig1=0b01110 size=0b10 ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b010 B:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMMLA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=SMMLA_advsimd_vec cclass=SMMLA_asimdsame2_G',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SIGNED',
};

ENCODING SMOV_asimdins_W_w => {
  name       => '32-bit',
  diagram    => 'ig0=0b0 Q:u=0b0 op=0b0 ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4=0b0101 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'datatype=32 instr-class=advsimd isa=A64 mnemonic=SMOV vector-xfer-type=general-from-element',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD datasize=32',
  tags       => 'group=simd_dp iclass=asimdins page=SMOV_advsimd cclass=SMOV_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|SIGNED',
};

ENCODING SMOV_asimdins_X_x => {
  name       => '64-reg,SMOV-64-reg',
  diagram    => 'ig0=0b0 Q:u=0b1 op=0b0 ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4=0b0101 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'atomic-ops=SMOV-64-reg instr-class=advsimd isa=A64 mnemonic=SMOV reg-type=64-reg vector-xfer-type=general-from-element',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=SMOV_advsimd cclass=SMOV_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|SIGNED',
};

ENCODING SMULL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1010 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SMULL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SMULL_advsimd_elt cclass=SMULL_asimdelem_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SIGNED',
};

ENCODING SMULL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b1100 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMULL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SMULL_advsimd_vec cclass=SMULL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SIGNED',
};

ENCODING SQABS_asisdmisc_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b00111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQABS',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=SQABS_advsimd cclass=SQABS_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|SATURATING|SIGNED',
};

ENCODING SQABS_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b00111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQABS',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SQABS_advsimd cclass=SQABS_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|SATURATING|SIGNED',
};

ENCODING SQADD_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQADD',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SQADD_advsimd cclass=SQADD_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQADD_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQADD',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SQADD_advsimd cclass=SQADD_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQDMLAL_asisdelem_L => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b0 ig3=0b11 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-scalar advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQDMLAL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=SQDMLAL_advsimd_elt cclass=SQDMLAL_asisdelem_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQDMLAL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b0 ig3=0b11 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SQDMLAL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SQDMLAL_advsimd_elt cclass=SQDMLAL_asimdelem_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQDMLAL_asisddiff_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b10 o1:u=0b0 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQDMLAL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisddiff page=SQDMLAL_advsimd_vec cclass=SQDMLAL_asisddiff_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQDMLAL_asimddiff_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b10 o1:u=0b0 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQDMLAL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SQDMLAL_advsimd_vec cclass=SQDMLAL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQDMLSL_asisdelem_L => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b1 ig3=0b11 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-scalar advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQDMLSL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=SQDMLSL_advsimd_elt cclass=SQDMLSL_asisdelem_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQDMLSL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b1 ig3=0b11 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SQDMLSL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SQDMLSL_advsimd_elt cclass=SQDMLSL_asimdelem_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQDMLSL_asisddiff_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b10 o1:u=0b1 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQDMLSL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisddiff page=SQDMLSL_advsimd_vec cclass=SQDMLSL_asisddiff_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQDMLSL_asimddiff_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b10 o1:u=0b1 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQDMLSL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SQDMLSL_advsimd_vec cclass=SQDMLSL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQDMULH_asisdelem_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b110 op:u=0b0 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-scalar advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQDMULH',
  docvars2   => 'doubling=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=SQDMULH_advsimd_elt cclass=SQDMULH_asisdelem_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQDMULH_asimdelem_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b110 op:u=0b0 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SQDMULH',
  docvars2   => 'doubling=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SQDMULH_advsimd_elt cclass=SQDMULH_asimdelem_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQDMULH_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10110 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQDMULH',
  docvars2   => 'doubling=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SQDMULH_advsimd_vec cclass=SQDMULH_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQDMULH_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10110 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQDMULH',
  docvars2   => 'doubling=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SQDMULH_advsimd_vec cclass=SQDMULH_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQDMULL_asisdelem_L => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1011 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-scalar advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQDMULL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=SQDMULL_advsimd_elt cclass=SQDMULL_asisdelem_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQDMULL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1011 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SQDMULL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SQDMULL_advsimd_elt cclass=SQDMULL_asimdelem_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQDMULL_asisddiff_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b1101 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQDMULL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisddiff page=SQDMULL_advsimd_vec cclass=SQDMULL_asisddiff_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQDMULL_asimddiff_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b1101 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQDMULL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SQDMULL_advsimd_vec cclass=SQDMULL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQNEG_asisdmisc_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b00111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQNEG',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=SQNEG_advsimd cclass=SQNEG_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE|SATURATING|SIGNED',
};

ENCODING SQNEG_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b00111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQNEG',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SQNEG_advsimd cclass=SQNEG_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE|SATURATING|SIGNED',
};

ENCODING SQRDMLAH_asisdelem_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b11 S:u=0b0 ig3=0b1 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-scalar advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRDMLAH',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=SQRDMLAH_advsimd_elt cclass=SQRDMLAH_asisdelem_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING SQRDMLAH_asimdelem_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b11 S:u=0b0 ig3=0b1 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SQRDMLAH',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SQRDMLAH_advsimd_elt cclass=SQRDMLAH_asimdelem_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING SQRDMLAH_asisdsame2_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b000 S:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRDMLAH',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame2 page=SQRDMLAH_advsimd_vec cclass=SQRDMLAH_asisdsame2_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING SQRDMLAH_asimdsame2_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b000 S:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQRDMLAH',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=SQRDMLAH_advsimd_vec cclass=SQRDMLAH_asimdsame2_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING SQRDMLSH_asisdelem_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b11 S:u=0b1 ig3=0b1 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-scalar advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRDMLSH',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=SQRDMLSH_advsimd_elt cclass=SQRDMLSH_asisdelem_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING SQRDMLSH_asimdelem_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b11 S:u=0b1 ig3=0b1 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SQRDMLSH',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SQRDMLSH_advsimd_elt cclass=SQRDMLSH_asimdelem_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING SQRDMLSH_asisdsame2_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b000 S:u=0b1 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRDMLSH',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame2 page=SQRDMLSH_advsimd_vec cclass=SQRDMLSH_asisdsame2_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING SQRDMLSH_asimdsame2_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b000 S:u=0b1 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQRDMLSH',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=SQRDMLSH_advsimd_vec cclass=SQRDMLSH_asimdsame2_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING SQRDMULH_asisdelem_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b110 op:u=0b1 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-scalar advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRDMULH',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=SQRDMULH_advsimd_elt cclass=SQRDMULH_asisdelem_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING SQRDMULH_asimdelem_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b110 op:u=0b1 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SQRDMULH',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SQRDMULH_advsimd_elt cclass=SQRDMULH_asimdelem_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING SQRDMULH_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10110 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRDMULH',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SQRDMULH_advsimd_vec cclass=SQRDMULH_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING SQRDMULH_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10110 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQRDMULH',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SQRDMULH_advsimd_vec cclass=SQRDMULH_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING SQRSHL_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b1 S:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRSHL',
  docvars2   => 'rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SQRSHL_advsimd cclass=SQRSHL_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|ROUNDING|SATURATING|SIGNED',
};

ENCODING SQRSHL_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b1 S:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQRSHL',
  docvars2   => 'rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SQRSHL_advsimd cclass=SQRSHL_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|ROUNDING|SATURATING|SIGNED',
};

ENCODING SQRSHRN_asisdshf_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1001 op:u=0b1 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRSHRN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SQRSHRN_advsimd cclass=SQRSHRN_asisdshf_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|SATURATING|SIGNED',
};

ENCODING SQRSHRN_asimdshf_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1001 op:u=0b1 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQRSHRN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SQRSHRN_advsimd cclass=SQRSHRN_asimdshf_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|SATURATING|SIGNED',
};

ENCODING SQRSHRUN_asisdshf_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1000 op:u=0b1 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRSHRUN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SQRSHRUN_advsimd cclass=SQRSHRUN_asisdshf_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|SATURATING',
};

ENCODING SQRSHRUN_asimdshf_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1000 op:u=0b1 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQRSHRUN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SQRSHRUN_advsimd cclass=SQRSHRUN_asimdshf_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|SATURATING',
};

ENCODING SQSHL_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b011 op:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQSHL',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SQSHL_advsimd_imm cclass=SQSHL_asisdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING|SIGNED',
};

ENCODING SQSHL_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b011 op:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQSHL',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SQSHL_advsimd_imm cclass=SQSHL_asimdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING|SIGNED',
};

ENCODING SQSHL_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b0 S:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQSHL',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SQSHL_advsimd_reg cclass=SQSHL_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING|SIGNED',
};

ENCODING SQSHL_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b0 S:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQSHL',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SQSHL_advsimd_reg cclass=SQSHL_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING|SIGNED',
};

ENCODING SQSHLU_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b011 op:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQSHLU',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SQSHLU_advsimd cclass=SQSHLU_asisdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING',
};

ENCODING SQSHLU_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b011 op:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQSHLU',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SQSHLU_advsimd cclass=SQSHLU_asimdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING',
};

ENCODING SQSHRN_asisdshf_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1001 op:u=0b0 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQSHRN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SQSHRN_advsimd cclass=SQSHRN_asisdshf_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|SATURATING|SIGNED',
};

ENCODING SQSHRN_asimdshf_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1001 op:u=0b0 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQSHRN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SQSHRN_advsimd cclass=SQSHRN_asimdshf_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|SATURATING|SIGNED',
};

ENCODING SQSHRUN_asisdshf_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1000 op:u=0b0 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQSHRUN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SQSHRUN_advsimd cclass=SQSHRUN_asisdshf_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|SATURATING',
};

ENCODING SQSHRUN_asimdshf_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1000 op:u=0b0 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQSHRUN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SQSHRUN_advsimd cclass=SQSHRUN_asimdshf_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|SATURATING',
};

ENCODING SQSUB_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00101 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQSUB',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SQSUB_advsimd cclass=SQSUB_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQSUB_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00101 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQSUB',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SQSUB_advsimd cclass=SQSUB_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING SQXTN_asisdmisc_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b10100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQXTN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=SQXTN_advsimd cclass=SQXTN_asisdmisc_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|EXTRACTION|SATURATING|SIGNED',
};

ENCODING SQXTN_asimdmisc_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b10100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQXTN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SQXTN_advsimd cclass=SQXTN_asimdmisc_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|EXTRACTION|SATURATING|SIGNED',
};

ENCODING SQXTUN_asisdmisc_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b10010 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQXTUN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=SQXTUN_advsimd cclass=SQXTUN_asisdmisc_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|EXTRACTION|SATURATING',
};

ENCODING SQXTUN_asimdmisc_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b10010 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQXTUN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SQXTUN_advsimd cclass=SQXTUN_asimdmisc_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|EXTRACTION|SATURATING',
};

ENCODING SRHADD_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00010 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SRHADD',
  docvars2   => 'halving=1 rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SRHADD_advsimd cclass=SRHADD_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING|ROUNDING|SIGNED',
};

ENCODING SRI_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx opcode=0b01000 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SRI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SRI_advsimd cclass=SRI_asisdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|INSERTION',
};

ENCODING SRI_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b01000 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SRI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SRI_advsimd cclass=SRI_asimdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|INSERTION',
};

ENCODING SRSHL_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b1 S:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SRSHL',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SRSHL_advsimd cclass=SRSHL_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|ROUNDING|SIGNED',
};

ENCODING SRSHL_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b1 S:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SRSHL',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SRSHL_advsimd cclass=SRSHL_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|ROUNDING|SIGNED',
};

ENCODING SRSHR_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b1 o0:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SRSHR',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SRSHR_advsimd cclass=SRSHR_asisdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|SIGNED',
};

ENCODING SRSHR_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b1 o0:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SRSHR',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SRSHR_advsimd cclass=SRSHR_asimdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|SIGNED',
};

ENCODING SRSRA_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b1 o0:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SRSRA',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SRSRA_advsimd cclass=SRSRA_asisdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|CARITHMETIC|ROUNDING|SIGNED',
};

ENCODING SRSRA_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b1 o0:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SRSRA',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SRSRA_advsimd cclass=SRSRA_asimdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|CARITHMETIC|ROUNDING|SIGNED',
};

ENCODING SSHL_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b0 S:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SSHL_advsimd cclass=SSHL_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SIGNED',
};

ENCODING SSHL_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b0 S:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SSHL_advsimd cclass=SSHL_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SIGNED',
};

ENCODING SSHLL_asimdshf_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b10100 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SSHLL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SSHLL_advsimd cclass=SSHLL_advsimd_SSHLL_asimdshf_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SIGNED',
};

ENCODING SSHR_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b0 o0:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SSHR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SSHR_advsimd cclass=SSHR_asisdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|SIGNED',
};

ENCODING SSHR_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b0 o0:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SSHR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SSHR_advsimd cclass=SSHR_asimdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|SIGNED',
};

ENCODING SSRA_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b0 o0:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SSRA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SSRA_advsimd cclass=SSRA_asisdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|CARITHMETIC|SIGNED',
};

ENCODING SSRA_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b0 o0:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SSRA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SSRA_advsimd cclass=SSRA_asimdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|CARITHMETIC|SIGNED',
};

ENCODING SSUBL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b00 o1:u=0b1 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SSUBL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SSUBL_advsimd cclass=SSUBL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SIGNED',
};

ENCODING SSUBW_asimddiff_W => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b00 o1:u=0b1 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SSUBW',
  docvars2   => 'shape=wide',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SSUBW_advsimd cclass=SSUBW_asimddiff_W',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SIGNED',
};

ENCODING ST1_asisdlse_R1_1v => {
  name       => 'One register',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b0 ig2=0b000000 opcode:u=0b0111 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b0111',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST1 ststruct-regcount=from-1reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=ST1_advsimd_mult cclass=ST1_advsimd_mult_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlse_R2_2v => {
  name       => 'Two registers',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b0 ig2=0b000000 opcode:u=0b1010 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b1010',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST1 ststruct-regcount=from-2reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=ST1_advsimd_mult cclass=ST1_advsimd_mult_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlse_R3_3v => {
  name       => 'Three registers',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b0 ig2=0b000000 opcode:u=0b0110 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b0110',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST1 ststruct-regcount=from-3reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=ST1_advsimd_mult cclass=ST1_advsimd_mult_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlse_R4_4v => {
  name       => 'Four registers',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b0 ig2=0b000000 opcode:u=0b0010 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b0010',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST1 ststruct-regcount=from-4reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=ST1_advsimd_mult cclass=ST1_advsimd_mult_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlsep_I1_i1 => {
  name       => 'One register, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0b11111 opcode:u=0b0111 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b0111',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST1 sti-mult-labels=from-1reg-post-index-imm ststruct-regcount=from-1reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST1_advsimd_mult cclass=ST1_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlsep_R1_r1 => {
  name       => 'One register, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b0111 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b0111',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST1 sti-mult-labels=from-1reg-post-index-reg ststruct-regcount=from-1reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST1_advsimd_mult cclass=ST1_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlsep_I2_i2 => {
  name       => 'Two registers, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0b11111 opcode:u=0b1010 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b1010',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST1 sti-mult-labels=from-2reg-post-index-imm ststruct-regcount=from-2reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST1_advsimd_mult cclass=ST1_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlsep_R2_r2 => {
  name       => 'Two registers, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b1010 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b1010',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST1 sti-mult-labels=from-2reg-post-index-reg ststruct-regcount=from-2reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST1_advsimd_mult cclass=ST1_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlsep_I3_i3 => {
  name       => 'Three registers, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0b11111 opcode:u=0b0110 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b0110',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST1 sti-mult-labels=from-3reg-post-index-imm ststruct-regcount=from-3reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST1_advsimd_mult cclass=ST1_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlsep_R3_r3 => {
  name       => 'Three registers, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b0110 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b0110',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST1 sti-mult-labels=from-3reg-post-index-reg ststruct-regcount=from-3reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST1_advsimd_mult cclass=ST1_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlsep_I4_i4 => {
  name       => 'Four registers, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0b11111 opcode:u=0b0010 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b0010',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST1 sti-mult-labels=from-4reg-post-index-imm ststruct-regcount=from-4reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST1_advsimd_mult cclass=ST1_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlsep_R4_r4 => {
  name       => 'Four registers, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b0010 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b0010',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST1 sti-mult-labels=from-4reg-post-index-reg ststruct-regcount=from-4reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST1_advsimd_mult cclass=ST1_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlso_B1_1b => {
  name       => '8-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b0 ig2=0b00000 opcode:u=0b000 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b000',
  docvars    => 'as-structure-org=of-bytes as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST1_advsimd_sngl cclass=ST1_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlso_H1_1h => {
  name       => '16-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b0 ig2=0b00000 opcode:u=0b010 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b010 && size == 0bx0',
  docvars    => 'as-structure-org=of-halfwords as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST1_advsimd_sngl cclass=ST1_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlso_S1_1s => {
  name       => '32-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b0 ig2=0b00000 opcode:u=0b100 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b100 && size == 0b00',
  docvars    => 'as-structure-org=of-words as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST1_advsimd_sngl cclass=ST1_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlso_D1_1d => {
  name       => '64-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b0 ig2=0b00000 opcode:u=0b100 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b100 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-org=of-doublewords as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST1_advsimd_sngl cclass=ST1_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlsop_B1_i1b => {
  name       => '8-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0b11111 opcode:u=0b000 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b000',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-bytes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-bytes-post-index-imm mnemonic=ST1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST1_advsimd_sngl cclass=ST1_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlsop_BX1_r1b => {
  name       => '8-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b000 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b000',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-bytes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-bytes-post-index-reg mnemonic=ST1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST1_advsimd_sngl cclass=ST1_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlsop_H1_i1h => {
  name       => '16-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0b11111 opcode:u=0b010 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b010 && size == 0bx0',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-halfwords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-halfwords-post-index-imm mnemonic=ST1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST1_advsimd_sngl cclass=ST1_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlsop_HX1_r1h => {
  name       => '16-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b010 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b010 && size == 0bx0',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-halfwords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-halfwords-post-index-reg mnemonic=ST1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST1_advsimd_sngl cclass=ST1_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlsop_S1_i1s => {
  name       => '32-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0b11111 opcode:u=0b100 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b100 && size == 0b00',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-words as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-words-post-index-imm mnemonic=ST1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST1_advsimd_sngl cclass=ST1_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlsop_SX1_r1s => {
  name       => '32-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b100 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b100 && size == 0b00',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-words as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-words-post-index-reg mnemonic=ST1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST1_advsimd_sngl cclass=ST1_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlsop_D1_i1d => {
  name       => '64-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0b11111 opcode:u=0b100 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b100 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-doublewords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-doublewords-post-index-imm mnemonic=ST1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST1_advsimd_sngl cclass=ST1_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST1_asisdlsop_DX1_r1d => {
  name       => '64-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b100 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b100 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-doublewords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-doublewords-post-index-reg mnemonic=ST1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST1_advsimd_sngl cclass=ST1_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST2_asisdlse_R2 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b0 ig2=0b000000 opcode:u=0b1000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=ST2_advsimd_mult cclass=ST2_asisdlse_R2',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST2_asisdlsep_I2_i => {
  name       => 'Immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0b11111 opcode:u=0b1000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST2 sti-mult-labels=from-2reg-post-index-imm ststruct-regcount=from-2reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST2_advsimd_mult cclass=ST2_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST2_asisdlsep_R2_r => {
  name       => 'Register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b1000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST2 sti-mult-labels=from-2reg-post-index-reg ststruct-regcount=from-2reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST2_advsimd_mult cclass=ST2_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST2_asisdlso_B2_2b => {
  name       => '8-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b1 ig2=0b00000 opcode:u=0b000 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b000',
  docvars    => 'as-structure-org=of-bytes as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST2_advsimd_sngl cclass=ST2_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST2_asisdlso_H2_2h => {
  name       => '16-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b1 ig2=0b00000 opcode:u=0b010 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b010 && size == 0bx0',
  docvars    => 'as-structure-org=of-halfwords as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST2_advsimd_sngl cclass=ST2_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST2_asisdlso_S2_2s => {
  name       => '32-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b1 ig2=0b00000 opcode:u=0b100 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b100 && size == 0b00',
  docvars    => 'as-structure-org=of-words as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST2_advsimd_sngl cclass=ST2_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST2_asisdlso_D2_2d => {
  name       => '64-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b1 ig2=0b00000 opcode:u=0b100 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b100 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-org=of-doublewords as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST2_advsimd_sngl cclass=ST2_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST2_asisdlsop_B2_i2b => {
  name       => '8-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0b11111 opcode:u=0b000 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b000',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-bytes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-bytes-post-index-imm mnemonic=ST2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST2_advsimd_sngl cclass=ST2_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST2_asisdlsop_BX2_r2b => {
  name       => '8-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b000 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b000',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-bytes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-bytes-post-index-reg mnemonic=ST2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST2_advsimd_sngl cclass=ST2_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST2_asisdlsop_H2_i2h => {
  name       => '16-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0b11111 opcode:u=0b010 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b010 && size == 0bx0',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-halfwords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-halfwords-post-index-imm mnemonic=ST2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST2_advsimd_sngl cclass=ST2_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST2_asisdlsop_HX2_r2h => {
  name       => '16-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b010 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b010 && size == 0bx0',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-halfwords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-halfwords-post-index-reg mnemonic=ST2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST2_advsimd_sngl cclass=ST2_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST2_asisdlsop_S2_i2s => {
  name       => '32-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0b11111 opcode:u=0b100 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b100 && size == 0b00',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-words as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-words-post-index-imm mnemonic=ST2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST2_advsimd_sngl cclass=ST2_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST2_asisdlsop_SX2_r2s => {
  name       => '32-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b100 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b100 && size == 0b00',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-words as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-words-post-index-reg mnemonic=ST2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST2_advsimd_sngl cclass=ST2_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST2_asisdlsop_D2_i2d => {
  name       => '64-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0b11111 opcode:u=0b100 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b100 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-doublewords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-doublewords-post-index-imm mnemonic=ST2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST2_advsimd_sngl cclass=ST2_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST2_asisdlsop_DX2_r2d => {
  name       => '64-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b100 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b100 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-doublewords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-doublewords-post-index-reg mnemonic=ST2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST2_advsimd_sngl cclass=ST2_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST3_asisdlse_R3 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b0 ig2=0b000000 opcode:u=0b0100 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=ST3_advsimd_mult cclass=ST3_asisdlse_R3',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST3_asisdlsep_I3_i => {
  name       => 'Immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0b11111 opcode:u=0b0100 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST3 sti-mult-labels=from-3reg-post-index-imm ststruct-regcount=from-3reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST3_advsimd_mult cclass=ST3_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST3_asisdlsep_R3_r => {
  name       => 'Register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b0100 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST3 sti-mult-labels=from-3reg-post-index-reg ststruct-regcount=from-3reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST3_advsimd_mult cclass=ST3_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST3_asisdlso_B3_3b => {
  name       => '8-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b0 ig2=0b00000 opcode:u=0b001 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b001',
  docvars    => 'as-structure-org=of-bytes as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST3_advsimd_sngl cclass=ST3_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST3_asisdlso_H3_3h => {
  name       => '16-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b0 ig2=0b00000 opcode:u=0b011 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b011 && size == 0bx0',
  docvars    => 'as-structure-org=of-halfwords as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST3_advsimd_sngl cclass=ST3_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST3_asisdlso_S3_3s => {
  name       => '32-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b0 ig2=0b00000 opcode:u=0b101 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b101 && size == 0b00',
  docvars    => 'as-structure-org=of-words as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST3_advsimd_sngl cclass=ST3_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST3_asisdlso_D3_3d => {
  name       => '64-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b0 ig2=0b00000 opcode:u=0b101 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b101 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-org=of-doublewords as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST3_advsimd_sngl cclass=ST3_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST3_asisdlsop_B3_i3b => {
  name       => '8-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0b11111 opcode:u=0b001 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b001',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-bytes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-bytes-post-index-imm mnemonic=ST3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST3_advsimd_sngl cclass=ST3_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST3_asisdlsop_BX3_r3b => {
  name       => '8-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b001 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b001',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-bytes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-bytes-post-index-reg mnemonic=ST3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST3_advsimd_sngl cclass=ST3_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST3_asisdlsop_H3_i3h => {
  name       => '16-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0b11111 opcode:u=0b011 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b011 && size == 0bx0',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-halfwords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-halfwords-post-index-imm mnemonic=ST3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST3_advsimd_sngl cclass=ST3_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST3_asisdlsop_HX3_r3h => {
  name       => '16-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b011 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b011 && size == 0bx0',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-halfwords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-halfwords-post-index-reg mnemonic=ST3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST3_advsimd_sngl cclass=ST3_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST3_asisdlsop_S3_i3s => {
  name       => '32-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0b11111 opcode:u=0b101 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b101 && size == 0b00',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-words as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-words-post-index-imm mnemonic=ST3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST3_advsimd_sngl cclass=ST3_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST3_asisdlsop_SX3_r3s => {
  name       => '32-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b101 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b101 && size == 0b00',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-words as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-words-post-index-reg mnemonic=ST3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST3_advsimd_sngl cclass=ST3_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST3_asisdlsop_D3_i3d => {
  name       => '64-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0b11111 opcode:u=0b101 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b101 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-doublewords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-doublewords-post-index-imm mnemonic=ST3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST3_advsimd_sngl cclass=ST3_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST3_asisdlsop_DX3_r3d => {
  name       => '64-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b101 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b101 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-doublewords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-doublewords-post-index-reg mnemonic=ST3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST3_advsimd_sngl cclass=ST3_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST4_asisdlse_R4 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b0 ig2=0b000000 opcode:u=0b0000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=ST4_advsimd_mult cclass=ST4_asisdlse_R4',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST4_asisdlsep_I4_i => {
  name       => 'Immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0b11111 opcode:u=0b0000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST4 sti-mult-labels=from-4reg-post-index-imm ststruct-regcount=from-4reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST4_advsimd_mult cclass=ST4_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST4_asisdlsep_R4_r => {
  name       => 'Register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b0000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST4 sti-mult-labels=from-4reg-post-index-reg ststruct-regcount=from-4reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST4_advsimd_mult cclass=ST4_advsimd_mult_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST4_asisdlso_B4_4b => {
  name       => '8-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b1 ig2=0b00000 opcode:u=0b001 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b001',
  docvars    => 'as-structure-org=of-bytes as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST4_advsimd_sngl cclass=ST4_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST4_asisdlso_H4_4h => {
  name       => '16-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b1 ig2=0b00000 opcode:u=0b011 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b011 && size == 0bx0',
  docvars    => 'as-structure-org=of-halfwords as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST4_advsimd_sngl cclass=ST4_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST4_asisdlso_S4_4s => {
  name       => '32-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b1 ig2=0b00000 opcode:u=0b101 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b101 && size == 0b00',
  docvars    => 'as-structure-org=of-words as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST4_advsimd_sngl cclass=ST4_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST4_asisdlso_D4_4d => {
  name       => '64-bit',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b1 ig2=0b00000 opcode:u=0b101 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opcode == 0b101 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-org=of-doublewords as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST4_advsimd_sngl cclass=ST4_advsimd_sngl_as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST4_asisdlsop_B4_i4b => {
  name       => '8-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0b11111 opcode:u=0b001 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b001',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-bytes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-bytes-post-index-imm mnemonic=ST4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST4_advsimd_sngl cclass=ST4_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST4_asisdlsop_BX4_r4b => {
  name       => '8-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b001 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b001',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-bytes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-bytes-post-index-reg mnemonic=ST4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST4_advsimd_sngl cclass=ST4_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST4_asisdlsop_H4_i4h => {
  name       => '16-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0b11111 opcode:u=0b011 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b011 && size == 0bx0',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-halfwords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-halfwords-post-index-imm mnemonic=ST4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST4_advsimd_sngl cclass=ST4_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST4_asisdlsop_HX4_r4h => {
  name       => '16-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b011 S:u=0bx size:u=0bx0 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b011 && size == 0bx0',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-halfwords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-halfwords-post-index-reg mnemonic=ST4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST4_advsimd_sngl cclass=ST4_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST4_asisdlsop_S4_i4s => {
  name       => '32-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0b11111 opcode:u=0b101 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b101 && size == 0b00',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-words as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-words-post-index-imm mnemonic=ST4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST4_advsimd_sngl cclass=ST4_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST4_asisdlsop_SX4_r4s => {
  name       => '32-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b101 S:u=0bx size:u=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b101 && size == 0b00',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-words as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-words-post-index-reg mnemonic=ST4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST4_advsimd_sngl cclass=ST4_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST4_asisdlsop_D4_i4d => {
  name       => '64-bit, immediate offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0b11111 opcode:u=0b101 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'Rm == 0b11111 && opcode == 0b101 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-index-source=post-index-imm as-structure-org=of-doublewords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-doublewords-post-index-imm mnemonic=ST4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST4_advsimd_sngl cclass=ST4_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING ST4_asisdlsop_DX4_r4d => {
  name       => '64-bit, register offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b101 S:u=0b0 size:u=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111',
  bitdiffs   => 'Rm != 0b11111 && opcode == 0b101 && S == 0b0 && size == 0b01',
  docvars    => 'as-structure-index-source=post-index-reg as-structure-org=of-doublewords as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ld1-single-labels=of-doublewords-post-index-reg mnemonic=ST4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST4_advsimd_sngl cclass=ST4_advsimd_sngl_as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING STNP_S_ldstnapair_offs => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b101 V=0b1 ig1=0b000 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-words atomic-ops=STNP-pair-words instr-class=fpsimd isa=A64 mnemonic=STNP offset-type=off7s_s reg-type=pair-words',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstnapair_offs page=STNP_fpsimd cclass=STNP_fpsimd_signed_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|HINT|STORE',
};

ENCODING STNP_D_ldstnapair_offs => {
  name       => '64-bit',
  diagram    => 'opc:u=0b01 ig0=0b101 V=0b1 ig1=0b000 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b01',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-doublewords atomic-ops=STNP-pair-doublewords instr-class=fpsimd isa=A64 mnemonic=STNP offset-type=off7s_s reg-type=pair-doublewords',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstnapair_offs page=STNP_fpsimd cclass=STNP_fpsimd_signed_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|HINT|STORE',
};

ENCODING STNP_Q_ldstnapair_offs => {
  name       => '128-bit',
  diagram    => 'opc:u=0b10 ig0=0b101 V=0b1 ig1=0b000 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-quadwords atomic-ops=STNP-pair-quadwords instr-class=fpsimd isa=A64 mnemonic=STNP offset-type=off7s_s reg-type=pair-quadwords',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstnapair_offs page=STNP_fpsimd cclass=STNP_fpsimd_signed_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|HINT|STORE',
};

ENCODING STP_S_ldstpair_post => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b101 V=0b1 ig1=0b001 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-pair-words atomic-ops=STP-pair-words instr-class=fpsimd isa=A64 mnemonic=STP offset-type=off7s_s reg-type=pair-words',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_post page=STP_fpsimd cclass=STP_fpsimd_post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STP_D_ldstpair_post => {
  name       => '64-bit',
  diagram    => 'opc:u=0b01 ig0=0b101 V=0b1 ig1=0b001 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b01',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-pair-doublewords atomic-ops=STP-pair-doublewords instr-class=fpsimd isa=A64 mnemonic=STP offset-type=off7s_s reg-type=pair-doublewords',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_post page=STP_fpsimd cclass=STP_fpsimd_post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STP_Q_ldstpair_post => {
  name       => '128-bit',
  diagram    => 'opc:u=0b10 ig0=0b101 V=0b1 ig1=0b001 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-pair-quadwords atomic-ops=STP-pair-quadwords instr-class=fpsimd isa=A64 mnemonic=STP offset-type=off7s_s reg-type=pair-quadwords',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_post page=STP_fpsimd cclass=STP_fpsimd_post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STP_S_ldstpair_pre => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b101 V=0b1 ig1=0b011 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-pair-words atomic-ops=STP-pair-words instr-class=fpsimd isa=A64 mnemonic=STP offset-type=off7s_s reg-type=pair-words',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_pre page=STP_fpsimd cclass=STP_fpsimd_pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STP_D_ldstpair_pre => {
  name       => '64-bit',
  diagram    => 'opc:u=0b01 ig0=0b101 V=0b1 ig1=0b011 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b01',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-pair-doublewords atomic-ops=STP-pair-doublewords instr-class=fpsimd isa=A64 mnemonic=STP offset-type=off7s_s reg-type=pair-doublewords',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_pre page=STP_fpsimd cclass=STP_fpsimd_pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STP_Q_ldstpair_pre => {
  name       => '128-bit',
  diagram    => 'opc:u=0b10 ig0=0b101 V=0b1 ig1=0b011 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-pair-quadwords atomic-ops=STP-pair-quadwords instr-class=fpsimd isa=A64 mnemonic=STP offset-type=off7s_s reg-type=pair-quadwords',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_pre page=STP_fpsimd cclass=STP_fpsimd_pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STP_S_ldstpair_off => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b101 V=0b1 ig1=0b010 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-words atomic-ops=STP-pair-words instr-class=fpsimd isa=A64 mnemonic=STP offset-type=off7s_s reg-type=pair-words',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_off page=STP_fpsimd cclass=STP_fpsimd_signed_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STP_D_ldstpair_off => {
  name       => '64-bit',
  diagram    => 'opc:u=0b01 ig0=0b101 V=0b1 ig1=0b010 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b01',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-doublewords atomic-ops=STP-pair-doublewords instr-class=fpsimd isa=A64 mnemonic=STP offset-type=off7s_s reg-type=pair-doublewords',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_off page=STP_fpsimd cclass=STP_fpsimd_signed_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STP_Q_ldstpair_off => {
  name       => '128-bit',
  diagram    => 'opc:u=0b10 ig0=0b101 V=0b1 ig1=0b010 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-quadwords atomic-ops=STP-pair-quadwords instr-class=fpsimd isa=A64 mnemonic=STP offset-type=off7s_s reg-type=pair-quadwords',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_off page=STP_fpsimd cclass=STP_fpsimd_signed_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_B_ldst_immpost => {
  name       => '8-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b00',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-8-fsreg atomic-ops=STR-8-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off9s_u reg-type=8-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpost page=STR_imm_fpsimd cclass=STR_imm_fpsimd_post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_H_ldst_immpost => {
  name       => '16-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b01 && opc == 0b00',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-16-fsreg atomic-ops=STR-16-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off9s_u reg-type=16-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpost page=STR_imm_fpsimd cclass=STR_imm_fpsimd_post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_S_ldst_immpost => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && opc == 0b00',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-32-fsreg atomic-ops=STR-32-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off9s_u reg-type=32-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpost page=STR_imm_fpsimd cclass=STR_imm_fpsimd_post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_D_ldst_immpost => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && opc == 0b00',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-64-fsreg atomic-ops=STR-64-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off9s_u reg-type=64-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpost page=STR_imm_fpsimd cclass=STR_imm_fpsimd_post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_Q_ldst_immpost => {
  name       => '128-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b10',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-128-fsreg atomic-ops=STR-128-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off9s_u reg-type=128-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpost page=STR_imm_fpsimd cclass=STR_imm_fpsimd_post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_B_ldst_immpre => {
  name       => '8-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b00',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-8-fsreg atomic-ops=STR-8-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off9s_u reg-type=8-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpre page=STR_imm_fpsimd cclass=STR_imm_fpsimd_pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_H_ldst_immpre => {
  name       => '16-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b01 && opc == 0b00',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-16-fsreg atomic-ops=STR-16-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off9s_u reg-type=16-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpre page=STR_imm_fpsimd cclass=STR_imm_fpsimd_pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_S_ldst_immpre => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && opc == 0b00',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-32-fsreg atomic-ops=STR-32-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off9s_u reg-type=32-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpre page=STR_imm_fpsimd cclass=STR_imm_fpsimd_pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_D_ldst_immpre => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && opc == 0b00',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-64-fsreg atomic-ops=STR-64-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off9s_u reg-type=64-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpre page=STR_imm_fpsimd cclass=STR_imm_fpsimd_pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_Q_ldst_immpre => {
  name       => '128-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b10',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-128-fsreg atomic-ops=STR-128-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off9s_u reg-type=128-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpre page=STR_imm_fpsimd cclass=STR_imm_fpsimd_pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_B_ldst_pos => {
  name       => '8-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b01 opc:u=0b00 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b00',
  docvars    => 'address-form=unsigned-scaled-offset address-form-reg-type=unsigned-scaled-offset-8-fsreg atomic-ops=STR-8-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off12u_s reg-type=8-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_pos page=STR_imm_fpsimd cclass=STR_imm_fpsimd_unsigned_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_H_ldst_pos => {
  name       => '16-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b1 ig1=0b01 opc:u=0b00 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b01 && opc == 0b00',
  docvars    => 'address-form=unsigned-scaled-offset address-form-reg-type=unsigned-scaled-offset-16-fsreg atomic-ops=STR-16-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off12u_s reg-type=16-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_pos page=STR_imm_fpsimd cclass=STR_imm_fpsimd_unsigned_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_S_ldst_pos => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b1 ig1=0b01 opc:u=0b00 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && opc == 0b00',
  docvars    => 'address-form=unsigned-scaled-offset address-form-reg-type=unsigned-scaled-offset-32-fsreg atomic-ops=STR-32-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off12u_s reg-type=32-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_pos page=STR_imm_fpsimd cclass=STR_imm_fpsimd_unsigned_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_D_ldst_pos => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b1 ig1=0b01 opc:u=0b00 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && opc == 0b00',
  docvars    => 'address-form=unsigned-scaled-offset address-form-reg-type=unsigned-scaled-offset-64-fsreg atomic-ops=STR-64-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off12u_s reg-type=64-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_pos page=STR_imm_fpsimd cclass=STR_imm_fpsimd_unsigned_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_Q_ldst_pos => {
  name       => '128-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b01 opc:u=0b10 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b10',
  docvars    => 'address-form=unsigned-scaled-offset address-form-reg-type=unsigned-scaled-offset-128-fsreg atomic-ops=STR-128-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off12u_s reg-type=128-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_pos page=STR_imm_fpsimd cclass=STR_imm_fpsimd_unsigned_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_B_ldst_regoff => {
  name       => '8-fsreg,STR-8-fsreg',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b00 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'option!=0b011',
  bitdiffs   => 'size == 0b00 && opc == 0b00 && option != 0b011',
  docvars    => 'atomic-ops=STR-8-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off-reg reg-type=8-fsreg reg-type-and-use=8-fsreg-ext-reg reguse=ext-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_regoff page=STR_reg_fpsimd cclass=STR_reg_fpsimd_fpsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_BL_ldst_regoff => {
  name       => '8-fsreg,STR-8-fsreg',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b00 ig2=0b1 Rm:u=0bxxxxx option:u=0b011 S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b00 && option == 0b011',
  docvars    => 'atomic-ops=STR-8-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off-reg reg-type=8-fsreg reg-type-and-use=8-fsreg-shifted-reg reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_regoff page=STR_reg_fpsimd cclass=STR_reg_fpsimd_fpsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_H_ldst_regoff => {
  name       => '16-fsreg,STR-16-fsreg',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b00 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b01 && opc == 0b00',
  docvars    => 'atomic-ops=STR-16-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off-reg reg-type=16-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_regoff page=STR_reg_fpsimd cclass=STR_reg_fpsimd_fpsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_S_ldst_regoff => {
  name       => '32-fsreg,STR-32-fsreg',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b00 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && opc == 0b00',
  docvars    => 'atomic-ops=STR-32-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off-reg reg-type=32-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_regoff page=STR_reg_fpsimd cclass=STR_reg_fpsimd_fpsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_D_ldst_regoff => {
  name       => '64-fsreg,STR-64-fsreg',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b00 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && opc == 0b00',
  docvars    => 'atomic-ops=STR-64-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off-reg reg-type=64-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_regoff page=STR_reg_fpsimd cclass=STR_reg_fpsimd_fpsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STR_Q_ldst_regoff => {
  name       => '128-fsreg,STR-128-fsreg',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b10 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b10',
  docvars    => 'atomic-ops=STR-128-fsreg instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off-reg reg-type=128-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_regoff page=STR_reg_fpsimd cclass=STR_reg_fpsimd_fpsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STUR_B_ldst_unscaled => {
  name       => '8-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b00',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-8-fsreg atomic-ops=STUR-8-fsreg instr-class=fpsimd isa=A64 mnemonic=STUR offset-type=off9s_u reg-type=8-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_unscaled page=STUR_fpsimd cclass=STUR_fpsimd_base_plus_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STUR_H_ldst_unscaled => {
  name       => '16-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b01 && opc == 0b00',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-16-fsreg atomic-ops=STUR-16-fsreg instr-class=fpsimd isa=A64 mnemonic=STUR offset-type=off9s_u reg-type=16-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_unscaled page=STUR_fpsimd cclass=STUR_fpsimd_base_plus_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STUR_S_ldst_unscaled => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && opc == 0b00',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-32-fsreg atomic-ops=STUR-32-fsreg instr-class=fpsimd isa=A64 mnemonic=STUR offset-type=off9s_u reg-type=32-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_unscaled page=STUR_fpsimd cclass=STUR_fpsimd_base_plus_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STUR_D_ldst_unscaled => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && opc == 0b00',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-64-fsreg atomic-ops=STUR-64-fsreg instr-class=fpsimd isa=A64 mnemonic=STUR offset-type=off9s_u reg-type=64-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_unscaled page=STUR_fpsimd cclass=STUR_fpsimd_base_plus_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING STUR_Q_ldst_unscaled => {
  name       => '128-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b1 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b00 && opc == 0b10',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-128-fsreg atomic-ops=STUR-128-fsreg instr-class=fpsimd isa=A64 mnemonic=STUR offset-type=off9s_u reg-type=128-fsreg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_unscaled page=STUR_fpsimd cclass=STUR_fpsimd_base_plus_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
};

ENCODING SUB_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SUB_advsimd cclass=SUB_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING SUB_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SUB_advsimd cclass=SUB_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING SUBHN_asimddiff_N => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 o1:u=0b1 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SUBHN',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SUBHN_advsimd cclass=SUBHN_asimddiff_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING SUDOT_asimdelem_D => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 US:u=0b0 ig2=0b0 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1111 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SUDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SUDOT_advsimd_elt cclass=SUDOT_asimdelem_D',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING SUQADD_asisdmisc_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b00011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SUQADD',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=SUQADD_advsimd cclass=SUQADD_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING',
};

ENCODING SUQADD_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b00011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SUQADD',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SUQADD_advsimd cclass=SUQADD_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING',
};

ENCODING TBL_asimdtbl_L2_2 => {
  name       => 'Two register table',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 op2=0b00 ig2=0b0 Rm:u=0bxxxxx ig3=0b0 len:u=0b01 op:u=0b0 ig4=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'len == 0b01',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TBL no-reg-for-table=tbl2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdtbl page=TBL_advsimd cclass=TBL_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|TABLE_LOOKUP',
};

ENCODING TBL_asimdtbl_L3_3 => {
  name       => 'Three register table',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 op2=0b00 ig2=0b0 Rm:u=0bxxxxx ig3=0b0 len:u=0b10 op:u=0b0 ig4=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'len == 0b10',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TBL no-reg-for-table=tbl3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdtbl page=TBL_advsimd cclass=TBL_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|TABLE_LOOKUP',
};

ENCODING TBL_asimdtbl_L4_4 => {
  name       => 'Four register table',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 op2=0b00 ig2=0b0 Rm:u=0bxxxxx ig3=0b0 len:u=0b11 op:u=0b0 ig4=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'len == 0b11',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TBL no-reg-for-table=tbl4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdtbl page=TBL_advsimd cclass=TBL_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|TABLE_LOOKUP',
};

ENCODING TBL_asimdtbl_L1_1 => {
  name       => 'Single register table',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 op2=0b00 ig2=0b0 Rm:u=0bxxxxx ig3=0b0 len:u=0b00 op:u=0b0 ig4=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'len == 0b00',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TBL no-reg-for-table=tbl1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdtbl page=TBL_advsimd cclass=TBL_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|TABLE_LOOKUP',
};

ENCODING TBX_asimdtbl_L2_2 => {
  name       => 'Two register table',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 op2=0b00 ig2=0b0 Rm:u=0bxxxxx ig3=0b0 len:u=0b01 op:u=0b1 ig4=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'len == 0b01',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TBX no-reg-for-table=tbl2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdtbl page=TBX_advsimd cclass=TBX_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|TABLE_LOOKUP',
};

ENCODING TBX_asimdtbl_L3_3 => {
  name       => 'Three register table',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 op2=0b00 ig2=0b0 Rm:u=0bxxxxx ig3=0b0 len:u=0b10 op:u=0b1 ig4=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'len == 0b10',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TBX no-reg-for-table=tbl3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdtbl page=TBX_advsimd cclass=TBX_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|TABLE_LOOKUP',
};

ENCODING TBX_asimdtbl_L4_4 => {
  name       => 'Four register table',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 op2=0b00 ig2=0b0 Rm:u=0bxxxxx ig3=0b0 len:u=0b11 op:u=0b1 ig4=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'len == 0b11',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TBX no-reg-for-table=tbl4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdtbl page=TBX_advsimd cclass=TBX_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|TABLE_LOOKUP',
};

ENCODING TBX_asimdtbl_L1_1 => {
  name       => 'Single register table',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 op2=0b00 ig2=0b0 Rm:u=0bxxxxx ig3=0b0 len:u=0b00 op:u=0b1 ig4=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'len == 0b00',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TBX no-reg-for-table=tbl1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdtbl page=TBX_advsimd cclass=TBX_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|TABLE_LOOKUP',
};

ENCODING TRN1_asimdperm_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 op:u=0b0 ig4=0b10 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TRN1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdperm page=TRN1_advsimd cclass=TRN1_asimdperm_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING TRN2_asimdperm_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 op:u=0b1 ig4=0b10 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TRN2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdperm page=TRN2_advsimd cclass=TRN2_asimdperm_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING UABA_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0111 ac:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UABA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UABA_advsimd cclass=UABA_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|UNSIGNED',
};

ENCODING UABAL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b0 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UABAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=UABAL_advsimd cclass=UABAL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|UNSIGNED',
};

ENCODING UABD_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0111 ac:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UABD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UABD_advsimd cclass=UABD_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|UNSIGNED',
};

ENCODING UABDL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b1 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UABDL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=UABDL_advsimd cclass=UABDL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|UNSIGNED',
};

ENCODING UADALP_asimdmisc_P => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b00 op:u=0b1 ig4=0b10 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UADALP',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=UADALP_advsimd cclass=UADALP_asimdmisc_P',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|UNSIGNED',
};

ENCODING UADDL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b00 o1:u=0b0 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UADDL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=UADDL_advsimd cclass=UADDL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|UNSIGNED',
};

ENCODING UADDLP_asimdmisc_P => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b00 op:u=0b0 ig4=0b10 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UADDLP',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=UADDLP_advsimd cclass=UADDLP_asimdmisc_P',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|UNSIGNED',
};

ENCODING UADDLV_asimdall_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b11000 opcode=0b00011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UADDLV',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=UADDLV_advsimd cclass=UADDLV_asimdall_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|UNSIGNED',
};

ENCODING UADDW_asimddiff_W => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b00 o1:u=0b0 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UADDW',
  docvars2   => 'shape=wide',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=UADDW_advsimd cclass=UADDW_asimddiff_W',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|UNSIGNED',
};

ENCODING UCVTF_asisdshf_C => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx opcode=0b11100 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=UCVTF_advsimd_fix cclass=UCVTF_asisdshf_C',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING UCVTF_asimdshf_C => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b11100 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=UCVTF_advsimd_fix cclass=UCVTF_asimdshf_C',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING UCVTF_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 a=0b0 ig2=0b111100 opcode=0b11101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=UCVTF_advsimd_int cclass=UCVTF_asisdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING UCVTF_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b11101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=UCVTF_advsimd_int cclass=UCVTF_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING UCVTF_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a=0b0 ig2=0b111100 opcode=0b11101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=UCVTF_advsimd_int cclass=UCVTF_asimdmiscfp16_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING UCVTF_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b11101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=UCVTF_advsimd_int cclass=UCVTF_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION|UNSIGNED',
};

ENCODING UCVTF_H32_float2fix => {
  name       => '32-bit to half-precision',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b0 rmode:u=0b00 opcode:u=0b011 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11',
  docvars    => 'convert-type=fix32-to-half instr-class=float isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=UCVTF_float_fix cclass=UCVTF_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING UCVTF_S32_float2fix => {
  name       => '32-bit to single-precision',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b0 rmode:u=0b00 opcode:u=0b011 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00',
  docvars    => 'convert-type=fix32-to-single instr-class=float isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=UCVTF_float_fix cclass=UCVTF_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING UCVTF_D32_float2fix => {
  name       => '32-bit to double-precision',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b0 rmode:u=0b00 opcode:u=0b011 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b01',
  docvars    => 'convert-type=fix32-to-double instr-class=float isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=UCVTF_float_fix cclass=UCVTF_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING UCVTF_H64_float2fix => {
  name       => '64-bit to half-precision',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b0 rmode:u=0b00 opcode:u=0b011 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11',
  docvars    => 'convert-type=fix64-to-half instr-class=float isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=UCVTF_float_fix cclass=UCVTF_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING UCVTF_S64_float2fix => {
  name       => '64-bit to single-precision',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b0 rmode:u=0b00 opcode:u=0b011 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b00',
  docvars    => 'convert-type=fix64-to-single instr-class=float isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=UCVTF_float_fix cclass=UCVTF_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING UCVTF_D64_float2fix => {
  name       => '64-bit to double-precision',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b0 rmode:u=0b00 opcode:u=0b011 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01',
  docvars    => 'convert-type=fix64-to-double instr-class=float isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=UCVTF_float_fix cclass=UCVTF_float_fix_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING UCVTF_H32_float2int => {
  name       => '32-bit to half-precision',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b00 opcode:u=0b011 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b11',
  docvars    => 'convert-type=32-to-half instr-class=float isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=UCVTF_float_int cclass=UCVTF_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING UCVTF_S32_float2int => {
  name       => '32-bit to single-precision',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b00 opcode:u=0b011 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b00',
  docvars    => 'convert-type=32-to-single instr-class=float isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=UCVTF_float_int cclass=UCVTF_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING UCVTF_D32_float2int => {
  name       => '32-bit to double-precision',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b00 opcode:u=0b011 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && ftype == 0b01',
  docvars    => 'convert-type=32-to-double instr-class=float isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=UCVTF_float_int cclass=UCVTF_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING UCVTF_H64_float2int => {
  name       => '64-bit to half-precision',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b11 ig2=0b1 rmode:u=0b00 opcode:u=0b011 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b11',
  docvars    => 'convert-type=64-to-half instr-class=float isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=UCVTF_float_int cclass=UCVTF_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING UCVTF_S64_float2int => {
  name       => '64-bit to single-precision',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b00 ig2=0b1 rmode:u=0b00 opcode:u=0b011 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b00',
  docvars    => 'convert-type=64-to-single instr-class=float isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=UCVTF_float_int cclass=UCVTF_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING UCVTF_D64_float2int => {
  name       => '64-bit to double-precision',
  diagram    => 'sf:u=0b1 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b00 opcode:u=0b011 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && ftype == 0b01',
  docvars    => 'convert-type=64-to-double instr-class=float isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=UCVTF_float_int cclass=UCVTF_float_int_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION|UNSIGNED',
};

ENCODING UDOT_asimdelem_D => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1110 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=UDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=UDOT_advsimd_elt cclass=UDOT_asimdelem_D',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|UNSIGNED',
};

ENCODING UDOT_asimdsame2_D => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 opcode=0b0010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=UDOT_advsimd_vec cclass=UDOT_asimdsame2_D',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|UNSIGNED',
};

ENCODING UHADD_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UHADD',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UHADD_advsimd cclass=UHADD_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING|UNSIGNED',
};

ENCODING UHSUB_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00100 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UHSUB',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UHSUB_advsimd cclass=UHSUB_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING|UNSIGNED',
};

ENCODING UMAX_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0110 o1:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMAX',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UMAX_advsimd cclass=UMAX_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX|UNSIGNED',
};

ENCODING UMAXP_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b1010 o1:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMAXP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UMAXP_advsimd cclass=UMAXP_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX|UNSIGNED',
};

ENCODING UMAXV_asimdall_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b11000 op:u=0b0 ig3=0b1010 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UMAXV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=UMAXV_advsimd cclass=UMAXV_asimdall_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX|UNSIGNED',
};

ENCODING UMIN_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0110 o1:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMIN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UMIN_advsimd cclass=UMIN_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX|UNSIGNED',
};

ENCODING UMINP_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b1010 o1:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMINP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UMINP_advsimd cclass=UMINP_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX|UNSIGNED',
};

ENCODING UMINV_asimdall_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b11000 op:u=0b1 ig3=0b1010 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UMINV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=UMINV_advsimd cclass=UMINV_asimdall_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX|UNSIGNED',
};

ENCODING UMLAL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b0 ig3=0b10 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=UMLAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=UMLAL_advsimd_elt cclass=UMLAL_asimdelem_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|UNSIGNED',
};

ENCODING UMLAL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b10 o1:u=0b0 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMLAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=UMLAL_advsimd_vec cclass=UMLAL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|UNSIGNED',
};

ENCODING UMLSL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b1 ig3=0b10 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=UMLSL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=UMLSL_advsimd_elt cclass=UMLSL_asimdelem_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|UNSIGNED',
};

ENCODING UMLSL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b10 o1:u=0b1 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMLSL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=UMLSL_advsimd_vec cclass=UMLSL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|UNSIGNED',
};

ENCODING UMMLA_asimdsame2_G => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q=0b1 U:u=0b1 ig1=0b01110 size=0b10 ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b010 B:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMMLA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=UMMLA_advsimd_vec cclass=UMMLA_asimdsame2_G',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|UNSIGNED',
};

ENCODING UMOV_asimdins_W_w => {
  name       => '32-bit',
  diagram    => 'ig0=0b0 Q:u=0b0 op=0b0 ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4=0b0111 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'datatype=32 instr-class=advsimd isa=A64 mnemonic=UMOV vector-xfer-type=general-from-element',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD datasize=32',
  tags       => 'group=simd_dp iclass=asimdins page=UMOV_advsimd cclass=UMOV_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|UNSIGNED',
};

ENCODING UMOV_asimdins_X_x => {
  name       => '64-reg,UMOV-64-reg',
  diagram    => 'ig0=0b0 Q:u=0b1 op=0b0 ig1=0b01110000 imm5:u=0bx1000 ig2=0b0 imm4=0b0111 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'Q == 0b1 && imm5 == 0bx1000',
  docvars    => 'atomic-ops=UMOV-64-reg instr-class=advsimd isa=A64 mnemonic=UMOV reg-type=64-reg vector-xfer-type=general-from-element',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=UMOV_advsimd cclass=UMOV_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|UNSIGNED',
};

ENCODING UMULL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1010 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=UMULL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=UMULL_advsimd_elt cclass=UMULL_asimdelem_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|UNSIGNED',
};

ENCODING UMULL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b1100 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMULL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=UMULL_advsimd_vec cclass=UMULL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|UNSIGNED',
};

ENCODING UQADD_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UQADD',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=UQADD_advsimd cclass=UQADD_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING UQADD_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UQADD',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UQADD_advsimd cclass=UQADD_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING UQRSHL_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b1 S:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UQRSHL',
  docvars2   => 'rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=UQRSHL_advsimd cclass=UQRSHL_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|ROUNDING|SATURATING|UNSIGNED',
};

ENCODING UQRSHL_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b1 S:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UQRSHL',
  docvars2   => 'rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UQRSHL_advsimd cclass=UQRSHL_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|ROUNDING|SATURATING|UNSIGNED',
};

ENCODING UQRSHRN_asisdshf_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1001 op:u=0b1 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UQRSHRN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=UQRSHRN_advsimd cclass=UQRSHRN_asisdshf_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|SATURATING|UNSIGNED',
};

ENCODING UQRSHRN_asimdshf_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1001 op:u=0b1 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UQRSHRN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=UQRSHRN_advsimd cclass=UQRSHRN_asimdshf_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|SATURATING|UNSIGNED',
};

ENCODING UQSHL_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b011 op:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UQSHL',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=UQSHL_advsimd_imm cclass=UQSHL_asisdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING|UNSIGNED',
};

ENCODING UQSHL_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b011 op:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UQSHL',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=UQSHL_advsimd_imm cclass=UQSHL_asimdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING|UNSIGNED',
};

ENCODING UQSHL_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b0 S:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UQSHL',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=UQSHL_advsimd_reg cclass=UQSHL_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING|UNSIGNED',
};

ENCODING UQSHL_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b0 S:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UQSHL',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UQSHL_advsimd_reg cclass=UQSHL_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING|UNSIGNED',
};

ENCODING UQSHRN_asisdshf_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1001 op:u=0b0 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UQSHRN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=UQSHRN_advsimd cclass=UQSHRN_asisdshf_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|SATURATING|UNSIGNED',
};

ENCODING UQSHRN_asimdshf_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1001 op:u=0b0 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UQSHRN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=UQSHRN_advsimd cclass=UQSHRN_asimdshf_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|SATURATING|UNSIGNED',
};

ENCODING UQSUB_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00101 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UQSUB',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=UQSUB_advsimd cclass=UQSUB_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING UQSUB_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00101 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UQSUB',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UQSUB_advsimd cclass=UQSUB_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING UQXTN_asisdmisc_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b10100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UQXTN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=UQXTN_advsimd cclass=UQXTN_asisdmisc_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|EXTRACTION|SATURATING|UNSIGNED',
};

ENCODING UQXTN_asimdmisc_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b10100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UQXTN',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=UQXTN_advsimd cclass=UQXTN_asimdmisc_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|EXTRACTION|SATURATING|UNSIGNED',
};

ENCODING URECPE_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b11100 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=URECPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=URECPE_advsimd cclass=URECPE_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|UNSIGNED',
};

ENCODING URHADD_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00010 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=URHADD',
  docvars2   => 'halving=1 rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=URHADD_advsimd cclass=URHADD_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING|ROUNDING|UNSIGNED',
};

ENCODING URSHL_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b1 S:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=URSHL',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=URSHL_advsimd cclass=URSHL_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|ROUNDING|UNSIGNED',
};

ENCODING URSHL_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b1 S:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=URSHL',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=URSHL_advsimd cclass=URSHL_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|ROUNDING|UNSIGNED',
};

ENCODING URSHR_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b1 o0:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=URSHR',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=URSHR_advsimd cclass=URSHR_asisdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|UNSIGNED',
};

ENCODING URSHR_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b1 o0:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=URSHR',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=URSHR_advsimd cclass=URSHR_asimdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|UNSIGNED',
};

ENCODING URSQRTE_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b11100 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=URSQRTE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=URSQRTE_advsimd cclass=URSQRTE_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|UNSIGNED',
};

ENCODING URSRA_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b1 o0:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=URSRA',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=URSRA_advsimd cclass=URSRA_asisdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|CARITHMETIC|ROUNDING|UNSIGNED',
};

ENCODING URSRA_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b1 o0:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=URSRA',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=URSRA_advsimd cclass=URSRA_asimdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|CARITHMETIC|ROUNDING|UNSIGNED',
};

ENCODING USDOT_asimdelem_D => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 US:u=0b1 ig2=0b0 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1111 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=USDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=USDOT_advsimd_elt cclass=USDOT_asimdelem_D',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING USDOT_asimdsame2_D => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size=0b10 ig2=0b0 Rm:u=0bxxxxx ig3=0b1 opcode=0b0011 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=USDOT_advsimd_vec cclass=USDOT_asimdsame2_D',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING USHL_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b0 S:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=USHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=USHL_advsimd cclass=USHL_asisdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|UNSIGNED',
};

ENCODING USHL_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b0 S:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=USHL_advsimd cclass=USHL_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|UNSIGNED',
};

ENCODING USHLL_asimdshf_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b10100 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USHLL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=USHLL_advsimd cclass=USHLL_advsimd_USHLL_asimdshf_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|UNSIGNED',
};

ENCODING USHR_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b0 o0:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=USHR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=USHR_advsimd cclass=USHR_asisdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|UNSIGNED',
};

ENCODING USHR_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b0 o0:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USHR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=USHR_advsimd cclass=USHR_asimdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|UNSIGNED',
};

ENCODING USMMLA_asimdsame2_G => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q=0b1 U:u=0b0 ig1=0b01110 size=0b10 ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b010 B:u=0b1 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USMMLA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=USMMLA_advsimd_vec cclass=USMMLA_asimdsame2_G',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING USQADD_asisdmisc_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b00011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=USQADD',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=USQADD_advsimd cclass=USQADD_asisdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING',
};

ENCODING USQADD_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b00011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USQADD',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=USQADD_advsimd cclass=USQADD_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING',
};

ENCODING USRA_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b0 o0:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=USRA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=USRA_advsimd cclass=USRA_asisdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|CARITHMETIC|UNSIGNED',
};

ENCODING USRA_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b0 o0:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USRA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=USRA_advsimd cclass=USRA_asimdshf_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|CARITHMETIC|UNSIGNED',
};

ENCODING USUBL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b00 o1:u=0b1 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USUBL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=USUBL_advsimd cclass=USUBL_asimddiff_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|UNSIGNED',
};

ENCODING USUBW_asimddiff_W => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b00 o1:u=0b1 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USUBW',
  docvars2   => 'shape=wide',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=USUBW_advsimd cclass=USUBW_asimddiff_W',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|UNSIGNED',
};

ENCODING UZP1_asimdperm_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 op:u=0b0 ig4=0b01 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UZP1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdperm page=UZP1_advsimd cclass=UZP1_asimdperm_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING UZP2_asimdperm_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 op:u=0b1 ig4=0b01 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UZP2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdperm page=UZP2_advsimd cclass=UZP2_asimdperm_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING XAR_VVV2_crypto3_imm6 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110100 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=XAR',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=crypto3_imm6 page=XAR_advsimd cclass=XAR_VVV2_crypto3_imm6',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING XTN_asimdmisc_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b10010 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=XTN',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=XTN_advsimd cclass=XTN_asimdmisc_N',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|EXTRACTION',
};

ENCODING ZIP1_asimdperm_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 op:u=0b0 ig4=0b11 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ZIP1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdperm page=ZIP1_advsimd cclass=ZIP1_asimdperm_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING ZIP2_asimdperm_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 op:u=0b1 ig4=0b11 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ZIP2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdperm page=ZIP2_advsimd cclass=ZIP2_asimdperm_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING MOV_DUP_asisdone_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 op=0b0 ig1=0b11110000 imm5:u=0bxxxxx ig2=0b0 imm4=0b0000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MOV instr-class=advsimd isa=A64 mnemonic=DUP vector-xfer-type=scalar-from-element',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=DUP_asisdone_only alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdone page=MOV_DUP_advsimd_elt cclass=MOV_DUP_advsimd_elt_DUP_asisdone_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
  eq_to      => 'DUP  <V><d>, <Vn>.<T>[<index>]',
};

ENCODING MOV_INS_asimdins_IV_v => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q=0b1 op=0b1 ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4:u=0bxxxx ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MOV instr-class=advsimd isa=A64 mnemonic=INS vector-xfer-type=element-from-element',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=INS_asimdins_IV_v alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=MOV_INS_advsimd_elt cclass=MOV_INS_advsimd_elt_INS_asimdins_IV_v',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
  eq_to      => 'INS  <Vd>.<Ts>[<index1>], <Vn>.<Ts>[<index2>]',
};

ENCODING MOV_INS_asimdins_IR_r => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q=0b1 op=0b0 ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4=0b0011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MOV instr-class=advsimd isa=A64 mnemonic=INS vector-xfer-type=vector-from-general',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=INS_asimdins_IR_r alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=MOV_INS_advsimd_gen cclass=MOV_INS_advsimd_gen_INS_asimdins_IR_r',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
  eq_to      => 'INS  <Vd>.<Ts>[<index>], <R><n>',
};

ENCODING MOV_ORR_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0b10 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  aliascond  => 'Rm == Rn',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd alias_mnemonic=MOV instr-class=advsimd isa=A64 mnemonic=ORR vector-xfer-type=vector-from-vector',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ORR_asimdsame_only alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=MOV_ORR_advsimd_reg cclass=MOV_ORR_advsimd_reg_ORR_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
  eq_to      => 'ORR  <Vd>.<T>, <Vn>.<T>, <Vn>.<T>',
};

ENCODING MOV_UMOV_asimdins_W_w => {
  name       => '32-bit',
  diagram    => 'ig0=0b0 Q:u=0b0 op=0b0 ig1=0b01110000 imm5:u=0bxx100 ig2=0b0 imm4=0b0111 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'Q == 0b0 && imm5 == 0bxx100',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MOV datatype=32 instr-class=advsimd isa=A64 mnemonic=UMOV vector-xfer-type=general-from-element',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=UMOV_asimdins_W_w alphaindex=FPSIMD datasize=32',
  tags       => 'group=simd_dp iclass=asimdins page=MOV_UMOV_advsimd cclass=MOV_UMOV_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
  eq_to      => 'UMOV <Wd>, <Vn>.S[<index>]',
};

ENCODING MOV_UMOV_asimdins_X_x => {
  name       => '64-reg,UMOV-64-reg',
  diagram    => 'ig0=0b0 Q:u=0b1 op=0b0 ig1=0b01110000 imm5:u=0bx1000 ig2=0b0 imm4=0b0111 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'Q == 0b1 && imm5 == 0bx1000',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MOV atomic-ops=UMOV-64-reg instr-class=advsimd isa=A64 mnemonic=UMOV reg-type=64-reg vector-xfer-type=general-from-element',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=UMOV_asimdins_X_x alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=MOV_UMOV_advsimd cclass=MOV_UMOV_advsimd_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
  eq_to      => 'UMOV <Xd>, <Vn>.D[<index>]',
};

ENCODING MVN_NOT_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size=0b00 ig2=0b10000 opcode=0b00101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'advsimd-type=simd alias_mnemonic=MVN instr-class=advsimd isa=A64 mnemonic=NOT',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=NOT_asimdmisc_R alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=MVN_NOT_advsimd cclass=MVN_NOT_advsimd_NOT_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
  eq_to      => 'NOT  <Vd>.<T>, <Vn>.<T>',
};

ENCODING SXTL_SSHLL_asimdshf_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0b000 opcode=0b10100 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  aliascond  => 'BitCount(immh) == 1',
  docvars    => 'advsimd-type=simd alias_mnemonic=SXTL instr-class=advsimd isa=A64 mnemonic=SSHLL',
  docvars2   => 'shape=long',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SSHLL_asimdshf_L alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SXTL_SSHLL_advsimd cclass=SXTL_SSHLL_advsimd_SSHLL_asimdshf_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SIGN_EXTEND|SIGNED',
  eq_to      => 'SSHLL{2}  <Vd>.<Ta>, <Vn>.<Tb>, #0',
};

ENCODING UXTL_USHLL_asimdshf_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0b000 opcode=0b10100 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  aliascond  => 'BitCount(immh) == 1',
  docvars    => 'advsimd-type=simd alias_mnemonic=UXTL instr-class=advsimd isa=A64 mnemonic=USHLL',
  docvars2   => 'shape=long',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=USHLL_asimdshf_L alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=UXTL_USHLL_advsimd cclass=UXTL_USHLL_advsimd_USHLL_asimdshf_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SIGN_EXTEND|UNSIGNED',
  eq_to      => 'USHLL{2}  <Vd>.<Ta>, <Vn>.<Tb>, #0',
};


1;

