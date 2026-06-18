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

PAGE ADDPD => {
  title => 'ADDPD-Add Packed Double-Precision Floating-Point Values',
};

PAGE ADDPS => {
  title => 'ADDPS-Add Packed Single-Precision Floating-Point Values',
};

PAGE ADDSD => {
  title => 'ADDSD-Add Scalar Double-Precision Floating-Point Values',
};

PAGE ADDSS => {
  title => 'ADDSS-Add Scalar Single-Precision Floating-Point Values',
};

PAGE ADDSUBPD => {
  title => 'ADDSUBPD-Packed Double-FP Add/Subtract',
};

PAGE ADDSUBPS => {
  title => 'ADDSUBPS-Packed Single-FP Add/Subtract',
};

PAGE ANDNPD => {
  title => 'ANDNPD-Bitwise Logical AND NOT of Packed Double Precision Floating-Point Values',
};

PAGE ANDNPS => {
  title => 'ANDNPS-Bitwise Logical AND NOT of Packed Single Precision Floating-Point Values',
};

PAGE ANDPD => {
  title => 'ANDPD-Bitwise Logical AND of Packed Double Precision Floating-Point Values',
};

PAGE ANDPS => {
  title => 'ANDPS-Bitwise Logical AND of Packed Single Precision Floating-Point Values',
};

PAGE BLENDPD => {
  title => 'BLENDPD-Blend Packed Double Precision Floating-Point Values',
};

PAGE BLENDPS => {
  title => 'BLENDPS-Blend Packed Single Precision Floating-Point Values',
};

PAGE BLENDVPD => {
  title => 'BLENDVPD-Variable Blend Packed Double Precision Floating-Point Values',
};

PAGE BLENDVPS => {
  title => 'BLENDVPS-Variable Blend Packed Single Precision Floating-Point Values',
};

PAGE CMPPD => {
  title => 'CMPPD-Compare Packed Double-Precision Floating-Point Values',
};

PAGE CMPPS => {
  title => 'CMPPS-Compare Packed Single-Precision Floating-Point Values',
};

PAGE CMPSS => {
  title => 'CMPSS-Compare Scalar Single-Precision Floating-Point Value',
};

PAGE COMISD => {
  title => 'COMISD-Compare Scalar Ordered Double-Precision Floating-Point Values and Set EFLAGS',
};

PAGE COMISS => {
  title => 'COMISS-Compare Scalar Ordered Single-Precision Floating-Point Values and Set EFLAGS',
};

PAGE CRC32 => {
  title => 'CRC32-Accumulate CRC32 Value',
};

PAGE CVTDQ2PD => {
  title => 'CVTDQ2PD-Convert Packed Doubleword Integers to Packed Double-Precision Floating-Point Values',
};

PAGE CVTDQ2PS => {
  title => 'CVTDQ2PS-Convert Packed Doubleword Integers to Packed Single-Precision Floating-Point Values',
};

PAGE CVTPD2DQ => {
  title => 'CVTPD2DQ-Convert Packed Double-Precision Floating-Point Values to Packed Doubleword Integers',
};

PAGE CVTPD2PI => {
  title => 'CVTPD2PI-Convert Packed Double-Precision FP Values to Packed Dword Integers',
};

PAGE CVTPD2PS => {
  title => 'CVTPD2PS-Convert Packed Double-Precision Floating-Point Values to Packed Single-Precision Floating-Point Values',
};

PAGE CVTPI2PD => {
  title => 'CVTPI2PD-Convert Packed Dword Integers to Packed Double-Precision FP Values',
};

PAGE CVTPI2PS => {
  title => 'CVTPI2PS-Convert Packed Dword Integers to Packed Single-Precision FP Values',
};

PAGE CVTPS2DQ => {
  title => 'CVTPS2DQ-Convert Packed Single-Precision Floating-Point Values to Packed Signed Doubleword Integer Values .3-258',
};

PAGE CVTPS2PD => {
  title => 'CVTPS2PD-Convert Packed Single-Precision Floating-Point Values to Packed Double-Precision Floating-Point Values',
};

PAGE CVTPS2PI => {
  title => 'CVTPS2PI-Convert Packed Single-Precision FP Values to Packed Dword Integers',
};

PAGE CVTSD2SI => {
  title => 'CVTSD2SI-Convert Scalar Double-Precision Floating-Point Value to Doubleword Integer',
};

PAGE CVTSD2SS => {
  title => 'CVTSD2SS-Convert Scalar Double-Precision Floating-Point Value to Scalar Single-Precision Floating-Point Value. .3-267',
};

PAGE CVTSI2SD => {
  title => 'CVTSI2SD-Convert Doubleword Integer to Scalar Double-Precision Floating-Point Value',
};

PAGE CVTSI2SS => {
  title => 'CVTSI2SS-Convert Doubleword Integer to Scalar Single-Precision Floating-Point Value',
};

PAGE CVTSS2SD => {
  title => 'CVTSS2SD-Convert Scalar Single-Precision Floating-Point Value to Scalar Double-Precision Floating-Point Value. .3-273',
};

PAGE CVTSS2SI => {
  title => 'CVTSS2SI-Convert Scalar Single-Precision Floating-Point Value to Doubleword Integer',
};

PAGE CVTTPD2DQ => {
  title => 'CVTTPD2DQ-Convert with Truncation Packed Double-Precision Floating-Point Values to Packed Doubleword Integers',
};

PAGE CVTTPD2PI => {
  title => 'CVTTPD2PI-Convert with Truncation Packed Double-Precision FP Values to Packed Dword Integers',
};

PAGE CVTTPS2DQ => {
  title => 'CVTTPS2DQ-Convert with Truncation Packed Single-Precision Floating-Point Values to Packed Signed Doubleword Integer Values',
};

PAGE CVTTPS2PI => {
  title => 'CVTTPS2PI-Convert with Truncation Packed Single-Precision FP Values to Packed Dword Integers',
};

PAGE CVTTSD2SI => {
  title => 'CVTTSD2SI-Convert with Truncation Scalar Double-Precision Floating-Point Value to Signed Integer',
};

PAGE CVTTSS2SI => {
  title => 'CVTTSS2SI-Convert with Truncation Scalar Single-Precision Floating-Point Value to Integer',
};

PAGE DIVPD => {
  title => 'DIVPD-Divide Packed Double-Precision Floating-Point Values',
};

PAGE DIVPS => {
  title => 'DIVPS-Divide Packed Single-Precision Floating-Point Values',
};

PAGE DIVSD => {
  title => 'DIVSD-Divide Scalar Double-Precision Floating-Point Value',
};

PAGE DIVSS => {
  title => 'DIVSS-Divide Scalar Single-Precision Floating-Point Values',
};

PAGE DPPD => {
  title => 'DPPD-Dot Product of Packed Double Precision Floating-Point Values',
};

PAGE DPPS => {
  title => 'DPPS-Dot Product of Packed Single Precision Floating-Point Values',
};

PAGE EXTRACTPS => {
  title => 'EXTRACTPS-Extract Packed Floating-Point Values',
};

PAGE FISTTP => {
  title => 'FISTTP-Store Integer with Truncation',
};

PAGE FXRSTOR => {
  title => 'FXRSTOR-Restore x87 FPU, MMX, XMM, and MXCSR State',
};

PAGE FXSAVE => {
  title => 'FXSAVE-Save x87 FPU, MMX Technology, and SSE State',
};

PAGE HADDPD => {
  title => 'HADDPD-Packed Double-FP Horizontal Add',
};

PAGE HADDPS => {
  title => 'HADDPS-Packed Single-FP Horizontal Add',
};

PAGE HSUBPD => {
  title => 'HSUBPD-Packed Double-FP Horizontal Subtract',
};

PAGE HSUBPS => {
  title => 'HSUBPS-Packed Single-FP Horizontal Subtract',
};

PAGE INSERTPS => {
  title => 'INSERTPS-Insert Scalar Single-Precision Floating-Point Value',
};

PAGE LDDQU => {
  title => 'LDDQU-Load Unaligned Integer 128 Bits',
};

PAGE LDMXCSR => {
  title => 'LDMXCSR-Load MXCSR Register',
};

PAGE LFENCE => {
  title => 'LFENCE-Load Fence',
};

PAGE MASKMOVDQU => {
  title => 'MASKMOVDQU-Store Selected Bytes of Double Quadword',
};

PAGE MAXPD => {
  title => 'MAXPD-Maximum of Packed Double-Precision Floating-Point Values',
};

PAGE MAXPS => {
  title => 'MAXPS-Maximum of Packed Single-Precision Floating-Point Values',
};

PAGE MAXSD => {
  title => 'MAXSD-Return Maximum Scalar Double-Precision Floating-Point Value',
};

PAGE MAXSS => {
  title => 'MAXSS-Return Maximum Scalar Single-Precision Floating-Point Value',
};

PAGE MFENCE => {
  title => 'MFENCE-Memory Fence',
};

PAGE MINPD => {
  title => 'MINPD-Minimum of Packed Double-Precision Floating-Point Values',
};

PAGE MINPS => {
  title => 'MINPS-Minimum of Packed Single-Precision Floating-Point Values',
};

PAGE MINSD => {
  title => 'MINSD-Return Minimum Scalar Double-Precision Floating-Point Value',
};

PAGE MINSS => {
  title => 'MINSS-Return Minimum Scalar Single-Precision Floating-Point Value',
};

PAGE MOVAPD => {
  title => 'MOVAPD-Move Aligned Packed Double-Precision Floating-Point Values',
};

PAGE MOVAPS => {
  title => 'MOVAPS-Move Aligned Packed Single-Precision Floating-Point Values',
};

PAGE MOVDDUP => {
  title => 'MOVDDUP-Replicate Double FP Values',
};

PAGE MOVDQ2Q => {
  title => 'MOVDQ2Q-Move Quadword from XMM to MMX Technology Register',
};

PAGE MOVDQA => {
  title => 'MOVDQA,VMOVDQA32/64-Move Aligned Packed Integer Values',
};

PAGE MOVDQU => {
  title => 'MOVDQU,VMOVDQU8/16/32/64-Move Unaligned Packed Integer Values',
};

PAGE MOVHLPS => {
  title => 'MOVHLPS-Move Packed Single-Precision Floating-Point Values High to Low',
};

PAGE MOVHPD => {
  title => 'MOVHPD-Move High Packed Double-Precision Floating-Point Value',
};

PAGE MOVHPS => {
  title => 'MOVHPS-Move High Packed Single-Precision Floating-Point Values',
};

PAGE MOVLHPS => {
  title => 'MOVLHPS-Move Packed Single-Precision Floating-Point Values Low to High',
};

PAGE MOVLPD => {
  title => 'MOVLPD-Move Low Packed Double-Precision Floating-Point Value',
};

PAGE MOVLPS => {
  title => 'MOVLPS-Move Low Packed Single-Precision Floating-Point Values',
};

PAGE MOVMSKPD => {
  title => 'MOVMSKPD-Extract Packed Double-Precision Floating-Point Sign Mask',
};

PAGE MOVMSKPS => {
  title => 'MOVMSKPS-Extract Packed Single-Precision Floating-Point Sign Mask',
};

PAGE MOVNTDQ => {
  title => 'MOVNTDQ-Store Packed Integers Using Non-Temporal Hint',
};

PAGE MOVNTDQA => {
  title => 'MOVNTDQA-Load Double Quadword Non-Temporal Aligned Hint',
};

PAGE MOVNTI => {
  title => 'MOVNTI-Store Doubleword Using Non-Temporal Hint',
};

PAGE MOVNTPD => {
  title => 'MOVNTPD-Store Packed Double-Precision Floating-Point Values Using Non-Temporal Hint',
};

PAGE MOVNTPS => {
  title => 'MOVNTPS-Store Packed Single-Precision Floating-Point Values Using Non-Temporal Hint',
};

PAGE MOVQ => {
  title => 'MOVQ-Move Quadword',
};

PAGE MOVQ2DQ => {
  title => 'MOVQ2DQ-Move Quadword from MMX Technology to XMM Register',
};

PAGE MOVSHDUP => {
  title => 'MOVSHDUP-Replicate Single FP Values',
};

PAGE MOVSLDUP => {
  title => 'MOVSLDUP-Replicate Single FP Values',
};

PAGE MOVSS => {
  title => 'MOVSS-Move or Merge Scalar Single-Precision Floating-Point Value',
};

PAGE MOVUPD => {
  title => 'MOVUPD-Move Unaligned Packed Double-Precision Floating-Point Values',
};

PAGE MOVUPS => {
  title => 'MOVUPS-Move Unaligned Packed Single-Precision Floating-Point Values',
};

PAGE MOVx => {
  title => 'MOVD/MOVQ-Move Doubleword/Move Quadword',
};

PAGE MPSADBW => {
  title => 'MPSADBW-Compute Multiple Packed Sums of Absolute Difference',
};

PAGE MULPD => {
  title => 'MULPD-Multiply Packed Double-Precision Floating-Point Values',
};

PAGE MULPS => {
  title => 'MULPS-Multiply Packed Single-Precision Floating-Point Values',
};

PAGE MULSD => {
  title => 'MULSD-Multiply Scalar Double-Precision Floating-Point Value',
};

PAGE MULSS => {
  title => 'MULSS-Multiply Scalar Single-Precision Floating-Point Values',
};

PAGE ORPD => {
  title => 'ORPD-Bitwise Logical OR of Packed Double Precision Floating-Point Values',
};

PAGE ORPS => {
  title => 'ORPS-Bitwise Logical OR of Packed Single Precision Floating-Point Values',
};

PAGE PABSx => {
  title => 'PABSB/PABSW/PABSD/PABSQ-Packed Absolute Value',
};

PAGE PACKSSxx => {
  title => 'PACKSSWB/PACKSSDW-Pack with Signed Saturation',
};

PAGE PACKUSDW => {
  title => 'PACKUSDW-Pack with Unsigned Saturation',
};

PAGE PACKUSWB => {
  title => 'PACKUSWB-Pack with Unsigned Saturation',
};

PAGE PADDSx => {
  title => 'PADDSB/PADDSW-Add Packed Signed Integers with Signed Saturation',
};

PAGE PADDUSx => {
  title => 'PADDUSB/PADDUSW-Add Packed Unsigned Integers with Unsigned Saturation',
};

PAGE PADDx => {
  title => 'PADDB/PADDW/PADDD/PADDQ-Add Packed Integers',
};

PAGE PALIGNR => {
  title => 'PALIGNR-Packed Align Right',
};

PAGE PAND => {
  title => 'PAND-Logical AND',
};

PAGE PANDN => {
  title => 'PANDN-Logical AND NOT',
};

PAGE PAVGx => {
  title => 'PAVGB/PAVGW-Average Packed Integers',
};

PAGE PBLENDVB => {
  title => 'PBLENDVB-Variable Blend Packed Bytes',
};

PAGE PBLENDW => {
  title => 'PBLENDW-Blend Packed Words',
};

PAGE PCMPEQQ => {
  title => 'PCMPEQQ-Compare Packed Qword Data for Equal',
};

PAGE PCMPEQx => {
  title => 'PCMPEQB/PCMPEQW/PCMPEQD-Compare Packed Data for Equal',
};

PAGE PCMPESTRI => {
  title => 'PCMPESTRI-Packed Compare Explicit Length Strings, Return Index',
};

PAGE PCMPESTRM => {
  title => 'PCMPESTRM-Packed Compare Explicit Length Strings, Return Mask',
};

PAGE PCMPGTQ => {
  title => 'PCMPGTQ-Compare Packed Data for Greater Than',
};

PAGE PCMPGTx => {
  title => 'PCMPGTB/PCMPGTW/PCMPGTD-Compare Packed Signed Integers for Greater Than',
};

PAGE PCMPISTRI => {
  title => 'PCMPISTRI-Packed Compare Implicit Length Strings, Return Index',
};

PAGE PCMPISTRM => {
  title => 'PCMPISTRM-Packed Compare Implicit Length Strings, Return Mask',
};

PAGE PEXTRW => {
  title => 'PEXTRW-Extract Word',
};

PAGE PEXTRx => {
  title => 'PEXTRB/PEXTRD/PEXTRQ-Extract Byte/Dword/Qword',
};

PAGE PHADDSW => {
  title => 'PHADDSW-Packed Horizontal Add and Saturate',
};

PAGE PHADDx => {
  title => 'PHADDW/PHADDD-Packed Horizontal Add',
};

PAGE PHMINPOSUW => {
  title => 'PHMINPOSUW-Packed Horizontal Word Minimum',
};

PAGE PHSUBSW => {
  title => 'PHSUBSW-Packed Horizontal Subtract and Saturate',
};

PAGE PHSUBx => {
  title => 'PHSUBW/PHSUBD-Packed Horizontal Subtract',
};

PAGE PINSRW => {
  title => 'PINSRW-Insert Word',
};

PAGE PINSRx => {
  title => 'PINSRB/PINSRD/PINSRQ-Insert Byte/Dword/Qword',
};

PAGE PMADDUBSW => {
  title => 'PMADDUBSW-Multiply and Add Packed Signed and Unsigned Bytes',
};

PAGE PMADDWD => {
  title => 'PMADDWD-Multiply and Add Packed Integers',
};

PAGE PMAXSx => {
  title => 'PMAXSB/PMAXSW/PMAXSD/PMAXSQ-Maximum of Packed Signed Integers',
};

PAGE PMAXUbw => {
  title => 'PMAXUB/PMAXUW-Maximum of Packed Unsigned Integers',
};

PAGE PMAXUy => {
  title => 'PMAXUD/PMAXUQ-Maximum of Packed Unsigned Integers',
};

PAGE PMINSbw => {
  title => 'PMINSB/PMINSW-Minimum of Packed Signed Integers',
};

PAGE PMINSy => {
  title => 'PMINSD/PMINSQ-Minimum of Packed Signed Integers',
};

PAGE PMINUbw => {
  title => 'PMINUB/PMINUW-Minimum of Packed Unsigned Integers',
};

PAGE PMINUy => {
  title => 'PMINUD/PMINUQ-Minimum of Packed Unsigned Integers',
};

PAGE PMOVMSKB => {
  title => 'PMOVMSKB-Move Byte Mask',
};

PAGE PMOVSX => {
  title => 'PMOVSX-Packed Move with Sign Extend',
};

PAGE PMOVZX => {
  title => 'PMOVZX-Packed Move with Zero Extend',
};

PAGE PMULDQ => {
  title => 'PMULDQ-Multiply Packed Doubleword Integers',
};

PAGE PMULHRSW => {
  title => 'PMULHRSW-Packed Multiply High with Round and Scale',
};

PAGE PMULHUW => {
  title => 'PMULHUW-Multiply Packed Unsigned Integers and Store High Result',
};

PAGE PMULHW => {
  title => 'PMULHW-Multiply Packed Signed Integers and Store High Result',
};

PAGE PMULLW => {
  title => 'PMULLW-Multiply Packed Signed Integers and Store Low Result',
};

PAGE PMULLx => {
  title => 'PMULLD/PMULLQ-Multiply Packed Integers and Store Low Result',
};

PAGE PMULUDQ => {
  title => 'PMULUDQ-Multiply Packed Unsigned Doubleword Integers',
};

PAGE POPCNT => {
  title => 'POPCNT-Return the Count of Number of Bits Set to 1',
};

PAGE POR => {
  title => 'POR-Bitwise Logical OR',
};

PAGE PREFETCHh => {
  title => 'PREFETCHh-Prefetch Data Into Caches',
};

PAGE PSADBW => {
  title => 'PSADBW-Compute Sum of Absolute Differences',
};

PAGE PSHUFB => {
  title => 'PSHUFB-Packed Shuffle Bytes',
};

PAGE PSHUFD => {
  title => 'PSHUFD-Shuffle Packed Doublewords',
};

PAGE PSHUFHW => {
  title => 'PSHUFHW-Shuffle Packed High Words',
};

PAGE PSHUFLW => {
  title => 'PSHUFLW-Shuffle Packed Low Words',
};

PAGE PSIGNx => {
  title => 'PSIGNB/PSIGNW/PSIGND-Packed SIGN',
};

PAGE PSLLDQ => {
  title => 'PSLLDQ-Shift Double Quadword Left Logical',
};

PAGE PSLLx => {
  title => 'PSLLW/PSLLD/PSLLQ-Shift Packed Data Left Logical',
};

PAGE PSRAx => {
  title => 'PSRAW/PSRAD/PSRAQ-Shift Packed Data Right Arithmetic',
};

PAGE PSRLDQ => {
  title => 'PSRLDQ-Shift Double Quadword Right Logical',
};

PAGE PSRLx => {
  title => 'PSRLW/PSRLD/PSRLQ-Shift Packed Data Right Logical',
};

PAGE PSUBQ => {
  title => 'PSUBQ-Subtract Packed Quadword Integers',
};

PAGE PSUBSx => {
  title => 'PSUBSB/PSUBSW-Subtract Packed Signed Integers with Signed Saturation',
};

PAGE PSUBUSx => {
  title => 'PSUBUSB/PSUBUSW-Subtract Packed Unsigned Integers with Unsigned Saturation',
};

PAGE PSUBx => {
  title => 'PSUBB/PSUBW/PSUBD-Subtract Packed Integers',
};

PAGE PTEST => {
  title => 'PTEST-Logical Compare',
};

PAGE PUNPCKHxx => {
  title => 'PUNPCKHBW/PUNPCKHWD/PUNPCKHDQ/PUNPCKHQDQ-Unpack High Data',
};

PAGE PUNPCKLxx => {
  title => 'PUNPCKLBW/PUNPCKLWD/PUNPCKLDQ/PUNPCKLQDQ-Unpack Low Data',
};

PAGE PXOR => {
  title => 'PXOR-Logical Exclusive OR',
};

PAGE RCPPS => {
  title => 'RCPPS-Compute Reciprocals of Packed Single-Precision Floating-Point Values',
};

PAGE RCPSS => {
  title => 'RCPSS-Compute Reciprocal of Scalar Single-Precision Floating-Point Values',
};

PAGE ROUNDPD => {
  title => 'ROUNDPD-Round Packed Double Precision Floating-Point Values',
};

PAGE ROUNDPS => {
  title => 'ROUNDPS-Round Packed Single Precision Floating-Point Values',
};

PAGE ROUNDSD => {
  title => 'ROUNDSD-Round Scalar Double Precision Floating-Point Values',
};

PAGE ROUNDSS => {
  title => 'ROUNDSS-Round Scalar Single Precision Floating-Point Values',
};

PAGE RSQRTPS => {
  title => 'RSQRTPS-Compute Reciprocals of Square Roots of Packed Single-Precision Floating-Point Values',
};

PAGE RSQRTSS => {
  title => 'RSQRTSS-Compute Reciprocal of Square Root of Scalar Single-Precision Floating-Point Value',
};

PAGE SFENCE => {
  title => 'SFENCE-Store Fence',
};

PAGE SHUFPD => {
  title => 'SHUFPD-Packed Interleave Shuffle of Pairs of Double-Precision Floating-Point Values',
};

PAGE SHUFPS => {
  title => 'SHUFPS-Packed Interleave Shuffle of Quadruplets of Single-Precision Floating-Point Values',
};

PAGE SQRTPD => {
  title => 'SQRTPD-Square Root of Double-Precision Floating-Point Values',
};

PAGE SQRTPS => {
  title => 'SQRTPS-Square Root of Single-Precision Floating-Point Values',
};

PAGE SQRTSD => {
  title => 'SQRTSD-Compute Square Root of Scalar Double-Precision Floating-Point Value',
};

PAGE SQRTSS => {
  title => 'SQRTSS-Compute Square Root of Scalar Single-Precision Value',
};

PAGE STMXCSR => {
  title => 'STMXCSR-Store MXCSR Register State',
};

PAGE SUBPD => {
  title => 'SUBPD-Subtract Packed Double-Precision Floating-Point Values',
};

PAGE SUBPS => {
  title => 'SUBPS-Subtract Packed Single-Precision Floating-Point Values',
};

PAGE SUBSD => {
  title => 'SUBSD-Subtract Scalar Double-Precision Floating-Point Value',
};

PAGE SUBSS => {
  title => 'SUBSS-Subtract Scalar Single-Precision Floating-Point Value',
};

PAGE UCOMISD => {
  title => 'UCOMISD-Unordered Compare Scalar Double-Precision Floating-Point Values and Set EFLAGS',
};

PAGE UCOMISS => {
  title => 'UCOMISS-Unordered Compare Scalar Single-Precision Floating-Point Values and Set EFLAGS',
};

PAGE UNPCKHPD => {
  title => 'UNPCKHPD-Unpack and Interleave High Packed Double-Precision Floating-Point Values',
};

PAGE UNPCKHPS => {
  title => 'UNPCKHPS-Unpack and Interleave High Packed Single-Precision Floating-Point Values',
};

PAGE UNPCKLPD => {
  title => 'UNPCKLPD-Unpack and Interleave Low Packed Double-Precision Floating-Point Values',
};

PAGE UNPCKLPS => {
  title => 'UNPCKLPS-Unpack and Interleave Low Packed Single-Precision Floating-Point Values',
};

PAGE XORPD => {
  title => 'XORPD-Bitwise Logical XOR of Packed Double Precision Floating-Point Values',
};

PAGE XORPS => {
  title => 'XORPS-Bitwise Logical XOR of Packed Single Precision Floating-Point Values',
};

