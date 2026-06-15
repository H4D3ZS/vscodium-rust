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

CLASS abs_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 opc=0b110 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ABS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_0 page=abs_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS adclb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 ig1=0b0 sz:u=0bx ig2=0b0 Zm:u=0bxxxxx ig3=0b11010 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=ADCLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_adc_long page=adclb_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS adclt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 ig1=0b0 sz:u=0bx ig2=0b0 Zm:u=0bxxxxx ig3=0b11010 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=ADCLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_adc_long page=adclt_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS add_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b000 opc=0b000 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_0 page=add_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS add_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 opc=0b000 ig2=0b11 sh:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm0 page=add_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size:sh == 1)',
};

CLASS add_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b000 opc=0b000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_bin_cons_arit_0 page=add_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS addhnb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S=0b0 R=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=ADDHNB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_arith_narrow page=addhnb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS addhnt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S=0b0 R=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=ADDHNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_arith_narrow page=addhnt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS addp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 opc=0b00 U=0b1 ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=ADDP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_arith_binary_pairs page=addp_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS addpl_r_ri_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b000001000 op=0b1 ig1=0b1 Rn:u=0bxxxxx ig2=0b01010 imm6:u=0bxxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADDPL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_alloca iclass=sve_int_arith_vl page=addpl_r_ri',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS addvl_r_ri_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b000001000 op=0b0 ig1=0b1 Rn:u=0bxxxxx ig2=0b01010 imm6:u=0bxxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADDVL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_alloca iclass=sve_int_arith_vl page=addvl_r_ri',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS adr_z_az_sd_same_scaled => {
  name       => 'Packed offsets',
  diagram    => 'ig0=0b00000100 ig1=0b1 sz:u=0bx ig2=0b1 Zm:u=0bxxxxx ig3=0b1010 msz:u=0bxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADR status=Green sve-offset-type=off_pkd',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_bin_cons_misc_0_a page=adr_z_az',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS adr_z_az_d_s32_scaled => {
  name       => 'Unpacked 32-bit signed offsets',
  diagram    => 'ig0=0b00000100 opc=0b00 ig1=0b1 Zm:u=0bxxxxx ig2=0b1010 msz:u=0bxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADR status=Green sve-offset-type=off_s_s32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_bin_cons_misc_0_a page=adr_z_az',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS adr_z_az_d_u32_scaled => {
  name       => 'Unpacked 32-bit unsigned offsets',
  diagram    => 'ig0=0b00000100 opc=0b01 ig1=0b1 Zm:u=0bxxxxx ig2=0b1010 msz:u=0bxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADR status=Green sve-offset-type=off_s_u32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_bin_cons_misc_0_a page=adr_z_az',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS aesd_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size=0b00 ig1=0b10001 op=0b0 ig2=0b11100 o2=0b1 Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=AESD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_crypto iclass=sve_crypto_binary_dest page=aesd_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2AES()',
};

CLASS aese_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size=0b00 ig1=0b10001 op=0b0 ig2=0b11100 o2=0b0 Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=AESE status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_crypto iclass=sve_crypto_binary_dest page=aese_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2AES()',
};

CLASS aesimc_z_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size=0b00 ig1=0b10000011100 op=0b1 ig2=0b00000 Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=AESIMC status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_crypto iclass=sve_crypto_unary page=aesimc_z_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2AES()',
};

CLASS aesmc_z_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size=0b00 ig1=0b10000011100 op=0b0 ig2=0b00000 Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=AESMC status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_crypto iclass=sve_crypto_unary page=aesmc_z_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2AES()',
};

CLASS and_p_p_pp_and_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S:u=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=AND status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=and_p_p_pp xid=and_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS and_p_p_pp_ands_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S:u=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=ANDS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=and_p_p_pp xid=ands_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS and_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b010 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=AND status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_log page=and_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS and_z_zi_and_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 opc=0b10 ig1=0b0000 imm13:u=0bxxxxxxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=AND status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_maskimm iclass=sve_int_log_imm page=and_z_zi xid=and_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS and_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 opc=0b00 ig1=0b1 Zm:u=0bxxxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=AND status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_bin_cons_log page=and_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS andv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b010 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ANDV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_2 page=andv_r_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS asr_z_p_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b00 L=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=asr_z_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS asr_z_p_zw_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 R=0b0 L=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_2 page=asr_z_p_zw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 3)',
};

CLASS asr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 R=0b0 L=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_1 page=asr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS asr_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b1001 ig3=0b0 U=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_shift iclass=sve_int_bin_cons_shift_b page=asr_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS asr_z_zw_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b1000 ig3=0b0 U=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_shift iclass=sve_int_bin_cons_shift_a page=asr_z_zw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 3)',
};

CLASS asrd_z_p_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b01 L=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASRD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=asrd_z_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS asrr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 R=0b1 L=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASRR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_1 page=asrr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS bcax_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 opc=0b01 ig1=0b1 Zm:u=0bxxxxx ig2=0b00111 o2=0b0 Zk:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BCAX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_tern_log page=bcax_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS bdep_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1011 opc=0b01 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BDEP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_perm_bit page=bdep_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2BitPerm()',
};

CLASS bext_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1011 opc=0b00 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BEXT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_perm_bit page=bext_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2BitPerm()',
};

CLASS bfcvt_z_p_z_s2bf => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 opc=0b10 ig1=0b0010 opc2=0b10 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFCVT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_0 page=bfcvt_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE() || !HaveBF16Ext()',
};

CLASS bfcvtnt_z_p_z_s2bf => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100100 opc=0b10 ig1=0b0010 opc2=0b10 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFCVTNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fcvt2 page=bfcvtnt_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE() || !HaveBF16Ext()',
};

CLASS bfdot_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b011001000 op=0b1 ig1=0b1 Zm:u=0bxxxxx ig2=0b100000 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long iclass=sve_fp_fdot page=bfdot_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE() || !HaveBF16Ext()',
};

CLASS bfdot_z_zzzi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b011001000 op=0b1 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b010000 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long_by_indexed_elem iclass=sve_fp_fdot_by_indexed_elem page=bfdot_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE() || !HaveBF16Ext()',
};

CLASS bfmlalb_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b011001001 o2=0b1 ig1=0b1 Zm:u=0bxxxxx ig2=0b10 op=0b0 ig3=0b00 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFMLALB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long iclass=sve_fp_fma_long page=bfmlalb_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE() || !HaveBF16Ext()',
};

CLASS bfmlalb_z_zzzi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b011001001 o2=0b1 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b01 op=0b0 ig3=0b0 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFMLALB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long_by_indexed_elem iclass=sve_fp_fma_long_by_indexed_elem page=bfmlalb_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE() || !HaveBF16Ext()',
};

CLASS bfmlalt_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b011001001 o2=0b1 ig1=0b1 Zm:u=0bxxxxx ig2=0b10 op=0b0 ig3=0b00 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFMLALT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long iclass=sve_fp_fma_long page=bfmlalt_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE() || !HaveBF16Ext()',
};

CLASS bfmlalt_z_zzzi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b011001001 o2=0b1 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b01 op=0b0 ig3=0b0 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFMLALT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long_by_indexed_elem iclass=sve_fp_fma_long_by_indexed_elem page=bfmlalt_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE() || !HaveBF16Ext()',
};

CLASS bfmmla_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100100 opc=0b01 ig1=0b1 Zm:u=0bxxxxx ig2=0b111001 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFMMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fmmla page=bfmmla_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE() || !HaveBF16Ext()',
};

CLASS bgrp_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1011 opc=0b10 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BGRP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_perm_bit page=bgrp_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2BitPerm()',
};

CLASS bic_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=BIC status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=bic_p_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS bics_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=BICS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=bic_p_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS bic_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b011 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BIC status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_log page=bic_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS bic_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 opc=0b11 ig1=0b1 Zm:u=0bxxxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BIC status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_bin_cons_log page=bic_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS brka_p_p_p_ => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 B=0b0 S=0b0 ig1=0b01000001 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx M:u=0bx Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=BRKA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_c iclass=sve_int_break page=brka_p_p_p',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS brkas_p_p_p_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 B=0b0 S=0b1 ig1=0b01000001 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx M=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=BRKAS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_c iclass=sve_int_break page=brka_p_p_p',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS brkb_p_p_p_ => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 B=0b1 S=0b0 ig1=0b01000001 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx M:u=0bx Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=BRKB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_c iclass=sve_int_break page=brkb_p_p_p',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS brkbs_p_p_p_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 B=0b1 S=0b1 ig1=0b01000001 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx M=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=BRKBS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_c iclass=sve_int_break page=brkb_p_p_p',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS brkn_p_p_pp_ => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b001001010 S=0b0 ig1=0b01100001 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx ig3=0b0 Pdm:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=BRKN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_c iclass=sve_int_brkn page=brkn_p_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS brkns_p_p_pp_ => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b001001010 S=0b1 ig1=0b01100001 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx ig3=0b0 Pdm:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=BRKNS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_c iclass=sve_int_brkn page=brkn_p_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS brkpa_p_p_pp_ => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b11 Pg:u=0bxxxx ig3=0b0 Pn:u=0bxxxx B=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=BRKPA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_b iclass=sve_int_brkp page=brkpa_p_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS brkpas_p_p_pp_ => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b11 Pg:u=0bxxxx ig3=0b0 Pn:u=0bxxxx B=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=BRKPAS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_b iclass=sve_int_brkp page=brkpa_p_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS brkpb_p_p_pp_ => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b11 Pg:u=0bxxxx ig3=0b0 Pn:u=0bxxxx B=0b1 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=BRKPB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_b iclass=sve_int_brkp page=brkpb_p_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS brkpbs_p_p_pp_ => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b11 Pg:u=0bxxxx ig3=0b0 Pn:u=0bxxxx B=0b1 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=BRKPBS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_b iclass=sve_int_brkp page=brkpb_p_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS bsl1n_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 opc=0b01 ig1=0b1 Zm:u=0bxxxxx ig2=0b00111 o2=0b1 Zk:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BSL1N status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_tern_log page=bsl1n_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS bsl2n_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 opc=0b10 ig1=0b1 Zm:u=0bxxxxx ig2=0b00111 o2=0b1 Zk:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BSL2N status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_tern_log page=bsl2n_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS bsl_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 opc=0b00 ig1=0b1 Zm:u=0bxxxxx ig2=0b00111 o2=0b1 Zk:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BSL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_tern_log page=bsl_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS cadd_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b00000 op=0b0 ig2=0b11011 rot:u=0bx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=CADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_cadd page=cadd_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS cdot_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0001 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=CDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_cdot page=cdot_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0b0x)',
};

CLASS cdot_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b0100 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=CDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_cdot_by_indexed_elem page=cdot_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS cdot_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b0100 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=CDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_cdot_by_indexed_elem page=cdot_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS clasta_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b11000 B=0b0 ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_clast_rz page=clasta_r_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS clasta_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10101 B=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Vdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_clast_vz page=clasta_v_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS clasta_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10100 B=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_clast_zz page=clasta_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS clastb_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b11000 B=0b1 ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_clast_rz page=clastb_r_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS clastb_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10101 B=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Vdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_clast_vz page=clastb_v_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS clastb_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10100 B=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_clast_zz page=clastb_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cls_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b000 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_1 page=cls_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS clz_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b001 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLZ status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_1 page=clz_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cmla_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b001 op=0b0 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=CMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_cmla page=cmla_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS cmla_z_zzzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b0110 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=CMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_cmla_by_indexed_elem page=cmla_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS cmla_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b0110 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=CMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_cmla_by_indexed_elem page=cmla_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS cmpeq_p_p_zi_ => {
  name       => 'Equal',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b0 imm5:u=0bxxxxx op=0b1 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPEQ status=Green sve-compare-type=eq',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_scmp_vi page=cmpeq_p_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cmpgt_p_p_zi_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b0 imm5:u=0bxxxxx op=0b0 ig2=0b0 lt=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPGT status=Green sve-compare-type=gt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_scmp_vi page=cmpeq_p_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cmpge_p_p_zi_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b0 imm5:u=0bxxxxx op=0b0 ig2=0b0 lt=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPGE status=Green sve-compare-type=ge',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_scmp_vi page=cmpeq_p_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cmphi_p_p_zi_ => {
  name       => 'Higher',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b1 imm7:u=0bxxxxxxx lt=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPHI status=Green sve-compare-type=hi',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_ucmp_vi page=cmpeq_p_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cmphs_p_p_zi_ => {
  name       => 'Higher or same',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b1 imm7:u=0bxxxxxxx lt=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPHS status=Green sve-compare-type=hs',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_ucmp_vi page=cmpeq_p_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cmplt_p_p_zi_ => {
  name       => 'Less than',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b0 imm5:u=0bxxxxx op=0b0 ig2=0b0 lt=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPLT status=Green sve-compare-type=lt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_scmp_vi page=cmpeq_p_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cmple_p_p_zi_ => {
  name       => 'Less than or equal',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b0 imm5:u=0bxxxxx op=0b0 ig2=0b0 lt=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPLE status=Green sve-compare-type=le',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_scmp_vi page=cmpeq_p_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cmplo_p_p_zi_ => {
  name       => 'Lower',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b1 imm7:u=0bxxxxxxx lt=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPLO status=Green sve-compare-type=lo',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_ucmp_vi page=cmpeq_p_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cmpls_p_p_zi_ => {
  name       => 'Lower or same',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b1 imm7:u=0bxxxxxxx lt=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPLS status=Green sve-compare-type=ls',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_ucmp_vi page=cmpeq_p_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cmpne_p_p_zi_ => {
  name       => 'Not equal',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b0 imm5:u=0bxxxxx op=0b1 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPNE status=Green sve-compare-type=ne',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_scmp_vi page=cmpeq_p_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cmpeq_p_p_zw_ => {
  name       => 'Equal',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b0 o2=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPEQ status=Green sve-compare-type=eq',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=cmpeq_p_p_zw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 3)',
};

CLASS cmpgt_p_p_zw_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U=0b0 ig2=0b1 lt=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPGT status=Green sve-compare-type=gt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_1 page=cmpeq_p_p_zw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 3)',
};

CLASS cmpge_p_p_zw_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U=0b0 ig2=0b1 lt=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPGE status=Green sve-compare-type=ge',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_1 page=cmpeq_p_p_zw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 3)',
};

CLASS cmphi_p_p_zw_ => {
  name       => 'Higher',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U=0b1 ig2=0b1 lt=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPHI status=Green sve-compare-type=hi',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_1 page=cmpeq_p_p_zw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 3)',
};

CLASS cmphs_p_p_zw_ => {
  name       => 'Higher or same',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U=0b1 ig2=0b1 lt=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPHS status=Green sve-compare-type=hs',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_1 page=cmpeq_p_p_zw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 3)',
};

CLASS cmplt_p_p_zw_ => {
  name       => 'Less than',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U=0b0 ig2=0b1 lt=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPLT status=Green sve-compare-type=lt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_1 page=cmpeq_p_p_zw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 3)',
};

CLASS cmple_p_p_zw_ => {
  name       => 'Less than or equal',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U=0b0 ig2=0b1 lt=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPLE status=Green sve-compare-type=le',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_1 page=cmpeq_p_p_zw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 3)',
};

CLASS cmplo_p_p_zw_ => {
  name       => 'Lower',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U=0b1 ig2=0b1 lt=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPLO status=Green sve-compare-type=lo',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_1 page=cmpeq_p_p_zw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 3)',
};

CLASS cmpls_p_p_zw_ => {
  name       => 'Lower or same',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U=0b1 ig2=0b1 lt=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPLS status=Green sve-compare-type=ls',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_1 page=cmpeq_p_p_zw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 3)',
};

CLASS cmpne_p_p_zw_ => {
  name       => 'Not equal',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b0 o2=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPNE status=Green sve-compare-type=ne',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=cmpeq_p_p_zw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 3)',
};

CLASS cmpeq_p_p_zz_ => {
  name       => 'Equal',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b0 o2=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPEQ status=Green sve-compare-type=eq',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=cmpeq_p_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cmpeq_p_p_zz_cmpgt_p_p_zz_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPGT status=Green sve-compare-type=gt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=cmpeq_p_p_zz xid=cmpgt_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cmpeq_p_p_zz_cmpge_p_p_zz_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPGE status=Green sve-compare-type=ge',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=cmpeq_p_p_zz xid=cmpge_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cmpeq_p_p_zz_cmphi_p_p_zz_ => {
  name       => 'Higher',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPHI status=Green sve-compare-type=hi',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=cmpeq_p_p_zz xid=cmphi_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cmpeq_p_p_zz_cmphs_p_p_zz_ => {
  name       => 'Higher or same',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPHS status=Green sve-compare-type=hs',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=cmpeq_p_p_zz xid=cmphs_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cmpne_p_p_zz_ => {
  name       => 'Not equal',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b0 o2=0b1 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPNE status=Green sve-compare-type=ne',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=cmpeq_p_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cnot_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b011 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CNOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_1 page=cnot_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cnt_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b010 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_1 page=cnt_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cntb_r_s_ => {
  name       => 'Byte',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b10 imm4:u=0bxxxx ig2=0b11100 op=0b0 pattern:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CNTB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_count page=cntb_r_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cntd_r_s_ => {
  name       => 'Doubleword',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b10 imm4:u=0bxxxx ig2=0b11100 op=0b0 pattern:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CNTD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_count page=cntb_r_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cnth_r_s_ => {
  name       => 'Halfword',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b10 imm4:u=0bxxxx ig2=0b11100 op=0b0 pattern:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CNTH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_count page=cntb_r_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cntw_r_s_ => {
  name       => 'Word',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b10 imm4:u=0bxxxx ig2=0b11100 op=0b0 pattern:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CNTW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_count page=cntb_r_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cntp_r_p_p_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 opc=0b000 ig2=0b10 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CNTP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pcount_pred page=cntp_r_p_p',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS compact_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b100001100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=COMPACT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_compact page=compact_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0b0x)',
};

CLASS cpy_z_o_i_cpy_z_o_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b0 M=0b0 sh:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_pred iclass=sve_int_dup_imm_pred page=cpy_z_o_i xid=cpy_z_o_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size:sh == 1)',
};

CLASS cpy_z_p_i_cpy_z_p_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b0 M=0b1 sh:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_wideimm_pred iclass=sve_int_dup_imm_pred page=cpy_z_p_i xid=cpy_z_p_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size:sh == 1)',
};

CLASS cpy_z_p_r_cpy_z_p_r_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b101000101 Pg:u=0bxxx Rn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_cpy_r page=cpy_z_p_r xid=cpy_z_p_r_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS cpy_z_p_v_cpy_z_p_v_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b100000100 Pg:u=0bxxx Vn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_cpy_v page=cpy_z_p_v xid=cpy_z_p_v_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ctermeq_rr_ => {
  name       => 'Equal',
  diagram    => 'ig0=0b00 ig1=0b100101 op=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b001000 Rn:u=0bxxxxx ne=0b0 ig4=0b0 ig5=0b0 ig6=0b00',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CTERMEQ status=Green sve-compare-type=eq',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_cterm page=ctermeq_rr',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ctermne_rr_ => {
  name       => 'Not equal',
  diagram    => 'ig0=0b00 ig1=0b100101 op=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b001000 Rn:u=0bxxxxx ne=0b1 ig4=0b0 ig5=0b0 ig6=0b00',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CTERMNE status=Green sve-compare-type=ne',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_cterm page=ctermeq_rr',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS decb_r_rs_ => {
  name       => 'Byte',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_a page=decb_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS decd_r_rs_ => {
  name       => 'Doubleword',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_a page=decb_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS dech_r_rs_ => {
  name       => 'Halfword',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_a page=decb_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS decw_r_rs_ => {
  name       => 'Word',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_a page=decb_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS decd_z_zs_ => {
  name       => 'Doubleword',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b11 imm4:u=0bxxxx ig2=0b11000 D=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv1 page=decd_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS dech_z_zs_ => {
  name       => 'Halfword',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b11 imm4:u=0bxxxx ig2=0b11000 D=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv1 page=decd_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS decw_z_zs_ => {
  name       => 'Word',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b11 imm4:u=0bxxxx ig2=0b11000 D=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv1 page=decd_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS decp_r_p_r_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1011 op=0b0 D=0b1 ig2=0b10001 opc2=0b00 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r page=decp_r_p_r',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS decp_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1011 op=0b0 D=0b1 ig2=0b10000 opc2=0b00 Pm:u=0bxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_v page=decp_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS dup_z_i_dup_z_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b111 opc=0b00 ig2=0b011 sh:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1 broadcast=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_dup_imm page=dup_z_i xid=dup_z_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size:sh == 1)',
};

CLASS dup_z_r_dup_z_r_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b100000001110 Rn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1 broadcast=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_dup_r page=dup_z_r xid=dup_z_r_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS dup_z_zi_dup_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 imm2:u=0bxx ig1=0b1 tsz:u=0bxxxxx ig2=0b001000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1 broadcast=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_dup_i page=dup_z_zi xid=dup_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS dupm_z_i_dupm_z_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101110000 imm13:u=0bxxxxxxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DUPM status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1 broadcast=1',
  tags       => 'group=sve_maskimm iclass=sve_int_dup_mask_imm page=dupm_z_i xid=dupm_z_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS eor3_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 opc=0b00 ig1=0b1 Zm:u=0bxxxxx ig2=0b00111 o2=0b0 Zk:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=EOR3 status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_tern_log page=eor3_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS eor_p_p_pp_eor_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=EOR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=eor_p_p_pp xid=eor_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS eor_p_p_pp_eors_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=EORS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=eor_p_p_pp xid=eors_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS eor_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b001 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EOR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_log page=eor_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS eor_z_zi_eor_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 opc=0b01 ig1=0b0000 imm13:u=0bxxxxxxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EOR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_maskimm iclass=sve_int_log_imm page=eor_z_zi xid=eor_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS eor_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 opc=0b10 ig1=0b1 Zm:u=0bxxxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EOR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_bin_cons_log page=eor_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS eorbt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b10010 tb=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=EORBT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_eorx page=eorbt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS eortb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b10010 tb=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=EORTB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_eorx page=eortb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS eorv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b001 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EORV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_2 page=eorv_r_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ext_z_zi_con => {
  name       => 'Constructive',
  diagram    => 'ig0=0b00000101011 imm8h:u=0bxxxxx ig1=0b000 imm8l:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EXT status=Green sve-dest_const=sve-const',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_extract iclass=sve_intx_perm_extract_i page=ext_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ext_z_zi_des => {
  name       => 'Destructive',
  diagram    => 'ig0=0b00000101001 imm8h:u=0bxxxxx ig1=0b000 imm8l:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EXT status=Green sve-dest_const=sve-dest',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_extract iclass=sve_int_perm_extract_i page=ext_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fabd_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b1000 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FABD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fabd_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fabs_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b100 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FABS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_1 page=fabs_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS facge_p_p_zz_facgt_p_p_zz_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b1 o2=0b1 Pg:u=0bxxx Zn:u=0bxxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FACGT status=Green sve-compare-type=gt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=facge_p_p_zz xid=facgt_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS facge_p_p_zz_facge_p_p_zz_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b1 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FACGE status=Green sve-compare-type=ge',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=facge_p_p_zz xid=facge_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fadd_z_p_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b000 ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_i_p_zds page=fadd_z_p_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fadd_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b0000 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fadd_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fadd_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b000 opc=0b000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_u_zd page=fadd_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fadda_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b000 ig2=0b001 Pg:u=0bxxx Zm:u=0bxxxxx Vdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FADDA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_2op_p_vd page=fadda_v_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS faddp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b010 opc=0b000 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FADDP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_pairwise page=faddp_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS faddv_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b000 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FADDV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fast_red page=faddv_v_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fcadd_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b00000 rot:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve iclass=sve_fp_fcadd page=fcadd_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fcmeq_p_p_z0_ => {
  name       => 'Equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0100 eq=0b1 lt=0b0 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMEQ status=Green sve-compare-type=eq',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_cmpzero iclass=sve_fp_2op_p_pd page=fcmeq_p_p_z0',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fcmgt_p_p_z0_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0100 eq=0b0 lt=0b0 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMGT status=Green sve-compare-type=gt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_cmpzero iclass=sve_fp_2op_p_pd page=fcmeq_p_p_z0',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fcmge_p_p_z0_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0100 eq=0b0 lt=0b0 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMGE status=Green sve-compare-type=ge',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_cmpzero iclass=sve_fp_2op_p_pd page=fcmeq_p_p_z0',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fcmlt_p_p_z0_ => {
  name       => 'Less than',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0100 eq=0b0 lt=0b1 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMLT status=Green sve-compare-type=lt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_cmpzero iclass=sve_fp_2op_p_pd page=fcmeq_p_p_z0',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fcmle_p_p_z0_ => {
  name       => 'Less than or equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0100 eq=0b0 lt=0b1 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMLE status=Green sve-compare-type=le',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_cmpzero iclass=sve_fp_2op_p_pd page=fcmeq_p_p_z0',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fcmne_p_p_z0_ => {
  name       => 'Not equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0100 eq=0b1 lt=0b1 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMNE status=Green sve-compare-type=ne',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_cmpzero iclass=sve_fp_2op_p_pd page=fcmeq_p_p_z0',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fcmeq_p_p_zz_ => {
  name       => 'Equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b1 cmph=0b1 Pg:u=0bxxx Zn:u=0bxxxxx cmpl=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMEQ status=Green sve-compare-type=eq',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=fcmeq_p_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fcmeq_p_p_zz_fcmgt_p_p_zz_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b1 cmph=0b0 Pg:u=0bxxx Zn:u=0bxxxxx cmpl=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMGT status=Green sve-compare-type=gt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=fcmeq_p_p_zz xid=fcmgt_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fcmeq_p_p_zz_fcmge_p_p_zz_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b1 cmph=0b0 Pg:u=0bxxx Zn:u=0bxxxxx cmpl=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMGE status=Green sve-compare-type=ge',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=fcmeq_p_p_zz xid=fcmge_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fcmne_p_p_zz_ => {
  name       => 'Not equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b1 cmph=0b1 Pg:u=0bxxx Zn:u=0bxxxxx cmpl=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMNE status=Green sve-compare-type=ne',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=fcmeq_p_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fcmuo_p_p_zz_ => {
  name       => 'Unordered',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b1 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMUO status=Green sve-compare-type=uo',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=fcmeq_p_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fcmla_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 rot:u=0bxx Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve iclass=sve_fp_fcmla page=fcmla_z_p_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fcmla_z_zzzi_h => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01100100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b0001 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=sve isa=A64 mnemonic=FCMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fcmla_by_indexed_elem page=fcmla_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcmla_z_zzzi_s => {
  name       => 'Single-precision',
  diagram    => 'ig0=0b01100100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b0001 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=single instr-class=sve isa=A64 mnemonic=FCMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fcmla_by_indexed_elem page=fcmla_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcpy_z_p_i_fcpy_z_p_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b110 imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCPY status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_wideimm_pred iclass=sve_int_dup_fpimm_pred page=fcpy_z_p_i xid=fcpy_z_p_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fcvt_z_p_z_h2s => {
  name       => 'Half-precision to single-precision',
  diagram    => 'ig0=0b01100101 opc=0b10 ig1=0b0010 opc2=0b01 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-single instr-class=sve isa=A64 mnemonic=FCVT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_0 page=fcvt_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvt_z_p_z_h2d => {
  name       => 'Half-precision to double-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b0010 opc2=0b01 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-double instr-class=sve isa=A64 mnemonic=FCVT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_0 page=fcvt_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvt_z_p_z_s2h => {
  name       => 'Single-precision to half-precision',
  diagram    => 'ig0=0b01100101 opc=0b10 ig1=0b0010 opc2=0b00 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-half instr-class=sve isa=A64 mnemonic=FCVT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_0 page=fcvt_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvt_z_p_z_s2d => {
  name       => 'Single-precision to double-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b0010 opc2=0b11 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-double instr-class=sve isa=A64 mnemonic=FCVT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_0 page=fcvt_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvt_z_p_z_d2h => {
  name       => 'Double-precision to half-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b0010 opc2=0b00 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-half instr-class=sve isa=A64 mnemonic=FCVT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_0 page=fcvt_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvt_z_p_z_d2s => {
  name       => 'Double-precision to single-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b0010 opc2=0b10 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-single instr-class=sve isa=A64 mnemonic=FCVT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_0 page=fcvt_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvtlt_z_p_z_h2s => {
  name       => 'Half-precision to single-precision',
  diagram    => 'ig0=0b01100100 opc=0b10 ig1=0b0010 opc2=0b01 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-single instr-class=sve2 isa=A64 mnemonic=FCVTLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fcvt2 page=fcvtlt_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS fcvtlt_z_p_z_s2d => {
  name       => 'Single-precision to double-precision',
  diagram    => 'ig0=0b01100100 opc=0b11 ig1=0b0010 opc2=0b11 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-double instr-class=sve2 isa=A64 mnemonic=FCVTLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fcvt2 page=fcvtlt_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS fcvtnt_z_p_z_s2h => {
  name       => 'Single-precision to half-precision',
  diagram    => 'ig0=0b01100100 opc=0b10 ig1=0b0010 opc2=0b00 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-half instr-class=sve2 isa=A64 mnemonic=FCVTNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fcvt2 page=fcvtnt_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS fcvtnt_z_p_z_d2s => {
  name       => 'Double-precision to single-precision',
  diagram    => 'ig0=0b01100100 opc=0b11 ig1=0b0010 opc2=0b10 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-single instr-class=sve2 isa=A64 mnemonic=FCVTNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fcvt2 page=fcvtnt_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS fcvtx_z_p_z_d2s => {
  name       => 'Double-precision to single-precision',
  diagram    => 'ig0=0b01100101 opc=0b00 ig1=0b0010 opc2=0b10 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-single instr-class=sve2 isa=A64 mnemonic=FCVTX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_0 page=fcvtx_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS fcvtxnt_z_p_z_d2s => {
  name       => 'Double-precision to single-precision',
  diagram    => 'ig0=0b01100100 opc=0b00 ig1=0b0010 opc2=0b10 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-single instr-class=sve2 isa=A64 mnemonic=FCVTXNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fcvt2 page=fcvtxnt_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS fcvtzs_z_p_z_fp162h => {
  name       => 'Half-precision to 16-bit',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b011 opc2=0b01 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-16 instr-class=sve isa=A64 mnemonic=FCVTZS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzs_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvtzs_z_p_z_fp162w => {
  name       => 'Half-precision to 32-bit',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b011 opc2=0b10 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-32 instr-class=sve isa=A64 mnemonic=FCVTZS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzs_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvtzs_z_p_z_fp162x => {
  name       => 'Half-precision to 64-bit',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b011 opc2=0b11 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-64 instr-class=sve isa=A64 mnemonic=FCVTZS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzs_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvtzs_z_p_z_s2w => {
  name       => 'Single-precision to 32-bit',
  diagram    => 'ig0=0b01100101 opc=0b10 ig1=0b011 opc2=0b10 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-32 instr-class=sve isa=A64 mnemonic=FCVTZS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzs_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvtzs_z_p_z_s2x => {
  name       => 'Single-precision to 64-bit',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b011 opc2=0b10 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-64 instr-class=sve isa=A64 mnemonic=FCVTZS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzs_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvtzs_z_p_z_d2w => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b011 opc2=0b00 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-32 instr-class=sve isa=A64 mnemonic=FCVTZS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzs_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvtzs_z_p_z_d2x => {
  name       => 'Double-precision to 64-bit',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b011 opc2=0b11 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-64 instr-class=sve isa=A64 mnemonic=FCVTZS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzs_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvtzu_z_p_z_fp162h => {
  name       => 'Half-precision to 16-bit',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b011 opc2=0b01 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-16 instr-class=sve isa=A64 mnemonic=FCVTZU status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzu_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvtzu_z_p_z_fp162w => {
  name       => 'Half-precision to 32-bit',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b011 opc2=0b10 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-32 instr-class=sve isa=A64 mnemonic=FCVTZU status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzu_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvtzu_z_p_z_fp162x => {
  name       => 'Half-precision to 64-bit',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b011 opc2=0b11 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=half-to-64 instr-class=sve isa=A64 mnemonic=FCVTZU status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzu_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvtzu_z_p_z_s2w => {
  name       => 'Single-precision to 32-bit',
  diagram    => 'ig0=0b01100101 opc=0b10 ig1=0b011 opc2=0b10 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-32 instr-class=sve isa=A64 mnemonic=FCVTZU status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzu_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvtzu_z_p_z_s2x => {
  name       => 'Single-precision to 64-bit',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b011 opc2=0b10 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-64 instr-class=sve isa=A64 mnemonic=FCVTZU status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzu_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvtzu_z_p_z_d2w => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b011 opc2=0b00 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-32 instr-class=sve isa=A64 mnemonic=FCVTZU status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzu_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fcvtzu_z_p_z_d2x => {
  name       => 'Double-precision to 64-bit',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b011 opc2=0b11 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-64 instr-class=sve isa=A64 mnemonic=FCVTZU status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=fcvtzu_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fdiv_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b1101 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FDIV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fdiv_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fdivr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b1100 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FDIVR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fdivr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fdup_z_i_fdup_z_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b111 opc=0b00 ig2=0b111 o2=0b0 imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FDUP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 broadcast=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_dup_fpimm page=fdup_z_i xid=fdup_z_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fexpa_z_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 opc=0b00000 ig2=0b101110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FEXPA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_int_unpred_misc iclass=sve_int_bin_cons_misc_0_c page=fexpa_z_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS flogb_z_p_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01100101 opc=0b00 ig1=0b011 size:u=0bxx U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FLOGB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_d page=flogb_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS fmad_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Za:u=0bxxxxx ig2=0b1 N=0b0 op=0b0 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_fma iclass=sve_fp_3op_p_zds_b page=fmad_z_p_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fmax_z_p_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b110 ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_i_p_zds page=fmax_z_p_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fmax_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b0110 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fmax_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fmaxnm_z_p_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b100 ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAXNM status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_i_p_zds page=fmaxnm_z_p_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fmaxnm_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b0100 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAXNM status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fmaxnm_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fmaxnmp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b010 opc=0b100 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMAXNMP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_pairwise page=fmaxnmp_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS fmaxnmv_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b100 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAXNMV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fast_red page=fmaxnmv_v_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fmaxp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b010 opc=0b110 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMAXP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_pairwise page=fmaxp_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS fmaxv_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b110 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAXV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fast_red page=fmaxv_v_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fmin_z_p_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b111 ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMIN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_i_p_zds page=fmin_z_p_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fmin_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b0111 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMIN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fmin_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fminnm_z_p_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b101 ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMINNM status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_i_p_zds page=fminnm_z_p_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fminnm_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b0101 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMINNM status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fminnm_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fminnmp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b010 opc=0b101 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMINNMP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_pairwise page=fminnmp_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS fminnmv_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b101 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMINNMV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fast_red page=fminnmv_v_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fminp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b010 opc=0b111 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMINP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_pairwise page=fminp_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS fminv_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b111 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMINV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fast_red page=fminv_v_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fmla_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 N=0b0 op=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_fma iclass=sve_fp_3op_p_zds_a page=fmla_z_p_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fmla_z_zzzi_h => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01100100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b00000 op=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=sve isa=A64 mnemonic=FMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fma_by_indexed_elem page=fmla_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fmla_z_zzzi_s => {
  name       => 'Single-precision',
  diagram    => 'ig0=0b01100100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00000 op=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=single instr-class=sve isa=A64 mnemonic=FMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fma_by_indexed_elem page=fmla_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fmla_z_zzzi_d => {
  name       => 'Double-precision',
  diagram    => 'ig0=0b01100100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b00000 op=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=double instr-class=sve isa=A64 mnemonic=FMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fma_by_indexed_elem page=fmla_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fmlalb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b011001001 o2=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b10 op=0b0 ig3=0b00 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMLALB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long iclass=sve_fp_fma_long page=fmlalb_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS fmlalb_z_zzzi_s => {
  name       => 'Single-precision',
  diagram    => 'ig0=0b011001001 o2=0b0 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b01 op=0b0 ig3=0b0 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=single instr-class=sve2 isa=A64 mnemonic=FMLALB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long_by_indexed_elem iclass=sve_fp_fma_long_by_indexed_elem page=fmlalb_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS fmlalt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b011001001 o2=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b10 op=0b0 ig3=0b00 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMLALT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long iclass=sve_fp_fma_long page=fmlalt_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS fmlalt_z_zzzi_s => {
  name       => 'Single-precision',
  diagram    => 'ig0=0b011001001 o2=0b0 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b01 op=0b0 ig3=0b0 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=single instr-class=sve2 isa=A64 mnemonic=FMLALT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long_by_indexed_elem iclass=sve_fp_fma_long_by_indexed_elem page=fmlalt_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS fmls_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 N=0b0 op=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_fma iclass=sve_fp_3op_p_zds_a page=fmls_z_p_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fmls_z_zzzi_h => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01100100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b00000 op=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=sve isa=A64 mnemonic=FMLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fma_by_indexed_elem page=fmls_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fmls_z_zzzi_s => {
  name       => 'Single-precision',
  diagram    => 'ig0=0b01100100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00000 op=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=single instr-class=sve isa=A64 mnemonic=FMLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fma_by_indexed_elem page=fmls_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fmls_z_zzzi_d => {
  name       => 'Double-precision',
  diagram    => 'ig0=0b01100100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b00000 op=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=double instr-class=sve isa=A64 mnemonic=FMLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fma_by_indexed_elem page=fmls_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fmlslb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b011001001 o2=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b10 op=0b1 ig3=0b00 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMLSLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long iclass=sve_fp_fma_long page=fmlslb_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS fmlslb_z_zzzi_s => {
  name       => 'Single-precision',
  diagram    => 'ig0=0b011001001 o2=0b0 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b01 op=0b1 ig3=0b0 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=single instr-class=sve2 isa=A64 mnemonic=FMLSLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long_by_indexed_elem iclass=sve_fp_fma_long_by_indexed_elem page=fmlslb_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS fmlslt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b011001001 o2=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b10 op=0b1 ig3=0b00 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMLSLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long iclass=sve_fp_fma_long page=fmlslt_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS fmlslt_z_zzzi_s => {
  name       => 'Single-precision',
  diagram    => 'ig0=0b011001001 o2=0b0 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b01 op=0b1 ig3=0b0 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'datatype=single instr-class=sve2 isa=A64 mnemonic=FMLSLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_fma_long_by_indexed_elem iclass=sve_fp_fma_long_by_indexed_elem page=fmlslt_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS fmmla_z_zzz_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b01100100 opc=0b10 ig1=0b1 Zm:u=0bxxxxx ig2=0b111001 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMMLA status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fmmla page=fmmla_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVEFP32MatMulExt()',
};

CLASS fmmla_z_zzz_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b01100100 opc=0b11 ig1=0b1 Zm:u=0bxxxxx ig2=0b111001 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMMLA status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fmmla page=fmmla_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVEFP64MatMulExt()',
};

CLASS fmsb_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Za:u=0bxxxxx ig2=0b1 N=0b0 op=0b1 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMSB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_fma iclass=sve_fp_3op_p_zds_b page=fmsb_z_p_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fmul_z_p_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b010 ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_i_p_zds page=fmul_z_p_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fmul_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b0010 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fmul_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fmul_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b000 opc=0b010 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_u_zd page=fmul_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fmul_z_zzi_h => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01100100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b001000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=sve isa=A64 mnemonic=FMUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fmul_by_indexed_elem page=fmul_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fmul_z_zzi_s => {
  name       => 'Single-precision',
  diagram    => 'ig0=0b01100100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b001000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'datatype=single instr-class=sve isa=A64 mnemonic=FMUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fmul_by_indexed_elem page=fmul_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fmul_z_zzi_d => {
  name       => 'Double-precision',
  diagram    => 'ig0=0b01100100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b001000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'datatype=double instr-class=sve isa=A64 mnemonic=FMUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_fmul_by_indexed_elem page=fmul_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS fmulx_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b1010 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMULX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fmulx_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fneg_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b101 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FNEG status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_1 page=fneg_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fnmad_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Za:u=0bxxxxx ig2=0b1 N=0b1 op=0b0 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FNMAD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_fma iclass=sve_fp_3op_p_zds_b page=fnmad_z_p_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fnmla_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 N=0b1 op=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FNMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_fma iclass=sve_fp_3op_p_zds_a page=fnmla_z_p_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fnmls_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 N=0b1 op=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FNMLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_fma iclass=sve_fp_3op_p_zds_a page=fnmls_z_p_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fnmsb_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Za:u=0bxxxxx ig2=0b1 N=0b1 op=0b1 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FNMSB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_fma iclass=sve_fp_3op_p_zds_b page=fnmsb_z_p_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS frecpe_z_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b001 opc=0b110 ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FRECPE status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_unary_unpred iclass=sve_fp_2op_u_zd page=frecpe_z_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS frecps_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b000 opc=0b110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FRECPS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_u_zd page=frecps_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS frecpx_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0011 opc=0b00 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FRECPX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_1 page=frecpx_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS frinti_z_p_z_ => {
  name       => 'Current mode',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b111 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'frint-type=frint-i instr-class=sve isa=A64 mnemonic=FRINTI status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_a page=frinta_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS frintx_z_p_z_ => {
  name       => 'Current mode signalling inexact',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b110 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'frint-type=frint-x instr-class=sve isa=A64 mnemonic=FRINTX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_a page=frinta_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS frinta_z_p_z_ => {
  name       => 'Nearest with ties to away',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b100 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'frint-type=frint-a instr-class=sve isa=A64 mnemonic=FRINTA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_a page=frinta_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS frintn_z_p_z_ => {
  name       => 'Nearest with ties to even',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b000 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'frint-type=frint-n instr-class=sve isa=A64 mnemonic=FRINTN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_a page=frinta_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS frintz_z_p_z_ => {
  name       => 'Toward zero',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b011 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'frint-type=frint-z instr-class=sve isa=A64 mnemonic=FRINTZ status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_a page=frinta_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS frintm_z_p_z_ => {
  name       => 'Toward minus infinity',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b010 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'frint-type=frint-m instr-class=sve isa=A64 mnemonic=FRINTM status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_a page=frinta_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS frintp_z_p_z_ => {
  name       => 'Toward plus infinity',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc=0b001 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'frint-type=frint-p instr-class=sve isa=A64 mnemonic=FRINTP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_a page=frinta_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS frsqrte_z_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b001 opc=0b111 ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FRSQRTE status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_unary_unpred iclass=sve_fp_2op_u_zd page=frsqrte_z_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS frsqrts_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b000 opc=0b111 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FRSQRTS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_u_zd page=frsqrts_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fscale_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b1001 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSCALE status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fscale_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fsqrt_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0011 opc=0b01 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSQRT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_b_1 page=fsqrt_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fsub_z_p_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b001 ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_i_p_zds page=fsub_z_p_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fsub_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b0001 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fsub_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fsub_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b000 opc=0b001 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_u_zd page=fsub_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fsubr_z_p_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc=0b011 ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSUBR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_i_p_zds page=fsubr_z_p_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS fsubr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc=0b0011 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSUBR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_pred iclass=sve_fp_2op_p_zds page=fsubr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS ftmad_z_zzi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b010 imm3:u=0bxxx ig2=0b100000 Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FTMAD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
  tags       => 'group=sve_fp_pred iclass=sve_fp_ftmad page=ftmad_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS ftsmul_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b000 opc=0b011 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FTSMUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_u_zd page=ftsmul_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS ftssel_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b10110 op=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FTSSEL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_int_unpred_misc iclass=sve_int_bin_cons_misc_0_b page=ftssel_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS histcnt_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=HISTCNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve iclass=sve_intx_histcnt page=histcnt_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0b0x)',
};

CLASS histseg_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b101000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=HISTSEG status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_histseg iclass=sve_intx_histseg page=histseg_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size != 0)',
};

CLASS incb_r_rs_ => {
  name       => 'Byte',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_a page=incb_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS incd_r_rs_ => {
  name       => 'Doubleword',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_a page=incb_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS inch_r_rs_ => {
  name       => 'Halfword',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_a page=incb_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS incw_r_rs_ => {
  name       => 'Word',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_a page=incb_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS incd_z_zs_ => {
  name       => 'Doubleword',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b11 imm4:u=0bxxxx ig2=0b11000 D=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv1 page=incd_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS inch_z_zs_ => {
  name       => 'Halfword',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b11 imm4:u=0bxxxx ig2=0b11000 D=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv1 page=incd_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS incw_z_zs_ => {
  name       => 'Word',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b11 imm4:u=0bxxxx ig2=0b11000 D=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv1 page=incd_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS incp_r_p_r_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1011 op=0b0 D=0b0 ig2=0b10001 opc2=0b00 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r page=incp_r_p_r',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS incp_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1011 op=0b0 D=0b0 ig2=0b10000 opc2=0b00 Pm:u=0bxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_v page=incp_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS index_z_ii_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 imm5b:u=0bxxxxx ig2=0b010000 imm5:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INDEX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_index iclass=sve_int_index_ii page=index_z_ii',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS index_z_ir_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b010010 imm5:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INDEX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_index iclass=sve_int_index_ir page=index_z_ir',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS index_z_ri_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 imm5:u=0bxxxxx ig2=0b010001 Rn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INDEX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_index iclass=sve_int_index_ri page=index_z_ri',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS index_z_rr_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b010011 Rn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INDEX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_index iclass=sve_int_index_rr page=index_z_rr',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS insr_z_r_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b100100001110 Rm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INSR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_insrs page=insr_z_r',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS insr_z_v_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b110100001110 Vm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INSR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_insrv page=insr_z_v',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS lasta_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10000 B=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LASTA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_last_r page=lasta_r_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS lasta_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10001 B=0b0 ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LASTA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_last_v page=lasta_v_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS lastb_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10000 B=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LASTB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_last_r page=lastb_r_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS lastb_v_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10001 B=0b1 ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LASTB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_last_v page=lastb_v_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1b_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b00 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ld1b_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1b_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ld1b_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1b_z_p_bi_u8 => {
  name       => '8-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0000 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=8-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1b_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1b_z_p_bi_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0001 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1b_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1b_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0010 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1b_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1b_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0011 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1b_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1b_z_p_br_u8 => {
  name       => '8-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0000 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=8-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1b_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1b_z_p_br_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0001 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1b_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1b_z_p_br_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0010 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1b_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1b_z_p_br_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0011 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1b_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1b_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ld1b_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1b_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b00 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ld1b_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1b_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ld1b_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1d_z_p_ai_d => {
  name       => 'SVE',
  diagram    => 'ig0=0b1100010 msz=0b11 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ld1d_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1d_z_p_bi_u64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 dtype=0b1111 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1d_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1d_z_p_br_u64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 dtype=0b1111 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1d_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1d_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b11 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ld1d_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1d_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b11 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ld1d_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1d_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b11 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ld1d_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1d_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b11 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ld1d_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1h_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b01 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ld1h_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1h_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ld1h_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1h_z_p_bi_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0101 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1h_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1h_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0110 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1h_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1h_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0111 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1h_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1h_z_p_br_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0101 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1h_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1h_z_p_br_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0110 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1h_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1h_z_p_br_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0111 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1h_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1h_z_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001001 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_sv_a page=ld1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1h_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b01 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ld1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1h_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ld1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1h_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b01 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ld1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1h_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b01 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ld1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1h_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ld1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rb_z_p_bi_u8 => {
  name       => '8-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b00 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b00 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RB status=Green sve-elem-type=8-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rb_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rb_z_p_bi_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b00 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b01 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RB status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rb_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rb_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b00 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b10 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RB status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rb_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rb_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b00 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b11 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RB status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rb_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rd_z_p_bi_u64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010 dtypeh=0b11 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b11 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RD status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rd_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rh_z_p_bi_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b01 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b01 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RH status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rh_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rh_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b01 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b10 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RH status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rh_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rh_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b01 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b11 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RH status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rh_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rob_z_p_bi_u8 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 ssz=0b01 ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROB status=Amber',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_si page=ld1rob_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVEFP64MatMulExt()',
};

CLASS ld1rob_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 ssz=0b01 Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROB status=Amber',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_ss page=ld1rob_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVEFP64MatMulExt()) || (Rm == 31)',
};

CLASS ld1rod_z_p_bi_u64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 ssz=0b01 ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROD status=Amber',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_si page=ld1rod_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVEFP64MatMulExt()',
};

CLASS ld1rod_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 ssz=0b01 Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROD status=Amber',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_ss page=ld1rod_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVEFP64MatMulExt()) || (Rm == 31)',
};

CLASS ld1roh_z_p_bi_u16 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 ssz=0b01 ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROH status=Amber',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_si page=ld1roh_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVEFP64MatMulExt()',
};

CLASS ld1roh_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 ssz=0b01 Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROH status=Amber',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_ss page=ld1roh_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVEFP64MatMulExt()) || (Rm == 31)',
};

CLASS ld1row_z_p_bi_u32 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 ssz=0b01 ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROW status=Amber',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_si page=ld1row_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVEFP64MatMulExt()',
};

CLASS ld1row_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 ssz=0b01 Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROW status=Amber',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_ss page=ld1row_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVEFP64MatMulExt()) || (Rm == 31)',
};

CLASS ld1rqb_z_p_bi_u8 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 ssz=0b00 ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQB status=Green sve-elem-type=8-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_si page=ld1rqb_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rqb_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 ssz=0b00 Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_ss page=ld1rqb_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1rqd_z_p_bi_u64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 ssz=0b00 ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQD status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_si page=ld1rqd_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rqd_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 ssz=0b00 Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_ss page=ld1rqd_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1rqh_z_p_bi_u16 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 ssz=0b00 ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQH status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_si page=ld1rqh_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rqh_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 ssz=0b00 Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_ss page=ld1rqh_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1rqw_z_p_bi_u32 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 ssz=0b00 ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQW status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_si page=ld1rqw_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rqw_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 ssz=0b00 Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQW status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_ldqr_ss page=ld1rqw_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1rsb_z_p_bi_s16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b11 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b10 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RSB status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rsb_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rsb_z_p_bi_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b11 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b01 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RSB status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rsb_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rsb_z_p_bi_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b11 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b00 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RSB status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rsb_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rsh_z_p_bi_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b10 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b01 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RSH status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rsh_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rsh_z_p_bi_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b10 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b00 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RSH status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rsh_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rsw_z_p_bi_s64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010 dtypeh=0b01 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b00 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RSW status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rsw_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rw_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b10 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b10 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RW status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rw_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1rw_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1000010 dtypeh=0b10 ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel=0b11 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RW status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 broadcast=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_ld_dup page=ld1rw_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sb_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b00 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ld1sb_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sb_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ld1sb_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sb_z_p_bi_s16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1110 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1sb_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sb_z_p_bi_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1101 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1sb_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sb_z_p_bi_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1100 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1sb_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sb_z_p_br_s16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1110 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1sb_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1sb_z_p_br_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1101 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1sb_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1sb_z_p_br_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1100 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1sb_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1sb_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ld1sb_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sb_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b00 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ld1sb_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sb_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ld1sb_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sh_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b01 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ld1sh_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sh_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ld1sh_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sh_z_p_bi_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1001 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1sh_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sh_z_p_bi_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1000 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1sh_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sh_z_p_br_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1001 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1sh_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1sh_z_p_br_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1000 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1sh_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1sh_z_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001001 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_sv_a page=ld1sh_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sh_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b01 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ld1sh_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sh_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ld1sh_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sh_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b01 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ld1sh_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sh_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b01 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ld1sh_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sh_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ld1sh_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sw_z_p_ai_d => {
  name       => 'SVE',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ld1sw_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sw_z_p_bi_s64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 dtype=0b0100 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1sw_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sw_z_p_br_s64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 dtype=0b0100 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1sw_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1sw_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b10 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ld1sw_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sw_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ld1sw_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sw_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b10 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ld1sw_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1sw_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ld1sw_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1w_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b10 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ld1w_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1w_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ld1w_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1w_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1010 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1w_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1w_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1011 ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_si page=ld1w_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1w_z_p_br_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1010 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1w_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1w_z_p_br_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1011 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cld_ss page=ld1w_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld1w_z_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001010 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_sv_b page=ld1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1w_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b10 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ld1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1w_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ld1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1w_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b10 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ld1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1w_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b10 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ld1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld1w_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ld1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld2b_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 opc=0b01 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld2b_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld2b_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 opc=0b01 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld2b_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld2d_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 opc=0b01 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld2d_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld2d_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 opc=0b01 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld2d_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld2h_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 opc=0b01 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld2h_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld2h_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 opc=0b01 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld2h_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld2w_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 opc=0b01 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld2w_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld2w_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 opc=0b01 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld2w_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld3b_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 opc=0b10 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld3b_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld3b_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 opc=0b10 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld3b_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld3d_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 opc=0b10 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld3d_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld3d_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 opc=0b10 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld3d_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld3h_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 opc=0b10 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld3h_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld3h_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 opc=0b10 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld3h_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld3w_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 opc=0b10 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld3w_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld3w_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 opc=0b10 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld3w_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld4b_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 opc=0b11 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld4b_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld4b_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 opc=0b11 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld4b_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld4d_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 opc=0b11 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld4d_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld4d_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 opc=0b11 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld4d_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld4h_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 opc=0b11 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld4h_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld4h_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 opc=0b11 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld4h_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ld4w_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 opc=0b11 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_si page=ld4w_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ld4w_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 opc=0b11 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_eld_ss page=ld4w_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ldff1b_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b00 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ldff1b_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1b_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ldff1b_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1b_z_p_br_u8 => {
  name       => '8-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0000 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-elem-type=8-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1b_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1b_z_p_br_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0001 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1b_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1b_z_p_br_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0010 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1b_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1b_z_p_br_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0011 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1b_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1b_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ldff1b_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1b_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b00 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ldff1b_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1b_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ldff1b_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1d_z_p_ai_d => {
  name       => 'SVE',
  diagram    => 'ig0=0b1100010 msz=0b11 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ldff1d_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1d_z_p_br_u64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 dtype=0b1111 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1d_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1d_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b11 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1D status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ldff1d_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1d_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b11 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1D status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ldff1d_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1d_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b11 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1D status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ldff1d_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1d_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b11 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1D status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ldff1d_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1h_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b01 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ldff1h_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1h_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ldff1h_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1h_z_p_br_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0101 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1h_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1h_z_p_br_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0110 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1h_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1h_z_p_br_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0111 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1h_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1h_z_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001001 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_sv_a page=ldff1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1h_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b01 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ldff1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1h_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ldff1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1h_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b01 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ldff1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1h_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b01 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ldff1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1h_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ldff1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sb_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b00 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ldff1sb_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sb_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ldff1sb_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sb_z_p_br_s16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1110 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1sb_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sb_z_p_br_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1101 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1sb_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sb_z_p_br_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1100 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1sb_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sb_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ldff1sb_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sb_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b00 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ldff1sb_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sb_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ldff1sb_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sh_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b01 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ldff1sh_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sh_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ldff1sh_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sh_z_p_br_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1001 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1sh_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sh_z_p_br_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1000 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1sh_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sh_z_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001001 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_sv_a page=ldff1sh_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sh_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b01 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ldff1sh_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sh_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ldff1sh_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sh_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b01 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ldff1sh_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sh_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b01 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ldff1sh_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sh_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ldff1sh_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sw_z_p_ai_d => {
  name       => 'SVE',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SW status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ldff1sw_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sw_z_p_br_s64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 dtype=0b0100 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SW status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1sw_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sw_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b10 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SW status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ldff1sw_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sw_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SW status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ldff1sw_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sw_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b10 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SW status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ldff1sw_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1sw_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b0 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SW status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ldff1sw_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1w_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b10 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vi page=ldff1w_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1w_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vi page=ldff1w_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1w_z_p_br_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1010 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1w_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1w_z_p_br_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1011 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldff_ss page=ldff1w_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1w_z_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001010 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_sv_b page=ldff1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1w_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b10 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv page=ldff1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1w_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs page=ldff1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1w_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 opc=0b10 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gld_vs page=ldff1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1w_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1100010 opc=0b10 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_sv2 page=ldff1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldff1w_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U=0b1 ff=0b1 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gld_vs2 page=ldff1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnf1b_z_p_bi_u8 => {
  name       => '8-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0000 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1B status=Green sve-elem-type=8-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1b_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnf1b_z_p_bi_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0001 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1B status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1b_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnf1b_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0010 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1B status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1b_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnf1b_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0011 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1B status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1b_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnf1d_z_p_bi_u64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 dtype=0b1111 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1D status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1d_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnf1h_z_p_bi_u16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0101 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1H status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1h_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnf1h_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0110 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1H status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1h_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnf1h_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b0111 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1H status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1h_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnf1sb_z_p_bi_s16 => {
  name       => '16-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1110 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1SB status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1sb_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnf1sb_z_p_bi_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1101 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1SB status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1sb_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnf1sb_z_p_bi_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1100 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1SB status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1sb_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnf1sh_z_p_bi_s32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1001 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1SH status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1sh_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnf1sh_z_p_bi_s64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1000 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1SH status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1sh_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnf1sw_z_p_bi_s64 => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 dtype=0b0100 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1SW status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1sw_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnf1w_z_p_bi_u32 => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1010 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1W status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1w_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnf1w_z_p_bi_u64 => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1010010 dtype=0b1011 ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1W status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnf_si page=ldnf1w_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnt1b_z_p_ar_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 msz=0b00 ig1=0b00 Rm:u=0bxxxxx ig2=0b10 U=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1B status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gldnt_vs page=ldnt1b_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ldnt1b_z_p_ar_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b00 Rm:u=0bxxxxx ig2=0b1 U=0b1 ig3=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1B status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gldnt_vs page=ldnt1b_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ldnt1b_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 ig1=0b000 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnt_si page=ldnt1b_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnt1b_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b00 ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnt_ss page=ldnt1b_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ldnt1d_z_p_ar_d_64_unscaled => {
  name       => 'SVE2',
  diagram    => 'ig0=0b1100010 msz=0b11 ig1=0b00 Rm:u=0bxxxxx ig2=0b1 U=0b1 ig3=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1D status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gldnt_vs page=ldnt1d_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ldnt1d_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 ig1=0b000 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnt_si page=ldnt1d_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnt1d_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b11 ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnt_ss page=ldnt1d_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ldnt1h_z_p_ar_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 msz=0b01 ig1=0b00 Rm:u=0bxxxxx ig2=0b10 U=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1H status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gldnt_vs page=ldnt1h_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ldnt1h_z_p_ar_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b00 Rm:u=0bxxxxx ig2=0b1 U=0b1 ig3=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1H status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gldnt_vs page=ldnt1h_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ldnt1h_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 ig1=0b000 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnt_si page=ldnt1h_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnt1h_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b01 ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnt_ss page=ldnt1h_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ldnt1sb_z_p_ar_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 msz=0b00 ig1=0b00 Rm:u=0bxxxxx ig2=0b10 U=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1SB status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gldnt_vs page=ldnt1sb_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ldnt1sb_z_p_ar_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b00 Rm:u=0bxxxxx ig2=0b1 U=0b0 ig3=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1SB status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gldnt_vs page=ldnt1sb_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ldnt1sh_z_p_ar_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 msz=0b01 ig1=0b00 Rm:u=0bxxxxx ig2=0b10 U=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1SH status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gldnt_vs page=ldnt1sh_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ldnt1sh_z_p_ar_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b00 Rm:u=0bxxxxx ig2=0b1 U=0b0 ig3=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1SH status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gldnt_vs page=ldnt1sh_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ldnt1sw_z_p_ar_d_64_unscaled => {
  name       => 'SVE2',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b00 Rm:u=0bxxxxx ig2=0b1 U=0b0 ig3=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1SW status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gldnt_vs page=ldnt1sw_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ldnt1w_z_p_ar_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1000010 msz=0b10 ig1=0b00 Rm:u=0bxxxxx ig2=0b10 U=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1W status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_gldnt_vs page=ldnt1w_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ldnt1w_z_p_ar_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b00 Rm:u=0bxxxxx ig2=0b1 U=0b1 ig3=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1W status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_gldnt_vs page=ldnt1w_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ldnt1w_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 ig1=0b000 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnt_si page=ldnt1w_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldnt1w_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1010010 msz=0b10 ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memcld iclass=sve_mem_cldnt_ss page=ldnt1w_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS ldr_p_bi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010110 imm9h:u=0bxxxxxx ig1=0b000 imm9l:u=0bxxx Rn:u=0bxxxxx ig2=0b0 Pt:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_pfill page=ldr_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ldr_z_bi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010110 imm9h:u=0bxxxxxx ig1=0b010 imm9l:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_fill page=ldr_z_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS lsl_z_p_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b00 L=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=lsl_z_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS lsl_z_p_zw_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 R=0b0 L=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_2 page=lsl_z_p_zw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 3)',
};

CLASS lsl_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 R=0b0 L=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_1 page=lsl_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS lsl_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b1001 opc=0b11 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_shift iclass=sve_int_bin_cons_shift_b page=lsl_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS lsl_z_zw_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b1000 opc=0b11 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_shift iclass=sve_int_bin_cons_shift_a page=lsl_z_zw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 3)',
};

CLASS lslr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 R=0b1 L=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSLR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_1 page=lslr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS lsr_z_p_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b00 L=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=lsr_z_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS lsr_z_p_zw_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 R=0b0 L=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_2 page=lsr_z_p_zw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 3)',
};

CLASS lsr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 R=0b0 L=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_1 page=lsr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS lsr_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b1001 ig3=0b0 U=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_shift iclass=sve_int_bin_cons_shift_b page=lsr_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS lsr_z_zw_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b1000 ig3=0b0 U=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_shift iclass=sve_int_bin_cons_shift_a page=lsr_z_zw',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 3)',
};

CLASS lsrr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 R=0b1 L=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSRR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_1 page=lsrr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS mad_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b11 op=0b0 Pg:u=0bxxx Za:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MAD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_muladd_pred iclass=sve_int_mladdsub_vvv_pred page=mad_z_p_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS match_p_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx op=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=MATCH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve iclass=sve_intx_match page=match_p_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0b1x)',
};

CLASS mla_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b01 op=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_muladd_pred iclass=sve_int_mlas_vvv_pred page=mla_z_p_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS mla_z_zzzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b00001 S=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=MLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_by_indexed_elem page=mla_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS mla_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00001 S=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=MLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_by_indexed_elem page=mla_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS mla_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b00001 S=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=MLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_by_indexed_elem page=mla_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS mls_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b01 op=0b1 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_muladd_pred iclass=sve_int_mlas_vvv_pred page=mls_z_p_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS mls_z_zzzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b00001 S=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=MLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_by_indexed_elem page=mls_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS mls_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00001 S=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=MLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_by_indexed_elem page=mls_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS mls_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b00001 S=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=MLS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_by_indexed_elem page=mls_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS movprfx_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 opc=0b00 M:u=0bx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MOVPRFX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_movprfx_pred page=movprfx_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS movprfx_z_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 opc=0b00 ig1=0b1 opc2=0b00000 ig2=0b101111 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MOVPRFX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_misc iclass=sve_int_bin_cons_misc_0_d page=movprfx_z_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS msb_z_p_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b11 op=0b1 Pg:u=0bxxx Za:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MSB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_muladd_pred iclass=sve_int_mladdsub_vvv_pred page=msb_z_p_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS mul_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0100 H=0b0 U=0b0 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_2 page=mul_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS mul_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b110 opc=0b000 ig2=0b11 o2=0b0 imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm2 page=mul_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS mul_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0110 opc=0b00 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=MUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_arit_b iclass=sve_int_mul_b page=mul_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS mul_z_zzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b111110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=MUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_by_indexed_elem page=mul_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS mul_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b111110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=MUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_by_indexed_elem page=mul_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS mul_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b111110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=MUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_by_indexed_elem page=mul_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS nand_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=NAND status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=nand_p_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS nands_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=NANDS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=nand_p_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS nbsl_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 opc=0b11 ig1=0b1 Zm:u=0bxxxxx ig2=0b00111 o2=0b1 Zk:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=NBSL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_tern_log page=nbsl_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS neg_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 opc=0b111 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=NEG status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_0 page=neg_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS nmatch_p_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx op=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=NMATCH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve iclass=sve_intx_match page=nmatch_p_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0b1x)',
};

CLASS nor_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=NOR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=nor_p_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS nors_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=NORS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=nor_p_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS not_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b110 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=NOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_1 page=not_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS orn_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=ORN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=orn_p_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS orns_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=ORNS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=orn_p_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS orr_p_p_pp_orr_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S:u=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=orr_p_p_pp xid=orr_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS orr_p_p_pp_orrs_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S:u=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=ORRS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=orr_p_p_pp xid=orrs_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS orr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b000 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_log page=orr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS orr_z_zi_orr_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 opc=0b00 ig1=0b0000 imm13:u=0bxxxxxxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_maskimm iclass=sve_int_log_imm page=orr_z_zi xid=orr_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS orr_z_zz_orr_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 opc=0b01 ig1=0b1 Zm:u=0bxxxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_bin_cons_log page=orr_z_zz xid=orr_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS orv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc=0b000 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ORV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_2 page=orv_r_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS pfalse_p_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b011000111001 ig2=0b000000 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PFALSE status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_pfalse page=pfalse_p',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS pfirst_p_p_p_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b1 ig1=0b011000110000 ig2=0b0 Pg:u=0bxxxx ig3=0b0 Pdn:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PFIRST status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_pfirst page=pfirst_p_p_p',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS pmul_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 Zm:u=0bxxxxx ig2=0b0110 opc=0b01 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=PMUL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_arit_b iclass=sve_int_mul_b page=pmul_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS pmullb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op=0b0 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=PMULLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_mul_long page=pmullb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0 && !HaveSVE2PMULL128())',
};

CLASS pmullt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op=0b0 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=PMULLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_mul_long page=pmullt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0 && !HaveSVE2PMULL128())',
};

CLASS pnext_p_p_p_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b011001110001 ig2=0b0 Pg:u=0bxxxx ig3=0b0 Pdn:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PNEXT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_pnext page=pnext_p_p_p',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfb_i_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b00 ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-elem-type=32-elem sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_prfm_vi page=prfb_i_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfb_i_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b00 ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-elem-type=64-elem sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_vi page=prfb_i_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfb_i_p_bi_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010111 imm6:u=0bxxxxxx ig1=0b0 msz=0b00 Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_prfm_si page=prfb_i_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfb_i_p_br_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010 msz=0b00 ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_prfm_ss page=prfb_i_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS prfb_i_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz=0b00 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-esize=esize-byte sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_prfm_sv page=prfb_i_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfb_i_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b110001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz=0b00 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-esize=esize-byte sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_sv page=prfb_i_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfb_i_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b11000100011 Zm:u=0bxxxxx ig1=0b1 msz=0b00 Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-esize=esize-byte sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_sv2 page=prfb_i_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfd_i_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b11 ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-elem-type=32-elem sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_prfm_vi page=prfd_i_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfd_i_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b11 ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-elem-type=64-elem sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_vi page=prfd_i_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfd_i_p_bi_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010111 imm6:u=0bxxxxxx ig1=0b0 msz=0b11 Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_prfm_si page=prfd_i_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfd_i_p_br_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010 msz=0b11 ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_prfm_ss page=prfd_i_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS prfd_i_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz=0b11 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-esize=esize-doubleword sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_prfm_sv page=prfd_i_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfd_i_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b110001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz=0b11 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-esize=esize-doubleword sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_sv page=prfd_i_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfd_i_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b11000100011 Zm:u=0bxxxxx ig1=0b1 msz=0b11 Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-esize=esize-doubleword sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_sv2 page=prfd_i_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfh_i_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b01 ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-elem-type=32-elem sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_prfm_vi page=prfh_i_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfh_i_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b01 ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-elem-type=64-elem sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_vi page=prfh_i_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfh_i_p_bi_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010111 imm6:u=0bxxxxxx ig1=0b0 msz=0b01 Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_prfm_si page=prfh_i_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfh_i_p_br_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010 msz=0b01 ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_prfm_ss page=prfh_i_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS prfh_i_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz=0b01 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-esize=esize-halfword sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_prfm_sv page=prfh_i_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfh_i_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b110001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz=0b01 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-esize=esize-halfword sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_sv page=prfh_i_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfh_i_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b11000100011 Zm:u=0bxxxxx ig1=0b1 msz=0b01 Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-esize=esize-halfword sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_sv2 page=prfh_i_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfw_i_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1000010 msz=0b10 ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-elem-type=32-elem sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_prfm_vi page=prfw_i_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfw_i_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1100010 msz=0b10 ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-elem-type=64-elem sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_vi page=prfw_i_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfw_i_p_bi_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010111 imm6:u=0bxxxxxx ig1=0b0 msz=0b10 Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_prfm_si page=prfw_i_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfw_i_p_br_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b1000010 msz=0b10 ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_prfm_ss page=prfw_i_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS prfw_i_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b100001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz=0b10 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-esize=esize-word sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem32 iclass=sve_mem_32b_prfm_sv page=prfw_i_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfw_i_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b110001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz=0b10 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-esize=esize-word sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_sv page=prfw_i_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS prfw_i_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b11000100011 Zm:u=0bxxxxx ig1=0b1 msz=0b10 Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-esize=esize-word sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_mem64 iclass=sve_mem_64b_prfm_sv2 page=prfw_i_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ptest_p_p_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00 ig1=0b100101 op=0b0 S=0b1 ig2=0b01 ig3=0b000011 Pg:u=0bxxxx ig4=0b0 Pn:u=0bxxxx ig5=0b0 opc2=0b0000',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PTEST status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_ptest page=ptest_p_p',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ptrue_p_s_ => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b01100 S=0b0 ig2=0b111000 pattern:u=0bxxxxx ig3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=PTRUE status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_ptrue page=ptrue_p_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ptrues_p_s_ => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b01100 S=0b1 ig2=0b111000 pattern:u=0bxxxxx ig3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=PTRUES status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_ptrue page=ptrue_p_s',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS punpkhi_p_p_ => {
  name       => 'High half',
  diagram    => 'ig0=0b000001010011000 H=0b1 ig1=0b010000 ig2=0b0 Pn:u=0bxxxx ig3=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PUNPKHI status=Green sve-half=sve-high-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_punpk page=punpkhi_p_p',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS punpklo_p_p_ => {
  name       => 'Low half',
  diagram    => 'ig0=0b000001010011000 H=0b0 ig1=0b010000 ig2=0b0 Pn:u=0bxxxx ig3=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PUNPKLO status=Green sve-half=sve-low-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_punpk page=punpkhi_p_p',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS raddhnb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S=0b0 R=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RADDHNB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_arith_narrow page=raddhnb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS raddhnt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S=0b0 R=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RADDHNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_arith_narrow page=raddhnt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS rax1_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size=0b00 ig1=0b1 Zm:u=0bxxxxx ig2=0b11110 op=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RAX1 status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_crypto iclass=sve_crypto_binary_const page=rax1_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2SHA3()',
};

CLASS rbit_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1001 opc=0b11 ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=RBIT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_rev page=rbit_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS rdffr_p_f_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b011001111100 ig2=0b000000 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=RDFFR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_rdffr_2 page=rdffr_p_f',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS rdffr_p_p_f_ => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b011000111100 ig2=0b0 Pg:u=0bxxxx ig3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=RDFFR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_rdffr page=rdffr_p_p_f',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS rdffrs_p_p_f_ => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b1 ig1=0b011000111100 ig2=0b0 Pg:u=0bxxxx ig3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=RDFFRS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_gen_d iclass=sve_int_rdffr page=rdffr_p_p_f',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS rdvl_r_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b000001001 op=0b0 ig1=0b1 opc2=0b11111 ig2=0b01010 imm6:u=0bxxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=RDVL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_alloca iclass=sve_int_read_vl_a page=rdvl_r_i',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS rev_p_p_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b110100010000 ig2=0b0 Pn:u=0bxxxx ig3=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=REV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_reverse_p page=rev_p_p',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS rev_z_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b111000001110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=REV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_reverse_z page=rev_z_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS revb_z_z_ => {
  name       => 'Byte',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1001 opc=0b00 ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=REVB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_rev page=revb_z_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS revh_z_z_ => {
  name       => 'Halfword',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1001 opc=0b01 ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=REVH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_rev page=revb_z_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size != 0b1x)',
};

CLASS revw_z_z_ => {
  name       => 'Word',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1001 opc=0b10 ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=REVW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_rev page=revb_z_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size != 3)',
};

CLASS rshrnb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b0 U=0b1 R=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RSHRNB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=rshrnb_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS rshrnt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b0 U=0b1 R=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RSHRNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=rshrnt_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS rsubhnb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S=0b1 R=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RSUBHNB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_arith_narrow page=rsubhnb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS rsubhnt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S=0b1 R=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RSUBHNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_arith_narrow page=rsubhnt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS saba_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b11111 U=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SABA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_aba page=saba_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sabalb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1100 U=0b0 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SABALB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_aba_long page=sabalb_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS sabalt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1100 U=0b0 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SABALT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_aba_long page=sabalt_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS sabd_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b10 U=0b0 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SABD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_1 page=sabd_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sabdlb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b1 S=0b1 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SABDLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=sabdlb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS sabdlt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b1 S=0b1 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SABDLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=sabdlt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS sadalp_z_p_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00010 U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADALP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_accumulate_long_pairs page=sadalp_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS saddlb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b0 S=0b0 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADDLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=saddlb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS saddlbt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1000 S=0b0 tb=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADDLBT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_clong page=saddlbt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS saddlt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b0 S=0b0 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADDLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=saddlt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS saddv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b000 opc=0b00 U=0b0 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SADDV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_0 page=saddv_r_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 3)',
};

CLASS saddwb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b0 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADDWB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_wide page=saddwb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS saddwt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b0 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADDWT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_wide page=saddwt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS sbclb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 ig1=0b1 sz:u=0bx ig2=0b0 Zm:u=0bxxxxx ig3=0b11010 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SBCLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_adc_long page=sbclb_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sbclt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 ig1=0b1 sz:u=0bx ig2=0b0 Zm:u=0bxxxxx ig3=0b11010 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SBCLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_adc_long page=sbclt_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS scvtf_z_p_z_h2fp16 => {
  name       => '16-bit to half-precision',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b010 opc2=0b01 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=16-to-half instr-class=sve isa=A64 mnemonic=SCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=scvtf_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS scvtf_z_p_z_w2fp16 => {
  name       => '32-bit to half-precision',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b010 opc2=0b10 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=32-to-half instr-class=sve isa=A64 mnemonic=SCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=scvtf_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS scvtf_z_p_z_w2s => {
  name       => '32-bit to single-precision',
  diagram    => 'ig0=0b01100101 opc=0b10 ig1=0b010 opc2=0b10 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=32-to-single instr-class=sve isa=A64 mnemonic=SCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=scvtf_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS scvtf_z_p_z_w2d => {
  name       => '32-bit to double-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b010 opc2=0b00 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=32-to-double instr-class=sve isa=A64 mnemonic=SCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=scvtf_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS scvtf_z_p_z_x2fp16 => {
  name       => '64-bit to half-precision',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b010 opc2=0b11 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=64-to-half instr-class=sve isa=A64 mnemonic=SCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=scvtf_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS scvtf_z_p_z_x2s => {
  name       => '64-bit to single-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b010 opc2=0b10 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=64-to-single instr-class=sve isa=A64 mnemonic=SCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=scvtf_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS scvtf_z_p_z_x2d => {
  name       => '64-bit to double-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b010 opc2=0b11 int_U=0b0 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=64-to-double instr-class=sve isa=A64 mnemonic=SCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=scvtf_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sdiv_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0101 R=0b0 U=0b0 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SDIV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_div page=sdiv_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0b0x)',
};

CLASS sdivr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0101 R=0b1 U=0b0 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SDIVR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_div page=sdivr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0b0x)',
};

CLASS sdot_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00000 U=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_dot page=sdot_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0b0x)',
};

CLASS sdot_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00000 U=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve isa=A64 mnemonic=SDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_dot_by_indexed_elem page=sdot_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sdot_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b00000 U=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve isa=A64 mnemonic=SDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_dot_by_indexed_elem page=sdot_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sel_p_p_pp_sel_p_p_pp_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SEL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=sel_p_p_pp xid=sel_p_p_pp_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sel_z_p_zz_sel_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b11 Pg:u=0bxxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SEL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_sel_vvv page=sel_z_p_zz xid=sel_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS setffr_f_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00 ig1=0b100101 opc=0b00 ig2=0b10 ig3=0b1100100100 ig4=0b000000 ig5=0b0 ig6=0b0 ig7=0b00',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SETFFR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_wrffr iclass=sve_int_setffr page=setffr_f',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS shadd_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 R=0b0 S=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SHADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary page=shadd_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS shrnb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b0 U=0b1 R=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SHRNB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=shrnb_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS shrnt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b0 U=0b1 R=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SHRNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=shrnt_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS shsub_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 R=0b0 S=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SHSUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary page=shsub_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS shsubr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 R=0b1 S=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SHSUBR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary page=shsubr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sli_z_zzi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 tszh:u=0bxx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b11110 op=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SLI status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_shift_insert page=sli_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sm4e_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size=0b00 ig1=0b10001 op=0b1 ig2=0b11100 o2=0b0 Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SM4E status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_crypto iclass=sve_crypto_binary_dest page=sm4e_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2SM4()',
};

CLASS sm4ekey_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size=0b00 ig1=0b1 Zm:u=0bxxxxx ig2=0b11110 op=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SM4EKEY status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_crypto iclass=sve_crypto_binary_const page=sm4ekey_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2SM4()',
};

CLASS smax_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b00 U=0b0 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMAX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_1 page=smax_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS smax_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b101 ig2=0b00 U=0b0 ig3=0b11 o2=0b0 imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMAX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm1 page=smax_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS smaxp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 opc=0b10 U=0b0 ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMAXP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_arith_binary_pairs page=smaxp_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS smaxv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b00 U=0b0 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMAXV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_1 page=smaxv_r_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS smin_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b01 U=0b0 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMIN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_1 page=smin_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS smin_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b101 ig2=0b01 U=0b0 ig3=0b11 o2=0b0 imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMIN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm1 page=smin_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sminp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 opc=0b11 U=0b0 ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMINP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_arith_binary_pairs page=sminp_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sminv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b01 U=0b0 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMINV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_1 page=sminv_r_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS smlalb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b0 U=0b0 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMLALB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mlal_long page=smlalb_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS smlalb_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b10 S=0b0 U=0b0 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SMLALB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=smlalb_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS smlalb_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b10 S=0b0 U=0b0 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SMLALB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=smlalb_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS smlalt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b0 U=0b0 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMLALT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mlal_long page=smlalt_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS smlalt_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b10 S=0b0 U=0b0 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SMLALT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=smlalt_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS smlalt_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b10 S=0b0 U=0b0 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SMLALT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=smlalt_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS smlslb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b1 U=0b0 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMLSLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mlal_long page=smlslb_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS smlslb_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b10 S=0b1 U=0b0 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SMLSLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=smlslb_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS smlslb_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b10 S=0b1 U=0b0 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SMLSLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=smlslb_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS smlslt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b1 U=0b0 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMLSLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mlal_long page=smlslt_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS smlslt_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b10 S=0b1 U=0b0 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SMLSLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=smlslt_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS smlslt_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b10 S=0b1 U=0b0 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SMLSLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=smlslt_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS smmla_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01000101 uns=0b00 ig1=0b0 Zm:u=0bxxxxx ig2=0b100110 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_mmla page=smmla_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE() || !HaveInt8MatMulExt()',
};

CLASS smulh_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0100 H=0b1 U=0b0 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMULH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_2 page=smulh_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS smulh_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0110 ig3=0b1 U=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMULH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_arit_b iclass=sve_int_mul_b page=smulh_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS smullb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op=0b1 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMULLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_mul_long page=smullb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS smullb_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b110 U=0b0 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SMULLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_long_by_indexed_elem page=smullb_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS smullb_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b110 U=0b0 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SMULLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_long_by_indexed_elem page=smullb_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS smullt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op=0b1 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMULLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_mul_long page=smullt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS smullt_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b110 U=0b0 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SMULLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_long_by_indexed_elem page=smullt_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS smullt_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b110 U=0b0 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SMULLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_long_by_indexed_elem page=smullt_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS splice_z_p_zz_con => {
  name       => 'Constructive',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b101101100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SPLICE status=Green sve-dest_const=sve-const',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_intx_perm_splice page=splice_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS splice_z_p_zz_des => {
  name       => 'Destructive',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b101100100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SPLICE status=Green sve-dest_const=sve-dest',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_splice page=splice_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqabs_z_p_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 ig2=0b0 opc=0b00 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQABS status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_unary page=sqabs_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqadd_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b011 op=0b0 S=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary_sat page=sqadd_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqadd_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 ig2=0b10 U=0b0 ig3=0b11 sh:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm0 page=sqadd_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size:sh == 1)',
};

CLASS sqadd_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b10 U=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve iclass=sve_int_bin_cons_arit_0 page=sqadd_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqcadd_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b00000 op=0b1 ig2=0b11011 rot:u=0bx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQCADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_acc iclass=sve_intx_cadd page=sqcadd_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqdecb_r_rs_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECB-32-fsreg instr-class=sve isa=A64 mnemonic=SQDECB reg-type=32-fsreg status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqdecb_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqdecb_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECB-64-fsreg instr-class=sve isa=A64 mnemonic=SQDECB reg-type=64-fsreg status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqdecb_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqdecd_r_rs_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECD-32-fsreg instr-class=sve isa=A64 mnemonic=SQDECD reg-type=32-fsreg status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqdecd_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqdecd_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECD-64-fsreg instr-class=sve isa=A64 mnemonic=SQDECD reg-type=64-fsreg status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqdecd_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqdecd_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b1 U=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQDECD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=sqdecd_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqdech_r_rs_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECH-32-fsreg instr-class=sve isa=A64 mnemonic=SQDECH reg-type=32-fsreg status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqdech_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqdech_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECH-64-fsreg instr-class=sve isa=A64 mnemonic=SQDECH reg-type=64-fsreg status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqdech_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqdech_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b1 U=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQDECH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=sqdech_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqdecp_r_p_r_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b1 U=0b0 ig2=0b10001 sf=0b0 op=0b0 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECP-32-fsreg instr-class=sve isa=A64 mnemonic=SQDECP reg-type=32-fsreg status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r_sat page=sqdecp_r_p_r',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqdecp_r_p_r_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b1 U=0b0 ig2=0b10001 sf=0b1 op=0b0 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECP-64-fsreg instr-class=sve isa=A64 mnemonic=SQDECP reg-type=64-fsreg status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r_sat page=sqdecp_r_p_r',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqdecp_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b1 U=0b0 ig2=0b10000 opc=0b00 Pm:u=0bxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQDECP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_v_sat page=sqdecp_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS sqdecw_r_rs_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECW-32-fsreg instr-class=sve isa=A64 mnemonic=SQDECW reg-type=32-fsreg status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqdecw_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqdecw_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQDECW-64-fsreg instr-class=sve isa=A64 mnemonic=SQDECW reg-type=64-fsreg status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqdecw_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqdecw_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b1 U=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQDECW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=sqdecw_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqdmlalb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0110 S=0b0 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLALB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_qdmlal_long page=sqdmlalb_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS sqdmlalb_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b001 S=0b0 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQDMLALB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmla_long_by_indexed_elem page=sqdmlalb_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqdmlalb_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b001 S=0b0 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQDMLALB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmla_long_by_indexed_elem page=sqdmlalb_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqdmlalbt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00001 S=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLALBT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_qdmlalbt page=sqdmlalbt_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS sqdmlalt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0110 S=0b0 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLALT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_qdmlal_long page=sqdmlalt_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS sqdmlalt_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b001 S=0b0 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQDMLALT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmla_long_by_indexed_elem page=sqdmlalt_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqdmlalt_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b001 S=0b0 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQDMLALT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmla_long_by_indexed_elem page=sqdmlalt_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqdmlslb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0110 S=0b1 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLSLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_qdmlal_long page=sqdmlslb_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS sqdmlslb_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b001 S=0b1 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQDMLSLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmla_long_by_indexed_elem page=sqdmlslb_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqdmlslb_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b001 S=0b1 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQDMLSLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmla_long_by_indexed_elem page=sqdmlslb_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqdmlslbt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00001 S=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLSLBT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_qdmlalbt page=sqdmlslbt_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS sqdmlslt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0110 S=0b1 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLSLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_qdmlal_long page=sqdmlslt_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS sqdmlslt_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b001 S=0b1 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQDMLSLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmla_long_by_indexed_elem page=sqdmlslt_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqdmlslt_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b001 S=0b1 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQDMLSLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmla_long_by_indexed_elem page=sqdmlslt_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqdmulh_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b01110 R=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMULH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_int_unpred_arit_b iclass=sve_int_sqdmulh page=sqdmulh_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqdmulh_z_zzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b11110 R=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=SQDMULH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmulh_by_indexed_elem page=sqdmulh_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqdmulh_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b11110 R=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQDMULH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmulh_by_indexed_elem page=sqdmulh_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqdmulh_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b11110 R=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQDMULH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmulh_by_indexed_elem page=sqdmulh_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqdmullb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op=0b0 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMULLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_mul_long page=sqdmullb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS sqdmullb_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b1110 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQDMULLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmul_long_by_indexed_elem page=sqdmullb_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqdmullb_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b1110 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQDMULLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmul_long_by_indexed_elem page=sqdmullb_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqdmullt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op=0b0 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMULLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_mul_long page=sqdmullt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS sqdmullt_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b1110 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQDMULLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmul_long_by_indexed_elem page=sqdmullt_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqdmullt_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b1110 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQDMULLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmul_long_by_indexed_elem page=sqdmullt_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqincb_r_rs_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCB-32-fsreg instr-class=sve isa=A64 mnemonic=SQINCB reg-type=32-fsreg status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqincb_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqincb_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCB-64-fsreg instr-class=sve isa=A64 mnemonic=SQINCB reg-type=64-fsreg status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqincb_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqincd_r_rs_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCD-32-fsreg instr-class=sve isa=A64 mnemonic=SQINCD reg-type=32-fsreg status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqincd_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqincd_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCD-64-fsreg instr-class=sve isa=A64 mnemonic=SQINCD reg-type=64-fsreg status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqincd_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqincd_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b0 U=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQINCD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=sqincd_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqinch_r_rs_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCH-32-fsreg instr-class=sve isa=A64 mnemonic=SQINCH reg-type=32-fsreg status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqinch_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqinch_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCH-64-fsreg instr-class=sve isa=A64 mnemonic=SQINCH reg-type=64-fsreg status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqinch_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqinch_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b0 U=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQINCH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=sqinch_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqincp_r_p_r_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b0 U=0b0 ig2=0b10001 sf=0b0 op=0b0 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCP-32-fsreg instr-class=sve isa=A64 mnemonic=SQINCP reg-type=32-fsreg status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r_sat page=sqincp_r_p_r',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqincp_r_p_r_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b0 U=0b0 ig2=0b10001 sf=0b1 op=0b0 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCP-64-fsreg instr-class=sve isa=A64 mnemonic=SQINCP reg-type=64-fsreg status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r_sat page=sqincp_r_p_r',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqincp_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b0 U=0b0 ig2=0b10000 opc=0b00 Pm:u=0bxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQINCP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_v_sat page=sqincp_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS sqincw_r_rs_sx => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCW-32-fsreg instr-class=sve isa=A64 mnemonic=SQINCW reg-type=32-fsreg status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqincw_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqincw_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b0 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=SQINCW-64-fsreg instr-class=sve isa=A64 mnemonic=SQINCW reg-type=64-fsreg status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=sqincw_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqincw_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b0 U=0b0 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQINCW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=sqincw_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqneg_z_p_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 ig2=0b0 opc=0b01 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQNEG status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_unary page=sqneg_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrdcmlah_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b001 op=0b1 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRDCMLAH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_cmla page=sqrdcmlah_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrdcmlah_z_zzzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b0111 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=SQRDCMLAH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qrdcmla_by_indexed_elem page=sqrdcmlah_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrdcmlah_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b0111 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQRDCMLAH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qrdcmla_by_indexed_elem page=sqrdcmlah_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrdmlah_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b01110 S=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRDMLAH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_qrdmlah page=sqrdmlah_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrdmlah_z_zzzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b00010 S=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=SQRDMLAH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qrdmlah_by_indexed_elem page=sqrdmlah_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrdmlah_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00010 S=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQRDMLAH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qrdmlah_by_indexed_elem page=sqrdmlah_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrdmlah_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b00010 S=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQRDMLAH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qrdmlah_by_indexed_elem page=sqrdmlah_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrdmlsh_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b01110 S=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRDMLSH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_qrdmlah page=sqrdmlsh_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrdmlsh_z_zzzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b00010 S=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=SQRDMLSH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qrdmlah_by_indexed_elem page=sqrdmlsh_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrdmlsh_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00010 S=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQRDMLSH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qrdmlah_by_indexed_elem page=sqrdmlsh_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrdmlsh_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b00010 S=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQRDMLSH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qrdmlah_by_indexed_elem page=sqrdmlsh_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrdmulh_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b01110 R=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRDMULH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_int_unpred_arit_b iclass=sve_int_sqdmulh page=sqrdmulh_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrdmulh_z_zzi_h => {
  name       => '16-bit',
  diagram    => 'ig0=0b01000100 ig1=0b0 i3h:u=0bx ig2=0b1 i3l:u=0bxx Zm:u=0bxxx ig3=0b11110 R=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-halfwords instr-class=sve2 isa=A64 mnemonic=SQRDMULH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmulh_by_indexed_elem page=sqrdmulh_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrdmulh_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b11110 R=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=SQRDMULH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmulh_by_indexed_elem page=sqrdmulh_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrdmulh_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b11110 R=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=SQRDMULH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_qdmulh_by_indexed_elem page=sqrdmulh_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrshl_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 R=0b0 N=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=sqrshl_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrshlr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 R=0b1 N=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHLR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=sqrshlr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrshrnb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b1 U=0b0 R=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHRNB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=sqrshrnb_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrshrnt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b1 U=0b0 R=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHRNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=sqrshrnt_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrshrunb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b0 U=0b0 R=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHRUNB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=sqrshrunb_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqrshrunt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b0 U=0b0 R=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHRUNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=sqrshrunt_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqshl_z_p_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b01 L=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=sqshl_z_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqshl_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 R=0b0 N=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=sqshl_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqshlr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 R=0b1 N=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHLR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=sqshlr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqshlu_z_p_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b11 L=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHLU status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=sqshlu_z_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqshrnb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b1 U=0b0 R=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHRNB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=sqshrnb_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqshrnt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b1 U=0b0 R=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHRNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=sqshrnt_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqshrunb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b0 U=0b0 R=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHRUNB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=sqshrunb_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqshrunt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b0 U=0b0 R=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHRUNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=sqshrunt_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqsub_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b011 op=0b0 S=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary_sat page=sqsub_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqsub_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 ig2=0b11 U=0b0 ig3=0b11 sh:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQSUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm0 page=sqsub_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size:sh == 1)',
};

CLASS sqsub_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b11 U=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQSUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve iclass=sve_int_bin_cons_arit_0 page=sqsub_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sqsubr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b011 op=0b1 S=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSUBR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary_sat page=sqsubr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqxtnb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx ig2=0b000010 ig3=0b0 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQXTNB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_extract_narrow page=sqxtnb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqxtnt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx ig2=0b000010 ig3=0b0 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQXTNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_extract_narrow page=sqxtnt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqxtunb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx ig2=0b000010 opc=0b10 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQXTUNB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_extract_narrow page=sqxtunb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sqxtunt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx ig2=0b000010 opc=0b10 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQXTUNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_extract_narrow page=sqxtunt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS srhadd_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 R=0b1 S=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRHADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary page=srhadd_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sri_z_zzi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 tszh:u=0bxx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b11110 op=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRI status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_shift_insert page=sri_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS srshl_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b0 R=0b0 N=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRSHL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=srshl_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS srshlr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b0 R=0b1 N=0b1 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRSHLR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=srshlr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS srshr_z_p_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b11 L=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRSHR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=srshr_z_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS srsra_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 tszh:u=0bxx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1110 R=0b1 U=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRSRA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_acc iclass=sve_intx_sra page=srsra_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sshllb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1010 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSHLLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_shift_long page=sshllb_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sshllt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1010 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSHLLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_shift_long page=sshllt_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ssra_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 tszh:u=0bxx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1110 R=0b0 U=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSRA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_sra page=ssra_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ssublb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b0 S=0b1 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=ssublb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS ssublbt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1000 S=0b1 tb=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBLBT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_clong page=ssublbt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS ssublt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b0 S=0b1 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=ssublt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS ssubltb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1000 S=0b1 tb=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBLTB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_clong page=ssubltb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS ssubwb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b1 U=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBWB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_wide page=ssubwb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS ssubwt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b1 U=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBWT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_wide page=ssubwt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS st1b_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b11 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vi_b page=st1b_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1b_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b10 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vi_a page=st1b_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1b_z_p_bi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 size:u=0bxx ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_cst_si page=st1b_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1b_z_p_br_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 ig1=0b00 size:u=0bxx Rm:u=0bxxxxx ig2=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_cs iclass=sve_mem_cst_ss page=st1b_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS st1b_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b00 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_vs_a page=st1b_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1b_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_vs_b page=st1b_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1b_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b00 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vs2 page=st1b_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1d_z_p_ai_d => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 ig1=0b10 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vi_a page=st1d_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1d_z_p_bi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 size:u=0bxx ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_cst_si page=st1d_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size != 3)',
};

CLASS st1d_z_p_br_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 opc=0b111 o2=0b1 Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_cs iclass=sve_mem_cst_ss page=st1d_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS st1d_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1110010 msz=0b11 ig1=0b01 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_sv_a page=st1d_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1d_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b11 ig1=0b00 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_vs_a page=st1d_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1d_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1110010 msz=0b11 ig1=0b01 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_sv2 page=st1d_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1d_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b11 ig1=0b00 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vs2 page=st1d_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1h_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b11 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vi_b page=st1h_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1h_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b10 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vi_a page=st1h_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1h_z_p_bi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 size:u=0bxx ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_cst_si page=st1h_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS st1h_z_p_br_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 ig1=0b01 size:u=0bxx Rm:u=0bxxxxx ig2=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_cs iclass=sve_mem_cst_ss page=st1h_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0) || (Rm == 31)',
};

CLASS st1h_z_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_sv_b page=st1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1h_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b01 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_sv_a page=st1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1h_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b00 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_vs_a page=st1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1h_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_vs_b page=st1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1h_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b01 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_sv2 page=st1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1h_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b00 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vs2 page=st1h_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1w_z_p_ai_s => {
  name       => '32-bit element',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b11 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vi_b page=st1w_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1w_z_p_ai_d => {
  name       => '64-bit element',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b10 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vi_a page=st1w_z_p_ai',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1w_z_p_bi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 size:u=0bxx ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_cst_si page=st1w_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size != 0b1x)',
};

CLASS st1w_z_p_br_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 ig1=0b10 size:u=0bxx Rm:u=0bxxxxx ig2=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_cs iclass=sve_mem_cst_ss page=st1w_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size != 0b1x) || (Rm == 31)',
};

CLASS st1w_z_p_bz_s_x32_scaled => {
  name       => '32-bit scaled offset',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green sve-offset-type=off_s_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_sv_b page=st1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1w_z_p_bz_d_x32_scaled => {
  name       => '32-bit unpacked scaled offset',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b01 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green sve-offset-type=off_d_x32_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_sv_a page=st1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1w_z_p_bz_d_x32_unscaled => {
  name       => '32-bit unpacked unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b00 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green sve-offset-type=off_d_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_vs_a page=st1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1w_z_p_bz_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b10 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss iclass=sve_mem_sst_vs_b page=st1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1w_z_p_bz_d_64_scaled => {
  name       => '64-bit scaled offset',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b01 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green sve-offset-type=off_d_64_scaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_sv2 page=st1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st1w_z_p_bz_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b00 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_ss2 iclass=sve_mem_sst_vs2 page=st1w_z_p_bz',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st2b_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 opc=0b01 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st2b_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st2b_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 opc=0b01 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st2b_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS st2d_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 opc=0b01 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st2d_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st2d_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 opc=0b01 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st2d_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS st2h_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 opc=0b01 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st2h_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st2h_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 opc=0b01 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st2h_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS st2w_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 opc=0b01 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st2w_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st2w_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 opc=0b01 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st2w_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS st3b_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 opc=0b10 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st3b_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st3b_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 opc=0b10 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st3b_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS st3d_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 opc=0b10 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st3d_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st3d_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 opc=0b10 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st3d_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS st3h_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 opc=0b10 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st3h_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st3h_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 opc=0b10 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st3h_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS st3w_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 opc=0b10 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st3w_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st3w_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 opc=0b10 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st3w_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS st4b_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 opc=0b11 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st4b_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st4b_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 opc=0b11 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st4b_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS st4d_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 opc=0b11 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st4d_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st4d_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 opc=0b11 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st4d_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS st4h_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 opc=0b11 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st4h_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st4h_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 opc=0b11 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st4h_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS st4w_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 opc=0b11 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_est_si page=st4w_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS st4w_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 opc=0b11 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_est_ss page=st4w_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS stnt1b_z_p_ar_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b10 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1B status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_sstnt_32b_vs page=stnt1b_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS stnt1b_z_p_ar_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b00 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1B status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_sstnt_64b_vs page=stnt1b_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS stnt1b_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b001 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_cstnt_si page=stnt1b_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS stnt1b_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b00 ig1=0b00 Rm:u=0bxxxxx ig2=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1B status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_cstnt_ss page=stnt1b_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS stnt1d_z_p_ar_d_64_unscaled => {
  name       => 'SVE2',
  diagram    => 'ig0=0b1110010 msz=0b11 ig1=0b00 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1D status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_sstnt_64b_vs page=stnt1d_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS stnt1d_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 ig1=0b001 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_cstnt_si page=stnt1d_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS stnt1d_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b11 ig1=0b00 Rm:u=0bxxxxx ig2=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1D status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_cstnt_ss page=stnt1d_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS stnt1h_z_p_ar_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b10 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1H status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_sstnt_32b_vs page=stnt1h_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS stnt1h_z_p_ar_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b00 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1H status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_sstnt_64b_vs page=stnt1h_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS stnt1h_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b001 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_cstnt_si page=stnt1h_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS stnt1h_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b01 ig1=0b00 Rm:u=0bxxxxx ig2=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1H status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_cstnt_ss page=stnt1h_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS stnt1w_z_p_ar_s_x32_unscaled => {
  name       => '32-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b10 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1W status=Green sve-offset-type=off_s_x32_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_sstnt_32b_vs page=stnt1w_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS stnt1w_z_p_ar_d_64_unscaled => {
  name       => '64-bit unscaled offset',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b00 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1W status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_sstnt_64b_vs page=stnt1w_z_p_ar',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS stnt1w_z_p_bi_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b001 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_si iclass=sve_mem_cstnt_si page=stnt1w_z_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS stnt1w_z_p_br_contiguous => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010 msz=0b10 ig1=0b00 Rm:u=0bxxxxx ig2=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1W status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1',
  tags       => 'group=sve_memst_nt iclass=sve_mem_cstnt_ss page=stnt1w_z_p_br',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (Rm == 31)',
};

CLASS str_p_bi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010110 imm9h:u=0bxxxxxx ig1=0b000 imm9l:u=0bxxx Rn:u=0bxxxxx ig2=0b0 Pt:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_cs iclass=sve_mem_pspill page=str_p_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS str_z_bi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b1110010110 imm9h:u=0bxxxxxx ig1=0b010 imm9l:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_cs iclass=sve_mem_spill page=str_z_bi',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sub_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b000 opc=0b001 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_0 page=sub_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS sub_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 opc=0b001 ig2=0b11 sh:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm0 page=sub_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size:sh == 1)',
};

CLASS sub_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b000 opc=0b001 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_bin_cons_arit_0 page=sub_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS subhnb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S=0b1 R=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SUBHNB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_arith_narrow page=subhnb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS subhnt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S=0b1 R=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SUBHNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_arith_narrow page=subhnt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS subr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b000 opc=0b011 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUBR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_0 page=subr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS subr_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 opc=0b011 ig2=0b11 sh:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUBR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm0 page=subr_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size:sh == 1)',
};

CLASS sudot_z_zzzi_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00011 U=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mixed_dot_by_indexed_elem page=sudot_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE() || !HaveInt8MatMulExt()',
};

CLASS sunpkhi_z_z_ => {
  name       => 'High half',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1100 U=0b0 H=0b1 ig2=0b001110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUNPKHI status=Green sve-half=sve-high-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_unpk page=sunpkhi_z_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS sunpklo_z_z_ => {
  name       => 'Low half',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1100 U=0b0 H=0b0 ig2=0b001110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUNPKLO status=Green sve-half=sve-low-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_unpk page=sunpkhi_z_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS suqadd_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b011 op=0b1 S=0b0 U=0b0 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SUQADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary_sat page=suqadd_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS sxtb_z_p_z_ => {
  name       => 'Byte',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 ig2=0b00 U=0b0 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SXTB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_0 page=sxtb_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS sxth_z_p_z_ => {
  name       => 'Halfword',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 ig2=0b01 U=0b0 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SXTH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_0 page=sxtb_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size != 0b1x)',
};

CLASS sxtw_z_p_z_ => {
  name       => 'Word',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 ig2=0b10 U=0b0 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SXTW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_0 page=sxtb_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size != 3)',
};

CLASS tbl_z_zz_1 => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=TBL no-reg-for-table=tbl1 status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_tbl page=tbl_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS tbl_z_zz_2 => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b00101 op=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=TBL no-reg-for-table=tbl2 status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_tbl_3src page=tbl_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS tbx_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b00101 op=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=TBX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_tbl_3src page=tbx_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS trn1_p_pp_ => {
  name       => 'Even',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10 Pm:u=0bxxxx ig2=0b010 opc=0b10 H=0b0 ig3=0b0 Pn:u=0bxxxx ig4=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=TRN1 status=Green sve-odd-even=sve-even',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_bin_perm_pp page=trn1_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS trn2_p_pp_ => {
  name       => 'Odd',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10 Pm:u=0bxxxx ig2=0b010 opc=0b10 H=0b1 ig3=0b0 Pn:u=0bxxxx ig4=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=TRN2 status=Green sve-odd-even=sve-odd',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_bin_perm_pp page=trn1_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS trn1_z_zz_ => {
  name       => 'Even',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 ig3=0b10 H=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=TRN1 status=Green sve-odd-even=sve-even',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_perm_zz page=trn1_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS trn1_z_zz_q => {
  name       => 'Even (quadwords)',
  diagram    => 'ig0=0b000001011 op=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b11 H=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=TRN1 status=Green sve-odd-even=sve-even-quad',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_long_perm_zz page=trn1_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVEFP64MatMulExt()',
};

CLASS trn2_z_zz_ => {
  name       => 'Odd',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 ig3=0b10 H=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=TRN2 status=Green sve-odd-even=sve-odd',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_perm_zz page=trn1_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS trn2_z_zz_q => {
  name       => 'Odd (quadwords)',
  diagram    => 'ig0=0b000001011 op=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b11 H=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=TRN2 status=Green sve-odd-even=sve-odd-quad',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_long_perm_zz page=trn1_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVEFP64MatMulExt()',
};

CLASS uaba_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b11111 U=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UABA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_aba page=uaba_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uabalb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1100 U=0b1 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UABALB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_aba_long page=uabalb_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS uabalt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1100 U=0b1 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UABALT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_aba_long page=uabalt_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS uabd_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b10 U=0b1 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UABD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_1 page=uabd_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uabdlb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b1 S=0b1 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UABDLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=uabdlb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS uabdlt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b1 S=0b1 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UABDLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=uabdlt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS uadalp_z_p_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00010 U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UADALP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_accumulate_long_pairs page=uadalp_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS uaddlb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b0 S=0b0 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UADDLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=uaddlb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS uaddlt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b0 S=0b0 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UADDLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=uaddlt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS uaddv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b000 opc=0b00 U=0b1 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UADDV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_0 page=uaddv_r_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uaddwb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b0 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UADDWB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_wide page=uaddwb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS uaddwt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b0 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UADDWT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_wide page=uaddwt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS ucvtf_z_p_z_h2fp16 => {
  name       => '16-bit to half-precision',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b010 opc2=0b01 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=16-to-half instr-class=sve isa=A64 mnemonic=UCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=ucvtf_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ucvtf_z_p_z_w2fp16 => {
  name       => '32-bit to half-precision',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b010 opc2=0b10 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=32-to-half instr-class=sve isa=A64 mnemonic=UCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=ucvtf_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ucvtf_z_p_z_w2s => {
  name       => '32-bit to single-precision',
  diagram    => 'ig0=0b01100101 opc=0b10 ig1=0b010 opc2=0b10 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=32-to-single instr-class=sve isa=A64 mnemonic=UCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=ucvtf_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ucvtf_z_p_z_w2d => {
  name       => '32-bit to double-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b010 opc2=0b00 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=32-to-double instr-class=sve isa=A64 mnemonic=UCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=ucvtf_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ucvtf_z_p_z_x2fp16 => {
  name       => '64-bit to half-precision',
  diagram    => 'ig0=0b01100101 opc=0b01 ig1=0b010 opc2=0b11 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=64-to-half instr-class=sve isa=A64 mnemonic=UCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=ucvtf_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ucvtf_z_p_z_x2s => {
  name       => '64-bit to single-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b010 opc2=0b10 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=64-to-single instr-class=sve isa=A64 mnemonic=UCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=ucvtf_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ucvtf_z_p_z_x2d => {
  name       => '64-bit to double-precision',
  diagram    => 'ig0=0b01100101 opc=0b11 ig1=0b010 opc2=0b11 int_U=0b1 ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'convert-type=64-to-double instr-class=sve isa=A64 mnemonic=UCVTF status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_fp_unary iclass=sve_fp_2op_p_zd_c page=ucvtf_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS udiv_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0101 R=0b0 U=0b1 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UDIV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_div page=udiv_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0b0x)',
};

CLASS udivr_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0101 R=0b1 U=0b1 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UDIVR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_div page=udivr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0b0x)',
};

CLASS udot_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00000 U=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_dot page=udot_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0b0x)',
};

CLASS udot_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00000 U=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve isa=A64 mnemonic=UDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_dot_by_indexed_elem page=udot_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS udot_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i1:u=0bx Zm:u=0bxxxx ig2=0b00000 U=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve isa=A64 mnemonic=UDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_dot_by_indexed_elem page=udot_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uhadd_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 R=0b0 S=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UHADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary page=uhadd_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uhsub_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 R=0b0 S=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UHSUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary page=uhsub_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uhsubr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 R=0b1 S=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UHSUBR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary page=uhsubr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS umax_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b00 U=0b1 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMAX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_1 page=umax_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS umax_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b101 ig2=0b00 U=0b1 ig3=0b11 o2=0b0 imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMAX status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm1 page=umax_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS umaxp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 opc=0b10 U=0b1 ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMAXP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_arith_binary_pairs page=umaxp_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS umaxv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b00 U=0b1 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMAXV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_1 page=umaxv_r_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS umin_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b01 U=0b1 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMIN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_1 page=umin_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS umin_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b101 ig2=0b01 U=0b1 ig3=0b11 o2=0b0 imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMIN status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm1 page=umin_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uminp_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 opc=0b11 U=0b1 ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMINP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_arith_binary_pairs page=uminp_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uminv_r_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc=0b01 U=0b1 ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMINV status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_pred_red iclass=sve_int_reduce_1 page=uminv_r_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS umlalb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b0 U=0b1 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMLALB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mlal_long page=umlalb_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS umlalb_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b10 S=0b0 U=0b1 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=UMLALB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=umlalb_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS umlalb_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b10 S=0b0 U=0b1 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=UMLALB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=umlalb_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS umlalt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b0 U=0b1 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMLALT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mlal_long page=umlalt_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS umlalt_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b10 S=0b0 U=0b1 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=UMLALT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=umlalt_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS umlalt_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b10 S=0b0 U=0b1 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=UMLALT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=umlalt_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS umlslb_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b1 U=0b1 T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMLSLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mlal_long page=umlslb_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS umlslb_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b10 S=0b1 U=0b1 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=UMLSLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=umlslb_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS umlslb_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b10 S=0b1 U=0b1 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=UMLSLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=umlslb_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS umlslt_z_zzz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b1 U=0b1 T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMLSLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mlal_long page=umlslt_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS umlslt_z_zzzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b10 S=0b1 U=0b1 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=UMLSLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=umlslt_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS umlslt_z_zzzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b10 S=0b1 U=0b1 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=UMLSLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mla_long_by_indexed_elem page=umlslt_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ummla_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01000101 uns=0b11 ig1=0b0 Zm:u=0bxxxxx ig2=0b100110 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_mmla page=ummla_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE() || !HaveInt8MatMulExt()',
};

CLASS umulh_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0100 H=0b1 U=0b1 ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMULH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_bin iclass=sve_int_bin_pred_arit_2 page=umulh_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS umulh_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0110 ig3=0b1 U=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMULH status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_arit_b iclass=sve_int_mul_b page=umulh_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS umullb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op=0b1 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMULLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_mul_long page=umullb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS umullb_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b110 U=0b1 i3l:u=0bx T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=UMULLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_long_by_indexed_elem page=umullb_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS umullb_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b110 U=0b1 i2l:u=0bx T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=UMULLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_long_by_indexed_elem page=umullb_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS umullt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op=0b1 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMULLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_mul_long page=umullt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS umullt_z_zzi_s => {
  name       => '32-bit',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b110 U=0b1 i3l:u=0bx T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-words instr-class=sve2 isa=A64 mnemonic=UMULLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_long_by_indexed_elem page=umullt_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS umullt_z_zzi_d => {
  name       => '64-bit',
  diagram    => 'ig0=0b01000100 size=0b11 ig1=0b1 i2h:u=0bx Zm:u=0bxxxx ig2=0b110 U=0b1 i2l:u=0bx T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'as-structure-org=of-doublewords instr-class=sve2 isa=A64 mnemonic=UMULLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mul_long_by_indexed_elem page=umullt_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uqadd_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b011 op=0b0 S=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary_sat page=uqadd_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uqadd_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 ig2=0b10 U=0b1 ig3=0b11 sh:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm0 page=uqadd_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size:sh == 1)',
};

CLASS uqadd_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b10 U=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve iclass=sve_int_bin_cons_arit_0 page=uqadd_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqdecb_r_rs_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECB-32-fsreg instr-class=sve isa=A64 mnemonic=UQDECB reg-type=32-fsreg status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqdecb_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqdecb_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECB-64-fsreg instr-class=sve isa=A64 mnemonic=UQDECB reg-type=64-fsreg status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqdecb_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqdecd_r_rs_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECD-32-fsreg instr-class=sve isa=A64 mnemonic=UQDECD reg-type=32-fsreg status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqdecd_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqdecd_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECD-64-fsreg instr-class=sve isa=A64 mnemonic=UQDECD reg-type=64-fsreg status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqdecd_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqdecd_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b1 U=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQDECD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=uqdecd_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqdech_r_rs_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECH-32-fsreg instr-class=sve isa=A64 mnemonic=UQDECH reg-type=32-fsreg status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqdech_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqdech_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECH-64-fsreg instr-class=sve isa=A64 mnemonic=UQDECH reg-type=64-fsreg status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqdech_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqdech_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b1 U=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQDECH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=uqdech_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqdecp_r_p_r_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b1 U=0b1 ig2=0b10001 sf=0b0 op=0b0 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECP-32-fsreg instr-class=sve isa=A64 mnemonic=UQDECP reg-type=32-fsreg status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r_sat page=uqdecp_r_p_r',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqdecp_r_p_r_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b1 U=0b1 ig2=0b10001 sf=0b1 op=0b0 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECP-64-fsreg instr-class=sve isa=A64 mnemonic=UQDECP reg-type=64-fsreg status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r_sat page=uqdecp_r_p_r',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqdecp_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b1 U=0b1 ig2=0b10000 opc=0b00 Pm:u=0bxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQDECP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_v_sat page=uqdecp_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS uqdecw_r_rs_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECW-32-fsreg instr-class=sve isa=A64 mnemonic=UQDECW reg-type=32-fsreg status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqdecw_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqdecw_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b1 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQDECW-64-fsreg instr-class=sve isa=A64 mnemonic=UQDECW reg-type=64-fsreg status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqdecw_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqdecw_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b1 U=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQDECW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=uqdecw_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqincb_r_rs_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCB-32-fsreg instr-class=sve isa=A64 mnemonic=UQINCB reg-type=32-fsreg status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqincb_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqincb_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b00 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCB-64-fsreg instr-class=sve isa=A64 mnemonic=UQINCB reg-type=64-fsreg status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqincb_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqincd_r_rs_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCD-32-fsreg instr-class=sve isa=A64 mnemonic=UQINCD reg-type=32-fsreg status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqincd_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqincd_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCD-64-fsreg instr-class=sve isa=A64 mnemonic=UQINCD reg-type=64-fsreg status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqincd_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqincd_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b11 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b0 U=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQINCD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=uqincd_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqinch_r_rs_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCH-32-fsreg instr-class=sve isa=A64 mnemonic=UQINCH reg-type=32-fsreg status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqinch_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqinch_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCH-64-fsreg instr-class=sve isa=A64 mnemonic=UQINCH reg-type=64-fsreg status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqinch_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqinch_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b01 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b0 U=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQINCH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=uqinch_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqincp_r_p_r_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b0 U=0b1 ig2=0b10001 sf=0b0 op=0b0 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCP-32-fsreg instr-class=sve isa=A64 mnemonic=UQINCP reg-type=32-fsreg status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r_sat page=uqincp_r_p_r',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqincp_r_p_r_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b0 U=0b1 ig2=0b10001 sf=0b1 op=0b0 Pm:u=0bxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCP-64-fsreg instr-class=sve isa=A64 mnemonic=UQINCP reg-type=64-fsreg status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_r_sat page=uqincp_r_p_r',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqincp_z_p_z_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D=0b0 U=0b1 ig2=0b10000 opc=0b00 Pm:u=0bxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQINCP status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_pred_count_b iclass=sve_int_count_v_sat page=uqincp_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS uqincw_r_rs_uw => {
  name       => '32-bit',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b1 sf=0b0 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCW-32-fsreg instr-class=sve isa=A64 mnemonic=UQINCW reg-type=32-fsreg status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqincw_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqincw_r_rs_x => {
  name       => '64-bit',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b1 sf=0b1 imm4:u=0bxxxx ig2=0b1111 D=0b0 U=0b1 pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  docvars    => 'atomic-ops=UQINCW-64-fsreg instr-class=sve isa=A64 mnemonic=UQINCW reg-type=64-fsreg status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_pred_pattern_b page=uqincw_r_rs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqincw_z_zs_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size=0b10 ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D=0b0 U=0b1 pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQINCW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_countelt iclass=sve_int_countvlv0 page=uqincw_z_zs',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqrshl_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 R=0b0 N=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQRSHL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=uqrshl_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uqrshlr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 R=0b1 N=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQRSHLR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=uqrshlr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uqrshrnb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b1 U=0b1 R=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQRSHRNB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=uqrshrnb_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uqrshrnt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b1 U=0b1 R=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQRSHRNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=uqrshrnt_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uqshl_z_p_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b01 L=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSHL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=uqshl_z_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uqshl_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 R=0b0 N=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSHL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=uqshl_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uqshlr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b1 R=0b1 N=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSHLR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=uqshlr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uqshrnb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b1 U=0b1 R=0b0 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSHRNB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=uqshrnb_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uqshrnt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op=0b1 U=0b1 R=0b0 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSHRNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_shift_narrow page=uqshrnt_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uqsub_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b011 op=0b0 S=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary_sat page=uqsub_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uqsub_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 ig2=0b11 U=0b1 ig3=0b11 sh:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQSUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_arith_imm0 page=uqsub_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size:sh == 1)',
};

CLASS uqsub_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b11 U=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQSUB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve iclass=sve_int_bin_cons_arit_0 page=uqsub_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uqsubr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b011 op=0b1 S=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSUBR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary_sat page=uqsubr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uqxtnb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx ig2=0b000010 ig3=0b0 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQXTNB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_extract_narrow page=uqxtnb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS uqxtnt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx ig2=0b000010 ig3=0b0 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQXTNT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_narrowing iclass=sve_intx_extract_narrow page=uqxtnt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS urecpe_z_p_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b0 ig2=0b0 opc=0b00 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URECPE status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_unary page=urecpe_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size != 2)',
};

CLASS urhadd_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 R=0b1 S=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URHADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary page=urhadd_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS urshl_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b0 R=0b0 N=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URSHL status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=urshl_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS urshlr_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b0 R=0b1 N=0b1 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URSHLR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_bin_pred_shift_sat_round page=urshlr_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS urshr_z_p_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc=0b11 L=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URSHR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_int_pred_shift iclass=sve_int_bin_pred_shift_0 page=urshr_z_p_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ursqrte_z_p_z_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q=0b0 ig2=0b0 opc=0b01 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URSQRTE status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_unary page=ursqrte_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size != 2)',
};

CLASS ursra_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 tszh:u=0bxx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1110 R=0b1 U=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URSRA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_acc iclass=sve_intx_sra page=ursra_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS usdot_z_zzz_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b0 Zm:u=0bxxxxx ig2=0b011110 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=USDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_muladd_unpred iclass=sve_intx_mixed_dot page=usdot_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE() || !HaveInt8MatMulExt()',
};

CLASS usdot_z_zzzi_s => {
  name       => 'SVE',
  diagram    => 'ig0=0b01000100 size=0b10 ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b00011 U=0b0 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=USDOT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_by_indexed_elem iclass=sve_intx_mixed_dot_by_indexed_elem page=usdot_z_zzzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE() || !HaveInt8MatMulExt()',
};

CLASS ushllb_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1010 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USHLLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_shift_long page=ushllb_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS ushllt_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1010 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USHLLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_shift_long page=ushllt_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS usmmla_z_zzz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b01000101 uns=0b10 ig1=0b0 Zm:u=0bxxxxx ig2=0b100110 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=USMMLA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_intx_constructive iclass=sve_intx_mmla page=usmmla_z_zzz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE() || !HaveInt8MatMulExt()',
};

CLASS usqadd_z_p_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b011 op=0b1 S=0b0 U=0b1 ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USQADD status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
  tags       => 'group=sve_intx_predicated iclass=sve_intx_pred_arith_binary_sat page=usqadd_z_p_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS usra_z_zi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 tszh:u=0bxx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1110 R=0b0 U=0b1 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USRA status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_acc iclass=sve_intx_sra page=usra_z_zi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS usublb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b0 S=0b1 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USUBLB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=usublb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS usublt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op=0b0 S=0b1 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USUBLT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_long page=usublt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS usubwb_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b1 U=0b1 T=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USUBWB status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_wide page=usubwb_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS usubwt_z_zz_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S=0b1 U=0b1 T=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USUBWT status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_intx_cons_widening iclass=sve_intx_cons_arith_wide page=usubwt_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE2()) || (size == 0)',
};

CLASS uunpkhi_z_z_ => {
  name       => 'High half',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1100 U=0b1 H=0b1 ig2=0b001110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UUNPKHI status=Green sve-half=sve-high-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_unpk page=uunpkhi_z_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS uunpklo_z_z_ => {
  name       => 'Low half',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1100 U=0b1 H=0b0 ig2=0b001110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UUNPKLO status=Green sve-half=sve-low-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_unpk page=uunpkhi_z_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS uxtb_z_p_z_ => {
  name       => 'Byte',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 ig2=0b00 U=0b1 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UXTB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_0 page=uxtb_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS uxth_z_p_z_ => {
  name       => 'Halfword',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 ig2=0b01 U=0b1 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UXTH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_0 page=uxtb_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size != 0b1x)',
};

CLASS uxtw_z_p_z_ => {
  name       => 'Word',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 ig2=0b10 U=0b1 ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UXTW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_int_pred_un iclass=sve_int_un_pred_arit_0 page=uxtb_z_p_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size != 3)',
};

CLASS uzp1_p_pp_ => {
  name       => 'Even',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10 Pm:u=0bxxxx ig2=0b010 opc=0b01 H=0b0 ig3=0b0 Pn:u=0bxxxx ig4=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UZP1 status=Green sve-odd-even=sve-even',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_bin_perm_pp page=uzp1_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uzp2_p_pp_ => {
  name       => 'Odd',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10 Pm:u=0bxxxx ig2=0b010 opc=0b01 H=0b1 ig3=0b0 Pn:u=0bxxxx ig4=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UZP2 status=Green sve-odd-even=sve-odd',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_bin_perm_pp page=uzp1_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uzp1_z_zz_ => {
  name       => 'Even',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 ig3=0b01 H=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UZP1 status=Green sve-odd-even=sve-even',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_perm_zz page=uzp1_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uzp1_z_zz_q => {
  name       => 'Even (quadwords)',
  diagram    => 'ig0=0b000001011 op=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b01 H=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UZP1 status=Green sve-odd-even=sve-even-quad',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_long_perm_zz page=uzp1_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVEFP64MatMulExt()',
};

CLASS uzp2_z_zz_ => {
  name       => 'Odd',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 ig3=0b01 H=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UZP2 status=Green sve-odd-even=sve-odd',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_perm_zz page=uzp1_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS uzp2_z_zz_q => {
  name       => 'Odd (quadwords)',
  diagram    => 'ig0=0b000001011 op=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b01 H=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UZP2 status=Green sve-odd-even=sve-odd-quad',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_long_perm_zz page=uzp1_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVEFP64MatMulExt()',
};

CLASS whilege_p_p_rr_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U=0b0 lt=0b0 Rn:u=0bxxxxx eq=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILEGE status=Green sve-compare-type=ge',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_while_rr page=whilege_p_p_rr',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS whilegt_p_p_rr_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U=0b0 lt=0b0 Rn:u=0bxxxxx eq=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILEGT status=Green sve-compare-type=gt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_while_rr page=whilegt_p_p_rr',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS whilehi_p_p_rr_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U=0b1 lt=0b0 Rn:u=0bxxxxx eq=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILEHI status=Green sve-compare-type=hi',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_while_rr page=whilehi_p_p_rr',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS whilehs_p_p_rr_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U=0b1 lt=0b0 Rn:u=0bxxxxx eq=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILEHS status=Green sve-compare-type=hs',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_while_rr page=whilehs_p_p_rr',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS whilele_p_p_rr_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U=0b0 lt=0b1 Rn:u=0bxxxxx eq=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=WHILELE status=Green sve-compare-type=le',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_while_rr page=whilele_p_p_rr',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS whilelo_p_p_rr_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U=0b1 lt=0b1 Rn:u=0bxxxxx eq=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=WHILELO status=Green sve-compare-type=lo',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_while_rr page=whilelo_p_p_rr',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS whilels_p_p_rr_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U=0b1 lt=0b1 Rn:u=0bxxxxx eq=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=WHILELS status=Green sve-compare-type=ls',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_while_rr page=whilels_p_p_rr',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS whilelt_p_p_rr_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U=0b0 lt=0b1 Rn:u=0bxxxxx eq=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=WHILELT status=Green sve-compare-type=lt',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpgpr iclass=sve_int_while_rr page=whilelt_p_p_rr',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS whilerw_p_rr_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b001100 Rn:u=0bxxxxx rw=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILERW status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_cmpgpr iclass=sve_int_whilenc page=whilerw_p_rr',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS whilewr_p_rr_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b001100 Rn:u=0bxxxxx rw=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILEWR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
  tags       => 'group=sve_cmpgpr iclass=sve_int_whilenc page=whilewr_p_rr',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS wrffr_f_p_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00 ig1=0b100101 opc=0b00 ig2=0b10 ig3=0b1000100100 ig4=0b0 Pn:u=0bxxxx ig5=0b0 ig6=0b0 ig7=0b0 ig8=0b00',
  docvars    => 'instr-class=sve isa=A64 mnemonic=WRFFR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_pred_wrffr iclass=sve_int_wrffr page=wrffr_f_p',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS xar_z_zzi_ => {
  name       => 'SVE2',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b001101 Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=XAR status=Green',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_rotate_imm page=xar_z_zzi',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE2()',
};

CLASS zip2_p_pp_ => {
  name       => 'High halves',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10 Pm:u=0bxxxx ig2=0b010 opc=0b00 H=0b1 ig3=0b0 Pn:u=0bxxxx ig4=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ZIP2 status=Green sve-halves=sve-high-halves',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_bin_perm_pp page=zip1_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS zip1_p_pp_ => {
  name       => 'Low halves',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10 Pm:u=0bxxxx ig2=0b010 opc=0b00 H=0b0 ig3=0b0 Pn:u=0bxxxx ig4=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ZIP1 status=Green sve-halves=sve-low-halves',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_predicates iclass=sve_int_perm_bin_perm_pp page=zip1_p_pp',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS zip2_z_zz_ => {
  name       => 'High halves',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 ig3=0b00 H=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ZIP2 status=Green sve-halves=sve-high-halves',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_perm_zz page=zip1_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS zip2_z_zz_q => {
  name       => 'High halves (quadwords)',
  diagram    => 'ig0=0b000001011 op=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b00 H=0b1 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ZIP2 status=Green sve-halves=sve-high-halves-quad',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_long_perm_zz page=zip1_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVEFP64MatMulExt()',
};

CLASS zip1_z_zz_ => {
  name       => 'Low halves',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 ig3=0b00 H=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ZIP1 status=Green sve-halves=sve-low-halves',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_perm_zz page=zip1_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS zip1_z_zz_q => {
  name       => 'Low halves (quadwords)',
  diagram    => 'ig0=0b000001011 op=0b0 ig1=0b1 Zm:u=0bxxxxx ig2=0b000 ig3=0b00 H=0b0 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ZIP1 status=Green sve-halves=sve-low-halves-quad',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_perm_bin_long_perm_zz page=zip1_z_zz',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVEFP64MatMulExt()',
};

CLASS BIC_and_z_zi_and_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 opc=0b10 ig1=0b0000 imm13:u=0bxxxxxxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=AND status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_maskimm iclass=sve_int_log_imm page=BIC_and_z_zi xid=and_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS CMPLE_cmpeq_p_p_zz_cmpge_p_p_zz_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPGE status=Green sve-compare-type=ge',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=CMPLE_cmpeq_p_p_zz xid=cmpge_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS CMPLO_cmpeq_p_p_zz_cmphi_p_p_zz_ => {
  name       => 'Higher',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPHI status=Green sve-compare-type=hi',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=CMPLO_cmpeq_p_p_zz xid=cmphi_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS CMPLS_cmpeq_p_p_zz_cmphs_p_p_zz_ => {
  name       => 'Higher or same',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPHS status=Green sve-compare-type=hs',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=CMPLS_cmpeq_p_p_zz xid=cmphs_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS CMPLT_cmpeq_p_p_zz_cmpgt_p_p_zz_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b0 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx ne=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CMPGT status=Green sve-compare-type=gt',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_cmpvec iclass=sve_int_cmp_0 page=CMPLT_cmpeq_p_p_zz xid=cmpgt_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS EON_eor_z_zi_eor_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 opc=0b01 ig1=0b0000 imm13:u=0bxxxxxxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EOR status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_maskimm iclass=sve_int_log_imm page=EON_eor_z_zi xid=eor_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS FACLE_facge_p_p_zz_facge_p_p_zz_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b1 o2=0b0 Pg:u=0bxxx Zn:u=0bxxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FACGE status=Green sve-compare-type=ge',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=FACLE_facge_p_p_zz xid=facge_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS FACLT_facge_p_p_zz_facgt_p_p_zz_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b1 ig2=0b1 o2=0b1 Pg:u=0bxxx Zn:u=0bxxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FACGT status=Green sve-compare-type=gt',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=FACLT_facge_p_p_zz xid=facgt_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS FCMLE_fcmeq_p_p_zz_fcmge_p_p_zz_ => {
  name       => 'Greater than or equal',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b1 cmph=0b0 Pg:u=0bxxx Zn:u=0bxxxxx cmpl=0b0 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMGE status=Green sve-compare-type=ge',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=FCMLE_fcmeq_p_p_zz xid=fcmge_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS FCMLT_fcmeq_p_p_zz_fcmgt_p_p_zz_ => {
  name       => 'Greater than',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op=0b0 ig2=0b1 cmph=0b0 Pg:u=0bxxx Zn:u=0bxxxxx cmpl=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMGT status=Green sve-compare-type=gt',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve iclass=sve_fp_3op_p_pd page=FCMLT_fcmeq_p_p_zz xid=fcmgt_p_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS FMOV_cpy_z_p_i_cpy_z_p_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b0 M=0b1 sh:u=0b0 imm8:u=0b00000000 Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_wideimm_pred iclass=sve_int_dup_imm_pred page=FMOV_cpy_z_p_i xid=cpy_z_p_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size:sh == 1)',
};

CLASS FMOV_dup_z_i_dup_z_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b111 opc=0b00 ig2=0b011 sh:u=0b0 imm8:u=0b00000000 Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_dup_imm page=FMOV_dup_z_i xid=dup_z_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size:sh == 1)',
};

CLASS FMOV_fcpy_z_p_i_fcpy_z_p_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b110 imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCPY status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
  tags       => 'group=sve_wideimm_pred iclass=sve_int_dup_fpimm_pred page=FMOV_fcpy_z_p_i xid=fcpy_z_p_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS FMOV_fdup_z_i_fdup_z_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b111 opc=0b00 ig2=0b111 o2=0b0 imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FDUP status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_dup_fpimm page=FMOV_fdup_z_i xid=fdup_z_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size == 0)',
};

CLASS MOV_and_p_p_pp_and_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S:u=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=AND status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=MOV_and_p_p_pp xid=and_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS MOV_cpy_z_o_i_cpy_z_o_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b0 M=0b0 sh:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_pred iclass=sve_int_dup_imm_pred page=MOV_cpy_z_o_i xid=cpy_z_o_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size:sh == 1)',
};

CLASS MOV_cpy_z_p_i_cpy_z_p_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b0 M=0b1 sh:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_wideimm_pred iclass=sve_int_dup_imm_pred page=MOV_cpy_z_p_i xid=cpy_z_p_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size:sh == 1)',
};

CLASS MOV_cpy_z_p_r_cpy_z_p_r_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b101000101 Pg:u=0bxxx Rn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_cpy_r page=MOV_cpy_z_p_r xid=cpy_z_p_r_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS MOV_cpy_z_p_v_cpy_z_p_v_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b100000100 Pg:u=0bxxx Vn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
  tags       => 'group=sve_perm_pred iclass=sve_int_perm_cpy_v page=MOV_cpy_z_p_v xid=cpy_z_p_v_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS MOV_dup_z_i_dup_z_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b111 opc=0b00 ig2=0b011 sh:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_wideimm_unpred iclass=sve_int_dup_imm page=MOV_dup_z_i xid=dup_z_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSVE()) || (size:sh == 1)',
};

CLASS MOV_dup_z_r_dup_z_r_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b100000001110 Rn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_dup_r page=MOV_dup_z_r xid=dup_z_r_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS MOV_dup_z_zi_dup_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 imm2:u=0bxx ig1=0b1 tsz:u=0bxxxxx ig2=0b001000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_perm_unpred iclass=sve_int_perm_dup_i page=MOV_dup_z_zi xid=dup_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS MOV_dupm_z_i_dupm_z_i_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101110000 imm13:u=0bxxxxxxxxxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DUPM status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_maskimm iclass=sve_int_dup_mask_imm page=MOV_dupm_z_i xid=dupm_z_i_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS MOV_orr_p_p_pp_orr_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S:u=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=MOV_orr_p_p_pp xid=orr_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS MOV_orr_z_zz_orr_z_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000100 opc=0b01 ig1=0b1 Zm:u=0bxxxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_int_unpred_logical iclass=sve_int_bin_cons_log page=MOV_orr_z_zz xid=orr_z_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS MOV_sel_p_p_pp_sel_p_p_pp_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b1 Pd:u=0bxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SEL status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=MOV_sel_p_p_pp xid=sel_p_p_pp_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS MOV_sel_z_p_zz_sel_z_p_zz_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b11 Pg:u=0bxxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SEL status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_sel_vvv page=MOV_sel_z_p_zz xid=sel_z_p_zz_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS MOVS_and_p_p_pp_ands_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S:u=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=ANDS status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=MOVS_and_p_p_pp xid=ands_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS MOVS_orr_p_p_pp_orrs_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b1 S:u=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b0 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=ORRS status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=MOVS_orr_p_p_pp xid=orrs_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS NOT_eor_p_p_pp_eor_p_p_pp_z => {
  name       => 'Not setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b0 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=no-s instr-class=sve isa=A64 mnemonic=EOR status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=NOT_eor_p_p_pp xid=eor_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS NOTS_eor_p_p_pp_eors_p_p_pp_z => {
  name       => 'Setting the condition flags',
  diagram    => 'ig0=0b00100101 op=0b0 S=0b1 ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2=0b1 Pn:u=0bxxxx o3=0b0 Pd:u=0bxxxx',
  docvars    => 'cond-setting=s instr-class=sve isa=A64 mnemonic=EORS status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve iclass=sve_int_pred_log page=NOTS_eor_p_p_pp xid=eors_p_p_pp_z',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};

CLASS ORN_orr_z_zi_orr_z_zi_ => {
  name       => 'SVE',
  diagram    => 'ig0=0b00000101 opc=0b00 ig1=0b0000 imm13:u=0bxxxxxxxxxxxxx Zdn:u=0bxxxxx',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
  tags       => 'group=sve_maskimm iclass=sve_int_log_imm page=ORN_orr_z_zi xid=orr_z_zi_',
  exceptions => 'ADVSIMDFPACCESSTRAP|SVEACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSVE()',
};


1;

