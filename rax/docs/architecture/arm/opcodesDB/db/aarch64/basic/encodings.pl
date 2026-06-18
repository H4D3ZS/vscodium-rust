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

ENCODING ADC_32_addsub_carry => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S:u=0b0 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=no-s datatype=32 instr-class=general isa=A64 mnemonic=ADC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_carry page=ADC cclass=ADC_no_s',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADC_64_addsub_carry => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S:u=0b0 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=no-s datatype=64 instr-class=general isa=A64 mnemonic=ADC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_carry page=ADC cclass=ADC_no_s',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADCS_32_addsub_carry => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S:u=0b1 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=S datatype=32 instr-class=general isa=A64 mnemonic=ADCS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_carry page=ADCS cclass=ADCS_s',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADCS_64_addsub_carry => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S:u=0b1 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=S datatype=64 instr-class=general isa=A64 mnemonic=ADCS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_carry page=ADCS cclass=ADCS_s',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADD_32_addsub_ext => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S:u=0b0 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=no-s datatype=32 instr-class=general isa=A64 mnemonic=ADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_ext page=ADD_addsub_ext cclass=ADD_addsub_ext_no_s',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_64_addsub_ext => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S:u=0b0 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=no-s datatype=64 instr-class=general isa=A64 mnemonic=ADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_ext page=ADD_addsub_ext cclass=ADD_addsub_ext_no_s',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_32_addsub_imm => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S:u=0b0 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=no-s datatype=32 immediate-type=imm12u instr-class=general isa=A64 mnemonic=ADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=addsub_imm page=ADD_addsub_imm cclass=ADD_addsub_imm_no_s',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_64_addsub_imm => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S:u=0b0 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=no-s datatype=64 immediate-type=imm12u instr-class=general isa=A64 mnemonic=ADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=addsub_imm page=ADD_addsub_imm cclass=ADD_addsub_imm_no_s',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_32_addsub_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S:u=0b0 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=no-s datatype=32 instr-class=general isa=A64 mnemonic=ADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_shift page=ADD_addsub_shift cclass=ADD_addsub_shift_no_s',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_64_addsub_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S:u=0b0 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=no-s datatype=64 instr-class=general isa=A64 mnemonic=ADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_shift page=ADD_addsub_shift cclass=ADD_addsub_shift_no_s',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDG_64_addsub_immtags => {
  name       => 'Integer',
  diagram    => 'sf=0b1 op=0b0 S=0b0 ig0=0b100011 o2=0b0 uimm6:u=0bxxxxxx op3:u=0bZZ uimm4:u=0bxxxx Xn:u=0bxxxxx Xd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=ADDG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=addsub_immtags page=ADDG cclass=ADDG_64_addsub_immtags',
  categories => 'GENERAL|ARITHMETIC|TAGS',
};

ENCODING ADDS_32S_addsub_ext => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S:u=0b1 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=S datatype=32 instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_ext page=ADDS_addsub_ext cclass=ADDS_addsub_ext_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_64S_addsub_ext => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S:u=0b1 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=S datatype=64 instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_ext page=ADDS_addsub_ext cclass=ADDS_addsub_ext_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_32S_addsub_imm => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S:u=0b1 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=S datatype=32 immediate-type=imm12u instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=addsub_imm page=ADDS_addsub_imm cclass=ADDS_addsub_imm_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_64S_addsub_imm => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S:u=0b1 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=S datatype=64 immediate-type=imm12u instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=addsub_imm page=ADDS_addsub_imm cclass=ADDS_addsub_imm_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_32_addsub_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S:u=0b1 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=S datatype=32 instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_shift page=ADDS_addsub_shift cclass=ADDS_addsub_shift_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_64_addsub_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S:u=0b1 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=S datatype=64 instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_shift page=ADDS_addsub_shift cclass=ADDS_addsub_shift_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADR_only_pcreladdr => {
  name       => 'Literal',
  diagram    => 'op:u=0b0 immlo:u=0bxx ig0=0b10000 immhi:u=0bxxxxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  docvars    => 'address-form=literal instr-class=general isa=A64 mnemonic=ADR offset-type=off19s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=pcreladdr page=ADR cclass=ADR_only_pcreladdr',
  categories => 'GENERAL|MISC',
};

ENCODING ADRP_only_pcreladdr => {
  name       => 'Literal',
  diagram    => 'op:u=0b1 immlo:u=0bxx ig0=0b10000 immhi:u=0bxxxxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  docvars    => 'address-form=literal instr-class=general isa=A64 mnemonic=ADRP offset-type=off19s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=pcreladdr page=ADRP cclass=ADRP_only_pcreladdr',
  categories => 'GENERAL|MISC',
};

ENCODING AND_32_log_imm => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b00 ig0=0b100100 N:u=0b0 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  docvars    => 'cond-setting=no-s datatype=32 immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=AND',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=log_imm page=AND_log_imm cclass=AND_log_imm_no_s',
  categories => 'GENERAL|BITWISE',
};

ENCODING AND_64_log_imm => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b00 ig0=0b100100 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=no-s datatype=64 immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=AND',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=log_imm page=AND_log_imm cclass=AND_log_imm_no_s',
  categories => 'GENERAL|BITWISE',
};

ENCODING AND_32_log_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b00 ig0=0b01010 shift:u=0bxx N:u=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=no-s datatype=32 datatype-reguse=32-shifted-reg instr-class=general isa=A64 mnemonic=AND reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=log_shift page=AND_log_shift cclass=AND_log_shift_no_s',
  categories => 'GENERAL|BITWISE',
};

ENCODING AND_64_log_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b00 ig0=0b01010 shift:u=0bxx N:u=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=no-s datatype=64 datatype-reguse=64-shifted-reg instr-class=general isa=A64 mnemonic=AND reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=log_shift page=AND_log_shift cclass=AND_log_shift_no_s',
  categories => 'GENERAL|BITWISE',
};

ENCODING ANDS_32S_log_imm => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b11 ig0=0b100100 N:u=0b0 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  docvars    => 'cond-setting=S datatype=32 immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=ANDS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=log_imm page=ANDS_log_imm cclass=ANDS_log_imm_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING ANDS_64S_log_imm => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b11 ig0=0b100100 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=S datatype=64 immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=ANDS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=log_imm page=ANDS_log_imm cclass=ANDS_log_imm_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING ANDS_32_log_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b11 ig0=0b01010 shift:u=0bxx N:u=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=S datatype=32 datatype-reguse=32-shifted-reg instr-class=general isa=A64 mnemonic=ANDS reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=log_shift page=ANDS_log_shift cclass=ANDS_log_shift_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING ANDS_64_log_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b11 ig0=0b01010 shift:u=0bxx N:u=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=S datatype=64 datatype-reguse=64-shifted-reg instr-class=general isa=A64 mnemonic=ANDS reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=log_shift page=ANDS_log_shift cclass=ANDS_log_shift_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING ASRV_32_dp_2src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=ASRV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_2src page=ASRV cclass=ASRV_general',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
};

ENCODING ASRV_64_dp_2src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=ASRV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_2src page=ASRV cclass=ASRV_general',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
};

ENCODING AUTDA_64P_dp_1src => {
  name       => 'AUTDA',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0b0 opcode[2:0]=0b110 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'Z == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=AUTDA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=AUTDA cclass=AUTDA_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING AUTDZA_64Z_dp_1src => {
  name       => 'AUTDZA',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0b1 opcode[2:0]=0b110 Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'Z == 0b1 && Rn == 0b11111',
  docvars    => 'instr-class=general isa=A64 mnemonic=AUTDZA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=AUTDA cclass=AUTDA_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING AUTDB_64P_dp_1src => {
  name       => 'AUTDB',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0b0 opcode[2:0]=0b111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'Z == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=AUTDB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=AUTDB cclass=AUTDB_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING AUTDZB_64Z_dp_1src => {
  name       => 'AUTDZB',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0b1 opcode[2:0]=0b111 Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'Z == 0b1 && Rn == 0b11111',
  docvars    => 'instr-class=general isa=A64 mnemonic=AUTDZB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=AUTDB cclass=AUTDB_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING AUTIA_64P_dp_1src => {
  name       => 'AUTIA',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0b0 opcode[2:0]=0b100 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'Z == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=AUTIA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=AUTIA cclass=AUTIA_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING AUTIZA_64Z_dp_1src => {
  name       => 'AUTIZA',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0b1 opcode[2:0]=0b100 Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'Z == 0b1 && Rn == 0b11111',
  docvars    => 'instr-class=general isa=A64 mnemonic=AUTIZA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=AUTIA cclass=AUTIA_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING AUTIA1716_HI_hints => {
  name       => 'AUTIA1716',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0001 op2:u=0b100 Rt=0b11111',
  bitdiffs   => 'CRm == 0b0001 && op2 == 0b100',
  docvars    => 'instr-class=system isa=A64 mnemonic=AUTIA1716',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=AUTIA cclass=AUTIA_system',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'SYSTEM|PAC',
};

ENCODING AUTIASP_HI_hints => {
  name       => 'AUTIASP',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0011 op2:u=0b101 Rt=0b11111',
  bitdiffs   => 'CRm == 0b0011 && op2 == 0b101',
  docvars    => 'instr-class=system isa=A64 mnemonic=AUTIASP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=AUTIA cclass=AUTIA_system',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'SYSTEM|PAC',
};

ENCODING AUTIAZ_HI_hints => {
  name       => 'AUTIAZ',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0011 op2:u=0b100 Rt=0b11111',
  bitdiffs   => 'CRm == 0b0011 && op2 == 0b100',
  docvars    => 'instr-class=system isa=A64 mnemonic=AUTIAZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=AUTIA cclass=AUTIA_system',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'SYSTEM|PAC',
};

ENCODING AUTIB_64P_dp_1src => {
  name       => 'AUTIB',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0b0 opcode[2:0]=0b101 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'Z == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=AUTIB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=AUTIB cclass=AUTIB_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING AUTIZB_64Z_dp_1src => {
  name       => 'AUTIZB',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0b1 opcode[2:0]=0b101 Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'Z == 0b1 && Rn == 0b11111',
  docvars    => 'instr-class=general isa=A64 mnemonic=AUTIZB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=AUTIB cclass=AUTIB_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING AUTIB1716_HI_hints => {
  name       => 'AUTIB1716',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0001 op2:u=0b110 Rt=0b11111',
  bitdiffs   => 'CRm == 0b0001 && op2 == 0b110',
  docvars    => 'instr-class=system isa=A64 mnemonic=AUTIB1716',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=AUTIB cclass=AUTIB_system',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'SYSTEM|PAC',
};

ENCODING AUTIBSP_HI_hints => {
  name       => 'AUTIBSP',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0011 op2:u=0b111 Rt=0b11111',
  bitdiffs   => 'CRm == 0b0011 && op2 == 0b111',
  docvars    => 'instr-class=system isa=A64 mnemonic=AUTIBSP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=AUTIB cclass=AUTIB_system',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'SYSTEM|PAC',
};

ENCODING AUTIBZ_HI_hints => {
  name       => 'AUTIBZ',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0011 op2:u=0b110 Rt=0b11111',
  bitdiffs   => 'CRm == 0b0011 && op2 == 0b110',
  docvars    => 'instr-class=system isa=A64 mnemonic=AUTIBZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=AUTIB cclass=AUTIB_system',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'SYSTEM|PAC',
};

ENCODING AXFLAG_M_pstate => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b000 CRn=0b0100 CRm:u=0bZZZZ op2=0b010 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=AXFLAG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=control iclass=pstate page=AXFLAG cclass=AXFLAG_M_pstate',
  pstate     => 'C=R N=U V=R Z=R',
  categories => 'SYSTEM|CONVERSION',
};

ENCODING B_only_condbranch => {
  name       => '19-bit signed PC-relative branch offset',
  diagram    => 'ig0=0b0101010 o1=0b0 imm19:u=0bxxxxxxxxxxxxxxxxxxx o0=0b0 cond:u=0bxxxx',
  docvars    => 'branch-offset=br19 compare-with=cmp-cond instr-class=general isa=A64 mnemonic=B',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=condbranch page=B_cond cclass=B_only_condbranch',
  categories => 'GENERAL|BRANCH|CONDITIONALLY',
};

ENCODING B_only_branch_imm => {
  name       => '26-bit signed PC-relative branch offset',
  diagram    => 'op:u=0b0 ig0=0b00101 imm26:u=0bxxxxxxxxxxxxxxxxxxxxxxxxxx',
  docvars    => 'branch-offset=br26 instr-class=general isa=A64 mnemonic=B',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_imm page=B_uncond cclass=B_only_branch_imm',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING BFM_32M_bitfield => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b01 ig0=0b100110 N:u=0b0 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  docvars    => 'bitfield-fill=nofill datatype=32 instr-class=general isa=A64 mnemonic=BFM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=bitfield page=BFM cclass=BFM_nofill',
  categories => 'GENERAL|BITFIELD|DATA_TRANSFER',
};

ENCODING BFM_64M_bitfield => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b01 ig0=0b100110 N:u=0b1 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && N == 0b1',
  docvars    => 'bitfield-fill=nofill datatype=64 instr-class=general isa=A64 mnemonic=BFM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=bitfield page=BFM cclass=BFM_nofill',
  categories => 'GENERAL|BITFIELD|DATA_TRANSFER',
};

ENCODING BIC_32_log_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b00 ig0=0b01010 shift:u=0bxx N:u=0b1 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=no-s datatype=32 datatype-reguse=32-shifted-reg instr-class=general isa=A64 mnemonic=BIC reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=log_shift page=BIC_log_shift cclass=BIC_log_shift_no_s',
  categories => 'GENERAL|BITWISE',
};

ENCODING BIC_64_log_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b00 ig0=0b01010 shift:u=0bxx N:u=0b1 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=no-s datatype=64 datatype-reguse=64-shifted-reg instr-class=general isa=A64 mnemonic=BIC reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=log_shift page=BIC_log_shift cclass=BIC_log_shift_no_s',
  categories => 'GENERAL|BITWISE',
};

ENCODING BICS_32_log_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b11 ig0=0b01010 shift:u=0bxx N:u=0b1 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=S datatype=32 datatype-reguse=32-shifted-reg instr-class=general isa=A64 mnemonic=BICS reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=log_shift page=BICS cclass=BICS_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING BICS_64_log_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b11 ig0=0b01010 shift:u=0bxx N:u=0b1 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=S datatype=64 datatype-reguse=64-shifted-reg instr-class=general isa=A64 mnemonic=BICS reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=log_shift page=BICS cclass=BICS_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING BL_only_branch_imm => {
  name       => '26-bit signed PC-relative branch offset',
  diagram    => 'op:u=0b1 ig0=0b00101 imm26:u=0bxxxxxxxxxxxxxxxxxxxxxxxxxx',
  docvars    => 'branch-offset=br26 instr-class=general isa=A64 mnemonic=BL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_imm page=BL cclass=BL_only_branch_imm',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING BLR_64_branch_reg => {
  name       => 'Integer',
  diagram    => 'ig0=0b1101011 Z:u=0b0 opc[2:1]=0b0 op:u=0b01 op2=0b11111 op3[5:2]=0b0000 A:u=0b0 M:u=0b0 Rn:u=0bxxxxx Rm:u=0b00000',
  docvars    => 'instr-class=general isa=A64 mnemonic=BLR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=BLR cclass=BLR_64_branch_reg',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING BLRAAZ_64_branch_reg => {
  name       => 'Key A, zero modifier',
  diagram    => 'ig0=0b1101011 Z:u=0b0 opc[2:1]=0b0 op:u=0b01 op2=0b11111 op3[5:2]=0b0000 A:u=0b1 M:u=0b0 Rn:u=0bxxxxx Rm:u=0b11111',
  bitdiffs   => 'Z == 0b0 && M == 0b0 && Rm == 0b11111',
  docvars    => 'instr-class=general isa=A64 loadstore-bra=key-a-zmod mnemonic=BLRAAZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=BLRA cclass=BLRA_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY|PAC',
};

ENCODING BLRAA_64P_branch_reg => {
  name       => 'Key A, register modifier',
  diagram    => 'ig0=0b1101011 Z:u=0b1 opc[2:1]=0b0 op:u=0b01 op2=0b11111 op3[5:2]=0b0000 A:u=0b1 M:u=0b0 Rn:u=0bxxxxx Rm:u=0bxxxxx',
  bitdiffs   => 'Z == 0b1 && M == 0b0',
  docvars    => 'instr-class=general isa=A64 loadstore-bra=key-a-regmod mnemonic=BLRAA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=BLRA cclass=BLRA_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY|PAC',
};

ENCODING BLRABZ_64_branch_reg => {
  name       => 'Key B, zero modifier',
  diagram    => 'ig0=0b1101011 Z:u=0b0 opc[2:1]=0b0 op:u=0b01 op2=0b11111 op3[5:2]=0b0000 A:u=0b1 M:u=0b1 Rn:u=0bxxxxx Rm:u=0b11111',
  bitdiffs   => 'Z == 0b0 && M == 0b1 && Rm == 0b11111',
  docvars    => 'instr-class=general isa=A64 loadstore-bra=key-b-zmod mnemonic=BLRABZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=BLRA cclass=BLRA_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY|PAC',
};

ENCODING BLRAB_64P_branch_reg => {
  name       => 'Key B, register modifier',
  diagram    => 'ig0=0b1101011 Z:u=0b1 opc[2:1]=0b0 op:u=0b01 op2=0b11111 op3[5:2]=0b0000 A:u=0b1 M:u=0b1 Rn:u=0bxxxxx Rm:u=0bxxxxx',
  bitdiffs   => 'Z == 0b1 && M == 0b1',
  docvars    => 'instr-class=general isa=A64 loadstore-bra=key-b-regmod mnemonic=BLRAB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=BLRA cclass=BLRA_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY|PAC',
};

ENCODING BR_64_branch_reg => {
  name       => 'Integer',
  diagram    => 'ig0=0b1101011 Z:u=0b0 opc[2:1]=0b0 op:u=0b00 op2=0b11111 op3[5:2]=0b0000 A:u=0b0 M:u=0b0 Rn:u=0bxxxxx Rm:u=0b00000',
  docvars    => 'instr-class=general isa=A64 mnemonic=BR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=BR cclass=BR_64_branch_reg',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING BRAAZ_64_branch_reg => {
  name       => 'Key A, zero modifier',
  diagram    => 'ig0=0b1101011 Z:u=0b0 opc[2:1]=0b0 op:u=0b00 op2=0b11111 op3[5:2]=0b0000 A:u=0b1 M:u=0b0 Rn:u=0bxxxxx Rm:u=0b11111',
  bitdiffs   => 'Z == 0b0 && M == 0b0 && Rm == 0b11111',
  docvars    => 'instr-class=general isa=A64 loadstore-bra=key-a-zmod mnemonic=BRAAZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=BRA cclass=BRA_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY|PAC',
};

ENCODING BRAA_64P_branch_reg => {
  name       => 'Key A, register modifier',
  diagram    => 'ig0=0b1101011 Z:u=0b1 opc[2:1]=0b0 op:u=0b00 op2=0b11111 op3[5:2]=0b0000 A:u=0b1 M:u=0b0 Rn:u=0bxxxxx Rm:u=0bxxxxx',
  bitdiffs   => 'Z == 0b1 && M == 0b0',
  docvars    => 'instr-class=general isa=A64 loadstore-bra=key-a-regmod mnemonic=BRAA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=BRA cclass=BRA_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY|PAC',
};

ENCODING BRABZ_64_branch_reg => {
  name       => 'Key B, zero modifier',
  diagram    => 'ig0=0b1101011 Z:u=0b0 opc[2:1]=0b0 op:u=0b00 op2=0b11111 op3[5:2]=0b0000 A:u=0b1 M:u=0b1 Rn:u=0bxxxxx Rm:u=0b11111',
  bitdiffs   => 'Z == 0b0 && M == 0b1 && Rm == 0b11111',
  docvars    => 'instr-class=general isa=A64 loadstore-bra=key-b-zmod mnemonic=BRABZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=BRA cclass=BRA_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY|PAC',
};

ENCODING BRAB_64P_branch_reg => {
  name       => 'Key B, register modifier',
  diagram    => 'ig0=0b1101011 Z:u=0b1 opc[2:1]=0b0 op:u=0b00 op2=0b11111 op3[5:2]=0b0000 A:u=0b1 M:u=0b1 Rn:u=0bxxxxx Rm:u=0bxxxxx',
  bitdiffs   => 'Z == 0b1 && M == 0b1',
  docvars    => 'instr-class=general isa=A64 loadstore-bra=key-b-regmod mnemonic=BRAB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=BRA cclass=BRA_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY|PAC',
};

ENCODING BRK_EX_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b001 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL=0b00',
  docvars    => 'instr-class=system isa=A64 mnemonic=BRK',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=BRK cclass=BRK_EX_exception',
  exceptions => 'SOFTWAREBREAKPOINT',
  categories => 'SYSTEM|DEBUG',
};

ENCODING BTI_HB_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0100 op2:u=0bxx0 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=BTI',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=BTI cclass=BTI_HB_hints',
  exceptions => 'WFXTRAP',
  pstate     => 'EL=CR',
  categories => 'SYSTEM|MISC',
};

ENCODING CAS_C32_ldstexcl => {
  name       => '32-bit CAS',
  diagram    => 'size:u=0b10 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b0 Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && L == 0b0 && o0 == 0b0',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=CAS-32-reg instr-class=general isa=A64 mnemonic=CAS reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CAS cclass=CAS_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASA_C32_ldstexcl => {
  name       => '32-bit CASA',
  diagram    => 'size:u=0b10 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b1 Rs:u=0bxxxxx o0:u=0b0 Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && L == 0b1 && o0 == 0b0',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=CASA-32-reg instr-class=general isa=A64 mnemonic=CASA reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CAS cclass=CAS_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASAL_C32_ldstexcl => {
  name       => '32-bit CASAL',
  diagram    => 'size:u=0b10 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b1 Rs:u=0bxxxxx o0:u=0b1 Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && L == 0b1 && o0 == 0b1',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=CASAL-32-reg instr-class=general isa=A64 mnemonic=CASAL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CAS cclass=CAS_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASL_C32_ldstexcl => {
  name       => '32-bit CASL',
  diagram    => 'size:u=0b10 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b1 Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && L == 0b0 && o0 == 0b1',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=CASL-32-reg instr-class=general isa=A64 mnemonic=CASL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CAS cclass=CAS_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CAS_C64_ldstexcl => {
  name       => '64-bit CAS',
  diagram    => 'size:u=0b11 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b0 Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && L == 0b0 && o0 == 0b0',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-64-reg atomic-ops=CAS-64-reg instr-class=general isa=A64 mnemonic=CAS reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CAS cclass=CAS_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASA_C64_ldstexcl => {
  name       => '64-bit CASA',
  diagram    => 'size:u=0b11 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b1 Rs:u=0bxxxxx o0:u=0b0 Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && L == 0b1 && o0 == 0b0',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-64-reg atomic-ops=CASA-64-reg instr-class=general isa=A64 mnemonic=CASA reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CAS cclass=CAS_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASAL_C64_ldstexcl => {
  name       => '64-bit CASAL',
  diagram    => 'size:u=0b11 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b1 Rs:u=0bxxxxx o0:u=0b1 Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && L == 0b1 && o0 == 0b1',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-64-reg atomic-ops=CASAL-64-reg instr-class=general isa=A64 mnemonic=CASAL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CAS cclass=CAS_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASL_C64_ldstexcl => {
  name       => '64-bit CASL',
  diagram    => 'size:u=0b11 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b1 Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && L == 0b0 && o0 == 0b1',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-64-reg atomic-ops=CASL-64-reg instr-class=general isa=A64 mnemonic=CASL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CAS cclass=CAS_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASAB_C32_ldstexcl => {
  name       => 'CASAB',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b1 Rs:u=0bxxxxx o0:u=0b0 Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'L == 0b1 && o0 == 0b0',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=CASAB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASB cclass=CASB_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASALB_C32_ldstexcl => {
  name       => 'CASALB',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b1 Rs:u=0bxxxxx o0:u=0b1 Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'L == 0b1 && o0 == 0b1',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=CASALB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASB cclass=CASB_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASB_C32_ldstexcl => {
  name       => 'CASB',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b0 Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'L == 0b0 && o0 == 0b0',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=CASB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASB cclass=CASB_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASLB_C32_ldstexcl => {
  name       => 'CASLB',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b1 Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'L == 0b0 && o0 == 0b1',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=CASLB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASB cclass=CASB_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASAH_C32_ldstexcl => {
  name       => 'CASAH',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b1 Rs:u=0bxxxxx o0:u=0b0 Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'L == 0b1 && o0 == 0b0',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=CASAH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASH cclass=CASH_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASALH_C32_ldstexcl => {
  name       => 'CASALH',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b1 Rs:u=0bxxxxx o0:u=0b1 Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'L == 0b1 && o0 == 0b1',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=CASALH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASH cclass=CASH_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASH_C32_ldstexcl => {
  name       => 'CASH',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b0 Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'L == 0b0 && o0 == 0b0',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=CASH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASH cclass=CASH_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASLH_C32_ldstexcl => {
  name       => 'CASLH',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b1 Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'L == 0b0 && o0 == 0b1',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=CASLH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASH cclass=CASH_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASP_CP32_ldstexcl => {
  name       => '32-bit CASP',
  diagram    => 'ig0=0b0 sz:u=0b0 ig1=0b001000 o2=0b0 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b0 Rt2:u=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sz == 0b0 && L == 0b0 && o0 == 0b0',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-pair-32 atomic-ops=CASP-pair-32 instr-class=general isa=A64 mnemonic=CASP reg-type=pair-32',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASP cclass=CASP_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASPA_CP32_ldstexcl => {
  name       => '32-bit CASPA',
  diagram    => 'ig0=0b0 sz:u=0b0 ig1=0b001000 o2=0b0 L:u=0b1 o1=0b1 Rs:u=0bxxxxx o0:u=0b0 Rt2:u=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sz == 0b0 && L == 0b1 && o0 == 0b0',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-pair-32 atomic-ops=CASPA-pair-32 instr-class=general isa=A64 mnemonic=CASPA reg-type=pair-32',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASP cclass=CASP_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASPAL_CP32_ldstexcl => {
  name       => '32-bit CASPAL',
  diagram    => 'ig0=0b0 sz:u=0b0 ig1=0b001000 o2=0b0 L:u=0b1 o1=0b1 Rs:u=0bxxxxx o0:u=0b1 Rt2:u=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sz == 0b0 && L == 0b1 && o0 == 0b1',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-pair-32 atomic-ops=CASPAL-pair-32 instr-class=general isa=A64 mnemonic=CASPAL reg-type=pair-32',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASP cclass=CASP_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASPL_CP32_ldstexcl => {
  name       => '32-bit CASPL',
  diagram    => 'ig0=0b0 sz:u=0b0 ig1=0b001000 o2=0b0 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b1 Rt2:u=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sz == 0b0 && L == 0b0 && o0 == 0b1',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-pair-32 atomic-ops=CASPL-pair-32 instr-class=general isa=A64 mnemonic=CASPL reg-type=pair-32',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASP cclass=CASP_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASP_CP64_ldstexcl => {
  name       => '64-bit CASP',
  diagram    => 'ig0=0b0 sz:u=0b1 ig1=0b001000 o2=0b0 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b0 Rt2:u=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sz == 0b1 && L == 0b0 && o0 == 0b0',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-pair-64 atomic-ops=CASP-pair-64 instr-class=general isa=A64 mnemonic=CASP reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASP cclass=CASP_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASPA_CP64_ldstexcl => {
  name       => '64-bit CASPA',
  diagram    => 'ig0=0b0 sz:u=0b1 ig1=0b001000 o2=0b0 L:u=0b1 o1=0b1 Rs:u=0bxxxxx o0:u=0b0 Rt2:u=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sz == 0b1 && L == 0b1 && o0 == 0b0',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-pair-64 atomic-ops=CASPA-pair-64 instr-class=general isa=A64 mnemonic=CASPA reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASP cclass=CASP_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASPAL_CP64_ldstexcl => {
  name       => '64-bit CASPAL',
  diagram    => 'ig0=0b0 sz:u=0b1 ig1=0b001000 o2=0b0 L:u=0b1 o1=0b1 Rs:u=0bxxxxx o0:u=0b1 Rt2:u=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sz == 0b1 && L == 0b1 && o0 == 0b1',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-pair-64 atomic-ops=CASPAL-pair-64 instr-class=general isa=A64 mnemonic=CASPAL reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASP cclass=CASP_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CASPL_CP64_ldstexcl => {
  name       => '64-bit CASPL',
  diagram    => 'ig0=0b0 sz:u=0b1 ig1=0b001000 o2=0b0 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b1 Rt2:u=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sz == 0b1 && L == 0b0 && o0 == 0b1',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-pair-64 atomic-ops=CASPL-pair-64 instr-class=general isa=A64 mnemonic=CASPL reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASP cclass=CASP_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|COMPARISON',
};

ENCODING CBNZ_32_compbranch => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 ig0=0b011010 op:u=0b1 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'branch-offset=br19 compare-with=cmp-nonzero datatype=32 instr-class=general isa=A64 mnemonic=CBNZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=control iclass=compbranch page=CBNZ cclass=CBNZ_br19',
  categories => 'GENERAL|BRANCH|CONDITIONALLY|COMPARISON',
};

ENCODING CBNZ_64_compbranch => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 ig0=0b011010 op:u=0b1 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'branch-offset=br19 compare-with=cmp-nonzero datatype=64 instr-class=general isa=A64 mnemonic=CBNZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=control iclass=compbranch page=CBNZ cclass=CBNZ_br19',
  categories => 'GENERAL|BRANCH|CONDITIONALLY|COMPARISON',
};

ENCODING CBZ_32_compbranch => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 ig0=0b011010 op:u=0b0 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'branch-offset=br19 compare-with=cmp-zero datatype=32 instr-class=general isa=A64 mnemonic=CBZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=control iclass=compbranch page=CBZ cclass=CBZ_br19',
  categories => 'GENERAL|BRANCH|CONDITIONALLY|COMPARISON',
};

ENCODING CBZ_64_compbranch => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 ig0=0b011010 op:u=0b0 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'branch-offset=br19 compare-with=cmp-zero datatype=64 instr-class=general isa=A64 mnemonic=CBZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=control iclass=compbranch page=CBZ cclass=CBZ_br19',
  categories => 'GENERAL|BRANCH|CONDITIONALLY|COMPARISON',
};

ENCODING CCMN_32_condcmp_imm => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S=0b1 ig0=0b11010010 imm5:u=0bxxxxx cond:u=0bxxxx ig1=0b1 o2=0b0 Rn:u=0bxxxxx o3=0b0 nzcv:u=0bxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 immediate-type=imm5u instr-class=general isa=A64 mnemonic=CCMN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=condcmp_imm page=CCMN_imm cclass=CCMN_imm_imm5u',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON|CONDITIONALLY',
};

ENCODING CCMN_64_condcmp_imm => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S=0b1 ig0=0b11010010 imm5:u=0bxxxxx cond:u=0bxxxx ig1=0b1 o2=0b0 Rn:u=0bxxxxx o3=0b0 nzcv:u=0bxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 immediate-type=imm5u instr-class=general isa=A64 mnemonic=CCMN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=condcmp_imm page=CCMN_imm cclass=CCMN_imm_imm5u',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON|CONDITIONALLY',
};

ENCODING CCMN_32_condcmp_reg => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S=0b1 ig0=0b11010010 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2=0b0 Rn:u=0bxxxxx o3=0b0 nzcv:u=0bxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=CCMN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=condcmp_reg page=CCMN_reg cclass=CCMN_reg_general',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON|CONDITIONALLY',
};

ENCODING CCMN_64_condcmp_reg => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S=0b1 ig0=0b11010010 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2=0b0 Rn:u=0bxxxxx o3=0b0 nzcv:u=0bxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=CCMN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=condcmp_reg page=CCMN_reg cclass=CCMN_reg_general',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON|CONDITIONALLY',
};

ENCODING CCMP_32_condcmp_imm => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S=0b1 ig0=0b11010010 imm5:u=0bxxxxx cond:u=0bxxxx ig1=0b1 o2=0b0 Rn:u=0bxxxxx o3=0b0 nzcv:u=0bxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 immediate-type=imm5u instr-class=general isa=A64 mnemonic=CCMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=condcmp_imm page=CCMP_imm cclass=CCMP_imm_imm5u',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON|CONDITIONALLY',
};

ENCODING CCMP_64_condcmp_imm => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S=0b1 ig0=0b11010010 imm5:u=0bxxxxx cond:u=0bxxxx ig1=0b1 o2=0b0 Rn:u=0bxxxxx o3=0b0 nzcv:u=0bxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 immediate-type=imm5u instr-class=general isa=A64 mnemonic=CCMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=condcmp_imm page=CCMP_imm cclass=CCMP_imm_imm5u',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON|CONDITIONALLY',
};

ENCODING CCMP_32_condcmp_reg => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S=0b1 ig0=0b11010010 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2=0b0 Rn:u=0bxxxxx o3=0b0 nzcv:u=0bxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=CCMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=condcmp_reg page=CCMP_reg cclass=CCMP_reg_general',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON|CONDITIONALLY',
};

ENCODING CCMP_64_condcmp_reg => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S=0b1 ig0=0b11010010 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2=0b0 Rn:u=0bxxxxx o3=0b0 nzcv:u=0bxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=CCMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=condcmp_reg page=CCMP_reg cclass=CCMP_reg_general',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON|CONDITIONALLY',
};

ENCODING CFINV_M_pstate => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b000 CRn=0b0100 CRm:u=0bZZZZ op2=0b000 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=CFINV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=control iclass=pstate page=CFINV cclass=CFINV_M_pstate',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'SYSTEM|BITWISE|CARRY',
};

ENCODING CLREX_BN_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm:u=0bxxxx op2=0b010 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=CLREX',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=CLREX cclass=CLREX_BN_barriers',
  categories => 'SYSTEM|MISC',
};

ENCODING CLS_32_dp_1src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:1]=0b00010 op:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=CLS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_1src page=CLS_int cclass=CLS_int_general',
  categories => 'GENERAL|BITFIELD',
};

ENCODING CLS_64_dp_1src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:1]=0b00010 op:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=CLS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_1src page=CLS_int cclass=CLS_int_general',
  categories => 'GENERAL|BITFIELD',
};

ENCODING CLZ_32_dp_1src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:1]=0b00010 op:u=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=CLZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_1src page=CLZ_int cclass=CLZ_int_general',
  categories => 'GENERAL|BITFIELD',
};

ENCODING CLZ_64_dp_1src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:1]=0b00010 op:u=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=CLZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_1src page=CLZ_int cclass=CLZ_int_general',
  categories => 'GENERAL|BITFIELD',
};

ENCODING CRC32B_32C_dp_2src => {
  name       => 'CRC32B',
  diagram    => 'sf:u=0b0 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b010 C:u=0b0 sz:u=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && sz == 0b00',
  docvars    => 'datatype=32 feature=crc instr-class=general isa=A64 mnemonic=CRC32B',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_2src page=CRC32 cclass=CRC32_crc',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32H_32C_dp_2src => {
  name       => 'CRC32H',
  diagram    => 'sf:u=0b0 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b010 C:u=0b0 sz:u=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && sz == 0b01',
  docvars    => 'datatype=32 feature=crc instr-class=general isa=A64 mnemonic=CRC32H',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_2src page=CRC32 cclass=CRC32_crc',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32W_32C_dp_2src => {
  name       => 'CRC32W',
  diagram    => 'sf:u=0b0 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b010 C:u=0b0 sz:u=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && sz == 0b10',
  docvars    => 'datatype=32 feature=crc instr-class=general isa=A64 mnemonic=CRC32W',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_2src page=CRC32 cclass=CRC32_crc',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32X_64C_dp_2src => {
  name       => 'CRC32X',
  diagram    => 'sf:u=0b1 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b010 C:u=0b0 sz:u=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && sz == 0b11',
  docvars    => 'datatype=64 feature=crc instr-class=general isa=A64 mnemonic=CRC32X',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_2src page=CRC32 cclass=CRC32_crc',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32CB_32C_dp_2src => {
  name       => 'CRC32CB',
  diagram    => 'sf:u=0b0 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b010 C:u=0b1 sz:u=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && sz == 0b00',
  docvars    => 'datatype=32 feature=crc instr-class=general isa=A64 mnemonic=CRC32CB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_2src page=CRC32C cclass=CRC32C_crc',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32CH_32C_dp_2src => {
  name       => 'CRC32CH',
  diagram    => 'sf:u=0b0 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b010 C:u=0b1 sz:u=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && sz == 0b01',
  docvars    => 'datatype=32 feature=crc instr-class=general isa=A64 mnemonic=CRC32CH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_2src page=CRC32C cclass=CRC32C_crc',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32CW_32C_dp_2src => {
  name       => 'CRC32CW',
  diagram    => 'sf:u=0b0 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b010 C:u=0b1 sz:u=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && sz == 0b10',
  docvars    => 'datatype=32 feature=crc instr-class=general isa=A64 mnemonic=CRC32CW',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_2src page=CRC32C cclass=CRC32C_crc',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32CX_64C_dp_2src => {
  name       => 'CRC32CX',
  diagram    => 'sf:u=0b1 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b010 C:u=0b1 sz:u=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && sz == 0b11',
  docvars    => 'datatype=64 feature=crc instr-class=general isa=A64 mnemonic=CRC32CX',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_2src page=CRC32C cclass=CRC32C_crc',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CSDB_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0010 op2:u=0b100 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=CSDB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=CSDB cclass=CSDB_HI_hints',
  categories => 'SYSTEM',
};

ENCODING CSEL_32_condsel => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=CSEL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=condsel page=CSEL cclass=CSEL_general',
  categories => 'GENERAL|SELECTION|CONDITIONALLY',
};

ENCODING CSEL_64_condsel => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=CSEL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=condsel page=CSEL cclass=CSEL_general',
  categories => 'GENERAL|SELECTION|CONDITIONALLY',
};

ENCODING CSINC_32_condsel => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=CSINC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=condsel page=CSINC cclass=CSINC_general',
  categories => 'GENERAL|SELECTION|ARITHMETIC|CONDITIONALLY|INC_DEC',
};

ENCODING CSINC_64_condsel => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=CSINC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=condsel page=CSINC cclass=CSINC_general',
  categories => 'GENERAL|SELECTION|ARITHMETIC|CONDITIONALLY|INC_DEC',
};

ENCODING CSINV_32_condsel => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=CSINV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=condsel page=CSINV cclass=CSINV_general',
  categories => 'GENERAL|SELECTION|CONDITIONALLY',
};

ENCODING CSINV_64_condsel => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=CSINV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=condsel page=CSINV cclass=CSINV_general',
  categories => 'GENERAL|SELECTION|CONDITIONALLY',
};

ENCODING CSNEG_32_condsel => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=CSNEG',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=condsel page=CSNEG cclass=CSNEG_general',
  categories => 'GENERAL|SELECTION|CONDITIONALLY',
};

ENCODING CSNEG_64_condsel => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=CSNEG',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=condsel page=CSNEG cclass=CSNEG_general',
  categories => 'GENERAL|SELECTION|CONDITIONALLY',
};

ENCODING DCPS1_DC_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b101 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL:u=0b01',
  docvars    => 'instr-class=system isa=A64 mnemonic=DCPS1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=DCPS1 cclass=DCPS1_DC_exception',
  categories => 'SYSTEM|DEBUG',
};

ENCODING DCPS2_DC_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b101 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL:u=0b10',
  docvars    => 'instr-class=system isa=A64 mnemonic=DCPS2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=DCPS2 cclass=DCPS2_DC_exception',
  categories => 'SYSTEM|DEBUG',
};

ENCODING DCPS3_DC_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b101 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL:u=0b11',
  docvars    => 'instr-class=system isa=A64 mnemonic=DCPS3',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v0-A alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=DCPS3 cclass=DCPS3_DC_exception',
  categories => 'SYSTEM|DEBUG',
};

ENCODING DGH_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0000 op2:u=0b110 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=DGH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=DGH cclass=DGH_HI_hints',
  categories => 'SYSTEM|HINT',
};

ENCODING DMB_BO_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm:u=0bxxxx ig1=0b1 opc:u=0b01 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=DMB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=DMB cclass=DMB_BO_barriers',
  categories => 'SYSTEM',
};

ENCODING DRPS_64E_branch_reg => {
  name       => 'System',
  diagram    => 'ig0=0b1101011 opc=0b0101 op2=0b11111 op3=0b000000 Rt=0b11111 op4=0b00000',
  docvars    => 'instr-class=system isa=A64 mnemonic=DRPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=DRPS cclass=DRPS_64E_branch_reg',
  categories => 'SYSTEM|DEBUG',
};

ENCODING DSB_BO_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm:u=0bxxxx ig1=0b1 opc:u=0b00 Rt=0b11111',
  cnsts      => 'CRm!=0b0x00',
  docvars    => 'instr-class=system isa=A64 mnemonic=DSB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=DSB cclass=DSB_DSB_BO_barriers',
  categories => 'SYSTEM',
};

ENCODING EON_32_log_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b10 ig0=0b01010 shift:u=0bxx N:u=0b1 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=no-s datatype=32 datatype-reguse=32-shifted-reg instr-class=general isa=A64 mnemonic=EON reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=log_shift page=EON cclass=EON_no_s',
  categories => 'GENERAL|BITWISE',
};

ENCODING EON_64_log_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b10 ig0=0b01010 shift:u=0bxx N:u=0b1 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=no-s datatype=64 datatype-reguse=64-shifted-reg instr-class=general isa=A64 mnemonic=EON reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=log_shift page=EON cclass=EON_no_s',
  categories => 'GENERAL|BITWISE',
};

ENCODING EOR_32_log_imm => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b10 ig0=0b100100 N:u=0b0 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  docvars    => 'cond-setting=no-s datatype=32 immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=EOR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=log_imm page=EOR_log_imm cclass=EOR_log_imm_no_s',
  categories => 'GENERAL|BITWISE',
};

ENCODING EOR_64_log_imm => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b10 ig0=0b100100 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=no-s datatype=64 immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=EOR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=log_imm page=EOR_log_imm cclass=EOR_log_imm_no_s',
  categories => 'GENERAL|BITWISE',
};

ENCODING EOR_32_log_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b10 ig0=0b01010 shift:u=0bxx N:u=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 datatype-reguse=32-shifted-reg instr-class=general isa=A64 mnemonic=EOR reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=log_shift page=EOR_log_shift cclass=EOR_log_shift_general',
  categories => 'GENERAL|BITWISE',
};

ENCODING EOR_64_log_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b10 ig0=0b01010 shift:u=0bxx N:u=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 datatype-reguse=64-shifted-reg instr-class=general isa=A64 mnemonic=EOR reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=log_shift page=EOR_log_shift cclass=EOR_log_shift_general',
  categories => 'GENERAL|BITWISE',
};

ENCODING ERET_64E_branch_reg => {
  name       => 'System',
  diagram    => 'ig0=0b1101011 opc[3]=0b0 opc[2:0]=0b100 op2=0b11111 op3[5:2]=0b0000 A:u=0b0 M:u=0b0 Rn:u=0b11111 op4:u=0b00000',
  docvars    => 'instr-class=system isa=A64 mnemonic=ERET',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=ERET cclass=ERET_64E_branch_reg',
  exceptions => 'ERETTRAP',
  categories => 'SYSTEM|RET|EXCEPTION',
};

ENCODING ERETAA_64E_branch_reg => {
  name       => 'ERETAA',
  diagram    => 'ig0=0b1101011 opc[3]=0b0 opc[2:0]=0b100 op2=0b11111 op3[5:2]=0b0000 A:u=0b1 M:u=0b0 Rn:u=0b11111 op4:u=0b11111',
  bitdiffs   => 'M == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=ERETAA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=ERETA cclass=ERETA_general',
  exceptions => 'ERETTRAP|PACFAIL|PACTRAP',
  categories => 'GENERAL|RET|EXCEPTION|PAC',
};

ENCODING ERETAB_64E_branch_reg => {
  name       => 'ERETAB',
  diagram    => 'ig0=0b1101011 opc[3]=0b0 opc[2:0]=0b100 op2=0b11111 op3[5:2]=0b0000 A:u=0b1 M:u=0b1 Rn:u=0b11111 op4:u=0b11111',
  bitdiffs   => 'M == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=ERETAB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=ERETA cclass=ERETA_general',
  exceptions => 'ERETTRAP|PACFAIL|PACTRAP',
  categories => 'GENERAL|RET|EXCEPTION|PAC',
};

ENCODING ESB_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0010 op2:u=0b000 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=ESB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=ESB cclass=ESB_HI_hints',
  pstate     => 'EL=CR',
  categories => 'SYSTEM',
};

ENCODING EXTR_32_extract => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op21=0b00 ig0=0b100111 N:u=0b0 o0=0b0 Rm:u=0bxxxxx imms:u=0b0xxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0 && imms == 0b0xxxxx',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=EXTR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=extract page=EXTR cclass=EXTR_general',
  categories => 'GENERAL|EXTRACTION',
};

ENCODING EXTR_64_extract => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op21=0b00 ig0=0b100111 N:u=0b1 o0=0b0 Rm:u=0bxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && N == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=EXTR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=extract page=EXTR cclass=EXTR_general',
  categories => 'GENERAL|EXTRACTION',
};

ENCODING GMI_64G_dp_2src => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b0 S=0b0 ig1=0b11010110 Xm:u=0bxxxxx opcode=0b000101 Xn:u=0bxxxxx Xd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=GMI',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=GMI cclass=GMI_64G_dp_2src',
  categories => 'GENERAL|INSERTION|TAGS',
};

ENCODING HINT_HM_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0bxxxx op2:u=0bxxx Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=HINT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=HINT cclass=HINT_HM_hints',
  exceptions => 'WFXTRAP',
  pstate     => 'EL=CR',
  categories => 'SYSTEM|HINT',
};

ENCODING HLT_EX_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b010 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL=0b00',
  docvars    => 'instr-class=system isa=A64 mnemonic=HLT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=HLT cclass=HLT_EX_exception',
  categories => 'SYSTEM|HALT',
};

ENCODING HVC_EX_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b000 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL=0b10',
  docvars    => 'instr-class=system isa=A64 mnemonic=HVC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=HVC cclass=HVC_EX_exception',
  exceptions => 'HYPERVISORCALL',
  pstate     => 'EL=CR',
  categories => 'SYSTEM|CALL',
};

ENCODING IRG_64I_dp_2src => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b0 S=0b0 ig1=0b11010110 Xm:u=0bxxxxx opcode=0b000100 Xn:u=0bxxxxx Xd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=IRG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=IRG cclass=IRG_64I_dp_2src',
  categories => 'GENERAL|INSERTION|TAGS',
};

ENCODING ISB_BI_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm:u=0bxxxx ig1=0b1 opc:u=0b10 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=ISB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=ISB cclass=ISB_BI_barriers',
  categories => 'SYSTEM',
};

ENCODING LDADD_32_memop => {
  name       => '32-bit LDADD',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=LDADD-32-reg instr-class=general isa=A64 mnemonic=LDADD reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADD cclass=LDADD_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
};

ENCODING LDADDA_32_memop => {
  name       => '32-bit LDADDA',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=LDADDA-32-reg instr-class=general isa=A64 mnemonic=LDADDA reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADD cclass=LDADD_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
};

ENCODING LDADDAL_32_memop => {
  name       => '32-bit LDADDAL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=LDADDAL-32-reg instr-class=general isa=A64 mnemonic=LDADDAL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADD cclass=LDADD_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
};

ENCODING LDADDL_32_memop => {
  name       => '32-bit LDADDL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=LDADDL-32-reg instr-class=general isa=A64 mnemonic=LDADDL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADD cclass=LDADD_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
};

ENCODING LDADD_64_memop => {
  name       => '64-bit LDADD',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=LDADD-64-reg instr-class=general isa=A64 mnemonic=LDADD reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADD cclass=LDADD_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
};

ENCODING LDADDA_64_memop => {
  name       => '64-bit LDADDA',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=LDADDA-64-reg instr-class=general isa=A64 mnemonic=LDADDA reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADD cclass=LDADD_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
};

ENCODING LDADDAL_64_memop => {
  name       => '64-bit LDADDAL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=LDADDAL-64-reg instr-class=general isa=A64 mnemonic=LDADDAL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADD cclass=LDADD_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
};

ENCODING LDADDL_64_memop => {
  name       => '64-bit LDADDL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=LDADDL-64-reg instr-class=general isa=A64 mnemonic=LDADDL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADD cclass=LDADD_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
};

ENCODING LDADDAB_32_memop => {
  name       => 'LDADDAB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDADDAB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADDB cclass=LDADDB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
};

ENCODING LDADDALB_32_memop => {
  name       => 'LDADDALB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDADDALB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADDB cclass=LDADDB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
};

ENCODING LDADDB_32_memop => {
  name       => 'LDADDB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDADDB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADDB cclass=LDADDB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
};

ENCODING LDADDLB_32_memop => {
  name       => 'LDADDLB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDADDLB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADDB cclass=LDADDB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
};

ENCODING LDADDAH_32_memop => {
  name       => 'LDADDAH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDADDAH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADDH cclass=LDADDH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
};

ENCODING LDADDALH_32_memop => {
  name       => 'LDADDALH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDADDALH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADDH cclass=LDADDH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
};

ENCODING LDADDH_32_memop => {
  name       => 'LDADDH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDADDH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADDH cclass=LDADDH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
};

ENCODING LDADDLH_32_memop => {
  name       => 'LDADDLH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDADDLH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADDH cclass=LDADDH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
};

ENCODING LDAPR_32L_memop => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A=0b1 R=0b0 ig2=0b1 Rs:u=0bSSSSS o3=0b1 opc=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'atomic-ops=LDAPR-32-reg instr-class=general isa=A64 mnemonic=LDAPR reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDAPR cclass=LDAPR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAPR_64L_memop => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A=0b1 R=0b0 ig2=0b1 Rs:u=0bSSSSS o3=0b1 opc=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'atomic-ops=LDAPR-64-reg instr-class=general isa=A64 mnemonic=LDAPR reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDAPR cclass=LDAPR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAPRB_32L_memop => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A=0b1 R=0b0 ig2=0b1 Rs:u=0bSSSSS o3=0b1 opc=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDAPRB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDAPRB cclass=LDAPRB_32L_memop',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAPRH_32L_memop => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A=0b1 R=0b0 ig2=0b1 Rs:u=0bSSSSS o3=0b1 opc=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDAPRH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDAPRH cclass=LDAPRH_32L_memop',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAPUR_32_ldapstl_unscaled => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b011001 opc:u=0b01 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-32-reg atomic-ops=LDAPUR-32-reg instr-class=general isa=A64 mnemonic=LDAPUR offset-type=off9s_u reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=LDAPUR_gen cclass=LDAPUR_gen_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAPUR_64_ldapstl_unscaled => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b011001 opc:u=0b01 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-64-reg atomic-ops=LDAPUR-64-reg instr-class=general isa=A64 mnemonic=LDAPUR offset-type=off9s_u reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=LDAPUR_gen cclass=LDAPUR_gen_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAPURB_32_ldapstl_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b00 ig0=0b011001 opc:u=0b01 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDAPURB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=LDAPURB cclass=LDAPURB_32_ldapstl_unscaled',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAPURH_32_ldapstl_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b01 ig0=0b011001 opc:u=0b01 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDAPURH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=LDAPURH cclass=LDAPURH_32_ldapstl_unscaled',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAPURSB_32_ldapstl_unscaled => {
  name       => '32-bit',
  diagram    => 'size:u=0b00 ig0=0b011001 opc:u=0b11 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b11',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDAPURSB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=LDAPURSB cclass=LDAPURSB_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDAPURSB_64_ldapstl_unscaled => {
  name       => '64-bit',
  diagram    => 'size:u=0b00 ig0=0b011001 opc:u=0b10 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=base-plus-offset datatype=64 instr-class=general isa=A64 mnemonic=LDAPURSB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=LDAPURSB cclass=LDAPURSB_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDAPURSH_32_ldapstl_unscaled => {
  name       => '32-bit',
  diagram    => 'size:u=0b01 ig0=0b011001 opc:u=0b11 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b11',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDAPURSH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=LDAPURSH cclass=LDAPURSH_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDAPURSH_64_ldapstl_unscaled => {
  name       => '64-bit',
  diagram    => 'size:u=0b01 ig0=0b011001 opc:u=0b10 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=base-plus-offset datatype=64 instr-class=general isa=A64 mnemonic=LDAPURSH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=LDAPURSH cclass=LDAPURSH_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDAPURSW_64_ldapstl_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b10 ig0=0b011001 opc:u=0b10 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=64 instr-class=general isa=A64 mnemonic=LDAPURSW offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=LDAPURSW cclass=LDAPURSW_64_ldapstl_unscaled',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDAR_LR32_ldstexcl => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDAR-32-reg instr-class=general isa=A64 mnemonic=LDAR reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDAR cclass=LDAR_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAR_LR64_ldstexcl => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-64-reg atomic-ops=LDAR-64-reg instr-class=general isa=A64 mnemonic=LDAR reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDAR cclass=LDAR_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDARB_LR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDARB-32-reg instr-class=general isa=A64 mnemonic=LDARB reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDARB cclass=LDARB_LR32_ldstexcl',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDARH_LR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDARH-32-reg instr-class=general isa=A64 mnemonic=LDARH reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDARH cclass=LDARH_LR32_ldstexcl',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAXP_LP32_ldstexcl => {
  name       => '32-bit',
  diagram    => 'ig0=0b1 sz:u=0b0 ig1=0b001000 o2=0b0 L:u=0b1 o1=0b1 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sz == 0b0',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-pair-32 atomic-ops=LDAXP-pair-32 instr-class=general isa=A64 mnemonic=LDAXP reg-type=pair-32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDAXP cclass=LDAXP_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAXP_LP64_ldstexcl => {
  name       => '64-bit',
  diagram    => 'ig0=0b1 sz:u=0b1 ig1=0b001000 o2=0b0 L:u=0b1 o1=0b1 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sz == 0b1',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-pair-64 atomic-ops=LDAXP-pair-64 instr-class=general isa=A64 mnemonic=LDAXP reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDAXP cclass=LDAXP_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAXR_LR32_ldstexcl => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b001000 o2=0b0 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDAXR-32-reg instr-class=general isa=A64 mnemonic=LDAXR reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDAXR cclass=LDAXR_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAXR_LR64_ldstexcl => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b001000 o2=0b0 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-64-reg atomic-ops=LDAXR-64-reg instr-class=general isa=A64 mnemonic=LDAXR reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDAXR cclass=LDAXR_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAXRB_LR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b0 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDAXRB-32-reg instr-class=general isa=A64 mnemonic=LDAXRB reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDAXRB cclass=LDAXRB_LR32_ldstexcl',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAXRH_LR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b0 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDAXRH-32-reg instr-class=general isa=A64 mnemonic=LDAXRH reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDAXRH cclass=LDAXRH_LR32_ldstexcl',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDCLR_32_memop => {
  name       => '32-bit LDCLR',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=LDCLR-32-reg instr-class=general isa=A64 mnemonic=LDCLR reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLR cclass=LDCLR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDCLRA_32_memop => {
  name       => '32-bit LDCLRA',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=LDCLRA-32-reg instr-class=general isa=A64 mnemonic=LDCLRA reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLR cclass=LDCLR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDCLRAL_32_memop => {
  name       => '32-bit LDCLRAL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=LDCLRAL-32-reg instr-class=general isa=A64 mnemonic=LDCLRAL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLR cclass=LDCLR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDCLRL_32_memop => {
  name       => '32-bit LDCLRL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=LDCLRL-32-reg instr-class=general isa=A64 mnemonic=LDCLRL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLR cclass=LDCLR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDCLR_64_memop => {
  name       => '64-bit LDCLR',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=LDCLR-64-reg instr-class=general isa=A64 mnemonic=LDCLR reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLR cclass=LDCLR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDCLRA_64_memop => {
  name       => '64-bit LDCLRA',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=LDCLRA-64-reg instr-class=general isa=A64 mnemonic=LDCLRA reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLR cclass=LDCLR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDCLRAL_64_memop => {
  name       => '64-bit LDCLRAL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=LDCLRAL-64-reg instr-class=general isa=A64 mnemonic=LDCLRAL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLR cclass=LDCLR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDCLRL_64_memop => {
  name       => '64-bit LDCLRL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=LDCLRL-64-reg instr-class=general isa=A64 mnemonic=LDCLRL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLR cclass=LDCLR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDCLRAB_32_memop => {
  name       => 'LDCLRAB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDCLRAB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLRB cclass=LDCLRB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDCLRALB_32_memop => {
  name       => 'LDCLRALB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDCLRALB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLRB cclass=LDCLRB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDCLRB_32_memop => {
  name       => 'LDCLRB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDCLRB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLRB cclass=LDCLRB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDCLRLB_32_memop => {
  name       => 'LDCLRLB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDCLRLB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLRB cclass=LDCLRB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDCLRAH_32_memop => {
  name       => 'LDCLRAH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDCLRAH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLRH cclass=LDCLRH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDCLRALH_32_memop => {
  name       => 'LDCLRALH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDCLRALH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLRH cclass=LDCLRH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDCLRH_32_memop => {
  name       => 'LDCLRH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDCLRH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLRH cclass=LDCLRH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDCLRLH_32_memop => {
  name       => 'LDCLRLH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDCLRLH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLRH cclass=LDCLRH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDEOR_32_memop => {
  name       => '32-bit LDEOR',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=LDEOR-32-reg instr-class=general isa=A64 mnemonic=LDEOR reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEOR cclass=LDEOR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDEORA_32_memop => {
  name       => '32-bit LDEORA',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=LDEORA-32-reg instr-class=general isa=A64 mnemonic=LDEORA reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEOR cclass=LDEOR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDEORAL_32_memop => {
  name       => '32-bit LDEORAL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=LDEORAL-32-reg instr-class=general isa=A64 mnemonic=LDEORAL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEOR cclass=LDEOR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDEORL_32_memop => {
  name       => '32-bit LDEORL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=LDEORL-32-reg instr-class=general isa=A64 mnemonic=LDEORL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEOR cclass=LDEOR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDEOR_64_memop => {
  name       => '64-bit LDEOR',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=LDEOR-64-reg instr-class=general isa=A64 mnemonic=LDEOR reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEOR cclass=LDEOR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDEORA_64_memop => {
  name       => '64-bit LDEORA',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=LDEORA-64-reg instr-class=general isa=A64 mnemonic=LDEORA reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEOR cclass=LDEOR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDEORAL_64_memop => {
  name       => '64-bit LDEORAL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=LDEORAL-64-reg instr-class=general isa=A64 mnemonic=LDEORAL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEOR cclass=LDEOR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDEORL_64_memop => {
  name       => '64-bit LDEORL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=LDEORL-64-reg instr-class=general isa=A64 mnemonic=LDEORL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEOR cclass=LDEOR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDEORAB_32_memop => {
  name       => 'LDEORAB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDEORAB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEORB cclass=LDEORB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDEORALB_32_memop => {
  name       => 'LDEORALB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDEORALB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEORB cclass=LDEORB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDEORB_32_memop => {
  name       => 'LDEORB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDEORB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEORB cclass=LDEORB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDEORLB_32_memop => {
  name       => 'LDEORLB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDEORLB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEORB cclass=LDEORB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDEORAH_32_memop => {
  name       => 'LDEORAH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDEORAH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEORH cclass=LDEORH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDEORALH_32_memop => {
  name       => 'LDEORALH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDEORALH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEORH cclass=LDEORH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDEORH_32_memop => {
  name       => 'LDEORH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDEORH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEORH cclass=LDEORH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDEORLH_32_memop => {
  name       => 'LDEORLH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDEORLH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEORH cclass=LDEORH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDG_64Loffset_ldsttags => {
  name       => 'Integer',
  diagram    => 'ig0=0b11011001 opc=0b01 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b00 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=LDG cclass=LDG_64Loffset_ldsttags',
  exceptions => 'SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|LOAD',
};

ENCODING LDGM_64bulk_ldsttags => {
  name       => 'Integer',
  diagram    => 'ig0=0b11011001 opc=0b11 ig1=0b1 imm9=0b000000000 op2=0b00 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDGM',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=LDGM cclass=LDGM_64bulk_ldsttags',
  exceptions => 'SPALIGNMENT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|LOAD',
};

ENCODING LDLAR_LR32_ldstexcl => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDLAR-32-reg instr-class=general isa=A64 mnemonic=LDLAR reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDLAR cclass=LDLAR_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDLAR_LR64_ldstexcl => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-64-reg atomic-ops=LDLAR-64-reg instr-class=general isa=A64 mnemonic=LDLAR reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDLAR cclass=LDLAR_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDLARB_LR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDLARB-32-reg instr-class=general isa=A64 mnemonic=LDLARB reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDLARB cclass=LDLARB_LR32_ldstexcl',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDLARH_LR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDLARH-32-reg instr-class=general isa=A64 mnemonic=LDLARH reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDLARH cclass=LDLARH_LR32_ldstexcl',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDNP_32_ldstnapair_offs => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b101 V=0b0 ig1=0b000 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-32 atomic-ops=LDNP-pair-32 instr-class=general isa=A64 mnemonic=LDNP offset-type=off7s_s reg-type=pair-32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstnapair_offs page=LDNP_gen cclass=LDNP_gen_signed_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|HINT|LOAD',
};

ENCODING LDNP_64_ldstnapair_offs => {
  name       => '64-bit',
  diagram    => 'opc:u=0b10 ig0=0b101 V=0b0 ig1=0b000 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-64 atomic-ops=LDNP-pair-64 instr-class=general isa=A64 mnemonic=LDNP offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstnapair_offs page=LDNP_gen cclass=LDNP_gen_signed_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|HINT|LOAD',
};

ENCODING LDP_32_ldstpair_post => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b101 V=0b0 ig1=0b001 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-pair-32 atomic-ops=LDP-pair-32 instr-class=general isa=A64 mnemonic=LDP offset-type=off7s_s reg-type=pair-32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_post page=LDP_gen cclass=LDP_gen_post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDP_64_ldstpair_post => {
  name       => '64-bit',
  diagram    => 'opc:u=0b10 ig0=0b101 V=0b0 ig1=0b001 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-pair-64 atomic-ops=LDP-pair-64 instr-class=general isa=A64 mnemonic=LDP offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_post page=LDP_gen cclass=LDP_gen_post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDP_32_ldstpair_pre => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b101 V=0b0 ig1=0b011 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-pair-32 atomic-ops=LDP-pair-32 instr-class=general isa=A64 mnemonic=LDP offset-type=off7s_s reg-type=pair-32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_pre page=LDP_gen cclass=LDP_gen_pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDP_64_ldstpair_pre => {
  name       => '64-bit',
  diagram    => 'opc:u=0b10 ig0=0b101 V=0b0 ig1=0b011 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-pair-64 atomic-ops=LDP-pair-64 instr-class=general isa=A64 mnemonic=LDP offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_pre page=LDP_gen cclass=LDP_gen_pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDP_32_ldstpair_off => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b101 V=0b0 ig1=0b010 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-32 atomic-ops=LDP-pair-32 instr-class=general isa=A64 mnemonic=LDP offset-type=off7s_s reg-type=pair-32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_off page=LDP_gen cclass=LDP_gen_signed_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDP_64_ldstpair_off => {
  name       => '64-bit',
  diagram    => 'opc:u=0b10 ig0=0b101 V=0b0 ig1=0b010 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-64 atomic-ops=LDP-pair-64 instr-class=general isa=A64 mnemonic=LDP offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_off page=LDP_gen cclass=LDP_gen_signed_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDPSW_64_ldstpair_post => {
  name       => 'Post-index',
  diagram    => 'opc:u=0b01 ig0=0b101 V=0b0 ig1=0b001 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-pair-64 atomic-ops=LDPSW-pair-64 instr-class=general isa=A64 mnemonic=LDPSW offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_post page=LDPSW cclass=LDPSW_64_ldstpair_post',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDPSW_64_ldstpair_pre => {
  name       => 'Pre-index',
  diagram    => 'opc:u=0b01 ig0=0b101 V=0b0 ig1=0b011 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-pair-64 atomic-ops=LDPSW-pair-64 instr-class=general isa=A64 mnemonic=LDPSW offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_pre page=LDPSW cclass=LDPSW_64_ldstpair_pre',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDPSW_64_ldstpair_off => {
  name       => 'Signed offset',
  diagram    => 'opc:u=0b01 ig0=0b101 V=0b0 ig1=0b010 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-64 atomic-ops=LDPSW-pair-64 instr-class=general isa=A64 mnemonic=LDPSW offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_off page=LDPSW cclass=LDPSW_64_ldstpair_off',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDR_32_ldst_immpost => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-32-reg atomic-ops=LDR-32-reg instr-class=general isa=A64 mnemonic=LDR offset-type=off9s_u reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_immpost page=LDR_imm_gen cclass=LDR_imm_gen_post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_64_ldst_immpost => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-64-reg atomic-ops=LDR-64-reg instr-class=general isa=A64 mnemonic=LDR offset-type=off9s_u reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_immpost page=LDR_imm_gen cclass=LDR_imm_gen_post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_32_ldst_immpre => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-32-reg atomic-ops=LDR-32-reg instr-class=general isa=A64 mnemonic=LDR offset-type=off9s_u reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_immpre page=LDR_imm_gen cclass=LDR_imm_gen_pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_64_ldst_immpre => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-64-reg atomic-ops=LDR-64-reg instr-class=general isa=A64 mnemonic=LDR offset-type=off9s_u reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_immpre page=LDR_imm_gen cclass=LDR_imm_gen_pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_32_ldst_pos => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b01 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=unsigned-scaled-offset address-form-reg-type=unsigned-scaled-offset-32-reg atomic-ops=LDR-32-reg instr-class=general isa=A64 mnemonic=LDR offset-type=off12u_s reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_pos page=LDR_imm_gen cclass=LDR_imm_gen_unsigned_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_64_ldst_pos => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b01 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=unsigned-scaled-offset address-form-reg-type=unsigned-scaled-offset-64-reg atomic-ops=LDR-64-reg instr-class=general isa=A64 mnemonic=LDR offset-type=off12u_s reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_pos page=LDR_imm_gen cclass=LDR_imm_gen_unsigned_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_32_loadlit => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b011 V=0b0 ig1=0b00 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=literal address-form-reg-type=literal-32-reg atomic-ops=LDR-32-reg instr-class=general isa=A64 mnemonic=LDR offset-type=off19s reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=loadlit page=LDR_lit_gen cclass=LDR_lit_gen_literal',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_64_loadlit => {
  name       => '64-bit',
  diagram    => 'opc:u=0b01 ig0=0b011 V=0b0 ig1=0b00 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b01',
  docvars    => 'address-form=literal address-form-reg-type=literal-64-reg atomic-ops=LDR-64-reg instr-class=general isa=A64 mnemonic=LDR offset-type=off19s reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=loadlit page=LDR_lit_gen cclass=LDR_lit_gen_literal',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_32_ldst_regoff => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'atomic-ops=LDR-32-reg instr-class=general isa=A64 mnemonic=LDR offset-type=off-reg reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_regoff page=LDR_reg_gen cclass=LDR_reg_gen_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_64_ldst_regoff => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'atomic-ops=LDR-64-reg instr-class=general isa=A64 mnemonic=LDR offset-type=off-reg reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_regoff page=LDR_reg_gen cclass=LDR_reg_gen_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRAA_64_ldst_pac => {
  name       => 'Key A, offset',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 M:u=0b0 S:u=0bx ig2=0b1 imm9:u=0bxxxxxxxxx W:u=0b0 ig3=0b1 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'M == 0b0 && W == 0b0',
  docvars    => 'address-form=base-plus-offset isa=A64 loadstore-pa=key-a-offs mnemonic=LDRAA offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_pac page=LDRA cclass=LDRA_base_plus_offset',
  exceptions => 'DATAABORT|PACFAIL|PACTRAP|SPALIGNMENT',
  categories => 'DATA_TRANSFER|PAC|LOAD',
};

ENCODING LDRAA_64W_ldst_pac => {
  name       => 'Key A, pre-indexed',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 M:u=0b0 S:u=0bx ig2=0b1 imm9:u=0bxxxxxxxxx W:u=0b1 ig3=0b1 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'M == 0b0 && W == 0b1',
  docvars    => 'address-form=base-plus-offset isa=A64 loadstore-pa=key-a-preind mnemonic=LDRAA offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_pac page=LDRA cclass=LDRA_base_plus_offset',
  exceptions => 'DATAABORT|PACFAIL|PACTRAP|SPALIGNMENT',
  categories => 'DATA_TRANSFER|PAC|LOAD',
};

ENCODING LDRAB_64_ldst_pac => {
  name       => 'Key B, offset',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 M:u=0b1 S:u=0bx ig2=0b1 imm9:u=0bxxxxxxxxx W:u=0b0 ig3=0b1 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'M == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset isa=A64 loadstore-pa=key-b-offs mnemonic=LDRAB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_pac page=LDRA cclass=LDRA_base_plus_offset',
  exceptions => 'DATAABORT|PACFAIL|PACTRAP|SPALIGNMENT',
  categories => 'DATA_TRANSFER|PAC|LOAD',
};

ENCODING LDRAB_64W_ldst_pac => {
  name       => 'Key B, pre-indexed',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 M:u=0b1 S:u=0bx ig2=0b1 imm9:u=0bxxxxxxxxx W:u=0b1 ig3=0b1 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'M == 0b1 && W == 0b1',
  docvars    => 'address-form=base-plus-offset isa=A64 loadstore-pa=key-b-preind mnemonic=LDRAB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_pac page=LDRA cclass=LDRA_base_plus_offset',
  exceptions => 'DATAABORT|PACFAIL|PACTRAP|SPALIGNMENT',
  categories => 'DATA_TRANSFER|PAC|LOAD',
};

ENCODING LDRB_32_ldst_immpost => {
  name       => 'Post-index',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed datatype=32 instr-class=general isa=A64 mnemonic=LDRB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpost page=LDRB_imm cclass=LDRB_32_ldst_immpost',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_32_ldst_immpre => {
  name       => 'Pre-index',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed datatype=32 instr-class=general isa=A64 mnemonic=LDRB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpre page=LDRB_imm cclass=LDRB_32_ldst_immpre',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_32_ldst_pos => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b01 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset datatype=32 instr-class=general isa=A64 mnemonic=LDRB offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_pos page=LDRB_imm cclass=LDRB_32_ldst_pos',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_32B_ldst_regoff => {
  name       => 'Extended register',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'option!=0b011',
  bitdiffs   => 'option != 0b011',
  docvars    => 'datatype=32 datatype-reguse=32-ext-reg instr-class=general isa=A64 mnemonic=LDRB offset-type=off-reg reguse=ext-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_regoff page=LDRB_reg cclass=LDRB_reg_32',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_32BL_ldst_regoff => {
  name       => 'Shifted register',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b1 Rm:u=0bxxxxx option:u=0b011 S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'option == 0b011',
  docvars    => 'datatype=32 datatype-reguse=32-shifted-reg instr-class=general isa=A64 mnemonic=LDRB offset-type=off-reg reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_regoff page=LDRB_reg cclass=LDRB_reg_32',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_32_ldst_immpost => {
  name       => 'Post-index',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed datatype=32 instr-class=general isa=A64 mnemonic=LDRH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpost page=LDRH_imm cclass=LDRH_32_ldst_immpost',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_32_ldst_immpre => {
  name       => 'Pre-index',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed datatype=32 instr-class=general isa=A64 mnemonic=LDRH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpre page=LDRH_imm cclass=LDRH_32_ldst_immpre',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_32_ldst_pos => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b01 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset datatype=32 instr-class=general isa=A64 mnemonic=LDRH offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_pos page=LDRH_imm cclass=LDRH_32_ldst_pos',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_32_ldst_regoff => {
  name       => '32-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=LDRH offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_regoff page=LDRH_reg cclass=LDRH_32_ldst_regoff',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRSB_32_ldst_immpost => {
  name       => '32-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b11 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b11',
  docvars    => 'address-form=post-indexed datatype=32 instr-class=general isa=A64 mnemonic=LDRSB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpost page=LDRSB_imm cclass=LDRSB_imm_post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_64_ldst_immpost => {
  name       => '64-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=post-indexed datatype=64 instr-class=general isa=A64 mnemonic=LDRSB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_immpost page=LDRSB_imm cclass=LDRSB_imm_post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_32_ldst_immpre => {
  name       => '32-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b11 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b11',
  docvars    => 'address-form=pre-indexed datatype=32 instr-class=general isa=A64 mnemonic=LDRSB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpre page=LDRSB_imm cclass=LDRSB_imm_pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_64_ldst_immpre => {
  name       => '64-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=pre-indexed datatype=64 instr-class=general isa=A64 mnemonic=LDRSB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_immpre page=LDRSB_imm cclass=LDRSB_imm_pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_32_ldst_pos => {
  name       => '32-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b11 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b11',
  docvars    => 'address-form=unsigned-scaled-offset datatype=32 instr-class=general isa=A64 mnemonic=LDRSB offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_pos page=LDRSB_imm cclass=LDRSB_imm_unsigned_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_64_ldst_pos => {
  name       => '64-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b10 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=unsigned-scaled-offset datatype=64 instr-class=general isa=A64 mnemonic=LDRSB offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_pos page=LDRSB_imm cclass=LDRSB_imm_unsigned_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_32B_ldst_regoff => {
  name       => '32-bit with extended register offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b11 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'option!=0b011',
  bitdiffs   => 'opc == 0b11 && option != 0b011',
  docvars    => 'datatype=32 datatype-reguse=32-ext-reg instr-class=general isa=A64 mnemonic=LDRSB offset-type=off-reg reguse=ext-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_regoff page=LDRSB_reg cclass=LDRSB_reg_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_32BL_ldst_regoff => {
  name       => '32-bit with shifted register offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b11 ig2=0b1 Rm:u=0bxxxxx option:u=0b011 S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b11 && option == 0b011',
  docvars    => 'datatype=32 datatype-reguse=32-shifted-reg instr-class=general isa=A64 mnemonic=LDRSB offset-type=off-reg reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_regoff page=LDRSB_reg cclass=LDRSB_reg_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_64B_ldst_regoff => {
  name       => '64-bit with extended register offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'option!=0b011',
  bitdiffs   => 'opc == 0b10 && option != 0b011',
  docvars    => 'datatype=64 datatype-reguse=64-ext-reg instr-class=general isa=A64 mnemonic=LDRSB offset-type=off-reg reguse=ext-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_regoff page=LDRSB_reg cclass=LDRSB_reg_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_64BL_ldst_regoff => {
  name       => '64-bit with shifted register offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b1 Rm:u=0bxxxxx option:u=0b011 S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10 && option == 0b011',
  docvars    => 'datatype=64 datatype-reguse=64-shifted-reg instr-class=general isa=A64 mnemonic=LDRSB offset-type=off-reg reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_regoff page=LDRSB_reg cclass=LDRSB_reg_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_32_ldst_immpost => {
  name       => '32-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b11 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b11',
  docvars    => 'address-form=post-indexed datatype=32 instr-class=general isa=A64 mnemonic=LDRSH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpost page=LDRSH_imm cclass=LDRSH_imm_post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_64_ldst_immpost => {
  name       => '64-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=post-indexed datatype=64 instr-class=general isa=A64 mnemonic=LDRSH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_immpost page=LDRSH_imm cclass=LDRSH_imm_post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_32_ldst_immpre => {
  name       => '32-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b11 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b11',
  docvars    => 'address-form=pre-indexed datatype=32 instr-class=general isa=A64 mnemonic=LDRSH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpre page=LDRSH_imm cclass=LDRSH_imm_pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_64_ldst_immpre => {
  name       => '64-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=pre-indexed datatype=64 instr-class=general isa=A64 mnemonic=LDRSH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_immpre page=LDRSH_imm cclass=LDRSH_imm_pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_32_ldst_pos => {
  name       => '32-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b11 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b11',
  docvars    => 'address-form=unsigned-scaled-offset datatype=32 instr-class=general isa=A64 mnemonic=LDRSH offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_pos page=LDRSH_imm cclass=LDRSH_imm_unsigned_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_64_ldst_pos => {
  name       => '64-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b10 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=unsigned-scaled-offset datatype=64 instr-class=general isa=A64 mnemonic=LDRSH offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_pos page=LDRSH_imm cclass=LDRSH_imm_unsigned_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_32_ldst_regoff => {
  name       => '32-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b11 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b11',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=LDRSH offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_regoff page=LDRSH_reg cclass=LDRSH_reg_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_64_ldst_regoff => {
  name       => '64-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=LDRSH offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_regoff page=LDRSH_reg cclass=LDRSH_reg_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSW_64_ldst_immpost => {
  name       => 'Post-index',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed datatype=64 instr-class=general isa=A64 mnemonic=LDRSW offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_immpost page=LDRSW_imm cclass=LDRSW_64_ldst_immpost',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSW_64_ldst_immpre => {
  name       => 'Pre-index',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed datatype=64 instr-class=general isa=A64 mnemonic=LDRSW offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_immpre page=LDRSW_imm cclass=LDRSW_64_ldst_immpre',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSW_64_ldst_pos => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b10 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset datatype=64 instr-class=general isa=A64 mnemonic=LDRSW offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_pos page=LDRSW_imm cclass=LDRSW_64_ldst_pos',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSW_64_loadlit => {
  name       => 'Literal',
  diagram    => 'opc:u=0b10 ig0=0b011 V=0b0 ig1=0b00 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=literal address-form-reg-type=literal-64-reg atomic-ops=LDRSW-64-reg instr-class=general isa=A64 mnemonic=LDRSW offset-type=off19s reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=loadlit page=LDRSW_lit cclass=LDRSW_64_loadlit',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSW_64_ldst_regoff => {
  name       => '64-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=LDRSW offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_regoff page=LDRSW_reg cclass=LDRSW_64_ldst_regoff',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDSET_32_memop => {
  name       => '32-bit LDSET',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=LDSET-32-reg instr-class=general isa=A64 mnemonic=LDSET reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSET cclass=LDSET_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDSETA_32_memop => {
  name       => '32-bit LDSETA',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=LDSETA-32-reg instr-class=general isa=A64 mnemonic=LDSETA reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSET cclass=LDSET_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDSETAL_32_memop => {
  name       => '32-bit LDSETAL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=LDSETAL-32-reg instr-class=general isa=A64 mnemonic=LDSETAL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSET cclass=LDSET_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDSETL_32_memop => {
  name       => '32-bit LDSETL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=LDSETL-32-reg instr-class=general isa=A64 mnemonic=LDSETL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSET cclass=LDSET_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDSET_64_memop => {
  name       => '64-bit LDSET',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=LDSET-64-reg instr-class=general isa=A64 mnemonic=LDSET reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSET cclass=LDSET_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDSETA_64_memop => {
  name       => '64-bit LDSETA',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=LDSETA-64-reg instr-class=general isa=A64 mnemonic=LDSETA reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSET cclass=LDSET_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDSETAL_64_memop => {
  name       => '64-bit LDSETAL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=LDSETAL-64-reg instr-class=general isa=A64 mnemonic=LDSETAL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSET cclass=LDSET_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDSETL_64_memop => {
  name       => '64-bit LDSETL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=LDSETL-64-reg instr-class=general isa=A64 mnemonic=LDSETL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSET cclass=LDSET_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDSETAB_32_memop => {
  name       => 'LDSETAB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSETAB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSETB cclass=LDSETB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDSETALB_32_memop => {
  name       => 'LDSETALB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSETALB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSETB cclass=LDSETB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDSETB_32_memop => {
  name       => 'LDSETB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSETB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSETB cclass=LDSETB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDSETLB_32_memop => {
  name       => 'LDSETLB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSETLB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSETB cclass=LDSETB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDSETAH_32_memop => {
  name       => 'LDSETAH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSETAH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSETH cclass=LDSETH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDSETALH_32_memop => {
  name       => 'LDSETALH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSETALH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSETH cclass=LDSETH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDSETH_32_memop => {
  name       => 'LDSETH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSETH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSETH cclass=LDSETH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDSETLH_32_memop => {
  name       => 'LDSETLH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSETLH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSETH cclass=LDSETH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
};

ENCODING LDSMAX_32_memop => {
  name       => '32-bit LDSMAX',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=LDSMAX-32-reg instr-class=general isa=A64 mnemonic=LDSMAX reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAX cclass=LDSMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMAXA_32_memop => {
  name       => '32-bit LDSMAXA',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=LDSMAXA-32-reg instr-class=general isa=A64 mnemonic=LDSMAXA reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAX cclass=LDSMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMAXAL_32_memop => {
  name       => '32-bit LDSMAXAL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=LDSMAXAL-32-reg instr-class=general isa=A64 mnemonic=LDSMAXAL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAX cclass=LDSMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMAXL_32_memop => {
  name       => '32-bit LDSMAXL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=LDSMAXL-32-reg instr-class=general isa=A64 mnemonic=LDSMAXL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAX cclass=LDSMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMAX_64_memop => {
  name       => '64-bit LDSMAX',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=LDSMAX-64-reg instr-class=general isa=A64 mnemonic=LDSMAX reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAX cclass=LDSMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMAXA_64_memop => {
  name       => '64-bit LDSMAXA',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=LDSMAXA-64-reg instr-class=general isa=A64 mnemonic=LDSMAXA reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAX cclass=LDSMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMAXAL_64_memop => {
  name       => '64-bit LDSMAXAL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=LDSMAXAL-64-reg instr-class=general isa=A64 mnemonic=LDSMAXAL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAX cclass=LDSMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMAXL_64_memop => {
  name       => '64-bit LDSMAXL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=LDSMAXL-64-reg instr-class=general isa=A64 mnemonic=LDSMAXL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAX cclass=LDSMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMAXAB_32_memop => {
  name       => 'LDSMAXAB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSMAXAB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAXB cclass=LDSMAXB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMAXALB_32_memop => {
  name       => 'LDSMAXALB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSMAXALB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAXB cclass=LDSMAXB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMAXB_32_memop => {
  name       => 'LDSMAXB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSMAXB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAXB cclass=LDSMAXB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMAXLB_32_memop => {
  name       => 'LDSMAXLB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSMAXLB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAXB cclass=LDSMAXB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMAXAH_32_memop => {
  name       => 'LDSMAXAH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSMAXAH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAXH cclass=LDSMAXH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMAXALH_32_memop => {
  name       => 'LDSMAXALH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSMAXALH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAXH cclass=LDSMAXH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMAXH_32_memop => {
  name       => 'LDSMAXH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSMAXH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAXH cclass=LDSMAXH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMAXLH_32_memop => {
  name       => 'LDSMAXLH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSMAXLH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAXH cclass=LDSMAXH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMIN_32_memop => {
  name       => '32-bit LDSMIN',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=LDSMIN-32-reg instr-class=general isa=A64 mnemonic=LDSMIN reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMIN cclass=LDSMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMINA_32_memop => {
  name       => '32-bit LDSMINA',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=LDSMINA-32-reg instr-class=general isa=A64 mnemonic=LDSMINA reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMIN cclass=LDSMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMINAL_32_memop => {
  name       => '32-bit LDSMINAL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=LDSMINAL-32-reg instr-class=general isa=A64 mnemonic=LDSMINAL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMIN cclass=LDSMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMINL_32_memop => {
  name       => '32-bit LDSMINL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=LDSMINL-32-reg instr-class=general isa=A64 mnemonic=LDSMINL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMIN cclass=LDSMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMIN_64_memop => {
  name       => '64-bit LDSMIN',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=LDSMIN-64-reg instr-class=general isa=A64 mnemonic=LDSMIN reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMIN cclass=LDSMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMINA_64_memop => {
  name       => '64-bit LDSMINA',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=LDSMINA-64-reg instr-class=general isa=A64 mnemonic=LDSMINA reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMIN cclass=LDSMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMINAL_64_memop => {
  name       => '64-bit LDSMINAL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=LDSMINAL-64-reg instr-class=general isa=A64 mnemonic=LDSMINAL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMIN cclass=LDSMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMINL_64_memop => {
  name       => '64-bit LDSMINL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=LDSMINL-64-reg instr-class=general isa=A64 mnemonic=LDSMINL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMIN cclass=LDSMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMINAB_32_memop => {
  name       => 'LDSMINAB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSMINAB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMINB cclass=LDSMINB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMINALB_32_memop => {
  name       => 'LDSMINALB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSMINALB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMINB cclass=LDSMINB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMINB_32_memop => {
  name       => 'LDSMINB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSMINB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMINB cclass=LDSMINB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMINLB_32_memop => {
  name       => 'LDSMINLB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSMINLB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMINB cclass=LDSMINB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMINAH_32_memop => {
  name       => 'LDSMINAH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSMINAH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMINH cclass=LDSMINH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMINALH_32_memop => {
  name       => 'LDSMINALH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSMINALH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMINH cclass=LDSMINH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMINH_32_memop => {
  name       => 'LDSMINH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSMINH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMINH cclass=LDSMINH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDSMINLH_32_memop => {
  name       => 'LDSMINLH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDSMINLH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMINH cclass=LDSMINH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
};

ENCODING LDTR_32_ldst_unpriv => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDTR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unpriv page=LDTR cclass=LDTR_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDTR_64_ldst_unpriv => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-plus-offset datatype=64 instr-class=general isa=A64 mnemonic=LDTR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_unpriv page=LDTR cclass=LDTR_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDTRB_32_ldst_unpriv => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDTRB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unpriv page=LDTRB cclass=LDTRB_32_ldst_unpriv',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDTRH_32_ldst_unpriv => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDTRH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unpriv page=LDTRH cclass=LDTRH_32_ldst_unpriv',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDTRSB_32_ldst_unpriv => {
  name       => '32-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b11 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b11',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDTRSB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unpriv page=LDTRSB cclass=LDTRSB_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDTRSB_64_ldst_unpriv => {
  name       => '64-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=base-plus-offset datatype=64 instr-class=general isa=A64 mnemonic=LDTRSB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_unpriv page=LDTRSB cclass=LDTRSB_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDTRSH_32_ldst_unpriv => {
  name       => '32-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b11 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b11',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDTRSH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unpriv page=LDTRSH cclass=LDTRSH_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDTRSH_64_ldst_unpriv => {
  name       => '64-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=base-plus-offset datatype=64 instr-class=general isa=A64 mnemonic=LDTRSH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_unpriv page=LDTRSH cclass=LDTRSH_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDTRSW_64_ldst_unpriv => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=64 instr-class=general isa=A64 mnemonic=LDTRSW offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_unpriv page=LDTRSW cclass=LDTRSW_64_ldst_unpriv',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDUMAX_32_memop => {
  name       => '32-bit LDUMAX',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=LDUMAX-32-reg instr-class=general isa=A64 mnemonic=LDUMAX reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAX cclass=LDUMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMAXA_32_memop => {
  name       => '32-bit LDUMAXA',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=LDUMAXA-32-reg instr-class=general isa=A64 mnemonic=LDUMAXA reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAX cclass=LDUMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMAXAL_32_memop => {
  name       => '32-bit LDUMAXAL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=LDUMAXAL-32-reg instr-class=general isa=A64 mnemonic=LDUMAXAL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAX cclass=LDUMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMAXL_32_memop => {
  name       => '32-bit LDUMAXL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=LDUMAXL-32-reg instr-class=general isa=A64 mnemonic=LDUMAXL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAX cclass=LDUMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMAX_64_memop => {
  name       => '64-bit LDUMAX',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=LDUMAX-64-reg instr-class=general isa=A64 mnemonic=LDUMAX reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAX cclass=LDUMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMAXA_64_memop => {
  name       => '64-bit LDUMAXA',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=LDUMAXA-64-reg instr-class=general isa=A64 mnemonic=LDUMAXA reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAX cclass=LDUMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMAXAL_64_memop => {
  name       => '64-bit LDUMAXAL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=LDUMAXAL-64-reg instr-class=general isa=A64 mnemonic=LDUMAXAL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAX cclass=LDUMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMAXL_64_memop => {
  name       => '64-bit LDUMAXL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=LDUMAXL-64-reg instr-class=general isa=A64 mnemonic=LDUMAXL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAX cclass=LDUMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMAXAB_32_memop => {
  name       => 'LDUMAXAB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDUMAXAB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAXB cclass=LDUMAXB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMAXALB_32_memop => {
  name       => 'LDUMAXALB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDUMAXALB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAXB cclass=LDUMAXB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMAXB_32_memop => {
  name       => 'LDUMAXB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDUMAXB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAXB cclass=LDUMAXB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMAXLB_32_memop => {
  name       => 'LDUMAXLB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDUMAXLB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAXB cclass=LDUMAXB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMAXAH_32_memop => {
  name       => 'LDUMAXAH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDUMAXAH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAXH cclass=LDUMAXH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMAXALH_32_memop => {
  name       => 'LDUMAXALH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDUMAXALH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAXH cclass=LDUMAXH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMAXH_32_memop => {
  name       => 'LDUMAXH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDUMAXH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAXH cclass=LDUMAXH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMAXLH_32_memop => {
  name       => 'LDUMAXLH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDUMAXLH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAXH cclass=LDUMAXH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMIN_32_memop => {
  name       => '32-bit LDUMIN',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=LDUMIN-32-reg instr-class=general isa=A64 mnemonic=LDUMIN reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMIN cclass=LDUMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMINA_32_memop => {
  name       => '32-bit LDUMINA',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=LDUMINA-32-reg instr-class=general isa=A64 mnemonic=LDUMINA reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMIN cclass=LDUMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMINAL_32_memop => {
  name       => '32-bit LDUMINAL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=LDUMINAL-32-reg instr-class=general isa=A64 mnemonic=LDUMINAL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMIN cclass=LDUMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMINL_32_memop => {
  name       => '32-bit LDUMINL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=LDUMINL-32-reg instr-class=general isa=A64 mnemonic=LDUMINL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMIN cclass=LDUMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMIN_64_memop => {
  name       => '64-bit LDUMIN',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=LDUMIN-64-reg instr-class=general isa=A64 mnemonic=LDUMIN reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMIN cclass=LDUMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMINA_64_memop => {
  name       => '64-bit LDUMINA',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=LDUMINA-64-reg instr-class=general isa=A64 mnemonic=LDUMINA reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMIN cclass=LDUMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMINAL_64_memop => {
  name       => '64-bit LDUMINAL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=LDUMINAL-64-reg instr-class=general isa=A64 mnemonic=LDUMINAL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMIN cclass=LDUMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMINL_64_memop => {
  name       => '64-bit LDUMINL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=LDUMINL-64-reg instr-class=general isa=A64 mnemonic=LDUMINL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMIN cclass=LDUMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMINAB_32_memop => {
  name       => 'LDUMINAB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDUMINAB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMINB cclass=LDUMINB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMINALB_32_memop => {
  name       => 'LDUMINALB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDUMINALB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMINB cclass=LDUMINB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMINB_32_memop => {
  name       => 'LDUMINB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDUMINB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMINB cclass=LDUMINB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMINLB_32_memop => {
  name       => 'LDUMINLB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDUMINLB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMINB cclass=LDUMINB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMINAH_32_memop => {
  name       => 'LDUMINAH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDUMINAH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMINH cclass=LDUMINH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMINALH_32_memop => {
  name       => 'LDUMINALH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDUMINALH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMINH cclass=LDUMINH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMINH_32_memop => {
  name       => 'LDUMINH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDUMINH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMINH cclass=LDUMINH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUMINLH_32_memop => {
  name       => 'LDUMINLH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDUMINLH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMINH cclass=LDUMINH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
};

ENCODING LDUR_32_ldst_unscaled => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-32-reg atomic-ops=LDUR-32-reg instr-class=general isa=A64 mnemonic=LDUR offset-type=off9s_u reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDUR_gen cclass=LDUR_gen_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDUR_64_ldst_unscaled => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-64-reg atomic-ops=LDUR-64-reg instr-class=general isa=A64 mnemonic=LDUR offset-type=off9s_u reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDUR_gen cclass=LDUR_gen_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDURB_32_ldst_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDURB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDURB cclass=LDURB_32_ldst_unscaled',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDURH_32_ldst_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDURH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDURH cclass=LDURH_32_ldst_unscaled',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDURSB_32_ldst_unscaled => {
  name       => '32-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b11 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b11',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDURSB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDURSB cclass=LDURSB_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDURSB_64_ldst_unscaled => {
  name       => '64-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=base-plus-offset datatype=64 instr-class=general isa=A64 mnemonic=LDURSB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDURSB cclass=LDURSB_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDURSH_32_ldst_unscaled => {
  name       => '32-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b11 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b11',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDURSH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDURSH cclass=LDURSH_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDURSH_64_ldst_unscaled => {
  name       => '64-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=base-plus-offset datatype=64 instr-class=general isa=A64 mnemonic=LDURSH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDURSH cclass=LDURSH_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDURSW_64_ldst_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=64 instr-class=general isa=A64 mnemonic=LDURSW offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDURSW cclass=LDURSW_64_ldst_unscaled',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDXP_LP32_ldstexcl => {
  name       => '32-bit',
  diagram    => 'ig0=0b1 sz:u=0b0 ig1=0b001000 o2=0b0 L:u=0b1 o1=0b1 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sz == 0b0',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-pair-32 atomic-ops=LDXP-pair-32 instr-class=general isa=A64 mnemonic=LDXP reg-type=pair-32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDXP cclass=LDXP_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDXP_LP64_ldstexcl => {
  name       => '64-bit',
  diagram    => 'ig0=0b1 sz:u=0b1 ig1=0b001000 o2=0b0 L:u=0b1 o1=0b1 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sz == 0b1',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-pair-64 atomic-ops=LDXP-pair-64 instr-class=general isa=A64 mnemonic=LDXP reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDXP cclass=LDXP_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDXR_LR32_ldstexcl => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b001000 o2=0b0 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDXR-32-reg instr-class=general isa=A64 mnemonic=LDXR reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDXR cclass=LDXR_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDXR_LR64_ldstexcl => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b001000 o2=0b0 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-64-reg atomic-ops=LDXR-64-reg instr-class=general isa=A64 mnemonic=LDXR reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDXR cclass=LDXR_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDXRB_LR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b0 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDXRB-32-reg instr-class=general isa=A64 mnemonic=LDXRB reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDXRB cclass=LDXRB_LR32_ldstexcl',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDXRH_LR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b0 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDXRH-32-reg instr-class=general isa=A64 mnemonic=LDXRH reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDXRH cclass=LDXRH_LR32_ldstexcl',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LSLV_32_dp_2src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=LSLV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_2src page=LSLV cclass=LSLV_general',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
};

ENCODING LSLV_64_dp_2src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=LSLV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_2src page=LSLV cclass=LSLV_general',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
};

ENCODING LSRV_32_dp_2src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=LSRV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_2src page=LSRV cclass=LSRV_general',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
};

ENCODING LSRV_64_dp_2src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=LSRV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_2src page=LSRV cclass=LSRV_general',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
};

ENCODING MADD_32A_dp_3src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op54=0b00 ig0=0b11011 op31=0b000 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=MADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_3src page=MADD cclass=MADD_general',
  categories => 'GENERAL|CARITHMETIC',
};

ENCODING MADD_64A_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op54=0b00 ig0=0b11011 op31=0b000 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=MADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=MADD cclass=MADD_general',
  categories => 'GENERAL|CARITHMETIC',
};

ENCODING MOVK_32_movewide => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b11 ig0=0b100101 hw:u=0b0x imm16:u=0bxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && hw == 0b0x',
  docvars    => 'datatype=32 immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVK',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=movewide page=MOVK cclass=MOVK_imm18_packed',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOVK_64_movewide => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b11 ig0=0b100101 hw:u=0bxx imm16:u=0bxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVK',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=movewide page=MOVK cclass=MOVK_imm18_packed',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOVN_32_movewide => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b00 ig0=0b100101 hw:u=0b0x imm16:u=0bxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && hw == 0b0x',
  docvars    => 'datatype=32 immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=movewide page=MOVN cclass=MOVN_imm18_packed',
  categories => 'GENERAL|DATA_TRANSFER|BITWISE',
};

ENCODING MOVN_64_movewide => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b00 ig0=0b100101 hw:u=0bxx imm16:u=0bxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=movewide page=MOVN cclass=MOVN_imm18_packed',
  categories => 'GENERAL|DATA_TRANSFER|BITWISE',
};

ENCODING MOVZ_32_movewide => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b10 ig0=0b100101 hw:u=0b0x imm16:u=0bxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && hw == 0b0x',
  docvars    => 'datatype=32 immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=movewide page=MOVZ cclass=MOVZ_imm18_packed',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOVZ_64_movewide => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b10 ig0=0b100101 hw:u=0bxx imm16:u=0bxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=movewide page=MOVZ cclass=MOVZ_imm18_packed',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MRS_RS_systemmove => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b1 op0=0b1 o0:u=0bx op1:u=0bxxx CRn:u=0bxxxx CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=MRS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=systemmove page=MRS cclass=MRS_RS_systemmove',
  categories => 'SYSTEM|DATA_TRANSFER',
};

ENCODING MSR_SI_pstate => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1:u=0bxxx CRn=0b0100 CRm:u=0bxxxx op2:u=0bxxx Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=MSR msr-sysreg-target=whole-register',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=control iclass=pstate page=MSR_imm cclass=MSR_SI_pstate',
  pstate     => 'A=RCW D=RCW DIT=CW F=RCW I=RCW PAN=CW SP=CW SSBS=CW TCO=CW UAO=CW',
  categories => 'SYSTEM|DATA_TRANSFER',
};

ENCODING MSR_SR_systemmove => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b1 o0:u=0bx op1:u=0bxxx CRn:u=0bxxxx CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=MSR msr-sysreg-target=register-field',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=systemmove page=MSR_reg cclass=MSR_SR_systemmove',
  categories => 'SYSTEM|DATA_TRANSFER',
};

ENCODING MSUB_32A_dp_3src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op54=0b00 ig0=0b11011 op31=0b000 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=MSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_3src page=MSUB cclass=MSUB_general',
  categories => 'GENERAL|CARITHMETIC',
};

ENCODING MSUB_64A_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op54=0b00 ig0=0b11011 op31=0b000 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=MSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=MSUB cclass=MSUB_general',
  categories => 'GENERAL|CARITHMETIC',
};

ENCODING NOP_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0000 op2:u=0b000 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=NOP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=NOP cclass=NOP_HI_hints',
  categories => 'SYSTEM|NOP',
};

ENCODING ORN_32_log_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b01 ig0=0b01010 shift:u=0bxx N:u=0b1 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=no-s datatype=32 datatype-reguse=32-shifted-reg instr-class=general isa=A64 mnemonic=ORN reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=log_shift page=ORN_log_shift cclass=ORN_log_shift_no_s',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORN_64_log_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b01 ig0=0b01010 shift:u=0bxx N:u=0b1 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=no-s datatype=64 datatype-reguse=64-shifted-reg instr-class=general isa=A64 mnemonic=ORN reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=log_shift page=ORN_log_shift cclass=ORN_log_shift_no_s',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORR_32_log_imm => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b01 ig0=0b100100 N:u=0b0 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  docvars    => 'cond-setting=no-s datatype=32 immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=ORR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=log_imm page=ORR_log_imm cclass=ORR_log_imm_no_s',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORR_64_log_imm => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b01 ig0=0b100100 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=no-s datatype=64 immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=ORR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=log_imm page=ORR_log_imm cclass=ORR_log_imm_no_s',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORR_32_log_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b01 ig0=0b01010 shift:u=0bxx N:u=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=no-s datatype=32 datatype-reguse=32-shifted-reg instr-class=general isa=A64 mnemonic=ORR reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=log_shift page=ORR_log_shift cclass=ORR_log_shift_no_s',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORR_64_log_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b01 ig0=0b01010 shift:u=0bxx N:u=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=no-s datatype=64 datatype-reguse=64-shifted-reg instr-class=general isa=A64 mnemonic=ORR reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=log_shift page=ORR_log_shift cclass=ORR_log_shift_no_s',
  categories => 'GENERAL|BITWISE',
};

ENCODING PACDA_64P_dp_1src => {
  name       => 'PACDA',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0b0 opcode[2:0]=0b010 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'Z == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=PACDA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=PACDA cclass=PACDA_general',
  exceptions => 'PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING PACDZA_64Z_dp_1src => {
  name       => 'PACDZA',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0b1 opcode[2:0]=0b010 Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'Z == 0b1 && Rn == 0b11111',
  docvars    => 'instr-class=general isa=A64 mnemonic=PACDZA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=PACDA cclass=PACDA_general',
  exceptions => 'PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING PACDB_64P_dp_1src => {
  name       => 'PACDB',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0b0 opcode[2:0]=0b011 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'Z == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=PACDB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=PACDB cclass=PACDB_general',
  exceptions => 'PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING PACDZB_64Z_dp_1src => {
  name       => 'PACDZB',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0b1 opcode[2:0]=0b011 Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'Z == 0b1 && Rn == 0b11111',
  docvars    => 'instr-class=general isa=A64 mnemonic=PACDZB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=PACDB cclass=PACDB_general',
  exceptions => 'PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING PACGA_64P_dp_2src => {
  name       => 'Integer',
  diagram    => 'sf=0b1 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b001100 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=PACGA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=PACGA cclass=PACGA_64P_dp_2src',
  exceptions => 'PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING PACIA_64P_dp_1src => {
  name       => 'PACIA',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0b0 opcode[2:0]=0b000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'Z == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=PACIA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=PACIA cclass=PACIA_general',
  exceptions => 'PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING PACIZA_64Z_dp_1src => {
  name       => 'PACIZA',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0b1 opcode[2:0]=0b000 Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'Z == 0b1 && Rn == 0b11111',
  docvars    => 'instr-class=general isa=A64 mnemonic=PACIZA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=PACIA cclass=PACIA_general',
  exceptions => 'PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING PACIA1716_HI_hints => {
  name       => 'PACIA1716',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0001 op2:u=0b000 Rt=0b11111',
  bitdiffs   => 'CRm == 0b0001 && op2 == 0b000',
  docvars    => 'instr-class=system isa=A64 mnemonic=PACIA1716',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=PACIA cclass=PACIA_system',
  exceptions => 'PACTRAP',
  categories => 'SYSTEM|PAC',
};

ENCODING PACIASP_HI_hints => {
  name       => 'PACIASP',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0011 op2:u=0b001 Rt=0b11111',
  bitdiffs   => 'CRm == 0b0011 && op2 == 0b001',
  docvars    => 'instr-class=system isa=A64 mnemonic=PACIASP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=PACIA cclass=PACIA_system',
  exceptions => 'PACTRAP',
  categories => 'SYSTEM|PAC',
};

ENCODING PACIAZ_HI_hints => {
  name       => 'PACIAZ',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0011 op2:u=0b000 Rt=0b11111',
  bitdiffs   => 'CRm == 0b0011 && op2 == 0b000',
  docvars    => 'instr-class=system isa=A64 mnemonic=PACIAZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=PACIA cclass=PACIA_system',
  exceptions => 'PACTRAP',
  categories => 'SYSTEM|PAC',
};

ENCODING PACIB_64P_dp_1src => {
  name       => 'PACIB',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0b0 opcode[2:0]=0b001 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'Z == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=PACIB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=PACIB cclass=PACIB_general',
  exceptions => 'PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING PACIZB_64Z_dp_1src => {
  name       => 'PACIZB',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0b1 opcode[2:0]=0b001 Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'Z == 0b1 && Rn == 0b11111',
  docvars    => 'instr-class=general isa=A64 mnemonic=PACIZB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=PACIB cclass=PACIB_general',
  exceptions => 'PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING PACIB1716_HI_hints => {
  name       => 'PACIB1716',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0001 op2:u=0b010 Rt=0b11111',
  bitdiffs   => 'CRm == 0b0001 && op2 == 0b010',
  docvars    => 'instr-class=system isa=A64 mnemonic=PACIB1716',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=PACIB cclass=PACIB_system',
  exceptions => 'PACTRAP',
  categories => 'SYSTEM|PAC',
};

ENCODING PACIBSP_HI_hints => {
  name       => 'PACIBSP',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0011 op2:u=0b011 Rt=0b11111',
  bitdiffs   => 'CRm == 0b0011 && op2 == 0b011',
  docvars    => 'instr-class=system isa=A64 mnemonic=PACIBSP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=PACIB cclass=PACIB_system',
  exceptions => 'PACTRAP',
  categories => 'SYSTEM|PAC',
};

ENCODING PACIBZ_HI_hints => {
  name       => 'PACIBZ',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0011 op2:u=0b010 Rt=0b11111',
  bitdiffs   => 'CRm == 0b0011 && op2 == 0b010',
  docvars    => 'instr-class=system isa=A64 mnemonic=PACIBZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=PACIB cclass=PACIB_system',
  exceptions => 'PACTRAP',
  categories => 'SYSTEM|PAC',
};

ENCODING PRFM_P_ldst_pos => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b10 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset instr-class=general isa=A64 mnemonic=PRFM offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_pos page=PRFM_imm cclass=PRFM_P_ldst_pos',
  categories => 'GENERAL|PREFETCH',
};

ENCODING PRFM_P_loadlit => {
  name       => 'Literal',
  diagram    => 'opc:u=0b11 ig0=0b011 V=0b0 ig1=0b00 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=literal instr-class=general isa=A64 mnemonic=PRFM offset-type=off19s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=loadlit page=PRFM_lit cclass=PRFM_P_loadlit',
  categories => 'GENERAL|PREFETCH',
};

ENCODING PRFM_P_ldst_regoff => {
  name       => 'Integer',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=PRFM offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_regoff page=PRFM_reg cclass=PRFM_P_ldst_regoff',
  categories => 'GENERAL|PREFETCH',
};

ENCODING PRFUM_P_ldst_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=PRFUM offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unscaled page=PRFUM cclass=PRFUM_P_ldst_unscaled',
  categories => 'GENERAL|PREFETCH',
};

ENCODING PSB_HC_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0010 op2:u=0b001 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=PSB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=PSB cclass=PSB_HC_hints',
  categories => 'SYSTEM',
};

ENCODING PSSBB_only_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm:u=0b0100 ig1=0b1 opc:u=0b00 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=PSSBB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=PSSBB cclass=PSSBB_only_barriers',
  categories => 'SYSTEM',
};

ENCODING RBIT_32_dp_1src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:2]=0b0000 opcode[1:0]=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=RBIT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_1src page=RBIT_int cclass=RBIT_int_general',
  categories => 'GENERAL|MISC',
};

ENCODING RBIT_64_dp_1src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:2]=0b0000 opcode[1:0]=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=RBIT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_1src page=RBIT_int cclass=RBIT_int_general',
  categories => 'GENERAL|MISC',
};

ENCODING RET_64R_branch_reg => {
  name       => 'Integer',
  diagram    => 'ig0=0b1101011 Z:u=0b0 opc[2:1]=0b0 op:u=0b10 op2=0b11111 op3[5:2]=0b0000 A:u=0b0 M:u=0b0 Rn:u=0bxxxxx Rm:u=0b00000',
  docvars    => 'instr-class=general isa=A64 mnemonic=RET',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=RET cclass=RET_64R_branch_reg',
  categories => 'GENERAL',
};

ENCODING RETAA_64E_branch_reg => {
  name       => 'RETAA',
  diagram    => 'ig0=0b1101011 Z:u=0b0 opc[2:1]=0b0 op:u=0b10 op2=0b11111 op3[5:2]=0b0000 A:u=0b1 M:u=0b0 Rn:u=0b11111 Rm:u=0b11111',
  bitdiffs   => 'M == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=RETAA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=RETA cclass=RETA_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING RETAB_64E_branch_reg => {
  name       => 'RETAB',
  diagram    => 'ig0=0b1101011 Z:u=0b0 opc[2:1]=0b0 op:u=0b10 op2=0b11111 op3[5:2]=0b0000 A:u=0b1 M:u=0b1 Rn:u=0b11111 Rm:u=0b11111',
  bitdiffs   => 'M == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=RETAB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=RETA cclass=RETA_general',
  exceptions => 'PACFAIL|PACTRAP',
  categories => 'GENERAL|PAC',
};

ENCODING REV_32_dp_1src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:2]=0b0000 opc:u=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && opc == 0b10',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=REV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_1src page=REV cclass=REV_general',
  categories => 'GENERAL|MISC',
};

ENCODING REV_64_dp_1src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:2]=0b0000 opc:u=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && opc == 0b11',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=REV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_1src page=REV cclass=REV_general',
  categories => 'GENERAL|MISC',
};

ENCODING REV16_32_dp_1src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:2]=0b0000 opc:u=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=REV16',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_1src page=REV16_int cclass=REV16_int_general',
  categories => 'GENERAL|MISC',
};

ENCODING REV16_64_dp_1src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:2]=0b0000 opc:u=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=REV16',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_1src page=REV16_int cclass=REV16_int_general',
  categories => 'GENERAL|MISC',
};

ENCODING REV32_64_dp_1src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:2]=0b0000 opc:u=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=REV32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_1src page=REV32_int cclass=REV32_64_dp_1src',
  categories => 'GENERAL|MISC',
};

ENCODING RMIF_only_rmif => {
  name       => 'Integer',
  diagram    => 'sf:u=0b1 op=0b0 S=0b1 ig0=0b11010000 imm6:u=0bxxxxxx ig1=0b00001 Rn:u=0bxxxxx o2=0b0 mask:u=0bxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=RMIF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=rmif page=RMIF cclass=RMIF_only_rmif',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|INSERTION',
};

ENCODING RORV_32_dp_2src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=RORV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_2src page=RORV cclass=RORV_general',
  categories => 'GENERAL|ROTATE_RIGHT',
};

ENCODING RORV_64_dp_2src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=RORV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_2src page=RORV cclass=RORV_general',
  categories => 'GENERAL|ROTATE_RIGHT',
};

ENCODING SB_only_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm:u=0bZZZZ ig1=0b1 opc:u=0b11 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=SB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=SB cclass=SB_only_barriers',
  categories => 'SYSTEM',
};

ENCODING SBC_32_addsub_carry => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S:u=0b0 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=no-s datatype=32 instr-class=general isa=A64 mnemonic=SBC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_carry page=SBC cclass=SBC_no_s',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBC_64_addsub_carry => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S:u=0b0 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=no-s datatype=64 instr-class=general isa=A64 mnemonic=SBC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_carry page=SBC cclass=SBC_no_s',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBCS_32_addsub_carry => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S:u=0b1 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=S datatype=32 instr-class=general isa=A64 mnemonic=SBCS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_carry page=SBCS cclass=SBCS_s',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBCS_64_addsub_carry => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S:u=0b1 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=S datatype=64 instr-class=general isa=A64 mnemonic=SBCS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_carry page=SBCS cclass=SBCS_s',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBFM_32M_bitfield => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b00 ig0=0b100110 N:u=0b0 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  docvars    => 'bitfield-fill=signed-fill datatype=32 instr-class=general isa=A64 mnemonic=SBFM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=bitfield page=SBFM cclass=SBFM_signed_fill',
  categories => 'GENERAL|BITFIELD|DATA_TRANSFER|SIGNED',
};

ENCODING SBFM_64M_bitfield => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b00 ig0=0b100110 N:u=0b1 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && N == 0b1',
  docvars    => 'bitfield-fill=signed-fill datatype=64 instr-class=general isa=A64 mnemonic=SBFM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=bitfield page=SBFM cclass=SBFM_signed_fill',
  categories => 'GENERAL|BITFIELD|DATA_TRANSFER|SIGNED',
};

ENCODING SDIV_32_dp_2src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b00001 o1:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=SDIV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_2src page=SDIV cclass=SDIV_general',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SDIV_64_dp_2src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b00001 o1:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=SDIV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_2src page=SDIV cclass=SDIV_general',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SETF8_only_setf => {
  name       => 'SETF8',
  diagram    => 'sf:u=0b0 op=0b0 S=0b1 ig0=0b11010000 opcode2=0b000000 sz:u=0b0 ig1=0b0010 Rn:u=0bxxxxx o3=0b0 mask=0b1101',
  bitdiffs   => 'sz == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=SETF8',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=setf page=SETF cclass=SETF_general',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL',
};

ENCODING SETF16_only_setf => {
  name       => 'SETF16',
  diagram    => 'sf:u=0b0 op=0b0 S=0b1 ig0=0b11010000 opcode2=0b000000 sz:u=0b1 ig1=0b0010 Rn:u=0bxxxxx o3=0b0 mask=0b1101',
  bitdiffs   => 'sz == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=SETF16',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=setf page=SETF cclass=SETF_general',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL',
};

ENCODING SEV_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0000 op2:u=0b100 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=SEV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=SEV cclass=SEV_HI_hints',
  categories => 'SYSTEM',
};

ENCODING SEVL_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0000 op2:u=0b101 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=SEVL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=SEVL cclass=SEVL_HI_hints',
  categories => 'SYSTEM',
};

ENCODING SMADDL_64WA_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b0 ig1=0b01 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=SMADDL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=SMADDL cclass=SMADDL_SMADDL_64WA_dp_3src',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMC_EX_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b000 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL=0b11',
  docvars    => 'instr-class=system isa=A64 mnemonic=SMC',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v0-A alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=SMC cclass=SMC_EX_exception',
  exceptions => 'MONITORCALL',
  categories => 'SYSTEM|CALL',
};

ENCODING SMSUBL_64WA_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b0 ig1=0b01 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=SMSUBL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=SMSUBL cclass=SMSUBL_SMSUBL_64WA_dp_3src',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMULH_64_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b0 ig1=0b10 Rm:u=0bxxxxx o0=0b0 Ra:u=0bSSSSS Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=SMULH',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=SMULH cclass=SMULH_64_dp_3src',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SSBB_only_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm:u=0b0000 ig1=0b1 opc:u=0b00 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=SSBB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=SSBB cclass=SSBB_only_barriers',
  categories => 'SYSTEM',
};

ENCODING ST2G_64Spost_ldsttags => {
  name       => 'Post-index',
  diagram    => 'ig0=0b11011001 opc=0b10 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b01 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=general isa=A64 mnemonic=ST2G',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=ST2G cclass=ST2G_64Spost_ldsttags',
  exceptions => 'SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|STORE',
};

ENCODING ST2G_64Spre_ldsttags => {
  name       => 'Pre-index',
  diagram    => 'ig0=0b11011001 opc=0b10 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b11 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=general isa=A64 mnemonic=ST2G',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=ST2G cclass=ST2G_64Spre_ldsttags',
  exceptions => 'SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|STORE',
};

ENCODING ST2G_64Soffset_ldsttags => {
  name       => 'Signed offset',
  diagram    => 'ig0=0b11011001 opc=0b10 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b10 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset instr-class=general isa=A64 mnemonic=ST2G',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=ST2G cclass=ST2G_64Soffset_ldsttags',
  exceptions => 'SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|STORE',
};

ENCODING STG_64Spost_ldsttags => {
  name       => 'Post-index',
  diagram    => 'ig0=0b11011001 opc=0b00 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b01 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=general isa=A64 mnemonic=STG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STG cclass=STG_64Spost_ldsttags',
  exceptions => 'SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|STORE',
};

ENCODING STG_64Spre_ldsttags => {
  name       => 'Pre-index',
  diagram    => 'ig0=0b11011001 opc=0b00 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b11 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=general isa=A64 mnemonic=STG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STG cclass=STG_64Spre_ldsttags',
  exceptions => 'SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|STORE',
};

ENCODING STG_64Soffset_ldsttags => {
  name       => 'Signed offset',
  diagram    => 'ig0=0b11011001 opc=0b00 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b10 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset instr-class=general isa=A64 mnemonic=STG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STG cclass=STG_64Soffset_ldsttags',
  exceptions => 'SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|STORE',
};

ENCODING STGM_64bulk_ldsttags => {
  name       => 'Integer',
  diagram    => 'ig0=0b11011001 opc=0b10 ig1=0b1 imm9=0b000000000 op2=0b00 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=STGM',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STGM cclass=STGM_64bulk_ldsttags',
  exceptions => 'SPALIGNMENT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|STORE',
};

ENCODING STGP_64_ldstpair_post => {
  name       => 'Post-index',
  diagram    => 'opc=0b01 ig0=0b101 V=0b0 ig1=0b001 L=0b0 simm7:u=0bxxxxxxx Xt2:u=0bxxxxx Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-pair-64 atomic-ops=STGP-pair-64 instr-class=general isa=A64 mnemonic=STGP offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_post page=STGP cclass=STGP_64_ldstpair_post',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|STORE',
};

ENCODING STGP_64_ldstpair_pre => {
  name       => 'Pre-index',
  diagram    => 'opc=0b01 ig0=0b101 V=0b0 ig1=0b011 L=0b0 simm7:u=0bxxxxxxx Xt2:u=0bxxxxx Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-pair-64 atomic-ops=STGP-pair-64 instr-class=general isa=A64 mnemonic=STGP offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_pre page=STGP cclass=STGP_64_ldstpair_pre',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|STORE',
};

ENCODING STGP_64_ldstpair_off => {
  name       => 'Signed offset',
  diagram    => 'opc=0b01 ig0=0b101 V=0b0 ig1=0b010 L=0b0 simm7:u=0bxxxxxxx Xt2:u=0bxxxxx Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-64 atomic-ops=STGP-pair-64 instr-class=general isa=A64 mnemonic=STGP offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_off page=STGP cclass=STGP_64_ldstpair_off',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|STORE',
};

ENCODING STLLR_SL32_ldstexcl => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLLR-32-reg instr-class=general isa=A64 mnemonic=STLLR reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLLR cclass=STLLR_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLLR_SL64_ldstexcl => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-64-reg atomic-ops=STLLR-64-reg instr-class=general isa=A64 mnemonic=STLLR reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLLR cclass=STLLR_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLLRB_SL32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLLRB-32-reg instr-class=general isa=A64 mnemonic=STLLRB reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLLRB cclass=STLLRB_SL32_ldstexcl',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLLRH_SL32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLLRH-32-reg instr-class=general isa=A64 mnemonic=STLLRH reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLLRH cclass=STLLRH_SL32_ldstexcl',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLR_SL32_ldstexcl => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLR-32-reg instr-class=general isa=A64 mnemonic=STLR reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLR cclass=STLR_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLR_SL64_ldstexcl => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-64-reg atomic-ops=STLR-64-reg instr-class=general isa=A64 mnemonic=STLR reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLR cclass=STLR_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLRB_SL32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLRB-32-reg instr-class=general isa=A64 mnemonic=STLRB reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLRB cclass=STLRB_SL32_ldstexcl',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLRH_SL32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLRH-32-reg instr-class=general isa=A64 mnemonic=STLRH reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLRH cclass=STLRH_SL32_ldstexcl',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLUR_32_ldapstl_unscaled => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b011001 opc:u=0b00 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-32-reg atomic-ops=STLUR-32-reg instr-class=general isa=A64 mnemonic=STLUR offset-type=off9s_u reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=STLUR_gen cclass=STLUR_gen_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLUR_64_ldapstl_unscaled => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b011001 opc:u=0b00 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-64-reg atomic-ops=STLUR-64-reg instr-class=general isa=A64 mnemonic=STLUR offset-type=off9s_u reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=STLUR_gen cclass=STLUR_gen_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLURB_32_ldapstl_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b00 ig0=0b011001 opc:u=0b00 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STLURB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=STLURB cclass=STLURB_32_ldapstl_unscaled',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLURH_32_ldapstl_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b01 ig0=0b011001 opc:u=0b00 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STLURH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=STLURH cclass=STLURH_32_ldapstl_unscaled',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLXP_SP32_ldstexcl => {
  name       => '32-bit',
  diagram    => 'ig0=0b1 sz:u=0b0 ig1=0b001000 o2=0b0 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b1 Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sz == 0b0',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-pair-32 atomic-ops=STLXP-pair-32 instr-class=general isa=A64 mnemonic=STLXP reg-type=pair-32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLXP cclass=STLXP_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLXP_SP64_ldstexcl => {
  name       => '64-bit',
  diagram    => 'ig0=0b1 sz:u=0b1 ig1=0b001000 o2=0b0 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b1 Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sz == 0b1',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-pair-64 atomic-ops=STLXP-pair-64 instr-class=general isa=A64 mnemonic=STLXP reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLXP cclass=STLXP_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLXR_SR32_ldstexcl => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b001000 o2=0b0 L:u=0b0 o1=0b0 Rs:u=0bxxxxx o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLXR-32-reg instr-class=general isa=A64 mnemonic=STLXR reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLXR cclass=STLXR_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLXR_SR64_ldstexcl => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b001000 o2=0b0 L:u=0b0 o1=0b0 Rs:u=0bxxxxx o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-64-reg atomic-ops=STLXR-64-reg instr-class=general isa=A64 mnemonic=STLXR reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLXR cclass=STLXR_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLXRB_SR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b0 L:u=0b0 o1=0b0 Rs:u=0bxxxxx o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLXRB-32-reg instr-class=general isa=A64 mnemonic=STLXRB reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLXRB cclass=STLXRB_SR32_ldstexcl',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLXRH_SR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b0 L:u=0b0 o1=0b0 Rs:u=0bxxxxx o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLXRH-32-reg instr-class=general isa=A64 mnemonic=STLXRH reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLXRH cclass=STLXRH_SR32_ldstexcl',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STNP_32_ldstnapair_offs => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b101 V=0b0 ig1=0b000 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-32 atomic-ops=STNP-pair-32 instr-class=general isa=A64 mnemonic=STNP offset-type=off7s_s reg-type=pair-32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstnapair_offs page=STNP_gen cclass=STNP_gen_signed_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|HINT|STORE',
};

ENCODING STNP_64_ldstnapair_offs => {
  name       => '64-bit',
  diagram    => 'opc:u=0b10 ig0=0b101 V=0b0 ig1=0b000 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-64 atomic-ops=STNP-pair-64 instr-class=general isa=A64 mnemonic=STNP offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstnapair_offs page=STNP_gen cclass=STNP_gen_signed_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|HINT|STORE',
};

ENCODING STP_32_ldstpair_post => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b101 V=0b0 ig1=0b001 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-pair-32 atomic-ops=STP-pair-32 instr-class=general isa=A64 mnemonic=STP offset-type=off7s_s reg-type=pair-32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_post page=STP_gen cclass=STP_gen_post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STP_64_ldstpair_post => {
  name       => '64-bit',
  diagram    => 'opc:u=0b10 ig0=0b101 V=0b0 ig1=0b001 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-pair-64 atomic-ops=STP-pair-64 instr-class=general isa=A64 mnemonic=STP offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_post page=STP_gen cclass=STP_gen_post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STP_32_ldstpair_pre => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b101 V=0b0 ig1=0b011 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-pair-32 atomic-ops=STP-pair-32 instr-class=general isa=A64 mnemonic=STP offset-type=off7s_s reg-type=pair-32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_pre page=STP_gen cclass=STP_gen_pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STP_64_ldstpair_pre => {
  name       => '64-bit',
  diagram    => 'opc:u=0b10 ig0=0b101 V=0b0 ig1=0b011 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-pair-64 atomic-ops=STP-pair-64 instr-class=general isa=A64 mnemonic=STP offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_pre page=STP_gen cclass=STP_gen_pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STP_32_ldstpair_off => {
  name       => '32-bit',
  diagram    => 'opc:u=0b00 ig0=0b101 V=0b0 ig1=0b010 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b00',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-32 atomic-ops=STP-pair-32 instr-class=general isa=A64 mnemonic=STP offset-type=off7s_s reg-type=pair-32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_off page=STP_gen cclass=STP_gen_signed_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STP_64_ldstpair_off => {
  name       => '64-bit',
  diagram    => 'opc:u=0b10 ig0=0b101 V=0b0 ig1=0b010 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'opc == 0b10',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-64 atomic-ops=STP-pair-64 instr-class=general isa=A64 mnemonic=STP offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_off page=STP_gen cclass=STP_gen_signed_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_32_ldst_immpost => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-32-reg atomic-ops=STR-32-reg instr-class=general isa=A64 mnemonic=STR offset-type=off9s_u reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_immpost page=STR_imm_gen cclass=STR_imm_gen_post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_64_ldst_immpost => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-64-reg atomic-ops=STR-64-reg instr-class=general isa=A64 mnemonic=STR offset-type=off9s_u reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_immpost page=STR_imm_gen cclass=STR_imm_gen_post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_32_ldst_immpre => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-32-reg atomic-ops=STR-32-reg instr-class=general isa=A64 mnemonic=STR offset-type=off9s_u reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_immpre page=STR_imm_gen cclass=STR_imm_gen_pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_64_ldst_immpre => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-64-reg atomic-ops=STR-64-reg instr-class=general isa=A64 mnemonic=STR offset-type=off9s_u reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_immpre page=STR_imm_gen cclass=STR_imm_gen_pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_32_ldst_pos => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b00 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=unsigned-scaled-offset address-form-reg-type=unsigned-scaled-offset-32-reg atomic-ops=STR-32-reg instr-class=general isa=A64 mnemonic=STR offset-type=off12u_s reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_pos page=STR_imm_gen cclass=STR_imm_gen_unsigned_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_64_ldst_pos => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b00 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=unsigned-scaled-offset address-form-reg-type=unsigned-scaled-offset-64-reg atomic-ops=STR-64-reg instr-class=general isa=A64 mnemonic=STR offset-type=off12u_s reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_pos page=STR_imm_gen cclass=STR_imm_gen_unsigned_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_32_ldst_regoff => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'atomic-ops=STR-32-reg instr-class=general isa=A64 mnemonic=STR offset-type=off-reg reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_regoff page=STR_reg_gen cclass=STR_reg_gen_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_64_ldst_regoff => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'atomic-ops=STR-64-reg instr-class=general isa=A64 mnemonic=STR offset-type=off-reg reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_regoff page=STR_reg_gen cclass=STR_reg_gen_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_32_ldst_immpost => {
  name       => 'Post-index',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed datatype=32 instr-class=general isa=A64 mnemonic=STRB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpost page=STRB_imm cclass=STRB_32_ldst_immpost',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_32_ldst_immpre => {
  name       => 'Pre-index',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed datatype=32 instr-class=general isa=A64 mnemonic=STRB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpre page=STRB_imm cclass=STRB_32_ldst_immpre',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_32_ldst_pos => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b00 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset datatype=32 instr-class=general isa=A64 mnemonic=STRB offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_pos page=STRB_imm cclass=STRB_32_ldst_pos',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_32B_ldst_regoff => {
  name       => 'Extended register',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  cnsts      => 'option!=0b011',
  bitdiffs   => 'option != 0b011',
  docvars    => 'datatype=32 datatype-reguse=32-ext-reg instr-class=general isa=A64 mnemonic=STRB offset-type=off-reg reguse=ext-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_regoff page=STRB_reg cclass=STRB_reg_32',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_32BL_ldst_regoff => {
  name       => 'Shifted register',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b1 Rm:u=0bxxxxx option:u=0b011 S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'option == 0b011',
  docvars    => 'datatype=32 datatype-reguse=32-shifted-reg instr-class=general isa=A64 mnemonic=STRB offset-type=off-reg reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_regoff page=STRB_reg cclass=STRB_reg_32',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRH_32_ldst_immpost => {
  name       => 'Post-index',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed datatype=32 instr-class=general isa=A64 mnemonic=STRH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpost page=STRH_imm cclass=STRH_32_ldst_immpost',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRH_32_ldst_immpre => {
  name       => 'Pre-index',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed datatype=32 instr-class=general isa=A64 mnemonic=STRH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpre page=STRH_imm cclass=STRH_32_ldst_immpre',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRH_32_ldst_pos => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b00 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset datatype=32 instr-class=general isa=A64 mnemonic=STRH offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_pos page=STRH_imm cclass=STRH_32_ldst_pos',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRH_32_ldst_regoff => {
  name       => '32-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=STRH offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_regoff page=STRH_reg cclass=STRH_32_ldst_regoff',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STTR_32_ldst_unpriv => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STTR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unpriv page=STTR cclass=STTR_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STTR_64_ldst_unpriv => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-plus-offset datatype=64 instr-class=general isa=A64 mnemonic=STTR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_unpriv page=STTR cclass=STTR_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STTRB_32_ldst_unpriv => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STTRB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unpriv page=STTRB cclass=STTRB_32_ldst_unpriv',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STTRH_32_ldst_unpriv => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STTRH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unpriv page=STTRH cclass=STTRH_32_ldst_unpriv',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STUR_32_ldst_unscaled => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-32-reg atomic-ops=STUR-32-reg instr-class=general isa=A64 mnemonic=STUR offset-type=off9s_u reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unscaled page=STUR_gen cclass=STUR_gen_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STUR_64_ldst_unscaled => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-plus-offset address-form-reg-type=base-plus-offset-64-reg atomic-ops=STUR-64-reg instr-class=general isa=A64 mnemonic=STUR offset-type=off9s_u reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unscaled page=STUR_gen cclass=STUR_gen_base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STURB_32_ldst_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STURB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unscaled page=STURB cclass=STURB_32_ldst_unscaled',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STURH_32_ldst_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STURH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unscaled page=STURH cclass=STURH_32_ldst_unscaled',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STXP_SP32_ldstexcl => {
  name       => '32-bit',
  diagram    => 'ig0=0b1 sz:u=0b0 ig1=0b001000 o2=0b0 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b0 Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sz == 0b0',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-pair-32 atomic-ops=STXP-pair-32 instr-class=general isa=A64 mnemonic=STXP reg-type=pair-32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STXP cclass=STXP_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STXP_SP64_ldstexcl => {
  name       => '64-bit',
  diagram    => 'ig0=0b1 sz:u=0b1 ig1=0b001000 o2=0b0 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b0 Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'sz == 0b1',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-pair-64 atomic-ops=STXP-pair-64 instr-class=general isa=A64 mnemonic=STXP reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STXP cclass=STXP_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STXR_SR32_ldstexcl => {
  name       => '32-bit',
  diagram    => 'size:u=0b10 ig0=0b001000 o2=0b0 L:u=0b0 o1=0b0 Rs:u=0bxxxxx o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STXR-32-reg instr-class=general isa=A64 mnemonic=STXR reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STXR cclass=STXR_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STXR_SR64_ldstexcl => {
  name       => '64-bit',
  diagram    => 'size:u=0b11 ig0=0b001000 o2=0b0 L:u=0b0 o1=0b0 Rs:u=0bxxxxx o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-64-reg atomic-ops=STXR-64-reg instr-class=general isa=A64 mnemonic=STXR reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STXR cclass=STXR_base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STXRB_SR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b0 L:u=0b0 o1=0b0 Rs:u=0bxxxxx o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STXRB-32-reg instr-class=general isa=A64 mnemonic=STXRB reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STXRB cclass=STXRB_SR32_ldstexcl',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STXRH_SR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b0 L:u=0b0 o1=0b0 Rs:u=0bxxxxx o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STXRH-32-reg instr-class=general isa=A64 mnemonic=STXRH reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STXRH cclass=STXRH_SR32_ldstexcl',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STZ2G_64Spost_ldsttags => {
  name       => 'Post-index',
  diagram    => 'ig0=0b11011001 opc=0b11 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b01 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=general isa=A64 mnemonic=STZ2G',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STZ2G cclass=STZ2G_64Spost_ldsttags',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|STORE',
};

ENCODING STZ2G_64Spre_ldsttags => {
  name       => 'Pre-index',
  diagram    => 'ig0=0b11011001 opc=0b11 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b11 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=general isa=A64 mnemonic=STZ2G',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STZ2G cclass=STZ2G_64Spre_ldsttags',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|STORE',
};

ENCODING STZ2G_64Soffset_ldsttags => {
  name       => 'Signed offset',
  diagram    => 'ig0=0b11011001 opc=0b11 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b10 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset instr-class=general isa=A64 mnemonic=STZ2G',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STZ2G cclass=STZ2G_64Soffset_ldsttags',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|STORE',
};

ENCODING STZG_64Spost_ldsttags => {
  name       => 'Post-index',
  diagram    => 'ig0=0b11011001 opc=0b01 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b01 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=general isa=A64 mnemonic=STZG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STZG cclass=STZG_64Spost_ldsttags',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|STORE',
};

ENCODING STZG_64Spre_ldsttags => {
  name       => 'Pre-index',
  diagram    => 'ig0=0b11011001 opc=0b01 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b11 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=general isa=A64 mnemonic=STZG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STZG cclass=STZG_64Spre_ldsttags',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|STORE',
};

ENCODING STZG_64Soffset_ldsttags => {
  name       => 'Signed offset',
  diagram    => 'ig0=0b11011001 opc=0b01 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b10 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset instr-class=general isa=A64 mnemonic=STZG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STZG cclass=STZG_64Soffset_ldsttags',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|STORE',
};

ENCODING STZGM_64bulk_ldsttags => {
  name       => 'Integer',
  diagram    => 'ig0=0b11011001 opc=0b00 ig1=0b1 imm9=0b000000000 op2=0b00 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=STZGM',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STZGM cclass=STZGM_64bulk_ldsttags',
  exceptions => 'DATAABORT|SPALIGNMENT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|DATA_TRANSFER|TAGS|STORE',
};

ENCODING SUB_32_addsub_ext => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S:u=0b0 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=no-s datatype=32 instr-class=general isa=A64 mnemonic=SUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_ext page=SUB_addsub_ext cclass=SUB_addsub_ext_no_s',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_64_addsub_ext => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S:u=0b0 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=no-s datatype=64 instr-class=general isa=A64 mnemonic=SUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_ext page=SUB_addsub_ext cclass=SUB_addsub_ext_no_s',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_32_addsub_imm => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S:u=0b0 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=no-s datatype=32 immediate-type=imm12u instr-class=general isa=A64 mnemonic=SUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=addsub_imm page=SUB_addsub_imm cclass=SUB_addsub_imm_no_s',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_64_addsub_imm => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S:u=0b0 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=no-s datatype=64 immediate-type=imm12u instr-class=general isa=A64 mnemonic=SUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=addsub_imm page=SUB_addsub_imm cclass=SUB_addsub_imm_no_s',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_32_addsub_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S:u=0b0 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=no-s datatype=32 instr-class=general isa=A64 mnemonic=SUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_shift page=SUB_addsub_shift cclass=SUB_addsub_shift_no_s',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_64_addsub_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S:u=0b0 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=no-s datatype=64 instr-class=general isa=A64 mnemonic=SUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_shift page=SUB_addsub_shift cclass=SUB_addsub_shift_no_s',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBG_64_addsub_immtags => {
  name       => 'Integer',
  diagram    => 'sf=0b1 op=0b1 S=0b0 ig0=0b100011 o2=0b0 uimm6:u=0bxxxxxx op3:u=0bZZ uimm4:u=0bxxxx Xn:u=0bxxxxx Xd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=SUBG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=addsub_immtags page=SUBG cclass=SUBG_64_addsub_immtags',
  categories => 'GENERAL|ARITHMETIC|TAGS',
};

ENCODING SUBP_64S_dp_2src => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b0 S=0b0 ig1=0b11010110 Xm:u=0bxxxxx opcode=0b000000 Xn:u=0bxxxxx Xd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=SUBP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=SUBP cclass=SUBP_64S_dp_2src',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBPS_64S_dp_2src => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b0 S=0b1 ig1=0b11010110 Xm:u=0bxxxxx opcode=0b000000 Xn:u=0bxxxxx Xd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=SUBPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=SUBPS cclass=SUBPS_SUBPS_64S_dp_2src',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_32S_addsub_ext => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S:u=0b1 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=S datatype=32 instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_ext page=SUBS_addsub_ext cclass=SUBS_addsub_ext_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_64S_addsub_ext => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S:u=0b1 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=S datatype=64 instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_ext page=SUBS_addsub_ext cclass=SUBS_addsub_ext_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_32S_addsub_imm => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S:u=0b1 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=S datatype=32 immediate-type=imm12u instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=addsub_imm page=SUBS_addsub_imm cclass=SUBS_addsub_imm_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_64S_addsub_imm => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S:u=0b1 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=S datatype=64 immediate-type=imm12u instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=addsub_imm page=SUBS_addsub_imm cclass=SUBS_addsub_imm_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_32_addsub_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S:u=0b1 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'cond-setting=S datatype=32 instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_shift page=SUBS_addsub_shift cclass=SUBS_addsub_shift_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_64_addsub_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S:u=0b1 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'cond-setting=S datatype=64 instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_shift page=SUBS_addsub_shift cclass=SUBS_addsub_shift_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SVC_EX_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b000 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL=0b01',
  docvars    => 'instr-class=system isa=A64 mnemonic=SVC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=SVC cclass=SVC_EX_exception',
  exceptions => 'SUPERVISORCALL',
  categories => 'SYSTEM|CALL',
};

ENCODING SWP_32_memop => {
  name       => '32-bit SWP',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=SWP-32-reg instr-class=general isa=A64 mnemonic=SWP reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWP cclass=SWP_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING SWPA_32_memop => {
  name       => '32-bit SWPA',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=SWPA-32-reg instr-class=general isa=A64 mnemonic=SWPA reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWP cclass=SWP_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING SWPAL_32_memop => {
  name       => '32-bit SWPAL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=SWPAL-32-reg instr-class=general isa=A64 mnemonic=SWPAL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWP cclass=SWP_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING SWPL_32_memop => {
  name       => '32-bit SWPL',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b10 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=SWPL-32-reg instr-class=general isa=A64 mnemonic=SWPL reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWP cclass=SWP_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING SWP_64_memop => {
  name       => '64-bit SWP',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b0',
  docvars    => 'atomic-ops=SWP-64-reg instr-class=general isa=A64 mnemonic=SWP reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWP cclass=SWP_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING SWPA_64_memop => {
  name       => '64-bit SWPA',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b0',
  docvars    => 'atomic-ops=SWPA-64-reg instr-class=general isa=A64 mnemonic=SWPA reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWP cclass=SWP_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING SWPAL_64_memop => {
  name       => '64-bit SWPAL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b1 && R == 0b1',
  docvars    => 'atomic-ops=SWPAL-64-reg instr-class=general isa=A64 mnemonic=SWPAL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWP cclass=SWP_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING SWPL_64_memop => {
  name       => '64-bit SWPL',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'size == 0b11 && A == 0b0 && R == 0b1',
  docvars    => 'atomic-ops=SWPL-64-reg instr-class=general isa=A64 mnemonic=SWPL reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWP cclass=SWP_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING SWPAB_32_memop => {
  name       => 'SWPAB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=SWPAB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWPB cclass=SWPB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING SWPALB_32_memop => {
  name       => 'SWPALB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=SWPALB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWPB cclass=SWPB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING SWPB_32_memop => {
  name       => 'SWPB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=SWPB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWPB cclass=SWPB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING SWPLB_32_memop => {
  name       => 'SWPLB',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=SWPLB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWPB cclass=SWPB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING SWPAH_32_memop => {
  name       => 'SWPAH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=SWPAH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWPH cclass=SWPH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING SWPALH_32_memop => {
  name       => 'SWPALH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b1 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b1 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=SWPALH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWPH cclass=SWPH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING SWPH_32_memop => {
  name       => 'SWPH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=SWPH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWPH cclass=SWPH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING SWPLH_32_memop => {
  name       => 'SWPLH',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  bitdiffs   => 'A == 0b0 && R == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=SWPLH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWPH cclass=SWPH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING SYS_CR_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b01 op1:u=0bxxx CRn:u=0bxxxx CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=SYS cclass=SYS_SYS_CR_systeminstrs',
  categories => 'SYSTEM',
};

ENCODING SYSL_RC_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b1 op0=0b01 op1:u=0bxxx CRn:u=0bxxxx CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=SYSL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=SYSL cclass=SYSL_RC_systeminstrs',
  categories => 'SYSTEM',
};

ENCODING TBNZ_only_testbranch => {
  name       => '14-bit signed PC-relative branch offset',
  diagram    => 'b5:u=0bx ig0=0b011011 op:u=0b1 b40:u=0bxxxxx imm14:u=0bxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  docvars    => 'branch-offset=br14 instr-class=general isa=A64 mnemonic=TBNZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=testbranch page=TBNZ cclass=TBNZ_only_testbranch',
  categories => 'GENERAL|BRANCH|CONDITIONALLY|COMPARISON',
};

ENCODING TBZ_only_testbranch => {
  name       => '14-bit signed PC-relative branch offset',
  diagram    => 'b5:u=0bx ig0=0b011011 op:u=0b0 b40:u=0bxxxxx imm14:u=0bxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  docvars    => 'branch-offset=br14 instr-class=general isa=A64 mnemonic=TBZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=testbranch page=TBZ cclass=TBZ_only_testbranch',
  categories => 'GENERAL|BRANCH|CONDITIONALLY|COMPARISON',
};

ENCODING TCANCEL_EX_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b011 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL=0b00',
  docvars    => 'instr-class=system isa=A64 mnemonic=TCANCEL',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=TME alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=TCANCEL cclass=TCANCEL_EX_exception',
  categories => 'SYSTEM',
};

ENCODING TCOMMIT_only_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm=0b0000 op2=0b011 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=TCOMMIT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=TME alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=TCOMMIT cclass=TCOMMIT_only_barriers',
  categories => 'SYSTEM',
};

ENCODING TSB_HC_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0010 op2:u=0b010 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=TSB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=TSB cclass=TSB_HC_hints',
  categories => 'SYSTEM',
};

ENCODING TSTART_BR_systemresult => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b1 op0=0b00 op1=0b011 CRn=0b0011 CRm=0b0000 op2=0b011 Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=TSTART',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=TME alphaindex=BASIC',
  tags       => 'group=control iclass=systemresult page=TSTART cclass=TSTART_BR_systemresult',
  exceptions => 'TSTARTACCESSTRAP',
  pstate     => 'EL=CR',
  categories => 'SYSTEM',
};

ENCODING TTEST_BR_systemresult => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b1 op0=0b00 op1=0b011 CRn=0b0011 CRm=0b0001 op2=0b011 Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=TTEST',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=TME alphaindex=BASIC',
  tags       => 'group=control iclass=systemresult page=TTEST cclass=TTEST_BR_systemresult',
  categories => 'SYSTEM|COMPARISON',
};

ENCODING UBFM_32M_bitfield => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b10 ig0=0b100110 N:u=0b0 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  docvars    => 'bitfield-fill=zero-fill datatype=32 instr-class=general isa=A64 mnemonic=UBFM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=bitfield page=UBFM cclass=UBFM_zero_fill',
  categories => 'GENERAL|BITFIELD|DATA_TRANSFER|UNSIGNED',
};

ENCODING UBFM_64M_bitfield => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b10 ig0=0b100110 N:u=0b1 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && N == 0b1',
  docvars    => 'bitfield-fill=zero-fill datatype=64 instr-class=general isa=A64 mnemonic=UBFM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=bitfield page=UBFM cclass=UBFM_zero_fill',
  categories => 'GENERAL|BITFIELD|DATA_TRANSFER|UNSIGNED',
};

ENCODING UDF_only_perm_undef => {
  name       => 'Integer',
  diagram    => 'ig0=0b0000000000000000 imm16:u=0bxxxxxxxxxxxxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=UDF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=reserved iclass=perm_undef page=UDF_perm_undef cclass=UDF_only_perm_undef',
  categories => 'GENERAL|UNDEFINED',
};

ENCODING UDIV_32_dp_2src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b00001 o1:u=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=UDIV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_2src page=UDIV cclass=UDIV_general',
  categories => 'GENERAL|ARITHMETIC|UNSIGNED',
};

ENCODING UDIV_64_dp_2src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b00001 o1:u=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=UDIV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_2src page=UDIV cclass=UDIV_general',
  categories => 'GENERAL|ARITHMETIC|UNSIGNED',
};

ENCODING UMADDL_64WA_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b1 ig1=0b01 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=UMADDL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=UMADDL cclass=UMADDL_UMADDL_64WA_dp_3src',
  categories => 'GENERAL|CARITHMETIC|UNSIGNED',
};

ENCODING UMSUBL_64WA_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b1 ig1=0b01 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=UMSUBL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=UMSUBL cclass=UMSUBL_UMSUBL_64WA_dp_3src',
  categories => 'GENERAL|CARITHMETIC|UNSIGNED',
};

ENCODING UMULH_64_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b1 ig1=0b10 Rm:u=0bxxxxx o0=0b0 Ra:u=0bSSSSS Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=UMULH',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=UMULH cclass=UMULH_64_dp_3src',
  categories => 'GENERAL|ARITHMETIC|UNSIGNED',
};

ENCODING WFE_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0000 op2:u=0b010 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=WFE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=WFE cclass=WFE_HI_hints',
  exceptions => 'WFXTRAP',
  pstate     => 'EL=CR',
  categories => 'SYSTEM',
};

ENCODING WFI_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0000 op2:u=0b011 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=WFI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=WFI cclass=WFI_HI_hints',
  exceptions => 'WFXTRAP',
  pstate     => 'EL=CR',
  categories => 'SYSTEM|HINT',
};

ENCODING XAFLAG_M_pstate => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b000 CRn=0b0100 CRm:u=0bZZZZ op2=0b001 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=XAFLAG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=control iclass=pstate page=XAFLAG cclass=XAFLAG_M_pstate',
  pstate     => 'C=R N=U V=U Z=R',
  categories => 'SYSTEM|CONVERSION',
};

ENCODING XPACD_64Z_dp_1src => {
  name       => 'XPACD',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b1 opcode[3:1]=0b000 D:u=0b1 Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'D == 0b1',
  docvars    => 'instr-class=general isa=A64 mnemonic=XPACD',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=XPAC cclass=XPAC_general',
  categories => 'GENERAL|PAC',
};

ENCODING XPACI_64Z_dp_1src => {
  name       => 'XPACI',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b1 opcode[3:1]=0b000 D:u=0b0 Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'D == 0b0',
  docvars    => 'instr-class=general isa=A64 mnemonic=XPACI',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=XPAC cclass=XPAC_general',
  categories => 'GENERAL|PAC',
};

ENCODING XPACLRI_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm=0b0000 op2=0b111 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=XPACLRI',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=XPAC cclass=XPACLRI_HI_hints',
  categories => 'SYSTEM|PAC',
};

ENCODING YIELD_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0000 op2:u=0b001 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=YIELD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=YIELD cclass=YIELD_HI_hints',
  categories => 'SYSTEM|HINT',
};

ENCODING ASR_ASRV_32_dp_2src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ASR datatype=32 instr-class=general isa=A64 mnemonic=ASRV source-type=src-is-register',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ASRV_32_dp_2src alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_2src page=ASR_ASRV cclass=ASR_ASRV_general',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
  eq_to      => 'ASRV <Wd>, <Wn>, <Wm>',
};

ENCODING ASR_ASRV_64_dp_2src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ASR datatype=64 instr-class=general isa=A64 mnemonic=ASRV source-type=src-is-register',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ASRV_64_dp_2src alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_2src page=ASR_ASRV cclass=ASR_ASRV_general',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
  eq_to      => 'ASRV <Xd>, <Xn>, <Xm>',
};

ENCODING ASR_SBFM_32M_bitfield => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b00 ig0=0b100110 N:u=0b0 immr:u=0bxxxxxx imms:u=0b011111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0 && imms == 0b011111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ASR bitfield-fill=signed-fill datatype=32 instr-class=general isa=A64 mnemonic=SBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SBFM_32M_bitfield alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=bitfield page=ASR_SBFM cclass=ASR_SBFM_signed_fill',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
  eq_to      => 'SBFM <Wd>, <Wn>, #<shift>, #31',
};

ENCODING ASR_SBFM_64M_bitfield => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b00 ig0=0b100110 N:u=0b1 immr:u=0bxxxxxx imms:u=0b111111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && N == 0b1 && imms == 0b111111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ASR bitfield-fill=signed-fill datatype=64 instr-class=general isa=A64 mnemonic=SBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SBFM_64M_bitfield alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=bitfield page=ASR_SBFM cclass=ASR_SBFM_signed_fill',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
  eq_to      => 'SBFM <Xd>, <Xn>, #<shift>, #63',
};

ENCODING AT_SYS_CR_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b01 op1:u=0bxxx CRn:u=0b0111 CRm:u=0b100x op2:u=0bxxx Rt:u=0bxxxxx',
  aliascond  => 'SysOp(op1,0b0111,CRm,op2) == Sys_AT',
  docvars    => 'alias_mnemonic=AT instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SYS_CR_systeminstrs alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=AT_SYS cclass=AT_SYS_SYS_CR_systeminstrs',
  categories => 'SYSTEM',
  eq_to      => 'SYS #<op1>, C7, <Cm>, #<op2>, <Xt>',
};

ENCODING BFC_BFM_32M_bitfield => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b01 ig0=0b100110 N:u=0b0 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  aliascond  => 'UInt(imms) < UInt(immr)',
  docvars    => 'alias_mnemonic=BFC bitfield-fill=nofill datatype=32 instr-class=general isa=A64 mnemonic=BFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v2 aliasof=BFM_32M_bitfield alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=bitfield page=BFC_BFM cclass=BFC_BFM_nofill',
  categories => 'GENERAL|BITFIELD',
  eq_to      => 'BFM <Wd>, WZR, #(-<lsb> MOD 32), #(<width>-1)',
};

ENCODING BFC_BFM_64M_bitfield => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b01 ig0=0b100110 N:u=0b1 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && N == 0b1',
  aliascond  => 'UInt(imms) < UInt(immr)',
  docvars    => 'alias_mnemonic=BFC bitfield-fill=nofill datatype=64 instr-class=general isa=A64 mnemonic=BFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v2 aliasof=BFM_64M_bitfield alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=bitfield page=BFC_BFM cclass=BFC_BFM_nofill',
  categories => 'GENERAL|BITFIELD',
  eq_to      => 'BFM <Xd>, XZR, #(-<lsb> MOD 64), #(<width>-1)',
};

ENCODING BFI_BFM_32M_bitfield => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b01 ig0=0b100110 N:u=0b0 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'Rn!=0b11111',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  aliascond  => 'UInt(imms) < UInt(immr)',
  docvars    => 'alias_mnemonic=BFI bitfield-fill=nofill datatype=32 instr-class=general isa=A64 mnemonic=BFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=BFM_32M_bitfield alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=bitfield page=BFI_BFM cclass=BFI_BFM_nofill',
  categories => 'GENERAL|BITFIELD',
  eq_to      => 'BFM  <Wd>, <Wn>, #(-<lsb> MOD 32), #(<width>-1)',
};

ENCODING BFI_BFM_64M_bitfield => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b01 ig0=0b100110 N:u=0b1 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'Rn!=0b11111',
  bitdiffs   => 'sf == 0b1 && N == 0b1',
  aliascond  => 'UInt(imms) < UInt(immr)',
  docvars    => 'alias_mnemonic=BFI bitfield-fill=nofill datatype=64 instr-class=general isa=A64 mnemonic=BFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=BFM_64M_bitfield alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=bitfield page=BFI_BFM cclass=BFI_BFM_nofill',
  categories => 'GENERAL|BITFIELD',
  eq_to      => 'BFM  <Xd>, <Xn>, #(-<lsb> MOD 64), #(<width>-1)',
};

ENCODING BFXIL_BFM_32M_bitfield => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b01 ig0=0b100110 N:u=0b0 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  aliascond  => 'UInt(imms) >= UInt(immr)',
  docvars    => 'alias_mnemonic=BFXIL bitfield-fill=nofill datatype=32 instr-class=general isa=A64 mnemonic=BFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=BFM_32M_bitfield alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=bitfield page=BFXIL_BFM cclass=BFXIL_BFM_nofill',
  categories => 'GENERAL|BITFIELD',
  eq_to      => 'BFM  <Wd>, <Wn>, #<lsb>, #(<lsb>+<width>-1)',
};

ENCODING BFXIL_BFM_64M_bitfield => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b01 ig0=0b100110 N:u=0b1 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && N == 0b1',
  aliascond  => 'UInt(imms) >= UInt(immr)',
  docvars    => 'alias_mnemonic=BFXIL bitfield-fill=nofill datatype=64 instr-class=general isa=A64 mnemonic=BFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=BFM_64M_bitfield alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=bitfield page=BFXIL_BFM cclass=BFXIL_BFM_nofill',
  categories => 'GENERAL|BITFIELD',
  eq_to      => 'BFM  <Xd>, <Xn>, #<lsb>, #(<lsb>+<width>-1)',
};

ENCODING CFP_SYS_CR_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b01 op1:u=0b011 CRn:u=0b0111 CRm:u=0b0011 op2:u=0b100 Rt:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CFP instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v5 aliasof=SYS_CR_systeminstrs alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=CFP_SYS cclass=CFP_SYS_SYS_CR_systeminstrs',
  categories => 'SYSTEM|CONTROL_FLOW',
  eq_to      => 'SYS #3, C8, C3, #4, <Xt>',
};

ENCODING CINC_CSINC_32_condsel => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111 cond!=0b111x Rn!=0b11111',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Rn == Rm',
  docvars    => 'alias_mnemonic=CINC datatype=32 instr-class=general isa=A64 mnemonic=CSINC',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=CSINC_32_condsel alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=condsel page=CINC_CSINC cclass=CINC_CSINC_general',
  categories => 'GENERAL|ARITHMETIC|CONDITIONALLY|INC_DEC',
  eq_to      => 'CSINC <Wd>, <Wn>, <Wn>, invert(<cond>)',
};

ENCODING CINC_CSINC_64_condsel => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111 cond!=0b111x Rn!=0b11111',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Rn == Rm',
  docvars    => 'alias_mnemonic=CINC datatype=64 instr-class=general isa=A64 mnemonic=CSINC',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=CSINC_64_condsel alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=condsel page=CINC_CSINC cclass=CINC_CSINC_general',
  categories => 'GENERAL|ARITHMETIC|CONDITIONALLY|INC_DEC',
  eq_to      => 'CSINC <Xd>, <Xn>, <Xn>, invert(<cond>)',
};

ENCODING CINV_CSINV_32_condsel => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111 cond!=0b111x Rn!=0b11111',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Rn == Rm',
  docvars    => 'alias_mnemonic=CINV datatype=32 instr-class=general isa=A64 mnemonic=CSINV',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=CSINV_32_condsel alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=condsel page=CINV_CSINV cclass=CINV_CSINV_general',
  categories => 'GENERAL|BITWISE|CONDITIONALLY',
  eq_to      => 'CSINV <Wd>, <Wn>, <Wn>, invert(<cond>)',
};

ENCODING CINV_CSINV_64_condsel => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111 cond!=0b111x Rn!=0b11111',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Rn == Rm',
  docvars    => 'alias_mnemonic=CINV datatype=64 instr-class=general isa=A64 mnemonic=CSINV',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=CSINV_64_condsel alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=condsel page=CINV_CSINV cclass=CINV_CSINV_general',
  categories => 'GENERAL|BITWISE|CONDITIONALLY',
  eq_to      => 'CSINV <Xd>, <Xn>, <Xn>, invert(<cond>)',
};

ENCODING CMN_ADDS_32S_addsub_ext => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S:u=0b1 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0b11111',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CMN cond-setting=S datatype=32 instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ADDS_32S_addsub_ext alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_ext page=CMN_ADDS_addsub_ext cclass=CMN_ADDS_addsub_ext_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|COMPARISON',
  eq_to      => 'ADDS WZR, <Wn|WSP>, <Wm>{, <extend> {#<amount>}}',
};

ENCODING CMN_ADDS_64S_addsub_ext => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S:u=0b1 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0b11111',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CMN cond-setting=S datatype=64 instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ADDS_64S_addsub_ext alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_ext page=CMN_ADDS_addsub_ext cclass=CMN_ADDS_addsub_ext_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|COMPARISON',
  eq_to      => 'ADDS XZR, <Xn|SP>, <R><m>{, <extend> {#<amount>}}',
};

ENCODING CMN_ADDS_32S_addsub_imm => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S:u=0b1 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CMN cond-setting=S datatype=32 immediate-type=imm12u instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ADDS_32S_addsub_imm alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=addsub_imm page=CMN_ADDS_addsub_imm cclass=CMN_ADDS_addsub_imm_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|COMPARISON',
  eq_to      => 'ADDS WZR, <Wn|WSP>, #<imm> {, <shift>}',
};

ENCODING CMN_ADDS_64S_addsub_imm => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S:u=0b1 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CMN cond-setting=S datatype=64 immediate-type=imm12u instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ADDS_64S_addsub_imm alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=addsub_imm page=CMN_ADDS_addsub_imm cclass=CMN_ADDS_addsub_imm_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|COMPARISON',
  eq_to      => 'ADDS XZR, <Xn|SP>, #<imm> {, <shift>}',
};

ENCODING CMN_ADDS_32_addsub_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S:u=0b1 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CMN cond-setting=S datatype=32 instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ADDS_32_addsub_shift alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_shift page=CMN_ADDS_addsub_shift cclass=CMN_ADDS_addsub_shift_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|COMPARISON',
  eq_to      => 'ADDS WZR, <Wn>, <Wm> {, <shift> #<amount>}',
};

ENCODING CMN_ADDS_64_addsub_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S:u=0b1 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CMN cond-setting=S datatype=64 instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ADDS_64_addsub_shift alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_shift page=CMN_ADDS_addsub_shift cclass=CMN_ADDS_addsub_shift_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|COMPARISON',
  eq_to      => 'ADDS XZR, <Xn>, <Xm> {, <shift> #<amount>}',
};

ENCODING CMP_SUBS_32S_addsub_ext => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S:u=0b1 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0b11111',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CMP cond-setting=S datatype=32 instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SUBS_32S_addsub_ext alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_ext page=CMP_SUBS_addsub_ext cclass=CMP_SUBS_addsub_ext_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|COMPARISON',
  eq_to      => 'SUBS WZR, <Wn|WSP>, <Wm>{, <extend> {#<amount>}}',
};

ENCODING CMP_SUBS_64S_addsub_ext => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S:u=0b1 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0b11111',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CMP cond-setting=S datatype=64 instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SUBS_64S_addsub_ext alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_ext page=CMP_SUBS_addsub_ext cclass=CMP_SUBS_addsub_ext_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|COMPARISON',
  eq_to      => 'SUBS XZR, <Xn|SP>, <R><m>{, <extend> {#<amount>}}',
};

ENCODING CMP_SUBS_32S_addsub_imm => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S:u=0b1 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CMP cond-setting=S datatype=32 immediate-type=imm12u instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SUBS_32S_addsub_imm alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=addsub_imm page=CMP_SUBS_addsub_imm cclass=CMP_SUBS_addsub_imm_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|COMPARISON',
  eq_to      => 'SUBS WZR, <Wn|WSP>, #<imm> {, <shift>}',
};

ENCODING CMP_SUBS_64S_addsub_imm => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S:u=0b1 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CMP cond-setting=S datatype=64 immediate-type=imm12u instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SUBS_64S_addsub_imm alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=addsub_imm page=CMP_SUBS_addsub_imm cclass=CMP_SUBS_addsub_imm_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|COMPARISON',
  eq_to      => 'SUBS XZR, <Xn|SP>, #<imm> {, <shift>}',
};

ENCODING CMP_SUBS_32_addsub_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S:u=0b1 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CMP cond-setting=S datatype=32 instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SUBS_32_addsub_shift alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_shift page=CMP_SUBS_addsub_shift cclass=CMP_SUBS_addsub_shift_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|COMPARISON',
  eq_to      => 'SUBS WZR, <Wn>, <Wm> {, <shift> #<amount>}',
};

ENCODING CMP_SUBS_64_addsub_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S:u=0b1 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CMP cond-setting=S datatype=64 instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SUBS_64_addsub_shift alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_shift page=CMP_SUBS_addsub_shift cclass=CMP_SUBS_addsub_shift_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|COMPARISON',
  eq_to      => 'SUBS XZR, <Xn>, <Xm> {, <shift> #<amount>}',
};

ENCODING CMPP_SUBPS_64S_dp_2src => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b0 S=0b1 ig1=0b11010110 Xm:u=0bxxxxx opcode=0b000000 Xn:u=0bxxxxx Xd:u=0b11111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CMPP instr-class=general isa=A64 mnemonic=SUBPS source-type=src-is-register',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v5 aliasof=SUBPS_64S_dp_2src alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=CMPP_SUBPS cclass=CMPP_SUBPS_SUBPS_64S_dp_2src',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|COMPARISON|TAGS',
  eq_to      => 'SUBPS XZR, <Xn|SP>, <Xm|SP>',
};

ENCODING CNEG_CSNEG_32_condsel => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'cond!=0b111x',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Rn == Rm',
  docvars    => 'alias_mnemonic=CNEG datatype=32 instr-class=general isa=A64 mnemonic=CSNEG',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=CSNEG_32_condsel alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=condsel page=CNEG_CSNEG cclass=CNEG_CSNEG_general',
  categories => 'GENERAL|BITWISE|CONDITIONALLY',
  eq_to      => 'CSNEG <Wd>, <Wn>, <Wn>, invert(<cond>)',
};

ENCODING CNEG_CSNEG_64_condsel => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'cond!=0b111x',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Rn == Rm',
  docvars    => 'alias_mnemonic=CNEG datatype=64 instr-class=general isa=A64 mnemonic=CSNEG',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=CSNEG_64_condsel alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=condsel page=CNEG_CSNEG cclass=CNEG_CSNEG_general',
  categories => 'GENERAL|BITWISE|CONDITIONALLY',
  eq_to      => 'CSNEG <Xd>, <Xn>, <Xn>, invert(<cond>)',
};

ENCODING CPP_SYS_CR_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b01 op1:u=0b011 CRn:u=0b0111 CRm:u=0b0011 op2:u=0b111 Rt:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CPP instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v5 aliasof=SYS_CR_systeminstrs alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=CPP_SYS cclass=CPP_SYS_SYS_CR_systeminstrs',
  categories => 'SYSTEM|PREFETCH',
  eq_to      => 'SYS #3, C8, C3, #7, <Xt>',
};

ENCODING CSET_CSINC_32_condsel => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S=0b0 ig0=0b11010100 Rm:u=0b11111 cond:u=0bxxxx ig1=0b0 o2:u=0b1 Rn:u=0b11111 Rd:u=0bxxxxx',
  cnsts      => 'cond!=0b111x',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CSET datatype=32 instr-class=general isa=A64 mnemonic=CSINC',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=CSINC_32_condsel alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=condsel page=CSET_CSINC cclass=CSET_CSINC_general',
  categories => 'GENERAL|SELECTION|CONDITIONALLY',
  eq_to      => 'CSINC <Wd>, WZR, WZR, invert(<cond>)',
};

ENCODING CSET_CSINC_64_condsel => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S=0b0 ig0=0b11010100 Rm:u=0b11111 cond:u=0bxxxx ig1=0b0 o2:u=0b1 Rn:u=0b11111 Rd:u=0bxxxxx',
  cnsts      => 'cond!=0b111x',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CSET datatype=64 instr-class=general isa=A64 mnemonic=CSINC',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=CSINC_64_condsel alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=condsel page=CSET_CSINC cclass=CSET_CSINC_general',
  categories => 'GENERAL|SELECTION|CONDITIONALLY',
  eq_to      => 'CSINC <Xd>, XZR, XZR, invert(<cond>)',
};

ENCODING CSETM_CSINV_32_condsel => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S=0b0 ig0=0b11010100 Rm:u=0b11111 cond:u=0bxxxx ig1=0b0 o2:u=0b0 Rn:u=0b11111 Rd:u=0bxxxxx',
  cnsts      => 'cond!=0b111x',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CSETM datatype=32 instr-class=general isa=A64 mnemonic=CSINV',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=CSINV_32_condsel alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=condsel page=CSETM_CSINV cclass=CSETM_CSINV_general',
  categories => 'GENERAL|SELECTION|CONDITIONALLY',
  eq_to      => 'CSINV <Wd>, WZR, WZR, invert(<cond>)',
};

ENCODING CSETM_CSINV_64_condsel => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S=0b0 ig0=0b11010100 Rm:u=0b11111 cond:u=0bxxxx ig1=0b0 o2:u=0b0 Rn:u=0b11111 Rd:u=0bxxxxx',
  cnsts      => 'cond!=0b111x',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=CSETM datatype=64 instr-class=general isa=A64 mnemonic=CSINV',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=CSINV_64_condsel alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=condsel page=CSETM_CSINV cclass=CSETM_CSINV_general',
  categories => 'GENERAL|SELECTION|CONDITIONALLY',
  eq_to      => 'CSINV <Xd>, XZR, XZR, invert(<cond>)',
};

ENCODING DC_SYS_CR_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b01 op1:u=0bxxx CRn:u=0b0111 CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  aliascond  => 'SysOp(op1,0b0111,CRm,op2) == Sys_DC',
  docvars    => 'alias_mnemonic=DC instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SYS_CR_systeminstrs alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=DC_SYS cclass=DC_SYS_SYS_CR_systeminstrs',
  categories => 'SYSTEM',
  eq_to      => 'SYS #<op1>, C7, <Cm>, #<op2>, <Xt>',
};

ENCODING DFB_DSB_BO_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm:u=0b1100 ig1=0b1 opc:u=0b00 Rt=0b11111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=DFB instr-class=system isa=A64 mnemonic=DSB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=vv aliasof=DSB_BO_barriers alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=DFB_DSB cclass=DFB_DSB_DSB_BO_barriers',
  categories => 'SYSTEM',
  eq_to      => 'DSB #12',
};

ENCODING DVP_SYS_CR_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b01 op1:u=0b011 CRn:u=0b0111 CRm:u=0b0011 op2:u=0b101 Rt:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=DVP instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v5 aliasof=SYS_CR_systeminstrs alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=DVP_SYS cclass=DVP_SYS_SYS_CR_systeminstrs',
  categories => 'SYSTEM',
  eq_to      => 'SYS #3, C8, C3, #5, <Xt>',
};

ENCODING IC_SYS_CR_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b01 op1:u=0bxxx CRn:u=0b0111 CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  aliascond  => 'SysOp(op1,0b0111,CRm,op2) == Sys_IC',
  docvars    => 'alias_mnemonic=IC instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SYS_CR_systeminstrs alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=IC_SYS cclass=IC_SYS_SYS_CR_systeminstrs',
  categories => 'SYSTEM',
  eq_to      => 'SYS #<op1>, C7, <Cm>, #<op2>{, <Xt>}',
};

ENCODING LSL_LSLV_32_dp_2src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSL datatype=32 instr-class=general isa=A64 mnemonic=LSLV source-type=src-is-register',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=LSLV_32_dp_2src alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_2src page=LSL_LSLV cclass=LSL_LSLV_general',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
  eq_to      => 'LSLV <Wd>, <Wn>, <Wm>',
};

ENCODING LSL_LSLV_64_dp_2src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSL datatype=64 instr-class=general isa=A64 mnemonic=LSLV source-type=src-is-register',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=LSLV_64_dp_2src alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_2src page=LSL_LSLV cclass=LSL_LSLV_general',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
  eq_to      => 'LSLV <Xd>, <Xn>, <Xm>',
};

ENCODING LSL_UBFM_32M_bitfield => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b10 ig0=0b100110 N:u=0b0 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'imms!=0bx11111 imms!=0b0xxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0 && imms != 0b011111',
  aliascond  => 'imms + 1 == immr',
  docvars    => 'alias_mnemonic=LSL bitfield-fill=zero-fill datatype=32 instr-class=general isa=A64 mnemonic=UBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=UBFM_32M_bitfield alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=bitfield page=LSL_UBFM cclass=LSL_UBFM_zero_fill',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
  eq_to      => 'UBFM <Wd>, <Wn>, #(-<shift> MOD 32), #(31-<shift>)',
};

ENCODING LSL_UBFM_64M_bitfield => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b10 ig0=0b100110 N:u=0b1 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'imms!=0bx11111 imms!=0b1xxxxx',
  bitdiffs   => 'sf == 0b1 && N == 0b1 && imms != 0b111111',
  aliascond  => 'imms + 1 == immr',
  docvars    => 'alias_mnemonic=LSL bitfield-fill=zero-fill datatype=64 instr-class=general isa=A64 mnemonic=UBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=UBFM_64M_bitfield alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=bitfield page=LSL_UBFM cclass=LSL_UBFM_zero_fill',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
  eq_to      => 'UBFM <Xd>, <Xn>, #(-<shift> MOD 64), #(63-<shift>)',
};

ENCODING LSR_LSRV_32_dp_2src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSR datatype=32 instr-class=general isa=A64 mnemonic=LSRV source-type=src-is-register',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=LSRV_32_dp_2src alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_2src page=LSR_LSRV cclass=LSR_LSRV_general',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
  eq_to      => 'LSRV <Wd>, <Wn>, <Wm>',
};

ENCODING LSR_LSRV_64_dp_2src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSR datatype=64 instr-class=general isa=A64 mnemonic=LSRV source-type=src-is-register',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=LSRV_64_dp_2src alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_2src page=LSR_LSRV cclass=LSR_LSRV_general',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
  eq_to      => 'LSRV <Xd>, <Xn>, <Xm>',
};

ENCODING LSR_UBFM_32M_bitfield => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b10 ig0=0b100110 N:u=0b0 immr:u=0bxxxxxx imms:u=0b011111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0 && imms == 0b011111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSR bitfield-fill=zero-fill datatype=32 instr-class=general isa=A64 mnemonic=UBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=UBFM_32M_bitfield alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=bitfield page=LSR_UBFM cclass=LSR_UBFM_zero_fill',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
  eq_to      => 'UBFM <Wd>, <Wn>, #<shift>, #31',
};

ENCODING LSR_UBFM_64M_bitfield => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b10 ig0=0b100110 N:u=0b1 immr:u=0bxxxxxx imms:u=0b111111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && N == 0b1 && imms == 0b111111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSR bitfield-fill=zero-fill datatype=64 instr-class=general isa=A64 mnemonic=UBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=UBFM_64M_bitfield alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=bitfield page=LSR_UBFM cclass=LSR_UBFM_zero_fill',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
  eq_to      => 'UBFM <Xd>, <Xn>, #<shift>, #63',
};

ENCODING MNEG_MSUB_32A_dp_3src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op54=0b00 ig0=0b11011 op31=0b000 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0b11111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MNEG datatype=32 instr-class=general isa=A64 mnemonic=MSUB',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=MSUB_32A_dp_3src alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_3src page=MNEG_MSUB cclass=MNEG_MSUB_general',
  categories => 'GENERAL|CARITHMETIC',
  eq_to      => 'MSUB <Wd>, <Wn>, <Wm>, WZR',
};

ENCODING MNEG_MSUB_64A_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op54=0b00 ig0=0b11011 op31=0b000 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0b11111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MNEG datatype=64 instr-class=general isa=A64 mnemonic=MSUB',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=MSUB_64A_dp_3src alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=MNEG_MSUB cclass=MNEG_MSUB_general',
  categories => 'GENERAL|CARITHMETIC',
  eq_to      => 'MSUB <Xd>, <Xn>, <Xm>, XZR',
};

ENCODING MOV_ADD_32_addsub_imm => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b0 S:u=0b0 ig0=0b100010 sh:u=0b0 imm12:u=0b000000000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  aliascond  => '(Rd == 0b11111 || Rn == 0b11111)',
  docvars    => 'alias_mnemonic=MOV cond-setting=no-s datatype=32 immediate-type=imm12u instr-class=general isa=A64 mnemonic=ADD move-what=to-from-sp',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ADD_32_addsub_imm alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=addsub_imm page=MOV_ADD_addsub_imm cclass=MOV_ADD_addsub_imm_no_s',
  categories => 'GENERAL|STACK_TRANSFER',
  eq_to      => 'ADD <Wd|WSP>, <Wn|WSP>, #0',
};

ENCODING MOV_ADD_64_addsub_imm => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b0 S:u=0b0 ig0=0b100010 sh:u=0b0 imm12:u=0b000000000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  aliascond  => '(Rd == 0b11111 || Rn == 0b11111)',
  docvars    => 'alias_mnemonic=MOV cond-setting=no-s datatype=64 immediate-type=imm12u instr-class=general isa=A64 mnemonic=ADD move-what=to-from-sp',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ADD_64_addsub_imm alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=addsub_imm page=MOV_ADD_addsub_imm cclass=MOV_ADD_addsub_imm_no_s',
  categories => 'GENERAL|STACK_TRANSFER',
  eq_to      => 'ADD <Xd|SP>, <Xn|SP>, #0',
};

ENCODING MOV_MOVN_32_movewide => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b00 ig0=0b100101 hw:u=0b0x imm16:u=0bxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && hw == 0b0x',
  aliascond  => '!(IsZero(imm16) && hw != 0b00) && ! IsOnes(imm16)',
  docvars    => 'alias_mnemonic=MOV datatype=32 immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVN move-what=mov-wideinv',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=MOVN_32_movewide alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=movewide page=MOV_MOVN cclass=MOV_MOVN_imm18_packed',
  categories => 'GENERAL|DATA_TRANSFER',
  eq_to      => 'MOVN <Wd>, #<imm16>, LSL #<shift>',
};

ENCODING MOV_MOVN_64_movewide => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b00 ig0=0b100101 hw:u=0bxx imm16:u=0bxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  aliascond  => '!(IsZero(imm16) && hw != 0b00)',
  docvars    => 'alias_mnemonic=MOV datatype=64 immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVN move-what=mov-wideinv',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=MOVN_64_movewide alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=movewide page=MOV_MOVN cclass=MOV_MOVN_imm18_packed',
  categories => 'GENERAL|DATA_TRANSFER',
  eq_to      => 'MOVN <Xd>, #<imm16>, LSL #<shift>',
};

ENCODING MOV_MOVZ_32_movewide => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b10 ig0=0b100101 hw:u=0b0x imm16:u=0bxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && hw == 0b0x',
  aliascond  => '!(IsZero(imm16) && hw != 0b00)',
  docvars    => 'alias_mnemonic=MOV datatype=32 immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVZ move-what=mov-wideimm',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=MOVZ_32_movewide alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=movewide page=MOV_MOVZ cclass=MOV_MOVZ_imm18_packed',
  categories => 'GENERAL|DATA_TRANSFER',
  eq_to      => 'MOVZ <Wd>, #<imm16>, LSL #<shift>',
};

ENCODING MOV_MOVZ_64_movewide => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b10 ig0=0b100101 hw:u=0bxx imm16:u=0bxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  aliascond  => '!(IsZero(imm16) && hw != 0b00)',
  docvars    => 'alias_mnemonic=MOV datatype=64 immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVZ move-what=mov-wideimm',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=MOVZ_64_movewide alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=movewide page=MOV_MOVZ cclass=MOV_MOVZ_imm18_packed',
  categories => 'GENERAL|DATA_TRANSFER',
  eq_to      => 'MOVZ <Xd>, #<imm16>, LSL #<shift>',
};

ENCODING MOV_ORR_32_log_imm => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b01 ig0=0b100100 N:u=0b0 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  aliascond  => '! MoveWidePreferred(sf, N, imms, immr)',
  docvars    => 'alias_mnemonic=MOV cond-setting=no-s datatype=32 immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=ORR move-what=mov-bitmask',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ORR_32_log_imm alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=log_imm page=MOV_ORR_log_imm cclass=MOV_ORR_log_imm_no_s',
  categories => 'GENERAL|DATA_TRANSFER',
  eq_to      => 'ORR <Wd|WSP>, WZR, #<imm>',
};

ENCODING MOV_ORR_64_log_imm => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b01 ig0=0b100100 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  aliascond  => '! MoveWidePreferred(sf, N, imms, immr)',
  docvars    => 'alias_mnemonic=MOV cond-setting=no-s datatype=64 immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=ORR move-what=mov-bitmask',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ORR_64_log_imm alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=log_imm page=MOV_ORR_log_imm cclass=MOV_ORR_log_imm_no_s',
  categories => 'GENERAL|DATA_TRANSFER',
  eq_to      => 'ORR <Xd|SP>, XZR, #<imm>',
};

ENCODING MOV_ORR_32_log_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b01 ig0=0b01010 shift:u=0b00 N:u=0b0 Rm:u=0bxxxxx imm6:u=0b000000 Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MOV cond-setting=no-s datatype=32 datatype-reguse=32-shifted-reg instr-class=general isa=A64 mnemonic=ORR move-what=mov-register reguse=shifted-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ORR_32_log_shift alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=log_shift page=MOV_ORR_log_shift cclass=MOV_ORR_log_shift_no_s',
  categories => 'GENERAL|DATA_TRANSFER',
  eq_to      => 'ORR <Wd>, WZR, <Wm>',
};

ENCODING MOV_ORR_64_log_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b01 ig0=0b01010 shift:u=0b00 N:u=0b0 Rm:u=0bxxxxx imm6:u=0b000000 Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MOV cond-setting=no-s datatype=64 datatype-reguse=64-shifted-reg instr-class=general isa=A64 mnemonic=ORR move-what=mov-register reguse=shifted-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ORR_64_log_shift alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=log_shift page=MOV_ORR_log_shift cclass=MOV_ORR_log_shift_no_s',
  categories => 'GENERAL|DATA_TRANSFER',
  eq_to      => 'ORR <Xd>, XZR, <Xm>',
};

ENCODING MUL_MADD_32A_dp_3src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op54=0b00 ig0=0b11011 op31=0b000 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0b11111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MUL datatype=32 instr-class=general isa=A64 mnemonic=MADD',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=MADD_32A_dp_3src alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_3src page=MUL_MADD cclass=MUL_MADD_general',
  categories => 'GENERAL|ARITHMETIC',
  eq_to      => 'MADD <Wd>, <Wn>, <Wm>, WZR',
};

ENCODING MUL_MADD_64A_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op54=0b00 ig0=0b11011 op31=0b000 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0b11111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MUL datatype=64 instr-class=general isa=A64 mnemonic=MADD',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=MADD_64A_dp_3src alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=MUL_MADD cclass=MUL_MADD_general',
  categories => 'GENERAL|ARITHMETIC',
  eq_to      => 'MADD <Xd>, <Xn>, <Xm>, XZR',
};

ENCODING MVN_ORN_32_log_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b01 ig0=0b01010 shift:u=0bxx N:u=0b1 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MVN cond-setting=no-s datatype=32 datatype-reguse=32-shifted-reg instr-class=general isa=A64 mnemonic=ORN reguse=shifted-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ORN_32_log_shift alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=log_shift page=MVN_ORN_log_shift cclass=MVN_ORN_log_shift_no_s',
  categories => 'GENERAL|BITWISE',
  eq_to      => 'ORN <Wd>, WZR, <Wm>{, <shift> #<amount>}',
};

ENCODING MVN_ORN_64_log_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b01 ig0=0b01010 shift:u=0bxx N:u=0b1 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MVN cond-setting=no-s datatype=64 datatype-reguse=64-shifted-reg instr-class=general isa=A64 mnemonic=ORN reguse=shifted-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ORN_64_log_shift alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=log_shift page=MVN_ORN_log_shift cclass=MVN_ORN_log_shift_no_s',
  categories => 'GENERAL|BITWISE',
  eq_to      => 'ORN <Xd>, XZR, <Xm>{, <shift> #<amount>}',
};

ENCODING NEG_SUB_32_addsub_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S:u=0b0 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=NEG cond-setting=no-s datatype=32 instr-class=general isa=A64 mnemonic=SUB',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SUB_32_addsub_shift alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_shift page=NEG_SUB_addsub_shift cclass=NEG_SUB_addsub_shift_no_s',
  categories => 'GENERAL|BITWISE',
  eq_to      => 'SUB  <Wd>, WZR, <Wm> {, <shift> #<amount>}',
};

ENCODING NEG_SUB_64_addsub_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S:u=0b0 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=NEG cond-setting=no-s datatype=64 instr-class=general isa=A64 mnemonic=SUB',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SUB_64_addsub_shift alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_shift page=NEG_SUB_addsub_shift cclass=NEG_SUB_addsub_shift_no_s',
  categories => 'GENERAL|BITWISE',
  eq_to      => 'SUB  <Xd>, XZR, <Xm> {, <shift> #<amount>}',
};

ENCODING NEGS_SUBS_32_addsub_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S:u=0b1 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=NEGS cond-setting=S datatype=32 instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SUBS_32_addsub_shift alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_shift page=NEGS_SUBS_addsub_shift cclass=NEGS_SUBS_addsub_shift_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|BITWISE',
  eq_to      => 'SUBS <Wd>, WZR, <Wm> {, <shift> #<amount>}',
};

ENCODING NEGS_SUBS_64_addsub_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S:u=0b1 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=NEGS cond-setting=S datatype=64 instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SUBS_64_addsub_shift alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_shift page=NEGS_SUBS_addsub_shift cclass=NEGS_SUBS_addsub_shift_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|BITWISE',
  eq_to      => 'SUBS <Xd>, XZR, <Xm> {, <shift> #<amount>}',
};

ENCODING NGC_SBC_32_addsub_carry => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S:u=0b0 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=NGC cond-setting=no-s datatype=32 instr-class=general isa=A64 mnemonic=SBC',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SBC_32_addsub_carry alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_carry page=NGC_SBC cclass=NGC_SBC_no_s',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE|CARRY',
  eq_to      => 'SBC <Wd>, WZR, <Wm>',
};

ENCODING NGC_SBC_64_addsub_carry => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S:u=0b0 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=NGC cond-setting=no-s datatype=64 instr-class=general isa=A64 mnemonic=SBC',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SBC_64_addsub_carry alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_carry page=NGC_SBC cclass=NGC_SBC_no_s',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE|CARRY',
  eq_to      => 'SBC <Xd>, XZR, <Xm>',
};

ENCODING NGCS_SBCS_32_addsub_carry => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op:u=0b1 S:u=0b1 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=NGCS cond-setting=S datatype=32 instr-class=general isa=A64 mnemonic=SBCS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SBCS_32_addsub_carry alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=addsub_carry page=NGCS_SBCS cclass=NGCS_SBCS_s',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|BITWISE|CARRY',
  eq_to      => 'SBCS <Wd>, WZR, <Wm>',
};

ENCODING NGCS_SBCS_64_addsub_carry => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op:u=0b1 S:u=0b1 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0b11111 Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=NGCS cond-setting=S datatype=64 instr-class=general isa=A64 mnemonic=SBCS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SBCS_64_addsub_carry alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=addsub_carry page=NGCS_SBCS cclass=NGCS_SBCS_s',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|BITWISE|CARRY',
  eq_to      => 'SBCS <Xd>, XZR, <Xm>',
};

ENCODING REV64_REV_64_dp_1src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:2]=0b0000 opc:u=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=REV64 datatype=64 instr-class=general isa=A64 mnemonic=REV',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=REV_64_dp_1src alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_1src page=REV64_REV cclass=REV64_REV_general',
  categories => 'GENERAL|MISC',
  eq_to      => 'REV  <Xd>, <Xn>',
};

ENCODING ROR_EXTR_32_extract => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op21=0b00 ig0=0b100111 N:u=0b0 o0=0b0 Rm:u=0bxxxxx imms:u=0b0xxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0 && imms == 0b0xxxxx',
  aliascond  => 'Rn == Rm',
  docvars    => 'alias_mnemonic=ROR datatype=32 instr-class=general isa=A64 mnemonic=EXTR source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=EXTR_32_extract alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=extract page=ROR_EXTR cclass=ROR_EXTR_general',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'EXTR <Wd>, <Ws>, <Ws>, #<shift>',
};

ENCODING ROR_EXTR_64_extract => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op21=0b00 ig0=0b100111 N:u=0b1 o0=0b0 Rm:u=0bxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && N == 0b1',
  aliascond  => 'Rn == Rm',
  docvars    => 'alias_mnemonic=ROR datatype=64 instr-class=general isa=A64 mnemonic=EXTR source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=EXTR_64_extract alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=extract page=ROR_EXTR cclass=ROR_EXTR_general',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'EXTR <Xd>, <Xs>, <Xs>, #<shift>',
};

ENCODING ROR_RORV_32_dp_2src => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ROR datatype=32 instr-class=general isa=A64 mnemonic=RORV source-type=src-is-register',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=RORV_32_dp_2src alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=dp_2src page=ROR_RORV cclass=ROR_RORV_general',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'RORV <Wd>, <Wn>, <Wm>',
};

ENCODING ROR_RORV_64_dp_2src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ROR datatype=64 instr-class=general isa=A64 mnemonic=RORV source-type=src-is-register',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=RORV_64_dp_2src alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_2src page=ROR_RORV cclass=ROR_RORV_general',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'RORV <Xd>, <Xn>, <Xm>',
};

ENCODING SBFIZ_SBFM_32M_bitfield => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b00 ig0=0b100110 N:u=0b0 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  aliascond  => 'UInt(imms) < UInt(immr)',
  docvars    => 'alias_mnemonic=SBFIZ bitfield-fill=signed-fill datatype=32 instr-class=general isa=A64 mnemonic=SBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SBFM_32M_bitfield alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=bitfield page=SBFIZ_SBFM cclass=SBFIZ_SBFM_signed_fill',
  categories => 'GENERAL|BITFIELD|SIGNED',
  eq_to      => 'SBFM <Wd>, <Wn>, #(-<lsb> MOD 32), #(<width>-1)',
};

ENCODING SBFIZ_SBFM_64M_bitfield => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b00 ig0=0b100110 N:u=0b1 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && N == 0b1',
  aliascond  => 'UInt(imms) < UInt(immr)',
  docvars    => 'alias_mnemonic=SBFIZ bitfield-fill=signed-fill datatype=64 instr-class=general isa=A64 mnemonic=SBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SBFM_64M_bitfield alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=bitfield page=SBFIZ_SBFM cclass=SBFIZ_SBFM_signed_fill',
  categories => 'GENERAL|BITFIELD|SIGNED',
  eq_to      => 'SBFM <Xd>, <Xn>, #(-<lsb> MOD 64), #(<width>-1)',
};

ENCODING SBFX_SBFM_32M_bitfield => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b00 ig0=0b100110 N:u=0b0 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  aliascond  => 'BFXPreferred(sf, opc<1>, imms, immr)',
  docvars    => 'alias_mnemonic=SBFX bitfield-fill=signed-fill datatype=32 instr-class=general isa=A64 mnemonic=SBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SBFM_32M_bitfield alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=bitfield page=SBFX_SBFM cclass=SBFX_SBFM_signed_fill',
  categories => 'GENERAL|BITFIELD|SIGNED',
  eq_to      => 'SBFM <Wd>, <Wn>, #<lsb>, #(<lsb>+<width>-1)',
};

ENCODING SBFX_SBFM_64M_bitfield => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b00 ig0=0b100110 N:u=0b1 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && N == 0b1',
  aliascond  => 'BFXPreferred(sf, opc<1>, imms, immr)',
  docvars    => 'alias_mnemonic=SBFX bitfield-fill=signed-fill datatype=64 instr-class=general isa=A64 mnemonic=SBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SBFM_64M_bitfield alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=bitfield page=SBFX_SBFM cclass=SBFX_SBFM_signed_fill',
  categories => 'GENERAL|BITFIELD|SIGNED',
  eq_to      => 'SBFM <Xd>, <Xn>, #<lsb>, #(<lsb>+<width>-1)',
};

ENCODING SMNEGL_SMSUBL_64WA_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b0 ig1=0b01 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0b11111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=SMNEGL datatype=64 instr-class=general isa=A64 mnemonic=SMSUBL',
  docvars2   => 'shape=long',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SMSUBL_64WA_dp_3src alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=SMNEGL_SMSUBL cclass=SMNEGL_SMSUBL_SMSUBL_64WA_dp_3src',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
  eq_to      => 'SMSUBL <Xd>, <Wn>, <Wm>, XZR',
};

ENCODING SMULL_SMADDL_64WA_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b0 ig1=0b01 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0b11111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=SMULL datatype=64 instr-class=general isa=A64 mnemonic=SMADDL',
  docvars2   => 'shape=long',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SMADDL_64WA_dp_3src alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=SMULL_SMADDL cclass=SMULL_SMADDL_SMADDL_64WA_dp_3src',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
  eq_to      => 'SMADDL <Xd>, <Wn>, <Wm>, XZR',
};

ENCODING STADD_LDADD_32_memop => {
  name       => '32-bit LDADD alias',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b10 && R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STADD atomic-ops=LDADD-32-reg instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDADD reg-type=32-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDADD_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STADD_LDADD cclass=STADD_LDADD_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
  eq_to      => 'LDADD <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STADDL_LDADDL_32_memop => {
  name       => '32-bit LDADDL alias',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b10 && R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STADDL atomic-ops=LDADDL-32-reg instr-class=general isa=A64 loadstore-order=release mnemonic=LDADDL reg-type=32-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDADDL_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STADD_LDADD cclass=STADD_LDADD_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
  eq_to      => 'LDADDL <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STADD_LDADD_64_memop => {
  name       => '64-bit LDADD alias',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b11 && R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STADD atomic-ops=LDADD-64-reg instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDADD reg-type=64-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDADD_64_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STADD_LDADD cclass=STADD_LDADD_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
  eq_to      => 'LDADD <Xs>, XZR, [<Xn|SP>]',
};

ENCODING STADDL_LDADDL_64_memop => {
  name       => '64-bit LDADDL alias',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b11 && R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STADDL atomic-ops=LDADDL-64-reg instr-class=general isa=A64 loadstore-order=release mnemonic=LDADDL reg-type=64-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDADDL_64_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STADD_LDADD cclass=STADD_LDADD_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
  eq_to      => 'LDADDL <Xs>, XZR, [<Xn|SP>]',
};

ENCODING STADDB_LDADDB_32_memop => {
  name       => 'No memory ordering',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STADDB instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDADDB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDADDB_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STADDB_LDADDB cclass=STADDB_LDADDB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
  eq_to      => 'LDADDB <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STADDLB_LDADDLB_32_memop => {
  name       => 'Release',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STADDLB instr-class=general isa=A64 loadstore-order=release mnemonic=LDADDLB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDADDLB_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STADDB_LDADDB cclass=STADDB_LDADDB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
  eq_to      => 'LDADDLB <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STADDH_LDADDH_32_memop => {
  name       => 'No memory ordering',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STADDH instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDADDH',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDADDH_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STADDH_LDADDH cclass=STADDH_LDADDH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
  eq_to      => 'LDADDH <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STADDLH_LDADDLH_32_memop => {
  name       => 'Release',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STADDLH instr-class=general isa=A64 loadstore-order=release mnemonic=LDADDLH',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDADDLH_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STADDH_LDADDH cclass=STADDH_LDADDH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|ARITHMETIC',
  eq_to      => 'LDADDLH <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STCLR_LDCLR_32_memop => {
  name       => '32-bit LDCLR alias',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b10 && R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STCLR atomic-ops=LDCLR-32-reg instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDCLR reg-type=32-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDCLR_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STCLR_LDCLR cclass=STCLR_LDCLR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDCLR <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STCLRL_LDCLRL_32_memop => {
  name       => '32-bit LDCLRL alias',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b10 && R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STCLRL atomic-ops=LDCLRL-32-reg instr-class=general isa=A64 loadstore-order=release mnemonic=LDCLRL reg-type=32-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDCLRL_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STCLR_LDCLR cclass=STCLR_LDCLR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDCLRL <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STCLR_LDCLR_64_memop => {
  name       => '64-bit LDCLR alias',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b11 && R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STCLR atomic-ops=LDCLR-64-reg instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDCLR reg-type=64-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDCLR_64_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STCLR_LDCLR cclass=STCLR_LDCLR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDCLR <Xs>, XZR, [<Xn|SP>]',
};

ENCODING STCLRL_LDCLRL_64_memop => {
  name       => '64-bit LDCLRL alias',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b11 && R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STCLRL atomic-ops=LDCLRL-64-reg instr-class=general isa=A64 loadstore-order=release mnemonic=LDCLRL reg-type=64-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDCLRL_64_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STCLR_LDCLR cclass=STCLR_LDCLR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDCLRL <Xs>, XZR, [<Xn|SP>]',
};

ENCODING STCLRB_LDCLRB_32_memop => {
  name       => 'No memory ordering',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STCLRB instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDCLRB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDCLRB_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STCLRB_LDCLRB cclass=STCLRB_LDCLRB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDCLRB <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STCLRLB_LDCLRLB_32_memop => {
  name       => 'Release',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STCLRLB instr-class=general isa=A64 loadstore-order=release mnemonic=LDCLRLB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDCLRLB_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STCLRB_LDCLRB cclass=STCLRB_LDCLRB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDCLRLB <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STCLRH_LDCLRH_32_memop => {
  name       => 'No memory ordering',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STCLRH instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDCLRH',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDCLRH_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STCLRH_LDCLRH cclass=STCLRH_LDCLRH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDCLRH <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STCLRLH_LDCLRLH_32_memop => {
  name       => 'Release',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STCLRLH instr-class=general isa=A64 loadstore-order=release mnemonic=LDCLRLH',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDCLRLH_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STCLRH_LDCLRH cclass=STCLRH_LDCLRH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDCLRLH <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STEOR_LDEOR_32_memop => {
  name       => '32-bit LDEOR alias',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b10 && R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STEOR atomic-ops=LDEOR-32-reg instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDEOR reg-type=32-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDEOR_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STEOR_LDEOR cclass=STEOR_LDEOR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDEOR <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STEORL_LDEORL_32_memop => {
  name       => '32-bit LDEORL alias',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b10 && R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STEORL atomic-ops=LDEORL-32-reg instr-class=general isa=A64 loadstore-order=release mnemonic=LDEORL reg-type=32-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDEORL_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STEOR_LDEOR cclass=STEOR_LDEOR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDEORL <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STEOR_LDEOR_64_memop => {
  name       => '64-bit LDEOR alias',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b11 && R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STEOR atomic-ops=LDEOR-64-reg instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDEOR reg-type=64-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDEOR_64_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STEOR_LDEOR cclass=STEOR_LDEOR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDEOR <Xs>, XZR, [<Xn|SP>]',
};

ENCODING STEORL_LDEORL_64_memop => {
  name       => '64-bit LDEORL alias',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b11 && R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STEORL atomic-ops=LDEORL-64-reg instr-class=general isa=A64 loadstore-order=release mnemonic=LDEORL reg-type=64-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDEORL_64_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STEOR_LDEOR cclass=STEOR_LDEOR_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDEORL <Xs>, XZR, [<Xn|SP>]',
};

ENCODING STEORB_LDEORB_32_memop => {
  name       => 'No memory ordering',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STEORB instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDEORB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDEORB_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STEORB_LDEORB cclass=STEORB_LDEORB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDEORB <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STEORLB_LDEORLB_32_memop => {
  name       => 'Release',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STEORLB instr-class=general isa=A64 loadstore-order=release mnemonic=LDEORLB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDEORLB_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STEORB_LDEORB cclass=STEORB_LDEORB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDEORLB <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STEORH_LDEORH_32_memop => {
  name       => 'No memory ordering',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STEORH instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDEORH',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDEORH_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STEORH_LDEORH cclass=STEORH_LDEORH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDEORH <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STEORLH_LDEORLH_32_memop => {
  name       => 'Release',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STEORLH instr-class=general isa=A64 loadstore-order=release mnemonic=LDEORLH',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDEORLH_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STEORH_LDEORH cclass=STEORH_LDEORH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDEORLH <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSET_LDSET_32_memop => {
  name       => '32-bit LDSET alias',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b10 && R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSET atomic-ops=LDSET-32-reg instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDSET reg-type=32-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSET_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSET_LDSET cclass=STSET_LDSET_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDSET <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSETL_LDSETL_32_memop => {
  name       => '32-bit LDSETL alias',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b10 && R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSETL atomic-ops=LDSETL-32-reg instr-class=general isa=A64 loadstore-order=release mnemonic=LDSETL reg-type=32-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSETL_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSET_LDSET cclass=STSET_LDSET_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDSETL <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSET_LDSET_64_memop => {
  name       => '64-bit LDSET alias',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b11 && R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSET atomic-ops=LDSET-64-reg instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDSET reg-type=64-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSET_64_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSET_LDSET cclass=STSET_LDSET_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDSET <Xs>, XZR, [<Xn|SP>]',
};

ENCODING STSETL_LDSETL_64_memop => {
  name       => '64-bit LDSETL alias',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b11 && R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSETL atomic-ops=LDSETL-64-reg instr-class=general isa=A64 loadstore-order=release mnemonic=LDSETL reg-type=64-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSETL_64_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSET_LDSET cclass=STSET_LDSET_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDSETL <Xs>, XZR, [<Xn|SP>]',
};

ENCODING STSETB_LDSETB_32_memop => {
  name       => 'No memory ordering',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSETB instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDSETB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSETB_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSETB_LDSETB cclass=STSETB_LDSETB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDSETB <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSETLB_LDSETLB_32_memop => {
  name       => 'Release',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSETLB instr-class=general isa=A64 loadstore-order=release mnemonic=LDSETLB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSETLB_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSETB_LDSETB cclass=STSETB_LDSETB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDSETLB <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSETH_LDSETH_32_memop => {
  name       => 'No memory ordering',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSETH instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDSETH',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSETH_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSETH_LDSETH cclass=STSETH_LDSETH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDSETH <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSETLH_LDSETLH_32_memop => {
  name       => 'Release',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSETLH instr-class=general isa=A64 loadstore-order=release mnemonic=LDSETLH',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSETLH_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSETH_LDSETH cclass=STSETH_LDSETH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|BITWISE',
  eq_to      => 'LDSETLH <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSMAX_LDSMAX_32_memop => {
  name       => '32-bit LDSMAX alias',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b10 && R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSMAX atomic-ops=LDSMAX-32-reg instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDSMAX reg-type=32-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSMAX_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMAX_LDSMAX cclass=STSMAX_LDSMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
  eq_to      => 'LDSMAX <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSMAXL_LDSMAXL_32_memop => {
  name       => '32-bit LDSMAXL alias',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b10 && R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSMAXL atomic-ops=LDSMAXL-32-reg instr-class=general isa=A64 loadstore-order=release mnemonic=LDSMAXL reg-type=32-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSMAXL_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMAX_LDSMAX cclass=STSMAX_LDSMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
  eq_to      => 'LDSMAXL <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSMAX_LDSMAX_64_memop => {
  name       => '64-bit LDSMAX alias',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b11 && R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSMAX atomic-ops=LDSMAX-64-reg instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDSMAX reg-type=64-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSMAX_64_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMAX_LDSMAX cclass=STSMAX_LDSMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
  eq_to      => 'LDSMAX <Xs>, XZR, [<Xn|SP>]',
};

ENCODING STSMAXL_LDSMAXL_64_memop => {
  name       => '64-bit LDSMAXL alias',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b11 && R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSMAXL atomic-ops=LDSMAXL-64-reg instr-class=general isa=A64 loadstore-order=release mnemonic=LDSMAXL reg-type=64-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSMAXL_64_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMAX_LDSMAX cclass=STSMAX_LDSMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
  eq_to      => 'LDSMAXL <Xs>, XZR, [<Xn|SP>]',
};

ENCODING STSMAXB_LDSMAXB_32_memop => {
  name       => 'No memory ordering',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSMAXB instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDSMAXB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSMAXB_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMAXB_LDSMAXB cclass=STSMAXB_LDSMAXB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
  eq_to      => 'LDSMAXB <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSMAXLB_LDSMAXLB_32_memop => {
  name       => 'Release',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSMAXLB instr-class=general isa=A64 loadstore-order=release mnemonic=LDSMAXLB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSMAXLB_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMAXB_LDSMAXB cclass=STSMAXB_LDSMAXB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
  eq_to      => 'LDSMAXLB <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSMAXH_LDSMAXH_32_memop => {
  name       => 'No memory ordering',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSMAXH instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDSMAXH',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSMAXH_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMAXH_LDSMAXH cclass=STSMAXH_LDSMAXH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
  eq_to      => 'LDSMAXH <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSMAXLH_LDSMAXLH_32_memop => {
  name       => 'Release',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSMAXLH instr-class=general isa=A64 loadstore-order=release mnemonic=LDSMAXLH',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSMAXLH_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMAXH_LDSMAXH cclass=STSMAXH_LDSMAXH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
  eq_to      => 'LDSMAXLH <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSMIN_LDSMIN_32_memop => {
  name       => '32-bit LDSMIN alias',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b10 && R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSMIN atomic-ops=LDSMIN-32-reg instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDSMIN reg-type=32-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSMIN_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMIN_LDSMIN cclass=STSMIN_LDSMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
  eq_to      => 'LDSMIN <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSMINL_LDSMINL_32_memop => {
  name       => '32-bit LDSMINL alias',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b10 && R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSMINL atomic-ops=LDSMINL-32-reg instr-class=general isa=A64 loadstore-order=release mnemonic=LDSMINL reg-type=32-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSMINL_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMIN_LDSMIN cclass=STSMIN_LDSMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
  eq_to      => 'LDSMINL <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSMIN_LDSMIN_64_memop => {
  name       => '64-bit LDSMIN alias',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b11 && R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSMIN atomic-ops=LDSMIN-64-reg instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDSMIN reg-type=64-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSMIN_64_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMIN_LDSMIN cclass=STSMIN_LDSMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
  eq_to      => 'LDSMIN <Xs>, XZR, [<Xn|SP>]',
};

ENCODING STSMINL_LDSMINL_64_memop => {
  name       => '64-bit LDSMINL alias',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b11 && R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSMINL atomic-ops=LDSMINL-64-reg instr-class=general isa=A64 loadstore-order=release mnemonic=LDSMINL reg-type=64-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSMINL_64_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMIN_LDSMIN cclass=STSMIN_LDSMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
  eq_to      => 'LDSMINL <Xs>, XZR, [<Xn|SP>]',
};

ENCODING STSMINB_LDSMINB_32_memop => {
  name       => 'No memory ordering',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSMINB instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDSMINB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSMINB_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMINB_LDSMINB cclass=STSMINB_LDSMINB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
  eq_to      => 'LDSMINB <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSMINLB_LDSMINLB_32_memop => {
  name       => 'Release',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSMINLB instr-class=general isa=A64 loadstore-order=release mnemonic=LDSMINLB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSMINLB_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMINB_LDSMINB cclass=STSMINB_LDSMINB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
  eq_to      => 'LDSMINLB <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSMINH_LDSMINH_32_memop => {
  name       => 'No memory ordering',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSMINH instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDSMINH',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSMINH_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMINH_LDSMINH cclass=STSMINH_LDSMINH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
  eq_to      => 'LDSMINH <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STSMINLH_LDSMINLH_32_memop => {
  name       => 'Release',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STSMINLH instr-class=general isa=A64 loadstore-order=release mnemonic=LDSMINLH',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDSMINLH_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMINH_LDSMINH cclass=STSMINH_LDSMINH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|SIGNED',
  eq_to      => 'LDSMINLH <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STUMAX_LDUMAX_32_memop => {
  name       => '32-bit LDUMAX alias',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b10 && R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STUMAX atomic-ops=LDUMAX-32-reg instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDUMAX reg-type=32-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDUMAX_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMAX_LDUMAX cclass=STUMAX_LDUMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
  eq_to      => 'LDUMAX <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STUMAXL_LDUMAXL_32_memop => {
  name       => '32-bit LDUMAXL alias',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b10 && R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STUMAXL atomic-ops=LDUMAXL-32-reg instr-class=general isa=A64 loadstore-order=release mnemonic=LDUMAXL reg-type=32-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDUMAXL_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMAX_LDUMAX cclass=STUMAX_LDUMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
  eq_to      => 'LDUMAXL <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STUMAX_LDUMAX_64_memop => {
  name       => '64-bit LDUMAX alias',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b11 && R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STUMAX atomic-ops=LDUMAX-64-reg instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDUMAX reg-type=64-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDUMAX_64_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMAX_LDUMAX cclass=STUMAX_LDUMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
  eq_to      => 'LDUMAX <Xs>, XZR, [<Xn|SP>]',
};

ENCODING STUMAXL_LDUMAXL_64_memop => {
  name       => '64-bit LDUMAXL alias',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b11 && R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STUMAXL atomic-ops=LDUMAXL-64-reg instr-class=general isa=A64 loadstore-order=release mnemonic=LDUMAXL reg-type=64-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDUMAXL_64_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMAX_LDUMAX cclass=STUMAX_LDUMAX_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
  eq_to      => 'LDUMAXL <Xs>, XZR, [<Xn|SP>]',
};

ENCODING STUMAXB_LDUMAXB_32_memop => {
  name       => 'No memory ordering',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STUMAXB instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDUMAXB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDUMAXB_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMAXB_LDUMAXB cclass=STUMAXB_LDUMAXB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
  eq_to      => 'LDUMAXB <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STUMAXLB_LDUMAXLB_32_memop => {
  name       => 'Release',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STUMAXLB instr-class=general isa=A64 loadstore-order=release mnemonic=LDUMAXLB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDUMAXLB_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMAXB_LDUMAXB cclass=STUMAXB_LDUMAXB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
  eq_to      => 'LDUMAXLB <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STUMAXH_LDUMAXH_32_memop => {
  name       => 'No memory ordering',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STUMAXH instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDUMAXH',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDUMAXH_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMAXH_LDUMAXH cclass=STUMAXH_LDUMAXH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
  eq_to      => 'LDUMAXH <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STUMAXLH_LDUMAXLH_32_memop => {
  name       => 'Release',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STUMAXLH instr-class=general isa=A64 loadstore-order=release mnemonic=LDUMAXLH',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDUMAXLH_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMAXH_LDUMAXH cclass=STUMAXH_LDUMAXH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
  eq_to      => 'LDUMAXLH <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STUMIN_LDUMIN_32_memop => {
  name       => '32-bit LDUMIN alias',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b10 && R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STUMIN atomic-ops=LDUMIN-32-reg instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDUMIN reg-type=32-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDUMIN_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMIN_LDUMIN cclass=STUMIN_LDUMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
  eq_to      => 'LDUMIN <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STUMINL_LDUMINL_32_memop => {
  name       => '32-bit LDUMINL alias',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b10 && R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STUMINL atomic-ops=LDUMINL-32-reg instr-class=general isa=A64 loadstore-order=release mnemonic=LDUMINL reg-type=32-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDUMINL_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMIN_LDUMIN cclass=STUMIN_LDUMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
  eq_to      => 'LDUMINL <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STUMIN_LDUMIN_64_memop => {
  name       => '64-bit LDUMIN alias',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b11 && R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STUMIN atomic-ops=LDUMIN-64-reg instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDUMIN reg-type=64-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDUMIN_64_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMIN_LDUMIN cclass=STUMIN_LDUMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
  eq_to      => 'LDUMIN <Xs>, XZR, [<Xn|SP>]',
};

ENCODING STUMINL_LDUMINL_64_memop => {
  name       => '64-bit LDUMINL alias',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'size == 0b11 && R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STUMINL atomic-ops=LDUMINL-64-reg instr-class=general isa=A64 loadstore-order=release mnemonic=LDUMINL reg-type=64-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDUMINL_64_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMIN_LDUMIN cclass=STUMIN_LDUMIN_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
  eq_to      => 'LDUMINL <Xs>, XZR, [<Xn|SP>]',
};

ENCODING STUMINB_LDUMINB_32_memop => {
  name       => 'No memory ordering',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STUMINB instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDUMINB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDUMINB_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMINB_LDUMINB cclass=STUMINB_LDUMINB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
  eq_to      => 'LDUMINB <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STUMINLB_LDUMINLB_32_memop => {
  name       => 'Release',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STUMINLB instr-class=general isa=A64 loadstore-order=release mnemonic=LDUMINLB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDUMINLB_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMINB_LDUMINB cclass=STUMINB_LDUMINB_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
  eq_to      => 'LDUMINLB <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STUMINH_LDUMINH_32_memop => {
  name       => 'No memory ordering',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b0 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STUMINH instr-class=general isa=A64 loadstore-order=no-order mnemonic=LDUMINH',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDUMINH_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMINH_LDUMINH cclass=STUMINH_LDUMINH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
  eq_to      => 'LDUMINH <Ws>, WZR, [<Xn|SP>]',
};

ENCODING STUMINLH_LDUMINLH_32_memop => {
  name       => 'Release',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0b1 ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  bitdiffs   => 'R == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=STUMINLH instr-class=general isa=A64 loadstore-order=release mnemonic=LDUMINLH',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 aliasof=LDUMINLH_32_memop alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMINH_LDUMINH cclass=STUMINH_LDUMINH_general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  categories => 'GENERAL|ATOMIC|MATH|MIN_MAX|UNSIGNED',
  eq_to      => 'LDUMINLH <Ws>, WZR, [<Xn|SP>]',
};

ENCODING SXTB_SBFM_32M_bitfield => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b00 ig0=0b100110 N:u=0b0 immr:u=0b000000 imms:u=0b000111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=SXTB bitfield-fill=signed-fill datatype=32 instr-class=general isa=A64 mnemonic=SBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SBFM_32M_bitfield alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=bitfield page=SXTB_SBFM cclass=SXTB_SBFM_signed_fill',
  categories => 'GENERAL|SIGN_EXTEND|SIGNED',
  eq_to      => 'SBFM <Wd>, <Wn>, #0, #7',
};

ENCODING SXTB_SBFM_64M_bitfield => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b00 ig0=0b100110 N:u=0b1 immr:u=0b000000 imms:u=0b000111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && N == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=SXTB bitfield-fill=signed-fill datatype=64 instr-class=general isa=A64 mnemonic=SBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SBFM_64M_bitfield alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=bitfield page=SXTB_SBFM cclass=SXTB_SBFM_signed_fill',
  categories => 'GENERAL|SIGN_EXTEND|SIGNED',
  eq_to      => 'SBFM <Xd>, <Xn>, #0, #7',
};

ENCODING SXTH_SBFM_32M_bitfield => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b00 ig0=0b100110 N:u=0b0 immr:u=0b000000 imms:u=0b001111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=SXTH bitfield-fill=signed-fill datatype=32 instr-class=general isa=A64 mnemonic=SBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SBFM_32M_bitfield alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=bitfield page=SXTH_SBFM cclass=SXTH_SBFM_signed_fill',
  categories => 'GENERAL|SIGN_EXTEND',
  eq_to      => 'SBFM <Wd>, <Wn>, #0, #15',
};

ENCODING SXTH_SBFM_64M_bitfield => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b00 ig0=0b100110 N:u=0b1 immr:u=0b000000 imms:u=0b001111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && N == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=SXTH bitfield-fill=signed-fill datatype=64 instr-class=general isa=A64 mnemonic=SBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SBFM_64M_bitfield alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=bitfield page=SXTH_SBFM cclass=SXTH_SBFM_signed_fill',
  categories => 'GENERAL|SIGN_EXTEND',
  eq_to      => 'SBFM <Xd>, <Xn>, #0, #15',
};

ENCODING SXTW_SBFM_64M_bitfield => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b00 ig0=0b100110 N:u=0b1 immr:u=0b000000 imms:u=0b011111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=SXTW bitfield-fill=signed-fill datatype=64 instr-class=general isa=A64 mnemonic=SBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SBFM_64M_bitfield alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=bitfield page=SXTW_SBFM cclass=SXTW_SBFM_signed_fill',
  categories => 'GENERAL|SIGN_EXTEND',
  eq_to      => 'SBFM <Xd>, <Xn>, #0, #31',
};

ENCODING TLBI_SYS_CR_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b01 op1:u=0bxxx CRn:u=0b1000 CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  aliascond  => 'SysOp(op1,0b1000,CRm,op2) == Sys_TLBI',
  docvars    => 'alias_mnemonic=TLBI instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=SYS_CR_systeminstrs alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=TLBI_SYS cclass=TLBI_SYS_SYS_CR_systeminstrs',
  categories => 'SYSTEM',
  eq_to      => 'SYS #<op1>, C8, <Cm>, #<op2>{, <Xt>}',
};

ENCODING TST_ANDS_32S_log_imm => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b11 ig0=0b100100 N:u=0b0 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=TST cond-setting=S datatype=32 immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=ANDS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ANDS_32S_log_imm alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=log_imm page=TST_ANDS_log_imm cclass=TST_ANDS_log_imm_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|COMPARISON',
  eq_to      => 'ANDS WZR, <Wn>, #<imm>',
};

ENCODING TST_ANDS_64S_log_imm => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b11 ig0=0b100100 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=TST cond-setting=S datatype=64 immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=ANDS',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ANDS_64S_log_imm alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=log_imm page=TST_ANDS_log_imm cclass=TST_ANDS_log_imm_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|COMPARISON',
  eq_to      => 'ANDS XZR, <Xn>, #<imm>',
};

ENCODING TST_ANDS_32_log_shift => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b11 ig0=0b01010 shift:u=0bxx N:u=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  bitdiffs   => 'sf == 0b0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=TST cond-setting=S datatype=32 datatype-reguse=32-shifted-reg instr-class=general isa=A64 mnemonic=ANDS reguse=shifted-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ANDS_32_log_shift alphaindex=BASIC datasize=32',
  tags       => 'group=dpreg iclass=log_shift page=TST_ANDS_log_shift cclass=TST_ANDS_log_shift_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|COMPARISON',
  eq_to      => 'ANDS WZR, <Wn>, <Wm>{, <shift> #<amount>}',
};

ENCODING TST_ANDS_64_log_shift => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b11 ig0=0b01010 shift:u=0bxx N:u=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  bitdiffs   => 'sf == 0b1',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=TST cond-setting=S datatype=64 datatype-reguse=64-shifted-reg instr-class=general isa=A64 mnemonic=ANDS reguse=shifted-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ANDS_64_log_shift alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=log_shift page=TST_ANDS_log_shift cclass=TST_ANDS_log_shift_s',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|COMPARISON',
  eq_to      => 'ANDS XZR, <Xn>, <Xm>{, <shift> #<amount>}',
};

ENCODING UBFIZ_UBFM_32M_bitfield => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b10 ig0=0b100110 N:u=0b0 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  aliascond  => 'UInt(imms) < UInt(immr)',
  docvars    => 'alias_mnemonic=UBFIZ bitfield-fill=zero-fill datatype=32 instr-class=general isa=A64 mnemonic=UBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=UBFM_32M_bitfield alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=bitfield page=UBFIZ_UBFM cclass=UBFIZ_UBFM_zero_fill',
  categories => 'GENERAL|BITFIELD|UNSIGNED',
  eq_to      => 'UBFM <Wd>, <Wn>, #(-<lsb> MOD 32), #(<width>-1)',
};

ENCODING UBFIZ_UBFM_64M_bitfield => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b10 ig0=0b100110 N:u=0b1 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && N == 0b1',
  aliascond  => 'UInt(imms) < UInt(immr)',
  docvars    => 'alias_mnemonic=UBFIZ bitfield-fill=zero-fill datatype=64 instr-class=general isa=A64 mnemonic=UBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=UBFM_64M_bitfield alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=bitfield page=UBFIZ_UBFM cclass=UBFIZ_UBFM_zero_fill',
  categories => 'GENERAL|BITFIELD|UNSIGNED',
  eq_to      => 'UBFM <Xd>, <Xn>, #(-<lsb> MOD 64), #(<width>-1)',
};

ENCODING UBFX_UBFM_32M_bitfield => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b10 ig0=0b100110 N:u=0b0 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b0 && N == 0b0',
  aliascond  => 'BFXPreferred(sf, opc<1>, imms, immr)',
  docvars    => 'alias_mnemonic=UBFX bitfield-fill=zero-fill datatype=32 instr-class=general isa=A64 mnemonic=UBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=UBFM_32M_bitfield alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=bitfield page=UBFX_UBFM cclass=UBFX_UBFM_zero_fill',
  categories => 'GENERAL|BITFIELD|UNSIGNED',
  eq_to      => 'UBFM <Wd>, <Wn>, #<lsb>, #(<lsb>+<width>-1)',
};

ENCODING UBFX_UBFM_64M_bitfield => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 opc:u=0b10 ig0=0b100110 N:u=0b1 immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  bitdiffs   => 'sf == 0b1 && N == 0b1',
  aliascond  => 'BFXPreferred(sf, opc<1>, imms, immr)',
  docvars    => 'alias_mnemonic=UBFX bitfield-fill=zero-fill datatype=64 instr-class=general isa=A64 mnemonic=UBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=UBFM_64M_bitfield alphaindex=BASIC datasize=64',
  tags       => 'group=dpimm iclass=bitfield page=UBFX_UBFM cclass=UBFX_UBFM_zero_fill',
  categories => 'GENERAL|BITFIELD|UNSIGNED',
  eq_to      => 'UBFM <Xd>, <Xn>, #<lsb>, #(<lsb>+<width>-1)',
};

ENCODING UMNEGL_UMSUBL_64WA_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b1 ig1=0b01 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0b11111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=UMNEGL datatype=64 instr-class=general isa=A64 mnemonic=UMSUBL',
  docvars2   => 'shape=long',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=UMSUBL_64WA_dp_3src alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=UMNEGL_UMSUBL cclass=UMNEGL_UMSUBL_UMSUBL_64WA_dp_3src',
  categories => 'GENERAL|CARITHMETIC|UNSIGNED',
  eq_to      => 'UMSUBL <Xd>, <Wn>, <Wm>, XZR',
};

ENCODING UMULL_UMADDL_64WA_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b1 ig1=0b01 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0b11111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=UMULL datatype=64 instr-class=general isa=A64 mnemonic=UMADDL',
  docvars2   => 'shape=long',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=UMADDL_64WA_dp_3src alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=UMULL_UMADDL cclass=UMULL_UMADDL_UMADDL_64WA_dp_3src',
  categories => 'GENERAL|ARITHMETIC|UNSIGNED',
  eq_to      => 'UMADDL <Xd>, <Wn>, <Wm>, XZR',
};

ENCODING UXTB_UBFM_32M_bitfield => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b10 ig0=0b100110 N:u=0b0 immr:u=0b000000 imms:u=0b000111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=UXTB bitfield-fill=zero-fill datatype=32 instr-class=general isa=A64 mnemonic=UBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=UBFM_32M_bitfield alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=bitfield page=UXTB_UBFM cclass=UXTB_UBFM_zero_fill',
  categories => 'GENERAL|SIGN_EXTEND|UNSIGNED',
  eq_to      => 'UBFM <Wd>, <Wn>, #0, #7',
};

ENCODING UXTH_UBFM_32M_bitfield => {
  name       => '32-bit',
  diagram    => 'sf:u=0b0 opc:u=0b10 ig0=0b100110 N:u=0b0 immr:u=0b000000 imms:u=0b001111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=UXTH bitfield-fill=zero-fill datatype=32 instr-class=general isa=A64 mnemonic=UBFM source-type=src-is-immediate',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=UBFM_32M_bitfield alphaindex=BASIC datasize=32',
  tags       => 'group=dpimm iclass=bitfield page=UXTH_UBFM cclass=UXTH_UBFM_zero_fill',
  categories => 'GENERAL|SIGN_EXTEND|UNSIGNED',
  eq_to      => 'UBFM <Wd>, <Wn>, #0, #15',
};


1;

