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

PAGE ADC_i => {
  title      => 'ADC, ADCS (immediate)',
  brief      => 'Add with Carry (immediate)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADC_r => {
  title      => 'ADC, ADCS (register)',
  brief      => 'Add with Carry (register)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADC_rr => {
  title      => 'ADC, ADCS (register-shifted register)',
  brief      => 'Add with Carry (register-shifted register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADD_i => {
  title      => 'ADD, ADDS (immediate)',
  brief      => 'Add (immediate)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADD_r => {
  title      => 'ADD, ADDS (register)',
  brief      => 'Add (register)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADD_rr => {
  title      => 'ADD, ADDS (register-shifted register)',
  brief      => 'Add (register-shifted register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADD_SP_i => {
  title      => 'ADD, ADDS (SP plus immediate)',
  brief      => 'Add to SP (immediate)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADD_SP_r => {
  title      => 'ADD, ADDS (SP plus register)',
  brief      => 'Add to SP (register)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADR => {
  title      => 'ADR',
  brief      => 'Form PC-relative address',
  docvars    => 'instr-class=general mnemonic=ADR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE AND_i => {
  title      => 'AND, ANDS (immediate)',
  brief      => 'Bitwise AND (immediate)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE AND_r => {
  title      => 'AND, ANDS (register)',
  brief      => 'Bitwise AND (register)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE AND_rr => {
  title      => 'AND, ANDS (register-shifted register)',
  brief      => 'Bitwise AND (register-shifted register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE B => {
  title      => 'B',
  brief      => 'Branch',
  docvars    => 'instr-class=general mnemonic=B',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BFC => {
  title      => 'BFC',
  brief      => 'Bit Field Clear',
  docvars    => 'instr-class=general mnemonic=BFC',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BFI => {
  title      => 'BFI',
  brief      => 'Bit Field Insert',
  docvars    => 'instr-class=general mnemonic=BFI',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BIC_i => {
  title      => 'BIC, BICS (immediate)',
  brief      => 'Bitwise Bit Clear (immediate)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BIC_r => {
  title      => 'BIC, BICS (register)',
  brief      => 'Bitwise Bit Clear (register)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BIC_rr => {
  title      => 'BIC, BICS (register-shifted register)',
  brief      => 'Bitwise Bit Clear (register-shifted register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BKPT => {
  title      => 'BKPT',
  brief      => 'Breakpoint',
  docvars    => 'instr-class=general mnemonic=BKPT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BL_i => {
  title      => 'BL, BLX (immediate)',
  brief      => 'Branch with Link and optional Exchange (immediate)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BLX_r => {
  title      => 'BLX (register)',
  brief      => 'Branch with Link and Exchange (register)',
  docvars    => 'instr-class=general mnemonic=BLX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BX => {
  title      => 'BX',
  brief      => 'Branch and Exchange',
  docvars    => 'instr-class=general mnemonic=BX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BXJ => {
  title      => 'BXJ',
  brief      => 'Branch and Exchange, previously Branch and Exchange Jazelle',
  docvars    => 'instr-class=general mnemonic=BXJ',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CBNZ => {
  title      => 'CBNZ, CBZ',
  brief      => 'Compare and Branch on Nonzero or Zero',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CLREX => {
  title      => 'CLREX',
  brief      => 'Clear-Exclusive',
  docvars    => 'instr-class=general mnemonic=CLREX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CLZ => {
  title      => 'CLZ',
  brief      => 'Count Leading Zeros',
  docvars    => 'instr-class=general mnemonic=CLZ',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CMN_i => {
  title      => 'CMN (immediate)',
  brief      => 'Compare Negative (immediate)',
  docvars    => 'instr-class=general mnemonic=CMN',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CMN_r => {
  title      => 'CMN (register)',
  brief      => 'Compare Negative (register)',
  docvars    => 'instr-class=general mnemonic=CMN',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CMN_rr => {
  title      => 'CMN (register-shifted register)',
  brief      => 'Compare Negative (register-shifted register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CMN',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CMP_i => {
  title      => 'CMP (immediate)',
  brief      => 'Compare (immediate)',
  docvars    => 'instr-class=general mnemonic=CMP',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CMP_r => {
  title      => 'CMP (register)',
  brief      => 'Compare (register)',
  docvars    => 'instr-class=general mnemonic=CMP',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CMP_rr => {
  title      => 'CMP (register-shifted register)',
  brief      => 'Compare (register-shifted register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CMP',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CPS => {
  title      => 'CPS, CPSID, CPSIE',
  brief      => 'Change PE State',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CRC32 => {
  title      => 'CRC32',
  brief      => 'CRC32',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CRC32C => {
  title      => 'CRC32C',
  brief      => 'CRC32C',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CSDB => {
  title      => 'CSDB',
  brief      => 'Consumption of Speculative Data Barrier',
  docvars    => 'instr-class=general mnemonic=CSDB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE DBG => {
  title      => 'DBG',
  brief      => 'Debug hint',
  docvars    => 'instr-class=general mnemonic=DBG',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE DCPS1 => {
  title      => 'DCPS1',
  brief      => 'Debug Change PE State to EL1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=DCPS1',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE DCPS2 => {
  title      => 'DCPS2',
  brief      => 'Debug Change PE State to EL2',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=DCPS2',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE DCPS3 => {
  title      => 'DCPS3',
  brief      => 'Debug Change PE State to EL3',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=DCPS3',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE DMB => {
  title      => 'DMB',
  brief      => 'Data Memory Barrier',
  docvars    => 'instr-class=general mnemonic=DMB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE DSB => {
  title      => 'DSB',
  brief      => 'Data Synchronization Barrier',
  docvars    => 'instr-class=general mnemonic=DSB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE EOR_i => {
  title      => 'EOR, EORS (immediate)',
  brief      => 'Bitwise Exclusive OR (immediate)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE EOR_r => {
  title      => 'EOR, EORS (register)',
  brief      => 'Bitwise Exclusive OR (register)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE EOR_rr => {
  title      => 'EOR, EORS (register-shifted register)',
  brief      => 'Bitwise Exclusive OR (register-shifted register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ERET => {
  title      => 'ERET',
  brief      => 'Exception Return',
  docvars    => 'instr-class=general mnemonic=ERET',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ESB => {
  title      => 'ESB',
  brief      => 'Error Synchronization Barrier',
  docvars    => 'instr-class=general mnemonic=ESB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE HLT => {
  title      => 'HLT',
  brief      => 'Halting Breakpoint',
  docvars    => 'instr-class=general mnemonic=HLT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE HVC => {
  title      => 'HVC',
  brief      => 'Hypervisor Call',
  docvars    => 'instr-class=general mnemonic=HVC',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ISB => {
  title      => 'ISB',
  brief      => 'Instruction Synchronization Barrier',
  docvars    => 'instr-class=general mnemonic=ISB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE IT => {
  title      => 'IT',
  brief      => 'If-Then',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=IT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDA => {
  title      => 'LDA',
  brief      => 'Load-Acquire Word',
  docvars    => 'instr-class=general mnemonic=LDA',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDAB => {
  title      => 'LDAB',
  brief      => 'Load-Acquire Byte',
  docvars    => 'instr-class=general mnemonic=LDAB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDAEX => {
  title      => 'LDAEX',
  brief      => 'Load-Acquire Exclusive Word',
  docvars    => 'instr-class=general mnemonic=LDAEX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDAEXB => {
  title      => 'LDAEXB',
  brief      => 'Load-Acquire Exclusive Byte',
  docvars    => 'instr-class=general mnemonic=LDAEXB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDAEXD => {
  title      => 'LDAEXD',
  brief      => 'Load-Acquire Exclusive Doubleword',
  docvars    => 'instr-class=general mnemonic=LDAEXD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDAEXH => {
  title      => 'LDAEXH',
  brief      => 'Load-Acquire Exclusive Halfword',
  docvars    => 'instr-class=general mnemonic=LDAEXH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDAH => {
  title      => 'LDAH',
  brief      => 'Load-Acquire Halfword',
  docvars    => 'instr-class=general mnemonic=LDAH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDC_i => {
  title      => 'LDC (immediate)',
  brief      => 'Load data to System register (immediate)',
  docvars    => 'instr-class=general mnemonic=LDC',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDC_l => {
  title      => 'LDC (literal)',
  brief      => 'Load data to System register (literal)',
  docvars    => 'address-form=literal instr-class=general mnemonic=LDC',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDM => {
  title      => 'LDM, LDMIA, LDMFD',
  brief      => 'Load Multiple (Increment After, Full Descending)',
  docvars    => 'instr-class=general mnemonic=LDM',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDM_e => {
  title      => 'LDM (exception return)',
  brief      => 'Load Multiple (exception return)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDM_u => {
  title      => 'LDM (User registers)',
  brief      => 'Load Multiple (User registers)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDMDA => {
  title      => 'LDMDA, LDMFA',
  brief      => 'Load Multiple Decrement After (Full Ascending)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDMDB => {
  title      => 'LDMDB, LDMEA',
  brief      => 'Load Multiple Decrement Before (Empty Ascending)',
  docvars    => 'instr-class=general mnemonic=LDM',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDMIB => {
  title      => 'LDMIB, LDMED',
  brief      => 'Load Multiple Increment Before (Empty Descending)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDR_i => {
  title      => 'LDR (immediate)',
  brief      => 'Load Register (immediate)',
  docvars    => 'instr-class=general mnemonic=LDR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDR_l => {
  title      => 'LDR (literal)',
  brief      => 'Load Register (literal)',
  docvars    => 'address-form=literal instr-class=general mnemonic=LDR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDR_r => {
  title      => 'LDR (register)',
  brief      => 'Load Register (register)',
  docvars    => 'instr-class=general mnemonic=LDR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRB_i => {
  title      => 'LDRB (immediate)',
  brief      => 'Load Register Byte (immediate)',
  docvars    => 'instr-class=general mnemonic=LDRB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRB_l => {
  title      => 'LDRB (literal)',
  brief      => 'Load Register Byte (literal)',
  docvars    => 'address-form=literal instr-class=general mnemonic=LDRB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRB_r => {
  title      => 'LDRB (register)',
  brief      => 'Load Register Byte (register)',
  docvars    => 'instr-class=general mnemonic=LDRB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRBT => {
  title      => 'LDRBT',
  brief      => 'Load Register Byte Unprivileged',
  docvars    => 'instr-class=general mnemonic=LDRBT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRD_i => {
  title      => 'LDRD (immediate)',
  brief      => 'Load Register Dual (immediate)',
  docvars    => 'instr-class=general mnemonic=LDRD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRD_l => {
  title      => 'LDRD (literal)',
  brief      => 'Load Register Dual (literal)',
  docvars    => 'address-form=literal instr-class=general mnemonic=LDRD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRD_r => {
  title      => 'LDRD (register)',
  brief      => 'Load Register Dual (register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDREX => {
  title      => 'LDREX',
  brief      => 'Load Register Exclusive',
  docvars    => 'instr-class=general mnemonic=LDREX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDREXB => {
  title      => 'LDREXB',
  brief      => 'Load Register Exclusive Byte',
  docvars    => 'instr-class=general mnemonic=LDREXB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDREXD => {
  title      => 'LDREXD',
  brief      => 'Load Register Exclusive Doubleword',
  docvars    => 'instr-class=general mnemonic=LDREXD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDREXH => {
  title      => 'LDREXH',
  brief      => 'Load Register Exclusive Halfword',
  docvars    => 'instr-class=general mnemonic=LDREXH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRH_i => {
  title      => 'LDRH (immediate)',
  brief      => 'Load Register Halfword (immediate)',
  docvars    => 'instr-class=general mnemonic=LDRH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRH_l => {
  title      => 'LDRH (literal)',
  brief      => 'Load Register Halfword (literal)',
  docvars    => 'address-form=literal instr-class=general mnemonic=LDRH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRH_r => {
  title      => 'LDRH (register)',
  brief      => 'Load Register Halfword (register)',
  docvars    => 'instr-class=general mnemonic=LDRH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRHT => {
  title      => 'LDRHT',
  brief      => 'Load Register Halfword Unprivileged',
  docvars    => 'instr-class=general mnemonic=LDRHT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRSB_i => {
  title      => 'LDRSB (immediate)',
  brief      => 'Load Register Signed Byte (immediate)',
  docvars    => 'instr-class=general mnemonic=LDRSB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRSB_l => {
  title      => 'LDRSB (literal)',
  brief      => 'Load Register Signed Byte (literal)',
  docvars    => 'address-form=literal instr-class=general mnemonic=LDRSB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRSB_r => {
  title      => 'LDRSB (register)',
  brief      => 'Load Register Signed Byte (register)',
  docvars    => 'instr-class=general mnemonic=LDRSB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRSBT => {
  title      => 'LDRSBT',
  brief      => 'Load Register Signed Byte Unprivileged',
  docvars    => 'instr-class=general mnemonic=LDRSBT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRSH_i => {
  title      => 'LDRSH (immediate)',
  brief      => 'Load Register Signed Halfword (immediate)',
  docvars    => 'instr-class=general mnemonic=LDRSH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRSH_l => {
  title      => 'LDRSH (literal)',
  brief      => 'Load Register Signed Halfword (literal)',
  docvars    => 'address-form=literal instr-class=general mnemonic=LDRSH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRSH_r => {
  title      => 'LDRSH (register)',
  brief      => 'Load Register Signed Halfword (register)',
  docvars    => 'instr-class=general mnemonic=LDRSH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRSHT => {
  title      => 'LDRSHT',
  brief      => 'Load Register Signed Halfword Unprivileged',
  docvars    => 'instr-class=general mnemonic=LDRSHT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRT => {
  title      => 'LDRT',
  brief      => 'Load Register Unprivileged',
  docvars    => 'instr-class=general mnemonic=LDRT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MCR => {
  title      => 'MCR',
  brief      => 'Move to System register from general-purpose register or execute a System instruction',
  docvars    => 'instr-class=general mnemonic=MCR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MCRR => {
  title      => 'MCRR',
  brief      => 'Move to System register from two general-purpose registers',
  docvars    => 'instr-class=general mnemonic=MCRR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MLA => {
  title      => 'MLA, MLAS',
  brief      => 'Multiply Accumulate',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MLS => {
  title      => 'MLS',
  brief      => 'Multiply and Subtract',
  docvars    => 'instr-class=general mnemonic=MLS',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MOV_i => {
  title      => 'MOV, MOVS (immediate)',
  brief      => 'Move (immediate)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MOV_r => {
  title      => 'MOV, MOVS (register)',
  brief      => 'Move (register)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MOV_rr => {
  title      => 'MOV, MOVS (register-shifted register)',
  brief      => 'Move (register-shifted register)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MOVT => {
  title      => 'MOVT',
  brief      => 'Move Top',
  docvars    => 'instr-class=general mnemonic=MOVT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MRC => {
  title      => 'MRC',
  brief      => 'Move to general-purpose register from System register',
  docvars    => 'instr-class=general mnemonic=MRC',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MRRC => {
  title      => 'MRRC',
  brief      => 'Move to two general-purpose registers from System register',
  docvars    => 'instr-class=general mnemonic=MRRC',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MRS => {
  title      => 'MRS',
  brief      => 'Move Special register to general-purpose register',
  docvars    => 'instr-class=general mnemonic=MRS',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MRS_br => {
  title      => 'MRS (Banked register)',
  brief      => 'Move Banked or Special register to general-purpose register',
  docvars    => 'instr-class=general mnemonic=MRS',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MSR_br => {
  title      => 'MSR (Banked register)',
  brief      => 'Move general-purpose register to Banked or Special register',
  docvars    => 'instr-class=general mnemonic=MSR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MSR_i => {
  title      => 'MSR (immediate)',
  brief      => 'Move immediate value to Special register',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MSR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MSR_r => {
  title      => 'MSR (register)',
  brief      => 'Move general-purpose register to Special register',
  docvars    => 'instr-class=general mnemonic=MSR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MUL => {
  title      => 'MUL, MULS',
  brief      => 'Multiply',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MVN_i => {
  title      => 'MVN, MVNS (immediate)',
  brief      => 'Bitwise NOT (immediate)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MVN_r => {
  title      => 'MVN, MVNS (register)',
  brief      => 'Bitwise NOT (register)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MVN_rr => {
  title      => 'MVN, MVNS (register-shifted register)',
  brief      => 'Bitwise NOT (register-shifted register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE NOP => {
  title      => 'NOP',
  brief      => 'No Operation',
  docvars    => 'instr-class=general mnemonic=NOP',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ORN_i => {
  title      => 'ORN, ORNS (immediate)',
  brief      => 'Bitwise OR NOT (immediate)',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ORN_r => {
  title      => 'ORN, ORNS (register)',
  brief      => 'Bitwise OR NOT (register)',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ORR_i => {
  title      => 'ORR, ORRS (immediate)',
  brief      => 'Bitwise OR (immediate)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ORR_r => {
  title      => 'ORR, ORRS (register)',
  brief      => 'Bitwise OR (register)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ORR_rr => {
  title      => 'ORR, ORRS (register-shifted register)',
  brief      => 'Bitwise OR (register-shifted register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PKH => {
  title      => 'PKHBT, PKHTB',
  brief      => 'Pack Halfword',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PLD_i => {
  title      => 'PLD, PLDW (immediate)',
  brief      => 'Preload Data (immediate)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PLD_l => {
  title      => 'PLD (literal)',
  brief      => 'Preload Data (literal)',
  docvars    => 'instr-class=general mnemonic=PLD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PLD_r => {
  title      => 'PLD, PLDW (register)',
  brief      => 'Preload Data (register)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PLI_i => {
  title      => 'PLI (immediate, literal)',
  brief      => 'Preload Instruction (immediate, literal)',
  docvars    => 'instr-class=general mnemonic=PLI',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PLI_r => {
  title      => 'PLI (register)',
  brief      => 'Preload Instruction (register)',
  docvars    => 'instr-class=general mnemonic=PLI',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE POP => {
  title      => 'POP',
  brief      => 'Pop Multiple Registers from Stack',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=POP',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PSSBB => {
  title      => 'PSSBB',
  brief      => 'Physical Speculative Store Bypass Barrier',
  docvars    => 'instr-class=general mnemonic=PSSBB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PUSH => {
  title      => 'PUSH',
  brief      => 'Push Multiple Registers to Stack',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=PUSH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE QADD => {
  title      => 'QADD',
  brief      => 'Saturating Add',
  docvars    => 'instr-class=general mnemonic=QADD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE QADD16 => {
  title      => 'QADD16',
  brief      => 'Saturating Add 16',
  docvars    => 'instr-class=general mnemonic=QADD16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE QADD8 => {
  title      => 'QADD8',
  brief      => 'Saturating Add 8',
  docvars    => 'instr-class=general mnemonic=QADD8',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE QASX => {
  title      => 'QASX',
  brief      => 'Saturating Add and Subtract with Exchange',
  docvars    => 'instr-class=general mnemonic=QASX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE QDADD => {
  title      => 'QDADD',
  brief      => 'Saturating Double and Add',
  docvars    => 'instr-class=general mnemonic=QDADD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE QDSUB => {
  title      => 'QDSUB',
  brief      => 'Saturating Double and Subtract',
  docvars    => 'instr-class=general mnemonic=QDSUB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE QSAX => {
  title      => 'QSAX',
  brief      => 'Saturating Subtract and Add with Exchange',
  docvars    => 'instr-class=general mnemonic=QSAX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE QSUB => {
  title      => 'QSUB',
  brief      => 'Saturating Subtract',
  docvars    => 'instr-class=general mnemonic=QSUB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE QSUB16 => {
  title      => 'QSUB16',
  brief      => 'Saturating Subtract 16',
  docvars    => 'instr-class=general mnemonic=QSUB16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE QSUB8 => {
  title      => 'QSUB8',
  brief      => 'Saturating Subtract 8',
  docvars    => 'instr-class=general mnemonic=QSUB8',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE RBIT => {
  title      => 'RBIT',
  brief      => 'Reverse Bits',
  docvars    => 'instr-class=general mnemonic=RBIT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE REV => {
  title      => 'REV',
  brief      => 'Byte-Reverse Word',
  docvars    => 'instr-class=general mnemonic=REV',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE REV16 => {
  title      => 'REV16',
  brief      => 'Byte-Reverse Packed Halfword',
  docvars    => 'instr-class=general mnemonic=REV16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE REVSH => {
  title      => 'REVSH',
  brief      => 'Byte-Reverse Signed Halfword',
  docvars    => 'instr-class=general mnemonic=REVSH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE RFE => {
  title      => 'RFE, RFEDA, RFEDB, RFEIA, RFEIB',
  brief      => 'Return From Exception',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE RSB_i => {
  title      => 'RSB, RSBS (immediate)',
  brief      => 'Reverse Subtract (immediate)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE RSB_r => {
  title      => 'RSB, RSBS (register)',
  brief      => 'Reverse Subtract (register)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE RSB_rr => {
  title      => 'RSB, RSBS (register-shifted register)',
  brief      => 'Reverse Subtract (register-shifted register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE RSC_i => {
  title      => 'RSC, RSCS (immediate)',
  brief      => 'Reverse Subtract with Carry (immediate)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE RSC_r => {
  title      => 'RSC, RSCS (register)',
  brief      => 'Reverse Subtract with Carry (register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE RSC_rr => {
  title      => 'RSC, RSCS (register-shifted register)',
  brief      => 'Reverse Subtract (register-shifted register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SADD16 => {
  title      => 'SADD16',
  brief      => 'Signed Add 16',
  docvars    => 'instr-class=general mnemonic=SADD16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SADD8 => {
  title      => 'SADD8',
  brief      => 'Signed Add 8',
  docvars    => 'instr-class=general mnemonic=SADD8',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SASX => {
  title      => 'SASX',
  brief      => 'Signed Add and Subtract with Exchange',
  docvars    => 'instr-class=general mnemonic=SASX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SB => {
  title      => 'SB',
  brief      => 'Speculation Barrier',
  docvars    => 'instr-class=general mnemonic=SB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SBC_i => {
  title      => 'SBC, SBCS (immediate)',
  brief      => 'Subtract with Carry (immediate)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SBC_r => {
  title      => 'SBC, SBCS (register)',
  brief      => 'Subtract with Carry (register)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SBC_rr => {
  title      => 'SBC, SBCS (register-shifted register)',
  brief      => 'Subtract with Carry (register-shifted register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SBFX => {
  title      => 'SBFX',
  brief      => 'Signed Bit Field Extract',
  docvars    => 'instr-class=general mnemonic=SBFX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SDIV => {
  title      => 'SDIV',
  brief      => 'Signed Divide',
  docvars    => 'instr-class=general mnemonic=SDIV',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SEL => {
  title      => 'SEL',
  brief      => 'Select Bytes',
  docvars    => 'instr-class=general mnemonic=SEL',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SETEND => {
  title      => 'SETEND',
  brief      => 'Set Endianness',
  docvars    => 'instr-class=general mnemonic=SETEND',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SETPAN => {
  title      => 'SETPAN',
  brief      => 'Set Privileged Access Never',
  docvars    => 'instr-class=general mnemonic=SETPAN',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SEV => {
  title      => 'SEV',
  brief      => 'Send Event',
  docvars    => 'instr-class=general mnemonic=SEV',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SEVL => {
  title      => 'SEVL',
  brief      => 'Send Event Local',
  docvars    => 'instr-class=general mnemonic=SEVL',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SHADD16 => {
  title      => 'SHADD16',
  brief      => 'Signed Halving Add 16',
  docvars    => 'instr-class=general mnemonic=SHADD16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SHADD8 => {
  title      => 'SHADD8',
  brief      => 'Signed Halving Add 8',
  docvars    => 'instr-class=general mnemonic=SHADD8',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SHASX => {
  title      => 'SHASX',
  brief      => 'Signed Halving Add and Subtract with Exchange',
  docvars    => 'instr-class=general mnemonic=SHASX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SHSAX => {
  title      => 'SHSAX',
  brief      => 'Signed Halving Subtract and Add with Exchange',
  docvars    => 'instr-class=general mnemonic=SHSAX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SHSUB16 => {
  title      => 'SHSUB16',
  brief      => 'Signed Halving Subtract 16',
  docvars    => 'instr-class=general mnemonic=SHSUB16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SHSUB8 => {
  title      => 'SHSUB8',
  brief      => 'Signed Halving Subtract 8',
  docvars    => 'instr-class=general mnemonic=SHSUB8',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMC => {
  title      => 'SMC',
  brief      => 'Secure Monitor Call',
  docvars    => 'instr-class=general mnemonic=SMC',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMLABB => {
  title      => 'SMLABB, SMLABT, SMLATB, SMLATT',
  brief      => 'Signed Multiply Accumulate (halfwords)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMLAD => {
  title      => 'SMLAD, SMLADX',
  brief      => 'Signed Multiply Accumulate Dual',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMLAL => {
  title      => 'SMLAL, SMLALS',
  brief      => 'Signed Multiply Accumulate Long',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMLALBB => {
  title      => 'SMLALBB, SMLALBT, SMLALTB, SMLALTT',
  brief      => 'Signed Multiply Accumulate Long (halfwords)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMLALD => {
  title      => 'SMLALD, SMLALDX',
  brief      => 'Signed Multiply Accumulate Long Dual',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMLAWB => {
  title      => 'SMLAWB, SMLAWT',
  brief      => 'Signed Multiply Accumulate (word by halfword)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMLSD => {
  title      => 'SMLSD, SMLSDX',
  brief      => 'Signed Multiply Subtract Dual',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMLSLD => {
  title      => 'SMLSLD, SMLSLDX',
  brief      => 'Signed Multiply Subtract Long Dual',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMMLA => {
  title      => 'SMMLA, SMMLAR',
  brief      => 'Signed Most Significant Word Multiply Accumulate',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMMLS => {
  title      => 'SMMLS, SMMLSR',
  brief      => 'Signed Most Significant Word Multiply Subtract',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMMUL => {
  title      => 'SMMUL, SMMULR',
  brief      => 'Signed Most Significant Word Multiply',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMUAD => {
  title      => 'SMUAD, SMUADX',
  brief      => 'Signed Dual Multiply Add',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMULBB => {
  title      => 'SMULBB, SMULBT, SMULTB, SMULTT',
  brief      => 'Signed Multiply (halfwords)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMULL => {
  title      => 'SMULL, SMULLS',
  brief      => 'Signed Multiply Long',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMULWB => {
  title      => 'SMULWB, SMULWT',
  brief      => 'Signed Multiply (word by halfword)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMUSD => {
  title      => 'SMUSD, SMUSDX',
  brief      => 'Signed Multiply Subtract Dual',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SRS => {
  title      => 'SRS, SRSDA, SRSDB, SRSIA, SRSIB',
  brief      => 'Store Return State',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SSAT => {
  title      => 'SSAT',
  brief      => 'Signed Saturate',
  docvars    => 'instr-class=general mnemonic=SSAT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SSAT16 => {
  title      => 'SSAT16',
  brief      => 'Signed Saturate 16',
  docvars    => 'instr-class=general mnemonic=SSAT16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SSAX => {
  title      => 'SSAX',
  brief      => 'Signed Subtract and Add with Exchange',
  docvars    => 'instr-class=general mnemonic=SSAX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SSBB => {
  title      => 'SSBB',
  brief      => 'Speculative Store Bypass Barrier',
  docvars    => 'instr-class=general mnemonic=SSBB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SSUB16 => {
  title      => 'SSUB16',
  brief      => 'Signed Subtract 16',
  docvars    => 'instr-class=general mnemonic=SSUB16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SSUB8 => {
  title      => 'SSUB8',
  brief      => 'Signed Subtract 8',
  docvars    => 'instr-class=general mnemonic=SSUB8',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STC => {
  title      => 'STC',
  brief      => 'Store data to System register',
  docvars    => 'instr-class=general mnemonic=STC',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STL => {
  title      => 'STL',
  brief      => 'Store-Release Word',
  docvars    => 'instr-class=general mnemonic=STL',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STLB => {
  title      => 'STLB',
  brief      => 'Store-Release Byte',
  docvars    => 'instr-class=general mnemonic=STLB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STLEX => {
  title      => 'STLEX',
  brief      => 'Store-Release Exclusive Word',
  docvars    => 'instr-class=general mnemonic=STLEX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STLEXB => {
  title      => 'STLEXB',
  brief      => 'Store-Release Exclusive Byte',
  docvars    => 'instr-class=general mnemonic=STLEXB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STLEXD => {
  title      => 'STLEXD',
  brief      => 'Store-Release Exclusive Doubleword',
  docvars    => 'instr-class=general mnemonic=STLEXD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STLEXH => {
  title      => 'STLEXH',
  brief      => 'Store-Release Exclusive Halfword',
  docvars    => 'instr-class=general mnemonic=STLEXH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STLH => {
  title      => 'STLH',
  brief      => 'Store-Release Halfword',
  docvars    => 'instr-class=general mnemonic=STLH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STM => {
  title      => 'STM, STMIA, STMEA',
  brief      => 'Store Multiple (Increment After, Empty Ascending)',
  docvars    => 'instr-class=general mnemonic=STM',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STM_u => {
  title      => 'STM (User registers)',
  brief      => 'Store Multiple (User registers)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STM',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STMDA => {
  title      => 'STMDA, STMED',
  brief      => 'Store Multiple Decrement After (Empty Descending)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STM',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STMDB => {
  title      => 'STMDB, STMFD',
  brief      => 'Store Multiple Decrement Before (Full Descending)',
  docvars    => 'instr-class=general mnemonic=STM',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STMIB => {
  title      => 'STMIB, STMFA',
  brief      => 'Store Multiple Increment Before (Full Ascending)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STM',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STR_i => {
  title      => 'STR (immediate)',
  brief      => 'Store Register (immediate)',
  docvars    => 'instr-class=general mnemonic=STR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STR_r => {
  title      => 'STR (register)',
  brief      => 'Store Register (register)',
  docvars    => 'instr-class=general mnemonic=STR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STRB_i => {
  title      => 'STRB (immediate)',
  brief      => 'Store Register Byte (immediate)',
  docvars    => 'instr-class=general mnemonic=STRB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STRB_r => {
  title      => 'STRB (register)',
  brief      => 'Store Register Byte (register)',
  docvars    => 'instr-class=general mnemonic=STRB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STRBT => {
  title      => 'STRBT',
  brief      => 'Store Register Byte Unprivileged',
  docvars    => 'instr-class=general mnemonic=STRBT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STRD_i => {
  title      => 'STRD (immediate)',
  brief      => 'Store Register Dual (immediate)',
  docvars    => 'instr-class=general mnemonic=STRD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STRD_r => {
  title      => 'STRD (register)',
  brief      => 'Store Register Dual (register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STRD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STREX => {
  title      => 'STREX',
  brief      => 'Store Register Exclusive',
  docvars    => 'instr-class=general mnemonic=STREX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STREXB => {
  title      => 'STREXB',
  brief      => 'Store Register Exclusive Byte',
  docvars    => 'instr-class=general mnemonic=STREXB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STREXD => {
  title      => 'STREXD',
  brief      => 'Store Register Exclusive Doubleword',
  docvars    => 'instr-class=general mnemonic=STREXD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STREXH => {
  title      => 'STREXH',
  brief      => 'Store Register Exclusive Halfword',
  docvars    => 'instr-class=general mnemonic=STREXH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STRH_i => {
  title      => 'STRH (immediate)',
  brief      => 'Store Register Halfword (immediate)',
  docvars    => 'instr-class=general mnemonic=STRH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STRH_r => {
  title      => 'STRH (register)',
  brief      => 'Store Register Halfword (register)',
  docvars    => 'instr-class=general mnemonic=STRH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STRHT => {
  title      => 'STRHT',
  brief      => 'Store Register Halfword Unprivileged',
  docvars    => 'instr-class=general mnemonic=STRHT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STRT => {
  title      => 'STRT',
  brief      => 'Store Register Unprivileged',
  docvars    => 'instr-class=general mnemonic=STRT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SUB_i => {
  title      => 'SUB, SUBS (immediate)',
  brief      => 'Subtract (immediate)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SUB_r => {
  title      => 'SUB, SUBS (register)',
  brief      => 'Subtract (register)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SUB_rr => {
  title      => 'SUB, SUBS (register-shifted register)',
  brief      => 'Subtract (register-shifted register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SUB_SP_i => {
  title      => 'SUB, SUBS (SP minus immediate)',
  brief      => 'Subtract from SP (immediate)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SUB_SP_r => {
  title      => 'SUB, SUBS (SP minus register)',
  brief      => 'Subtract from SP (register)',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SVC => {
  title      => 'SVC',
  brief      => 'Supervisor Call',
  docvars    => 'instr-class=general mnemonic=SVC',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SXTAB => {
  title      => 'SXTAB',
  brief      => 'Signed Extend and Add Byte',
  docvars    => 'instr-class=general mnemonic=SXTAB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SXTAB16 => {
  title      => 'SXTAB16',
  brief      => 'Signed Extend and Add Byte 16',
  docvars    => 'instr-class=general mnemonic=SXTAB16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SXTAH => {
  title      => 'SXTAH',
  brief      => 'Signed Extend and Add Halfword',
  docvars    => 'instr-class=general mnemonic=SXTAH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SXTB => {
  title      => 'SXTB',
  brief      => 'Signed Extend Byte',
  docvars    => 'instr-class=general mnemonic=SXTB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SXTB16 => {
  title      => 'SXTB16',
  brief      => 'Signed Extend Byte 16',
  docvars    => 'instr-class=general mnemonic=SXTB16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SXTH => {
  title      => 'SXTH',
  brief      => 'Signed Extend Halfword',
  docvars    => 'instr-class=general mnemonic=SXTH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE TBB => {
  title      => 'TBB, TBH',
  brief      => 'Table Branch Byte or Halfword',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE TEQ_i => {
  title      => 'TEQ (immediate)',
  brief      => 'Test Equivalence (immediate)',
  docvars    => 'instr-class=general mnemonic=TEQ',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE TEQ_r => {
  title      => 'TEQ (register)',
  brief      => 'Test Equivalence (register)',
  docvars    => 'instr-class=general mnemonic=TEQ',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE TEQ_rr => {
  title      => 'TEQ (register-shifted register)',
  brief      => 'Test Equivalence (register-shifted register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TEQ',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE TSB => {
  title      => 'TSB CSYNC',
  brief      => 'Trace Synchronization Barrier',
  docvars    => 'instr-class=general mnemonic=TSB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE TST_i => {
  title      => 'TST (immediate)',
  brief      => 'Test (immediate)',
  docvars    => 'instr-class=general mnemonic=TST',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE TST_r => {
  title      => 'TST (register)',
  brief      => 'Test (register)',
  docvars    => 'instr-class=general mnemonic=TST',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE TST_rr => {
  title      => 'TST (register-shifted register)',
  brief      => 'Test (register-shifted register)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TST',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UADD16 => {
  title      => 'UADD16',
  brief      => 'Unsigned Add 16',
  docvars    => 'instr-class=general mnemonic=UADD16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UADD8 => {
  title      => 'UADD8',
  brief      => 'Unsigned Add 8',
  docvars    => 'instr-class=general mnemonic=UADD8',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UASX => {
  title      => 'UASX',
  brief      => 'Unsigned Add and Subtract with Exchange',
  docvars    => 'instr-class=general mnemonic=UASX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UBFX => {
  title      => 'UBFX',
  brief      => 'Unsigned Bit Field Extract',
  docvars    => 'instr-class=general mnemonic=UBFX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UDF => {
  title      => 'UDF',
  brief      => 'Permanently Undefined',
  docvars    => 'instr-class=general mnemonic=UDF',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UDIV => {
  title      => 'UDIV',
  brief      => 'Unsigned Divide',
  docvars    => 'instr-class=general mnemonic=UDIV',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UHADD16 => {
  title      => 'UHADD16',
  brief      => 'Unsigned Halving Add 16',
  docvars    => 'instr-class=general mnemonic=UHADD16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UHADD8 => {
  title      => 'UHADD8',
  brief      => 'Unsigned Halving Add 8',
  docvars    => 'instr-class=general mnemonic=UHADD8',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UHASX => {
  title      => 'UHASX',
  brief      => 'Unsigned Halving Add and Subtract with Exchange',
  docvars    => 'instr-class=general mnemonic=UHASX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UHSAX => {
  title      => 'UHSAX',
  brief      => 'Unsigned Halving Subtract and Add with Exchange',
  docvars    => 'instr-class=general mnemonic=UHSAX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UHSUB16 => {
  title      => 'UHSUB16',
  brief      => 'Unsigned Halving Subtract 16',
  docvars    => 'instr-class=general mnemonic=UHSUB16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UHSUB8 => {
  title      => 'UHSUB8',
  brief      => 'Unsigned Halving Subtract 8',
  docvars    => 'instr-class=general mnemonic=UHSUB8',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UMAAL => {
  title      => 'UMAAL',
  brief      => 'Unsigned Multiply Accumulate Accumulate Long',
  docvars    => 'instr-class=general mnemonic=UMAAL',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UMLAL => {
  title      => 'UMLAL, UMLALS',
  brief      => 'Unsigned Multiply Accumulate Long',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UMULL => {
  title      => 'UMULL, UMULLS',
  brief      => 'Unsigned Multiply Long',
  docvars    => 'instr-class=general',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UQADD16 => {
  title      => 'UQADD16',
  brief      => 'Unsigned Saturating Add 16',
  docvars    => 'instr-class=general mnemonic=UQADD16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UQADD8 => {
  title      => 'UQADD8',
  brief      => 'Unsigned Saturating Add 8',
  docvars    => 'instr-class=general mnemonic=UQADD8',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UQASX => {
  title      => 'UQASX',
  brief      => 'Unsigned Saturating Add and Subtract with Exchange',
  docvars    => 'instr-class=general mnemonic=UQASX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UQSAX => {
  title      => 'UQSAX',
  brief      => 'Unsigned Saturating Subtract and Add with Exchange',
  docvars    => 'instr-class=general mnemonic=UQSAX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UQSUB16 => {
  title      => 'UQSUB16',
  brief      => 'Unsigned Saturating Subtract 16',
  docvars    => 'instr-class=general mnemonic=UQSUB16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UQSUB8 => {
  title      => 'UQSUB8',
  brief      => 'Unsigned Saturating Subtract 8',
  docvars    => 'instr-class=general mnemonic=UQSUB8',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE USAD8 => {
  title      => 'USAD8',
  brief      => 'Unsigned Sum of Absolute Differences',
  docvars    => 'instr-class=general mnemonic=USAD8',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE USADA8 => {
  title      => 'USADA8',
  brief      => 'Unsigned Sum of Absolute Differences and Accumulate',
  docvars    => 'instr-class=general mnemonic=USADA8',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE USAT => {
  title      => 'USAT',
  brief      => 'Unsigned Saturate',
  docvars    => 'instr-class=general mnemonic=USAT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE USAT16 => {
  title      => 'USAT16',
  brief      => 'Unsigned Saturate 16',
  docvars    => 'instr-class=general mnemonic=USAT16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE USAX => {
  title      => 'USAX',
  brief      => 'Unsigned Subtract and Add with Exchange',
  docvars    => 'instr-class=general mnemonic=USAX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE USUB16 => {
  title      => 'USUB16',
  brief      => 'Unsigned Subtract 16',
  docvars    => 'instr-class=general mnemonic=USUB16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE USUB8 => {
  title      => 'USUB8',
  brief      => 'Unsigned Subtract 8',
  docvars    => 'instr-class=general mnemonic=USUB8',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UXTAB => {
  title      => 'UXTAB',
  brief      => 'Unsigned Extend and Add Byte',
  docvars    => 'instr-class=general mnemonic=UXTAB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UXTAB16 => {
  title      => 'UXTAB16',
  brief      => 'Unsigned Extend and Add Byte 16',
  docvars    => 'instr-class=general mnemonic=UXTAB16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UXTAH => {
  title      => 'UXTAH',
  brief      => 'Unsigned Extend and Add Halfword',
  docvars    => 'instr-class=general mnemonic=UXTAH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UXTB => {
  title      => 'UXTB',
  brief      => 'Unsigned Extend Byte',
  docvars    => 'instr-class=general mnemonic=UXTB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UXTB16 => {
  title      => 'UXTB16',
  brief      => 'Unsigned Extend Byte 16',
  docvars    => 'instr-class=general mnemonic=UXTB16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UXTH => {
  title      => 'UXTH',
  brief      => 'Unsigned Extend Halfword',
  docvars    => 'instr-class=general mnemonic=UXTH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE WFE => {
  title      => 'WFE',
  brief      => 'Wait For Event',
  docvars    => 'instr-class=general mnemonic=WFE',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE WFI => {
  title      => 'WFI',
  brief      => 'Wait For Interrupt',
  docvars    => 'instr-class=general mnemonic=WFI',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE YIELD => {
  title      => 'YIELD',
  brief      => 'Yield hint',
  docvars    => 'instr-class=general mnemonic=YIELD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADD_ADR => {
  title      => 'ADD (immediate, to PC)',
  brief      => 'Add to PC',
  docvars    => 'alias_mnemonic=ADD instr-class=general mnemonic=ADR',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE ASR_MOV_r => {
  title      => 'ASR (immediate)',
  brief      => 'Arithmetic Shift Right (immediate)',
  docvars    => 'alias_mnemonic=ASR instr-class=general mnemonic=MOV',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE ASR_MOV_rr => {
  title      => 'ASR (register)',
  brief      => 'Arithmetic Shift Right (register)',
  docvars    => 'alias_mnemonic=ASR instr-class=general mnemonic=MOV',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE ASRS_MOV_r => {
  title      => 'ASRS (immediate)',
  brief      => 'Arithmetic Shift Right, setting flags (immediate)',
  docvars    => 'alias_mnemonic=ASRS instr-class=general',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE ASRS_MOV_rr => {
  title      => 'ASRS (register)',
  brief      => 'Arithmetic Shift Right, setting flags (register)',
  docvars    => 'alias_mnemonic=ASRS instr-class=general',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE LSL_MOV_r => {
  title      => 'LSL (immediate)',
  brief      => 'Logical Shift Left (immediate)',
  docvars    => 'alias_mnemonic=LSL instr-class=general mnemonic=MOV',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE LSL_MOV_rr => {
  title      => 'LSL (register)',
  brief      => 'Logical Shift Left (register)',
  docvars    => 'alias_mnemonic=LSL instr-class=general mnemonic=MOV',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE LSLS_MOV_r => {
  title      => 'LSLS (immediate)',
  brief      => 'Logical Shift Left, setting flags (immediate)',
  docvars    => 'alias_mnemonic=LSLS instr-class=general',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE LSLS_MOV_rr => {
  title      => 'LSLS (register)',
  brief      => 'Logical Shift Left, setting flags (register)',
  docvars    => 'alias_mnemonic=LSLS instr-class=general',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE LSR_MOV_r => {
  title      => 'LSR (immediate)',
  brief      => 'Logical Shift Right (immediate)',
  docvars    => 'alias_mnemonic=LSR instr-class=general mnemonic=MOV',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE LSR_MOV_rr => {
  title      => 'LSR (register)',
  brief      => 'Logical Shift Right (register)',
  docvars    => 'alias_mnemonic=LSR instr-class=general mnemonic=MOV',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE LSRS_MOV_r => {
  title      => 'LSRS (immediate)',
  brief      => 'Logical Shift Right, setting flags (immediate)',
  docvars    => 'alias_mnemonic=LSRS instr-class=general',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE LSRS_MOV_rr => {
  title      => 'LSRS (register)',
  brief      => 'Logical Shift Right, setting flags (register)',
  docvars    => 'alias_mnemonic=LSRS instr-class=general',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE POP_LDM => {
  title      => 'POP (multiple registers)',
  brief      => 'Pop Multiple Registers from Stack',
  docvars    => 'alias_mnemonic=POP instr-class=general mnemonic=LDM',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE POP_LDR_i => {
  title      => 'POP (single register)',
  brief      => 'Pop Single Register from Stack',
  docvars    => 'address-form=post-indexed alias_mnemonic=POP instr-class=general mnemonic=LDR',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE PUSH_STMDB => {
  title      => 'PUSH (multiple registers)',
  brief      => 'Push multiple registers to Stack',
  docvars    => 'alias_mnemonic=PUSH instr-class=general mnemonic=STM',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE PUSH_STR_i => {
  title      => 'PUSH (single register)',
  brief      => 'Push Single Register to Stack',
  docvars    => 'address-form=pre-indexed alias_mnemonic=PUSH instr-class=general mnemonic=STR',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE ROR_MOV_r => {
  title      => 'ROR (immediate)',
  brief      => 'Rotate Right (immediate)',
  docvars    => 'alias_mnemonic=ROR instr-class=general mnemonic=MOV mnemonic-shift-type=MOV-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE ROR_MOV_rr => {
  title      => 'ROR (register)',
  brief      => 'Rotate Right (register)',
  docvars    => 'alias_mnemonic=ROR instr-class=general mnemonic=MOV',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE RORS_MOV_r => {
  title      => 'RORS (immediate)',
  brief      => 'Rotate Right, setting flags (immediate)',
  docvars    => 'alias_mnemonic=RORS instr-class=general mnemonic=MOVS mnemonic-shift-type=MOVS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE RORS_MOV_rr => {
  title      => 'RORS (register)',
  brief      => 'Rotate Right, setting flags (register)',
  docvars    => 'alias_mnemonic=RORS instr-class=general',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE RRX_MOV_r => {
  title      => 'RRX',
  brief      => 'Rotate Right with Extend',
  docvars    => 'alias_mnemonic=RRX instr-class=general mnemonic=MOV mnemonic-shift-type=MOV-rrx shift-type=rrx',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE RRXS_MOV_r => {
  title      => 'RRXS',
  brief      => 'Rotate Right with Extend, setting flags',
  docvars    => 'alias_mnemonic=RRXS instr-class=general mnemonic=MOVS mnemonic-shift-type=MOVS-rrx shift-type=rrx',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE SUB_ADR => {
  title      => 'SUB (immediate, from PC)',
  brief      => 'Subtract from PC',
  docvars    => 'alias_mnemonic=SUB instr-class=general mnemonic=ADR',
  metadata   => 'alias=1 alphaindex=BASIC',
};


1;

