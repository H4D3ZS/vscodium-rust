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


# ADDPD-Add Packed Double-Precision Floating-Point Values.
T['ADDPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG', 'ADDPD', ''];
T['ADDPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128', 'MOD=MEM', 'ADDPD', ''];

# ADDPS-Add Packed Single-Precision Floating-Point Values.
T['ADDPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'ADDPS', ''];
T['ADDPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'ADDPS', ''];

# ADDSD-Add Scalar Double-Precision Floating-Point Values.
T['ADDSD REG:rw:f64=$XMMr REG:r:f64=$XMMm ', 'MOD=REG', 'ADDSD', ''];
T['ADDSD REG:rw:f64=$XMMr MEM:r:f64=$mem64', 'MOD=MEM', 'ADDSD', ''];

# ADDSS-Add Scalar Single-Precision Floating-Point Values.
T['ADDSS REG:rw:f32=$XMMr REG:r:f32=$XMMm ', 'MOD=REG', 'ADDSS', ''];
T['ADDSS REG:rw:f32=$XMMr MEM:r:f32=$mem32', 'MOD=MEM', 'ADDSS', ''];

# ADDSUBPD-Packed Double-FP Add/Subtract.
T['ADDSUBPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG', 'ADDSUBPD', ''];
T['ADDSUBPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128', 'MOD=MEM', 'ADDSUBPD', ''];

# ADDSUBPS-Packed Single-FP Add/Subtract.
T['ADDSUBPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'ADDSUBPS', ''];
T['ADDSUBPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'ADDSUBPS', ''];

# ANDNPD-Bitwise Logical AND NOT of Packed Double Precision Floating-Point Values.
T['ANDNPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG', 'ANDNPD', ''];
T['ANDNPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128', 'MOD=MEM', 'ANDNPD', ''];

# ANDNPS-Bitwise Logical AND NOT of Packed Single Precision Floating-Point Values.
T['ANDNPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'ANDNPS', ''];
T['ANDNPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'ANDNPS', ''];

# ANDPD-Bitwise Logical AND of Packed Double Precision Floating-Point Values.
T['ANDPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG', 'ANDPD', ''];
T['ANDPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128', 'MOD=MEM', 'ANDPD', ''];

# ANDPS-Bitwise Logical AND of Packed Single Precision Floating-Point Values.
T['ANDPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'ANDPS', ''];
T['ANDPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'ANDPS', ''];

# BLENDPD-Blend Packed Double Precision Floating-Point Values.
T['BLENDPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'BLENDPD', ''];
T['BLENDPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'BLENDPD', ''];

# BLENDPS-Blend Packed Single Precision Floating-Point Values.
T['BLENDPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'BLENDPS', ''];
T['BLENDPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'BLENDPS', ''];

# BLENDVPD-Variable Blend Packed Double Precision Floating-Point Values.
T['BLENDVPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm  VREG:SUPP:r:u64=XMM0', 'MOD=REG', 'BLENDVPD', ''];
T['BLENDVPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128 VREG:SUPP:r:u64=XMM0', 'MOD=MEM', 'BLENDVPD', ''];

# BLENDVPS-Variable Blend Packed Single Precision Floating-Point Values.
T['BLENDVPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm  VREG:SUPP:r:u32=XMM0', 'MOD=REG', 'BLENDVPS', ''];
T['BLENDVPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128 VREG:SUPP:r:u32=XMM0', 'MOD=MEM', 'BLENDVPS', ''];

# CMPPD-Compare Packed Double-Precision Floating-Point Values.
T['CMPPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm  CC:u8=$cc3', 'MOD=REG', 'CMPPD', ''];
T['CMPPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128 CC:u8=$cc3', 'MOD=MEM', 'CMPPD', ''];

# CMPPS-Compare Packed Single-Precision Floating-Point Values.
T['CMPPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm  CC:u8=$cc3', 'MOD=REG', 'CMPPS', ''];
T['CMPPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128 CC:u8=$cc3', 'MOD=MEM', 'CMPPS', ''];

# CMPS/CMPSB/CMPSW/CMPSD/CMPSQ-Compare String Operands.
T['CMPSD REG:rw:f64=$XMMr REG:r:f64=$XMMm  CC:u8=$cc3', 'MOD=REG', 'CMPSD_rqmqub', ''];
T['CMPSD REG:rw:f64=$XMMr MEM:r:f64=$mem64 CC:u8=$cc3', 'MOD=MEM', 'CMPSD_rqmqub', ''];

# CMPSS-Compare Scalar Single-Precision Floating-Point Value.
T['CMPSS REG:rw:f32=$XMMr REG:r:f32=$XMMm  CC:u8=$cc3', 'MOD=REG', 'CMPSS', ''];
T['CMPSS REG:rw:f32=$XMMr MEM:r:f32=$mem32 CC:u8=$cc3', 'MOD=MEM', 'CMPSS', ''];

# COMISD-Compare Scalar Ordered Double-Precision Floating-Point Values and Set EFLAGS.
T['COMISD REG:r:f64=$XMMr REG:r:f64=$XMMm ', 'MOD=REG', 'COMISD', ''];
T['COMISD REG:r:f64=$XMMr MEM:r:f64=$mem64', 'MOD=MEM', 'COMISD', ''];

# COMISS-Compare Scalar Ordered Single-Precision Floating-Point Values and Set EFLAGS.
T['COMISS REG:r:f32=$XMMr REG:r:f32=$XMMm ', 'MOD=REG', 'COMISS', ''];
T['COMISS REG:r:f32=$XMMr MEM:r:f32=$mem32', 'MOD=MEM', 'COMISS', ''];

# CRC32-Accumulate CRC32 Value.
T['CRC32 REG:rw:sx=$GPRyr REG:r:u8=$GPR8m', 'MOD=REG', 'CRC32_rymb', ''];
T['CRC32 REG:rw:sx=$GPRyr MEM:r:u8=$mem8 ', 'MOD=MEM', 'CRC32_rymb', ''];
T['CRC32 REG:rw:sx=$GPRyr REG:r:sx=$GPRvm', 'MOD=REG', 'CRC32_rymv', ''];
T['CRC32 REG:rw:sx=$GPRyr MEM:r:sx=$memv ', 'MOD=MEM', 'CRC32_rymv', ''];

# CVTDQ2PD-Convert Packed Doubleword Integers to Packed Double-Precision Floating-Point Values.
T['CVTDQ2PD VREG:w:f64=$XMMr VREG:r:s32=$XMMm', 'MOD=REG', 'CVTDQ2PD', ''];
T['CVTDQ2PD VREG:w:f64=$XMMr MEM:r:s32=$mem64', 'MOD=MEM', 'CVTDQ2PD', ''];

# CVTDQ2PS-Convert Packed Doubleword Integers to Packed Single-Precision Floating-Point Values.
T['CVTDQ2PS VREG:w:f32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'CVTDQ2PS', ''];
T['CVTDQ2PS VREG:w:f32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'CVTDQ2PS', ''];

# CVTPD2DQ-Convert Packed Double-Precision Floating-Point Values to Packed Doubleword Integers.
T['CVTPD2DQ VREG:w:s32=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG', 'CVTPD2DQ', ''];
T['CVTPD2DQ VREG:w:s32=$XMMr MEM:r:f64=$mem128', 'MOD=MEM', 'CVTPD2DQ', ''];

# CVTPD2PI-Convert Packed Double-Precision FP Values to Packed Dword Integers.
T['CVTPD2PI REG:w:s32=$MMXr VREG:r:f64=$XMMm ', 'MOD=REG', 'CVTPD2PI', ''];
T['CVTPD2PI REG:w:s32=$MMXr MEM:r:f64=$mem128', 'MOD=MEM', 'CVTPD2PI', ''];

# CVTPD2PS-Convert Packed Double-Precision Floating-Point Values to Packed Single-Precision Floating-Point Values.
T['CVTPD2PS VREG:w:f32=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG', 'CVTPD2PS', ''];
T['CVTPD2PS VREG:w:f32=$XMMr MEM:r:f64=$mem128', 'MOD=MEM', 'CVTPD2PS', ''];

# CVTPI2PD-Convert Packed Dword Integers to Packed Double-Precision FP Values.
T['CVTPI2PD VREG:w:f64=$XMMr VREG:r:s32=$MMXm', 'MOD=REG', 'CVTPI2PD', ''];
T['CVTPI2PD VREG:w:f64=$XMMr MEM:r:s32=$mem64', 'MOD=MEM', 'CVTPI2PD', ''];

# CVTPI2PS-Convert Packed Dword Integers to Packed Single-Precision FP Values.
T['CVTPI2PS VREG:w:f32=$XMMr VREG:r:s32=$MMXm', 'MOD=REG', 'CVTPI2PS', ''];
T['CVTPI2PS VREG:w:f32=$XMMr MEM:r:s32=$mem64', 'MOD=MEM', 'CVTPI2PS', ''];

# CVTPS2DQ-Convert Packed Single-Precision Floating-Point Values to Packed Signed Doubleword Integer Values .3-258.
T['CVTPS2DQ VREG:w:s32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'CVTPS2DQ', ''];
T['CVTPS2DQ VREG:w:s32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'CVTPS2DQ', ''];

# CVTPS2PD-Convert Packed Single-Precision Floating-Point Values to Packed Double-Precision Floating-Point Values.
T['CVTPS2PD VREG:w:f64=$XMMr VREG:r:f32=$XMMm', 'MOD=REG', 'CVTPS2PD', ''];
T['CVTPS2PD VREG:w:f64=$XMMr MEM:r:f32=$mem64', 'MOD=MEM', 'CVTPS2PD', ''];

# CVTPS2PI-Convert Packed Single-Precision FP Values to Packed Dword Integers.
T['CVTPS2PI REG:w:f32=$MMXr VREG:r:s32=$XMMm', 'MOD=REG', 'CVTPS2PI', ''];
T['CVTPS2PI REG:w:f32=$MMXr MEM:r:s32=$mem64', 'MOD=MEM', 'CVTPS2PI', ''];

# CVTSD2SI-Convert Scalar Double-Precision Floating-Point Value to Doubleword Integer.
T['CVTSD2SI REG:w:s32=$GPR32r REG:r:f64=$XMMm ', 'MOD=REG W=0', 'CVTSD2SI', ''];
T['CVTSD2SI REG:w:s64=$GPR64r REG:r:f64=$XMMm ', 'MOD=REG W=1', 'CVTSD2SI', ''];
T['CVTSD2SI REG:w:s32=$GPR32r MEM:r:f64=$mem64', 'MOD=MEM W=0', 'CVTSD2SI', ''];
T['CVTSD2SI REG:w:s64=$GPR64r MEM:r:f64=$mem64', 'MOD=MEM W=1', 'CVTSD2SI', ''];

# CVTSD2SS-Convert Scalar Double-Precision Floating-Point Value to Scalar Single-Precision Floating-Point Value. .3-267.
T['CVTSD2SS REG:w:f32=$XMMr REG:r:f64=$XMMm ', 'MOD=REG', 'CVTSD2SS', ''];
T['CVTSD2SS REG:w:f32=$XMMr MEM:r:f64=$mem64', 'MOD=MEM', 'CVTSD2SS', ''];

# CVTSI2SD-Convert Doubleword Integer to Scalar Double-Precision Floating-Point Value.
T['CVTSI2SD REG:w:f64=$XMMr REG:r:s32=$GPR32m', 'MOD=REG W=0', 'CVTSI2SD', ''];
T['CVTSI2SD REG:w:f64=$XMMr REG:r:s64=$GPR64m', 'MOD=REG W=1', 'CVTSI2SD', ''];
T['CVTSI2SD REG:w:f64=$XMMr MEM:r:s32=$mem32 ', 'MOD=MEM W=0', 'CVTSI2SD', ''];
T['CVTSI2SD REG:w:f64=$XMMr MEM:r:s64=$mem64 ', 'MOD=MEM W=1', 'CVTSI2SD', ''];

# CVTSI2SS-Convert Doubleword Integer to Scalar Single-Precision Floating-Point Value.
T['CVTSI2SS REG:w:f32=$XMMr REG:r:s32=$GPR32m', 'MOD=REG W=0', 'CVTSI2SS', ''];
T['CVTSI2SS REG:w:f32=$XMMr REG:r:s32=$GPR64m', 'MOD=REG W=1', 'CVTSI2SS', ''];
T['CVTSI2SS REG:w:f32=$XMMr MEM:r:s32=$mem32 ', 'MOD=MEM W=0', 'CVTSI2SS', ''];
T['CVTSI2SS REG:w:f32=$XMMr MEM:r:s32=$mem64 ', 'MOD=MEM W=1', 'CVTSI2SS', ''];

# CVTSS2SD-Convert Scalar Single-Precision Floating-Point Value to Scalar Double-Precision Floating-Point Value. .3-273.
T['CVTSS2SD REG:w:f64=$XMMr REG:r:f32=$XMMm ', 'MOD=REG', 'CVTSS2SD', ''];
T['CVTSS2SD REG:w:f64=$XMMr MEM:r:f32=$mem32', 'MOD=MEM', 'CVTSS2SD', ''];

# CVTSS2SI-Convert Scalar Single-Precision Floating-Point Value to Doubleword Integer.
T['CVTSS2SI REG:w:s32=$GPR32r REG:r:f32=$XMMm ', 'MOD=REG W=0', 'CVTSS2SI', ''];
T['CVTSS2SI REG:w:s64=$GPR64r REG:r:f32=$XMMm ', 'MOD=REG W=1', 'CVTSS2SI', ''];
T['CVTSS2SI REG:w:s32=$GPR32r MEM:r:f32=$mem32', 'MOD=MEM W=0', 'CVTSS2SI', ''];
T['CVTSS2SI REG:w:s64=$GPR64r MEM:r:f32=$mem32', 'MOD=MEM W=1', 'CVTSS2SI', ''];

# CVTTPD2DQ-Convert with Truncation Packed Double-Precision Floating-Point Values to Packed Doubleword Integers.
T['CVTTPD2DQ VREG:w:s32=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG', 'CVTTPD2DQ', ''];
T['CVTTPD2DQ VREG:w:s32=$XMMr MEM:r:f64=$mem128', 'MOD=MEM', 'CVTTPD2DQ', ''];

# CVTTPD2PI-Convert with Truncation Packed Double-Precision FP Values to Packed Dword Integers.
T['CVTTPD2PI REG:w:s32=$MMXr VREG:r:f64=$XMMm ', 'MOD=REG', 'CVTTPD2PI', ''];
T['CVTTPD2PI REG:w:s32=$MMXr MEM:r:f64=$mem128', 'MOD=MEM', 'CVTTPD2PI', ''];

# CVTTPS2DQ-Convert with Truncation Packed Single-Precision Floating-Point Values to Packed Signed Doubleword Integer Values.
T['CVTTPS2DQ VREG:w:s32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'CVTTPS2DQ', ''];
T['CVTTPS2DQ VREG:w:s32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'CVTTPS2DQ', ''];

# CVTTPS2PI-Convert with Truncation Packed Single-Precision FP Values to Packed Dword Integers.
T['CVTTPS2PI REG:w:s32=$MMXr VREG:r:f32=$XMMm', 'MOD=REG', 'CVTTPS2PI', ''];
T['CVTTPS2PI REG:w:s32=$MMXr MEM:r:f32=$mem64', 'MOD=MEM', 'CVTTPS2PI', ''];

# CVTTSD2SI-Convert with Truncation Scalar Double-Precision Floating-Point Value to Signed Integer.
T['CVTTSD2SI REG:w:s32=$GPR32r REG:r:f64=$XMMm ', 'MOD=REG W=0', 'CVTTSD2SI', ''];
T['CVTTSD2SI REG:w:s64=$GPR64r REG:r:f64=$XMMm ', 'MOD=REG W=1', 'CVTTSD2SI', ''];
T['CVTTSD2SI REG:w:s32=$GPR32r MEM:r:f64=$mem64', 'MOD=MEM W=0', 'CVTTSD2SI', ''];
T['CVTTSD2SI REG:w:s64=$GPR64r MEM:r:f64=$mem64', 'MOD=MEM W=1', 'CVTTSD2SI', ''];

# CVTTSS2SI-Convert with Truncation Scalar Single-Precision Floating-Point Value to Integer.
T['CVTTSS2SI REG:w:s32=$GPR32r REG:r:f32=$XMMm ', 'MOD=REG W=0', 'CVTTSS2SI', ''];
T['CVTTSS2SI REG:w:s64=$GPR64r REG:r:f32=$XMMm ', 'MOD=REG W=1', 'CVTTSS2SI', ''];
T['CVTTSS2SI REG:w:s32=$GPR32r MEM:r:f32=$mem32', 'MOD=MEM W=0', 'CVTTSS2SI', ''];
T['CVTTSS2SI REG:w:s64=$GPR64r MEM:r:f32=$mem32', 'MOD=MEM W=1', 'CVTTSS2SI', ''];

# DIVPD-Divide Packed Double-Precision Floating-Point Values.
T['DIVPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG', 'DIVPD', ''];
T['DIVPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128', 'MOD=MEM', 'DIVPD', ''];

# DIVPS-Divide Packed Single-Precision Floating-Point Values.
T['DIVPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'DIVPS', ''];
T['DIVPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'DIVPS', ''];

# DIVSD-Divide Scalar Double-Precision Floating-Point Value.
T['DIVSD REG:rw:f64=$XMMr REG:r:f64=$XMMm ', 'MOD=REG', 'DIVSD', ''];
T['DIVSD REG:rw:f64=$XMMr MEM:r:f64=$mem64', 'MOD=MEM', 'DIVSD', ''];

# DIVSS-Divide Scalar Single-Precision Floating-Point Values.
T['DIVSS REG:rw:f32=$XMMr REG:r:f32=$XMMm ', 'MOD=REG', 'DIVSS', ''];
T['DIVSS REG:rw:f32=$XMMr MEM:r:f32=$mem32', 'MOD=MEM', 'DIVSS', ''];

# DPPD-Dot Product of Packed Double Precision Floating-Point Values.
T['DPPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'DPPD', ''];
T['DPPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'DPPD', ''];

# DPPS-Dot Product of Packed Single Precision Floating-Point Values.
T['DPPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'DPPS', ''];
T['DPPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'DPPS', ''];

# EXTRACTPS-Extract Packed Floating-Point Values.
T['EXTRACTPS REG:w:s32=$GPR32m VREG:r:s32=$XMMr IMM:u8=$index', 'MOD=REG', 'EXTRACTPS', ''];
T['EXTRACTPS MEM:w:s32=$mem32  VREG:r:f32=$XMMr IMM:u8=$index', 'MOD=MEM', 'EXTRACTPS', ''];

# FISTTP-Store Integer with Truncation.
T['FISTTP MEM:w:s32=$mem32 REG:r:f80=ST0', 'NONE', 'FISTTP_md80', ''];
T['FISTTP MEM:w:s64=$mem64 REG:r:f80=ST0', 'NONE', 'FISTTP_mq80', ''];
T['FISTTP MEM:w:s16=$mem16 REG:r:f80=ST0', 'NONE', 'FISTTP_mw80', ''];

# FXRSTOR-Restore x87 FPU, MMX, XMM, and MXCSR State.
T['FXRSTOR   MEM:r=$mem4096', 'NONE', 'FXRSTOR  ', ''];
T['FXRSTOR64 MEM:r=$mem4096', 'NONE', 'FXRSTOR64', ''];

# FXSAVE-Save x87 FPU, MMX Technology, and SSE State.
T['FXSAVE   MEM:w=$mem4096', 'NONE', 'FXSAVE  ', ''];
T['FXSAVE64 MEM:w=$mem4096', 'NONE', 'FXSAVE64', ''];

# HADDPD-Packed Double-FP Horizontal Add.
T['HADDPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG', 'HADDPD', ''];
T['HADDPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128', 'MOD=MEM', 'HADDPD', ''];

# HADDPS-Packed Single-FP Horizontal Add.
T['HADDPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'HADDPS', ''];
T['HADDPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'HADDPS', ''];

# HSUBPD-Packed Double-FP Horizontal Subtract.
T['HSUBPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG', 'HSUBPD', ''];
T['HSUBPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128', 'MOD=MEM', 'HSUBPD', ''];

# HSUBPS-Packed Single-FP Horizontal Subtract.
T['HSUBPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'HSUBPS', ''];
T['HSUBPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'HSUBPS', ''];

# INSERTPS-Insert Scalar Single-Precision Floating-Point Value.
T['INSERTPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm IMM:u8=$index', 'MOD=REG', 'INSERTPS', ''];
T['INSERTPS VREG:rw:f32=$XMMr MEM:r:s32=$mem32 IMM:u8=$index', 'MOD=MEM', 'INSERTPS', ''];

# LDDQU-Load Unaligned Integer 128 Bits.
T['LDDQU VREG:w:f64=$XMMr MEM:r:s32=$mem128', 'NONE', 'LDDQU', ''];

# LDMXCSR-Load MXCSR Register.
T['LDMXCSR MEM:r:s32=$mem32 REG:SUPP:w=MXCSR', 'NONE', 'LDMXCSR', ''];

# LFENCE-Load Fence.
T['LFENCE', 'NONE', 'LFENCE', ''];

# MASKMOVDQU-Store Selected Bytes of Double Quadword.
T['MASKMOVDQU VREG:r:s32=$XMMr VREG:r:s32=$XMMm MEM:SUPP:w:s32={SEG:r=DS BASE:r=$DIa SZ=128}', 'NONE', 'MASKMOVDQU', ''];

# MAXPD-Maximum of Packed Double-Precision Floating-Point Values.
T['MAXPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG', 'MAXPD', ''];
T['MAXPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128', 'MOD=MEM', 'MAXPD', ''];

# MAXPS-Maximum of Packed Single-Precision Floating-Point Values.
T['MAXPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'MAXPS', ''];
T['MAXPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'MAXPS', ''];

# MAXSD-Return Maximum Scalar Double-Precision Floating-Point Value.
T['MAXSD REG:rw:f64=$XMMr REG:r:f64=$XMMm ', 'MOD=REG', 'MAXSD', ''];
T['MAXSD REG:rw:f64=$XMMr MEM:r:f64=$mem64', 'MOD=MEM', 'MAXSD', ''];

# MAXSS-Return Maximum Scalar Single-Precision Floating-Point Value.
T['MAXSS REG:rw:f32=$XMMr REG:r:f32=$XMMm ', 'MOD=REG', 'MAXSS', ''];
T['MAXSS REG:rw:f32=$XMMr MEM:r:f32=$mem32', 'MOD=MEM', 'MAXSS', ''];

# MFENCE-Memory Fence.
T['MFENCE', 'NONE', 'MFENCE', ''];

# MINPD-Minimum of Packed Double-Precision Floating-Point Values.
T['MINPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG', 'MINPD', ''];
T['MINPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128', 'MOD=MEM', 'MINPD', ''];

# MINPS-Minimum of Packed Single-Precision Floating-Point Values.
T['MINPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'MINPS', ''];
T['MINPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'MINPS', ''];

# MINSD-Return Minimum Scalar Double-Precision Floating-Point Value.
T['MINSD REG:rw:f64=$XMMr REG:r:f64=$XMMm ', 'MOD=REG', 'MINSD', ''];
T['MINSD REG:rw:f64=$XMMr MEM:r:f64=$mem64', 'MOD=MEM', 'MINSD', ''];

# MINSS-Return Minimum Scalar Single-Precision Floating-Point Value.
T['MINSS REG:rw:f32=$XMMr REG:r:f32=$XMMm ', 'MOD=REG', 'MINSS', ''];
T['MINSS REG:rw:f32=$XMMr MEM:r:f32=$mem32', 'MOD=MEM', 'MINSS', ''];

# MOVAPD-Move Aligned Packed Double-Precision Floating-Point Values.
T['MOVAPD VREG:w:f64=$XMMm  VREG:r:f64=$XMMr ', 'MOD=REG', 'MOVAPD_moro', ''];
T['MOVAPD MEM:w:f64=$mem128 VREG:r:f64=$XMMr ', 'MOD=MEM', 'MOVAPD_moro', ''];
T['MOVAPD VREG:w:f64=$XMMr  VREG:r:f64=$XMMm ', 'MOD=REG', 'MOVAPD_romo', ''];
T['MOVAPD VREG:w:f64=$XMMr  MEM:r:f64=$mem128', 'MOD=MEM', 'MOVAPD_romo', ''];

# MOVAPS-Move Aligned Packed Single-Precision Floating-Point Values.
T['MOVAPS VREG:w:f32=$XMMm  VREG:r:f32=$XMMr ', 'MOD=REG', 'MOVAPS_moro', ''];
T['MOVAPS MEM:w:f32=$mem128 VREG:r:f32=$XMMr ', 'MOD=MEM', 'MOVAPS_moro', ''];
T['MOVAPS VREG:w:f32=$XMMr  VREG:r:f32=$XMMm ', 'MOD=REG', 'MOVAPS_romo', ''];
T['MOVAPS VREG:w:f32=$XMMr  MEM:r:f32=$mem128', 'MOD=MEM', 'MOVAPS_romo', ''];

# MOVDDUP-Replicate Double FP Values.
T['MOVDDUP VREG:w:s32=$XMMr REG:r:s64=$XMMm ', 'MOD=REG', 'MOVDDUP', ''];
T['MOVDDUP VREG:w:s32=$XMMr MEM:r:s64=$mem64', 'MOD=MEM', 'MOVDDUP', ''];

# MOVDQ2Q-Move Quadword from XMM to MMX Technology Register.
T['MOVDQ2Q REG:w:u64=$MMXr REG:r:u64=$XMMm', 'NONE', 'MOVDQ2Q', ''];

# MOVDQA,VMOVDQA32/64-Move Aligned Packed Integer Values.
T['MOVDQA VREG:w:s32=$XMMm  VREG:r:s32=$XMMr ', 'MOD=REG', 'MOVDQA_moro', ''];
T['MOVDQA MEM:w:s32=$mem128 VREG:r:s32=$XMMr ', 'MOD=MEM', 'MOVDQA_moro', ''];
T['MOVDQA VREG:w:s32=$XMMr  VREG:r:s32=$XMMm ', 'MOD=REG', 'MOVDQA_romo', ''];
T['MOVDQA VREG:w:s32=$XMMr  MEM:r:s32=$mem128', 'MOD=MEM', 'MOVDQA_romo', ''];

# MOVDQU,VMOVDQU8/16/32/64-Move Unaligned Packed Integer Values.
T['MOVDQU VREG:w:s32=$XMMm  VREG:r:s32=$XMMr ', 'MOD=REG', 'MOVDQU_moro', ''];
T['MOVDQU MEM:w:s32=$mem128 VREG:r:s32=$XMMr ', 'MOD=MEM', 'MOVDQU_moro', ''];
T['MOVDQU VREG:w:s32=$XMMr  VREG:r:s32=$XMMm ', 'MOD=REG', 'MOVDQU_romo', ''];
T['MOVDQU VREG:w:s32=$XMMr  MEM:r:s32=$mem128', 'MOD=MEM', 'MOVDQU_romo', ''];

# MOVHLPS-Move Packed Single-Precision Floating-Point Values High to Low.
T['MOVHLPS VREG:w:f32=$XMMr VREG:r:f32=$XMMm', 'NONE', 'MOVHLPS', ''];

# MOVHPD-Move High Packed Double-Precision Floating-Point Value.
T['MOVHPD MEM:w:s64=$mem64 REG:r:f64=$XMMr ', 'NONE', 'MOVHPD_mqrq', ''];
T['MOVHPD REG:w:f64=$XMMr  MEM:r:s64=$mem64', 'NONE', 'MOVHPD_rqmq', ''];

# MOVHPS-Move High Packed Single-Precision Floating-Point Values.
T['MOVHPS MEM:w:f32=$mem64 VREG:r:f32=$XMMr', 'NONE', 'MOVHPS_mqro', ''];
T['MOVHPS VREG:w:f32=$XMMr MEM:r:f32=$mem64', 'NONE', 'MOVHPS_rqmq', ''];

# MOVLHPS-Move Packed Single-Precision Floating-Point Values Low to High.
T['MOVLHPS VREG:w:f32=$XMMr VREG:r:f32=$XMMm', 'NONE', 'MOVLHPS', ''];

# MOVLPD-Move Low Packed Double-Precision Floating-Point Value.
T['MOVLPD MEM:w:s64=$mem64 REG:r:f64=$XMMr ', 'NONE', 'MOVLPD_mqrq', ''];
T['MOVLPD REG:w:f64=$XMMr  MEM:r:s64=$mem64', 'NONE', 'MOVLPD_rqmq', ''];

# MOVLPS-Move Low Packed Single-Precision Floating-Point Values.
T['MOVLPS MEM:w:f32=$mem64 VREG:r:f32=$XMMr', 'NONE', 'MOVLPS_mqro', ''];
T['MOVLPS VREG:w:f32=$XMMr MEM:r:f32=$mem64', 'NONE', 'MOVLPS_rqmq', ''];

# MOVMSKPD-Extract Packed Double-Precision Floating-Point Sign Mask.
T['MOVMSKPD REG:w=$GPR32r VREG:r:f64=$XMMm', 'NONE', 'MOVMSKPD', ''];

# MOVMSKPS-Extract Packed Single-Precision Floating-Point Sign Mask.
T['MOVMSKPS REG:w=$GPR32r VREG:r:f32=$XMMm', 'NONE', 'MOVMSKPS', ''];

# MOVNTDQ-Store Packed Integers Using Non-Temporal Hint.
T['MOVNTDQ MEM:w:s32=$mem128 VREG:r:s32=$XMMr', 'NONE', 'MOVNTDQ', ''];

# MOVNTDQA-Load Double Quadword Non-Temporal Aligned Hint.
T['MOVNTDQA VREG:w:s32=$XMMr MEM:r:s32=$mem128', 'NONE', 'MOVNTDQA', ''];

# MOVNTI-Store Doubleword Using Non-Temporal Hint.
T['MOVNTI MEM:w:s64=$mem64 REG:r=$GPR64r', 'OSZ=64', 'MOVNTI', ''];
T['MOVNTI MEM:w:s32=$mem32 REG:r=$GPR32r', 'OSZ=Z ', 'MOVNTI', ''];

# MOVNTPD-Store Packed Double-Precision Floating-Point Values Using Non-Temporal Hint.
T['MOVNTPD MEM:w:s32=$mem128 VREG:r:f64=$XMMr', 'NONE', 'MOVNTPD', ''];

# MOVNTPS-Store Packed Single-Precision Floating-Point Values Using Non-Temporal Hint.
T['MOVNTPS MEM:w:s32=$mem128 VREG:r:f32=$XMMr', 'NONE', 'MOVNTPS', ''];

# MOVQ-Move Quadword.
T['MOVQ REG:w=$GPR64m    REG:r:s64=$XMMr ', 'MOD=REG', 'MOVQ_mqrq_2', ''];
T['MOVQ MEM:w:s64=$mem64 REG:r:s64=$XMMr ', 'MOD=MEM', 'MOVQ_mqrq_2', ''];
T['MOVQ VREG:w:s32=$XMMm REG:r:s64=$XMMr ', 'MOD=REG', 'MOVQ_mxrq  ', ''];
T['MOVQ MEM:w:s64=$mem64 REG:r:s64=$XMMr ', 'MOD=MEM', 'MOVQ_mxrq  ', ''];
T['MOVQ VREG:w:s32=$XMMr REG:r=$GPR64m   ', 'MOD=REG', 'MOVQ_romq_1', ''];
T['MOVQ VREG:w:s32=$XMMr MEM:r:s64=$mem64', 'MOD=MEM', 'MOVQ_romq_1', ''];
T['MOVQ VREG:w:s32=$XMMr REG:r:s64=$XMMm ', 'MOD=REG', 'MOVQ_romq_2', ''];
T['MOVQ VREG:w:s32=$XMMr MEM:r:s64=$mem64', 'MOD=MEM', 'MOVQ_romq_2', ''];

# MOVQ2DQ-Move Quadword from MMX Technology to XMM Register.
T['MOVQ2DQ VREG:w:u64=$XMMr REG:r:u64=$MMXm', 'NONE', 'MOVQ2DQ', ''];

# MOVS/MOVSB/MOVSW/MOVSD/MOVSQ-Move Data from String to String.
T['MOVSD REG:w:f64=$XMMm  REG:r:f64=$XMMr ', 'MOD=REG', 'MOVSD_mqrq', ''];
T['MOVSD MEM:w:f64=$mem64 REG:r:f64=$XMMr ', 'MOD=MEM', 'MOVSD_mqrq', ''];
T['MOVSD REG:w:f64=$XMMr  REG:r:f64=$XMMm ', 'MOD=REG', 'MOVSD_rxmq', ''];
T['MOVSD VREG:w:f64=$XMMr MEM:r:f64=$mem64', 'MOD=MEM', 'MOVSD_rxmq', ''];

# MOVSHDUP-Replicate Single FP Values.
T['MOVSHDUP VREG:w:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'MOVSHDUP', ''];
T['MOVSHDUP VREG:w:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'MOVSHDUP', ''];

# MOVSLDUP-Replicate Single FP Values.
T['MOVSLDUP VREG:w:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'MOVSLDUP', ''];
T['MOVSLDUP VREG:w:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'MOVSLDUP', ''];

# MOVSS-Move or Merge Scalar Single-Precision Floating-Point Value.
T['MOVSS REG:w:f32=$XMMm  REG:r:f32=$XMMr ', 'MOD=REG', 'MOVSS_mdrd', ''];
T['MOVSS MEM:w:f32=$mem32 REG:r:f32=$XMMr ', 'MOD=MEM', 'MOVSS_mdrd', ''];
T['MOVSS REG:w:f32=$XMMr  REG:r:f32=$XMMm ', 'MOD=REG', 'MOVSS_rxmd', ''];
T['MOVSS VREG:w:f32=$XMMr MEM:r:f32=$mem32', 'MOD=MEM', 'MOVSS_rxmd', ''];

# MOVUPD-Move Unaligned Packed Double-Precision Floating-Point Values.
T['MOVUPD VREG:w:f64=$XMMm  VREG:r:f64=$XMMr ', 'MOD=REG', 'MOVUPD_moro', ''];
T['MOVUPD MEM:w:f64=$mem128 VREG:r:f64=$XMMr ', 'MOD=MEM', 'MOVUPD_moro', ''];
T['MOVUPD VREG:w:f64=$XMMr  VREG:r:f64=$XMMm ', 'MOD=REG', 'MOVUPD_romo', ''];
T['MOVUPD VREG:w:f64=$XMMr  MEM:r:f64=$mem128', 'MOD=MEM', 'MOVUPD_romo', ''];

# MOVUPS-Move Unaligned Packed Single-Precision Floating-Point Values.
T['MOVUPS VREG:w:f32=$XMMm  VREG:r:f32=$XMMr ', 'MOD=REG', 'MOVUPS_moro', ''];
T['MOVUPS MEM:w:f32=$mem128 VREG:r:f32=$XMMr ', 'MOD=MEM', 'MOVUPS_moro', ''];
T['MOVUPS VREG:w:f32=$XMMr  VREG:r:f32=$XMMm ', 'MOD=REG', 'MOVUPS_romo', ''];
T['MOVUPS VREG:w:f32=$XMMr  MEM:r:f32=$mem128', 'MOD=MEM', 'MOVUPS_romo', ''];

# MOVD/MOVQ-Move Doubleword/Move Quadword.
T['MOVD REG:w=$GPR32m    REG:r:s32=$XMMr ', 'MOD=REG MODE=64 W=0', 'MOVD_mdrd_2', ''];
T['MOVD MEM:w:s32=$mem32 REG:r:s32=$XMMr ', 'MOD=MEM MODE=64 W=0', 'MOVD_mdrd_2', ''];
T['MOVD REG:w=$GPR32m    REG:r:s32=$XMMr ', 'MOD=REG MODE=NO64  ', 'MOVD_mdrd_2', ''];
T['MOVD MEM:w:s32=$mem32 REG:r:s32=$XMMr ', 'MOD=MEM MODE=NO64  ', 'MOVD_mdrd_2', ''];
T['MOVD VREG:w:s32=$XMMr REG:r=$GPR32m   ', 'MOD=REG MODE=64 W=0', 'MOVD_romd  ', ''];
T['MOVD VREG:w:s32=$XMMr MEM:r:s32=$mem32', 'MOD=MEM MODE=64 W=0', 'MOVD_romd  ', ''];
T['MOVD VREG:w:s32=$XMMr REG:r=$GPR32m   ', 'MOD=REG MODE=NO64  ', 'MOVD_romd  ', ''];
T['MOVD VREG:w:s32=$XMMr MEM:r:s32=$mem32', 'MOD=MEM MODE=NO64  ', 'MOVD_romd  ', ''];

# MPSADBW-Compute Multiple Packed Sums of Absolute Difference.
T['MPSADBW VREG:rw:u8=$XMMr VREG:r:u8=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'MPSADBW', ''];
T['MPSADBW VREG:rw:u8=$XMMr MEM:r:u8=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'MPSADBW', ''];

# MULPD-Multiply Packed Double-Precision Floating-Point Values.
T['MULPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG', 'MULPD', ''];
T['MULPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128', 'MOD=MEM', 'MULPD', ''];

# MULPS-Multiply Packed Single-Precision Floating-Point Values.
T['MULPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'MULPS', ''];
T['MULPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'MULPS', ''];

# MULSD-Multiply Scalar Double-Precision Floating-Point Value.
T['MULSD REG:rw:f64=$XMMr REG:r:f64=$XMMm ', 'MOD=REG', 'MULSD', ''];
T['MULSD REG:rw:f64=$XMMr MEM:r:f64=$mem64', 'MOD=MEM', 'MULSD', ''];

# MULSS-Multiply Scalar Single-Precision Floating-Point Values.
T['MULSS REG:rw:f32=$XMMr REG:r:f32=$XMMm ', 'MOD=REG', 'MULSS', ''];
T['MULSS REG:rw:f32=$XMMr MEM:r:f32=$mem32', 'MOD=MEM', 'MULSS', ''];

# ORPD-Bitwise Logical OR of Packed Double Precision Floating-Point Values.
T['ORPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG', 'ORPD', ''];
T['ORPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128', 'MOD=MEM', 'ORPD', ''];

# ORPS-Bitwise Logical OR of Packed Single Precision Floating-Point Values.
T['ORPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'ORPS', ''];
T['ORPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'ORPS', ''];

# PABSB/PABSW/PABSD/PABSQ-Packed Absolute Value.
T['PABSB VREG:w:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PABSB_romo  ', ''];
T['PABSB VREG:w:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PABSB_romo  ', ''];
T['PABSB REG:w:s64=$MMXr  REG:r:s64=$MMXm  ', 'MOD=REG', 'PABSB_rqmq  ', ''];
T['PABSB REG:w:s64=$MMXr  MEM:r:s64=$mem64 ', 'MOD=MEM', 'PABSB_rqmq  ', ''];
T['PABSD VREG:w:s32=$XMMr VREG:r:s32=$XMMm ', 'NONE   ', 'PABSD_romo_1', ''];
T['PABSD VREG:w:s32=$XMMr MEM:r:s32=$mem128', 'NONE   ', 'PABSD_romo_2', ''];
T['PABSD REG:w:s64=$MMXr  REG:r:s64=$MMXm  ', 'NONE   ', 'PABSD_rqmq_1', ''];
T['PABSD REG:w:s64=$MMXr  MEM:r:s64=$mem64 ', 'NONE   ', 'PABSD_rqmq_2', ''];
T['PABSW VREG:w:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PABSW_romo  ', ''];
T['PABSW VREG:w:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PABSW_romo  ', ''];
T['PABSW REG:w:s64=$MMXr  REG:r:s64=$MMXm  ', 'MOD=REG', 'PABSW_rqmq  ', ''];
T['PABSW REG:w:s64=$MMXr  MEM:r:s64=$mem64 ', 'MOD=MEM', 'PABSW_rqmq  ', ''];

# PACKSSWB/PACKSSDW-Pack with Signed Saturation.
T['PACKSSDW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PACKSSDW_romo', ''];
T['PACKSSDW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PACKSSDW_romo', ''];
T['PACKSSWB VREG:rw:s16=$XMMr VREG:r:s16=$XMMm ', 'MOD=REG', 'PACKSSWB_romo', ''];
T['PACKSSWB VREG:rw:s16=$XMMr MEM:r:s16=$mem128', 'MOD=MEM', 'PACKSSWB_romo', ''];

# PACKUSDW-Pack with Unsigned Saturation.
T['PACKUSDW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PACKUSDW', ''];
T['PACKUSDW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PACKUSDW', ''];

# PACKUSWB-Pack with Unsigned Saturation.
T['PACKUSWB VREG:rw:s16=$XMMr VREG:r:s16=$XMMm ', 'MOD=REG', 'PACKUSWB_romo', ''];
T['PACKUSWB VREG:rw:s16=$XMMr MEM:r:s16=$mem128', 'MOD=MEM', 'PACKUSWB_romo', ''];

# PADDSB/PADDSW-Add Packed Signed Integers with Signed Saturation.
T['PADDSB VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PADDSB_romo', ''];
T['PADDSB VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PADDSB_romo', ''];
T['PADDSW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PADDSW_romo', ''];
T['PADDSW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PADDSW_romo', ''];

# PADDUSB/PADDUSW-Add Packed Unsigned Integers with Unsigned Saturation.
T['PADDUSB VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PADDUSB_romo', ''];
T['PADDUSB VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PADDUSB_romo', ''];
T['PADDUSW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PADDUSW_romo', ''];
T['PADDUSW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PADDUSW_romo', ''];

# PADDB/PADDW/PADDD/PADDQ-Add Packed Integers.
T['PADDB VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PADDB_romo', ''];
T['PADDB VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PADDB_romo', ''];
T['PADDD VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PADDD_romo', ''];
T['PADDD VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PADDD_romo', ''];
T['PADDQ VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PADDQ_romo', ''];
T['PADDQ VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PADDQ_romo', ''];
T['PADDQ REG:rw:u64=$MMXr  REG:r:u64=$MMXm  ', 'MOD=REG', 'PADDQ_rqmq', ''];
T['PADDQ REG:rw:u64=$MMXr  MEM:r:u64=$mem64 ', 'MOD=MEM', 'PADDQ_rqmq', ''];
T['PADDW VREG:rw:s16=$XMMr VREG:r:s16=$XMMm ', 'MOD=REG', 'PADDW_romo', ''];
T['PADDW VREG:rw:s16=$XMMr MEM:r:s16=$mem128', 'MOD=MEM', 'PADDW_romo', ''];

# PALIGNR-Packed Align Right.
T['PALIGNR VREG:rw:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'PALIGNR_romoub', ''];
T['PALIGNR VREG:rw:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'PALIGNR_romoub', ''];
T['PALIGNR REG:rw:s64=$MMXr  REG:r:s64=$MMXm   IMM:u8=$uimm8', 'MOD=REG', 'PALIGNR_rqmqub', ''];
T['PALIGNR REG:rw:s64=$MMXr  MEM:r:s64=$mem64  IMM:u8=$uimm8', 'MOD=MEM', 'PALIGNR_rqmqub', ''];

# PAND-Logical AND.
T['PAND VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PAND_romo', ''];
T['PAND VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PAND_romo', ''];

# PANDN-Logical AND NOT.
T['PANDN VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PANDN_romo', ''];
T['PANDN VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PANDN_romo', ''];

# PAVGB/PAVGW-Average Packed Integers.
T['PAVGB VREG:rw:u8=$XMMr  VREG:r:u8=$XMMm  ', 'MOD=REG', 'PAVGB_romo', ''];
T['PAVGB VREG:rw:u8=$XMMr  MEM:r:u8=$mem128 ', 'MOD=MEM', 'PAVGB_romo', ''];
T['PAVGW VREG:rw:u16=$XMMr VREG:r:u16=$XMMm ', 'MOD=REG', 'PAVGW_romo', ''];
T['PAVGW VREG:rw:u16=$XMMr MEM:r:u16=$mem128', 'MOD=MEM', 'PAVGW_romo', ''];

# PBLENDVB-Variable Blend Packed Bytes.
T['PBLENDVB VREG:rw:s32=$XMMr VREG:r:s32=$XMMm  VREG:SUPP:r:s32=XMM0', 'MOD=REG', 'PBLENDVB', ''];
T['PBLENDVB VREG:rw:s32=$XMMr MEM:r:s32=$mem128 VREG:SUPP:r:s32=XMM0', 'MOD=MEM', 'PBLENDVB', ''];

# PBLENDW-Blend Packed Words.
T['PBLENDW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'PBLENDW', ''];
T['PBLENDW VREG:rw:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'PBLENDW', ''];

# PCMPEQQ-Compare Packed Qword Data for Equal.
T['PCMPEQQ VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PCMPEQQ', ''];
T['PCMPEQQ VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PCMPEQQ', ''];

# PCMPEQB/PCMPEQW/PCMPEQD-Compare Packed Data for Equal.
T['PCMPEQB VREG:rw:s8=$XMMr  VREG:r:s8=$XMMm  ', 'MOD=REG', 'PCMPEQB_romo', ''];
T['PCMPEQB VREG:rw:s8=$XMMr  MEM:r:s8=$mem128 ', 'MOD=MEM', 'PCMPEQB_romo', ''];
T['PCMPEQD VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PCMPEQD_romo', ''];
T['PCMPEQD VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PCMPEQD_romo', ''];
T['PCMPEQW VREG:rw:s16=$XMMr VREG:r:s16=$XMMm ', 'MOD=REG', 'PCMPEQW_romo', ''];
T['PCMPEQW VREG:rw:s16=$XMMr MEM:r:s16=$mem128', 'MOD=MEM', 'PCMPEQW_romo', ''];

# PCMPESTRI-Packed Compare Explicit Length Strings, Return Index.
T['PCMPESTRI VREG:r:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8 REG:SUPP:r=EAX REG:SUPP:r=EDX REG:SUPP:w=ECX', 'MOD=REG W=0', 'PCMPESTRI', ''];
T['PCMPESTRI VREG:r:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8 REG:SUPP:r=RAX REG:SUPP:r=RDX REG:SUPP:w=RCX', 'MOD=REG W=1', 'PCMPESTRI', ''];
T['PCMPESTRI VREG:r:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8 REG:SUPP:r=EAX REG:SUPP:r=EDX REG:SUPP:w=ECX', 'MOD=MEM W=0', 'PCMPESTRI', ''];
T['PCMPESTRI VREG:r:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8 REG:SUPP:r=RAX REG:SUPP:r=RDX REG:SUPP:w=RCX', 'MOD=MEM W=1', 'PCMPESTRI', ''];

# PCMPESTRM-Packed Compare Explicit Length Strings, Return Mask.
T['PCMPESTRM VREG:r:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8 REG:SUPP:r=EAX REG:SUPP:r=EDX VREG:SUPP:w:s32=XMM0', 'MOD=REG W=0', 'PCMPESTRM', ''];
T['PCMPESTRM VREG:r:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8 REG:SUPP:r=RAX REG:SUPP:r=RDX VREG:SUPP:w:s32=XMM0', 'MOD=REG W=1', 'PCMPESTRM', ''];
T['PCMPESTRM VREG:r:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8 REG:SUPP:r=EAX REG:SUPP:r=EDX VREG:SUPP:w:s32=XMM0', 'MOD=MEM W=0', 'PCMPESTRM', ''];
T['PCMPESTRM VREG:r:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8 REG:SUPP:r=RAX REG:SUPP:r=RDX VREG:SUPP:w:s32=XMM0', 'MOD=MEM W=1', 'PCMPESTRM', ''];

# PCMPGTQ-Compare Packed Data for Greater Than.
T['PCMPGTQ VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PCMPGTQ', ''];
T['PCMPGTQ VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PCMPGTQ', ''];

# PCMPGTB/PCMPGTW/PCMPGTD-Compare Packed Signed Integers for Greater Than.
T['PCMPGTB VREG:rw:s8=$XMMr  VREG:r:s8=$XMMm  ', 'MOD=REG', 'PCMPGTB_romo', ''];
T['PCMPGTB VREG:rw:s8=$XMMr  MEM:r:s8=$mem128 ', 'MOD=MEM', 'PCMPGTB_romo', ''];
T['PCMPGTD VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PCMPGTD_romo', ''];
T['PCMPGTD VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PCMPGTD_romo', ''];
T['PCMPGTW VREG:rw:s16=$XMMr VREG:r:s16=$XMMm ', 'MOD=REG', 'PCMPGTW_romo', ''];
T['PCMPGTW VREG:rw:s16=$XMMr MEM:r:s16=$mem128', 'MOD=MEM', 'PCMPGTW_romo', ''];

# PCMPISTRI-Packed Compare Implicit Length Strings, Return Index.
T['PCMPISTRI VREG:r:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8 REG:SUPP:w=ECX', 'MOD=REG W=0', 'PCMPISTRI', ''];
T['PCMPISTRI VREG:r:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8 REG:SUPP:w=RCX', 'MOD=REG W=1', 'PCMPISTRI', ''];
T['PCMPISTRI VREG:r:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8 REG:SUPP:w=ECX', 'MOD=MEM W=0', 'PCMPISTRI', ''];
T['PCMPISTRI VREG:r:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8 REG:SUPP:w=RCX', 'MOD=MEM W=1', 'PCMPISTRI', ''];

# PCMPISTRM-Packed Compare Implicit Length Strings, Return Mask.
T['PCMPISTRM VREG:r:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8 VREG:SUPP:w:s32=XMM0', 'MOD=REG', 'PCMPISTRM', ''];
T['PCMPISTRM VREG:r:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8 VREG:SUPP:w:s32=XMM0', 'MOD=MEM', 'PCMPISTRM', ''];

# PEXTRW-Extract Word.
T['PEXTRW REG:w=$GPR32m    VREG:r:s32=$XMMr IMM:u8=$index', 'MOD=REG', 'PEXTRW_mxroub', ''];
T['PEXTRW MEM:w:s16=$mem16 VREG:r:s32=$XMMr IMM:u8=$index', 'MOD=MEM', 'PEXTRW_mxroub', ''];
T['PEXTRW REG:w=$GPR32r    VREG:r:s32=$XMMm IMM:u8=$index', 'NONE   ', 'PEXTRW_rdmoub', ''];

# PEXTRB/PEXTRD/PEXTRQ-Extract Byte/Dword/Qword.
T['PEXTRB REG:w:s32=$GPR32m VREG:r:s32=$XMMr IMM:u8=$index', 'MOD=REG    ', 'PEXTRB', ''];
T['PEXTRB MEM:w:u8=$mem8    VREG:r:s32=$XMMr IMM:u8=$index', 'MOD=MEM    ', 'PEXTRB', ''];
T['PEXTRD REG:w:s32=$GPR32m VREG:r:s32=$XMMr IMM:u8=$index', 'MOD=REG W=0', 'PEXTRx', ''];
T['PEXTRD MEM:w:s32=$mem32  VREG:r:s32=$XMMr IMM:u8=$index', 'MOD=MEM W=0', 'PEXTRx', ''];
T['PEXTRQ REG:w:s64=$GPR64m VREG:r:s32=$XMMr IMM:u8=$index', 'MOD=REG W=1', 'PEXTRx', ''];
T['PEXTRQ MEM:w:s64=$mem64  VREG:r:s32=$XMMr IMM:u8=$index', 'MOD=MEM W=1', 'PEXTRx', ''];

# PHADDSW-Packed Horizontal Add and Saturate.
T['PHADDSW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PHADDSW_romo', ''];
T['PHADDSW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PHADDSW_romo', ''];
T['PHADDSW REG:rw:s64=$MMXr  REG:r:s64=$MMXm  ', 'MOD=REG', 'PHADDSW_rqmq', ''];
T['PHADDSW REG:rw:s64=$MMXr  MEM:r:s64=$mem64 ', 'MOD=MEM', 'PHADDSW_rqmq', ''];

# PHADDW/PHADDD-Packed Horizontal Add.
T['PHADDD VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PHADDD_romo', ''];
T['PHADDD VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PHADDD_romo', ''];
T['PHADDD REG:rw:s64=$MMXr  REG:r:s64=$MMXm  ', 'MOD=REG', 'PHADDD_rqmq', ''];
T['PHADDD REG:rw:s64=$MMXr  MEM:r:s64=$mem64 ', 'MOD=MEM', 'PHADDD_rqmq', ''];
T['PHADDW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PHADDW_romo', ''];
T['PHADDW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PHADDW_romo', ''];
T['PHADDW REG:rw:s64=$MMXr  REG:r:s64=$MMXm  ', 'MOD=REG', 'PHADDW_rqmq', ''];
T['PHADDW REG:rw:s64=$MMXr  MEM:r:s64=$mem64 ', 'MOD=MEM', 'PHADDW_rqmq', ''];

# PHMINPOSUW-Packed Horizontal Word Minimum.
T['PHMINPOSUW VREG:w:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PHMINPOSUW', ''];
T['PHMINPOSUW VREG:w:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PHMINPOSUW', ''];

# PHSUBSW-Packed Horizontal Subtract and Saturate.
T['PHSUBSW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PHSUBSW_romo', ''];
T['PHSUBSW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PHSUBSW_romo', ''];
T['PHSUBSW REG:rw:s64=$MMXr  REG:r:s64=$MMXm  ', 'MOD=REG', 'PHSUBSW_rqmq', ''];
T['PHSUBSW REG:rw:s64=$MMXr  MEM:r:s64=$mem64 ', 'MOD=MEM', 'PHSUBSW_rqmq', ''];

# PHSUBW/PHSUBD-Packed Horizontal Subtract.
T['PHSUBD VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PHSUBD_romo', ''];
T['PHSUBD VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PHSUBD_romo', ''];
T['PHSUBD REG:rw:s64=$MMXr  REG:r:s64=$MMXm  ', 'MOD=REG', 'PHSUBD_rqmq', ''];
T['PHSUBD REG:rw:s64=$MMXr  MEM:r:s64=$mem64 ', 'MOD=MEM', 'PHSUBD_rqmq', ''];
T['PHSUBW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PHSUBW_romo', ''];
T['PHSUBW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PHSUBW_romo', ''];
T['PHSUBW REG:rw:s64=$MMXr  REG:r:s64=$MMXm  ', 'MOD=REG', 'PHSUBW_rqmq', ''];
T['PHSUBW REG:rw:s64=$MMXr  MEM:r:s64=$mem64 ', 'MOD=MEM', 'PHSUBW_rqmq', ''];

# PINSRW-Insert Word.
T['PINSRW VREG:rw:s32=$XMMr REG:r=$GPR32m    IMM:u8=$index', 'MOD=REG', 'PINSRW_romxub', ''];
T['PINSRW VREG:rw:s32=$XMMr MEM:r:s16=$mem16 IMM:u8=$index', 'MOD=MEM', 'PINSRW_romxub', ''];

# PINSRB/PINSRD/PINSRQ-Insert Byte/Dword/Qword.
T['PINSRB VREG:rw:s32=$XMMr REG:r:s32=$GPR32m IMM:u8=$index', 'MOD=REG    ', 'PINSRB', ''];
T['PINSRB VREG:rw:s32=$XMMr MEM:r:u8=$mem8    IMM:u8=$index', 'MOD=MEM    ', 'PINSRB', ''];
T['PINSRD VREG:rw:s32=$XMMr REG:r:s32=$GPR32m IMM:u8=$index', 'MOD=REG W=0', 'PINSRx', ''];
T['PINSRD VREG:rw:s32=$XMMr MEM:r:s32=$mem32  IMM:u8=$index', 'MOD=MEM W=0', 'PINSRx', ''];
T['PINSRQ VREG:rw:s32=$XMMr REG:r:s64=$GPR64m IMM:u8=$index', 'MOD=REG W=1', 'PINSRx', ''];
T['PINSRQ VREG:rw:s32=$XMMr MEM:r:s64=$mem64  IMM:u8=$index', 'MOD=MEM W=1', 'PINSRx', ''];

# PMADDUBSW-Multiply and Add Packed Signed and Unsigned Bytes.
T['PMADDUBSW VREG:rw:s8=$XMMr VREG:r:s8=$XMMm ', 'MOD=REG', 'PMADDUBSW_romo', ''];
T['PMADDUBSW VREG:rw:s8=$XMMr MEM:r:s8=$mem128', 'MOD=MEM', 'PMADDUBSW_romo', ''];
T['PMADDUBSW REG:rw:s8=$MMXr  REG:r:s8=$MMXm  ', 'MOD=REG', 'PMADDUBSW_rqmq', ''];
T['PMADDUBSW REG:rw:s8=$MMXr  MEM:r:s8=$mem64 ', 'MOD=MEM', 'PMADDUBSW_rqmq', ''];

# PMADDWD-Multiply and Add Packed Integers.
T['PMADDWD VREG:rw:s16=$XMMr VREG:r:s16=$XMMm ', 'MOD=REG', 'PMADDWD_romo', ''];
T['PMADDWD VREG:rw:s16=$XMMr MEM:r:s16=$mem128', 'MOD=MEM', 'PMADDWD_romo', ''];

# PMAXSB/PMAXSW/PMAXSD/PMAXSQ-Maximum of Packed Signed Integers.
T['PMAXSB VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PMAXSB     ', ''];
T['PMAXSB VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PMAXSB     ', ''];
T['PMAXSD VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PMAXSD     ', ''];
T['PMAXSD VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PMAXSD     ', ''];
T['PMAXSW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PMAXSW_romo', ''];
T['PMAXSW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PMAXSW_romo', ''];

# PMAXUB/PMAXUW-Maximum of Packed Unsigned Integers.
T['PMAXUB VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PMAXUB_romo', ''];
T['PMAXUB VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PMAXUB_romo', ''];
T['PMAXUW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PMAXUW     ', ''];
T['PMAXUW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PMAXUW     ', ''];

# PMAXUD/PMAXUQ-Maximum of Packed Unsigned Integers.
T['PMAXUD VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PMAXUD', ''];
T['PMAXUD VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PMAXUD', ''];

# PMINSB/PMINSW-Minimum of Packed Signed Integers.
T['PMINSB VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PMINSB     ', ''];
T['PMINSB VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PMINSB     ', ''];
T['PMINSW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PMINSW_romo', ''];
T['PMINSW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PMINSW_romo', ''];

# PMINSD/PMINSQ-Minimum of Packed Signed Integers.
T['PMINSD VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PMINSD', ''];
T['PMINSD VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PMINSD', ''];

# PMINUB/PMINUW-Minimum of Packed Unsigned Integers.
T['PMINUB VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PMINUB_romo', ''];
T['PMINUB VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PMINUB_romo', ''];
T['PMINUW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PMINUW     ', ''];
T['PMINUW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PMINUW     ', ''];

# PMINUD/PMINUQ-Minimum of Packed Unsigned Integers.
T['PMINUD VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PMINUD', ''];
T['PMINUD VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PMINUD', ''];

# PMOVMSKB-Move Byte Mask.
T['PMOVMSKB REG:w=$GPR32r VREG:r:s8=$XMMm', 'NONE', 'PMOVMSKB_rdmo', ''];

# PMOVSX-Packed Move with Sign Extend.
T['PMOVSXBD VREG:w:s32=$XMMr VREG:r:s8=$XMMm ', 'MOD=REG', 'PMOVSXBD', ''];
T['PMOVSXBD VREG:w:s32=$XMMr MEM:r:s8=$mem32 ', 'MOD=MEM', 'PMOVSXBD', ''];
T['PMOVSXBQ VREG:w:s64=$XMMr VREG:r:s8=$XMMm ', 'MOD=REG', 'PMOVSXBQ', ''];
T['PMOVSXBQ VREG:w:s64=$XMMr MEM:r:s8=$mem16 ', 'MOD=MEM', 'PMOVSXBQ', ''];
T['PMOVSXBW VREG:w:s16=$XMMr VREG:r:s8=$XMMm ', 'MOD=REG', 'PMOVSXBW', ''];
T['PMOVSXBW VREG:w:s16=$XMMr MEM:r:s8=$mem64 ', 'MOD=MEM', 'PMOVSXBW', ''];
T['PMOVSXDQ VREG:w:s64=$XMMr VREG:r:s32=$XMMm', 'MOD=REG', 'PMOVSXDQ', ''];
T['PMOVSXDQ VREG:w:s64=$XMMr MEM:r:s32=$mem64', 'MOD=MEM', 'PMOVSXDQ', ''];
T['PMOVSXWD VREG:w:s32=$XMMr VREG:r:s16=$XMMm', 'MOD=REG', 'PMOVSXWD', ''];
T['PMOVSXWD VREG:w:s32=$XMMr MEM:r:s16=$mem64', 'MOD=MEM', 'PMOVSXWD', ''];
T['PMOVSXWQ VREG:w:s64=$XMMr VREG:r:s16=$XMMm', 'MOD=REG', 'PMOVSXWQ', ''];
T['PMOVSXWQ VREG:w:s64=$XMMr MEM:r:s16=$mem32', 'MOD=MEM', 'PMOVSXWQ', ''];

# PMOVZX-Packed Move with Zero Extend.
T['PMOVZXBD VREG:w:u32=$XMMr VREG:r:u8=$XMMm ', 'MOD=REG', 'PMOVZXBD', ''];
T['PMOVZXBD VREG:w:u32=$XMMr MEM:r:u8=$mem32 ', 'MOD=MEM', 'PMOVZXBD', ''];
T['PMOVZXBQ VREG:w:u64=$XMMr VREG:r:u8=$XMMm ', 'MOD=REG', 'PMOVZXBQ', ''];
T['PMOVZXBQ VREG:w:u64=$XMMr MEM:r:u8=$mem16 ', 'MOD=MEM', 'PMOVZXBQ', ''];
T['PMOVZXBW VREG:w:u16=$XMMr VREG:r:u8=$XMMm ', 'MOD=REG', 'PMOVZXBW', ''];
T['PMOVZXBW VREG:w:u16=$XMMr MEM:r:u8=$mem64 ', 'MOD=MEM', 'PMOVZXBW', ''];
T['PMOVZXDQ VREG:w:u64=$XMMr VREG:r:u32=$XMMm', 'MOD=REG', 'PMOVZXDQ', ''];
T['PMOVZXDQ VREG:w:u64=$XMMr MEM:r:u32=$mem64', 'MOD=MEM', 'PMOVZXDQ', ''];
T['PMOVZXWD VREG:w:u32=$XMMr VREG:r:u16=$XMMm', 'MOD=REG', 'PMOVZXWD', ''];
T['PMOVZXWD VREG:w:u32=$XMMr MEM:r:u16=$mem64', 'MOD=MEM', 'PMOVZXWD', ''];
T['PMOVZXWQ VREG:w:u64=$XMMr VREG:r:u16=$XMMm', 'MOD=REG', 'PMOVZXWQ', ''];
T['PMOVZXWQ VREG:w:u64=$XMMr MEM:r:u16=$mem32', 'MOD=MEM', 'PMOVZXWQ', ''];

# PMULDQ-Multiply Packed Doubleword Integers.
T['PMULDQ VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PMULDQ', ''];
T['PMULDQ VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PMULDQ', ''];

# PMULHRSW-Packed Multiply High with Round and Scale.
T['PMULHRSW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PMULHRSW_romo', ''];
T['PMULHRSW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PMULHRSW_romo', ''];
T['PMULHRSW REG:rw:s64=$MMXr  REG:r:s64=$MMXm  ', 'MOD=REG', 'PMULHRSW_rqmq', ''];
T['PMULHRSW REG:rw:s64=$MMXr  MEM:r:s64=$mem64 ', 'MOD=MEM', 'PMULHRSW_rqmq', ''];

# PMULHUW-Multiply Packed Unsigned Integers and Store High Result.
T['PMULHUW VREG:rw:u16=$XMMr VREG:r:u16=$XMMm ', 'MOD=REG', 'PMULHUW_romo', ''];
T['PMULHUW VREG:rw:u16=$XMMr MEM:r:u16=$mem128', 'MOD=MEM', 'PMULHUW_romo', ''];

# PMULHW-Multiply Packed Signed Integers and Store High Result.
T['PMULHW VREG:rw:s16=$XMMr VREG:r:s16=$XMMm ', 'MOD=REG', 'PMULHW_romo', ''];
T['PMULHW VREG:rw:s16=$XMMr MEM:r:s16=$mem128', 'MOD=MEM', 'PMULHW_romo', ''];

# PMULLW-Multiply Packed Signed Integers and Store Low Result.
T['PMULLW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PMULLW_romo', ''];
T['PMULLW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PMULLW_romo', ''];

# PMULLD/PMULLQ-Multiply Packed Integers and Store Low Result.
T['PMULLD VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PMULLD', ''];
T['PMULLD VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PMULLD', ''];

# PMULUDQ-Multiply Packed Unsigned Doubleword Integers.
T['PMULUDQ VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PMULUDQ_romo', ''];
T['PMULUDQ VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PMULUDQ_romo', ''];
T['PMULUDQ REG:rw:u32=$MMXr  REG:r:u32=$MMXm  ', 'MOD=REG', 'PMULUDQ_rqmq', ''];
T['PMULUDQ REG:rw:u32=$MMXr  MEM:r:u32=$mem64 ', 'MOD=MEM', 'PMULUDQ_rqmq', ''];

# POPCNT-Return the Count of Number of Bits Set to 1.
T['POPCNT REG:w:sx=$GPRvr REG:r:sx=$GPRvm', 'MOD=REG', 'POPCNT_rvmv', ''];
T['POPCNT REG:w:sx=$GPRvr MEM:r:sx=$memv ', 'MOD=MEM', 'POPCNT_rvmv', ''];

# POR-Bitwise Logical OR.
T['POR VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'POR_romo', ''];
T['POR VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'POR_romo', ''];

# PREFETCHh-Prefetch Data Into Caches.
T['PREFETCHNTA MEM:r:s64=$mem8', 'NONE', 'PREFETCHNTA', ''];
T['PREFETCHT0  MEM:r:s64=$mem8', 'NONE', 'PREFETCHT0 ', ''];
T['PREFETCHT1  MEM:r:s64=$mem8', 'NONE', 'PREFETCHT1 ', ''];
T['PREFETCHT2  MEM:r:s64=$mem8', 'NONE', 'PREFETCHT2 ', ''];

# PSADBW-Compute Sum of Absolute Differences.
T['PSADBW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PSADBW_romo', ''];
T['PSADBW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PSADBW_romo', ''];

# PSHUFB-Packed Shuffle Bytes.
T['PSHUFB VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PSHUFB_romo', ''];
T['PSHUFB VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PSHUFB_romo', ''];
T['PSHUFB REG:rw:s64=$MMXr  REG:r:s64=$MMXm  ', 'MOD=REG', 'PSHUFB_rqmq', ''];
T['PSHUFB REG:rw:s64=$MMXr  MEM:r:s64=$mem64 ', 'MOD=MEM', 'PSHUFB_rqmq', ''];

# PSHUFD-Shuffle Packed Doublewords.
T['PSHUFD VREG:w:u32=$XMMr VREG:r:u32=$XMMm  ORDER:r:u8=$order', 'MOD=REG', 'PSHUFD', ''];
T['PSHUFD VREG:w:u32=$XMMr MEM:r:u32=$mem128 ORDER:r:u8=$order', 'MOD=MEM', 'PSHUFD', ''];

# PSHUFHW-Shuffle Packed High Words.
T['PSHUFHW VREG:w:u16=$XMMr VREG:r:u16=$XMMm  ORDER:r:u8=$order', 'MOD=REG', 'PSHUFHW', ''];
T['PSHUFHW VREG:w:u16=$XMMr MEM:r:u16=$mem128 ORDER:r:u8=$order', 'MOD=MEM', 'PSHUFHW', ''];

# PSHUFLW-Shuffle Packed Low Words.
T['PSHUFLW VREG:w:u16=$XMMr VREG:r:u16=$XMMm  ORDER:r:u8=$order', 'MOD=REG', 'PSHUFLW', ''];
T['PSHUFLW VREG:w:u16=$XMMr MEM:r:u16=$mem128 ORDER:r:u8=$order', 'MOD=MEM', 'PSHUFLW', ''];

# PSIGNB/PSIGNW/PSIGND-Packed SIGN.
T['PSIGNB VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PSIGNB_romo', ''];
T['PSIGNB VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PSIGNB_romo', ''];
T['PSIGNB REG:rw:s64=$MMXr  REG:r:s64=$MMXm  ', 'MOD=REG', 'PSIGNB_rqmq', ''];
T['PSIGNB REG:rw:s64=$MMXr  MEM:r:s64=$mem64 ', 'MOD=MEM', 'PSIGNB_rqmq', ''];
T['PSIGND VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PSIGND_romo', ''];
T['PSIGND VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PSIGND_romo', ''];
T['PSIGND REG:rw:s64=$MMXr  REG:r:s64=$MMXm  ', 'MOD=REG', 'PSIGND_rqmq', ''];
T['PSIGND REG:rw:s64=$MMXr  MEM:r:s64=$mem64 ', 'MOD=MEM', 'PSIGND_rqmq', ''];
T['PSIGNW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PSIGNW_romo', ''];
T['PSIGNW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PSIGNW_romo', ''];
T['PSIGNW REG:rw:s64=$MMXr  REG:r:s64=$MMXm  ', 'MOD=REG', 'PSIGNW_rqmq', ''];
T['PSIGNW REG:rw:s64=$MMXr  MEM:r:s64=$mem64 ', 'MOD=MEM', 'PSIGNW_rqmq', ''];

# PSLLDQ-Shift Double Quadword Left Logical.
T['PSLLDQ REG:rw:u128=$XMMm SHIFT:r:u8=$shl', 'NONE', 'PSLLDQ', ''];

# PSLLW/PSLLD/PSLLQ-Shift Packed Data Left Logical.
T['PSLLD VREG:rw:u32=$XMMm SHIFT:r:u8=$shl  ', 'NONE', 'PSLLD_moub  ', ''];
T['PSLLD VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'NONE', 'PSLLD_romo_1', ''];
T['PSLLD VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'NONE', 'PSLLD_romo_2', ''];
T['PSLLQ VREG:rw:u64=$XMMm SHIFT:r:u8=$shl  ', 'NONE', 'PSLLQ_moub  ', ''];
T['PSLLQ VREG:rw:u64=$XMMr VREG:r:u64=$XMMm ', 'NONE', 'PSLLQ_romo_1', ''];
T['PSLLQ VREG:rw:u64=$XMMr MEM:r:u64=$mem128', 'NONE', 'PSLLQ_romo_2', ''];
T['PSLLW VREG:rw:u16=$XMMm SHIFT:r:u8=$shl  ', 'NONE', 'PSLLW_moub  ', ''];
T['PSLLW VREG:rw:u16=$XMMr VREG:r:s32=$XMMm ', 'NONE', 'PSLLW_romo_1', ''];
T['PSLLW VREG:rw:u16=$XMMr MEM:r:s32=$mem128', 'NONE', 'PSLLW_romo_2', ''];

# PSRAW/PSRAD/PSRAQ-Shift Packed Data Right Arithmetic.
T['PSRAD VREG:rw:s32=$XMMm SHIFT:r:u8=$shr  ', 'NONE', 'PSRAD_moub  ', ''];
T['PSRAD VREG:rw:s32=$XMMr VREG:r:u64=$XMMm ', 'NONE', 'PSRAD_romo_1', ''];
T['PSRAD VREG:rw:s32=$XMMr MEM:r:u64=$mem128', 'NONE', 'PSRAD_romo_2', ''];
T['PSRAW VREG:rw:s16=$XMMm SHIFT:r:u8=$shr  ', 'NONE', 'PSRAW_moub  ', ''];
T['PSRAW VREG:rw:s16=$XMMr VREG:r:u64=$XMMm ', 'NONE', 'PSRAW_romo_1', ''];
T['PSRAW VREG:rw:s16=$XMMr MEM:r:u64=$mem128', 'NONE', 'PSRAW_romo_2', ''];

# PSRLDQ-Shift Double Quadword Right Logical.
T['PSRLDQ REG:rw:u128=$XMMm SHIFT:r:u8=$shr', 'NONE', 'PSRLDQ', ''];

# PSRLW/PSRLD/PSRLQ-Shift Packed Data Right Logical.
T['PSRLD VREG:rw:u32=$XMMm SHIFT:r:u8=$shr  ', 'NONE', 'PSRLD_moub  ', ''];
T['PSRLD VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'NONE', 'PSRLD_romo_1', ''];
T['PSRLD VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'NONE', 'PSRLD_romo_2', ''];
T['PSRLQ VREG:rw:u64=$XMMm SHIFT:r:u8=$shr  ', 'NONE', 'PSRLQ_moub  ', ''];
T['PSRLQ VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'NONE', 'PSRLQ_romo_1', ''];
T['PSRLQ VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'NONE', 'PSRLQ_romo_2', ''];
T['PSRLW VREG:rw:u16=$XMMm SHIFT:r:u8=$shr  ', 'NONE', 'PSRLW_moub  ', ''];
T['PSRLW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'NONE', 'PSRLW_romo_1', ''];
T['PSRLW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'NONE', 'PSRLW_romo_2', ''];

# PSUBQ-Subtract Packed Quadword Integers.
T['PSUBQ VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PSUBQ_romo', ''];
T['PSUBQ VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PSUBQ_romo', ''];
T['PSUBQ REG:rw:s64=$MMXr  REG:r:s64=$MMXm  ', 'MOD=REG', 'PSUBQ_rqmq', ''];
T['PSUBQ REG:rw:s64=$MMXr  MEM:r:s64=$mem64 ', 'MOD=MEM', 'PSUBQ_rqmq', ''];

# PSUBSB/PSUBSW-Subtract Packed Signed Integers with Signed Saturation.
T['PSUBSB VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PSUBSB_romo', ''];
T['PSUBSB VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PSUBSB_romo', ''];
T['PSUBSW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PSUBSW_romo', ''];
T['PSUBSW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PSUBSW_romo', ''];

# PSUBUSB/PSUBUSW-Subtract Packed Unsigned Integers with Unsigned Saturation.
T['PSUBUSB VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PSUBUSB_romo', ''];
T['PSUBUSB VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PSUBUSB_romo', ''];
T['PSUBUSW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PSUBUSW_romo', ''];
T['PSUBUSW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PSUBUSW_romo', ''];

# PSUBB/PSUBW/PSUBD-Subtract Packed Integers.
T['PSUBB VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PSUBB_romo', ''];
T['PSUBB VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PSUBB_romo', ''];
T['PSUBD VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PSUBD_romo', ''];
T['PSUBD VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PSUBD_romo', ''];
T['PSUBW VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PSUBW_romo', ''];
T['PSUBW VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PSUBW_romo', ''];

# PTEST-Logical Compare.
T['PTEST VREG:r:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PTEST', ''];
T['PTEST VREG:r:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PTEST', ''];

# PUNPCKHBW/PUNPCKHWD/PUNPCKHDQ/PUNPCKHQDQ-Unpack High Data.
T['PUNPCKHBW  VREG:rw:s32=$XMMr REG:r:s64=$XMMm  ', 'MOD=REG', 'PUNPCKHBW_romx', ''];
T['PUNPCKHBW  VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PUNPCKHBW_romx', ''];
T['PUNPCKHDQ  VREG:rw:s32=$XMMr REG:r:s64=$XMMm  ', 'MOD=REG', 'PUNPCKHDQ_romx', ''];
T['PUNPCKHDQ  VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PUNPCKHDQ_romx', ''];
T['PUNPCKHQDQ VREG:rw:s32=$XMMr REG:r:s64=$XMMm  ', 'MOD=REG', 'PUNPCKHQDQ    ', ''];
T['PUNPCKHQDQ VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PUNPCKHQDQ    ', ''];
T['PUNPCKHWD  VREG:rw:s32=$XMMr REG:r:s64=$XMMm  ', 'MOD=REG', 'PUNPCKHWD_romx', ''];
T['PUNPCKHWD  VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PUNPCKHWD_romx', ''];

# PUNPCKLBW/PUNPCKLWD/PUNPCKLDQ/PUNPCKLQDQ-Unpack Low Data.
T['PUNPCKLBW  VREG:rw:s32=$XMMr REG:r:s64=$XMMm  ', 'MOD=REG', 'PUNPCKLBW_romx', ''];
T['PUNPCKLBW  VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PUNPCKLBW_romx', ''];
T['PUNPCKLDQ  VREG:rw:s32=$XMMr REG:r:s64=$XMMm  ', 'MOD=REG', 'PUNPCKLDQ_romx', ''];
T['PUNPCKLDQ  VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PUNPCKLDQ_romx', ''];
T['PUNPCKLQDQ VREG:rw:s32=$XMMr REG:r:s64=$XMMm  ', 'MOD=REG', 'PUNPCKLQDQ    ', ''];
T['PUNPCKLQDQ VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PUNPCKLQDQ    ', ''];
T['PUNPCKLWD  VREG:rw:s32=$XMMr REG:r:s64=$XMMm  ', 'MOD=REG', 'PUNPCKLWD_romx', ''];
T['PUNPCKLWD  VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PUNPCKLWD_romx', ''];

# PXOR-Logical Exclusive OR.
T['PXOR VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'PXOR_romo', ''];
T['PXOR VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'PXOR_romo', ''];

# RCPPS-Compute Reciprocals of Packed Single-Precision Floating-Point Values.
T['RCPPS VREG:w:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'RCPPS', ''];
T['RCPPS VREG:w:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'RCPPS', ''];

# RCPSS-Compute Reciprocal of Scalar Single-Precision Floating-Point Values.
T['RCPSS REG:w:f32=$XMMr REG:r:f32=$XMMm ', 'MOD=REG', 'RCPSS', ''];
T['RCPSS REG:w:f32=$XMMr MEM:r:f32=$mem32', 'MOD=MEM', 'RCPSS', ''];

# ROUNDPD-Round Packed Double Precision Floating-Point Values.
T['ROUNDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMm  RNDC:r:u8=$rndc2', 'MOD=REG', 'ROUNDPD', ''];
T['ROUNDPD VREG:w:f64=$XMMr MEM:r:f64=$mem128 RNDC:r:u8=$rndc2', 'MOD=MEM', 'ROUNDPD', ''];

# ROUNDPS-Round Packed Single Precision Floating-Point Values.
T['ROUNDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMm  RNDC:r:u8=$rndc2', 'MOD=REG', 'ROUNDPS', ''];
T['ROUNDPS VREG:w:f32=$XMMr MEM:r:f32=$mem128 RNDC:r:u8=$rndc2', 'MOD=MEM', 'ROUNDPS', ''];

# ROUNDSD-Round Scalar Double Precision Floating-Point Values.
T['ROUNDSD REG:w:s64=$XMMr REG:r:s64=$XMMm  RNDC:r:u8=$rndc2', 'MOD=REG', 'ROUNDSD', ''];
T['ROUNDSD REG:w:s64=$XMMr MEM:r:s64=$mem64 RNDC:r:u8=$rndc2', 'MOD=MEM', 'ROUNDSD', ''];

# ROUNDSS-Round Scalar Single Precision Floating-Point Values.
T['ROUNDSS REG:w:s32=$XMMr REG:r:s32=$XMMm  RNDC:r:u8=$rndc2', 'MOD=REG', 'ROUNDSS', ''];
T['ROUNDSS REG:w:s32=$XMMr MEM:r:s32=$mem32 RNDC:r:u8=$rndc2', 'MOD=MEM', 'ROUNDSS', ''];

# RSQRTPS-Compute Reciprocals of Square Roots of Packed Single-Precision Floating-Point Values.
T['RSQRTPS VREG:w:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'RSQRTPS', ''];
T['RSQRTPS VREG:w:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'RSQRTPS', ''];

# RSQRTSS-Compute Reciprocal of Square Root of Scalar Single-Precision Floating-Point Value.
T['RSQRTSS REG:w:f32=$XMMr REG:r:f32=$XMMm ', 'MOD=REG', 'RSQRTSS', ''];
T['RSQRTSS REG:w:f32=$XMMr MEM:r:f32=$mem32', 'MOD=MEM', 'RSQRTSS', ''];

# SFENCE-Store Fence.
T['SFENCE', 'NONE', 'SFENCE', ''];

# SHUFPD-Packed Interleave Shuffle of Pairs of Double-Precision Floating-Point Values.
T['SHUFPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm  ORDER:r:u8=$order', 'MOD=REG', 'SHUFPD', ''];
T['SHUFPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128 ORDER:r:u8=$order', 'MOD=MEM', 'SHUFPD', ''];

# SHUFPS-Packed Interleave Shuffle of Quadruplets of Single-Precision Floating-Point Values.
T['SHUFPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm  ORDER:r:u8=$order', 'MOD=REG', 'SHUFPS', ''];
T['SHUFPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128 ORDER:r:u8=$order', 'MOD=MEM', 'SHUFPS', ''];

# SQRTPD-Square Root of Double-Precision Floating-Point Values.
T['SQRTPD VREG:w:f64=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG', 'SQRTPD', ''];
T['SQRTPD VREG:w:f64=$XMMr MEM:r:f64=$mem128', 'MOD=MEM', 'SQRTPD', ''];

# SQRTPS-Square Root of Single-Precision Floating-Point Values.
T['SQRTPS VREG:w:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'SQRTPS', ''];
T['SQRTPS VREG:w:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'SQRTPS', ''];

# SQRTSD-Compute Square Root of Scalar Double-Precision Floating-Point Value.
T['SQRTSD REG:w:f64=$XMMr REG:r:f64=$XMMm ', 'MOD=REG', 'SQRTSD', ''];
T['SQRTSD REG:w:f64=$XMMr MEM:r:f64=$mem64', 'MOD=MEM', 'SQRTSD', ''];

# SQRTSS-Compute Square Root of Scalar Single-Precision Value.
T['SQRTSS REG:w:f32=$XMMr REG:r:f32=$XMMm ', 'MOD=REG', 'SQRTSS', ''];
T['SQRTSS REG:w:f32=$XMMr MEM:r:f32=$mem32', 'MOD=MEM', 'SQRTSS', ''];

# STMXCSR-Store MXCSR Register State.
T['STMXCSR MEM:w:s32=$mem32 REG:SUPP:r=MXCSR', 'NONE', 'STMXCSR', ''];

# SUBPD-Subtract Packed Double-Precision Floating-Point Values.
T['SUBPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG', 'SUBPD', ''];
T['SUBPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128', 'MOD=MEM', 'SUBPD', ''];

# SUBPS-Subtract Packed Single-Precision Floating-Point Values.
T['SUBPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'SUBPS', ''];
T['SUBPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'SUBPS', ''];

# SUBSD-Subtract Scalar Double-Precision Floating-Point Value.
T['SUBSD REG:rw:f64=$XMMr REG:r:f64=$XMMm ', 'MOD=REG', 'SUBSD', ''];
T['SUBSD REG:rw:f64=$XMMr MEM:r:f64=$mem64', 'MOD=MEM', 'SUBSD', ''];

# SUBSS-Subtract Scalar Single-Precision Floating-Point Value.
T['SUBSS REG:rw:f32=$XMMr REG:r:f32=$XMMm ', 'MOD=REG', 'SUBSS', ''];
T['SUBSS REG:rw:f32=$XMMr MEM:r:f32=$mem32', 'MOD=MEM', 'SUBSS', ''];

# UCOMISD-Unordered Compare Scalar Double-Precision Floating-Point Values and Set EFLAGS.
T['UCOMISD REG:r:f64=$XMMr REG:r:f64=$XMMm ', 'MOD=REG', 'UCOMISD', ''];
T['UCOMISD REG:r:f64=$XMMr MEM:r:f64=$mem64', 'MOD=MEM', 'UCOMISD', ''];

# UCOMISS-Unordered Compare Scalar Single-Precision Floating-Point Values and Set EFLAGS.
T['UCOMISS REG:r:f32=$XMMr REG:r:f32=$XMMm ', 'MOD=REG', 'UCOMISS', ''];
T['UCOMISS REG:r:f32=$XMMr MEM:r:f32=$mem32', 'MOD=MEM', 'UCOMISS', ''];

# UNPCKHPD-Unpack and Interleave High Packed Double-Precision Floating-Point Values.
T['UNPCKHPD VREG:rw:f64=$XMMr REG:r:s64=$XMMm  ', 'MOD=REG', 'UNPCKHPD', ''];
T['UNPCKHPD VREG:rw:f64=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'UNPCKHPD', ''];

# UNPCKHPS-Unpack and Interleave High Packed Single-Precision Floating-Point Values.
T['UNPCKHPS VREG:rw:f32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'UNPCKHPS', ''];
T['UNPCKHPS VREG:rw:f32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'UNPCKHPS', ''];

# UNPCKLPD-Unpack and Interleave Low Packed Double-Precision Floating-Point Values.
T['UNPCKLPD VREG:rw:f64=$XMMr REG:r:s64=$XMMm  ', 'MOD=REG', 'UNPCKLPD', ''];
T['UNPCKLPD VREG:rw:f64=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'UNPCKLPD', ''];

# UNPCKLPS-Unpack and Interleave Low Packed Single-Precision Floating-Point Values.
T['UNPCKLPS VREG:rw:f32=$XMMr REG:r:s64=$XMMm  ', 'MOD=REG', 'UNPCKLPS', ''];
T['UNPCKLPS VREG:rw:f32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'UNPCKLPS', ''];

# XORPD-Bitwise Logical XOR of Packed Double Precision Floating-Point Values.
T['XORPD VREG:rw:f64=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG', 'XORPD', ''];
T['XORPD VREG:rw:f64=$XMMr MEM:r:f64=$mem128', 'MOD=MEM', 'XORPD', ''];

# XORPS-Bitwise Logical XOR of Packed Single Precision Floating-Point Values.
T['XORPS VREG:rw:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG', 'XORPS', ''];
T['XORPS VREG:rw:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM', 'XORPS', ''];
