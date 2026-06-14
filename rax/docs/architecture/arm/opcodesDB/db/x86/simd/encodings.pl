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
ENCODING ADDPD => {
  diagram    => 'MAP=0f MR=1 OP=0x58 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=ADDPD',
};


# ADDPS-Add Packed Single-Precision Floating-Point Values.
ENCODING ADDPS => {
  diagram    => 'MAP=0f MR=1 OP=0x58 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=ADDPS',
};


# ADDSD-Add Scalar Double-Precision Floating-Point Values.
ENCODING ADDSD => {
  diagram    => 'MAP=0f MR=1 OP=0x58 P66=0 PF2=1 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2',
  tags       => 'page=ADDSD',
};


# ADDSS-Add Scalar Single-Precision Floating-Point Values.
ENCODING ADDSS => {
  diagram    => 'MAP=0f MR=1 OP=0x58 P66=0 PF2=0 PF3=1',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE',
  tags       => 'page=ADDSS',
};


# ADDSUBPD-Packed Double-FP Add/Subtract.
ENCODING ADDSUBPD => {
  diagram    => 'MAP=0f MR=1 OP=0xd0 P66=1 PF2=0 PF3=0',
  extensions => 'SSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|CARITHMETIC',
  metadata   => 'isa=SSE3 requires_alignment=1',
  tags       => 'page=ADDSUBPD',
};


# ADDSUBPS-Packed Single-FP Add/Subtract.
ENCODING ADDSUBPS => {
  diagram    => 'MAP=0f MR=1 OP=0xd0 P66=0 PF2=1 PF3=0',
  extensions => 'SSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|CARITHMETIC',
  metadata   => 'isa=SSE3 requires_alignment=1',
  tags       => 'page=ADDSUBPS',
};


# ANDNPD-Bitwise Logical AND NOT of Packed Double Precision Floating-Point Values.
ENCODING ANDNPD => {
  diagram    => 'MAP=0f MR=1 OP=0x55 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=ANDNPD',
};


# ANDNPS-Bitwise Logical AND NOT of Packed Single Precision Floating-Point Values.
ENCODING ANDNPS => {
  diagram    => 'MAP=0f MR=1 OP=0x55 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=ANDNPS',
};


# ANDPD-Bitwise Logical AND of Packed Double Precision Floating-Point Values.
ENCODING ANDPD => {
  diagram    => 'MAP=0f MR=1 OP=0x54 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=ANDPD',
};


# ANDPS-Bitwise Logical AND of Packed Single Precision Floating-Point Values.
ENCODING ANDPS => {
  diagram    => 'MAP=0f MR=1 OP=0x54 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=ANDPS',
};


# BLENDPD-Blend Packed Double Precision Floating-Point Values.
ENCODING BLENDPD => {
  diagram    => 'MAP=0f3a MR=1 OP=0x0d P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=BLENDPD',
};


# BLENDPS-Blend Packed Single Precision Floating-Point Values.
ENCODING BLENDPS => {
  diagram    => 'MAP=0f3a MR=1 OP=0x0c P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=BLENDPS',
};


# BLENDVPD-Variable Blend Packed Double Precision Floating-Point Values.
ENCODING BLENDVPD => {
  diagram    => 'MAP=0f38 MR=1 OP=0x15 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=BLENDVPD',
};


# BLENDVPS-Variable Blend Packed Single Precision Floating-Point Values.
ENCODING BLENDVPS => {
  diagram    => 'MAP=0f38 MR=1 OP=0x14 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=BLENDVPS',
};


# CMPPD-Compare Packed Double-Precision Floating-Point Values.
ENCODING CMPPD => {
  diagram    => 'MAP=0f MR=1 OP=0xc2 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|COMPARISON',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=CMPPD',
};


# CMPPS-Compare Packed Single-Precision Floating-Point Values.
ENCODING CMPPS => {
  diagram    => 'MAP=0f MR=1 OP=0xc2 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|COMPARISON',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=CMPPS',
};


# CMPS/CMPSB/CMPSW/CMPSD/CMPSQ-Compare String Operands.
ENCODING CMPSD_rqmqub => {
  diagram    => 'MAP=0f MR=1 OP=0xc2 P66=0 PF2=1 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|COMPARISON',
  metadata   => 'isa=SSE2',
  tags       => 'page=CMPS',
};


# CMPSS-Compare Scalar Single-Precision Floating-Point Value.
ENCODING CMPSS => {
  diagram    => 'MAP=0f MR=1 OP=0xc2 P66=0 PF2=0 PF3=1',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|COMPARISON',
  metadata   => 'isa=SSE',
  tags       => 'page=CMPSS',
};


# COMISD-Compare Scalar Ordered Double-Precision Floating-Point Values and Set EFLAGS.
ENCODING COMISD => {
  diagram    => 'MAP=0f MR=1 OP=0x2f P66=1 PF2=0 PF3=0',
  iflags     => 'af=Z cf=W of=Z pf=W sf=Z zf=W',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|COMPARISON',
  metadata   => 'isa=SSE2',
  tags       => 'page=COMISD',
};


# COMISS-Compare Scalar Ordered Single-Precision Floating-Point Values and Set EFLAGS.
ENCODING COMISS => {
  diagram    => 'MAP=0f MR=1 OP=0x2f P66=0 PF2=0 PF3=0',
  iflags     => 'af=Z cf=W of=Z pf=W sf=Z zf=W',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|COMPARISON',
  metadata   => 'isa=SSE',
  tags       => 'page=COMISS',
};


# CRC32-Accumulate CRC32 Value.
ENCODING CRC32_rymb => {
  diagram    => 'MAP=0f38 MR=1 OP=0xf0 P66=0 PF2=1 PF3=0',
  extensions => 'SSE4',
  categories => 'SIMD|CRYPTO',
  metadata   => 'isa=SSE42',
  tags       => 'page=CRC32',
};

ENCODING CRC32_rymv => {
  diagram    => 'MAP=0f38 MR=1 OP=0xf1 P66=0 PF2=1 PF3=0',
  extensions => 'SSE4',
  categories => 'SIMD|CRYPTO',
  metadata   => 'isa=SSE42',
  tags       => 'page=CRC32',
};


# CVTDQ2PD-Convert Packed Doubleword Integers to Packed Double-Precision Floating-Point Values.
ENCODING CVTDQ2PD => {
  diagram    => 'MAP=0f MR=1 OP=0xe6 P66=0 PF2=0 PF3=1',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2',
  tags       => 'page=CVTDQ2PD',
};


# CVTDQ2PS-Convert Packed Doubleword Integers to Packed Single-Precision Floating-Point Values.
ENCODING CVTDQ2PS => {
  diagram    => 'MAP=0f MR=1 OP=0x5b P66=0 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=CVTDQ2PS',
};


# CVTPD2DQ-Convert Packed Double-Precision Floating-Point Values to Packed Doubleword Integers.
ENCODING CVTPD2DQ => {
  diagram    => 'MAP=0f MR=1 OP=0xe6 P66=0 PF2=1 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=CVTPD2DQ',
};


# CVTPD2PI-Convert Packed Double-Precision FP Values to Packed Dword Integers.
ENCODING CVTPD2PI => {
  diagram    => 'MAP=0f MR=1 OP=0x2d P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|MMX|MMX_FP_16ALIGN',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=CVTPD2PI',
};


# CVTPD2PS-Convert Packed Double-Precision Floating-Point Values to Packed Single-Precision Floating-Point Values.
ENCODING CVTPD2PS => {
  diagram    => 'MAP=0f MR=1 OP=0x5a P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=CVTPD2PS',
};


# CVTPI2PD-Convert Packed Dword Integers to Packed Double-Precision FP Values.
ENCODING CVTPI2PD => {
  diagram    => 'MAP=0f MR=1 OP=0x2a P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'MMX|MMX_NOFP',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2',
  tags       => 'page=CVTPI2PD',
};


# CVTPI2PS-Convert Packed Dword Integers to Packed Single-Precision FP Values.
ENCODING CVTPI2PS => {
  diagram    => 'MAP=0f MR=1 OP=0x2a P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'MMX|MMX_FP',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE',
  tags       => 'page=CVTPI2PS',
};


# CVTPS2DQ-Convert Packed Single-Precision Floating-Point Values to Packed Signed Doubleword Integer Values .3-258.
ENCODING CVTPS2DQ => {
  diagram    => 'MAP=0f MR=1 OP=0x5b P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=CVTPS2DQ',
};


# CVTPS2PD-Convert Packed Single-Precision Floating-Point Values to Packed Double-Precision Floating-Point Values.
ENCODING CVTPS2PD => {
  diagram    => 'MAP=0f MR=1 OP=0x5a P66=0 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2',
  tags       => 'page=CVTPS2PD',
};


# CVTPS2PI-Convert Packed Single-Precision FP Values to Packed Dword Integers.
ENCODING CVTPS2PI => {
  diagram    => 'MAP=0f MR=1 OP=0x2d P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'MMX|MMX_FP',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE',
  tags       => 'page=CVTPS2PI',
};


# CVTSD2SI-Convert Scalar Double-Precision Floating-Point Value to Doubleword Integer.
ENCODING CVTSD2SI => {
  diagram    => 'MAP=0f MR=1 OP=0x2d P66=0 PF2=1 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2',
  tags       => 'page=CVTSD2SI',
};


# CVTSD2SS-Convert Scalar Double-Precision Floating-Point Value to Scalar Single-Precision Floating-Point Value. .3-267.
ENCODING CVTSD2SS => {
  diagram    => 'MAP=0f MR=1 OP=0x5a P66=0 PF2=1 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2',
  tags       => 'page=CVTSD2SS',
};


# CVTSI2SD-Convert Doubleword Integer to Scalar Double-Precision Floating-Point Value.
ENCODING CVTSI2SD => {
  diagram    => 'MAP=0f MR=1 OP=0x2a P66=0 PF2=1 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2',
  tags       => 'page=CVTSI2SD',
};


# CVTSI2SS-Convert Doubleword Integer to Scalar Single-Precision Floating-Point Value.
ENCODING CVTSI2SS => {
  diagram    => 'MAP=0f MR=1 OP=0x2a P66=0 PF2=0 PF3=1',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE',
  tags       => 'page=CVTSI2SS',
};


# CVTSS2SD-Convert Scalar Single-Precision Floating-Point Value to Scalar Double-Precision Floating-Point Value. .3-273.
ENCODING CVTSS2SD => {
  diagram    => 'MAP=0f MR=1 OP=0x5a P66=0 PF2=0 PF3=1',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2',
  tags       => 'page=CVTSS2SD',
};


# CVTSS2SI-Convert Scalar Single-Precision Floating-Point Value to Doubleword Integer.
ENCODING CVTSS2SI => {
  diagram    => 'MAP=0f MR=1 OP=0x2d P66=0 PF2=0 PF3=1',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE',
  tags       => 'page=CVTSS2SI',
};


# CVTTPD2DQ-Convert with Truncation Packed Double-Precision Floating-Point Values to Packed Doubleword Integers.
ENCODING CVTTPD2DQ => {
  diagram    => 'MAP=0f MR=1 OP=0xe6 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=CVTTPD2DQ',
};


# CVTTPD2PI-Convert with Truncation Packed Double-Precision FP Values to Packed Dword Integers.
ENCODING CVTTPD2PI => {
  diagram    => 'MAP=0f MR=1 OP=0x2c P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|MMX|MMX_FP_16ALIGN',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=CVTTPD2PI',
};


# CVTTPS2DQ-Convert with Truncation Packed Single-Precision Floating-Point Values to Packed Signed Doubleword Integer Values.
ENCODING CVTTPS2DQ => {
  diagram    => 'MAP=0f MR=1 OP=0x5b P66=0 PF2=0 PF3=1',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=CVTTPS2DQ',
};


# CVTTPS2PI-Convert with Truncation Packed Single-Precision FP Values to Packed Dword Integers.
ENCODING CVTTPS2PI => {
  diagram    => 'MAP=0f MR=1 OP=0x2c P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'MMX|MMX_FP',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE',
  tags       => 'page=CVTTPS2PI',
};


# CVTTSD2SI-Convert with Truncation Scalar Double-Precision Floating-Point Value to Signed Integer.
ENCODING CVTTSD2SI => {
  diagram    => 'MAP=0f MR=1 OP=0x2c P66=0 PF2=1 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2',
  tags       => 'page=CVTTSD2SI',
};


# CVTTSS2SI-Convert with Truncation Scalar Single-Precision Floating-Point Value to Integer.
ENCODING CVTTSS2SI => {
  diagram    => 'MAP=0f MR=1 OP=0x2c P66=0 PF2=0 PF3=1',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE',
  tags       => 'page=CVTTSS2SI',
};


# DIVPD-Divide Packed Double-Precision Floating-Point Values.
ENCODING DIVPD => {
  diagram    => 'MAP=0f MR=1 OP=0x5e P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=DIVPD',
};


# DIVPS-Divide Packed Single-Precision Floating-Point Values.
ENCODING DIVPS => {
  diagram    => 'MAP=0f MR=1 OP=0x5e P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=DIVPS',
};


# DIVSD-Divide Scalar Double-Precision Floating-Point Value.
ENCODING DIVSD => {
  diagram    => 'MAP=0f MR=1 OP=0x5e P66=0 PF2=1 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2',
  tags       => 'page=DIVSD',
};


# DIVSS-Divide Scalar Single-Precision Floating-Point Values.
ENCODING DIVSS => {
  diagram    => 'MAP=0f MR=1 OP=0x5e P66=0 PF2=0 PF3=1',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE',
  tags       => 'page=DIVSS',
};


# DPPD-Dot Product of Packed Double Precision Floating-Point Values.
ENCODING DPPD => {
  diagram    => 'MAP=0f3a MR=1 OP=0x41 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_2D',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=DPPD',
};


# DPPS-Dot Product of Packed Single Precision Floating-Point Values.
ENCODING DPPS => {
  diagram    => 'MAP=0f3a MR=1 OP=0x40 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_2D',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=DPPS',
};


# EXTRACTPS-Extract Packed Floating-Point Values.
ENCODING EXTRACTPS => {
  diagram    => 'MAP=0f3a MR=1 OP=0x17 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE4',
  tags       => 'page=EXTRACTPS',
};


# FISTTP-Store Integer with Truncation.
ENCODING FISTTP_md80 => {
  diagram    => 'MOD=MEM MR=1 OP=0xdb REG=1',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'SSE3',
  categories => 'SIMD',
  metadata   => 'isa=SSE3',
  tags       => 'page=FISTTP',
};

ENCODING FISTTP_mq80 => {
  diagram    => 'MOD=MEM MR=1 OP=0xdd REG=1',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'SSE3',
  categories => 'SIMD',
  metadata   => 'isa=SSE3X87',
  tags       => 'page=FISTTP',
};

ENCODING FISTTP_mw80 => {
  diagram    => 'MOD=MEM MR=1 OP=0xdf REG=1',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'SSE3',
  categories => 'SIMD',
  metadata   => 'isa=SSE3X87',
  tags       => 'page=FISTTP',
};


# FXRSTOR-Restore x87 FPU, MMX, XMM, and MXCSR State.
ENCODING FXRSTOR => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xae P66=0 PF2=0 PF3=0 REG=1 W=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|OS',
  metadata   => 'isa=FXSAVE requires_alignment=1',
  docvars    => 'XMM_STATE_W=1',
  tags       => 'page=FXRSTOR',
};

ENCODING FXRSTOR64 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xae P66=0 PF2=0 PF3=0 REG=1 W=1',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|OS',
  metadata   => 'isa=FXSAVE64 requires_alignment=1',
  docvars    => 'XMM_STATE_W=1',
  tags       => 'page=FXRSTOR',
};


# FXSAVE-Save x87 FPU, MMX Technology, and SSE State.
ENCODING FXSAVE => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xae P66=0 PF2=0 PF3=0 REG=0 W=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|OS',
  metadata   => 'isa=FXSAVE requires_alignment=1',
  docvars    => 'XMM_STATE_R=1',
  tags       => 'page=FXSAVE',
};

ENCODING FXSAVE64 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xae P66=0 PF2=0 PF3=0 REG=0 W=1',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|OS',
  metadata   => 'isa=FXSAVE64 requires_alignment=1',
  docvars    => 'XMM_STATE_R=1',
  tags       => 'page=FXSAVE',
};


# HADDPD-Packed Double-FP Horizontal Add.
ENCODING HADDPD => {
  diagram    => 'MAP=0f MR=1 OP=0x7c P66=1 PF2=0 PF3=0',
  extensions => 'SSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE3 requires_alignment=1',
  tags       => 'page=HADDPD',
};


# HADDPS-Packed Single-FP Horizontal Add.
ENCODING HADDPS => {
  diagram    => 'MAP=0f MR=1 OP=0x7c P66=0 PF2=1 PF3=0',
  extensions => 'SSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE3 requires_alignment=1',
  tags       => 'page=HADDPS',
};


# HSUBPD-Packed Double-FP Horizontal Subtract.
ENCODING HSUBPD => {
  diagram    => 'MAP=0f MR=1 OP=0x7d P66=1 PF2=0 PF3=0',
  extensions => 'SSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE3 requires_alignment=1',
  tags       => 'page=HSUBPD',
};


# HSUBPS-Packed Single-FP Horizontal Subtract.
ENCODING HSUBPS => {
  diagram    => 'MAP=0f MR=1 OP=0x7d P66=0 PF2=1 PF3=0',
  extensions => 'SSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE3 requires_alignment=1',
  tags       => 'page=HSUBPS',
};


# INSERTPS-Insert Scalar Single-Precision Floating-Point Value.
ENCODING INSERTPS => {
  diagram    => 'MAP=0f3a MR=1 OP=0x21 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE4',
  tags       => 'page=INSERTPS',
};


# LDDQU-Load Unaligned Integer 128 Bits.
ENCODING LDDQU => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xf0 P66=0 PF2=1 PF3=0',
  extensions => 'SSE3',
  exceptions => 'SSE_TYPE_4',
  categories => 'SIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=SSE3',
  tags       => 'page=LDDQU',
};


# LDMXCSR-Load MXCSR Register.
ENCODING LDMXCSR => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xae P66=0 PF2=0 PF3=0 REG=2',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD',
  metadata   => 'isa=SSEMXCSR',
  tags       => 'page=LDMXCSR',
};


# LFENCE-Load Fence.
ENCODING LFENCE => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xae P66=0 PF2=0 PF3=0 REG=5',
  extensions => 'SSE2',
  categories => 'SIMD',
  metadata   => 'isa=SSE2',
  tags       => 'page=LFENCE',
};


# MASKMOVDQU-Store Selected Bytes of Double Quadword.
ENCODING MASKMOVDQU => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xf7 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_4',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2',
  tags       => 'page=MASKMOVDQU',
};


# MAXPD-Maximum of Packed Double-Precision Floating-Point Values.
ENCODING MAXPD => {
  diagram    => 'MAP=0f MR=1 OP=0x5f P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=MAXPD',
};


# MAXPS-Maximum of Packed Single-Precision Floating-Point Values.
ENCODING MAXPS => {
  diagram    => 'MAP=0f MR=1 OP=0x5f P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=MAXPS',
};


# MAXSD-Return Maximum Scalar Double-Precision Floating-Point Value.
ENCODING MAXSD => {
  diagram    => 'MAP=0f MR=1 OP=0x5f P66=0 PF2=1 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE2',
  tags       => 'page=MAXSD',
};


# MAXSS-Return Maximum Scalar Single-Precision Floating-Point Value.
ENCODING MAXSS => {
  diagram    => 'MAP=0f MR=1 OP=0x5f P66=0 PF2=0 PF3=1',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE',
  tags       => 'page=MAXSS',
};


# MFENCE-Memory Fence.
ENCODING MFENCE => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xae P66=0 PF2=0 PF3=0 REG=6',
  extensions => 'SSE2',
  categories => 'SIMD',
  metadata   => 'isa=SSE2',
  tags       => 'page=MFENCE',
};


# MINPD-Minimum of Packed Double-Precision Floating-Point Values.
ENCODING MINPD => {
  diagram    => 'MAP=0f MR=1 OP=0x5d P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=MINPD',
};


# MINPS-Minimum of Packed Single-Precision Floating-Point Values.
ENCODING MINPS => {
  diagram    => 'MAP=0f MR=1 OP=0x5d P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=MINPS',
};


# MINSD-Return Minimum Scalar Double-Precision Floating-Point Value.
ENCODING MINSD => {
  diagram    => 'MAP=0f MR=1 OP=0x5d P66=0 PF2=1 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE2',
  tags       => 'page=MINSD',
};


# MINSS-Return Minimum Scalar Single-Precision Floating-Point Value.
ENCODING MINSS => {
  diagram    => 'MAP=0f MR=1 OP=0x5d P66=0 PF2=0 PF3=1',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE',
  tags       => 'page=MINSS',
};


# MOVAPD-Move Aligned Packed Double-Precision Floating-Point Values.
ENCODING MOVAPD_moro => {
  diagram    => 'MAP=0f MR=1 OP=0x29 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_1',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=MOVAPD',
};

ENCODING MOVAPD_romo => {
  diagram    => 'MAP=0f MR=1 OP=0x28 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_1',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=MOVAPD',
};


# MOVAPS-Move Aligned Packed Single-Precision Floating-Point Values.
ENCODING MOVAPS_moro => {
  diagram    => 'MAP=0f MR=1 OP=0x29 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_1',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=MOVAPS',
};

ENCODING MOVAPS_romo => {
  diagram    => 'MAP=0f MR=1 OP=0x28 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_1',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=MOVAPS',
};


# MOVDDUP-Replicate Double FP Values.
ENCODING MOVDDUP => {
  diagram    => 'MAP=0f MR=1 OP=0x12 P66=0 PF2=1 PF3=0',
  extensions => 'SSE3',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|BROADCASTING|SWIZZLE',
  metadata   => 'isa=SSE3',
  tags       => 'page=MOVDDUP',
};


# MOVDQ2Q-Move Quadword from XMM to MMX Technology Register.
ENCODING MOVDQ2Q => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xd6 P66=0 PF2=1 PF3=0',
  extensions => 'SSE2',
  exceptions => 'MMX',
  categories => 'SIMD|MISC',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVDQ2Q',
};


# MOVDQA,VMOVDQA32/64-Move Aligned Packed Integer Values.
ENCODING MOVDQA_moro => {
  diagram    => 'MAP=0f MR=1 OP=0x7f P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_1',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=MOVDQA',
};

ENCODING MOVDQA_romo => {
  diagram    => 'MAP=0f MR=1 OP=0x6f P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_1',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=MOVDQA',
};


# MOVDQU,VMOVDQU8/16/32/64-Move Unaligned Packed Integer Values.
ENCODING MOVDQU_moro => {
  diagram    => 'MAP=0f MR=1 OP=0x7f P66=0 PF2=0 PF3=1',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_4M',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVDQU',
};

ENCODING MOVDQU_romo => {
  diagram    => 'MAP=0f MR=1 OP=0x6f P66=0 PF2=0 PF3=1',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_4M',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVDQU',
};


# MOVHLPS-Move Packed Single-Precision Floating-Point Values High to Low.
ENCODING MOVHLPS => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x12 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_7',
  categories => 'SIMD|DATA_TRANSFER',
  metadata   => 'isa=SSE',
  tags       => 'page=MOVHLPS',
};


# MOVHPD-Move High Packed Double-Precision Floating-Point Value.
ENCODING MOVHPD_mqrq => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x17 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVHPD',
};

ENCODING MOVHPD_rqmq => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x16 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVHPD',
};


# MOVHPS-Move High Packed Single-Precision Floating-Point Values.
ENCODING MOVHPS_mqro => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x17 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE',
  tags       => 'page=MOVHPS',
};

ENCODING MOVHPS_rqmq => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x16 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE',
  tags       => 'page=MOVHPS',
};


# MOVLHPS-Move Packed Single-Precision Floating-Point Values Low to High.
ENCODING MOVLHPS => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x16 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_7',
  categories => 'SIMD|DATA_TRANSFER',
  metadata   => 'isa=SSE',
  tags       => 'page=MOVLHPS',
};


# MOVLPD-Move Low Packed Double-Precision Floating-Point Value.
ENCODING MOVLPD_mqrq => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x13 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVLPD',
};

ENCODING MOVLPD_rqmq => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x12 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVLPD',
};


# MOVLPS-Move Low Packed Single-Precision Floating-Point Values.
ENCODING MOVLPS_mqro => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x13 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE',
  tags       => 'page=MOVLPS',
};

ENCODING MOVLPS_rqmq => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x12 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE',
  tags       => 'page=MOVLPS',
};


# MOVMSKPD-Extract Packed Double-Precision Floating-Point Sign Mask.
ENCODING MOVMSKPD => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x50 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_7',
  categories => 'SIMD|MISC',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVMSKPD',
};


# MOVMSKPS-Extract Packed Single-Precision Floating-Point Sign Mask.
ENCODING MOVMSKPS => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x50 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_7',
  categories => 'SIMD|MISC',
  metadata   => 'isa=SSE',
  tags       => 'page=MOVMSKPS',
};


# MOVNTDQ-Store Packed Integers Using Non-Temporal Hint.
ENCODING MOVNTDQ => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xe7 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_1',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=MOVNTDQ',
};


# MOVNTDQA-Load Double Quadword Non-Temporal Aligned Hint.
ENCODING MOVNTDQA => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0x2a P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_1',
  categories => 'SIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=MOVNTDQA',
};


# MOVNTI-Store Doubleword Using Non-Temporal Hint.
ENCODING MOVNTI => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xc3 P66=0 PF2=0 PF3=0',
  extensions => 'SSE2',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVNTI',
};


# MOVNTPD-Store Packed Double-Precision Floating-Point Values Using Non-Temporal Hint.
ENCODING MOVNTPD => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x2b P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_1',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=MOVNTPD',
};


# MOVNTPS-Store Packed Single-Precision Floating-Point Values Using Non-Temporal Hint.
ENCODING MOVNTPS => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x2b P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_1',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=MOVNTPS',
};


# MOVQ-Move Quadword.
ENCODING MOVQ_mqrq_2 => {
  diagram    => 'MAP=0f MODE=64 MR=1 OP=0x7e P66=1 PF2=0 PF3=0 W=1',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVQ',
};

ENCODING MOVQ_mxrq => {
  diagram    => 'MAP=0f MR=1 OP=0xd6 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVQ',
};

ENCODING MOVQ_romq_1 => {
  diagram    => 'MAP=0f MODE=64 MR=1 OP=0x6e P66=1 PF2=0 PF3=0 W=1',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVQ',
};

ENCODING MOVQ_romq_2 => {
  diagram    => 'MAP=0f MR=1 OP=0x7e P66=0 PF2=0 PF3=1',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVQ',
};


# MOVQ2DQ-Move Quadword from MMX Technology to XMM Register.
ENCODING MOVQ2DQ => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xd6 P66=0 PF2=0 PF3=1',
  extensions => 'SSE2',
  exceptions => 'MMX',
  categories => 'SIMD|DATA_TRANSFER',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVQ2DQ',
};


# MOVS/MOVSB/MOVSW/MOVSD/MOVSQ-Move Data from String to String.
ENCODING MOVSD_mqrq => {
  diagram    => 'MAP=0f MR=1 OP=0x11 P66=0 PF2=1 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVS',
};

ENCODING MOVSD_rxmq => {
  diagram    => 'MAP=0f MR=1 OP=0x10 P66=0 PF2=1 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVS',
};


# MOVSHDUP-Replicate Single FP Values.
ENCODING MOVSHDUP => {
  diagram    => 'MAP=0f MR=1 OP=0x16 P66=0 PF2=0 PF3=1',
  extensions => 'SSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|DATA_TRANSFER',
  metadata   => 'isa=SSE3 requires_alignment=1',
  tags       => 'page=MOVSHDUP',
};


# MOVSLDUP-Replicate Single FP Values.
ENCODING MOVSLDUP => {
  diagram    => 'MAP=0f MR=1 OP=0x12 P66=0 PF2=0 PF3=1',
  extensions => 'SSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|DATA_TRANSFER',
  metadata   => 'isa=SSE3 requires_alignment=1',
  tags       => 'page=MOVSLDUP',
};


# MOVSS-Move or Merge Scalar Single-Precision Floating-Point Value.
ENCODING MOVSS_mdrd => {
  diagram    => 'MAP=0f MR=1 OP=0x11 P66=0 PF2=0 PF3=1',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE',
  tags       => 'page=MOVSS',
};

ENCODING MOVSS_rxmd => {
  diagram    => 'MAP=0f MR=1 OP=0x10 P66=0 PF2=0 PF3=1',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE',
  tags       => 'page=MOVSS',
};


# MOVUPD-Move Unaligned Packed Double-Precision Floating-Point Values.
ENCODING MOVUPD_moro => {
  diagram    => 'MAP=0f MR=1 OP=0x11 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_4M',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVUPD',
};

ENCODING MOVUPD_romo => {
  diagram    => 'MAP=0f MR=1 OP=0x10 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_4M',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVUPD',
};


# MOVUPS-Move Unaligned Packed Single-Precision Floating-Point Values.
ENCODING MOVUPS_moro => {
  diagram    => 'MAP=0f MR=1 OP=0x11 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_4M',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE',
  tags       => 'page=MOVUPS',
};

ENCODING MOVUPS_romo => {
  diagram    => 'MAP=0f MR=1 OP=0x10 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_4M',
  categories => 'SIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=SSE',
  tags       => 'page=MOVUPS',
};


# MOVD/MOVQ-Move Doubleword/Move Quadword.
ENCODING MOVD_mdrd_2 => {
  diagram    => 'MAP=0f MR=1 OP=0x7e P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVx',
};

ENCODING MOVD_romd => {
  diagram    => 'MAP=0f MR=1 OP=0x6e P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE2',
  tags       => 'page=MOVx',
};


# MPSADBW-Compute Multiple Packed Sums of Absolute Difference.
ENCODING MPSADBW => {
  diagram    => 'MAP=0f3a MR=1 OP=0x42 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC|MISC',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=MPSADBW',
};


# MULPD-Multiply Packed Double-Precision Floating-Point Values.
ENCODING MULPD => {
  diagram    => 'MAP=0f MR=1 OP=0x59 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=MULPD',
};


# MULPS-Multiply Packed Single-Precision Floating-Point Values.
ENCODING MULPS => {
  diagram    => 'MAP=0f MR=1 OP=0x59 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=MULPS',
};


# MULSD-Multiply Scalar Double-Precision Floating-Point Value.
ENCODING MULSD => {
  diagram    => 'MAP=0f MR=1 OP=0x59 P66=0 PF2=1 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2',
  tags       => 'page=MULSD',
};


# MULSS-Multiply Scalar Single-Precision Floating-Point Values.
ENCODING MULSS => {
  diagram    => 'MAP=0f MR=1 OP=0x59 P66=0 PF2=0 PF3=1',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE',
  tags       => 'page=MULSS',
};


# ORPD-Bitwise Logical OR of Packed Double Precision Floating-Point Values.
ENCODING ORPD => {
  diagram    => 'MAP=0f MR=1 OP=0x56 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=ORPD',
};


# ORPS-Bitwise Logical OR of Packed Single Precision Floating-Point Values.
ENCODING ORPS => {
  diagram    => 'MAP=0f MR=1 OP=0x56 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=ORPS',
};


# PABSB/PABSW/PABSD/PABSQ-Packed Absolute Value.
ENCODING PABSB_romo => {
  diagram    => 'MAP=0f38 MR=1 OP=0x1c P66=1 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MATH',
  metadata   => 'isa=SSSE3 requires_alignment=1',
  tags       => 'page=PABSx',
};

ENCODING PABSB_rqmq => {
  diagram    => 'MAP=0f38 MR=1 OP=0x1c P66=0 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|MATH',
  metadata   => 'isa=SSSE3MMX',
  tags       => 'page=PABSx',
};

ENCODING PABSD_romo_1 => {
  diagram    => 'MAP=0f38 MOD=REG MR=1 OP=0x1e P66=1 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'SSE_TYPE_4',
  categories => 'SIMD|MATH',
  metadata   => 'isa=SSSE3',
  tags       => 'page=PABSx',
};

ENCODING PABSD_romo_2 => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0x1e P66=1 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MATH',
  metadata   => 'isa=SSSE3 requires_alignment=1',
  tags       => 'page=PABSx',
};

ENCODING PABSD_rqmq_1 => {
  diagram    => 'MAP=0f38 MOD=REG MR=1 OP=0x1e P66=0 PF2=0 PF3=0',
  extensions => 'SSSE3',
  categories => 'SIMD|MATH',
  metadata   => 'isa=SSSE3MMX',
  tags       => 'page=PABSx',
};

ENCODING PABSD_rqmq_2 => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0x1e P66=0 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|MATH',
  metadata   => 'isa=SSSE3MMX',
  tags       => 'page=PABSx',
};

ENCODING PABSW_romo => {
  diagram    => 'MAP=0f38 MR=1 OP=0x1d P66=1 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MATH',
  metadata   => 'isa=SSSE3 requires_alignment=1',
  tags       => 'page=PABSx',
};

ENCODING PABSW_rqmq => {
  diagram    => 'MAP=0f38 MR=1 OP=0x1d P66=0 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|MATH',
  metadata   => 'isa=SSSE3MMX',
  tags       => 'page=PABSx',
};


# PACKSSWB/PACKSSDW-Pack with Signed Saturation.
ENCODING PACKSSDW_romo => {
  diagram    => 'MAP=0f MR=1 OP=0x6b P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MISC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PACKSSxx',
};

ENCODING PACKSSWB_romo => {
  diagram    => 'MAP=0f MR=1 OP=0x63 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MISC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PACKSSxx',
};


# PACKUSDW-Pack with Unsigned Saturation.
ENCODING PACKUSDW => {
  diagram    => 'MAP=0f38 MR=1 OP=0x2b P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=PACKUSDW',
};


# PACKUSWB-Pack with Unsigned Saturation.
ENCODING PACKUSWB_romo => {
  diagram    => 'MAP=0f MR=1 OP=0x67 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MISC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PACKUSWB',
};


# PADDSB/PADDSW-Add Packed Signed Integers with Signed Saturation.
ENCODING PADDSB_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xec P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PADDSx',
};

ENCODING PADDSW_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xed P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PADDSx',
};


# PADDUSB/PADDUSW-Add Packed Unsigned Integers with Unsigned Saturation.
ENCODING PADDUSB_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xdc P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PADDUSx',
};

ENCODING PADDUSW_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xdd P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PADDUSx',
};


# PADDB/PADDW/PADDD/PADDQ-Add Packed Integers.
ENCODING PADDB_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xfc P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PADDx',
};

ENCODING PADDD_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xfe P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PADDx',
};

ENCODING PADDQ_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xd4 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PADDx',
};

ENCODING PADDQ_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xd4 P66=0 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2MMX',
  tags       => 'page=PADDx',
};

ENCODING PADDW_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xfd P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PADDx',
};


# PALIGNR-Packed Align Right.
ENCODING PALIGNR_romoub => {
  diagram    => 'MAP=0f3a MR=1 OP=0x0f P66=1 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MISC',
  metadata   => 'isa=SSSE3 requires_alignment=1',
  tags       => 'page=PALIGNR',
};

ENCODING PALIGNR_rqmqub => {
  diagram    => 'MAP=0f3a MR=1 OP=0x0f P66=0 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|MISC',
  metadata   => 'isa=SSSE3MMX',
  tags       => 'page=PALIGNR',
};


# PAND-Logical AND.
ENCODING PAND_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xdb P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|BITWISE|LOGICAL',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PAND',
};


# PANDN-Logical AND NOT.
ENCODING PANDN_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xdf P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|BITWISE|LOGICAL',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PANDN',
};


# PAVGB/PAVGW-Average Packed Integers.
ENCODING PAVGB_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xe0 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|STAT',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PAVGx',
};

ENCODING PAVGW_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xe3 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|STAT',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PAVGx',
};


# PBLENDVB-Variable Blend Packed Bytes.
ENCODING PBLENDVB => {
  diagram    => 'MAP=0f38 MR=1 OP=0x10 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=PBLENDVB',
};


# PBLENDW-Blend Packed Words.
ENCODING PBLENDW => {
  diagram    => 'MAP=0f3a MR=1 OP=0x0e P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=PBLENDW',
};


# PCMPEQQ-Compare Packed Qword Data for Equal.
ENCODING PCMPEQQ => {
  diagram    => 'MAP=0f38 MR=1 OP=0x29 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|COMPARISON',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=PCMPEQQ',
};


# PCMPEQB/PCMPEQW/PCMPEQD-Compare Packed Data for Equal.
ENCODING PCMPEQB_romo => {
  diagram    => 'MAP=0f MR=1 OP=0x74 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|COMPARISON',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PCMPEQx',
};

ENCODING PCMPEQD_romo => {
  diagram    => 'MAP=0f MR=1 OP=0x76 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|BITWISE|LOGICAL',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PCMPEQx',
};

ENCODING PCMPEQW_romo => {
  diagram    => 'MAP=0f MR=1 OP=0x75 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|COMPARISON',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PCMPEQx',
};


# PCMPESTRI-Packed Compare Explicit Length Strings, Return Index.
ENCODING PCMPESTRI => {
  diagram    => 'MAP=0f3a MR=1 OP=0x61 P66=1 PF2=0 PF3=0',
  iflags     => 'af=Z cf=W of=W pf=Z sf=W zf=W',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_4',
  categories => 'SIMD|STRING|COMPARISON',
  metadata   => 'isa=SSE42',
  tags       => 'page=PCMPESTRI',
};


# PCMPESTRM-Packed Compare Explicit Length Strings, Return Mask.
ENCODING PCMPESTRM => {
  diagram    => 'MAP=0f3a MR=1 OP=0x60 P66=1 PF2=0 PF3=0',
  iflags     => 'af=Z cf=W of=W pf=Z sf=W zf=W',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_4',
  categories => 'SIMD|STRING|COMPARISON',
  metadata   => 'isa=SSE42',
  tags       => 'page=PCMPESTRM',
};


# PCMPGTQ-Compare Packed Data for Greater Than.
ENCODING PCMPGTQ => {
  diagram    => 'MAP=0f38 MR=1 OP=0x37 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|COMPARISON',
  metadata   => 'isa=SSE42 requires_alignment=1',
  tags       => 'page=PCMPGTQ',
};


# PCMPGTB/PCMPGTW/PCMPGTD-Compare Packed Signed Integers for Greater Than.
ENCODING PCMPGTB_romo => {
  diagram    => 'MAP=0f MR=1 OP=0x64 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|COMPARISON',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PCMPGTx',
};

ENCODING PCMPGTD_romo => {
  diagram    => 'MAP=0f MR=1 OP=0x66 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|COMPARISON',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PCMPGTx',
};

ENCODING PCMPGTW_romo => {
  diagram    => 'MAP=0f MR=1 OP=0x65 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|COMPARISON',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PCMPGTx',
};


# PCMPISTRI-Packed Compare Implicit Length Strings, Return Index.
ENCODING PCMPISTRI => {
  diagram    => 'MAP=0f3a MR=1 OP=0x63 P66=1 PF2=0 PF3=0',
  iflags     => 'af=Z cf=W of=W pf=Z sf=W zf=W',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_4',
  categories => 'SIMD|STRING|COMPARISON',
  metadata   => 'isa=SSE42',
  tags       => 'page=PCMPISTRI',
};


# PCMPISTRM-Packed Compare Implicit Length Strings, Return Mask.
ENCODING PCMPISTRM => {
  diagram    => 'MAP=0f3a MR=1 OP=0x62 P66=1 PF2=0 PF3=0',
  iflags     => 'af=Z cf=W of=W pf=Z sf=W zf=W',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_4',
  categories => 'SIMD|STRING|COMPARISON',
  metadata   => 'isa=SSE42',
  tags       => 'page=PCMPISTRM',
};


# PEXTRW-Extract Word.
ENCODING PEXTRW_mxroub => {
  diagram    => 'MAP=0f3a MR=1 OP=0x15 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE4',
  tags       => 'page=PEXTRW',
};

ENCODING PEXTRW_rdmoub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xc5 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE2',
  tags       => 'page=PEXTRW',
};


# PEXTRB/PEXTRD/PEXTRQ-Extract Byte/Dword/Qword.
ENCODING PEXTRB => {
  diagram    => 'MAP=0f3a MR=1 OP=0x14 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE4',
  tags       => 'page=PEXTRx',
};

ENCODING PEXTRx => {
  diagram    => 'MAP=0f3a MR=1 OP=0x16 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE4',
  tags       => 'page=PEXTRx',
};


# PHADDSW-Packed Horizontal Add and Saturate.
ENCODING PHADDSW_romo => {
  diagram    => 'MAP=0f38 MR=1 OP=0x03 P66=1 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3 requires_alignment=1',
  tags       => 'page=PHADDSW',
};

ENCODING PHADDSW_rqmq => {
  diagram    => 'MAP=0f38 MR=1 OP=0x03 P66=0 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3MMX',
  tags       => 'page=PHADDSW',
};


# PHADDW/PHADDD-Packed Horizontal Add.
ENCODING PHADDD_romo => {
  diagram    => 'MAP=0f38 MR=1 OP=0x02 P66=1 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3 requires_alignment=1',
  tags       => 'page=PHADDx',
};

ENCODING PHADDD_rqmq => {
  diagram    => 'MAP=0f38 MR=1 OP=0x02 P66=0 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3MMX',
  tags       => 'page=PHADDx',
};

ENCODING PHADDW_romo => {
  diagram    => 'MAP=0f38 MR=1 OP=0x01 P66=1 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3 requires_alignment=1',
  tags       => 'page=PHADDx',
};

ENCODING PHADDW_rqmq => {
  diagram    => 'MAP=0f38 MR=1 OP=0x01 P66=0 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3MMX',
  tags       => 'page=PHADDx',
};


# PHMINPOSUW-Packed Horizontal Word Minimum.
ENCODING PHMINPOSUW => {
  diagram    => 'MAP=0f38 MR=1 OP=0x41 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MISC',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=PHMINPOSUW',
};


# PHSUBSW-Packed Horizontal Subtract and Saturate.
ENCODING PHSUBSW_romo => {
  diagram    => 'MAP=0f38 MR=1 OP=0x07 P66=1 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3 requires_alignment=1',
  tags       => 'page=PHSUBSW',
};

ENCODING PHSUBSW_rqmq => {
  diagram    => 'MAP=0f38 MR=1 OP=0x07 P66=0 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3MMX',
  tags       => 'page=PHSUBSW',
};


# PHSUBW/PHSUBD-Packed Horizontal Subtract.
ENCODING PHSUBD_romo => {
  diagram    => 'MAP=0f38 MR=1 OP=0x06 P66=1 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3 requires_alignment=1',
  tags       => 'page=PHSUBx',
};

ENCODING PHSUBD_rqmq => {
  diagram    => 'MAP=0f38 MR=1 OP=0x06 P66=0 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3MMX',
  tags       => 'page=PHSUBx',
};

ENCODING PHSUBW_romo => {
  diagram    => 'MAP=0f38 MR=1 OP=0x05 P66=1 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3 requires_alignment=1',
  tags       => 'page=PHSUBx',
};

ENCODING PHSUBW_rqmq => {
  diagram    => 'MAP=0f38 MR=1 OP=0x05 P66=0 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3MMX',
  tags       => 'page=PHSUBx',
};


# PINSRW-Insert Word.
ENCODING PINSRW_romxub => {
  diagram    => 'MAP=0f MR=1 OP=0xc4 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE2',
  tags       => 'page=PINSRW',
};


# PINSRB/PINSRD/PINSRQ-Insert Byte/Dword/Qword.
ENCODING PINSRB => {
  diagram    => 'MAP=0f3a MR=1 OP=0x20 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE4',
  tags       => 'page=PINSRx',
};

ENCODING PINSRx => {
  diagram    => 'MAP=0f3a MR=1 OP=0x22 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE4',
  tags       => 'page=PINSRx',
};


# PMADDUBSW-Multiply and Add Packed Signed and Unsigned Bytes.
ENCODING PMADDUBSW_romo => {
  diagram    => 'MAP=0f38 MR=1 OP=0x04 P66=1 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|CARITHMETIC',
  metadata   => 'isa=SSSE3 requires_alignment=1',
  tags       => 'page=PMADDUBSW',
};

ENCODING PMADDUBSW_rqmq => {
  diagram    => 'MAP=0f38 MR=1 OP=0x04 P66=0 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|CARITHMETIC',
  metadata   => 'isa=SSSE3MMX',
  tags       => 'page=PMADDUBSW',
};


# PMADDWD-Multiply and Add Packed Integers.
ENCODING PMADDWD_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xf5 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|CARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PMADDWD',
};


# PMAXSB/PMAXSW/PMAXSD/PMAXSQ-Maximum of Packed Signed Integers.
ENCODING PMAXSB => {
  diagram    => 'MAP=0f38 MR=1 OP=0x3c P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=PMAXSx',
};

ENCODING PMAXSD => {
  diagram    => 'MAP=0f38 MR=1 OP=0x3d P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=PMAXSx',
};

ENCODING PMAXSW_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xee P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PMAXSx',
};


# PMAXUB/PMAXUW-Maximum of Packed Unsigned Integers.
ENCODING PMAXUB_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xde P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PMAXUbw',
};

ENCODING PMAXUW => {
  diagram    => 'MAP=0f38 MR=1 OP=0x3e P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=PMAXUbw',
};


# PMAXUD/PMAXUQ-Maximum of Packed Unsigned Integers.
ENCODING PMAXUD => {
  diagram    => 'MAP=0f38 MR=1 OP=0x3f P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=PMAXUy',
};


# PMINSB/PMINSW-Minimum of Packed Signed Integers.
ENCODING PMINSB => {
  diagram    => 'MAP=0f38 MR=1 OP=0x38 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=PMINSbw',
};

ENCODING PMINSW_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xea P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PMINSbw',
};


# PMINSD/PMINSQ-Minimum of Packed Signed Integers.
ENCODING PMINSD => {
  diagram    => 'MAP=0f38 MR=1 OP=0x39 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=PMINSy',
};


# PMINUB/PMINUW-Minimum of Packed Unsigned Integers.
ENCODING PMINUB_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xda P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PMINUbw',
};

ENCODING PMINUW => {
  diagram    => 'MAP=0f38 MR=1 OP=0x3a P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=PMINUbw',
};


# PMINUD/PMINUQ-Minimum of Packed Unsigned Integers.
ENCODING PMINUD => {
  diagram    => 'MAP=0f38 MR=1 OP=0x3b P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|MATH|MIN_MAX',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=PMINUy',
};


# PMOVMSKB-Move Byte Mask.
ENCODING PMOVMSKB_rdmo => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xd7 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_7',
  categories => 'SIMD|MISC',
  metadata   => 'isa=SSE2',
  tags       => 'page=PMOVMSKB',
};


# PMOVSX-Packed Move with Sign Extend.
ENCODING PMOVSXBD => {
  diagram    => 'MAP=0f38 MR=1 OP=0x21 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE4',
  tags       => 'page=PMOVSX',
};

ENCODING PMOVSXBQ => {
  diagram    => 'MAP=0f38 MR=1 OP=0x22 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE4',
  tags       => 'page=PMOVSX',
};

ENCODING PMOVSXBW => {
  diagram    => 'MAP=0f38 MR=1 OP=0x20 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE4',
  tags       => 'page=PMOVSX',
};

ENCODING PMOVSXDQ => {
  diagram    => 'MAP=0f38 MR=1 OP=0x25 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE4',
  tags       => 'page=PMOVSX',
};

ENCODING PMOVSXWD => {
  diagram    => 'MAP=0f38 MR=1 OP=0x23 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE4',
  tags       => 'page=PMOVSX',
};

ENCODING PMOVSXWQ => {
  diagram    => 'MAP=0f38 MR=1 OP=0x24 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE4',
  tags       => 'page=PMOVSX',
};


# PMOVZX-Packed Move with Zero Extend.
ENCODING PMOVZXBD => {
  diagram    => 'MAP=0f38 MR=1 OP=0x31 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE4',
  tags       => 'page=PMOVZX',
};

ENCODING PMOVZXBQ => {
  diagram    => 'MAP=0f38 MR=1 OP=0x32 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE4',
  tags       => 'page=PMOVZX',
};

ENCODING PMOVZXBW => {
  diagram    => 'MAP=0f38 MR=1 OP=0x30 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE4',
  tags       => 'page=PMOVZX',
};

ENCODING PMOVZXDQ => {
  diagram    => 'MAP=0f38 MR=1 OP=0x35 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE4',
  tags       => 'page=PMOVZX',
};

ENCODING PMOVZXWD => {
  diagram    => 'MAP=0f38 MR=1 OP=0x33 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE4',
  tags       => 'page=PMOVZX',
};

ENCODING PMOVZXWQ => {
  diagram    => 'MAP=0f38 MR=1 OP=0x34 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=SSE4',
  tags       => 'page=PMOVZX',
};


# PMULDQ-Multiply Packed Doubleword Integers.
ENCODING PMULDQ => {
  diagram    => 'MAP=0f38 MR=1 OP=0x28 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=PMULDQ',
};


# PMULHRSW-Packed Multiply High with Round and Scale.
ENCODING PMULHRSW_romo => {
  diagram    => 'MAP=0f38 MR=1 OP=0x0b P66=1 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3 requires_alignment=1',
  tags       => 'page=PMULHRSW',
};

ENCODING PMULHRSW_rqmq => {
  diagram    => 'MAP=0f38 MR=1 OP=0x0b P66=0 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3MMX',
  tags       => 'page=PMULHRSW',
};


# PMULHUW-Multiply Packed Unsigned Integers and Store High Result.
ENCODING PMULHUW_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xe4 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PMULHUW',
};


# PMULHW-Multiply Packed Signed Integers and Store High Result.
ENCODING PMULHW_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xe5 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PMULHW',
};


# PMULLW-Multiply Packed Signed Integers and Store Low Result.
ENCODING PMULLW_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xd5 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PMULLW',
};


# PMULLD/PMULLQ-Multiply Packed Integers and Store Low Result.
ENCODING PMULLD => {
  diagram    => 'MAP=0f38 MR=1 OP=0x40 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=PMULLx',
};


# PMULUDQ-Multiply Packed Unsigned Doubleword Integers.
ENCODING PMULUDQ_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xf4 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PMULUDQ',
};

ENCODING PMULUDQ_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xf4 P66=0 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2MMX',
  tags       => 'page=PMULUDQ',
};


# POPCNT-Return the Count of Number of Bits Set to 1.
ENCODING POPCNT_rvmv => {
  diagram    => 'MAP=0f MR=1 OP=0xb8 P66=0 PF2=0 PF3=1',
  iflags     => 'af=Z cf=Z of=Z pf=Z sf=Z zf=W',
  extensions => 'SSE4',
  categories => 'SIMD|BITFIELD',
  metadata   => 'isa=POPCNT',
  tags       => 'page=POPCNT',
};


# POR-Bitwise Logical OR.
ENCODING POR_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xeb P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|BITWISE|LOGICAL',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=POR',
};


# PREFETCHh-Prefetch Data Into Caches.
ENCODING PREFETCHNTA => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x18 REG=0',
  extensions => 'SSE',
  categories => 'SIMD',
  metadata   => 'isa=SSE_PREFETCH',
  tags       => 'page=PREFETCHh',
};

ENCODING PREFETCHT0 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x18 REG=1',
  extensions => 'SSE',
  categories => 'SIMD',
  metadata   => 'isa=SSE_PREFETCH',
  tags       => 'page=PREFETCHh',
};

ENCODING PREFETCHT1 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x18 REG=2',
  extensions => 'SSE',
  categories => 'SIMD',
  metadata   => 'isa=SSE_PREFETCH',
  tags       => 'page=PREFETCHh',
};

ENCODING PREFETCHT2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x18 REG=3',
  extensions => 'SSE',
  categories => 'SIMD',
  metadata   => 'isa=SSE_PREFETCH',
  tags       => 'page=PREFETCHh',
};


# PSADBW-Compute Sum of Absolute Differences.
ENCODING PSADBW_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xf6 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC|MISC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSADBW',
};


# PSHUFB-Packed Shuffle Bytes.
ENCODING PSHUFB_romo => {
  diagram    => 'MAP=0f38 MR=1 OP=0x00 P66=1 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSSE3 requires_alignment=1',
  tags       => 'page=PSHUFB',
};

ENCODING PSHUFB_rqmq => {
  diagram    => 'MAP=0f38 MR=1 OP=0x00 P66=0 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSSE3MMX',
  tags       => 'page=PSHUFB',
};


# PSHUFD-Shuffle Packed Doublewords.
ENCODING PSHUFD => {
  diagram    => 'MAP=0f MR=1 OP=0x70 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSHUFD',
};


# PSHUFHW-Shuffle Packed High Words.
ENCODING PSHUFHW => {
  diagram    => 'MAP=0f MR=1 OP=0x70 P66=0 PF2=0 PF3=1',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSHUFHW',
};


# PSHUFLW-Shuffle Packed Low Words.
ENCODING PSHUFLW => {
  diagram    => 'MAP=0f MR=1 OP=0x70 P66=0 PF2=1 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSHUFLW',
};


# PSIGNB/PSIGNW/PSIGND-Packed SIGN.
ENCODING PSIGNB_romo => {
  diagram    => 'MAP=0f38 MR=1 OP=0x08 P66=1 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3 requires_alignment=1',
  tags       => 'page=PSIGNx',
};

ENCODING PSIGNB_rqmq => {
  diagram    => 'MAP=0f38 MR=1 OP=0x08 P66=0 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3MMX',
  tags       => 'page=PSIGNx',
};

ENCODING PSIGND_romo => {
  diagram    => 'MAP=0f38 MR=1 OP=0x0a P66=1 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3 requires_alignment=1',
  tags       => 'page=PSIGNx',
};

ENCODING PSIGND_rqmq => {
  diagram    => 'MAP=0f38 MR=1 OP=0x0a P66=0 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3MMX',
  tags       => 'page=PSIGNx',
};

ENCODING PSIGNW_romo => {
  diagram    => 'MAP=0f38 MR=1 OP=0x09 P66=1 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3 requires_alignment=1',
  tags       => 'page=PSIGNx',
};

ENCODING PSIGNW_rqmq => {
  diagram    => 'MAP=0f38 MR=1 OP=0x09 P66=0 PF2=0 PF3=0',
  extensions => 'SSSE3',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSSE3MMX',
  tags       => 'page=PSIGNx',
};


# PSLLDQ-Shift Double Quadword Left Logical.
ENCODING PSLLDQ => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=7',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_7',
  categories => 'SIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=SSE2',
  tags       => 'page=PSLLDQ',
};


# PSLLW/PSLLD/PSLLQ-Shift Packed Data Left Logical.
ENCODING PSLLD_moub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=6',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_7',
  categories => 'SIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=SSE2',
  tags       => 'page=PSLLx',
};

ENCODING PSLLD_romo_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xf2 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_7',
  categories => 'SIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSLLx',
};

ENCODING PSLLD_romo_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xf2 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSLLx',
};

ENCODING PSLLQ_moub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=6',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_7',
  categories => 'SIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=SSE2',
  tags       => 'page=PSLLx',
};

ENCODING PSLLQ_romo_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xf3 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_7',
  categories => 'SIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSLLx',
};

ENCODING PSLLQ_romo_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xf3 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSLLx',
};

ENCODING PSLLW_moub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=6',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_7',
  categories => 'SIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=SSE2',
  tags       => 'page=PSLLx',
};

ENCODING PSLLW_romo_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xf1 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_7',
  categories => 'SIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSLLx',
};

ENCODING PSLLW_romo_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xf1 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSLLx',
};


# PSRAW/PSRAD/PSRAQ-Shift Packed Data Right Arithmetic.
ENCODING PSRAD_moub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=4',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_7',
  categories => 'SIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=SSE2',
  tags       => 'page=PSRAx',
};

ENCODING PSRAD_romo_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xe2 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_7',
  categories => 'SIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSRAx',
};

ENCODING PSRAD_romo_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xe2 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSRAx',
};

ENCODING PSRAW_moub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=4',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_7',
  categories => 'SIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=SSE2',
  tags       => 'page=PSRAx',
};

ENCODING PSRAW_romo_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xe1 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_7',
  categories => 'SIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSRAx',
};

ENCODING PSRAW_romo_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xe1 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSRAx',
};


# PSRLDQ-Shift Double Quadword Right Logical.
ENCODING PSRLDQ => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=3',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_7',
  categories => 'SIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=SSE2',
  tags       => 'page=PSRLDQ',
};


# PSRLW/PSRLD/PSRLQ-Shift Packed Data Right Logical.
ENCODING PSRLD_moub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=2',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_7',
  categories => 'SIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=SSE2',
  tags       => 'page=PSRLx',
};

ENCODING PSRLD_romo_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xd2 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_7',
  categories => 'SIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSRLx',
};

ENCODING PSRLD_romo_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xd2 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSRLx',
};

ENCODING PSRLQ_moub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=2',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_7',
  categories => 'SIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=SSE2',
  tags       => 'page=PSRLx',
};

ENCODING PSRLQ_romo_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xd3 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_7',
  categories => 'SIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSRLx',
};

ENCODING PSRLQ_romo_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xd3 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSRLx',
};

ENCODING PSRLW_moub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=2',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_7',
  categories => 'SIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=SSE2',
  tags       => 'page=PSRLx',
};

ENCODING PSRLW_romo_1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xd1 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_7',
  categories => 'SIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSRLx',
};

ENCODING PSRLW_romo_2 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xd1 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSRLx',
};


# PSUBQ-Subtract Packed Quadword Integers.
ENCODING PSUBQ_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xfb P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSUBQ',
};

ENCODING PSUBQ_rqmq => {
  diagram    => 'MAP=0f MR=1 OP=0xfb P66=0 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'MMX_MEM',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2MMX',
  tags       => 'page=PSUBQ',
};


# PSUBSB/PSUBSW-Subtract Packed Signed Integers with Signed Saturation.
ENCODING PSUBSB_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xe8 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSUBSx',
};

ENCODING PSUBSW_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xe9 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSUBSx',
};


# PSUBUSB/PSUBUSW-Subtract Packed Unsigned Integers with Unsigned Saturation.
ENCODING PSUBUSB_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xd8 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSUBUSx',
};

ENCODING PSUBUSW_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xd9 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSUBUSx',
};


# PSUBB/PSUBW/PSUBD-Subtract Packed Integers.
ENCODING PSUBB_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xf8 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSUBx',
};

ENCODING PSUBD_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xfa P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSUBx',
};

ENCODING PSUBW_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xf9 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PSUBx',
};


# PTEST-Logical Compare.
ENCODING PTEST => {
  diagram    => 'MAP=0f38 MR=1 OP=0x17 P66=1 PF2=0 PF3=0',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|BITWISE|LOGICAL',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=PTEST',
};


# PUNPCKHBW/PUNPCKHWD/PUNPCKHDQ/PUNPCKHQDQ-Unpack High Data.
ENCODING PUNPCKHBW_romx => {
  diagram    => 'MAP=0f MR=1 OP=0x68 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PUNPCKHxx',
};

ENCODING PUNPCKHDQ_romx => {
  diagram    => 'MAP=0f MR=1 OP=0x6a P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PUNPCKHxx',
};

ENCODING PUNPCKHQDQ => {
  diagram    => 'MAP=0f MR=1 OP=0x6d P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PUNPCKHxx',
};

ENCODING PUNPCKHWD_romx => {
  diagram    => 'MAP=0f MR=1 OP=0x69 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PUNPCKHxx',
};


# PUNPCKLBW/PUNPCKLWD/PUNPCKLDQ/PUNPCKLQDQ-Unpack Low Data.
ENCODING PUNPCKLBW_romx => {
  diagram    => 'MAP=0f MR=1 OP=0x60 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PUNPCKLxx',
};

ENCODING PUNPCKLDQ_romx => {
  diagram    => 'MAP=0f MR=1 OP=0x62 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PUNPCKLxx',
};

ENCODING PUNPCKLQDQ => {
  diagram    => 'MAP=0f MR=1 OP=0x6c P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PUNPCKLxx',
};

ENCODING PUNPCKLWD_romx => {
  diagram    => 'MAP=0f MR=1 OP=0x61 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PUNPCKLxx',
};


# PXOR-Logical Exclusive OR.
ENCODING PXOR_romo => {
  diagram    => 'MAP=0f MR=1 OP=0xef P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|BITWISE|LOGICAL',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=PXOR',
};


# RCPPS-Compute Reciprocals of Packed Single-Precision Floating-Point Values.
ENCODING RCPPS => {
  diagram    => 'MAP=0f MR=1 OP=0x53 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|EMATH',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=RCPPS',
};


# RCPSS-Compute Reciprocal of Scalar Single-Precision Floating-Point Values.
ENCODING RCPSS => {
  diagram    => 'MAP=0f MR=1 OP=0x53 P66=0 PF2=0 PF3=1',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|EMATH',
  metadata   => 'isa=SSE',
  tags       => 'page=RCPSS',
};


# ROUNDPD-Round Packed Double Precision Floating-Point Values.
ENCODING ROUNDPD => {
  diagram    => 'MAP=0f3a MR=1 OP=0x09 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|ROUNDING',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=ROUNDPD',
};


# ROUNDPS-Round Packed Single Precision Floating-Point Values.
ENCODING ROUNDPS => {
  diagram    => 'MAP=0f3a MR=1 OP=0x08 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|ROUNDING',
  metadata   => 'isa=SSE4 requires_alignment=1',
  tags       => 'page=ROUNDPS',
};


# ROUNDSD-Round Scalar Double Precision Floating-Point Values.
ENCODING ROUNDSD => {
  diagram    => 'MAP=0f3a MR=1 OP=0x0b P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|ROUNDING',
  metadata   => 'isa=SSE4',
  tags       => 'page=ROUNDSD',
};


# ROUNDSS-Round Scalar Single Precision Floating-Point Values.
ENCODING ROUNDSS => {
  diagram    => 'MAP=0f3a MR=1 OP=0x0a P66=1 PF2=0 PF3=0',
  extensions => 'SSE4',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|ROUNDING',
  metadata   => 'isa=SSE4',
  tags       => 'page=ROUNDSS',
};


# RSQRTPS-Compute Reciprocals of Square Roots of Packed Single-Precision Floating-Point Values.
ENCODING RSQRTPS => {
  diagram    => 'MAP=0f MR=1 OP=0x52 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|EMATH',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=RSQRTPS',
};


# RSQRTSS-Compute Reciprocal of Square Root of Scalar Single-Precision Floating-Point Value.
ENCODING RSQRTSS => {
  diagram    => 'MAP=0f MR=1 OP=0x52 P66=0 PF2=0 PF3=1',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD|EMATH',
  metadata   => 'isa=SSE',
  tags       => 'page=RSQRTSS',
};


# SFENCE-Store Fence.
ENCODING SFENCE => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xae P66=0 PF2=0 PF3=0 REG=7',
  extensions => 'SSE',
  categories => 'SIMD',
  metadata   => 'isa=SSE',
  tags       => 'page=SFENCE',
};


# SHUFPD-Packed Interleave Shuffle of Pairs of Double-Precision Floating-Point Values.
ENCODING SHUFPD => {
  diagram    => 'MAP=0f MR=1 OP=0xc6 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=SHUFPD',
};


# SHUFPS-Packed Interleave Shuffle of Quadruplets of Single-Precision Floating-Point Values.
ENCODING SHUFPS => {
  diagram    => 'MAP=0f MR=1 OP=0xc6 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=SHUFPS',
};


# SQRTPD-Square Root of Double-Precision Floating-Point Values.
ENCODING SQRTPD => {
  diagram    => 'MAP=0f MR=1 OP=0x51 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|EMATH',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=SQRTPD',
};


# SQRTPS-Square Root of Single-Precision Floating-Point Values.
ENCODING SQRTPS => {
  diagram    => 'MAP=0f MR=1 OP=0x51 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|EMATH',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=SQRTPS',
};


# SQRTSD-Compute Square Root of Scalar Double-Precision Floating-Point Value.
ENCODING SQRTSD => {
  diagram    => 'MAP=0f MR=1 OP=0x51 P66=0 PF2=1 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|EMATH',
  metadata   => 'isa=SSE2',
  tags       => 'page=SQRTSD',
};


# SQRTSS-Compute Square Root of Scalar Single-Precision Value.
ENCODING SQRTSS => {
  diagram    => 'MAP=0f MR=1 OP=0x51 P66=0 PF2=0 PF3=1',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|EMATH',
  metadata   => 'isa=SSE',
  tags       => 'page=SQRTSS',
};


# STMXCSR-Store MXCSR Register State.
ENCODING STMXCSR => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xae P66=0 PF2=0 PF3=0 REG=3',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_5',
  categories => 'SIMD',
  metadata   => 'isa=SSEMXCSR',
  tags       => 'page=STMXCSR',
};


# SUBPD-Subtract Packed Double-Precision Floating-Point Values.
ENCODING SUBPD => {
  diagram    => 'MAP=0f MR=1 OP=0x5c P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=SUBPD',
};


# SUBPS-Subtract Packed Single-Precision Floating-Point Values.
ENCODING SUBPS => {
  diagram    => 'MAP=0f MR=1 OP=0x5c P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_2',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=SUBPS',
};


# SUBSD-Subtract Scalar Double-Precision Floating-Point Value.
ENCODING SUBSD => {
  diagram    => 'MAP=0f MR=1 OP=0x5c P66=0 PF2=1 PF3=0',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE2',
  tags       => 'page=SUBSD',
};


# SUBSS-Subtract Scalar Single-Precision Floating-Point Value.
ENCODING SUBSS => {
  diagram    => 'MAP=0f MR=1 OP=0x5c P66=0 PF2=0 PF3=1',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|ARITHMETIC',
  metadata   => 'isa=SSE',
  tags       => 'page=SUBSS',
};


# UCOMISD-Unordered Compare Scalar Double-Precision Floating-Point Values and Set EFLAGS.
ENCODING UCOMISD => {
  diagram    => 'MAP=0f MR=1 OP=0x2e P66=1 PF2=0 PF3=0',
  iflags     => 'af=Z cf=W of=Z pf=W sf=Z zf=W',
  extensions => 'SSE2',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|COMPARISON',
  metadata   => 'isa=SSE2',
  tags       => 'page=UCOMISD',
};


# UCOMISS-Unordered Compare Scalar Single-Precision Floating-Point Values and Set EFLAGS.
ENCODING UCOMISS => {
  diagram    => 'MAP=0f MR=1 OP=0x2e P66=0 PF2=0 PF3=0',
  iflags     => 'af=Z cf=W of=Z pf=W sf=Z zf=W',
  extensions => 'SSE',
  exceptions => 'SSE_TYPE_3',
  categories => 'SIMD|COMPARISON',
  metadata   => 'isa=SSE',
  tags       => 'page=UCOMISS',
};


# UNPCKHPD-Unpack and Interleave High Packed Double-Precision Floating-Point Values.
ENCODING UNPCKHPD => {
  diagram    => 'MAP=0f MR=1 OP=0x15 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=UNPCKHPD',
};


# UNPCKHPS-Unpack and Interleave High Packed Single-Precision Floating-Point Values.
ENCODING UNPCKHPS => {
  diagram    => 'MAP=0f MR=1 OP=0x15 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=UNPCKHPS',
};


# UNPCKLPD-Unpack and Interleave Low Packed Double-Precision Floating-Point Values.
ENCODING UNPCKLPD => {
  diagram    => 'MAP=0f MR=1 OP=0x14 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=UNPCKLPD',
};


# UNPCKLPS-Unpack and Interleave Low Packed Single-Precision Floating-Point Values.
ENCODING UNPCKLPS => {
  diagram    => 'MAP=0f MR=1 OP=0x14 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|SWIZZLE',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=UNPCKLPS',
};


# XORPD-Bitwise Logical XOR of Packed Double Precision Floating-Point Values.
ENCODING XORPD => {
  diagram    => 'MAP=0f MR=1 OP=0x57 P66=1 PF2=0 PF3=0',
  extensions => 'SSE2',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=SSE2 requires_alignment=1',
  tags       => 'page=XORPD',
};


# XORPS-Bitwise Logical XOR of Packed Single Precision Floating-Point Values.
ENCODING XORPS => {
  diagram    => 'MAP=0f MR=1 OP=0x57 P66=0 PF2=0 PF3=0',
  extensions => 'SSE',
  exceptions => 'ALIGNMENT|SSE_TYPE_4',
  categories => 'SIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=SSE requires_alignment=1',
  tags       => 'page=XORPS',
};

