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
ENCODING VADDPD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x58 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=ADDPD',
};

ENCODING VADDPD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x58 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=ADDPD',
};

ENCODING VADDPD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x58 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=ADDPD',
};

ENCODING VADDPD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x58 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=ADDPD',
};


# ADDPS-Add Packed Single-Precision Floating-Point Values.
ENCODING VADDPS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x58 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=ADDPS',
};

ENCODING VADDPS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x58 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=ADDPS',
};

ENCODING VADDPS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x58 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=ADDPS',
};

ENCODING VADDPS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x58 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=ADDPS',
};


# ADDSD-Add Scalar Double-Precision Floating-Point Values.
ENCODING VADDSD_ronomq => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x58 P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=ADDSD',
};

ENCODING VADDSD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x58 P66=0 PF2=1 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=ADDSD',
};


# ADDSS-Add Scalar Single-Precision Floating-Point Values.
ENCODING VADDSS_ronomd => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x58 P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=ADDSS',
};

ENCODING VADDSS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x58 P66=0 PF2=0 PF3=1 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=ADDSS',
};


# ADDSUBPD-Packed Double-FP Add/Subtract.
ENCODING VADDSUBPD => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xd0 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=ADDSUBPD',
};


# ADDSUBPS-Packed Single-FP Add/Subtract.
ENCODING VADDSUBPS => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xd0 P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=ADDSUBPS',
};


# AESDEC-Perform One Round of an AES Decryption Flow.
ENCODING VAESDEC_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xde P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVXAES',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|CRYPTO',
  metadata   => 'isa=AVXAES',
  tags       => 'page=AESDEC',
};

ENCODING VAESDEC_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0xde P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|VAES',
  metadata   => 'isa=AVX512_VAES_128',
  tags       => 'page=AESDEC',
};

ENCODING VAESDEC_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xde P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'VAES',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|VAES',
  metadata   => 'isa=VAES',
  tags       => 'page=AESDEC',
};

ENCODING VAESDEC_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0xde P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|VAES',
  metadata   => 'isa=AVX512_VAES_256',
  tags       => 'page=AESDEC',
};

ENCODING VAESDEC_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0xde P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|VAES',
  metadata   => 'isa=AVX512_VAES_512',
  tags       => 'page=AESDEC',
};


# AESDECLAST-Perform Last Round of an AES Decryption Flow.
ENCODING VAESDECLAST_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xdf P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVXAES',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|CRYPTO',
  metadata   => 'isa=AVXAES',
  tags       => 'page=AESDECLAST',
};

ENCODING VAESDECLAST_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0xdf P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|VAES',
  metadata   => 'isa=AVX512_VAES_128',
  tags       => 'page=AESDECLAST',
};

ENCODING VAESDECLAST_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xdf P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'VAES',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|VAES',
  metadata   => 'isa=VAES',
  tags       => 'page=AESDECLAST',
};

ENCODING VAESDECLAST_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0xdf P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|VAES',
  metadata   => 'isa=AVX512_VAES_256',
  tags       => 'page=AESDECLAST',
};

ENCODING VAESDECLAST_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0xdf P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|VAES',
  metadata   => 'isa=AVX512_VAES_512',
  tags       => 'page=AESDECLAST',
};


# AESENC-Perform One Round of an AES Encryption Flow.
ENCODING VAESENC_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xdc P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVXAES',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|CRYPTO',
  metadata   => 'isa=AVXAES',
  tags       => 'page=AESENC',
};

ENCODING VAESENC_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0xdc P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|VAES',
  metadata   => 'isa=AVX512_VAES_128',
  tags       => 'page=AESENC',
};

ENCODING VAESENC_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xdc P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'VAES',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|VAES',
  metadata   => 'isa=VAES',
  tags       => 'page=AESENC',
};

ENCODING VAESENC_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0xdc P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|VAES',
  metadata   => 'isa=AVX512_VAES_256',
  tags       => 'page=AESENC',
};

ENCODING VAESENC_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0xdc P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|VAES',
  metadata   => 'isa=AVX512_VAES_512',
  tags       => 'page=AESENC',
};


# AESENCLAST-Perform Last Round of an AES Encryption Flow.
ENCODING VAESENCLAST_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xdd P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVXAES',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|CRYPTO',
  metadata   => 'isa=AVXAES',
  tags       => 'page=AESENCLAST',
};

ENCODING VAESENCLAST_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0xdd P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|VAES',
  metadata   => 'isa=AVX512_VAES_128',
  tags       => 'page=AESENCLAST',
};

ENCODING VAESENCLAST_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xdd P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'VAES',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|VAES',
  metadata   => 'isa=VAES',
  tags       => 'page=AESENCLAST',
};

ENCODING VAESENCLAST_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0xdd P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|VAES',
  metadata   => 'isa=AVX512_VAES_256',
  tags       => 'page=AESENCLAST',
};

ENCODING VAESENCLAST_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0xdd P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|VAES',
  metadata   => 'isa=AVX512_VAES_512',
  tags       => 'page=AESENCLAST',
};


# AESIMC-Perform the AES InvMixColumn Transformation.
ENCODING VAESIMC => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xdb P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVXAES',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|CRYPTO',
  metadata   => 'isa=AVXAES',
  tags       => 'page=AESIMC',
};


# AESKEYGENASSIST-AES Round Key Generation Assist.
ENCODING VAESKEYGENASSIST => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0xdf P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVXAES',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|CRYPTO',
  metadata   => 'isa=AVXAES',
  tags       => 'page=AESKEYGENASSIST',
};


# ANDNPD-Bitwise Logical AND NOT of Packed Double Precision Floating-Point Values.
ENCODING VANDNPD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x55 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=ANDNPD',
};

ENCODING VANDNPD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x55 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX',
  tags       => 'page=ANDNPD',
};

ENCODING VANDNPD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x55 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=ANDNPD',
};

ENCODING VANDNPD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x55 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=ANDNPD',
};


# ANDNPS-Bitwise Logical AND NOT of Packed Single Precision Floating-Point Values.
ENCODING VANDNPS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x55 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=ANDNPS',
};

ENCODING VANDNPS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x55 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX',
  tags       => 'page=ANDNPS',
};

ENCODING VANDNPS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x55 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=ANDNPS',
};

ENCODING VANDNPS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x55 P66=0 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=ANDNPS',
};


# ANDPD-Bitwise Logical AND of Packed Double Precision Floating-Point Values.
ENCODING VANDPD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x54 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=ANDPD',
};

ENCODING VANDPD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x54 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX',
  tags       => 'page=ANDPD',
};

ENCODING VANDPD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x54 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=ANDPD',
};

ENCODING VANDPD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x54 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=ANDPD',
};


# ANDPS-Bitwise Logical AND of Packed Single Precision Floating-Point Values.
ENCODING VANDPS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x54 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=ANDPS',
};

ENCODING VANDPS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x54 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX',
  tags       => 'page=ANDPS',
};

ENCODING VANDPS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x54 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=ANDPS',
};

ENCODING VANDPS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x54 P66=0 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=ANDPS',
};


# BLENDPD-Blend Packed Double Precision Floating-Point Values.
ENCODING VBLENDPD => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x0d P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX',
  tags       => 'page=BLENDPD',
};


# BLENDPS-Blend Packed Single Precision Floating-Point Values.
ENCODING VBLENDPS => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x0c P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX',
  tags       => 'page=BLENDPS',
};


# BLENDVPD-Variable Blend Packed Double Precision Floating-Point Values.
ENCODING VBLENDVPD => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x4b P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX',
  tags       => 'page=BLENDVPD',
};


# BLENDVPS-Variable Blend Packed Single Precision Floating-Point Values.
ENCODING VBLENDVPS => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x4a P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX',
  tags       => 'page=BLENDVPS',
};


# CMPPD-Compare Packed Double-Precision Floating-Point Values.
ENCODING VCMPPD_nomoub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xc2 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=CMPPD',
};

ENCODING VCMPPD_nymyub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xc2 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=CMPPD',
};

ENCODING VCMPPD_nzmzub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xc2 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=CMPPD',
};

ENCODING VCMPPD_rxnxmxub => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xc2 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX',
  tags       => 'page=CMPPD',
};


# CMPPS-Compare Packed Single-Precision Floating-Point Values.
ENCODING VCMPPS_nomoub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xc2 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=CMPPS',
};

ENCODING VCMPPS_nymyub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xc2 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=CMPPS',
};

ENCODING VCMPPS_nzmzub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xc2 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=CMPPS',
};

ENCODING VCMPPS_rxnxmxub => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xc2 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX',
  tags       => 'page=CMPPS',
};


# CMPS/CMPSB/CMPSW/CMPSD/CMPSQ-Compare String Operands.
ENCODING VCMPSD_nomxub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xc2 P66=0 PF2=1 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=CMPS',
};

ENCODING VCMPSD_ronomqub => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xc2 P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX',
  tags       => 'page=CMPS',
};


# CMPSS-Compare Scalar Single-Precision Floating-Point Value.
ENCODING VCMPSS_nomxub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xc2 P66=0 PF2=0 PF3=1 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=CMPSS',
};

ENCODING VCMPSS_ronomdub => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xc2 P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX',
  tags       => 'page=CMPSS',
};


# COMISD-Compare Scalar Ordered Double-Precision Floating-Point Values and Set EFLAGS.
ENCODING VCOMISD_romx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x2f P66=1 PF2=0 PF3=0 W=1',
  iflags     => 'af=Z cf=W of=Z pf=W sf=Z zf=W',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3NF',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=COMISD',
};

ENCODING VCOMISD_rqmq => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x2f P66=1 PF2=0 PF3=0',
  iflags     => 'af=Z cf=W of=Z pf=W sf=Z zf=W',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX',
  tags       => 'page=COMISD',
};


# COMISS-Compare Scalar Ordered Single-Precision Floating-Point Values and Set EFLAGS.
ENCODING VCOMISS_rdmd => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x2f P66=0 PF2=0 PF3=0',
  iflags     => 'af=Z cf=W of=Z pf=W sf=Z zf=W',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX',
  tags       => 'page=COMISS',
};

ENCODING VCOMISS_romx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x2f P66=0 PF2=0 PF3=0 W=0',
  iflags     => 'af=Z cf=W of=Z pf=W sf=Z zf=W',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3NF',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=COMISS',
};


# CVTDQ2PD-Convert Packed Doubleword Integers to Packed Double-Precision Floating-Point Values.
ENCODING VCVTDQ2PD_romx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xe6 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=CVTDQ2PD',
};

ENCODING VCVTDQ2PD_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe6 P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=CVTDQ2PD',
};

ENCODING VCVTDQ2PD_rymo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xe6 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=CVTDQ2PD',
};

ENCODING VCVTDQ2PD_rzmy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xe6 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=CVTDQ2PD',
};


# CVTDQ2PS-Convert Packed Doubleword Integers to Packed Single-Precision Floating-Point Values.
ENCODING VCVTDQ2PS_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5b P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=CVTDQ2PS',
};

ENCODING VCVTDQ2PS_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5b P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=CVTDQ2PS',
};

ENCODING VCVTDQ2PS_rymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5b P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=CVTDQ2PS',
};

ENCODING VCVTDQ2PS_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5b P66=0 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=CVTDQ2PS',
};


# CVTPD2DQ-Convert Packed Double-Precision Floating-Point Values to Packed Doubleword Integers.
ENCODING VCVTPD2DQ_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xe6 P66=0 PF2=1 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=CVTPD2DQ',
};

ENCODING VCVTPD2DQ_romx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe6 P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=CVTPD2DQ',
};

ENCODING VCVTPD2DQ_romy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xe6 P66=0 PF2=1 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=CVTPD2DQ',
};

ENCODING VCVTPD2DQ_rymz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xe6 P66=0 PF2=1 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=CVTPD2DQ',
};


# CVTPD2PS-Convert Packed Double-Precision Floating-Point Values to Packed Single-Precision Floating-Point Values.
ENCODING VCVTPD2PS_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5a P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=CVTPD2PS',
};

ENCODING VCVTPD2PS_romx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5a P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=CVTPD2PS',
};

ENCODING VCVTPD2PS_romy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5a P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=CVTPD2PS',
};

ENCODING VCVTPD2PS_rymz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5a P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=CVTPD2PS',
};


# CVTPS2DQ-Convert Packed Single-Precision Floating-Point Values to Packed Signed Doubleword Integer Values .3-258.
ENCODING VCVTPS2DQ_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5b P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=CVTPS2DQ',
};

ENCODING VCVTPS2DQ_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5b P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=CVTPS2DQ',
};

ENCODING VCVTPS2DQ_rymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5b P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=CVTPS2DQ',
};

ENCODING VCVTPS2DQ_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5b P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=CVTPS2DQ',
};


# CVTPS2PD-Convert Packed Single-Precision Floating-Point Values to Packed Double-Precision Floating-Point Values.
ENCODING VCVTPS2PD_romx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5a P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=CVTPS2PD',
};

ENCODING VCVTPS2PD_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5a P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=CVTPS2PD',
};

ENCODING VCVTPS2PD_rymo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5a P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=CVTPS2PD',
};

ENCODING VCVTPS2PD_rzmy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5a P66=0 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=CVTPS2PD',
};


# CVTSD2SI-Convert Scalar Double-Precision Floating-Point Value to Doubleword Integer.
ENCODING VCVTSD2SI_rxmq => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x2d P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=CVTSD2SI',
};

ENCODING VCVTSD2SI_rxmx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x2d P66=0 PF2=1 PF3=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=CVTSD2SI',
};


# CVTSD2SS-Convert Scalar Double-Precision Floating-Point Value to Scalar Single-Precision Floating-Point Value. .3-267.
ENCODING VCVTSD2SS_ronomq => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5a P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=CVTSD2SS',
};

ENCODING VCVTSD2SS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5a P66=0 PF2=1 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=CVTSD2SS',
};


# CVTSI2SD-Convert Doubleword Integer to Scalar Double-Precision Floating-Point Value.
ENCODING VCVTSI2SD_ronomd => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x2a P66=0 PF2=1 PF3=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E10NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=CVTSI2SD',
};

ENCODING VCVTSI2SD_ronomq => {
  diagram    => 'ENC=EVEX MAP=0f MODE=64 MR=1 OP=0x2a P66=0 PF2=1 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=CVTSI2SD',
};

ENCODING VCVTSI2SD_ronomx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x2a P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=CVTSI2SD',
};


# CVTSI2SS-Convert Doubleword Integer to Scalar Single-Precision Floating-Point Value.
ENCODING VCVTSI2SS_ronomx_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x2a P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=CVTSI2SS',
};

ENCODING VCVTSI2SS_ronomx_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x2a P66=0 PF2=0 PF3=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=CVTSI2SS',
};


# CVTSS2SD-Convert Scalar Single-Precision Floating-Point Value to Scalar Double-Precision Floating-Point Value. .3-273.
ENCODING VCVTSS2SD_ronomd => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5a P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=CVTSS2SD',
};

ENCODING VCVTSS2SD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5a P66=0 PF2=0 PF3=1 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=CVTSS2SD',
};


# CVTSS2SI-Convert Scalar Single-Precision Floating-Point Value to Doubleword Integer.
ENCODING VCVTSS2SI_rxmd => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x2d P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=CVTSS2SI',
};

ENCODING VCVTSS2SI_rxmx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x2d P66=0 PF2=0 PF3=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=CVTSS2SI',
};


# CVTTPD2DQ-Convert with Truncation Packed Double-Precision Floating-Point Values to Packed Doubleword Integers.
ENCODING VCVTTPD2DQ_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xe6 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=CVTTPD2DQ',
};

ENCODING VCVTTPD2DQ_romx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe6 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=CVTTPD2DQ',
};

ENCODING VCVTTPD2DQ_romy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xe6 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=CVTTPD2DQ',
};

ENCODING VCVTTPD2DQ_rymz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xe6 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=CVTTPD2DQ',
};


# CVTTPS2DQ-Convert with Truncation Packed Single-Precision Floating-Point Values to Packed Signed Doubleword Integer Values.
ENCODING VCVTTPS2DQ_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5b P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=CVTTPS2DQ',
};

ENCODING VCVTTPS2DQ_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5b P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=CVTTPS2DQ',
};

ENCODING VCVTTPS2DQ_rymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5b P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=CVTTPS2DQ',
};

ENCODING VCVTTPS2DQ_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5b P66=0 PF2=0 PF3=1 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=CVTTPS2DQ',
};


# CVTTSD2SI-Convert with Truncation Scalar Double-Precision Floating-Point Value to Signed Integer.
ENCODING VCVTTSD2SI_rxmq => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x2c P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=CVTTSD2SI',
};

ENCODING VCVTTSD2SI_rxmx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x2c P66=0 PF2=1 PF3=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=CVTTSD2SI',
};


# CVTTSS2SI-Convert with Truncation Scalar Single-Precision Floating-Point Value to Integer.
ENCODING VCVTTSS2SI_rxmd => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x2c P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=CVTTSS2SI',
};

ENCODING VCVTTSS2SI_rxmx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x2c P66=0 PF2=0 PF3=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=CVTTSS2SI',
};


# DIVPD-Divide Packed Double-Precision Floating-Point Values.
ENCODING VDIVPD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5e P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=DIVPD',
};

ENCODING VDIVPD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5e P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=DIVPD',
};

ENCODING VDIVPD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5e P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=DIVPD',
};

ENCODING VDIVPD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5e P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=DIVPD',
};


# DIVPS-Divide Packed Single-Precision Floating-Point Values.
ENCODING VDIVPS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5e P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=DIVPS',
};

ENCODING VDIVPS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5e P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=DIVPS',
};

ENCODING VDIVPS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5e P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=DIVPS',
};

ENCODING VDIVPS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5e P66=0 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=DIVPS',
};


# DIVSD-Divide Scalar Double-Precision Floating-Point Value.
ENCODING VDIVSD_ronomq => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5e P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=DIVSD',
};

ENCODING VDIVSD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5e P66=0 PF2=1 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=DIVSD',
};


# DIVSS-Divide Scalar Single-Precision Floating-Point Values.
ENCODING VDIVSS_ronomd => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5e P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=DIVSS',
};

ENCODING VDIVSS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5e P66=0 PF2=0 PF3=1 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=DIVSS',
};


# DPPD-Dot Product of Packed Double Precision Floating-Point Values.
ENCODING VDPPD => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x41 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2D',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=DPPD',
};


# DPPS-Dot Product of Packed Single Precision Floating-Point Values.
ENCODING VDPPS => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x40 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2D',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=DPPS',
};


# EXTRACTPS-Extract Packed Floating-Point Values.
ENCODING VEXTRACTPS_mdroub_1 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x17 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=EXTRACTPS',
};

ENCODING VEXTRACTPS_mdroub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x17 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=EXTRACTPS',
};


# GF2P8AFFINEINVQB-Galois Field Affine Transformation Inverse.
ENCODING GF2P8AFFINEINVQB => {
  diagram    => 'MAP=0f3a MR=1 OP=0xcf P66=1 PF2=0 PF3=0',
  extensions => 'GFNI',
  exceptions => 'E4',
  categories => 'ADVSIMD',
  metadata   => 'isa=GFNI',
  tags       => 'page=GF2P8AFFINEINVQB',
};

ENCODING VGF2P8AFFINEINVQB_ronomoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0xcf P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_GFNI_128',
  tags       => 'page=GF2P8AFFINEINVQB',
};

ENCODING VGF2P8AFFINEINVQB_rxnxmxub => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0xcf P66=1 PF2=0 PF3=0 W=1',
  extensions => 'GFNI',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX_GFNI',
  tags       => 'page=GF2P8AFFINEINVQB',
};

ENCODING VGF2P8AFFINEINVQB_rynymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0xcf P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_GFNI_256',
  tags       => 'page=GF2P8AFFINEINVQB',
};

ENCODING VGF2P8AFFINEINVQB_rznzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0xcf P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_GFNI_512',
  tags       => 'page=GF2P8AFFINEINVQB',
};


# GF2P8AFFINEQB-Galois Field Affine Transformation.
ENCODING GF2P8AFFINEQB => {
  diagram    => 'MAP=0f3a MR=1 OP=0xce P66=1 PF2=0 PF3=0',
  extensions => 'GFNI',
  exceptions => 'E4',
  categories => 'ADVSIMD',
  metadata   => 'isa=GFNI',
  tags       => 'page=GF2P8AFFINEQB',
};

ENCODING VGF2P8AFFINEQB_ronomoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0xce P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_GFNI_128',
  tags       => 'page=GF2P8AFFINEQB',
};

ENCODING VGF2P8AFFINEQB_rxnxmxub => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0xce P66=1 PF2=0 PF3=0 W=1',
  extensions => 'GFNI',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX_GFNI',
  tags       => 'page=GF2P8AFFINEQB',
};

ENCODING VGF2P8AFFINEQB_rynymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0xce P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_GFNI_256',
  tags       => 'page=GF2P8AFFINEQB',
};

ENCODING VGF2P8AFFINEQB_rznzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0xce P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_GFNI_512',
  tags       => 'page=GF2P8AFFINEQB',
};


# GF2P8MULB-Galois Field Multiply Bytes.
ENCODING GF2P8MULB => {
  diagram    => 'MAP=0f38 MR=1 OP=0xcf P66=1 PF2=0 PF3=0',
  extensions => 'GFNI',
  exceptions => 'E4',
  categories => 'ADVSIMD',
  metadata   => 'isa=GFNI',
  tags       => 'page=GF2P8MULB',
};

ENCODING VGF2P8MULB_ronomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0xcf P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_GFNI_128',
  tags       => 'page=GF2P8MULB',
};

ENCODING VGF2P8MULB_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xcf P66=1 PF2=0 PF3=0 W=0',
  extensions => 'GFNI',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX_GFNI',
  tags       => 'page=GF2P8MULB',
};

ENCODING VGF2P8MULB_rynymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0xcf P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_GFNI_256',
  tags       => 'page=GF2P8MULB',
};

ENCODING VGF2P8MULB_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0xcf P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_GFNI_512',
  tags       => 'page=GF2P8MULB',
};


# HADDPD-Packed Double-FP Horizontal Add.
ENCODING VHADDPD => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x7c P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=HADDPD',
};


# HADDPS-Packed Single-FP Horizontal Add.
ENCODING VHADDPS => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x7c P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=HADDPS',
};


# HSUBPD-Packed Double-FP Horizontal Subtract.
ENCODING VHSUBPD => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x7d P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=HSUBPD',
};


# HSUBPS-Packed Single-FP Horizontal Subtract.
ENCODING VHSUBPS => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x7d P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=HSUBPS',
};


# INSERTPS-Insert Scalar Single-Precision Floating-Point Value.
ENCODING VINSERTPS_ronomxub_1 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x21 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=INSERTPS',
};

ENCODING VINSERTPS_ronomxub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x21 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=INSERTPS',
};


# KADDW/KADDB/KADDQ/KADDD-ADD Two Masks.
ENCODING KADDx_V_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x4a PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512DQ_KOP',
  tags       => 'page=KADDx',
};

ENCODING KADDx_V_2 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x4a PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512BW_KOP',
  tags       => 'page=KADDx',
};


# KANDNW/KANDNB/KANDNQ/KANDND-Bitwise Logical AND NOT Masks.
ENCODING KANDNB => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x42 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512DQ_KOP',
  tags       => 'page=KANDNx',
};

ENCODING KANDNW => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x42 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512F_KOP',
  tags       => 'page=KANDNx',
};

ENCODING KANDNx => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x42 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512BW_KOP',
  tags       => 'page=KANDNx',
};


# KANDW/KANDB/KANDQ/KANDD-Bitwise Logical AND Masks.
ENCODING KANDB => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x41 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512DQ_KOP',
  tags       => 'page=KANDx',
};

ENCODING KANDW => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x41 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512F_KOP',
  tags       => 'page=KANDx',
};

ENCODING KANDx => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x41 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512BW_KOP',
  tags       => 'page=KANDx',
};


# KMOVW/KMOVB/KMOVQ/KMOVD-Move from and to Mask Registers.
ENCODING KMOVB_mb_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x90 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K21',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512DQ_KOP',
  tags       => 'page=KMOVx',
};

ENCODING KMOVB_mb_2 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0x91 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K21',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512DQ_KOP',
  tags       => 'page=KMOVx',
};

ENCODING KMOVB_md => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x92 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512DQ_KOP',
  tags       => 'page=KMOVx',
};

ENCODING KMOVB_rd => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x93 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512DQ_KOP',
  tags       => 'page=KMOVx',
};

ENCODING KMOVW_md => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x92 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K21',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512F_KOP',
  tags       => 'page=KMOVx',
};

ENCODING KMOVW_mw_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x90 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K21',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512F_KOP',
  tags       => 'page=KMOVx',
};

ENCODING KMOVW_mw_2 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0x91 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K21',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512F_KOP',
  tags       => 'page=KMOVx',
};

ENCODING KMOVW_rd => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x93 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512F_KOP',
  tags       => 'page=KMOVx',
};

ENCODING KMOVx_mx_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x90 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K21',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512BW_KOP',
  tags       => 'page=KMOVx',
};

ENCODING KMOVx_mx_2 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0x91 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K21',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512BW_KOP',
  tags       => 'page=KMOVx',
};

ENCODING KMOVx_mx_3 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x92 P66=0 PF2=1 PF3=0 VL=128',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512BW_KOP',
  tags       => 'page=KMOVx',
};

ENCODING KMOVx_rx => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x93 P66=0 PF2=1 PF3=0 VL=128',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512BW_KOP',
  tags       => 'page=KMOVx',
};


# KNOTW/KNOTB/KNOTQ/KNOTD-NOT Mask Register.
ENCODING KNOTB => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x44 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512DQ_KOP',
  tags       => 'page=KNOTx',
};

ENCODING KNOTW => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x44 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512F_KOP',
  tags       => 'page=KNOTx',
};

ENCODING KNOTx => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x44 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512BW_KOP',
  tags       => 'page=KNOTx',
};


# KORTESTW/KORTESTB/KORTESTQ/KORTESTD-OR Masks And Set Flags.
ENCODING KORTESTB => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x98 P66=1 PF2=0 PF3=0 VL=128 W=0',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512DQ_KOP',
  tags       => 'page=KORTESTx',
};

ENCODING KORTESTW => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x98 P66=0 PF2=0 PF3=0 VL=128 W=0',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512F_KOP',
  tags       => 'page=KORTESTx',
};

ENCODING KORTESTx => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x98 PF2=0 PF3=0 VL=128 W=1',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512BW_KOP',
  tags       => 'page=KORTESTx',
};


# KORW/KORB/KORQ/KORD-Bitwise Logical OR Masks.
ENCODING KORB => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x45 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512DQ_KOP',
  tags       => 'page=KORx',
};

ENCODING KORW => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x45 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512F_KOP',
  tags       => 'page=KORx',
};

ENCODING KORx => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x45 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512BW_KOP',
  tags       => 'page=KORx',
};


# KSHIFTLW/KSHIFTLB/KSHIFTLQ/KSHIFTLD-Shift Left Mask Registers.
ENCODING KSHIFTLB => {
  diagram    => 'ENC=VEX MAP=0f3a MOD=REG MR=1 OP=0x32 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|SHIFT_LEFT',
  metadata   => 'isa=AVX512DQ_KOP',
  tags       => 'page=KSHIFTLx',
};

ENCODING KSHIFTLW => {
  diagram    => 'ENC=VEX MAP=0f3a MOD=REG MR=1 OP=0x32 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|SHIFT_LEFT',
  metadata   => 'isa=AVX512F_KOP',
  tags       => 'page=KSHIFTLx',
};

ENCODING KSHIFTLx => {
  diagram    => 'ENC=VEX MAP=0f3a MOD=REG MR=1 OP=0x33 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|SHIFT_LEFT',
  metadata   => 'isa=AVX512BW_KOP',
  tags       => 'page=KSHIFTLx',
};


# KSHIFTRW/KSHIFTRB/KSHIFTRQ/KSHIFTRD-Shift Right Mask Registers.
ENCODING KSHIFTRB => {
  diagram    => 'ENC=VEX MAP=0f3a MOD=REG MR=1 OP=0x30 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|SHIFT_RIGHT',
  metadata   => 'isa=AVX512DQ_KOP',
  tags       => 'page=KSHIFTRx',
};

ENCODING KSHIFTRW => {
  diagram    => 'ENC=VEX MAP=0f3a MOD=REG MR=1 OP=0x30 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|SHIFT_RIGHT',
  metadata   => 'isa=AVX512F_KOP',
  tags       => 'page=KSHIFTRx',
};

ENCODING KSHIFTRx => {
  diagram    => 'ENC=VEX MAP=0f3a MOD=REG MR=1 OP=0x31 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|SHIFT_RIGHT',
  metadata   => 'isa=AVX512BW_KOP',
  tags       => 'page=KSHIFTRx',
};


# KTESTW/KTESTB/KTESTQ/KTESTD-Packed Bit Test Masks and Set Flags.
ENCODING KTESTx_V_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x99 PF2=0 PF3=0 VL=128 W=0',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512DQ_KOP',
  tags       => 'page=KTESTx',
};

ENCODING KTESTx_V_2 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x99 PF2=0 PF3=0 VL=128 W=1',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512BW_KOP',
  tags       => 'page=KTESTx',
};


# KUNPCKBW/KUNPCKWD/KUNPCKDQ-Unpack for Mask Registers.
ENCODING KUNPCKBW => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x4b P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512F_KOP',
  tags       => 'page=KUNPCKxx',
};

ENCODING KUNPCKxx => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x4b P66=0 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_KOP',
  tags       => 'page=KUNPCKxx',
};


# KXNORW/KXNORB/KXNORQ/KXNORD-Bitwise Logical XNOR Masks.
ENCODING KXNORB => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x46 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512DQ_KOP',
  tags       => 'page=KXNORx',
};

ENCODING KXNORW => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x46 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512F_KOP',
  tags       => 'page=KXNORx',
};

ENCODING KXNORx => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x46 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512BW_KOP',
  tags       => 'page=KXNORx',
};


# KXORW/KXORB/KXORQ/KXORD-Bitwise Logical XOR Masks.
ENCODING KXORB => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x47 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512DQ_KOP',
  tags       => 'page=KXORx',
};

ENCODING KXORW => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x47 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512F_KOP',
  tags       => 'page=KXORx',
};

ENCODING KXORx => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x47 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512VEX',
  exceptions => 'AVX512_K20',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512BW_KOP',
  tags       => 'page=KXORx',
};


# LDDQU-Load Unaligned Integer 128 Bits.
ENCODING VLDDQU => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0xf0 P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX',
  tags       => 'page=LDDQU',
};


# LDMXCSR-Load MXCSR Register.
ENCODING VLDMXCSR => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0xae P66=0 PF2=0 PF3=0 REG=2 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5L',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=LDMXCSR',
};


# MASKMOVDQU-Store Selected Bytes of Double Quadword.
ENCODING VMASKMOVDQU => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0xf7 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=MASKMOVDQU',
};


# MAXPD-Maximum of Packed Double-Precision Floating-Point Values.
ENCODING VMAXPD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5f P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MAXPD',
};

ENCODING VMAXPD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5f P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=MAXPD',
};

ENCODING VMAXPD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5f P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MAXPD',
};

ENCODING VMAXPD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5f P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MAXPD',
};


# MAXPS-Maximum of Packed Single-Precision Floating-Point Values.
ENCODING VMAXPS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5f P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MAXPS',
};

ENCODING VMAXPS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5f P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=MAXPS',
};

ENCODING VMAXPS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5f P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MAXPS',
};

ENCODING VMAXPS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5f P66=0 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MAXPS',
};


# MAXSD-Return Maximum Scalar Double-Precision Floating-Point Value.
ENCODING VMAXSD_ronomq => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5f P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|MATH|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=MAXSD',
};

ENCODING VMAXSD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5f P66=0 PF2=1 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MATH|MIN_MAX',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=MAXSD',
};


# MAXSS-Return Maximum Scalar Single-Precision Floating-Point Value.
ENCODING VMAXSS_ronomd => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5f P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|MATH|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=MAXSS',
};

ENCODING VMAXSS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5f P66=0 PF2=0 PF3=1 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MATH|MIN_MAX',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=MAXSS',
};


# MINPD-Minimum of Packed Double-Precision Floating-Point Values.
ENCODING VMINPD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5d P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MINPD',
};

ENCODING VMINPD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5d P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=MINPD',
};

ENCODING VMINPD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5d P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MINPD',
};

ENCODING VMINPD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5d P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MINPD',
};


# MINPS-Minimum of Packed Single-Precision Floating-Point Values.
ENCODING VMINPS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5d P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MINPS',
};

ENCODING VMINPS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5d P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=MINPS',
};

ENCODING VMINPS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5d P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MINPS',
};

ENCODING VMINPS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5d P66=0 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MINPS',
};


# MINSD-Return Minimum Scalar Double-Precision Floating-Point Value.
ENCODING VMINSD_ronomq => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5d P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|MATH|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=MINSD',
};

ENCODING VMINSD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5d P66=0 PF2=1 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MATH|MIN_MAX',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=MINSD',
};


# MINSS-Return Minimum Scalar Single-Precision Floating-Point Value.
ENCODING VMINSS_ronomd => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5d P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|MATH|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=MINSS',
};

ENCODING VMINSS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5d P66=0 PF2=0 PF3=1 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MATH|MIN_MAX',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=MINSS',
};


# MOVAPD-Move Aligned Packed Double-Precision Floating-Point Values.
ENCODING VMOVAPD_moro_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x29 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MOVAPD',
};

ENCODING VMOVAPD_moro_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x29 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_128 requires_alignment=1',
  tags       => 'page=MOVAPD',
};

ENCODING VMOVAPD_mxrx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x29 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'ALIGNMENT|AVX_TYPE_1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX requires_alignment=1',
  tags       => 'page=MOVAPD',
};

ENCODING VMOVAPD_myry_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x29 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MOVAPD',
};

ENCODING VMOVAPD_myry_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x29 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_256 requires_alignment=1',
  tags       => 'page=MOVAPD',
};

ENCODING VMOVAPD_mzrz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x29 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MOVAPD',
};

ENCODING VMOVAPD_mzrz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x29 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_512 requires_alignment=1',
  tags       => 'page=MOVAPD',
};

ENCODING VMOVAPD_romo_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x28 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MOVAPD',
};

ENCODING VMOVAPD_romo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x28 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_128 requires_alignment=1',
  tags       => 'page=MOVAPD',
};

ENCODING VMOVAPD_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x28 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'ALIGNMENT|AVX_TYPE_1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX requires_alignment=1',
  tags       => 'page=MOVAPD',
};

ENCODING VMOVAPD_rymy_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x28 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MOVAPD',
};

ENCODING VMOVAPD_rymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x28 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_256 requires_alignment=1',
  tags       => 'page=MOVAPD',
};

ENCODING VMOVAPD_rzmz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x28 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MOVAPD',
};

ENCODING VMOVAPD_rzmz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x28 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_512 requires_alignment=1',
  tags       => 'page=MOVAPD',
};


# MOVAPS-Move Aligned Packed Single-Precision Floating-Point Values.
ENCODING VMOVAPS_moro_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x29 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MOVAPS',
};

ENCODING VMOVAPS_moro_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x29 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_128 requires_alignment=1',
  tags       => 'page=MOVAPS',
};

ENCODING VMOVAPS_mxrx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x29 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'ALIGNMENT|AVX_TYPE_1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX requires_alignment=1',
  tags       => 'page=MOVAPS',
};

ENCODING VMOVAPS_myry_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x29 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MOVAPS',
};

ENCODING VMOVAPS_myry_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x29 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_256 requires_alignment=1',
  tags       => 'page=MOVAPS',
};

ENCODING VMOVAPS_mzrz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x29 P66=0 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MOVAPS',
};

ENCODING VMOVAPS_mzrz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x29 P66=0 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_512 requires_alignment=1',
  tags       => 'page=MOVAPS',
};

ENCODING VMOVAPS_romo_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x28 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MOVAPS',
};

ENCODING VMOVAPS_romo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x28 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_128 requires_alignment=1',
  tags       => 'page=MOVAPS',
};

ENCODING VMOVAPS_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x28 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'ALIGNMENT|AVX_TYPE_1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX requires_alignment=1',
  tags       => 'page=MOVAPS',
};

ENCODING VMOVAPS_rymy_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x28 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MOVAPS',
};

ENCODING VMOVAPS_rymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x28 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_256 requires_alignment=1',
  tags       => 'page=MOVAPS',
};

ENCODING VMOVAPS_rzmz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x28 P66=0 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MOVAPS',
};

ENCODING VMOVAPS_rzmz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x28 P66=0 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_512 requires_alignment=1',
  tags       => 'page=MOVAPS',
};


# MOVDDUP-Replicate Double FP Values.
ENCODING VMOVDDUP_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x12 P66=0 PF2=1 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5NF',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MOVDDUP',
};

ENCODING VMOVDDUP_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x12 P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVDDUP',
};

ENCODING VMOVDDUP_rymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x12 P66=0 PF2=1 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5NF',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MOVDDUP',
};

ENCODING VMOVDDUP_rzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x12 P66=0 PF2=1 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5NF',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MOVDDUP',
};


# MOVDQA,VMOVDQA32/64-Move Aligned Packed Integer Values.
ENCODING VMOVDQA_mxrx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x7f P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'ALIGNMENT|AVX_TYPE_1',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX requires_alignment=1',
  tags       => 'page=MOVDQA',
};

ENCODING VMOVDQA_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x6f P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'ALIGNMENT|AVX_TYPE_1',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX requires_alignment=1',
  tags       => 'page=MOVDQA',
};

ENCODING VMOVDQAxx_moro_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x7f P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MOVDQA',
};

ENCODING VMOVDQAxx_moro_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x7f P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_128 requires_alignment=1',
  tags       => 'page=MOVDQA',
};

ENCODING VMOVDQAxx_myry_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x7f P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MOVDQA',
};

ENCODING VMOVDQAxx_myry_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x7f P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_256 requires_alignment=1',
  tags       => 'page=MOVDQA',
};

ENCODING VMOVDQAxx_mzrz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x7f P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MOVDQA',
};

ENCODING VMOVDQAxx_mzrz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x7f P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_512 requires_alignment=1',
  tags       => 'page=MOVDQA',
};

ENCODING VMOVDQAxx_romo_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x6f P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MOVDQA',
};

ENCODING VMOVDQAxx_romo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x6f P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_128 requires_alignment=1',
  tags       => 'page=MOVDQA',
};

ENCODING VMOVDQAxx_rymy_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x6f P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MOVDQA',
};

ENCODING VMOVDQAxx_rymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x6f P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_256 requires_alignment=1',
  tags       => 'page=MOVDQA',
};

ENCODING VMOVDQAxx_rzmz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x6f P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MOVDQA',
};

ENCODING VMOVDQAxx_rzmz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x6f P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_512 requires_alignment=1',
  tags       => 'page=MOVDQA',
};


# MOVDQU,VMOVDQU8/16/32/64-Move Unaligned Packed Integer Values.
ENCODING VMOVDQU_mxrx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x7f P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4M',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVDQU',
};

ENCODING VMOVDQU_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x6f P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4M',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVDQU',
};

ENCODING VMOVDQUx_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x7f P66=0 PF2=1 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=MOVDQU',
};

ENCODING VMOVDQUx_myry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x7f P66=0 PF2=1 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=MOVDQU',
};

ENCODING VMOVDQUx_mzrz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x7f P66=0 PF2=1 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=MOVDQU',
};

ENCODING VMOVDQUx_romo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x6f P66=0 PF2=1 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=MOVDQU',
};

ENCODING VMOVDQUx_rymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x6f P66=0 PF2=1 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=MOVDQU',
};

ENCODING VMOVDQUx_rzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x6f P66=0 PF2=1 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=MOVDQU',
};

ENCODING VMOVDQUxx_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x7f P66=0 PF2=0 PF3=1 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MOVDQU',
};

ENCODING VMOVDQUxx_myry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x7f P66=0 PF2=0 PF3=1 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MOVDQU',
};

ENCODING VMOVDQUxx_mzrz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x7f P66=0 PF2=0 PF3=1 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MOVDQU',
};

ENCODING VMOVDQUxx_romo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x6f P66=0 PF2=0 PF3=1 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MOVDQU',
};

ENCODING VMOVDQUxx_rymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x6f P66=0 PF2=0 PF3=1 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MOVDQU',
};

ENCODING VMOVDQUxx_rzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x6f P66=0 PF2=0 PF3=1 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MOVDQU',
};


# MOVHLPS-Move Packed Single-Precision Floating-Point Values High to Low.
ENCODING VMOVHLPS_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x12 P66=0 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVHLPS',
};

ENCODING VMOVHLPS_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x12 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM128',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=MOVHLPS',
};


# MOVHPD-Move High Packed Double-Precision Floating-Point Value.
ENCODING VMOVHPD_mqro_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0x17 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVHPD',
};

ENCODING VMOVHPD_mqro_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x17 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=MOVHPD',
};

ENCODING VMOVHPD_ronqmq_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0x16 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVHPD',
};

ENCODING VMOVHPD_ronqmq_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x16 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=MOVHPD',
};


# MOVHPS-Move High Packed Single-Precision Floating-Point Values.
ENCODING VMOVHPS_mqro_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0x17 P66=0 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVHPS',
};

ENCODING VMOVHPS_mqro_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x17 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=MOVHPS',
};

ENCODING VMOVHPS_ronqmq_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0x16 P66=0 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVHPS',
};

ENCODING VMOVHPS_ronqmq_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x16 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=MOVHPS',
};


# MOVLHPS-Move Packed Single-Precision Floating-Point Values Low to High.
ENCODING VMOVLHPS_ronqmq_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x16 P66=0 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVLHPS',
};

ENCODING VMOVLHPS_ronqmq_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x16 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM128',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=MOVLHPS',
};


# MOVLPD-Move Low Packed Double-Precision Floating-Point Value.
ENCODING VMOVLPD_mqrq_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0x13 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVLPD',
};

ENCODING VMOVLPD_mqrq_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x13 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=MOVLPD',
};

ENCODING VMOVLPD_ronomq_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0x12 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVLPD',
};

ENCODING VMOVLPD_ronomq_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x12 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=MOVLPD',
};


# MOVLPS-Move Low Packed Single-Precision Floating-Point Values.
ENCODING VMOVLPS_mqrq_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0x13 P66=0 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVLPS',
};

ENCODING VMOVLPS_mqrq_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x13 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=MOVLPS',
};

ENCODING VMOVLPS_ronomq_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0x12 P66=0 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVLPS',
};

ENCODING VMOVLPS_ronomq_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x12 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=MOVLPS',
};


# MOVMSKPD-Extract Packed Double-Precision Floating-Point Sign Mask.
ENCODING VMOVMSKPD => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x50 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVMSKPD',
};


# MOVMSKPS-Extract Packed Single-Precision Floating-Point Sign Mask.
ENCODING VMOVMSKPS => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x50 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVMSKPS',
};


# MOVNTDQ-Store Packed Integers Using Non-Temporal Hint.
ENCODING VMOVNTDQ_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0xe7 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1NF',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_128 requires_alignment=1',
  tags       => 'page=MOVNTDQ',
};

ENCODING VMOVNTDQ_mxrx => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0xe7 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'ALIGNMENT|AVX_TYPE_1',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX requires_alignment=1',
  tags       => 'page=MOVNTDQ',
};

ENCODING VMOVNTDQ_myry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0xe7 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1NF',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_256 requires_alignment=1',
  tags       => 'page=MOVNTDQ',
};

ENCODING VMOVNTDQ_mzrz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0xe7 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1NF',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_512 requires_alignment=1',
  tags       => 'page=MOVNTDQ',
};


# MOVNTDQA-Load Double Quadword Non-Temporal Aligned Hint.
ENCODING VMOVNTDQA_romo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MOD=MEM MR=1 OP=0x2a P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'ALIGNMENT|AVX_TYPE_1',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX requires_alignment=1',
  tags       => 'page=MOVNTDQA',
};

ENCODING VMOVNTDQA_romo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x2a P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1NF',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_128 requires_alignment=1',
  tags       => 'page=MOVNTDQA',
};

ENCODING VMOVNTDQA_rymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MOD=MEM MR=1 OP=0x2a P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'ALIGNMENT|AVX_TYPE_1',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX2 requires_alignment=1',
  tags       => 'page=MOVNTDQA',
};

ENCODING VMOVNTDQA_rymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x2a P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1NF',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_256 requires_alignment=1',
  tags       => 'page=MOVNTDQA',
};

ENCODING VMOVNTDQA_rzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x2a P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1NF',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_512 requires_alignment=1',
  tags       => 'page=MOVNTDQA',
};


# MOVNTPD-Store Packed Double-Precision Floating-Point Values Using Non-Temporal Hint.
ENCODING VMOVNTPD_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x2b P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1NF',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_128 requires_alignment=1',
  tags       => 'page=MOVNTPD',
};

ENCODING VMOVNTPD_mxrx => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0x2b P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'ALIGNMENT|AVX_TYPE_1',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX requires_alignment=1',
  tags       => 'page=MOVNTPD',
};

ENCODING VMOVNTPD_myry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x2b P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1NF',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_256 requires_alignment=1',
  tags       => 'page=MOVNTPD',
};

ENCODING VMOVNTPD_mzrz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x2b P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1NF',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_512 requires_alignment=1',
  tags       => 'page=MOVNTPD',
};


# MOVNTPS-Store Packed Single-Precision Floating-Point Values Using Non-Temporal Hint.
ENCODING VMOVNTPS_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x2b P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1NF',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_128 requires_alignment=1',
  tags       => 'page=MOVNTPS',
};

ENCODING VMOVNTPS_mxrx => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0x2b P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'ALIGNMENT|AVX_TYPE_1',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX requires_alignment=1',
  tags       => 'page=MOVNTPS',
};

ENCODING VMOVNTPS_myry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x2b P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1NF',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_256 requires_alignment=1',
  tags       => 'page=MOVNTPS',
};

ENCODING VMOVNTPS_mzrz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x2b P66=0 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'ALIGNMENT|AVX512_E1NF',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_512 requires_alignment=1',
  tags       => 'page=MOVNTPS',
};


# MOVQ-Move Quadword.
ENCODING VMOVQ_mqro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MODE=64 MR=1 OP=0x7e P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=MOVQ',
};

ENCODING VMOVQ_mqrq => {
  diagram    => 'ENC=VEX MAP=0f MODE=64 MR=1 OP=0x7e P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVQ',
};

ENCODING VMOVQ_mxro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd6 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=MOVQ',
};

ENCODING VMOVQ_mxrq => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xd6 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVQ',
};

ENCODING VMOVQ_romq_1 => {
  diagram    => 'ENC=VEX MAP=0f MODE=64 MR=1 OP=0x6e P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVQ',
};

ENCODING VMOVQ_romq_2 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x7e P66=0 PF2=0 PF3=1 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVQ',
};

ENCODING VMOVQ_romq_3 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MODE=64 MR=1 OP=0x6e P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=MOVQ',
};

ENCODING VMOVQ_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x7e P66=0 PF2=0 PF3=1 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=MOVQ',
};


# MOVS/MOVSB/MOVSW/MOVSD/MOVSQ-Move Data from String to String.
ENCODING VMOVSD_monoro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x11 P66=0 PF2=1 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=MOVS',
};

ENCODING VMOVSD_monorq => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x11 P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVS',
};

ENCODING VMOVSD_mqro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x11 P66=0 PF2=1 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=MOVS',
};

ENCODING VMOVSD_mqrq => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0x11 P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVS',
};

ENCODING VMOVSD_romq_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0x10 P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVS',
};

ENCODING VMOVSD_romq_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x10 P66=0 PF2=1 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=MOVS',
};

ENCODING VMOVSD_ronomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x10 P66=0 PF2=1 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=MOVS',
};

ENCODING VMOVSD_ronomq => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x10 P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVS',
};


# MOVSHDUP-Replicate Single FP Values.
ENCODING VMOVSHDUP_romo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x16 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MOVSHDUP',
};

ENCODING VMOVSHDUP_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x16 P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVSHDUP',
};

ENCODING VMOVSHDUP_rymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x16 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MOVSHDUP',
};

ENCODING VMOVSHDUP_rzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x16 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MOVSHDUP',
};


# MOVSLDUP-Replicate Single FP Values.
ENCODING VMOVSLDUP_romo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x12 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MOVSLDUP',
};

ENCODING VMOVSLDUP_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x12 P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVSLDUP',
};

ENCODING VMOVSLDUP_rymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x12 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MOVSLDUP',
};

ENCODING VMOVSLDUP_rzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x12 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MOVSLDUP',
};


# MOVSS-Move or Merge Scalar Single-Precision Floating-Point Value.
ENCODING VMOVSS_mdrd => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0x11 P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVSS',
};

ENCODING VMOVSS_mdro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x11 P66=0 PF2=0 PF3=1 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=MOVSS',
};

ENCODING VMOVSS_monord => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x11 P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVSS',
};

ENCODING VMOVSS_monoro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x11 P66=0 PF2=0 PF3=1 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=MOVSS',
};

ENCODING VMOVSS_romd_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0x10 P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVSS',
};

ENCODING VMOVSS_romd_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=MEM MR=1 OP=0x10 P66=0 PF2=0 PF3=1 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=MOVSS',
};

ENCODING VMOVSS_ronomd => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x10 P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVSS',
};

ENCODING VMOVSS_ronomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0x10 P66=0 PF2=0 PF3=1 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=MOVSS',
};


# MOVUPD-Move Unaligned Packed Double-Precision Floating-Point Values.
ENCODING VMOVUPD_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x11 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MOVUPD',
};

ENCODING VMOVUPD_mxrx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x11 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4M',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVUPD',
};

ENCODING VMOVUPD_myry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x11 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MOVUPD',
};

ENCODING VMOVUPD_mzrz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x11 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MOVUPD',
};

ENCODING VMOVUPD_romo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x10 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MOVUPD',
};

ENCODING VMOVUPD_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x10 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4M',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVUPD',
};

ENCODING VMOVUPD_rymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x10 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MOVUPD',
};

ENCODING VMOVUPD_rzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x10 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MOVUPD',
};


# MOVUPS-Move Unaligned Packed Single-Precision Floating-Point Values.
ENCODING VMOVUPS_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x11 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MOVUPS',
};

ENCODING VMOVUPS_mxrx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x11 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4M',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVUPS',
};

ENCODING VMOVUPS_myry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x11 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MOVUPS',
};

ENCODING VMOVUPS_mzrz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x11 P66=0 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MOVUPS',
};

ENCODING VMOVUPS_romo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x10 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MOVUPS',
};

ENCODING VMOVUPS_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x10 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4M',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVUPS',
};

ENCODING VMOVUPS_rymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x10 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MOVUPS',
};

ENCODING VMOVUPS_rzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x10 P66=0 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MOVUPS',
};


# MOVD/MOVQ-Move Doubleword/Move Quadword.
ENCODING VMOVD_mdrd => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x7e P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVx',
};

ENCODING VMOVD_mdro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x7e P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=MOVx',
};

ENCODING VMOVD_romd_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x6e P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=MOVx',
};

ENCODING VMOVD_romd_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x6e P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_128N',
  tags       => 'page=MOVx',
};


# MPSADBW-Compute Multiple Packed Sums of Absolute Difference.
ENCODING VMPSADBW_ronomoub => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x42 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=MPSADBW',
};

ENCODING VMPSADBW_rynymyub => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x42 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=MPSADBW',
};


# MULPD-Multiply Packed Double-Precision Floating-Point Values.
ENCODING VMULPD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x59 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MULPD',
};

ENCODING VMULPD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x59 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=MULPD',
};

ENCODING VMULPD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x59 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MULPD',
};

ENCODING VMULPD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x59 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MULPD',
};


# MULPS-Multiply Packed Single-Precision Floating-Point Values.
ENCODING VMULPS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x59 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=MULPS',
};

ENCODING VMULPS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x59 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=MULPS',
};

ENCODING VMULPS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x59 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=MULPS',
};

ENCODING VMULPS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x59 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=MULPS',
};


# MULSD-Multiply Scalar Double-Precision Floating-Point Value.
ENCODING VMULSD_ronomq => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x59 P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=MULSD',
};

ENCODING VMULSD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x59 P66=0 PF2=1 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=MULSD',
};


# MULSS-Multiply Scalar Single-Precision Floating-Point Values.
ENCODING VMULSS_ronomd => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x59 P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=MULSS',
};

ENCODING VMULSS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x59 P66=0 PF2=0 PF3=1 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=MULSS',
};


# ORPD-Bitwise Logical OR of Packed Double Precision Floating-Point Values.
ENCODING VORPD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x56 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=ORPD',
};

ENCODING VORPD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x56 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX',
  tags       => 'page=ORPD',
};

ENCODING VORPD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x56 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=ORPD',
};

ENCODING VORPD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x56 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=ORPD',
};


# ORPS-Bitwise Logical OR of Packed Single Precision Floating-Point Values.
ENCODING VORPS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x56 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=ORPS',
};

ENCODING VORPS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x56 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX',
  tags       => 'page=ORPS',
};

ENCODING VORPS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x56 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=ORPS',
};

ENCODING VORPS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x56 P66=0 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=ORPS',
};


# PABSB/PABSW/PABSD/PABSQ-Packed Absolute Value.
ENCODING VPABSB_romo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x1c P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PABSx',
};

ENCODING VPABSB_romo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x1c P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PABSx',
};

ENCODING VPABSB_rymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x1c P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PABSx',
};

ENCODING VPABSB_rymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x1c P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PABSx',
};

ENCODING VPABSB_rzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x1c P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PABSx',
};

ENCODING VPABSD_romo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x1e P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PABSx',
};

ENCODING VPABSD_romo_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x1e P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PABSx',
};

ENCODING VPABSD_rymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x1e P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PABSx',
};

ENCODING VPABSD_rymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x1e P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PABSx',
};

ENCODING VPABSD_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x1e P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PABSx',
};

ENCODING VPABSQ_romo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x1f P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PABSx',
};

ENCODING VPABSQ_rymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x1f P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PABSx',
};

ENCODING VPABSQ_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x1f P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PABSx',
};

ENCODING VPABSW_romo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x1d P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PABSx',
};

ENCODING VPABSW_romo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x1d P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PABSx',
};

ENCODING VPABSW_rymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x1d P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PABSx',
};

ENCODING VPABSW_rymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x1d P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PABSx',
};

ENCODING VPABSW_rzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x1d P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PABSx',
};


# PACKSSWB/PACKSSDW-Pack with Signed Saturation.
ENCODING VPACKSSDW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x6b P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PACKSSxx',
};

ENCODING VPACKSSDW_ronomo_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x6b P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PACKSSxx',
};

ENCODING VPACKSSDW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x6b P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PACKSSxx',
};

ENCODING VPACKSSDW_rynymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x6b P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PACKSSxx',
};

ENCODING VPACKSSDW_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x6b P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PACKSSxx',
};

ENCODING VPACKSSWB_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x63 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PACKSSxx',
};

ENCODING VPACKSSWB_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x63 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PACKSSxx',
};

ENCODING VPACKSSWB_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x63 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PACKSSxx',
};

ENCODING VPACKSSWB_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x63 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PACKSSxx',
};

ENCODING VPACKSSWB_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x63 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PACKSSxx',
};


# PACKUSDW-Pack with Unsigned Saturation.
ENCODING VPACKUSDW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x2b P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PACKUSDW',
};

ENCODING VPACKUSDW_ronomo_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x2b P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PACKUSDW',
};

ENCODING VPACKUSDW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x2b P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PACKUSDW',
};

ENCODING VPACKUSDW_rynymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x2b P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PACKUSDW',
};

ENCODING VPACKUSDW_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x2b P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PACKUSDW',
};


# PACKUSWB-Pack with Unsigned Saturation.
ENCODING VPACKUSWB_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x67 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PACKUSWB',
};

ENCODING VPACKUSWB_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x67 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PACKUSWB',
};

ENCODING VPACKUSWB_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x67 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PACKUSWB',
};

ENCODING VPACKUSWB_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x67 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PACKUSWB',
};

ENCODING VPACKUSWB_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x67 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'SYSTEM|CONVERSION|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PACKUSWB',
};


# PADDSB/PADDSW-Add Packed Signed Integers with Signed Saturation.
ENCODING VPADDSB_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xec P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PADDSx',
};

ENCODING VPADDSB_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xec P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PADDSx',
};

ENCODING VPADDSB_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xec P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PADDSx',
};

ENCODING VPADDSB_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xec P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PADDSx',
};

ENCODING VPADDSB_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xec P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PADDSx',
};

ENCODING VPADDSW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xed P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PADDSx',
};

ENCODING VPADDSW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xed P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PADDSx',
};

ENCODING VPADDSW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xed P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PADDSx',
};

ENCODING VPADDSW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xed P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PADDSx',
};

ENCODING VPADDSW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xed P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PADDSx',
};


# PADDUSB/PADDUSW-Add Packed Unsigned Integers with Unsigned Saturation.
ENCODING VPADDUSB_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xdc P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PADDUSx',
};

ENCODING VPADDUSB_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xdc P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PADDUSx',
};

ENCODING VPADDUSB_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xdc P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PADDUSx',
};

ENCODING VPADDUSB_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xdc P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PADDUSx',
};

ENCODING VPADDUSB_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xdc P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PADDUSx',
};

ENCODING VPADDUSW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xdd P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PADDUSx',
};

ENCODING VPADDUSW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xdd P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PADDUSx',
};

ENCODING VPADDUSW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xdd P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PADDUSx',
};

ENCODING VPADDUSW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xdd P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PADDUSx',
};

ENCODING VPADDUSW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xdd P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PADDUSx',
};


# PADDB/PADDW/PADDD/PADDQ-Add Packed Integers.
ENCODING VPADDB_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xfc P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PADDx',
};

ENCODING VPADDB_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xfc P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PADDx',
};

ENCODING VPADDB_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xfc P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PADDx',
};

ENCODING VPADDB_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xfc P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PADDx',
};

ENCODING VPADDB_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xfc P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PADDx',
};

ENCODING VPADDD_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xfe P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PADDx',
};

ENCODING VPADDD_ronomo_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xfe P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PADDx',
};

ENCODING VPADDD_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xfe P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PADDx',
};

ENCODING VPADDD_rynymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xfe P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PADDx',
};

ENCODING VPADDD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xfe P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PADDx',
};

ENCODING VPADDQ_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xd4 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PADDx',
};

ENCODING VPADDQ_ronomo_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xd4 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PADDx',
};

ENCODING VPADDQ_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xd4 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PADDx',
};

ENCODING VPADDQ_rynymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xd4 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PADDx',
};

ENCODING VPADDQ_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xd4 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PADDx',
};

ENCODING VPADDW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xfd P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PADDx',
};

ENCODING VPADDW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xfd P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PADDx',
};

ENCODING VPADDW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xfd P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PADDx',
};

ENCODING VPADDW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xfd P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PADDx',
};

ENCODING VPADDW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xfd P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PADDx',
};


# PALIGNR-Packed Align Right.
ENCODING VPALIGNR_ronomoub_1 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x0f P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PALIGNR',
};

ENCODING VPALIGNR_ronomoub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x0f P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PALIGNR',
};

ENCODING VPALIGNR_rynymyub_1 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x0f P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PALIGNR',
};

ENCODING VPALIGNR_rynymyub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x0f P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PALIGNR',
};

ENCODING VPALIGNR_rznzmzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x0f P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PALIGNR',
};


# PAND-Logical AND.
ENCODING VPAND_ronomo => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xdb P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PAND',
};

ENCODING VPAND_rynymy => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xdb P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=PAND',
};


# PANDN-Logical AND NOT.
ENCODING VPANDN_ronomo => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xdf P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PANDN',
};

ENCODING VPANDN_rynymy => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xdf P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=PANDN',
};


# PAVGB/PAVGW-Average Packed Integers.
ENCODING VPAVGB_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe0 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PAVGx',
};

ENCODING VPAVGB_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe0 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PAVGx',
};

ENCODING VPAVGB_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe0 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PAVGx',
};

ENCODING VPAVGB_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe0 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PAVGx',
};

ENCODING VPAVGB_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe0 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PAVGx',
};

ENCODING VPAVGW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe3 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PAVGx',
};

ENCODING VPAVGW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe3 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PAVGx',
};

ENCODING VPAVGW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe3 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PAVGx',
};

ENCODING VPAVGW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe3 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PAVGx',
};

ENCODING VPAVGW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe3 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PAVGx',
};


# PBLENDVB-Variable Blend Packed Bytes.
ENCODING VPBLENDVB_ronomois4o => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x4c P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX',
  tags       => 'page=PBLENDVB',
};

ENCODING VPBLENDVB_rynymyis4y => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x4c P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX2',
  tags       => 'page=PBLENDVB',
};


# PBLENDW-Blend Packed Words.
ENCODING VPBLENDW_ronomoub => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x0e P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX',
  tags       => 'page=PBLENDW',
};

ENCODING VPBLENDW_rynymyub => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x0e P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX2',
  tags       => 'page=PBLENDW',
};


# PCLMULQDQ-Carry-Less Multiplication Quadword.
ENCODING VPCLMULQDQ_ronomoub_1 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x44 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|APP',
  metadata   => 'isa=AVX',
  tags       => 'page=PCLMULQDQ',
};

ENCODING VPCLMULQDQ_ronomoub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x44 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|APP',
  metadata   => 'isa=AVX512_VPCLMULQDQ_128',
  tags       => 'page=PCLMULQDQ',
};

ENCODING VPCLMULQDQ_rynymyub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x44 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|APP',
  metadata   => 'isa=AVX512_VPCLMULQDQ_256',
  tags       => 'page=PCLMULQDQ',
};

ENCODING VPCLMULQDQ_rznzmzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x44 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|APP',
  metadata   => 'isa=AVX512_VPCLMULQDQ_512',
  tags       => 'page=PCLMULQDQ',
};


# PCMPEQQ-Compare Packed Qword Data for Equal.
ENCODING VPCMPEQQ_nomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x29 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PCMPEQQ',
};

ENCODING VPCMPEQQ_nymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x29 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PCMPEQQ',
};

ENCODING VPCMPEQQ_nzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x29 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PCMPEQQ',
};

ENCODING VPCMPEQQ_ronomo => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x29 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX',
  tags       => 'page=PCMPEQQ',
};

ENCODING VPCMPEQQ_rynymy => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x29 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX2',
  tags       => 'page=PCMPEQQ',
};


# PCMPEQB/PCMPEQW/PCMPEQD-Compare Packed Data for Equal.
ENCODING VPCMPEQB_nomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x74 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PCMPEQx',
};

ENCODING VPCMPEQB_nymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x74 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PCMPEQx',
};

ENCODING VPCMPEQB_nzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x74 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PCMPEQx',
};

ENCODING VPCMPEQB_ronomo => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x74 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX',
  tags       => 'page=PCMPEQx',
};

ENCODING VPCMPEQB_rynymy => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x74 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX2',
  tags       => 'page=PCMPEQx',
};

ENCODING VPCMPEQD_nomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x76 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PCMPEQx',
};

ENCODING VPCMPEQD_nymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x76 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PCMPEQx',
};

ENCODING VPCMPEQD_nzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x76 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PCMPEQx',
};

ENCODING VPCMPEQD_ronomo => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x76 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX',
  tags       => 'page=PCMPEQx',
};

ENCODING VPCMPEQD_rynymy => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x76 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX2',
  tags       => 'page=PCMPEQx',
};

ENCODING VPCMPEQW_nomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x75 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PCMPEQx',
};

ENCODING VPCMPEQW_nymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x75 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PCMPEQx',
};

ENCODING VPCMPEQW_nzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x75 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PCMPEQx',
};

ENCODING VPCMPEQW_ronomo => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x75 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX',
  tags       => 'page=PCMPEQx',
};

ENCODING VPCMPEQW_rynymy => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x75 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX2',
  tags       => 'page=PCMPEQx',
};


# PCMPESTRI-Packed Compare Explicit Length Strings, Return Index.
ENCODING VPCMPESTRI => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x61 P66=1 PF2=0 PF3=0 VL=128',
  iflags     => 'af=Z cf=W of=W pf=Z sf=W zf=W',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=PCMPESTRI',
};


# PCMPESTRM-Packed Compare Explicit Length Strings, Return Mask.
ENCODING VPCMPESTRM => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x60 P66=1 PF2=0 PF3=0 VL=128',
  iflags     => 'af=Z cf=W of=W pf=Z sf=W zf=W',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=PCMPESTRM',
};


# PCMPGTQ-Compare Packed Data for Greater Than.
ENCODING VPCMPGTQ_nomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x37 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PCMPGTQ',
};

ENCODING VPCMPGTQ_nymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x37 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PCMPGTQ',
};

ENCODING VPCMPGTQ_nzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x37 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PCMPGTQ',
};

ENCODING VPCMPGTQ_ronomo => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x37 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX',
  tags       => 'page=PCMPGTQ',
};

ENCODING VPCMPGTQ_rynymy => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x37 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX2',
  tags       => 'page=PCMPGTQ',
};


# PCMPGTB/PCMPGTW/PCMPGTD-Compare Packed Signed Integers for Greater Than.
ENCODING VPCMPGTB_nomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x64 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PCMPGTx',
};

ENCODING VPCMPGTB_nymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x64 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PCMPGTx',
};

ENCODING VPCMPGTB_nzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x64 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PCMPGTx',
};

ENCODING VPCMPGTB_ronomo => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x64 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX',
  tags       => 'page=PCMPGTx',
};

ENCODING VPCMPGTB_rynymy => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x64 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX2',
  tags       => 'page=PCMPGTx',
};

ENCODING VPCMPGTD_nomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x66 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PCMPGTx',
};

ENCODING VPCMPGTD_nymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x66 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PCMPGTx',
};

ENCODING VPCMPGTD_nzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x66 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PCMPGTx',
};

ENCODING VPCMPGTD_ronomo => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x66 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX',
  tags       => 'page=PCMPGTx',
};

ENCODING VPCMPGTD_rynymy => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x66 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX2',
  tags       => 'page=PCMPGTx',
};

ENCODING VPCMPGTW_nomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x65 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PCMPGTx',
};

ENCODING VPCMPGTW_nymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x65 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PCMPGTx',
};

ENCODING VPCMPGTW_nzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x65 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PCMPGTx',
};

ENCODING VPCMPGTW_ronomo => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x65 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX',
  tags       => 'page=PCMPGTx',
};

ENCODING VPCMPGTW_rynymy => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x65 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX2',
  tags       => 'page=PCMPGTx',
};


# PCMPISTRI-Packed Compare Implicit Length Strings, Return Index.
ENCODING VPCMPISTRI => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x63 P66=1 PF2=0 PF3=0 VL=128',
  iflags     => 'af=Z cf=W of=W pf=Z sf=W zf=W',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=PCMPISTRI',
};


# PCMPISTRM-Packed Compare Implicit Length Strings, Return Mask.
ENCODING VPCMPISTRM => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x62 P66=1 PF2=0 PF3=0 VL=128',
  iflags     => 'af=Z cf=W of=W pf=Z sf=W zf=W',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=PCMPISTRM',
};


# PEXTRW-Extract Word.
ENCODING VPEXTRW_mxroub_1 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x15 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=PEXTRW',
};

ENCODING VPEXTRW_mxroub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x15 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512BW_128N',
  tags       => 'page=PEXTRW',
};

ENCODING VPEXTRW_rdmoub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0xc5 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=PEXTRW',
};

ENCODING VPEXTRW_rdmoub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MOD=REG MR=1 OP=0xc5 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512BW_128N',
  tags       => 'page=PEXTRW',
};


# PEXTRB/PEXTRD/PEXTRQ-Extract Byte/Dword/Qword.
ENCODING VPEXTRB_mxroub_1 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x14 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=PEXTRx',
};

ENCODING VPEXTRB_mxroub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x14 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512BW_128N',
  tags       => 'page=PEXTRx',
};

ENCODING VPEXTRx_mxroub_1 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x16 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=PEXTRx',
};

ENCODING VPEXTRx_mxroub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x16 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512DQ_128N',
  tags       => 'page=PEXTRx',
};


# PHADDSW-Packed Horizontal Add and Saturate.
ENCODING VPHADDSW_ronomo => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x03 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PHADDSW',
};

ENCODING VPHADDSW_rynymy => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x03 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PHADDSW',
};


# PHADDW/PHADDD-Packed Horizontal Add.
ENCODING VPHADDD_ronomo => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x02 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PHADDx',
};

ENCODING VPHADDD_rynymy => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x02 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PHADDx',
};

ENCODING VPHADDW_ronomo => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x01 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PHADDx',
};

ENCODING VPHADDW_rynymy => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x01 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PHADDx',
};


# PHMINPOSUW-Packed Horizontal Word Minimum.
ENCODING VPHMINPOSUW => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x41 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=PHMINPOSUW',
};


# PHSUBSW-Packed Horizontal Subtract and Saturate.
ENCODING VPHSUBSW_ronomo => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x07 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PHSUBSW',
};

ENCODING VPHSUBSW_rynymy => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x07 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PHSUBSW',
};


# PHSUBW/PHSUBD-Packed Horizontal Subtract.
ENCODING VPHSUBD_ronomo => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x06 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PHSUBx',
};

ENCODING VPHSUBD_rynymy => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x06 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PHSUBx',
};

ENCODING VPHSUBW_ronomo => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x05 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PHSUBx',
};

ENCODING VPHSUBW_rynymy => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x05 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PHSUBx',
};


# PINSRW-Insert Word.
ENCODING VPINSRW_ronomxub_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xc4 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=PINSRW',
};

ENCODING VPINSRW_ronomxub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xc4 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512BW_128N',
  tags       => 'page=PINSRW',
};


# PINSRB/PINSRD/PINSRQ-Insert Byte/Dword/Qword.
ENCODING VPINSRB_ronomxub_1 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x20 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=PINSRx',
};

ENCODING VPINSRB_ronomxub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x20 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512BW_128N',
  tags       => 'page=PINSRx',
};

ENCODING VPINSRx_ronomxub_1 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x22 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=PINSRx',
};

ENCODING VPINSRx_ronomxub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x22 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E9NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512DQ_128N',
  tags       => 'page=PINSRx',
};


# PMADDUBSW-Multiply and Add Packed Signed and Unsigned Bytes.
ENCODING VPMADDUBSW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x04 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PMADDUBSW',
};

ENCODING VPMADDUBSW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x04 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PMADDUBSW',
};

ENCODING VPMADDUBSW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x04 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMADDUBSW',
};

ENCODING VPMADDUBSW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x04 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PMADDUBSW',
};

ENCODING VPMADDUBSW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x04 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PMADDUBSW',
};


# PMADDWD-Multiply and Add Packed Integers.
ENCODING VPMADDWD_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xf5 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PMADDWD',
};

ENCODING VPMADDWD_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf5 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PMADDWD',
};

ENCODING VPMADDWD_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xf5 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMADDWD',
};

ENCODING VPMADDWD_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf5 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PMADDWD',
};

ENCODING VPMADDWD_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf5 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PMADDWD',
};


# PMAXSB/PMAXSW/PMAXSD/PMAXSQ-Maximum of Packed Signed Integers.
ENCODING VPMAXSB_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x3c P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=PMAXSx',
};

ENCODING VPMAXSB_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x3c P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PMAXSx',
};

ENCODING VPMAXSB_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x3c P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMAXSx',
};

ENCODING VPMAXSB_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x3c P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PMAXSx',
};

ENCODING VPMAXSB_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x3c P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PMAXSx',
};

ENCODING VPMAXSD_ronomo => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x3d P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=PMAXSx',
};

ENCODING VPMAXSD_rynymy => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x3d P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMAXSx',
};

ENCODING VPMAXSW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xee P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=PMAXSx',
};

ENCODING VPMAXSW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xee P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PMAXSx',
};

ENCODING VPMAXSW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xee P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMAXSx',
};

ENCODING VPMAXSW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xee P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PMAXSx',
};

ENCODING VPMAXSW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xee P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PMAXSx',
};

ENCODING VPMAXSx_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x3d P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PMAXSx',
};

ENCODING VPMAXSx_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x3d P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PMAXSx',
};

ENCODING VPMAXSx_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x3d P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PMAXSx',
};


# PMAXUB/PMAXUW-Maximum of Packed Unsigned Integers.
ENCODING VPMAXUB_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xde P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=PMAXUbw',
};

ENCODING VPMAXUB_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xde P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PMAXUbw',
};

ENCODING VPMAXUB_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xde P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMAXUbw',
};

ENCODING VPMAXUB_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xde P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PMAXUbw',
};

ENCODING VPMAXUB_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xde P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PMAXUbw',
};

ENCODING VPMAXUW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x3e P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=PMAXUbw',
};

ENCODING VPMAXUW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x3e P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PMAXUbw',
};

ENCODING VPMAXUW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x3e P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMAXUbw',
};

ENCODING VPMAXUW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x3e P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PMAXUbw',
};

ENCODING VPMAXUW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x3e P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PMAXUbw',
};


# PMAXUD/PMAXUQ-Maximum of Packed Unsigned Integers.
ENCODING VPMAXUD_ronomo => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x3f P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=PMAXUy',
};

ENCODING VPMAXUD_rynymy => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x3f P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMAXUy',
};

ENCODING VPMAXUx_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x3f P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PMAXUy',
};

ENCODING VPMAXUx_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x3f P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PMAXUy',
};

ENCODING VPMAXUx_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x3f P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PMAXUy',
};


# PMINSB/PMINSW-Minimum of Packed Signed Integers.
ENCODING VPMINSB_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x38 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=PMINSbw',
};

ENCODING VPMINSB_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x38 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PMINSbw',
};

ENCODING VPMINSB_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x38 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMINSbw',
};

ENCODING VPMINSB_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x38 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PMINSbw',
};

ENCODING VPMINSB_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x38 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PMINSbw',
};

ENCODING VPMINSW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xea P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=PMINSbw',
};

ENCODING VPMINSW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xea P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PMINSbw',
};

ENCODING VPMINSW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xea P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMINSbw',
};

ENCODING VPMINSW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xea P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PMINSbw',
};

ENCODING VPMINSW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xea P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PMINSbw',
};


# PMINSD/PMINSQ-Minimum of Packed Signed Integers.
ENCODING VPMINSD_ronomo => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x39 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=PMINSy',
};

ENCODING VPMINSD_rynymy => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x39 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMINSy',
};

ENCODING VPMINSx_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x39 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PMINSy',
};

ENCODING VPMINSx_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x39 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PMINSy',
};

ENCODING VPMINSx_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x39 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PMINSy',
};


# PMINUB/PMINUW-Minimum of Packed Unsigned Integers.
ENCODING VPMINUB_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xda P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=PMINUbw',
};

ENCODING VPMINUB_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xda P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PMINUbw',
};

ENCODING VPMINUB_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xda P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMINUbw',
};

ENCODING VPMINUB_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xda P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PMINUbw',
};

ENCODING VPMINUB_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xda P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PMINUbw',
};

ENCODING VPMINUW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x3a P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=PMINUbw',
};

ENCODING VPMINUW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x3a P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PMINUbw',
};

ENCODING VPMINUW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x3a P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMINUbw',
};

ENCODING VPMINUW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x3a P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PMINUbw',
};

ENCODING VPMINUW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x3a P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PMINUbw',
};


# PMINUD/PMINUQ-Minimum of Packed Unsigned Integers.
ENCODING VPMINUD_ronomo => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x3b P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX',
  tags       => 'page=PMINUy',
};

ENCODING VPMINUD_rynymy => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x3b P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMINUy',
};

ENCODING VPMINUx_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x3b P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PMINUy',
};

ENCODING VPMINUx_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x3b P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PMINUy',
};

ENCODING VPMINUx_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x3b P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PMINUy',
};


# PMOVMSKB-Move Byte Mask.
ENCODING VPMOVMSKB_rdmo => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0xd7 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PMOVMSKB',
};

ENCODING VPMOVMSKB_rdmy => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0xd7 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMOVMSKB',
};


# PMOVSX-Packed Move with Sign Extend.
ENCODING VPMOVSXBD_romd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x21 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXBD_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x21 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXBD_rymq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x21 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXBD_rymx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x21 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXBD_rzmo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x21 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXBQ_romw => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x22 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXBQ_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x22 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXBQ_rymd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x22 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXBQ_rymx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x22 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXBQ_rzmx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x22 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXBW_romq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x20 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXBW_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x20 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXBW_rymo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x20 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXBW_rymo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x20 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXBW_rzmy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x20 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXDQ_romq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x25 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXDQ_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x25 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXDQ_rymo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x25 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXDQ_rymo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x25 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXDQ_rzmy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x25 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXWD_romq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x23 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXWD_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x23 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXWD_rymo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x23 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXWD_rymo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x23 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXWD_rzmy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x23 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXWQ_romd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x24 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXWQ_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x24 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXWQ_rymq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x24 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXWQ_rymx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x24 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PMOVSX',
};

ENCODING VPMOVSXWQ_rzmo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x24 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PMOVSX',
};


# PMOVZX-Packed Move with Zero Extend.
ENCODING VPMOVZXBD_romd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x31 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXBD_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x31 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXBD_rymq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x31 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXBD_rymx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x31 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXBD_rzmo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x31 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXBQ_romw => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x32 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXBQ_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x32 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXBQ_rymd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x32 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXBQ_rymx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x32 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXBQ_rzmx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x32 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXBW_romq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x30 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXBW_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x30 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXBW_rymo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x30 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXBW_rymo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x30 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXBW_rzmy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x30 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXDQ_romq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x35 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXDQ_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x35 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXDQ_rymo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x35 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXDQ_rymo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x35 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXDQ_rzmy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x35 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXWD_romq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x33 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXWD_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x33 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXWD_rymo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x33 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXWD_rymo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x33 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXWD_rzmy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x33 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXWQ_romd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x34 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXWQ_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x34 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXWQ_rymq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x34 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXWQ_rymx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x34 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PMOVZX',
};

ENCODING VPMOVZXWQ_rzmo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x34 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PMOVZX',
};


# PMULDQ-Multiply Packed Doubleword Integers.
ENCODING VPMULDQ_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x28 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PMULDQ',
};

ENCODING VPMULDQ_ronomo_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x28 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PMULDQ',
};

ENCODING VPMULDQ_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x28 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMULDQ',
};

ENCODING VPMULDQ_rynymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x28 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PMULDQ',
};

ENCODING VPMULDQ_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x28 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PMULDQ',
};


# PMULHRSW-Packed Multiply High with Round and Scale.
ENCODING VPMULHRSW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x0b P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PMULHRSW',
};

ENCODING VPMULHRSW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x0b P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PMULHRSW',
};

ENCODING VPMULHRSW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x0b P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMULHRSW',
};

ENCODING VPMULHRSW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x0b P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PMULHRSW',
};

ENCODING VPMULHRSW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x0b P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PMULHRSW',
};


# PMULHUW-Multiply Packed Unsigned Integers and Store High Result.
ENCODING VPMULHUW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe4 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PMULHUW',
};

ENCODING VPMULHUW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe4 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PMULHUW',
};

ENCODING VPMULHUW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe4 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMULHUW',
};

ENCODING VPMULHUW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe4 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PMULHUW',
};

ENCODING VPMULHUW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe4 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PMULHUW',
};


# PMULHW-Multiply Packed Signed Integers and Store High Result.
ENCODING VPMULHW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe5 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PMULHW',
};

ENCODING VPMULHW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe5 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PMULHW',
};

ENCODING VPMULHW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe5 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMULHW',
};

ENCODING VPMULHW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe5 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PMULHW',
};

ENCODING VPMULHW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe5 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PMULHW',
};


# PMULLW-Multiply Packed Signed Integers and Store Low Result.
ENCODING VPMULLW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xd5 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PMULLW',
};

ENCODING VPMULLW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd5 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PMULLW',
};

ENCODING VPMULLW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xd5 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMULLW',
};

ENCODING VPMULLW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd5 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PMULLW',
};

ENCODING VPMULLW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd5 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PMULLW',
};


# PMULLD/PMULLQ-Multiply Packed Integers and Store Low Result.
ENCODING VPMULLD_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x40 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PMULLx',
};

ENCODING VPMULLD_ronomo_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x40 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PMULLx',
};

ENCODING VPMULLD_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x40 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMULLx',
};

ENCODING VPMULLD_rynymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x40 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PMULLx',
};

ENCODING VPMULLD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x40 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PMULLx',
};

ENCODING VPMULLQ_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x40 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=PMULLx',
};

ENCODING VPMULLQ_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x40 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=PMULLx',
};

ENCODING VPMULLQ_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x40 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=PMULLx',
};


# PMULUDQ-Multiply Packed Unsigned Doubleword Integers.
ENCODING VPMULUDQ_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xf4 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PMULUDQ',
};

ENCODING VPMULUDQ_ronomo_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xf4 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PMULUDQ',
};

ENCODING VPMULUDQ_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xf4 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PMULUDQ',
};

ENCODING VPMULUDQ_rynymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xf4 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PMULUDQ',
};

ENCODING VPMULUDQ_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xf4 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PMULUDQ',
};


# POR-Bitwise Logical OR.
ENCODING VPOR_ronomo => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xeb P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=POR',
};

ENCODING VPOR_rynymy => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xeb P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=POR',
};


# PSADBW-Compute Sum of Absolute Differences.
ENCODING VPSADBW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xf6 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSADBW',
};

ENCODING VPSADBW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf6 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSADBW',
};

ENCODING VPSADBW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xf6 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSADBW',
};

ENCODING VPSADBW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf6 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSADBW',
};

ENCODING VPSADBW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf6 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSADBW',
};


# PSHUFB-Packed Shuffle Bytes.
ENCODING VPSHUFB_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x00 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSHUFB',
};

ENCODING VPSHUFB_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x00 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSHUFB',
};

ENCODING VPSHUFB_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x00 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSHUFB',
};

ENCODING VPSHUFB_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x00 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSHUFB',
};

ENCODING VPSHUFB_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x00 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSHUFB',
};


# PSHUFD-Shuffle Packed Doublewords.
ENCODING VPSHUFD_romoub_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x70 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSHUFD',
};

ENCODING VPSHUFD_romoub_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x70 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PSHUFD',
};

ENCODING VPSHUFD_rymyub_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x70 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSHUFD',
};

ENCODING VPSHUFD_rymyub_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x70 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PSHUFD',
};

ENCODING VPSHUFD_rzmzub_1 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x70 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PSHUFD',
};


# PSHUFHW-Shuffle Packed High Words.
ENCODING VPSHUFHW_romoub_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x70 P66=0 PF2=0 PF3=1 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSHUFHW',
};

ENCODING VPSHUFHW_romoub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x70 P66=0 PF2=0 PF3=1 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSHUFHW',
};

ENCODING VPSHUFHW_rymyub_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x70 P66=0 PF2=0 PF3=1 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSHUFHW',
};

ENCODING VPSHUFHW_rymyub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x70 P66=0 PF2=0 PF3=1 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSHUFHW',
};

ENCODING VPSHUFHW_rzmzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x70 P66=0 PF2=0 PF3=1 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSHUFHW',
};


# PSHUFLW-Shuffle Packed Low Words.
ENCODING VPSHUFLW_romoub_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x70 P66=0 PF2=1 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSHUFLW',
};

ENCODING VPSHUFLW_romoub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x70 P66=0 PF2=1 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSHUFLW',
};

ENCODING VPSHUFLW_rymyub_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x70 P66=0 PF2=1 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSHUFLW',
};

ENCODING VPSHUFLW_rymyub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x70 P66=0 PF2=1 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSHUFLW',
};

ENCODING VPSHUFLW_rzmzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x70 P66=0 PF2=1 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSHUFLW',
};


# PSIGNB/PSIGNW/PSIGND-Packed SIGN.
ENCODING VPSIGNB_ronomo => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x08 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSIGNx',
};

ENCODING VPSIGNB_rynymy => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x08 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSIGNx',
};

ENCODING VPSIGND_ronomo => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x0a P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSIGNx',
};

ENCODING VPSIGND_rynymy => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x0a P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSIGNx',
};

ENCODING VPSIGNW_ronomo => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x09 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSIGNx',
};

ENCODING VPSIGNW_rynymy => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x09 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSIGNx',
};


# PSLLDQ-Shift Double Quadword Left Logical.
ENCODING VPSLLDQ_nomoub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=7 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PSLLDQ',
};

ENCODING VPSLLDQ_nomoub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=7 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSLLDQ',
};

ENCODING VPSLLDQ_nymyub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=7 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSLLDQ',
};

ENCODING VPSLLDQ_nymyub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=7 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSLLDQ',
};

ENCODING VPSLLDQ_nzmzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=7 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSLLDQ',
};


# PSLLW/PSLLD/PSLLQ-Shift Packed Data Left Logical.
ENCODING VPSLLD_nomoub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=6 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLD_nomoub_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=6 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLD_nymyub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=6 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLD_nymyub_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=6 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLD_nzmzub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=6 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLD_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xf2 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLD_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf2 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLD_rynymo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf2 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLD_rynymx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xf2 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLD_rznzmo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf2 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLQ_nomoub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=6 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLQ_nomoub_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=6 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLQ_nymyub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=6 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLQ_nymyub_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=6 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLQ_nzmzub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=6 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLQ_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xf3 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLQ_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf3 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLQ_rynymo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf3 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLQ_rynymx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xf3 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLQ_rznzmo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf3 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLW_nomoub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=6 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLW_nomoub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=6 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLW_nymyub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=6 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLW_nymyub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=6 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLW_nzmzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=6 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xf1 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf1 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLW_rynymo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf1 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLW_rynymx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xf1 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSLLx',
};

ENCODING VPSLLW_rznzmo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf1 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSLLx',
};


# PSRAW/PSRAD/PSRAQ-Shift Packed Data Right Arithmetic.
ENCODING VPSRAD_nomoub => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=4 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAD_nymyub => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=4 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAD_ronomo => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe2 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAD_rynymx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe2 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAW_nomoub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=4 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAW_nomoub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=4 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAW_nymyub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=4 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAW_nymyub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=4 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAW_nzmzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=4 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe1 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe1 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAW_rynymo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe1 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAW_rynymx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe1 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAW_rznzmo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe1 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAx_nomoub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=4 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAx_nymyub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=4 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAx_nzmzub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=4 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAx_ronomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe2 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAx_rynymo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe2 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PSRAx',
};

ENCODING VPSRAx_rznzmo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe2 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PSRAx',
};


# PSRLDQ-Shift Double Quadword Right Logical.
ENCODING VPSRLDQ_nomoub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=3 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PSRLDQ',
};

ENCODING VPSRLDQ_nomoub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=3 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSRLDQ',
};

ENCODING VPSRLDQ_nymyub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=3 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSRLDQ',
};

ENCODING VPSRLDQ_nymyub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=3 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSRLDQ',
};

ENCODING VPSRLDQ_nzmzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=3 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSRLDQ',
};


# PSRLW/PSRLD/PSRLQ-Shift Packed Data Right Logical.
ENCODING VPSRLD_nomoub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=2 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLD_nomoub_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=2 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLD_nymyub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=2 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLD_nymyub_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=2 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLD_nzmzub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=2 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLD_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xd2 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLD_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd2 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLD_rynymo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd2 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLD_rynymx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xd2 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLD_rznzmo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd2 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLQ_nomoub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=2 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLQ_nomoub_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=2 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLQ_nymyub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=2 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLQ_nymyub_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=2 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLQ_nzmzub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x73 P66=1 PF2=0 PF3=0 REG=2 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLQ_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xd3 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLQ_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd3 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLQ_rynymo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd3 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLQ_rynymx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xd3 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLQ_rznzmo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd3 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLW_nomoub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=2 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLW_nomoub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=2 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLW_nymyub_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=2 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLW_nymyub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=2 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLW_nzmzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x71 P66=1 PF2=0 PF3=0 REG=2 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xd1 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_7',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd1 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLW_rynymo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd1 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLW_rynymx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xd1 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSRLx',
};

ENCODING VPSRLW_rznzmo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd1 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSRLx',
};


# PSUBQ-Subtract Packed Quadword Integers.
ENCODING VPSUBQ_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xfb P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSUBQ',
};

ENCODING VPSUBQ_ronomo_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xfb P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PSUBQ',
};

ENCODING VPSUBQ_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xfb P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSUBQ',
};

ENCODING VPSUBQ_rynymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xfb P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PSUBQ',
};

ENCODING VPSUBQ_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xfb P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PSUBQ',
};


# PSUBSB/PSUBSW-Subtract Packed Signed Integers with Signed Saturation.
ENCODING VPSUBSB_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe8 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSUBSx',
};

ENCODING VPSUBSB_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe8 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSUBSx',
};

ENCODING VPSUBSB_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe8 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSUBSx',
};

ENCODING VPSUBSB_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe8 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSUBSx',
};

ENCODING VPSUBSB_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe8 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSUBSx',
};

ENCODING VPSUBSW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe9 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSUBSx',
};

ENCODING VPSUBSW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe9 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSUBSx',
};

ENCODING VPSUBSW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xe9 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSUBSx',
};

ENCODING VPSUBSW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe9 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSUBSx',
};

ENCODING VPSUBSW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xe9 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSUBSx',
};


# PSUBUSB/PSUBUSW-Subtract Packed Unsigned Integers with Unsigned Saturation.
ENCODING VPSUBUSB_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xd8 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSUBUSx',
};

ENCODING VPSUBUSB_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd8 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSUBUSx',
};

ENCODING VPSUBUSB_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xd8 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSUBUSx',
};

ENCODING VPSUBUSB_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd8 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSUBUSx',
};

ENCODING VPSUBUSB_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd8 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSUBUSx',
};

ENCODING VPSUBUSW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xd9 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSUBUSx',
};

ENCODING VPSUBUSW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd9 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSUBUSx',
};

ENCODING VPSUBUSW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xd9 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSUBUSx',
};

ENCODING VPSUBUSW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd9 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSUBUSx',
};

ENCODING VPSUBUSW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xd9 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSUBUSx',
};


# PSUBB/PSUBW/PSUBD-Subtract Packed Integers.
ENCODING VPSUBB_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xf8 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSUBx',
};

ENCODING VPSUBB_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf8 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSUBx',
};

ENCODING VPSUBB_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xf8 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSUBx',
};

ENCODING VPSUBB_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf8 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSUBx',
};

ENCODING VPSUBB_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf8 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSUBx',
};

ENCODING VPSUBD_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xfa P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSUBx',
};

ENCODING VPSUBD_ronomo_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xfa P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PSUBx',
};

ENCODING VPSUBD_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xfa P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSUBx',
};

ENCODING VPSUBD_rynymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xfa P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PSUBx',
};

ENCODING VPSUBD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xfa P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PSUBx',
};

ENCODING VPSUBW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xf9 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=PSUBx',
};

ENCODING VPSUBW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf9 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PSUBx',
};

ENCODING VPSUBW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xf9 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PSUBx',
};

ENCODING VPSUBW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf9 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PSUBx',
};

ENCODING VPSUBW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0xf9 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PSUBx',
};


# PTEST-Logical Compare.
ENCODING VPTEST => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x17 P66=1 PF2=0 PF3=0',
  iflags     => 'cf=W zf=W',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PTEST',
};


# PUNPCKHBW/PUNPCKHWD/PUNPCKHDQ/PUNPCKHQDQ-Unpack High Data.
ENCODING VPUNPCKHBW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x68 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHBW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x68 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHBW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x68 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHBW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x68 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHBW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x68 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHDQ_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x6a P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHDQ_ronomo_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x6a P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHDQ_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x6a P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHDQ_rynymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x6a P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHDQ_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x6a P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHQDQ_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x6d P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHQDQ_ronomo_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x6d P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHQDQ_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x6d P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHQDQ_rynymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x6d P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHQDQ_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x6d P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHWD_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x69 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHWD_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x69 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHWD_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x69 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHWD_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x69 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PUNPCKHxx',
};

ENCODING VPUNPCKHWD_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x69 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PUNPCKHxx',
};


# PUNPCKLBW/PUNPCKLWD/PUNPCKLDQ/PUNPCKLQDQ-Unpack Low Data.
ENCODING VPUNPCKLBW_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x60 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLBW_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x60 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLBW_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x60 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLBW_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x60 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLBW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x60 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLDQ_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x62 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLDQ_ronomo_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x62 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLDQ_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x62 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLDQ_rynymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x62 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLDQ_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x62 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLQDQ_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x6c P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLQDQ_ronomo_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x6c P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLQDQ_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x6c P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLQDQ_rynymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x6c P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLQDQ_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x6c P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLWD_ronomo_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x61 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLWD_ronomo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x61 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLWD_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x61 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLWD_rynymy_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x61 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=PUNPCKLxx',
};

ENCODING VPUNPCKLWD_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x61 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=PUNPCKLxx',
};


# PXOR-Logical Exclusive OR.
ENCODING VPXOR_ronomo => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xef P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX',
  tags       => 'page=PXOR',
};

ENCODING VPXOR_rynymy => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xef P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=PXOR',
};


# RCPPS-Compute Reciprocals of Packed Single-Precision Floating-Point Values.
ENCODING VRCPPS => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x53 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX',
  tags       => 'page=RCPPS',
};


# RCPSS-Compute Reciprocal of Scalar Single-Precision Floating-Point Values.
ENCODING VRCPSS => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x53 P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=RCPSS',
};


# ROUNDPD-Round Packed Double Precision Floating-Point Values.
ENCODING VROUNDPD => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x09 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ROUNDING',
  metadata   => 'isa=AVX',
  tags       => 'page=ROUNDPD',
};


# ROUNDPS-Round Packed Single Precision Floating-Point Values.
ENCODING VROUNDPS => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x08 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ROUNDING',
  metadata   => 'isa=AVX',
  tags       => 'page=ROUNDPS',
};


# ROUNDSD-Round Scalar Double Precision Floating-Point Values.
ENCODING VROUNDSD => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x0b P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=ROUNDSD',
};


# ROUNDSS-Round Scalar Single Precision Floating-Point Values.
ENCODING VROUNDSS => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x0a P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=ROUNDSS',
};


# RSQRTPS-Compute Reciprocals of Square Roots of Packed Single-Precision Floating-Point Values.
ENCODING VRSQRTPS => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x52 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX',
  tags       => 'page=RSQRTPS',
};


# RSQRTSS-Compute Reciprocal of Square Root of Scalar Single-Precision Floating-Point Value.
ENCODING VRSQRTSS => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x52 P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=RSQRTSS',
};


# SHUFPD-Packed Interleave Shuffle of Pairs of Double-Precision Floating-Point Values.
ENCODING VSHUFPD_ronomoub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=SHUFPD',
};

ENCODING VSHUFPD_rxnxmxub => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xc6 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=SHUFPD',
};

ENCODING VSHUFPD_rynymyub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=SHUFPD',
};

ENCODING VSHUFPD_rznzmzub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=SHUFPD',
};


# SHUFPS-Packed Interleave Shuffle of Quadruplets of Single-Precision Floating-Point Values.
ENCODING VSHUFPS_ronomoub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xc6 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=SHUFPS',
};

ENCODING VSHUFPS_rxnxmxub => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0xc6 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=SHUFPS',
};

ENCODING VSHUFPS_rynymyub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xc6 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=SHUFPS',
};

ENCODING VSHUFPS_rznzmzub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xc6 P66=0 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=SHUFPS',
};


# SQRTPD-Square Root of Double-Precision Floating-Point Values.
ENCODING VSQRTPD_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x51 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=SQRTPD',
};

ENCODING VSQRTPD_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x51 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=SQRTPD',
};

ENCODING VSQRTPD_rymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x51 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=SQRTPD',
};

ENCODING VSQRTPD_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x51 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=SQRTPD',
};


# SQRTPS-Square Root of Single-Precision Floating-Point Values.
ENCODING VSQRTPS_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x51 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=SQRTPS',
};

ENCODING VSQRTPS_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x51 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=SQRTPS',
};

ENCODING VSQRTPS_rymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x51 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=SQRTPS',
};

ENCODING VSQRTPS_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x51 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=SQRTPS',
};


# SQRTSD-Compute Square Root of Scalar Double-Precision Floating-Point Value.
ENCODING VSQRTSD_ronomq => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x51 P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX',
  tags       => 'page=SQRTSD',
};

ENCODING VSQRTSD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x51 P66=0 PF2=1 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=SQRTSD',
};


# SQRTSS-Compute Square Root of Scalar Single-Precision Value.
ENCODING VSQRTSS_ronomd => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x51 P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX',
  tags       => 'page=SQRTSS',
};

ENCODING VSQRTSS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x51 P66=0 PF2=0 PF3=1 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=SQRTSS',
};


# STMXCSR-Store MXCSR Register State.
ENCODING VSTMXCSR => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MR=1 OP=0xae P66=0 PF2=0 PF3=0 REG=3 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_5',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=STMXCSR',
};


# SUBPD-Subtract Packed Double-Precision Floating-Point Values.
ENCODING VSUBPD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5c P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=SUBPD',
};

ENCODING VSUBPD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5c P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=SUBPD',
};

ENCODING VSUBPD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5c P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=SUBPD',
};

ENCODING VSUBPD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5c P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=SUBPD',
};


# SUBPS-Subtract Packed Single-Precision Floating-Point Values.
ENCODING VSUBPS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5c P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=SUBPS',
};

ENCODING VSUBPS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5c P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=SUBPS',
};

ENCODING VSUBPS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5c P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=SUBPS',
};

ENCODING VSUBPS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5c P66=0 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=SUBPS',
};


# SUBSD-Subtract Scalar Double-Precision Floating-Point Value.
ENCODING VSUBSD_ronomq => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5c P66=0 PF2=1 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=SUBSD',
};

ENCODING VSUBSD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5c P66=0 PF2=1 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=SUBSD',
};


# SUBSS-Subtract Scalar Single-Precision Floating-Point Value.
ENCODING VSUBSS_ronomd => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x5c P66=0 PF2=0 PF3=1',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX',
  tags       => 'page=SUBSS',
};

ENCODING VSUBSS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5c P66=0 PF2=0 PF3=1 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=SUBSS',
};


# UCOMISD-Unordered Compare Scalar Double-Precision Floating-Point Values and Set EFLAGS.
ENCODING VUCOMISD_romq => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x2e P66=1 PF2=0 PF3=0',
  iflags     => 'af=Z cf=W of=Z pf=W sf=Z zf=W',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  tags       => 'page=UCOMISD',
};

ENCODING VUCOMISD_romx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x2e P66=1 PF2=0 PF3=0 W=1',
  iflags     => 'af=Z cf=W of=Z pf=W sf=Z zf=W',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3NF',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=UCOMISD',
};


# UCOMISS-Unordered Compare Scalar Single-Precision Floating-Point Values and Set EFLAGS.
ENCODING VUCOMISS_romd => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x2e P66=0 PF2=0 PF3=0',
  iflags     => 'af=Z cf=W of=Z pf=W sf=Z zf=W',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX',
  tags       => 'page=UCOMISS',
};

ENCODING VUCOMISS_romx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x2e P66=0 PF2=0 PF3=0 W=0',
  iflags     => 'af=Z cf=W of=Z pf=W sf=Z zf=W',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3NF',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=UCOMISS',
};


# UNPCKHPD-Unpack and Interleave High Packed Double-Precision Floating-Point Values.
ENCODING VUNPCKHPD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x15 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=UNPCKHPD',
};

ENCODING VUNPCKHPD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x15 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=UNPCKHPD',
};

ENCODING VUNPCKHPD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x15 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=UNPCKHPD',
};

ENCODING VUNPCKHPD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x15 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=UNPCKHPD',
};


# UNPCKHPS-Unpack and Interleave High Packed Single-Precision Floating-Point Values.
ENCODING VUNPCKHPS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x15 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=UNPCKHPS',
};

ENCODING VUNPCKHPS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x15 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=UNPCKHPS',
};

ENCODING VUNPCKHPS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x15 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=UNPCKHPS',
};

ENCODING VUNPCKHPS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x15 P66=0 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=UNPCKHPS',
};


# UNPCKLPD-Unpack and Interleave Low Packed Double-Precision Floating-Point Values.
ENCODING VUNPCKLPD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x14 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=UNPCKLPD',
};

ENCODING VUNPCKLPD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x14 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=UNPCKLPD',
};

ENCODING VUNPCKLPD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x14 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=UNPCKLPD',
};

ENCODING VUNPCKLPD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x14 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=UNPCKLPD',
};


# UNPCKLPS-Unpack and Interleave Low Packed Single-Precision Floating-Point Values.
ENCODING VUNPCKLPS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x14 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=UNPCKLPS',
};

ENCODING VUNPCKLPS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x14 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=UNPCKLPS',
};

ENCODING VUNPCKLPS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x14 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=UNPCKLPS',
};

ENCODING VUNPCKLPS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x14 P66=0 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=UNPCKLPS',
};


# VALIGND/VALIGNQ-Align Doubleword/Quadword Vectors.
ENCODING VALIGNx_ronomoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x03 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VALIGNx',
};

ENCODING VALIGNx_rynymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x03 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VALIGNx',
};

ENCODING VALIGNx_rznzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x03 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VALIGNx',
};


# VBLENDMPD/VBLENDMPS-Blend Float64/Float32 Vectors Using an OpMask Control.
ENCODING VBLENDMPx_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x65 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VBLENDMPx',
};

ENCODING VBLENDMPx_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x65 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VBLENDMPx',
};

ENCODING VBLENDMPx_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x65 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VBLENDMPx',
};


# VBROADCAST-Load with Broadcast Floating-Point Data.
ENCODING VBROADCASTF128 => {
  diagram    => 'ENC=VEX MAP=0f38 MOD=MEM MR=1 OP=0x1a P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|BROADCASTING|LOAD|SWIZZLE',
  metadata   => 'isa=AVX',
  tags       => 'page=VBROADCAST',
};

ENCODING VBROADCASTF32X2_rymx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x19 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VBROADCAST',
};

ENCODING VBROADCASTF32X2_rzmx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x19 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VBROADCAST',
};

ENCODING VBROADCASTF32X4_rymo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x1a P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VBROADCAST',
};

ENCODING VBROADCASTF32X4_rzmo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x1a P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VBROADCAST',
};

ENCODING VBROADCASTF32X8 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x1b P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VBROADCAST',
};

ENCODING VBROADCASTF64X2_rymo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x1a P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VBROADCAST',
};

ENCODING VBROADCASTF64X2_rzmo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x1a P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VBROADCAST',
};

ENCODING VBROADCASTF64X4_rzmy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x1b P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING|LOAD',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VBROADCAST',
};

ENCODING VBROADCASTSD_rymo => {
  diagram    => 'ENC=VEX MAP=0f38 MOD=REG MR=1 OP=0x19 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX2',
  tags       => 'page=VBROADCAST',
};

ENCODING VBROADCASTSD_rymq => {
  diagram    => 'ENC=VEX MAP=0f38 MOD=MEM MR=1 OP=0x19 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX',
  tags       => 'page=VBROADCAST',
};

ENCODING VBROADCASTSD_rymx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x19 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VBROADCAST',
};

ENCODING VBROADCASTSD_rzmx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x19 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VBROADCAST',
};

ENCODING VBROADCASTSS_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x18 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VBROADCAST',
};

ENCODING VBROADCASTSS_rxmd => {
  diagram    => 'ENC=VEX MAP=0f38 MOD=MEM MR=1 OP=0x18 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX',
  tags       => 'page=VBROADCAST',
};

ENCODING VBROADCASTSS_rxmo => {
  diagram    => 'ENC=VEX MAP=0f38 MOD=REG MR=1 OP=0x18 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX2',
  tags       => 'page=VBROADCAST',
};

ENCODING VBROADCASTSS_rymx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x18 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VBROADCAST',
};

ENCODING VBROADCASTSS_rzmx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x18 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VBROADCAST',
};


# VCOMPRESSPD-Store Sparse Packed Double-Precision Floating-Point Values into Dense Memory.
ENCODING VCOMPRESSPD_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8a P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VCOMPRESSPD',
};

ENCODING VCOMPRESSPD_myry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8a P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VCOMPRESSPD',
};

ENCODING VCOMPRESSPD_mzrz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8a P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VCOMPRESSPD',
};


# VCOMPRESSPS-Store Sparse Packed Single-Precision Floating-Point Values into Dense Memory.
ENCODING VCOMPRESSPS_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8a P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VCOMPRESSPS',
};

ENCODING VCOMPRESSPS_myry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8a P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VCOMPRESSPS',
};

ENCODING VCOMPRESSPS_mzrz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8a P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VCOMPRESSPS',
};


# VCVTNE2PS2BF16.
ENCODING VCVTNE2PS2BF16_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x72 P66=0 PF2=1 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512_BF16_128',
  tags       => 'page=VCVTNE2PS2BF16',
};

ENCODING VCVTNE2PS2BF16_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x72 P66=0 PF2=1 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512_BF16_256',
  tags       => 'page=VCVTNE2PS2BF16',
};

ENCODING VCVTNE2PS2BF16_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x72 P66=0 PF2=1 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512_BF16_512',
  tags       => 'page=VCVTNE2PS2BF16',
};


# VCVTNEPS2BF16.
ENCODING VCVTNEPS2BF16_romo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x72 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512_BF16_128',
  tags       => 'page=VCVTNEPS2BF16',
};

ENCODING VCVTNEPS2BF16_romy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x72 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512_BF16_256',
  tags       => 'page=VCVTNEPS2BF16',
};

ENCODING VCVTNEPS2BF16_rymz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x72 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512_BF16_512',
  tags       => 'page=VCVTNEPS2BF16',
};


# VCVTPD2QQ-Convert Packed Double-Precision Floating-Point Values to Packed Quadword Integers.
ENCODING VCVTPD2QQ_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7b P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VCVTPD2QQ',
};

ENCODING VCVTPD2QQ_rymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7b P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VCVTPD2QQ',
};

ENCODING VCVTPD2QQ_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7b P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VCVTPD2QQ',
};


# VCVTPD2UDQ-Convert Packed Double-Precision Floating-Point Values to Packed Unsigned Doubleword Integers . 5-27.
ENCODING VCVTPD2UDQ_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x79 P66=0 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VCVTPD2UDQ',
};

ENCODING VCVTPD2UDQ_romy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x79 P66=0 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VCVTPD2UDQ',
};

ENCODING VCVTPD2UDQ_rymz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x79 P66=0 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VCVTPD2UDQ',
};


# VCVTPD2UQQ-Convert Packed Double-Precision Floating-Point Values to Packed Unsigned Quadword Integers.
ENCODING VCVTPD2UQQ_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x79 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VCVTPD2UQQ',
};

ENCODING VCVTPD2UQQ_rymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x79 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VCVTPD2UQQ',
};

ENCODING VCVTPD2UQQ_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x79 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VCVTPD2UQQ',
};


# VCVTPH2PS-Convert 16-bit FP values to Single-Precision FP values.
ENCODING VCVTPH2PS_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x13 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E11',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VCVTPH2PS',
};

ENCODING VCVTPH2PS_rymo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x13 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E11',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VCVTPH2PS',
};

ENCODING VCVTPH2PS_rzmy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x13 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E11',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VCVTPH2PS',
};


# VCVTPS2PH-Convert Single-Precision FP value to 16-bit FP value.
ENCODING VCVTPS2PH_moroub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MOD=REG MR=1 OP=0x1d P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E11NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VCVTPS2PH',
};

ENCODING VCVTPS2PH_moryub_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MOD=REG MR=1 OP=0x1d P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E11NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VCVTPS2PH',
};

ENCODING VCVTPS2PH_moryub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MOD=MEM MR=1 OP=0x1d P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E11',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VCVTPS2PH',
};

ENCODING VCVTPS2PH_mqroub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MOD=MEM MR=1 OP=0x1d P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E11',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VCVTPS2PH',
};

ENCODING VCVTPS2PH_myrzub_1 => {
  diagram    => 'ENC=EVEX MAP=0f3a MOD=REG MR=1 OP=0x1d P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E11NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VCVTPS2PH',
};

ENCODING VCVTPS2PH_myrzub_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MOD=MEM MR=1 OP=0x1d P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E11',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VCVTPS2PH',
};


# VCVTPS2QQ-Convert Packed Single Precision Floating-Point Values to Packed Signed Quadword Integer Values . . 5-43.
ENCODING VCVTPS2QQ_romx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7b P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VCVTPS2QQ',
};

ENCODING VCVTPS2QQ_rymo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7b P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VCVTPS2QQ',
};

ENCODING VCVTPS2QQ_rzmy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7b P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VCVTPS2QQ',
};


# VCVTPS2UDQ-Convert Packed Single-Precision Floating-Point Values to Packed Unsigned Doubleword Integer Values.
ENCODING VCVTPS2UDQ_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x79 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VCVTPS2UDQ',
};

ENCODING VCVTPS2UDQ_rymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x79 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VCVTPS2UDQ',
};

ENCODING VCVTPS2UDQ_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x79 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VCVTPS2UDQ',
};


# VCVTPS2UQQ-Convert Packed Single Precision Floating-Point Values to Packed Unsigned Quadword Integer Values.
ENCODING VCVTPS2UQQ_romx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x79 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VCVTPS2UQQ',
};

ENCODING VCVTPS2UQQ_rymo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x79 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VCVTPS2UQQ',
};

ENCODING VCVTPS2UQQ_rzmy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x79 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VCVTPS2UQQ',
};


# VCVTQQ2PD-Convert Packed Quadword Integers to Packed Double-Precision Floating-Point Values.
ENCODING VCVTQQ2PD_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xe6 P66=0 PF2=0 PF3=1 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VCVTQQ2PD',
};

ENCODING VCVTQQ2PD_rymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xe6 P66=0 PF2=0 PF3=1 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VCVTQQ2PD',
};

ENCODING VCVTQQ2PD_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xe6 P66=0 PF2=0 PF3=1 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VCVTQQ2PD',
};


# VCVTQQ2PS-Convert Packed Quadword Integers to Packed Single-Precision Floating-Point Values.
ENCODING VCVTQQ2PS_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5b P66=0 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VCVTQQ2PS',
};

ENCODING VCVTQQ2PS_romy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5b P66=0 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VCVTQQ2PS',
};

ENCODING VCVTQQ2PS_rymz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x5b P66=0 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VCVTQQ2PS',
};


# VCVTSD2USI-Convert Scalar Double-Precision Floating-Point Value to Unsigned Doubleword Integer.
ENCODING VCVTSD2USI => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x79 P66=0 PF2=1 PF3=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VCVTSD2USI',
};


# VCVTSS2USI-Convert Scalar Single-Precision Floating-Point Value to Unsigned Doubleword Integer.
ENCODING VCVTSS2USI => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x79 P66=0 PF2=0 PF3=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VCVTSS2USI',
};


# VCVTTPD2QQ-Convert with Truncation Packed Double-Precision Floating-Point Values to Packed Quadword Integers.
ENCODING VCVTTPD2QQ_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VCVTTPD2QQ',
};

ENCODING VCVTTPD2QQ_rymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VCVTTPD2QQ',
};

ENCODING VCVTTPD2QQ_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VCVTTPD2QQ',
};


# VCVTTPD2UDQ-Convert with Truncation Packed Double-Precision Floating-Point Values to Packed Unsigned Doubleword Integers.
ENCODING VCVTTPD2UDQ_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x78 P66=0 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VCVTTPD2UDQ',
};

ENCODING VCVTTPD2UDQ_romy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x78 P66=0 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VCVTTPD2UDQ',
};

ENCODING VCVTTPD2UDQ_rymz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x78 P66=0 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VCVTTPD2UDQ',
};


# VCVTTPD2UQQ-Convert with Truncation Packed Double-Precision Floating-Point Values to Packed Unsigned Quadword Integers.
ENCODING VCVTTPD2UQQ_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x78 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VCVTTPD2UQQ',
};

ENCODING VCVTTPD2UQQ_rymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x78 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VCVTTPD2UQQ',
};

ENCODING VCVTTPD2UQQ_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x78 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VCVTTPD2UQQ',
};


# VCVTTPS2QQ-Convert with Truncation Packed Single Precision Floating-Point Values to Packed Signed Quadword Integer Values.
ENCODING VCVTTPS2QQ_romx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VCVTTPS2QQ',
};

ENCODING VCVTTPS2QQ_rymo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VCVTTPS2QQ',
};

ENCODING VCVTTPS2QQ_rzmy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VCVTTPS2QQ',
};


# VCVTTPS2UDQ-Convert with Truncation Packed Single-Precision Floating-Point Values to Packed Unsigned Doubleword Integer Values.
ENCODING VCVTTPS2UDQ_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x78 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VCVTTPS2UDQ',
};

ENCODING VCVTTPS2UDQ_rymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x78 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VCVTTPS2UDQ',
};

ENCODING VCVTTPS2UDQ_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x78 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VCVTTPS2UDQ',
};


# VCVTTPS2UQQ-Convert with Truncation Packed Single Precision Floating-Point Values to Packed Unsigned Quadword Integer Values.
ENCODING VCVTTPS2UQQ_romx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x78 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VCVTTPS2UQQ',
};

ENCODING VCVTTPS2UQQ_rymo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x78 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VCVTTPS2UQQ',
};

ENCODING VCVTTPS2UQQ_rzmy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x78 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VCVTTPS2UQQ',
};


# VCVTTSD2USI-Convert with Truncation Scalar Double-Precision Floating-Point Value to Unsigned Integer.
ENCODING VCVTTSD2USI => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x78 P66=0 PF2=1 PF3=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VCVTTSD2USI',
};


# VCVTTSS2USI-Convert with Truncation Scalar Single-Precision Floating-Point Value to Unsigned Integer.
ENCODING VCVTTSS2USI => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x78 P66=0 PF2=0 PF3=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VCVTTSS2USI',
};


# VCVTUDQ2PD-Convert Packed Unsigned Doubleword Integers to Packed Double-Precision Floating-Point Values . 5-72.
ENCODING VCVTUDQ2PD_romx => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VCVTUDQ2PD',
};

ENCODING VCVTUDQ2PD_rymo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VCVTUDQ2PD',
};

ENCODING VCVTUDQ2PD_rzmy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E5',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VCVTUDQ2PD',
};


# VCVTUDQ2PS-Convert Packed Unsigned Doubleword Integers to Packed Single-Precision Floating-Point Values . . 5-74.
ENCODING VCVTUDQ2PS_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=0 PF2=1 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VCVTUDQ2PS',
};

ENCODING VCVTUDQ2PS_rymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=0 PF2=1 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VCVTUDQ2PS',
};

ENCODING VCVTUDQ2PS_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=0 PF2=1 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VCVTUDQ2PS',
};


# VCVTUQQ2PD-Convert Packed Unsigned Quadword Integers to Packed Double-Precision Floating-Point Values.
ENCODING VCVTUQQ2PD_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=0 PF2=0 PF3=1 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VCVTUQQ2PD',
};

ENCODING VCVTUQQ2PD_rymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=0 PF2=0 PF3=1 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VCVTUQQ2PD',
};

ENCODING VCVTUQQ2PD_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=0 PF2=0 PF3=1 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VCVTUQQ2PD',
};


# VCVTUQQ2PS-Convert Packed Unsigned Quadword Integers to Packed Single-Precision Floating-Point Values.
ENCODING VCVTUQQ2PS_romo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=0 PF2=1 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VCVTUQQ2PS',
};

ENCODING VCVTUQQ2PS_romy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=0 PF2=1 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VCVTUQQ2PS',
};

ENCODING VCVTUQQ2PS_rymz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7a P66=0 PF2=1 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VCVTUQQ2PS',
};


# VCVTUSI2SD-Convert Unsigned Integer to Scalar Double-Precision Floating-Point Value.
ENCODING VCVTUSI2SD_ronomd => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f MR=1 OP=0x7b P66=0 PF2=1 PF3=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E10NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VCVTUSI2SD',
};

ENCODING VCVTUSI2SD_ronomq => {
  diagram    => 'ENC=EVEX MAP=0f MODE=64 MR=1 OP=0x7b P66=0 PF2=1 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VCVTUSI2SD',
};


# VCVTUSI2SS-Convert Unsigned Integer to Scalar Single-Precision Floating-Point Value.
ENCODING VCVTUSI2SS => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x7b P66=0 PF2=0 PF3=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VCVTUSI2SS',
};


# VDBPSADBW-Double Block Packed Sum-Absolute-Differences (SAD) on Unsigned Bytes.
ENCODING VDBPSADBW_ronomoub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x42 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VDBPSADBW',
};

ENCODING VDBPSADBW_rynymyub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x42 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VDBPSADBW',
};

ENCODING VDBPSADBW_rznzmzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x42 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VDBPSADBW',
};


# VDPBF16PS.
ENCODING VDPBF16PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x52 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_BF16_128',
  tags       => 'page=VDPBF16PS',
};

ENCODING VDPBF16PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x52 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_BF16_256',
  tags       => 'page=VDPBF16PS',
};

ENCODING VDPBF16PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x52 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_BF16_512',
  tags       => 'page=VDPBF16PS',
};


# VEXPANDPD-Load Sparse Packed Double-Precision Floating-Point Values from Dense Memory.
ENCODING VEXPANDPD_romo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x88 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VEXPANDPD',
};

ENCODING VEXPANDPD_rymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x88 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VEXPANDPD',
};

ENCODING VEXPANDPD_rzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x88 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VEXPANDPD',
};


# VEXPANDPS-Load Sparse Packed Single-Precision Floating-Point Values from Dense Memory.
ENCODING VEXPANDPS_romo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x88 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VEXPANDPS',
};

ENCODING VEXPANDPS_rymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x88 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VEXPANDPS',
};

ENCODING VEXPANDPS_rzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x88 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VEXPANDPS',
};


# VEXTRACTF128/VEXTRACTF32x4/VEXTRACTF64x2/VEXTRACTF32x8/VEXTRACTF64x4-Extract Packed Floating-Point Values.
ENCODING VEXTRACTF128 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x19 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX',
  tags       => 'page=VEXTRACTFxxx',
};

ENCODING VEXTRACTF32X4_moryub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x19 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VEXTRACTFxxx',
};

ENCODING VEXTRACTF32X4_morzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x19 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VEXTRACTFxxx',
};

ENCODING VEXTRACTF32X8 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x1b P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VEXTRACTFxxx',
};

ENCODING VEXTRACTF64X2_moryub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x19 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VEXTRACTFxxx',
};

ENCODING VEXTRACTF64X2_morzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x19 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VEXTRACTFxxx',
};

ENCODING VEXTRACTF64X4 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x1b P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VEXTRACTFxxx',
};


# VEXTRACTI128/VEXTRACTI32x4/VEXTRACTI64x2/VEXTRACTI32x8/VEXTRACTI64x4-Extract packed Integer Values.
ENCODING VEXTRACTI128 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x39 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX2',
  tags       => 'page=VEXTRACTIxxx',
};

ENCODING VEXTRACTI32X4_moryub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x39 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VEXTRACTIxxx',
};

ENCODING VEXTRACTI32X4_morzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x39 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VEXTRACTIxxx',
};

ENCODING VEXTRACTI32X8 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x3b P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VEXTRACTIxxx',
};

ENCODING VEXTRACTI64X2_moryub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x39 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VEXTRACTIxxx',
};

ENCODING VEXTRACTI64X2_morzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x39 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VEXTRACTIxxx',
};

ENCODING VEXTRACTI64X4 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x3b P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VEXTRACTIxxx',
};


# VFIXUPIMMPD-Fix Up Special Packed Float64 Values.
ENCODING VFIXUPIMMPD_ronomoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x54 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFIXUPIMMPD',
};

ENCODING VFIXUPIMMPD_rynymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x54 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFIXUPIMMPD',
};

ENCODING VFIXUPIMMPD_rznzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x54 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFIXUPIMMPD',
};


# VFIXUPIMMPS-Fix Up Special Packed Float32 Values.
ENCODING VFIXUPIMMPS_ronomoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x54 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFIXUPIMMPS',
};

ENCODING VFIXUPIMMPS_rynymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x54 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFIXUPIMMPS',
};

ENCODING VFIXUPIMMPS_rznzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x54 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFIXUPIMMPS',
};


# VFIXUPIMMSD-Fix Up Special Scalar Float64 Value.
ENCODING VFIXUPIMMSD => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x55 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFIXUPIMMSD',
};


# VFIXUPIMMSS-Fix Up Special Scalar Float32 Value.
ENCODING VFIXUPIMMSS => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x55 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFIXUPIMMSS',
};


# VFMADDSUB132PD/VFMADDSUB213PD/VFMADDSUB231PD-Fused Multiply-Alternating Add/Subtract of Packed Double-Precision Floating-Point Values.
ENCODING VFMADDSUB132PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x96 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMADDSUBxxxPD',
};

ENCODING VFMADDSUB132PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x96 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDSUBxxxPD',
};

ENCODING VFMADDSUB132PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x96 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMADDSUBxxxPD',
};

ENCODING VFMADDSUB132PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x96 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMADDSUBxxxPD',
};

ENCODING VFMADDSUB213PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa6 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMADDSUBxxxPD',
};

ENCODING VFMADDSUB213PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xa6 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDSUBxxxPD',
};

ENCODING VFMADDSUB213PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa6 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMADDSUBxxxPD',
};

ENCODING VFMADDSUB213PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa6 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMADDSUBxxxPD',
};

ENCODING VFMADDSUB231PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb6 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMADDSUBxxxPD',
};

ENCODING VFMADDSUB231PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xb6 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDSUBxxxPD',
};

ENCODING VFMADDSUB231PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb6 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMADDSUBxxxPD',
};

ENCODING VFMADDSUB231PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb6 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMADDSUBxxxPD',
};


# VFMADDSUB132PS/VFMADDSUB213PS/VFMADDSUB231PS-Fused Multiply-Alternating Add/Subtract of Packed Single-Precision Floating-Point Values.
ENCODING VFMADDSUB132PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x96 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMADDSUBxxxPS',
};

ENCODING VFMADDSUB132PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x96 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDSUBxxxPS',
};

ENCODING VFMADDSUB132PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x96 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMADDSUBxxxPS',
};

ENCODING VFMADDSUB132PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x96 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMADDSUBxxxPS',
};

ENCODING VFMADDSUB213PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa6 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMADDSUBxxxPS',
};

ENCODING VFMADDSUB213PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xa6 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDSUBxxxPS',
};

ENCODING VFMADDSUB213PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa6 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMADDSUBxxxPS',
};

ENCODING VFMADDSUB213PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa6 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMADDSUBxxxPS',
};

ENCODING VFMADDSUB231PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb6 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMADDSUBxxxPS',
};

ENCODING VFMADDSUB231PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xb6 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDSUBxxxPS',
};

ENCODING VFMADDSUB231PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb6 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMADDSUBxxxPS',
};

ENCODING VFMADDSUB231PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb6 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMADDSUBxxxPS',
};


# VFMADD132PD/VFMADD213PD/VFMADD231PD-Fused Multiply-Add of Packed Double-Precision Floating-Point Values.
ENCODING VFMADD132PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x98 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMADDxxxPD',
};

ENCODING VFMADD132PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x98 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDxxxPD',
};

ENCODING VFMADD132PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x98 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMADDxxxPD',
};

ENCODING VFMADD132PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x98 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMADDxxxPD',
};

ENCODING VFMADD213PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa8 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMADDxxxPD',
};

ENCODING VFMADD213PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xa8 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDxxxPD',
};

ENCODING VFMADD213PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa8 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMADDxxxPD',
};

ENCODING VFMADD213PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa8 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMADDxxxPD',
};

ENCODING VFMADD231PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb8 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMADDxxxPD',
};

ENCODING VFMADD231PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xb8 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDxxxPD',
};

ENCODING VFMADD231PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb8 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMADDxxxPD',
};

ENCODING VFMADD231PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb8 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMADDxxxPD',
};


# VFMADD132PS/VFMADD213PS/VFMADD231PS-Fused Multiply-Add of Packed Single-Precision Floating-Point Values.
ENCODING VFMADD132PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x98 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMADDxxxPS',
};

ENCODING VFMADD132PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x98 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDxxxPS',
};

ENCODING VFMADD132PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x98 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMADDxxxPS',
};

ENCODING VFMADD132PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x98 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMADDxxxPS',
};

ENCODING VFMADD213PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa8 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMADDxxxPS',
};

ENCODING VFMADD213PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xa8 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDxxxPS',
};

ENCODING VFMADD213PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa8 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMADDxxxPS',
};

ENCODING VFMADD213PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa8 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMADDxxxPS',
};

ENCODING VFMADD231PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb8 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMADDxxxPS',
};

ENCODING VFMADD231PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xb8 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDxxxPS',
};

ENCODING VFMADD231PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb8 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMADDxxxPS',
};

ENCODING VFMADD231PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb8 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMADDxxxPS',
};


# VFMADD132SD/VFMADD213SD/VFMADD231SD-Fused Multiply-Add of Scalar Double-Precision Floating-Point Values.
ENCODING VFMADD132SD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x99 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFMADDxxxSD',
};

ENCODING VFMADD132SD_ronqmq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x99 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDxxxSD',
};

ENCODING VFMADD213SD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa9 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFMADDxxxSD',
};

ENCODING VFMADD213SD_ronqmq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xa9 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDxxxSD',
};

ENCODING VFMADD231SD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb9 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFMADDxxxSD',
};

ENCODING VFMADD231SD_ronqmq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xb9 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDxxxSD',
};


# VFMADD132SS/VFMADD213SS/VFMADD231SS-Fused Multiply-Add of Scalar Single-Precision Floating-Point Values.
ENCODING VFMADD132SS_rondmd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x99 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDxxxSS',
};

ENCODING VFMADD132SS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x99 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFMADDxxxSS',
};

ENCODING VFMADD213SS_rondmd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xa9 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDxxxSS',
};

ENCODING VFMADD213SS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa9 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFMADDxxxSS',
};

ENCODING VFMADD231SS_rondmd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xb9 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMADDxxxSS',
};

ENCODING VFMADD231SS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb9 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFMADDxxxSS',
};


# VFMSUBADD132PD/VFMSUBADD213PD/VFMSUBADD231PD-Fused Multiply-Alternating Subtract/Add of Packed Double-Precision Floating-Point Values.
ENCODING VFMSUBADD132PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x97 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMSUBADDxxxPD',
};

ENCODING VFMSUBADD132PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x97 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBADDxxxPD',
};

ENCODING VFMSUBADD132PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x97 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMSUBADDxxxPD',
};

ENCODING VFMSUBADD132PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x97 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMSUBADDxxxPD',
};

ENCODING VFMSUBADD213PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa7 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMSUBADDxxxPD',
};

ENCODING VFMSUBADD213PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xa7 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBADDxxxPD',
};

ENCODING VFMSUBADD213PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa7 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMSUBADDxxxPD',
};

ENCODING VFMSUBADD213PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa7 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMSUBADDxxxPD',
};

ENCODING VFMSUBADD231PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb7 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMSUBADDxxxPD',
};

ENCODING VFMSUBADD231PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xb7 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBADDxxxPD',
};

ENCODING VFMSUBADD231PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb7 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMSUBADDxxxPD',
};

ENCODING VFMSUBADD231PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb7 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMSUBADDxxxPD',
};


# VFMSUBADD132PS/VFMSUBADD213PS/VFMSUBADD231PS-Fused Multiply-Alternating Subtract/Add of Packed Single-Precision Floating-Point Values.
ENCODING VFMSUBADD132PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x97 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMSUBADDxxxPS',
};

ENCODING VFMSUBADD132PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x97 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBADDxxxPS',
};

ENCODING VFMSUBADD132PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x97 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMSUBADDxxxPS',
};

ENCODING VFMSUBADD132PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x97 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMSUBADDxxxPS',
};

ENCODING VFMSUBADD213PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa7 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMSUBADDxxxPS',
};

ENCODING VFMSUBADD213PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xa7 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBADDxxxPS',
};

ENCODING VFMSUBADD213PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa7 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMSUBADDxxxPS',
};

ENCODING VFMSUBADD213PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xa7 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMSUBADDxxxPS',
};

ENCODING VFMSUBADD231PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb7 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMSUBADDxxxPS',
};

ENCODING VFMSUBADD231PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xb7 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBADDxxxPS',
};

ENCODING VFMSUBADD231PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb7 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMSUBADDxxxPS',
};

ENCODING VFMSUBADD231PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb7 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMSUBADDxxxPS',
};


# VFMSUB132PD/VFMSUB213PD/VFMSUB231PD-Fused Multiply-Subtract of Packed Double-Precision Floating-Point Values.
ENCODING VFMSUB132PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9a P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMSUBxxxPD',
};

ENCODING VFMSUB132PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x9a P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBxxxPD',
};

ENCODING VFMSUB132PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9a P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMSUBxxxPD',
};

ENCODING VFMSUB132PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9a P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMSUBxxxPD',
};

ENCODING VFMSUB213PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xaa P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMSUBxxxPD',
};

ENCODING VFMSUB213PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xaa P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBxxxPD',
};

ENCODING VFMSUB213PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xaa P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMSUBxxxPD',
};

ENCODING VFMSUB213PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xaa P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMSUBxxxPD',
};

ENCODING VFMSUB231PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xba P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMSUBxxxPD',
};

ENCODING VFMSUB231PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xba P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBxxxPD',
};

ENCODING VFMSUB231PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xba P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMSUBxxxPD',
};

ENCODING VFMSUB231PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xba P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMSUBxxxPD',
};


# VFMSUB132PS/VFMSUB213PS/VFMSUB231PS-Fused Multiply-Subtract of Packed Single-Precision Floating-Point Values.
ENCODING VFMSUB132PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9a P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMSUBxxxPS',
};

ENCODING VFMSUB132PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x9a P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBxxxPS',
};

ENCODING VFMSUB132PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9a P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMSUBxxxPS',
};

ENCODING VFMSUB132PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9a P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMSUBxxxPS',
};

ENCODING VFMSUB213PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xaa P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMSUBxxxPS',
};

ENCODING VFMSUB213PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xaa P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBxxxPS',
};

ENCODING VFMSUB213PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xaa P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMSUBxxxPS',
};

ENCODING VFMSUB213PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xaa P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMSUBxxxPS',
};

ENCODING VFMSUB231PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xba P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFMSUBxxxPS',
};

ENCODING VFMSUB231PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xba P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBxxxPS',
};

ENCODING VFMSUB231PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xba P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFMSUBxxxPS',
};

ENCODING VFMSUB231PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xba P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFMSUBxxxPS',
};


# VFMSUB132SD/VFMSUB213SD/VFMSUB231SD-Fused Multiply-Subtract of Scalar Double-Precision Floating-Point Values.
ENCODING VFMSUB132SD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9b P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFMSUBxxxSD',
};

ENCODING VFMSUB132SD_ronqmq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x9b P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBxxxSD',
};

ENCODING VFMSUB213SD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xab P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFMSUBxxxSD',
};

ENCODING VFMSUB213SD_ronqmq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xab P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBxxxSD',
};

ENCODING VFMSUB231SD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbb P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFMSUBxxxSD',
};

ENCODING VFMSUB231SD_ronqmq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xbb P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBxxxSD',
};


# VFMSUB132SS/VFMSUB213SS/VFMSUB231SS-Fused Multiply-Subtract of Scalar Single-Precision Floating-Point Values.
ENCODING VFMSUB132SS_rondmd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x9b P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBxxxSS',
};

ENCODING VFMSUB132SS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9b P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFMSUBxxxSS',
};

ENCODING VFMSUB213SS_rondmd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xab P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBxxxSS',
};

ENCODING VFMSUB213SS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xab P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFMSUBxxxSS',
};

ENCODING VFMSUB231SS_rondmd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xbb P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFMSUBxxxSS',
};

ENCODING VFMSUB231SS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbb P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFMSUBxxxSS',
};


# VFNMADD132PD/VFNMADD213PD/VFNMADD231PD-Fused Negative Multiply-Add of Packed Double-Precision Floating-Point Values.
ENCODING VFNMADD132PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9c P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFNMADDxxxPD',
};

ENCODING VFNMADD132PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x9c P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMADDxxxPD',
};

ENCODING VFNMADD132PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9c P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFNMADDxxxPD',
};

ENCODING VFNMADD132PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9c P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFNMADDxxxPD',
};

ENCODING VFNMADD213PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xac P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFNMADDxxxPD',
};

ENCODING VFNMADD213PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xac P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMADDxxxPD',
};

ENCODING VFNMADD213PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xac P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFNMADDxxxPD',
};

ENCODING VFNMADD213PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xac P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFNMADDxxxPD',
};

ENCODING VFNMADD231PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbc P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFNMADDxxxPD',
};

ENCODING VFNMADD231PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xbc P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMADDxxxPD',
};

ENCODING VFNMADD231PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbc P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFNMADDxxxPD',
};

ENCODING VFNMADD231PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbc P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFNMADDxxxPD',
};


# VFNMADD132PS/VFNMADD213PS/VFNMADD231PS-Fused Negative Multiply-Add of Packed Single-Precision Floating-Point Values.
ENCODING VFNMADD132PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9c P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFNMADDxxxPS',
};

ENCODING VFNMADD132PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x9c P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMADDxxxPS',
};

ENCODING VFNMADD132PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9c P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFNMADDxxxPS',
};

ENCODING VFNMADD132PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9c P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFNMADDxxxPS',
};

ENCODING VFNMADD213PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xac P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFNMADDxxxPS',
};

ENCODING VFNMADD213PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xac P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMADDxxxPS',
};

ENCODING VFNMADD213PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xac P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFNMADDxxxPS',
};

ENCODING VFNMADD213PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xac P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFNMADDxxxPS',
};

ENCODING VFNMADD231PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbc P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFNMADDxxxPS',
};

ENCODING VFNMADD231PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xbc P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMADDxxxPS',
};

ENCODING VFNMADD231PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbc P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFNMADDxxxPS',
};

ENCODING VFNMADD231PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbc P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFNMADDxxxPS',
};


# VFNMADD132SD/VFNMADD213SD/VFNMADD231SD-Fused Negative Multiply-Add of Scalar Double-Precision Floating-Point Values.
ENCODING VFNMADD132SD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9d P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFNMADDxxxSD',
};

ENCODING VFNMADD132SD_ronqmq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x9d P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMADDxxxSD',
};

ENCODING VFNMADD213SD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xad P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFNMADDxxxSD',
};

ENCODING VFNMADD213SD_ronqmq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xad P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMADDxxxSD',
};

ENCODING VFNMADD231SD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbd P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFNMADDxxxSD',
};

ENCODING VFNMADD231SD_ronqmq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xbd P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMADDxxxSD',
};


# VFNMADD132SS/VFNMADD213SS/VFNMADD231SS-Fused Negative Multiply-Add of Scalar Single-Precision Floating-Point Values.
ENCODING VFNMADD132SS_rondmd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x9d P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMADDxxxSS',
};

ENCODING VFNMADD132SS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9d P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFNMADDxxxSS',
};

ENCODING VFNMADD213SS_rondmd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xad P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMADDxxxSS',
};

ENCODING VFNMADD213SS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xad P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFNMADDxxxSS',
};

ENCODING VFNMADD231SS_rondmd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xbd P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMADDxxxSS',
};

ENCODING VFNMADD231SS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbd P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFNMADDxxxSS',
};


# VFNMSUB132PD/VFNMSUB213PD/VFNMSUB231PD-Fused Negative Multiply-Subtract of Packed Double-Precision Floating-Point Values.
ENCODING VFNMSUB132PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9e P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFNMSUBxxxPD',
};

ENCODING VFNMSUB132PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x9e P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMSUBxxxPD',
};

ENCODING VFNMSUB132PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9e P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFNMSUBxxxPD',
};

ENCODING VFNMSUB132PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9e P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFNMSUBxxxPD',
};

ENCODING VFNMSUB213PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xae P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFNMSUBxxxPD',
};

ENCODING VFNMSUB213PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xae P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMSUBxxxPD',
};

ENCODING VFNMSUB213PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xae P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFNMSUBxxxPD',
};

ENCODING VFNMSUB213PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xae P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFNMSUBxxxPD',
};

ENCODING VFNMSUB231PD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbe P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFNMSUBxxxPD',
};

ENCODING VFNMSUB231PD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xbe P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMSUBxxxPD',
};

ENCODING VFNMSUB231PD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbe P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFNMSUBxxxPD',
};

ENCODING VFNMSUB231PD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbe P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFNMSUBxxxPD',
};


# VFNMSUB132PS/VFNMSUB213PS/VFNMSUB231PS-Fused Negative Multiply-Subtract of Packed Single-Precision Floating-Point Values.
ENCODING VFNMSUB132PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9e P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFNMSUBxxxPS',
};

ENCODING VFNMSUB132PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x9e P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMSUBxxxPS',
};

ENCODING VFNMSUB132PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9e P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFNMSUBxxxPS',
};

ENCODING VFNMSUB132PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9e P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFNMSUBxxxPS',
};

ENCODING VFNMSUB213PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xae P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFNMSUBxxxPS',
};

ENCODING VFNMSUB213PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xae P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMSUBxxxPS',
};

ENCODING VFNMSUB213PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xae P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFNMSUBxxxPS',
};

ENCODING VFNMSUB213PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xae P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFNMSUBxxxPS',
};

ENCODING VFNMSUB231PS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbe P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VFNMSUBxxxPS',
};

ENCODING VFNMSUB231PS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xbe P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMSUBxxxPS',
};

ENCODING VFNMSUB231PS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbe P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VFNMSUBxxxPS',
};

ENCODING VFNMSUB231PS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbe P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VFNMSUBxxxPS',
};


# VFNMSUB132SD/VFNMSUB213SD/VFNMSUB231SD-Fused Negative Multiply-Subtract of Scalar Double-Precision Floating-Point Values.
ENCODING VFNMSUB132SD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9f P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFNMSUBxxxSD',
};

ENCODING VFNMSUB132SD_ronqmq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x9f P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMSUBxxxSD',
};

ENCODING VFNMSUB213SD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xaf P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFNMSUBxxxSD',
};

ENCODING VFNMSUB213SD_ronqmq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xaf P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMSUBxxxSD',
};

ENCODING VFNMSUB231SD_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbf P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFNMSUBxxxSD',
};

ENCODING VFNMSUB231SD_ronqmq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xbf P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMSUBxxxSD',
};


# VFNMSUB132SS/VFNMSUB213SS/VFNMSUB231SS-Fused Negative Multiply-Subtract of Scalar Single-Precision Floating-Point Values.
ENCODING VFNMSUB132SS_rondmd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x9f P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMSUBxxxSS',
};

ENCODING VFNMSUB132SS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x9f P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFNMSUBxxxSS',
};

ENCODING VFNMSUB213SS_rondmd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xaf P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMSUBxxxSS',
};

ENCODING VFNMSUB213SS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xaf P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFNMSUBxxxSS',
};

ENCODING VFNMSUB231SS_rondmd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0xbf P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA',
  exceptions => 'AVX_TYPE_3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=FMA',
  tags       => 'page=VFNMSUBxxxSS',
};

ENCODING VFNMSUB231SS_ronomx => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xbf P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VFNMSUBxxxSS',
};


# VFPCLASSPD-Tests Types Of a Packed Float64 Values.
ENCODING VFPCLASSPD_moub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x66 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VFPCLASSPD',
};

ENCODING VFPCLASSPD_myub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x66 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VFPCLASSPD',
};

ENCODING VFPCLASSPD_mzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x66 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VFPCLASSPD',
};


# VFPCLASSPS-Tests Types Of a Packed Float32 Values.
ENCODING VFPCLASSPS_moub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x66 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VFPCLASSPS',
};

ENCODING VFPCLASSPS_myub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x66 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VFPCLASSPS',
};

ENCODING VFPCLASSPS_mzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x66 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VFPCLASSPS',
};


# VFPCLASSSD-Tests Types Of a Scalar Float64 Values.
ENCODING VFPCLASSSD => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x67 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_SCALAR',
  tags       => 'page=VFPCLASSSD',
};


# VFPCLASSSS-Tests Types Of a Scalar Float32 Values.
ENCODING VFPCLASSSS => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x67 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_SCALAR',
  tags       => 'page=VFPCLASSSS',
};


# VGATHERDPS/VGATHERDPD-Gather Packed Single, Packed Double with Signed Dword.
ENCODING VGATHERDPx_romx => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x92 P66=1 PF2=0 PF3=0 RM=4 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VGATHERDPx',
};

ENCODING VGATHERDPx_rxmxnx => {
  diagram    => 'ASZ=Y ENC=VEX MAP=0f38 MOD=MEM MR=1 OP=0x92 P66=1 PF2=0 PF3=0 RM=4',
  extensions => 'AVX2GATHER',
  exceptions => 'AVX_TYPE_12',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX2GATHER',
  tags       => 'page=VGATHERDPx',
};

ENCODING VGATHERDPx_rymx => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x92 P66=1 PF2=0 PF3=0 RM=4 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VGATHERDPx',
};

ENCODING VGATHERDPx_rzmx => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x92 P66=1 PF2=0 PF3=0 RM=4 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VGATHERDPx',
};


# VGATHERQPS/VGATHERQPD-Gather Packed Single, Packed Double with Signed Qword Indices.
ENCODING VGATHERQPx_romx => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x93 P66=1 PF2=0 PF3=0 RM=4 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VGATHERQPx',
};

ENCODING VGATHERQPx_rxmx_1 => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x93 P66=1 PF2=0 PF3=0 RM=4 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VGATHERQPx',
};

ENCODING VGATHERQPx_rxmx_2 => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x93 P66=1 PF2=0 PF3=0 RM=4 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VGATHERQPx',
};

ENCODING VGATHERQPx_rxmxnx => {
  diagram    => 'ASZ=Y ENC=VEX MAP=0f38 MOD=MEM MR=1 OP=0x93 P66=1 PF2=0 PF3=0 RM=4',
  extensions => 'AVX2GATHER',
  exceptions => 'AVX_TYPE_12',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX2GATHER',
  tags       => 'page=VGATHERQPx',
};


# VGETEXPPD-Convert Exponents of Packed DP FP Values to DP FP Values.
ENCODING VGETEXPPD_romo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x42 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VGETEXPPD',
};

ENCODING VGETEXPPD_rymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x42 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VGETEXPPD',
};

ENCODING VGETEXPPD_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x42 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VGETEXPPD',
};


# VGETEXPPS-Convert Exponents of Packed SP FP Values to SP FP Values.
ENCODING VGETEXPPS_romo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x42 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VGETEXPPS',
};

ENCODING VGETEXPPS_rymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x42 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VGETEXPPS',
};

ENCODING VGETEXPPS_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x42 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VGETEXPPS',
};


# VGETEXPSD-Convert Exponents of Scalar DP FP Values to DP FP Value.
ENCODING VGETEXPSD => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x43 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VGETEXPSD',
};


# VGETEXPSS-Convert Exponents of Scalar SP FP Values to SP FP Value.
ENCODING VGETEXPSS => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x43 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VGETEXPSS',
};


# VGETMANTPD-Extract Float64 Vector of Normalized Mantissas from Float64 Vector.
ENCODING VGETMANTPD_romoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x26 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VGETMANTPD',
};

ENCODING VGETMANTPD_rymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x26 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VGETMANTPD',
};

ENCODING VGETMANTPD_rzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x26 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VGETMANTPD',
};


# VGETMANTPS-Extract Float32 Vector of Normalized Mantissas from Float32 Vector.
ENCODING VGETMANTPS_romoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x26 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VGETMANTPS',
};

ENCODING VGETMANTPS_rymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x26 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VGETMANTPS',
};

ENCODING VGETMANTPS_rzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x26 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VGETMANTPS',
};


# VGETMANTSD-Extract Float64 of Normalized Mantissas from Float64 Scalar.
ENCODING VGETMANTSD => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x27 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VGETMANTSD',
};


# VGETMANTSS-Extract Float32 Vector of Normalized Mantissa from Float32 Vector.
ENCODING VGETMANTSS => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x27 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VGETMANTSS',
};


# VINSERTF128/VINSERTF32x4/VINSERTF64x2/VINSERTF32x8/VINSERTF64x4-Insert Packed Floating-Point Values.
ENCODING VINSERTF128 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x18 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|SET',
  metadata   => 'isa=AVX',
  tags       => 'page=VINSERTFxxx',
};

ENCODING VINSERTF32X4_rynymoub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x18 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VINSERTFxxx',
};

ENCODING VINSERTF32X4_rznzmoub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x18 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VINSERTFxxx',
};

ENCODING VINSERTF32X8 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x1a P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VINSERTFxxx',
};

ENCODING VINSERTF64X2_rynymoub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x18 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VINSERTFxxx',
};

ENCODING VINSERTF64X2_rznzmoub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x18 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VINSERTFxxx',
};

ENCODING VINSERTF64X4 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x1a P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VINSERTFxxx',
};


# VINSERTI128/VINSERTI32x4/VINSERTI64x2/VINSERTI32x8/VINSERTI64x4-Insert Packed Integer Values.
ENCODING VINSERTI128 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x38 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX2',
  tags       => 'page=VINSERTIxxx',
};

ENCODING VINSERTI32X4_rynymoub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x38 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VINSERTIxxx',
};

ENCODING VINSERTI32X4_rznzmoub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x38 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VINSERTIxxx',
};

ENCODING VINSERTI32X8 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x3a P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VINSERTIxxx',
};

ENCODING VINSERTI64X2_rynymoub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x38 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VINSERTIxxx',
};

ENCODING VINSERTI64X2_rznzmoub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x38 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VINSERTIxxx',
};

ENCODING VINSERTI64X4 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x3a P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VINSERTIxxx',
};


# VMASKMOV-Conditional SIMD Packed Loads and Stores.
ENCODING VMASKMOVPD_mxnxrx => {
  diagram    => 'ENC=VEX MAP=0f38 MOD=MEM MR=1 OP=0x2f P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX',
  tags       => 'page=VMASKMOV',
};

ENCODING VMASKMOVPD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MOD=MEM MR=1 OP=0x2d P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX',
  tags       => 'page=VMASKMOV',
};

ENCODING VMASKMOVPS_mxnxrx => {
  diagram    => 'ENC=VEX MAP=0f38 MOD=MEM MR=1 OP=0x2e P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX',
  tags       => 'page=VMASKMOV',
};

ENCODING VMASKMOVPS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MOD=MEM MR=1 OP=0x2c P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX',
  tags       => 'page=VMASKMOV',
};


# VP2INTERSECTD.
ENCODING VP2INTERSECTD_nomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x68 P66=0 PF2=1 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512_VP2INTERSECT_128',
  tags       => 'page=VP2INTERSECTD',
};

ENCODING VP2INTERSECTD_nymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x68 P66=0 PF2=1 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512_VP2INTERSECT_256',
  tags       => 'page=VP2INTERSECTD',
};

ENCODING VP2INTERSECTD_nzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x68 P66=0 PF2=1 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512_VP2INTERSECT_512',
  tags       => 'page=VP2INTERSECTD',
};


# VP2INTERSECTQ.
ENCODING VP2INTERSECTQ_nomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x68 P66=0 PF2=1 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512_VP2INTERSECT_128',
  tags       => 'page=VP2INTERSECTQ',
};

ENCODING VP2INTERSECTQ_nymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x68 P66=0 PF2=1 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512_VP2INTERSECT_256',
  tags       => 'page=VP2INTERSECTQ',
};

ENCODING VP2INTERSECTQ_nzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x68 P66=0 PF2=1 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=AVX512_VP2INTERSECT_512',
  tags       => 'page=VP2INTERSECTQ',
};


# VPBLENDD-Blend Packed Dwords.
ENCODING VPBLENDD => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x02 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX2',
  tags       => 'page=VPBLENDD',
};


# VPBLENDMB/VPBLENDMW-Blend Byte/Word Vectors Using an Opmask Control.
ENCODING VPBLENDMx_ronomo_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x66 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPBLENDMbw',
};

ENCODING VPBLENDMx_rynymy_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x66 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPBLENDMbw',
};

ENCODING VPBLENDMx_rznzmz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x66 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPBLENDMbw',
};


# VPBLENDMD/VPBLENDMQ-Blend Int32/Int64 Vectors Using an OpMask Control.
ENCODING VPBLENDMx_ronomo_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x64 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPBLENDMy',
};

ENCODING VPBLENDMx_rynymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x64 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPBLENDMy',
};

ENCODING VPBLENDMx_rznzmz_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x64 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPBLENDMy',
};


# VPBROADCAST-Load Integer and Broadcast.
ENCODING VBROADCASTI128 => {
  diagram    => 'ENC=VEX MAP=0f38 MOD=MEM MR=1 OP=0x5a P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|BROADCASTING|SWIZZLE',
  metadata   => 'isa=AVX2',
  tags       => 'page=VPBROADCAST',
};

ENCODING VBROADCASTI32X2_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x59 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VPBROADCAST',
};

ENCODING VBROADCASTI32X2_rymx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x59 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VPBROADCAST',
};

ENCODING VBROADCASTI32X2_rzmx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x59 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VPBROADCAST',
};

ENCODING VBROADCASTI32X4_rymo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x5a P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPBROADCAST',
};

ENCODING VBROADCASTI32X4_rzmo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x5a P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPBROADCAST',
};

ENCODING VBROADCASTI32X8 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x5b P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VPBROADCAST',
};

ENCODING VBROADCASTI64X2_rymo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x5a P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VPBROADCAST',
};

ENCODING VBROADCASTI64X2_rzmo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x5a P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VPBROADCAST',
};

ENCODING VBROADCASTI64X4_rzmy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x5b P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING|LOAD',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTB_romd => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x7a P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTB_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x78 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTB_rxmb => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x78 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX2',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTB_rymd => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x7a P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTB_rymx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x78 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTB_rzmd => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x7a P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTB_rzmx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x78 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTD_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x58 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTD_rxmd => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x58 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX2',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTD_rymx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x58 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTD_rzmx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x58 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTQ_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x59 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTQ_rxmq => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x59 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX2',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTQ_rymx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x59 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTQ_rzmx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x59 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTW_romd => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x7b P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTW_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x79 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTW_rxmw => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x79 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX2',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTW_rymd => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x7b P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTW_rymx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x79 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTW_rzmd => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x7b P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTW_rzmx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x79 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTx_romx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x7c P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTx_rymx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x7c P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPBROADCAST',
};

ENCODING VPBROADCASTx_rzmx => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x7c P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPBROADCAST',
};


# VPBROADCASTM-Broadcast Mask to Vector Register.
ENCODING VPBROADCASTMB2Q_ro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x2a P66=0 PF2=0 PF3=1 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512CD_128',
  tags       => 'page=VPBROADCASTM',
};

ENCODING VPBROADCASTMB2Q_ry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x2a P66=0 PF2=0 PF3=1 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512CD_256',
  tags       => 'page=VPBROADCASTM',
};

ENCODING VPBROADCASTMB2Q_rz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x2a P66=0 PF2=0 PF3=1 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512CD_512',
  tags       => 'page=VPBROADCASTM',
};

ENCODING VPBROADCASTMW2D_ro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x3a P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512CD_128',
  tags       => 'page=VPBROADCASTM',
};

ENCODING VPBROADCASTMW2D_ry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x3a P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512CD_256',
  tags       => 'page=VPBROADCASTM',
};

ENCODING VPBROADCASTMW2D_rz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x3a P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=AVX512CD_512',
  tags       => 'page=VPBROADCASTM',
};


# VPCMPB/VPCMPUB-Compare Packed Byte Values Into Mask.
ENCODING VPCMPB_nomoub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x3f P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPCMPxB',
};

ENCODING VPCMPB_nymyub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x3f P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPCMPxB',
};

ENCODING VPCMPB_nzmzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x3f P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPCMPxB',
};

ENCODING VPCMPUB_nomoub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x3e P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPCMPxB',
};

ENCODING VPCMPUB_nymyub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x3e P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPCMPxB',
};

ENCODING VPCMPUB_nzmzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x3e P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPCMPxB',
};


# VPCMPD/VPCMPUD-Compare Packed Integer Values into Mask.
ENCODING VPCMPD_nomoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x1f P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPCMPxD',
};

ENCODING VPCMPD_nymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x1f P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPCMPxD',
};

ENCODING VPCMPD_nzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x1f P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPCMPxD',
};

ENCODING VPCMPUD_nomoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x1e P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPCMPxD',
};

ENCODING VPCMPUD_nymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x1e P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPCMPxD',
};

ENCODING VPCMPUD_nzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x1e P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPCMPxD',
};


# VPCMPQ/VPCMPUQ-Compare Packed Integer Values into Mask.
ENCODING VPCMPQ_nomoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x1f P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPCMPxQ',
};

ENCODING VPCMPQ_nymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x1f P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPCMPxQ',
};

ENCODING VPCMPQ_nzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x1f P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPCMPxQ',
};

ENCODING VPCMPUQ_nomoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x1e P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPCMPxQ',
};

ENCODING VPCMPUQ_nymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x1e P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPCMPxQ',
};

ENCODING VPCMPUQ_nzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x1e P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPCMPxQ',
};


# VPCMPW/VPCMPUW-Compare Packed Word Values Into Mask.
ENCODING VPCMPUW_nomoub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x3e P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPCMPxW',
};

ENCODING VPCMPUW_nymyub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x3e P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPCMPxW',
};

ENCODING VPCMPUW_nzmzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x3e P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPCMPxW',
};

ENCODING VPCMPW_nomoub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x3f P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPCMPxW',
};

ENCODING VPCMPW_nymyub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x3f P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPCMPxW',
};

ENCODING VPCMPW_nzmzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x3f P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPCMPxW',
};


# VPCOMPRESSB/VCOMPRESSW-Store Sparse Packed Byte/Word Integer Values into Dense Memory/Register.
ENCODING VPCOMPRESSx_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x63 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512_VBMI2_128',
  tags       => 'page=VPCOMPRESS',
};

ENCODING VPCOMPRESSx_myry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x63 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512_VBMI2_256',
  tags       => 'page=VPCOMPRESS',
};

ENCODING VPCOMPRESSx_mzrz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x63 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512_VBMI2_512',
  tags       => 'page=VPCOMPRESS',
};


# VPCOMPRESSD-Store Sparse Packed Doubleword Integer Values into Dense Memory/Register.
ENCODING VPCOMPRESSD_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8b P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPCOMPRESSD',
};

ENCODING VPCOMPRESSD_myry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8b P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPCOMPRESSD',
};

ENCODING VPCOMPRESSD_mzrz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8b P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPCOMPRESSD',
};


# VPCOMPRESSQ-Store Sparse Packed Quadword Integer Values into Dense Memory/Register.
ENCODING VPCOMPRESSQ_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8b P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPCOMPRESSQ',
};

ENCODING VPCOMPRESSQ_myry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8b P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPCOMPRESSQ',
};

ENCODING VPCOMPRESSQ_mzrz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8b P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPCOMPRESSQ',
};


# VPCONFLICTD/Q-Detect Conflicts Within a Vector of Packed Dword/Qword Values into Dense Memory/ Register. 5-334.
ENCODING VPCONFLICTx_romo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xc4 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512CD_128',
  tags       => 'page=VPCONFLICTx',
};

ENCODING VPCONFLICTx_rymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xc4 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512CD_256',
  tags       => 'page=VPCONFLICTx',
};

ENCODING VPCONFLICTx_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xc4 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=AVX512CD_512',
  tags       => 'page=VPCONFLICTx',
};


# VPDPBUSD-Multiply and Add Unsigned and Signed Bytes.
ENCODING VPDPBUSD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x50 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_VNNI_128',
  tags       => 'page=VPDPBUSD',
};

ENCODING VPDPBUSD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x50 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_VNNI_256',
  tags       => 'page=VPDPBUSD',
};

ENCODING VPDPBUSD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x50 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_VNNI_512',
  tags       => 'page=VPDPBUSD',
};


# VPDPBUSDS-Multiply and Add Unsigned and Signed Bytes with Saturation.
ENCODING VPDPBUSDS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x51 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_VNNI_128',
  tags       => 'page=VPDPBUSDS',
};

ENCODING VPDPBUSDS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x51 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_VNNI_256',
  tags       => 'page=VPDPBUSDS',
};

ENCODING VPDPBUSDS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x51 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_VNNI_512',
  tags       => 'page=VPDPBUSDS',
};


# VPDPWSSD-Multiply and Add Signed Word Integers.
ENCODING VPDPWSSD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x52 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_VNNI_128',
  tags       => 'page=VPDPWSSD',
};

ENCODING VPDPWSSD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x52 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_VNNI_256',
  tags       => 'page=VPDPWSSD',
};

ENCODING VPDPWSSD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x52 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_VNNI_512',
  tags       => 'page=VPDPWSSD',
};


# VPDPWSSDS-Multiply and Add Signed Word Integers with Saturation.
ENCODING VPDPWSSDS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x53 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_VNNI_128',
  tags       => 'page=VPDPWSSDS',
};

ENCODING VPDPWSSDS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x53 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_VNNI_256',
  tags       => 'page=VPDPWSSDS',
};

ENCODING VPDPWSSDS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x53 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_VNNI_512',
  tags       => 'page=VPDPWSSDS',
};


# VPERM2F128-Permute Floating-Point Values.
ENCODING VPERM2F128 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x06 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX',
  tags       => 'page=VPERM2F128',
};


# VPERM2I128-Permute Integer Values.
ENCODING VPERM2I128 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x46 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX2',
  tags       => 'page=VPERM2I128',
};


# VPERMB-Permute Packed Bytes Elements.
ENCODING VPERMB_ronomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8d P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512_VBMI_128',
  tags       => 'page=VPERMB',
};

ENCODING VPERMB_rynymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8d P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512_VBMI_256',
  tags       => 'page=VPERMB',
};

ENCODING VPERMB_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8d P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512_VBMI_512',
  tags       => 'page=VPERMB',
};


# VPERMI2B-Full Permute of Bytes from Two Tables Overwriting the Index.
ENCODING VPERMI2B_ronomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x75 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512_VBMI_128',
  tags       => 'page=VPERMI2B',
};

ENCODING VPERMI2B_rynymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x75 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512_VBMI_256',
  tags       => 'page=VPERMI2B',
};

ENCODING VPERMI2B_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x75 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512_VBMI_512',
  tags       => 'page=VPERMI2B',
};


# VPERMI2W/D/Q/PS/PD-Full Permute From Two Tables Overwriting the Index.
ENCODING VPERMI2Px_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x77 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPERMI2x',
};

ENCODING VPERMI2Px_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x77 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPERMI2x',
};

ENCODING VPERMI2Px_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x77 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPERMI2x',
};

ENCODING VPERMI2W_ronomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x75 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPERMI2x',
};

ENCODING VPERMI2W_rynymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x75 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPERMI2x',
};

ENCODING VPERMI2W_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x75 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPERMI2x',
};

ENCODING VPERMI2x_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x76 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPERMI2x',
};

ENCODING VPERMI2x_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x76 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPERMI2x',
};

ENCODING VPERMI2x_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x76 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPERMI2x',
};


# VPERMILPD-Permute In-Lane of Pairs of Double-Precision Floating-Point Values.
ENCODING VPERMILPD_romoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x05 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPERMILPD',
};

ENCODING VPERMILPD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x0d P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPERMILPD',
};

ENCODING VPERMILPD_rxmxub => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x05 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=VPERMILPD',
};

ENCODING VPERMILPD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x0d P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=VPERMILPD',
};

ENCODING VPERMILPD_rymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x05 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPERMILPD',
};

ENCODING VPERMILPD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x0d P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPERMILPD',
};

ENCODING VPERMILPD_rzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x05 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPERMILPD',
};

ENCODING VPERMILPD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x0d P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPERMILPD',
};


# VPERMILPS-Permute In-Lane of Quadruples of Single-Precision Floating-Point Values.
ENCODING VPERMILPS_romoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x04 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPERMILPS',
};

ENCODING VPERMILPS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x0c P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPERMILPS',
};

ENCODING VPERMILPS_rxmxub => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x04 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=VPERMILPS',
};

ENCODING VPERMILPS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x0c P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX',
  tags       => 'page=VPERMILPS',
};

ENCODING VPERMILPS_rymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x04 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPERMILPS',
};

ENCODING VPERMILPS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x0c P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPERMILPS',
};

ENCODING VPERMILPS_rzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x04 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPERMILPS',
};

ENCODING VPERMILPS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x0c P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPERMILPS',
};


# VPERMPD-Permute Double-Precision Floating-Point Elements.
ENCODING VPERMPD_rymyub_1 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x01 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=VPERMPD',
};

ENCODING VPERMPD_rymyub_2 => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x01 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPERMPD',
};

ENCODING VPERMPD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x16 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPERMPD',
};

ENCODING VPERMPD_rzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x01 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPERMPD',
};

ENCODING VPERMPD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x16 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPERMPD',
};


# VPERMPS-Permute Single-Precision Floating-Point Elements.
ENCODING VPERMPS_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x16 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=VPERMPS',
};

ENCODING VPERMPS_rynymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x16 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPERMPS',
};

ENCODING VPERMPS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x16 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPERMPS',
};


# VPERMQ-Qwords Element Permutation.
ENCODING VPERMQ_rymyub_1 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x00 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=VPERMQ',
};

ENCODING VPERMQ_rymyub_2 => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x00 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPERMQ',
};

ENCODING VPERMQ_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x36 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPERMQ',
};

ENCODING VPERMQ_rzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x00 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPERMQ',
};

ENCODING VPERMQ_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x36 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPERMQ',
};


# VPERMT2B-Full Permute of Bytes from Two Tables Overwriting a Table.
ENCODING VPERMT2B_ronomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x7d P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512_VBMI_128',
  tags       => 'page=VPERMT2B',
};

ENCODING VPERMT2B_rynymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x7d P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512_VBMI_256',
  tags       => 'page=VPERMT2B',
};

ENCODING VPERMT2B_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x7d P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512_VBMI_512',
  tags       => 'page=VPERMT2B',
};


# VPERMT2W/D/Q/PS/PD-Full Permute from Two Tables Overwriting one Table.
ENCODING VPERMT2Px_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x7f P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPERMT2x',
};

ENCODING VPERMT2Px_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x7f P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPERMT2x',
};

ENCODING VPERMT2Px_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x7f P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPERMT2x',
};

ENCODING VPERMT2W_ronomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x7d P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPERMT2x',
};

ENCODING VPERMT2W_rynymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x7d P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPERMT2x',
};

ENCODING VPERMT2W_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x7d P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPERMT2x',
};

ENCODING VPERMT2x_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x7e P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPERMT2x',
};

ENCODING VPERMT2x_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x7e P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPERMT2x',
};

ENCODING VPERMT2x_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x7e P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPERMT2x',
};


# VPERMD/VPERMW-Permute Packed Doublewords/Words Elements.
ENCODING VPERMD_rynymy_1 => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x36 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX2',
  tags       => 'page=VPERMx',
};

ENCODING VPERMD_rynymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x36 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPERMx',
};

ENCODING VPERMD_rznzmz_1 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x36 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPERMx',
};

ENCODING VPERMW_ronomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8d P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPERMx',
};

ENCODING VPERMW_rynymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8d P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPERMx',
};

ENCODING VPERMW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8d P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPERMx',
};


# VPEXPANDD-Load Sparse Packed Doubleword Integer Values from Dense Memory / Register.
ENCODING VPEXPANDD_romo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x89 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPEXPANDD',
};

ENCODING VPEXPANDD_rymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x89 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPEXPANDD',
};

ENCODING VPEXPANDD_rzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x89 P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPEXPANDD',
};


# VPEXPANDQ-Load Sparse Packed Quadword Integer Values from Dense Memory / Register.
ENCODING VPEXPANDQ_romo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x89 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPEXPANDQ',
};

ENCODING VPEXPANDQ_rymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x89 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPEXPANDQ',
};

ENCODING VPEXPANDQ_rzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x89 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPEXPANDQ',
};


# VPEXPANDB/VPEXPANDW-Expand Byte/Word Values.
ENCODING VPEXPANDx_romo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x62 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512_VBMI2_128',
  tags       => 'page=VPEXPANDx',
};

ENCODING VPEXPANDx_rymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x62 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512_VBMI2_256',
  tags       => 'page=VPEXPANDx',
};

ENCODING VPEXPANDx_rzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x62 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=AVX512_VBMI2_512',
  tags       => 'page=VPEXPANDx',
};


# VPGATHERDD/VPGATHERDQ-Gather Packed Dword, Packed Qword with Signed Dword Indices.
ENCODING VPGATHERDx_romx => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x90 P66=1 PF2=0 PF3=0 RM=4 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPGATHERDx',
};

ENCODING VPGATHERDx_rxmxnx => {
  diagram    => 'ASZ=Y ENC=VEX MAP=0f38 MOD=MEM MR=1 OP=0x90 P66=1 PF2=0 PF3=0 RM=4',
  extensions => 'AVX2GATHER',
  exceptions => 'AVX_TYPE_12',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX2GATHER',
  tags       => 'page=VPGATHERDx',
};

ENCODING VPGATHERDx_rymx => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x90 P66=1 PF2=0 PF3=0 RM=4 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPGATHERDx',
};

ENCODING VPGATHERDx_rzmx => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x90 P66=1 PF2=0 PF3=0 RM=4 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPGATHERDx',
};


# VPGATHERQD/VPGATHERQQ-Gather Packed Dword, Packed Qword with Signed Qword Indices.
ENCODING VPGATHERQx_romx => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x91 P66=1 PF2=0 PF3=0 RM=4 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPGATHERQx',
};

ENCODING VPGATHERQx_rxmx_1 => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x91 P66=1 PF2=0 PF3=0 RM=4 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPGATHERQx',
};

ENCODING VPGATHERQx_rxmx_2 => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x91 P66=1 PF2=0 PF3=0 RM=4 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPGATHERQx',
};

ENCODING VPGATHERQx_rxmxnx => {
  diagram    => 'ASZ=Y ENC=VEX MAP=0f38 MOD=MEM MR=1 OP=0x91 P66=1 PF2=0 PF3=0 RM=4',
  extensions => 'AVX2GATHER',
  exceptions => 'AVX_TYPE_12',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX2GATHER',
  tags       => 'page=VPGATHERQx',
};


# VPLZCNTD/Q-Count the Number of Leading Zero Bits for Packed Dword, Packed Qword Values.
ENCODING VPLZCNTx_romo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x44 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=AVX512CD_128',
  tags       => 'page=VPLZCNTx',
};

ENCODING VPLZCNTx_rymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x44 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=AVX512CD_256',
  tags       => 'page=VPLZCNTx',
};

ENCODING VPLZCNTx_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x44 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=AVX512CD_512',
  tags       => 'page=VPLZCNTx',
};


# VPMADD52HUQ-Packed Multiply of Unsigned 52-bit Unsigned Integers and Add High 52-bit Products to 64-bit Accumulators.
ENCODING VPMADD52HUQ_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb5 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX512_IFMA_128',
  tags       => 'page=VPMADD52HUQ',
};

ENCODING VPMADD52HUQ_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb5 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX512_IFMA_256',
  tags       => 'page=VPMADD52HUQ',
};

ENCODING VPMADD52HUQ_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb5 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX512_IFMA_512',
  tags       => 'page=VPMADD52HUQ',
};


# VPMADD52LUQ-Packed Multiply of Unsigned 52-bit Integers and Add the Low 52-bit Products to Qword Accumulators.
ENCODING VPMADD52LUQ_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb4 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX512_IFMA_128',
  tags       => 'page=VPMADD52LUQ',
};

ENCODING VPMADD52LUQ_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb4 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX512_IFMA_256',
  tags       => 'page=VPMADD52LUQ',
};

ENCODING VPMADD52LUQ_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xb4 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=AVX512_IFMA_512',
  tags       => 'page=VPMADD52LUQ',
};


# VPMASKMOV-Conditional SIMD Integer Packed Loads and Stores.
ENCODING VPMASKMOVx_mxnxrx => {
  diagram    => 'ENC=VEX MAP=0f38 MOD=MEM MR=1 OP=0x8e P66=1 PF2=0 PF3=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX2',
  tags       => 'page=VPMASKMOV',
};

ENCODING VPMASKMOVx_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MOD=MEM MR=1 OP=0x8c P66=1 PF2=0 PF3=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_6',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX2',
  tags       => 'page=VPMASKMOV',
};


# VPMOVM2B/VPMOVM2W/VPMOVM2D/VPMOVM2Q-Convert a Mask Register to a Vector Register.
ENCODING VPMOVM2x_ro_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x28 P66=0 PF2=0 PF3=1 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPMOVM2x',
};

ENCODING VPMOVM2x_ro_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x38 P66=0 PF2=0 PF3=1 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VPMOVM2x',
};

ENCODING VPMOVM2x_ry_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x28 P66=0 PF2=0 PF3=1 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPMOVM2x',
};

ENCODING VPMOVM2x_ry_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x38 P66=0 PF2=0 PF3=1 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VPMOVM2x',
};

ENCODING VPMOVM2x_rz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x28 P66=0 PF2=0 PF3=1 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPMOVM2x',
};

ENCODING VPMOVM2x_rz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x38 P66=0 PF2=0 PF3=1 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VPMOVM2x',
};


# VPMOVB2M/VPMOVW2M/VPMOVD2M/VPMOVQ2M-Convert a Vector Register to a Mask.
ENCODING VPMOVx2M_mo_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x29 P66=0 PF2=0 PF3=1 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPMOVx2M',
};

ENCODING VPMOVx2M_mo_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x39 P66=0 PF2=0 PF3=1 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VPMOVx2M',
};

ENCODING VPMOVx2M_my_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x29 P66=0 PF2=0 PF3=1 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPMOVx2M',
};

ENCODING VPMOVx2M_my_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x39 P66=0 PF2=0 PF3=1 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VPMOVx2M',
};

ENCODING VPMOVx2M_mz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x29 P66=0 PF2=0 PF3=1 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPMOVx2M',
};

ENCODING VPMOVx2M_mz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x39 P66=0 PF2=0 PF3=1 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E7NM',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VPMOVx2M',
};


# VPMOVDB/VPMOVSDB/VPMOVUSDB-Down Convert DWord to Byte.
ENCODING VPMOVDB_mdro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x31 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxDB',
};

ENCODING VPMOVDB_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x31 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxDB',
};

ENCODING VPMOVDB_mory => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x31 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxDB',
};

ENCODING VPMOVDB_morz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x31 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxDB',
};

ENCODING VPMOVDB_morz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x31 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxDB',
};

ENCODING VPMOVDB_mqry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x31 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxDB',
};

ENCODING VPMOVSDB_mdro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x21 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxDB',
};

ENCODING VPMOVSDB_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x21 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxDB',
};

ENCODING VPMOVSDB_mory => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x21 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxDB',
};

ENCODING VPMOVSDB_morz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x21 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxDB',
};

ENCODING VPMOVSDB_morz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x21 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxDB',
};

ENCODING VPMOVSDB_mqry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x21 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxDB',
};

ENCODING VPMOVUSDB_mdro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x11 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxDB',
};

ENCODING VPMOVUSDB_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x11 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxDB',
};

ENCODING VPMOVUSDB_mory => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x11 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxDB',
};

ENCODING VPMOVUSDB_morz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x11 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxDB',
};

ENCODING VPMOVUSDB_morz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x11 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxDB',
};

ENCODING VPMOVUSDB_mqry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x11 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxDB',
};


# VPMOVDW/VPMOVSDW/VPMOVUSDW-Down Convert DWord to Word.
ENCODING VPMOVDW_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x33 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxDW',
};

ENCODING VPMOVDW_mory_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x33 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxDW',
};

ENCODING VPMOVDW_mory_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x33 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxDW',
};

ENCODING VPMOVDW_mqro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x33 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxDW',
};

ENCODING VPMOVDW_myrz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x33 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxDW',
};

ENCODING VPMOVDW_myrz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x33 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxDW',
};

ENCODING VPMOVSDW_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x23 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxDW',
};

ENCODING VPMOVSDW_mory_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x23 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxDW',
};

ENCODING VPMOVSDW_mory_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x23 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxDW',
};

ENCODING VPMOVSDW_mqro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x23 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxDW',
};

ENCODING VPMOVSDW_myrz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x23 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxDW',
};

ENCODING VPMOVSDW_myrz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x23 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxDW',
};

ENCODING VPMOVUSDW_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x13 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxDW',
};

ENCODING VPMOVUSDW_mory_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x13 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxDW',
};

ENCODING VPMOVUSDW_mory_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x13 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxDW',
};

ENCODING VPMOVUSDW_mqro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x13 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxDW',
};

ENCODING VPMOVUSDW_myrz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x13 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxDW',
};

ENCODING VPMOVUSDW_myrz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x13 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxDW',
};


# VPMOVQB/VPMOVSQB/VPMOVUSQB-Down Convert QWord to Byte.
ENCODING VPMOVQB_mdry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x32 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQB',
};

ENCODING VPMOVQB_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x32 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQB',
};

ENCODING VPMOVQB_mory => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x32 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQB',
};

ENCODING VPMOVQB_morz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x32 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQB',
};

ENCODING VPMOVQB_mqrz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x32 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQB',
};

ENCODING VPMOVQB_mwro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x32 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQB',
};

ENCODING VPMOVSQB_mdry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x22 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQB',
};

ENCODING VPMOVSQB_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x22 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQB',
};

ENCODING VPMOVSQB_mory => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x22 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQB',
};

ENCODING VPMOVSQB_morz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x22 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQB',
};

ENCODING VPMOVSQB_mqrz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x22 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQB',
};

ENCODING VPMOVSQB_mwro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x22 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQB',
};

ENCODING VPMOVUSQB_mdry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x12 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQB',
};

ENCODING VPMOVUSQB_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x12 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQB',
};

ENCODING VPMOVUSQB_mory => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x12 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQB',
};

ENCODING VPMOVUSQB_morz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x12 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQB',
};

ENCODING VPMOVUSQB_mqrz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x12 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQB',
};

ENCODING VPMOVUSQB_mwro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x12 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQB',
};


# VPMOVQD/VPMOVSQD/VPMOVUSQD-Down Convert QWord to DWord.
ENCODING VPMOVQD_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x35 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQD',
};

ENCODING VPMOVQD_mory_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x35 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQD',
};

ENCODING VPMOVQD_mory_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x35 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQD',
};

ENCODING VPMOVQD_mqro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x35 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQD',
};

ENCODING VPMOVQD_myrz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x35 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQD',
};

ENCODING VPMOVQD_myrz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x35 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQD',
};

ENCODING VPMOVSQD_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x25 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQD',
};

ENCODING VPMOVSQD_mory_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x25 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQD',
};

ENCODING VPMOVSQD_mory_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x25 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQD',
};

ENCODING VPMOVSQD_mqro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x25 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQD',
};

ENCODING VPMOVSQD_myrz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x25 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQD',
};

ENCODING VPMOVSQD_myrz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x25 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQD',
};

ENCODING VPMOVUSQD_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x15 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQD',
};

ENCODING VPMOVUSQD_mory_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x15 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQD',
};

ENCODING VPMOVUSQD_mory_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x15 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQD',
};

ENCODING VPMOVUSQD_mqro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x15 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQD',
};

ENCODING VPMOVUSQD_myrz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x15 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQD',
};

ENCODING VPMOVUSQD_myrz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x15 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQD',
};


# VPMOVQW/VPMOVSQW/VPMOVUSQW-Down Convert QWord to Word.
ENCODING VPMOVQW_mdro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x34 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQW',
};

ENCODING VPMOVQW_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x34 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQW',
};

ENCODING VPMOVQW_mory => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x34 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQW',
};

ENCODING VPMOVQW_morz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x34 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQW',
};

ENCODING VPMOVQW_morz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x34 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQW',
};

ENCODING VPMOVQW_mqry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x34 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQW',
};

ENCODING VPMOVSQW_mdro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x24 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQW',
};

ENCODING VPMOVSQW_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x24 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQW',
};

ENCODING VPMOVSQW_mory => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x24 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQW',
};

ENCODING VPMOVSQW_morz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x24 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQW',
};

ENCODING VPMOVSQW_morz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x24 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQW',
};

ENCODING VPMOVSQW_mqry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x24 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQW',
};

ENCODING VPMOVUSQW_mdro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x14 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQW',
};

ENCODING VPMOVUSQW_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x14 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPMOVxxQW',
};

ENCODING VPMOVUSQW_mory => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x14 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQW',
};

ENCODING VPMOVUSQW_morz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x14 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQW',
};

ENCODING VPMOVUSQW_morz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x14 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPMOVxxQW',
};

ENCODING VPMOVUSQW_mqry => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x14 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPMOVxxQW',
};


# VPMOVWB/VPMOVSWB/VPMOVUSWB-Down Convert Word to Byte.
ENCODING VPMOVSWB_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x20 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPMOVxxWB',
};

ENCODING VPMOVSWB_mory_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x20 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPMOVxxWB',
};

ENCODING VPMOVSWB_mory_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x20 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPMOVxxWB',
};

ENCODING VPMOVSWB_mqro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x20 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPMOVxxWB',
};

ENCODING VPMOVSWB_myrz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x20 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPMOVxxWB',
};

ENCODING VPMOVSWB_myrz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x20 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPMOVxxWB',
};

ENCODING VPMOVUSWB_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x10 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPMOVxxWB',
};

ENCODING VPMOVUSWB_mory_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x10 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPMOVxxWB',
};

ENCODING VPMOVUSWB_mory_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x10 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPMOVxxWB',
};

ENCODING VPMOVUSWB_mqro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x10 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPMOVxxWB',
};

ENCODING VPMOVUSWB_myrz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x10 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPMOVxxWB',
};

ENCODING VPMOVUSWB_myrz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x10 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPMOVxxWB',
};

ENCODING VPMOVWB_moro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x30 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPMOVxxWB',
};

ENCODING VPMOVWB_mory_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x30 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPMOVxxWB',
};

ENCODING VPMOVWB_mory_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x30 P66=0 PF2=0 PF3=1 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPMOVxxWB',
};

ENCODING VPMOVWB_mqro => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x30 P66=0 PF2=0 PF3=1 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPMOVxxWB',
};

ENCODING VPMOVWB_myrz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=REG MR=1 OP=0x30 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6NF',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPMOVxxWB',
};

ENCODING VPMOVWB_myrz_2 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x30 P66=0 PF2=0 PF3=1 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E6',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPMOVxxWB',
};


# VPMULTISHIFTQB.
ENCODING VPMULTISHIFTQB_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x83 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=AVX512_VBMI_128',
  tags       => 'page=VPMULTISHIFTQB',
};

ENCODING VPMULTISHIFTQB_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x83 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=AVX512_VBMI_256',
  tags       => 'page=VPMULTISHIFTQB',
};

ENCODING VPMULTISHIFTQB_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x83 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=AVX512_VBMI_512',
  tags       => 'page=VPMULTISHIFTQB',
};


# VPOPCNT-Return the Count of Number of Bits Set to 1 in BYTE/WORD/DWORD/QWORD.
ENCODING VPOPCNTx_romo_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x54 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=AVX512_BITALG_128',
  tags       => 'page=VPOPCNT',
};

ENCODING VPOPCNTx_romo_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x55 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=AVX512_VPOPCNTDQ_128',
  tags       => 'page=VPOPCNT',
};

ENCODING VPOPCNTx_rymy_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x54 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=AVX512_BITALG_256',
  tags       => 'page=VPOPCNT',
};

ENCODING VPOPCNTx_rymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x55 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=AVX512_VPOPCNTDQ_256',
  tags       => 'page=VPOPCNT',
};

ENCODING VPOPCNTx_rzmz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x54 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=AVX512_BITALG_512',
  tags       => 'page=VPOPCNT',
};

ENCODING VPOPCNTx_rzmz_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x55 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=AVX512_VPOPCNTDQ_512',
  tags       => 'page=VPOPCNT',
};


# VPROLD/VPROLVD/VPROLQ/VPROLVQ-Bit Rotate Left.
ENCODING VPROLVx_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x15 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ROTATE_LEFT',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPROLx',
};

ENCODING VPROLVx_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x15 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ROTATE_LEFT',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPROLx',
};

ENCODING VPROLVx_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x15 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ROTATE_LEFT',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPROLx',
};

ENCODING VPROLx_nomoub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=1 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ROTATE_LEFT',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPROLx',
};

ENCODING VPROLx_nymyub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=1 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ROTATE_LEFT',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPROLx',
};

ENCODING VPROLx_nzmzub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=1 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ROTATE_LEFT',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPROLx',
};


# VPRORD/VPRORVD/VPRORQ/VPRORVQ-Bit Rotate Right.
ENCODING VPRORVx_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x14 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ROTATE_RIGHT',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPRORx',
};

ENCODING VPRORVx_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x14 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ROTATE_RIGHT',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPRORx',
};

ENCODING VPRORVx_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x14 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ROTATE_RIGHT',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPRORx',
};

ENCODING VPRORx_nomoub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ROTATE_RIGHT',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPRORx',
};

ENCODING VPRORx_nymyub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ROTATE_RIGHT',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPRORx',
};

ENCODING VPRORx_nzmzub => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ROTATE_RIGHT',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPRORx',
};


# VPSCATTERDD/VPSCATTERDQ/VPSCATTERQD/VPSCATTERQQ-Scatter Packed Dword, Packed Qword with Signed Dword, Signed Qword Indices.
ENCODING VPSCATTERDx_mxro => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xa0 P66=1 PF2=0 PF3=0 RM=4 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPSCATTERxx',
};

ENCODING VPSCATTERDx_mxry => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xa0 P66=1 PF2=0 PF3=0 RM=4 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPSCATTERxx',
};

ENCODING VPSCATTERDx_mxrz => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xa0 P66=1 PF2=0 PF3=0 RM=4 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPSCATTERxx',
};

ENCODING VPSCATTERQx_mxro => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xa1 P66=1 PF2=0 PF3=0 RM=4 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPSCATTERxx',
};

ENCODING VPSCATTERQx_mxrx_1 => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xa1 P66=1 PF2=0 PF3=0 RM=4 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPSCATTERxx',
};

ENCODING VPSCATTERQx_mxrx_2 => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xa1 P66=1 PF2=0 PF3=0 RM=4 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPSCATTERxx',
};


# VPSHLD-Concatenate and Shift Packed Data Left Logical.
ENCODING VPSHLDW_ronomoub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x70 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_128',
  tags       => 'page=VPSHLD',
};

ENCODING VPSHLDW_rynymyub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x70 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_256',
  tags       => 'page=VPSHLD',
};

ENCODING VPSHLDW_rznzmzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x70 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_512',
  tags       => 'page=VPSHLD',
};

ENCODING VPSHLDx_ronomoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x71 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_128',
  tags       => 'page=VPSHLD',
};

ENCODING VPSHLDx_rynymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x71 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_256',
  tags       => 'page=VPSHLD',
};

ENCODING VPSHLDx_rznzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x71 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_512',
  tags       => 'page=VPSHLD',
};


# VPSHLDV-Concatenate and Variable Shift Packed Data Left Logical.
ENCODING VPSHLDVW_ronomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x70 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_128',
  tags       => 'page=VPSHLDV',
};

ENCODING VPSHLDVW_rynymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x70 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_256',
  tags       => 'page=VPSHLDV',
};

ENCODING VPSHLDVW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x70 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_512',
  tags       => 'page=VPSHLDV',
};

ENCODING VPSHLDVx_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x71 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_128',
  tags       => 'page=VPSHLDV',
};

ENCODING VPSHLDVx_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x71 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_256',
  tags       => 'page=VPSHLDV',
};

ENCODING VPSHLDVx_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x71 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_512',
  tags       => 'page=VPSHLDV',
};


# VPSHRD-Concatenate and Shift Packed Data Right Logical.
ENCODING VPSHRDW_ronomoub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x72 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_128',
  tags       => 'page=VPSHRD',
};

ENCODING VPSHRDW_rynymyub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x72 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_256',
  tags       => 'page=VPSHRD',
};

ENCODING VPSHRDW_rznzmzub => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f3a MR=1 OP=0x72 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_512',
  tags       => 'page=VPSHRD',
};

ENCODING VPSHRDx_ronomoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x73 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_128',
  tags       => 'page=VPSHRD',
};

ENCODING VPSHRDx_rynymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x73 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_256',
  tags       => 'page=VPSHRD',
};

ENCODING VPSHRDx_rznzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x73 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_512',
  tags       => 'page=VPSHRD',
};


# VPSHRDV-Concatenate and Variable Shift Packed Data Right Logical.
ENCODING VPSHRDVW_ronomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x72 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_128',
  tags       => 'page=VPSHRDV',
};

ENCODING VPSHRDVW_rynymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x72 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_256',
  tags       => 'page=VPSHRDV',
};

ENCODING VPSHRDVW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x72 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_512',
  tags       => 'page=VPSHRDV',
};

ENCODING VPSHRDVx_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x73 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_128',
  tags       => 'page=VPSHRDV',
};

ENCODING VPSHRDVx_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x73 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_256',
  tags       => 'page=VPSHRDV',
};

ENCODING VPSHRDVx_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x73 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512_VBMI2_512',
  tags       => 'page=VPSHRDV',
};


# VPSHUFBITQMB-Shuffle Bits from Quadword Elements Using Byte Indexes into Mask.
ENCODING VPSHUFBITQMB_nomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8f P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=AVX512_BITALG_128',
  tags       => 'page=VPSHUFBITQMB',
};

ENCODING VPSHUFBITQMB_nymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8f P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=AVX512_BITALG_256',
  tags       => 'page=VPSHUFBITQMB',
};

ENCODING VPSHUFBITQMB_nzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x8f P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=AVX512_BITALG_512',
  tags       => 'page=VPSHUFBITQMB',
};


# VPSLLVW/VPSLLVD/VPSLLVQ-Variable Bit Shift Left Logical.
ENCODING VPSLLVW_ronomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x12 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPSLLVx',
};

ENCODING VPSLLVW_rynymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x12 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPSLLVx',
};

ENCODING VPSLLVW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x12 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPSLLVx',
};

ENCODING VPSLLVx_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x47 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPSLLVx',
};

ENCODING VPSLLVx_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x47 P66=1 PF2=0 PF3=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=VPSLLVx',
};

ENCODING VPSLLVx_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x47 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPSLLVx',
};

ENCODING VPSLLVx_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x47 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPSLLVx',
};


# VPSRAVW/VPSRAVD/VPSRAVQ-Variable Bit Shift Right Arithmetic.
ENCODING VPSRAVD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x46 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX2',
  tags       => 'page=VPSRAVx',
};

ENCODING VPSRAVW_ronomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x11 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPSRAVx',
};

ENCODING VPSRAVW_rynymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x11 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPSRAVx',
};

ENCODING VPSRAVW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x11 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPSRAVx',
};

ENCODING VPSRAVx_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x46 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPSRAVx',
};

ENCODING VPSRAVx_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x46 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPSRAVx',
};

ENCODING VPSRAVx_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x46 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPSRAVx',
};


# VPSRLVW/VPSRLVD/VPSRLVQ-Variable Bit Shift Right Logical.
ENCODING VPSRLVW_ronomo => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x10 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPSRLVx',
};

ENCODING VPSRLVW_rynymy => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x10 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPSRLVx',
};

ENCODING VPSRLVW_rznzmz => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x10 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPSRLVx',
};

ENCODING VPSRLVx_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x45 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPSRLVx',
};

ENCODING VPSRLVx_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x45 P66=1 PF2=0 PF3=0',
  extensions => 'AVX2',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX2',
  tags       => 'page=VPSRLVx',
};

ENCODING VPSRLVx_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x45 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPSRLVx',
};

ENCODING VPSRLVx_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x45 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPSRLVx',
};


# VPTERNLOGD/VPTERNLOGQ-Bitwise Ternary Logic.
ENCODING VPTERNLOGx_ronomoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x25 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPTERNLOGx',
};

ENCODING VPTERNLOGx_rynymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x25 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPTERNLOGx',
};

ENCODING VPTERNLOGx_rznzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x25 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPTERNLOGx',
};


# VPTESTMB/VPTESTMW/VPTESTMD/VPTESTMQ-Logical AND and Set Mask.
ENCODING VPTESTMx_nomo_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x26 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPTESTMx',
};

ENCODING VPTESTMx_nomo_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x27 P66=1 PF2=0 PF3=0 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPTESTMx',
};

ENCODING VPTESTMx_nymy_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x26 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPTESTMx',
};

ENCODING VPTESTMx_nymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x27 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPTESTMx',
};

ENCODING VPTESTMx_nzmz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x26 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPTESTMx',
};

ENCODING VPTESTMx_nzmz_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x27 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPTESTMx',
};


# VPTESTNMB/W/D/Q-Logical NAND and Set.
ENCODING VPTESTNMx_nomo_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x26 P66=0 PF2=0 PF3=1 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512BW_128',
  tags       => 'page=VPTESTNMx',
};

ENCODING VPTESTNMx_nomo_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x27 P66=0 PF2=0 PF3=1 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VPTESTNMx',
};

ENCODING VPTESTNMx_nymy_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x26 P66=0 PF2=0 PF3=1 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512BW_256',
  tags       => 'page=VPTESTNMx',
};

ENCODING VPTESTNMx_nymy_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x27 P66=0 PF2=0 PF3=1 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VPTESTNMx',
};

ENCODING VPTESTNMx_nzmz_1 => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x26 P66=0 PF2=0 PF3=1 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512BW_512',
  tags       => 'page=VPTESTNMx',
};

ENCODING VPTESTNMx_nzmz_2 => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x27 P66=0 PF2=0 PF3=1 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VPTESTNMx',
};


# VRANGEPD-Range Restriction Calculation For Packed Pairs of Float64 Values.
ENCODING VRANGEPD_ronomoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x50 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VRANGEPD',
};

ENCODING VRANGEPD_rynymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x50 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VRANGEPD',
};

ENCODING VRANGEPD_rznzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x50 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VRANGEPD',
};


# VRANGEPS-Range Restriction Calculation For Packed Pairs of Float32 Values.
ENCODING VRANGEPS_ronomoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x50 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VRANGEPS',
};

ENCODING VRANGEPS_rynymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x50 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VRANGEPS',
};

ENCODING VRANGEPS_rznzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x50 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VRANGEPS',
};


# VRANGESD-Range Restriction Calculation From a pair of Scalar Float64 Values.
ENCODING VRANGESD => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x51 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_SCALAR',
  tags       => 'page=VRANGESD',
};


# VRANGESS-Range Restriction Calculation From a Pair of Scalar Float32 Values.
ENCODING VRANGESS => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x51 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_SCALAR',
  tags       => 'page=VRANGESS',
};


# VRCP14PD-Compute Approximate Reciprocals of Packed Float64 Values.
ENCODING VRCP14PD_romo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x4c P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VRCP14PD',
};

ENCODING VRCP14PD_rymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x4c P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VRCP14PD',
};

ENCODING VRCP14PD_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x4c P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VRCP14PD',
};


# VRCP14PS-Compute Approximate Reciprocals of Packed Float32 Values.
ENCODING VRCP14PS_romo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x4c P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VRCP14PS',
};

ENCODING VRCP14PS_rymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x4c P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VRCP14PS',
};

ENCODING VRCP14PS_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x4c P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VRCP14PS',
};


# VRCP14SD-Compute Approximate Reciprocal of Scalar Float64 Value.
ENCODING VRCP14SD => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x4d P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E10',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VRCP14SD',
};


# VRCP14SS-Compute Approximate Reciprocal of Scalar Float32 Value.
ENCODING VRCP14SS => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x4d P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E10',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VRCP14SS',
};


# VREDUCEPD-Perform Reduction Transformation on Packed Float64 Values.
ENCODING VREDUCEPD_romoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x56 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VREDUCEPD',
};

ENCODING VREDUCEPD_rymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x56 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VREDUCEPD',
};

ENCODING VREDUCEPD_rzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x56 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VREDUCEPD',
};


# VREDUCEPS-Perform Reduction Transformation on Packed Float32 Values.
ENCODING VREDUCEPS_romoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x56 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=VREDUCEPS',
};

ENCODING VREDUCEPS_rymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x56 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=VREDUCEPS',
};

ENCODING VREDUCEPS_rzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x56 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=VREDUCEPS',
};


# VREDUCESD-Perform a Reduction Transformation on a Scalar Float64 Value.
ENCODING VREDUCESD => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x57 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_SCALAR',
  tags       => 'page=VREDUCESD',
};


# VREDUCESS-Perform a Reduction Transformation on a Scalar Float32 Value.
ENCODING VREDUCESS => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x57 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512DQ_SCALAR',
  tags       => 'page=VREDUCESS',
};


# VRNDSCALEPD-Round Packed Float64 Values To Include A Given Number Of Fraction Bits.
ENCODING VRNDSCALEPD_romoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x09 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VRNDSCALEPD',
};

ENCODING VRNDSCALEPD_rymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x09 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VRNDSCALEPD',
};

ENCODING VRNDSCALEPD_rzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x09 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VRNDSCALEPD',
};


# VRNDSCALEPS-Round Packed Float32 Values To Include A Given Number Of Fraction Bits.
ENCODING VRNDSCALEPS_romoub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x08 P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VRNDSCALEPS',
};

ENCODING VRNDSCALEPS_rymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x08 P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VRNDSCALEPS',
};

ENCODING VRNDSCALEPS_rzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x08 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VRNDSCALEPS',
};


# VRNDSCALESD-Round Scalar Float64 Value To Include A Given Number Of Fraction Bits.
ENCODING VRNDSCALESD => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x0b P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VRNDSCALESD',
};


# VRNDSCALESS-Round Scalar Float32 Value To Include A Given Number Of Fraction Bits.
ENCODING VRNDSCALESS => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x0a P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VRNDSCALESS',
};


# VRSQRT14PD-Compute Approximate Reciprocals of Square Roots of Packed Float64 Values.
ENCODING VRSQRT14PD_romo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x4e P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VRSQRT14PD',
};

ENCODING VRSQRT14PD_rymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x4e P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VRSQRT14PD',
};

ENCODING VRSQRT14PD_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x4e P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VRSQRT14PD',
};


# VRSQRT14PS-Compute Approximate Reciprocals of Square Roots of Packed Float32 Values.
ENCODING VRSQRT14PS_romo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x4e P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VRSQRT14PS',
};

ENCODING VRSQRT14PS_rymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x4e P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VRSQRT14PS',
};

ENCODING VRSQRT14PS_rzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x4e P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VRSQRT14PS',
};


# VRSQRT14SD-Compute Approximate Reciprocal of Square Root of Scalar Float64 Value.
ENCODING VRSQRT14SD => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x4f P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E10',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VRSQRT14SD',
};


# VRSQRT14SS-Compute Approximate Reciprocal of Square Root of Scalar Float32 Value.
ENCODING VRSQRT14SS => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MR=1 OP=0x4f P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E10',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VRSQRT14SS',
};


# VSCALEFPD-Scale Packed Float64 Values With Float64 Values.
ENCODING VSCALEFPD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x2c P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VSCALEFPD',
};

ENCODING VSCALEFPD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x2c P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VSCALEFPD',
};

ENCODING VSCALEFPD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x2c P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VSCALEFPD',
};


# VSCALEFPS-Scale Packed Float32 Values With Float32 Values.
ENCODING VSCALEFPS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x2c P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VSCALEFPS',
};

ENCODING VSCALEFPS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x2c P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VSCALEFPS',
};

ENCODING VSCALEFPS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x2c P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VSCALEFPS',
};


# VSCALEFSD-Scale Scalar Float64 Values With Float64 Values.
ENCODING VSCALEFSD => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x2d P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VSCALEFSD',
};


# VSCALEFSS-Scale Scalar Float32 Value With Float32 Value.
ENCODING VSCALEFSS => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0x2d P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_SCALAR',
  tags       => 'page=VSCALEFSS',
};


# VSCATTERDPS/VSCATTERDPD/VSCATTERQPS/VSCATTERQPD-Scatter Packed Single, Packed Double with Signed Dword and Qword Indices.
ENCODING VSCATTERDPx_mxro => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xa2 P66=1 PF2=0 PF3=0 RM=4 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VSCATTERxPx',
};

ENCODING VSCATTERDPx_mxry => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xa2 P66=1 PF2=0 PF3=0 RM=4 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VSCATTERxPx',
};

ENCODING VSCATTERDPx_mxrz => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xa2 P66=1 PF2=0 PF3=0 RM=4 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VSCATTERxPx',
};

ENCODING VSCATTERQPx_mxro => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xa3 P66=1 PF2=0 PF3=0 RM=4 VL=128',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=VSCATTERxPx',
};

ENCODING VSCATTERQPx_mxrx_1 => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xa3 P66=1 PF2=0 PF3=0 RM=4 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VSCATTERxPx',
};

ENCODING VSCATTERQPx_mxrx_2 => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xa3 P66=1 PF2=0 PF3=0 RM=4 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VSCATTERxPx',
};


# VSHUFF32x4/VSHUFF64x2/VSHUFI32x4/VSHUFI64x2-Shuffle Packed Values at 128-bit Granularity.
ENCODING VSHUFFxxXx_rynymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x23 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VSHUFFIDQ',
};

ENCODING VSHUFFxxXx_rznzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x23 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VSHUFFIDQ',
};

ENCODING VSHUFIxxXx_rynymyub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x43 P66=1 PF2=0 PF3=0 VL=256',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=VSHUFFIDQ',
};

ENCODING VSHUFIxxXx_rznzmzub => {
  diagram    => 'ENC=EVEX MAP=0f3a MR=1 OP=0x43 P66=1 PF2=0 PF3=0 VL=512',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4NF',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=VSHUFFIDQ',
};


# VTESTPD/VTESTPS-Packed Bit Test.
ENCODING VTESTPD => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x0f P66=1 PF2=0 PF3=0 W=0',
  iflags     => 'cf=W zf=W',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX',
  tags       => 'page=VTESTPx',
};

ENCODING VTESTPS => {
  diagram    => 'ENC=VEX MAP=0f38 MR=1 OP=0x0e P66=1 PF2=0 PF3=0 W=0',
  iflags     => 'cf=W zf=W',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX',
  tags       => 'page=VTESTPx',
};


# VZEROALL-Zero XMM, YMM and ZMM Registers.
ENCODING VZEROALL => {
  diagram    => 'ENC=VEX MAP=0f OP=0x77 P66=0 PF2=0 PF3=0 VL=256',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_8',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  docvars    => 'XMM_STATE_W=1',
  tags       => 'page=VZEROALL',
};


# VZEROUPPER-Zero Upper Bits of YMM and ZMM Registers.
ENCODING VZEROUPPER => {
  diagram    => 'ENC=VEX MAP=0f OP=0x77 P66=0 PF2=0 PF3=0 VL=128',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_8',
  categories => 'ADVSIMD',
  metadata   => 'isa=AVX',
  docvars    => 'XMM_STATE_W=1',
  tags       => 'page=VZEROUPPER',
};


# VPANDD-Bitwise AND Int32 Vectors.
ENCODING VPANDD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xdb P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=XEON_VPANDD',
};

ENCODING VPANDD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xdb P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=XEON_VPANDD',
};

ENCODING VPANDD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xdb P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=XEON_VPANDD',
};


# VPANDND-Bitwise AND NOT Int32 Vectors.
ENCODING VPANDND_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xdf P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=XEON_VPANDND',
};

ENCODING VPANDND_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xdf P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=XEON_VPANDND',
};

ENCODING VPANDND_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xdf P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=XEON_VPANDND',
};


# VPANDNQ-Bitwise AND NOT Int64 Vectors.
ENCODING VPANDNQ_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xdf P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=XEON_VPANDNQ',
};

ENCODING VPANDNQ_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xdf P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=XEON_VPANDNQ',
};

ENCODING VPANDNQ_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xdf P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=XEON_VPANDNQ',
};


# VPANDQ-Bitwise AND Int64 Vectors.
ENCODING VPANDQ_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xdb P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=XEON_VPANDQ',
};

ENCODING VPANDQ_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xdb P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=XEON_VPANDQ',
};

ENCODING VPANDQ_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xdb P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=XEON_VPANDQ',
};


# VPORD-Bitwise OR Int32 Vectors.
ENCODING VPORD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xeb P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=XEON_VPORD',
};

ENCODING VPORD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xeb P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=XEON_VPORD',
};

ENCODING VPORD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xeb P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=XEON_VPORD',
};


# VPORQ-Bitwise OR Int64 Vectors.
ENCODING VPORQ_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xeb P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=XEON_VPORQ',
};

ENCODING VPORQ_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xeb P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=XEON_VPORQ',
};

ENCODING VPORQ_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xeb P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=XEON_VPORQ',
};


# VPXORD-Bitwise XOR Int32 Vectors.
ENCODING VPXORD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xef P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=XEON_VPXORD',
};

ENCODING VPXORD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xef P66=1 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=XEON_VPXORD',
};

ENCODING VPXORD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xef P66=1 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=XEON_VPXORD',
};


# VPXORQ-Bitwise XOR Int64 Vectors.
ENCODING VPXORQ_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xef P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_128',
  tags       => 'page=XEON_VPXORQ',
};

ENCODING VPXORQ_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xef P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_256',
  tags       => 'page=XEON_VPXORQ',
};

ENCODING VPXORQ_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0xef P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=AVX512F_512',
  tags       => 'page=XEON_VPXORQ',
};


# XORPD-Bitwise Logical XOR of Packed Double Precision Floating-Point Values.
ENCODING VXORPD_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x57 P66=1 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=XORPD',
};

ENCODING VXORPD_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x57 P66=1 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX',
  tags       => 'page=XORPD',
};

ENCODING VXORPD_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x57 P66=1 PF2=0 PF3=0 VL=256 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=XORPD',
};

ENCODING VXORPD_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x57 P66=1 PF2=0 PF3=0 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=XORPD',
};


# XORPS-Bitwise Logical XOR of Packed Single Precision Floating-Point Values.
ENCODING VXORPS_ronomo => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x57 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_128',
  tags       => 'page=XORPS',
};

ENCODING VXORPS_rxnxmx => {
  diagram    => 'ENC=VEX MAP=0f MR=1 OP=0x57 P66=0 PF2=0 PF3=0',
  extensions => 'AVX',
  exceptions => 'AVX_TYPE_4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX',
  tags       => 'page=XORPS',
};

ENCODING VXORPS_rynymy => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x57 P66=0 PF2=0 PF3=0 VL=256 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_256',
  tags       => 'page=XORPS',
};

ENCODING VXORPS_rznzmz => {
  diagram    => 'ENC=EVEX MAP=0f MR=1 OP=0x57 P66=0 PF2=0 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|BITWISE|LOGICAL_FP',
  metadata   => 'isa=AVX512DQ_512',
  tags       => 'page=XORPS',
};

