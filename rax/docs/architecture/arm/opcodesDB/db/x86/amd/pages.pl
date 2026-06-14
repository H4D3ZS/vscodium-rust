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

PAGE BEXTR => {
  title => 'BEXTR-Bit Field Extract',
};

PAGE BLCFILL => {
  title => 'BLCFILL',
};

PAGE BLCI => {
  title => 'BLCI',
};

PAGE BLCIC => {
  title => 'BLCIC',
};

PAGE BLCMSK => {
  title => 'BLCMSK',
};

PAGE BLCS => {
  title => 'BLCS',
};

PAGE BLSFILL => {
  title => 'BLSFILL',
};

PAGE BLSIC => {
  title => 'BLSIC',
};

PAGE CLZERO => {
  title => 'CLZERO',
};

PAGE EXTRQ => {
  title => 'EXTRQ',
};

PAGE INSERTQ => {
  title => 'INSERTQ',
};

PAGE LPS_LLWPCB => {
  title => 'LLWPCB-Load LWPCB Address',
};

PAGE LPS_LWPINS => {
  title => 'LWPINS-Insert User Event Record in LWP Ring Buffer',
};

PAGE LPS_LWPVAL => {
  title => 'LWPVAL-Insert Value Sample in LWP Ring Buffer',
};

PAGE LPS_SLWPCB => {
  title => 'SLWPCB-Store LWPCB Address',
};

PAGE MONITORX => {
  title => 'MONITORX',
};

PAGE MOVNTSD => {
  title => 'MOVNTSD',
};

PAGE MOVNTSS => {
  title => 'MOVNTSS',
};

PAGE MWAITX => {
  title => 'MWAITX',
};

PAGE RDPRU => {
  title => 'RDPRU',
};

PAGE SVM_CLGI => {
  title => 'CLGI-Clear Global Interrupt Flag',
};

PAGE SVM_INVLPGA => {
  title => 'INVLPGA-Invalidate TLB Entry in a Specified ASID',
};

PAGE SVM_SKINIT => {
  title => 'SKINIT-Secure Init and Jump with Attestation',
};

PAGE SVM_STGI => {
  title => 'STGI-Set Global Interrupt Flag',
};

PAGE SVM_VMLOAD => {
  title => 'VMLOAD-Load State from VMCB',
};

PAGE SVM_VMMCALL => {
  title => 'VMMCALL-Call VMM',
};

PAGE SVM_VMRUN => {
  title => 'VMRUN-Run Virtual Machine',
};

PAGE SVM_VMSAVE => {
  title => 'VMSAVE-Save State to VMCB',
};

PAGE SYSCALL => {
  title => 'SYSCALL-Fast System Call',
};

PAGE SYSRET => {
  title => 'SYSRET-Return From Fast System Call',
};

PAGE T1MSKC => {
  title => 'T1MSKC',
};

PAGE TZMSK => {
  title => 'TZMSK',
};

PAGE VEX_VFMADDPD => {
  title => 'VFMADDPD-Multiply and Add Packed Double-Precision Floating-Point',
};

PAGE VEX_VFMADDPS => {
  title => 'VFMADDPS-Multiply and Add Packed Single-Precision Floating-Point',
};

PAGE VEX_VFMADDSD => {
  title => 'VFMADDSD-Multiply and Add Scalar Double-Precision Floating-Point',
};

PAGE VEX_VFMADDSS => {
  title => 'VFMADDSS-Multiply and Add Scalar Single-Precision Floating-Point',
};

PAGE VEX_VFMADDSUBPD => {
  title => 'VFMADDSUBPD-Multiply with Alternating Add/Subtract of Packed Double-Precision Floating-Point',
};

PAGE VEX_VFMADDSUBPS => {
  title => 'VFMADDSUBPS-Multiply with Alternating Add/Subtract of Packed Single-Precision Floating-Point',
};

PAGE VEX_VFMSUBADDPD => {
  title => 'VFMSUBADDPD-Multiply with Alternating Subtract/Add of Packed Double-Precision Floating-Point',
};

PAGE VEX_VFMSUBADDPS => {
  title => 'VFMSUBADDPS-Multiply with Alternating Subtract/Add of Packed Single-Precision Floating-Point',
};

PAGE VEX_VFMSUBPD => {
  title => 'VFMSUBPD-Multiply and Subtract Packed Double-Precision Floating-Point',
};

PAGE VEX_VFMSUBPS => {
  title => 'VFMSUBPS-Multiply and Subtract Packed Single-Precision Floating-Point',
};

PAGE VEX_VFMSUBSD => {
  title => 'VFMSUBSD-Multiply and Subtract Scalar Double-Precision Floating-Point',
};

PAGE VEX_VFMSUBSS => {
  title => 'VFMSUBSS-Multiply and Subtract Scalar Single-Precision Floating-Point',
};

PAGE VEX_VFNMADDPD => {
  title => 'VFNMADDPD-Negative Multiply and Add Packed Double-Precision Floating-Point',
};

PAGE VEX_VFNMADDPS => {
  title => 'VFNMADDPS-Negative Multiply and Add Packed Single-Precision Floating-Point',
};

PAGE VEX_VFNMADDSD => {
  title => 'VFNMADDSD-Negative Multiply and Add Scalar Double-Precision Floating-Point',
};

PAGE VEX_VFNMADDSS => {
  title => 'VFNMADDSS-Negative Multiply and Add Scalar Single-Precision Floating-Point',
};

PAGE VEX_VFNMSUBPD => {
  title => 'VFNMSUBPD-Negative Multiply and Subtract Packed Double-Precision Floating-Point',
};

PAGE VEX_VFNMSUBPS => {
  title => 'VFNMSUBPS-Negative Multiply and Subtract Packed Single-Precision Floating-Point',
};

PAGE VEX_VFNMSUBSD => {
  title => 'VFNMSUBSD-Negative Multiply and Subtract Scalar Double-Precision Floating-Point',
};

PAGE VEX_VFNMSUBSS => {
  title => 'VFNMSUBSS-Negative Multiply and Subtract Scalar Single-Precision Floating-Point',
};

PAGE VEX_VPERMIL2PD => {
  title => 'VPERMIL2PD-Permute Two-Source Double-Precision Floating-Point Values',
};

PAGE VPHADDUWD => {
  title => 'VPHADDUWD',
};

PAGE XOP_VFRCZPD => {
  title => 'VFRCZPD-Extract Fraction Packed Double-Precision Floating-Point',
};

PAGE XOP_VFRCZPS => {
  title => 'VFRCZPS-Extract Fraction Packed Single-Precision Floating-Point',
};

PAGE XOP_VFRCZSD => {
  title => 'VFRCZSD-Extract Fraction Scalar Double-Precision Floating-Point',
};

PAGE XOP_VFRCZSS => {
  title => 'VFRCZSS-Extract Fraction Scalar Single-Precision Floating Point',
};

PAGE XOP_VPCMOV => {
  title => 'VPCMOV-Vector Conditional Moves',
};

PAGE XOP_VPCOMB => {
  title => 'VPCOMB-Compare Vector Signed Bytes',
};

PAGE XOP_VPCOMD => {
  title => 'VPCOMD-Compare Vector Signed Doublewords',
};

PAGE XOP_VPCOMQ => {
  title => 'VPCOMQ-Compare Vector Signed Quadwords',
};

PAGE XOP_VPCOMUB => {
  title => 'VPCOMUB-Compare Vector Unsigned Bytes',
};

PAGE XOP_VPCOMUD => {
  title => 'VPCOMUD-Compare Vector Unsigned Doublewords',
};

PAGE XOP_VPCOMUQ => {
  title => 'VPCOMUQ-Compare Vector Unsigned Quadwords',
};

PAGE XOP_VPCOMUW => {
  title => 'VPCOMUW-Compare Vector Unsigned Words',
};

PAGE XOP_VPCOMW => {
  title => 'VPCOMW-Compare Vector Signed Words',
};

PAGE XOP_VPERMIL2PS => {
  title => 'VPERMIL2PS-Permute Two-Source Single-Precision Floating-Point Values',
};

PAGE XOP_VPHADDBD => {
  title => 'VPHADDBD-Packed Horizontal Add Signed Byte to Signed Doubleword',
};

PAGE XOP_VPHADDBQ => {
  title => 'VPHADDBQ-Packed Horizontal Add Signed Byte to Signed Quadword',
};

PAGE XOP_VPHADDBW => {
  title => 'VPHADDBW-Packed Horizontal Add Signed Byte to Signed Word',
};

PAGE XOP_VPHADDDQ => {
  title => 'VPHADDDQ-Packed Horizontal Add Signed Doubleword to Signed Quadword',
};

PAGE XOP_VPHADDUBD => {
  title => 'VPHADDUBD-Packed Horizontal Add Unsigned Byte to Doubleword',
};

PAGE XOP_VPHADDUBQ => {
  title => 'VPHADDUBQ-Packed Horizontal Add Unsigned Byte to Quadword',
};

PAGE XOP_VPHADDUBW => {
  title => 'VPHADDUBW-Packed Horizontal Add Unsigned Byte to Word',
};

PAGE XOP_VPHADDUDQ => {
  title => 'VPHADDUDQ-Packed Horizontal Add Unsigned Doubleword to Quadword',
};

PAGE XOP_VPHADDUWQ => {
  title => 'VPHADDUWQ-Packed Horizontal Add Unsigned Word to Quadword',
};

PAGE XOP_VPHADDWD => {
  title => 'VPHADDWD-Packed Horizontal Add Signed Word to Signed Doubleword',
};

PAGE XOP_VPHADDWQ => {
  title => 'VPHADDWQ-Packed Horizontal Add Signed Word to Signed Quadword',
};

PAGE XOP_VPHSUBBW => {
  title => 'VPHSUBBW-Packed Horizontal Subtract Signed Byte to Signed Word',
};

PAGE XOP_VPHSUBDQ => {
  title => 'VPHSUBDQ-Packed Horizontal Subtract Signed Doubleword to Signed Quadword',
};

PAGE XOP_VPHSUBWD => {
  title => 'VPHSUBWD-Packed Horizontal Subtract Signed Word to Signed Doubleword',
};

PAGE XOP_VPMACSDD => {
  title => 'VPMACSDD-Packed Multiply Accumulate Signed Doubleword to Signed Doubleword',
};

PAGE XOP_VPMACSDQH => {
  title => 'VPMACSDQH-Packed Multiply Accumulate Signed High Doubleword to Signed Quadword',
};

PAGE XOP_VPMACSDQL => {
  title => 'VPMACSDQL-Packed Multiply Accumulate Signed Low Doubleword to Signed Quadword',
};

PAGE XOP_VPMACSSDD => {
  title => 'VPMACSSDD-Packed Multiply Accumulate Signed Doubleword to Signed Doubleword with Saturation',
};

PAGE XOP_VPMACSSDQH => {
  title => 'VPMACSSDQH-Packed Multiply Accumulate Signed High Doubleword to Signed Quadword with',
};

PAGE XOP_VPMACSSDQL => {
  title => 'VPMACSSDQL-Packed Multiply Accumulate Signed Low Doubleword to Signed Quadword with',
};

PAGE XOP_VPMACSSWD => {
  title => 'VPMACSSWD-Packed Multiply Accumulate Signed Word to Signed Doubleword with Saturation',
};

PAGE XOP_VPMACSSWW => {
  title => 'VPMACSSWW-Packed Multiply Accumulate Signed Word to Signed Word with Saturation',
};

PAGE XOP_VPMACSWD => {
  title => 'VPMACSWD-Packed Multiply Accumulate Signed Word to Signed Doubleword',
};

PAGE XOP_VPMACSWW => {
  title => 'VPMACSWW-Packed Multiply Accumulate Signed Word to Signed Word',
};

PAGE XOP_VPMADCSSWD => {
  title => 'VPMADCSSWD-Packed Multiply, Add and Accumulate Signed Word to Signed Doubleword with Saturation',
};

PAGE XOP_VPMADCSWD => {
  title => 'VPMADCSWD-Packed Multiply Add and Accumulate Signed Word to Signed Doubleword',
};

PAGE XOP_VPPERM => {
  title => 'VPPERM-Packed Permute Bytes',
};

PAGE XOP_VPROTB => {
  title => 'VPROTB-Packed Rotate Bytes',
};

PAGE XOP_VPROTD => {
  title => 'VPROTD-Packed Rotate Doublewords',
};

PAGE XOP_VPROTQ => {
  title => 'VPROTQ-Packed Rotate Quadwords',
};

PAGE XOP_VPROTW => {
  title => 'VPROTW-Packed Rotate Words',
};

PAGE XOP_VPSHAB => {
  title => 'VPSHAB-Packed Shift Arithmetic Bytes',
};

PAGE XOP_VPSHAD => {
  title => 'VPSHAD-Packed Shift Arithmetic Doublewords',
};

PAGE XOP_VPSHAQ => {
  title => 'VPSHAQ-Packed Shift Arithmetic Quadwords',
};

PAGE XOP_VPSHAW => {
  title => 'VPSHAW-Packed Shift Arithmetic Words',
};

PAGE XOP_VPSHLB => {
  title => 'VPSHLB-Packed Shift Logical Bytes',
};

PAGE XOP_VPSHLD => {
  title => 'VPSHLD-Packed Shift Logical Doublewords',
};

PAGE XOP_VPSHLQ => {
  title => 'VPSHLQ-Packed Shift Logical Quadwords',
};

PAGE XOP_VPSHLW => {
  title => 'VPSHLW-Packed Shift Logical Words',
};

