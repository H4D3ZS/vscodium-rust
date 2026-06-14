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


# PF2IW-Packed Floating-Point to Integer Word Conversion with Sign Extend.
ENCODING PF2IW => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0x1c',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3NEXT_PF2IW',
};


# PFNACC-Packed Floating-Point Negative Accumulate.
ENCODING PFNACC => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0x8a',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3NEXT_PFNACC',
};


# PFPNACC-Packed Floating-Point Mixed Positive-Negative Accumulate.
ENCODING PFPNACC => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0x8e',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3NEXT_PFPNACC',
};


# PI2FW-Packed Integer Word to Floating-Point Conversion.
ENCODING PI2FW => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0x0c',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3NEXT_PI2FW',
};


# PSWAPD-Packed Swap Doubleword.
ENCODING PSWAPD => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0xbb',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3NEXT_PSWAPD',
};


# FEMMS-Faster entry/exit of the MMX or floating-point state.
ENCODING FEMMS => {
  diagram    => 'MAP=0f OP=0x0e',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  docvars    => 'X87_MMX_STATE_W=1',
  tags       => 'page=D3N_FEMMS',
};


# PAVGUSB-Packed 8-bit Unsigned Integer Averaging.
ENCODING PAVGUSB => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0xbf',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3N_PAVGUSB',
};


# PF2ID-Packed Floating-Point to 32-bit Integer.
ENCODING PF2ID => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0x1d',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3N_PF2ID',
};


# PFACC-Packed Floating-Point Accumulate.
ENCODING PFACC => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0xae',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3N_PFACC',
};


# PFADD-Packed Floating-Point Addition.
ENCODING PFADD => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0x9e',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3N_PFADD',
};


# PFCMPEQ-Packed Floating-Point Comparison, Equal.
ENCODING PFCMPEQ => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0xb0',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3N_PFCMPEQ',
};


# PFCMPGE-Packed Floating-Point Comparison, Greater or Equal.
ENCODING PFCMPGE => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0x90',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3N_PFCMPGE',
};


# PFCMPGT-Packed Floating-Point Comparison, Greater.
ENCODING PFCMPGT => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0xa0',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3N_PFCMPGT',
};


# PFMAX-Packed Floating-Point Maximum.
ENCODING PFMAX => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0xa4',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3N_PFMAX',
};


# PFMIN-Packed Floating-Point Minimum.
ENCODING PFMIN => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0x94',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3N_PFMIN',
};


# PFMUL-Packed Floating-Point Multiplication.
ENCODING PFMUL => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0xb4',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3N_PFMUL',
};


# PFRCP-Packed Floating-Point Reciprocal Approximation.
ENCODING PFRCP => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0x96',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3N_PFRCP',
};


# PFRCPIT2-Packed Floating-Point Reciprocal/Reciprocal Square Root Second Iteration Step.
ENCODING PFRCPIT2 => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0xb6',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3N_PFRCPIT2',
};


# PFRSQIT1-Packed Floating-Point Reciprocal Square Root First Iteration Step.
ENCODING PFRSQIT1 => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0xa7',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3N_PFRSQIT1',
};


# PFSUB-Packed Floating-Point Subtraction.
ENCODING PFSUB => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0x9a',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3N_PFSUB',
};


# PFSUBR-Packed Floating-Point Reverse Subtraction.
ENCODING PFSUBR => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0xaa',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3N_PFSUBR',
};


# PI2FD-Packed 32-bit Integer to Floating-Point Conversion.
ENCODING PI2FD => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0x0d',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3N_PI2FD',
};


# PMULHRW-Packed 16-bit Integer Multiply with rounding.
ENCODING PMULHRW => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0xb7',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=D3N_PMULHRW',
};


# PREFETCH/PREFETCHW* Prefetch at least a 32-byte line into L1 data cache (Dcache).
ENCODING PREFETCH_mb_1 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x0d REG=0',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=PREFETCH_NOP deprecated=1',
  tags       => 'page=D3N_PREFETCH',
};

ENCODING PREFETCH_mb_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x0d REG=3',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=PREFETCH_NOP deprecated=1',
  tags       => 'page=D3N_PREFETCH',
};

ENCODING PREFETCH_mb_3 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x0d REG=4',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=PREFETCH_NOP deprecated=1',
  tags       => 'page=D3N_PREFETCH',
};

ENCODING PREFETCH_mb_4 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x0d REG=5',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=PREFETCH_NOP deprecated=1',
  tags       => 'page=D3N_PREFETCH',
};

ENCODING PREFETCH_mb_5 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x0d REG=6',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=PREFETCH_NOP deprecated=1',
  tags       => 'page=D3N_PREFETCH',
};

ENCODING PREFETCH_mb_6 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x0d REG=7',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=PREFETCH_NOP deprecated=1',
  tags       => 'page=D3N_PREFETCH',
};


# EMMS-Empty MMX Technology State.
ENCODING EMMS => {
  diagram    => 'MAP=0f OP=0x77 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD',
  metadata   => 'isa=PENTIUMMMX',
  docvars    => 'X87_MMX_STATE_W=1',
  tags       => 'page=EMMS',
};


# MASKMOVQ-Store Selected Bytes of Quadword.
ENCODING MASKMOVQ => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xf7 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_NOFP2',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=MASKMOVQ',
};


# MOVNTQ-Store of Quadword Using Non-Temporal Hint.
ENCODING MOVNTQ => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xe7 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_NOFP2',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=MOVNTQ',
};


# MOVQ-Move Quadword.
ENCODING MOVQ_mqrq_1 => {
  diagram    => 'MAP=0f MODE=64 MR=1 OP=0x7e P66=0 PF2=0 PF3=0 W=1',
  extensions => 'MMX',
  exceptions => 'MMX_NOFP2',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=MOVQ',
};

ENCODING MOVQ_mqrq_3 => {
  diagram    => 'MAP=0f MR=1 OP=0x7f P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_NOFP2',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=MOVQ',
};

ENCODING MOVQ_rqmq_1 => {
  diagram    => 'MAP=0f MODE=64 MR=1 OP=0x6e P66=0 PF2=0 PF3=0 W=1',
  extensions => 'MMX',
  exceptions => 'MMX_NOFP2',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=MOVQ',
};

ENCODING MOVQ_rqmq_2 => {
  diagram    => 'MAP=0f MR=1 OP=0x6f P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_NOFP2',
  categories => 'FPSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=MOVQ',
};


# MOVD/MOVQ-Move Doubleword/Move Quadword.
ENCODING MOVD_mdrd_1 => {
  diagram    => 'MAP=0f MR=1 OP=0x7e P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=MOVx',
};

ENCODING MOVD_rqmd => {
  diagram    => 'MAP=0f MR=1 OP=0x6e P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=MOVx',
};


# PACKSSWB/PACKSSDW-Pack with Signed Saturation.
ENCODING PACKSSDW_rqmq_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x6b P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|MISC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PACKSSxx',
};

ENCODING PACKSSDW_rqmq_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x6b P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|MISC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PACKSSxx',
};

ENCODING PACKSSWB_rqmq_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x63 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|MISC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PACKSSxx',
};

ENCODING PACKSSWB_rqmq_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x63 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|MISC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PACKSSxx',
};


# PACKUSWB-Pack with Unsigned Saturation.
ENCODING PACKUSWB_rqmq_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x67 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|MISC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PACKUSWB',
};

ENCODING PACKUSWB_rqmq_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x67 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|MISC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PACKUSWB',
};


# PADDSB/PADDSW-Add Packed Signed Integers with Signed Saturation.
ENCODING PADDSB_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xec P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PADDSx',
};

ENCODING PADDSW_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xed P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PADDSx',
};


# PADDUSB/PADDUSW-Add Packed Unsigned Integers with Unsigned Saturation.
ENCODING PADDUSB_rqmq_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xdc P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PADDUSx',
};

ENCODING PADDUSB_rqmq_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xdc P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PADDUSx',
};

ENCODING PADDUSW_rqmq_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xdd P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PADDUSx',
};

ENCODING PADDUSW_rqmq_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xdd P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PADDUSx',
};


# PADDB/PADDW/PADDD/PADDQ-Add Packed Integers.
ENCODING PADDB_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xfc P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PADDx',
};

ENCODING PADDD_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xfe P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PADDx',
};

ENCODING PADDW_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xfd P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PADDx',
};


# PAND-Logical AND.
ENCODING PAND_rqmq_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xdb P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PAND',
};

ENCODING PAND_rqmq_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xdb P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PAND',
};


# PANDN-Logical AND NOT.
ENCODING PANDN_rqmq_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xdf P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PANDN',
};

ENCODING PANDN_rqmq_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xdf P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PANDN',
};


# PAVGB/PAVGW-Average Packed Integers.
ENCODING PAVGB_rqmq_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xe0 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|STAT',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PAVGx',
};

ENCODING PAVGB_rqmq_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xe0 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|STAT',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PAVGx',
};

ENCODING PAVGW_rqmq_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xe3 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|STAT',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PAVGx',
};

ENCODING PAVGW_rqmq_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xe3 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|STAT',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PAVGx',
};


# PCMPEQB/PCMPEQW/PCMPEQD-Compare Packed Data for Equal.
ENCODING PCMPEQB_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0x74 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|COMPARISON',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PCMPEQx',
};

ENCODING PCMPEQD_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0x76 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PCMPEQx',
};

ENCODING PCMPEQW_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0x75 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|COMPARISON',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PCMPEQx',
};


# PCMPGTB/PCMPGTW/PCMPGTD-Compare Packed Signed Integers for Greater Than.
ENCODING PCMPGTB_rqmq_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x64 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|COMPARISON',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PCMPGTx',
};

ENCODING PCMPGTB_rqmq_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x64 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|COMPARISON',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PCMPGTx',
};

ENCODING PCMPGTD_rqmq_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x66 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|COMPARISON',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PCMPGTx',
};

ENCODING PCMPGTD_rqmq_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x66 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|COMPARISON',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PCMPGTx',
};

ENCODING PCMPGTW_rqmq_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x65 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|COMPARISON',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PCMPGTx',
};

ENCODING PCMPGTW_rqmq_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x65 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|COMPARISON',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PCMPGTx',
};


# PEXTRW-Extract Word.
ENCODING PEXTRW_rdmqub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xc5 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_NOMEM',
  categories => 'FPSIMD|SWIZZLE',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PEXTRW',
};


# PFCPIT1.
ENCODING PFCPIT1 => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0xa6',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=PFCPIT1',
};


# PFSQRT.
ENCODING PFSQRT => {
  diagram    => 'ENC=3DNOW MAP=0f0f MR=1 OP=0x97',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=AMD3DNOW deprecated=1',
  tags       => 'page=PFSQRT',
};


# PINSRW-Insert Word.
ENCODING PINSRW_rqmxub => {
  diagram    => 'MAP=0f MR=1 OP=0xc4 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SWIZZLE',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PINSRW',
};


# PMADDWD-Multiply and Add Packed Integers.
ENCODING PMADDWD_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xf5 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|CARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PMADDWD',
};


# PMAXSB/PMAXSW/PMAXSD/PMAXSQ-Maximum of Packed Signed Integers.
ENCODING PMAXSW_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xee P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|MATH|MIN_MAX',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PMAXSx',
};


# PMAXUB/PMAXUW-Maximum of Packed Unsigned Integers.
ENCODING PMAXUB_rqmq_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xde P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|MATH|MIN_MAX',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PMAXUbw',
};

ENCODING PMAXUB_rqmq_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xde P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|MATH|MIN_MAX',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PMAXUbw',
};


# PMINSB/PMINSW-Minimum of Packed Signed Integers.
ENCODING PMINSW_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xea P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|MATH|MIN_MAX',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PMINSbw',
};


# PMINUB/PMINUW-Minimum of Packed Unsigned Integers.
ENCODING PMINUB_rqmq_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xda P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|MATH|MIN_MAX',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PMINUbw',
};

ENCODING PMINUB_rqmq_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xda P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|MATH|MIN_MAX',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PMINUbw',
};


# PMOVMSKB-Move Byte Mask.
ENCODING PMOVMSKB_rdmq => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xd7 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_NOMEM',
  categories => 'FPSIMD|MISC',
  metadata   => 'isa=SSE',
  tags       => 'page=PMOVMSKB',
};


# PMULHUW-Multiply Packed Unsigned Integers and Store High Result.
ENCODING PMULHUW_rqmq_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xe4 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PMULHUW',
};

ENCODING PMULHUW_rqmq_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xe4 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PMULHUW',
};


# PMULHW-Multiply Packed Signed Integers and Store High Result.
ENCODING PMULHW_rqmq_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xe5 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PMULHW',
};

ENCODING PMULHW_rqmq_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xe5 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PMULHW',
};


# PMULLW-Multiply Packed Signed Integers and Store Low Result.
ENCODING PMULLW_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xd5 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PMULLW',
};


# POR-Bitwise Logical OR.
ENCODING POR_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xeb P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=POR',
};


# PREFETCHW-Prefetch Data into Caches in Anticipation of a Write.
ENCODING PREFETCHW => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x0d REG=1',
  extensions => 'AMD3DNOW',
  categories => 'FPSIMD',
  metadata   => 'isa=PREFETCH_NOP deprecated=1',
  tags       => 'page=PREFETCHW',
};


# PSADBW-Compute Sum of Absolute Differences.
ENCODING PSADBW_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xf6 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC|MISC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSADBW',
};


# PSHUFW-Shuffle Packed Words.
ENCODING PSHUFW => {
  diagram    => 'MAP=0f MR=1 OP=0x70 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SWIZZLE',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSHUFW',
};


# PSLLW/PSLLD/PSLLQ-Shift Packed Data Left Logical.
ENCODING PSLLD_mqub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x72 P66=0 PF2=0 PF3=0 REG=6',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSLLx',
};

ENCODING PSLLD_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xf2 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSLLx',
};

ENCODING PSLLQ_mqub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x73 P66=0 PF2=0 PF3=0 REG=6',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSLLx',
};

ENCODING PSLLQ_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xf3 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSLLx',
};

ENCODING PSLLW_mqub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x71 P66=0 PF2=0 PF3=0 REG=6',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSLLx',
};

ENCODING PSLLW_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xf1 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSLLx',
};


# PSRAW/PSRAD/PSRAQ-Shift Packed Data Right Arithmetic.
ENCODING PSRAD_mqub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x72 P66=0 PF2=0 PF3=0 REG=4',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSRAx',
};

ENCODING PSRAD_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xe2 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSRAx',
};

ENCODING PSRAW_mqub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x71 P66=0 PF2=0 PF3=0 REG=4',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSRAx',
};

ENCODING PSRAW_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xe1 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSRAx',
};


# PSRLW/PSRLD/PSRLQ-Shift Packed Data Right Logical.
ENCODING PSRLD_mqub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x72 P66=0 PF2=0 PF3=0 REG=2',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSRLx',
};

ENCODING PSRLD_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xd2 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSRLx',
};

ENCODING PSRLQ_mqub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x73 P66=0 PF2=0 PF3=0 REG=2',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSRLx',
};

ENCODING PSRLQ_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xd3 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSRLx',
};

ENCODING PSRLW_mqub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x71 P66=0 PF2=0 PF3=0 REG=2',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSRLx',
};

ENCODING PSRLW_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xd1 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSRLx',
};


# PSUBSB/PSUBSW-Subtract Packed Signed Integers with Signed Saturation.
ENCODING PSUBSB_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xe8 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSUBSx',
};

ENCODING PSUBSW_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xe9 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSUBSx',
};


# PSUBUSB/PSUBUSW-Subtract Packed Unsigned Integers with Unsigned Saturation.
ENCODING PSUBUSB_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xd8 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSUBUSx',
};

ENCODING PSUBUSW_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xd9 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSUBUSx',
};


# PSUBB/PSUBW/PSUBD-Subtract Packed Integers.
ENCODING PSUBB_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xf8 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSUBx',
};

ENCODING PSUBD_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xfa P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSUBx',
};

ENCODING PSUBW_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xf9 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|ARITHMETIC',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PSUBx',
};


# PUNPCKHBW/PUNPCKHWD/PUNPCKHDQ/PUNPCKHQDQ-Unpack High Data.
ENCODING PUNPCKHBW_rqmx => {
  diagram    => 'MAP=0f MR=1 OP=0x68 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SWIZZLE',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PUNPCKHxx',
};

ENCODING PUNPCKHDQ_rqmx => {
  diagram    => 'MAP=0f MR=1 OP=0x6a P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SWIZZLE',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PUNPCKHxx',
};

ENCODING PUNPCKHWD_rqmx => {
  diagram    => 'MAP=0f MR=1 OP=0x69 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SWIZZLE',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PUNPCKHxx',
};


# PUNPCKLBW/PUNPCKLWD/PUNPCKLDQ/PUNPCKLQDQ-Unpack Low Data.
ENCODING PUNPCKLBW_rqmd => {
  diagram    => 'MAP=0f MR=1 OP=0x60 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SWIZZLE',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PUNPCKLxx',
};

ENCODING PUNPCKLDQ_rqmd => {
  diagram    => 'MAP=0f MR=1 OP=0x62 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  categories => 'FPSIMD|SWIZZLE',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PUNPCKLxx',
};

ENCODING PUNPCKLWD_rqmd => {
  diagram    => 'MAP=0f MR=1 OP=0x61 P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|SWIZZLE',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PUNPCKLxx',
};


# PXOR-Logical Exclusive OR.
ENCODING PXOR_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xef P66=0 PF2=0 PF3=0',
  extensions => 'MMX',
  exceptions => 'MMX_MEM',
  categories => 'FPSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=PENTIUMMMX',
  tags       => 'page=PXOR',
};

