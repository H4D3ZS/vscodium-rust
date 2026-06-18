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

PAGE GF2P8AFFINEINVQB => {
  title => 'GF2P8AFFINEINVQB-Galois Field Affine Transformation Inverse',
};

PAGE GF2P8AFFINEQB => {
  title => 'GF2P8AFFINEQB-Galois Field Affine Transformation',
};

PAGE GF2P8MULB => {
  title => 'GF2P8MULB-Galois Field Multiply Bytes',
};

PAGE KADDx => {
  title => 'KADDW/KADDB/KADDQ/KADDD-ADD Two Masks',
};

PAGE KANDNx => {
  title => 'KANDNW/KANDNB/KANDNQ/KANDND-Bitwise Logical AND NOT Masks',
};

PAGE KANDx => {
  title => 'KANDW/KANDB/KANDQ/KANDD-Bitwise Logical AND Masks',
};

PAGE KMOVx => {
  title => 'KMOVW/KMOVB/KMOVQ/KMOVD-Move from and to Mask Registers',
};

PAGE KNOTx => {
  title => 'KNOTW/KNOTB/KNOTQ/KNOTD-NOT Mask Register',
};

PAGE KORTESTx => {
  title => 'KORTESTW/KORTESTB/KORTESTQ/KORTESTD-OR Masks And Set Flags',
};

PAGE KORx => {
  title => 'KORW/KORB/KORQ/KORD-Bitwise Logical OR Masks',
};

PAGE KSHIFTLx => {
  title => 'KSHIFTLW/KSHIFTLB/KSHIFTLQ/KSHIFTLD-Shift Left Mask Registers',
};

PAGE KSHIFTRx => {
  title => 'KSHIFTRW/KSHIFTRB/KSHIFTRQ/KSHIFTRD-Shift Right Mask Registers',
};

PAGE KTESTx => {
  title => 'KTESTW/KTESTB/KTESTQ/KTESTD-Packed Bit Test Masks and Set Flags',
};

PAGE KUNPCKxx => {
  title => 'KUNPCKBW/KUNPCKWD/KUNPCKDQ-Unpack for Mask Registers',
};

PAGE KXNORx => {
  title => 'KXNORW/KXNORB/KXNORQ/KXNORD-Bitwise Logical XNOR Masks',
};

PAGE KXORx => {
  title => 'KXORW/KXORB/KXORQ/KXORD-Bitwise Logical XOR Masks',
};

PAGE VALIGNx => {
  title => 'VALIGND/VALIGNQ-Align Doubleword/Quadword Vectors',
};

PAGE VBLENDMPx => {
  title => 'VBLENDMPD/VBLENDMPS-Blend Float64/Float32 Vectors Using an OpMask Control',
};

PAGE VBROADCAST => {
  title => 'VBROADCAST-Load with Broadcast Floating-Point Data',
};

PAGE VCOMPRESSPD => {
  title => 'VCOMPRESSPD-Store Sparse Packed Double-Precision Floating-Point Values into Dense Memory',
};

PAGE VCOMPRESSPS => {
  title => 'VCOMPRESSPS-Store Sparse Packed Single-Precision Floating-Point Values into Dense Memory',
};

PAGE VCVTNE2PS2BF16 => {
  title => 'VCVTNE2PS2BF16',
};

PAGE VCVTNEPS2BF16 => {
  title => 'VCVTNEPS2BF16',
};

PAGE VCVTPD2QQ => {
  title => 'VCVTPD2QQ-Convert Packed Double-Precision Floating-Point Values to Packed Quadword Integers',
};

PAGE VCVTPD2UDQ => {
  title => 'VCVTPD2UDQ-Convert Packed Double-Precision Floating-Point Values to Packed Unsigned Doubleword Integers . 5-27',
};

PAGE VCVTPD2UQQ => {
  title => 'VCVTPD2UQQ-Convert Packed Double-Precision Floating-Point Values to Packed Unsigned Quadword Integers',
};

PAGE VCVTPS2QQ => {
  title => 'VCVTPS2QQ-Convert Packed Single Precision Floating-Point Values to Packed Signed Quadword Integer Values . . 5-43',
};

PAGE VCVTPS2UDQ => {
  title => 'VCVTPS2UDQ-Convert Packed Single-Precision Floating-Point Values to Packed Unsigned Doubleword Integer Values',
};

PAGE VCVTPS2UQQ => {
  title => 'VCVTPS2UQQ-Convert Packed Single Precision Floating-Point Values to Packed Unsigned Quadword Integer Values',
};

PAGE VCVTQQ2PD => {
  title => 'VCVTQQ2PD-Convert Packed Quadword Integers to Packed Double-Precision Floating-Point Values',
};

PAGE VCVTQQ2PS => {
  title => 'VCVTQQ2PS-Convert Packed Quadword Integers to Packed Single-Precision Floating-Point Values',
};

PAGE VCVTSD2USI => {
  title => 'VCVTSD2USI-Convert Scalar Double-Precision Floating-Point Value to Unsigned Doubleword Integer',
};

PAGE VCVTSS2USI => {
  title => 'VCVTSS2USI-Convert Scalar Single-Precision Floating-Point Value to Unsigned Doubleword Integer',
};

PAGE VCVTTPD2QQ => {
  title => 'VCVTTPD2QQ-Convert with Truncation Packed Double-Precision Floating-Point Values to Packed Quadword Integers',
};

PAGE VCVTTPD2UDQ => {
  title => 'VCVTTPD2UDQ-Convert with Truncation Packed Double-Precision Floating-Point Values to Packed Unsigned Doubleword Integers',
};

PAGE VCVTTPD2UQQ => {
  title => 'VCVTTPD2UQQ-Convert with Truncation Packed Double-Precision Floating-Point Values to Packed Unsigned Quadword Integers',
};

PAGE VCVTTPS2QQ => {
  title => 'VCVTTPS2QQ-Convert with Truncation Packed Single Precision Floating-Point Values to Packed Signed Quadword Integer Values',
};

PAGE VCVTTPS2UDQ => {
  title => 'VCVTTPS2UDQ-Convert with Truncation Packed Single-Precision Floating-Point Values to Packed Unsigned Doubleword Integer Values',
};

PAGE VCVTTPS2UQQ => {
  title => 'VCVTTPS2UQQ-Convert with Truncation Packed Single Precision Floating-Point Values to Packed Unsigned Quadword Integer Values',
};

PAGE VCVTTSD2USI => {
  title => 'VCVTTSD2USI-Convert with Truncation Scalar Double-Precision Floating-Point Value to Unsigned Integer',
};

PAGE VCVTTSS2USI => {
  title => 'VCVTTSS2USI-Convert with Truncation Scalar Single-Precision Floating-Point Value to Unsigned Integer',
};

PAGE VCVTUDQ2PD => {
  title => 'VCVTUDQ2PD-Convert Packed Unsigned Doubleword Integers to Packed Double-Precision Floating-Point Values . 5-72',
};

PAGE VCVTUDQ2PS => {
  title => 'VCVTUDQ2PS-Convert Packed Unsigned Doubleword Integers to Packed Single-Precision Floating-Point Values . . 5-74',
};

PAGE VCVTUQQ2PD => {
  title => 'VCVTUQQ2PD-Convert Packed Unsigned Quadword Integers to Packed Double-Precision Floating-Point Values',
};

PAGE VCVTUQQ2PS => {
  title => 'VCVTUQQ2PS-Convert Packed Unsigned Quadword Integers to Packed Single-Precision Floating-Point Values',
};

PAGE VCVTUSI2SD => {
  title => 'VCVTUSI2SD-Convert Unsigned Integer to Scalar Double-Precision Floating-Point Value',
};

PAGE VCVTUSI2SS => {
  title => 'VCVTUSI2SS-Convert Unsigned Integer to Scalar Single-Precision Floating-Point Value',
};

PAGE VDBPSADBW => {
  title => 'VDBPSADBW-Double Block Packed Sum-Absolute-Differences (SAD) on Unsigned Bytes',
};

PAGE VDPBF16PS => {
  title => 'VDPBF16PS',
};

PAGE VEXPANDPD => {
  title => 'VEXPANDPD-Load Sparse Packed Double-Precision Floating-Point Values from Dense Memory',
};

PAGE VEXPANDPS => {
  title => 'VEXPANDPS-Load Sparse Packed Single-Precision Floating-Point Values from Dense Memory',
};

PAGE VEXTRACTFxxx => {
  title => 'VEXTRACTF128/VEXTRACTF32x4/VEXTRACTF64x2/VEXTRACTF32x8/VEXTRACTF64x4-Extract Packed Floating-Point Values',
};

PAGE VEXTRACTIxxx => {
  title => 'VEXTRACTI128/VEXTRACTI32x4/VEXTRACTI64x2/VEXTRACTI32x8/VEXTRACTI64x4-Extract packed Integer Values',
};

PAGE VFIXUPIMMPD => {
  title => 'VFIXUPIMMPD-Fix Up Special Packed Float64 Values',
};

PAGE VFIXUPIMMPS => {
  title => 'VFIXUPIMMPS-Fix Up Special Packed Float32 Values',
};

PAGE VFIXUPIMMSD => {
  title => 'VFIXUPIMMSD-Fix Up Special Scalar Float64 Value',
};

PAGE VFIXUPIMMSS => {
  title => 'VFIXUPIMMSS-Fix Up Special Scalar Float32 Value',
};

PAGE VFMADDSUBxxxPD => {
  title => 'VFMADDSUB132PD/VFMADDSUB213PD/VFMADDSUB231PD-Fused Multiply-Alternating Add/Subtract of Packed Double-Precision Floating-Point Values',
};

PAGE VFMADDSUBxxxPS => {
  title => 'VFMADDSUB132PS/VFMADDSUB213PS/VFMADDSUB231PS-Fused Multiply-Alternating Add/Subtract of Packed Single-Precision Floating-Point Values',
};

PAGE VFMADDxxxPD => {
  title => 'VFMADD132PD/VFMADD213PD/VFMADD231PD-Fused Multiply-Add of Packed Double-Precision Floating-Point Values',
};

PAGE VFMADDxxxPS => {
  title => 'VFMADD132PS/VFMADD213PS/VFMADD231PS-Fused Multiply-Add of Packed Single-Precision Floating-Point Values',
};

PAGE VFMADDxxxSD => {
  title => 'VFMADD132SD/VFMADD213SD/VFMADD231SD-Fused Multiply-Add of Scalar Double-Precision Floating-Point Values',
};

PAGE VFMADDxxxSS => {
  title => 'VFMADD132SS/VFMADD213SS/VFMADD231SS-Fused Multiply-Add of Scalar Single-Precision Floating-Point Values',
};

PAGE VFMSUBADDxxxPD => {
  title => 'VFMSUBADD132PD/VFMSUBADD213PD/VFMSUBADD231PD-Fused Multiply-Alternating Subtract/Add of Packed Double-Precision Floating-Point Values',
};

PAGE VFMSUBADDxxxPS => {
  title => 'VFMSUBADD132PS/VFMSUBADD213PS/VFMSUBADD231PS-Fused Multiply-Alternating Subtract/Add of Packed Single-Precision Floating-Point Values',
};

PAGE VFMSUBxxxPD => {
  title => 'VFMSUB132PD/VFMSUB213PD/VFMSUB231PD-Fused Multiply-Subtract of Packed Double-Precision Floating-Point Values',
};

PAGE VFMSUBxxxPS => {
  title => 'VFMSUB132PS/VFMSUB213PS/VFMSUB231PS-Fused Multiply-Subtract of Packed Single-Precision Floating-Point Values',
};

PAGE VFMSUBxxxSD => {
  title => 'VFMSUB132SD/VFMSUB213SD/VFMSUB231SD-Fused Multiply-Subtract of Scalar Double-Precision Floating-Point Values',
};

PAGE VFMSUBxxxSS => {
  title => 'VFMSUB132SS/VFMSUB213SS/VFMSUB231SS-Fused Multiply-Subtract of Scalar Single-Precision Floating-Point Values',
};

PAGE VFNMADDxxxPD => {
  title => 'VFNMADD132PD/VFNMADD213PD/VFNMADD231PD-Fused Negative Multiply-Add of Packed Double-Precision Floating-Point Values',
};

PAGE VFNMADDxxxPS => {
  title => 'VFNMADD132PS/VFNMADD213PS/VFNMADD231PS-Fused Negative Multiply-Add of Packed Single-Precision Floating-Point Values',
};

PAGE VFNMADDxxxSD => {
  title => 'VFNMADD132SD/VFNMADD213SD/VFNMADD231SD-Fused Negative Multiply-Add of Scalar Double-Precision Floating-Point Values',
};

PAGE VFNMADDxxxSS => {
  title => 'VFNMADD132SS/VFNMADD213SS/VFNMADD231SS-Fused Negative Multiply-Add of Scalar Single-Precision Floating-Point Values',
};

PAGE VFNMSUBxxxPD => {
  title => 'VFNMSUB132PD/VFNMSUB213PD/VFNMSUB231PD-Fused Negative Multiply-Subtract of Packed Double-Precision Floating-Point Values',
};

PAGE VFNMSUBxxxPS => {
  title => 'VFNMSUB132PS/VFNMSUB213PS/VFNMSUB231PS-Fused Negative Multiply-Subtract of Packed Single-Precision Floating-Point Values',
};

PAGE VFNMSUBxxxSD => {
  title => 'VFNMSUB132SD/VFNMSUB213SD/VFNMSUB231SD-Fused Negative Multiply-Subtract of Scalar Double-Precision Floating-Point Values',
};

PAGE VFNMSUBxxxSS => {
  title => 'VFNMSUB132SS/VFNMSUB213SS/VFNMSUB231SS-Fused Negative Multiply-Subtract of Scalar Single-Precision Floating-Point Values',
};

PAGE VFPCLASSPD => {
  title => 'VFPCLASSPD-Tests Types Of a Packed Float64 Values',
};

PAGE VFPCLASSPS => {
  title => 'VFPCLASSPS-Tests Types Of a Packed Float32 Values',
};

PAGE VFPCLASSSD => {
  title => 'VFPCLASSSD-Tests Types Of a Scalar Float64 Values',
};

PAGE VFPCLASSSS => {
  title => 'VFPCLASSSS-Tests Types Of a Scalar Float32 Values',
};

PAGE VGATHERDPx => {
  title => 'VGATHERDPS/VGATHERDPD-Gather Packed Single, Packed Double with Signed Dword',
};

PAGE VGATHERQPx => {
  title => 'VGATHERQPS/VGATHERQPD-Gather Packed Single, Packed Double with Signed Qword Indices',
};

PAGE VGETEXPPD => {
  title => 'VGETEXPPD-Convert Exponents of Packed DP FP Values to DP FP Values',
};

PAGE VGETEXPPS => {
  title => 'VGETEXPPS-Convert Exponents of Packed SP FP Values to SP FP Values',
};

PAGE VGETEXPSD => {
  title => 'VGETEXPSD-Convert Exponents of Scalar DP FP Values to DP FP Value',
};

PAGE VGETEXPSS => {
  title => 'VGETEXPSS-Convert Exponents of Scalar SP FP Values to SP FP Value',
};

PAGE VGETMANTPD => {
  title => 'VGETMANTPD-Extract Float64 Vector of Normalized Mantissas from Float64 Vector',
};

PAGE VGETMANTPS => {
  title => 'VGETMANTPS-Extract Float32 Vector of Normalized Mantissas from Float32 Vector',
};

PAGE VGETMANTSD => {
  title => 'VGETMANTSD-Extract Float64 of Normalized Mantissas from Float64 Scalar',
};

PAGE VGETMANTSS => {
  title => 'VGETMANTSS-Extract Float32 Vector of Normalized Mantissa from Float32 Vector',
};

PAGE VINSERTFxxx => {
  title => 'VINSERTF128/VINSERTF32x4/VINSERTF64x2/VINSERTF32x8/VINSERTF64x4-Insert Packed Floating-Point Values',
};

PAGE VINSERTIxxx => {
  title => 'VINSERTI128/VINSERTI32x4/VINSERTI64x2/VINSERTI32x8/VINSERTI64x4-Insert Packed Integer Values',
};

PAGE VMASKMOV => {
  title => 'VMASKMOV-Conditional SIMD Packed Loads and Stores',
};

PAGE VP2INTERSECTD => {
  title => 'VP2INTERSECTD',
};

PAGE VP2INTERSECTQ => {
  title => 'VP2INTERSECTQ',
};

PAGE VPBLENDD => {
  title => 'VPBLENDD-Blend Packed Dwords',
};

PAGE VPBLENDMbw => {
  title => 'VPBLENDMB/VPBLENDMW-Blend Byte/Word Vectors Using an Opmask Control',
};

PAGE VPBLENDMy => {
  title => 'VPBLENDMD/VPBLENDMQ-Blend Int32/Int64 Vectors Using an OpMask Control',
};

PAGE VPBROADCAST => {
  title => 'VPBROADCAST-Load Integer and Broadcast',
};

PAGE VPBROADCASTM => {
  title => 'VPBROADCASTM-Broadcast Mask to Vector Register',
};

PAGE VPCMPxB => {
  title => 'VPCMPB/VPCMPUB-Compare Packed Byte Values Into Mask',
};

PAGE VPCMPxD => {
  title => 'VPCMPD/VPCMPUD-Compare Packed Integer Values into Mask',
};

PAGE VPCMPxQ => {
  title => 'VPCMPQ/VPCMPUQ-Compare Packed Integer Values into Mask',
};

PAGE VPCMPxW => {
  title => 'VPCMPW/VPCMPUW-Compare Packed Word Values Into Mask',
};

PAGE VPCOMPRESS => {
  title => 'VPCOMPRESSB/VCOMPRESSW-Store Sparse Packed Byte/Word Integer Values into Dense Memory/Register',
};

PAGE VPCOMPRESSD => {
  title => 'VPCOMPRESSD-Store Sparse Packed Doubleword Integer Values into Dense Memory/Register',
};

PAGE VPCOMPRESSQ => {
  title => 'VPCOMPRESSQ-Store Sparse Packed Quadword Integer Values into Dense Memory/Register',
};

PAGE VPCONFLICTx => {
  title => 'VPCONFLICTD/Q-Detect Conflicts Within a Vector of Packed Dword/Qword Values into Dense Memory/ Register. 5-334',
};

PAGE VPDPBUSD => {
  title => 'VPDPBUSD-Multiply and Add Unsigned and Signed Bytes',
};

PAGE VPDPBUSDS => {
  title => 'VPDPBUSDS-Multiply and Add Unsigned and Signed Bytes with Saturation',
};

PAGE VPDPWSSD => {
  title => 'VPDPWSSD-Multiply and Add Signed Word Integers',
};

PAGE VPDPWSSDS => {
  title => 'VPDPWSSDS-Multiply and Add Signed Word Integers with Saturation',
};

PAGE VPERM2F128 => {
  title => 'VPERM2F128-Permute Floating-Point Values',
};

PAGE VPERM2I128 => {
  title => 'VPERM2I128-Permute Integer Values',
};

PAGE VPERMB => {
  title => 'VPERMB-Permute Packed Bytes Elements',
};

PAGE VPERMI2B => {
  title => 'VPERMI2B-Full Permute of Bytes from Two Tables Overwriting the Index',
};

PAGE VPERMI2x => {
  title => 'VPERMI2W/D/Q/PS/PD-Full Permute From Two Tables Overwriting the Index',
};

PAGE VPERMILPD => {
  title => 'VPERMILPD-Permute In-Lane of Pairs of Double-Precision Floating-Point Values',
};

PAGE VPERMILPS => {
  title => 'VPERMILPS-Permute In-Lane of Quadruples of Single-Precision Floating-Point Values',
};

PAGE VPERMPD => {
  title => 'VPERMPD-Permute Double-Precision Floating-Point Elements',
};

PAGE VPERMPS => {
  title => 'VPERMPS-Permute Single-Precision Floating-Point Elements',
};

PAGE VPERMQ => {
  title => 'VPERMQ-Qwords Element Permutation',
};

PAGE VPERMT2B => {
  title => 'VPERMT2B-Full Permute of Bytes from Two Tables Overwriting a Table',
};

PAGE VPERMT2x => {
  title => 'VPERMT2W/D/Q/PS/PD-Full Permute from Two Tables Overwriting one Table',
};

PAGE VPERMx => {
  title => 'VPERMD/VPERMW-Permute Packed Doublewords/Words Elements',
};

PAGE VPEXPANDD => {
  title => 'VPEXPANDD-Load Sparse Packed Doubleword Integer Values from Dense Memory / Register',
};

PAGE VPEXPANDQ => {
  title => 'VPEXPANDQ-Load Sparse Packed Quadword Integer Values from Dense Memory / Register',
};

PAGE VPEXPANDx => {
  title => 'VPEXPANDB/VPEXPANDW-Expand Byte/Word Values',
};

PAGE VPGATHERDx => {
  title => 'VPGATHERDD/VPGATHERDQ-Gather Packed Dword, Packed Qword with Signed Dword Indices',
};

PAGE VPGATHERQx => {
  title => 'VPGATHERQD/VPGATHERQQ-Gather Packed Dword, Packed Qword with Signed Qword Indices',
};

PAGE VPLZCNTx => {
  title => 'VPLZCNTD/Q-Count the Number of Leading Zero Bits for Packed Dword, Packed Qword Values',
};

PAGE VPMADD52HUQ => {
  title => 'VPMADD52HUQ-Packed Multiply of Unsigned 52-bit Unsigned Integers and Add High 52-bit Products to 64-bit Accumulators',
};

PAGE VPMADD52LUQ => {
  title => 'VPMADD52LUQ-Packed Multiply of Unsigned 52-bit Integers and Add the Low 52-bit Products to Qword Accumulators',
};

PAGE VPMASKMOV => {
  title => 'VPMASKMOV-Conditional SIMD Integer Packed Loads and Stores',
};

PAGE VPMOVM2x => {
  title => 'VPMOVM2B/VPMOVM2W/VPMOVM2D/VPMOVM2Q-Convert a Mask Register to a Vector Register',
};

PAGE VPMOVx2M => {
  title => 'VPMOVB2M/VPMOVW2M/VPMOVD2M/VPMOVQ2M-Convert a Vector Register to a Mask',
};

PAGE VPMOVxxDB => {
  title => 'VPMOVDB/VPMOVSDB/VPMOVUSDB-Down Convert DWord to Byte',
};

PAGE VPMOVxxDW => {
  title => 'VPMOVDW/VPMOVSDW/VPMOVUSDW-Down Convert DWord to Word',
};

PAGE VPMOVxxQB => {
  title => 'VPMOVQB/VPMOVSQB/VPMOVUSQB-Down Convert QWord to Byte',
};

PAGE VPMOVxxQD => {
  title => 'VPMOVQD/VPMOVSQD/VPMOVUSQD-Down Convert QWord to DWord',
};

PAGE VPMOVxxQW => {
  title => 'VPMOVQW/VPMOVSQW/VPMOVUSQW-Down Convert QWord to Word',
};

PAGE VPMOVxxWB => {
  title => 'VPMOVWB/VPMOVSWB/VPMOVUSWB-Down Convert Word to Byte',
};

PAGE VPMULTISHIFTQB => {
  title => 'VPMULTISHIFTQB',
};

PAGE VPOPCNT => {
  title => 'VPOPCNT-Return the Count of Number of Bits Set to 1 in BYTE/WORD/DWORD/QWORD',
};

PAGE VPROLx => {
  title => 'VPROLD/VPROLVD/VPROLQ/VPROLVQ-Bit Rotate Left',
};

PAGE VPRORx => {
  title => 'VPRORD/VPRORVD/VPRORQ/VPRORVQ-Bit Rotate Right',
};

PAGE VPSCATTERxx => {
  title => 'VPSCATTERDD/VPSCATTERDQ/VPSCATTERQD/VPSCATTERQQ-Scatter Packed Dword, Packed Qword with Signed Dword, Signed Qword Indices',
};

PAGE VPSHLD => {
  title => 'VPSHLD-Concatenate and Shift Packed Data Left Logical',
};

PAGE VPSHLDV => {
  title => 'VPSHLDV-Concatenate and Variable Shift Packed Data Left Logical',
};

PAGE VPSHRD => {
  title => 'VPSHRD-Concatenate and Shift Packed Data Right Logical',
};

PAGE VPSHRDV => {
  title => 'VPSHRDV-Concatenate and Variable Shift Packed Data Right Logical',
};

PAGE VPSHUFBITQMB => {
  title => 'VPSHUFBITQMB-Shuffle Bits from Quadword Elements Using Byte Indexes into Mask',
};

PAGE VPSLLVx => {
  title => 'VPSLLVW/VPSLLVD/VPSLLVQ-Variable Bit Shift Left Logical',
};

PAGE VPSRAVx => {
  title => 'VPSRAVW/VPSRAVD/VPSRAVQ-Variable Bit Shift Right Arithmetic',
};

PAGE VPSRLVx => {
  title => 'VPSRLVW/VPSRLVD/VPSRLVQ-Variable Bit Shift Right Logical',
};

PAGE VPTERNLOGx => {
  title => 'VPTERNLOGD/VPTERNLOGQ-Bitwise Ternary Logic',
};

PAGE VPTESTMx => {
  title => 'VPTESTMB/VPTESTMW/VPTESTMD/VPTESTMQ-Logical AND and Set Mask',
};

PAGE VPTESTNMx => {
  title => 'VPTESTNMB/W/D/Q-Logical NAND and Set',
};

PAGE VRANGEPD => {
  title => 'VRANGEPD-Range Restriction Calculation For Packed Pairs of Float64 Values',
};

PAGE VRANGEPS => {
  title => 'VRANGEPS-Range Restriction Calculation For Packed Pairs of Float32 Values',
};

PAGE VRANGESD => {
  title => 'VRANGESD-Range Restriction Calculation From a pair of Scalar Float64 Values',
};

PAGE VRANGESS => {
  title => 'VRANGESS-Range Restriction Calculation From a Pair of Scalar Float32 Values',
};

PAGE VRCP14PD => {
  title => 'VRCP14PD-Compute Approximate Reciprocals of Packed Float64 Values',
};

PAGE VRCP14PS => {
  title => 'VRCP14PS-Compute Approximate Reciprocals of Packed Float32 Values',
};

PAGE VRCP14SD => {
  title => 'VRCP14SD-Compute Approximate Reciprocal of Scalar Float64 Value',
};

PAGE VRCP14SS => {
  title => 'VRCP14SS-Compute Approximate Reciprocal of Scalar Float32 Value',
};

PAGE VREDUCEPD => {
  title => 'VREDUCEPD-Perform Reduction Transformation on Packed Float64 Values',
};

PAGE VREDUCEPS => {
  title => 'VREDUCEPS-Perform Reduction Transformation on Packed Float32 Values',
};

PAGE VREDUCESD => {
  title => 'VREDUCESD-Perform a Reduction Transformation on a Scalar Float64 Value',
};

PAGE VREDUCESS => {
  title => 'VREDUCESS-Perform a Reduction Transformation on a Scalar Float32 Value',
};

PAGE VRNDSCALEPD => {
  title => 'VRNDSCALEPD-Round Packed Float64 Values To Include A Given Number Of Fraction Bits',
};

PAGE VRNDSCALEPS => {
  title => 'VRNDSCALEPS-Round Packed Float32 Values To Include A Given Number Of Fraction Bits',
};

PAGE VRNDSCALESD => {
  title => 'VRNDSCALESD-Round Scalar Float64 Value To Include A Given Number Of Fraction Bits',
};

PAGE VRNDSCALESS => {
  title => 'VRNDSCALESS-Round Scalar Float32 Value To Include A Given Number Of Fraction Bits',
};

PAGE VRSQRT14PD => {
  title => 'VRSQRT14PD-Compute Approximate Reciprocals of Square Roots of Packed Float64 Values',
};

PAGE VRSQRT14PS => {
  title => 'VRSQRT14PS-Compute Approximate Reciprocals of Square Roots of Packed Float32 Values',
};

PAGE VRSQRT14SD => {
  title => 'VRSQRT14SD-Compute Approximate Reciprocal of Square Root of Scalar Float64 Value',
};

PAGE VRSQRT14SS => {
  title => 'VRSQRT14SS-Compute Approximate Reciprocal of Square Root of Scalar Float32 Value',
};

PAGE VSCALEFPD => {
  title => 'VSCALEFPD-Scale Packed Float64 Values With Float64 Values',
};

PAGE VSCALEFPS => {
  title => 'VSCALEFPS-Scale Packed Float32 Values With Float32 Values',
};

PAGE VSCALEFSD => {
  title => 'VSCALEFSD-Scale Scalar Float64 Values With Float64 Values',
};

PAGE VSCALEFSS => {
  title => 'VSCALEFSS-Scale Scalar Float32 Value With Float32 Value',
};

PAGE VSCATTERxPx => {
  title => 'VSCATTERDPS/VSCATTERDPD/VSCATTERQPS/VSCATTERQPD-Scatter Packed Single, Packed Double with Signed Dword and Qword Indices',
};

PAGE VSHUFFIDQ => {
  title => 'VSHUFF32x4/VSHUFF64x2/VSHUFI32x4/VSHUFI64x2-Shuffle Packed Values at 128-bit Granularity',
};

PAGE VTESTPx => {
  title => 'VTESTPD/VTESTPS-Packed Bit Test',
};

PAGE VZEROALL => {
  title => 'VZEROALL-Zero XMM, YMM and ZMM Registers',
};

PAGE VZEROUPPER => {
  title => 'VZEROUPPER-Zero Upper Bits of YMM and ZMM Registers',
};

PAGE XEON_VPANDD => {
  title => 'VPANDD-Bitwise AND Int32 Vectors',
};

PAGE XEON_VPANDND => {
  title => 'VPANDND-Bitwise AND NOT Int32 Vectors',
};

PAGE XEON_VPANDNQ => {
  title => 'VPANDNQ-Bitwise AND NOT Int64 Vectors',
};

PAGE XEON_VPANDQ => {
  title => 'VPANDQ-Bitwise AND Int64 Vectors',
};

PAGE XEON_VPORD => {
  title => 'VPORD-Bitwise OR Int32 Vectors',
};

PAGE XEON_VPORQ => {
  title => 'VPORQ-Bitwise OR Int64 Vectors',
};

PAGE XEON_VPXORD => {
  title => 'VPXORD-Bitwise XOR Int32 Vectors',
};

PAGE XEON_VPXORQ => {
  title => 'VPXORQ-Bitwise XOR Int64 Vectors',
};

