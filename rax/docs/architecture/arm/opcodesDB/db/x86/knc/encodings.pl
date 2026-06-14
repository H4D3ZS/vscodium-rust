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


# NOP-No Operation.
ENCODING NOP_mv_11 => {
  diagram    => 'KNC=1 MAP=0f MR=1 OP=0x1f REG=0',
  extensions => 'BASE',
  categories => 'GENERAL|NOP',
  metadata   => 'isa=KNC_MISC',
  tags       => 'page=NOP',
};


# CLEVICT0-Evict L1 line.
ENCODING CLEVICT0_mb_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MODE=64 MR=1 OP=0xae P66=0 PF2=1 PF3=0 REG=7 VL=128',
  extensions => 'KNC',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_CLEVICT0',
};

ENCODING CLEVICT0_mb_2 => {
  diagram    => 'ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0xae P66=0 PF2=1 PF3=0 REG=7 VL=512',
  extensions => 'KNCE',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_CLEVICT0',
};


# CLEVICT1-Evict L2 line.
ENCODING CLEVICT1_mb_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MODE=64 MR=1 OP=0xae P66=0 PF2=0 PF3=1 REG=7 VL=128',
  extensions => 'KNC',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_CLEVICT1',
};

ENCODING CLEVICT1_mb_2 => {
  diagram    => 'ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0xae P66=0 PF2=0 PF3=1 REG=7 VL=512',
  extensions => 'KNCE',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_CLEVICT1',
};


# DELAY-Stall Thread.
ENCODING DELAY => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MODE=64 MR=1 OP=0xae P66=0 PF2=0 PF3=1 REG=6 VL=128',
  extensions => 'KNC',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_DELAY',
};


# JKNZD-Jump near if mask is not zero.
ENCODING JKNZD_s8 => {
  diagram    => 'ENC=VEX MODE=64 OP=0x75 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'KNCV',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=KNCJKBR',
  tags       => 'page=XEON_JKNZD',
};

ENCODING JKNZD_sz => {
  diagram    => 'ENC=VEX MAP=0f MODE=64 OP=0x85 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'KNCV',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=KNCJKBR',
  tags       => 'page=XEON_JKNZD',
};


# JKZD-Jump near if mask is zero.
ENCODING JKZD_s8 => {
  diagram    => 'ENC=VEX MODE=64 OP=0x74 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'KNCV',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=KNCJKBR',
  tags       => 'page=XEON_JKZD',
};

ENCODING JKZD_sz => {
  diagram    => 'ENC=VEX MAP=0f MODE=64 OP=0x84 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'KNCV',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY|SHORT',
  metadata   => 'isa=KNCJKBR',
  tags       => 'page=XEON_JKZD',
};


# KAND-AND Vector Mask.
ENCODING KAND => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MODE=64 MR=1 OP=0x41 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'KNC',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_KAND',
};


# KANDN-AND NOT Vector Mask.
ENCODING KANDN => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MODE=64 MR=1 OP=0x42 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'KNC',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_KANDN',
};


# KANDNR-Reverse AND NOT Vector Mask.
ENCODING KANDNR => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MODE=64 MR=1 OP=0x43 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'KNC',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_KANDNR',
};


# KCONCATH-Pack and Move High Vector Mask.
ENCODING KCONCATH => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MODE=64 MR=1 OP=0x95 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'KNC',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_KCONCATH',
};


# KCONCATL-Pack and Move Low Vector Mask.
ENCODING KCONCATL => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MODE=64 MR=1 OP=0x97 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'KNC',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_KCONCATL',
};


# KEXTRACT-Extract Vector Mask From Register.
ENCODING KEXTRACT => {
  diagram    => 'ENC=VEX MAP=0f3a MOD=REG MODE=64 MR=1 OP=0x3e P66=1 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'KNC',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_KEXTRACT',
};


# KMERGE2L1H-Swap and Merge High Element Portion and Low Portion of Vector Masks.
ENCODING KMERGE2L1H => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MODE=64 MR=1 OP=0x48 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'KNC',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_KMERGE2L1H',
};


# KMERGE2L1L-Move Low Element Portion into High Portion of Vector Mask.
ENCODING KMERGE2L1L => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MODE=64 MR=1 OP=0x49 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'KNC',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_KMERGE2L1L',
};


# KMOV-Move Vector Mask.
ENCODING KMOV_V => {
  diagram    => 'ENC=VEX KNC=1 MAP=0f MOD=REG MODE=64 MR=1 OP=0x90 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'KNC',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_KMOV',
};

ENCODING KMOV_mq => {
  diagram    => 'ENC=VEX KNC=1 MAP=0f MOD=REG MODE=64 MR=1 OP=0x92 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'KNC',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_KMOV',
};

ENCODING KMOV_rq => {
  diagram    => 'ENC=VEX KNC=1 MAP=0f MOD=REG MODE=64 MR=1 OP=0x93 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'KNC',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_KMOV',
};


# KNOT-Not Vector Mask.
ENCODING KNOT => {
  diagram    => 'ENC=VEX KNC=1 MAP=0f MOD=REG MODE=64 MR=1 OP=0x44 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'KNC',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_KNOT',
};


# KOR-OR Vector Masks.
ENCODING KOR => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MODE=64 MR=1 OP=0x45 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'KNC',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_KOR',
};


# KORTEST-OR Vector Mask And Set EFLAGS.
ENCODING KORTEST => {
  diagram    => 'ENC=VEX KNC=1 MAP=0f MOD=REG MODE=64 MR=1 OP=0x98 P66=0 PF2=0 PF3=0 VL=128 W=0',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'KNC',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_KORTEST',
};


# KXNOR-XNOR Vector Masks.
ENCODING KXNOR => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MODE=64 MR=1 OP=0x46 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'KNC',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_KXNOR',
};


# KXOR-XOR Vector Masks.
ENCODING KXOR => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MODE=64 MR=1 OP=0x47 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'KNC',
  categories => 'ADVSIMD|MASK',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_KXOR',
};


# LZCNT-Leading Zero Count.
ENCODING LZCNT_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MODE=64 MR=1 OP=0xbd P66=0 PF2=0 PF3=1 VL=128',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'KNC',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_LZCNT',
};


# POPCNT-Return the Count of Number of Bits Set to 1.
ENCODING POPCNT_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MODE=64 MR=1 OP=0xb8 P66=0 PF2=0 PF3=1 VL=128',
  iflags     => 'af=Z cf=Z of=Z pf=Z sf=Z zf=W',
  extensions => 'KNC',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_POPCNT',
};


# PREFETCHWT1-Prefetch Vector Data Into Caches with Intent to Write and T1 Hint.
ENCODING PREFETCHWT1 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x0d REG=2',
  extensions => 'PREFETCHWT1',
  categories => 'GENERAL',
  metadata   => 'isa=PREFETCHWT1',
  tags       => 'page=XEON_PREFETCHWT1',
};


# SPFLT-Set performance monitor filtering mask.
ENCODING SPFLT => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MODE=64 MR=1 OP=0xae P66=0 PF2=1 PF3=0 REG=6 VL=128',
  extensions => 'KNC',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_SPFLT',
};


# TZCNT-Trailing Zero Count.
ENCODING TZCNT_rxmx => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MODE=64 MR=1 OP=0xbc P66=0 PF2=0 PF3=1 VL=128',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'KNC',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_TZCNT',
};


# TZCNTI-Initialized Trailing Zero Count.
ENCODING TZCNTI => {
  diagram    => 'ENC=VEX MAP=0f MOD=REG MODE=64 MR=1 OP=0xbc P66=0 PF2=1 PF3=0 VL=128',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'KNC',
  categories => 'ADVSIMD|BITFIELD',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_TZCNTI',
};


# V4FMADDPS-Packed Single-Precision Floating-Point Fused Multiply-Add (4-iterations).
ENCODING V4FMADDPS => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x9a P66=0 PF2=1 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512_4FMAPS_512',
  tags       => 'page=XEON_V4FMADDPS',
};


# V4FMADDSS-Scalar Single-Precision Floating-Point Fused Multiply-Add (4-iterations).
ENCODING V4FMADDSS => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x9b P66=0 PF2=1 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512_4FMAPS_SCALAR',
  tags       => 'page=XEON_V4FMADDSS',
};


# V4FNMADDPS-Packed Single-Precision Floating-Point Fused Multiply-Add (4-iterations).
ENCODING V4FNMADDPS => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xaa P66=0 PF2=1 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512_4FMAPS_512',
  tags       => 'page=XEON_V4FNMADDPS',
};


# V4FNMADDSS-Scalar Single-Precision Floating-Point Fused Multiply-Add (4-iterations).
ENCODING V4FNMADDSS => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xab P66=0 PF2=1 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=AVX512_4FMAPS_SCALAR',
  tags       => 'page=XEON_V4FNMADDSS',
};


# VADDNPD-Add and Negate Float64 Vectors.
ENCODING VADDNPD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x50 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VADDNPD',
};


# VADDNPS-Add and Negate Float32 Vectors.
ENCODING VADDNPS => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x50 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VADDNPS',
};


# VADDPD-Add Float64 Vectors.
ENCODING VADDPD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x58 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VADDPD',
};


# VADDPS-Add Float32 Vectors.
ENCODING VADDPS_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x58 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VADDPS',
};


# VADDSETSPS-Add Float32 Vectors and Set Mask to Sign.
ENCODING VADDSETSPS_rznzm0 => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xcc P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VADDSETSPS',
};

ENCODING VADDSETSPS_rznzmz => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=REG MR=1 OP=0xcc P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VADDSETSPS',
};


# VALIGND-Align Doubleword Vectors.
ENCODING VALIGND => {
  diagram    => 'ENC=MVEX MAP=0f3a MR=1 OP=0x03 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VALIGND',
};


# VBLENDMPD-Blend Float64 Vectors using the Instruction Mask.
ENCODING VBLENDMPD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x65 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VBLENDMPD',
};


# VBLENDMPS-Blend Float32 Vectors using the Instruction Mask.
ENCODING VBLENDMPS => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x65 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VBLENDMPS',
};


# VBROADCASTF32X4-Broadcast 4xFloat32 Vector.
ENCODING VBROADCASTF32X4_rzm0 => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0x1a P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VBROADCASTF32X4',
};


# VBROADCASTF64X4-Broadcast 4xFloat64 Vector.
ENCODING VBROADCASTF64X4_rzm0 => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0x1b P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|BROADCASTING|LOAD',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VBROADCASTF64X4',
};


# VBROADCASTI32X4-Broadcast 4xInt32 Vector.
ENCODING VBROADCASTI32X4_rzm0 => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0x5a P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VBROADCASTI32X4',
};


# VBROADCASTI64X4-Broadcast 4xInt64 Vector.
ENCODING VBROADCASTI64X4_rzm0 => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0x5b P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|BROADCASTING|LOAD',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VBROADCASTI64X4',
};


# VBROADCASTSD-Broadcast Float64 Vector.
ENCODING VBROADCASTSD_rzm0 => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0x19 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VBROADCASTSD',
};


# VBROADCASTSS-Broadcast Float32 Vector.
ENCODING VBROADCASTSS_rzm0 => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0x18 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|BROADCASTING',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VBROADCASTSS',
};


# VCMPPD-Compare Float64 Vectors and Set Vector Mask.
ENCODING VCMPPD_nzmxub => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0xc2 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VCMPPD',
};


# VCMPPS-Compare Float32 Vectors and Set Vector Mask.
ENCODING VCMPPS_nzmxub => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0xc2 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VCMPPS',
};


# VCVTDQ2PD-Convert Int32 Vector to Float64 Vector.
ENCODING VCVTDQ2PD_rzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0xe6 P66=0 PF2=0 PF3=1 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VCVTDQ2PD',
};


# VCVTFXPNTDQ2PS-Convert Fixed Point Int32 Vector to Float32 Vector.
ENCODING VCVTFXPNTDQ2PS => {
  diagram    => 'ENC=MVEX MAP=0f3a MR=1 OP=0xcb P66=0 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VCVTFXPNTDQ2PS',
};


# VCVTFXPNTPD2DQ-Convert Float64 Vector to Fixed Point Int32 Vector.
ENCODING VCVTFXPNTPD2DQ => {
  diagram    => 'ENC=MVEX MAP=0f3a MR=1 OP=0xe6 P66=0 PF2=1 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VCVTFXPNTPD2DQ',
};


# VCVTFXPNTPD2UDQ-Convert Float64 Vector to Fixed Point Uint32 Vector.
ENCODING VCVTFXPNTPD2UDQ => {
  diagram    => 'ENC=MVEX MAP=0f3a MR=1 OP=0xca P66=0 PF2=1 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VCVTFXPNTPD2UDQ',
};


# VCVTFXPNTPS2DQ-Convert Float32 Vector to Fixed Point Int32 Vector.
ENCODING VCVTFXPNTPS2DQ => {
  diagram    => 'ENC=MVEX MAP=0f3a MR=1 OP=0xcb P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VCVTFXPNTPS2DQ',
};


# VCVTFXPNTPS2UDQ-Convert Float32 Vector to Fixed Point Uint32 Vector.
ENCODING VCVTFXPNTPS2UDQ => {
  diagram    => 'ENC=MVEX MAP=0f3a MR=1 OP=0xca P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VCVTFXPNTPS2UDQ',
};


# VCVTFXPNTUDQ2PS-Convert Fixed Point Uint32 Vector to Float32 Vector.
ENCODING VCVTFXPNTUDQ2PS => {
  diagram    => 'ENC=MVEX MAP=0f3a MR=1 OP=0xca P66=0 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VCVTFXPNTUDQ2PS',
};


# VCVTPD2PS-Convert Float64 Vector to Float32 Vector.
ENCODING VCVTPD2PS_rzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x5a P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VCVTPD2PS',
};


# VCVTPS2PD-Convert Float32 Vector to Float64 Vector.
ENCODING VCVTPS2PD_rzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x5a P66=0 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VCVTPS2PD',
};


# VCVTUDQ2PD-Convert Uint32 Vector to Float64 Vector.
ENCODING VCVTUDQ2PD_rzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x7a P66=0 PF2=0 PF3=1 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VCVTUDQ2PD',
};


# VEXP223PS-Base-2 Exponential Calculation of Float32 Vector.
ENCODING VEXP223PS => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xc8 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VEXP223PS',
};


# VEXP2PD-Approximation to the Exponential 2^x of Packed Double-Precision Floating-Point Values with Less Than 2^-23 Relative Error.
ENCODING VEXP2PD => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xc8 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX512ER_512',
  tags       => 'page=XEON_VEXP2PD',
};


# VEXP2PS-Approximation to the Exponential 2^x of Packed Single-Precision Floating-Point Values with Less Than 2^-23 Relative Error.
ENCODING VEXP2PS => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xc8 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX512ER_512',
  tags       => 'page=XEON_VEXP2PS',
};


# VFIXUPNANPD-Fix Up Special Float64 Vector Numbers With NaN Passthrough.
ENCODING VFIXUPNANPD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x55 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFIXUPNANPD',
};


# VFIXUPNANPS-Fix Up Special Float32 Vector Numbers With NaN Passthrough.
ENCODING VFIXUPNANPS => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x55 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFIXUPNANPS',
};


# VFMADD132PD-Multiply Destination By Second Source and Add To First Source Float64 Vectors 201.
ENCODING VFMADD132PD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x98 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFMADD132PD',
};


# VFMADD132PS-Multiply Destination By Second Source and Add To First Source Float32 Vectors 205.
ENCODING VFMADD132PS_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x98 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFMADD132PS',
};


# VFMADD213PD-Multiply First Source By Destination and Add Second Source Float64 Vectors . 208.
ENCODING VFMADD213PD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xa8 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFMADD213PD',
};


# VFMADD213PS-Multiply First Source By Destination and Add Second Source Float32 Vectors . . 212.
ENCODING VFMADD213PS_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xa8 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFMADD213PS',
};


# VFMADD231PD-Multiply First Source By Second Source and Add To Destination Float64 Vectors 216.
ENCODING VFMADD231PD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xb8 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFMADD231PD',
};


# VFMADD231PS-Multiply First Source By Second Source and Add To Destination Float32 Vectors 220.
ENCODING VFMADD231PS_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xb8 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFMADD231PS',
};


# VFMADD233PS-Multiply First Source By Specially Swizzled Second Source and Add To Second Source Float32 Vectors.
ENCODING VFMADD233PS => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xa4 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFMADD233PS',
};


# VFMSUB132PD-Multiply Destination By Second Source and Subtract First Source Float64 Vectors228.
ENCODING VFMSUB132PD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x9a P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFMSUB132PD',
};


# VFMSUB132PS-Multiply Destination By Second Source and Subtract First Source Float32 Vectors232.
ENCODING VFMSUB132PS_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x9a P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFMSUB132PS',
};


# VFMSUB213PD-Multiply First Source By Destination and Subtract Second Source Float64 Vectors235.
ENCODING VFMSUB213PD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xaa P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFMSUB213PD',
};


# VFMSUB213PS-Multiply First Source By Destination and Subtract Second Source Float32 Vectors239.
ENCODING VFMSUB213PS_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xaa P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFMSUB213PS',
};


# VFMSUB231PD-Multiply First Source By Second Source and Subtract Destination Float64 Vectors242.
ENCODING VFMSUB231PD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xba P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFMSUB231PD',
};


# VFMSUB231PS-Multiply First Source By Second Source and Subtract Destination Float32 Vectors246.
ENCODING VFMSUB231PS_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xba P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFMSUB231PS',
};


# VFNMADD132PD-Multiply Destination By Second Source and Subtract From First Source Float64 Vectors.
ENCODING VFNMADD132PD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x9c P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFNMADD132PD',
};


# VFNMADD132PS-Multiply Destination By Second Source and Subtract From First Source Float32 Vectors.
ENCODING VFNMADD132PS_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x9c P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFNMADD132PS',
};


# VFNMADD213PD-Multiply First Source By Destination and Subtract From Second Source Float64 Vectors.
ENCODING VFNMADD213PD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xac P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFNMADD213PD',
};


# VFNMADD213PS-Multiply First Source By Destination and Subtract From Second Source Float32 Vectors.
ENCODING VFNMADD213PS_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xac P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFNMADD213PS',
};


# VFNMADD231PD-Multiply First Source By Second Source and Subtract From Destination Float64 Vectors.
ENCODING VFNMADD231PD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xbc P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFNMADD231PD',
};


# VFNMADD231PS-Multiply First Source By Second Source and Subtract From Destination Float32 Vectors.
ENCODING VFNMADD231PS_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xbc P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFNMADD231PS',
};


# VFNMSUB132PD-Multiply Destination By Second Source, Negate, and Subtract First Source Float64 Vectors.
ENCODING VFNMSUB132PD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x9e P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFNMSUB132PD',
};


# VFNMSUB132PS-Multiply Destination By Second Source, Negate, and Subtract First Source Float32 Vectors.
ENCODING VFNMSUB132PS_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x9e P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFNMSUB132PS',
};


# VFNMSUB213PD-Multiply First Source By Destination, Negate, and Subtract Second Source Float64 Vectors.
ENCODING VFNMSUB213PD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xae P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFNMSUB213PD',
};


# VFNMSUB213PS-Multiply First Source By Destination, Negate, and Subtract Second Source Float32 Vectors.
ENCODING VFNMSUB213PS_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xae P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFNMSUB213PS',
};


# VFNMSUB231PD-Multiply First Source By Second Source, Negate, and Subtract Destination Float64 Vectors.
ENCODING VFNMSUB231PD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xbe P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFNMSUB231PD',
};


# VFNMSUB231PS-Multiply First Source By Second Source, Negate, and Subtract Destination Float32 Vectors.
ENCODING VFNMSUB231PS_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xbe P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|FUSED_OP',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VFNMSUB231PS',
};


# VGATHERDPD-Gather Float64 Vector With Signed Dword Indices.
ENCODING VGATHERDPD => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0x92 P66=1 PF2=0 PF3=0 RM=4 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VGATHERDPD',
};


# VGATHERDPS-Gather Float32 Vector With Signed Dword Indices.
ENCODING VGATHERDPS => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0x92 P66=1 PF2=0 PF3=0 RM=4 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VGATHERDPS',
};


# VGATHERPF0DPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint.
ENCODING VGATHERPF0DPD => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 REG=1 RM=4 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12NP',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512PF_512',
  tags       => 'page=XEON_VGATHERPF0DPD',
};


# VGATHERPF0DPS-Gather Prefetch Float32 Vector With Signed Dword Indices Into L1.
ENCODING VGATHERPF0DPS_m0 => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 REG=1 RM=4 W=0',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VGATHERPF0DPS',
};

ENCODING VGATHERPF0DPS_md => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 REG=1 RM=4 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12NP',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512PF_512',
  tags       => 'page=XEON_VGATHERPF0DPS',
};


# VGATHERPF0HINTDPD-Gather Prefetch Float64 Vector Hint With Signed Dword Indices.
ENCODING VGATHERPF0HINTDPD => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 REG=0 RM=4 W=1',
  extensions => 'KNCE',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNC_PF_HINT',
  tags       => 'page=XEON_VGATHERPF0HINTDPD',
};


# VGATHERPF0HINTDPS-Gather Prefetch Float32 Vector Hint With Signed Dword Indices.
ENCODING VGATHERPF0HINTDPS => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 REG=0 RM=4 W=0',
  extensions => 'KNCE',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNC_PF_HINT',
  tags       => 'page=XEON_VGATHERPF0HINTDPS',
};


# VGATHERPF0QPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint.
ENCODING VGATHERPF0QPD => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xc7 P66=1 PF2=0 PF3=0 REG=1 RM=4 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12NP',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512PF_512',
  tags       => 'page=XEON_VGATHERPF0QPD',
};


# VGATHERPF0QPS-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint.
ENCODING VGATHERPF0QPS => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xc7 P66=1 PF2=0 PF3=0 REG=1 RM=4 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12NP',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512PF_512',
  tags       => 'page=XEON_VGATHERPF0QPS',
};


# VGATHERPF1DPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint.
ENCODING VGATHERPF1DPD => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 REG=2 RM=4 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12NP',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512PF_512',
  tags       => 'page=XEON_VGATHERPF1DPD',
};


# VGATHERPF1DPS-Gather Prefetch Float32 Vector With Signed Dword Indices Into L2.
ENCODING VGATHERPF1DPS_m0 => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 REG=2 RM=4 W=0',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VGATHERPF1DPS',
};

ENCODING VGATHERPF1DPS_md => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 REG=2 RM=4 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12NP',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512PF_512',
  tags       => 'page=XEON_VGATHERPF1DPS',
};


# VGATHERPF1QPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint.
ENCODING VGATHERPF1QPD => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xc7 P66=1 PF2=0 PF3=0 REG=2 RM=4 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12NP',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512PF_512',
  tags       => 'page=XEON_VGATHERPF1QPD',
};


# VGATHERPF1QPS-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint.
ENCODING VGATHERPF1QPS => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xc7 P66=1 PF2=0 PF3=0 REG=2 RM=4 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12NP',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=AVX512PF_512',
  tags       => 'page=XEON_VGATHERPF1QPS',
};


# VGETEXPPD-Extract Float64 Vector of Exponents from Float64 Vector.
ENCODING VGETEXPPD_rzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x42 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VGETEXPPD',
};


# VGETEXPPS-Extract Float32 Vector of Exponents from Float32 Vector.
ENCODING VGETEXPPS_rzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x42 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VGETEXPPS',
};


# VGETMANTPD-Extract Float64 Vector of Normalized Mantissas from Float64 Vector.
ENCODING VGETMANTPD_rzmxub => {
  diagram    => 'ENC=MVEX MAP=0f3a MR=1 OP=0x26 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VGETMANTPD',
};


# VGETMANTPS-Extract Float32 Vector of Normalized Mantissas from Float32 Vector.
ENCODING VGETMANTPS_rzmxub => {
  diagram    => 'ENC=MVEX MAP=0f3a MR=1 OP=0x26 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VGETMANTPS',
};


# VGMAXABSPS-Absolute Maximum of Float32 Vectors.
ENCODING VGMAXABSPS => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x51 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MATH|MIN_MAX',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VGMAXABSPS',
};


# VGMAXPD-Maximum of Float64 Vectors.
ENCODING VGMAXPD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x53 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MATH|MIN_MAX',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VGMAXPD',
};


# VGMAXPS-Maximum of Float32 Vectors.
ENCODING VGMAXPS => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x53 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MATH|MIN_MAX',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VGMAXPS',
};


# VGMINPD-Minimum of Float64 Vectors.
ENCODING VGMINPD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x52 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MATH|MIN_MAX',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VGMINPD',
};


# VGMINPS-Minimum of Float32 Vectors.
ENCODING VGMINPS => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x52 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MATH|MIN_MAX',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VGMINPS',
};


# VLOADUNPACKHD-Load Unaligned High And Unpack To Doubleword Vector.
ENCODING VLOADUNPACKHD => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xd4 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VLOADUNPACKHD',
};


# VLOADUNPACKHPD-Load Unaligned High And Unpack To Float64 Vector.
ENCODING VLOADUNPACKHPD => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xd5 P66=0 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VLOADUNPACKHPD',
};


# VLOADUNPACKHPS-Load Unaligned High And Unpack To Float32 Vector.
ENCODING VLOADUNPACKHPS => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xd5 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VLOADUNPACKHPS',
};


# VLOADUNPACKHQ-Load Unaligned High And Unpack To Int64 Vector.
ENCODING VLOADUNPACKHQ => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xd4 P66=0 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VLOADUNPACKHQ',
};


# VLOADUNPACKLD-Load Unaligned Low And Unpack To Doubleword Vector.
ENCODING VLOADUNPACKLD => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xd0 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VLOADUNPACKLD',
};


# VLOADUNPACKLPD-Load Unaligned Low And Unpack To Float64 Vector.
ENCODING VLOADUNPACKLPD => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xd1 P66=0 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VLOADUNPACKLPD',
};


# VLOADUNPACKLPS-Load Unaligned Low And Unpack To Float32 Vector.
ENCODING VLOADUNPACKLPS => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xd1 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VLOADUNPACKLPS',
};


# VLOADUNPACKLQ-Load Unaligned Low And Unpack To Int64 Vector.
ENCODING VLOADUNPACKLQ => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xd0 P66=0 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VLOADUNPACKLQ',
};


# VLOG2PS-Vector Logarithm Base-2 of Float32 Vector.
ENCODING VLOG2PS => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xc9 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VLOG2PS',
};


# VMOVAPD-Move Aligned Float64 Vector.
ENCODING VMOVAPD_mzrz_3 => {
  diagram    => 'ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0x29 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VMOVAPD',
};

ENCODING VMOVAPD_rzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x28 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VMOVAPD',
};


# VMOVAPS-Move Aligned Float32 Vector.
ENCODING VMOVAPS_m0rz => {
  diagram    => 'ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0x29 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VMOVAPS',
};

ENCODING VMOVAPS_rzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x28 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VMOVAPS',
};


# VMOVDQA32-Move Aligned Int32 Vector.
ENCODING VMOVDQA32_m0rz => {
  diagram    => 'ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0x7f P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VMOVDQA32',
};

ENCODING VMOVDQA32_rzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x6f P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VMOVDQA32',
};


# VMOVDQA64-Move Aligned Int64 Vector.
ENCODING VMOVDQA64_mzrz => {
  diagram    => 'ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0x7f P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VMOVDQA64',
};

ENCODING VMOVDQA64_rzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x6f P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VMOVDQA64',
};


# VMOVNRAPD-Store Aligned Float64 Vector With No-Read Hint.
ENCODING VMOVNRAPD => {
  diagram    => 'E=0 ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0x29 P66=0 PF2=0 PF3=1 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCSTREAM requires_alignment=1',
  tags       => 'page=XEON_VMOVNRAPD',
};


# VMOVNRAPS-Store Aligned Float32 Vector With No-Read Hint.
ENCODING VMOVNRAPS => {
  diagram    => 'E=0 ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0x29 P66=0 PF2=1 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCSTREAM requires_alignment=1',
  tags       => 'page=XEON_VMOVNRAPS',
};


# VMOVNRNGOAPD-Non-globally Ordered Store Aligned Float64 Vector With No-Read Hint.
ENCODING VMOVNRNGOAPD => {
  diagram    => 'E=1 ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0x29 P66=0 PF2=0 PF3=1 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCSTREAM requires_alignment=1',
  tags       => 'page=XEON_VMOVNRNGOAPD',
};


# VMOVNRNGOAPS-Non-globally Ordered Store Aligned Float32 Vector With No-Read Hint.
ENCODING VMOVNRNGOAPS => {
  diagram    => 'E=1 ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0x29 P66=0 PF2=1 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCSTREAM requires_alignment=1',
  tags       => 'page=XEON_VMOVNRNGOAPS',
};


# VMULPD-Multiply Float64 Vectors.
ENCODING VMULPD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x59 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VMULPD',
};


# VMULPS-Multiply Float32 Vectors.
ENCODING VMULPS_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x59 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VMULPS',
};


# VP4DPWSSD-Dot Product of Signed Words with Dword Accumulation (4-iterations).
ENCODING VP4DPWSSD => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x52 P66=0 PF2=1 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_4VNNIW_512',
  tags       => 'page=XEON_VP4DPWSSD',
};


# VP4DPWSSDS-Dot Product of Signed Words with Dword Accumulation and Saturation (4-iterations).
ENCODING VP4DPWSSDS => {
  diagram    => 'BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0x53 P66=0 PF2=1 PF3=0 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E4',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=AVX512_4VNNIW_512',
  tags       => 'page=XEON_VP4DPWSSDS',
};


# VPACKSTOREHD-Pack And Store Unaligned High From Int32 Vector.
ENCODING VPACKSTOREHD => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xd4 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VPACKSTOREHD',
};


# VPACKSTOREHPD-Pack And Store Unaligned High From Float64 Vector.
ENCODING VPACKSTOREHPD => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xd5 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VPACKSTOREHPD',
};


# VPACKSTOREHPS-Pack And Store Unaligned High From Float32 Vector.
ENCODING VPACKSTOREHPS => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xd5 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VPACKSTOREHPS',
};


# VPACKSTOREHQ-Pack And Store Unaligned High From Int64 Vector.
ENCODING VPACKSTOREHQ => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xd4 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VPACKSTOREHQ',
};


# VPACKSTORELD-Pack and Store Unaligned Low From Int32 Vector.
ENCODING VPACKSTORELD => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xd0 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VPACKSTORELD',
};


# VPACKSTORELPD-Pack and Store Unaligned Low From Float64 Vector.
ENCODING VPACKSTORELPD => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xd1 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VPACKSTORELPD',
};


# VPACKSTORELPS-Pack and Store Unaligned Low From Float32 Vector.
ENCODING VPACKSTORELPS => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xd1 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VPACKSTORELPS',
};


# VPACKSTORELQ-Pack and Store Unaligned Low From Int64 Vector.
ENCODING VPACKSTORELQ => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xd0 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VPACKSTORELQ',
};


# VPADCD-Add Int32 Vectors with Carry.
ENCODING VPADCD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x5c P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPADCD',
};


# VPADDD-Add Int32 Vectors.
ENCODING VPADDD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0xfe P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPADDD',
};


# VPADDSETCD-Add Int32 Vectors and Set Mask to Carry.
ENCODING VPADDSETCD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x5d P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPADDSETCD',
};


# VPADDSETSD-Add Int32 Vectors and Set Mask to Sign.
ENCODING VPADDSETSD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xcd P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPADDSETSD',
};


# VPANDD-Bitwise AND Int32 Vectors.
ENCODING VPANDD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0xdb P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPANDD',
};


# VPANDND-Bitwise AND NOT Int32 Vectors.
ENCODING VPANDND_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0xdf P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPANDND',
};


# VPANDNQ-Bitwise AND NOT Int64 Vectors.
ENCODING VPANDNQ_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0xdf P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPANDNQ',
};


# VPANDQ-Bitwise AND Int64 Vectors.
ENCODING VPANDQ_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0xdb P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPANDQ',
};


# VPBLENDMD-Blend Int32 Vectors using the Instruction Mask.
ENCODING VPBLENDMD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x64 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPBLENDMD',
};


# VPBLENDMQ-Blend Int64 Vectors using the Instruction Mask.
ENCODING VPBLENDMQ => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x64 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPBLENDMQ',
};


# VPBROADCASTD-Broadcast Int32 Vector.
ENCODING VPBROADCASTD_rzm0 => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0x58 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPBROADCASTD',
};


# VPBROADCASTQ-Broadcast Int64 Vector.
ENCODING VPBROADCASTQ_rzm0 => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0x59 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|BROADCASTING|SET',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPBROADCASTQ',
};


# VPCMPD-Compare Int32 Vectors and Set Vector Mask.
ENCODING VPCMPD_nzmxub => {
  diagram    => 'ENC=MVEX MAP=0f3a MR=1 OP=0x1f P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPCMPD',
};


# VPCMPEQD-Compare Equal Int32 Vectors and Set Vector Mask.
ENCODING VPCMPEQD_nzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x76 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPCMPEQD',
};


# VPCMPGTD-Compare Greater Than Int32 Vectors and Set Vector Mask.
ENCODING VPCMPGTD_nzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x66 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPCMPGTD',
};


# VPCMPLTD-Compare Less Than Int32 Vectors and Set Vector Mask.
ENCODING VPCMPLTD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x74 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPCMPLTD',
};


# VPCMPUD-Compare Uint32 Vectors and Set Vector Mask.
ENCODING VPCMPUD_nzmxub => {
  diagram    => 'ENC=MVEX MAP=0f3a MR=1 OP=0x1e P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPCMPUD',
};


# VPERMD-Permutes Int32 Vectors.
ENCODING VPERMD_rznzmz_2 => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x36 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPERMD',
};


# VPERMF32X4-Shuffle Vector Dqwords.
ENCODING VPERMF32X4 => {
  diagram    => 'ENC=MVEX MAP=0f3a MR=1 OP=0x07 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|SWIZZLE',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPERMF32X4',
};


# VPGATHERDD-Gather Int32 Vector With Signed Dword Indices.
ENCODING VPGATHERDD => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0x90 P66=1 PF2=0 PF3=0 RM=4 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPGATHERDD',
};


# VPGATHERDQ-Gather Int64 Vector With Signed Dword Indices.
ENCODING VPGATHERDQ => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0x90 P66=1 PF2=0 PF3=0 RM=4 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPGATHERDQ',
};


# VPMADD231D-Multiply First Source By Second Source and Add To Destination Int32 Vectors.
ENCODING VPMADD231D => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xb5 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPMADD231D',
};


# VPMADD233D-Multiply First Source By Specially Swizzled Second Source and Add To Second Source Int32 Vectors.
ENCODING VPMADD233D => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xb4 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|CARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPMADD233D',
};


# VPMAXSD-Maximum of Int32 Vectors.
ENCODING VPMAXSD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x3d P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPMAXSD',
};


# VPMAXUD-Maximum of Uint32 Vectors.
ENCODING VPMAXUD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x3f P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPMAXUD',
};


# VPMINSD-Minimum of Int32 Vectors.
ENCODING VPMINSD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x39 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPMINSD',
};


# VPMINUD-Minimum of Uint32 Vectors.
ENCODING VPMINUD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x3b P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC|MIN_MAX',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPMINUD',
};


# VPMULHD-Multiply Int32 Vectors And Store High Result.
ENCODING VPMULHD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x87 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPMULHD',
};


# VPMULHUD-Multiply Uint32 Vectors And Store High Result.
ENCODING VPMULHUD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x86 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPMULHUD',
};


# VPMULLD-Multiply Int32 Vectors And Store Low Result.
ENCODING VPMULLD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x40 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPMULLD',
};


# VPORD-Bitwise OR Int32 Vectors.
ENCODING VPORD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0xeb P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPORD',
};


# VPORQ-Bitwise OR Int64 Vectors.
ENCODING VPORQ_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0xeb P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPORQ',
};


# VPREFETCH0-Prefetch memory line using T0 hint.
ENCODING VPREFETCH0_mb_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MODE=64 MR=1 OP=0x18 P66=0 PF2=0 PF3=0 REG=1 VL=128',
  extensions => 'KNC',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_VPREFETCH0',
};

ENCODING VPREFETCH0_mb_2 => {
  diagram    => 'ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0x18 P66=0 PF2=0 PF3=0 REG=1 VL=512',
  extensions => 'KNCE',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VPREFETCH0',
};


# VPREFETCH1-Prefetch memory line using T1 hint.
ENCODING VPREFETCH1_mb_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MODE=64 MR=1 OP=0x18 P66=0 PF2=0 PF3=0 REG=2 VL=128',
  extensions => 'KNC',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_VPREFETCH1',
};

ENCODING VPREFETCH1_mb_2 => {
  diagram    => 'ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0x18 P66=0 PF2=0 PF3=0 REG=2 VL=512',
  extensions => 'KNCE',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VPREFETCH1',
};


# VPREFETCH2-Prefetch memory line using T2 hint.
ENCODING VPREFETCH2_mb_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MODE=64 MR=1 OP=0x18 P66=0 PF2=0 PF3=0 REG=3 VL=128',
  extensions => 'KNC',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_VPREFETCH2',
};

ENCODING VPREFETCH2_mb_2 => {
  diagram    => 'ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0x18 P66=0 PF2=0 PF3=0 REG=3 VL=512',
  extensions => 'KNCE',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VPREFETCH2',
};


# VPREFETCHE0-Prefetch memory line using T0 hint, with intent to write.
ENCODING VPREFETCHE0_mb_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MODE=64 MR=1 OP=0x18 P66=0 PF2=0 PF3=0 REG=5 VL=128',
  extensions => 'KNC',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_VPREFETCHE0',
};

ENCODING VPREFETCHE0_mb_2 => {
  diagram    => 'ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0x18 P66=0 PF2=0 PF3=0 REG=5 VL=512',
  extensions => 'KNCE',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VPREFETCHE0',
};


# VPREFETCHE1-Prefetch memory line using T1 hint, with intent to write.
ENCODING VPREFETCHE1_mb_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MODE=64 MR=1 OP=0x18 P66=0 PF2=0 PF3=0 REG=6 VL=128',
  extensions => 'KNC',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_VPREFETCHE1',
};

ENCODING VPREFETCHE1_mb_2 => {
  diagram    => 'ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0x18 P66=0 PF2=0 PF3=0 REG=6 VL=512',
  extensions => 'KNCE',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VPREFETCHE1',
};


# VPREFETCHE2-Prefetch memory line using T2 hint, with intent to write.
ENCODING VPREFETCHE2_mb_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MODE=64 MR=1 OP=0x18 P66=0 PF2=0 PF3=0 REG=7 VL=128',
  extensions => 'KNC',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_VPREFETCHE2',
};

ENCODING VPREFETCHE2_mb_2 => {
  diagram    => 'ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0x18 P66=0 PF2=0 PF3=0 REG=7 VL=512',
  extensions => 'KNCE',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VPREFETCHE2',
};


# VPREFETCHENTA-Prefetch memory line using NTA hint, with intent to write.
ENCODING VPREFETCHENTA_mb_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MODE=64 MR=1 OP=0x18 P66=0 PF2=0 PF3=0 REG=4 VL=128',
  extensions => 'KNC',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_VPREFETCHENTA',
};

ENCODING VPREFETCHENTA_mb_2 => {
  diagram    => 'ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0x18 P66=0 PF2=0 PF3=0 REG=4 VL=512',
  extensions => 'KNCE',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VPREFETCHENTA',
};


# VPREFETCHNTA-Prefetch memory line using NTA hint.
ENCODING VPREFETCHNTA_mb_1 => {
  diagram    => 'ENC=VEX MAP=0f MOD=MEM MODE=64 MR=1 OP=0x18 P66=0 PF2=0 PF3=0 REG=0 VL=128',
  extensions => 'KNC',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCV',
  tags       => 'page=XEON_VPREFETCHNTA',
};

ENCODING VPREFETCHNTA_mb_2 => {
  diagram    => 'ENC=MVEX MAP=0f MOD=MEM MR=1 OP=0x18 P66=0 PF2=0 PF3=0 REG=0 VL=512',
  extensions => 'KNCE',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VPREFETCHNTA',
};


# VPSBBD-Subtract Int32 Vectors with Borrow.
ENCODING VPSBBD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x5e P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPSBBD',
};


# VPSBBRD-Reverse Subtract Int32 Vectors with Borrow.
ENCODING VPSBBRD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x6e P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPSBBRD',
};


# VPSCATTERDD-Scatter Int32 Vector With Signed Dword Indices.
ENCODING VPSCATTERDD => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xa0 P66=1 PF2=0 PF3=0 RM=4 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPSCATTERDD',
};


# VPSCATTERDQ-Scatter Int64 Vector With Signed Dword Indices.
ENCODING VPSCATTERDQ => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xa0 P66=1 PF2=0 PF3=0 RM=4 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPSCATTERDQ',
};


# VPSHUFD-Shuffle Vector Doublewords.
ENCODING VPSHUFD_rzmzub_2 => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x70 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPSHUFD',
};


# VPSLLD-Shift Int32 Vector Immediate Left Logical.
ENCODING VPSLLD_nzmxub => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=6 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPSLLD',
};


# VPSLLVD-Shift Int32 Vector Left Logical.
ENCODING VPSLLVD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x47 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|SHIFT_LEFT|LOGICAL',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPSLLVD',
};


# VPSRAD-Shift Int32 Vector Immediate Right Arithmetic.
ENCODING VPSRAD_nzmxub => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=4 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPSRAD',
};


# VPSRAVD-Shift Int32 Vector Right Arithmetic.
ENCODING VPSRAVD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x46 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|SHIFT_RIGHT|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPSRAVD',
};


# VPSRLD-Shift Int32 Vector Immediate Right Logical.
ENCODING VPSRLD_nzmxub => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x72 P66=1 PF2=0 PF3=0 REG=2 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPSRLD',
};


# VPSRLVD-Shift Int32 Vector Right Logical.
ENCODING VPSRLVD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x45 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|SHIFT_RIGHT|LOGICAL',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPSRLVD',
};


# VPSUBD-Subtract Int32 Vectors.
ENCODING VPSUBD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0xfa P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPSUBD',
};


# VPSUBRD-Reverse Subtract Int32 Vectors.
ENCODING VPSUBRD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x6c P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPSUBRD',
};


# VPSUBRSETBD-Reverse Subtract Int32 Vectors and Set Borrow.
ENCODING VPSUBRSETBD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x6f P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPSUBRSETBD',
};


# VPSUBSETBD-Subtract Int32 Vectors and Set Borrow.
ENCODING VPSUBSETBD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x5f P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPSUBSETBD',
};


# VPTESTMD-Logical AND Int32 Vectors and Set Vector Mask.
ENCODING VPTESTMD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x27 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|COMPARISON',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPTESTMD',
};


# VPXORD-Bitwise XOR Int32 Vectors.
ENCODING VPXORD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0xef P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPXORD',
};


# VPXORQ-Bitwise XOR Int64 Vectors.
ENCODING VPXORQ_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0xef P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|BITWISE|LOGICAL',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VPXORQ',
};


# VRCP23PS-Reciprocal of Float32 Vector.
ENCODING VRCP23PS => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xca P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VRCP23PS',
};


# VRCP28PD-Approximation to the Reciprocal of Packed Double-Precision Floating-Point Values with Less Than 2^-28 Relative Error.
ENCODING VRCP28PD => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xca P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX512ER_512',
  tags       => 'page=XEON_VRCP28PD',
};


# VRCP28PS-Approximation to the Reciprocal of Packed Single-Precision Floating-Point Values with Less Than 2^-28 Relative Error.
ENCODING VRCP28PS => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xca P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX512ER_512',
  tags       => 'page=XEON_VRCP28PS',
};


# VRCP28SD-Approximation to the Reciprocal of Scalar Double-Precision Floating-Point Value with Less Than 2^-28 Relative Error.
ENCODING VRCP28SD => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xcb P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX512ER_SCALAR',
  tags       => 'page=XEON_VRCP28SD',
};


# VRCP28SS-Approximation to the Reciprocal of Scalar Single-Precision Floating-Point Value with Less Than 2^-28 Relative Error.
ENCODING VRCP28SS => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xcb P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX512ER_SCALAR',
  tags       => 'page=XEON_VRCP28SS',
};


# VRNDFXPNTPD-Round Float64 Vector.
ENCODING VRNDFXPNTPD => {
  diagram    => 'ENC=MVEX MAP=0f3a MR=1 OP=0x52 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|CONVERSION',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VRNDFXPNTPD',
};


# VRNDFXPNTPS-Round Float32 Vector.
ENCODING VRNDFXPNTPS => {
  diagram    => 'ENC=MVEX MAP=0f3a MR=1 OP=0x52 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|MISC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VRNDFXPNTPS',
};


# VRSQRT23PS-Vector Reciprocal Square Root of Float32 Vector.
ENCODING VRSQRT23PS => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0xcb P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VRSQRT23PS',
};


# VRSQRT28PD-Approximation to the Reciprocal Square Root of Packed Double-Precision Floating-Point Values with Less Than 2^-28 Relative Error.
ENCODING VRSQRT28PD => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xcc P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX512ER_512',
  tags       => 'page=XEON_VRSQRT28PD',
};


# VRSQRT28PS-Approximation to the Reciprocal Square Root of Packed Single-Precision Floating-Point Values with Less Than 2^-28 Relative Error.
ENCODING VRSQRT28PS => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xcc P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E2',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX512ER_512',
  tags       => 'page=XEON_VRSQRT28PS',
};


# VRSQRT28SD-Approximation to the Reciprocal Square Root of Scalar Double-Precision Floating-Point Value with Less Than 2^-28 Relative Error.
ENCODING VRSQRT28SD => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xcd P66=1 PF2=0 PF3=0 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX512ER_SCALAR',
  tags       => 'page=XEON_VRSQRT28SD',
};


# VRSQRT28SS-Approximation to the Reciprocal Square Root of Scalar Single-Precision Floating- Point Value with Less Than 2^-28 Relative Error.
ENCODING VRSQRT28SS => {
  diagram    => 'ENC=EVEX MAP=0f38 MR=1 OP=0xcd P66=1 PF2=0 PF3=0 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E3',
  categories => 'ADVSIMD|EMATH',
  metadata   => 'isa=AVX512ER_SCALAR',
  tags       => 'page=XEON_VRSQRT28SS',
};


# VSCALEPS-Scale Float32 Vectors.
ENCODING VSCALEPS => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x84 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VSCALEPS',
};


# VSCATTERDPD-Scatter Float64 Vector With Signed Dword Indices.
ENCODING VSCATTERDPD => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xa2 P66=1 PF2=0 PF3=0 RM=4 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VSCATTERDPD',
};


# VSCATTERDPS-Scatter Float32 Vector With Signed Dword Indices.
ENCODING VSCATTERDPS => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xa2 P66=1 PF2=0 PF3=0 RM=4 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VSCATTERDPS',
};


# VSCATTERPF0DPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint with Intent to Write.
ENCODING VSCATTERPF0DPD => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 REG=5 RM=4 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12NP',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512PF_512',
  tags       => 'page=XEON_VSCATTERPF0DPD',
};


# VSCATTERPF0DPS-Scatter Prefetch Float32 Vector With Signed Dword Indices Into L1.
ENCODING VSCATTERPF0DPS_m0 => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 REG=5 RM=4 W=0',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VSCATTERPF0DPS',
};

ENCODING VSCATTERPF0DPS_md => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 REG=5 RM=4 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12NP',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512PF_512',
  tags       => 'page=XEON_VSCATTERPF0DPS',
};


# VSCATTERPF0HINTDPD-Scatter Prefetch Float64 Vector Hint With Signed Dword Indices.
ENCODING VSCATTERPF0HINTDPD => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 REG=4 RM=4 W=1',
  extensions => 'KNCE',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNC_PF_HINT',
  tags       => 'page=XEON_VSCATTERPF0HINTDPD',
};


# VSCATTERPF0HINTDPS-Scatter Prefetch Float32 Vector Hint With Signed Dword Indices.
ENCODING VSCATTERPF0HINTDPS => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 REG=4 RM=4 W=0',
  extensions => 'KNCE',
  categories => 'ADVSIMD',
  metadata   => 'isa=KNC_PF_HINT',
  tags       => 'page=XEON_VSCATTERPF0HINTDPS',
};


# VSCATTERPF0QPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint with Intent to Write.
ENCODING VSCATTERPF0QPD => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xc7 P66=1 PF2=0 PF3=0 REG=5 RM=4 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12NP',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512PF_512',
  tags       => 'page=XEON_VSCATTERPF0QPD',
};


# VSCATTERPF0QPS-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint with Intent to Write.
ENCODING VSCATTERPF0QPS => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xc7 P66=1 PF2=0 PF3=0 REG=5 RM=4 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12NP',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512PF_512',
  tags       => 'page=XEON_VSCATTERPF0QPS',
};


# VSCATTERPF1DPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint with Intent to Write.
ENCODING VSCATTERPF1DPD => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 REG=6 RM=4 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12NP',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512PF_512',
  tags       => 'page=XEON_VSCATTERPF1DPD',
};


# VSCATTERPF1DPS-Scatter Prefetch Float32 Vector With Signed Dword Indices Into L2.
ENCODING VSCATTERPF1DPS_m0 => {
  diagram    => 'ENC=MVEX MAP=0f38 MOD=MEM MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 REG=6 RM=4 W=0',
  extensions => 'KNCE',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=KNCE',
  tags       => 'page=XEON_VSCATTERPF1DPS',
};

ENCODING VSCATTERPF1DPS_md => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xc6 P66=1 PF2=0 PF3=0 REG=6 RM=4 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12NP',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512PF_512',
  tags       => 'page=XEON_VSCATTERPF1DPS',
};


# VSCATTERPF1QPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint with Intent to Write.
ENCODING VSCATTERPF1QPD => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xc7 P66=1 PF2=0 PF3=0 REG=6 RM=4 VL=512 W=1',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12NP',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512PF_512',
  tags       => 'page=XEON_VSCATTERPF1QPD',
};


# VSCATTERPF1QPS-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint with Intent to Write.
ENCODING VSCATTERPF1QPS => {
  diagram    => 'ASZ=Y BP=0 ENC=EVEX MAP=0f38 MOD=MEM MR=1 OP=0xc7 P66=1 PF2=0 PF3=0 REG=6 RM=4 VL=512 W=0',
  extensions => 'AVX512EVEX',
  exceptions => 'AVX512_E12NP',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
  metadata   => 'isa=AVX512PF_512',
  tags       => 'page=XEON_VSCATTERPF1QPS',
};


# VSUBPD-Subtract Float64 Vectors.
ENCODING VSUBPD_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x5c P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VSUBPD',
};


# VSUBPS-Subtract Float32 Vectors.
ENCODING VSUBPS_rznzmx => {
  diagram    => 'ENC=MVEX MAP=0f MR=1 OP=0x5c P66=0 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VSUBPS',
};


# VSUBRPD-Reverse Subtract Float64 Vectors.
ENCODING VSUBRPD => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x6d P66=1 PF2=0 PF3=0 W=1',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VSUBRPD',
};


# VSUBRPS-Reverse Subtract Float32 Vectors.
ENCODING VSUBRPS => {
  diagram    => 'ENC=MVEX MAP=0f38 MR=1 OP=0x6d P66=1 PF2=0 PF3=0 W=0',
  extensions => 'KNCE',
  exceptions => 'ALIGNMENT',
  categories => 'ADVSIMD|ARITHMETIC',
  metadata   => 'isa=KNCE requires_alignment=1',
  tags       => 'page=XEON_VSUBRPS',
};

