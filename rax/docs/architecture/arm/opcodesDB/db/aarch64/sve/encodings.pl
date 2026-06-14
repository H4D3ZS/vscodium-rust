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

ENCODING abs_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 opc=0b110 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ABS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_0 page=abs_z_p_z cclass=abs_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH',
};

ENCODING adclb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 ig1=0b0 sz:u=0bx ig2=0b0 Zm:u=0bxxxxx ig3=0b11010 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=ADCLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_adc_long page=adclb_z_zzz cclass=adclb_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|CARRY',
};

ENCODING adclt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 ig1=0b0 sz:u=0bx ig2=0b0 Zm:u=0bxxxxx ig3=0b11010 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=ADCLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_adc_long page=adclt_z_zzz cclass=adclt_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|CARRY',
};

ENCODING add_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b000 opc=0b000 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_0 page=add_z_p_zz cclass=add_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING add_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 opc=0b000 ig2=0b11 sh:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm0 page=add_z_zi cclass=add_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING add_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b000 opc=0b000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_bin_cons_arit_0 page=add_z_zz cclass=add_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING addhnb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S=0b0 R=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=ADDHNB status=Green',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_arith_narrow page=addhnb_z_zz cclass=addhnb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING addhnt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S=0b0 R=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=ADDHNT status=Green',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_arith_narrow page=addhnt_z_zz cclass=addhnt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING addp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 opc=0b00 U=0b1 ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=ADDP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_arith_binary_pairs page=addp_z_p_zz cclass=addp_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING addpl_r_ri_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b000001000 op=0b1 ig1=0b1 Rn:u=0bxxxxx ig2=0b01010 imm6:u=0bxxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADDPL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_alloca iclass=sve_int_arith_vl page=addpl_r_ri cclass=addpl_r_ri_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING addvl_r_ri_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b000001000 op=0b0 ig1=0b1 Rn:u=0bxxxxx ig2=0b01010 imm6:u=0bxxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADDVL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_alloca iclass=sve_int_arith_vl page=addvl_r_ri cclass=addvl_r_ri_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING adr_z_az_sd_same_scaled => {
  name       => 'Packed offsets',
  diagram    => 'ig0=0b00000100 ig1=0b1 sz:u=0bx ig2=0b1 Zm:u=0bxxxxx ig3=0b1010 msz:u=0bxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADR status=Green sve-offset-type=off_pkd',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_bin_cons_misc_0_a page=adr_z_az cclass=adr_z_az_sd_same_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MISC',
};

ENCODING adr_z_az_d_s32_scaled => {
  name       => 'Unpacked 32-bit signed offsets',
  diagram    => 'ig0=0b00000100 opc=0b00 ig1=0b1 Zm:u=0bxxxxx ig2=0b1010 msz:u=0bxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADR status=Green sve-offset-type=off_s_s32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_bin_cons_misc_0_a page=adr_z_az cclass=adr_z_az_d_s32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MISC',
};

ENCODING adr_z_az_d_u32_scaled => {
  name       => 'Unpacked 32-bit unsigned offsets',
  diagram    => 'ig0=0b00000100 opc=0b01 ig1=0b1 Zm:u=0bxxxxx ig2=0b1010 msz:u=0bxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADR status=Green sve-offset-type=off_s_u32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_bin_cons_misc_0_a page=adr_z_az cclass=adr_z_az_d_u32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MISC',
};

ENCODING aesd_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size=0b00 ig1=0b10001 op=0b0 ig2=0b11100 o2=0b1 Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=AESD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_crypto iclass=sve_crypto_binary_dest page=aesd_z_zz cclass=aesd_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CRYPTO',
};

ENCODING aese_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size=0b00 ig1=0b10001 op=0b0 ig2=0b11100 o2=0b0 Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=AESE status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_crypto iclass=sve_crypto_binary_dest page=aese_z_zz cclass=aese_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CRYPTO',
};

ENCODING aesimc_z_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size=0b00 ig1=0b10000011100 op=0b1 ig2=0b00000 Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=AESIMC status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_crypto iclass=sve_crypto_unary page=aesimc_z_z cclass=aesimc_z_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CRYPTO',
};

ENCODING aesmc_z_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size=0b00 ig1=0b10000011100 op=0b0 ig2=0b00000 Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=AESMC status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_crypto iclass=sve_crypto_unary page=aesmc_z_z cclass=aesmc_z_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CRYPTO',
};

ENCODING and_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S:u=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=AND status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=and_p_p_pp cclass=and_p_p_pp_and_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|BITWISE',
};

ENCODING ands_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S:u=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=ANDS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=and_p_p_pp cclass=and_p_p_pp_ands_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|BITWISE',
};

ENCODING and_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b010 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=AND status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_log page=and_z_p_zz cclass=and_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING and_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 opc=0b10 ig1=0b0000 imm13:u=0bxxxxxxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=AND status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_maskimm iclass=sve_int_log_imm page=and_z_zi cclass=and_z_zi_and_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING and_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 opc=0b00 ig1=0b1 Zm:u=0bxxxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=AND status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_bin_cons_log page=and_z_zz cclass=and_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING andv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b010 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ANDV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_2 page=andv_r_p_z cclass=andv_r_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING asr_z_p_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b00 L=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=asr_z_p_zi cclass=asr_z_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|ARITHMETIC',
};

ENCODING asr_z_p_zw_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 R=0b0 L=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_2 page=asr_z_p_zw cclass=asr_z_p_zw_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|ARITHMETIC',
};

ENCODING asr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 R=0b0 L=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_1 page=asr_z_p_zz cclass=asr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|ARITHMETIC',
};

ENCODING asr_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b1001 ig3=0b0 U=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_shift iclass=sve_int_bin_cons_shift_b page=asr_z_zi cclass=asr_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|ARITHMETIC',
};

ENCODING asr_z_zw_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b1000 ig3=0b0 U=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_shift iclass=sve_int_bin_cons_shift_a page=asr_z_zw cclass=asr_z_zw_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|ARITHMETIC',
};

ENCODING asrd_z_p_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b01 L=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASRD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=asrd_z_p_zi cclass=asrd_z_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|ARITHMETIC',
};

ENCODING asrr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 R=0b1 L=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASRR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_1 page=asrr_z_p_zz cclass=asrr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|ARITHMETIC',
};

ENCODING bcax_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 opc=0b01 ig1=0b1 Zm:u=0bxxxxx ig2=0b00111 o2=0b0 Zk:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BCAX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_tern_log page=bcax_z_zzz cclass=bcax_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING bdep_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1011 opc=0b01 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BDEP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_perm_bit page=bdep_z_zz cclass=bdep_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING bext_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1011 opc=0b00 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BEXT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_perm_bit page=bext_z_zz cclass=bext_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING bfcvt_z_p_z_s2bf => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 opc=0b10 ig1=0b0010 opc2=0b10 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFCVT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_0 page=bfcvt_z_p_z cclass=bfcvt_z_p_z_s2bf',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION',
};

ENCODING bfcvtnt_z_p_z_s2bf => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100100 opc=0b10 ig1=0b0010 opc2=0b10 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFCVTNT status=Green',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fcvt2 page=bfcvtnt_z_p_z cclass=bfcvtnt_z_p_z_s2bf',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION',
};

ENCODING bfdot_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b011001000 op=0b1 ig1=0b1 Zm:u=0bxxxxx ig2=0b100000 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long iclass=sve_fp_fdot page=bfdot_z_zzz cclass=bfdot_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DOT_PRODUCT',
};

ENCODING bfdot_z_zzzi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b011001000 op=0b1 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b010000 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long_by_indexed_elem iclass=sve_fp_fdot_by_indexed_elem page=bfdot_z_zzzi cclass=bfdot_z_zzzi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DOT_PRODUCT',
};

ENCODING bfmlalb_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b011001001 o2=0b1 ig1=0b1 Zm:u=0bxxxxx ig2=0b10 op=0b0 ig3=0b00 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFMLALB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long iclass=sve_fp_fma_long page=bfmlalb_z_zzz cclass=bfmlalb_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING bfmlalb_z_zzzi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b011001001 o2=0b1 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b01 op=0b0 ig3=0b0 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFMLALB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long_by_indexed_elem iclass=sve_fp_fma_long_by_indexed_elem page=bfmlalb_z_zzzi cclass=bfmlalb_z_zzzi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING bfmlalt_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b011001001 o2=0b1 ig1=0b1 Zm:u=0bxxxxx ig2=0b10 op=0b0 ig3=0b00 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFMLALT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long iclass=sve_fp_fma_long page=bfmlalt_z_zzz cclass=bfmlalt_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING bfmlalt_z_zzzi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b011001001 o2=0b1 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b01 op=0b0 ig3=0b0 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFMLALT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long_by_indexed_elem iclass=sve_fp_fma_long_by_indexed_elem page=bfmlalt_z_zzzi cclass=bfmlalt_z_zzzi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING bfmmla_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100100 opc=0b01 ig1=0b1 Zm:u=0bxxxxx ig2=0b111001 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFMMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fmmla page=bfmmla_z_zzz cclass=bfmmla_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING bgrp_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1011 opc=0b10 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BGRP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_perm_bit page=bgrp_z_zz cclass=bgrp_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING bic_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=BIC status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=bic_p_p_pp cclass=bic_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|BITWISE',
};

ENCODING bics_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=BICS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=bic_p_p_pp cclass=bics_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|BITWISE',
};

ENCODING bic_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b011 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BIC status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_log page=bic_z_p_zz cclass=bic_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING bic_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 opc=0b11 ig1=0b1 Zm:u=0bxxxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BIC status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_bin_cons_log page=bic_z_zz cclass=bic_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING brka_p_p_p_ => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 B=0b0 S=0b0 ig1=0b01000001 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx M:u=0bx Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=BRKA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_c iclass=sve_int_break page=brka_p_p_p cclass=brka_p_p_p_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|MISC',
};

ENCODING brkas_p_p_p_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 B=0b0 S=0b1 ig1=0b01000001 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx M=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=BRKAS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_c iclass=sve_int_break page=brka_p_p_p cclass=brkas_p_p_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|MISC',
};

ENCODING brkb_p_p_p_ => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 B=0b1 S=0b0 ig1=0b01000001 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx M:u=0bx Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=BRKB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_c iclass=sve_int_break page=brkb_p_p_p cclass=brkb_p_p_p_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|MISC',
};

ENCODING brkbs_p_p_p_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 B=0b1 S=0b1 ig1=0b01000001 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx M=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=BRKBS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_c iclass=sve_int_break page=brkb_p_p_p cclass=brkbs_p_p_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|MISC',
};

ENCODING brkn_p_p_pp_ => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b001001010 S=0b0 ig1=0b01100001 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx ig3=0b0 Pdm:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=BRKN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_c iclass=sve_int_brkn page=brkn_p_p_pp cclass=brkn_p_p_pp_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|MISC',
};

ENCODING brkns_p_p_pp_ => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b001001010 S=0b1 ig1=0b01100001 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx ig3=0b0 Pdm:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=BRKNS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_c iclass=sve_int_brkn page=brkn_p_p_pp cclass=brkns_p_p_pp_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|MISC',
};

ENCODING brkpa_p_p_pp_ => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b11 Pg:u=0bxxxx ig3=0b0 Pn:u=0bxxxx B=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=BRKPA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_b iclass=sve_int_brkp page=brkpa_p_p_pp cclass=brkpa_p_p_pp_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|MISC',
};

ENCODING brkpas_p_p_pp_ => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b11 Pg:u=0bxxxx ig3=0b0 Pn:u=0bxxxx B=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=BRKPAS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_b iclass=sve_int_brkp page=brkpa_p_p_pp cclass=brkpas_p_p_pp_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|MISC',
};

ENCODING brkpb_p_p_pp_ => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b11 Pg:u=0bxxxx ig3=0b0 Pn:u=0bxxxx B=0b1 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=BRKPB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_b iclass=sve_int_brkp page=brkpb_p_p_pp cclass=brkpb_p_p_pp_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|MISC',
};

ENCODING brkpbs_p_p_pp_ => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b11 Pg:u=0bxxxx ig3=0b0 Pn:u=0bxxxx B=0b1 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=BRKPBS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_b iclass=sve_int_brkp page=brkpb_p_p_pp cclass=brkpbs_p_p_pp_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|MISC',
};

ENCODING bsl1n_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 opc=0b01 ig1=0b1 Zm:u=0bxxxxx ig2=0b00111 o2=0b1 Zk:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BSL1N status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_tern_log page=bsl1n_z_zzz cclass=bsl1n_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SELECTION',
};

ENCODING bsl2n_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 opc=0b10 ig1=0b1 Zm:u=0bxxxxx ig2=0b00111 o2=0b1 Zk:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BSL2N status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_tern_log page=bsl2n_z_zzz cclass=bsl2n_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SELECTION',
};

ENCODING bsl_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 opc=0b00 ig1=0b1 Zm:u=0bxxxxx ig2=0b00111 o2=0b1 Zk:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BSL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_tern_log page=bsl_z_zzz cclass=bsl_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SELECTION',
};

ENCODING cadd_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b00000 op=0b0 ig2=0b11011 rot:u=0bx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=CADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_cadd page=cadd_z_zz cclass=cadd_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPLEX',
};

ENCODING cdot_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0001 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=CDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_cdot page=cdot_z_zzz cclass=cdot_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DOT_PRODUCT|COMPLEX',
};

ENCODING cdot_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b0100 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=CDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_cdot_by_indexed_elem page=cdot_z_zzzi cclass=cdot_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DOT_PRODUCT|COMPLEX',
};

ENCODING cdot_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b0100 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=CDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_cdot_by_indexed_elem page=cdot_z_zzzi cclass=cdot_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DOT_PRODUCT|COMPLEX',
};

ENCODING clasta_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b11000 B=0b0 ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_clast_rz page=clasta_r_p_z cclass=clasta_r_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION|CONDITIONALLY',
};

ENCODING clasta_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10101 B=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Vdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_clast_vz page=clasta_v_p_z cclass=clasta_v_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION|CONDITIONALLY',
};

ENCODING clasta_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10100 B=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_clast_zz page=clasta_z_p_zz cclass=clasta_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION|CONDITIONALLY',
};

ENCODING clastb_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b11000 B=0b1 ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_clast_rz page=clastb_r_p_z cclass=clastb_r_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION|CONDITIONALLY',
};

ENCODING clastb_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10101 B=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Vdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_clast_vz page=clastb_v_p_z cclass=clastb_v_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION|CONDITIONALLY',
};

ENCODING clastb_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10100 B=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_clast_zz page=clastb_z_p_zz cclass=clastb_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION|CONDITIONALLY',
};

ENCODING cls_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b000 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_1 page=cls_z_p_z cclass=cls_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITFIELD',
};

ENCODING clz_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b001 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLZ status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_1 page=clz_z_p_z cclass=clz_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITFIELD',
};

ENCODING cmla_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b001 op=0b0 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=CMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_cmla page=cmla_z_zzz cclass=cmla_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPLEX',
};

ENCODING cmla_z_zzzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b0110 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=CMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_cmla_by_indexed_elem page=cmla_z_zzzi cclass=cmla_z_zzzi_h',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPLEX',
};

ENCODING cmla_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b0110 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=CMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_cmla_by_indexed_elem page=cmla_z_zzzi cclass=cmla_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPLEX',
};

ENCODING cmpeq_p_p_zi_ => {
  name       => 'Equal',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b0 imm5:u=0bxxxxx op=0b1 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPEQ status=Green sve-compare-type=eq',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_scmp_vi page=cmpeq_p_p_zi cclass=cmpeq_p_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmpgt_p_p_zi_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b0 imm5:u=0bxxxxx op=0b0 ig2=0b0 lt=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPGT status=Green sve-compare-type=gt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_scmp_vi page=cmpeq_p_p_zi cclass=cmpgt_p_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmpge_p_p_zi_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b0 imm5:u=0bxxxxx op=0b0 ig2=0b0 lt=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPGE status=Green sve-compare-type=ge',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_scmp_vi page=cmpeq_p_p_zi cclass=cmpge_p_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmphi_p_p_zi_ => {
  name       => 'Higher',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b1 imm7:u=0bxxxxxxx lt=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPHI status=Green sve-compare-type=hi',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_ucmp_vi page=cmpeq_p_p_zi cclass=cmphi_p_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmphs_p_p_zi_ => {
  name       => 'Higher or same',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b1 imm7:u=0bxxxxxxx lt=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPHS status=Green sve-compare-type=hs',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_ucmp_vi page=cmpeq_p_p_zi cclass=cmphs_p_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|COMPARISON',
};

ENCODING cmplt_p_p_zi_ => {
  name       => 'Less than',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b0 imm5:u=0bxxxxx op=0b0 ig2=0b0 lt=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPLT status=Green sve-compare-type=lt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_scmp_vi page=cmpeq_p_p_zi cclass=cmplt_p_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmple_p_p_zi_ => {
  name       => 'Less than or equal',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b0 imm5:u=0bxxxxx op=0b0 ig2=0b0 lt=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPLE status=Green sve-compare-type=le',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_scmp_vi page=cmpeq_p_p_zi cclass=cmple_p_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmplo_p_p_zi_ => {
  name       => 'Lower',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b1 imm7:u=0bxxxxxxx lt=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPLO status=Green sve-compare-type=lo',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_ucmp_vi page=cmpeq_p_p_zi cclass=cmplo_p_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmpls_p_p_zi_ => {
  name       => 'Lower or same',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b1 imm7:u=0bxxxxxxx lt=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPLS status=Green sve-compare-type=ls',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_ucmp_vi page=cmpeq_p_p_zi cclass=cmpls_p_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|COMPARISON',
};

ENCODING cmpne_p_p_zi_ => {
  name       => 'Not equal',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b0 imm5:u=0bxxxxx op=0b1 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPNE status=Green sve-compare-type=ne',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_scmp_vi page=cmpeq_p_p_zi cclass=cmpne_p_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmpeq_p_p_zw_ => {
  name       => 'Equal',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b0 o2=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPEQ status=Green sve-compare-type=eq',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=cmpeq_p_p_zw cclass=cmpeq_p_p_zw_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmpgt_p_p_zw_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U=0b0 ig2=0b1 lt=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPGT status=Green sve-compare-type=gt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_1 page=cmpeq_p_p_zw cclass=cmpgt_p_p_zw_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmpge_p_p_zw_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U=0b0 ig2=0b1 lt=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPGE status=Green sve-compare-type=ge',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_1 page=cmpeq_p_p_zw cclass=cmpge_p_p_zw_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmphi_p_p_zw_ => {
  name       => 'Higher',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U=0b1 ig2=0b1 lt=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPHI status=Green sve-compare-type=hi',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_1 page=cmpeq_p_p_zw cclass=cmphi_p_p_zw_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmphs_p_p_zw_ => {
  name       => 'Higher or same',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U=0b1 ig2=0b1 lt=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPHS status=Green sve-compare-type=hs',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_1 page=cmpeq_p_p_zw cclass=cmphs_p_p_zw_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|COMPARISON',
};

ENCODING cmplt_p_p_zw_ => {
  name       => 'Less than',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U=0b0 ig2=0b1 lt=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPLT status=Green sve-compare-type=lt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_1 page=cmpeq_p_p_zw cclass=cmplt_p_p_zw_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmple_p_p_zw_ => {
  name       => 'Less than or equal',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U=0b0 ig2=0b1 lt=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPLE status=Green sve-compare-type=le',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_1 page=cmpeq_p_p_zw cclass=cmple_p_p_zw_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmplo_p_p_zw_ => {
  name       => 'Lower',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U=0b1 ig2=0b1 lt=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPLO status=Green sve-compare-type=lo',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_1 page=cmpeq_p_p_zw cclass=cmplo_p_p_zw_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmpls_p_p_zw_ => {
  name       => 'Lower or same',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U=0b1 ig2=0b1 lt=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPLS status=Green sve-compare-type=ls',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_1 page=cmpeq_p_p_zw cclass=cmpls_p_p_zw_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|COMPARISON',
};

ENCODING cmpne_p_p_zw_ => {
  name       => 'Not equal',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b0 o2=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPNE status=Green sve-compare-type=ne',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=cmpeq_p_p_zw cclass=cmpne_p_p_zw_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmpeq_p_p_zz_ => {
  name       => 'Equal',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b0 o2=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPEQ status=Green sve-compare-type=eq',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=cmpeq_p_p_zz cclass=cmpeq_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmpgt_p_p_zz_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPGT status=Green sve-compare-type=gt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=cmpeq_p_p_zz cclass=cmpeq_p_p_zz_cmpgt_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmpge_p_p_zz_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPGE status=Green sve-compare-type=ge',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=cmpeq_p_p_zz cclass=cmpeq_p_p_zz_cmpge_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmphi_p_p_zz_ => {
  name       => 'Higher',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPHI status=Green sve-compare-type=hi',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=cmpeq_p_p_zz cclass=cmpeq_p_p_zz_cmphi_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cmphs_p_p_zz_ => {
  name       => 'Higher or same',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPHS status=Green sve-compare-type=hs',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=cmpeq_p_p_zz cclass=cmpeq_p_p_zz_cmphs_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|COMPARISON',
};

ENCODING cmpne_p_p_zz_ => {
  name       => 'Not equal',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b0 o2=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPNE status=Green sve-compare-type=ne',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=cmpeq_p_p_zz cclass=cmpne_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING cnot_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b011 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CNOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_1 page=cnot_z_p_z cclass=cnot_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING cnt_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b010 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_1 page=cnt_z_p_z cclass=cnt_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITFIELD',
};

ENCODING cntb_r_s_ => {
  name       => 'Byte',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b10 imm4:u=0bxxxx ig2=0b11100 op=0b0 pattern:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CNTB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_count page=cntb_r_s cclass=cntb_r_s_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING cntd_r_s_ => {
  name       => 'Doubleword',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b10 imm4:u=0bxxxx ig2=0b11100 op=0b0 pattern:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CNTD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_count page=cntb_r_s cclass=cntd_r_s_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING cnth_r_s_ => {
  name       => 'Halfword',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b10 imm4:u=0bxxxx ig2=0b11100 op=0b0 pattern:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CNTH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_count page=cntb_r_s cclass=cnth_r_s_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING cntw_r_s_ => {
  name       => 'Word',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b10 imm4:u=0bxxxx ig2=0b11100 op=0b0 pattern:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CNTW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_count page=cntb_r_s cclass=cntw_r_s_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING cntp_r_p_p_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 opc=0b000 ig2=0b10 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CNTP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pcount_pred page=cntp_r_p_p cclass=cntp_r_p_p_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITFIELD',
};

ENCODING compact_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b100001100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=COMPACT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_compact page=compact_z_p_z cclass=compact_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING cpy_z_o_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b0 M=0b0 sh:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_pred iclass=sve_int_dup_imm_pred page=cpy_z_o_i cclass=cpy_z_o_i_cpy_z_o_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED',
};

ENCODING cpy_z_p_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b0 M=0b1 sh:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_wideimm_pred iclass=sve_int_dup_imm_pred page=cpy_z_p_i cclass=cpy_z_p_i_cpy_z_p_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED',
};

ENCODING cpy_z_p_r_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b101000101 Pg:u=0bxxx Rn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_cpy_r page=cpy_z_p_r cclass=cpy_z_p_r_cpy_z_p_r_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
};

ENCODING cpy_z_p_v_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b100000100 Pg:u=0bxxx Vn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_cpy_v page=cpy_z_p_v cclass=cpy_z_p_v_cpy_z_p_v_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
};

ENCODING ctermeq_rr_ => {
  name       => 'Equal',
  diagram    => 'ig0=0b00 ig1=0b100101 op=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b001000 Rn:u=0bxxxxx ne=0b0 ig4=0b0 ig5=0b0 ig6=0b00',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CTERMEQ status=Green sve-compare-type=eq',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_cterm page=ctermeq_rr cclass=ctermeq_rr_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING ctermne_rr_ => {
  name       => 'Not equal',
  diagram    => 'ig0=0b00 ig1=0b100101 op=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b001000 Rn:u=0bxxxxx ne=0b1 ig4=0b0 ig5=0b0 ig6=0b00',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CTERMNE status=Green sve-compare-type=ne',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_cterm page=ctermeq_rr cclass=ctermne_rr_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'SVE|COMPARISON',
};

ENCODING decb_r_rs_ => {
  name       => 'Byte',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_a page=decb_r_rs cclass=decb_r_rs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING decd_r_rs_ => {
  name       => 'Doubleword',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_a page=decb_r_rs cclass=decd_r_rs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING dech_r_rs_ => {
  name       => 'Halfword',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_a page=decb_r_rs cclass=dech_r_rs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING decw_r_rs_ => {
  name       => 'Word',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_a page=decb_r_rs cclass=decw_r_rs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING decd_z_zs_ => {
  name       => 'Doubleword',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b11 imm4:u=0bxxxx ig2=0b11000 D=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv1 page=decd_z_zs cclass=decd_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING dech_z_zs_ => {
  name       => 'Halfword',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b11 imm4:u=0bxxxx ig2=0b11000 D=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv1 page=decd_z_zs cclass=dech_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING decw_z_zs_ => {
  name       => 'Word',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b11 imm4:u=0bxxxx ig2=0b11000 D=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv1 page=decd_z_zs cclass=decw_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING decp_r_p_r_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1011 op=0b0 D=0b1 ig2=0b10001 opc2=0b00 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r page=decp_r_p_r cclass=decp_r_p_r_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING decp_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1011 op=0b0 D=0b1 ig2=0b10000 opc2=0b00 Pm:u=0bxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_v page=decp_z_p_z cclass=decp_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING dup_z_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b111 opc=0b00 ig2=0b011 sh:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1 broadcast=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_dup_imm page=dup_z_i cclass=dup_z_i_dup_z_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING|SIGNED',
};

ENCODING dup_z_r_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b100000001110 Rn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1 broadcast=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_dup_r page=dup_z_r cclass=dup_z_r_dup_z_r_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING',
};

ENCODING dup_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 imm2:u=0bxx ig1=0b1 tsz:u=0bxxxxx ig2=0b001000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1 broadcast=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_dup_i page=dup_z_zi cclass=dup_z_zi_dup_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING',
};

ENCODING dupm_z_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101110000 imm13:u=0bxxxxxxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DUPM status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1 broadcast=1',
  tags       => 'group=sve_maskimm iclass=sve_int_dup_mask_imm page=dupm_z_i cclass=dupm_z_i_dupm_z_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING',
};

ENCODING eor3_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 opc=0b00 ig1=0b1 Zm:u=0bxxxxx ig2=0b00111 o2=0b0 Zk:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=EOR3 status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_tern_log page=eor3_z_zzz cclass=eor3_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING eor_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=EOR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=eor_p_p_pp cclass=eor_p_p_pp_eor_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|BITWISE',
};

ENCODING eors_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=EORS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=eor_p_p_pp cclass=eor_p_p_pp_eors_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|BITWISE',
};

ENCODING eor_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b001 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EOR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_log page=eor_z_p_zz cclass=eor_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING eor_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 opc=0b01 ig1=0b0000 imm13:u=0bxxxxxxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EOR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_maskimm iclass=sve_int_log_imm page=eor_z_zi cclass=eor_z_zi_eor_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING eor_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 opc=0b10 ig1=0b1 Zm:u=0bxxxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EOR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_bin_cons_log page=eor_z_zz cclass=eor_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING eorbt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b10010 tb=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=EORBT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_eorx page=eorbt_z_zz cclass=eorbt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING eortb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b10010 tb=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=EORTB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_eorx page=eortb_z_zz cclass=eortb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING eorv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b001 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EORV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_2 page=eorv_r_p_z cclass=eorv_r_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING ext_z_zi_con => {
  name       => 'Constructive',
  diagram    => 'ig0=0b00000101011 imm8h:u=0bxxxxx ig1=0b000 imm8l:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EXT status=Green sve-dest_const=sve-const',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_extract iclass=sve_intx_perm_extract_i page=ext_z_zi cclass=ext_z_zi_con',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION',
};

ENCODING ext_z_zi_des => {
  name       => 'Destructive',
  diagram    => 'ig0=0b00000101001 imm8h:u=0bxxxxx ig1=0b000 imm8l:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EXT status=Green sve-dest_const=sve-dest',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_extract iclass=sve_int_perm_extract_i page=ext_z_zi cclass=ext_z_zi_des',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION',
};

ENCODING fabd_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b1000 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FABD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fabd_z_p_zz cclass=fabd_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH',
};

ENCODING fabs_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b100 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FABS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_1 page=fabs_z_p_z cclass=fabs_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH',
};

ENCODING facgt_p_p_zz_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b1 o2=0b1 Pg:u=0bxxx Zn:u=0bxxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FACGT status=Green sve-compare-type=gt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=facge_p_p_zz cclass=facge_p_p_zz_facgt_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPARISON',
};

ENCODING facge_p_p_zz_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b1 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FACGE status=Green sve-compare-type=ge',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=facge_p_p_zz cclass=facge_p_p_zz_facge_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPARISON',
};

ENCODING fadd_z_p_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b000 ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_i_p_zds page=fadd_z_p_zs cclass=fadd_z_p_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING fadd_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b0000 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fadd_z_p_zz cclass=fadd_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING fadd_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b000 opc=0b000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_u_zd page=fadd_z_zz cclass=fadd_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING fadda_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b000 ig2=0b001 Pg:u=0bxxx Zm:u=0bxxxxx Vdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FADDA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_2op_p_vd page=fadda_v_p_z cclass=fadda_v_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING faddp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b010 opc=0b000 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FADDP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_pairwise page=faddp_z_p_zz cclass=faddp_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING faddv_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b000 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FADDV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fast_red page=faddv_v_p_z cclass=faddv_v_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING fcadd_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b00000 rot:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve iclass=sve_fp_fcadd page=fcadd_z_p_zz cclass=fcadd_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPLEX',
};

ENCODING fcmeq_p_p_z0_ => {
  name       => 'Equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0100 eq=0b1 lt=0b0 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMEQ status=Green sve-compare-type=eq',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_cmpzero iclass=sve_fp_2op_p_pd page=fcmeq_p_p_z0 cclass=fcmeq_p_p_z0_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPARISON',
};

ENCODING fcmgt_p_p_z0_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0100 eq=0b0 lt=0b0 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMGT status=Green sve-compare-type=gt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_cmpzero iclass=sve_fp_2op_p_pd page=fcmeq_p_p_z0 cclass=fcmgt_p_p_z0_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPARISON',
};

ENCODING fcmge_p_p_z0_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0100 eq=0b0 lt=0b0 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMGE status=Green sve-compare-type=ge',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_cmpzero iclass=sve_fp_2op_p_pd page=fcmeq_p_p_z0 cclass=fcmge_p_p_z0_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPARISON',
};

ENCODING fcmlt_p_p_z0_ => {
  name       => 'Less than',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0100 eq=0b0 lt=0b1 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMLT status=Green sve-compare-type=lt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_cmpzero iclass=sve_fp_2op_p_pd page=fcmeq_p_p_z0 cclass=fcmlt_p_p_z0_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPARISON',
};

ENCODING fcmle_p_p_z0_ => {
  name       => 'Less than or equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0100 eq=0b0 lt=0b1 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMLE status=Green sve-compare-type=le',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_cmpzero iclass=sve_fp_2op_p_pd page=fcmeq_p_p_z0 cclass=fcmle_p_p_z0_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPARISON',
};

ENCODING fcmne_p_p_z0_ => {
  name       => 'Not equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0100 eq=0b1 lt=0b1 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMNE status=Green sve-compare-type=ne',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_cmpzero iclass=sve_fp_2op_p_pd page=fcmeq_p_p_z0 cclass=fcmne_p_p_z0_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPARISON',
};

ENCODING fcmeq_p_p_zz_ => {
  name       => 'Equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b1 cmph=0b1 Pg:u=0bxxx Zn:u=0bxxxxx cmpl=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMEQ status=Green sve-compare-type=eq',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=fcmeq_p_p_zz cclass=fcmeq_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPARISON',
};

ENCODING fcmgt_p_p_zz_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b1 cmph=0b0 Pg:u=0bxxx Zn:u=0bxxxxx cmpl=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMGT status=Green sve-compare-type=gt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=fcmeq_p_p_zz cclass=fcmeq_p_p_zz_fcmgt_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPARISON',
};

ENCODING fcmge_p_p_zz_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b1 cmph=0b0 Pg:u=0bxxx Zn:u=0bxxxxx cmpl=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMGE status=Green sve-compare-type=ge',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=fcmeq_p_p_zz cclass=fcmeq_p_p_zz_fcmge_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPARISON',
};

ENCODING fcmne_p_p_zz_ => {
  name       => 'Not equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b1 cmph=0b1 Pg:u=0bxxx Zn:u=0bxxxxx cmpl=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMNE status=Green sve-compare-type=ne',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=fcmeq_p_p_zz cclass=fcmne_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPARISON',
};

ENCODING fcmuo_p_p_zz_ => {
  name       => 'Unordered',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b1 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMUO status=Green sve-compare-type=uo',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=fcmeq_p_p_zz cclass=fcmuo_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPARISON',
};

ENCODING fcmla_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 rot:u=0bxx Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve iclass=sve_fp_fcmla page=fcmla_z_p_zzz cclass=fcmla_z_p_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPLEX',
};

ENCODING fcmla_z_zzzi_h => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01100100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b0001 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=sve isa=A64 mnemonic=FCMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fcmla_by_indexed_elem page=fcmla_z_zzzi cclass=fcmla_z_zzzi_h',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPLEX',
};

ENCODING fcmla_z_zzzi_s => {
  name       => 'Single-precision',
  diagram    => 'ig0=0b01100100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b0001 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=single instr-class=sve isa=A64 mnemonic=FCMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fcmla_by_indexed_elem page=fcmla_z_zzzi cclass=fcmla_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPLEX',
};

ENCODING fcpy_z_p_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b110 imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCPY status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_wideimm_pred iclass=sve_int_dup_fpimm_pred page=fcpy_z_p_i cclass=fcpy_z_p_i_fcpy_z_p_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
};

ENCODING fcvt_z_p_z_h2s => {
  name       => 'Half-precision to single-precision',
  diagram    => 'ig0=0b01100101 opc=0b10 ig1=0b0010 opc2=0b01 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-single instr-class=sve isa=A64 mnemonic=FCVT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_0 page=fcvt_z_p_z cclass=fcvt_z_p_z_h2s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION',
};

ENCODING fcvt_z_p_z_h2d => {
  name       => 'Half-precision to double-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b0010 opc2=0b01 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-double instr-class=sve isa=A64 mnemonic=FCVT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_0 page=fcvt_z_p_z cclass=fcvt_z_p_z_h2d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION',
};

ENCODING fcvt_z_p_z_s2h => {
  name       => 'Single-precision to half-precision',
  diagram    => 'ig0=0b01100101 opc=0b10 ig1=0b0010 opc2=0b00 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-half instr-class=sve isa=A64 mnemonic=FCVT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_0 page=fcvt_z_p_z cclass=fcvt_z_p_z_s2h',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION',
};

ENCODING fcvt_z_p_z_s2d => {
  name       => 'Single-precision to double-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b0010 opc2=0b11 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-double instr-class=sve isa=A64 mnemonic=FCVT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_0 page=fcvt_z_p_z cclass=fcvt_z_p_z_s2d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION',
};

ENCODING fcvt_z_p_z_d2h => {
  name       => 'Double-precision to half-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b0010 opc2=0b00 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-half instr-class=sve isa=A64 mnemonic=FCVT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_0 page=fcvt_z_p_z cclass=fcvt_z_p_z_d2h',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION',
};

ENCODING fcvt_z_p_z_d2s => {
  name       => 'Double-precision to single-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b0010 opc2=0b10 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-single instr-class=sve isa=A64 mnemonic=FCVT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_0 page=fcvt_z_p_z cclass=fcvt_z_p_z_d2s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION',
};

ENCODING fcvtlt_z_p_z_h2s => {
  name       => 'Half-precision to single-precision',
  diagram    => 'ig0=0b01100100 opc=0b10 ig1=0b0010 opc2=0b01 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-single instr-class=sve2 isa=A64 mnemonic=FCVTLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fcvt2 page=fcvtlt_z_p_z cclass=fcvtlt_z_p_z_h2s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION',
};

ENCODING fcvtlt_z_p_z_s2d => {
  name       => 'Single-precision to double-precision',
  diagram    => 'ig0=0b01100100 opc=0b11 ig1=0b0010 opc2=0b11 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-double instr-class=sve2 isa=A64 mnemonic=FCVTLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fcvt2 page=fcvtlt_z_p_z cclass=fcvtlt_z_p_z_s2d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION',
};

ENCODING fcvtnt_z_p_z_s2h => {
  name       => 'Single-precision to half-precision',
  diagram    => 'ig0=0b01100100 opc=0b10 ig1=0b0010 opc2=0b00 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-half instr-class=sve2 isa=A64 mnemonic=FCVTNT status=Green',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fcvt2 page=fcvtnt_z_p_z cclass=fcvtnt_z_p_z_s2h',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION',
};

ENCODING fcvtnt_z_p_z_d2s => {
  name       => 'Double-precision to single-precision',
  diagram    => 'ig0=0b01100100 opc=0b11 ig1=0b0010 opc2=0b10 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-single instr-class=sve2 isa=A64 mnemonic=FCVTNT status=Green',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fcvt2 page=fcvtnt_z_p_z cclass=fcvtnt_z_p_z_d2s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION',
};

ENCODING fcvtx_z_p_z_d2s => {
  name       => 'Double-precision to single-precision',
  diagram    => 'ig0=0b01100101 opc=0b00 ig1=0b0010 opc2=0b10 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-single instr-class=sve2 isa=A64 mnemonic=FCVTX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_0 page=fcvtx_z_p_z cclass=fcvtx_z_p_z_d2s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION',
};

ENCODING fcvtxnt_z_p_z_d2s => {
  name       => 'Double-precision to single-precision',
  diagram    => 'ig0=0b01100100 opc=0b00 ig1=0b0010 opc2=0b10 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-single instr-class=sve2 isa=A64 mnemonic=FCVTXNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fcvt2 page=fcvtxnt_z_p_z cclass=fcvtxnt_z_p_z_d2s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION',
};

ENCODING fcvtzs_z_p_z_fp162h => {
  name       => 'Half-precision to 16-bit',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b011 opc2=0b01 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-16 instr-class=sve isa=A64 mnemonic=FCVTZS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzs_z_p_z cclass=fcvtzs_z_p_z_fp162h',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|SIGNED',
};

ENCODING fcvtzs_z_p_z_fp162w => {
  name       => 'Half-precision to 32-bit',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b011 opc2=0b10 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-32 instr-class=sve isa=A64 mnemonic=FCVTZS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzs_z_p_z cclass=fcvtzs_z_p_z_fp162w',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|SIGNED',
};

ENCODING fcvtzs_z_p_z_fp162x => {
  name       => 'Half-precision to 64-bit',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b011 opc2=0b11 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-64 instr-class=sve isa=A64 mnemonic=FCVTZS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzs_z_p_z cclass=fcvtzs_z_p_z_fp162x',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|SIGNED',
};

ENCODING fcvtzs_z_p_z_s2w => {
  name       => 'Single-precision to 32-bit',
  diagram    => 'ig0=0b01100101 opc=0b10 ig1=0b011 opc2=0b10 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-32 instr-class=sve isa=A64 mnemonic=FCVTZS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzs_z_p_z cclass=fcvtzs_z_p_z_s2w',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|SIGNED',
};

ENCODING fcvtzs_z_p_z_s2x => {
  name       => 'Single-precision to 64-bit',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b011 opc2=0b10 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-64 instr-class=sve isa=A64 mnemonic=FCVTZS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzs_z_p_z cclass=fcvtzs_z_p_z_s2x',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|SIGNED',
};

ENCODING fcvtzs_z_p_z_d2w => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b011 opc2=0b00 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-32 instr-class=sve isa=A64 mnemonic=FCVTZS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzs_z_p_z cclass=fcvtzs_z_p_z_d2w',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|SIGNED',
};

ENCODING fcvtzs_z_p_z_d2x => {
  name       => 'Double-precision to 64-bit',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b011 opc2=0b11 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-64 instr-class=sve isa=A64 mnemonic=FCVTZS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzs_z_p_z cclass=fcvtzs_z_p_z_d2x',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|SIGNED',
};

ENCODING fcvtzu_z_p_z_fp162h => {
  name       => 'Half-precision to 16-bit',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b011 opc2=0b01 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-16 instr-class=sve isa=A64 mnemonic=FCVTZU status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzu_z_p_z cclass=fcvtzu_z_p_z_fp162h',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|UNSIGNED',
};

ENCODING fcvtzu_z_p_z_fp162w => {
  name       => 'Half-precision to 32-bit',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b011 opc2=0b10 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-32 instr-class=sve isa=A64 mnemonic=FCVTZU status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzu_z_p_z cclass=fcvtzu_z_p_z_fp162w',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|UNSIGNED',
};

ENCODING fcvtzu_z_p_z_fp162x => {
  name       => 'Half-precision to 64-bit',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b011 opc2=0b11 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-64 instr-class=sve isa=A64 mnemonic=FCVTZU status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzu_z_p_z cclass=fcvtzu_z_p_z_fp162x',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|UNSIGNED',
};

ENCODING fcvtzu_z_p_z_s2w => {
  name       => 'Single-precision to 32-bit',
  diagram    => 'ig0=0b01100101 opc=0b10 ig1=0b011 opc2=0b10 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-32 instr-class=sve isa=A64 mnemonic=FCVTZU status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzu_z_p_z cclass=fcvtzu_z_p_z_s2w',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|UNSIGNED',
};

ENCODING fcvtzu_z_p_z_s2x => {
  name       => 'Single-precision to 64-bit',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b011 opc2=0b10 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-64 instr-class=sve isa=A64 mnemonic=FCVTZU status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzu_z_p_z cclass=fcvtzu_z_p_z_s2x',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|UNSIGNED',
};

ENCODING fcvtzu_z_p_z_d2w => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b011 opc2=0b00 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-32 instr-class=sve isa=A64 mnemonic=FCVTZU status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzu_z_p_z cclass=fcvtzu_z_p_z_d2w',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|UNSIGNED',
};

ENCODING fcvtzu_z_p_z_d2x => {
  name       => 'Double-precision to 64-bit',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b011 opc2=0b11 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-64 instr-class=sve isa=A64 mnemonic=FCVTZU status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzu_z_p_z cclass=fcvtzu_z_p_z_d2x',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|UNSIGNED',
};

ENCODING fdiv_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b1101 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FDIV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fdiv_z_p_zz cclass=fdiv_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING fdivr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b1100 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FDIVR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fdivr_z_p_zz cclass=fdivr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING fdup_z_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b111 opc=0b00 ig2=0b111 o2=0b0 imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FDUP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 broadcast=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_dup_fpimm page=fdup_z_i cclass=fdup_z_i_fdup_z_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING',
};

ENCODING fexpa_z_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 opc=0b00000 ig2=0b101110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FEXPA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_int_unpred_misc iclass=sve_int_bin_cons_misc_0_c page=fexpa_z_z cclass=fexpa_z_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING flogb_z_p_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01100101 opc=0b00 ig1=0b011 size:u=0bxx U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FLOGB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=flogb_z_p_z cclass=flogb_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING fmad_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Za:u=0bxxxxx ig2=0b1 N=0b0 op=0b0 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_fma iclass=sve_fp_3op_p_zds_b page=fmad_z_p_zzz cclass=fmad_z_p_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmax_z_p_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b110 ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_i_p_zds page=fmax_z_p_zs cclass=fmax_z_p_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX',
};

ENCODING fmax_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b0110 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fmax_z_p_zz cclass=fmax_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX',
};

ENCODING fmaxnm_z_p_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b100 ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAXNM status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_i_p_zds page=fmaxnm_z_p_zs cclass=fmaxnm_z_p_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX',
};

ENCODING fmaxnm_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b0100 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAXNM status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fmaxnm_z_p_zz cclass=fmaxnm_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX',
};

ENCODING fmaxnmp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b010 opc=0b100 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMAXNMP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_pairwise page=fmaxnmp_z_p_zz cclass=fmaxnmp_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX',
};

ENCODING fmaxnmv_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b100 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAXNMV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fast_red page=fmaxnmv_v_p_z cclass=fmaxnmv_v_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX',
};

ENCODING fmaxp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b010 opc=0b110 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMAXP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_pairwise page=fmaxp_z_p_zz cclass=fmaxp_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX',
};

ENCODING fmaxv_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b110 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAXV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fast_red page=fmaxv_v_p_z cclass=fmaxv_v_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX',
};

ENCODING fmin_z_p_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b111 ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMIN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_i_p_zds page=fmin_z_p_zs cclass=fmin_z_p_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX',
};

ENCODING fmin_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b0111 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMIN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fmin_z_p_zz cclass=fmin_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX',
};

ENCODING fminnm_z_p_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b101 ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMINNM status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_i_p_zds page=fminnm_z_p_zs cclass=fminnm_z_p_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX',
};

ENCODING fminnm_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b0101 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMINNM status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fminnm_z_p_zz cclass=fminnm_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX',
};

ENCODING fminnmp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b010 opc=0b101 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMINNMP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_pairwise page=fminnmp_z_p_zz cclass=fminnmp_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX',
};

ENCODING fminnmv_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b101 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMINNMV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fast_red page=fminnmv_v_p_z cclass=fminnmv_v_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX',
};

ENCODING fminp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b010 opc=0b111 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMINP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_pairwise page=fminp_z_p_zz cclass=fminp_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX',
};

ENCODING fminv_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b111 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMINV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fast_red page=fminv_v_p_z cclass=fminv_v_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX',
};

ENCODING fmla_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 N=0b0 op=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_fma iclass=sve_fp_3op_p_zds_a page=fmla_z_p_zzz cclass=fmla_z_p_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmla_z_zzzi_h => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01100100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b00000 op=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=sve isa=A64 mnemonic=FMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fma_by_indexed_elem page=fmla_z_zzzi cclass=fmla_z_zzzi_h',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmla_z_zzzi_s => {
  name       => 'Single-precision',
  diagram    => 'ig0=0b01100100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00000 op=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=single instr-class=sve isa=A64 mnemonic=FMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fma_by_indexed_elem page=fmla_z_zzzi cclass=fmla_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmla_z_zzzi_d => {
  name       => 'Double-precision',
  diagram    => 'ig0=0b01100100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b00000 op=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=double instr-class=sve isa=A64 mnemonic=FMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fma_by_indexed_elem page=fmla_z_zzzi cclass=fmla_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmlalb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b011001001 o2=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b10 op=0b0 ig3=0b00 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMLALB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long iclass=sve_fp_fma_long page=fmlalb_z_zzz cclass=fmlalb_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmlalb_z_zzzi_s => {
  name       => 'Single-precision',
  diagram    => 'ig0=0b011001001 o2=0b0 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b01 op=0b0 ig3=0b0 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=single instr-class=sve2 isa=A64 mnemonic=FMLALB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long_by_indexed_elem iclass=sve_fp_fma_long_by_indexed_elem page=fmlalb_z_zzzi cclass=fmlalb_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmlalt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b011001001 o2=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b10 op=0b0 ig3=0b00 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMLALT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long iclass=sve_fp_fma_long page=fmlalt_z_zzz cclass=fmlalt_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmlalt_z_zzzi_s => {
  name       => 'Single-precision',
  diagram    => 'ig0=0b011001001 o2=0b0 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b01 op=0b0 ig3=0b0 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=single instr-class=sve2 isa=A64 mnemonic=FMLALT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long_by_indexed_elem iclass=sve_fp_fma_long_by_indexed_elem page=fmlalt_z_zzzi cclass=fmlalt_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmls_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 N=0b0 op=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_fma iclass=sve_fp_3op_p_zds_a page=fmls_z_p_zzz cclass=fmls_z_p_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmls_z_zzzi_h => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01100100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b00000 op=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=sve isa=A64 mnemonic=FMLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fma_by_indexed_elem page=fmls_z_zzzi cclass=fmls_z_zzzi_h',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmls_z_zzzi_s => {
  name       => 'Single-precision',
  diagram    => 'ig0=0b01100100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00000 op=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=single instr-class=sve isa=A64 mnemonic=FMLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fma_by_indexed_elem page=fmls_z_zzzi cclass=fmls_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmls_z_zzzi_d => {
  name       => 'Double-precision',
  diagram    => 'ig0=0b01100100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b00000 op=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=double instr-class=sve isa=A64 mnemonic=FMLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fma_by_indexed_elem page=fmls_z_zzzi cclass=fmls_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmlslb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b011001001 o2=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b10 op=0b1 ig3=0b00 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMLSLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long iclass=sve_fp_fma_long page=fmlslb_z_zzz cclass=fmlslb_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmlslb_z_zzzi_s => {
  name       => 'Single-precision',
  diagram    => 'ig0=0b011001001 o2=0b0 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b01 op=0b1 ig3=0b0 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=single instr-class=sve2 isa=A64 mnemonic=FMLSLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long_by_indexed_elem iclass=sve_fp_fma_long_by_indexed_elem page=fmlslb_z_zzzi cclass=fmlslb_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmlslt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b011001001 o2=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b10 op=0b1 ig3=0b00 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMLSLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long iclass=sve_fp_fma_long page=fmlslt_z_zzz cclass=fmlslt_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmlslt_z_zzzi_s => {
  name       => 'Single-precision',
  diagram    => 'ig0=0b011001001 o2=0b0 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b01 op=0b1 ig3=0b0 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=single instr-class=sve2 isa=A64 mnemonic=FMLSLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long_by_indexed_elem iclass=sve_fp_fma_long_by_indexed_elem page=fmlslt_z_zzzi cclass=fmlslt_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmmla_z_zzz_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b01100100 opc=0b10 ig1=0b1 Zm:u=0bxxxxx ig2=0b111001 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMMLA status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fmmla page=fmmla_z_zzz cclass=fmmla_z_zzz_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmmla_z_zzz_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b01100100 opc=0b11 ig1=0b1 Zm:u=0bxxxxx ig2=0b111001 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMMLA status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fmmla page=fmmla_z_zzz cclass=fmmla_z_zzz_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmsb_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Za:u=0bxxxxx ig2=0b1 N=0b0 op=0b1 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMSB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_fma iclass=sve_fp_3op_p_zds_b page=fmsb_z_p_zzz cclass=fmsb_z_p_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fmul_z_p_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b010 ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_i_p_zds page=fmul_z_p_zs cclass=fmul_z_p_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING fmul_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b0010 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fmul_z_p_zz cclass=fmul_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING fmul_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b000 opc=0b010 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_u_zd page=fmul_z_zz cclass=fmul_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING fmul_z_zzi_h => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01100100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b001000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=sve isa=A64 mnemonic=FMUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fmul_by_indexed_elem page=fmul_z_zzi cclass=fmul_z_zzi_h',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING fmul_z_zzi_s => {
  name       => 'Single-precision',
  diagram    => 'ig0=0b01100100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b001000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'datatype=single instr-class=sve isa=A64 mnemonic=FMUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fmul_by_indexed_elem page=fmul_z_zzi cclass=fmul_z_zzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING fmul_z_zzi_d => {
  name       => 'Double-precision',
  diagram    => 'ig0=0b01100100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b001000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'datatype=double instr-class=sve isa=A64 mnemonic=FMUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fmul_by_indexed_elem page=fmul_z_zzi cclass=fmul_z_zzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING fmulx_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b1010 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMULX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fmulx_z_p_zz cclass=fmulx_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING fneg_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b101 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FNEG status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_1 page=fneg_z_p_z cclass=fneg_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING fnmad_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Za:u=0bxxxxx ig2=0b1 N=0b1 op=0b0 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FNMAD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_fma iclass=sve_fp_3op_p_zds_b page=fnmad_z_p_zzz cclass=fnmad_z_p_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fnmla_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 N=0b1 op=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FNMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_fma iclass=sve_fp_3op_p_zds_a page=fnmla_z_p_zzz cclass=fnmla_z_p_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fnmls_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 N=0b1 op=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FNMLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_fma iclass=sve_fp_3op_p_zds_a page=fnmls_z_p_zzz cclass=fnmls_z_p_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING fnmsb_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Za:u=0bxxxxx ig2=0b1 N=0b1 op=0b1 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FNMSB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_fma iclass=sve_fp_3op_p_zds_b page=fnmsb_z_p_zzz cclass=fnmsb_z_p_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|FUSED_OP',
};

ENCODING frecpe_z_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b001 opc=0b110 ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FRECPE status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_unary_unpred iclass=sve_fp_2op_u_zd page=frecpe_z_z cclass=frecpe_z_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH',
};

ENCODING frecps_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b000 opc=0b110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FRECPS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_u_zd page=frecps_z_zz cclass=frecps_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH',
};

ENCODING frecpx_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0011 opc=0b00 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FRECPX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_1 page=frecpx_z_p_z cclass=frecpx_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH',
};

ENCODING frinti_z_p_z_ => {
  name       => 'Current mode',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b111 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'frint-type=frint-i instr-class=sve isa=A64 mnemonic=FRINTI status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_a page=frinta_z_p_z cclass=frinti_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ROUNDING',
};

ENCODING frintx_z_p_z_ => {
  name       => 'Current mode signalling inexact',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b110 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'frint-type=frint-x instr-class=sve isa=A64 mnemonic=FRINTX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_a page=frinta_z_p_z cclass=frintx_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ROUNDING',
};

ENCODING frinta_z_p_z_ => {
  name       => 'Nearest with ties to away',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b100 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'frint-type=frint-a instr-class=sve isa=A64 mnemonic=FRINTA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_a page=frinta_z_p_z cclass=frinta_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ROUNDING',
};

ENCODING frintn_z_p_z_ => {
  name       => 'Nearest with ties to even',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b000 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'frint-type=frint-n instr-class=sve isa=A64 mnemonic=FRINTN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_a page=frinta_z_p_z cclass=frintn_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ROUNDING',
};

ENCODING frintz_z_p_z_ => {
  name       => 'Toward zero',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b011 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'frint-type=frint-z instr-class=sve isa=A64 mnemonic=FRINTZ status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_a page=frinta_z_p_z cclass=frintz_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ROUNDING',
};

ENCODING frintm_z_p_z_ => {
  name       => 'Toward minus infinity',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b010 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'frint-type=frint-m instr-class=sve isa=A64 mnemonic=FRINTM status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_a page=frinta_z_p_z cclass=frintm_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ROUNDING',
};

ENCODING frintp_z_p_z_ => {
  name       => 'Toward plus infinity',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b001 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'frint-type=frint-p instr-class=sve isa=A64 mnemonic=FRINTP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_a page=frinta_z_p_z cclass=frintp_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ROUNDING',
};

ENCODING frsqrte_z_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b001 opc=0b111 ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FRSQRTE status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_unary_unpred iclass=sve_fp_2op_u_zd page=frsqrte_z_z cclass=frsqrte_z_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH',
};

ENCODING frsqrts_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b000 opc=0b111 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FRSQRTS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_u_zd page=frsqrts_z_zz cclass=frsqrts_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH',
};

ENCODING fscale_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b1001 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSCALE status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fscale_z_p_zz cclass=fscale_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING fsqrt_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0011 opc=0b01 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSQRT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_1 page=fsqrt_z_p_z cclass=fsqrt_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH',
};

ENCODING fsub_z_p_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b001 ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_i_p_zds page=fsub_z_p_zs cclass=fsub_z_p_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING fsub_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b0001 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fsub_z_p_zz cclass=fsub_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING fsub_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b000 opc=0b001 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_u_zd page=fsub_z_zz cclass=fsub_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING fsubr_z_p_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b011 ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSUBR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_i_p_zds page=fsubr_z_p_zs cclass=fsubr_z_p_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING fsubr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b0011 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSUBR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fsubr_z_p_zz cclass=fsubr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING ftmad_z_zzi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b010 imm3:u=0bxxx ig2=0b100000 Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FTMAD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_pred iclass=sve_fp_ftmad page=ftmad_z_zzi cclass=ftmad_z_zzi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|TRIGONOMETRIC',
};

ENCODING ftsmul_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b000 opc=0b011 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FTSMUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_u_zd page=ftsmul_z_zz cclass=ftsmul_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|TRIGONOMETRIC',
};

ENCODING ftssel_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b10110 op=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FTSSEL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_int_unpred_misc iclass=sve_int_bin_cons_misc_0_b page=ftssel_z_zz cclass=ftssel_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SELECTION|TRIGONOMETRIC',
};

ENCODING histcnt_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=HISTCNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve iclass=sve_intx_histcnt page=histcnt_z_p_zz cclass=histcnt_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING histseg_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b101000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=HISTSEG status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_histseg iclass=sve_intx_histseg page=histseg_z_zz cclass=histseg_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING incb_r_rs_ => {
  name       => 'Byte',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_a page=incb_r_rs cclass=incb_r_rs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING incd_r_rs_ => {
  name       => 'Doubleword',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_a page=incb_r_rs cclass=incd_r_rs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING inch_r_rs_ => {
  name       => 'Halfword',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_a page=incb_r_rs cclass=inch_r_rs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING incw_r_rs_ => {
  name       => 'Word',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_a page=incb_r_rs cclass=incw_r_rs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING incd_z_zs_ => {
  name       => 'Doubleword',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b11 imm4:u=0bxxxx ig2=0b11000 D=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv1 page=incd_z_zs cclass=incd_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING inch_z_zs_ => {
  name       => 'Halfword',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b11 imm4:u=0bxxxx ig2=0b11000 D=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv1 page=incd_z_zs cclass=inch_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING incw_z_zs_ => {
  name       => 'Word',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b11 imm4:u=0bxxxx ig2=0b11000 D=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv1 page=incd_z_zs cclass=incw_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING incp_r_p_r_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1011 op=0b0 D=0b0 ig2=0b10001 opc2=0b00 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r page=incp_r_p_r cclass=incp_r_p_r_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING incp_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1011 op=0b0 D=0b0 ig2=0b10000 opc2=0b00 Pm:u=0bxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_v page=incp_z_p_z cclass=incp_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC',
};

ENCODING index_z_ii_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 imm5b:u=0bxxxxx ig2=0b010000 imm5:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INDEX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_index iclass=sve_int_index_ii page=index_z_ii cclass=index_z_ii_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING index_z_ir_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b010010 imm5:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INDEX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_index iclass=sve_int_index_ir page=index_z_ir cclass=index_z_ir_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING index_z_ri_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 imm5:u=0bxxxxx ig2=0b010001 Rn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INDEX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_index iclass=sve_int_index_ri page=index_z_ri cclass=index_z_ri_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING index_z_rr_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b010011 Rn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INDEX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_index iclass=sve_int_index_rr page=index_z_rr cclass=index_z_rr_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING insr_z_r_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b100100001110 Rm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INSR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_insrs page=insr_z_r cclass=insr_z_r_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|INSERTION',
};

ENCODING insr_z_v_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b110100001110 Vm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INSR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_insrv page=insr_z_v cclass=insr_z_v_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|INSERTION',
};

ENCODING lasta_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10000 B=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LASTA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_last_r page=lasta_r_p_z cclass=lasta_r_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION',
};

ENCODING lasta_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10001 B=0b0 ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LASTA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_last_v page=lasta_v_p_z cclass=lasta_v_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION',
};

ENCODING lastb_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10000 B=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LASTB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_last_r page=lastb_r_p_z cclass=lastb_r_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION',
};

ENCODING lastb_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10001 B=0b1 ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LASTB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_last_v page=lastb_v_p_z cclass=lastb_v_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION',
};

ENCODING ld1b_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b00 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ld1b_z_p_ai cclass=ld1b_z_p_ai_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1b_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ld1b_z_p_ai cclass=ld1b_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1b_z_p_bi_u8 => {
  name       => '8-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0000 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=8-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1b_z_p_bi cclass=ld1b_z_p_bi_u8',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1b_z_p_bi_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0001 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1b_z_p_bi cclass=ld1b_z_p_bi_u16',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1b_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0010 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1b_z_p_bi cclass=ld1b_z_p_bi_u32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1b_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0011 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1b_z_p_bi cclass=ld1b_z_p_bi_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1b_z_p_br_u8 => {
  name       => '8-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0000 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=8-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1b_z_p_br cclass=ld1b_z_p_br_u8',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1b_z_p_br_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0001 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1b_z_p_br cclass=ld1b_z_p_br_u16',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1b_z_p_br_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0010 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1b_z_p_br cclass=ld1b_z_p_br_u32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1b_z_p_br_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0011 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1b_z_p_br cclass=ld1b_z_p_br_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1b_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ld1b_z_p_bz cclass=ld1b_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1b_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b00 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ld1b_z_p_bz cclass=ld1b_z_p_bz_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1b_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ld1b_z_p_bz cclass=ld1b_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1d_z_p_ai_d => {
  name       => 'SVE',
  diagram    => 'ig0=0b1100010 msz=0b11 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ld1d_z_p_ai cclass=ld1d_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1d_z_p_bi_u64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 dtype=0b1111 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1d_z_p_bi cclass=ld1d_z_p_bi_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1d_z_p_br_u64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 dtype=0b1111 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1d_z_p_br cclass=ld1d_z_p_br_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1d_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b11 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ld1d_z_p_bz cclass=ld1d_z_p_bz_d_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1d_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b11 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ld1d_z_p_bz cclass=ld1d_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1d_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b11 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ld1d_z_p_bz cclass=ld1d_z_p_bz_d_64_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1d_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b11 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ld1d_z_p_bz cclass=ld1d_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1h_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b01 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ld1h_z_p_ai cclass=ld1h_z_p_ai_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1h_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ld1h_z_p_ai cclass=ld1h_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1h_z_p_bi_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0101 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1h_z_p_bi cclass=ld1h_z_p_bi_u16',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1h_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0110 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1h_z_p_bi cclass=ld1h_z_p_bi_u32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1h_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0111 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1h_z_p_bi cclass=ld1h_z_p_bi_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1h_z_p_br_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0101 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1h_z_p_br cclass=ld1h_z_p_br_u16',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1h_z_p_br_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0110 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1h_z_p_br cclass=ld1h_z_p_br_u32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1h_z_p_br_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0111 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1h_z_p_br cclass=ld1h_z_p_br_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1h_z_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001001 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_sv_a page=ld1h_z_p_bz cclass=ld1h_z_p_bz_s_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1h_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b01 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ld1h_z_p_bz cclass=ld1h_z_p_bz_d_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1h_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ld1h_z_p_bz cclass=ld1h_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1h_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b01 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ld1h_z_p_bz cclass=ld1h_z_p_bz_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1h_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b01 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ld1h_z_p_bz cclass=ld1h_z_p_bz_d_64_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1h_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ld1h_z_p_bz cclass=ld1h_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1rb_z_p_bi_u8 => {
  name       => '8-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b00 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b00 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RB status=Green sve-elem-type=8-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rb_z_p_bi cclass=ld1rb_z_p_bi_u8',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING|UNSIGNED|LOAD',
};

ENCODING ld1rb_z_p_bi_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b00 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b01 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RB status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rb_z_p_bi cclass=ld1rb_z_p_bi_u16',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING|UNSIGNED|LOAD',
};

ENCODING ld1rb_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b00 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b10 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RB status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rb_z_p_bi cclass=ld1rb_z_p_bi_u32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING|UNSIGNED|LOAD',
};

ENCODING ld1rb_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b00 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b11 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RB status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rb_z_p_bi cclass=ld1rb_z_p_bi_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING|UNSIGNED|LOAD',
};

ENCODING ld1rd_z_p_bi_u64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010 dtypeh=0b11 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b11 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RD status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rd_z_p_bi cclass=ld1rd_z_p_bi_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING|LOAD',
};

ENCODING ld1rh_z_p_bi_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b01 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b01 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RH status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rh_z_p_bi cclass=ld1rh_z_p_bi_u16',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING|UNSIGNED|LOAD',
};

ENCODING ld1rh_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b01 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b10 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RH status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rh_z_p_bi cclass=ld1rh_z_p_bi_u32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING|UNSIGNED|LOAD',
};

ENCODING ld1rh_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b01 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b11 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RH status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rh_z_p_bi cclass=ld1rh_z_p_bi_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING|UNSIGNED|LOAD',
};

ENCODING ld1rob_z_p_bi_u8 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 ssz=0b01 ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROB status=Amber',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_si page=ld1rob_z_p_bi cclass=ld1rob_z_p_bi_u8',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1rob_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 ssz=0b01 Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROB status=Amber',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_ss page=ld1rob_z_p_br cclass=ld1rob_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1rod_z_p_bi_u64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 ssz=0b01 ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROD status=Amber',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_si page=ld1rod_z_p_bi cclass=ld1rod_z_p_bi_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1rod_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 ssz=0b01 Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROD status=Amber',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_ss page=ld1rod_z_p_br cclass=ld1rod_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1roh_z_p_bi_u16 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 ssz=0b01 ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROH status=Amber',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_si page=ld1roh_z_p_bi cclass=ld1roh_z_p_bi_u16',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1roh_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 ssz=0b01 Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROH status=Amber',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_ss page=ld1roh_z_p_br cclass=ld1roh_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1row_z_p_bi_u32 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 ssz=0b01 ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROW status=Amber',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_si page=ld1row_z_p_bi cclass=ld1row_z_p_bi_u32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1row_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 ssz=0b01 Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROW status=Amber',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_ss page=ld1row_z_p_br cclass=ld1row_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1rqb_z_p_bi_u8 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 ssz=0b00 ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQB status=Green sve-elem-type=8-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_si page=ld1rqb_z_p_bi cclass=ld1rqb_z_p_bi_u8',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1rqb_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 ssz=0b00 Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_ss page=ld1rqb_z_p_br cclass=ld1rqb_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1rqd_z_p_bi_u64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 ssz=0b00 ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQD status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_si page=ld1rqd_z_p_bi cclass=ld1rqd_z_p_bi_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1rqd_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 ssz=0b00 Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_ss page=ld1rqd_z_p_br cclass=ld1rqd_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1rqh_z_p_bi_u16 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 ssz=0b00 ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQH status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_si page=ld1rqh_z_p_bi cclass=ld1rqh_z_p_bi_u16',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1rqh_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 ssz=0b00 Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_ss page=ld1rqh_z_p_br cclass=ld1rqh_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1rqw_z_p_bi_u32 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 ssz=0b00 ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQW status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_si page=ld1rqw_z_p_bi cclass=ld1rqw_z_p_bi_u32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1rqw_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 ssz=0b00 Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQW status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_ss page=ld1rqw_z_p_br cclass=ld1rqw_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld1rsb_z_p_bi_s16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b11 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b10 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RSB status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rsb_z_p_bi cclass=ld1rsb_z_p_bi_s16',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING|SIGNED|LOAD',
};

ENCODING ld1rsb_z_p_bi_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b11 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b01 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RSB status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rsb_z_p_bi cclass=ld1rsb_z_p_bi_s32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING|SIGNED|LOAD',
};

ENCODING ld1rsb_z_p_bi_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b11 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b00 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RSB status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rsb_z_p_bi cclass=ld1rsb_z_p_bi_s64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING|SIGNED|LOAD',
};

ENCODING ld1rsh_z_p_bi_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b10 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b01 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RSH status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rsh_z_p_bi cclass=ld1rsh_z_p_bi_s32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING|SIGNED|LOAD',
};

ENCODING ld1rsh_z_p_bi_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b10 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b00 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RSH status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rsh_z_p_bi cclass=ld1rsh_z_p_bi_s64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING|SIGNED|LOAD',
};

ENCODING ld1rsw_z_p_bi_s64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010 dtypeh=0b01 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b00 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RSW status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rsw_z_p_bi cclass=ld1rsw_z_p_bi_s64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING|SIGNED|LOAD',
};

ENCODING ld1rw_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b10 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b10 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RW status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rw_z_p_bi cclass=ld1rw_z_p_bi_u32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING|UNSIGNED|LOAD',
};

ENCODING ld1rw_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b10 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b11 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RW status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rw_z_p_bi cclass=ld1rw_z_p_bi_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BROADCASTING|UNSIGNED|LOAD',
};

ENCODING ld1sb_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b00 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ld1sb_z_p_ai cclass=ld1sb_z_p_ai_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sb_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ld1sb_z_p_ai cclass=ld1sb_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sb_z_p_bi_s16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1110 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1sb_z_p_bi cclass=ld1sb_z_p_bi_s16',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sb_z_p_bi_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1101 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1sb_z_p_bi cclass=ld1sb_z_p_bi_s32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sb_z_p_bi_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1100 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1sb_z_p_bi cclass=ld1sb_z_p_bi_s64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sb_z_p_br_s16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1110 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1sb_z_p_br cclass=ld1sb_z_p_br_s16',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sb_z_p_br_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1101 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1sb_z_p_br cclass=ld1sb_z_p_br_s32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sb_z_p_br_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1100 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1sb_z_p_br cclass=ld1sb_z_p_br_s64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sb_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ld1sb_z_p_bz cclass=ld1sb_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sb_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b00 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ld1sb_z_p_bz cclass=ld1sb_z_p_bz_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sb_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ld1sb_z_p_bz cclass=ld1sb_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sh_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b01 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ld1sh_z_p_ai cclass=ld1sh_z_p_ai_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sh_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ld1sh_z_p_ai cclass=ld1sh_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sh_z_p_bi_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1001 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1sh_z_p_bi cclass=ld1sh_z_p_bi_s32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sh_z_p_bi_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1000 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1sh_z_p_bi cclass=ld1sh_z_p_bi_s64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sh_z_p_br_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1001 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1sh_z_p_br cclass=ld1sh_z_p_br_s32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sh_z_p_br_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1000 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1sh_z_p_br cclass=ld1sh_z_p_br_s64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sh_z_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001001 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_sv_a page=ld1sh_z_p_bz cclass=ld1sh_z_p_bz_s_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sh_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b01 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ld1sh_z_p_bz cclass=ld1sh_z_p_bz_d_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sh_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ld1sh_z_p_bz cclass=ld1sh_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sh_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b01 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ld1sh_z_p_bz cclass=ld1sh_z_p_bz_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sh_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b01 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ld1sh_z_p_bz cclass=ld1sh_z_p_bz_d_64_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sh_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ld1sh_z_p_bz cclass=ld1sh_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sw_z_p_ai_d => {
  name       => 'SVE',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ld1sw_z_p_ai cclass=ld1sw_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sw_z_p_bi_s64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 dtype=0b0100 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1sw_z_p_bi cclass=ld1sw_z_p_bi_s64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sw_z_p_br_s64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 dtype=0b0100 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1sw_z_p_br cclass=ld1sw_z_p_br_s64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sw_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b10 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ld1sw_z_p_bz cclass=ld1sw_z_p_bz_d_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sw_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ld1sw_z_p_bz cclass=ld1sw_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sw_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b10 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ld1sw_z_p_bz cclass=ld1sw_z_p_bz_d_64_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1sw_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ld1sw_z_p_bz cclass=ld1sw_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ld1w_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b10 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ld1w_z_p_ai cclass=ld1w_z_p_ai_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1w_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ld1w_z_p_ai cclass=ld1w_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1w_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1010 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1w_z_p_bi cclass=ld1w_z_p_bi_u32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1w_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1011 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1w_z_p_bi cclass=ld1w_z_p_bi_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1w_z_p_br_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1010 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1w_z_p_br cclass=ld1w_z_p_br_u32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1w_z_p_br_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1011 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1w_z_p_br cclass=ld1w_z_p_br_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1w_z_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001010 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_sv_b page=ld1w_z_p_bz cclass=ld1w_z_p_bz_s_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1w_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b10 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ld1w_z_p_bz cclass=ld1w_z_p_bz_d_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1w_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ld1w_z_p_bz cclass=ld1w_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1w_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b10 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ld1w_z_p_bz cclass=ld1w_z_p_bz_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1w_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b10 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ld1w_z_p_bz cclass=ld1w_z_p_bz_d_64_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld1w_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ld1w_z_p_bz cclass=ld1w_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ld2b_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 opc=0b01 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld2b_z_p_bi cclass=ld2b_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld2b_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 opc=0b01 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld2b_z_p_br cclass=ld2b_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld2d_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 opc=0b01 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld2d_z_p_bi cclass=ld2d_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld2d_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 opc=0b01 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld2d_z_p_br cclass=ld2d_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld2h_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 opc=0b01 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld2h_z_p_bi cclass=ld2h_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld2h_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 opc=0b01 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld2h_z_p_br cclass=ld2h_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld2w_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 opc=0b01 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld2w_z_p_bi cclass=ld2w_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld2w_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 opc=0b01 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld2w_z_p_br cclass=ld2w_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld3b_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 opc=0b10 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld3b_z_p_bi cclass=ld3b_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld3b_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 opc=0b10 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld3b_z_p_br cclass=ld3b_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld3d_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 opc=0b10 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld3d_z_p_bi cclass=ld3d_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld3d_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 opc=0b10 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld3d_z_p_br cclass=ld3d_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld3h_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 opc=0b10 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld3h_z_p_bi cclass=ld3h_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld3h_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 opc=0b10 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld3h_z_p_br cclass=ld3h_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld3w_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 opc=0b10 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld3w_z_p_bi cclass=ld3w_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld3w_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 opc=0b10 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld3w_z_p_br cclass=ld3w_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld4b_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 opc=0b11 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld4b_z_p_bi cclass=ld4b_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld4b_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 opc=0b11 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld4b_z_p_br cclass=ld4b_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld4d_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 opc=0b11 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld4d_z_p_bi cclass=ld4d_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld4d_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 opc=0b11 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld4d_z_p_br cclass=ld4d_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld4h_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 opc=0b11 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld4h_z_p_bi cclass=ld4h_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld4h_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 opc=0b11 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld4h_z_p_br cclass=ld4h_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld4w_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 opc=0b11 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld4w_z_p_bi cclass=ld4w_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ld4w_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 opc=0b11 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld4w_z_p_br cclass=ld4w_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ldff1b_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b00 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ldff1b_z_p_ai cclass=ldff1b_z_p_ai_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1b_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ldff1b_z_p_ai cclass=ldff1b_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1b_z_p_br_u8 => {
  name       => '8-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0000 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-elem-type=8-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1b_z_p_br cclass=ldff1b_z_p_br_u8',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1b_z_p_br_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0001 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1b_z_p_br cclass=ldff1b_z_p_br_u16',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1b_z_p_br_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0010 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1b_z_p_br cclass=ldff1b_z_p_br_u32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1b_z_p_br_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0011 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1b_z_p_br cclass=ldff1b_z_p_br_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1b_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ldff1b_z_p_bz cclass=ldff1b_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1b_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b00 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ldff1b_z_p_bz cclass=ldff1b_z_p_bz_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1b_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ldff1b_z_p_bz cclass=ldff1b_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1d_z_p_ai_d => {
  name       => 'SVE',
  diagram    => 'ig0=0b1100010 msz=0b11 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ldff1d_z_p_ai cclass=ldff1d_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ldff1d_z_p_br_u64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 dtype=0b1111 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1d_z_p_br cclass=ldff1d_z_p_br_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ldff1d_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b11 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1D status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ldff1d_z_p_bz cclass=ldff1d_z_p_bz_d_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ldff1d_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b11 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1D status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ldff1d_z_p_bz cclass=ldff1d_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ldff1d_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b11 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1D status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ldff1d_z_p_bz cclass=ldff1d_z_p_bz_d_64_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ldff1d_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b11 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1D status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ldff1d_z_p_bz cclass=ldff1d_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ldff1h_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b01 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ldff1h_z_p_ai cclass=ldff1h_z_p_ai_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1h_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ldff1h_z_p_ai cclass=ldff1h_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1h_z_p_br_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0101 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1h_z_p_br cclass=ldff1h_z_p_br_u16',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1h_z_p_br_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0110 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1h_z_p_br cclass=ldff1h_z_p_br_u32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1h_z_p_br_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0111 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1h_z_p_br cclass=ldff1h_z_p_br_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1h_z_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001001 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_sv_a page=ldff1h_z_p_bz cclass=ldff1h_z_p_bz_s_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1h_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b01 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ldff1h_z_p_bz cclass=ldff1h_z_p_bz_d_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1h_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ldff1h_z_p_bz cclass=ldff1h_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1h_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b01 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ldff1h_z_p_bz cclass=ldff1h_z_p_bz_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1h_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b01 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ldff1h_z_p_bz cclass=ldff1h_z_p_bz_d_64_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1h_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ldff1h_z_p_bz cclass=ldff1h_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1sb_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b00 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ldff1sb_z_p_ai cclass=ldff1sb_z_p_ai_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sb_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ldff1sb_z_p_ai cclass=ldff1sb_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sb_z_p_br_s16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1110 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1sb_z_p_br cclass=ldff1sb_z_p_br_s16',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sb_z_p_br_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1101 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1sb_z_p_br cclass=ldff1sb_z_p_br_s32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sb_z_p_br_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1100 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1sb_z_p_br cclass=ldff1sb_z_p_br_s64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sb_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ldff1sb_z_p_bz cclass=ldff1sb_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sb_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b00 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ldff1sb_z_p_bz cclass=ldff1sb_z_p_bz_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sb_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ldff1sb_z_p_bz cclass=ldff1sb_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sh_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b01 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ldff1sh_z_p_ai cclass=ldff1sh_z_p_ai_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sh_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ldff1sh_z_p_ai cclass=ldff1sh_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sh_z_p_br_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1001 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1sh_z_p_br cclass=ldff1sh_z_p_br_s32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sh_z_p_br_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1000 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1sh_z_p_br cclass=ldff1sh_z_p_br_s64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sh_z_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001001 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_sv_a page=ldff1sh_z_p_bz cclass=ldff1sh_z_p_bz_s_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sh_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b01 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ldff1sh_z_p_bz cclass=ldff1sh_z_p_bz_d_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sh_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ldff1sh_z_p_bz cclass=ldff1sh_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sh_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b01 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ldff1sh_z_p_bz cclass=ldff1sh_z_p_bz_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sh_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b01 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ldff1sh_z_p_bz cclass=ldff1sh_z_p_bz_d_64_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sh_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ldff1sh_z_p_bz cclass=ldff1sh_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sw_z_p_ai_d => {
  name       => 'SVE',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SW status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ldff1sw_z_p_ai cclass=ldff1sw_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sw_z_p_br_s64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 dtype=0b0100 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SW status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1sw_z_p_br cclass=ldff1sw_z_p_br_s64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sw_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b10 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SW status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ldff1sw_z_p_bz cclass=ldff1sw_z_p_bz_d_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sw_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SW status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ldff1sw_z_p_bz cclass=ldff1sw_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sw_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b10 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SW status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ldff1sw_z_p_bz cclass=ldff1sw_z_p_bz_d_64_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1sw_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SW status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ldff1sw_z_p_bz cclass=ldff1sw_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldff1w_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b10 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ldff1w_z_p_ai cclass=ldff1w_z_p_ai_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1w_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ldff1w_z_p_ai cclass=ldff1w_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1w_z_p_br_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1010 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1w_z_p_br cclass=ldff1w_z_p_br_u32',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1w_z_p_br_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1011 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1w_z_p_br cclass=ldff1w_z_p_br_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1w_z_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001010 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_sv_b page=ldff1w_z_p_bz cclass=ldff1w_z_p_bz_s_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1w_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b10 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ldff1w_z_p_bz cclass=ldff1w_z_p_bz_d_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1w_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ldff1w_z_p_bz cclass=ldff1w_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1w_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b10 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ldff1w_z_p_bz cclass=ldff1w_z_p_bz_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1w_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b10 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ldff1w_z_p_bz cclass=ldff1w_z_p_bz_d_64_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldff1w_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ldff1w_z_p_bz cclass=ldff1w_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldnf1b_z_p_bi_u8 => {
  name       => '8-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0000 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1B status=Green sve-elem-type=8-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1b_z_p_bi cclass=ldnf1b_z_p_bi_u8',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldnf1b_z_p_bi_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0001 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1B status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1b_z_p_bi cclass=ldnf1b_z_p_bi_u16',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldnf1b_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0010 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1B status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1b_z_p_bi cclass=ldnf1b_z_p_bi_u32',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldnf1b_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0011 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1B status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1b_z_p_bi cclass=ldnf1b_z_p_bi_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldnf1d_z_p_bi_u64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 dtype=0b1111 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1D status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1d_z_p_bi cclass=ldnf1d_z_p_bi_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ldnf1h_z_p_bi_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0101 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1H status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1h_z_p_bi cclass=ldnf1h_z_p_bi_u16',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldnf1h_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0110 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1H status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1h_z_p_bi cclass=ldnf1h_z_p_bi_u32',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldnf1h_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0111 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1H status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1h_z_p_bi cclass=ldnf1h_z_p_bi_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldnf1sb_z_p_bi_s16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1110 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1SB status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1sb_z_p_bi cclass=ldnf1sb_z_p_bi_s16',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldnf1sb_z_p_bi_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1101 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1SB status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1sb_z_p_bi cclass=ldnf1sb_z_p_bi_s32',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldnf1sb_z_p_bi_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1100 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1SB status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1sb_z_p_bi cclass=ldnf1sb_z_p_bi_s64',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldnf1sh_z_p_bi_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1001 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1SH status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1sh_z_p_bi cclass=ldnf1sh_z_p_bi_s32',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldnf1sh_z_p_bi_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1000 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1SH status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1sh_z_p_bi cclass=ldnf1sh_z_p_bi_s64',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldnf1sw_z_p_bi_s64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 dtype=0b0100 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1SW status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1sw_z_p_bi cclass=ldnf1sw_z_p_bi_s64',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING ldnf1w_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1010 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1W status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1w_z_p_bi cclass=ldnf1w_z_p_bi_u32',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldnf1w_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1011 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1W status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1w_z_p_bi cclass=ldnf1w_z_p_bi_u64',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|UNSIGNED|LOAD',
};

ENCODING ldnt1b_z_p_ar_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 msz=0b00 ig1=0b00 Rm:u=0bxxxxx ig2=0b10 U=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1B status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gldnt_vs page=ldnt1b_z_p_ar cclass=ldnt1b_z_p_ar_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|UNSIGNED|LOAD',
};

ENCODING ldnt1b_z_p_ar_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b00 Rm:u=0bxxxxx ig2=0b1 U=0b1 ig3=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1B status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gldnt_vs page=ldnt1b_z_p_ar cclass=ldnt1b_z_p_ar_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|UNSIGNED|LOAD',
};

ENCODING ldnt1b_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 ig1=0b000 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnt_si page=ldnt1b_z_p_bi cclass=ldnt1b_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|LOAD',
};

ENCODING ldnt1b_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnt_ss page=ldnt1b_z_p_br cclass=ldnt1b_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|LOAD',
};

ENCODING ldnt1d_z_p_ar_d_64_unscaled => {
  name       => 'SVE2',
  diagram    => 'ig0=0b1100010 msz=0b11 ig1=0b00 Rm:u=0bxxxxx ig2=0b1 U=0b1 ig3=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1D status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gldnt_vs page=ldnt1d_z_p_ar cclass=ldnt1d_z_p_ar_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|UNSIGNED|LOAD',
};

ENCODING ldnt1d_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 ig1=0b000 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnt_si page=ldnt1d_z_p_bi cclass=ldnt1d_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|LOAD',
};

ENCODING ldnt1d_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnt_ss page=ldnt1d_z_p_br cclass=ldnt1d_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|LOAD',
};

ENCODING ldnt1h_z_p_ar_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 msz=0b01 ig1=0b00 Rm:u=0bxxxxx ig2=0b10 U=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1H status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gldnt_vs page=ldnt1h_z_p_ar cclass=ldnt1h_z_p_ar_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|UNSIGNED|LOAD',
};

ENCODING ldnt1h_z_p_ar_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b00 Rm:u=0bxxxxx ig2=0b1 U=0b1 ig3=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1H status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gldnt_vs page=ldnt1h_z_p_ar cclass=ldnt1h_z_p_ar_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|UNSIGNED|LOAD',
};

ENCODING ldnt1h_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 ig1=0b000 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnt_si page=ldnt1h_z_p_bi cclass=ldnt1h_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|LOAD',
};

ENCODING ldnt1h_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnt_ss page=ldnt1h_z_p_br cclass=ldnt1h_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|LOAD',
};

ENCODING ldnt1sb_z_p_ar_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 msz=0b00 ig1=0b00 Rm:u=0bxxxxx ig2=0b10 U=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1SB status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gldnt_vs page=ldnt1sb_z_p_ar cclass=ldnt1sb_z_p_ar_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|SIGNED|LOAD',
};

ENCODING ldnt1sb_z_p_ar_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b00 Rm:u=0bxxxxx ig2=0b1 U=0b0 ig3=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1SB status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gldnt_vs page=ldnt1sb_z_p_ar cclass=ldnt1sb_z_p_ar_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|SIGNED|LOAD',
};

ENCODING ldnt1sh_z_p_ar_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 msz=0b01 ig1=0b00 Rm:u=0bxxxxx ig2=0b10 U=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1SH status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gldnt_vs page=ldnt1sh_z_p_ar cclass=ldnt1sh_z_p_ar_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|SIGNED|LOAD',
};

ENCODING ldnt1sh_z_p_ar_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b00 Rm:u=0bxxxxx ig2=0b1 U=0b0 ig3=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1SH status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gldnt_vs page=ldnt1sh_z_p_ar cclass=ldnt1sh_z_p_ar_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|SIGNED|LOAD',
};

ENCODING ldnt1sw_z_p_ar_d_64_unscaled => {
  name       => 'SVE2',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b00 Rm:u=0bxxxxx ig2=0b1 U=0b0 ig3=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1SW status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gldnt_vs page=ldnt1sw_z_p_ar cclass=ldnt1sw_z_p_ar_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|SIGNED|LOAD',
};

ENCODING ldnt1w_z_p_ar_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 msz=0b10 ig1=0b00 Rm:u=0bxxxxx ig2=0b10 U=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1W status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gldnt_vs page=ldnt1w_z_p_ar cclass=ldnt1w_z_p_ar_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|UNSIGNED|LOAD',
};

ENCODING ldnt1w_z_p_ar_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b00 Rm:u=0bxxxxx ig2=0b1 U=0b1 ig3=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1W status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gldnt_vs page=ldnt1w_z_p_ar cclass=ldnt1w_z_p_ar_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|UNSIGNED|LOAD',
};

ENCODING ldnt1w_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 ig1=0b000 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnt_si page=ldnt1w_z_p_bi cclass=ldnt1w_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|LOAD',
};

ENCODING ldnt1w_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnt_ss page=ldnt1w_z_p_br cclass=ldnt1w_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|LOAD',
};

ENCODING ldr_p_bi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010110 imm9h:u=0bxxxxxx ig1=0b000 imm9l:u=0bxxx Rn:u=0bxxxxx ig2=0b0 Pt:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_pfill page=ldr_p_bi cclass=ldr_p_bi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING ldr_z_bi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010110 imm9h:u=0bxxxxxx ig1=0b010 imm9l:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_fill page=ldr_z_bi cclass=ldr_z_bi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|LOAD',
};

ENCODING lsl_z_p_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b00 L=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=lsl_z_p_zi cclass=lsl_z_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|LOGICAL',
};

ENCODING lsl_z_p_zw_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 R=0b0 L=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_2 page=lsl_z_p_zw cclass=lsl_z_p_zw_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|LOGICAL',
};

ENCODING lsl_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 R=0b0 L=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_1 page=lsl_z_p_zz cclass=lsl_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|LOGICAL',
};

ENCODING lsl_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b1001 opc=0b11 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_shift iclass=sve_int_bin_cons_shift_b page=lsl_z_zi cclass=lsl_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|LOGICAL',
};

ENCODING lsl_z_zw_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b1000 opc=0b11 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_shift iclass=sve_int_bin_cons_shift_a page=lsl_z_zw cclass=lsl_z_zw_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|LOGICAL',
};

ENCODING lslr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 R=0b1 L=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSLR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_1 page=lslr_z_p_zz cclass=lslr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|LOGICAL',
};

ENCODING lsr_z_p_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b00 L=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=lsr_z_p_zi cclass=lsr_z_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|LOGICAL',
};

ENCODING lsr_z_p_zw_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 R=0b0 L=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_2 page=lsr_z_p_zw cclass=lsr_z_p_zw_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|LOGICAL',
};

ENCODING lsr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 R=0b0 L=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_1 page=lsr_z_p_zz cclass=lsr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|LOGICAL',
};

ENCODING lsr_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b1001 ig3=0b0 U=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_shift iclass=sve_int_bin_cons_shift_b page=lsr_z_zi cclass=lsr_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|LOGICAL',
};

ENCODING lsr_z_zw_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b1000 ig3=0b0 U=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_shift iclass=sve_int_bin_cons_shift_a page=lsr_z_zw cclass=lsr_z_zw_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|LOGICAL',
};

ENCODING lsrr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 R=0b1 L=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSRR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_1 page=lsrr_z_p_zz cclass=lsrr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|LOGICAL',
};

ENCODING mad_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b11 op=0b0 Pg:u=0bxxx Za:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MAD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_muladd_pred iclass=sve_int_mladdsub_vvv_pred page=mad_z_p_zzz cclass=mad_z_p_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC',
};

ENCODING match_p_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx op=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=MATCH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve iclass=sve_intx_match page=match_p_p_zz cclass=match_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE',
};

ENCODING mla_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b01 op=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_muladd_pred iclass=sve_int_mlas_vvv_pred page=mla_z_p_zzz cclass=mla_z_p_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC',
};

ENCODING mla_z_zzzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b00001 S=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=MLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_by_indexed_elem page=mla_z_zzzi cclass=mla_z_zzzi_h',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC',
};

ENCODING mla_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00001 S=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=MLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_by_indexed_elem page=mla_z_zzzi cclass=mla_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC',
};

ENCODING mla_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b00001 S=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=MLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_by_indexed_elem page=mla_z_zzzi cclass=mla_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC',
};

ENCODING mls_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b01 op=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_muladd_pred iclass=sve_int_mlas_vvv_pred page=mls_z_p_zzz cclass=mls_z_p_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC',
};

ENCODING mls_z_zzzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b00001 S=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=MLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_by_indexed_elem page=mls_z_zzzi cclass=mls_z_zzzi_h',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC',
};

ENCODING mls_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00001 S=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=MLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_by_indexed_elem page=mls_z_zzzi cclass=mls_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC',
};

ENCODING mls_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b00001 S=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=MLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_by_indexed_elem page=mls_z_zzzi cclass=mls_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC',
};

ENCODING movprfx_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 opc=0b00 M:u=0bx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MOVPRFX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_movprfx_pred page=movprfx_z_p_z cclass=movprfx_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
};

ENCODING movprfx_z_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 opc=0b00 ig1=0b1 opc2=0b00000 ig2=0b101111 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MOVPRFX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_misc iclass=sve_int_bin_cons_misc_0_d page=movprfx_z_z cclass=movprfx_z_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
};

ENCODING msb_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b11 op=0b1 Pg:u=0bxxx Za:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MSB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_muladd_pred iclass=sve_int_mladdsub_vvv_pred page=msb_z_p_zzz cclass=msb_z_p_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC',
};

ENCODING mul_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0100 H=0b0 U=0b0 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_2 page=mul_z_p_zz cclass=mul_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING mul_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b110 opc=0b000 ig2=0b11 o2=0b0 imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm2 page=mul_z_zi cclass=mul_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING mul_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0110 opc=0b00 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=MUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_arit_b iclass=sve_int_mul_b page=mul_z_zz cclass=mul_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING mul_z_zzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b111110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=MUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_by_indexed_elem page=mul_z_zzi cclass=mul_z_zzi_h',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING mul_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b111110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=MUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_by_indexed_elem page=mul_z_zzi cclass=mul_z_zzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING mul_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b111110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=MUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_by_indexed_elem page=mul_z_zzi cclass=mul_z_zzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING nand_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=NAND status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=nand_p_p_pp cclass=nand_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|BITWISE',
};

ENCODING nands_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=NANDS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=nand_p_p_pp cclass=nands_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|BITWISE',
};

ENCODING nbsl_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 opc=0b11 ig1=0b1 Zm:u=0bxxxxx ig2=0b00111 o2=0b1 Zk:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=NBSL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_tern_log page=nbsl_z_zzz cclass=nbsl_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SELECTION',
};

ENCODING neg_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 opc=0b111 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=NEG status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_0 page=neg_z_p_z cclass=neg_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING nmatch_p_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx op=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=NMATCH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve iclass=sve_intx_match page=nmatch_p_p_zz cclass=nmatch_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE',
};

ENCODING nor_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=NOR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=nor_p_p_pp cclass=nor_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|BITWISE',
};

ENCODING nors_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=NORS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=nor_p_p_pp cclass=nors_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|BITWISE',
};

ENCODING not_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b110 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=NOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_1 page=not_z_p_z cclass=not_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING orn_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=ORN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=orn_p_p_pp cclass=orn_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|BITWISE',
};

ENCODING orns_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=ORNS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=orn_p_p_pp cclass=orns_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|BITWISE',
};

ENCODING orr_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S:u=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=orr_p_p_pp cclass=orr_p_p_pp_orr_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|BITWISE',
};

ENCODING orrs_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S:u=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=ORRS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=orr_p_p_pp cclass=orr_p_p_pp_orrs_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|BITWISE',
};

ENCODING orr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b000 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_log page=orr_z_p_zz cclass=orr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING orr_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 opc=0b00 ig1=0b0000 imm13:u=0bxxxxxxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_maskimm iclass=sve_int_log_imm page=orr_z_zi cclass=orr_z_zi_orr_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING orr_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 opc=0b01 ig1=0b1 Zm:u=0bxxxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_bin_cons_log page=orr_z_zz cclass=orr_z_zz_orr_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING orv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b000 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ORV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_2 page=orv_r_p_z cclass=orv_r_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
};

ENCODING pfalse_p_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b011000111001 ig2=0b000000 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PFALSE status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_pfalse page=pfalse_p cclass=pfalse_p_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING pfirst_p_p_p_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b1 ig1=0b011000110000 ig2=0b0 Pg:u=0bxxxx ig3=0b0 Pdn:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PFIRST status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_pfirst page=pfirst_p_p_p cclass=pfirst_p_p_p_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE',
};

ENCODING pmul_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 Zm:u=0bxxxxx ig2=0b0110 opc=0b01 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=PMUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_arit_b iclass=sve_int_mul_b page=pmul_z_zz cclass=pmul_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|POLYNOMIAL',
};

ENCODING pmullb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op=0b0 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=PMULLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_mul_long page=pmullb_z_zz cclass=pmullb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|POLYNOMIAL',
};

ENCODING pmullt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op=0b0 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=PMULLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_mul_long page=pmullt_z_zz cclass=pmullt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|POLYNOMIAL',
};

ENCODING pnext_p_p_p_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b011001110001 ig2=0b0 Pg:u=0bxxxx ig3=0b0 Pdn:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PNEXT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_pnext page=pnext_p_p_p cclass=pnext_p_p_p_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE',
};

ENCODING prfb_i_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b00 ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-elem-type=32-elem sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_prfm_vi page=prfb_i_p_ai cclass=prfb_i_p_ai_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfb_i_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-elem-type=64-elem sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_vi page=prfb_i_p_ai cclass=prfb_i_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfb_i_p_bi_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010111 imm6:u=0bxxxxxx ig1=0b0 msz=0b00 Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_prfm_si page=prfb_i_p_bi cclass=prfb_i_p_bi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfb_i_p_br_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010 msz=0b00 ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_prfm_ss page=prfb_i_p_br cclass=prfb_i_p_br_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfb_i_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz=0b00 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-esize=esize-byte sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_prfm_sv page=prfb_i_p_bz cclass=prfb_i_p_bz_s_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfb_i_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b110001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz=0b00 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-esize=esize-byte sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_sv page=prfb_i_p_bz cclass=prfb_i_p_bz_d_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfb_i_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b11000100011 Zm:u=0bxxxxx ig1=0b1 msz=0b00 Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-esize=esize-byte sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_sv2 page=prfb_i_p_bz cclass=prfb_i_p_bz_d_64_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfd_i_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b11 ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-elem-type=32-elem sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_prfm_vi page=prfd_i_p_ai cclass=prfd_i_p_ai_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfd_i_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b11 ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-elem-type=64-elem sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_vi page=prfd_i_p_ai cclass=prfd_i_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfd_i_p_bi_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010111 imm6:u=0bxxxxxx ig1=0b0 msz=0b11 Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_prfm_si page=prfd_i_p_bi cclass=prfd_i_p_bi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfd_i_p_br_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010 msz=0b11 ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_prfm_ss page=prfd_i_p_br cclass=prfd_i_p_br_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfd_i_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz=0b11 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-esize=esize-doubleword sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_prfm_sv page=prfd_i_p_bz cclass=prfd_i_p_bz_s_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfd_i_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b110001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz=0b11 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-esize=esize-doubleword sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_sv page=prfd_i_p_bz cclass=prfd_i_p_bz_d_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfd_i_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b11000100011 Zm:u=0bxxxxx ig1=0b1 msz=0b11 Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-esize=esize-doubleword sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_sv2 page=prfd_i_p_bz cclass=prfd_i_p_bz_d_64_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfh_i_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b01 ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-elem-type=32-elem sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_prfm_vi page=prfh_i_p_ai cclass=prfh_i_p_ai_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfh_i_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-elem-type=64-elem sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_vi page=prfh_i_p_ai cclass=prfh_i_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfh_i_p_bi_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010111 imm6:u=0bxxxxxx ig1=0b0 msz=0b01 Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_prfm_si page=prfh_i_p_bi cclass=prfh_i_p_bi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfh_i_p_br_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010 msz=0b01 ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_prfm_ss page=prfh_i_p_br cclass=prfh_i_p_br_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfh_i_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz=0b01 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-esize=esize-halfword sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_prfm_sv page=prfh_i_p_bz cclass=prfh_i_p_bz_s_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfh_i_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b110001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz=0b01 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-esize=esize-halfword sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_sv page=prfh_i_p_bz cclass=prfh_i_p_bz_d_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfh_i_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b11000100011 Zm:u=0bxxxxx ig1=0b1 msz=0b01 Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-esize=esize-halfword sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_sv2 page=prfh_i_p_bz cclass=prfh_i_p_bz_d_64_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfw_i_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b10 ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-elem-type=32-elem sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_prfm_vi page=prfw_i_p_ai cclass=prfw_i_p_ai_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfw_i_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-elem-type=64-elem sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_vi page=prfw_i_p_ai cclass=prfw_i_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfw_i_p_bi_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010111 imm6:u=0bxxxxxx ig1=0b0 msz=0b10 Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_prfm_si page=prfw_i_p_bi cclass=prfw_i_p_bi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfw_i_p_br_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010 msz=0b10 ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_prfm_ss page=prfw_i_p_br cclass=prfw_i_p_br_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfw_i_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz=0b10 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-esize=esize-word sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_prfm_sv page=prfw_i_p_bz cclass=prfw_i_p_bz_s_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfw_i_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b110001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz=0b10 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-esize=esize-word sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_sv page=prfw_i_p_bz cclass=prfw_i_p_bz_d_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING prfw_i_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b11000100011 Zm:u=0bxxxxx ig1=0b1 msz=0b10 Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-esize=esize-word sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_sv2 page=prfw_i_p_bz cclass=prfw_i_p_bz_d_64_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|PREFETCH',
};

ENCODING ptest_p_p_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00 ig1=0b100101 op=0b0 S=0b1 ig2=0b01 ig3=0b000011 Pg:u=0bxxxx ig4=0b0 Pn:u=0bxxxx ig5=0b0 opc2=0b0000',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PTEST status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_ptest page=ptest_p_p cclass=ptest_p_p_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE',
};

ENCODING ptrue_p_s_ => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b01100 S=0b0 ig2=0b111000 pattern:u=0bxxxxx ig3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=PTRUE status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_ptrue page=ptrue_p_s cclass=ptrue_p_s_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE',
};

ENCODING ptrues_p_s_ => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b01100 S=0b1 ig2=0b111000 pattern:u=0bxxxxx ig3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=PTRUES status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_ptrue page=ptrue_p_s cclass=ptrues_p_s_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE',
};

ENCODING punpkhi_p_p_ => {
  name       => 'High half',
  diagram    => 'ig0=0b000001010011000 H=0b1 ig1=0b010000 ig2=0b0 Pn:u=0bxxxx ig3=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PUNPKHI status=Green sve-half=sve-high-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_punpk page=punpkhi_p_p cclass=punpkhi_p_p_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING punpklo_p_p_ => {
  name       => 'Low half',
  diagram    => 'ig0=0b000001010011000 H=0b0 ig1=0b010000 ig2=0b0 Pn:u=0bxxxx ig3=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PUNPKLO status=Green sve-half=sve-low-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_punpk page=punpkhi_p_p cclass=punpklo_p_p_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING raddhnb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S=0b0 R=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RADDHNB status=Green',
  docvars2   => 'rounding=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_arith_narrow page=raddhnb_z_zz cclass=raddhnb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|ROUNDING',
};

ENCODING raddhnt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S=0b0 R=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RADDHNT status=Green',
  docvars2   => 'rounding=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_arith_narrow page=raddhnt_z_zz cclass=raddhnt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|ROUNDING',
};

ENCODING rax1_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size=0b00 ig1=0b1 Zm:u=0bxxxxx ig2=0b11110 op=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RAX1 status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_crypto iclass=sve_crypto_binary_const page=rax1_z_zz cclass=rax1_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ROTATE_LEFT',
};

ENCODING rbit_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1001 opc=0b11 ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=RBIT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_rev page=rbit_z_p_z cclass=rbit_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MISC',
};

ENCODING rdffr_p_f_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b011001111100 ig2=0b000000 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=RDFFR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_rdffr_2 page=rdffr_p_f cclass=rdffr_p_f_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING rdffr_p_p_f_ => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b011000111100 ig2=0b0 Pg:u=0bxxxx ig3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=RDFFR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_rdffr page=rdffr_p_p_f cclass=rdffr_p_p_f_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE',
};

ENCODING rdffrs_p_p_f_ => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b1 ig1=0b011000111100 ig2=0b0 Pg:u=0bxxxx ig3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=RDFFRS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_rdffr page=rdffr_p_p_f cclass=rdffrs_p_p_f_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE',
};

ENCODING rdvl_r_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b000001001 op=0b0 ig1=0b1 opc2=0b11111 ig2=0b01010 imm6:u=0bxxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=RDVL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_alloca iclass=sve_int_read_vl_a page=rdvl_r_i cclass=rdvl_r_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING rev_p_p_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b110100010000 ig2=0b0 Pn:u=0bxxxx ig3=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=REV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_reverse_p page=rev_p_p cclass=rev_p_p_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MISC',
};

ENCODING rev_z_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b111000001110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=REV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_reverse_z page=rev_z_z cclass=rev_z_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MISC',
};

ENCODING revb_z_z_ => {
  name       => 'Byte',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1001 opc=0b00 ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=REVB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_rev page=revb_z_z cclass=revb_z_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MISC',
};

ENCODING revh_z_z_ => {
  name       => 'Halfword',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1001 opc=0b01 ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=REVH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_rev page=revb_z_z cclass=revh_z_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MISC',
};

ENCODING revw_z_z_ => {
  name       => 'Word',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1001 opc=0b10 ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=REVW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_rev page=revb_z_z cclass=revw_z_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MISC',
};

ENCODING rshrnb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b0 U=0b1 R=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RSHRNB status=Green',
  docvars2   => 'rounding=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=rshrnb_z_zi cclass=rshrnb_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|ROUNDING',
};

ENCODING rshrnt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b0 U=0b1 R=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RSHRNT status=Green',
  docvars2   => 'rounding=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=rshrnt_z_zi cclass=rshrnt_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|ROUNDING',
};

ENCODING rsubhnb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S=0b1 R=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RSUBHNB status=Green',
  docvars2   => 'rounding=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_arith_narrow page=rsubhnb_z_zz cclass=rsubhnb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|ROUNDING',
};

ENCODING rsubhnt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S=0b1 R=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RSUBHNT status=Green',
  docvars2   => 'rounding=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_arith_narrow page=rsubhnt_z_zz cclass=rsubhnt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|ROUNDING',
};

ENCODING saba_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b11111 U=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SABA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_aba page=saba_z_zzz cclass=saba_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|SIGNED',
};

ENCODING sabalb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1100 U=0b0 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SABALB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_aba_long page=sabalb_z_zzz cclass=sabalb_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|SIGNED',
};

ENCODING sabalt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1100 U=0b0 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SABALT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_aba_long page=sabalt_z_zzz cclass=sabalt_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|SIGNED',
};

ENCODING sabd_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b10 U=0b0 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SABD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_1 page=sabd_z_p_zz cclass=sabd_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|SIGNED',
};

ENCODING sabdlb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b1 S=0b1 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SABDLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=sabdlb_z_zz cclass=sabdlb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|SIGNED',
};

ENCODING sabdlt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b1 S=0b1 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SABDLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=sabdlt_z_zz cclass=sabdlt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|SIGNED',
};

ENCODING sadalp_z_p_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00010 U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADALP status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_accumulate_long_pairs page=sadalp_z_p_z cclass=sadalp_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SIGNED',
};

ENCODING saddlb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b0 S=0b0 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADDLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=saddlb_z_zz cclass=saddlb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING saddlbt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1000 S=0b0 tb=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADDLBT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_clong page=saddlbt_z_zz cclass=saddlbt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING saddlt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b0 S=0b0 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADDLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=saddlt_z_zz cclass=saddlt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING saddv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b000 opc=0b00 U=0b0 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SADDV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_0 page=saddv_r_p_z cclass=saddv_r_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING saddwb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b0 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADDWB status=Green',
  docvars2   => 'shape=wide',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_wide page=saddwb_z_zz cclass=saddwb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING saddwt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b0 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADDWT status=Green',
  docvars2   => 'shape=wide',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_wide page=saddwt_z_zz cclass=saddwt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING sbclb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 ig1=0b1 sz:u=0bx ig2=0b0 Zm:u=0bxxxxx ig3=0b11010 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SBCLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_adc_long page=sbclb_z_zzz cclass=sbclb_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|CARRY',
};

ENCODING sbclt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 ig1=0b1 sz:u=0bx ig2=0b0 Zm:u=0bxxxxx ig3=0b11010 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SBCLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_adc_long page=sbclt_z_zzz cclass=sbclt_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|CARRY',
};

ENCODING scvtf_z_p_z_h2fp16 => {
  name       => '16-bit to half-precision',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b010 opc2=0b01 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=16-to-half instr-class=sve isa=A64 mnemonic=SCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=scvtf_z_p_z cclass=scvtf_z_p_z_h2fp16',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|SIGNED',
};

ENCODING scvtf_z_p_z_w2fp16 => {
  name       => '32-bit to half-precision',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b010 opc2=0b10 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=32-to-half instr-class=sve isa=A64 mnemonic=SCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=scvtf_z_p_z cclass=scvtf_z_p_z_w2fp16',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|SIGNED',
};

ENCODING scvtf_z_p_z_w2s => {
  name       => '32-bit to single-precision',
  diagram    => 'ig0=0b01100101 opc=0b10 ig1=0b010 opc2=0b10 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=32-to-single instr-class=sve isa=A64 mnemonic=SCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=scvtf_z_p_z cclass=scvtf_z_p_z_w2s',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|SIGNED',
};

ENCODING scvtf_z_p_z_w2d => {
  name       => '32-bit to double-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b010 opc2=0b00 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=32-to-double instr-class=sve isa=A64 mnemonic=SCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=scvtf_z_p_z cclass=scvtf_z_p_z_w2d',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|SIGNED',
};

ENCODING scvtf_z_p_z_x2fp16 => {
  name       => '64-bit to half-precision',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b010 opc2=0b11 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=64-to-half instr-class=sve isa=A64 mnemonic=SCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=scvtf_z_p_z cclass=scvtf_z_p_z_x2fp16',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|SIGNED',
};

ENCODING scvtf_z_p_z_x2s => {
  name       => '64-bit to single-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b010 opc2=0b10 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=64-to-single instr-class=sve isa=A64 mnemonic=SCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=scvtf_z_p_z cclass=scvtf_z_p_z_x2s',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|SIGNED',
};

ENCODING scvtf_z_p_z_x2d => {
  name       => '64-bit to double-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b010 opc2=0b11 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=64-to-double instr-class=sve isa=A64 mnemonic=SCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=scvtf_z_p_z cclass=scvtf_z_p_z_x2d',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|SIGNED',
};

ENCODING sdiv_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0101 R=0b0 U=0b0 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SDIV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_div page=sdiv_z_p_zz cclass=sdiv_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING sdivr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0101 R=0b1 U=0b0 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SDIVR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_div page=sdivr_z_p_zz cclass=sdivr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING sdot_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00000 U=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_dot page=sdot_z_zzz cclass=sdot_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DOT_PRODUCT|SIGNED',
};

ENCODING sdot_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00000 U=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve isa=A64 mnemonic=SDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_dot_by_indexed_elem page=sdot_z_zzzi cclass=sdot_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DOT_PRODUCT|SIGNED',
};

ENCODING sdot_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b00000 U=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve isa=A64 mnemonic=SDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_dot_by_indexed_elem page=sdot_z_zzzi cclass=sdot_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DOT_PRODUCT|SIGNED',
};

ENCODING sel_p_p_pp_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SEL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=sel_p_p_pp cclass=sel_p_p_pp_sel_p_p_pp_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SELECTION|CONDITIONALLY',
};

ENCODING sel_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b11 Pg:u=0bxxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SEL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_sel_vvv page=sel_z_p_zz cclass=sel_z_p_zz_sel_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SELECTION|CONDITIONALLY',
};

ENCODING setffr_f_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00 ig1=0b100101 opc=0b00 ig2=0b10 ig3=0b1100100100 ig4=0b000000 ig5=0b0 ig6=0b0 ig7=0b00',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SETFFR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_wrffr iclass=sve_int_setffr page=setffr_f cclass=setffr_f_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING shadd_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 R=0b0 S=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SHADD status=Green',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary page=shadd_z_p_zz cclass=shadd_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|HALVING|SIGNED',
};

ENCODING shrnb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b0 U=0b1 R=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SHRNB status=Green',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=shrnb_z_zi cclass=shrnb_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT',
};

ENCODING shrnt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b0 U=0b1 R=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SHRNT status=Green',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=shrnt_z_zi cclass=shrnt_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT',
};

ENCODING shsub_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 R=0b0 S=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SHSUB status=Green',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary page=shsub_z_p_zz cclass=shsub_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|HALVING|SIGNED',
};

ENCODING shsubr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 R=0b1 S=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SHSUBR status=Green',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary page=shsubr_z_p_zz cclass=shsubr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|HALVING|SIGNED',
};

ENCODING sli_z_zzi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 tszh:u=0bxx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b11110 op=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SLI status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_shift_insert page=sli_z_zzi cclass=sli_z_zzi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|INSERTION',
};

ENCODING sm4e_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size=0b00 ig1=0b10001 op=0b1 ig2=0b11100 o2=0b0 Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SM4E status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_crypto iclass=sve_crypto_binary_dest page=sm4e_z_zz cclass=sm4e_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CRYPTO',
};

ENCODING sm4ekey_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size=0b00 ig1=0b1 Zm:u=0bxxxxx ig2=0b11110 op=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SM4EKEY status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_crypto iclass=sve_crypto_binary_const page=sm4ekey_z_zz cclass=sm4ekey_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CRYPTO',
};

ENCODING smax_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b00 U=0b0 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMAX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_1 page=smax_z_p_zz cclass=smax_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX|SIGNED',
};

ENCODING smax_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b101 ig2=0b00 U=0b0 ig3=0b11 o2=0b0 imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMAX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm1 page=smax_z_zi cclass=smax_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX|SIGNED',
};

ENCODING smaxp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 opc=0b10 U=0b0 ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMAXP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_arith_binary_pairs page=smaxp_z_p_zz cclass=smaxp_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX|SIGNED',
};

ENCODING smaxv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b00 U=0b0 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMAXV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_1 page=smaxv_r_p_z cclass=smaxv_r_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX|SIGNED',
};

ENCODING smin_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b01 U=0b0 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMIN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_1 page=smin_z_p_zz cclass=smin_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX|SIGNED',
};

ENCODING smin_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b101 ig2=0b01 U=0b0 ig3=0b11 o2=0b0 imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMIN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm1 page=smin_z_zi cclass=smin_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX|SIGNED',
};

ENCODING sminp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 opc=0b11 U=0b0 ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMINP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_arith_binary_pairs page=sminp_z_p_zz cclass=sminp_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX|SIGNED',
};

ENCODING sminv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b01 U=0b0 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMINV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_1 page=sminv_r_p_z cclass=sminv_r_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX|SIGNED',
};

ENCODING smlalb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b0 U=0b0 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMLALB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mlal_long page=smlalb_z_zzz cclass=smlalb_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SIGNED',
};

ENCODING smlalb_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b10 S=0b0 U=0b0 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SMLALB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=smlalb_z_zzzi cclass=smlalb_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SIGNED',
};

ENCODING smlalb_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b10 S=0b0 U=0b0 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SMLALB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=smlalb_z_zzzi cclass=smlalb_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SIGNED',
};

ENCODING smlalt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b0 U=0b0 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMLALT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mlal_long page=smlalt_z_zzz cclass=smlalt_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SIGNED',
};

ENCODING smlalt_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b10 S=0b0 U=0b0 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SMLALT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=smlalt_z_zzzi cclass=smlalt_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SIGNED',
};

ENCODING smlalt_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b10 S=0b0 U=0b0 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SMLALT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=smlalt_z_zzzi cclass=smlalt_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SIGNED',
};

ENCODING smlslb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b1 U=0b0 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMLSLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mlal_long page=smlslb_z_zzz cclass=smlslb_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SIGNED',
};

ENCODING smlslb_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b10 S=0b1 U=0b0 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SMLSLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=smlslb_z_zzzi cclass=smlslb_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SIGNED',
};

ENCODING smlslb_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b10 S=0b1 U=0b0 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SMLSLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=smlslb_z_zzzi cclass=smlslb_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SIGNED',
};

ENCODING smlslt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b1 U=0b0 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMLSLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mlal_long page=smlslt_z_zzz cclass=smlslt_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SIGNED',
};

ENCODING smlslt_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b10 S=0b1 U=0b0 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SMLSLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=smlslt_z_zzzi cclass=smlslt_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SIGNED',
};

ENCODING smlslt_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b10 S=0b1 U=0b0 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SMLSLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=smlslt_z_zzzi cclass=smlslt_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SIGNED',
};

ENCODING smmla_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01000101 uns=0b00 ig1=0b0 Zm:u=0bxxxxx ig2=0b100110 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_mmla page=smmla_z_zzz cclass=smmla_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SIGNED',
};

ENCODING smulh_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0100 H=0b1 U=0b0 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMULH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_2 page=smulh_z_p_zz cclass=smulh_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING smulh_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0110 ig3=0b1 U=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMULH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_arit_b iclass=sve_int_mul_b page=smulh_z_zz cclass=smulh_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING smullb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op=0b1 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMULLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_mul_long page=smullb_z_zz cclass=smullb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING smullb_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b110 U=0b0 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SMULLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_long_by_indexed_elem page=smullb_z_zzi cclass=smullb_z_zzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING smullb_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b110 U=0b0 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SMULLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_long_by_indexed_elem page=smullb_z_zzi cclass=smullb_z_zzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING smullt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op=0b1 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMULLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_mul_long page=smullt_z_zz cclass=smullt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING smullt_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b110 U=0b0 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SMULLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_long_by_indexed_elem page=smullt_z_zzi cclass=smullt_z_zzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING smullt_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b110 U=0b0 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SMULLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_long_by_indexed_elem page=smullt_z_zzi cclass=smullt_z_zzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING splice_z_p_zz_con => {
  name       => 'Constructive',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b101101100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SPLICE status=Green sve-dest_const=sve-const',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_intx_perm_splice page=splice_z_p_zz cclass=splice_z_p_zz_con',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MISC',
};

ENCODING splice_z_p_zz_des => {
  name       => 'Destructive',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b101100100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SPLICE status=Green sve-dest_const=sve-dest',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_splice page=splice_z_p_zz cclass=splice_z_p_zz_des',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MISC',
};

ENCODING sqabs_z_p_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 ig2=0b0 opc=0b00 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQABS status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_unary page=sqabs_z_p_z cclass=sqabs_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|SATURATING|SIGNED',
};

ENCODING sqadd_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b011 op=0b0 S=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQADD status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary_sat page=sqadd_z_p_zz cclass=sqadd_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqadd_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 ig2=0b10 U=0b0 ig3=0b11 sh:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQADD status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm0 page=sqadd_z_zi cclass=sqadd_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqadd_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b10 U=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQADD status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve iclass=sve_int_bin_cons_arit_0 page=sqadd_z_zz cclass=sqadd_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqcadd_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b00000 op=0b1 ig2=0b11011 rot:u=0bx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQCADD status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_acc iclass=sve_intx_cadd page=sqcadd_z_zz cclass=sqcadd_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPLEX|SATURATING',
};

ENCODING sqdecb_r_rs_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECB-32-fsreg instr-class=sve isa=A64 mnemonic=SQDECB reg-type=32-fsreg status=Green sve-esize=esize-byte',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqdecb_r_rs cclass=sqdecb_r_rs_sx',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqdecb_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECB-64-fsreg instr-class=sve isa=A64 mnemonic=SQDECB reg-type=64-fsreg status=Green sve-esize=esize-byte',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqdecb_r_rs cclass=sqdecb_r_rs_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqdecd_r_rs_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECD-32-fsreg instr-class=sve isa=A64 mnemonic=SQDECD reg-type=32-fsreg status=Green sve-esize=esize-doubleword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqdecd_r_rs cclass=sqdecd_r_rs_sx',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqdecd_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECD-64-fsreg instr-class=sve isa=A64 mnemonic=SQDECD reg-type=64-fsreg status=Green sve-esize=esize-doubleword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqdecd_r_rs cclass=sqdecd_r_rs_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqdecd_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b1 U=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQDECD status=Green sve-esize=esize-doubleword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=sqdecd_z_zs cclass=sqdecd_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqdech_r_rs_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECH-32-fsreg instr-class=sve isa=A64 mnemonic=SQDECH reg-type=32-fsreg status=Green sve-esize=esize-halfword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqdech_r_rs cclass=sqdech_r_rs_sx',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqdech_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECH-64-fsreg instr-class=sve isa=A64 mnemonic=SQDECH reg-type=64-fsreg status=Green sve-esize=esize-halfword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqdech_r_rs cclass=sqdech_r_rs_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqdech_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b1 U=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQDECH status=Green sve-esize=esize-halfword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=sqdech_z_zs cclass=sqdech_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqdecp_r_p_r_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b1 U=0b0 ig2=0b10001 sf=0b0 op=0b0 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECP-32-fsreg instr-class=sve isa=A64 mnemonic=SQDECP reg-type=32-fsreg status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r_sat page=sqdecp_r_p_r cclass=sqdecp_r_p_r_sx',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqdecp_r_p_r_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b1 U=0b0 ig2=0b10001 sf=0b1 op=0b0 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECP-64-fsreg instr-class=sve isa=A64 mnemonic=SQDECP reg-type=64-fsreg status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r_sat page=sqdecp_r_p_r cclass=sqdecp_r_p_r_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqdecp_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b1 U=0b0 ig2=0b10000 opc=0b00 Pm:u=0bxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQDECP status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_v_sat page=sqdecp_z_p_z cclass=sqdecp_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqdecw_r_rs_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECW-32-fsreg instr-class=sve isa=A64 mnemonic=SQDECW reg-type=32-fsreg status=Green sve-esize=esize-word',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqdecw_r_rs cclass=sqdecw_r_rs_sx',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqdecw_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECW-64-fsreg instr-class=sve isa=A64 mnemonic=SQDECW reg-type=64-fsreg status=Green sve-esize=esize-word',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqdecw_r_rs cclass=sqdecw_r_rs_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqdecw_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b1 U=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQDECW status=Green sve-esize=esize-word',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=sqdecw_z_zs cclass=sqdecw_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqdmlalb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0110 S=0b0 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLALB status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_qdmlal_long page=sqdmlalb_z_zzz cclass=sqdmlalb_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmlalb_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b001 S=0b0 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQDMLALB status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmla_long_by_indexed_elem page=sqdmlalb_z_zzzi cclass=sqdmlalb_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmlalb_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b001 S=0b0 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQDMLALB status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmla_long_by_indexed_elem page=sqdmlalb_z_zzzi cclass=sqdmlalb_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmlalbt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00001 S=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLALBT status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_qdmlalbt page=sqdmlalbt_z_zzz cclass=sqdmlalbt_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmlalt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0110 S=0b0 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLALT status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_qdmlal_long page=sqdmlalt_z_zzz cclass=sqdmlalt_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmlalt_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b001 S=0b0 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQDMLALT status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmla_long_by_indexed_elem page=sqdmlalt_z_zzzi cclass=sqdmlalt_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmlalt_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b001 S=0b0 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQDMLALT status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmla_long_by_indexed_elem page=sqdmlalt_z_zzzi cclass=sqdmlalt_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmlslb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0110 S=0b1 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLSLB status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_qdmlal_long page=sqdmlslb_z_zzz cclass=sqdmlslb_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmlslb_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b001 S=0b1 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQDMLSLB status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmla_long_by_indexed_elem page=sqdmlslb_z_zzzi cclass=sqdmlslb_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmlslb_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b001 S=0b1 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQDMLSLB status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmla_long_by_indexed_elem page=sqdmlslb_z_zzzi cclass=sqdmlslb_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmlslbt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00001 S=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLSLBT status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_qdmlalbt page=sqdmlslbt_z_zzz cclass=sqdmlslbt_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmlslt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0110 S=0b1 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLSLT status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_qdmlal_long page=sqdmlslt_z_zzz cclass=sqdmlslt_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmlslt_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b001 S=0b1 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQDMLSLT status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmla_long_by_indexed_elem page=sqdmlslt_z_zzzi cclass=sqdmlslt_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmlslt_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b001 S=0b1 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQDMLSLT status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmla_long_by_indexed_elem page=sqdmlslt_z_zzzi cclass=sqdmlslt_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmulh_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b01110 R=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMULH status=Green',
  docvars2   => 'doubling=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_int_unpred_arit_b iclass=sve_int_sqdmulh page=sqdmulh_z_zz cclass=sqdmulh_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmulh_z_zzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b11110 R=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=SQDMULH status=Green',
  docvars2   => 'doubling=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmulh_by_indexed_elem page=sqdmulh_z_zzi cclass=sqdmulh_z_zzi_h',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmulh_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b11110 R=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQDMULH status=Green',
  docvars2   => 'doubling=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmulh_by_indexed_elem page=sqdmulh_z_zzi cclass=sqdmulh_z_zzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmulh_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b11110 R=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQDMULH status=Green',
  docvars2   => 'doubling=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmulh_by_indexed_elem page=sqdmulh_z_zzi cclass=sqdmulh_z_zzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmullb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op=0b0 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMULLB status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_mul_long page=sqdmullb_z_zz cclass=sqdmullb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmullb_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b1110 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQDMULLB status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmul_long_by_indexed_elem page=sqdmullb_z_zzi cclass=sqdmullb_z_zzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmullb_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b1110 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQDMULLB status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmul_long_by_indexed_elem page=sqdmullb_z_zzi cclass=sqdmullb_z_zzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmullt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op=0b0 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMULLT status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_mul_long page=sqdmullt_z_zz cclass=sqdmullt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmullt_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b1110 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQDMULLT status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmul_long_by_indexed_elem page=sqdmullt_z_zzi cclass=sqdmullt_z_zzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqdmullt_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b1110 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQDMULLT status=Green',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmul_long_by_indexed_elem page=sqdmullt_z_zzi cclass=sqdmullt_z_zzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqincb_r_rs_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCB-32-fsreg instr-class=sve isa=A64 mnemonic=SQINCB reg-type=32-fsreg status=Green sve-esize=esize-byte',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqincb_r_rs cclass=sqincb_r_rs_sx',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqincb_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCB-64-fsreg instr-class=sve isa=A64 mnemonic=SQINCB reg-type=64-fsreg status=Green sve-esize=esize-byte',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqincb_r_rs cclass=sqincb_r_rs_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqincd_r_rs_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCD-32-fsreg instr-class=sve isa=A64 mnemonic=SQINCD reg-type=32-fsreg status=Green sve-esize=esize-doubleword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqincd_r_rs cclass=sqincd_r_rs_sx',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqincd_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCD-64-fsreg instr-class=sve isa=A64 mnemonic=SQINCD reg-type=64-fsreg status=Green sve-esize=esize-doubleword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqincd_r_rs cclass=sqincd_r_rs_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqincd_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b0 U=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQINCD status=Green sve-esize=esize-doubleword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=sqincd_z_zs cclass=sqincd_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqinch_r_rs_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCH-32-fsreg instr-class=sve isa=A64 mnemonic=SQINCH reg-type=32-fsreg status=Green sve-esize=esize-halfword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqinch_r_rs cclass=sqinch_r_rs_sx',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqinch_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCH-64-fsreg instr-class=sve isa=A64 mnemonic=SQINCH reg-type=64-fsreg status=Green sve-esize=esize-halfword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqinch_r_rs cclass=sqinch_r_rs_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqinch_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b0 U=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQINCH status=Green sve-esize=esize-halfword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=sqinch_z_zs cclass=sqinch_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqincp_r_p_r_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b0 U=0b0 ig2=0b10001 sf=0b0 op=0b0 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCP-32-fsreg instr-class=sve isa=A64 mnemonic=SQINCP reg-type=32-fsreg status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r_sat page=sqincp_r_p_r cclass=sqincp_r_p_r_sx',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqincp_r_p_r_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b0 U=0b0 ig2=0b10001 sf=0b1 op=0b0 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCP-64-fsreg instr-class=sve isa=A64 mnemonic=SQINCP reg-type=64-fsreg status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r_sat page=sqincp_r_p_r cclass=sqincp_r_p_r_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqincp_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b0 U=0b0 ig2=0b10000 opc=0b00 Pm:u=0bxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQINCP status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_v_sat page=sqincp_z_p_z cclass=sqincp_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqincw_r_rs_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCW-32-fsreg instr-class=sve isa=A64 mnemonic=SQINCW reg-type=32-fsreg status=Green sve-esize=esize-word',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqincw_r_rs cclass=sqincw_r_rs_sx',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqincw_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCW-64-fsreg instr-class=sve isa=A64 mnemonic=SQINCW reg-type=64-fsreg status=Green sve-esize=esize-word',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqincw_r_rs cclass=sqincw_r_rs_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqincw_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b0 U=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQINCW status=Green sve-esize=esize-word',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=sqincw_z_zs cclass=sqincw_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|SIGNED',
};

ENCODING sqneg_z_p_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 ig2=0b0 opc=0b01 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQNEG status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_unary page=sqneg_z_p_z cclass=sqneg_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE|SATURATING|SIGNED',
};

ENCODING sqrdcmlah_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b001 op=0b1 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRDCMLAH status=Green',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_cmla page=sqrdcmlah_z_zzz cclass=sqrdcmlah_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPLEX|ROUNDING|SATURATING',
};

ENCODING sqrdcmlah_z_zzzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b0111 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=SQRDCMLAH status=Green',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qrdcmla_by_indexed_elem page=sqrdcmlah_z_zzzi cclass=sqrdcmlah_z_zzzi_h',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPLEX|ROUNDING|SATURATING',
};

ENCODING sqrdcmlah_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b0111 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQRDCMLAH status=Green',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qrdcmla_by_indexed_elem page=sqrdcmlah_z_zzzi cclass=sqrdcmlah_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPLEX|ROUNDING|SATURATING',
};

ENCODING sqrdmlah_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b01110 S=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRDMLAH status=Green',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_qrdmlah page=sqrdmlah_z_zzz cclass=sqrdmlah_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING sqrdmlah_z_zzzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b00010 S=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=SQRDMLAH status=Green',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qrdmlah_by_indexed_elem page=sqrdmlah_z_zzzi cclass=sqrdmlah_z_zzzi_h',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING sqrdmlah_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00010 S=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQRDMLAH status=Green',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qrdmlah_by_indexed_elem page=sqrdmlah_z_zzzi cclass=sqrdmlah_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING sqrdmlah_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b00010 S=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQRDMLAH status=Green',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qrdmlah_by_indexed_elem page=sqrdmlah_z_zzzi cclass=sqrdmlah_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING sqrdmlsh_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b01110 S=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRDMLSH status=Green',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_qrdmlah page=sqrdmlsh_z_zzz cclass=sqrdmlsh_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING sqrdmlsh_z_zzzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b00010 S=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=SQRDMLSH status=Green',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qrdmlah_by_indexed_elem page=sqrdmlsh_z_zzzi cclass=sqrdmlsh_z_zzzi_h',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING sqrdmlsh_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00010 S=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQRDMLSH status=Green',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qrdmlah_by_indexed_elem page=sqrdmlsh_z_zzzi cclass=sqrdmlsh_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING sqrdmlsh_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b00010 S=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQRDMLSH status=Green',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qrdmlah_by_indexed_elem page=sqrdmlsh_z_zzzi cclass=sqrdmlsh_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING sqrdmulh_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b01110 R=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRDMULH status=Green',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_int_unpred_arit_b iclass=sve_int_sqdmulh page=sqrdmulh_z_zz cclass=sqrdmulh_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING sqrdmulh_z_zzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b11110 R=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=SQRDMULH status=Green',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmulh_by_indexed_elem page=sqrdmulh_z_zzi cclass=sqrdmulh_z_zzi_h',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING sqrdmulh_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b11110 R=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQRDMULH status=Green',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmulh_by_indexed_elem page=sqrdmulh_z_zzi cclass=sqrdmulh_z_zzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING sqrdmulh_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b11110 R=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQRDMULH status=Green',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmulh_by_indexed_elem page=sqrdmulh_z_zzi cclass=sqrdmulh_z_zzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|ROUNDING|SATURATING|SIGNED',
};

ENCODING sqrshl_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 R=0b0 N=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHL status=Green',
  docvars2   => 'rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=sqrshl_z_p_zz cclass=sqrshl_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|ROUNDING|SATURATING|SIGNED',
};

ENCODING sqrshlr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 R=0b1 N=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHLR status=Green',
  docvars2   => 'rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=sqrshlr_z_p_zz cclass=sqrshlr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|ROUNDING|SATURATING|SIGNED',
};

ENCODING sqrshrnb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b1 U=0b0 R=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHRNB status=Green',
  docvars2   => 'rounding=1 saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=sqrshrnb_z_zi cclass=sqrshrnb_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|ROUNDING|SATURATING|SIGNED',
};

ENCODING sqrshrnt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b1 U=0b0 R=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHRNT status=Green',
  docvars2   => 'rounding=1 saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=sqrshrnt_z_zi cclass=sqrshrnt_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|ROUNDING|SATURATING|SIGNED',
};

ENCODING sqrshrunb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b0 U=0b0 R=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHRUNB status=Green',
  docvars2   => 'rounding=1 saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=sqrshrunb_z_zi cclass=sqrshrunb_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|ROUNDING|SATURATING',
};

ENCODING sqrshrunt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b0 U=0b0 R=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHRUNT status=Green',
  docvars2   => 'rounding=1 saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=sqrshrunt_z_zi cclass=sqrshrunt_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|ROUNDING|SATURATING',
};

ENCODING sqshl_z_p_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b01 L=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHL status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=sqshl_z_p_zi cclass=sqshl_z_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|SATURATING|SIGNED',
};

ENCODING sqshl_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 R=0b0 N=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHL status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=sqshl_z_p_zz cclass=sqshl_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|SATURATING|SIGNED',
};

ENCODING sqshlr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 R=0b1 N=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHLR status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=sqshlr_z_p_zz cclass=sqshlr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|SATURATING|SIGNED',
};

ENCODING sqshlu_z_p_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b11 L=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHLU status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=sqshlu_z_p_zi cclass=sqshlu_z_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|SATURATING',
};

ENCODING sqshrnb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b1 U=0b0 R=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHRNB status=Green',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=sqshrnb_z_zi cclass=sqshrnb_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|SATURATING|SIGNED',
};

ENCODING sqshrnt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b1 U=0b0 R=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHRNT status=Green',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=sqshrnt_z_zi cclass=sqshrnt_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|SATURATING|SIGNED',
};

ENCODING sqshrunb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b0 U=0b0 R=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHRUNB status=Green',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=sqshrunb_z_zi cclass=sqshrunb_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|SATURATING',
};

ENCODING sqshrunt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b0 U=0b0 R=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHRUNT status=Green',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=sqshrunt_z_zi cclass=sqshrunt_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|SATURATING',
};

ENCODING sqsub_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b011 op=0b0 S=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSUB status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary_sat page=sqsub_z_p_zz cclass=sqsub_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SATURATING|SIGNED',
};

ENCODING sqsub_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 ig2=0b11 U=0b0 ig3=0b11 sh:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQSUB status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm0 page=sqsub_z_zi cclass=sqsub_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqsub_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b11 U=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQSUB status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve iclass=sve_int_bin_cons_arit_0 page=sqsub_z_zz cclass=sqsub_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|SIGNED',
};

ENCODING sqsubr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b011 op=0b1 S=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSUBR status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary_sat page=sqsubr_z_p_zz cclass=sqsubr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SATURATING|SIGNED',
};

ENCODING sqxtnb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx ig2=0b000010 ig3=0b0 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQXTNB status=Green',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_extract_narrow page=sqxtnb_z_zz cclass=sqxtnb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION|SATURATING|SIGNED',
};

ENCODING sqxtnt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx ig2=0b000010 ig3=0b0 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQXTNT status=Green',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_extract_narrow page=sqxtnt_z_zz cclass=sqxtnt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION|SATURATING|SIGNED',
};

ENCODING sqxtunb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx ig2=0b000010 opc=0b10 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQXTUNB status=Green',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_extract_narrow page=sqxtunb_z_zz cclass=sqxtunb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION|SATURATING',
};

ENCODING sqxtunt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx ig2=0b000010 opc=0b10 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQXTUNT status=Green',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_extract_narrow page=sqxtunt_z_zz cclass=sqxtunt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION|SATURATING',
};

ENCODING srhadd_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 R=0b1 S=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRHADD status=Green',
  docvars2   => 'halving=1 rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary page=srhadd_z_p_zz cclass=srhadd_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|HALVING|ROUNDING|SIGNED',
};

ENCODING sri_z_zzi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 tszh:u=0bxx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b11110 op=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRI status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_shift_insert page=sri_z_zzi cclass=sri_z_zzi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|INSERTION',
};

ENCODING srshl_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b0 R=0b0 N=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRSHL status=Green',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=srshl_z_p_zz cclass=srshl_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|ROUNDING|SIGNED',
};

ENCODING srshlr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b0 R=0b1 N=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRSHLR status=Green',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=srshlr_z_p_zz cclass=srshlr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|ROUNDING|SIGNED',
};

ENCODING srshr_z_p_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b11 L=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRSHR status=Green',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=srshr_z_p_zi cclass=srshr_z_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|ROUNDING|SIGNED',
};

ENCODING srsra_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 tszh:u=0bxx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1110 R=0b1 U=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRSRA status=Green',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_acc iclass=sve_intx_sra page=srsra_z_zi cclass=srsra_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|CARITHMETIC|ROUNDING|SIGNED',
};

ENCODING sshllb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1010 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSHLLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_shift_long page=sshllb_z_zi cclass=sshllb_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|SIGNED',
};

ENCODING sshllt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1010 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSHLLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_shift_long page=sshllt_z_zi cclass=sshllt_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|SIGNED',
};

ENCODING ssra_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 tszh:u=0bxx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1110 R=0b0 U=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSRA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_sra page=ssra_z_zi cclass=ssra_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|CARITHMETIC|SIGNED',
};

ENCODING ssublb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b0 S=0b1 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=ssublb_z_zz cclass=ssublb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING ssublbt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1000 S=0b1 tb=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBLBT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_clong page=ssublbt_z_zz cclass=ssublbt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING ssublt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b0 S=0b1 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=ssublt_z_zz cclass=ssublt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING ssubltb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1000 S=0b1 tb=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBLTB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_clong page=ssubltb_z_zz cclass=ssubltb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING ssubwb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b1 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBWB status=Green',
  docvars2   => 'shape=wide',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_wide page=ssubwb_z_zz cclass=ssubwb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING ssubwt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b1 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBWT status=Green',
  docvars2   => 'shape=wide',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_wide page=ssubwt_z_zz cclass=ssubwt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SIGNED',
};

ENCODING st1b_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b11 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vi_b page=st1b_z_p_ai cclass=st1b_z_p_ai_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1b_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b10 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vi_a page=st1b_z_p_ai cclass=st1b_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1b_z_p_bi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 size:u=0bxx ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_cst_si page=st1b_z_p_bi cclass=st1b_z_p_bi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1b_z_p_br_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 ig1=0b00 size:u=0bxx Rm:u=0bxxxxx ig2=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_cs iclass=sve_mem_cst_ss page=st1b_z_p_br cclass=st1b_z_p_br_',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1b_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b00 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_vs_a page=st1b_z_p_bz cclass=st1b_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1b_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_vs_b page=st1b_z_p_bz cclass=st1b_z_p_bz_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1b_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b00 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vs2 page=st1b_z_p_bz cclass=st1b_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1d_z_p_ai_d => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 ig1=0b10 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vi_a page=st1d_z_p_ai cclass=st1d_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1d_z_p_bi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 size:u=0bxx ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_cst_si page=st1d_z_p_bi cclass=st1d_z_p_bi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1d_z_p_br_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 opc=0b111 o2=0b1 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_cs iclass=sve_mem_cst_ss page=st1d_z_p_br cclass=st1d_z_p_br_',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1d_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1110010 msz=0b11 ig1=0b01 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_sv_a page=st1d_z_p_bz cclass=st1d_z_p_bz_d_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1d_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b11 ig1=0b00 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_vs_a page=st1d_z_p_bz cclass=st1d_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1d_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1110010 msz=0b11 ig1=0b01 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_sv2 page=st1d_z_p_bz cclass=st1d_z_p_bz_d_64_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1d_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b11 ig1=0b00 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vs2 page=st1d_z_p_bz cclass=st1d_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1h_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b11 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vi_b page=st1h_z_p_ai cclass=st1h_z_p_ai_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1h_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b10 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vi_a page=st1h_z_p_ai cclass=st1h_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1h_z_p_bi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 size:u=0bxx ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_cst_si page=st1h_z_p_bi cclass=st1h_z_p_bi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1h_z_p_br_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 ig1=0b01 size:u=0bxx Rm:u=0bxxxxx ig2=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_cs iclass=sve_mem_cst_ss page=st1h_z_p_br cclass=st1h_z_p_br_',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1h_z_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_sv_b page=st1h_z_p_bz cclass=st1h_z_p_bz_s_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1h_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b01 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_sv_a page=st1h_z_p_bz cclass=st1h_z_p_bz_d_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1h_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b00 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_vs_a page=st1h_z_p_bz cclass=st1h_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1h_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_vs_b page=st1h_z_p_bz cclass=st1h_z_p_bz_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1h_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b01 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_sv2 page=st1h_z_p_bz cclass=st1h_z_p_bz_d_64_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1h_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b00 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vs2 page=st1h_z_p_bz cclass=st1h_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1w_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b11 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vi_b page=st1w_z_p_ai cclass=st1w_z_p_ai_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1w_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b10 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vi_a page=st1w_z_p_ai cclass=st1w_z_p_ai_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1w_z_p_bi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 size:u=0bxx ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_cst_si page=st1w_z_p_bi cclass=st1w_z_p_bi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1w_z_p_br_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 ig1=0b10 size:u=0bxx Rm:u=0bxxxxx ig2=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_cs iclass=sve_mem_cst_ss page=st1w_z_p_br cclass=st1w_z_p_br_',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1w_z_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_sv_b page=st1w_z_p_bz cclass=st1w_z_p_bz_s_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1w_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b01 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_sv_a page=st1w_z_p_bz cclass=st1w_z_p_bz_d_x32_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1w_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b00 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_vs_a page=st1w_z_p_bz cclass=st1w_z_p_bz_d_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1w_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_vs_b page=st1w_z_p_bz cclass=st1w_z_p_bz_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1w_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b01 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_sv2 page=st1w_z_p_bz cclass=st1w_z_p_bz_d_64_scaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st1w_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b00 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vs2 page=st1w_z_p_bz cclass=st1w_z_p_bz_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st2b_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 opc=0b01 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st2b_z_p_bi cclass=st2b_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st2b_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 opc=0b01 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st2b_z_p_br cclass=st2b_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st2d_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 opc=0b01 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st2d_z_p_bi cclass=st2d_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st2d_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 opc=0b01 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st2d_z_p_br cclass=st2d_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st2h_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 opc=0b01 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st2h_z_p_bi cclass=st2h_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st2h_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 opc=0b01 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st2h_z_p_br cclass=st2h_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st2w_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 opc=0b01 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st2w_z_p_bi cclass=st2w_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st2w_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 opc=0b01 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st2w_z_p_br cclass=st2w_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st3b_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 opc=0b10 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st3b_z_p_bi cclass=st3b_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st3b_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 opc=0b10 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st3b_z_p_br cclass=st3b_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st3d_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 opc=0b10 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st3d_z_p_bi cclass=st3d_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st3d_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 opc=0b10 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st3d_z_p_br cclass=st3d_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st3h_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 opc=0b10 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st3h_z_p_bi cclass=st3h_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st3h_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 opc=0b10 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st3h_z_p_br cclass=st3h_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st3w_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 opc=0b10 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st3w_z_p_bi cclass=st3w_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st3w_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 opc=0b10 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st3w_z_p_br cclass=st3w_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st4b_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 opc=0b11 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st4b_z_p_bi cclass=st4b_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st4b_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 opc=0b11 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st4b_z_p_br cclass=st4b_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st4d_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 opc=0b11 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st4d_z_p_bi cclass=st4d_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st4d_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 opc=0b11 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st4d_z_p_br cclass=st4d_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st4h_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 opc=0b11 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st4h_z_p_bi cclass=st4h_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st4h_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 opc=0b11 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st4h_z_p_br cclass=st4h_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st4w_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 opc=0b11 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st4w_z_p_bi cclass=st4w_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING st4w_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 opc=0b11 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st4w_z_p_br cclass=st4w_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING stnt1b_z_p_ar_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b10 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1B status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_sstnt_32b_vs page=stnt1b_z_p_ar cclass=stnt1b_z_p_ar_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|STORE',
};

ENCODING stnt1b_z_p_ar_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b00 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1B status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_sstnt_64b_vs page=stnt1b_z_p_ar cclass=stnt1b_z_p_ar_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|STORE',
};

ENCODING stnt1b_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b001 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_cstnt_si page=stnt1b_z_p_bi cclass=stnt1b_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|STORE',
};

ENCODING stnt1b_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b00 Rm:u=0bxxxxx ig2=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_cstnt_ss page=stnt1b_z_p_br cclass=stnt1b_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|STORE',
};

ENCODING stnt1d_z_p_ar_d_64_unscaled => {
  name       => 'SVE2',
  diagram    => 'ig0=0b1110010 msz=0b11 ig1=0b00 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1D status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_sstnt_64b_vs page=stnt1d_z_p_ar cclass=stnt1d_z_p_ar_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|STORE',
};

ENCODING stnt1d_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 ig1=0b001 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_cstnt_si page=stnt1d_z_p_bi cclass=stnt1d_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|STORE',
};

ENCODING stnt1d_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 ig1=0b00 Rm:u=0bxxxxx ig2=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_cstnt_ss page=stnt1d_z_p_br cclass=stnt1d_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|STORE',
};

ENCODING stnt1h_z_p_ar_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b10 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1H status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_sstnt_32b_vs page=stnt1h_z_p_ar cclass=stnt1h_z_p_ar_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|STORE',
};

ENCODING stnt1h_z_p_ar_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b00 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1H status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_sstnt_64b_vs page=stnt1h_z_p_ar cclass=stnt1h_z_p_ar_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|STORE',
};

ENCODING stnt1h_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b001 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_cstnt_si page=stnt1h_z_p_bi cclass=stnt1h_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|STORE',
};

ENCODING stnt1h_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b00 Rm:u=0bxxxxx ig2=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_cstnt_ss page=stnt1h_z_p_br cclass=stnt1h_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|STORE',
};

ENCODING stnt1w_z_p_ar_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b10 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1W status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_sstnt_32b_vs page=stnt1w_z_p_ar cclass=stnt1w_z_p_ar_s_x32_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|STORE',
};

ENCODING stnt1w_z_p_ar_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b00 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1W status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_sstnt_64b_vs page=stnt1w_z_p_ar cclass=stnt1w_z_p_ar_d_64_unscaled',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|STORE',
};

ENCODING stnt1w_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b001 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_cstnt_si page=stnt1w_z_p_bi cclass=stnt1w_z_p_bi_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|STORE',
};

ENCODING stnt1w_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b00 Rm:u=0bxxxxx ig2=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_cstnt_ss page=stnt1w_z_p_br cclass=stnt1w_z_p_br_contiguous',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|HINT|STORE',
};

ENCODING str_p_bi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010110 imm9h:u=0bxxxxxx ig1=0b000 imm9l:u=0bxxx Rn:u=0bxxxxx ig2=0b0 Pt:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_cs iclass=sve_mem_pspill page=str_p_bi cclass=str_p_bi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING str_z_bi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010110 imm9h:u=0bxxxxxx ig1=0b010 imm9l:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_cs iclass=sve_mem_spill page=str_z_bi cclass=str_z_bi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|STORE',
};

ENCODING sub_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b000 opc=0b001 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_0 page=sub_z_p_zz cclass=sub_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING sub_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 opc=0b001 ig2=0b11 sh:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm0 page=sub_z_zi cclass=sub_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING sub_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b000 opc=0b001 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_bin_cons_arit_0 page=sub_z_zz cclass=sub_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING subhnb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S=0b1 R=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SUBHNB status=Green',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_arith_narrow page=subhnb_z_zz cclass=subhnb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING subhnt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S=0b1 R=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SUBHNT status=Green',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_arith_narrow page=subhnt_z_zz cclass=subhnt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING subr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b000 opc=0b011 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUBR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_0 page=subr_z_p_zz cclass=subr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING subr_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 opc=0b011 ig2=0b11 sh:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUBR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm0 page=subr_z_zi cclass=subr_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC',
};

ENCODING sudot_z_zzzi_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00011 U=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mixed_dot_by_indexed_elem page=sudot_z_zzzi cclass=sudot_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DOT_PRODUCT',
};

ENCODING sunpkhi_z_z_ => {
  name       => 'High half',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1100 U=0b0 H=0b1 ig2=0b001110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUNPKHI status=Green sve-half=sve-high-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_unpk page=sunpkhi_z_z cclass=sunpkhi_z_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SIGNED',
};

ENCODING sunpklo_z_z_ => {
  name       => 'Low half',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1100 U=0b0 H=0b0 ig2=0b001110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUNPKLO status=Green sve-half=sve-low-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_unpk page=sunpkhi_z_z cclass=sunpklo_z_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SIGNED',
};

ENCODING suqadd_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b011 op=0b1 S=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SUQADD status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary_sat page=suqadd_z_p_zz cclass=suqadd_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING',
};

ENCODING sxtb_z_p_z_ => {
  name       => 'Byte',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 ig2=0b00 U=0b0 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SXTB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_0 page=sxtb_z_p_z cclass=sxtb_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SIGNED',
};

ENCODING sxth_z_p_z_ => {
  name       => 'Halfword',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 ig2=0b01 U=0b0 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SXTH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_0 page=sxtb_z_p_z cclass=sxth_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SIGNED',
};

ENCODING sxtw_z_p_z_ => {
  name       => 'Word',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 ig2=0b10 U=0b0 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SXTW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_0 page=sxtb_z_p_z cclass=sxtw_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SIGNED',
};

ENCODING tbl_z_zz_1 => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=TBL no-reg-for-table=tbl1 status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_tbl page=tbl_z_zz cclass=tbl_z_zz_1',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|TABLE_LOOKUP',
};

ENCODING tbl_z_zz_2 => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b00101 op=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=TBL no-reg-for-table=tbl2 status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_tbl_3src page=tbl_z_zz cclass=tbl_z_zz_2',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|TABLE_LOOKUP',
};

ENCODING tbx_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b00101 op=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=TBX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_tbl_3src page=tbx_z_zz cclass=tbx_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|TABLE_LOOKUP',
};

ENCODING trn1_p_pp_ => {
  name       => 'Even',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10 Pm:u=0bxxxx ig2=0b010 opc=0b10 H=0b0 ig3=0b0 Pn:u=0bxxxx ig4=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=TRN1 status=Green sve-odd-even=sve-even',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_bin_perm_pp page=trn1_p_pp cclass=trn1_p_pp_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING trn2_p_pp_ => {
  name       => 'Odd',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10 Pm:u=0bxxxx ig2=0b010 opc=0b10 H=0b1 ig3=0b0 Pn:u=0bxxxx ig4=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=TRN2 status=Green sve-odd-even=sve-odd',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_bin_perm_pp page=trn1_p_pp cclass=trn2_p_pp_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING trn1_z_zz_ => {
  name       => 'Even',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 ig3=0b10 H=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=TRN1 status=Green sve-odd-even=sve-even',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_perm_zz page=trn1_z_zz cclass=trn1_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING trn1_z_zz_q => {
  name       => 'Even (quadwords)',
  diagram    => 'ig0=0b000001011 op=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b11 H=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=TRN1 status=Green sve-odd-even=sve-even-quad',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_long_perm_zz page=trn1_z_zz cclass=trn1_z_zz_q',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING trn2_z_zz_ => {
  name       => 'Odd',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 ig3=0b10 H=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=TRN2 status=Green sve-odd-even=sve-odd',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_perm_zz page=trn1_z_zz cclass=trn2_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING trn2_z_zz_q => {
  name       => 'Odd (quadwords)',
  diagram    => 'ig0=0b000001011 op=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b11 H=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=TRN2 status=Green sve-odd-even=sve-odd-quad',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_long_perm_zz page=trn1_z_zz cclass=trn2_z_zz_q',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING uaba_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b11111 U=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UABA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_aba page=uaba_z_zzz cclass=uaba_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|UNSIGNED',
};

ENCODING uabalb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1100 U=0b1 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UABALB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_aba_long page=uabalb_z_zzz cclass=uabalb_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|UNSIGNED',
};

ENCODING uabalt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1100 U=0b1 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UABALT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_aba_long page=uabalt_z_zzz cclass=uabalt_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|UNSIGNED',
};

ENCODING uabd_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b10 U=0b1 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UABD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_1 page=uabd_z_p_zz cclass=uabd_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|UNSIGNED',
};

ENCODING uabdlb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b1 S=0b1 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UABDLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=uabdlb_z_zz cclass=uabdlb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|UNSIGNED',
};

ENCODING uabdlt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b1 S=0b1 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UABDLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=uabdlt_z_zz cclass=uabdlt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|UNSIGNED',
};

ENCODING uadalp_z_p_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00010 U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UADALP status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_accumulate_long_pairs page=uadalp_z_p_z cclass=uadalp_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|UNSIGNED',
};

ENCODING uaddlb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b0 S=0b0 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UADDLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=uaddlb_z_zz cclass=uaddlb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING uaddlt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b0 S=0b0 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UADDLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=uaddlt_z_zz cclass=uaddlt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING uaddv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b000 opc=0b00 U=0b1 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UADDV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_0 page=uaddv_r_p_z cclass=uaddv_r_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING uaddwb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b0 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UADDWB status=Green',
  docvars2   => 'shape=wide',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_wide page=uaddwb_z_zz cclass=uaddwb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING uaddwt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b0 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UADDWT status=Green',
  docvars2   => 'shape=wide',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_wide page=uaddwt_z_zz cclass=uaddwt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING ucvtf_z_p_z_h2fp16 => {
  name       => '16-bit to half-precision',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b010 opc2=0b01 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=16-to-half instr-class=sve isa=A64 mnemonic=UCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=ucvtf_z_p_z cclass=ucvtf_z_p_z_h2fp16',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|UNSIGNED',
};

ENCODING ucvtf_z_p_z_w2fp16 => {
  name       => '32-bit to half-precision',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b010 opc2=0b10 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=32-to-half instr-class=sve isa=A64 mnemonic=UCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=ucvtf_z_p_z cclass=ucvtf_z_p_z_w2fp16',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|UNSIGNED',
};

ENCODING ucvtf_z_p_z_w2s => {
  name       => '32-bit to single-precision',
  diagram    => 'ig0=0b01100101 opc=0b10 ig1=0b010 opc2=0b10 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=32-to-single instr-class=sve isa=A64 mnemonic=UCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=ucvtf_z_p_z cclass=ucvtf_z_p_z_w2s',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|UNSIGNED',
};

ENCODING ucvtf_z_p_z_w2d => {
  name       => '32-bit to double-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b010 opc2=0b00 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=32-to-double instr-class=sve isa=A64 mnemonic=UCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=ucvtf_z_p_z cclass=ucvtf_z_p_z_w2d',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|UNSIGNED',
};

ENCODING ucvtf_z_p_z_x2fp16 => {
  name       => '64-bit to half-precision',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b010 opc2=0b11 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=64-to-half instr-class=sve isa=A64 mnemonic=UCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=ucvtf_z_p_z cclass=ucvtf_z_p_z_x2fp16',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|UNSIGNED',
};

ENCODING ucvtf_z_p_z_x2s => {
  name       => '64-bit to single-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b010 opc2=0b10 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=64-to-single instr-class=sve isa=A64 mnemonic=UCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=ucvtf_z_p_z cclass=ucvtf_z_p_z_x2s',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|UNSIGNED',
};

ENCODING ucvtf_z_p_z_x2d => {
  name       => '64-bit to double-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b010 opc2=0b11 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=64-to-double instr-class=sve isa=A64 mnemonic=UCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=ucvtf_z_p_z cclass=ucvtf_z_p_z_x2d',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CONVERSION|UNSIGNED',
};

ENCODING udiv_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0101 R=0b0 U=0b1 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UDIV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_div page=udiv_z_p_zz cclass=udiv_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING udivr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0101 R=0b1 U=0b1 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UDIVR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_div page=udivr_z_p_zz cclass=udivr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING udot_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00000 U=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_dot page=udot_z_zzz cclass=udot_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DOT_PRODUCT|UNSIGNED',
};

ENCODING udot_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00000 U=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve isa=A64 mnemonic=UDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_dot_by_indexed_elem page=udot_z_zzzi cclass=udot_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DOT_PRODUCT|UNSIGNED',
};

ENCODING udot_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b00000 U=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve isa=A64 mnemonic=UDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_dot_by_indexed_elem page=udot_z_zzzi cclass=udot_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DOT_PRODUCT|UNSIGNED',
};

ENCODING uhadd_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 R=0b0 S=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UHADD status=Green',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary page=uhadd_z_p_zz cclass=uhadd_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|HALVING|UNSIGNED',
};

ENCODING uhsub_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 R=0b0 S=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UHSUB status=Green',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary page=uhsub_z_p_zz cclass=uhsub_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|HALVING|UNSIGNED',
};

ENCODING uhsubr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 R=0b1 S=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UHSUBR status=Green',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary page=uhsubr_z_p_zz cclass=uhsubr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|HALVING|UNSIGNED',
};

ENCODING umax_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b00 U=0b1 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMAX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_1 page=umax_z_p_zz cclass=umax_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX|UNSIGNED',
};

ENCODING umax_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b101 ig2=0b00 U=0b1 ig3=0b11 o2=0b0 imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMAX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm1 page=umax_z_zi cclass=umax_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX|UNSIGNED',
};

ENCODING umaxp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 opc=0b10 U=0b1 ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMAXP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_arith_binary_pairs page=umaxp_z_p_zz cclass=umaxp_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX|UNSIGNED',
};

ENCODING umaxv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b00 U=0b1 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMAXV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_1 page=umaxv_r_p_z cclass=umaxv_r_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX|UNSIGNED',
};

ENCODING umin_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b01 U=0b1 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMIN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_1 page=umin_z_p_zz cclass=umin_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX|UNSIGNED',
};

ENCODING umin_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b101 ig2=0b01 U=0b1 ig3=0b11 o2=0b0 imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMIN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm1 page=umin_z_zi cclass=umin_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX|UNSIGNED',
};

ENCODING uminp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 opc=0b11 U=0b1 ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMINP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_arith_binary_pairs page=uminp_z_p_zz cclass=uminp_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX|UNSIGNED',
};

ENCODING uminv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b01 U=0b1 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMINV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_1 page=uminv_r_p_z cclass=uminv_r_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|MIN_MAX|UNSIGNED',
};

ENCODING umlalb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b0 U=0b1 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMLALB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mlal_long page=umlalb_z_zzz cclass=umlalb_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|UNSIGNED',
};

ENCODING umlalb_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b10 S=0b0 U=0b1 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=UMLALB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=umlalb_z_zzzi cclass=umlalb_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|UNSIGNED',
};

ENCODING umlalb_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b10 S=0b0 U=0b1 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=UMLALB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=umlalb_z_zzzi cclass=umlalb_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|UNSIGNED',
};

ENCODING umlalt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b0 U=0b1 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMLALT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mlal_long page=umlalt_z_zzz cclass=umlalt_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|UNSIGNED',
};

ENCODING umlalt_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b10 S=0b0 U=0b1 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=UMLALT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=umlalt_z_zzzi cclass=umlalt_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|UNSIGNED',
};

ENCODING umlalt_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b10 S=0b0 U=0b1 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=UMLALT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=umlalt_z_zzzi cclass=umlalt_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|UNSIGNED',
};

ENCODING umlslb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b1 U=0b1 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMLSLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mlal_long page=umlslb_z_zzz cclass=umlslb_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|UNSIGNED',
};

ENCODING umlslb_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b10 S=0b1 U=0b1 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=UMLSLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=umlslb_z_zzzi cclass=umlslb_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|UNSIGNED',
};

ENCODING umlslb_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b10 S=0b1 U=0b1 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=UMLSLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=umlslb_z_zzzi cclass=umlslb_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|UNSIGNED',
};

ENCODING umlslt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b1 U=0b1 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMLSLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mlal_long page=umlslt_z_zzz cclass=umlslt_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|UNSIGNED',
};

ENCODING umlslt_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b10 S=0b1 U=0b1 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=UMLSLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=umlslt_z_zzzi cclass=umlslt_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|UNSIGNED',
};

ENCODING umlslt_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b10 S=0b1 U=0b1 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=UMLSLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=umlslt_z_zzzi cclass=umlslt_z_zzzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|UNSIGNED',
};

ENCODING ummla_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01000101 uns=0b11 ig1=0b0 Zm:u=0bxxxxx ig2=0b100110 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_mmla page=ummla_z_zzz cclass=ummla_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC|UNSIGNED',
};

ENCODING umulh_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0100 H=0b1 U=0b1 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMULH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_2 page=umulh_z_p_zz cclass=umulh_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING umulh_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0110 ig3=0b1 U=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMULH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_arit_b iclass=sve_int_mul_b page=umulh_z_zz cclass=umulh_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING umullb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op=0b1 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMULLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_mul_long page=umullb_z_zz cclass=umullb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING umullb_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b110 U=0b1 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=UMULLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_long_by_indexed_elem page=umullb_z_zzi cclass=umullb_z_zzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING umullb_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b110 U=0b1 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=UMULLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_long_by_indexed_elem page=umullb_z_zzi cclass=umullb_z_zzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING umullt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op=0b1 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMULLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_mul_long page=umullt_z_zz cclass=umullt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING umullt_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b110 U=0b1 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=UMULLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_long_by_indexed_elem page=umullt_z_zzi cclass=umullt_z_zzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING umullt_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b110 U=0b1 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=UMULLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_long_by_indexed_elem page=umullt_z_zzi cclass=umullt_z_zzi_d',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING uqadd_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b011 op=0b0 S=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQADD status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary_sat page=uqadd_z_p_zz cclass=uqadd_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING uqadd_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 ig2=0b10 U=0b1 ig3=0b11 sh:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQADD status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm0 page=uqadd_z_zi cclass=uqadd_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING uqadd_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b10 U=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQADD status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve iclass=sve_int_bin_cons_arit_0 page=uqadd_z_zz cclass=uqadd_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING uqdecb_r_rs_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECB-32-fsreg instr-class=sve isa=A64 mnemonic=UQDECB reg-type=32-fsreg status=Green sve-esize=esize-byte',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqdecb_r_rs cclass=uqdecb_r_rs_uw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqdecb_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECB-64-fsreg instr-class=sve isa=A64 mnemonic=UQDECB reg-type=64-fsreg status=Green sve-esize=esize-byte',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqdecb_r_rs cclass=uqdecb_r_rs_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqdecd_r_rs_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECD-32-fsreg instr-class=sve isa=A64 mnemonic=UQDECD reg-type=32-fsreg status=Green sve-esize=esize-doubleword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqdecd_r_rs cclass=uqdecd_r_rs_uw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqdecd_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECD-64-fsreg instr-class=sve isa=A64 mnemonic=UQDECD reg-type=64-fsreg status=Green sve-esize=esize-doubleword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqdecd_r_rs cclass=uqdecd_r_rs_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqdecd_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b1 U=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQDECD status=Green sve-esize=esize-doubleword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=uqdecd_z_zs cclass=uqdecd_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqdech_r_rs_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECH-32-fsreg instr-class=sve isa=A64 mnemonic=UQDECH reg-type=32-fsreg status=Green sve-esize=esize-halfword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqdech_r_rs cclass=uqdech_r_rs_uw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqdech_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECH-64-fsreg instr-class=sve isa=A64 mnemonic=UQDECH reg-type=64-fsreg status=Green sve-esize=esize-halfword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqdech_r_rs cclass=uqdech_r_rs_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqdech_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b1 U=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQDECH status=Green sve-esize=esize-halfword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=uqdech_z_zs cclass=uqdech_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqdecp_r_p_r_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b1 U=0b1 ig2=0b10001 sf=0b0 op=0b0 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECP-32-fsreg instr-class=sve isa=A64 mnemonic=UQDECP reg-type=32-fsreg status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r_sat page=uqdecp_r_p_r cclass=uqdecp_r_p_r_uw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqdecp_r_p_r_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b1 U=0b1 ig2=0b10001 sf=0b1 op=0b0 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECP-64-fsreg instr-class=sve isa=A64 mnemonic=UQDECP reg-type=64-fsreg status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r_sat page=uqdecp_r_p_r cclass=uqdecp_r_p_r_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqdecp_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b1 U=0b1 ig2=0b10000 opc=0b00 Pm:u=0bxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQDECP status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_v_sat page=uqdecp_z_p_z cclass=uqdecp_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqdecw_r_rs_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECW-32-fsreg instr-class=sve isa=A64 mnemonic=UQDECW reg-type=32-fsreg status=Green sve-esize=esize-word',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqdecw_r_rs cclass=uqdecw_r_rs_uw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqdecw_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECW-64-fsreg instr-class=sve isa=A64 mnemonic=UQDECW reg-type=64-fsreg status=Green sve-esize=esize-word',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqdecw_r_rs cclass=uqdecw_r_rs_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqdecw_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b1 U=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQDECW status=Green sve-esize=esize-word',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=uqdecw_z_zs cclass=uqdecw_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqincb_r_rs_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCB-32-fsreg instr-class=sve isa=A64 mnemonic=UQINCB reg-type=32-fsreg status=Green sve-esize=esize-byte',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqincb_r_rs cclass=uqincb_r_rs_uw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqincb_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCB-64-fsreg instr-class=sve isa=A64 mnemonic=UQINCB reg-type=64-fsreg status=Green sve-esize=esize-byte',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqincb_r_rs cclass=uqincb_r_rs_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqincd_r_rs_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCD-32-fsreg instr-class=sve isa=A64 mnemonic=UQINCD reg-type=32-fsreg status=Green sve-esize=esize-doubleword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqincd_r_rs cclass=uqincd_r_rs_uw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqincd_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCD-64-fsreg instr-class=sve isa=A64 mnemonic=UQINCD reg-type=64-fsreg status=Green sve-esize=esize-doubleword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqincd_r_rs cclass=uqincd_r_rs_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqincd_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b0 U=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQINCD status=Green sve-esize=esize-doubleword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=uqincd_z_zs cclass=uqincd_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqinch_r_rs_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCH-32-fsreg instr-class=sve isa=A64 mnemonic=UQINCH reg-type=32-fsreg status=Green sve-esize=esize-halfword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqinch_r_rs cclass=uqinch_r_rs_uw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqinch_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCH-64-fsreg instr-class=sve isa=A64 mnemonic=UQINCH reg-type=64-fsreg status=Green sve-esize=esize-halfword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqinch_r_rs cclass=uqinch_r_rs_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqinch_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b0 U=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQINCH status=Green sve-esize=esize-halfword',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=uqinch_z_zs cclass=uqinch_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqincp_r_p_r_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b0 U=0b1 ig2=0b10001 sf=0b0 op=0b0 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCP-32-fsreg instr-class=sve isa=A64 mnemonic=UQINCP reg-type=32-fsreg status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r_sat page=uqincp_r_p_r cclass=uqincp_r_p_r_uw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqincp_r_p_r_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b0 U=0b1 ig2=0b10001 sf=0b1 op=0b0 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCP-64-fsreg instr-class=sve isa=A64 mnemonic=UQINCP reg-type=64-fsreg status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r_sat page=uqincp_r_p_r cclass=uqincp_r_p_r_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqincp_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b0 U=0b1 ig2=0b10000 opc=0b00 Pm:u=0bxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQINCP status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_v_sat page=uqincp_z_p_z cclass=uqincp_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqincw_r_rs_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCW-32-fsreg instr-class=sve isa=A64 mnemonic=UQINCW reg-type=32-fsreg status=Green sve-esize=esize-word',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqincw_r_rs cclass=uqincw_r_rs_uw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqincw_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCW-64-fsreg instr-class=sve isa=A64 mnemonic=UQINCW reg-type=64-fsreg status=Green sve-esize=esize-word',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqincw_r_rs cclass=uqincw_r_rs_x',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqincw_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b0 U=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQINCW status=Green sve-esize=esize-word',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=uqincw_z_zs cclass=uqincw_z_zs_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|INC_DEC|SATURATING|UNSIGNED',
};

ENCODING uqrshl_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 R=0b0 N=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQRSHL status=Green',
  docvars2   => 'rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=uqrshl_z_p_zz cclass=uqrshl_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|ROUNDING|SATURATING|UNSIGNED',
};

ENCODING uqrshlr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 R=0b1 N=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQRSHLR status=Green',
  docvars2   => 'rounding=1 saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=uqrshlr_z_p_zz cclass=uqrshlr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|ROUNDING|SATURATING|UNSIGNED',
};

ENCODING uqrshrnb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b1 U=0b1 R=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQRSHRNB status=Green',
  docvars2   => 'rounding=1 saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=uqrshrnb_z_zi cclass=uqrshrnb_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|ROUNDING|SATURATING|UNSIGNED',
};

ENCODING uqrshrnt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b1 U=0b1 R=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQRSHRNT status=Green',
  docvars2   => 'rounding=1 saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=uqrshrnt_z_zi cclass=uqrshrnt_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|ROUNDING|SATURATING|UNSIGNED',
};

ENCODING uqshl_z_p_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b01 L=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSHL status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=uqshl_z_p_zi cclass=uqshl_z_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|SATURATING|UNSIGNED',
};

ENCODING uqshl_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 R=0b0 N=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSHL status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=uqshl_z_p_zz cclass=uqshl_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|SATURATING|UNSIGNED',
};

ENCODING uqshlr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 R=0b1 N=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSHLR status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=uqshlr_z_p_zz cclass=uqshlr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|SATURATING|UNSIGNED',
};

ENCODING uqshrnb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b1 U=0b1 R=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSHRNB status=Green',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=uqshrnb_z_zi cclass=uqshrnb_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|SATURATING|UNSIGNED',
};

ENCODING uqshrnt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b1 U=0b1 R=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSHRNT status=Green',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=uqshrnt_z_zi cclass=uqshrnt_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|SATURATING|UNSIGNED',
};

ENCODING uqsub_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b011 op=0b0 S=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSUB status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary_sat page=uqsub_z_p_zz cclass=uqsub_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SATURATING|UNSIGNED',
};

ENCODING uqsub_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 ig2=0b11 U=0b1 ig3=0b11 sh:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQSUB status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm0 page=uqsub_z_zi cclass=uqsub_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING uqsub_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b11 U=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQSUB status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve iclass=sve_int_bin_cons_arit_0 page=uqsub_z_zz cclass=uqsub_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING uqsubr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b011 op=0b1 S=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSUBR status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary_sat page=uqsubr_z_p_zz cclass=uqsubr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SATURATING|UNSIGNED',
};

ENCODING uqxtnb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx ig2=0b000010 ig3=0b0 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQXTNB status=Green',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_extract_narrow page=uqxtnb_z_zz cclass=uqxtnb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION|SATURATING|UNSIGNED',
};

ENCODING uqxtnt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx ig2=0b000010 ig3=0b0 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQXTNT status=Green',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_extract_narrow page=uqxtnt_z_zz cclass=uqxtnt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|EXTRACTION|SATURATING|UNSIGNED',
};

ENCODING urecpe_z_p_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b0 ig2=0b0 opc=0b00 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URECPE status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_unary page=urecpe_z_p_z cclass=urecpe_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|UNSIGNED',
};

ENCODING urhadd_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 R=0b1 S=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URHADD status=Green',
  docvars2   => 'halving=1 rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary page=urhadd_z_p_zz cclass=urhadd_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|HALVING|ROUNDING|UNSIGNED',
};

ENCODING urshl_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b0 R=0b0 N=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URSHL status=Green',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=urshl_z_p_zz cclass=urshl_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|ROUNDING|UNSIGNED',
};

ENCODING urshlr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b0 R=0b1 N=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URSHLR status=Green',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=urshlr_z_p_zz cclass=urshlr_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|ROUNDING|UNSIGNED',
};

ENCODING urshr_z_p_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b11 L=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URSHR status=Green',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=urshr_z_p_zi cclass=urshr_z_p_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|ROUNDING|UNSIGNED',
};

ENCODING ursqrte_z_p_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b0 ig2=0b0 opc=0b01 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URSQRTE status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_unary page=ursqrte_z_p_z cclass=ursqrte_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|MATH|UNSIGNED',
};

ENCODING ursra_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 tszh:u=0bxx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1110 R=0b1 U=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URSRA status=Green',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_acc iclass=sve_intx_sra page=ursra_z_zi cclass=ursra_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|CARITHMETIC|ROUNDING|UNSIGNED',
};

ENCODING usdot_z_zzz_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b0 Zm:u=0bxxxxx ig2=0b011110 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=USDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mixed_dot page=usdot_z_zzz cclass=usdot_z_zzz_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DOT_PRODUCT',
};

ENCODING usdot_z_zzzi_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00011 U=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=USDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mixed_dot_by_indexed_elem page=usdot_z_zzzi cclass=usdot_z_zzzi_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DOT_PRODUCT',
};

ENCODING ushllb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1010 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USHLLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_shift_long page=ushllb_z_zi cclass=ushllb_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|UNSIGNED',
};

ENCODING ushllt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1010 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USHLLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_shift_long page=ushllt_z_zi cclass=ushllt_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_LEFT|UNSIGNED',
};

ENCODING usmmla_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01000101 uns=0b10 ig1=0b0 Zm:u=0bxxxxx ig2=0b100110 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=USMMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_mmla page=usmmla_z_zzz cclass=usmmla_z_zzz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|CARITHMETIC',
};

ENCODING usqadd_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b011 op=0b1 S=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USQADD status=Green',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary_sat page=usqadd_z_p_zz cclass=usqadd_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|SATURATING',
};

ENCODING usra_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 tszh:u=0bxx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1110 R=0b0 U=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USRA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_sra page=usra_z_zi cclass=usra_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|SHIFT_RIGHT|CARITHMETIC|UNSIGNED',
};

ENCODING usublb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b0 S=0b1 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USUBLB status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=usublb_z_zz cclass=usublb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING usublt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b0 S=0b1 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USUBLT status=Green',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=usublt_z_zz cclass=usublt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING usubwb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b1 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USUBWB status=Green',
  docvars2   => 'shape=wide',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_wide page=usubwb_z_zz cclass=usubwb_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING usubwt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b1 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USUBWT status=Green',
  docvars2   => 'shape=wide',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_wide page=usubwt_z_zz cclass=usubwt_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ARITHMETIC|UNSIGNED',
};

ENCODING uunpkhi_z_z_ => {
  name       => 'High half',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1100 U=0b1 H=0b1 ig2=0b001110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UUNPKHI status=Green sve-half=sve-high-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_unpk page=uunpkhi_z_z cclass=uunpkhi_z_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|UNSIGNED',
};

ENCODING uunpklo_z_z_ => {
  name       => 'Low half',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1100 U=0b1 H=0b0 ig2=0b001110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UUNPKLO status=Green sve-half=sve-low-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_unpk page=uunpkhi_z_z cclass=uunpklo_z_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|UNSIGNED',
};

ENCODING uxtb_z_p_z_ => {
  name       => 'Byte',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 ig2=0b00 U=0b1 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UXTB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_0 page=uxtb_z_p_z cclass=uxtb_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|UNSIGNED',
};

ENCODING uxth_z_p_z_ => {
  name       => 'Halfword',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 ig2=0b01 U=0b1 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UXTH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_0 page=uxtb_z_p_z cclass=uxth_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|UNSIGNED',
};

ENCODING uxtw_z_p_z_ => {
  name       => 'Word',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 ig2=0b10 U=0b1 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UXTW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_0 page=uxtb_z_p_z cclass=uxtw_z_p_z_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|UNSIGNED',
};

ENCODING uzp1_p_pp_ => {
  name       => 'Even',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10 Pm:u=0bxxxx ig2=0b010 opc=0b01 H=0b0 ig3=0b0 Pn:u=0bxxxx ig4=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UZP1 status=Green sve-odd-even=sve-even',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_bin_perm_pp page=uzp1_p_pp cclass=uzp1_p_pp_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING uzp2_p_pp_ => {
  name       => 'Odd',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10 Pm:u=0bxxxx ig2=0b010 opc=0b01 H=0b1 ig3=0b0 Pn:u=0bxxxx ig4=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UZP2 status=Green sve-odd-even=sve-odd',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_bin_perm_pp page=uzp1_p_pp cclass=uzp2_p_pp_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING uzp1_z_zz_ => {
  name       => 'Even',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 ig3=0b01 H=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UZP1 status=Green sve-odd-even=sve-even',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_perm_zz page=uzp1_z_zz cclass=uzp1_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING uzp1_z_zz_q => {
  name       => 'Even (quadwords)',
  diagram    => 'ig0=0b000001011 op=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b01 H=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UZP1 status=Green sve-odd-even=sve-even-quad',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_long_perm_zz page=uzp1_z_zz cclass=uzp1_z_zz_q',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING uzp2_z_zz_ => {
  name       => 'Odd',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 ig3=0b01 H=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UZP2 status=Green sve-odd-even=sve-odd',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_perm_zz page=uzp1_z_zz cclass=uzp2_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING uzp2_z_zz_q => {
  name       => 'Odd (quadwords)',
  diagram    => 'ig0=0b000001011 op=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b01 H=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UZP2 status=Green sve-odd-even=sve-odd-quad',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_long_perm_zz page=uzp1_z_zz cclass=uzp2_z_zz_q',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING whilege_p_p_rr_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U=0b0 lt=0b0 Rn:u=0bxxxxx eq=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILEGE status=Green sve-compare-type=ge',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_while_rr page=whilege_p_p_rr cclass=whilege_p_p_rr_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|SIGNED',
};

ENCODING whilegt_p_p_rr_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U=0b0 lt=0b0 Rn:u=0bxxxxx eq=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILEGT status=Green sve-compare-type=gt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_while_rr page=whilegt_p_p_rr cclass=whilegt_p_p_rr_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|SIGNED',
};

ENCODING whilehi_p_p_rr_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U=0b1 lt=0b0 Rn:u=0bxxxxx eq=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILEHI status=Green sve-compare-type=hi',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_while_rr page=whilehi_p_p_rr cclass=whilehi_p_p_rr_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|UNSIGNED',
};

ENCODING whilehs_p_p_rr_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U=0b1 lt=0b0 Rn:u=0bxxxxx eq=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILEHS status=Green sve-compare-type=hs',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_while_rr page=whilehs_p_p_rr cclass=whilehs_p_p_rr_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|UNSIGNED',
};

ENCODING whilele_p_p_rr_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U=0b0 lt=0b1 Rn:u=0bxxxxx eq=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=WHILELE status=Green sve-compare-type=le',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_while_rr page=whilele_p_p_rr cclass=whilele_p_p_rr_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|SIGNED',
};

ENCODING whilelo_p_p_rr_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U=0b1 lt=0b1 Rn:u=0bxxxxx eq=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=WHILELO status=Green sve-compare-type=lo',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_while_rr page=whilelo_p_p_rr cclass=whilelo_p_p_rr_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|UNSIGNED',
};

ENCODING whilels_p_p_rr_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U=0b1 lt=0b1 Rn:u=0bxxxxx eq=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=WHILELS status=Green sve-compare-type=ls',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_while_rr page=whilels_p_p_rr cclass=whilels_p_p_rr_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|UNSIGNED',
};

ENCODING whilelt_p_p_rr_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U=0b0 lt=0b1 Rn:u=0bxxxxx eq=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=WHILELT status=Green sve-compare-type=lt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_while_rr page=whilelt_p_p_rr cclass=whilelt_p_p_rr_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|SIGNED',
};

ENCODING whilerw_p_rr_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b001100 Rn:u=0bxxxxx rw=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILERW status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_cmpgpr iclass=sve_int_whilenc page=whilerw_p_rr cclass=whilerw_p_rr_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE',
};

ENCODING whilewr_p_rr_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b001100 Rn:u=0bxxxxx rw=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILEWR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_cmpgpr iclass=sve_int_whilenc page=whilewr_p_rr cclass=whilewr_p_rr_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE',
};

ENCODING wrffr_f_p_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00 ig1=0b100101 opc=0b00 ig2=0b10 ig3=0b1000100100 ig4=0b0 Pn:u=0bxxxx ig5=0b0 ig6=0b0 ig7=0b0 ig8=0b00',
  docvars    => 'instr-class=sve isa=A64 mnemonic=WRFFR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_wrffr iclass=sve_int_wrffr page=wrffr_f_p cclass=wrffr_f_p_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING xar_z_zzi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b001101 Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=XAR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_rotate_imm page=xar_z_zzi cclass=xar_z_zzi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|ROTATE_RIGHT',
};

ENCODING zip2_p_pp_ => {
  name       => 'High halves',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10 Pm:u=0bxxxx ig2=0b010 opc=0b00 H=0b1 ig3=0b0 Pn:u=0bxxxx ig4=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ZIP2 status=Green sve-halves=sve-high-halves',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_bin_perm_pp page=zip1_p_pp cclass=zip2_p_pp_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING zip1_p_pp_ => {
  name       => 'Low halves',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10 Pm:u=0bxxxx ig2=0b010 opc=0b00 H=0b0 ig3=0b0 Pn:u=0bxxxx ig4=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ZIP1 status=Green sve-halves=sve-low-halves',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_bin_perm_pp page=zip1_p_pp cclass=zip1_p_pp_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING zip2_z_zz_ => {
  name       => 'High halves',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 ig3=0b00 H=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ZIP2 status=Green sve-halves=sve-high-halves',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_perm_zz page=zip1_z_zz cclass=zip2_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING zip2_z_zz_q => {
  name       => 'High halves (quadwords)',
  diagram    => 'ig0=0b000001011 op=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b00 H=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ZIP2 status=Green sve-halves=sve-high-halves-quad',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_long_perm_zz page=zip1_z_zz cclass=zip2_z_zz_q',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING zip1_z_zz_ => {
  name       => 'Low halves',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 ig3=0b00 H=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ZIP1 status=Green sve-halves=sve-low-halves',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_perm_zz page=zip1_z_zz cclass=zip1_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING zip1_z_zz_q => {
  name       => 'Low halves (quadwords)',
  diagram    => 'ig0=0b000001011 op=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b00 H=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ZIP1 status=Green sve-halves=sve-low-halves-quad',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_long_perm_zz page=zip1_z_zz cclass=zip1_z_zz_q',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE',
};

ENCODING BIC_and_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 opc=0b10 ig1=0b0000 imm13:u=0bxxxxxxxxxxxxx Zdn:u=0bxxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=BIC instr-class=sve isa=A64 mnemonic=AND status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=and_z_zi_ alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_maskimm iclass=sve_int_log_imm page=BIC_and_z_zi cclass=BIC_and_z_zi_and_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
  eq_to      => 'AND <Zdn>.<T>, <Zdn>.<T>, #(-<const> - 1)',
};

ENCODING CMPLE_cmpge_p_p_zz_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=CMPLE instr-class=sve isa=A64 mnemonic=CMPGE status=Green sve-compare-type=ge',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=cmpge_p_p_zz_ alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=CMPLE_cmpeq_p_p_zz cclass=CMPLE_cmpeq_p_p_zz_cmpge_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON|SIGNED',
  eq_to      => 'CMPGE   <Pd>.<T>, <Pg>/Z, <Zn>.<T>, <Zm>.<T>',
};

ENCODING CMPLO_cmphi_p_p_zz_ => {
  name       => 'Higher',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=CMPLO instr-class=sve isa=A64 mnemonic=CMPHI status=Green sve-compare-type=hi',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=cmphi_p_p_zz_ alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=CMPLO_cmpeq_p_p_zz cclass=CMPLO_cmpeq_p_p_zz_cmphi_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON|UNSIGNED',
  eq_to      => 'CMPHI   <Pd>.<T>, <Pg>/Z, <Zn>.<T>, <Zm>.<T>',
};

ENCODING CMPLS_cmphs_p_p_zz_ => {
  name       => 'Higher or same',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=CMPLS instr-class=sve isa=A64 mnemonic=CMPHS status=Green sve-compare-type=hs',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=cmphs_p_p_zz_ alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=CMPLS_cmpeq_p_p_zz cclass=CMPLS_cmpeq_p_p_zz_cmphs_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|COMPARISON|UNSIGNED',
  eq_to      => 'CMPHS   <Pd>.<T>, <Pg>/Z, <Zn>.<T>, <Zm>.<T>',
};

ENCODING CMPLT_cmpgt_p_p_zz_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=CMPLT instr-class=sve isa=A64 mnemonic=CMPGT status=Green sve-compare-type=gt',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=cmpgt_p_p_zz_ alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=CMPLT_cmpeq_p_p_zz cclass=CMPLT_cmpeq_p_p_zz_cmpgt_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|COMPARISON|SIGNED',
  eq_to      => 'CMPGT   <Pd>.<T>, <Pg>/Z, <Zn>.<T>, <Zm>.<T>',
};

ENCODING EON_eor_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 opc=0b01 ig1=0b0000 imm13:u=0bxxxxxxxxxxxxx Zdn:u=0bxxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=EON instr-class=sve isa=A64 mnemonic=EOR status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=eor_z_zi_ alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_maskimm iclass=sve_int_log_imm page=EON_eor_z_zi cclass=EON_eor_z_zi_eor_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
  eq_to      => 'EOR <Zdn>.<T>, <Zdn>.<T>, #(-<const> - 1)',
};

ENCODING FACLE_facge_p_p_zz_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b1 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx o3=0b1 Pd:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=FACLE instr-class=sve isa=A64 mnemonic=FACGE status=Green sve-compare-type=ge',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=facge_p_p_zz_ alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=FACLE_facge_p_p_zz cclass=FACLE_facge_p_p_zz_facge_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPARISON',
  eq_to      => 'FACGE   <Pd>.<T>, <Pg>/Z, <Zn>.<T>, <Zm>.<T>',
};

ENCODING FACLT_facgt_p_p_zz_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b1 o2=0b1 Pg:u=0bxxx Zn:u=0bxxxxx o3=0b1 Pd:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=FACLT instr-class=sve isa=A64 mnemonic=FACGT status=Green sve-compare-type=gt',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=facgt_p_p_zz_ alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=FACLT_facge_p_p_zz cclass=FACLT_facge_p_p_zz_facgt_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPARISON',
  eq_to      => 'FACGT   <Pd>.<T>, <Pg>/Z, <Zn>.<T>, <Zm>.<T>',
};

ENCODING FCMLE_fcmge_p_p_zz_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b1 cmph=0b0 Pg:u=0bxxx Zn:u=0bxxxxx cmpl=0b0 Pd:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=FCMLE instr-class=sve isa=A64 mnemonic=FCMGE status=Green sve-compare-type=ge',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=fcmge_p_p_zz_ alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=FCMLE_fcmeq_p_p_zz cclass=FCMLE_fcmeq_p_p_zz_fcmge_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPARISON',
  eq_to      => 'FCMGE   <Pd>.<T>, <Pg>/Z, <Zn>.<T>, <Zm>.<T>',
};

ENCODING FCMLT_fcmgt_p_p_zz_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b1 cmph=0b0 Pg:u=0bxxx Zn:u=0bxxxxx cmpl=0b1 Pd:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=FCMLT instr-class=sve isa=A64 mnemonic=FCMGT status=Green sve-compare-type=gt',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=fcmgt_p_p_zz_ alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=FCMLT_fcmeq_p_p_zz cclass=FCMLT_fcmeq_p_p_zz_fcmgt_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|COMPARISON',
  eq_to      => 'FCMGT   <Pd>.<T>, <Pg>/Z, <Zn>.<T>, <Zm>.<T>',
};

ENCODING FMOV_cpy_z_p_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b0 M=0b1 sh:u=0b0 imm8:u=0b00000000 Zd:u=0bxxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=FMOV instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=cpy_z_p_i_ alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_wideimm_pred iclass=sve_int_dup_imm_pred page=FMOV_cpy_z_p_i cclass=FMOV_cpy_z_p_i_cpy_z_p_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
  eq_to      => 'CPY <Zd>.<T>, <Pg>/M, #0',
};

ENCODING FMOV_dup_z_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b111 opc=0b00 ig2=0b011 sh:u=0b0 imm8:u=0b00000000 Zd:u=0bxxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=FMOV instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=dup_z_i_ alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_dup_imm page=FMOV_dup_z_i cclass=FMOV_dup_z_i_dup_z_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
  eq_to      => 'DUP <Zd>.<T>, #0',
};

ENCODING FMOV_fcpy_z_p_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b110 imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=FMOV instr-class=sve isa=A64 mnemonic=FCPY status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=fcpy_z_p_i_ alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_wideimm_pred iclass=sve_int_dup_fpimm_pred page=FMOV_fcpy_z_p_i cclass=FMOV_fcpy_z_p_i_fcpy_z_p_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
  eq_to      => 'FCPY    <Zd>.<T>, <Pg>/M, #<const>',
};

ENCODING FMOV_fdup_z_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b111 opc=0b00 ig2=0b111 o2=0b0 imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=FMOV instr-class=sve isa=A64 mnemonic=FDUP status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=fdup_z_i_ alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_dup_fpimm page=FMOV_fdup_z_i cclass=FMOV_fdup_z_i_fdup_z_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
  eq_to      => 'FDUP    <Zd>.<T>, #<const>',
};

ENCODING MOV_and_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S:u=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  aliascond  => 'S == 0b0 && Pn == Pm',
  docvars    => 'alias_mnemonic=MOV cond-setting=no-s instr-class=sve isa=A64 mnemonic=AND status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=and_p_p_pp_z alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=MOV_and_p_p_pp cclass=MOV_and_p_p_pp_and_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|DATA_TRANSFER',
  eq_to      => 'AND <Pd>.B, <Pg>/Z, <Pn>.B, <Pn>.B',
};

ENCODING MOV_cpy_z_o_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b0 M=0b0 sh:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=cpy_z_o_i_ alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_pred iclass=sve_int_dup_imm_pred page=MOV_cpy_z_o_i cclass=MOV_cpy_z_o_i_cpy_z_o_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED',
  eq_to      => 'CPY     <Zd>.<T>, <Pg>/Z, #<imm>{, <shift>}',
};

ENCODING MOV_cpy_z_p_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b0 M=0b1 sh:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=cpy_z_p_i_ alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_wideimm_pred iclass=sve_int_dup_imm_pred page=MOV_cpy_z_p_i cclass=MOV_cpy_z_p_i_cpy_z_p_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED',
  eq_to      => 'CPY     <Zd>.<T>, <Pg>/M, #<imm>{, <shift>}',
};

ENCODING MOV_cpy_z_p_r_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b101000101 Pg:u=0bxxx Rn:u=0bxxxxx Zd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=cpy_z_p_r_ alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_cpy_r page=MOV_cpy_z_p_r cclass=MOV_cpy_z_p_r_cpy_z_p_r_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
  eq_to      => 'CPY     <Zd>.<T>, <Pg>/M, <R><n|SP>',
};

ENCODING MOV_cpy_z_p_v_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b100000100 Pg:u=0bxxx Vn:u=0bxxxxx Zd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=cpy_z_p_v_ alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_cpy_v page=MOV_cpy_z_p_v cclass=MOV_cpy_z_p_v_cpy_z_p_v_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
  eq_to      => 'CPY     <Zd>.<T>, <Pg>/M, <V><n>',
};

ENCODING MOV_dup_z_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b111 opc=0b00 ig2=0b011 sh:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=dup_z_i_ alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_dup_imm page=MOV_dup_z_i cclass=MOV_dup_z_i_dup_z_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER|SIGNED',
  eq_to      => 'DUP     <Zd>.<T>, #<imm>{, <shift>}',
};

ENCODING MOV_dup_z_r_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b100000001110 Rn:u=0bxxxxx Zd:u=0bxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=dup_z_r_ alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_dup_r page=MOV_dup_z_r cclass=MOV_dup_z_r_dup_z_r_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
  eq_to      => 'DUP     <Zd>.<T>, <R><n|SP>',
};

ENCODING MOV_dup_z_zi_ => {
  diagram    => 'ig0=0b00000101 imm2:u=0bxx ig1=0b1 tsz:u=0bxxxxx ig2=0b001000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  aliascond  => 'BitCount(imm2:tsz) > 1',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=dup_z_zi_ alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_dup_i page=MOV_dup_z_zi cclass=MOV_dup_z_zi_dup_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
  eq_to      => 'DUP     <Zd>.<T>, <Zn>.<T>[<imm>]',
};

ENCODING MOV_dup_z_zi_ => {
  diagram    => 'ig0=0b00000101 imm2:u=0bxx ig1=0b1 tsz:u=0bxxxxx ig2=0b001000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  aliascond  => 'BitCount(imm2:tsz) == 1',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=dup_z_zi_ alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_dup_i page=MOV_dup_z_zi cclass=MOV_dup_z_zi_dup_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
  eq_to      => 'DUP <Zd>.<T>, <Zn>.<T>[0]',
};

ENCODING MOV_dupm_z_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101110000 imm13:u=0bxxxxxxxxxxxxx Zd:u=0bxxxxx',
  aliascond  => 'SVEMoveMaskPreferred(imm13)',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=DUPM status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=dupm_z_i_ alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_maskimm iclass=sve_int_dup_mask_imm page=MOV_dupm_z_i cclass=MOV_dupm_z_i_dupm_z_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
  eq_to      => 'DUPM    <Zd>.<T>, #<const>',
};

ENCODING MOV_orr_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S:u=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  aliascond  => 'S == 0b0 && Pn == Pm && Pm == Pg',
  docvars    => 'alias_mnemonic=MOV cond-setting=no-s instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=orr_p_p_pp_z alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=MOV_orr_p_p_pp cclass=MOV_orr_p_p_pp_orr_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|DATA_TRANSFER',
  eq_to      => 'ORR <Pd>.B, <Pn>/Z, <Pn>.B, <Pn>.B',
};

ENCODING MOV_orr_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 opc=0b01 ig1=0b1 Zm:u=0bxxxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  aliascond  => 'Zn == Zm',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=orr_z_zz_ alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_bin_cons_log page=MOV_orr_z_zz cclass=MOV_orr_z_zz_orr_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
  eq_to      => 'ORR <Zd>.D, <Zn>.D, <Zn>.D',
};

ENCODING MOV_sel_p_p_pp_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b1 Pd:u=0bxxxx',
  aliascond  => 'Pd == Pm',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=SEL status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=sel_p_p_pp_ alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=MOV_sel_p_p_pp cclass=MOV_sel_p_p_pp_sel_p_p_pp_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
  eq_to      => 'SEL <Pd>.B, <Pg>, <Pn>.B, <Pd>.B',
};

ENCODING MOV_sel_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b11 Pg:u=0bxxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  aliascond  => 'Zd == Zm',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=SEL status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=sel_z_p_zz_ alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_sel_vvv page=MOV_sel_z_p_zz cclass=MOV_sel_z_p_zz_sel_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|DATA_TRANSFER',
  eq_to      => 'SEL <Zd>.<T>, <Pg>, <Zn>.<T>, <Zd>.<T>',
};

ENCODING MOVS_ands_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S:u=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  aliascond  => 'S == 0b1 && Pn == Pm',
  docvars    => 'alias_mnemonic=MOVS cond-setting=s instr-class=sve isa=A64 mnemonic=ANDS status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=ands_p_p_pp_z alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=MOVS_and_p_p_pp cclass=MOVS_and_p_p_pp_ands_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|DATA_TRANSFER',
  eq_to      => 'ANDS <Pd>.B, <Pg>/Z, <Pn>.B, <Pn>.B',
};

ENCODING MOVS_orrs_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S:u=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  aliascond  => 'S == 0b1 && Pn == Pm && Pm == Pg',
  docvars    => 'alias_mnemonic=MOVS cond-setting=s instr-class=sve isa=A64 mnemonic=ORRS status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=orrs_p_p_pp_z alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=MOVS_orr_p_p_pp cclass=MOVS_orr_p_p_pp_orrs_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|DATA_TRANSFER',
  eq_to      => 'ORRS <Pd>.B, <Pn>/Z, <Pn>.B, <Pn>.B',
};

ENCODING NOT_eor_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  aliascond  => 'Pm == Pg',
  docvars    => 'alias_mnemonic=NOT cond-setting=no-s instr-class=sve isa=A64 mnemonic=EOR status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=eor_p_p_pp_z alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=NOT_eor_p_p_pp cclass=NOT_eor_p_p_pp_eor_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'SVE|BITWISE',
  eq_to      => 'EOR <Pd>.B, <Pg>/Z, <Pn>.B, <Pg>.B',
};

ENCODING NOTS_eors_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  aliascond  => 'Pm == Pg',
  docvars    => 'alias_mnemonic=NOTS cond-setting=s instr-class=sve isa=A64 mnemonic=EORS status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=eors_p_p_pp_z alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=NOTS_eor_p_p_pp cclass=NOTS_eor_p_p_pp_eors_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'SVE|BITWISE',
  eq_to      => 'EORS <Pd>.B, <Pg>/Z, <Pn>.B, <Pg>.B',
};

ENCODING ORN_orr_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 opc=0b00 ig1=0b0000 imm13:u=0bxxxxxxxxxxxxx Zdn:u=0bxxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=ORN instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 aliasof=orr_z_zi_ alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_maskimm iclass=sve_int_log_imm page=ORN_orr_z_zi cclass=ORN_orr_z_zi_orr_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  categories => 'SVE|BITWISE',
  eq_to      => 'ORR <Zdn>.<T>, <Zdn>.<T>, #(-<const> - 1)',
};


1;

