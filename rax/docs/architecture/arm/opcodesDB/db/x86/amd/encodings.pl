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


# BEXTR-Bit Field Extract.
ENCODING BEXTR_rxmxud => {
  diagram    => 'ENC=XOP MAP=xopa MR=1 OP=0x10 P66=0 PF2=0 PF3=0 VL=128',
  iflags     => 'af=U cf=Z of=Z pf=U sf=U zf=W',
  extensions => 'TBM',
  categories => 'GENERAL|BITFIELD',
  metadata   => 'isa=TBM',
  tags       => 'page=BEXTR',
};


# BLCFILL.
ENCODING BLCFILL => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=1 VL=128',
  iflags     => 'af=U cf=W of=Z pf=U sf=W zf=W',
  extensions => 'TBM',
  categories => 'GENERAL',
  metadata   => 'isa=TBM',
  tags       => 'page=BLCFILL',
};


# BLCI.
ENCODING BLCI => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x02 P66=0 PF2=0 PF3=0 REG=6 VL=128',
  iflags     => 'af=U cf=W of=Z pf=U sf=W zf=W',
  extensions => 'TBM',
  categories => 'GENERAL',
  metadata   => 'isa=TBM',
  tags       => 'page=BLCI',
};


# BLCIC.
ENCODING BLCIC => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=5 VL=128',
  iflags     => 'af=U cf=W of=Z pf=U sf=W zf=W',
  extensions => 'TBM',
  categories => 'GENERAL',
  metadata   => 'isa=TBM',
  tags       => 'page=BLCIC',
};


# BLCMSK.
ENCODING BLCMSK => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x02 P66=0 PF2=0 PF3=0 REG=1 VL=128',
  iflags     => 'af=U cf=W of=Z pf=U sf=W zf=W',
  extensions => 'TBM',
  categories => 'GENERAL',
  metadata   => 'isa=TBM',
  tags       => 'page=BLCMSK',
};


# BLCS.
ENCODING BLCS => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=3 VL=128',
  iflags     => 'af=U cf=W of=Z pf=U sf=W zf=W',
  extensions => 'TBM',
  categories => 'GENERAL',
  metadata   => 'isa=TBM',
  tags       => 'page=BLCS',
};


# BLSFILL.
ENCODING BLSFILL => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=2 VL=128',
  iflags     => 'af=U cf=W of=Z pf=U sf=W zf=W',
  extensions => 'TBM',
  categories => 'GENERAL',
  metadata   => 'isa=TBM',
  tags       => 'page=BLSFILL',
};


# BLSIC.
ENCODING BLSIC => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=6 VL=128',
  iflags     => 'af=U cf=W of=Z pf=U sf=W zf=W',
  extensions => 'TBM',
  categories => 'GENERAL',
  metadata   => 'isa=TBM',
  tags       => 'page=BLSIC',
};


# CLZERO.
ENCODING CLZERO => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 REG=7 RM=4',
  extensions => 'CLZERO',
  categories => 'GENERAL',
  metadata   => 'isa=CLZERO',
  tags       => 'page=CLZERO',
};


# EXTRQ.
ENCODING EXTRQ_mqub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x78 P66=1 PF2=0 PF3=0 REG=0',
  extensions => 'SSE4A',
  categories => 'SIMD|BITFIELD',
  metadata   => 'isa=SSE4A',
  tags       => 'page=EXTRQ',
};

ENCODING EXTRQ_rqmo => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x79 P66=1 PF2=0 PF3=0',
  extensions => 'SSE4A',
  categories => 'SIMD|BITFIELD',
  metadata   => 'isa=SSE4A',
  tags       => 'page=EXTRQ',
};


# INSERTQ.
ENCODING INSERTQ_rqmo => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x79 P66=0 PF2=1 PF3=0',
  extensions => 'SSE4A',
  categories => 'SIMD|BITFIELD',
  metadata   => 'isa=SSE4A',
  tags       => 'page=INSERTQ',
};

ENCODING INSERTQ_rqmqub => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x78 P66=0 PF2=1 PF3=0',
  extensions => 'SSE4A',
  categories => 'SIMD|BITFIELD',
  metadata   => 'isa=SSE4A',
  tags       => 'page=INSERTQ',
};


# LLWPCB-Load LWPCB Address.
ENCODING LLWPCB => {
  diagram    => 'ENC=XOP MAP=xop9 MOD=REG MR=1 OP=0x12 P66=0 PF2=0 PF3=0 REG=0 VL=128',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=LPS_LLWPCB',
};


# LWPINS-Insert User Event Record in LWP Ring Buffer.
ENCODING LWPINS => {
  diagram    => 'ENC=XOP MAP=xopa MR=1 OP=0x12 P66=0 PF2=0 PF3=0 REG=0 VL=128',
  iflags     => 'cf=W',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=LPS_LWPINS',
};


# LWPVAL-Insert Value Sample in LWP Ring Buffer.
ENCODING LWPVAL => {
  diagram    => 'ENC=XOP MAP=xopa MR=1 OP=0x12 P66=0 PF2=0 PF3=0 REG=1 VL=128',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=LPS_LWPVAL',
};


# SLWPCB-Store LWPCB Address.
ENCODING SLWPCB => {
  diagram    => 'ENC=XOP MAP=xop9 MOD=REG MR=1 OP=0x12 P66=0 PF2=0 PF3=0 REG=1 VL=128',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=LPS_SLWPCB',
};


# MONITORX.
ENCODING MONITORX => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=7 RM=2',
  extensions => 'MONITORX',
  categories => 'SYSTEM',
  metadata   => 'isa=MONITORX',
  tags       => 'page=MONITORX',
};


# MOVNTSD.
ENCODING MOVNTSD => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x2b P66=0 PF2=1 PF3=0',
  extensions => 'SSE4A',
  categories => 'SIMD',
  metadata   => 'isa=SSE4A',
  tags       => 'page=MOVNTSD',
};


# MOVNTSS.
ENCODING MOVNTSS => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x2b P66=0 PF2=0 PF3=1',
  extensions => 'SSE4A',
  categories => 'SIMD',
  metadata   => 'isa=SSE4A',
  tags       => 'page=MOVNTSS',
};


# MWAITX.
ENCODING MWAITX => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=7 RM=3',
  extensions => 'MONITORX',
  categories => 'SYSTEM',
  metadata   => 'isa=MONITORX',
  tags       => 'page=MWAITX',
};


# RDPRU.
ENCODING RDPRU => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 REG=7 RM=5',
  extensions => 'RDPRU',
  categories => 'SYSTEM',
  metadata   => 'isa=RDPRU',
  tags       => 'page=RDPRU',
};


# CLGI-Clear Global Interrupt Flag.
ENCODING CLGI => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 REG=3 RM=5',
  extensions => 'SVM',
  categories => 'SYSTEM',
  metadata   => 'isa=SVM',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=SVM_CLGI',
};


# INVLPGA-Invalidate TLB Entry in a Specified ASID.
ENCODING INVLPGA => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 REG=3 RM=7',
  extensions => 'SVM',
  categories => 'SYSTEM',
  metadata   => 'isa=SVM cpl=RING0',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=SVM_INVLPGA',
};


# SKINIT-Secure Init and Jump with Attestation.
ENCODING SKINIT => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 REG=3 RM=6',
  extensions => 'SVM',
  categories => 'SYSTEM',
  metadata   => 'isa=SVM',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=SVM_SKINIT',
};


# STGI-Set Global Interrupt Flag.
ENCODING STGI => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 REG=3 RM=4',
  extensions => 'SVM',
  categories => 'SYSTEM',
  metadata   => 'isa=SVM',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=SVM_STGI',
};


# VMLOAD-Load State from VMCB.
ENCODING VMLOAD => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 REG=3 RM=2',
  extensions => 'SVM',
  categories => 'SYSTEM',
  metadata   => 'isa=SVM',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=SVM_VMLOAD',
};


# VMMCALL-Call VMM.
ENCODING VMMCALL => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 REG=3 RM=1',
  extensions => 'SVM',
  categories => 'SYSTEM',
  metadata   => 'isa=SVM',
  tags       => 'page=SVM_VMMCALL',
};


# VMRUN-Run Virtual Machine.
ENCODING VMRUN => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 REG=3 RM=0',
  extensions => 'SVM',
  categories => 'SYSTEM',
  metadata   => 'isa=SVM',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=SVM_VMRUN',
};


# VMSAVE-Save State to VMCB.
ENCODING VMSAVE => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 REG=3 RM=3',
  extensions => 'SVM',
  categories => 'SYSTEM',
  metadata   => 'isa=SVM',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=SVM_VMSAVE',
};


# SYSCALL-Fast System Call.
ENCODING SYSCALL_2 => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x05',
  iflags     => 'ac=W af=W cf=W df=W id=W _if=W iopl=W nt=W of=W pf=W rf=Z sf=W tf=W vif=W vip=W vm=Z zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|SYSCALL',
  metadata   => 'isa=AMD',
  tags       => 'page=SYSCALL',
};


# SYSRET-Return From Fast System Call.
ENCODING SYSRET_2 => {
  diagram    => 'MAP=0f MODE=NO64 OP=0x07',
  iflags     => 'ac=W af=W cf=W df=W id=W _if=W iopl=W nt=W of=W pf=W rf=Z sf=W tf=W vif=W vip=W zf=W',
  extensions => 'BASE',
  categories => 'GENERAL|SYSRET',
  metadata   => 'isa=AMD cpl=RING0',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=SYSRET',
};


# T1MSKC.
ENCODING T1MSKC => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=7 VL=128',
  iflags     => 'af=U cf=W of=Z pf=U sf=W zf=W',
  extensions => 'TBM',
  categories => 'GENERAL',
  metadata   => 'isa=TBM',
  tags       => 'page=T1MSKC',
};


# TZMSK.
ENCODING TZMSK => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=4 VL=128',
  iflags     => 'af=U cf=W of=Z pf=U sf=W zf=W',
  extensions => 'TBM',
  categories => 'GENERAL',
  metadata   => 'isa=TBM',
  tags       => 'page=TZMSK',
};


# VFMADDPD-Multiply and Add Packed Double-Precision Floating-Point.
ENCODING VFMADDPD_rxnxis4xmx => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x69 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMADDPD',
};

ENCODING VFMADDPD_rxnxmxis4x => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x69 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMADDPD',
};


# VFMADDPS-Multiply and Add Packed Single-Precision Floating-Point.
ENCODING VFMADDPS_rxnxis4xmx => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x68 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMADDPS',
};

ENCODING VFMADDPS_rxnxmxis4x => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x68 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMADDPS',
};


# VFMADDSD-Multiply and Add Scalar Double-Precision Floating-Point.
ENCODING VFMADDSD_ronois4omq => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x6b P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMADDSD',
};

ENCODING VFMADDSD_ronomqis4o => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x6b P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMADDSD',
};


# VFMADDSS-Multiply and Add Scalar Single-Precision Floating-Point.
ENCODING VFMADDSS_ronois4omd => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x6a P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMADDSS',
};

ENCODING VFMADDSS_ronomdis4o => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x6a P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMADDSS',
};


# VFMADDSUBPD-Multiply with Alternating Add/Subtract of Packed Double-Precision Floating-Point.
ENCODING VFMADDSUBPD_rxnxis4xmx => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x5d P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMADDSUBPD',
};

ENCODING VFMADDSUBPD_rxnxmxis4x => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x5d P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMADDSUBPD',
};


# VFMADDSUBPS-Multiply with Alternating Add/Subtract of Packed Single-Precision Floating-Point.
ENCODING VFMADDSUBPS_rxnxis4xmx => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x5c P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMADDSUBPS',
};

ENCODING VFMADDSUBPS_rxnxmxis4x => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x5c P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMADDSUBPS',
};


# VFMSUBADDPD-Multiply with Alternating Subtract/Add of Packed Double-Precision Floating-Point.
ENCODING VFMSUBADDPD_rxnxis4xmx => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x5f P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMSUBADDPD',
};

ENCODING VFMSUBADDPD_rxnxmxis4x => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x5f P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMSUBADDPD',
};


# VFMSUBADDPS-Multiply with Alternating Subtract/Add of Packed Single-Precision Floating-Point.
ENCODING VFMSUBADDPS_rxnxis4xmx => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x5e P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMSUBADDPS',
};

ENCODING VFMSUBADDPS_rxnxmxis4x => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x5e P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMSUBADDPS',
};


# VFMSUBPD-Multiply and Subtract Packed Double-Precision Floating-Point.
ENCODING VFMSUBPD_rxnxis4xmx => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x6d P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMSUBPD',
};

ENCODING VFMSUBPD_rxnxmxis4x => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x6d P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMSUBPD',
};


# VFMSUBPS-Multiply and Subtract Packed Single-Precision Floating-Point.
ENCODING VFMSUBPS_rxnxis4xmx => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x6c P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMSUBPS',
};

ENCODING VFMSUBPS_rxnxmxis4x => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x6c P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMSUBPS',
};


# VFMSUBSD-Multiply and Subtract Scalar Double-Precision Floating-Point.
ENCODING VFMSUBSD_ronois4omq => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x6f P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMSUBSD',
};

ENCODING VFMSUBSD_ronomqis4o => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x6f P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMSUBSD',
};


# VFMSUBSS-Multiply and Subtract Scalar Single-Precision Floating-Point.
ENCODING VFMSUBSS_ronois4omd => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x6e P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMSUBSS',
};

ENCODING VFMSUBSS_ronomdis4o => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x6e P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFMSUBSS',
};


# VFNMADDPD-Negative Multiply and Add Packed Double-Precision Floating-Point.
ENCODING VFNMADDPD_rxnxis4xmx => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x79 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFNMADDPD',
};

ENCODING VFNMADDPD_rxnxmxis4x => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x79 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFNMADDPD',
};


# VFNMADDPS-Negative Multiply and Add Packed Single-Precision Floating-Point.
ENCODING VFNMADDPS_rxnxis4xmx => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x78 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFNMADDPS',
};

ENCODING VFNMADDPS_rxnxmxis4x => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x78 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFNMADDPS',
};


# VFNMADDSD-Negative Multiply and Add Scalar Double-Precision Floating-Point.
ENCODING VFNMADDSD_ronois4omq => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x7b P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFNMADDSD',
};

ENCODING VFNMADDSD_ronomqis4o => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x7b P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFNMADDSD',
};


# VFNMADDSS-Negative Multiply and Add Scalar Single-Precision Floating-Point.
ENCODING VFNMADDSS_ronois4omd => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x7a P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFNMADDSS',
};

ENCODING VFNMADDSS_ronomdis4o => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x7a P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFNMADDSS',
};


# VFNMSUBPD-Negative Multiply and Subtract Packed Double-Precision Floating-Point.
ENCODING VFNMSUBPD_rxnxis4xmx => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x7d P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFNMSUBPD',
};

ENCODING VFNMSUBPD_rxnxmxis4x => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x7d P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFNMSUBPD',
};


# VFNMSUBPS-Negative Multiply and Subtract Packed Single-Precision Floating-Point.
ENCODING VFNMSUBPS_rxnxis4xmx => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x7c P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFNMSUBPS',
};

ENCODING VFNMSUBPS_rxnxmxis4x => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x7c P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFNMSUBPS',
};


# VFNMSUBSD-Negative Multiply and Subtract Scalar Double-Precision Floating-Point.
ENCODING VFNMSUBSD_ronois4omq => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x7f P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFNMSUBSD',
};

ENCODING VFNMSUBSD_ronomqis4o => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x7f P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFNMSUBSD',
};


# VFNMSUBSS-Negative Multiply and Subtract Scalar Single-Precision Floating-Point.
ENCODING VFNMSUBSS_ronois4omd => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x7e P66=1 PF2=0 PF3=0 W=1',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFNMSUBSS',
};

ENCODING VFNMSUBSS_ronomdis4o => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x7e P66=1 PF2=0 PF3=0 W=0',
  extensions => 'FMA4',
  categories => 'ADVSIMD',
  metadata   => 'isa=FMA4',
  tags       => 'page=VEX_VFNMSUBSS',
};


# VPERMIL2PD-Permute Two-Source Double-Precision Floating-Point Values.
ENCODING VPERMIL2PD_rxnxis4xmxis4 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x49 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=VEX_VPERMIL2PD',
};

ENCODING VPERMIL2PD_rxnxmxis4xis4 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x49 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=VEX_VPERMIL2PD',
};


# VPHADDUWD.
ENCODING VPHADDUWD => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0xd6 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=VPHADDUWD',
};


# VFRCZPD-Extract Fraction Packed Double-Precision Floating-Point.
ENCODING VFRCZPD => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x81 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VFRCZPD',
};


# VFRCZPS-Extract Fraction Packed Single-Precision Floating-Point.
ENCODING VFRCZPS => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x80 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VFRCZPS',
};


# VFRCZSD-Extract Fraction Scalar Double-Precision Floating-Point.
ENCODING VFRCZSD => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x83 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VFRCZSD',
};


# VFRCZSS-Extract Fraction Scalar Single-Precision Floating Point.
ENCODING VFRCZSS => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x82 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VFRCZSS',
};


# VPCMOV-Vector Conditional Moves.
ENCODING VPCMOV_rxnxis4xmx => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xa2 P66=0 PF2=0 PF3=0 W=1',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPCMOV',
};

ENCODING VPCMOV_rxnxmxis4x => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xa2 P66=0 PF2=0 PF3=0 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPCMOV',
};


# VPCOMB-Compare Vector Signed Bytes.
ENCODING VPCOMB => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xcc P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPCOMB',
};


# VPCOMD-Compare Vector Signed Doublewords.
ENCODING VPCOMD => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xce P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPCOMD',
};


# VPCOMQ-Compare Vector Signed Quadwords.
ENCODING VPCOMQ => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xcf P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPCOMQ',
};


# VPCOMUB-Compare Vector Unsigned Bytes.
ENCODING VPCOMUB => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xec P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPCOMUB',
};


# VPCOMUD-Compare Vector Unsigned Doublewords.
ENCODING VPCOMUD => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xee P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPCOMUD',
};


# VPCOMUQ-Compare Vector Unsigned Quadwords.
ENCODING VPCOMUQ => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xef P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPCOMUQ',
};


# VPCOMUW-Compare Vector Unsigned Words.
ENCODING VPCOMUW => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xed P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPCOMUW',
};


# VPCOMW-Compare Vector Signed Words.
ENCODING VPCOMW => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xcd P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPCOMW',
};


# VPERMIL2PS-Permute Two-Source Single-Precision Floating-Point Values.
ENCODING VPERMIL2PS_rxnxis4xmxis4 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x48 P66=1 PF2=0 PF3=0 W=1',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPERMIL2PS',
};

ENCODING VPERMIL2PS_rxnxmxis4xis4 => {
  diagram    => 'ENC=VEX MAP=0f3a MR=1 OP=0x48 P66=1 PF2=0 PF3=0 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPERMIL2PS',
};


# VPHADDBD-Packed Horizontal Add Signed Byte to Signed Doubleword.
ENCODING VPHADDBD => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0xc2 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPHADDBD',
};


# VPHADDBQ-Packed Horizontal Add Signed Byte to Signed Quadword.
ENCODING VPHADDBQ => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0xc3 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPHADDBQ',
};


# VPHADDBW-Packed Horizontal Add Signed Byte to Signed Word.
ENCODING VPHADDBW => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0xc1 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPHADDBW',
};


# VPHADDDQ-Packed Horizontal Add Signed Doubleword to Signed Quadword.
ENCODING VPHADDDQ => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0xcb P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPHADDDQ',
};


# VPHADDUBD-Packed Horizontal Add Unsigned Byte to Doubleword.
ENCODING VPHADDUBD => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0xd2 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPHADDUBD',
};


# VPHADDUBQ-Packed Horizontal Add Unsigned Byte to Quadword.
ENCODING VPHADDUBQ => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0xd3 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPHADDUBQ',
};


# VPHADDUBW-Packed Horizontal Add Unsigned Byte to Word.
ENCODING VPHADDUBW => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0xd1 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPHADDUBW',
};


# VPHADDUDQ-Packed Horizontal Add Unsigned Doubleword to Quadword.
ENCODING VPHADDUDQ => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0xdb P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPHADDUDQ',
};


# VPHADDUWQ-Packed Horizontal Add Unsigned Word to Quadword.
ENCODING VPHADDUWQ => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0xd7 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPHADDUWQ',
};


# VPHADDWD-Packed Horizontal Add Signed Word to Signed Doubleword.
ENCODING VPHADDWD => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0xc6 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPHADDWD',
};


# VPHADDWQ-Packed Horizontal Add Signed Word to Signed Quadword.
ENCODING VPHADDWQ => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0xc7 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPHADDWQ',
};


# VPHSUBBW-Packed Horizontal Subtract Signed Byte to Signed Word.
ENCODING VPHSUBBW => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0xe1 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPHSUBBW',
};


# VPHSUBDQ-Packed Horizontal Subtract Signed Doubleword to Signed Quadword.
ENCODING VPHSUBDQ => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0xe3 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPHSUBDQ',
};


# VPHSUBWD-Packed Horizontal Subtract Signed Word to Signed Doubleword.
ENCODING VPHSUBWD => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0xe2 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPHSUBWD',
};


# VPMACSDD-Packed Multiply Accumulate Signed Doubleword to Signed Doubleword.
ENCODING VPMACSDD => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0x9e P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPMACSDD',
};


# VPMACSDQH-Packed Multiply Accumulate Signed High Doubleword to Signed Quadword.
ENCODING VPMACSDQH => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0x9f P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPMACSDQH',
};


# VPMACSDQL-Packed Multiply Accumulate Signed Low Doubleword to Signed Quadword.
ENCODING VPMACSDQL => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0x97 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPMACSDQL',
};


# VPMACSSDD-Packed Multiply Accumulate Signed Doubleword to Signed Doubleword with Saturation.
ENCODING VPMACSSDD => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0x8e P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPMACSSDD',
};


# VPMACSSDQH-Packed Multiply Accumulate Signed High Doubleword to Signed Quadword with.
ENCODING VPMACSSDQH => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0x8f P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPMACSSDQH',
};


# VPMACSSDQL-Packed Multiply Accumulate Signed Low Doubleword to Signed Quadword with.
ENCODING VPMACSSDQL => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0x87 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPMACSSDQL',
};


# VPMACSSWD-Packed Multiply Accumulate Signed Word to Signed Doubleword with Saturation.
ENCODING VPMACSSWD => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0x86 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPMACSSWD',
};


# VPMACSSWW-Packed Multiply Accumulate Signed Word to Signed Word with Saturation.
ENCODING VPMACSSWW => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0x85 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPMACSSWW',
};


# VPMACSWD-Packed Multiply Accumulate Signed Word to Signed Doubleword.
ENCODING VPMACSWD => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0x96 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPMACSWD',
};


# VPMACSWW-Packed Multiply Accumulate Signed Word to Signed Word.
ENCODING VPMACSWW => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0x95 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPMACSWW',
};


# VPMADCSSWD-Packed Multiply, Add and Accumulate Signed Word to Signed Doubleword with Saturation.
ENCODING VPMADCSSWD => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xa6 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPMADCSSWD',
};


# VPMADCSWD-Packed Multiply Add and Accumulate Signed Word to Signed Doubleword.
ENCODING VPMADCSWD => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xb6 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPMADCSWD',
};


# VPPERM-Packed Permute Bytes.
ENCODING VPPERM_ronois4omo => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xa3 P66=0 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPPERM',
};

ENCODING VPPERM_ronomois4o => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xa3 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPPERM',
};


# VPROTB-Packed Rotate Bytes.
ENCODING VPROTB_romono => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x90 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPROTB',
};

ENCODING VPROTB_romoub => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xc0 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPROTB',
};

ENCODING VPROTB_ronomo => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x90 P66=0 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPROTB',
};


# VPROTD-Packed Rotate Doublewords.
ENCODING VPROTD_romono => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x92 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPROTD',
};

ENCODING VPROTD_romoub => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xc2 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPROTD',
};

ENCODING VPROTD_ronomo => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x92 P66=0 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPROTD',
};


# VPROTQ-Packed Rotate Quadwords.
ENCODING VPROTQ_romono => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x93 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPROTQ',
};

ENCODING VPROTQ_romoub => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xc3 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPROTQ',
};

ENCODING VPROTQ_ronomo => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x93 P66=0 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPROTQ',
};


# VPROTW-Packed Rotate Words.
ENCODING VPROTW_romono => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x91 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPROTW',
};

ENCODING VPROTW_romoub => {
  diagram    => 'ENC=XOP MAP=xop8 MR=1 OP=0xc1 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPROTW',
};

ENCODING VPROTW_ronomo => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x91 P66=0 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPROTW',
};


# VPSHAB-Packed Shift Arithmetic Bytes.
ENCODING VPSHAB_romono => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x98 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPSHAB',
};

ENCODING VPSHAB_ronomo => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x98 P66=0 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPSHAB',
};


# VPSHAD-Packed Shift Arithmetic Doublewords.
ENCODING VPSHAD_romono => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x9a P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPSHAD',
};

ENCODING VPSHAD_ronomo => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x9a P66=0 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPSHAD',
};


# VPSHAQ-Packed Shift Arithmetic Quadwords.
ENCODING VPSHAQ_romono => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x9b P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPSHAQ',
};

ENCODING VPSHAQ_ronomo => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x9b P66=0 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPSHAQ',
};


# VPSHAW-Packed Shift Arithmetic Words.
ENCODING VPSHAW_romono => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x99 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPSHAW',
};

ENCODING VPSHAW_ronomo => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x99 P66=0 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPSHAW',
};


# VPSHLB-Packed Shift Logical Bytes.
ENCODING VPSHLB_romono => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x94 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPSHLB',
};

ENCODING VPSHLB_ronomo => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x94 P66=0 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPSHLB',
};


# VPSHLD-Packed Shift Logical Doublewords.
ENCODING VPSHLD_romono => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x96 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPSHLD',
};

ENCODING VPSHLD_ronomo => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x96 P66=0 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPSHLD',
};


# VPSHLQ-Packed Shift Logical Quadwords.
ENCODING VPSHLQ_romono => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x97 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPSHLQ',
};

ENCODING VPSHLQ_ronomo => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x97 P66=0 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPSHLQ',
};


# VPSHLW-Packed Shift Logical Words.
ENCODING VPSHLW_romono => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x95 P66=0 PF2=0 PF3=0 VL=128 W=0',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPSHLW',
};

ENCODING VPSHLW_ronomo => {
  diagram    => 'ENC=XOP MAP=xop9 MR=1 OP=0x95 P66=0 PF2=0 PF3=0 VL=128 W=1',
  extensions => 'XOP',
  categories => 'ADVSIMD',
  metadata   => 'isa=XOP',
  tags       => 'page=XOP_VPSHLW',
};

