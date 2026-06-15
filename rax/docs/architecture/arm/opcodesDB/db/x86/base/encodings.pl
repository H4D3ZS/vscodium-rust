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


# AAA-ASCII Adjust After Addition.
ENCODING AAA => {
  diagram    => 'MODE=NO64 OP=0x37',
  iflags     => 'af=X cf=W of=U pf=U sf=U zf=U',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|DECIMAL',
  metadata   => 'isa=I86 deprecated=1',
  tags       => 'page=AAA',
};


# AAD-ASCII Adjust AX Before Division.
ENCODING AAD => {
  diagram    => 'MODE=NO64 OP=0xd5',
  iflags     => 'af=U cf=U of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|DECIMAL',
  metadata   => 'isa=I86 deprecated=1',
  tags       => 'page=AAD',
};


# AAM-ASCII Adjust AX After Multiply.
ENCODING AAM => {
  diagram    => 'MODE=NO64 OP=0xd4',
  iflags     => 'af=U cf=U of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|DECIMAL',
  metadata   => 'isa=I86 deprecated=1',
  tags       => 'page=AAM',
};


# AAS-ASCII Adjust AL After Subtraction.
ENCODING AAS => {
  diagram    => 'MODE=NO64 OP=0x3f',
  iflags     => 'af=X cf=W of=U pf=U sf=U zf=U',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|DECIMAL',
  metadata   => 'isa=I86 deprecated=1',
  tags       => 'page=AAS',
};


# ADC-Add with Carry.
ENCODING ADC_bsb => {
  diagram    => 'OP=0x14',
  iflags     => 'af=W cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADC',
};

ENCODING ADC_mbrb => {
  diagram    => 'MR=1 OP=0x10',
  iflags     => 'af=W cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADC',
};

ENCODING ADC_mbsb_1 => {
  diagram    => 'MR=1 OP=0x80 REG=2',
  iflags     => 'af=W cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADC',
};

ENCODING ADC_mbsb_2 => {
  diagram    => 'MODE=NO64 MR=1 OP=0x82 REG=2',
  iflags     => 'af=W cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADC',
};

ENCODING ADC_mvrv => {
  diagram    => 'MR=1 OP=0x11',
  iflags     => 'af=W cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADC',
};

ENCODING ADC_mvsb => {
  diagram    => 'MR=1 OP=0x83 REG=2',
  iflags     => 'af=W cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADC',
};

ENCODING ADC_mvsz => {
  diagram    => 'MR=1 OP=0x81 REG=2',
  iflags     => 'af=W cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADC',
};

ENCODING ADC_rbmb => {
  diagram    => 'MR=1 OP=0x12',
  iflags     => 'af=W cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADC',
};

ENCODING ADC_rvmv => {
  diagram    => 'MR=1 OP=0x13',
  iflags     => 'af=W cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADC',
};

ENCODING ADC_vsz => {
  diagram    => 'OP=0x15',
  iflags     => 'af=W cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADC',
};


# ADCX-Unsigned Integer Addition of Two Operands with Carry Flag.
ENCODING ADCX => {
  diagram    => 'MAP=0f38 MR=1 OP=0xf6 P66=1 PF2=0 PF3=0',
  iflags     => 'cf=X',
  extensions => 'ADOX_ADCX',
  categories => 'GENERAL|ARITHMETIC',
  metadata   => 'isa=ADOX_ADCX',
  tags       => 'page=ADCX',
};


# ADD-Add.
ENCODING ADD_bsb => {
  diagram    => 'OP=0x04',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADD',
};

ENCODING ADD_mbrb => {
  diagram    => 'MR=1 OP=0x00',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADD',
};

ENCODING ADD_mbsb_1 => {
  diagram    => 'MR=1 OP=0x80 REG=0',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADD',
};

ENCODING ADD_mbsb_2 => {
  diagram    => 'MODE=NO64 MR=1 OP=0x82 REG=0',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADD',
};

ENCODING ADD_mvrv => {
  diagram    => 'MR=1 OP=0x01',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADD',
};

ENCODING ADD_mvsb => {
  diagram    => 'MR=1 OP=0x83 REG=0',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADD',
};

ENCODING ADD_mvsz => {
  diagram    => 'MR=1 OP=0x81 REG=0',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADD',
};

ENCODING ADD_rbmb => {
  diagram    => 'MR=1 OP=0x02',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADD',
};

ENCODING ADD_rvmv => {
  diagram    => 'MR=1 OP=0x03',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADD',
};

ENCODING ADD_vsz => {
  diagram    => 'OP=0x05',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=ADD',
};


# ADOX-Unsigned Integer Addition of Two Operands with Overflow Flag.
ENCODING ADOX => {
  diagram    => 'MAP=0f38 MR=1 OP=0xf6 P66=0 PF2=0 PF3=1',
  iflags     => 'of=X',
  extensions => 'ADOX_ADCX',
  categories => 'GENERAL|ARITHMETIC',
  metadata   => 'isa=ADOX_ADCX',
  tags       => 'page=ADOX',
};


# AESDEC-Perform One Round of an AES Decryption Flow.
ENCODING AESDEC => {
  diagram    => 'MAP=0f38 MR=1 OP=0xde P66=1 PF2=0 PF3=0',
  extensions => 'AES',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=AES requires_alignment=1',
  tags       => 'page=AESDEC',
};


# AESDECLAST-Perform Last Round of an AES Decryption Flow.
ENCODING AESDECLAST => {
  diagram    => 'MAP=0f38 MR=1 OP=0xdf P66=1 PF2=0 PF3=0',
  extensions => 'AES',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=AES requires_alignment=1',
  tags       => 'page=AESDECLAST',
};


# AESENC-Perform One Round of an AES Encryption Flow.
ENCODING AESENC => {
  diagram    => 'MAP=0f38 MR=1 OP=0xdc P66=1 PF2=0 PF3=0',
  extensions => 'AES',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=AES requires_alignment=1',
  tags       => 'page=AESENC',
};


# AESENCLAST-Perform Last Round of an AES Encryption Flow.
ENCODING AESENCLAST => {
  diagram    => 'MAP=0f38 MR=1 OP=0xdd P66=1 PF2=0 PF3=0',
  extensions => 'AES',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=AES requires_alignment=1',
  tags       => 'page=AESENCLAST',
};


# AESIMC-Perform the AES InvMixColumn Transformation.
ENCODING AESIMC => {
  diagram    => 'MAP=0f38 MR=1 OP=0xdb P66=1 PF2=0 PF3=0',
  extensions => 'AES',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=AES requires_alignment=1',
  tags       => 'page=AESIMC',
};


# AESKEYGENASSIST-AES Round Key Generation Assist.
ENCODING AESKEYGENASSIST => {
  diagram    => 'MAP=0f3a MR=1 OP=0xdf P66=1 PF2=0 PF3=0',
  extensions => 'AES',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=AES requires_alignment=1',
  tags       => 'page=AESKEYGENASSIST',
};


# AND-Logical AND.
ENCODING AND_bsb => {
  diagram    => 'OP=0x24',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=AND',
};

ENCODING AND_mbrb => {
  diagram    => 'MR=1 OP=0x20',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=AND',
};

ENCODING AND_mbub_1 => {
  diagram    => 'MR=1 OP=0x80 REG=4',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=AND',
};

ENCODING AND_mbub_2 => {
  diagram    => 'MODE=NO64 MR=1 OP=0x82 REG=4',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=AND',
};

ENCODING AND_mvrv => {
  diagram    => 'MR=1 OP=0x21',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=AND',
};

ENCODING AND_mvsb => {
  diagram    => 'MR=1 OP=0x83 REG=4',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=AND',
};

ENCODING AND_mvsz => {
  diagram    => 'MR=1 OP=0x81 REG=4',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=AND',
};

ENCODING AND_rbmb => {
  diagram    => 'MR=1 OP=0x22',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=AND',
};

ENCODING AND_rvmv => {
  diagram    => 'MR=1 OP=0x23',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=AND',
};

ENCODING AND_vsz => {
  diagram    => 'OP=0x25',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=AND',
};


# ANDN-Logical AND NOT.
ENCODING ANDN => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xf2 P66=0 PF2=0 PF3=0 VL=128',
  iflags     => 'af=U cf=Z of=Z pf=U sf=W zf=W',
  extensions => 'BMI1',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=BMI1',
  tags       => 'page=ANDN',
};


# ARPL-Adjust RPL Field of Segment Selector.
ENCODING ARPL => {
  diagram    => 'MODE=NO64 MR=1 OP=0x63',
  iflags     => 'zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I286PROTECTED',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=ARPL',
};


# BEXTR-Bit Field Extract.
ENCODING BEXTR_rxmxnx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xf7 P66=0 PF2=0 PF3=0 VL=128',
  iflags     => 'af=U cf=U of=U pf=U sf=U zf=W',
  extensions => 'BMI1',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=BMI1',
  tags       => 'page=BEXTR',
};


# BLSI-Extract Lowest Set Isolated Bit.
ENCODING BLSI => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xf3 P66=0 PF2=0 PF3=0 REG=3 VL=128',
  iflags     => 'af=U cf=Z of=Z pf=U sf=W zf=W',
  extensions => 'BMI1',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=BMI1',
  tags       => 'page=BLSI',
};


# BLSMSK-Get Mask Up to Lowest Set Bit.
ENCODING BLSMSK => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xf3 P66=0 PF2=0 PF3=0 REG=2 VL=128',
  iflags     => 'af=U cf=W of=Z pf=U sf=W zf=Z',
  extensions => 'BMI1',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=BMI1',
  tags       => 'page=BLSMSK',
};


# BLSR-Reset Lowest Set Bit.
ENCODING BLSR => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xf3 P66=0 PF2=0 PF3=0 REG=1 VL=128',
  iflags     => 'af=U cf=W of=Z pf=U sf=W zf=W',
  extensions => 'BMI1',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=BMI1',
  tags       => 'page=BLSR',
};


# BOUND-Check Array Index Against Bounds.
ENCODING BOUND => {
  diagram    => 'MOD=MEM MODE=NO64 MR=1 OP=0x62',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I186 deprecated=1',
  tags       => 'page=BOUND',
};


# BSF-Bit Scan Forward.
ENCODING BSF => {
  diagram    => 'MAP=0f MR=1 OP=0xbc',
  iflags     => 'af=U cf=U of=U pf=U sf=U zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I386',
  tags       => 'page=BSF',
};


# BSR-Bit Scan Reverse.
ENCODING BSR => {
  diagram    => 'MAP=0f MR=1 OP=0xbd',
  iflags     => 'af=U cf=U of=U pf=U sf=U zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I386',
  tags       => 'page=BSR',
};


# BSWAP-Byte Swap.
ENCODING BSWAP_ov_1 => {
  diagram    => 'MAP=0f OP=0xc8',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I486REAL',
  tags       => 'page=BSWAP',
};

ENCODING BSWAP_ov_2 => {
  diagram    => 'MAP=0f OP=0xc9',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I486REAL',
  tags       => 'page=BSWAP',
};

ENCODING BSWAP_ov_3 => {
  diagram    => 'MAP=0f OP=0xca',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I486REAL',
  tags       => 'page=BSWAP',
};

ENCODING BSWAP_ov_4 => {
  diagram    => 'MAP=0f OP=0xcb',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I486REAL',
  tags       => 'page=BSWAP',
};

ENCODING BSWAP_ov_5 => {
  diagram    => 'MAP=0f OP=0xcc',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I486REAL',
  tags       => 'page=BSWAP',
};

ENCODING BSWAP_ov_6 => {
  diagram    => 'MAP=0f OP=0xcd',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I486REAL',
  tags       => 'page=BSWAP',
};

ENCODING BSWAP_ov_7 => {
  diagram    => 'MAP=0f OP=0xce',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I486REAL',
  tags       => 'page=BSWAP',
};

ENCODING BSWAP_ov_8 => {
  diagram    => 'MAP=0f OP=0xcf',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I486REAL',
  tags       => 'page=BSWAP',
};


# BT-Bit Test.
ENCODING BT_mvrv => {
  diagram    => 'MAP=0f MR=1 OP=0xa3',
  iflags     => 'af=U cf=W of=U pf=U sf=U',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I386',
  tags       => 'page=BT',
};

ENCODING BT_mvub => {
  diagram    => 'MAP=0f MR=1 OP=0xba REG=4',
  iflags     => 'af=U cf=W of=U pf=U sf=U',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I386',
  tags       => 'page=BT',
};


# BTC-Bit Test and Complement.
ENCODING BTC_mvrv => {
  diagram    => 'MAP=0f MR=1 OP=0xbb',
  iflags     => 'af=U cf=W of=U pf=U sf=U',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I386',
  tags       => 'page=BTC',
};

ENCODING BTC_mvub => {
  diagram    => 'MAP=0f MR=1 OP=0xba REG=7',
  iflags     => 'af=U cf=W of=U pf=U sf=U',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I386',
  tags       => 'page=BTC',
};


# BTR-Bit Test and Reset.
ENCODING BTR_mvrv => {
  diagram    => 'MAP=0f MR=1 OP=0xb3',
  iflags     => 'af=U cf=W of=U pf=U sf=U',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I386',
  tags       => 'page=BTR',
};

ENCODING BTR_mvub => {
  diagram    => 'MAP=0f MR=1 OP=0xba REG=6',
  iflags     => 'af=U cf=W of=U pf=U sf=U',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I386',
  tags       => 'page=BTR',
};


# BTS-Bit Test and Set.
ENCODING BTS_mvrv => {
  diagram    => 'MAP=0f MR=1 OP=0xab',
  iflags     => 'af=U cf=W of=U pf=U sf=U',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I386',
  tags       => 'page=BTS',
};

ENCODING BTS_mvub => {
  diagram    => 'MAP=0f MR=1 OP=0xba REG=5',
  iflags     => 'af=U cf=W of=U pf=U sf=U',
  extensions => 'BASE',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=I386',
  tags       => 'page=BTS',
};


# BZHI-Zero High Bits Starting with Specified Bit Position.
ENCODING BZHI => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xf5 P66=0 PF2=0 PF3=0 VL=128',
  iflags     => 'af=U cf=W of=Z pf=U sf=W zf=W',
  extensions => 'BMI2',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=BMI2',
  tags       => 'page=BZHI',
};


# CALL-Call Procedure.
ENCODING CALL => {
  diagram    => 'MODE=NO64 OP=0x9a',
  extensions => 'BASE',
  categories => 'GENERAL|CALL|FAR',
  metadata   => 'isa=I86',
  docvars    => 'FAR_XFER=1',
  tags       => 'page=CALL',
};

ENCODING CALL_mdpr => {
  diagram    => 'MOD=MEM MR=1 OP=0xff REG=3',
  extensions => 'BASE',
  categories => 'GENERAL|CALL|FAR',
  metadata   => 'isa=I86',
  docvars    => 'FAR_XFER=1',
  tags       => 'page=CALL',
};

ENCODING CALL_mv => {
  diagram    => 'MR=1 OP=0xff REG=2',
  extensions => 'BASE',
  categories => 'GENERAL|CALL|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=CALL',
};

ENCODING CALL_sx => {
  diagram    => 'OP=0xe8',
  extensions => 'BASE',
  categories => 'GENERAL|CALL|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=CALL',
};


# CBW/CWDE/CDQE-Convert Byte to Word/Convert Word to Doubleword/Convert Doubleword to Quadword.
ENCODING CBW => {
  diagram    => 'OP=0x98 OSZ=16',
  extensions => 'BASE',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=I86',
  tags       => 'page=CBWD2WDQ',
};

ENCODING CDQE => {
  diagram    => 'OP=0x98 OSZ=64 W=1',
  extensions => 'LONGMODE',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=LONGMODE',
  tags       => 'page=CBWD2WDQ',
};

ENCODING CWDE => {
  diagram    => 'OP=0x98 OSZ=32',
  extensions => 'BASE',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=I386',
  tags       => 'page=CBWD2WDQ',
};


# CLC-Clear Carry Flag.
ENCODING CLC => {
  diagram    => 'OP=0xf8',
  iflags     => 'cf=Z',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=CLC',
};


# CLD-Clear Direction Flag.
ENCODING CLD => {
  diagram    => 'OP=0xfc',
  iflags     => 'df=Z',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=CLD',
};


# CLI-Clear Interrupt Flag.
ENCODING CLI => {
  diagram    => 'OP=0xfa',
  iflags     => '_if=W iopl=R vif=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=CLI',
};


# CLTS-Clear Task-Switched Flag in CR0.
ENCODING CLTS => {
  diagram    => 'MAP=0f OP=0x06',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I286REAL cpl=RING0',
  tags       => 'page=CLTS',
};


# CMC-Complement Carry Flag.
ENCODING CMC => {
  diagram    => 'OP=0xf5',
  iflags     => 'cf=X',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=CMC',
};


# CMOVcc-Conditional Move.
ENCODING CMOVB => {
  diagram    => 'MAP=0f MR=1 OP=0x42',
  iflags     => 'cf=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=CMOV',
  tags       => 'page=CMOVcc',
};

ENCODING CMOVBE => {
  diagram    => 'MAP=0f MR=1 OP=0x46',
  iflags     => 'cf=R zf=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=CMOV',
  tags       => 'page=CMOVcc',
};

ENCODING CMOVL => {
  diagram    => 'MAP=0f MR=1 OP=0x4c',
  iflags     => 'of=R sf=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=CMOV',
  tags       => 'page=CMOVcc',
};

ENCODING CMOVLE => {
  diagram    => 'MAP=0f MR=1 OP=0x4e',
  iflags     => 'of=R sf=R zf=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=CMOV',
  tags       => 'page=CMOVcc',
};

ENCODING CMOVNB => {
  diagram    => 'MAP=0f MR=1 OP=0x43',
  iflags     => 'cf=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=CMOV',
  tags       => 'page=CMOVcc',
};

ENCODING CMOVNBE => {
  diagram    => 'MAP=0f MR=1 OP=0x47',
  iflags     => 'cf=R zf=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=CMOV',
  tags       => 'page=CMOVcc',
};

ENCODING CMOVNL => {
  diagram    => 'MAP=0f MR=1 OP=0x4d',
  iflags     => 'of=R sf=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=CMOV',
  tags       => 'page=CMOVcc',
};

ENCODING CMOVNLE => {
  diagram    => 'MAP=0f MR=1 OP=0x4f',
  iflags     => 'of=R sf=R zf=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=CMOV',
  tags       => 'page=CMOVcc',
};

ENCODING CMOVNO => {
  diagram    => 'MAP=0f MR=1 OP=0x41',
  iflags     => 'of=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=CMOV',
  tags       => 'page=CMOVcc',
};

ENCODING CMOVNP => {
  diagram    => 'MAP=0f MR=1 OP=0x4b',
  iflags     => 'pf=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=CMOV',
  tags       => 'page=CMOVcc',
};

ENCODING CMOVNS => {
  diagram    => 'MAP=0f MR=1 OP=0x49',
  iflags     => 'sf=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=CMOV',
  tags       => 'page=CMOVcc',
};

ENCODING CMOVNZ => {
  diagram    => 'MAP=0f MR=1 OP=0x45',
  iflags     => 'zf=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=CMOV',
  tags       => 'page=CMOVcc',
};

ENCODING CMOVO => {
  diagram    => 'MAP=0f MR=1 OP=0x40',
  iflags     => 'of=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=CMOV',
  tags       => 'page=CMOVcc',
};

ENCODING CMOVP => {
  diagram    => 'MAP=0f MR=1 OP=0x4a',
  iflags     => 'pf=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=CMOV',
  tags       => 'page=CMOVcc',
};

ENCODING CMOVS => {
  diagram    => 'MAP=0f MR=1 OP=0x48',
  iflags     => 'sf=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=CMOV',
  tags       => 'page=CMOVcc',
};

ENCODING CMOVZ => {
  diagram    => 'MAP=0f MR=1 OP=0x44',
  iflags     => 'zf=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=CMOV',
  tags       => 'page=CMOVcc',
};


# CMP-Compare Two Operands.
ENCODING CMP_bsb => {
  diagram    => 'OP=0x3c',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=CMP',
};

ENCODING CMP_mbrb => {
  diagram    => 'MR=1 OP=0x38',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=CMP',
};

ENCODING CMP_mbsb_1 => {
  diagram    => 'MR=1 OP=0x80 REG=7',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=CMP',
};

ENCODING CMP_mbsb_2 => {
  diagram    => 'MODE=NO64 MR=1 OP=0x82 REG=7',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=CMP',
};

ENCODING CMP_mvrv => {
  diagram    => 'MR=1 OP=0x39',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=CMP',
};

ENCODING CMP_mvsb => {
  diagram    => 'MR=1 OP=0x83 REG=7',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=CMP',
};

ENCODING CMP_mvsz => {
  diagram    => 'MR=1 OP=0x81 REG=7',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=CMP',
};

ENCODING CMP_rbmb => {
  diagram    => 'MR=1 OP=0x3a',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=CMP',
};

ENCODING CMP_rvmv => {
  diagram    => 'MR=1 OP=0x3b',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=CMP',
};

ENCODING CMP_vsz => {
  diagram    => 'OP=0x3d',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=CMP',
};


# CMPS/CMPSB/CMPSW/CMPSD/CMPSQ-Compare String Operands.
ENCODING CMPSB_1 => {
  diagram    => 'OP=0xa6 PF2=0 PF3=0',
  iflags     => 'af=W cf=W df=R of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I86',
  tags       => 'page=CMPS',
};

ENCODING CMPSB_2 => {
  diagram    => 'OP=0xa6 P66=0',
  iflags     => 'af=CW cf=CW df=R of=CW pf=CW sf=CW zf=RCW',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I86',
  tags       => 'page=CMPS',
};

ENCODING CMPSD_1 => {
  diagram    => 'OP=0xa7 OSZ=32 PF2=0 PF3=0',
  iflags     => 'af=W cf=W df=R of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I386',
  tags       => 'page=CMPS',
};

ENCODING CMPSD_2 => {
  diagram    => 'OP=0xa7 OSZ=32 P66=0',
  iflags     => 'af=CW cf=CW df=R of=CW pf=CW sf=CW zf=RCW',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I386',
  tags       => 'page=CMPS',
};

ENCODING CMPSQ_1 => {
  diagram    => 'OP=0xa7 OSZ=64 PF2=0 PF3=0',
  iflags     => 'af=W cf=W df=R of=W pf=W sf=W zf=W',
  extensions => 'LONGMODE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=LONGMODE',
  tags       => 'page=CMPS',
};

ENCODING CMPSQ_2 => {
  diagram    => 'OP=0xa7 OSZ=64 P66=0',
  iflags     => 'af=CW cf=CW df=R of=CW pf=CW sf=CW zf=RCW',
  extensions => 'LONGMODE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=LONGMODE',
  tags       => 'page=CMPS',
};

ENCODING CMPSW_1 => {
  diagram    => 'OP=0xa7 OSZ=16 PF2=0 PF3=0',
  iflags     => 'af=W cf=W df=R of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I86',
  tags       => 'page=CMPS',
};

ENCODING CMPSW_2 => {
  diagram    => 'OP=0xa7 OSZ=16 P66=0',
  iflags     => 'af=CW cf=CW df=R of=CW pf=CW sf=CW zf=RCW',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I86',
  tags       => 'page=CMPS',
};


# CMPXCHG-Compare and Exchange.
ENCODING CMPXCHG_mbrb => {
  diagram    => 'MAP=0f MR=1 OP=0xb0',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I486REAL',
  tags       => 'page=CMPXCHG',
};

ENCODING CMPXCHG_mvrv => {
  diagram    => 'MAP=0f MR=1 OP=0xb1',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I486REAL',
  tags       => 'page=CMPXCHG',
};


# CMPXCHG8B/CMPXCHG16B-Compare and Exchange Bytes.
ENCODING CMPXCHG16B => {
  diagram    => 'MAP=0f MOD=MEM MODE=64 MR=1 OP=0xc7 REG=1 W=1',
  iflags     => 'zf=W',
  extensions => 'LONGMODE',
  exceptions => 'ALIGNMENT',
  categories => 'GENERAL',
  metadata   => 'isa=CMPXCHG16B requires_alignment=1',
  tags       => 'page=CMPXCHGxx',
};

ENCODING CMPXCHG8B => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xc7 REG=1',
  iflags     => 'zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=PENTIUMREAL',
  tags       => 'page=CMPXCHGxx',
};


# CPUID-CPU Identification.
ENCODING CPUID => {
  diagram    => 'MAP=0f OP=0xa2',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I486REAL',
  tags       => 'page=CPUID',
};


# CWD/CDQ/CQO-Convert Word to Doubleword/Convert Doubleword to Quadword.
ENCODING CDQ => {
  diagram    => 'OP=0x99 OSZ=32',
  extensions => 'BASE',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=I386',
  tags       => 'page=CWD2DQ',
};

ENCODING CQO => {
  diagram    => 'OP=0x99 OSZ=64 W=1',
  extensions => 'LONGMODE',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=LONGMODE',
  tags       => 'page=CWD2DQ',
};

ENCODING CWD => {
  diagram    => 'OP=0x99 OSZ=16',
  extensions => 'BASE',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=I86',
  tags       => 'page=CWD2DQ',
};


# DAA-Decimal Adjust AL after Addition.
ENCODING DAA => {
  diagram    => 'MODE=NO64 OP=0x27',
  iflags     => 'af=X cf=X of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|DECIMAL',
  metadata   => 'isa=I86 deprecated=1',
  tags       => 'page=DAA',
};


# DAS-Decimal Adjust AL after Subtraction.
ENCODING DAS => {
  diagram    => 'MODE=NO64 OP=0x2f',
  iflags     => 'af=X cf=X of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|DECIMAL',
  metadata   => 'isa=I86 deprecated=1',
  tags       => 'page=DAS',
};


# DEC-Decrement by 1.
ENCODING DEC_mb => {
  diagram    => 'MR=1 OP=0xfe REG=1',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=DEC',
};

ENCODING DEC_mv => {
  diagram    => 'MR=1 OP=0xff REG=1',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=DEC',
};

ENCODING DEC_ov_1 => {
  diagram    => 'MODE=NO64 OP=0x48',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=DEC',
};

ENCODING DEC_ov_2 => {
  diagram    => 'MODE=NO64 OP=0x49',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=DEC',
};

ENCODING DEC_ov_3 => {
  diagram    => 'MODE=NO64 OP=0x4a',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=DEC',
};

ENCODING DEC_ov_4 => {
  diagram    => 'MODE=NO64 OP=0x4b',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=DEC',
};

ENCODING DEC_ov_5 => {
  diagram    => 'MODE=NO64 OP=0x4c',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=DEC',
};

ENCODING DEC_ov_6 => {
  diagram    => 'MODE=NO64 OP=0x4d',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=DEC',
};

ENCODING DEC_ov_7 => {
  diagram    => 'MODE=NO64 OP=0x4e',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=DEC',
};

ENCODING DEC_ov_8 => {
  diagram    => 'MODE=NO64 OP=0x4f',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=DEC',
};


# DIV-Unsigned Divide.
ENCODING DIV_mb => {
  diagram    => 'MR=1 OP=0xf6 REG=6',
  iflags     => 'af=U cf=U of=U pf=U sf=U zf=U',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=DIV',
};

ENCODING DIV_mv => {
  diagram    => 'MR=1 OP=0xf7 REG=6',
  iflags     => 'af=U cf=U of=U pf=U sf=U zf=U',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=DIV',
};


# ENTER-Make Stack Frame for Procedure Parameters.
ENCODING ENTER => {
  diagram    => 'OP=0xc8',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I186',
  tags       => 'page=ENTER',
};


# HLT-Halt.
ENCODING HLT => {
  diagram    => 'OP=0xf4',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86 cpl=RING0',
  tags       => 'page=HLT',
};


# IDIV-Signed Divide.
ENCODING IDIV_mb => {
  diagram    => 'MR=1 OP=0xf6 REG=7',
  iflags     => 'af=U cf=U of=U pf=U sf=U zf=U',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=IDIV',
};

ENCODING IDIV_mv => {
  diagram    => 'MR=1 OP=0xf7 REG=7',
  iflags     => 'af=U cf=U of=U pf=U sf=U zf=U',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=IDIV',
};


# IMUL-Signed Multiply.
ENCODING IMUL_mb => {
  diagram    => 'MR=1 OP=0xf6 REG=5',
  iflags     => 'af=U cf=W of=W pf=U sf=U zf=U',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=IMUL',
};

ENCODING IMUL_mv => {
  diagram    => 'MR=1 OP=0xf7 REG=5',
  iflags     => 'af=U cf=W of=W pf=U sf=U zf=U',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=IMUL',
};

ENCODING IMUL_rvmv => {
  diagram    => 'MAP=0f MR=1 OP=0xaf',
  iflags     => 'af=U cf=W of=W pf=U sf=U zf=U',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=IMUL',
};

ENCODING IMUL_rvmvsb => {
  diagram    => 'MR=1 OP=0x6b',
  iflags     => 'af=U cf=W of=W pf=U sf=U zf=U',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I186',
  tags       => 'page=IMUL',
};

ENCODING IMUL_rvmvsz => {
  diagram    => 'MR=1 OP=0x69',
  iflags     => 'af=U cf=W of=W pf=U sf=U zf=U',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I186',
  tags       => 'page=IMUL',
};


# IN-Input from Port.
ENCODING IN_bub => {
  diagram    => 'OP=0xe4',
  iflags     => 'iopl=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=IN',
};

ENCODING IN_bw => {
  diagram    => 'OP=0xec',
  iflags     => 'iopl=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=IN',
};

ENCODING IN_x => {
  diagram    => 'OP=0xed',
  iflags     => 'iopl=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=IN',
};

ENCODING IN_xub => {
  diagram    => 'OP=0xe5',
  iflags     => 'iopl=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=IN',
};


# INC-Increment by 1.
ENCODING INC_mb => {
  diagram    => 'MR=1 OP=0xfe REG=0',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=INC',
};

ENCODING INC_mv => {
  diagram    => 'MR=1 OP=0xff REG=0',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=INC',
};

ENCODING INC_ov_1 => {
  diagram    => 'MODE=NO64 OP=0x40',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=INC',
};

ENCODING INC_ov_2 => {
  diagram    => 'MODE=NO64 OP=0x41',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=INC',
};

ENCODING INC_ov_3 => {
  diagram    => 'MODE=NO64 OP=0x42',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=INC',
};

ENCODING INC_ov_4 => {
  diagram    => 'MODE=NO64 OP=0x43',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=INC',
};

ENCODING INC_ov_5 => {
  diagram    => 'MODE=NO64 OP=0x44',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=INC',
};

ENCODING INC_ov_6 => {
  diagram    => 'MODE=NO64 OP=0x45',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=INC',
};

ENCODING INC_ov_7 => {
  diagram    => 'MODE=NO64 OP=0x46',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=INC',
};

ENCODING INC_ov_8 => {
  diagram    => 'MODE=NO64 OP=0x47',
  iflags     => 'af=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=INC',
};


# INS/INSB/INSW/INSD-Input from Port to String.
ENCODING INSB => {
  diagram    => 'OP=0x6c',
  iflags     => 'df=R iopl=R',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I186',
  tags       => 'page=INS',
};

ENCODING INSD => {
  diagram    => 'OP=0x6d',
  iflags     => 'df=R iopl=R',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I386',
  tags       => 'page=INS',
};

ENCODING INSW => {
  diagram    => 'OP=0x6d OSZ=16',
  iflags     => 'df=R iopl=R',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I186',
  tags       => 'page=INS',
};


# INT-n/INTO/INT3/INT1-Call to Interrupt Procedure.
ENCODING INT => {
  diagram    => 'OP=0xcd',
  iflags     => 'ac=W _if=W iopl=R nt=W rf=Z tf=Z vif=W vm=X',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=INT',
};

ENCODING INT1 => {
  diagram    => 'OP=0xf1',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=INT',
};

ENCODING INT3 => {
  diagram    => 'OP=0xcc',
  iflags     => 'ac=W _if=W iopl=R nt=W rf=Z tf=Z vif=W vm=X',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=INT',
};

ENCODING INTO => {
  diagram    => 'MODE=NO64 OP=0xce',
  iflags     => 'ac=W _if=W iopl=R nt=W of=R rf=Z tf=W vm=X',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86 deprecated=1',
  tags       => 'page=INT',
};


# INVD-Invalidate Internal Caches.
ENCODING INVD => {
  diagram    => 'MAP=0f OP=0x08',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I486REAL cpl=RING0',
  tags       => 'page=INVD',
};


# INVLPG-Invalidate TLB Entries.
ENCODING INVLPG => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x01 REG=7',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I486REAL cpl=RING0',
  tags       => 'page=INVLPG',
};


# IRET/IRETD/IRETQ-Interrupt Return.
ENCODING IRET => {
  diagram    => 'OP=0xcf OSZ=16',
  iflags     => 'ac=W af=W cf=W df=W id=W _if=W iopl=X nt=X of=W pf=W rf=W sf=W tf=W vif=W vip=W vm=X zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|RET',
  metadata   => 'isa=I86',
  docvars    => 'NOTSX=1',
  tags       => 'page=IRET',
};

ENCODING IRETD => {
  diagram    => 'OP=0xcf OSZ=32',
  iflags     => 'ac=W af=W cf=W df=W id=W _if=W iopl=X nt=X of=W pf=W rf=W sf=W tf=W vif=W vip=W vm=X zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|RET',
  metadata   => 'isa=I386',
  docvars    => 'NOTSX=1',
  tags       => 'page=IRET',
};

ENCODING IRETQ => {
  diagram    => 'OP=0xcf OSZ=64',
  iflags     => 'ac=W af=W cf=W df=W id=W _if=W iopl=X nt=X of=W pf=W rf=W sf=W tf=W vif=W vip=W vm=X zf=W',
  extensions => 'LONGMODE',
  categories => 'GENERAL|RET',
  metadata   => 'isa=LONGMODE',
  docvars    => 'NOTSX=1',
  tags       => 'page=IRET',
};


# JMP-Jump.
ENCODING JMP => {
  diagram    => 'MODE=NO64 OP=0xea',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|UNCONDITIONALLY|FAR',
  metadata   => 'isa=I86',
  docvars    => 'FAR_XFER=1',
  tags       => 'page=JMP',
};

ENCODING JMP_mdpr => {
  diagram    => 'MOD=MEM MR=1 OP=0xff REG=5',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|UNCONDITIONALLY|FAR',
  metadata   => 'isa=I86',
  docvars    => 'FAR_XFER=1',
  tags       => 'page=JMP',
};

ENCODING JMP_mv => {
  diagram    => 'MR=1 OP=0xff REG=4',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|UNCONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=JMP',
};

ENCODING JMP_s8 => {
  diagram    => 'OP=0xeb',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|UNCONDITIONALLY|SHORT',
  metadata   => 'isa=I86',
  tags       => 'page=JMP',
};

ENCODING JMP_sx => {
  diagram    => 'OP=0xe9',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|UNCONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=JMP',
};


# Jcc-Jump if Condition Is Met.
ENCODING JBE_s8 => {
  diagram    => 'OP=0x76',
  iflags     => 'cf=R zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JBE_sx => {
  diagram    => 'MAP=0f MODE=64 OP=0x86',
  iflags     => 'cf=R zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JBE_sz => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x86',
  iflags     => 'cf=R zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'SCALABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JB_s8 => {
  diagram    => 'OP=0x72',
  iflags     => 'cf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JB_sx => {
  diagram    => 'MAP=0f MODE=64 OP=0x82',
  iflags     => 'cf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JB_sz => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x82',
  iflags     => 'cf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'SCALABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JCXZ => {
  diagram    => 'ASZ=16 OP=0xe3',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I386',
  tags       => 'page=Jcc',
};

ENCODING JECXZ => {
  diagram    => 'ASZ=32 OP=0xe3',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I386',
  tags       => 'page=Jcc',
};

ENCODING JLE_s8 => {
  diagram    => 'OP=0x7e',
  iflags     => 'of=R sf=R zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JLE_sx => {
  diagram    => 'MAP=0f MODE=64 OP=0x8e',
  iflags     => 'of=R sf=R zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JLE_sz => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x8e',
  iflags     => 'of=R sf=R zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'SCALABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JL_s8 => {
  diagram    => 'OP=0x7c',
  iflags     => 'of=R sf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JL_sx => {
  diagram    => 'MAP=0f MODE=64 OP=0x8c',
  iflags     => 'of=R sf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JL_sz => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x8c',
  iflags     => 'of=R sf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'SCALABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNBE_s8 => {
  diagram    => 'OP=0x77',
  iflags     => 'cf=R zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNBE_sx => {
  diagram    => 'MAP=0f MODE=64 OP=0x87',
  iflags     => 'cf=R zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNBE_sz => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x87',
  iflags     => 'cf=R zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'SCALABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNB_s8 => {
  diagram    => 'OP=0x73',
  iflags     => 'cf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNB_sx => {
  diagram    => 'MAP=0f MODE=64 OP=0x83',
  iflags     => 'cf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNB_sz => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x83',
  iflags     => 'cf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'SCALABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNLE_s8 => {
  diagram    => 'OP=0x7f',
  iflags     => 'of=R sf=R zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNLE_sx => {
  diagram    => 'MAP=0f MODE=64 OP=0x8f',
  iflags     => 'of=R sf=R zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNLE_sz => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x8f',
  iflags     => 'of=R sf=R zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'SCALABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNL_s8 => {
  diagram    => 'OP=0x7d',
  iflags     => 'of=R sf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNL_sx => {
  diagram    => 'MAP=0f MODE=64 OP=0x8d',
  iflags     => 'of=R sf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNL_sz => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x8d',
  iflags     => 'of=R sf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'SCALABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNO_s8 => {
  diagram    => 'OP=0x71',
  iflags     => 'of=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNO_sx => {
  diagram    => 'MAP=0f MODE=64 OP=0x81',
  iflags     => 'of=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNO_sz => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x81',
  iflags     => 'of=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'SCALABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNP_s8 => {
  diagram    => 'OP=0x7b',
  iflags     => 'pf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNP_sx => {
  diagram    => 'MAP=0f MODE=64 OP=0x8b',
  iflags     => 'pf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNP_sz => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x8b',
  iflags     => 'pf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'SCALABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNS_s8 => {
  diagram    => 'OP=0x79',
  iflags     => 'sf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNS_sx => {
  diagram    => 'MAP=0f MODE=64 OP=0x89',
  iflags     => 'sf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNS_sz => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x89',
  iflags     => 'sf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'SCALABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNZ_s8 => {
  diagram    => 'OP=0x75',
  iflags     => 'zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNZ_sx => {
  diagram    => 'MAP=0f MODE=64 OP=0x85',
  iflags     => 'zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JNZ_sz => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x85',
  iflags     => 'zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'SCALABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JO_s8 => {
  diagram    => 'OP=0x70',
  iflags     => 'of=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JO_sx => {
  diagram    => 'MAP=0f MODE=64 OP=0x80',
  iflags     => 'of=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JO_sz => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x80',
  iflags     => 'of=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'SCALABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JP_s8 => {
  diagram    => 'OP=0x7a',
  iflags     => 'pf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JP_sx => {
  diagram    => 'MAP=0f MODE=64 OP=0x8a',
  iflags     => 'pf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JP_sz => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x8a',
  iflags     => 'pf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'SCALABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JRCXZ => {
  diagram    => 'ASZ=64 OP=0xe3',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=LONGMODE',
  tags       => 'page=Jcc',
};

ENCODING JS_s8 => {
  diagram    => 'OP=0x78',
  iflags     => 'sf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JS_sx => {
  diagram    => 'MAP=0f MODE=64 OP=0x88',
  iflags     => 'sf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JS_sz => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x88',
  iflags     => 'sf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'SCALABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JZ_s8 => {
  diagram    => 'OP=0x74',
  iflags     => 'zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JZ_sx => {
  diagram    => 'MAP=0f MODE=64 OP=0x84',
  iflags     => 'zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=Jcc',
};

ENCODING JZ_sz => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x84',
  iflags     => 'zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'SCALABLE=1',
  tags       => 'page=Jcc',
};


# LAHF-Load Status Flags into AH Register.
ENCODING LAHF => {
  diagram    => 'OP=0x9f',
  iflags     => 'af=R cf=R pf=R sf=R zf=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=LAHF',
  tags       => 'page=LAHF',
};


# LAR-Load Access Rights Byte.
ENCODING LAR => {
  diagram    => 'MAP=0f MR=1 OP=0x02',
  iflags     => 'zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I286PROTECTED',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=LAR',
};


# LEA-Load Effective Address.
ENCODING LEA => {
  diagram    => 'MOD=MEM MR=1 OP=0x8d',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=LEA',
};


# LEAVE-High Level Procedure Exit.
ENCODING LEAVE => {
  diagram    => 'OP=0xc9',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I186',
  tags       => 'page=LEAVE',
};


# LLDT-Load Local Descriptor Table Register.
ENCODING LLDT => {
  diagram    => 'MAP=0f MR=1 OP=0x00 REG=2',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I286PROTECTED cpl=RING0',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=LLDT',
};


# LMSW-Load Machine Status Word.
ENCODING LMSW => {
  diagram    => 'MAP=0f MR=1 OP=0x01 REG=6',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I286REAL cpl=RING0',
  tags       => 'page=LMSW',
};


# LODS/LODSB/LODSW/LODSD/LODSQ-Load String.
ENCODING LODSB => {
  diagram    => 'OP=0xac',
  iflags     => 'df=R',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I86',
  tags       => 'page=LODS',
};

ENCODING LODSD => {
  diagram    => 'OP=0xad OSZ=32',
  iflags     => 'df=R',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I386',
  tags       => 'page=LODS',
};

ENCODING LODSQ => {
  diagram    => 'OP=0xad OSZ=64',
  iflags     => 'df=R',
  extensions => 'LONGMODE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=LONGMODE',
  tags       => 'page=LODS',
};

ENCODING LODSW => {
  diagram    => 'OP=0xad OSZ=16',
  iflags     => 'df=R',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I86',
  tags       => 'page=LODS',
};


# LOOP/LOOPcc-Loop According to ECX Counter.
ENCODING LOOP => {
  diagram    => 'OP=0xe2',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY',
  metadata   => 'isa=I86',
  tags       => 'page=LOOP',
};

ENCODING LOOPE => {
  diagram    => 'OP=0xe1',
  iflags     => 'zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY',
  metadata   => 'isa=I86',
  tags       => 'page=LOOP',
};

ENCODING LOOPNE => {
  diagram    => 'OP=0xe0',
  iflags     => 'zf=R',
  extensions => 'BASE',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY',
  metadata   => 'isa=I86',
  tags       => 'page=LOOP',
};


# LSL-Load Segment Limit.
ENCODING LSL => {
  diagram    => 'MAP=0f MR=1 OP=0x03',
  iflags     => 'zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I286PROTECTED',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=LSL',
};


# LTR-Load Task Register.
ENCODING LTR => {
  diagram    => 'MAP=0f MR=1 OP=0x00 REG=3',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I286PROTECTED cpl=RING0',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=LTR',
};


# LZCNT-Count the Number of Leading Zero Bits.
ENCODING LZCNT_rvmv => {
  diagram    => 'BMI=1 MAP=0f MR=1 OP=0xbd P66=0 PF2=0 PF3=1',
  iflags     => 'af=U cf=W of=U pf=U sf=U zf=W',
  extensions => 'LZCNT',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=LZCNT',
  tags       => 'page=LZCNT',
};


# LGDT/LIDT-Load Global/Interrupt Descriptor Table Register.
ENCODING LGDT => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x01 REG=2',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I286REAL',
  tags       => 'page=LxDT',
};

ENCODING LIDT => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x01 REG=3',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I286REAL cpl=RING0',
  tags       => 'page=LxDT',
};


# LDS/LES/LFS/LGS/LSS-Load Far Pointer.
ENCODING LDS => {
  diagram    => 'MOD=MEM MODE=NO64 MR=1 OP=0xc5',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=LxS',
};

ENCODING LES => {
  diagram    => 'MOD=MEM MODE=NO64 MR=1 OP=0xc4',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=LxS',
};

ENCODING LFS => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xb4',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I386',
  tags       => 'page=LxS',
};

ENCODING LGS => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xb5',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I386',
  tags       => 'page=LxS',
};

ENCODING LSS => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xb2',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I386',
  tags       => 'page=LxS',
};


# MOV-Move.
ENCODING MOV_bs8 => {
  diagram    => 'OP=0xa0',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_mbrb => {
  diagram    => 'MR=1 OP=0x88',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_mbub => {
  diagram    => 'MR=1 OP=0xc6 REG=0',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_mvrv => {
  diagram    => 'MR=1 OP=0x89',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_mvsz => {
  diagram    => 'MR=1 OP=0xc7 REG=0',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_mxrw => {
  diagram    => 'MR=1 OP=0x8c',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_obub_1 => {
  diagram    => 'OP=0xb0',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_obub_2 => {
  diagram    => 'OP=0xb1',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_obub_3 => {
  diagram    => 'OP=0xb2',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_obub_4 => {
  diagram    => 'OP=0xb3',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_obub_5 => {
  diagram    => 'OP=0xb4',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_obub_6 => {
  diagram    => 'OP=0xb5',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_obub_7 => {
  diagram    => 'OP=0xb6',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_obub_8 => {
  diagram    => 'OP=0xb7',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_ovuv_1 => {
  diagram    => 'OP=0xb8',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_ovuv_2 => {
  diagram    => 'OP=0xb9',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_ovuv_3 => {
  diagram    => 'OP=0xba',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_ovuv_4 => {
  diagram    => 'OP=0xbb',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_ovuv_5 => {
  diagram    => 'OP=0xbc',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_ovuv_6 => {
  diagram    => 'OP=0xbd',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_ovuv_7 => {
  diagram    => 'OP=0xbe',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_ovuv_8 => {
  diagram    => 'OP=0xbf',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_rbmb => {
  diagram    => 'MR=1 OP=0x8a',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_rvmv => {
  diagram    => 'MR=1 OP=0x8b',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_rwmw => {
  diagram    => 'MR=1 OP=0x8e',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_s8b => {
  diagram    => 'OP=0xa2',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_svv => {
  diagram    => 'OP=0xa3',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};

ENCODING MOV_vsv => {
  diagram    => 'OP=0xa1',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=MOV',
};


# MOVBE-Move Data After Swapping Bytes.
ENCODING MOVBE_mvrv => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0xf1 PF2=0 PF3=0',
  extensions => 'MOVBE',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
  metadata   => 'isa=MOVBE',
  tags       => 'page=MOVBE',
};

ENCODING MOVBE_rvmv => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0xf0 PF2=0 PF3=0',
  extensions => 'MOVBE',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
  metadata   => 'isa=MOVBE',
  tags       => 'page=MOVBE',
};


# MOVDIR64B-Move 64 Bytes as Direct Store.
ENCODING MOVDIR64B => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0xf8 P66=1 PF2=0 PF3=0',
  extensions => 'MOVDIR',
  exceptions => 'ALIGNMENT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
  metadata   => 'isa=MOVDIR requires_alignment=1',
  tags       => 'page=MOVDIR64B',
};


# MOVDIRI-Move Doubleword as Direct Store.
ENCODING MOVDIRI => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0xf9 P66=0 PF2=0 PF3=0',
  extensions => 'MOVDIR',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
  metadata   => 'isa=MOVDIR',
  tags       => 'page=MOVDIRI',
};


# MOVS/MOVSB/MOVSW/MOVSD/MOVSQ-Move Data from String to String.
ENCODING MOVSB => {
  diagram    => 'OP=0xa4',
  iflags     => 'df=R',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I86',
  tags       => 'page=MOVS',
};

ENCODING MOVSD => {
  diagram    => 'OP=0xa5 OSZ=32',
  iflags     => 'df=R',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I386',
  tags       => 'page=MOVS',
};

ENCODING MOVSQ => {
  diagram    => 'OP=0xa5 OSZ=64',
  iflags     => 'df=R',
  extensions => 'LONGMODE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=LONGMODE',
  tags       => 'page=MOVS',
};

ENCODING MOVSW => {
  diagram    => 'OP=0xa5 OSZ=16',
  iflags     => 'df=R',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I86',
  tags       => 'page=MOVS',
};


# MOVSX/MOVSXD-Move with Sign-Extension.
ENCODING MOVSXD => {
  diagram    => 'MODE=64 MR=1 OP=0x63',
  extensions => 'LONGMODE',
  categories => 'GENERAL',
  metadata   => 'isa=LONGMODE',
  tags       => 'page=MOVSX',
};

ENCODING MOVSX_rvmb => {
  diagram    => 'MAP=0f MR=1 OP=0xbe',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I386',
  tags       => 'page=MOVSX',
};

ENCODING MOVSX_rvmw => {
  diagram    => 'MAP=0f MR=1 OP=0xbf',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I386',
  tags       => 'page=MOVSX',
};


# MOVZX-Move with Zero-Extend.
ENCODING MOVZX_rvmb => {
  diagram    => 'MAP=0f MR=1 OP=0xb6',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I386',
  tags       => 'page=MOVZX',
};

ENCODING MOVZX_rvmw => {
  diagram    => 'MAP=0f MR=1 OP=0xb7',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I386',
  tags       => 'page=MOVZX',
};


# MOV-Move to/from Control Registers.
ENCODING MOV_mxrq_1 => {
  diagram    => 'MAP=0f MR=1 OP=0x20',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86 cpl=RING0',
  docvars    => 'RING0=1',
  tags       => 'page=MOVcntrl',
};

ENCODING MOV_rqmx_1 => {
  diagram    => 'MAP=0f MR=1 OP=0x22',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86 cpl=RING0',
  docvars    => 'RING0=1',
  tags       => 'page=MOVcntrl',
};


# MOV-Move to/from Debug Registers.
ENCODING MOV_mxrq_2 => {
  diagram    => 'MAP=0f MR=1 OP=0x21',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86 cpl=RING0',
  docvars    => 'RING0=1',
  tags       => 'page=MOVdbg',
};

ENCODING MOV_rqmx_2 => {
  diagram    => 'MAP=0f MR=1 OP=0x23',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86 cpl=RING0',
  docvars    => 'RING0=1',
  tags       => 'page=MOVdbg',
};


# MUL-Unsigned Multiply.
ENCODING MUL_mb => {
  diagram    => 'MR=1 OP=0xf6 REG=4',
  iflags     => 'af=U cf=W of=W pf=U sf=U zf=U',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=MUL',
};

ENCODING MUL_mv => {
  diagram    => 'MR=1 OP=0xf7 REG=4',
  iflags     => 'af=U cf=W of=W pf=U sf=U zf=U',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=MUL',
};


# MULX-Unsigned Multiply Without Affecting Flags.
ENCODING MULX => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xf6 P66=0 PF2=1 PF3=0 VL=128',
  extensions => 'BMI2',
  categories => 'GENERAL|ARITHMETIC',
  metadata   => 'isa=BMI2',
  tags       => 'page=MULX',
};


# NEG-Two's Complement Negation.
ENCODING NEG_mb => {
  diagram    => 'MR=1 OP=0xf6 REG=3',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=NEG',
};

ENCODING NEG_mv => {
  diagram    => 'MR=1 OP=0xf7 REG=3',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=NEG',
};


# NOP-No Operation.
ENCODING NOP => {
  diagram    => 'B=0 OP=0x90 PF3=0',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=I86',
  tags       => 'page=NOP',
};

ENCODING NOP_mv_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x18 REG=0',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=FAT_NOP',
  tags       => 'page=NOP',
};

ENCODING NOP_mv_10 => {
  diagram    => 'MAP=0f MOD=MEM MPX=0 MR=1 OP=0x1b',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=PPRO',
  tags       => 'page=NOP',
};

ENCODING NOP_mv_2 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x18 REG=1',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=FAT_NOP',
  tags       => 'page=NOP',
};

ENCODING NOP_mv_3 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x18 REG=2',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=FAT_NOP',
  tags       => 'page=NOP',
};

ENCODING NOP_mv_4 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x18 REG=3',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=FAT_NOP',
  tags       => 'page=NOP',
};

ENCODING NOP_mv_5 => {
  diagram    => 'MAP=0f MR=1 OP=0x18 REG=4',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=FAT_NOP',
  tags       => 'page=NOP',
};

ENCODING NOP_mv_6 => {
  diagram    => 'MAP=0f MR=1 OP=0x18 REG=5',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=FAT_NOP',
  tags       => 'page=NOP',
};

ENCODING NOP_mv_7 => {
  diagram    => 'MAP=0f MR=1 OP=0x18 REG=6',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=FAT_NOP',
  tags       => 'page=NOP',
};

ENCODING NOP_mv_8 => {
  diagram    => 'MAP=0f MR=1 OP=0x18 REG=7',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=FAT_NOP',
  tags       => 'page=NOP',
};

ENCODING NOP_mv_9 => {
  diagram    => 'MAP=0f MOD=MEM MPX=0 MR=1 OP=0x1a',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=PPRO',
  tags       => 'page=NOP',
};

ENCODING NOP_mvrv_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x0d',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=PREFETCH_NOP',
  tags       => 'page=NOP',
};

ENCODING NOP_mvrv_10 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x1e PF2=0 PF3=1 REG=3',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=PPRO',
  tags       => 'page=NOP',
};

ENCODING NOP_mvrv_11 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x1e PF2=0 PF3=1 REG=4',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=PPRO',
  tags       => 'page=NOP',
};

ENCODING NOP_mvrv_12 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x1e PF2=0 PF3=1 REG=5',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=PPRO',
  tags       => 'page=NOP',
};

ENCODING NOP_mvrv_13 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x1e PF2=0 PF3=1 REG=6',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=PPRO',
  tags       => 'page=NOP',
};

ENCODING NOP_mvrv_14 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x1e PF2=0 PF3=1 REG=7',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=PPRO',
  tags       => 'page=NOP',
};

ENCODING NOP_mvrv_15 => {
  diagram    => 'CET=0 MAP=0f MOD=REG MR=1 OP=0x1e PF2=0 PF3=1 REG=1',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=PPRO',
  tags       => 'page=NOP',
};

ENCODING NOP_mvrv_16 => {
  diagram    => 'KNC=0 MAP=0f MR=1 OP=0x1f REG=0',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=FAT_NOP',
  tags       => 'page=NOP',
};

ENCODING NOP_mvrv_17 => {
  diagram    => 'MAP=0f MR=1 OP=0x1f REG=NNN',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=FAT_NOP',
  tags       => 'page=NOP',
};

ENCODING NOP_mvrv_2 => {
  diagram    => 'MAP=0f MR=1 OP=0x19',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=FAT_NOP',
  tags       => 'page=NOP',
};

ENCODING NOP_mvrv_3 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x1a',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=PPRO',
  tags       => 'page=NOP',
};

ENCODING NOP_mvrv_4 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x1b',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=PPRO',
  tags       => 'page=NOP',
};

ENCODING NOP_mvrv_5 => {
  diagram    => 'MAP=0f MR=1 OP=0x1c',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=FAT_NOP',
  tags       => 'page=NOP',
};

ENCODING NOP_mvrv_6 => {
  diagram    => 'MAP=0f MR=1 OP=0x1d',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=FAT_NOP',
  tags       => 'page=NOP',
};

ENCODING NOP_mvrv_7 => {
  diagram    => 'MAP=0f MR=1 OP=0x1e',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=PPRO',
  tags       => 'page=NOP',
};

ENCODING NOP_mvrv_8 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x1e PF2=0 PF3=1 REG=0',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=PPRO',
  tags       => 'page=NOP',
};

ENCODING NOP_mvrv_9 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x1e PF2=0 PF3=1 REG=2',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=PPRO',
  tags       => 'page=NOP',
};


# NOT-One's Complement Negation.
ENCODING NOT_mb => {
  diagram    => 'MR=1 OP=0xf6 REG=2',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=NOT',
};

ENCODING NOT_mv => {
  diagram    => 'MR=1 OP=0xf7 REG=2',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=NOT',
};


# OR-Logical Inclusive OR.
ENCODING OR_bub => {
  diagram    => 'OP=0x0c',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=OR',
};

ENCODING OR_mbrb => {
  diagram    => 'MR=1 OP=0x08',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=OR',
};

ENCODING OR_mbsb_1 => {
  diagram    => 'MR=1 OP=0x80 REG=1',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=OR',
};

ENCODING OR_mbsb_2 => {
  diagram    => 'MODE=NO64 MR=1 OP=0x82 REG=1',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=OR',
};

ENCODING OR_mvrv => {
  diagram    => 'MR=1 OP=0x09',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=OR',
};

ENCODING OR_mvsb => {
  diagram    => 'MR=1 OP=0x83 REG=1',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=OR',
};

ENCODING OR_mvsz => {
  diagram    => 'MR=1 OP=0x81 REG=1',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=OR',
};

ENCODING OR_rbmb => {
  diagram    => 'MR=1 OP=0x0a',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=OR',
};

ENCODING OR_rvmv => {
  diagram    => 'MR=1 OP=0x0b',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=OR',
};

ENCODING OR_vsz => {
  diagram    => 'OP=0x0d',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=OR',
};


# OUT-Output to Port.
ENCODING OUT_ubb => {
  diagram    => 'OP=0xe6',
  iflags     => 'iopl=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=OUT',
};

ENCODING OUT_ubx => {
  diagram    => 'OP=0xe7',
  iflags     => 'iopl=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=OUT',
};

ENCODING OUT_w => {
  diagram    => 'OP=0xef',
  iflags     => 'iopl=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=OUT',
};

ENCODING OUT_wb => {
  diagram    => 'OP=0xee',
  iflags     => 'iopl=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=OUT',
};


# OUTS/OUTSB/OUTSW/OUTSD-Output String to Port.
ENCODING OUTSB => {
  diagram    => 'OP=0x6e',
  iflags     => 'df=R iopl=R',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I186',
  tags       => 'page=OUTS',
};

ENCODING OUTSD => {
  diagram    => 'OP=0x6f',
  iflags     => 'df=R iopl=R',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I386',
  tags       => 'page=OUTS',
};

ENCODING OUTSW => {
  diagram    => 'OP=0x6f OSZ=16',
  iflags     => 'df=R iopl=R',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I186',
  tags       => 'page=OUTS',
};


# PAUSE-Spin Loop Hint.
ENCODING PAUSE => {
  diagram    => 'OP=0x90 P66=0 PF2=0 PF3=1',
  extensions => 'PAUSE',
  categories => 'GENERAL',
  metadata   => 'isa=PAUSE',
  tags       => 'page=PAUSE',
};


# PCLMULQDQ-Carry-Less Multiplication Quadword.
ENCODING PCLMULQDQ => {
  diagram    => 'MAP=0f3a MR=1 OP=0x44 P66=1 PF2=0 PF3=0',
  extensions => 'PCLMULQDQ',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'GENERAL|APP',
  metadata   => 'isa=PCLMULQDQ requires_alignment=1',
  tags       => 'page=PCLMULQDQ',
};

ENCODING VPCLMULQDQ_rynymyub_1 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x44 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'VPCLMULQDQ',
  exceptions => 'AVX_TYPE_4',
  categories => 'GENERAL|APP',
  metadata   => 'isa=VPCLMULQDQ',
  tags       => 'page=PCLMULQDQ',
};


# PCONFIG.
ENCODING PCONFIG => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=0 RM=5',
  iflags     => 'af=Z cf=Z of=Z pf=Z sf=Z zf=W',
  extensions => 'PCONFIG',
  categories => 'GENERAL|MISC',
  metadata   => 'isa=PCONFIG cpl=RING0',
  tags       => 'page=PCONFIG',
};


# PDEP-Parallel Bits Deposit.
ENCODING PDEP => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xf5 P66=0 PF2=1 PF3=0 VL=128',
  extensions => 'BMI2',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=BMI2',
  tags       => 'page=PDEP',
};


# PEXT-Parallel Bits Extract.
ENCODING PEXT => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xf5 P66=0 PF2=0 PF3=1 VL=128',
  extensions => 'BMI2',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=BMI2',
  tags       => 'page=PEXT',
};


# POP-Pop a Value from the Stack.
ENCODING POP_mv => {
  diagram    => 'MR=1 OP=0x8f REG=0',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=POP',
};

ENCODING POP_ov_1 => {
  diagram    => 'OP=0x58',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=POP',
};

ENCODING POP_ov_2 => {
  diagram    => 'OP=0x59',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=POP',
};

ENCODING POP_ov_3 => {
  diagram    => 'OP=0x5a',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=POP',
};

ENCODING POP_ov_4 => {
  diagram    => 'OP=0x5b',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=POP',
};

ENCODING POP_ov_5 => {
  diagram    => 'OP=0x5c',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=POP',
};

ENCODING POP_ov_6 => {
  diagram    => 'OP=0x5d',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=POP',
};

ENCODING POP_ov_7 => {
  diagram    => 'OP=0x5e',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=POP',
};

ENCODING POP_ov_8 => {
  diagram    => 'OP=0x5f',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=POP',
};

ENCODING POP_w_1 => {
  diagram    => 'MODE=NO64 OP=0x07',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=POP',
};

ENCODING POP_w_2 => {
  diagram    => 'MODE=NO64 OP=0x17',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=POP',
};

ENCODING POP_w_3 => {
  diagram    => 'MODE=NO64 OP=0x1f',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=POP',
};

ENCODING POP_w_4 => {
  diagram    => 'MAP=0f OP=0xa1',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=POP',
};

ENCODING POP_w_5 => {
  diagram    => 'MAP=0f OP=0xa9',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=POP',
};


# POPA/POPAD-Pop All General-Purpose Registers.
ENCODING POPA => {
  diagram    => 'MODE=NO64 OP=0x61 OSZ=16',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I186 deprecated=1',
  tags       => 'page=POPA',
};

ENCODING POPAD => {
  diagram    => 'MODE=NO64 OP=0x61 OSZ=32',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I386 deprecated=1',
  tags       => 'page=POPA',
};


# POPF/POPFD/POPFQ-Pop Stack into EFLAGS Register.
ENCODING POPF => {
  diagram    => 'OP=0x9d OSZ=16',
  iflags     => 'ac=W af=W cf=W df=W id=W _if=W iopl=X nt=W of=W pf=W rf=Z sf=W tf=W vif=W vip=R vm=R zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=POPF',
};

ENCODING POPFD => {
  diagram    => 'MODE=NO64 OP=0x9d OSZ=32',
  iflags     => 'ac=W af=W cf=W df=W id=W _if=W iopl=X nt=W of=W pf=W rf=Z sf=W tf=W vif=W vip=R vm=R zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I386',
  tags       => 'page=POPF',
};

ENCODING POPFQ => {
  diagram    => 'MODE=64 OP=0x9d',
  iflags     => 'ac=W af=W cf=W df=W id=W _if=W iopl=X nt=W of=W pf=W rf=Z sf=W tf=W vif=W vip=R vm=R zf=W',
  extensions => 'LONGMODE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=LONGMODE',
  tags       => 'page=POPF',
};


# PUSH-Push Word, Doubleword or Quadword Onto the Stack.
ENCODING PUSH_mv => {
  diagram    => 'MR=1 OP=0xff REG=6',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=PUSH',
};

ENCODING PUSH_ov_1 => {
  diagram    => 'OP=0x50',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=PUSH',
};

ENCODING PUSH_ov_2 => {
  diagram    => 'OP=0x51',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=PUSH',
};

ENCODING PUSH_ov_3 => {
  diagram    => 'OP=0x52',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=PUSH',
};

ENCODING PUSH_ov_4 => {
  diagram    => 'OP=0x53',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=PUSH',
};

ENCODING PUSH_ov_5 => {
  diagram    => 'OP=0x54',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=PUSH',
};

ENCODING PUSH_ov_6 => {
  diagram    => 'OP=0x55',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=PUSH',
};

ENCODING PUSH_ov_7 => {
  diagram    => 'OP=0x56',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=PUSH',
};

ENCODING PUSH_ov_8 => {
  diagram    => 'OP=0x57',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=PUSH',
};

ENCODING PUSH_sb => {
  diagram    => 'OP=0x6a',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I186',
  tags       => 'page=PUSH',
};

ENCODING PUSH_sz => {
  diagram    => 'OP=0x68',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I186',
  tags       => 'page=PUSH',
};

ENCODING PUSH_w_1 => {
  diagram    => 'MODE=NO64 OP=0x06',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=PUSH',
};

ENCODING PUSH_w_2 => {
  diagram    => 'MODE=NO64 OP=0x0e',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=PUSH',
};

ENCODING PUSH_w_3 => {
  diagram    => 'MODE=NO64 OP=0x16',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=PUSH',
};

ENCODING PUSH_w_4 => {
  diagram    => 'MODE=NO64 OP=0x1e',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=PUSH',
};

ENCODING PUSH_w_5 => {
  diagram    => 'MAP=0f OP=0xa0',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=PUSH',
};

ENCODING PUSH_w_6 => {
  diagram    => 'MAP=0f OP=0xa8',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=PUSH',
};


# PUSHA/PUSHAD-Push All General-Purpose Registers.
ENCODING PUSHA => {
  diagram    => 'MODE=NO64 OP=0x60 OSZ=16',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I186 deprecated=1',
  tags       => 'page=PUSHA',
};

ENCODING PUSHAD => {
  diagram    => 'MODE=NO64 OP=0x60 OSZ=32',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I386 deprecated=1',
  tags       => 'page=PUSHA',
};


# PUSHF/PUSHFD/PUSHFQ-Push EFLAGS Register onto the Stack.
ENCODING PUSHF => {
  diagram    => 'OP=0x9c OSZ=16',
  iflags     => 'ac=R af=R cf=R df=R id=R _if=R iopl=R nt=R of=R pf=R rf=R sf=R tf=R vif=R vip=R vm=R zf=R',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I86',
  tags       => 'page=PUSHF',
};

ENCODING PUSHFD => {
  diagram    => 'MODE=NO64 OP=0x9c OSZ=32',
  iflags     => 'ac=R af=R cf=R df=R id=R _if=R iopl=R nt=R of=R pf=R rf=R sf=R tf=R vif=R vip=R vm=R zf=R',
  extensions => 'BASE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=I386',
  tags       => 'page=PUSHF',
};

ENCODING PUSHFQ => {
  diagram    => 'MODE=64 OP=0x9c',
  iflags     => 'ac=R af=R cf=R df=R id=R _if=R iopl=R nt=R of=R pf=R rf=R sf=R tf=R vif=R vip=R vm=R zf=R',
  extensions => 'LONGMODE',
  categories => 'GENERAL|STACK_TRANSFER',
  metadata   => 'isa=LONGMODE',
  tags       => 'page=PUSHF',
};


# RDMSR-Read from Model Specific Register.
ENCODING RDMSR => {
  diagram    => 'MAP=0f OP=0x32',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=PENTIUMREAL cpl=RING0',
  tags       => 'page=RDMSR',
};


# RDPMC-Read Performance-Monitoring Counters.
ENCODING RDPMC => {
  diagram    => 'MAP=0f OP=0x33',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=RDPMC',
  tags       => 'page=RDPMC',
};


# RDTSC-Read Time-Stamp Counter.
ENCODING RDTSC => {
  diagram    => 'MAP=0f OP=0x31',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=PENTIUMREAL',
  tags       => 'page=RDTSC',
};


# RET-Return from Procedure.
ENCODING RET_1 => {
  diagram    => 'OP=0xc3',
  extensions => 'BASE',
  categories => 'GENERAL|RET|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=RET',
};

ENCODING RET_2 => {
  diagram    => 'OP=0xcb',
  extensions => 'BASE',
  categories => 'GENERAL|RET|FAR',
  metadata   => 'isa=I86',
  docvars    => 'FAR_XFER=1',
  tags       => 'page=RET',
};

ENCODING RET_uw_1 => {
  diagram    => 'OP=0xc2',
  extensions => 'BASE',
  categories => 'GENERAL|RET|NEAR',
  metadata   => 'isa=I86',
  docvars    => 'MPX_PREFIX_ABLE=1',
  tags       => 'page=RET',
};

ENCODING RET_uw_2 => {
  diagram    => 'OP=0xca',
  extensions => 'BASE',
  categories => 'GENERAL|RET|FAR',
  metadata   => 'isa=I86',
  docvars    => 'FAR_XFER=1',
  tags       => 'page=RET',
};


# RORX-Rotate Right Logical Without Affecting Flags.
ENCODING RORX => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0xf0 P66=0 PF2=1 PF3=0 VL=128',
  extensions => 'BMI2',
  categories => 'GENERAL|ROTATE_RIGHT|LOGICAL',
  metadata   => 'isa=BMI2',
  tags       => 'page=RORX',
};


# RCL/RCR/ROL/ROR-Rotate.
ENCODING RCL_mb => {
  diagram    => 'MR=1 OP=0xd0 REG=2',
  iflags     => 'cf=X of=U',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=ROTATE',
};

ENCODING RCL_mbb => {
  diagram    => 'MR=1 OP=0xd2 REG=2',
  iflags     => 'cf=RCW of=U',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=ROTATE',
};

ENCODING RCL_mbub => {
  diagram    => 'MR=1 OP=0xc0 REG=2',
  iflags     => 'cf=X of=U',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I186',
  tags       => 'page=ROTATE',
};

ENCODING RCL_mv => {
  diagram    => 'MR=1 OP=0xd1 REG=2',
  iflags     => 'cf=X of=U',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=ROTATE',
};

ENCODING RCL_mvb => {
  diagram    => 'MR=1 OP=0xd3 REG=2',
  iflags     => 'cf=RCW of=U',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=ROTATE',
};

ENCODING RCL_mvub => {
  diagram    => 'MR=1 OP=0xc1 REG=2',
  iflags     => 'cf=X of=U',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I186',
  tags       => 'page=ROTATE',
};

ENCODING RCR_mb => {
  diagram    => 'MR=1 OP=0xd0 REG=3',
  iflags     => 'cf=X of=U',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=ROTATE',
};

ENCODING RCR_mbb => {
  diagram    => 'MR=1 OP=0xd2 REG=3',
  iflags     => 'cf=RCW of=U',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=ROTATE',
};

ENCODING RCR_mbub => {
  diagram    => 'MR=1 OP=0xc0 REG=3',
  iflags     => 'cf=X of=U',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I186',
  tags       => 'page=ROTATE',
};

ENCODING RCR_mv => {
  diagram    => 'MR=1 OP=0xd1 REG=3',
  iflags     => 'cf=X of=U',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=ROTATE',
};

ENCODING RCR_mvb => {
  diagram    => 'MR=1 OP=0xd3 REG=3',
  iflags     => 'cf=RCW of=U',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=ROTATE',
};

ENCODING RCR_mvub => {
  diagram    => 'MR=1 OP=0xc1 REG=3',
  iflags     => 'cf=X of=U',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I186',
  tags       => 'page=ROTATE',
};

ENCODING ROL_mb => {
  diagram    => 'MR=1 OP=0xd0 REG=0',
  iflags     => 'cf=W of=U',
  extensions => 'BASE',
  categories => 'GENERAL|SHIFT_LEFT',
  metadata   => 'isa=I86',
  tags       => 'page=ROTATE',
};

ENCODING ROL_mbb => {
  diagram    => 'MR=1 OP=0xd2 REG=0',
  iflags     => 'cf=CW of=U',
  extensions => 'BASE',
  categories => 'GENERAL|SHIFT_LEFT',
  metadata   => 'isa=I86',
  tags       => 'page=ROTATE',
};

ENCODING ROL_mbub => {
  diagram    => 'MR=1 OP=0xc0 REG=0',
  iflags     => 'cf=W of=U',
  extensions => 'BASE',
  categories => 'GENERAL|SHIFT_LEFT',
  metadata   => 'isa=I186',
  tags       => 'page=ROTATE',
};

ENCODING ROL_mv => {
  diagram    => 'MR=1 OP=0xd1 REG=0',
  iflags     => 'cf=W of=U',
  extensions => 'BASE',
  categories => 'GENERAL|SHIFT_LEFT',
  metadata   => 'isa=I86',
  tags       => 'page=ROTATE',
};

ENCODING ROL_mvb => {
  diagram    => 'MR=1 OP=0xd3 REG=0',
  iflags     => 'cf=CW of=U',
  extensions => 'BASE',
  categories => 'GENERAL|SHIFT_LEFT',
  metadata   => 'isa=I86',
  tags       => 'page=ROTATE',
};

ENCODING ROL_mvub => {
  diagram    => 'MR=1 OP=0xc1 REG=0',
  iflags     => 'cf=W of=U',
  extensions => 'BASE',
  categories => 'GENERAL|SHIFT_LEFT',
  metadata   => 'isa=I186',
  tags       => 'page=ROTATE',
};

ENCODING ROR_mb => {
  diagram    => 'MR=1 OP=0xd0 REG=1',
  iflags     => 'cf=W of=U',
  extensions => 'BASE',
  categories => 'GENERAL|SHIFT_RIGHT',
  metadata   => 'isa=I86',
  tags       => 'page=ROTATE',
};

ENCODING ROR_mbb => {
  diagram    => 'MR=1 OP=0xd2 REG=1',
  iflags     => 'cf=CW of=U',
  extensions => 'BASE',
  categories => 'GENERAL|SHIFT_RIGHT',
  metadata   => 'isa=I86',
  tags       => 'page=ROTATE',
};

ENCODING ROR_mbub => {
  diagram    => 'MR=1 OP=0xc0 REG=1',
  iflags     => 'cf=W of=U',
  extensions => 'BASE',
  categories => 'GENERAL|SHIFT_RIGHT',
  metadata   => 'isa=I186',
  tags       => 'page=ROTATE',
};

ENCODING ROR_mv => {
  diagram    => 'MR=1 OP=0xd1 REG=1',
  iflags     => 'cf=W of=U',
  extensions => 'BASE',
  categories => 'GENERAL|SHIFT_RIGHT',
  metadata   => 'isa=I86',
  tags       => 'page=ROTATE',
};

ENCODING ROR_mvb => {
  diagram    => 'MR=1 OP=0xd3 REG=1',
  iflags     => 'cf=CW of=U',
  extensions => 'BASE',
  categories => 'GENERAL|SHIFT_RIGHT',
  metadata   => 'isa=I86',
  tags       => 'page=ROTATE',
};

ENCODING ROR_mvub => {
  diagram    => 'MR=1 OP=0xc1 REG=1',
  iflags     => 'cf=W of=U',
  extensions => 'BASE',
  categories => 'GENERAL|SHIFT_RIGHT',
  metadata   => 'isa=I186',
  tags       => 'page=ROTATE',
};


# RSM-Resume from System Management Mode.
ENCODING RSM => {
  diagram    => 'MAP=0f OP=0xaa',
  iflags     => 'ac=W af=W cf=W df=W id=W _if=W iopl=W nt=W of=W pf=W rf=W sf=W tf=W vif=W vip=W vm=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|SYSRET',
  metadata   => 'isa=I486',
  tags       => 'page=RSM',
};


# SAHF-Store AH into Flags.
ENCODING SAHF => {
  diagram    => 'OP=0x9e',
  iflags     => 'af=W cf=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=LAHF',
  tags       => 'page=SAHF',
};


# SALC.
ENCODING SALC => {
  diagram    => 'MODE=NO64 OP=0xd6',
  iflags     => 'cf=R',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=SALC',
};


# SBB-Integer Subtraction with Borrow.
ENCODING SBB_bsb => {
  diagram    => 'OP=0x1c',
  iflags     => 'af=U cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SBB',
};

ENCODING SBB_mbrb => {
  diagram    => 'MR=1 OP=0x18',
  iflags     => 'af=U cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SBB',
};

ENCODING SBB_mbsb_1 => {
  diagram    => 'MR=1 OP=0x80 REG=3',
  iflags     => 'af=U cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SBB',
};

ENCODING SBB_mbsb_2 => {
  diagram    => 'MODE=NO64 MR=1 OP=0x82 REG=3',
  iflags     => 'af=U cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SBB',
};

ENCODING SBB_mvrv => {
  diagram    => 'MR=1 OP=0x19',
  iflags     => 'af=U cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SBB',
};

ENCODING SBB_mvsb => {
  diagram    => 'MR=1 OP=0x83 REG=3',
  iflags     => 'af=U cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SBB',
};

ENCODING SBB_mvsz => {
  diagram    => 'MR=1 OP=0x81 REG=3',
  iflags     => 'af=U cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SBB',
};

ENCODING SBB_rbmb => {
  diagram    => 'MR=1 OP=0x1a',
  iflags     => 'af=U cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SBB',
};

ENCODING SBB_rvmv => {
  diagram    => 'MR=1 OP=0x1b',
  iflags     => 'af=U cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SBB',
};

ENCODING SBB_vsz => {
  diagram    => 'OP=0x1d',
  iflags     => 'af=U cf=X of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SBB',
};


# SCAS/SCASB/SCASW/SCASD-Scan String.
ENCODING SCASB_1 => {
  diagram    => 'OP=0xae PF2=0 PF3=0',
  iflags     => 'af=W cf=W df=R of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I86',
  tags       => 'page=SCAS',
};

ENCODING SCASB_2 => {
  diagram    => 'OP=0xae P66=0',
  iflags     => 'af=CW cf=CW df=R of=CW pf=CW sf=CW zf=RCW',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I86',
  tags       => 'page=SCAS',
};

ENCODING SCASD_1 => {
  diagram    => 'OP=0xaf OSZ=32 PF2=0 PF3=0',
  iflags     => 'af=W cf=W df=R of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I386',
  tags       => 'page=SCAS',
};

ENCODING SCASD_2 => {
  diagram    => 'OP=0xaf OSZ=32 P66=0',
  iflags     => 'af=CW cf=CW df=R of=CW pf=CW sf=CW zf=RCW',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I386',
  tags       => 'page=SCAS',
};

ENCODING SCASQ_1 => {
  diagram    => 'OP=0xaf OSZ=64 PF2=0 PF3=0',
  iflags     => 'af=W cf=W df=R of=W pf=W sf=W zf=W',
  extensions => 'LONGMODE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=LONGMODE',
  tags       => 'page=SCAS',
};

ENCODING SCASQ_2 => {
  diagram    => 'OP=0xaf OSZ=64 P66=0',
  iflags     => 'af=CW cf=CW df=R of=CW pf=CW sf=CW zf=RCW',
  extensions => 'LONGMODE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=LONGMODE',
  tags       => 'page=SCAS',
};

ENCODING SCASW_1 => {
  diagram    => 'OP=0xaf OSZ=16 PF2=0 PF3=0',
  iflags     => 'af=W cf=W df=R of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I86',
  tags       => 'page=SCAS',
};

ENCODING SCASW_2 => {
  diagram    => 'OP=0xaf OSZ=16 P66=0',
  iflags     => 'af=CW cf=CW df=R of=CW pf=CW sf=CW zf=RCW',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I86',
  tags       => 'page=SCAS',
};


# SETcc-Set Byte on Condition.
ENCODING SETB => {
  diagram    => 'MAP=0f MR=1 OP=0x92',
  iflags     => 'cf=R',
  extensions => 'BASE',
  categories => 'GENERAL|SETCC',
  metadata   => 'isa=I386',
  tags       => 'page=SETcc',
};

ENCODING SETBE => {
  diagram    => 'MAP=0f MR=1 OP=0x96',
  iflags     => 'cf=R zf=R',
  extensions => 'BASE',
  categories => 'GENERAL|SETCC',
  metadata   => 'isa=I386',
  tags       => 'page=SETcc',
};

ENCODING SETL => {
  diagram    => 'MAP=0f MR=1 OP=0x9c',
  iflags     => 'of=R sf=R',
  extensions => 'BASE',
  categories => 'GENERAL|SETCC',
  metadata   => 'isa=I386',
  tags       => 'page=SETcc',
};

ENCODING SETLE => {
  diagram    => 'MAP=0f MR=1 OP=0x9e',
  iflags     => 'of=R sf=R zf=R',
  extensions => 'BASE',
  categories => 'GENERAL|SETCC',
  metadata   => 'isa=I386',
  tags       => 'page=SETcc',
};

ENCODING SETNB => {
  diagram    => 'MAP=0f MR=1 OP=0x93',
  iflags     => 'cf=R',
  extensions => 'BASE',
  categories => 'GENERAL|SETCC',
  metadata   => 'isa=I386',
  tags       => 'page=SETcc',
};

ENCODING SETNBE => {
  diagram    => 'MAP=0f MR=1 OP=0x97',
  iflags     => 'cf=R zf=R',
  extensions => 'BASE',
  categories => 'GENERAL|SETCC',
  metadata   => 'isa=I386',
  tags       => 'page=SETcc',
};

ENCODING SETNL => {
  diagram    => 'MAP=0f MR=1 OP=0x9d',
  iflags     => 'of=R sf=R',
  extensions => 'BASE',
  categories => 'GENERAL|SETCC',
  metadata   => 'isa=I386',
  tags       => 'page=SETcc',
};

ENCODING SETNLE => {
  diagram    => 'MAP=0f MR=1 OP=0x9f',
  iflags     => 'of=R sf=R zf=R',
  extensions => 'BASE',
  categories => 'GENERAL|SETCC',
  metadata   => 'isa=I386',
  tags       => 'page=SETcc',
};

ENCODING SETNO => {
  diagram    => 'MAP=0f MR=1 OP=0x91',
  iflags     => 'of=R',
  extensions => 'BASE',
  categories => 'GENERAL|SETCC',
  metadata   => 'isa=I386',
  tags       => 'page=SETcc',
};

ENCODING SETNP => {
  diagram    => 'MAP=0f MR=1 OP=0x9b',
  iflags     => 'pf=R',
  extensions => 'BASE',
  categories => 'GENERAL|SETCC',
  metadata   => 'isa=I386',
  tags       => 'page=SETcc',
};

ENCODING SETNS => {
  diagram    => 'MAP=0f MR=1 OP=0x99',
  iflags     => 'sf=R',
  extensions => 'BASE',
  categories => 'GENERAL|SETCC',
  metadata   => 'isa=I386',
  tags       => 'page=SETcc',
};

ENCODING SETNZ => {
  diagram    => 'MAP=0f MR=1 OP=0x95',
  iflags     => 'zf=R',
  extensions => 'BASE',
  categories => 'GENERAL|SETCC',
  metadata   => 'isa=I386',
  tags       => 'page=SETcc',
};

ENCODING SETO => {
  diagram    => 'MAP=0f MR=1 OP=0x90',
  iflags     => 'of=R',
  extensions => 'BASE',
  categories => 'GENERAL|SETCC',
  metadata   => 'isa=I386',
  tags       => 'page=SETcc',
};

ENCODING SETP => {
  diagram    => 'MAP=0f MR=1 OP=0x9a',
  iflags     => 'pf=R',
  extensions => 'BASE',
  categories => 'GENERAL|SETCC',
  metadata   => 'isa=I386',
  tags       => 'page=SETcc',
};

ENCODING SETS => {
  diagram    => 'MAP=0f MR=1 OP=0x98',
  iflags     => 'sf=R',
  extensions => 'BASE',
  categories => 'GENERAL|SETCC',
  metadata   => 'isa=I386',
  tags       => 'page=SETcc',
};

ENCODING SETZ => {
  diagram    => 'MAP=0f MR=1 OP=0x94',
  iflags     => 'zf=R',
  extensions => 'BASE',
  categories => 'GENERAL|SETCC',
  metadata   => 'isa=I386',
  tags       => 'page=SETcc',
};


# SGDT-Store Global Descriptor Table Register.
ENCODING SGDT => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x01 REG=0',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I286REAL',
  tags       => 'page=SGDT',
};


# SHA1MSG1-Perform an Intermediate Calculation for the Next Four SHA1 Message Dwords.
ENCODING SHA1MSG1_romo_1 => {
  diagram    => 'MAP=0f38 MOD=REG MR=1 OP=0xc9 P66=0 PF2=0 PF3=0',
  extensions => 'SHA',
  exceptions => 'SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=SHA',
  tags       => 'page=SHA1MSG1',
};

ENCODING SHA1MSG1_romo_2 => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0xc9 P66=0 PF2=0 PF3=0',
  extensions => 'SHA',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=SHA requires_alignment=1',
  tags       => 'page=SHA1MSG1',
};


# SHA1MSG2-Perform a Final Calculation for the Next Four SHA1 Message Dwords.
ENCODING SHA1MSG2_romo_1 => {
  diagram    => 'MAP=0f38 MOD=REG MR=1 OP=0xca P66=0 PF2=0 PF3=0',
  extensions => 'SHA',
  exceptions => 'SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=SHA',
  tags       => 'page=SHA1MSG2',
};

ENCODING SHA1MSG2_romo_2 => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0xca P66=0 PF2=0 PF3=0',
  extensions => 'SHA',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=SHA requires_alignment=1',
  tags       => 'page=SHA1MSG2',
};


# SHA1NEXTE-Calculate SHA1 State Variable E after Four Rounds.
ENCODING SHA1NEXTE_romo_1 => {
  diagram    => 'MAP=0f38 MOD=REG MR=1 OP=0xc8 P66=0 PF2=0 PF3=0',
  extensions => 'SHA',
  exceptions => 'SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=SHA',
  tags       => 'page=SHA1NEXTE',
};

ENCODING SHA1NEXTE_romo_2 => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0xc8 P66=0 PF2=0 PF3=0',
  extensions => 'SHA',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=SHA requires_alignment=1',
  tags       => 'page=SHA1NEXTE',
};


# SHA1RNDS4-Perform Four Rounds of SHA1 Operation.
ENCODING SHA1RNDS4_romoub_1 => {
  diagram    => 'MAP=0f3a MOD=REG MR=1 OP=0xcc P66=0 PF2=0 PF3=0',
  extensions => 'SHA',
  exceptions => 'SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=SHA',
  tags       => 'page=SHA1RNDS4',
};

ENCODING SHA1RNDS4_romoub_2 => {
  diagram    => 'MAP=0f3a MOD=MEM MR=1 OP=0xcc P66=0 PF2=0 PF3=0',
  extensions => 'SHA',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=SHA requires_alignment=1',
  tags       => 'page=SHA1RNDS4',
};


# SHA256MSG1-Perform an Intermediate Calculation for the Next Four SHA256 Message Dwords.
ENCODING SHA256MSG1_romo_1 => {
  diagram    => 'MAP=0f38 MOD=REG MR=1 OP=0xcc P66=0 PF2=0 PF3=0',
  extensions => 'SHA',
  exceptions => 'SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=SHA',
  tags       => 'page=SHA256MSG1',
};

ENCODING SHA256MSG1_romo_2 => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0xcc P66=0 PF2=0 PF3=0',
  extensions => 'SHA',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=SHA requires_alignment=1',
  tags       => 'page=SHA256MSG1',
};


# SHA256MSG2-Perform a Final Calculation for the Next Four SHA256 Message Dwords.
ENCODING SHA256MSG2_romo_1 => {
  diagram    => 'MAP=0f38 MOD=REG MR=1 OP=0xcd P66=0 PF2=0 PF3=0',
  extensions => 'SHA',
  exceptions => 'SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=SHA',
  tags       => 'page=SHA256MSG2',
};

ENCODING SHA256MSG2_romo_2 => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0xcd P66=0 PF2=0 PF3=0',
  extensions => 'SHA',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=SHA requires_alignment=1',
  tags       => 'page=SHA256MSG2',
};


# SHA256RNDS2-Perform Two Rounds of SHA256 Operation.
ENCODING SHA256RNDS2_romo_1 => {
  diagram    => 'MAP=0f38 MOD=REG MR=1 OP=0xcb P66=0 PF2=0 PF3=0',
  extensions => 'SHA',
  exceptions => 'SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=SHA',
  tags       => 'page=SHA256RNDS2',
};

ENCODING SHA256RNDS2_romo_2 => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0xcb P66=0 PF2=0 PF3=0',
  extensions => 'SHA',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'GENERAL|CRYPTO',
  metadata   => 'isa=SHA requires_alignment=1',
  tags       => 'page=SHA256RNDS2',
};


# SAL/SAR/SHL/SHR-Shift.
ENCODING SAR_mb => {
  diagram    => 'MR=1 OP=0xd0 REG=7',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=SHIFT',
};

ENCODING SAR_mbb => {
  diagram    => 'MR=1 OP=0xd2 REG=7',
  iflags     => 'af=U cf=CW of=U pf=CW sf=CW zf=CW',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=SHIFT',
};

ENCODING SAR_mbub => {
  diagram    => 'MR=1 OP=0xc0 REG=7',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I186',
  tags       => 'page=SHIFT',
};

ENCODING SAR_mv => {
  diagram    => 'MR=1 OP=0xd1 REG=7',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=SHIFT',
};

ENCODING SAR_mvb => {
  diagram    => 'MR=1 OP=0xd3 REG=7',
  iflags     => 'af=U cf=CW of=U pf=CW sf=CW zf=CW',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=SHIFT',
};

ENCODING SAR_mvub => {
  diagram    => 'MR=1 OP=0xc1 REG=7',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I186',
  tags       => 'page=SHIFT',
};

ENCODING SHL_mb_1 => {
  diagram    => 'MR=1 OP=0xd0 REG=4',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=SHIFT',
};

ENCODING SHL_mb_2 => {
  diagram    => 'MR=1 OP=0xd0 REG=6',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=SHIFT',
};

ENCODING SHL_mbb_1 => {
  diagram    => 'MR=1 OP=0xd2 REG=4',
  iflags     => 'af=U cf=CW of=U pf=CW sf=CW zf=CW',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=SHIFT',
};

ENCODING SHL_mbb_2 => {
  diagram    => 'MR=1 OP=0xd2 REG=6',
  iflags     => 'af=U cf=CW of=U pf=CW sf=CW zf=CW',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=SHIFT',
};

ENCODING SHL_mbub_1 => {
  diagram    => 'MR=1 OP=0xc0 REG=4',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I186',
  tags       => 'page=SHIFT',
};

ENCODING SHL_mbub_2 => {
  diagram    => 'MR=1 OP=0xc0 REG=6',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I186',
  tags       => 'page=SHIFT',
};

ENCODING SHL_mv_1 => {
  diagram    => 'MR=1 OP=0xd1 REG=4',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=SHIFT',
};

ENCODING SHL_mv_2 => {
  diagram    => 'MR=1 OP=0xd1 REG=6',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=SHIFT',
};

ENCODING SHL_mvb_1 => {
  diagram    => 'MR=1 OP=0xd3 REG=4',
  iflags     => 'af=U cf=CW of=U pf=CW sf=CW zf=CW',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=SHIFT',
};

ENCODING SHL_mvb_2 => {
  diagram    => 'MR=1 OP=0xd3 REG=6',
  iflags     => 'af=U cf=CW of=U pf=CW sf=CW zf=CW',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=SHIFT',
};

ENCODING SHL_mvub_1 => {
  diagram    => 'MR=1 OP=0xc1 REG=4',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I186',
  tags       => 'page=SHIFT',
};

ENCODING SHL_mvub_2 => {
  diagram    => 'MR=1 OP=0xc1 REG=6',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I186',
  tags       => 'page=SHIFT',
};

ENCODING SHR_mb => {
  diagram    => 'MR=1 OP=0xd0 REG=5',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=SHIFT',
};

ENCODING SHR_mbb => {
  diagram    => 'MR=1 OP=0xd2 REG=5',
  iflags     => 'af=U cf=CW of=U pf=CW sf=CW zf=CW',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=SHIFT',
};

ENCODING SHR_mbub => {
  diagram    => 'MR=1 OP=0xc0 REG=5',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I186',
  tags       => 'page=SHIFT',
};

ENCODING SHR_mv => {
  diagram    => 'MR=1 OP=0xd1 REG=5',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=SHIFT',
};

ENCODING SHR_mvb => {
  diagram    => 'MR=1 OP=0xd3 REG=5',
  iflags     => 'af=U cf=CW of=U pf=CW sf=CW zf=CW',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=SHIFT',
};

ENCODING SHR_mvub => {
  diagram    => 'MR=1 OP=0xc1 REG=5',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I186',
  tags       => 'page=SHIFT',
};


# SARX/SHLX/SHRX-Shift Without Affecting Flags.
ENCODING SHLX => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xf7 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'BMI2',
  categories => 'GENERAL',
  metadata   => 'isa=BMI2',
  tags       => 'page=SHIFTX',
};

ENCODING SxRX => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xf7 P66=0 VL=128',
  extensions => 'BMI2',
  categories => 'GENERAL',
  metadata   => 'isa=BMI2',
  tags       => 'page=SHIFTX',
};


# SHLD-Double Precision Shift Left.
ENCODING SHLD_mvrvb => {
  diagram    => 'MAP=0f MR=1 OP=0xa5',
  iflags     => 'af=U cf=CW of=U pf=CW sf=CW zf=CW',
  extensions => 'BASE',
  categories => 'GENERAL|SHIFT_LEFT',
  metadata   => 'isa=I386',
  tags       => 'page=SHLD',
};

ENCODING SHLD_mvrvub => {
  diagram    => 'MAP=0f MR=1 OP=0xa4',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|SHIFT_LEFT',
  metadata   => 'isa=I386',
  tags       => 'page=SHLD',
};


# SHRD-Double Precision Shift Right.
ENCODING SHRD_mvrvb => {
  diagram    => 'MAP=0f MR=1 OP=0xad',
  iflags     => 'af=U cf=CW of=U pf=CW sf=CW zf=CW',
  extensions => 'BASE',
  categories => 'GENERAL|SHIFT_RIGHT',
  metadata   => 'isa=I386',
  tags       => 'page=SHRD',
};

ENCODING SHRD_mvrvub => {
  diagram    => 'MAP=0f MR=1 OP=0xac',
  iflags     => 'af=U cf=W of=U pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|SHIFT_RIGHT',
  metadata   => 'isa=I386',
  tags       => 'page=SHRD',
};


# SIDT-Store Interrupt Descriptor Table Register.
ENCODING SIDT => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x01 REG=1',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I286REAL',
  tags       => 'page=SIDT',
};


# SLDT-Store Local Descriptor Table Register.
ENCODING SLDT => {
  diagram    => 'MAP=0f MR=1 OP=0x00 REG=0',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I286PROTECTED',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=SLDT',
};


# SMSW-Store Machine Status Word.
ENCODING SMSW => {
  diagram    => 'MAP=0f MR=1 OP=0x01 REG=4',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I286REAL',
  tags       => 'page=SMSW',
};


# STC-Set Carry Flag.
ENCODING STC => {
  diagram    => 'OP=0xf9',
  iflags     => 'cf=S',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=STC',
};


# STD-Set Direction Flag.
ENCODING STD => {
  diagram    => 'OP=0xfd',
  iflags     => 'df=S',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=STD',
};


# STI-Set Interrupt Flag.
ENCODING STI => {
  diagram    => 'OP=0xfb',
  iflags     => '_if=W iopl=R vif=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=STI',
};


# STOS/STOSB/STOSW/STOSD/STOSQ-Store String.
ENCODING STOSB => {
  diagram    => 'OP=0xaa',
  iflags     => 'df=R',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I86',
  tags       => 'page=STOS',
};

ENCODING STOSD => {
  diagram    => 'OP=0xab OSZ=32',
  iflags     => 'df=R',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I386',
  tags       => 'page=STOS',
};

ENCODING STOSQ => {
  diagram    => 'OP=0xab OSZ=64',
  iflags     => 'df=R',
  extensions => 'LONGMODE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=LONGMODE',
  tags       => 'page=STOS',
};

ENCODING STOSW => {
  diagram    => 'OP=0xab OSZ=16',
  iflags     => 'df=R',
  extensions => 'BASE',
  categories => 'GENERAL|STRING',
  metadata   => 'isa=I86',
  tags       => 'page=STOS',
};


# STR-Store Task Register.
ENCODING STR => {
  diagram    => 'MAP=0f MR=1 OP=0x00 REG=1',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I286PROTECTED',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=STR',
};


# SUB-Subtract.
ENCODING SUB_bsb => {
  diagram    => 'OP=0x2c',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SUB',
};

ENCODING SUB_mbrb => {
  diagram    => 'MR=1 OP=0x28',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SUB',
};

ENCODING SUB_mbsb_1 => {
  diagram    => 'MR=1 OP=0x80 REG=5',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SUB',
};

ENCODING SUB_mbsb_2 => {
  diagram    => 'MODE=NO64 MR=1 OP=0x82 REG=5',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SUB',
};

ENCODING SUB_mvrv => {
  diagram    => 'MR=1 OP=0x29',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SUB',
};

ENCODING SUB_mvsb => {
  diagram    => 'MR=1 OP=0x83 REG=5',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SUB',
};

ENCODING SUB_mvsz => {
  diagram    => 'MR=1 OP=0x81 REG=5',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SUB',
};

ENCODING SUB_rbmb => {
  diagram    => 'MR=1 OP=0x2a',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SUB',
};

ENCODING SUB_rvmv => {
  diagram    => 'MR=1 OP=0x2b',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SUB',
};

ENCODING SUB_vsz => {
  diagram    => 'OP=0x2d',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|ARITHMETIC|BINARY',
  metadata   => 'isa=I86',
  tags       => 'page=SUB',
};


# SWAPGS-Swap GS Base Register.
ENCODING SWAPGS => {
  diagram    => 'MAP=0f MOD=REG MODE=64 MR=1 OP=0x01 REG=7 RM=0',
  extensions => 'LONGMODE',
  categories => 'GENERAL',
  metadata   => 'isa=LONGMODE cpl=RING0',
  tags       => 'page=SWAPGS',
};


# SYSCALL-Fast System Call.
ENCODING SYSCALL_1 => {
  diagram    => 'MAP=0f MODE=64 OP=0x05',
  iflags     => 'ac=W af=W cf=W df=W id=W _if=W iopl=W nt=W of=W pf=W rf=Z sf=W tf=W vif=W vip=W vm=Z zf=W',
  extensions => 'LONGMODE',
  categories => 'GENERAL|SYSCALL',
  metadata   => 'isa=LONGMODE',
  tags       => 'page=SYSCALL',
};


# SYSENTER-Fast System Call.
ENCODING SYSENTER => {
  diagram    => 'MAP=0f OP=0x34',
  iflags     => '_if=Z rf=Z vm=Z',
  extensions => 'BASE',
  categories => 'GENERAL|SYSCALL',
  metadata   => 'isa=PPRO',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=SYSENTER',
};


# SYSEXIT-Fast Return from Fast System Call.
ENCODING SYSEXIT => {
  diagram    => 'MAP=0f OP=0x35',
  extensions => 'BASE',
  categories => 'GENERAL|SYSRET',
  metadata   => 'isa=PPRO cpl=RING0',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=SYSEXIT',
};


# SYSRET-Return From Fast System Call.
ENCODING SYSRET_1 => {
  diagram    => 'MAP=0f MODE=64 OP=0x07',
  iflags     => 'ac=W af=W cf=W df=W id=W _if=W iopl=W nt=W of=W pf=W rf=Z sf=W tf=W vif=W vip=W zf=W',
  extensions => 'LONGMODE',
  categories => 'GENERAL|SYSRET',
  metadata   => 'isa=LONGMODE cpl=RING0',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=SYSRET',
};


# TEST-Logical Compare.
ENCODING TEST_bsb => {
  diagram    => 'OP=0xa8',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=TEST',
};

ENCODING TEST_mbrb => {
  diagram    => 'MR=1 OP=0x84',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=TEST',
};

ENCODING TEST_mbsb_1 => {
  diagram    => 'MR=1 OP=0xf6 REG=0',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=TEST',
};

ENCODING TEST_mbsb_2 => {
  diagram    => 'MR=1 OP=0xf6 REG=1',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=TEST',
};

ENCODING TEST_mvrv => {
  diagram    => 'MR=1 OP=0x85',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=TEST',
};

ENCODING TEST_mvsz_1 => {
  diagram    => 'MR=1 OP=0xf7 REG=0',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=TEST',
};

ENCODING TEST_mvsz_2 => {
  diagram    => 'MR=1 OP=0xf7 REG=1',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=TEST',
};

ENCODING TEST_vsz => {
  diagram    => 'OP=0xa9',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=TEST',
};


# TZCNT-Count the Number of Trailing Zero Bits.
ENCODING TZCNT_rvmv => {
  diagram    => 'BMI=1 MAP=0f MR=1 OP=0xbc P66=0 PF2=0 PF3=1',
  iflags     => 'af=U cf=W of=U pf=U sf=U zf=W',
  extensions => 'BMI1',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=BMI1',
  tags       => 'page=TZCNT',
};


# UD-Undefined Instruction.
ENCODING UD0 => {
  diagram    => 'MAP=0f MR=1 OP=0xff',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=PPRO',
  tags       => 'page=UD',
};

ENCODING UD1 => {
  diagram    => 'MAP=0f MR=1 OP=0xb9',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=PPRO',
  tags       => 'page=UD',
};

ENCODING UD2 => {
  diagram    => 'MAP=0f OP=0x0b',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=PPRO',
  tags       => 'page=UD',
};


# VCVTPH2PS-Convert 16-bit FP values to Single-Precision FP values.
ENCODING VCVTPH2PS_rxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x13 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'F16C',
  exceptions => 'AVX_TYPE_11',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=F16C',
  tags       => 'page=VCVTPH2PS',
};


# VCVTPS2PH-Convert Single-Precision FP value to 16-bit FP value.
ENCODING VCVTPS2PH_mxrxub => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x1d P66=1 PF2=0 PF3=0 W=0',
  extensions => 'F16C',
  exceptions => 'AVX_TYPE_11',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=F16C',
  tags       => 'page=VCVTPS2PH',
};


# VERR/VERW-Verify a Segment for Reading or Writing.
ENCODING VERR => {
  diagram    => 'MAP=0f MR=1 OP=0x00 REG=4',
  iflags     => 'zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I286PROTECTED',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=VERx',
};

ENCODING VERW => {
  diagram    => 'MAP=0f MR=1 OP=0x00 REG=5',
  iflags     => 'zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I286PROTECTED',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=VERx',
};


# WBINVD-Write Back and Invalidate Cache.
ENCODING WBINVD => {
  diagram    => 'MAP=0f OP=0x09',
  extensions => 'BASE',
  categories => 'GENERAL|MISC',
  metadata   => 'isa=I486REAL cpl=RING0',
  tags       => 'page=WBINVD',
};


# WRMSR-Write to Model Specific Register.
ENCODING WRMSR => {
  diagram    => 'MAP=0f OP=0x30',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=PENTIUMREAL cpl=RING0',
  tags       => 'page=WRMSR',
};


# XADD-Exchange and Add.
ENCODING XADD_mbrb => {
  diagram    => 'MAP=0f MR=1 OP=0xc0',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I486REAL',
  tags       => 'page=XADD',
};

ENCODING XADD_mvrv => {
  diagram    => 'MAP=0f MR=1 OP=0xc1',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I486REAL',
  tags       => 'page=XADD',
};


# XCHG-Exchange Register/Memory with Register.
ENCODING XCHG_mbrb => {
  diagram    => 'MR=1 OP=0x86',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=XCHG',
};

ENCODING XCHG_mvrv => {
  diagram    => 'MR=1 OP=0x87',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=XCHG',
};

ENCODING XCHG_ovv_1 => {
  diagram    => 'B=1 OP=0x90 PF3=0',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=XCHG',
};

ENCODING XCHG_ovv_2 => {
  diagram    => 'OP=0x91',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=XCHG',
};

ENCODING XCHG_ovv_3 => {
  diagram    => 'OP=0x92',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=XCHG',
};

ENCODING XCHG_ovv_4 => {
  diagram    => 'OP=0x93',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=XCHG',
};

ENCODING XCHG_ovv_5 => {
  diagram    => 'OP=0x94',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=XCHG',
};

ENCODING XCHG_ovv_6 => {
  diagram    => 'OP=0x95',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=XCHG',
};

ENCODING XCHG_ovv_7 => {
  diagram    => 'OP=0x96',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=XCHG',
};

ENCODING XCHG_ovv_8 => {
  diagram    => 'OP=0x97',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=XCHG',
};


# XLAT/XLATB-Table Look-up Translation.
ENCODING XLAT => {
  diagram    => 'OP=0xd7',
  extensions => 'BASE',
  categories => 'GENERAL',
  metadata   => 'isa=I86',
  tags       => 'page=XLAT',
};


# XOR-Logical Exclusive OR.
ENCODING XOR_bub => {
  diagram    => 'OP=0x34',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=XOR',
};

ENCODING XOR_mbrb => {
  diagram    => 'MR=1 OP=0x30',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=XOR',
};

ENCODING XOR_mbub_1 => {
  diagram    => 'MR=1 OP=0x80 REG=6',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=XOR',
};

ENCODING XOR_mbub_2 => {
  diagram    => 'MODE=NO64 MR=1 OP=0x82 REG=6',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=XOR',
};

ENCODING XOR_mvrv => {
  diagram    => 'MR=1 OP=0x31',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=XOR',
};

ENCODING XOR_mvsb => {
  diagram    => 'MR=1 OP=0x83 REG=6',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=XOR',
};

ENCODING XOR_mvsz => {
  diagram    => 'MR=1 OP=0x81 REG=6',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=XOR',
};

ENCODING XOR_rbmb => {
  diagram    => 'MR=1 OP=0x32',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=XOR',
};

ENCODING XOR_rvmv => {
  diagram    => 'MR=1 OP=0x33',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=XOR',
};

ENCODING XOR_vsz => {
  diagram    => 'OP=0x35',
  iflags     => 'af=U cf=Z of=Z pf=W sf=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|BITWISE|LOGICAL',
  metadata   => 'isa=I86',
  tags       => 'page=XOR',
};

