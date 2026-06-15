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

CLASS ADC_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b0 S:u=0b0 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=ADC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_carry page=ADC xid=no_s',
};

CLASS ADCS_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b0 S:u=0b1 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=ADCS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_carry page=ADCS xid=s',
};

CLASS ADD_addsub_ext_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b0 S:u=0b0 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=ADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_ext page=ADD_addsub_ext xid=no_s',
  undefined  => 'UInt(imm3) > 4',
};

CLASS ADD_addsub_imm_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b0 S:u=0b0 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s immediate-type=imm12u instr-class=general isa=A64 mnemonic=ADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=addsub_imm page=ADD_addsub_imm xid=no_s',
};

CLASS ADD_addsub_shift_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b0 S:u=0b0 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=ADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_shift page=ADD_addsub_shift xid=no_s',
  undefined  => '(shift == 3) || (sf == 0 && imm6<5> == 1)',
};

CLASS ADDG_64_addsub_immtags => {
  name       => 'Integer',
  diagram    => 'sf=0b1 op=0b0 S=0b0 ig0=0b100011 o2=0b0 uimm6:u=0bxxxxxx op3:u=0bZZ uimm4:u=0bxxxx Xn:u=0bxxxxx Xd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=ADDG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=addsub_immtags page=ADDG',
};

CLASS ADDS_addsub_ext_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b0 S:u=0b1 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_ext page=ADDS_addsub_ext xid=s',
  undefined  => 'UInt(imm3) > 4',
};

CLASS ADDS_addsub_imm_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b0 S:u=0b1 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=S immediate-type=imm12u instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=addsub_imm page=ADDS_addsub_imm xid=s',
};

CLASS ADDS_addsub_shift_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b0 S:u=0b1 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_shift page=ADDS_addsub_shift xid=s',
  undefined  => '(shift == 3) || (sf == 0 && imm6<5> == 1)',
};

CLASS ADR_only_pcreladdr => {
  name       => 'Literal',
  diagram    => 'op:u=0b0 immlo:u=0bxx ig0=0b10000 immhi:u=0bxxxxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  docvars    => 'address-form=literal instr-class=general isa=A64 mnemonic=ADR offset-type=off19s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=pcreladdr page=ADR',
};

CLASS ADRP_only_pcreladdr => {
  name       => 'Literal',
  diagram    => 'op:u=0b1 immlo:u=0bxx ig0=0b10000 immhi:u=0bxxxxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  docvars    => 'address-form=literal instr-class=general isa=A64 mnemonic=ADRP offset-type=off19s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=pcreladdr page=ADRP',
};

CLASS AND_log_imm_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx opc:u=0b00 ig0=0b100100 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=AND',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log_imm page=AND_log_imm xid=no_s',
  undefined  => 'sf == 0 && N != 0',
};

CLASS AND_log_shift_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx opc:u=0b00 ig0=0b01010 shift:u=0bxx N:u=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=AND reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=log_shift page=AND_log_shift xid=no_s',
  undefined  => 'sf == 0 && imm6<5> == 1',
};

CLASS ANDS_log_imm_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx opc:u=0b11 ig0=0b100100 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=S immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=ANDS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log_imm page=ANDS_log_imm xid=s',
  undefined  => 'sf == 0 && N != 0',
};

CLASS ANDS_log_shift_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx opc:u=0b11 ig0=0b01010 shift:u=0bxx N:u=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=ANDS reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=log_shift page=ANDS_log_shift xid=s',
  undefined  => 'sf == 0 && imm6<5> == 1',
};

CLASS ASRV_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=ASRV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=ASRV xid=general',
};

CLASS AUTDA_general => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0bx opcode[2:0]=0b110 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=AUTDA xid=general',
  exceptions => 'PACFAIL|PACTRAP',
};

CLASS AUTDB_general => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0bx opcode[2:0]=0b111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=AUTDB xid=general',
  exceptions => 'PACFAIL|PACTRAP',
};

CLASS AUTIA_general => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0bx opcode[2:0]=0b100 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=AUTIA xid=general',
  exceptions => 'PACFAIL|PACTRAP',
};

CLASS AUTIA_system => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b00x1 op2:u=0b10x Rt=0b11111',
  docvars    => 'instr-class=system isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=AUTIA xid=system',
  exceptions => 'PACFAIL|PACTRAP',
};

CLASS AUTIB_general => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0bx opcode[2:0]=0b101 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=AUTIB xid=general',
  exceptions => 'PACFAIL|PACTRAP',
};

CLASS AUTIB_system => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b00x1 op2:u=0b11x Rt=0b11111',
  docvars    => 'instr-class=system isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=AUTIB xid=system',
  exceptions => 'PACFAIL|PACTRAP',
};

CLASS AXFLAG_M_pstate => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b000 CRn=0b0100 CRm:u=0bZZZZ op2=0b010 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=AXFLAG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=control iclass=pstate page=AXFLAG',
  undefined  => '!HaveFlagFormatExt()',
};

CLASS B_only_condbranch => {
  name       => '19-bit signed PC-relative branch offset',
  diagram    => 'ig0=0b0101010 o1=0b0 imm19:u=0bxxxxxxxxxxxxxxxxxxx o0=0b0 cond:u=0bxxxx',
  docvars    => 'branch-offset=br19 compare-with=cmp-cond instr-class=general isa=A64 mnemonic=B',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=condbranch page=B_cond',
};

CLASS B_only_branch_imm => {
  name       => '26-bit signed PC-relative branch offset',
  diagram    => 'op:u=0b0 ig0=0b00101 imm26:u=0bxxxxxxxxxxxxxxxxxxxxxxxxxx',
  docvars    => 'branch-offset=br26 instr-class=general isa=A64 mnemonic=B',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_imm page=B_uncond',
};

CLASS BFM_nofill => {
  name       => 'Leaving other bits unchanged',
  diagram    => 'sf:u=0bx opc:u=0b01 ig0=0b100110 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'bitfield-fill=nofill instr-class=general isa=A64 mnemonic=BFM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=BFM xid=nofill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};

CLASS BIC_log_shift_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx opc:u=0b00 ig0=0b01010 shift:u=0bxx N:u=0b1 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=BIC reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=log_shift page=BIC_log_shift xid=no_s',
  undefined  => 'sf == 0 && imm6<5> == 1',
};

CLASS BICS_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx opc:u=0b11 ig0=0b01010 shift:u=0bxx N:u=0b1 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=BICS reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=log_shift page=BICS xid=s',
  undefined  => 'sf == 0 && imm6<5> == 1',
};

CLASS BL_only_branch_imm => {
  name       => '26-bit signed PC-relative branch offset',
  diagram    => 'op:u=0b1 ig0=0b00101 imm26:u=0bxxxxxxxxxxxxxxxxxxxxxxxxxx',
  docvars    => 'branch-offset=br26 instr-class=general isa=A64 mnemonic=BL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_imm page=BL',
};

CLASS BLR_64_branch_reg => {
  name       => 'Integer',
  diagram    => 'ig0=0b1101011 Z:u=0b0 opc[2:1]=0b0 op:u=0b01 op2=0b11111 op3[5:2]=0b0000 A:u=0b0 M:u=0b0 Rn:u=0bxxxxx Rm:u=0b00000',
  docvars    => 'instr-class=general isa=A64 mnemonic=BLR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=BLR',
};

CLASS BLRA_general => {
  name       => 'Integer',
  diagram    => 'ig0=0b1101011 Z:u=0bx opc[2:1]=0b0 op:u=0b01 op2=0b11111 op3[5:2]=0b0000 A:u=0b1 M:u=0bx Rn:u=0bxxxxx Rm:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=BLRA xid=general',
  exceptions => 'PACFAIL|PACTRAP',
};

CLASS BR_64_branch_reg => {
  name       => 'Integer',
  diagram    => 'ig0=0b1101011 Z:u=0b0 opc[2:1]=0b0 op:u=0b00 op2=0b11111 op3[5:2]=0b0000 A:u=0b0 M:u=0b0 Rn:u=0bxxxxx Rm:u=0b00000',
  docvars    => 'instr-class=general isa=A64 mnemonic=BR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=BR',
};

CLASS BRA_general => {
  name       => 'Integer',
  diagram    => 'ig0=0b1101011 Z:u=0bx opc[2:1]=0b0 op:u=0b00 op2=0b11111 op3[5:2]=0b0000 A:u=0b1 M:u=0bx Rn:u=0bxxxxx Rm:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=BRA xid=general',
  exceptions => 'PACFAIL|PACTRAP',
};

CLASS BRK_EX_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b001 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL=0b00',
  docvars    => 'instr-class=system isa=A64 mnemonic=BRK',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=BRK',
  exceptions => 'SOFTWAREBREAKPOINT',
};

CLASS BTI_HB_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0100 op2:u=0bxx0 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=BTI',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=BTI',
  exceptions => 'WFXTRAP',
};

CLASS CAS_base_register => {
  name       => 'No offset',
  diagram    => 'size:u=0b1x ig0=0b001000 o2=0b1 L:u=0bx o1=0b1 Rs:u=0bxxxxx o0:u=0bx Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CAS xid=base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS CASB_base_register => {
  name       => 'No offset',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b1 L:u=0bx o1=0b1 Rs:u=0bxxxxx o0:u=0bx Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASB xid=base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS CASH_base_register => {
  name       => 'No offset',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b1 L:u=0bx o1=0b1 Rs:u=0bxxxxx o0:u=0bx Rt2=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASH xid=base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS CASP_base_register => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 sz:u=0bx ig1=0b001000 o2=0b0 L:u=0bx o1=0b1 Rs:u=0bxxxxx o0:u=0bx Rt2:u=0b11111 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=CASP xid=base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '(!HaveAtomicExt()) || (Rs<0> == 1) || (Rt<0> == 1)',
};

CLASS CBNZ_br19 => {
  name       => '19-bit signed PC-relative branch offset',
  diagram    => 'sf:u=0bx ig0=0b011010 op:u=0b1 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  docvars    => 'branch-offset=br19 compare-with=cmp-nonzero instr-class=general isa=A64 mnemonic=CBNZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=compbranch page=CBNZ xid=br19',
};

CLASS CBZ_br19 => {
  name       => '19-bit signed PC-relative branch offset',
  diagram    => 'sf:u=0bx ig0=0b011010 op:u=0b0 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  docvars    => 'branch-offset=br19 compare-with=cmp-zero instr-class=general isa=A64 mnemonic=CBZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=compbranch page=CBZ xid=br19',
};

CLASS CCMN_imm_imm5u => {
  name       => '5-bit unsigned immediate',
  diagram    => 'sf:u=0bx op:u=0b0 S=0b1 ig0=0b11010010 imm5:u=0bxxxxx cond:u=0bxxxx ig1=0b1 o2=0b0 Rn:u=0bxxxxx o3=0b0 nzcv:u=0bxxxx',
  docvars    => 'immediate-type=imm5u instr-class=general isa=A64 mnemonic=CCMN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=condcmp_imm page=CCMN_imm xid=imm5u',
};

CLASS CCMN_reg_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op:u=0b0 S=0b1 ig0=0b11010010 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2=0b0 Rn:u=0bxxxxx o3=0b0 nzcv:u=0bxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=CCMN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=condcmp_reg page=CCMN_reg xid=general',
};

CLASS CCMP_imm_imm5u => {
  name       => '5-bit unsigned immediate',
  diagram    => 'sf:u=0bx op:u=0b1 S=0b1 ig0=0b11010010 imm5:u=0bxxxxx cond:u=0bxxxx ig1=0b1 o2=0b0 Rn:u=0bxxxxx o3=0b0 nzcv:u=0bxxxx',
  docvars    => 'immediate-type=imm5u instr-class=general isa=A64 mnemonic=CCMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=condcmp_imm page=CCMP_imm xid=imm5u',
};

CLASS CCMP_reg_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op:u=0b1 S=0b1 ig0=0b11010010 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2=0b0 Rn:u=0bxxxxx o3=0b0 nzcv:u=0bxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=CCMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=condcmp_reg page=CCMP_reg xid=general',
};

CLASS CFINV_M_pstate => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b000 CRn=0b0100 CRm:u=0bZZZZ op2=0b000 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=CFINV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=control iclass=pstate page=CFINV',
  undefined  => '!HaveFlagManipulateExt()',
};

CLASS CLREX_BN_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm:u=0bxxxx op2=0b010 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=CLREX',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=CLREX',
};

CLASS CLS_int_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:1]=0b00010 op:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=CLS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=CLS_int xid=general',
};

CLASS CLZ_int_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:1]=0b00010 op:u=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=CLZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=CLZ_int xid=general',
};

CLASS CRC32_crc => {
  name       => 'CRC',
  diagram    => 'sf:u=0bx op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b010 C:u=0b0 sz:u=0bxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'feature=crc instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=CRC32 xid=crc',
  undefined  => '(!HaveCRCExt()) || (sf == 1 && sz != 3) || (sf == 0 && sz == 3)',
};

CLASS CRC32C_crc => {
  name       => 'CRC',
  diagram    => 'sf:u=0bx op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b010 C:u=0b1 sz:u=0bxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'feature=crc instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=CRC32C xid=crc',
  undefined  => '(!HaveCRCExt()) || (sf == 1 && sz != 3) || (sf == 0 && sz == 3)',
};

CLASS CSDB_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0010 op2:u=0b100 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=CSDB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=CSDB',
};

CLASS CSEL_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op:u=0b0 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=CSEL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=condsel page=CSEL xid=general',
};

CLASS CSINC_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op:u=0b0 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=CSINC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=condsel page=CSINC xid=general',
};

CLASS CSINV_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op:u=0b1 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=CSINV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=condsel page=CSINV xid=general',
};

CLASS CSNEG_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op:u=0b1 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=CSNEG',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=condsel page=CSNEG xid=general',
};

CLASS DCPS1_DC_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b101 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL:u=0b01',
  docvars    => 'instr-class=system isa=A64 mnemonic=DCPS1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=DCPS1',
  undefined  => '!Halted()',
};

CLASS DCPS2_DC_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b101 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL:u=0b10',
  docvars    => 'instr-class=system isa=A64 mnemonic=DCPS2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=DCPS2',
  undefined  => '!Halted()',
};

CLASS DCPS3_DC_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b101 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL:u=0b11',
  docvars    => 'instr-class=system isa=A64 mnemonic=DCPS3',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v0-A alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=DCPS3',
  undefined  => '!Halted()',
};

CLASS DGH_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0000 op2:u=0b110 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=DGH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=DGH',
};

CLASS DMB_BO_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm:u=0bxxxx ig1=0b1 opc:u=0b01 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=DMB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=DMB',
};

CLASS DRPS_64E_branch_reg => {
  name       => 'System',
  diagram    => 'ig0=0b1101011 opc=0b0101 op2=0b11111 op3=0b000000 Rt=0b11111 op4=0b00000',
  docvars    => 'instr-class=system isa=A64 mnemonic=DRPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=DRPS',
  undefined  => '!Halted() || PSTATE.EL == EL0',
};

CLASS DSB_DSB_BO_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm:u=0bxxxx ig1=0b1 opc:u=0b00 Rt=0b11111',
  cnsts      => 'CRm!=0b0x00',
  docvars    => 'instr-class=system isa=A64 mnemonic=DSB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=DSB xid=DSB_BO_barriers',
};

CLASS EON_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx opc:u=0b10 ig0=0b01010 shift:u=0bxx N:u=0b1 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=EON reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=log_shift page=EON xid=no_s',
  undefined  => 'sf == 0 && imm6<5> == 1',
};

CLASS EOR_log_imm_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx opc:u=0b10 ig0=0b100100 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=EOR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log_imm page=EOR_log_imm xid=no_s',
  undefined  => 'sf == 0 && N != 0',
};

CLASS EOR_log_shift_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx opc:u=0b10 ig0=0b01010 shift:u=0bxx N:u=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=EOR reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=log_shift page=EOR_log_shift xid=general',
  undefined  => 'sf == 0 && imm6<5> == 1',
};

CLASS ERET_64E_branch_reg => {
  name       => 'System',
  diagram    => 'ig0=0b1101011 opc[3]=0b0 opc[2:0]=0b100 op2=0b11111 op3[5:2]=0b0000 A:u=0b0 M:u=0b0 Rn:u=0b11111 op4:u=0b00000',
  docvars    => 'instr-class=system isa=A64 mnemonic=ERET',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=ERET',
  exceptions => 'ERETTRAP',
  undefined  => 'PSTATE.EL == EL0',
};

CLASS ERETA_general => {
  name       => 'Integer',
  diagram    => 'ig0=0b1101011 opc[3]=0b0 opc[2:0]=0b100 op2=0b11111 op3[5:2]=0b0000 A:u=0b1 M:u=0bx Rn:u=0b11111 op4:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=ERETA xid=general',
  exceptions => 'ERETTRAP|PACFAIL|PACTRAP',
  undefined  => 'PSTATE.EL == EL0',
};

CLASS ESB_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0010 op2:u=0b000 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=ESB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=ESB',
};

CLASS EXTR_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op21=0b00 ig0=0b100111 N:u=0bx o0=0b0 Rm:u=0bxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=EXTR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=extract page=EXTR xid=general',
  undefined  => '(N != sf) || (sf == 0 && imms<5> == 1)',
};

CLASS GMI_64G_dp_2src => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b0 S=0b0 ig1=0b11010110 Xm:u=0bxxxxx opcode=0b000101 Xn:u=0bxxxxx Xd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=GMI',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=GMI',
};

CLASS HINT_HM_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0bxxxx op2:u=0bxxx Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=HINT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=HINT',
  exceptions => 'WFXTRAP',
};

CLASS HLT_EX_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b010 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL=0b00',
  docvars    => 'instr-class=system isa=A64 mnemonic=HLT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=HLT',
  undefined  => 'EDSCR.HDE == 0 || !HaltingAllowed()',
};

CLASS HVC_EX_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b000 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL=0b10',
  docvars    => 'instr-class=system isa=A64 mnemonic=HVC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=HVC',
  exceptions => 'HYPERVISORCALL',
};

CLASS IRG_64I_dp_2src => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b0 S=0b0 ig1=0b11010110 Xm:u=0bxxxxx opcode=0b000100 Xn:u=0bxxxxx Xd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=IRG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=IRG',
};

CLASS ISB_BI_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm:u=0bxxxx ig1=0b1 opc:u=0b10 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=ISB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=ISB',
};

CLASS LDADD_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADD xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDADDB_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADDB xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDADDH_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDADDH xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDAPR_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A=0b1 R=0b0 ig2=0b1 Rs:u=0bSSSSS o3=0b1 opc=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDAPR',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDAPR xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDAPRB_32L_memop => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A=0b1 R=0b0 ig2=0b1 Rs:u=0bSSSSS o3=0b1 opc=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDAPRB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDAPRB',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDAPRH_32L_memop => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A=0b1 R=0b0 ig2=0b1 Rs:u=0bSSSSS o3=0b1 opc=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDAPRH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDAPRH',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDAPUR_gen_base_plus_offset => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b1x ig0=0b011001 opc:u=0b01 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDAPUR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=LDAPUR_gen xid=base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDAPURB_32_ldapstl_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b00 ig0=0b011001 opc:u=0b01 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDAPURB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=LDAPURB',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDAPURH_32_ldapstl_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b01 ig0=0b011001 opc:u=0b01 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDAPURH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=LDAPURH',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDAPURSB_base_plus_offset => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b00 ig0=0b011001 opc:u=0b1x ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDAPURSB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=LDAPURSB xid=base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDAPURSH_base_plus_offset => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b01 ig0=0b011001 opc:u=0b1x ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDAPURSH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=LDAPURSH xid=base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDAPURSW_64_ldapstl_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b10 ig0=0b011001 opc:u=0b10 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=64 instr-class=general isa=A64 mnemonic=LDAPURSW offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=LDAPURSW',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDAR_base_register => {
  name       => 'No offset',
  diagram    => 'size:u=0b1x ig0=0b001000 o2=0b1 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=LDAR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDAR xid=base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDARB_LR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDARB-32-reg instr-class=general isa=A64 mnemonic=LDARB reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDARB',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDARH_LR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDARH-32-reg instr-class=general isa=A64 mnemonic=LDARH reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDARH',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDAXP_base_register => {
  name       => 'No offset',
  diagram    => 'ig0=0b1 sz:u=0bx ig1=0b001000 o2=0b0 L:u=0b1 o1=0b1 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=LDAXP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDAXP xid=base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDAXR_base_register => {
  name       => 'No offset',
  diagram    => 'size:u=0b1x ig0=0b001000 o2=0b0 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=LDAXR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDAXR xid=base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDAXRB_LR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b0 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDAXRB-32-reg instr-class=general isa=A64 mnemonic=LDAXRB reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDAXRB',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDAXRH_LR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b0 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDAXRH-32-reg instr-class=general isa=A64 mnemonic=LDAXRH reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDAXRH',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDCLR_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLR xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDCLRB_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLRB xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDCLRH_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDCLRH xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDEOR_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEOR xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDEORB_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEORB xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDEORH_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDEORH xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDG_64Loffset_ldsttags => {
  name       => 'Integer',
  diagram    => 'ig0=0b11011001 opc=0b01 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b00 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=LDG',
  exceptions => 'SPALIGNMENT',
};

CLASS LDGM_64bulk_ldsttags => {
  name       => 'Integer',
  diagram    => 'ig0=0b11011001 opc=0b11 ig1=0b1 imm9=0b000000000 op2=0b00 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDGM',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=LDGM',
  exceptions => 'SPALIGNMENT',
};

CLASS LDLAR_base_register => {
  name       => 'No offset',
  diagram    => 'size:u=0b1x ig0=0b001000 o2=0b1 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=LDLAR',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDLAR xid=base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDLARB_LR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDLARB-32-reg instr-class=general isa=A64 mnemonic=LDLARB reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDLARB',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDLARH_LR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b1 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDLARH-32-reg instr-class=general isa=A64 mnemonic=LDLARH reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDLARH',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDNP_gen_signed_scaled_offset => {
  name       => 'Signed offset',
  diagram    => 'opc:u=0bx0 ig0=0b101 V=0b0 ig1=0b000 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset instr-class=general isa=A64 mnemonic=LDNP offset-type=off7s_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstnapair_offs page=LDNP_gen xid=signed_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDP_gen_post_indexed => {
  name       => 'Post-index',
  diagram    => 'opc:u=0bx0 ig0=0b101 V=0b0 ig1=0b001 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=general isa=A64 mnemonic=LDP offset-type=off7s_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_post page=LDP_gen xid=post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDP_gen_pre_indexed => {
  name       => 'Pre-index',
  diagram    => 'opc:u=0bx0 ig0=0b101 V=0b0 ig1=0b011 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=general isa=A64 mnemonic=LDP offset-type=off7s_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_pre page=LDP_gen xid=pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDP_gen_signed_scaled_offset => {
  name       => 'Signed offset',
  diagram    => 'opc:u=0bx0 ig0=0b101 V=0b0 ig1=0b010 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset instr-class=general isa=A64 mnemonic=LDP offset-type=off7s_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_off page=LDP_gen xid=signed_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDPSW_64_ldstpair_post => {
  name       => 'Post-index',
  diagram    => 'opc:u=0b01 ig0=0b101 V=0b0 ig1=0b001 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-pair-64 atomic-ops=LDPSW-pair-64 instr-class=general isa=A64 mnemonic=LDPSW offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_post page=LDPSW',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDPSW_64_ldstpair_pre => {
  name       => 'Pre-index',
  diagram    => 'opc:u=0b01 ig0=0b101 V=0b0 ig1=0b011 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-pair-64 atomic-ops=LDPSW-pair-64 instr-class=general isa=A64 mnemonic=LDPSW offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_pre page=LDPSW',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDPSW_64_ldstpair_off => {
  name       => 'Signed offset',
  diagram    => 'opc:u=0b01 ig0=0b101 V=0b0 ig1=0b010 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-64 atomic-ops=LDPSW-pair-64 instr-class=general isa=A64 mnemonic=LDPSW offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_off page=LDPSW',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDR_imm_gen_post_indexed => {
  name       => 'Post-index',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=general isa=A64 mnemonic=LDR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_immpost page=LDR_imm_gen xid=post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDR_imm_gen_pre_indexed => {
  name       => 'Pre-index',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=general isa=A64 mnemonic=LDR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_immpre page=LDR_imm_gen xid=pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDR_imm_gen_unsigned_scaled_offset => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b01 opc:u=0b01 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset instr-class=general isa=A64 mnemonic=LDR offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_pos page=LDR_imm_gen xid=unsigned_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDR_lit_gen_literal => {
  name       => 'Literal',
  diagram    => 'opc:u=0b0x ig0=0b011 V=0b0 ig1=0b00 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=literal instr-class=general isa=A64 mnemonic=LDR offset-type=off19s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=loadlit page=LDR_lit_gen xid=literal',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDR_reg_gen_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDR offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_regoff page=LDR_reg_gen xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => 'option<1> == 0',
};

CLASS LDRA_base_plus_offset => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 M:u=0bx S:u=0bx ig2=0b1 imm9:u=0bxxxxxxxxx W:u=0bx ig3=0b1 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset isa=A64 offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_pac page=LDRA xid=base_plus_offset',
  exceptions => 'DATAABORT|PACFAIL|PACTRAP|SPALIGNMENT',
  undefined  => '!HavePACExt()',
};

CLASS LDRB_32_ldst_immpost => {
  name       => 'Post-index',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed datatype=32 instr-class=general isa=A64 mnemonic=LDRB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpost page=LDRB_imm',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDRB_32_ldst_immpre => {
  name       => 'Pre-index',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed datatype=32 instr-class=general isa=A64 mnemonic=LDRB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpre page=LDRB_imm',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDRB_32_ldst_pos => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b01 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset datatype=32 instr-class=general isa=A64 mnemonic=LDRB offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_pos page=LDRB_imm',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDRB_reg_32 => {
  name       => '32-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=LDRB offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_regoff page=LDRB_reg xid=32',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => 'option<1> == 0',
};

CLASS LDRH_32_ldst_immpost => {
  name       => 'Post-index',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed datatype=32 instr-class=general isa=A64 mnemonic=LDRH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpost page=LDRH_imm',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDRH_32_ldst_immpre => {
  name       => 'Pre-index',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed datatype=32 instr-class=general isa=A64 mnemonic=LDRH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpre page=LDRH_imm',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDRH_32_ldst_pos => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b01 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset datatype=32 instr-class=general isa=A64 mnemonic=LDRH offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_pos page=LDRH_imm',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDRH_32_ldst_regoff => {
  name       => '32-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=LDRH offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_regoff page=LDRH_reg',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => 'option<1> == 0',
};

CLASS LDRSB_imm_post_indexed => {
  name       => 'Post-index',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b1x ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=general isa=A64 mnemonic=LDRSB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_immpost page=LDRSB_imm xid=post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDRSB_imm_pre_indexed => {
  name       => 'Pre-index',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b1x ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=general isa=A64 mnemonic=LDRSB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_immpre page=LDRSB_imm xid=pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDRSB_imm_unsigned_scaled_offset => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b1x imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset instr-class=general isa=A64 mnemonic=LDRSB offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_pos page=LDRSB_imm xid=unsigned_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDRSB_reg_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b1x ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDRSB offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_regoff page=LDRSB_reg xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => 'option<1> == 0',
};

CLASS LDRSH_imm_post_indexed => {
  name       => 'Post-index',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b1x ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=general isa=A64 mnemonic=LDRSH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_immpost page=LDRSH_imm xid=post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDRSH_imm_pre_indexed => {
  name       => 'Pre-index',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b1x ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=general isa=A64 mnemonic=LDRSH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_immpre page=LDRSH_imm xid=pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDRSH_imm_unsigned_scaled_offset => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b1x imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset instr-class=general isa=A64 mnemonic=LDRSH offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_pos page=LDRSH_imm xid=unsigned_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDRSH_reg_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b1x ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDRSH offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_regoff page=LDRSH_reg xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => 'option<1> == 0',
};

CLASS LDRSW_64_ldst_immpost => {
  name       => 'Post-index',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed datatype=64 instr-class=general isa=A64 mnemonic=LDRSW offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_immpost page=LDRSW_imm',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDRSW_64_ldst_immpre => {
  name       => 'Pre-index',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed datatype=64 instr-class=general isa=A64 mnemonic=LDRSW offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_immpre page=LDRSW_imm',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDRSW_64_ldst_pos => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b10 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset datatype=64 instr-class=general isa=A64 mnemonic=LDRSW offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_pos page=LDRSW_imm',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDRSW_64_loadlit => {
  name       => 'Literal',
  diagram    => 'opc:u=0b10 ig0=0b011 V=0b0 ig1=0b00 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=literal address-form-reg-type=literal-64-reg atomic-ops=LDRSW-64-reg instr-class=general isa=A64 mnemonic=LDRSW offset-type=off19s reg-type=64-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=loadlit page=LDRSW_lit',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDRSW_64_ldst_regoff => {
  name       => '64-bit',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=LDRSW offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_regoff page=LDRSW_reg',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => 'option<1> == 0',
};

CLASS LDSET_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSET xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDSETB_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSETB xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDSETH_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSETH xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDSMAX_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAX xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDSMAXB_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAXB xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDSMAXH_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMAXH xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDSMIN_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMIN xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDSMINB_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMINB xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDSMINH_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDSMINH xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDTR_base_plus_offset => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDTR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unpriv page=LDTR xid=base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDTRB_32_ldst_unpriv => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDTRB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unpriv page=LDTRB',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDTRH_32_ldst_unpriv => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDTRH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unpriv page=LDTRH',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDTRSB_base_plus_offset => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b1x ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDTRSB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unpriv page=LDTRSB xid=base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDTRSH_base_plus_offset => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b1x ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDTRSH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unpriv page=LDTRSH xid=base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDTRSW_64_ldst_unpriv => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=64 instr-class=general isa=A64 mnemonic=LDTRSW offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_unpriv page=LDTRSW',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDUMAX_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAX xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDUMAXB_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAXB xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDUMAXH_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMAXH xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDUMIN_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMIN xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDUMINB_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMINB xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDUMINH_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=LDUMINH xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS LDUR_gen_base_plus_offset => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDUR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDUR_gen xid=base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDURB_32_ldst_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDURB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDURB',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDURH_32_ldst_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b01 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDURH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDURH',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDURSB_base_plus_offset => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b1x ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDURSB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDURSB xid=base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDURSH_base_plus_offset => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b1x ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDURSH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDURSH xid=base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDURSW_64_ldst_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b10 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=64 instr-class=general isa=A64 mnemonic=LDURSW offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDURSW',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDXP_base_register => {
  name       => 'No offset',
  diagram    => 'ig0=0b1 sz:u=0bx ig1=0b001000 o2=0b0 L:u=0b1 o1=0b1 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=LDXP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDXP xid=base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDXR_base_register => {
  name       => 'No offset',
  diagram    => 'size:u=0b1x ig0=0b001000 o2=0b0 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=LDXR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDXR xid=base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDXRB_LR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b0 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDXRB-32-reg instr-class=general isa=A64 mnemonic=LDXRB reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDXRB',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LDXRH_LR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b0 L:u=0b1 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDXRH-32-reg instr-class=general isa=A64 mnemonic=LDXRH reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=LDXRH',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS LSLV_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=LSLV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=LSLV xid=general',
};

CLASS LSRV_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=LSRV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=LSRV xid=general',
};

CLASS MADD_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op54=0b00 ig0=0b11011 op31=0b000 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=MADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_3src page=MADD xid=general',
};

CLASS MOVK_imm18_packed => {
  name       => 'Immediate packed into 16-bit value and 2-bit shift',
  diagram    => 'sf:u=0bx opc:u=0b11 ig0=0b100101 hw:u=0bxx imm16:u=0bxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  docvars    => 'immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVK',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movewide page=MOVK xid=imm18_packed',
  undefined  => 'sf == 0 && hw<1> == 1',
};

CLASS MOVN_imm18_packed => {
  name       => 'Immediate packed into 16-bit value and 2-bit shift',
  diagram    => 'sf:u=0bx opc:u=0b00 ig0=0b100101 hw:u=0bxx imm16:u=0bxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  docvars    => 'immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movewide page=MOVN xid=imm18_packed',
  undefined  => 'sf == 0 && hw<1> == 1',
};

CLASS MOVZ_imm18_packed => {
  name       => 'Immediate packed into 16-bit value and 2-bit shift',
  diagram    => 'sf:u=0bx opc:u=0b10 ig0=0b100101 hw:u=0bxx imm16:u=0bxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  docvars    => 'immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movewide page=MOVZ xid=imm18_packed',
  undefined  => 'sf == 0 && hw<1> == 1',
};

CLASS MRS_RS_systemmove => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b1 op0=0b1 o0:u=0bx op1:u=0bxxx CRn:u=0bxxxx CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=MRS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=systemmove page=MRS',
};

CLASS MSR_SI_pstate => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1:u=0bxxx CRn=0b0100 CRm:u=0bxxxx op2:u=0bxxx Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=MSR msr-sysreg-target=whole-register',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=control iclass=pstate page=MSR_imm',
};

CLASS MSR_SR_systemmove => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b1 o0:u=0bx op1:u=0bxxx CRn:u=0bxxxx CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=MSR msr-sysreg-target=register-field',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=systemmove page=MSR_reg',
};

CLASS MSUB_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op54=0b00 ig0=0b11011 op31=0b000 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=MSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_3src page=MSUB xid=general',
};

CLASS NOP_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0000 op2:u=0b000 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=NOP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=NOP',
};

CLASS ORN_log_shift_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx opc:u=0b01 ig0=0b01010 shift:u=0bxx N:u=0b1 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=ORN reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=log_shift page=ORN_log_shift xid=no_s',
  undefined  => 'sf == 0 && imm6<5> == 1',
};

CLASS ORR_log_imm_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx opc:u=0b01 ig0=0b100100 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=ORR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log_imm page=ORR_log_imm xid=no_s',
  undefined  => 'sf == 0 && N != 0',
};

CLASS ORR_log_shift_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx opc:u=0b01 ig0=0b01010 shift:u=0bxx N:u=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=ORR reguse=shifted-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=log_shift page=ORR_log_shift xid=no_s',
  undefined  => 'sf == 0 && imm6<5> == 1',
};

CLASS PACDA_general => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0bx opcode[2:0]=0b010 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=PACDA xid=general',
  exceptions => 'PACTRAP',
};

CLASS PACDB_general => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0bx opcode[2:0]=0b011 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=PACDB xid=general',
  exceptions => 'PACTRAP',
};

CLASS PACGA_64P_dp_2src => {
  name       => 'Integer',
  diagram    => 'sf=0b1 op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b001100 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=PACGA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=PACGA',
  exceptions => 'PACTRAP',
};

CLASS PACIA_general => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0bx opcode[2:0]=0b000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=PACIA xid=general',
  exceptions => 'PACTRAP',
};

CLASS PACIA_system => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b00x1 op2:u=0b00x Rt=0b11111',
  docvars    => 'instr-class=system isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=PACIA xid=system',
  exceptions => 'PACTRAP',
};

CLASS PACIB_general => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b0 Z:u=0bx opcode[2:0]=0b001 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=PACIB xid=general',
  exceptions => 'PACTRAP',
};

CLASS PACIB_system => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b00x1 op2:u=0b01x Rt=0b11111',
  docvars    => 'instr-class=system isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=PACIB xid=system',
  exceptions => 'PACTRAP',
};

CLASS PRFM_P_ldst_pos => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b10 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset instr-class=general isa=A64 mnemonic=PRFM offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_pos page=PRFM_imm',
};

CLASS PRFM_P_loadlit => {
  name       => 'Literal',
  diagram    => 'opc:u=0b11 ig0=0b011 V=0b0 ig1=0b00 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=literal instr-class=general isa=A64 mnemonic=PRFM offset-type=off19s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=loadlit page=PRFM_lit',
};

CLASS PRFM_P_ldst_regoff => {
  name       => 'Integer',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=PRFM offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_regoff page=PRFM_reg',
  undefined  => 'option<1> == 0',
};

CLASS PRFUM_P_ldst_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b11 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b10 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=PRFUM offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unscaled page=PRFUM',
};

CLASS PSB_HC_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0010 op2:u=0b001 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=PSB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=PSB',
};

CLASS PSSBB_only_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm:u=0b0100 ig1=0b1 opc:u=0b00 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=PSSBB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=PSSBB',
};

CLASS RBIT_int_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:2]=0b0000 opcode[1:0]=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=RBIT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=RBIT_int xid=general',
};

CLASS RET_64R_branch_reg => {
  name       => 'Integer',
  diagram    => 'ig0=0b1101011 Z:u=0b0 opc[2:1]=0b0 op:u=0b10 op2=0b11111 op3[5:2]=0b0000 A:u=0b0 M:u=0b0 Rn:u=0bxxxxx Rm:u=0b00000',
  docvars    => 'instr-class=general isa=A64 mnemonic=RET',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=RET',
};

CLASS RETA_general => {
  name       => 'Integer',
  diagram    => 'ig0=0b1101011 Z:u=0b0 opc[2:1]=0b0 op:u=0b10 op2=0b11111 op3[5:2]=0b0000 A:u=0b1 M:u=0bx Rn:u=0b11111 Rm:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=branch_reg page=RETA xid=general',
  exceptions => 'PACFAIL|PACTRAP',
};

CLASS REV_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:2]=0b0000 opc:u=0b1x Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=REV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=REV xid=general',
};

CLASS REV16_int_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:2]=0b0000 opc:u=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=REV16',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=REV16_int xid=general',
};

CLASS REV32_64_dp_1src => {
  name       => '64-bit',
  diagram    => 'sf:u=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:2]=0b0000 opc:u=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=REV32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_1src page=REV32_int',
};

CLASS RMIF_only_rmif => {
  name       => 'Integer',
  diagram    => 'sf:u=0b1 op=0b0 S=0b1 ig0=0b11010000 imm6:u=0bxxxxxx ig1=0b00001 Rn:u=0bxxxxx o2=0b0 mask:u=0bxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=RMIF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=rmif page=RMIF',
  undefined  => '!HaveFlagManipulateExt()',
};

CLASS RORV_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=RORV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=RORV xid=general',
};

CLASS SB_only_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm:u=0bZZZZ ig1=0b1 opc:u=0b11 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=SB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=SB',
  undefined  => '!HaveSBExt()',
};

CLASS SBC_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b1 S:u=0b0 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=SBC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_carry page=SBC xid=no_s',
};

CLASS SBCS_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b1 S:u=0b1 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=SBCS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_carry page=SBCS xid=s',
};

CLASS SBFM_signed_fill => {
  name       => 'With sign replication to left and zeros to right',
  diagram    => 'sf:u=0bx opc:u=0b00 ig0=0b100110 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'bitfield-fill=signed-fill instr-class=general isa=A64 mnemonic=SBFM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=SBFM xid=signed_fill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};

CLASS SDIV_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b00001 o1:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=SDIV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=SDIV xid=general',
};

CLASS SETF_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0b0 op=0b0 S=0b1 ig0=0b11010000 opcode2=0b000000 sz:u=0bx ig1=0b0010 Rn:u=0bxxxxx o3=0b0 mask=0b1101',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=setf page=SETF xid=general',
  undefined  => '!HaveFlagManipulateExt()',
};

CLASS SEV_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0000 op2:u=0b100 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=SEV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=SEV',
};

CLASS SEVL_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0000 op2:u=0b101 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=SEVL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=SEVL',
};

CLASS SMADDL_SMADDL_64WA_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b0 ig1=0b01 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=SMADDL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=SMADDL xid=SMADDL_64WA_dp_3src',
};

CLASS SMC_EX_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b000 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL=0b11',
  docvars    => 'instr-class=system isa=A64 mnemonic=SMC',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v0-A alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=SMC',
  exceptions => 'MONITORCALL',
};

CLASS SMSUBL_SMSUBL_64WA_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b0 ig1=0b01 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=SMSUBL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=SMSUBL xid=SMSUBL_64WA_dp_3src',
};

CLASS SMULH_64_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b0 ig1=0b10 Rm:u=0bxxxxx o0=0b0 Ra:u=0bSSSSS Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=SMULH',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=SMULH',
};

CLASS SSBB_only_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm:u=0b0000 ig1=0b1 opc:u=0b00 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=SSBB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=SSBB',
};

CLASS ST2G_64Spost_ldsttags => {
  name       => 'Post-index',
  diagram    => 'ig0=0b11011001 opc=0b10 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b01 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=general isa=A64 mnemonic=ST2G',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=ST2G',
  exceptions => 'SPALIGNMENT',
};

CLASS ST2G_64Spre_ldsttags => {
  name       => 'Pre-index',
  diagram    => 'ig0=0b11011001 opc=0b10 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b11 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=general isa=A64 mnemonic=ST2G',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=ST2G',
  exceptions => 'SPALIGNMENT',
};

CLASS ST2G_64Soffset_ldsttags => {
  name       => 'Signed offset',
  diagram    => 'ig0=0b11011001 opc=0b10 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b10 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset instr-class=general isa=A64 mnemonic=ST2G',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=ST2G',
  exceptions => 'SPALIGNMENT',
};

CLASS STG_64Spost_ldsttags => {
  name       => 'Post-index',
  diagram    => 'ig0=0b11011001 opc=0b00 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b01 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=general isa=A64 mnemonic=STG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STG',
  exceptions => 'SPALIGNMENT',
};

CLASS STG_64Spre_ldsttags => {
  name       => 'Pre-index',
  diagram    => 'ig0=0b11011001 opc=0b00 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b11 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=general isa=A64 mnemonic=STG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STG',
  exceptions => 'SPALIGNMENT',
};

CLASS STG_64Soffset_ldsttags => {
  name       => 'Signed offset',
  diagram    => 'ig0=0b11011001 opc=0b00 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b10 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset instr-class=general isa=A64 mnemonic=STG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STG',
  exceptions => 'SPALIGNMENT',
};

CLASS STGM_64bulk_ldsttags => {
  name       => 'Integer',
  diagram    => 'ig0=0b11011001 opc=0b10 ig1=0b1 imm9=0b000000000 op2=0b00 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=STGM',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STGM',
  exceptions => 'SPALIGNMENT',
};

CLASS STGP_64_ldstpair_post => {
  name       => 'Post-index',
  diagram    => 'opc=0b01 ig0=0b101 V=0b0 ig1=0b001 L=0b0 simm7:u=0bxxxxxxx Xt2:u=0bxxxxx Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed address-form-reg-type=post-indexed-pair-64 atomic-ops=STGP-pair-64 instr-class=general isa=A64 mnemonic=STGP offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_post page=STGP',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STGP_64_ldstpair_pre => {
  name       => 'Pre-index',
  diagram    => 'opc=0b01 ig0=0b101 V=0b0 ig1=0b011 L=0b0 simm7:u=0bxxxxxxx Xt2:u=0bxxxxx Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed address-form-reg-type=pre-indexed-pair-64 atomic-ops=STGP-pair-64 instr-class=general isa=A64 mnemonic=STGP offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_pre page=STGP',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STGP_64_ldstpair_off => {
  name       => 'Signed offset',
  diagram    => 'opc=0b01 ig0=0b101 V=0b0 ig1=0b010 L=0b0 simm7:u=0bxxxxxxx Xt2:u=0bxxxxx Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset address-form-reg-type=signed-scaled-offset-pair-64 atomic-ops=STGP-pair-64 instr-class=general isa=A64 mnemonic=STGP offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_off page=STGP',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STLLR_base_register => {
  name       => 'No offset',
  diagram    => 'size:u=0b1x ig0=0b001000 o2=0b1 L:u=0b0 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=STLLR',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLLR xid=base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STLLRB_SL32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLLRB-32-reg instr-class=general isa=A64 mnemonic=STLLRB reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLLRB',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STLLRH_SL32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b0 Rs:u=0bSSSSS o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLLRH-32-reg instr-class=general isa=A64 mnemonic=STLLRH reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLLRH',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STLR_base_register => {
  name       => 'No offset',
  diagram    => 'size:u=0b1x ig0=0b001000 o2=0b1 L:u=0b0 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=STLR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLR xid=base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STLRB_SL32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLRB-32-reg instr-class=general isa=A64 mnemonic=STLRB reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLRB',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STLRH_SL32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b1 L:u=0b0 o1=0b0 Rs:u=0bSSSSS o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLRH-32-reg instr-class=general isa=A64 mnemonic=STLRH reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLRH',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STLUR_gen_base_plus_offset => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b1x ig0=0b011001 opc:u=0b00 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=STLUR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=STLUR_gen xid=base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STLURB_32_ldapstl_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b00 ig0=0b011001 opc:u=0b00 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STLURB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=STLURB',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STLURH_32_ldapstl_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b01 ig0=0b011001 opc:u=0b00 ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STLURH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldapstl_unscaled page=STLURH',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STLXP_base_register => {
  name       => 'No offset',
  diagram    => 'ig0=0b1 sz:u=0bx ig1=0b001000 o2=0b0 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b1 Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=STLXP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLXP xid=base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STLXR_base_register => {
  name       => 'No offset',
  diagram    => 'size:u=0b1x ig0=0b001000 o2=0b0 L:u=0b0 o1=0b0 Rs:u=0bxxxxx o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=STLXR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLXR xid=base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STLXRB_SR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b0 L:u=0b0 o1=0b0 Rs:u=0bxxxxx o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLXRB-32-reg instr-class=general isa=A64 mnemonic=STLXRB reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLXRB',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STLXRH_SR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b0 L:u=0b0 o1=0b0 Rs:u=0bxxxxx o0:u=0b1 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLXRH-32-reg instr-class=general isa=A64 mnemonic=STLXRH reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STLXRH',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STNP_gen_signed_scaled_offset => {
  name       => 'Signed offset',
  diagram    => 'opc:u=0bx0 ig0=0b101 V=0b0 ig1=0b000 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset instr-class=general isa=A64 mnemonic=STNP offset-type=off7s_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstnapair_offs page=STNP_gen xid=signed_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STP_gen_post_indexed => {
  name       => 'Post-index',
  diagram    => 'opc:u=0bx0 ig0=0b101 V=0b0 ig1=0b001 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=general isa=A64 mnemonic=STP offset-type=off7s_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_post page=STP_gen xid=post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STP_gen_pre_indexed => {
  name       => 'Pre-index',
  diagram    => 'opc:u=0bx0 ig0=0b101 V=0b0 ig1=0b011 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=general isa=A64 mnemonic=STP offset-type=off7s_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_pre page=STP_gen xid=pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STP_gen_signed_scaled_offset => {
  name       => 'Signed offset',
  diagram    => 'opc:u=0bx0 ig0=0b101 V=0b0 ig1=0b010 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset instr-class=general isa=A64 mnemonic=STP offset-type=off7s_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstpair_off page=STP_gen xid=signed_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STR_imm_gen_post_indexed => {
  name       => 'Post-index',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=general isa=A64 mnemonic=STR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_immpost page=STR_imm_gen xid=post_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STR_imm_gen_pre_indexed => {
  name       => 'Pre-index',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=general isa=A64 mnemonic=STR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_immpre page=STR_imm_gen xid=pre_indexed',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STR_imm_gen_unsigned_scaled_offset => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b01 opc:u=0b00 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset instr-class=general isa=A64 mnemonic=STR offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_pos page=STR_imm_gen xid=unsigned_scaled_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STR_reg_gen_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=STR offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_regoff page=STR_reg_gen xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => 'option<1> == 0',
};

CLASS STRB_32_ldst_immpost => {
  name       => 'Post-index',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed datatype=32 instr-class=general isa=A64 mnemonic=STRB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpost page=STRB_imm',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STRB_32_ldst_immpre => {
  name       => 'Pre-index',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed datatype=32 instr-class=general isa=A64 mnemonic=STRB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpre page=STRB_imm',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STRB_32_ldst_pos => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b00 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset datatype=32 instr-class=general isa=A64 mnemonic=STRB offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_pos page=STRB_imm',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STRB_reg_32 => {
  name       => '32-bit',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=STRB offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_regoff page=STRB_reg xid=32',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => 'option<1> == 0',
};

CLASS STRH_32_ldst_immpost => {
  name       => 'Post-index',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed datatype=32 instr-class=general isa=A64 mnemonic=STRH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpost page=STRH_imm',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STRH_32_ldst_immpre => {
  name       => 'Pre-index',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed datatype=32 instr-class=general isa=A64 mnemonic=STRH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_immpre page=STRH_imm',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STRH_32_ldst_pos => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b01 opc:u=0b00 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset datatype=32 instr-class=general isa=A64 mnemonic=STRH offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_pos page=STRH_imm',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STRH_32_ldst_regoff => {
  name       => '32-bit',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=STRH offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_regoff page=STRH_reg',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => 'option<1> == 0',
};

CLASS STTR_base_plus_offset => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=STTR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unpriv page=STTR xid=base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STTRB_32_ldst_unpriv => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STTRB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unpriv page=STTRB',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STTRH_32_ldst_unpriv => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STTRH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unpriv page=STTRH',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STUR_gen_base_plus_offset => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=STUR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unscaled page=STUR_gen xid=base_plus_offset',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STURB_32_ldst_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STURB offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unscaled page=STURB',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STURH_32_ldst_unscaled => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 opc:u=0b00 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STURH offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=32',
  tags       => 'group=ldst iclass=ldst_unscaled page=STURH',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STXP_base_register => {
  name       => 'No offset',
  diagram    => 'ig0=0b1 sz:u=0bx ig1=0b001000 o2=0b0 L:u=0b0 o1=0b1 Rs:u=0bxxxxx o0:u=0b0 Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=STXP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STXP xid=base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STXR_base_register => {
  name       => 'No offset',
  diagram    => 'size:u=0b1x ig0=0b001000 o2=0b0 L:u=0b0 o1=0b0 Rs:u=0bxxxxx o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=STXR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STXR xid=base_register',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STXRB_SR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b00 ig0=0b001000 o2=0b0 L:u=0b0 o1=0b0 Rs:u=0bxxxxx o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STXRB-32-reg instr-class=general isa=A64 mnemonic=STXRB reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STXRB',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STXRH_SR32_ldstexcl => {
  name       => 'No offset',
  diagram    => 'size:u=0b01 ig0=0b001000 o2=0b0 L:u=0b0 o1=0b0 Rs:u=0bxxxxx o0:u=0b0 Rt2:u=0bSSSSS Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STXRH-32-reg instr-class=general isa=A64 mnemonic=STXRH reg-type=32-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldstexcl page=STXRH',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STZ2G_64Spost_ldsttags => {
  name       => 'Post-index',
  diagram    => 'ig0=0b11011001 opc=0b11 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b01 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=general isa=A64 mnemonic=STZ2G',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STZ2G',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STZ2G_64Spre_ldsttags => {
  name       => 'Pre-index',
  diagram    => 'ig0=0b11011001 opc=0b11 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b11 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=general isa=A64 mnemonic=STZ2G',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STZ2G',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STZ2G_64Soffset_ldsttags => {
  name       => 'Signed offset',
  diagram    => 'ig0=0b11011001 opc=0b11 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b10 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset instr-class=general isa=A64 mnemonic=STZ2G',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STZ2G',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STZG_64Spost_ldsttags => {
  name       => 'Post-index',
  diagram    => 'ig0=0b11011001 opc=0b01 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b01 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=general isa=A64 mnemonic=STZG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STZG',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STZG_64Spre_ldsttags => {
  name       => 'Pre-index',
  diagram    => 'ig0=0b11011001 opc=0b01 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b11 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=general isa=A64 mnemonic=STZG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STZG',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STZG_64Soffset_ldsttags => {
  name       => 'Signed offset',
  diagram    => 'ig0=0b11011001 opc=0b01 ig1=0b1 imm9:u=0bxxxxxxxxx op2=0b10 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset instr-class=general isa=A64 mnemonic=STZG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STZG',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS STZGM_64bulk_ldsttags => {
  name       => 'Integer',
  diagram    => 'ig0=0b11011001 opc=0b00 ig1=0b1 imm9=0b000000000 op2=0b00 Xn:u=0bxxxxx Xt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=STZGM',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldsttags page=STZGM',
  exceptions => 'DATAABORT|SPALIGNMENT',
};

CLASS SUB_addsub_ext_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b1 S:u=0b0 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=SUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_ext page=SUB_addsub_ext xid=no_s',
  undefined  => 'UInt(imm3) > 4',
};

CLASS SUB_addsub_imm_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b1 S:u=0b0 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s immediate-type=imm12u instr-class=general isa=A64 mnemonic=SUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=addsub_imm page=SUB_addsub_imm xid=no_s',
};

CLASS SUB_addsub_shift_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b1 S:u=0b0 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=SUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_shift page=SUB_addsub_shift xid=no_s',
  undefined  => '(shift == 3) || (sf == 0 && imm6<5> == 1)',
};

CLASS SUBG_64_addsub_immtags => {
  name       => 'Integer',
  diagram    => 'sf=0b1 op=0b1 S=0b0 ig0=0b100011 o2=0b0 uimm6:u=0bxxxxxx op3:u=0bZZ uimm4:u=0bxxxx Xn:u=0bxxxxx Xd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=SUBG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=addsub_immtags page=SUBG',
};

CLASS SUBP_64S_dp_2src => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b0 S=0b0 ig1=0b11010110 Xm:u=0bxxxxx opcode=0b000000 Xn:u=0bxxxxx Xd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=SUBP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=SUBP',
};

CLASS SUBPS_SUBPS_64S_dp_2src => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b0 S=0b1 ig1=0b11010110 Xm:u=0bxxxxx opcode=0b000000 Xn:u=0bxxxxx Xd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=SUBPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=SUBPS xid=SUBPS_64S_dp_2src',
};

CLASS SUBS_addsub_ext_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b1 S:u=0b1 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_ext page=SUBS_addsub_ext xid=s',
  undefined  => 'UInt(imm3) > 4',
};

CLASS SUBS_addsub_imm_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b1 S:u=0b1 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=S immediate-type=imm12u instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=addsub_imm page=SUBS_addsub_imm xid=s',
};

CLASS SUBS_addsub_shift_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b1 S:u=0b1 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_shift page=SUBS_addsub_shift xid=s',
  undefined  => '(shift == 3) || (sf == 0 && imm6<5> == 1)',
};

CLASS SVC_EX_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b000 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL=0b01',
  docvars    => 'instr-class=system isa=A64 mnemonic=SVC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=SVC',
  exceptions => 'SUPERVISORCALL',
};

CLASS SWP_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWP xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS SWPB_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWPB xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS SWPH_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b1 opc=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=SWPH xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS SYS_SYS_CR_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b01 op1:u=0bxxx CRn:u=0bxxxx CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=SYS xid=SYS_CR_systeminstrs',
};

CLASS SYSL_RC_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b1 op0=0b01 op1:u=0bxxx CRn:u=0bxxxx CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=SYSL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=SYSL',
};

CLASS TBNZ_only_testbranch => {
  name       => '14-bit signed PC-relative branch offset',
  diagram    => 'b5:u=0bx ig0=0b011011 op:u=0b1 b40:u=0bxxxxx imm14:u=0bxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  docvars    => 'branch-offset=br14 instr-class=general isa=A64 mnemonic=TBNZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=testbranch page=TBNZ',
};

CLASS TBZ_only_testbranch => {
  name       => '14-bit signed PC-relative branch offset',
  diagram    => 'b5:u=0bx ig0=0b011011 op:u=0b0 b40:u=0bxxxxx imm14:u=0bxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  docvars    => 'branch-offset=br14 instr-class=general isa=A64 mnemonic=TBZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=testbranch page=TBZ',
};

CLASS TCANCEL_EX_exception => {
  name       => 'System',
  diagram    => 'ig0=0b11010100 opc=0b011 imm16:u=0bxxxxxxxxxxxxxxxx op2=0b000 LL=0b00',
  docvars    => 'instr-class=system isa=A64 mnemonic=TCANCEL',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=TME alphaindex=BASIC',
  tags       => 'group=control iclass=exception page=TCANCEL',
  undefined  => '!HaveTME()',
};

CLASS TCOMMIT_only_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm=0b0000 op2=0b011 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=TCOMMIT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=TME alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=TCOMMIT',
  undefined  => '!HaveTME()',
};

CLASS TSB_HC_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0010 op2:u=0b010 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=TSB',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=TSB',
};

CLASS TSTART_BR_systemresult => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b1 op0=0b00 op1=0b011 CRn=0b0011 CRm=0b0000 op2=0b011 Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=TSTART',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=TME alphaindex=BASIC',
  tags       => 'group=control iclass=systemresult page=TSTART',
  exceptions => 'TSTARTACCESSTRAP',
  undefined  => '!HaveTME()',
};

CLASS TTEST_BR_systemresult => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b1 op0=0b00 op1=0b011 CRn=0b0011 CRm=0b0001 op2=0b011 Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=TTEST',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=TME alphaindex=BASIC',
  tags       => 'group=control iclass=systemresult page=TTEST',
  undefined  => '!HaveTME()',
};

CLASS UBFM_zero_fill => {
  name       => 'With zeros to left and right',
  diagram    => 'sf:u=0bx opc:u=0b10 ig0=0b100110 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'bitfield-fill=zero-fill instr-class=general isa=A64 mnemonic=UBFM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=UBFM xid=zero_fill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};

CLASS UDF_only_perm_undef => {
  name       => 'Integer',
  diagram    => 'ig0=0b0000000000000000 imm16:u=0bxxxxxxxxxxxxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=UDF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=reserved iclass=perm_undef page=UDF_perm_undef',
};

CLASS UDIV_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b00001 o1:u=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=UDIV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=UDIV xid=general',
};

CLASS UMADDL_UMADDL_64WA_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b1 ig1=0b01 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=UMADDL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=UMADDL xid=UMADDL_64WA_dp_3src',
};

CLASS UMSUBL_UMSUBL_64WA_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b1 ig1=0b01 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=UMSUBL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=UMSUBL xid=UMSUBL_64WA_dp_3src',
};

CLASS UMULH_64_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b1 ig1=0b10 Rm:u=0bxxxxx o0=0b0 Ra:u=0bSSSSS Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=UMULH',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=UMULH',
};

CLASS WFE_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0000 op2:u=0b010 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=WFE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=WFE',
  exceptions => 'WFXTRAP',
};

CLASS WFI_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0000 op2:u=0b011 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=WFI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=WFI',
  exceptions => 'WFXTRAP',
};

CLASS XAFLAG_M_pstate => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b000 CRn=0b0100 CRm:u=0bZZZZ op2=0b001 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=XAFLAG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=control iclass=pstate page=XAFLAG',
  undefined  => '!HaveFlagFormatExt()',
};

CLASS XPAC_general => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00001 opcode[5]=0b0 opcode[4]=0b1 opcode[3:1]=0b000 D:u=0bx Rn:u=0b11111 Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=XPAC xid=general',
};

CLASS XPACLRI_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm=0b0000 op2=0b111 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=XPACLRI',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=XPAC',
};

CLASS YIELD_HI_hints => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0010 CRm:u=0b0000 op2:u=0b001 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=YIELD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=hints page=YIELD',
};

CLASS ASR_ASRV_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=ASRV',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=ASR_ASRV xid=general',
};

CLASS ASR_SBFM_signed_fill => {
  name       => 'With sign replication to left and zeros to right',
  diagram    => 'sf:u=0bx opc:u=0b00 ig0=0b100110 N:u=0bx immr:u=0bxxxxxx imms:u=0bx11111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'bitfield-fill=signed-fill instr-class=general isa=A64 mnemonic=SBFM',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=ASR_SBFM xid=signed_fill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};

CLASS AT_SYS_SYS_CR_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b01 op1:u=0bxxx CRn:u=0b0111 CRm:u=0b100x op2:u=0bxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=AT_SYS xid=SYS_CR_systeminstrs',
};

CLASS BFC_BFM_nofill => {
  name       => 'Leaving other bits unchanged',
  diagram    => 'sf:u=0bx opc:u=0b01 ig0=0b100110 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0b11111 Rd:u=0bxxxxx',
  docvars    => 'bitfield-fill=nofill instr-class=general isa=A64 mnemonic=BFM',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=BFC_BFM xid=nofill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};

CLASS BFI_BFM_nofill => {
  name       => 'Leaving other bits unchanged',
  diagram    => 'sf:u=0bx opc:u=0b01 ig0=0b100110 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'Rn!=0b11111',
  docvars    => 'bitfield-fill=nofill instr-class=general isa=A64 mnemonic=BFM',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=BFI_BFM xid=nofill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};

CLASS BFXIL_BFM_nofill => {
  name       => 'Leaving other bits unchanged',
  diagram    => 'sf:u=0bx opc:u=0b01 ig0=0b100110 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'bitfield-fill=nofill instr-class=general isa=A64 mnemonic=BFM',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=BFXIL_BFM xid=nofill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};

CLASS CFP_SYS_SYS_CR_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b01 op1:u=0b011 CRn:u=0b0111 CRm:u=0b0011 op2:u=0b100 Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=CFP_SYS xid=SYS_CR_systeminstrs',
};

CLASS CINC_CSINC_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op:u=0b0 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111 cond!=0b111x Rn!=0b11111',
  docvars    => 'instr-class=general isa=A64 mnemonic=CSINC',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=condsel page=CINC_CSINC xid=general',
};

CLASS CINV_CSINV_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op:u=0b1 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'Rm!=0b11111 cond!=0b111x Rn!=0b11111',
  docvars    => 'instr-class=general isa=A64 mnemonic=CSINV',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=condsel page=CINV_CSINV xid=general',
};

CLASS CMN_ADDS_addsub_ext_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b0 S:u=0b1 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0b11111',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_ext page=CMN_ADDS_addsub_ext xid=s',
  undefined  => 'UInt(imm3) > 4',
};

CLASS CMN_ADDS_addsub_imm_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b0 S:u=0b1 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  docvars    => 'cond-setting=S immediate-type=imm12u instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=addsub_imm page=CMN_ADDS_addsub_imm xid=s',
};

CLASS CMN_ADDS_addsub_shift_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b0 S:u=0b1 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_shift page=CMN_ADDS_addsub_shift xid=s',
  undefined  => '(shift == 3) || (sf == 0 && imm6<5> == 1)',
};

CLASS CMP_SUBS_addsub_ext_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b1 S:u=0b1 ig0=0b01011 opt=0b00 ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0b11111',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_ext page=CMP_SUBS_addsub_ext xid=s',
  undefined  => 'UInt(imm3) > 4',
};

CLASS CMP_SUBS_addsub_imm_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b1 S:u=0b1 ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  docvars    => 'cond-setting=S immediate-type=imm12u instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=addsub_imm page=CMP_SUBS_addsub_imm xid=s',
};

CLASS CMP_SUBS_addsub_shift_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b1 S:u=0b1 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_shift page=CMP_SUBS_addsub_shift xid=s',
  undefined  => '(shift == 3) || (sf == 0 && imm6<5> == 1)',
};

CLASS CMPP_SUBPS_SUBPS_64S_dp_2src => {
  name       => 'Integer',
  diagram    => 'sf=0b1 ig0=0b0 S=0b1 ig1=0b11010110 Xm:u=0bxxxxx opcode=0b000000 Xn:u=0bxxxxx Xd:u=0b11111',
  docvars    => 'instr-class=general isa=A64 mnemonic=SUBPS',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=CMPP_SUBPS xid=SUBPS_64S_dp_2src',
};

CLASS CNEG_CSNEG_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op:u=0b1 S=0b0 ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'cond!=0b111x',
  docvars    => 'instr-class=general isa=A64 mnemonic=CSNEG',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=condsel page=CNEG_CSNEG xid=general',
};

CLASS CPP_SYS_SYS_CR_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b01 op1:u=0b011 CRn:u=0b0111 CRm:u=0b0011 op2:u=0b111 Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=CPP_SYS xid=SYS_CR_systeminstrs',
};

CLASS CSET_CSINC_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op:u=0b0 S=0b0 ig0=0b11010100 Rm:u=0b11111 cond:u=0bxxxx ig1=0b0 o2:u=0b1 Rn:u=0b11111 Rd:u=0bxxxxx',
  cnsts      => 'cond!=0b111x',
  docvars    => 'instr-class=general isa=A64 mnemonic=CSINC',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=condsel page=CSET_CSINC xid=general',
};

CLASS CSETM_CSINV_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op:u=0b1 S=0b0 ig0=0b11010100 Rm:u=0b11111 cond:u=0bxxxx ig1=0b0 o2:u=0b0 Rn:u=0b11111 Rd:u=0bxxxxx',
  cnsts      => 'cond!=0b111x',
  docvars    => 'instr-class=general isa=A64 mnemonic=CSINV',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=condsel page=CSETM_CSINV xid=general',
};

CLASS DC_SYS_SYS_CR_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b01 op1:u=0bxxx CRn:u=0b0111 CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=DC_SYS xid=SYS_CR_systeminstrs',
};

CLASS DFB_DSB_DSB_BO_barriers => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L=0b0 op0=0b00 op1=0b011 CRn=0b0011 CRm:u=0b1100 ig1=0b1 opc:u=0b00 Rt=0b11111',
  docvars    => 'instr-class=system isa=A64 mnemonic=DSB',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=vv alphaindex=BASIC',
  tags       => 'group=control iclass=barriers page=DFB_DSB xid=DSB_BO_barriers',
};

CLASS DVP_SYS_SYS_CR_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b01 op1:u=0b011 CRn:u=0b0111 CRm:u=0b0011 op2:u=0b101 Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=DVP_SYS xid=SYS_CR_systeminstrs',
};

CLASS IC_SYS_SYS_CR_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b01 op1:u=0bxxx CRn:u=0b0111 CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=IC_SYS xid=SYS_CR_systeminstrs',
};

CLASS LSL_LSLV_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=LSLV',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=LSL_LSLV xid=general',
};

CLASS LSL_UBFM_zero_fill => {
  name       => 'With zeros to left and right',
  diagram    => 'sf:u=0bx opc:u=0b10 ig0=0b100110 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'imms!=0bx11111',
  docvars    => 'bitfield-fill=zero-fill instr-class=general isa=A64 mnemonic=UBFM',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=LSL_UBFM xid=zero_fill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};

CLASS LSR_LSRV_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=LSRV',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=LSR_LSRV xid=general',
};

CLASS LSR_UBFM_zero_fill => {
  name       => 'With zeros to left and right',
  diagram    => 'sf:u=0bx opc:u=0b10 ig0=0b100110 N:u=0bx immr:u=0bxxxxxx imms:u=0bx11111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'bitfield-fill=zero-fill instr-class=general isa=A64 mnemonic=UBFM',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=LSR_UBFM xid=zero_fill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};

CLASS MNEG_MSUB_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op54=0b00 ig0=0b11011 op31=0b000 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0b11111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=MSUB',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_3src page=MNEG_MSUB xid=general',
};

CLASS MOV_ADD_addsub_imm_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b0 S:u=0b0 ig0=0b100010 sh:u=0b0 imm12:u=0b000000000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s immediate-type=imm12u instr-class=general isa=A64 mnemonic=ADD',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=addsub_imm page=MOV_ADD_addsub_imm xid=no_s',
};

CLASS MOV_MOVN_imm18_packed => {
  name       => 'Immediate packed into 16-bit value and 2-bit shift',
  diagram    => 'sf:u=0bx opc:u=0b00 ig0=0b100101 hw:u=0bxx imm16:u=0bxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  docvars    => 'immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVN',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movewide page=MOV_MOVN xid=imm18_packed',
  undefined  => 'sf == 0 && hw<1> == 1',
};

CLASS MOV_MOVZ_imm18_packed => {
  name       => 'Immediate packed into 16-bit value and 2-bit shift',
  diagram    => 'sf:u=0bx opc:u=0b10 ig0=0b100101 hw:u=0bxx imm16:u=0bxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  docvars    => 'immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVZ',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movewide page=MOV_MOVZ xid=imm18_packed',
  undefined  => 'sf == 0 && hw<1> == 1',
};

CLASS MOV_ORR_log_imm_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx opc:u=0b01 ig0=0b100100 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0b11111 Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=ORR',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log_imm page=MOV_ORR_log_imm xid=no_s',
  undefined  => 'sf == 0 && N != 0',
};

CLASS MOV_ORR_log_shift_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx opc:u=0b01 ig0=0b01010 shift:u=0b00 N:u=0b0 Rm:u=0bxxxxx imm6:u=0b000000 Rn:u=0b11111 Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=ORR reguse=shifted-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=log_shift page=MOV_ORR_log_shift xid=no_s',
  undefined  => 'sf == 0 && imm6<5> == 1',
};

CLASS MUL_MADD_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op54=0b00 ig0=0b11011 op31=0b000 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0b11111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=MADD',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_3src page=MUL_MADD xid=general',
};

CLASS MVN_ORN_log_shift_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx opc:u=0b01 ig0=0b01010 shift:u=0bxx N:u=0b1 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0b11111 Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=ORN reguse=shifted-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=log_shift page=MVN_ORN_log_shift xid=no_s',
  undefined  => 'sf == 0 && imm6<5> == 1',
};

CLASS NEG_SUB_addsub_shift_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b1 S:u=0b0 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0b11111 Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=SUB',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_shift page=NEG_SUB_addsub_shift xid=no_s',
  undefined  => '(shift == 3) || (sf == 0 && imm6<5> == 1)',
};

CLASS NEGS_SUBS_addsub_shift_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b1 S:u=0b1 ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0b11111 Rd:u=0bxxxxx',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_shift page=NEGS_SUBS_addsub_shift xid=s',
  undefined  => '(shift == 3) || (sf == 0 && imm6<5> == 1)',
};

CLASS NGC_SBC_no_s => {
  name       => 'Not setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b1 S:u=0b0 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0b11111 Rd:u=0bxxxxx',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=SBC',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_carry page=NGC_SBC xid=no_s',
};

CLASS NGCS_SBCS_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx op:u=0b1 S:u=0b1 ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0b11111 Rd:u=0bxxxxx',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=SBCS',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=addsub_carry page=NGCS_SBCS xid=s',
};

CLASS REV64_REV_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0b1 ig0=0b1 S=0b0 ig1=0b11010110 opcode2=0b00000 opcode[5:2]=0b0000 opc:u=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=REV',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_1src page=REV64_REV xid=general',
};

CLASS ROR_EXTR_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op21=0b00 ig0=0b100111 N:u=0bx o0=0b0 Rm:u=0bxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=EXTR',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=extract page=ROR_EXTR xid=general',
  undefined  => '(N != sf) || (sf == 0 && imms<5> == 1)',
};

CLASS ROR_RORV_general => {
  name       => 'Integer',
  diagram    => 'sf:u=0bx op=0b0 S=0b0 ig0=0b11010110 Rm:u=0bxxxxx opcode2=0b0010 op2:u=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=general isa=A64 mnemonic=RORV',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=dp_2src page=ROR_RORV xid=general',
};

CLASS SBFIZ_SBFM_signed_fill => {
  name       => 'With sign replication to left and zeros to right',
  diagram    => 'sf:u=0bx opc:u=0b00 ig0=0b100110 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'bitfield-fill=signed-fill instr-class=general isa=A64 mnemonic=SBFM',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=SBFIZ_SBFM xid=signed_fill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};

CLASS SBFX_SBFM_signed_fill => {
  name       => 'With sign replication to left and zeros to right',
  diagram    => 'sf:u=0bx opc:u=0b00 ig0=0b100110 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'bitfield-fill=signed-fill instr-class=general isa=A64 mnemonic=SBFM',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=SBFX_SBFM xid=signed_fill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};

CLASS SMNEGL_SMSUBL_SMSUBL_64WA_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b0 ig1=0b01 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0b11111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=SMSUBL',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=SMNEGL_SMSUBL xid=SMSUBL_64WA_dp_3src',
};

CLASS SMULL_SMADDL_SMADDL_64WA_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b0 ig1=0b01 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0b11111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=SMADDL',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=SMULL_SMADDL xid=SMADDL_64WA_dp_3src',
};

CLASS STADD_LDADD_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STADD_LDADD xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STADDB_LDADDB_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STADDB_LDADDB xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STADDH_LDADDH_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b000 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STADDH_LDADDH xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STCLR_LDCLR_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STCLR_LDCLR xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STCLRB_LDCLRB_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STCLRB_LDCLRB xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STCLRH_LDCLRH_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b001 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STCLRH_LDCLRH xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STEOR_LDEOR_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STEOR_LDEOR xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STEORB_LDEORB_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STEORB_LDEORB xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STEORH_LDEORH_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b010 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STEORH_LDEORH xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STSET_LDSET_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSET_LDSET xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STSETB_LDSETB_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSETB_LDSETB xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STSETH_LDSETH_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b011 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSETH_LDSETH xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STSMAX_LDSMAX_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMAX_LDSMAX xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STSMAXB_LDSMAXB_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMAXB_LDSMAXB xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STSMAXH_LDSMAXH_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b100 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMAXH_LDSMAXH xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STSMIN_LDSMIN_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMIN_LDSMIN xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STSMINB_LDSMINB_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMINB_LDSMINB xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STSMINH_LDSMINH_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b101 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STSMINH_LDSMINH xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STUMAX_LDUMAX_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMAX_LDUMAX xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STUMAXB_LDUMAXB_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMAXB_LDUMAXB xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STUMAXH_LDUMAXH_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b110 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMAXH_LDUMAXH xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STUMIN_LDUMIN_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b1x ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMIN_LDUMIN xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STUMINB_LDUMINB_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b00 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMINB_LDUMINB xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS STUMINH_LDUMINH_general => {
  name       => 'Integer',
  diagram    => 'size:u=0b01 ig0=0b111 V=0b0 ig1=0b00 A:u=0b0 R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3=0b0 opc:u=0b111 ig3=0b00 Rn:u=0bxxxxx Rt:u=0b11111',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=ldst iclass=memop page=STUMINH_LDUMINH xid=general',
  exceptions => 'DATAABORT|SPALIGNMENT',
  undefined  => '!HaveAtomicExt()',
};

CLASS SXTB_SBFM_signed_fill => {
  name       => 'With sign replication to left and zeros to right',
  diagram    => 'sf:u=0bx opc:u=0b00 ig0=0b100110 N:u=0bx immr:u=0b000000 imms:u=0b000111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'bitfield-fill=signed-fill instr-class=general isa=A64 mnemonic=SBFM',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=SXTB_SBFM xid=signed_fill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};

CLASS SXTH_SBFM_signed_fill => {
  name       => 'With sign replication to left and zeros to right',
  diagram    => 'sf:u=0bx opc:u=0b00 ig0=0b100110 N:u=0bx immr:u=0b000000 imms:u=0b001111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'bitfield-fill=signed-fill instr-class=general isa=A64 mnemonic=SBFM',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=SXTH_SBFM xid=signed_fill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};

CLASS SXTW_SBFM_signed_fill => {
  name       => 'With sign replication to left and zeros to right',
  diagram    => 'sf:u=0b1 opc:u=0b00 ig0=0b100110 N:u=0b1 immr:u=0b000000 imms:u=0b011111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'bitfield-fill=signed-fill instr-class=general isa=A64 mnemonic=SBFM',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=SXTW_SBFM xid=signed_fill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};

CLASS TLBI_SYS_SYS_CR_systeminstrs => {
  name       => 'System',
  diagram    => 'ig0=0b1101010100 L:u=0b0 op0=0b01 op1:u=0bxxx CRn:u=0b1000 CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control iclass=systeminstrs page=TLBI_SYS xid=SYS_CR_systeminstrs',
};

CLASS TST_ANDS_log_imm_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx opc:u=0b11 ig0=0b100100 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  docvars    => 'cond-setting=S immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=ANDS',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log_imm page=TST_ANDS_log_imm xid=s',
  undefined  => 'sf == 0 && N != 0',
};

CLASS TST_ANDS_log_shift_s => {
  name       => 'Setting the condition flags',
  diagram    => 'sf:u=0bx opc:u=0b11 ig0=0b01010 shift:u=0bxx N:u=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0b11111',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=ANDS reguse=shifted-reg',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg iclass=log_shift page=TST_ANDS_log_shift xid=s',
  undefined  => 'sf == 0 && imm6<5> == 1',
};

CLASS UBFIZ_UBFM_zero_fill => {
  name       => 'With zeros to left and right',
  diagram    => 'sf:u=0bx opc:u=0b10 ig0=0b100110 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'bitfield-fill=zero-fill instr-class=general isa=A64 mnemonic=UBFM',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=UBFIZ_UBFM xid=zero_fill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};

CLASS UBFX_UBFM_zero_fill => {
  name       => 'With zeros to left and right',
  diagram    => 'sf:u=0bx opc:u=0b10 ig0=0b100110 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'bitfield-fill=zero-fill instr-class=general isa=A64 mnemonic=UBFM',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=UBFX_UBFM xid=zero_fill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};

CLASS UMNEGL_UMSUBL_UMSUBL_64WA_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b1 ig1=0b01 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0b11111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=UMSUBL',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=UMNEGL_UMSUBL xid=UMSUBL_64WA_dp_3src',
};

CLASS UMULL_UMADDL_UMADDL_64WA_dp_3src => {
  name       => '64-bit',
  diagram    => 'sf=0b1 op54=0b00 ig0=0b11011 U:u=0b1 ig1=0b01 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0b11111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=UMADDL',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC datasize=64',
  tags       => 'group=dpreg iclass=dp_3src page=UMULL_UMADDL xid=UMADDL_64WA_dp_3src',
};

CLASS UXTB_UBFM_zero_fill => {
  name       => 'With zeros to left and right',
  diagram    => 'sf:u=0b0 opc:u=0b10 ig0=0b100110 N:u=0b0 immr:u=0b000000 imms:u=0b000111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'bitfield-fill=zero-fill instr-class=general isa=A64 mnemonic=UBFM',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=UXTB_UBFM xid=zero_fill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};

CLASS UXTH_UBFM_zero_fill => {
  name       => 'With zeros to left and right',
  diagram    => 'sf:u=0b0 opc:u=0b10 ig0=0b100110 N:u=0b0 immr:u=0b000000 imms:u=0b001111 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'bitfield-fill=zero-fill instr-class=general isa=A64 mnemonic=UBFM',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=bitfield page=UXTH_UBFM xid=zero_fill',
  undefined  => '(sf == 1 && N != 1) || (sf == 0 && (N != 0 || immr<5> != 0 || imms<5> != 0))',
};


1;

