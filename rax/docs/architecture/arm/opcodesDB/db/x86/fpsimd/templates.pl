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
T['PF2IW REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PF2IW', ''];
T['PF2IW REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PF2IW', ''];

# PFNACC-Packed Floating-Point Negative Accumulate.
T['PFNACC REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PFNACC', ''];
T['PFNACC REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PFNACC', ''];

# PFPNACC-Packed Floating-Point Mixed Positive-Negative Accumulate.
T['PFPNACC REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PFPNACC', ''];
T['PFPNACC REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PFPNACC', ''];

# PI2FW-Packed Integer Word to Floating-Point Conversion.
T['PI2FW REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PI2FW', ''];
T['PI2FW REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PI2FW', ''];

# PSWAPD-Packed Swap Doubleword.
T['PSWAPD REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PSWAPD', ''];
T['PSWAPD REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PSWAPD', ''];

# FEMMS-Faster entry/exit of the MMX or floating-point state.
T['FEMMS', 'NONE', 'FEMMS', ''];

# PAVGUSB-Packed 8-bit Unsigned Integer Averaging.
T['PAVGUSB REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PAVGUSB', ''];
T['PAVGUSB REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PAVGUSB', ''];

# PF2ID-Packed Floating-Point to 32-bit Integer.
T['PF2ID REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PF2ID', ''];
T['PF2ID REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PF2ID', ''];

# PFACC-Packed Floating-Point Accumulate.
T['PFACC REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PFACC', ''];
T['PFACC REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PFACC', ''];

# PFADD-Packed Floating-Point Addition.
T['PFADD REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PFADD', ''];
T['PFADD REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PFADD', ''];

# PFCMPEQ-Packed Floating-Point Comparison, Equal.
T['PFCMPEQ REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PFCMPEQ', ''];
T['PFCMPEQ REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PFCMPEQ', ''];

# PFCMPGE-Packed Floating-Point Comparison, Greater or Equal.
T['PFCMPGE REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PFCMPGE', ''];
T['PFCMPGE REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PFCMPGE', ''];

# PFCMPGT-Packed Floating-Point Comparison, Greater.
T['PFCMPGT REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PFCMPGT', ''];
T['PFCMPGT REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PFCMPGT', ''];

# PFMAX-Packed Floating-Point Maximum.
T['PFMAX REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PFMAX', ''];
T['PFMAX REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PFMAX', ''];

# PFMIN-Packed Floating-Point Minimum.
T['PFMIN REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PFMIN', ''];
T['PFMIN REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PFMIN', ''];

# PFMUL-Packed Floating-Point Multiplication.
T['PFMUL REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PFMUL', ''];
T['PFMUL REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PFMUL', ''];

# PFRCP-Packed Floating-Point Reciprocal Approximation.
T['PFRCP REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PFRCP', ''];
T['PFRCP REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PFRCP', ''];

# PFRCPIT2-Packed Floating-Point Reciprocal/Reciprocal Square Root Second Iteration Step.
T['PFRCPIT2 REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PFRCPIT2', ''];
T['PFRCPIT2 REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PFRCPIT2', ''];

# PFRSQIT1-Packed Floating-Point Reciprocal Square Root First Iteration Step.
T['PFRSQIT1 REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PFRSQIT1', ''];
T['PFRSQIT1 REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PFRSQIT1', ''];

# PFSUB-Packed Floating-Point Subtraction.
T['PFSUB REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PFSUB', ''];
T['PFSUB REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PFSUB', ''];

# PFSUBR-Packed Floating-Point Reverse Subtraction.
T['PFSUBR REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PFSUBR', ''];
T['PFSUBR REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PFSUBR', ''];

# PI2FD-Packed 32-bit Integer to Floating-Point Conversion.
T['PI2FD REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PI2FD', ''];
T['PI2FD REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PI2FD', ''];

# PMULHRW-Packed 16-bit Integer Multiply with rounding.
T['PMULHRW REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PMULHRW', ''];
T['PMULHRW REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PMULHRW', ''];

# PREFETCH/PREFETCHW* Prefetch at least a 32-byte line into L1 data cache (Dcache).
T['PREFETCH MEM:r:s64=$mem8', 'NONE', 'PREFETCH_mb_1', ''];
T['PREFETCH MEM:r:s64=$mem8', 'NONE', 'PREFETCH_mb_2', ''];
T['PREFETCH MEM:r:s64=$mem8', 'NONE', 'PREFETCH_mb_3', ''];
T['PREFETCH MEM:r:s64=$mem8', 'NONE', 'PREFETCH_mb_4', ''];
T['PREFETCH MEM:r:s64=$mem8', 'NONE', 'PREFETCH_mb_5', ''];
T['PREFETCH MEM:r:s64=$mem8', 'NONE', 'PREFETCH_mb_6', ''];

# EMMS-Empty MMX Technology State.
T['EMMS', 'NONE', 'EMMS', ''];

# MASKMOVQ-Store Selected Bytes of Quadword.
T['MASKMOVQ REG:r:u8=$MMXr REG:r:s8=$MMXm MEM:SUPP:w:s64={SEG:r=DS BASE:r=$DIa SZ=64}', 'NONE', 'MASKMOVQ', ''];

# MOVNTQ-Store of Quadword Using Non-Temporal Hint.
T['MOVNTQ MEM:w:s64=$mem64 REG:r:s64=$MMXr', 'NONE', 'MOVNTQ', ''];

# MOVQ-Move Quadword.
T['MOVQ REG:w=$GPR64m    REG:r:s64=$MMXr ', 'MOD=REG', 'MOVQ_mqrq_1', ''];
T['MOVQ MEM:w:s64=$mem64 REG:r:s64=$MMXr ', 'MOD=MEM', 'MOVQ_mqrq_1', ''];
T['MOVQ REG:w:s64=$MMXm  REG:r:s64=$MMXr ', 'MOD=REG', 'MOVQ_mqrq_3', ''];
T['MOVQ MEM:w:s64=$mem64 REG:r:s64=$MMXr ', 'MOD=MEM', 'MOVQ_mqrq_3', ''];
T['MOVQ REG:w:s64=$MMXr  REG:r=$GPR64m   ', 'MOD=REG', 'MOVQ_rqmq_1', ''];
T['MOVQ REG:w:s64=$MMXr  MEM:r:s64=$mem64', 'MOD=MEM', 'MOVQ_rqmq_1', ''];
T['MOVQ REG:w:s64=$MMXr  REG:r:s64=$MMXm ', 'MOD=REG', 'MOVQ_rqmq_2', ''];
T['MOVQ REG:w:s64=$MMXr  MEM:r:s64=$mem64', 'MOD=MEM', 'MOVQ_rqmq_2', ''];

# MOVD/MOVQ-Move Doubleword/Move Quadword.
T['MOVD REG:w=$GPR32m    REG:r:s32=$MMXr ', 'MOD=REG MODE=64 W=0', 'MOVD_mdrd_1', ''];
T['MOVD MEM:w:s32=$mem32 REG:r:s32=$MMXr ', 'MOD=MEM MODE=64 W=0', 'MOVD_mdrd_1', ''];
T['MOVD REG:w=$GPR32m    REG:r:s32=$MMXr ', 'MOD=REG MODE=NO64  ', 'MOVD_mdrd_1', ''];
T['MOVD MEM:w:s32=$mem32 REG:r:s32=$MMXr ', 'MOD=MEM MODE=NO64  ', 'MOVD_mdrd_1', ''];
T['MOVD REG:w:s64=$MMXr  REG:r=$GPR32m   ', 'MOD=REG MODE=64 W=0', 'MOVD_rqmd  ', ''];
T['MOVD REG:w:s64=$MMXr  MEM:r:s32=$mem32', 'MOD=MEM MODE=64 W=0', 'MOVD_rqmd  ', ''];
T['MOVD REG:w:s64=$MMXr  REG:r=$GPR32m   ', 'MOD=REG MODE=NO64  ', 'MOVD_rqmd  ', ''];
T['MOVD REG:w:s64=$MMXr  MEM:r:s32=$mem32', 'MOD=MEM MODE=NO64  ', 'MOVD_rqmd  ', ''];

# PACKSSWB/PACKSSDW-Pack with Signed Saturation.
T['PACKSSDW REG:rw:s32=$MMXr REG:r:s32=$MMXm ', 'NONE', 'PACKSSDW_rqmq_1', ''];
T['PACKSSDW REG:rw:s32=$MMXr MEM:r:s32=$mem64', 'NONE', 'PACKSSDW_rqmq_2', ''];
T['PACKSSWB REG:rw:s16=$MMXr REG:r:s16=$MMXm ', 'NONE', 'PACKSSWB_rqmq_1', ''];
T['PACKSSWB REG:rw:s16=$MMXr MEM:r:s16=$mem64', 'NONE', 'PACKSSWB_rqmq_2', ''];

# PACKUSWB-Pack with Unsigned Saturation.
T['PACKUSWB REG:rw:s16=$MMXr REG:r:s16=$MMXm ', 'NONE', 'PACKUSWB_rqmq_1', ''];
T['PACKUSWB REG:rw:s16=$MMXr MEM:r:s16=$mem64', 'NONE', 'PACKUSWB_rqmq_2', ''];

# PADDSB/PADDSW-Add Packed Signed Integers with Signed Saturation.
T['PADDSB REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PADDSB_rqmq', ''];
T['PADDSB REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PADDSB_rqmq', ''];
T['PADDSW REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PADDSW_rqmq', ''];
T['PADDSW REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PADDSW_rqmq', ''];

# PADDUSB/PADDUSW-Add Packed Unsigned Integers with Unsigned Saturation.
T['PADDUSB REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'NONE', 'PADDUSB_rqmq_1', ''];
T['PADDUSB REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'NONE', 'PADDUSB_rqmq_2', ''];
T['PADDUSW REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'NONE', 'PADDUSW_rqmq_1', ''];
T['PADDUSW REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'NONE', 'PADDUSW_rqmq_2', ''];

# PADDB/PADDW/PADDD/PADDQ-Add Packed Integers.
T['PADDB REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PADDB_rqmq', ''];
T['PADDB REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PADDB_rqmq', ''];
T['PADDD REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PADDD_rqmq', ''];
T['PADDD REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PADDD_rqmq', ''];
T['PADDW REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PADDW_rqmq', ''];
T['PADDW REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PADDW_rqmq', ''];

# PAND-Logical AND.
T['PAND REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'NONE', 'PAND_rqmq_1', ''];
T['PAND REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'NONE', 'PAND_rqmq_2', ''];

# PANDN-Logical AND NOT.
T['PANDN REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'NONE', 'PANDN_rqmq_1', ''];
T['PANDN REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'NONE', 'PANDN_rqmq_2', ''];

# PAVGB/PAVGW-Average Packed Integers.
T['PAVGB REG:rw:s8=$MMXr  REG:r:s8=$MMXm  ', 'NONE', 'PAVGB_rqmq_1', ''];
T['PAVGB REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'NONE', 'PAVGB_rqmq_2', ''];
T['PAVGW REG:rw:s16=$MMXr REG:r:s16=$MMXm ', 'NONE', 'PAVGW_rqmq_1', ''];
T['PAVGW REG:rw:s16=$MMXr MEM:r:s16=$mem64', 'NONE', 'PAVGW_rqmq_2', ''];

# PCMPEQB/PCMPEQW/PCMPEQD-Compare Packed Data for Equal.
T['PCMPEQB REG:rw:s8=$MMXr  REG:r:s8=$MMXm  ', 'MOD=REG', 'PCMPEQB_rqmq', ''];
T['PCMPEQB REG:rw:s8=$MMXr  MEM:r:s8=$mem64 ', 'MOD=MEM', 'PCMPEQB_rqmq', ''];
T['PCMPEQD REG:rw:s32=$MMXr REG:r:s32=$MMXm ', 'MOD=REG', 'PCMPEQD_rqmq', ''];
T['PCMPEQD REG:rw:s32=$MMXr MEM:r:s32=$mem64', 'MOD=MEM', 'PCMPEQD_rqmq', ''];
T['PCMPEQW REG:rw:s16=$MMXr REG:r:s16=$MMXm ', 'MOD=REG', 'PCMPEQW_rqmq', ''];
T['PCMPEQW REG:rw:s16=$MMXr MEM:r:s16=$mem64', 'MOD=MEM', 'PCMPEQW_rqmq', ''];

# PCMPGTB/PCMPGTW/PCMPGTD-Compare Packed Signed Integers for Greater Than.
T['PCMPGTB REG:rw:s8=$MMXr  REG:r:s8=$MMXm  ', 'NONE', 'PCMPGTB_rqmq_1', ''];
T['PCMPGTB REG:rw:s8=$MMXr  MEM:r:s8=$mem64 ', 'NONE', 'PCMPGTB_rqmq_2', ''];
T['PCMPGTD REG:rw:s32=$MMXr REG:r:s32=$MMXm ', 'NONE', 'PCMPGTD_rqmq_1', ''];
T['PCMPGTD REG:rw:s32=$MMXr MEM:r:s32=$mem64', 'NONE', 'PCMPGTD_rqmq_2', ''];
T['PCMPGTW REG:rw:s16=$MMXr REG:r:s16=$MMXm ', 'NONE', 'PCMPGTW_rqmq_1', ''];
T['PCMPGTW REG:rw:s16=$MMXr MEM:r:s16=$mem64', 'NONE', 'PCMPGTW_rqmq_2', ''];

# PEXTRW-Extract Word.
T['PEXTRW REG:w=$GPR32r REG:r:u16=$MMXm IMM:u8=$index', 'NONE', 'PEXTRW_rdmqub', ''];

# PFCPIT1.
T['PFCPIT1 REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PFCPIT1', ''];
T['PFCPIT1 REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PFCPIT1', ''];

# PFSQRT.
T['PFSQRT REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PFSQRT', ''];
T['PFSQRT REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PFSQRT', ''];

# PINSRW-Insert Word.
T['PINSRW REG:rw:u16=$MMXr REG:r=$GPR32m    IMM:u8=$index', 'MOD=REG', 'PINSRW_rqmxub', ''];
T['PINSRW REG:rw:u16=$MMXr MEM:r:u16=$mem16 IMM:u8=$index', 'MOD=MEM', 'PINSRW_rqmxub', ''];

# PMADDWD-Multiply and Add Packed Integers.
T['PMADDWD REG:rw:s16=$MMXr REG:r:s16=$MMXm ', 'MOD=REG', 'PMADDWD_rqmq', ''];
T['PMADDWD REG:rw:s16=$MMXr MEM:r:s16=$mem64', 'MOD=MEM', 'PMADDWD_rqmq', ''];

# PMAXSB/PMAXSW/PMAXSD/PMAXSQ-Maximum of Packed Signed Integers.
T['PMAXSW REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PMAXSW_rqmq', ''];
T['PMAXSW REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PMAXSW_rqmq', ''];

# PMAXUB/PMAXUW-Maximum of Packed Unsigned Integers.
T['PMAXUB REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'NONE', 'PMAXUB_rqmq_1', ''];
T['PMAXUB REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'NONE', 'PMAXUB_rqmq_2', ''];

# PMINSB/PMINSW-Minimum of Packed Signed Integers.
T['PMINSW REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PMINSW_rqmq', ''];
T['PMINSW REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PMINSW_rqmq', ''];

# PMINUB/PMINUW-Minimum of Packed Unsigned Integers.
T['PMINUB REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'NONE', 'PMINUB_rqmq_1', ''];
T['PMINUB REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'NONE', 'PMINUB_rqmq_2', ''];

# PMOVMSKB-Move Byte Mask.
T['PMOVMSKB REG:w=$GPR32r REG:r:s8=$MMXm', 'NONE', 'PMOVMSKB_rdmq', ''];

# PMULHUW-Multiply Packed Unsigned Integers and Store High Result.
T['PMULHUW REG:rw:u16=$MMXr REG:r:u16=$MMXm ', 'NONE', 'PMULHUW_rqmq_1', ''];
T['PMULHUW REG:rw:u16=$MMXr MEM:r:u16=$mem64', 'NONE', 'PMULHUW_rqmq_2', ''];

# PMULHW-Multiply Packed Signed Integers and Store High Result.
T['PMULHW REG:rw:s16=$MMXr REG:r:s16=$MMXm ', 'NONE', 'PMULHW_rqmq_1', ''];
T['PMULHW REG:rw:s16=$MMXr MEM:r:s16=$mem64', 'NONE', 'PMULHW_rqmq_2', ''];

# PMULLW-Multiply Packed Signed Integers and Store Low Result.
T['PMULLW REG:rw:s16=$MMXr REG:r:s16=$MMXm ', 'MOD=REG', 'PMULLW_rqmq', ''];
T['PMULLW REG:rw:s16=$MMXr MEM:r:s16=$mem64', 'MOD=MEM', 'PMULLW_rqmq', ''];

# POR-Bitwise Logical OR.
T['POR REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'POR_rqmq', ''];
T['POR REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'POR_rqmq', ''];

# PREFETCHW-Prefetch Data into Caches in Anticipation of a Write.
T['PREFETCHW MEM:r:s64=$mem8', 'NONE', 'PREFETCHW', ''];

# PSADBW-Compute Sum of Absolute Differences.
T['PSADBW REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PSADBW_rqmq', ''];
T['PSADBW REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PSADBW_rqmq', ''];

# PSHUFW-Shuffle Packed Words.
T['PSHUFW REG:w:u16=$MMXr REG:r:u16=$MMXm  ORDER:r:u8=$order', 'MOD=REG', 'PSHUFW', ''];
T['PSHUFW REG:w:u16=$MMXr MEM:r:u16=$mem64 ORDER:r:u8=$order', 'MOD=MEM', 'PSHUFW', ''];

# PSLLW/PSLLD/PSLLQ-Shift Packed Data Left Logical.
T['PSLLD REG:rw:u32=$MMXm SHIFT:r:u8=$shl ', 'NONE   ', 'PSLLD_mqub', ''];
T['PSLLD REG:rw:u32=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PSLLD_rqmq', ''];
T['PSLLD REG:rw:u32=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PSLLD_rqmq', ''];
T['PSLLQ REG:rw:u64=$MMXm SHIFT:r:u8=$shl ', 'NONE   ', 'PSLLQ_mqub', ''];
T['PSLLQ REG:rw:u64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PSLLQ_rqmq', ''];
T['PSLLQ REG:rw:u64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PSLLQ_rqmq', ''];
T['PSLLW REG:rw:u16=$MMXm SHIFT:r:u8=$shl ', 'NONE   ', 'PSLLW_mqub', ''];
T['PSLLW REG:rw:u16=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PSLLW_rqmq', ''];
T['PSLLW REG:rw:u16=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PSLLW_rqmq', ''];

# PSRAW/PSRAD/PSRAQ-Shift Packed Data Right Arithmetic.
T['PSRAD REG:rw:s32=$MMXm SHIFT:r:u8=$shr ', 'NONE   ', 'PSRAD_mqub', ''];
T['PSRAD REG:rw:s32=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PSRAD_rqmq', ''];
T['PSRAD REG:rw:s32=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PSRAD_rqmq', ''];
T['PSRAW REG:rw:s16=$MMXm SHIFT:r:u8=$shr ', 'NONE   ', 'PSRAW_mqub', ''];
T['PSRAW REG:rw:s16=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PSRAW_rqmq', ''];
T['PSRAW REG:rw:s16=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PSRAW_rqmq', ''];

# PSRLW/PSRLD/PSRLQ-Shift Packed Data Right Logical.
T['PSRLD REG:rw:u32=$MMXm SHIFT:r:u8=$shr ', 'NONE   ', 'PSRLD_mqub', ''];
T['PSRLD REG:rw:u32=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PSRLD_rqmq', ''];
T['PSRLD REG:rw:u32=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PSRLD_rqmq', ''];
T['PSRLQ REG:rw:u64=$MMXm SHIFT:r:u8=$shr ', 'NONE   ', 'PSRLQ_mqub', ''];
T['PSRLQ REG:rw:u64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PSRLQ_rqmq', ''];
T['PSRLQ REG:rw:u64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PSRLQ_rqmq', ''];
T['PSRLW REG:rw:u16=$MMXm SHIFT:r:u8=$shr ', 'NONE   ', 'PSRLW_mqub', ''];
T['PSRLW REG:rw:u16=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PSRLW_rqmq', ''];
T['PSRLW REG:rw:u16=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PSRLW_rqmq', ''];

# PSUBSB/PSUBSW-Subtract Packed Signed Integers with Signed Saturation.
T['PSUBSB REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PSUBSB_rqmq', ''];
T['PSUBSB REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PSUBSB_rqmq', ''];
T['PSUBSW REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PSUBSW_rqmq', ''];
T['PSUBSW REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PSUBSW_rqmq', ''];

# PSUBUSB/PSUBUSW-Subtract Packed Unsigned Integers with Unsigned Saturation.
T['PSUBUSB REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PSUBUSB_rqmq', ''];
T['PSUBUSB REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PSUBUSB_rqmq', ''];
T['PSUBUSW REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PSUBUSW_rqmq', ''];
T['PSUBUSW REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PSUBUSW_rqmq', ''];

# PSUBB/PSUBW/PSUBD-Subtract Packed Integers.
T['PSUBB REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PSUBB_rqmq', ''];
T['PSUBB REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PSUBB_rqmq', ''];
T['PSUBD REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PSUBD_rqmq', ''];
T['PSUBD REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PSUBD_rqmq', ''];
T['PSUBW REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PSUBW_rqmq', ''];
T['PSUBW REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PSUBW_rqmq', ''];

# PUNPCKHBW/PUNPCKHWD/PUNPCKHDQ/PUNPCKHQDQ-Unpack High Data.
T['PUNPCKHBW REG:rw:s64=$MMXr REG:r:s32=$MMXm ', 'MOD=REG', 'PUNPCKHBW_rqmx', ''];
T['PUNPCKHBW REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PUNPCKHBW_rqmx', ''];
T['PUNPCKHDQ REG:rw:s64=$MMXr REG:r:s32=$MMXm ', 'MOD=REG', 'PUNPCKHDQ_rqmx', ''];
T['PUNPCKHDQ REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PUNPCKHDQ_rqmx', ''];
T['PUNPCKHWD REG:rw:s64=$MMXr REG:r:s32=$MMXm ', 'MOD=REG', 'PUNPCKHWD_rqmx', ''];
T['PUNPCKHWD REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PUNPCKHWD_rqmx', ''];

# PUNPCKLBW/PUNPCKLWD/PUNPCKLDQ/PUNPCKLQDQ-Unpack Low Data.
T['PUNPCKLBW REG:rw:u8=$MMXr  REG:r:u8=$MMXm  ', 'MOD=REG', 'PUNPCKLBW_rqmd', ''];
T['PUNPCKLBW REG:rw:u8=$MMXr  MEM:r:u8=$mem32 ', 'MOD=MEM', 'PUNPCKLBW_rqmd', ''];
T['PUNPCKLDQ REG:rw:u32=$MMXr REG:r:u32=$MMXm ', 'MOD=REG', 'PUNPCKLDQ_rqmd', ''];
T['PUNPCKLDQ REG:rw:u32=$MMXr MEM:r:u32=$mem32', 'MOD=MEM', 'PUNPCKLDQ_rqmd', ''];
T['PUNPCKLWD REG:rw:u16=$MMXr REG:r:u16=$MMXm ', 'MOD=REG', 'PUNPCKLWD_rqmd', ''];
T['PUNPCKLWD REG:rw:u16=$MMXr MEM:r:u16=$mem32', 'MOD=MEM', 'PUNPCKLWD_rqmd', ''];

# PXOR-Logical Exclusive OR.
T['PXOR REG:rw:s64=$MMXr REG:r:s64=$MMXm ', 'MOD=REG', 'PXOR_rqmq', ''];
T['PXOR REG:rw:s64=$MMXr MEM:r:s64=$mem64', 'MOD=MEM', 'PXOR_rqmq', ''];
