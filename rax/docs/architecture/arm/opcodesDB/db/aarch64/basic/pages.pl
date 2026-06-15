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

PAGE ADC => {
  title      => 'ADC',
  brief      => 'Add with Carry',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=ADC',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADCS => {
  title      => 'ADCS',
  brief      => 'Add with Carry, setting flags',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=ADCS',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADD_addsub_ext => {
  title      => 'ADD (extended register)',
  brief      => 'Add (extended register)',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=ADD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADD_addsub_imm => {
  title      => 'ADD (immediate)',
  brief      => 'Add (immediate)',
  docvars    => 'cond-setting=no-s immediate-type=imm12u instr-class=general isa=A64 mnemonic=ADD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADD_addsub_shift => {
  title      => 'ADD (shifted register)',
  brief      => 'Add (shifted register)',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=ADD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADDG => {
  title      => 'ADDG',
  brief      => 'Add with Tag',
  docvars    => 'instr-class=general isa=A64 mnemonic=ADDG',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADDS_addsub_ext => {
  title      => 'ADDS (extended register)',
  brief      => 'Add (extended register), setting flags',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADDS_addsub_imm => {
  title      => 'ADDS (immediate)',
  brief      => 'Add (immediate), setting flags',
  docvars    => 'cond-setting=S immediate-type=imm12u instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADDS_addsub_shift => {
  title      => 'ADDS (shifted register)',
  brief      => 'Add (shifted register), setting flags',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADR => {
  title      => 'ADR',
  brief      => 'Form PC-relative address',
  docvars    => 'address-form=literal instr-class=general isa=A64 mnemonic=ADR offset-type=off19s',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ADRP => {
  title      => 'ADRP',
  brief      => 'Form PC-relative address to 4KB page',
  docvars    => 'address-form=literal instr-class=general isa=A64 mnemonic=ADRP offset-type=off19s',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE AND_log_imm => {
  title      => 'AND (immediate)',
  brief      => 'Bitwise AND (immediate)',
  docvars    => 'cond-setting=no-s immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=AND',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE AND_log_shift => {
  title      => 'AND (shifted register)',
  brief      => 'Bitwise AND (shifted register)',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=AND reguse=shifted-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ANDS_log_imm => {
  title      => 'ANDS (immediate)',
  brief      => 'Bitwise AND (immediate), setting flags',
  docvars    => 'cond-setting=S immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=ANDS',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ANDS_log_shift => {
  title      => 'ANDS (shifted register)',
  brief      => 'Bitwise AND (shifted register), setting flags',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=ANDS reguse=shifted-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ASRV => {
  title      => 'ASRV',
  brief      => 'Arithmetic Shift Right Variable',
  docvars    => 'instr-class=general isa=A64 mnemonic=ASRV',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE AUTDA => {
  title      => 'AUTDA, AUTDZA',
  brief      => 'Authenticate Data address, using key A',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE AUTDB => {
  title      => 'AUTDB, AUTDZB',
  brief      => 'Authenticate Data address, using key B',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE AUTIA => {
  title      => 'AUTIA, AUTIA1716, AUTIASP, AUTIAZ, AUTIZA',
  brief      => 'Authenticate Instruction address, using key A',
  docvars    => 'isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE AUTIB => {
  title      => 'AUTIB, AUTIB1716, AUTIBSP, AUTIBZ, AUTIZB',
  brief      => 'Authenticate Instruction address, using key B',
  docvars    => 'isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE AXFLAG => {
  title      => 'AXFLAG',
  brief      => 'Convert floating-point condition flags from Arm to external format',
  docvars    => 'instr-class=system isa=A64 mnemonic=AXFLAG',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE B_cond => {
  title      => 'B.cond',
  brief      => 'Branch conditionally',
  docvars    => 'branch-offset=br19 compare-with=cmp-cond instr-class=general isa=A64 mnemonic=B',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE B_uncond => {
  title      => 'B',
  brief      => 'Branch',
  docvars    => 'branch-offset=br26 instr-class=general isa=A64 mnemonic=B',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BFM => {
  title      => 'BFM',
  brief      => 'Bitfield Move',
  docvars    => 'bitfield-fill=nofill instr-class=general isa=A64 mnemonic=BFM',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BIC_log_shift => {
  title      => 'BIC (shifted register)',
  brief      => 'Bitwise Bit Clear (shifted register)',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=BIC reguse=shifted-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BICS => {
  title      => 'BICS (shifted register)',
  brief      => 'Bitwise Bit Clear (shifted register), setting flags',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=BICS reguse=shifted-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BL => {
  title      => 'BL',
  brief      => 'Branch with Link',
  docvars    => 'branch-offset=br26 instr-class=general isa=A64 mnemonic=BL',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BLR => {
  title      => 'BLR',
  brief      => 'Branch with Link to Register',
  docvars    => 'instr-class=general isa=A64 mnemonic=BLR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BLRA => {
  title      => 'BLRAA, BLRAAZ, BLRAB, BLRABZ',
  brief      => 'Branch with Link to Register, with pointer authentication',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BR => {
  title      => 'BR',
  brief      => 'Branch to Register',
  docvars    => 'instr-class=general isa=A64 mnemonic=BR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BRA => {
  title      => 'BRAA, BRAAZ, BRAB, BRABZ',
  brief      => 'Branch to Register, with pointer authentication',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BRK => {
  title      => 'BRK',
  brief      => 'Breakpoint instruction',
  docvars    => 'instr-class=system isa=A64 mnemonic=BRK',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE BTI => {
  title      => 'BTI',
  brief      => 'Branch Target Identification',
  docvars    => 'instr-class=system isa=A64 mnemonic=BTI',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CAS => {
  title      => 'CAS, CASA, CASAL, CASL',
  brief      => 'Compare and Swap word or doubleword in memory',
  docvars    => 'address-form=base-register instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CASB => {
  title      => 'CASB, CASAB, CASALB, CASLB',
  brief      => 'Compare and Swap byte in memory',
  docvars    => 'address-form=base-register instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CASH => {
  title      => 'CASH, CASAH, CASALH, CASLH',
  brief      => 'Compare and Swap halfword in memory',
  docvars    => 'address-form=base-register instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CASP => {
  title      => 'CASP, CASPA, CASPAL, CASPL',
  brief      => 'Compare and Swap Pair of words or doublewords in memory',
  docvars    => 'address-form=base-register instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CBNZ => {
  title      => 'CBNZ',
  brief      => 'Compare and Branch on Nonzero',
  docvars    => 'branch-offset=br19 compare-with=cmp-nonzero instr-class=general isa=A64 mnemonic=CBNZ',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CBZ => {
  title      => 'CBZ',
  brief      => 'Compare and Branch on Zero',
  docvars    => 'branch-offset=br19 compare-with=cmp-zero instr-class=general isa=A64 mnemonic=CBZ',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CCMN_imm => {
  title      => 'CCMN (immediate)',
  brief      => 'Conditional Compare Negative (immediate)',
  docvars    => 'immediate-type=imm5u instr-class=general isa=A64 mnemonic=CCMN',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CCMN_reg => {
  title      => 'CCMN (register)',
  brief      => 'Conditional Compare Negative (register)',
  docvars    => 'instr-class=general isa=A64 mnemonic=CCMN',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CCMP_imm => {
  title      => 'CCMP (immediate)',
  brief      => 'Conditional Compare (immediate)',
  docvars    => 'immediate-type=imm5u instr-class=general isa=A64 mnemonic=CCMP',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CCMP_reg => {
  title      => 'CCMP (register)',
  brief      => 'Conditional Compare (register)',
  docvars    => 'instr-class=general isa=A64 mnemonic=CCMP',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CFINV => {
  title      => 'CFINV',
  brief      => 'Invert Carry Flag',
  docvars    => 'instr-class=system isa=A64 mnemonic=CFINV',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CLREX => {
  title      => 'CLREX',
  brief      => 'Clear Exclusive',
  docvars    => 'instr-class=system isa=A64 mnemonic=CLREX',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CLS_int => {
  title      => 'CLS',
  brief      => 'Count Leading Sign bits',
  docvars    => 'instr-class=general isa=A64 mnemonic=CLS',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CLZ_int => {
  title      => 'CLZ',
  brief      => 'Count Leading Zeros',
  docvars    => 'instr-class=general isa=A64 mnemonic=CLZ',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CRC32 => {
  title      => 'CRC32B, CRC32H, CRC32W, CRC32X',
  brief      => 'CRC32 checksum',
  docvars    => 'feature=crc instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CRC32C => {
  title      => 'CRC32CB, CRC32CH, CRC32CW, CRC32CX',
  brief      => 'CRC32C checksum',
  docvars    => 'feature=crc instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CSDB => {
  title      => 'CSDB',
  brief      => 'Consumption of Speculative Data Barrier',
  docvars    => 'instr-class=system isa=A64 mnemonic=CSDB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CSEL => {
  title      => 'CSEL',
  brief      => 'Conditional Select',
  docvars    => 'instr-class=general isa=A64 mnemonic=CSEL',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CSINC => {
  title      => 'CSINC',
  brief      => 'Conditional Select Increment',
  docvars    => 'instr-class=general isa=A64 mnemonic=CSINC',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CSINV => {
  title      => 'CSINV',
  brief      => 'Conditional Select Invert',
  docvars    => 'instr-class=general isa=A64 mnemonic=CSINV',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE CSNEG => {
  title      => 'CSNEG',
  brief      => 'Conditional Select Negation',
  docvars    => 'instr-class=general isa=A64 mnemonic=CSNEG',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE DCPS1 => {
  title      => 'DCPS1',
  brief      => 'Debug Change PE State to EL1.',
  docvars    => 'instr-class=system isa=A64 mnemonic=DCPS1',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE DCPS2 => {
  title      => 'DCPS2',
  brief      => 'Debug Change PE State to EL2.',
  docvars    => 'instr-class=system isa=A64 mnemonic=DCPS2',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE DCPS3 => {
  title      => 'DCPS3',
  brief      => 'Debug Change PE State to EL3',
  docvars    => 'instr-class=system isa=A64 mnemonic=DCPS3',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE DGH => {
  title      => 'DGH',
  brief      => 'Data Gathering Hint',
  docvars    => 'instr-class=system isa=A64 mnemonic=DGH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE DMB => {
  title      => 'DMB',
  brief      => 'Data Memory Barrier',
  docvars    => 'instr-class=system isa=A64 mnemonic=DMB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE DRPS => {
  title      => 'DRPS',
  brief      => 'Debug restore process state',
  docvars    => 'instr-class=system isa=A64 mnemonic=DRPS',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE DSB => {
  title      => 'DSB',
  brief      => 'Data Synchronization Barrier',
  docvars    => 'instr-class=system isa=A64 mnemonic=DSB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE EON => {
  title      => 'EON (shifted register)',
  brief      => 'Bitwise Exclusive OR NOT (shifted register)',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=EON reguse=shifted-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE EOR_log_imm => {
  title      => 'EOR (immediate)',
  brief      => 'Bitwise Exclusive OR (immediate)',
  docvars    => 'cond-setting=no-s immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=EOR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE EOR_log_shift => {
  title      => 'EOR (shifted register)',
  brief      => 'Bitwise Exclusive OR (shifted register)',
  docvars    => 'instr-class=general isa=A64 mnemonic=EOR reguse=shifted-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ERET => {
  title      => 'ERET',
  brief      => 'Exception Return',
  docvars    => 'instr-class=system isa=A64 mnemonic=ERET',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ERETA => {
  title      => 'ERETAA, ERETAB',
  brief      => 'Exception Return, with pointer authentication',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ESB => {
  title      => 'ESB',
  brief      => 'Error Synchronization Barrier',
  docvars    => 'instr-class=system isa=A64 mnemonic=ESB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE EXTR => {
  title      => 'EXTR',
  brief      => 'Extract register',
  docvars    => 'instr-class=general isa=A64 mnemonic=EXTR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE GMI => {
  title      => 'GMI',
  brief      => 'Tag Mask Insert',
  docvars    => 'instr-class=general isa=A64 mnemonic=GMI',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE HINT => {
  title      => 'HINT',
  brief      => 'Hint instruction',
  docvars    => 'instr-class=system isa=A64 mnemonic=HINT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE HLT => {
  title      => 'HLT',
  brief      => 'Halt instruction',
  docvars    => 'instr-class=system isa=A64 mnemonic=HLT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE HVC => {
  title      => 'HVC',
  brief      => 'Hypervisor Call',
  docvars    => 'instr-class=system isa=A64 mnemonic=HVC',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE IRG => {
  title      => 'IRG',
  brief      => 'Insert Random Tag',
  docvars    => 'instr-class=general isa=A64 mnemonic=IRG',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ISB => {
  title      => 'ISB',
  brief      => 'Instruction Synchronization Barrier',
  docvars    => 'instr-class=system isa=A64 mnemonic=ISB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDADD => {
  title      => 'LDADD, LDADDA, LDADDAL, LDADDL',
  brief      => 'Atomic add on word or doubleword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDADDB => {
  title      => 'LDADDB, LDADDAB, LDADDALB, LDADDLB',
  brief      => 'Atomic add on byte in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDADDH => {
  title      => 'LDADDH, LDADDAH, LDADDALH, LDADDLH',
  brief      => 'Atomic add on halfword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDAPR => {
  title      => 'LDAPR',
  brief      => 'Load-Acquire RCpc Register',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDAPR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDAPRB => {
  title      => 'LDAPRB',
  brief      => 'Load-Acquire RCpc Register Byte',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDAPRB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDAPRH => {
  title      => 'LDAPRH',
  brief      => 'Load-Acquire RCpc Register Halfword',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDAPRH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDAPUR_gen => {
  title      => 'LDAPUR',
  brief      => 'Load-Acquire RCpc Register (unscaled)',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDAPUR offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDAPURB => {
  title      => 'LDAPURB',
  brief      => 'Load-Acquire RCpc Register Byte (unscaled)',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDAPURB offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE LDAPURH => {
  title      => 'LDAPURH',
  brief      => 'Load-Acquire RCpc Register Halfword (unscaled)',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDAPURH offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE LDAPURSB => {
  title      => 'LDAPURSB',
  brief      => 'Load-Acquire RCpc Register Signed Byte (unscaled)',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDAPURSB offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDAPURSH => {
  title      => 'LDAPURSH',
  brief      => 'Load-Acquire RCpc Register Signed Halfword (unscaled)',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDAPURSH offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDAPURSW => {
  title      => 'LDAPURSW',
  brief      => 'Load-Acquire RCpc Register Signed Word (unscaled)',
  docvars    => 'address-form=base-plus-offset datatype=64 instr-class=general isa=A64 mnemonic=LDAPURSW offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC datasize=64',
};

PAGE LDAR => {
  title      => 'LDAR',
  brief      => 'Load-Acquire Register',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=LDAR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDARB => {
  title      => 'LDARB',
  brief      => 'Load-Acquire Register Byte',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDARB-32-reg instr-class=general isa=A64 mnemonic=LDARB reg-type=32-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDARH => {
  title      => 'LDARH',
  brief      => 'Load-Acquire Register Halfword',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDARH-32-reg instr-class=general isa=A64 mnemonic=LDARH reg-type=32-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDAXP => {
  title      => 'LDAXP',
  brief      => 'Load-Acquire Exclusive Pair of Registers',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=LDAXP',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDAXR => {
  title      => 'LDAXR',
  brief      => 'Load-Acquire Exclusive Register',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=LDAXR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDAXRB => {
  title      => 'LDAXRB',
  brief      => 'Load-Acquire Exclusive Register Byte',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDAXRB-32-reg instr-class=general isa=A64 mnemonic=LDAXRB reg-type=32-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDAXRH => {
  title      => 'LDAXRH',
  brief      => 'Load-Acquire Exclusive Register Halfword',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDAXRH-32-reg instr-class=general isa=A64 mnemonic=LDAXRH reg-type=32-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDCLR => {
  title      => 'LDCLR, LDCLRA, LDCLRAL, LDCLRL',
  brief      => 'Atomic bit clear on word or doubleword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDCLRB => {
  title      => 'LDCLRB, LDCLRAB, LDCLRALB, LDCLRLB',
  brief      => 'Atomic bit clear on byte in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDCLRH => {
  title      => 'LDCLRH, LDCLRAH, LDCLRALH, LDCLRLH',
  brief      => 'Atomic bit clear on halfword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDEOR => {
  title      => 'LDEOR, LDEORA, LDEORAL, LDEORL',
  brief      => 'Atomic exclusive OR on word or doubleword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDEORB => {
  title      => 'LDEORB, LDEORAB, LDEORALB, LDEORLB',
  brief      => 'Atomic exclusive OR on byte in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDEORH => {
  title      => 'LDEORH, LDEORAH, LDEORALH, LDEORLH',
  brief      => 'Atomic exclusive OR on halfword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDG => {
  title      => 'LDG',
  brief      => 'Load Allocation Tag',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDG',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDGM => {
  title      => 'LDGM',
  brief      => 'Load Tag Multiple',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDGM',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDLAR => {
  title      => 'LDLAR',
  brief      => 'Load LOAcquire Register',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=LDLAR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDLARB => {
  title      => 'LDLARB',
  brief      => 'Load LOAcquire Register Byte',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDLARB-32-reg instr-class=general isa=A64 mnemonic=LDLARB reg-type=32-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDLARH => {
  title      => 'LDLARH',
  brief      => 'Load LOAcquire Register Halfword',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDLARH-32-reg instr-class=general isa=A64 mnemonic=LDLARH reg-type=32-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDNP_gen => {
  title      => 'LDNP',
  brief      => 'Load Pair of Registers, with non-temporal hint',
  docvars    => 'address-form=signed-scaled-offset instr-class=general isa=A64 mnemonic=LDNP offset-type=off7s_s',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDP_gen => {
  title      => 'LDP',
  brief      => 'Load Pair of Registers',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDP offset-type=off7s_s',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDPSW => {
  title      => 'LDPSW',
  brief      => 'Load Pair of Registers Signed Word',
  docvars    => 'atomic-ops=LDPSW-pair-64 instr-class=general isa=A64 mnemonic=LDPSW offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDR_imm_gen => {
  title      => 'LDR (immediate)',
  brief      => 'Load Register (immediate)',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDR_lit_gen => {
  title      => 'LDR (literal)',
  brief      => 'Load Register (literal)',
  docvars    => 'address-form=literal instr-class=general isa=A64 mnemonic=LDR offset-type=off19s',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDR_reg_gen => {
  title      => 'LDR (register)',
  brief      => 'Load Register (register)',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDR offset-type=off-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRA => {
  title      => 'LDRAA, LDRAB',
  brief      => 'Load Register, with pointer authentication',
  docvars    => 'address-form=base-plus-offset isa=A64 offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRB_imm => {
  title      => 'LDRB (immediate)',
  brief      => 'Load Register Byte (immediate)',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=LDRB',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE LDRB_reg => {
  title      => 'LDRB (register)',
  brief      => 'Load Register Byte (register)',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=LDRB offset-type=off-reg',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE LDRH_imm => {
  title      => 'LDRH (immediate)',
  brief      => 'Load Register Halfword (immediate)',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=LDRH',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE LDRH_reg => {
  title      => 'LDRH (register)',
  brief      => 'Load Register Halfword (register)',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=LDRH offset-type=off-reg',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE LDRSB_imm => {
  title      => 'LDRSB (immediate)',
  brief      => 'Load Register Signed Byte (immediate)',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDRSB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRSB_reg => {
  title      => 'LDRSB (register)',
  brief      => 'Load Register Signed Byte (register)',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDRSB offset-type=off-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRSH_imm => {
  title      => 'LDRSH (immediate)',
  brief      => 'Load Register Signed Halfword (immediate)',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDRSH',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRSH_reg => {
  title      => 'LDRSH (register)',
  brief      => 'Load Register Signed Halfword (register)',
  docvars    => 'instr-class=general isa=A64 mnemonic=LDRSH offset-type=off-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRSW_imm => {
  title      => 'LDRSW (immediate)',
  brief      => 'Load Register Signed Word (immediate)',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=LDRSW',
  metadata   => 'alias=0 alphaindex=BASIC datasize=64',
};

PAGE LDRSW_lit => {
  title      => 'LDRSW (literal)',
  brief      => 'Load Register Signed Word (literal)',
  docvars    => 'address-form=literal address-form-reg-type=literal-64-reg atomic-ops=LDRSW-64-reg instr-class=general isa=A64 mnemonic=LDRSW offset-type=off19s reg-type=64-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDRSW_reg => {
  title      => 'LDRSW (register)',
  brief      => 'Load Register Signed Word (register)',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=LDRSW offset-type=off-reg',
  metadata   => 'alias=0 alphaindex=BASIC datasize=64',
};

PAGE LDSET => {
  title      => 'LDSET, LDSETA, LDSETAL, LDSETL',
  brief      => 'Atomic bit set on word or doubleword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDSETB => {
  title      => 'LDSETB, LDSETAB, LDSETALB, LDSETLB',
  brief      => 'Atomic bit set on byte in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDSETH => {
  title      => 'LDSETH, LDSETAH, LDSETALH, LDSETLH',
  brief      => 'Atomic bit set on halfword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDSMAX => {
  title      => 'LDSMAX, LDSMAXA, LDSMAXAL, LDSMAXL',
  brief      => 'Atomic signed maximum on word or doubleword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDSMAXB => {
  title      => 'LDSMAXB, LDSMAXAB, LDSMAXALB, LDSMAXLB',
  brief      => 'Atomic signed maximum on byte in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDSMAXH => {
  title      => 'LDSMAXH, LDSMAXAH, LDSMAXALH, LDSMAXLH',
  brief      => 'Atomic signed maximum on halfword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDSMIN => {
  title      => 'LDSMIN, LDSMINA, LDSMINAL, LDSMINL',
  brief      => 'Atomic signed minimum on word or doubleword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDSMINB => {
  title      => 'LDSMINB, LDSMINAB, LDSMINALB, LDSMINLB',
  brief      => 'Atomic signed minimum on byte in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDSMINH => {
  title      => 'LDSMINH, LDSMINAH, LDSMINALH, LDSMINLH',
  brief      => 'Atomic signed minimum on halfword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDTR => {
  title      => 'LDTR',
  brief      => 'Load Register (unprivileged)',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDTR offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDTRB => {
  title      => 'LDTRB',
  brief      => 'Load Register Byte (unprivileged)',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDTRB offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE LDTRH => {
  title      => 'LDTRH',
  brief      => 'Load Register Halfword (unprivileged)',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDTRH offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE LDTRSB => {
  title      => 'LDTRSB',
  brief      => 'Load Register Signed Byte (unprivileged)',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDTRSB offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDTRSH => {
  title      => 'LDTRSH',
  brief      => 'Load Register Signed Halfword (unprivileged)',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDTRSH offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDTRSW => {
  title      => 'LDTRSW',
  brief      => 'Load Register Signed Word (unprivileged)',
  docvars    => 'address-form=base-plus-offset datatype=64 instr-class=general isa=A64 mnemonic=LDTRSW offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC datasize=64',
};

PAGE LDUMAX => {
  title      => 'LDUMAX, LDUMAXA, LDUMAXAL, LDUMAXL',
  brief      => 'Atomic unsigned maximum on word or doubleword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDUMAXB => {
  title      => 'LDUMAXB, LDUMAXAB, LDUMAXALB, LDUMAXLB',
  brief      => 'Atomic unsigned maximum on byte in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDUMAXH => {
  title      => 'LDUMAXH, LDUMAXAH, LDUMAXALH, LDUMAXLH',
  brief      => 'Atomic unsigned maximum on halfword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDUMIN => {
  title      => 'LDUMIN, LDUMINA, LDUMINAL, LDUMINL',
  brief      => 'Atomic unsigned minimum on word or doubleword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDUMINB => {
  title      => 'LDUMINB, LDUMINAB, LDUMINALB, LDUMINLB',
  brief      => 'Atomic unsigned minimum on byte in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDUMINH => {
  title      => 'LDUMINH, LDUMINAH, LDUMINALH, LDUMINLH',
  brief      => 'Atomic unsigned minimum on halfword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDUR_gen => {
  title      => 'LDUR',
  brief      => 'Load Register (unscaled)',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDUR offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDURB => {
  title      => 'LDURB',
  brief      => 'Load Register Byte (unscaled)',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDURB offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE LDURH => {
  title      => 'LDURH',
  brief      => 'Load Register Halfword (unscaled)',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=LDURH offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE LDURSB => {
  title      => 'LDURSB',
  brief      => 'Load Register Signed Byte (unscaled)',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDURSB offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDURSH => {
  title      => 'LDURSH',
  brief      => 'Load Register Signed Halfword (unscaled)',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=LDURSH offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDURSW => {
  title      => 'LDURSW',
  brief      => 'Load Register Signed Word (unscaled)',
  docvars    => 'address-form=base-plus-offset datatype=64 instr-class=general isa=A64 mnemonic=LDURSW offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC datasize=64',
};

PAGE LDXP => {
  title      => 'LDXP',
  brief      => 'Load Exclusive Pair of Registers',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=LDXP',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDXR => {
  title      => 'LDXR',
  brief      => 'Load Exclusive Register',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=LDXR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDXRB => {
  title      => 'LDXRB',
  brief      => 'Load Exclusive Register Byte',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDXRB-32-reg instr-class=general isa=A64 mnemonic=LDXRB reg-type=32-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LDXRH => {
  title      => 'LDXRH',
  brief      => 'Load Exclusive Register Halfword',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=LDXRH-32-reg instr-class=general isa=A64 mnemonic=LDXRH reg-type=32-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LSLV => {
  title      => 'LSLV',
  brief      => 'Logical Shift Left Variable',
  docvars    => 'instr-class=general isa=A64 mnemonic=LSLV',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE LSRV => {
  title      => 'LSRV',
  brief      => 'Logical Shift Right Variable',
  docvars    => 'instr-class=general isa=A64 mnemonic=LSRV',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MADD => {
  title      => 'MADD',
  brief      => 'Multiply-Add',
  docvars    => 'instr-class=general isa=A64 mnemonic=MADD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MOVK => {
  title      => 'MOVK',
  brief      => 'Move wide with keep',
  docvars    => 'immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVK',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MOVN => {
  title      => 'MOVN',
  brief      => 'Move wide with NOT',
  docvars    => 'immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVN',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MOVZ => {
  title      => 'MOVZ',
  brief      => 'Move wide with zero',
  docvars    => 'immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVZ',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MRS => {
  title      => 'MRS',
  brief      => 'Move System Register',
  docvars    => 'instr-class=system isa=A64 mnemonic=MRS',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MSR_imm => {
  title      => 'MSR (immediate)',
  brief      => 'Move immediate value to Special Register',
  docvars    => 'instr-class=system isa=A64 mnemonic=MSR msr-sysreg-target=whole-register',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MSR_reg => {
  title      => 'MSR (register)',
  brief      => 'Move general-purpose register to System Register',
  docvars    => 'instr-class=system isa=A64 mnemonic=MSR msr-sysreg-target=register-field',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE MSUB => {
  title      => 'MSUB',
  brief      => 'Multiply-Subtract',
  docvars    => 'instr-class=general isa=A64 mnemonic=MSUB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE NOP => {
  title      => 'NOP',
  brief      => 'No Operation',
  docvars    => 'instr-class=system isa=A64 mnemonic=NOP',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ORN_log_shift => {
  title      => 'ORN (shifted register)',
  brief      => 'Bitwise OR NOT (shifted register)',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=ORN reguse=shifted-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ORR_log_imm => {
  title      => 'ORR (immediate)',
  brief      => 'Bitwise OR (immediate)',
  docvars    => 'cond-setting=no-s immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=ORR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ORR_log_shift => {
  title      => 'ORR (shifted register)',
  brief      => 'Bitwise OR (shifted register)',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=ORR reguse=shifted-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PACDA => {
  title      => 'PACDA, PACDZA',
  brief      => 'Pointer Authentication Code for Data address, using key A',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PACDB => {
  title      => 'PACDB, PACDZB',
  brief      => 'Pointer Authentication Code for Data address, using key B',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PACGA => {
  title      => 'PACGA',
  brief      => 'Pointer Authentication Code, using Generic key',
  docvars    => 'instr-class=general isa=A64 mnemonic=PACGA',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PACIA => {
  title      => 'PACIA, PACIA1716, PACIASP, PACIAZ, PACIZA',
  brief      => 'Pointer Authentication Code for Instruction address, using key A',
  docvars    => 'isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PACIB => {
  title      => 'PACIB, PACIB1716, PACIBSP, PACIBZ, PACIZB',
  brief      => 'Pointer Authentication Code for Instruction address, using key B',
  docvars    => 'isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PRFM_imm => {
  title      => 'PRFM (immediate)',
  brief      => 'Prefetch Memory (immediate)',
  docvars    => 'address-form=unsigned-scaled-offset instr-class=general isa=A64 mnemonic=PRFM offset-type=off12u_s',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PRFM_lit => {
  title      => 'PRFM (literal)',
  brief      => 'Prefetch Memory (literal)',
  docvars    => 'address-form=literal instr-class=general isa=A64 mnemonic=PRFM offset-type=off19s',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PRFM_reg => {
  title      => 'PRFM (register)',
  brief      => 'Prefetch Memory (register)',
  docvars    => 'instr-class=general isa=A64 mnemonic=PRFM offset-type=off-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PRFUM => {
  title      => 'PRFUM',
  brief      => 'Prefetch Memory (unscaled offset)',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=PRFUM offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PSB => {
  title      => 'PSB CSYNC',
  brief      => 'Profiling Synchronization Barrier',
  docvars    => 'instr-class=system isa=A64 mnemonic=PSB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE PSSBB => {
  title      => 'PSSBB',
  brief      => 'Physical Speculative Store Bypass Barrier',
  docvars    => 'instr-class=system isa=A64 mnemonic=PSSBB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE RBIT_int => {
  title      => 'RBIT',
  brief      => 'Reverse Bits',
  docvars    => 'instr-class=general isa=A64 mnemonic=RBIT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE RET => {
  title      => 'RET',
  brief      => 'Return from subroutine',
  docvars    => 'instr-class=general isa=A64 mnemonic=RET',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE RETA => {
  title      => 'RETAA, RETAB',
  brief      => 'Return from subroutine, with pointer authentication',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE REV => {
  title      => 'REV',
  brief      => 'Reverse Bytes',
  docvars    => 'instr-class=general isa=A64 mnemonic=REV',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE REV16_int => {
  title      => 'REV16',
  brief      => 'Reverse bytes in 16-bit halfwords',
  docvars    => 'instr-class=general isa=A64 mnemonic=REV16',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE REV32_int => {
  title      => 'REV32',
  brief      => 'Reverse bytes in 32-bit words',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=REV32',
  metadata   => 'alias=0 alphaindex=BASIC datasize=64',
};

PAGE RMIF => {
  title      => 'RMIF',
  brief      => 'Rotate, Mask Insert Flags',
  docvars    => 'instr-class=general isa=A64 mnemonic=RMIF',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE RORV => {
  title      => 'RORV',
  brief      => 'Rotate Right Variable',
  docvars    => 'instr-class=general isa=A64 mnemonic=RORV',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SB => {
  title      => 'SB',
  brief      => 'Speculation Barrier',
  docvars    => 'instr-class=system isa=A64 mnemonic=SB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SBC => {
  title      => 'SBC',
  brief      => 'Subtract with Carry',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=SBC',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SBCS => {
  title      => 'SBCS',
  brief      => 'Subtract with Carry, setting flags',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=SBCS',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SBFM => {
  title      => 'SBFM',
  brief      => 'Signed Bitfield Move',
  docvars    => 'bitfield-fill=signed-fill instr-class=general isa=A64 mnemonic=SBFM',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SDIV => {
  title      => 'SDIV',
  brief      => 'Signed Divide',
  docvars    => 'instr-class=general isa=A64 mnemonic=SDIV',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SETF => {
  title      => 'SETF8, SETF16',
  brief      => 'Evaluation of 8 or 16 bit flag values',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SEV => {
  title      => 'SEV',
  brief      => 'Send Event',
  docvars    => 'instr-class=system isa=A64 mnemonic=SEV',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SEVL => {
  title      => 'SEVL',
  brief      => 'Send Event Local',
  docvars    => 'instr-class=system isa=A64 mnemonic=SEVL',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMADDL => {
  title      => 'SMADDL',
  brief      => 'Signed Multiply-Add Long',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=SMADDL',
  metadata   => 'alias=0 alphaindex=BASIC datasize=64',
};

PAGE SMC => {
  title      => 'SMC',
  brief      => 'Secure Monitor Call',
  docvars    => 'instr-class=system isa=A64 mnemonic=SMC',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SMSUBL => {
  title      => 'SMSUBL',
  brief      => 'Signed Multiply-Subtract Long',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=SMSUBL',
  metadata   => 'alias=0 alphaindex=BASIC datasize=64',
};

PAGE SMULH => {
  title      => 'SMULH',
  brief      => 'Signed Multiply High',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=SMULH',
  metadata   => 'alias=0 alphaindex=BASIC datasize=64',
};

PAGE SSBB => {
  title      => 'SSBB',
  brief      => 'Speculative Store Bypass Barrier',
  docvars    => 'instr-class=system isa=A64 mnemonic=SSBB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ST2G => {
  title      => 'ST2G',
  brief      => 'Store Allocation Tags',
  docvars    => 'instr-class=general isa=A64 mnemonic=ST2G',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STG => {
  title      => 'STG',
  brief      => 'Store Allocation Tag',
  docvars    => 'instr-class=general isa=A64 mnemonic=STG',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STGM => {
  title      => 'STGM',
  brief      => 'Store Tag Multiple',
  docvars    => 'instr-class=general isa=A64 mnemonic=STGM',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STGP => {
  title      => 'STGP',
  brief      => 'Store Allocation Tag and Pair of registers',
  docvars    => 'atomic-ops=STGP-pair-64 instr-class=general isa=A64 mnemonic=STGP offset-type=off7s_s reg-type=pair-64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STLLR => {
  title      => 'STLLR',
  brief      => 'Store LORelease Register',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=STLLR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STLLRB => {
  title      => 'STLLRB',
  brief      => 'Store LORelease Register Byte',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLLRB-32-reg instr-class=general isa=A64 mnemonic=STLLRB reg-type=32-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STLLRH => {
  title      => 'STLLRH',
  brief      => 'Store LORelease Register Halfword',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLLRH-32-reg instr-class=general isa=A64 mnemonic=STLLRH reg-type=32-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STLR => {
  title      => 'STLR',
  brief      => 'Store-Release Register',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=STLR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STLRB => {
  title      => 'STLRB',
  brief      => 'Store-Release Register Byte',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLRB-32-reg instr-class=general isa=A64 mnemonic=STLRB reg-type=32-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STLRH => {
  title      => 'STLRH',
  brief      => 'Store-Release Register Halfword',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLRH-32-reg instr-class=general isa=A64 mnemonic=STLRH reg-type=32-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STLUR_gen => {
  title      => 'STLUR',
  brief      => 'Store-Release Register (unscaled)',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=STLUR offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STLURB => {
  title      => 'STLURB',
  brief      => 'Store-Release Register Byte (unscaled)',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STLURB offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE STLURH => {
  title      => 'STLURH',
  brief      => 'Store-Release Register Halfword (unscaled)',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STLURH offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE STLXP => {
  title      => 'STLXP',
  brief      => 'Store-Release Exclusive Pair of registers',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=STLXP',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STLXR => {
  title      => 'STLXR',
  brief      => 'Store-Release Exclusive Register',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=STLXR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STLXRB => {
  title      => 'STLXRB',
  brief      => 'Store-Release Exclusive Register Byte',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLXRB-32-reg instr-class=general isa=A64 mnemonic=STLXRB reg-type=32-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STLXRH => {
  title      => 'STLXRH',
  brief      => 'Store-Release Exclusive Register Halfword',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STLXRH-32-reg instr-class=general isa=A64 mnemonic=STLXRH reg-type=32-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STNP_gen => {
  title      => 'STNP',
  brief      => 'Store Pair of Registers, with non-temporal hint',
  docvars    => 'address-form=signed-scaled-offset instr-class=general isa=A64 mnemonic=STNP offset-type=off7s_s',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STP_gen => {
  title      => 'STP',
  brief      => 'Store Pair of Registers',
  docvars    => 'instr-class=general isa=A64 mnemonic=STP offset-type=off7s_s',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STR_imm_gen => {
  title      => 'STR (immediate)',
  brief      => 'Store Register (immediate)',
  docvars    => 'instr-class=general isa=A64 mnemonic=STR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STR_reg_gen => {
  title      => 'STR (register)',
  brief      => 'Store Register (register)',
  docvars    => 'instr-class=general isa=A64 mnemonic=STR offset-type=off-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STRB_imm => {
  title      => 'STRB (immediate)',
  brief      => 'Store Register Byte (immediate)',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=STRB',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE STRB_reg => {
  title      => 'STRB (register)',
  brief      => 'Store Register Byte (register)',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=STRB offset-type=off-reg',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE STRH_imm => {
  title      => 'STRH (immediate)',
  brief      => 'Store Register Halfword (immediate)',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=STRH',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE STRH_reg => {
  title      => 'STRH (register)',
  brief      => 'Store Register Halfword (register)',
  docvars    => 'datatype=32 instr-class=general isa=A64 mnemonic=STRH offset-type=off-reg',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE STTR => {
  title      => 'STTR',
  brief      => 'Store Register (unprivileged)',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=STTR offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STTRB => {
  title      => 'STTRB',
  brief      => 'Store Register Byte (unprivileged)',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STTRB offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE STTRH => {
  title      => 'STTRH',
  brief      => 'Store Register Halfword (unprivileged)',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STTRH offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE STUR_gen => {
  title      => 'STUR',
  brief      => 'Store Register (unscaled)',
  docvars    => 'address-form=base-plus-offset instr-class=general isa=A64 mnemonic=STUR offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STURB => {
  title      => 'STURB',
  brief      => 'Store Register Byte (unscaled)',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STURB offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE STURH => {
  title      => 'STURH',
  brief      => 'Store Register Halfword (unscaled)',
  docvars    => 'address-form=base-plus-offset datatype=32 instr-class=general isa=A64 mnemonic=STURH offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=BASIC datasize=32',
};

PAGE STXP => {
  title      => 'STXP',
  brief      => 'Store Exclusive Pair of registers',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=STXP',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STXR => {
  title      => 'STXR',
  brief      => 'Store Exclusive Register',
  docvars    => 'address-form=base-register instr-class=general isa=A64 mnemonic=STXR',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STXRB => {
  title      => 'STXRB',
  brief      => 'Store Exclusive Register Byte',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STXRB-32-reg instr-class=general isa=A64 mnemonic=STXRB reg-type=32-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STXRH => {
  title      => 'STXRH',
  brief      => 'Store Exclusive Register Halfword',
  docvars    => 'address-form=base-register address-form-reg-type=base-register-32-reg atomic-ops=STXRH-32-reg instr-class=general isa=A64 mnemonic=STXRH reg-type=32-reg',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STZ2G => {
  title      => 'STZ2G',
  brief      => 'Store Allocation Tags, Zeroing',
  docvars    => 'instr-class=general isa=A64 mnemonic=STZ2G',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STZG => {
  title      => 'STZG',
  brief      => 'Store Allocation Tag, Zeroing',
  docvars    => 'instr-class=general isa=A64 mnemonic=STZG',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE STZGM => {
  title      => 'STZGM',
  brief      => 'Store Tag and Zero Multiple',
  docvars    => 'instr-class=general isa=A64 mnemonic=STZGM',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SUB_addsub_ext => {
  title      => 'SUB (extended register)',
  brief      => 'Subtract (extended register)',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=SUB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SUB_addsub_imm => {
  title      => 'SUB (immediate)',
  brief      => 'Subtract (immediate)',
  docvars    => 'cond-setting=no-s immediate-type=imm12u instr-class=general isa=A64 mnemonic=SUB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SUB_addsub_shift => {
  title      => 'SUB (shifted register)',
  brief      => 'Subtract (shifted register)',
  docvars    => 'cond-setting=no-s instr-class=general isa=A64 mnemonic=SUB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SUBG => {
  title      => 'SUBG',
  brief      => 'Subtract with Tag',
  docvars    => 'instr-class=general isa=A64 mnemonic=SUBG',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SUBP => {
  title      => 'SUBP',
  brief      => 'Subtract Pointer',
  docvars    => 'instr-class=general isa=A64 mnemonic=SUBP',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SUBPS => {
  title      => 'SUBPS',
  brief      => 'Subtract Pointer, setting Flags',
  docvars    => 'instr-class=general isa=A64 mnemonic=SUBPS',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SUBS_addsub_ext => {
  title      => 'SUBS (extended register)',
  brief      => 'Subtract (extended register), setting flags',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SUBS_addsub_imm => {
  title      => 'SUBS (immediate)',
  brief      => 'Subtract (immediate), setting flags',
  docvars    => 'cond-setting=S immediate-type=imm12u instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SUBS_addsub_shift => {
  title      => 'SUBS (shifted register)',
  brief      => 'Subtract (shifted register), setting flags',
  docvars    => 'cond-setting=S instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SVC => {
  title      => 'SVC',
  brief      => 'Supervisor Call',
  docvars    => 'instr-class=system isa=A64 mnemonic=SVC',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SWP => {
  title      => 'SWP, SWPA, SWPAL, SWPL',
  brief      => 'Swap word or doubleword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SWPB => {
  title      => 'SWPB, SWPAB, SWPALB, SWPLB',
  brief      => 'Swap byte in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SWPH => {
  title      => 'SWPH, SWPAH, SWPALH, SWPLH',
  brief      => 'Swap halfword in memory',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SYS => {
  title      => 'SYS',
  brief      => 'System instruction',
  docvars    => 'instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE SYSL => {
  title      => 'SYSL',
  brief      => 'System instruction with result',
  docvars    => 'instr-class=system isa=A64 mnemonic=SYSL',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE TBNZ => {
  title      => 'TBNZ',
  brief      => 'Test bit and Branch if Nonzero',
  docvars    => 'branch-offset=br14 instr-class=general isa=A64 mnemonic=TBNZ',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE TBZ => {
  title      => 'TBZ',
  brief      => 'Test bit and Branch if Zero',
  docvars    => 'branch-offset=br14 instr-class=general isa=A64 mnemonic=TBZ',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE TCANCEL => {
  title      => 'TCANCEL',
  brief      => 'Cancel current transaction',
  docvars    => 'instr-class=system isa=A64 mnemonic=TCANCEL',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE TCOMMIT => {
  title      => 'TCOMMIT',
  brief      => 'Commit current transaction',
  docvars    => 'instr-class=system isa=A64 mnemonic=TCOMMIT',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE TSB => {
  title      => 'TSB CSYNC',
  brief      => 'Trace Synchronization Barrier',
  docvars    => 'instr-class=system isa=A64 mnemonic=TSB',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE TSTART => {
  title      => 'TSTART',
  brief      => 'Start transaction',
  docvars    => 'instr-class=system isa=A64 mnemonic=TSTART',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE TTEST => {
  title      => 'TTEST',
  brief      => 'Test transaction state',
  docvars    => 'instr-class=system isa=A64 mnemonic=TTEST',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UBFM => {
  title      => 'UBFM',
  brief      => 'Unsigned Bitfield Move',
  docvars    => 'bitfield-fill=zero-fill instr-class=general isa=A64 mnemonic=UBFM',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UDF_perm_undef => {
  title      => 'UDF',
  brief      => 'Permanently Undefined',
  docvars    => 'instr-class=general isa=A64 mnemonic=UDF',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UDIV => {
  title      => 'UDIV',
  brief      => 'Unsigned Divide',
  docvars    => 'instr-class=general isa=A64 mnemonic=UDIV',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE UMADDL => {
  title      => 'UMADDL',
  brief      => 'Unsigned Multiply-Add Long',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=UMADDL',
  metadata   => 'alias=0 alphaindex=BASIC datasize=64',
};

PAGE UMSUBL => {
  title      => 'UMSUBL',
  brief      => 'Unsigned Multiply-Subtract Long',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=UMSUBL',
  metadata   => 'alias=0 alphaindex=BASIC datasize=64',
};

PAGE UMULH => {
  title      => 'UMULH',
  brief      => 'Unsigned Multiply High',
  docvars    => 'datatype=64 instr-class=general isa=A64 mnemonic=UMULH',
  metadata   => 'alias=0 alphaindex=BASIC datasize=64',
};

PAGE WFE => {
  title      => 'WFE',
  brief      => 'Wait For Event',
  docvars    => 'instr-class=system isa=A64 mnemonic=WFE',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE WFI => {
  title      => 'WFI',
  brief      => 'Wait For Interrupt',
  docvars    => 'instr-class=system isa=A64 mnemonic=WFI',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE XAFLAG => {
  title      => 'XAFLAG',
  brief      => 'Convert floating-point condition flags from external format to Arm format',
  docvars    => 'instr-class=system isa=A64 mnemonic=XAFLAG',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE XPAC => {
  title      => 'XPACD, XPACI, XPACLRI',
  brief      => 'Strip Pointer Authentication Code',
  docvars    => 'isa=A64',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE YIELD => {
  title      => 'YIELD',
  brief      => 'YIELD',
  docvars    => 'instr-class=system isa=A64 mnemonic=YIELD',
  metadata   => 'alias=0 alphaindex=BASIC',
};

PAGE ASR_ASRV => {
  title      => 'ASR (register)',
  brief      => 'Arithmetic Shift Right (register)',
  docvars    => 'alias_mnemonic=ASR instr-class=general isa=A64 mnemonic=ASRV source-type=src-is-register',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE ASR_SBFM => {
  title      => 'ASR (immediate)',
  brief      => 'Arithmetic Shift Right (immediate)',
  docvars    => 'alias_mnemonic=ASR bitfield-fill=signed-fill instr-class=general isa=A64 mnemonic=SBFM source-type=src-is-immediate',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE AT_SYS => {
  title      => 'AT',
  brief      => 'Address Translate',
  docvars    => 'alias_mnemonic=AT instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE BFC_BFM => {
  title      => 'BFC',
  brief      => 'Bitfield Clear',
  docvars    => 'alias_mnemonic=BFC bitfield-fill=nofill instr-class=general isa=A64 mnemonic=BFM source-type=src-is-immediate',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE BFI_BFM => {
  title      => 'BFI',
  brief      => 'Bitfield Insert',
  docvars    => 'alias_mnemonic=BFI bitfield-fill=nofill instr-class=general isa=A64 mnemonic=BFM source-type=src-is-immediate',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE BFXIL_BFM => {
  title      => 'BFXIL',
  brief      => 'Bitfield extract and insert at low end',
  docvars    => 'alias_mnemonic=BFXIL bitfield-fill=nofill instr-class=general isa=A64 mnemonic=BFM source-type=src-is-immediate',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE CFP_SYS => {
  title      => 'CFP',
  brief      => 'Control Flow Prediction Restriction by Context',
  docvars    => 'alias_mnemonic=CFP instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE CINC_CSINC => {
  title      => 'CINC',
  brief      => 'Conditional Increment',
  docvars    => 'alias_mnemonic=CINC instr-class=general isa=A64 mnemonic=CSINC',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE CINV_CSINV => {
  title      => 'CINV',
  brief      => 'Conditional Invert',
  docvars    => 'alias_mnemonic=CINV instr-class=general isa=A64 mnemonic=CSINV',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE CMN_ADDS_addsub_ext => {
  title      => 'CMN (extended register)',
  brief      => 'Compare Negative (extended register)',
  docvars    => 'alias_mnemonic=CMN cond-setting=S instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE CMN_ADDS_addsub_imm => {
  title      => 'CMN (immediate)',
  brief      => 'Compare Negative (immediate)',
  docvars    => 'alias_mnemonic=CMN cond-setting=S immediate-type=imm12u instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE CMN_ADDS_addsub_shift => {
  title      => 'CMN (shifted register)',
  brief      => 'Compare Negative (shifted register)',
  docvars    => 'alias_mnemonic=CMN cond-setting=S instr-class=general isa=A64 mnemonic=ADDS',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE CMP_SUBS_addsub_ext => {
  title      => 'CMP (extended register)',
  brief      => 'Compare (extended register)',
  docvars    => 'alias_mnemonic=CMP cond-setting=S instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE CMP_SUBS_addsub_imm => {
  title      => 'CMP (immediate)',
  brief      => 'Compare (immediate)',
  docvars    => 'alias_mnemonic=CMP cond-setting=S immediate-type=imm12u instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE CMP_SUBS_addsub_shift => {
  title      => 'CMP (shifted register)',
  brief      => 'Compare (shifted register)',
  docvars    => 'alias_mnemonic=CMP cond-setting=S instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE CMPP_SUBPS => {
  title      => 'CMPP',
  brief      => 'Compare with Tag',
  docvars    => 'alias_mnemonic=CMPP instr-class=general isa=A64 mnemonic=SUBPS source-type=src-is-register',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE CNEG_CSNEG => {
  title      => 'CNEG',
  brief      => 'Conditional Negate',
  docvars    => 'alias_mnemonic=CNEG instr-class=general isa=A64 mnemonic=CSNEG',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE CPP_SYS => {
  title      => 'CPP',
  brief      => 'Cache Prefetch Prediction Restriction by Context',
  docvars    => 'alias_mnemonic=CPP instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE CSET_CSINC => {
  title      => 'CSET',
  brief      => 'Conditional Set',
  docvars    => 'alias_mnemonic=CSET instr-class=general isa=A64 mnemonic=CSINC',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE CSETM_CSINV => {
  title      => 'CSETM',
  brief      => 'Conditional Set Mask',
  docvars    => 'alias_mnemonic=CSETM instr-class=general isa=A64 mnemonic=CSINV',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE DC_SYS => {
  title      => 'DC',
  brief      => 'Data Cache operation',
  docvars    => 'alias_mnemonic=DC instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE DFB_DSB => {
  title      => 'DFB',
  brief      => 'Data Full Barrier',
  docvars    => 'alias_mnemonic=DFB instr-class=system isa=A64 mnemonic=DSB',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE DVP_SYS => {
  title      => 'DVP',
  brief      => 'Data Value Prediction Restriction by Context',
  docvars    => 'alias_mnemonic=DVP instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE IC_SYS => {
  title      => 'IC',
  brief      => 'Instruction Cache operation',
  docvars    => 'alias_mnemonic=IC instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE LSL_LSLV => {
  title      => 'LSL (register)',
  brief      => 'Logical Shift Left (register)',
  docvars    => 'alias_mnemonic=LSL instr-class=general isa=A64 mnemonic=LSLV source-type=src-is-register',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE LSL_UBFM => {
  title      => 'LSL (immediate)',
  brief      => 'Logical Shift Left (immediate)',
  docvars    => 'alias_mnemonic=LSL bitfield-fill=zero-fill instr-class=general isa=A64 mnemonic=UBFM source-type=src-is-immediate',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE LSR_LSRV => {
  title      => 'LSR (register)',
  brief      => 'Logical Shift Right (register)',
  docvars    => 'alias_mnemonic=LSR instr-class=general isa=A64 mnemonic=LSRV source-type=src-is-register',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE LSR_UBFM => {
  title      => 'LSR (immediate)',
  brief      => 'Logical Shift Right (immediate)',
  docvars    => 'alias_mnemonic=LSR bitfield-fill=zero-fill instr-class=general isa=A64 mnemonic=UBFM source-type=src-is-immediate',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE MNEG_MSUB => {
  title      => 'MNEG',
  brief      => 'Multiply-Negate',
  docvars    => 'alias_mnemonic=MNEG instr-class=general isa=A64 mnemonic=MSUB',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE MOV_ADD_addsub_imm => {
  title      => 'MOV (to/from SP)',
  brief      => 'Move between register and stack pointer',
  docvars    => 'alias_mnemonic=MOV cond-setting=no-s immediate-type=imm12u instr-class=general isa=A64 mnemonic=ADD move-what=to-from-sp',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE MOV_MOVN => {
  title      => 'MOV (inverted wide immediate)',
  brief      => 'Move (inverted wide immediate)',
  docvars    => 'alias_mnemonic=MOV immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVN move-what=mov-wideinv',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE MOV_MOVZ => {
  title      => 'MOV (wide immediate)',
  brief      => 'Move (wide immediate)',
  docvars    => 'alias_mnemonic=MOV immediate-type=imm18-packed instr-class=general isa=A64 mnemonic=MOVZ move-what=mov-wideimm',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE MOV_ORR_log_imm => {
  title      => 'MOV (bitmask immediate)',
  brief      => 'Move (bitmask immediate)',
  docvars    => 'alias_mnemonic=MOV cond-setting=no-s immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=ORR move-what=mov-bitmask',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE MOV_ORR_log_shift => {
  title      => 'MOV (register)',
  brief      => 'Move (register)',
  docvars    => 'alias_mnemonic=MOV cond-setting=no-s instr-class=general isa=A64 mnemonic=ORR move-what=mov-register reguse=shifted-reg',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE MUL_MADD => {
  title      => 'MUL',
  brief      => 'Multiply',
  docvars    => 'alias_mnemonic=MUL instr-class=general isa=A64 mnemonic=MADD',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE MVN_ORN_log_shift => {
  title      => 'MVN',
  brief      => 'Bitwise NOT',
  docvars    => 'alias_mnemonic=MVN cond-setting=no-s instr-class=general isa=A64 mnemonic=ORN reguse=shifted-reg',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE NEG_SUB_addsub_shift => {
  title      => 'NEG (shifted register)',
  brief      => 'Negate (shifted register)',
  docvars    => 'alias_mnemonic=NEG cond-setting=no-s instr-class=general isa=A64 mnemonic=SUB',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE NEGS_SUBS_addsub_shift => {
  title      => 'NEGS',
  brief      => 'Negate, setting flags',
  docvars    => 'alias_mnemonic=NEGS cond-setting=S instr-class=general isa=A64 mnemonic=SUBS',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE NGC_SBC => {
  title      => 'NGC',
  brief      => 'Negate with Carry',
  docvars    => 'alias_mnemonic=NGC cond-setting=no-s instr-class=general isa=A64 mnemonic=SBC',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE NGCS_SBCS => {
  title      => 'NGCS',
  brief      => 'Negate with Carry, setting flags',
  docvars    => 'alias_mnemonic=NGCS cond-setting=S instr-class=general isa=A64 mnemonic=SBCS',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE REV64_REV => {
  title      => 'REV64',
  brief      => 'Reverse Bytes',
  docvars    => 'alias_mnemonic=REV64 datatype=64 instr-class=general isa=A64 mnemonic=REV',
  metadata   => 'alias=1 alphaindex=BASIC datasize=64',
};

PAGE ROR_EXTR => {
  title      => 'ROR (immediate)',
  brief      => 'Rotate right (immediate)',
  docvars    => 'alias_mnemonic=ROR instr-class=general isa=A64 mnemonic=EXTR source-type=src-is-immediate',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE ROR_RORV => {
  title      => 'ROR (register)',
  brief      => 'Rotate Right (register)',
  docvars    => 'alias_mnemonic=ROR instr-class=general isa=A64 mnemonic=RORV source-type=src-is-register',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE SBFIZ_SBFM => {
  title      => 'SBFIZ',
  brief      => 'Signed Bitfield Insert in Zero',
  docvars    => 'alias_mnemonic=SBFIZ bitfield-fill=signed-fill instr-class=general isa=A64 mnemonic=SBFM source-type=src-is-immediate',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE SBFX_SBFM => {
  title      => 'SBFX',
  brief      => 'Signed Bitfield Extract',
  docvars    => 'alias_mnemonic=SBFX bitfield-fill=signed-fill instr-class=general isa=A64 mnemonic=SBFM source-type=src-is-immediate',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE SMNEGL_SMSUBL => {
  title      => 'SMNEGL',
  brief      => 'Signed Multiply-Negate Long',
  docvars    => 'alias_mnemonic=SMNEGL datatype=64 instr-class=general isa=A64 mnemonic=SMSUBL',
  metadata   => 'alias=1 alphaindex=BASIC datasize=64',
};

PAGE SMULL_SMADDL => {
  title      => 'SMULL',
  brief      => 'Signed Multiply Long',
  docvars    => 'alias_mnemonic=SMULL datatype=64 instr-class=general isa=A64 mnemonic=SMADDL',
  metadata   => 'alias=1 alphaindex=BASIC datasize=64',
};

PAGE STADD_LDADD => {
  title      => 'STADD, STADDL',
  brief      => 'Atomic add on word or doubleword in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STADDB_LDADDB => {
  title      => 'STADDB, STADDLB',
  brief      => 'Atomic add on byte in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STADDH_LDADDH => {
  title      => 'STADDH, STADDLH',
  brief      => 'Atomic add on halfword in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STCLR_LDCLR => {
  title      => 'STCLR, STCLRL',
  brief      => 'Atomic bit clear on word or doubleword in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STCLRB_LDCLRB => {
  title      => 'STCLRB, STCLRLB',
  brief      => 'Atomic bit clear on byte in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STCLRH_LDCLRH => {
  title      => 'STCLRH, STCLRLH',
  brief      => 'Atomic bit clear on halfword in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STEOR_LDEOR => {
  title      => 'STEOR, STEORL',
  brief      => 'Atomic exclusive OR on word or doubleword in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STEORB_LDEORB => {
  title      => 'STEORB, STEORLB',
  brief      => 'Atomic exclusive OR on byte in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STEORH_LDEORH => {
  title      => 'STEORH, STEORLH',
  brief      => 'Atomic exclusive OR on halfword in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STSET_LDSET => {
  title      => 'STSET, STSETL',
  brief      => 'Atomic bit set on word or doubleword in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STSETB_LDSETB => {
  title      => 'STSETB, STSETLB',
  brief      => 'Atomic bit set on byte in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STSETH_LDSETH => {
  title      => 'STSETH, STSETLH',
  brief      => 'Atomic bit set on halfword in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STSMAX_LDSMAX => {
  title      => 'STSMAX, STSMAXL',
  brief      => 'Atomic signed maximum on word or doubleword in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STSMAXB_LDSMAXB => {
  title      => 'STSMAXB, STSMAXLB',
  brief      => 'Atomic signed maximum on byte in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STSMAXH_LDSMAXH => {
  title      => 'STSMAXH, STSMAXLH',
  brief      => 'Atomic signed maximum on halfword in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STSMIN_LDSMIN => {
  title      => 'STSMIN, STSMINL',
  brief      => 'Atomic signed minimum on word or doubleword in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STSMINB_LDSMINB => {
  title      => 'STSMINB, STSMINLB',
  brief      => 'Atomic signed minimum on byte in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STSMINH_LDSMINH => {
  title      => 'STSMINH, STSMINLH',
  brief      => 'Atomic signed minimum on halfword in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STUMAX_LDUMAX => {
  title      => 'STUMAX, STUMAXL',
  brief      => 'Atomic unsigned maximum on word or doubleword in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STUMAXB_LDUMAXB => {
  title      => 'STUMAXB, STUMAXLB',
  brief      => 'Atomic unsigned maximum on byte in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STUMAXH_LDUMAXH => {
  title      => 'STUMAXH, STUMAXLH',
  brief      => 'Atomic unsigned maximum on halfword in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STUMIN_LDUMIN => {
  title      => 'STUMIN, STUMINL',
  brief      => 'Atomic unsigned minimum on word or doubleword in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STUMINB_LDUMINB => {
  title      => 'STUMINB, STUMINLB',
  brief      => 'Atomic unsigned minimum on byte in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE STUMINH_LDUMINH => {
  title      => 'STUMINH, STUMINLH',
  brief      => 'Atomic unsigned minimum on halfword in memory, without return',
  docvars    => 'instr-class=general isa=A64',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE SXTB_SBFM => {
  title      => 'SXTB',
  brief      => 'Signed Extend Byte',
  docvars    => 'alias_mnemonic=SXTB bitfield-fill=signed-fill instr-class=general isa=A64 mnemonic=SBFM source-type=src-is-immediate',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE SXTH_SBFM => {
  title      => 'SXTH',
  brief      => 'Sign Extend Halfword',
  docvars    => 'alias_mnemonic=SXTH bitfield-fill=signed-fill instr-class=general isa=A64 mnemonic=SBFM source-type=src-is-immediate',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE SXTW_SBFM => {
  title      => 'SXTW',
  brief      => 'Sign Extend Word',
  docvars    => 'alias_mnemonic=SXTW bitfield-fill=signed-fill datatype=64 instr-class=general isa=A64 mnemonic=SBFM source-type=src-is-immediate',
  metadata   => 'alias=1 alphaindex=BASIC datasize=64',
};

PAGE TLBI_SYS => {
  title      => 'TLBI',
  brief      => 'TLB Invalidate operation',
  docvars    => 'alias_mnemonic=TLBI instr-class=system isa=A64 mnemonic=SYS',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE TST_ANDS_log_imm => {
  title      => 'TST (immediate)',
  brief      => 'Test bits (immediate)',
  docvars    => 'alias_mnemonic=TST cond-setting=S immediate-type=imm12-bitfield instr-class=general isa=A64 mnemonic=ANDS',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE TST_ANDS_log_shift => {
  title      => 'TST (shifted register)',
  brief      => 'Test (shifted register)',
  docvars    => 'alias_mnemonic=TST cond-setting=S instr-class=general isa=A64 mnemonic=ANDS reguse=shifted-reg',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE UBFIZ_UBFM => {
  title      => 'UBFIZ',
  brief      => 'Unsigned Bitfield Insert in Zero',
  docvars    => 'alias_mnemonic=UBFIZ bitfield-fill=zero-fill instr-class=general isa=A64 mnemonic=UBFM source-type=src-is-immediate',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE UBFX_UBFM => {
  title      => 'UBFX',
  brief      => 'Unsigned Bitfield Extract',
  docvars    => 'alias_mnemonic=UBFX bitfield-fill=zero-fill instr-class=general isa=A64 mnemonic=UBFM source-type=src-is-immediate',
  metadata   => 'alias=1 alphaindex=BASIC',
};

PAGE UMNEGL_UMSUBL => {
  title      => 'UMNEGL',
  brief      => 'Unsigned Multiply-Negate Long',
  docvars    => 'alias_mnemonic=UMNEGL datatype=64 instr-class=general isa=A64 mnemonic=UMSUBL',
  metadata   => 'alias=1 alphaindex=BASIC datasize=64',
};

PAGE UMULL_UMADDL => {
  title      => 'UMULL',
  brief      => 'Unsigned Multiply Long',
  docvars    => 'alias_mnemonic=UMULL datatype=64 instr-class=general isa=A64 mnemonic=UMADDL',
  metadata   => 'alias=1 alphaindex=BASIC datasize=64',
};

PAGE UXTB_UBFM => {
  title      => 'UXTB',
  brief      => 'Unsigned Extend Byte',
  docvars    => 'alias_mnemonic=UXTB bitfield-fill=zero-fill datatype=32 instr-class=general isa=A64 mnemonic=UBFM source-type=src-is-immediate',
  metadata   => 'alias=1 alphaindex=BASIC datasize=32',
};

PAGE UXTH_UBFM => {
  title      => 'UXTH',
  brief      => 'Unsigned Extend Halfword',
  docvars    => 'alias_mnemonic=UXTH bitfield-fill=zero-fill datatype=32 instr-class=general isa=A64 mnemonic=UBFM source-type=src-is-immediate',
  metadata   => 'alias=1 alphaindex=BASIC datasize=32',
};


1;

