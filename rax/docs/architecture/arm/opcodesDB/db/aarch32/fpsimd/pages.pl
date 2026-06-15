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

PAGE AESD => {
  title      => 'AESD',
  brief      => 'AES single round decryption',
  docvars    => 'instr-class=fpsimd mnemonic=AESD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE AESE => {
  title      => 'AESE',
  brief      => 'AES single round encryption',
  docvars    => 'instr-class=fpsimd mnemonic=AESE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE AESIMC => {
  title      => 'AESIMC',
  brief      => 'AES inverse mix columns',
  docvars    => 'instr-class=fpsimd mnemonic=AESIMC',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE AESMC => {
  title      => 'AESMC',
  brief      => 'AES mix columns',
  docvars    => 'instr-class=fpsimd mnemonic=AESMC',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FLDMX => {
  title      => 'FLDM*X (FLDMDBX, FLDMIAX)',
  brief      => 'FLDM*X',
  docvars    => 'fpdatasize=doubleprec instr-class=fpsimd',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FSTMX => {
  title      => 'FSTMDBX, FSTMIAX',
  brief      => 'FSTMX',
  docvars    => 'fpdatasize=doubleprec instr-class=fpsimd',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA1C => {
  title      => 'SHA1C',
  brief      => 'SHA1 hash update (choose)',
  docvars    => 'instr-class=fpsimd mnemonic=SHA1C',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA1H => {
  title      => 'SHA1H',
  brief      => 'SHA1 fixed rotate',
  docvars    => 'instr-class=fpsimd mnemonic=SHA1H',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA1M => {
  title      => 'SHA1M',
  brief      => 'SHA1 hash update (majority)',
  docvars    => 'instr-class=fpsimd mnemonic=SHA1M',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA1P => {
  title      => 'SHA1P',
  brief      => 'SHA1 hash update (parity)',
  docvars    => 'instr-class=fpsimd mnemonic=SHA1P',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA1SU0 => {
  title      => 'SHA1SU0',
  brief      => 'SHA1 schedule update 0',
  docvars    => 'instr-class=fpsimd mnemonic=SHA1SU0',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA1SU1 => {
  title      => 'SHA1SU1',
  brief      => 'SHA1 schedule update 1',
  docvars    => 'instr-class=fpsimd mnemonic=SHA1SU1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA256H => {
  title      => 'SHA256H',
  brief      => 'SHA256 hash update part 1',
  docvars    => 'instr-class=fpsimd mnemonic=SHA256H',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA256H2 => {
  title      => 'SHA256H2',
  brief      => 'SHA256 hash update part 2',
  docvars    => 'instr-class=fpsimd mnemonic=SHA256H2',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA256SU0 => {
  title      => 'SHA256SU0',
  brief      => 'SHA256 schedule update 0',
  docvars    => 'instr-class=fpsimd mnemonic=SHA256SU0',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA256SU1 => {
  title      => 'SHA256SU1',
  brief      => 'SHA256 schedule update 1',
  docvars    => 'instr-class=fpsimd mnemonic=SHA256SU1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ST4_1 => {
  title      => 'VST4 (single 4-element structure from one lane)',
  brief      => 'Store single 4-element structure from one lane of four registers',
  docvars    => 'address-form=base-plus-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VABA => {
  title      => 'VABA',
  brief      => 'Vector Absolute Difference and Accumulate',
  docvars    => 'instr-class=fpsimd mnemonic=VABA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VABAL => {
  title      => 'VABAL',
  brief      => 'Vector Absolute Difference and Accumulate Long',
  docvars    => 'instr-class=fpsimd mnemonic=VABAL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VABD_f => {
  title      => 'VABD (floating-point)',
  brief      => 'Vector Absolute Difference (floating-point)',
  docvars    => 'instr-class=fpsimd mnemonic=VABD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VABD_i => {
  title      => 'VABD (integer)',
  brief      => 'Vector Absolute Difference (integer)',
  docvars    => 'instr-class=fpsimd mnemonic=VABD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VABDL_i => {
  title      => 'VABDL (integer)',
  brief      => 'Vector Absolute Difference Long (integer)',
  docvars    => 'instr-class=fpsimd mnemonic=VABDL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VABS => {
  title      => 'VABS',
  brief      => 'Vector Absolute',
  docvars    => 'instr-class=fpsimd mnemonic=VABS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VACGE => {
  title      => 'VACGE',
  brief      => 'Vector Absolute Compare Greater Than or Equal',
  docvars    => 'instr-class=fpsimd mnemonic=VACGE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VACGT => {
  title      => 'VACGT',
  brief      => 'Vector Absolute Compare Greater Than',
  docvars    => 'instr-class=fpsimd mnemonic=VACGT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VADD_f => {
  title      => 'VADD (floating-point)',
  brief      => 'Vector Add (floating-point)',
  docvars    => 'instr-class=fpsimd mnemonic=VADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VADD_i => {
  title      => 'VADD (integer)',
  brief      => 'Vector Add (integer)',
  docvars    => 'instr-class=fpsimd mnemonic=VADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VADDHN => {
  title      => 'VADDHN',
  brief      => 'Vector Add and Narrow, returning High Half',
  docvars    => 'instr-class=fpsimd mnemonic=VADDHN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VADDL => {
  title      => 'VADDL',
  brief      => 'Vector Add Long',
  docvars    => 'instr-class=fpsimd mnemonic=VADDL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VADDW => {
  title      => 'VADDW',
  brief      => 'Vector Add Wide',
  docvars    => 'instr-class=fpsimd mnemonic=VADDW',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VAND_r => {
  title      => 'VAND (register)',
  brief      => 'Vector Bitwise AND (register)',
  docvars    => 'instr-class=fpsimd mnemonic=VAND',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VBIC_i => {
  title      => 'VBIC (immediate)',
  brief      => 'Vector Bitwise Bit Clear (immediate)',
  docvars    => 'instr-class=fpsimd mnemonic=VBIC',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VBIC_r => {
  title      => 'VBIC (register)',
  brief      => 'Vector Bitwise Bit Clear (register)',
  docvars    => 'instr-class=fpsimd mnemonic=VBIC',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VBIF => {
  title      => 'VBIF',
  brief      => 'Vector Bitwise Insert if False',
  docvars    => 'instr-class=fpsimd mnemonic=VBIF',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VBIT => {
  title      => 'VBIT',
  brief      => 'Vector Bitwise Insert if True',
  docvars    => 'instr-class=fpsimd mnemonic=VBIT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VBSL => {
  title      => 'VBSL',
  brief      => 'Vector Bitwise Select',
  docvars    => 'instr-class=fpsimd mnemonic=VBSL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCADD => {
  title      => 'VCADD',
  brief      => 'Vector Complex Add',
  docvars    => 'instr-class=fpsimd mnemonic=VCADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCEQ_i => {
  title      => 'VCEQ (immediate #0)',
  brief      => 'Vector Compare Equal to Zero',
  docvars    => 'instr-class=fpsimd mnemonic=VCEQ',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCEQ_r => {
  title      => 'VCEQ (register)',
  brief      => 'Vector Compare Equal',
  docvars    => 'instr-class=fpsimd mnemonic=VCEQ',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCGE_i => {
  title      => 'VCGE (immediate #0)',
  brief      => 'Vector Compare Greater Than or Equal to Zero',
  docvars    => 'instr-class=fpsimd mnemonic=VCGE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCGE_r => {
  title      => 'VCGE (register)',
  brief      => 'Vector Compare Greater Than or Equal',
  docvars    => 'instr-class=fpsimd mnemonic=VCGE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCGT_i => {
  title      => 'VCGT (immediate #0)',
  brief      => 'Vector Compare Greater Than Zero',
  docvars    => 'instr-class=fpsimd mnemonic=VCGT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCGT_r => {
  title      => 'VCGT (register)',
  brief      => 'Vector Compare Greater Than',
  docvars    => 'instr-class=fpsimd mnemonic=VCGT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCLE_i => {
  title      => 'VCLE (immediate #0)',
  brief      => 'Vector Compare Less Than or Equal to Zero',
  docvars    => 'instr-class=fpsimd mnemonic=VCLE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCLS => {
  title      => 'VCLS',
  brief      => 'Vector Count Leading Sign Bits',
  docvars    => 'instr-class=fpsimd mnemonic=VCLS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCLT_i => {
  title      => 'VCLT (immediate #0)',
  brief      => 'Vector Compare Less Than Zero',
  docvars    => 'instr-class=fpsimd mnemonic=VCLT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCLZ => {
  title      => 'VCLZ',
  brief      => 'Vector Count Leading Zeros',
  docvars    => 'instr-class=fpsimd mnemonic=VCLZ',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCMLA => {
  title      => 'VCMLA',
  brief      => 'Vector Complex Multiply Accumulate',
  docvars    => 'instr-class=fpsimd mnemonic=VCMLA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCMLA_s => {
  title      => 'VCMLA (by element)',
  brief      => 'Vector Complex Multiply Accumulate (by element)',
  docvars    => 'instr-class=fpsimd mnemonic=VCMLA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCMP => {
  title      => 'VCMP',
  brief      => 'Vector Compare',
  docvars    => 'instr-class=fpsimd mnemonic=VCMP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCMPE => {
  title      => 'VCMPE',
  brief      => 'Vector Compare, raising Invalid Operation on NaN',
  docvars    => 'instr-class=fpsimd mnemonic=VCMPE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCNT => {
  title      => 'VCNT',
  brief      => 'Vector Count Set Bits',
  docvars    => 'instr-class=fpsimd mnemonic=VCNT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVT_bfs => {
  title      => 'VCVT (from single-precision to BFloat16, Advanced SIMD)',
  brief      => 'Vector Convert from single-precision to BFloat16',
  docvars    => 'instr-class=fpsimd mnemonic=VCVT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVT_ds => {
  title      => 'VCVT (between double-precision and single-precision)',
  brief      => 'Convert between double-precision and single-precision',
  docvars    => 'instr-class=fpsimd mnemonic=VCVT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVT_hs => {
  title      => 'VCVT (between half-precision and single-precision, Advanced SIMD)',
  brief      => 'Vector Convert between half-precision and single-precision',
  docvars    => 'instr-class=fpsimd mnemonic=VCVT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVT_is => {
  title      => 'VCVT (between floating-point and integer, Advanced SIMD)',
  brief      => 'Vector Convert between floating-point and integer',
  docvars    => 'instr-class=fpsimd mnemonic=VCVT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVT_iv => {
  title      => 'VCVT (floating-point to integer, floating-point)',
  brief      => 'Convert floating-point to integer with Round towards Zero',
  docvars    => 'instr-class=fpsimd mnemonic=VCVT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVT_vi => {
  title      => 'VCVT (integer to floating-point, floating-point)',
  brief      => 'Convert integer to floating-point',
  docvars    => 'instr-class=fpsimd mnemonic=VCVT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVT_xs => {
  title      => 'VCVT (between floating-point and fixed-point, Advanced SIMD)',
  brief      => 'Vector Convert between floating-point and fixed-point',
  docvars    => 'instr-class=fpsimd mnemonic=VCVT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVT_xv => {
  title      => 'VCVT (between floating-point and fixed-point, floating-point)',
  brief      => 'Convert between floating-point and fixed-point',
  docvars    => 'instr-class=fpsimd mnemonic=VCVT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVTA_asimd => {
  title      => 'VCVTA (Advanced SIMD)',
  brief      => 'Vector Convert floating-point to integer with Round to Nearest with Ties to Away',
  docvars    => 'instr-class=fpsimd mnemonic=VCVTA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVTA_vfp => {
  title      => 'VCVTA (floating-point)',
  brief      => 'Convert floating-point to integer with Round to Nearest with Ties to Away',
  docvars    => 'instr-class=fpsimd mnemonic=VCVTA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVTB => {
  title      => 'VCVTB',
  brief      => 'Convert to or from a half-precision value in the bottom half of a single-precision register',
  docvars    => 'instr-class=fpsimd mnemonic=VCVTB',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVTB_bfs => {
  title      => 'VCVTB (BFloat16)',
  brief      => 'Converts from a single-precision value to a BFloat16 value in the bottom half of a single-precision register',
  docvars    => 'instr-class=fpsimd mnemonic=VCVTB',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVTM_asimd => {
  title      => 'VCVTM (Advanced SIMD)',
  brief      => 'Vector Convert floating-point to integer with Round towards -Infinity',
  docvars    => 'instr-class=fpsimd mnemonic=VCVTM',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVTM_vfp => {
  title      => 'VCVTM (floating-point)',
  brief      => 'Convert floating-point to integer with Round towards -Infinity',
  docvars    => 'instr-class=fpsimd mnemonic=VCVTM',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVTN_asimd => {
  title      => 'VCVTN (Advanced SIMD)',
  brief      => 'Vector Convert floating-point to integer with Round to Nearest',
  docvars    => 'instr-class=fpsimd mnemonic=VCVTN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVTN_vfp => {
  title      => 'VCVTN (floating-point)',
  brief      => 'Convert floating-point to integer with Round to Nearest',
  docvars    => 'instr-class=fpsimd mnemonic=VCVTN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVTP_asimd => {
  title      => 'VCVTP (Advanced SIMD)',
  brief      => 'Vector Convert floating-point to integer with Round towards +Infinity',
  docvars    => 'instr-class=fpsimd mnemonic=VCVTP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVTP_vfp => {
  title      => 'VCVTP (floating-point)',
  brief      => 'Convert floating-point to integer with Round towards +Infinity',
  docvars    => 'instr-class=fpsimd mnemonic=VCVTP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVTR_iv => {
  title      => 'VCVTR',
  brief      => 'Convert floating-point to integer',
  docvars    => 'instr-class=fpsimd mnemonic=VCVTR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVTT => {
  title      => 'VCVTT',
  brief      => 'Convert to or from a half-precision value in the top half of a single-precision register',
  docvars    => 'instr-class=fpsimd mnemonic=VCVTT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VCVTT_bfs => {
  title      => 'VCVTT (BFloat16)',
  brief      => 'Converts from a single-precision value to a BFloat16 value in the top half of a single-precision register.',
  docvars    => 'instr-class=fpsimd mnemonic=VCVTT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VDIV => {
  title      => 'VDIV',
  brief      => 'Divide',
  docvars    => 'instr-class=fpsimd mnemonic=VDIV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VDOT => {
  title      => 'VDOT (vector)',
  brief      => 'BFloat16 floating-point (BF16) dot product (vector)',
  docvars    => 'instr-class=fpsimd mnemonic=VDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VDOT_s => {
  title      => 'VDOT (by element)',
  brief      => 'BFloat16 floating-point indexed dot product (vector, by element)',
  docvars    => 'instr-class=fpsimd mnemonic=VDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VDUP_r => {
  title      => 'VDUP (general-purpose register)',
  brief      => 'Duplicate general-purpose register to vector',
  docvars    => 'instr-class=fpsimd mnemonic=VDUP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VDUP_s => {
  title      => 'VDUP (scalar)',
  brief      => 'Duplicate vector element to vector',
  docvars    => 'instr-class=fpsimd mnemonic=VDUP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VEOR => {
  title      => 'VEOR',
  brief      => 'Vector Bitwise Exclusive OR',
  docvars    => 'instr-class=fpsimd mnemonic=VEOR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VEXT => {
  title      => 'VEXT (byte elements)',
  brief      => 'Vector Extract',
  docvars    => 'instr-class=fpsimd mnemonic=VEXT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VFMA => {
  title      => 'VFMA',
  brief      => 'Vector Fused Multiply Accumulate',
  docvars    => 'instr-class=fpsimd mnemonic=VFMA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VFMA_bf => {
  title      => 'VFMAB, VFMAT (BFloat16, vector)',
  brief      => 'BFloat16 floating-point widening multiply-add long (vector)',
  docvars    => 'instr-class=fpsimd mnemonic=VFMA simdvectorsize=quad',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VFMA_bfs => {
  title      => 'VFMAB, VFMAT (BFloat16, by scalar)',
  brief      => 'BFloat16 floating-point widening multiply-add long (by scalar)',
  docvars    => 'instr-class=fpsimd mnemonic=VFMA simdvectorsize=quad',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VFMAL => {
  title      => 'VFMAL (vector)',
  brief      => 'Vector Floating-point Multiply-Add Long to accumulator (vector)',
  docvars    => 'instr-class=fpsimd mnemonic=VFMAL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VFMAL_s => {
  title      => 'VFMAL (by scalar)',
  brief      => 'Vector Floating-point Multiply-Add Long to accumulator (by scalar)',
  docvars    => 'instr-class=fpsimd mnemonic=VFMAL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VFMS => {
  title      => 'VFMS',
  brief      => 'Vector Fused Multiply Subtract',
  docvars    => 'instr-class=fpsimd mnemonic=VFMS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VFMSL => {
  title      => 'VFMSL (vector)',
  brief      => 'Vector Floating-point Multiply-Subtract Long from accumulator (vector)',
  docvars    => 'instr-class=fpsimd mnemonic=VFMSL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VFMSL_s => {
  title      => 'VFMSL (by scalar)',
  brief      => 'Vector Floating-point Multiply-Subtract Long from accumulator (by scalar)',
  docvars    => 'instr-class=fpsimd mnemonic=VFMSL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VFNMA => {
  title      => 'VFNMA',
  brief      => 'Vector Fused Negate Multiply Accumulate',
  docvars    => 'instr-class=fpsimd mnemonic=VFNMA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VFNMS => {
  title      => 'VFNMS',
  brief      => 'Vector Fused Negate Multiply Subtract',
  docvars    => 'instr-class=fpsimd mnemonic=VFNMS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VHADD => {
  title      => 'VHADD',
  brief      => 'Vector Halving Add',
  docvars    => 'instr-class=fpsimd mnemonic=VHADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VHSUB => {
  title      => 'VHSUB',
  brief      => 'Vector Halving Subtract',
  docvars    => 'instr-class=fpsimd mnemonic=VHSUB',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VINS => {
  title      => 'VINS',
  brief      => 'Vector move Insertion',
  docvars    => 'instr-class=fpsimd mnemonic=VINS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VJCVT => {
  title      => 'VJCVT',
  brief      => 'Javascript Convert to signed fixed-point, rounding toward Zero',
  docvars    => 'convert-type=double-to-single instr-class=fpsimd mnemonic=VJCVT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VLD1_1 => {
  title      => 'VLD1 (single element to one lane)',
  brief      => 'Load single 1-element structure to one lane of one register',
  docvars    => 'instr-class=fpsimd mnemonic=VLD1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VLD1_a => {
  title      => 'VLD1 (single element to all lanes)',
  brief      => 'Load single 1-element structure and replicate to all lanes of one register',
  docvars    => 'instr-class=fpsimd mnemonic=VLD1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VLD1_m => {
  title      => 'VLD1 (multiple single elements)',
  brief      => 'Load multiple single 1-element structures to one, two, three, or four registers',
  docvars    => 'instr-class=fpsimd mnemonic=VLD1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VLD2_1 => {
  title      => 'VLD2 (single 2-element structure to one lane)',
  brief      => 'Load single 2-element structure to one lane of two registers',
  docvars    => 'instr-class=fpsimd mnemonic=VLD2',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VLD2_a => {
  title      => 'VLD2 (single 2-element structure to all lanes)',
  brief      => 'Load single 2-element structure and replicate to all lanes of two registers',
  docvars    => 'instr-class=fpsimd mnemonic=VLD2',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VLD2_m => {
  title      => 'VLD2 (multiple 2-element structures)',
  brief      => 'Load multiple 2-element structures to two or four registers',
  docvars    => 'instr-class=fpsimd mnemonic=VLD2',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VLD3_1 => {
  title      => 'VLD3 (single 3-element structure to one lane)',
  brief      => 'Load single 3-element structure to one lane of three registers',
  docvars    => 'instr-class=fpsimd mnemonic=VLD3',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VLD3_a => {
  title      => 'VLD3 (single 3-element structure to all lanes)',
  brief      => 'Load single 3-element structure and replicate to all lanes of three registers',
  docvars    => 'instr-class=fpsimd mnemonic=VLD3',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VLD3_m => {
  title      => 'VLD3 (multiple 3-element structures)',
  brief      => 'Load multiple 3-element structures to three registers',
  docvars    => 'instr-class=fpsimd mnemonic=VLD3',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VLD4_1 => {
  title      => 'VLD4 (single 4-element structure to one lane)',
  brief      => 'Load single 4-element structure to one lane of four registers',
  docvars    => 'instr-class=fpsimd mnemonic=VLD4',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VLD4_a => {
  title      => 'VLD4 (single 4-element structure to all lanes)',
  brief      => 'Load single 4-element structure and replicate to all lanes of four registers',
  docvars    => 'instr-class=fpsimd mnemonic=VLD4',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VLD4_m => {
  title      => 'VLD4 (multiple 4-element structures)',
  brief      => 'Load multiple 4-element structures to four registers',
  docvars    => 'instr-class=fpsimd mnemonic=VLD4',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VLDM => {
  title      => 'VLDM, VLDMDB, VLDMIA',
  brief      => 'Load Multiple SIMD&FP registers',
  docvars    => 'instr-class=fpsimd',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VLDR_i => {
  title      => 'VLDR (immediate)',
  brief      => 'Load SIMD&FP register (immediate)',
  docvars    => 'address-form=base-plus-offset instr-class=fpsimd mnemonic=VLDR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VLDR_l => {
  title      => 'VLDR (literal)',
  brief      => 'Load SIMD&FP register (literal)',
  docvars    => 'address-form=literal instr-class=fpsimd mnemonic=VLDR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMAX_f => {
  title      => 'VMAX (floating-point)',
  brief      => 'Vector Maximum (floating-point)',
  docvars    => 'instr-class=fpsimd mnemonic=VMAX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMAX_i => {
  title      => 'VMAX (integer)',
  brief      => 'Vector Maximum (integer)',
  docvars    => 'instr-class=fpsimd mnemonic=VMAX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMAXNM => {
  title      => 'VMAXNM',
  brief      => 'Floating-point Maximum Number',
  docvars    => 'instr-class=fpsimd mnemonic=VMAXNM',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMIN_f => {
  title      => 'VMIN (floating-point)',
  brief      => 'Vector Minimum (floating-point)',
  docvars    => 'instr-class=fpsimd mnemonic=VMIN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMIN_i => {
  title      => 'VMIN (integer)',
  brief      => 'Vector Minimum (integer)',
  docvars    => 'instr-class=fpsimd mnemonic=VMIN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMINNM => {
  title      => 'VMINNM',
  brief      => 'Floating-point Minimum Number',
  docvars    => 'instr-class=fpsimd mnemonic=VMINNM',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMLA_f => {
  title      => 'VMLA (floating-point)',
  brief      => 'Vector Multiply Accumulate (floating-point)',
  docvars    => 'instr-class=fpsimd mnemonic=VMLA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMLA_i => {
  title      => 'VMLA (integer)',
  brief      => 'Vector Multiply Accumulate (integer)',
  docvars    => 'instr-class=fpsimd mnemonic=VMLA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMLA_s => {
  title      => 'VMLA (by scalar)',
  brief      => 'Vector Multiply Accumulate (by scalar)',
  docvars    => 'instr-class=fpsimd mnemonic=VMLA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMLAL_i => {
  title      => 'VMLAL (integer)',
  brief      => 'Vector Multiply Accumulate Long (integer)',
  docvars    => 'instr-class=fpsimd mnemonic=VMLAL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMLAL_s => {
  title      => 'VMLAL (by scalar)',
  brief      => 'Vector Multiply Accumulate Long (by scalar)',
  docvars    => 'instr-class=fpsimd mnemonic=VMLAL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMLS_f => {
  title      => 'VMLS (floating-point)',
  brief      => 'Vector Multiply Subtract (floating-point)',
  docvars    => 'instr-class=fpsimd mnemonic=VMLS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMLS_i => {
  title      => 'VMLS (integer)',
  brief      => 'Vector Multiply Subtract (integer)',
  docvars    => 'instr-class=fpsimd mnemonic=VMLS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMLS_s => {
  title      => 'VMLS (by scalar)',
  brief      => 'Vector Multiply Subtract (by scalar)',
  docvars    => 'instr-class=fpsimd mnemonic=VMLS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMLSL_i => {
  title      => 'VMLSL (integer)',
  brief      => 'Vector Multiply Subtract Long (integer)',
  docvars    => 'instr-class=fpsimd mnemonic=VMLSL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMLSL_s => {
  title      => 'VMLSL (by scalar)',
  brief      => 'Vector Multiply Subtract Long (by scalar)',
  docvars    => 'instr-class=fpsimd mnemonic=VMLSL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMMLA => {
  title      => 'VMMLA',
  brief      => 'BFloat16 floating-point matrix multiply-accumulate',
  docvars    => 'instr-class=fpsimd mnemonic=VMMLA simdvectorsize=quad',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMOV_d => {
  title      => 'VMOV (between two general-purpose registers and a doubleword floating-point register)',
  brief      => 'Copy two general-purpose registers to or from a SIMD&FP register',
  docvars    => 'instr-class=fpsimd mnemonic=VMOV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMOV_h => {
  title      => 'VMOV (between general-purpose register and half-precision)',
  brief      => 'Copy 16 bits of a general-purpose register to or from a 32-bit SIMD&FP register',
  docvars    => 'instr-class=fpsimd mnemonic=VMOV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMOV_i => {
  title      => 'VMOV (immediate)',
  brief      => 'Copy immediate value to a SIMD&FP register',
  docvars    => 'instr-class=fpsimd mnemonic=VMOV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMOV_r => {
  title      => 'VMOV (register)',
  brief      => 'Copy between FP registers',
  docvars    => 'instr-class=fpsimd mnemonic=VMOV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMOV_rs => {
  title      => 'VMOV (general-purpose register to scalar)',
  brief      => 'Copy a general-purpose register to a vector element',
  docvars    => 'instr-class=fpsimd mnemonic=VMOV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMOV_s => {
  title      => 'VMOV (between general-purpose register and single-precision)',
  brief      => 'Copy a general-purpose register to or from a 32-bit SIMD&FP register',
  docvars    => 'instr-class=fpsimd mnemonic=VMOV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMOV_sr => {
  title      => 'VMOV (scalar to general-purpose register)',
  brief      => 'Copy a vector element to a general-purpose register with sign or zero extension',
  docvars    => 'instr-class=fpsimd mnemonic=VMOV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMOV_ss => {
  title      => 'VMOV (between two general-purpose registers and two single-precision registers)',
  brief      => 'Copy two general-purpose registers to a pair of 32-bit SIMD&FP registers',
  docvars    => 'instr-class=fpsimd mnemonic=VMOV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMOVL => {
  title      => 'VMOVL',
  brief      => 'Vector Move Long',
  docvars    => 'instr-class=fpsimd mnemonic=VMOVL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMOVN => {
  title      => 'VMOVN',
  brief      => 'Vector Move and Narrow',
  docvars    => 'instr-class=fpsimd mnemonic=VMOVN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMOVX => {
  title      => 'VMOVX',
  brief      => 'Vector Move extraction',
  docvars    => 'instr-class=fpsimd mnemonic=VMOVX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMRS => {
  title      => 'VMRS',
  brief      => 'Move SIMD&FP Special register to general-purpose register',
  docvars    => 'instr-class=fpsimd mnemonic=VMRS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMSR => {
  title      => 'VMSR',
  brief      => 'Move general-purpose register to SIMD&FP Special register',
  docvars    => 'instr-class=fpsimd mnemonic=VMSR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMUL_f => {
  title      => 'VMUL (floating-point)',
  brief      => 'Vector Multiply (floating-point)',
  docvars    => 'instr-class=fpsimd mnemonic=VMUL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMUL_i => {
  title      => 'VMUL (integer and polynomial)',
  brief      => 'Vector Multiply (integer and polynomial)',
  docvars    => 'instr-class=fpsimd mnemonic=VMUL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMUL_s => {
  title      => 'VMUL (by scalar)',
  brief      => 'Vector Multiply (by scalar)',
  docvars    => 'instr-class=fpsimd mnemonic=VMUL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMULL_i => {
  title      => 'VMULL (integer and polynomial)',
  brief      => 'Vector Multiply Long (integer and polynomial)',
  docvars    => 'instr-class=fpsimd mnemonic=VMULL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMULL_s => {
  title      => 'VMULL (by scalar)',
  brief      => 'Vector Multiply Long (by scalar)',
  docvars    => 'instr-class=fpsimd mnemonic=VMULL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMVN_i => {
  title      => 'VMVN (immediate)',
  brief      => 'Vector Bitwise NOT (immediate)',
  docvars    => 'instr-class=fpsimd mnemonic=VMVN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VMVN_r => {
  title      => 'VMVN (register)',
  brief      => 'Vector Bitwise NOT (register)',
  docvars    => 'instr-class=fpsimd mnemonic=VMVN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VNEG => {
  title      => 'VNEG',
  brief      => 'Vector Negate',
  docvars    => 'instr-class=fpsimd mnemonic=VNEG',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VNMLA => {
  title      => 'VNMLA',
  brief      => 'Vector Negate Multiply Accumulate',
  docvars    => 'instr-class=fpsimd mnemonic=VNMLA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VNMLS => {
  title      => 'VNMLS',
  brief      => 'Vector Negate Multiply Subtract',
  docvars    => 'instr-class=fpsimd mnemonic=VNMLS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VNMUL => {
  title      => 'VNMUL',
  brief      => 'Vector Negate Multiply',
  docvars    => 'instr-class=fpsimd mnemonic=VNMUL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VORN_r => {
  title      => 'VORN (register)',
  brief      => 'Vector bitwise OR NOT (register)',
  docvars    => 'instr-class=fpsimd mnemonic=VORN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VORR_i => {
  title      => 'VORR (immediate)',
  brief      => 'Vector Bitwise OR (immediate)',
  docvars    => 'instr-class=fpsimd mnemonic=VORR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VORR_r => {
  title      => 'VORR (register)',
  brief      => 'Vector bitwise OR (register)',
  docvars    => 'instr-class=fpsimd mnemonic=VORR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VPADAL => {
  title      => 'VPADAL',
  brief      => 'Vector Pairwise Add and Accumulate Long',
  docvars    => 'instr-class=fpsimd mnemonic=VPADAL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VPADD_f => {
  title      => 'VPADD (floating-point)',
  brief      => 'Vector Pairwise Add (floating-point)',
  docvars    => 'instr-class=fpsimd mnemonic=VPADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VPADD_i => {
  title      => 'VPADD (integer)',
  brief      => 'Vector Pairwise Add (integer)',
  docvars    => 'instr-class=fpsimd mnemonic=VPADD simdvectorsize=double',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VPADDL => {
  title      => 'VPADDL',
  brief      => 'Vector Pairwise Add Long',
  docvars    => 'instr-class=fpsimd mnemonic=VPADDL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VPMAX_f => {
  title      => 'VPMAX (floating-point)',
  brief      => 'Vector Pairwise Maximum (floating-point)',
  docvars    => 'instr-class=fpsimd mnemonic=VPMAX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VPMAX_i => {
  title      => 'VPMAX (integer)',
  brief      => 'Vector Pairwise Maximum (integer)',
  docvars    => 'instr-class=fpsimd mnemonic=VPMAX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VPMIN_f => {
  title      => 'VPMIN (floating-point)',
  brief      => 'Vector Pairwise Minimum (floating-point)',
  docvars    => 'instr-class=fpsimd mnemonic=VPMIN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VPMIN_i => {
  title      => 'VPMIN (integer)',
  brief      => 'Vector Pairwise Minimum (integer)',
  docvars    => 'instr-class=fpsimd mnemonic=VPMIN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VQABS => {
  title      => 'VQABS',
  brief      => 'Vector Saturating Absolute',
  docvars    => 'instr-class=fpsimd mnemonic=VQABS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VQADD => {
  title      => 'VQADD',
  brief      => 'Vector Saturating Add',
  docvars    => 'instr-class=fpsimd mnemonic=VQADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VQDMLAL => {
  title      => 'VQDMLAL',
  brief      => 'Vector Saturating Doubling Multiply Accumulate Long',
  docvars    => 'instr-class=fpsimd mnemonic=VQDMLAL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VQDMLSL => {
  title      => 'VQDMLSL',
  brief      => 'Vector Saturating Doubling Multiply Subtract Long',
  docvars    => 'instr-class=fpsimd mnemonic=VQDMLSL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VQDMULH => {
  title      => 'VQDMULH',
  brief      => 'Vector Saturating Doubling Multiply Returning High Half',
  docvars    => 'instr-class=fpsimd mnemonic=VQDMULH',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VQDMULL => {
  title      => 'VQDMULL',
  brief      => 'Vector Saturating Doubling Multiply Long',
  docvars    => 'instr-class=fpsimd mnemonic=VQDMULL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VQMOVN => {
  title      => 'VQMOVN, VQMOVUN',
  brief      => 'Vector Saturating Move and Narrow',
  docvars    => 'instr-class=fpsimd',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VQNEG => {
  title      => 'VQNEG',
  brief      => 'Vector Saturating Negate',
  docvars    => 'instr-class=fpsimd mnemonic=VQNEG',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VQRDMLAH => {
  title      => 'VQRDMLAH',
  brief      => 'Vector Saturating Rounding Doubling Multiply Accumulate Returning High Half',
  docvars    => 'instr-class=fpsimd mnemonic=VQRDMLAH',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VQRDMLSH => {
  title      => 'VQRDMLSH',
  brief      => 'Vector Saturating Rounding Doubling Multiply Subtract Returning High Half',
  docvars    => 'instr-class=fpsimd mnemonic=VQRDMLSH',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VQRDMULH => {
  title      => 'VQRDMULH',
  brief      => 'Vector Saturating Rounding Doubling Multiply Returning High Half',
  docvars    => 'instr-class=fpsimd mnemonic=VQRDMULH',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VQRSHL => {
  title      => 'VQRSHL',
  brief      => 'Vector Saturating Rounding Shift Left',
  docvars    => 'instr-class=fpsimd mnemonic=VQRSHL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VQRSHRN => {
  title      => 'VQRSHRN, VQRSHRUN',
  brief      => 'Vector Saturating Rounding Shift Right, Narrow',
  docvars    => 'instr-class=fpsimd',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VQSHL_i => {
  title      => 'VQSHL, VQSHLU (immediate)',
  brief      => 'Vector Saturating Shift Left (immediate)',
  docvars    => 'instr-class=fpsimd',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VQSHL_r => {
  title      => 'VQSHL (register)',
  brief      => 'Vector Saturating Shift Left (register)',
  docvars    => 'instr-class=fpsimd mnemonic=VQSHL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VQSHRN => {
  title      => 'VQSHRN, VQSHRUN',
  brief      => 'Vector Saturating Shift Right, Narrow',
  docvars    => 'instr-class=fpsimd',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VQSUB => {
  title      => 'VQSUB',
  brief      => 'Vector Saturating Subtract',
  docvars    => 'instr-class=fpsimd mnemonic=VQSUB',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRADDHN => {
  title      => 'VRADDHN',
  brief      => 'Vector Rounding Add and Narrow, returning High Half',
  docvars    => 'instr-class=fpsimd mnemonic=VRADDHN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRECPE => {
  title      => 'VRECPE',
  brief      => 'Vector Reciprocal Estimate',
  docvars    => 'instr-class=fpsimd mnemonic=VRECPE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRECPS => {
  title      => 'VRECPS',
  brief      => 'Vector Reciprocal Step',
  docvars    => 'instr-class=fpsimd mnemonic=VRECPS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VREV16 => {
  title      => 'VREV16',
  brief      => 'Vector Reverse in halfwords',
  docvars    => 'instr-class=fpsimd mnemonic=VREV16',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VREV32 => {
  title      => 'VREV32',
  brief      => 'Vector Reverse in words',
  docvars    => 'instr-class=fpsimd mnemonic=VREV32',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VREV64 => {
  title      => 'VREV64',
  brief      => 'Vector Reverse in doublewords',
  docvars    => 'instr-class=fpsimd mnemonic=VREV64',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRHADD => {
  title      => 'VRHADD',
  brief      => 'Vector Rounding Halving Add',
  docvars    => 'instr-class=fpsimd mnemonic=VRHADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRINTA_asimd => {
  title      => 'VRINTA (Advanced SIMD)',
  brief      => 'Vector Round floating-point to integer towards Nearest with Ties to Away',
  docvars    => 'instr-class=fpsimd mnemonic=VRINTA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRINTA_vfp => {
  title      => 'VRINTA (floating-point)',
  brief      => 'Round floating-point to integer to Nearest with Ties to Away',
  docvars    => 'instr-class=fpsimd mnemonic=VRINTA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRINTM_asimd => {
  title      => 'VRINTM (Advanced SIMD)',
  brief      => 'Vector Round floating-point to integer towards -Infinity',
  docvars    => 'instr-class=fpsimd mnemonic=VRINTM',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRINTM_vfp => {
  title      => 'VRINTM (floating-point)',
  brief      => 'Round floating-point to integer towards -Infinity',
  docvars    => 'instr-class=fpsimd mnemonic=VRINTM',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRINTN_asimd => {
  title      => 'VRINTN (Advanced SIMD)',
  brief      => 'Vector Round floating-point to integer to Nearest',
  docvars    => 'instr-class=fpsimd mnemonic=VRINTN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRINTN_vfp => {
  title      => 'VRINTN (floating-point)',
  brief      => 'Round floating-point to integer to Nearest',
  docvars    => 'instr-class=fpsimd mnemonic=VRINTN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRINTP_asimd => {
  title      => 'VRINTP (Advanced SIMD)',
  brief      => 'Vector Round floating-point to integer towards +Infinity',
  docvars    => 'instr-class=fpsimd mnemonic=VRINTP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRINTP_vfp => {
  title      => 'VRINTP (floating-point)',
  brief      => 'Round floating-point to integer towards +Infinity',
  docvars    => 'instr-class=fpsimd mnemonic=VRINTP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRINTR_vfp => {
  title      => 'VRINTR',
  brief      => 'Round floating-point to integer',
  docvars    => 'instr-class=fpsimd mnemonic=VRINTR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRINTX_asimd => {
  title      => 'VRINTX (Advanced SIMD)',
  brief      => 'Vector round floating-point to integer inexact',
  docvars    => 'instr-class=fpsimd mnemonic=VRINTX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRINTX_vfp => {
  title      => 'VRINTX (floating-point)',
  brief      => 'Round floating-point to integer inexact',
  docvars    => 'instr-class=fpsimd mnemonic=VRINTX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRINTZ_asimd => {
  title      => 'VRINTZ (Advanced SIMD)',
  brief      => 'Vector round floating-point to integer towards Zero',
  docvars    => 'instr-class=fpsimd mnemonic=VRINTZ',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRINTZ_vfp => {
  title      => 'VRINTZ (floating-point)',
  brief      => 'Round floating-point to integer towards Zero',
  docvars    => 'instr-class=fpsimd mnemonic=VRINTZ',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRSHL => {
  title      => 'VRSHL',
  brief      => 'Vector Rounding Shift Left',
  docvars    => 'instr-class=fpsimd mnemonic=VRSHL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRSHR => {
  title      => 'VRSHR',
  brief      => 'Vector Rounding Shift Right',
  docvars    => 'instr-class=fpsimd mnemonic=VRSHR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRSHRN => {
  title      => 'VRSHRN',
  brief      => 'Vector Rounding Shift Right and Narrow',
  docvars    => 'instr-class=fpsimd mnemonic=VRSHRN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRSQRTE => {
  title      => 'VRSQRTE',
  brief      => 'Vector Reciprocal Square Root Estimate',
  docvars    => 'instr-class=fpsimd mnemonic=VRSQRTE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRSQRTS => {
  title      => 'VRSQRTS',
  brief      => 'Vector Reciprocal Square Root Step',
  docvars    => 'instr-class=fpsimd mnemonic=VRSQRTS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRSRA => {
  title      => 'VRSRA',
  brief      => 'Vector Rounding Shift Right and Accumulate',
  docvars    => 'instr-class=fpsimd mnemonic=VRSRA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VRSUBHN => {
  title      => 'VRSUBHN',
  brief      => 'Vector Rounding Subtract and Narrow, returning High Half',
  docvars    => 'instr-class=fpsimd mnemonic=VRSUBHN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSDOT => {
  title      => 'VSDOT (vector)',
  brief      => 'Dot Product vector form with signed integers.',
  docvars    => 'instr-class=fpsimd mnemonic=VSDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSDOT_s => {
  title      => 'VSDOT (by element)',
  brief      => 'Dot Product index form with signed integers.',
  docvars    => 'instr-class=fpsimd mnemonic=VSDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSEL => {
  title      => 'VSELEQ, VSELGE, VSELGT, VSELVS',
  brief      => 'Floating-point conditional select',
  docvars    => 'instr-class=fpsimd',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSHL_i => {
  title      => 'VSHL (immediate)',
  brief      => 'Vector Shift Left (immediate)',
  docvars    => 'instr-class=fpsimd mnemonic=VSHL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSHL_r => {
  title      => 'VSHL (register)',
  brief      => 'Vector Shift Left (register)',
  docvars    => 'instr-class=fpsimd mnemonic=VSHL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSHLL => {
  title      => 'VSHLL',
  brief      => 'Vector Shift Left Long',
  docvars    => 'instr-class=fpsimd mnemonic=VSHLL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSHR => {
  title      => 'VSHR',
  brief      => 'Vector Shift Right',
  docvars    => 'instr-class=fpsimd mnemonic=VSHR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSHRN => {
  title      => 'VSHRN',
  brief      => 'Vector Shift Right Narrow',
  docvars    => 'instr-class=fpsimd mnemonic=VSHRN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSLI => {
  title      => 'VSLI',
  brief      => 'Vector Shift Left and Insert',
  docvars    => 'instr-class=fpsimd mnemonic=VSLI',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSMMLA => {
  title      => 'VSMMLA',
  brief      => 'Widening 8-bit signed integer matrix multiply-accumulate into 2x2 matrix',
  docvars    => 'instr-class=fpsimd mnemonic=VSMMLA simdvectorsize=quad',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSQRT => {
  title      => 'VSQRT',
  brief      => 'Square Root',
  docvars    => 'instr-class=fpsimd mnemonic=VSQRT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSRA => {
  title      => 'VSRA',
  brief      => 'Vector Shift Right and Accumulate',
  docvars    => 'instr-class=fpsimd mnemonic=VSRA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSRI => {
  title      => 'VSRI',
  brief      => 'Vector Shift Right and Insert',
  docvars    => 'instr-class=fpsimd mnemonic=VSRI',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VST1_1 => {
  title      => 'VST1 (single element from one lane)',
  brief      => 'Store single element from one lane of one register',
  docvars    => 'instr-class=fpsimd mnemonic=VST1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VST1_m => {
  title      => 'VST1 (multiple single elements)',
  brief      => 'Store multiple single elements from one, two, three, or four registers',
  docvars    => 'instr-class=fpsimd mnemonic=VST1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VST2_1 => {
  title      => 'VST2 (single 2-element structure from one lane)',
  brief      => 'Store single 2-element structure from one lane of two registers',
  docvars    => 'instr-class=fpsimd mnemonic=VST2',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VST2_m => {
  title      => 'VST2 (multiple 2-element structures)',
  brief      => 'Store multiple 2-element structures from two or four registers',
  docvars    => 'instr-class=fpsimd mnemonic=VST2',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VST3_1 => {
  title      => 'VST3 (single 3-element structure from one lane)',
  brief      => 'Store single 3-element structure from one lane of three registers',
  docvars    => 'instr-class=fpsimd mnemonic=VST3',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VST3_m => {
  title      => 'VST3 (multiple 3-element structures)',
  brief      => 'Store multiple 3-element structures from three registers',
  docvars    => 'instr-class=fpsimd mnemonic=VST3',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VST4_1 => {
  title      => 'VST4 (single 4-element structure from one lane)',
  brief      => 'Store single 4-element structure from one lane of four registers',
  docvars    => 'instr-class=fpsimd mnemonic=VST4',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VST4_m => {
  title      => 'VST4 (multiple 4-element structures)',
  brief      => 'Store multiple 4-element structures from four registers',
  docvars    => 'instr-class=fpsimd mnemonic=VST4',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSTM => {
  title      => 'VSTM, VSTMDB, VSTMIA',
  brief      => 'Store multiple SIMD&FP registers',
  docvars    => 'instr-class=fpsimd',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSTR => {
  title      => 'VSTR',
  brief      => 'Store SIMD&FP register',
  docvars    => 'address-form=base-plus-offset instr-class=fpsimd mnemonic=VSTR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSUB_f => {
  title      => 'VSUB (floating-point)',
  brief      => 'Vector Subtract (floating-point)',
  docvars    => 'instr-class=fpsimd mnemonic=VSUB',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSUB_i => {
  title      => 'VSUB (integer)',
  brief      => 'Vector Subtract (integer)',
  docvars    => 'instr-class=fpsimd mnemonic=VSUB',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSUBHN => {
  title      => 'VSUBHN',
  brief      => 'Vector Subtract and Narrow, returning High Half',
  docvars    => 'instr-class=fpsimd mnemonic=VSUBHN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSUBL => {
  title      => 'VSUBL',
  brief      => 'Vector Subtract Long',
  docvars    => 'instr-class=fpsimd mnemonic=VSUBL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSUBW => {
  title      => 'VSUBW',
  brief      => 'Vector Subtract Wide',
  docvars    => 'instr-class=fpsimd mnemonic=VSUBW',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSUDOT_s => {
  title      => 'VSUDOT (by element)',
  brief      => 'Dot Product index form with signed and unsigned integers (by element)',
  docvars    => 'instr-class=fpsimd mnemonic=VSUDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VSWP => {
  title      => 'VSWP',
  brief      => 'Vector Swap',
  docvars    => 'instr-class=fpsimd mnemonic=VSWP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VTBL => {
  title      => 'VTBL, VTBX',
  brief      => 'Vector Table Lookup and Extension',
  docvars    => 'instr-class=fpsimd',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VTRN => {
  title      => 'VTRN',
  brief      => 'Vector Transpose',
  docvars    => 'instr-class=fpsimd mnemonic=VTRN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VTST => {
  title      => 'VTST',
  brief      => 'Vector Test Bits',
  docvars    => 'instr-class=fpsimd mnemonic=VTST',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VUDOT => {
  title      => 'VUDOT (vector)',
  brief      => 'Dot Product vector form with unsigned integers.',
  docvars    => 'instr-class=fpsimd mnemonic=VUDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VUDOT_s => {
  title      => 'VUDOT (by element)',
  brief      => 'Dot Product index form with unsigned integers.',
  docvars    => 'instr-class=fpsimd mnemonic=VUDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VUMMLA => {
  title      => 'VUMMLA',
  brief      => 'Widening 8-bit unsigned integer matrix multiply-accumulate into 2x2 matrix',
  docvars    => 'instr-class=fpsimd mnemonic=VUMMLA simdvectorsize=quad',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VUSDOT => {
  title      => 'VUSDOT (vector)',
  brief      => 'Dot Product vector form with mixed-sign integers',
  docvars    => 'instr-class=fpsimd mnemonic=VUSDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VUSDOT_s => {
  title      => 'VUSDOT (by element)',
  brief      => 'Dot Product index form with unsigned and signed integers (by element)',
  docvars    => 'instr-class=fpsimd mnemonic=VUSDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VUSMMLA => {
  title      => 'VUSMMLA',
  brief      => 'Widening 8-bit mixed integer matrix multiply-accumulate into 2x2 matrix',
  docvars    => 'instr-class=fpsimd mnemonic=VUSMMLA simdvectorsize=quad',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VUZP => {
  title      => 'VUZP',
  brief      => 'Vector Unzip',
  docvars    => 'instr-class=fpsimd mnemonic=VUZP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VZIP => {
  title      => 'VZIP',
  brief      => 'Vector Zip',
  docvars    => 'instr-class=fpsimd mnemonic=VZIP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE VACLE_VACGE => {
  title      => 'VACLE',
  brief      => 'Vector Absolute Compare Less Than or Equal',
  docvars    => 'alias_mnemonic=VACLE instr-class=fpsimd mnemonic=VACGE',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VACLT_VACGT => {
  title      => 'VACLT',
  brief      => 'Vector Absolute Compare Less Than',
  docvars    => 'alias_mnemonic=VACLT instr-class=fpsimd mnemonic=VACGT',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VAND_VBIC_i => {
  title      => 'VAND (immediate)',
  brief      => 'Vector Bitwise AND (immediate)',
  docvars    => 'alias_mnemonic=VAND instr-class=fpsimd mnemonic=VBIC',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VCLE_VCGE_r => {
  title      => 'VCLE (register)',
  brief      => 'Vector Compare Less Than or Equal',
  docvars    => 'alias_mnemonic=VCLE instr-class=fpsimd mnemonic=VCGE',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VCLT_VCGT_r => {
  title      => 'VCLT (register)',
  brief      => 'Vector Compare Less Than',
  docvars    => 'alias_mnemonic=VCLT instr-class=fpsimd mnemonic=VCGT',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VEXT_VEXT => {
  title      => 'VEXT (multibyte elements)',
  brief      => 'Vector Extract',
  docvars    => 'alias_mnemonic=VEXT instr-class=fpsimd mnemonic=VEXT',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VMOV_VORR_r => {
  title      => 'VMOV (register, SIMD)',
  brief      => 'Copy between SIMD registers',
  docvars    => 'alias_mnemonic=VMOV instr-class=fpsimd mnemonic=VORR',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VORN_VORR_i => {
  title      => 'VORN (immediate)',
  brief      => 'Vector Bitwise OR NOT (immediate)',
  docvars    => 'alias_mnemonic=VORN instr-class=fpsimd mnemonic=VORR',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VPOP_VLDM => {
  title      => 'VPOP',
  brief      => 'Pop SIMD&FP registers from Stack',
  docvars    => 'alias_mnemonic=VPOP instr-class=fpsimd ldmstm-mode=inc-after mnemonic=VLDM',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VPUSH_VSTM => {
  title      => 'VPUSH',
  brief      => 'Push SIMD&FP registers to Stack',
  docvars    => 'alias_mnemonic=VPUSH instr-class=fpsimd ldmstm-mode=dec-before mnemonic=VSTMDB',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VQRSHRN_VQMOVN => {
  title      => 'VQRSHRN (zero)',
  brief      => 'Vector Saturating Rounding Shift Right, Narrow',
  docvars    => 'alias_mnemonic=VQRSHRN instr-class=fpsimd mnemonic=VQMOVN result-type=signed-result',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VQRSHRUN_VQMOVN => {
  title      => 'VQRSHRUN (zero)',
  brief      => 'Vector Saturating Rounding Shift Right, Narrow',
  docvars    => 'alias_mnemonic=VQRSHRUN instr-class=fpsimd mnemonic=VQMOVUN result-type=unsigned-result',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VQSHRN_VQMOVN => {
  title      => 'VQSHRN (zero)',
  brief      => 'Vector Saturating Shift Right, Narrow',
  docvars    => 'alias_mnemonic=VQSHRN instr-class=fpsimd mnemonic=VQMOVN result-type=signed-result',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VQSHRUN_VQMOVN => {
  title      => 'VQSHRUN (zero)',
  brief      => 'Vector Saturating Shift Right, Narrow',
  docvars    => 'alias_mnemonic=VQSHRUN instr-class=fpsimd mnemonic=VQMOVUN result-type=unsigned-result',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VRSHR_VORR_r => {
  title      => 'VRSHR (zero)',
  brief      => 'Vector Rounding Shift Right',
  docvars    => 'alias_mnemonic=VRSHR instr-class=fpsimd mnemonic=VORR',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VRSHRN_VMOVN => {
  title      => 'VRSHRN (zero)',
  brief      => 'Vector Rounding Shift Right and Narrow',
  docvars    => 'alias_mnemonic=VRSHRN instr-class=fpsimd mnemonic=VMOVN',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VSHR_VORR_r => {
  title      => 'VSHR (zero)',
  brief      => 'Vector Shift Right',
  docvars    => 'alias_mnemonic=VSHR instr-class=fpsimd mnemonic=VORR',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VSHRN_VMOVN => {
  title      => 'VSHRN (zero)',
  brief      => 'Vector Shift Right Narrow',
  docvars    => 'alias_mnemonic=VSHRN instr-class=fpsimd mnemonic=VMOVN',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VUZP_VTRN => {
  title      => 'VUZP (alias)',
  brief      => 'Vector Unzip',
  docvars    => 'alias_mnemonic=VUZP instr-class=fpsimd mnemonic=VTRN simdvectorsize=double',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE VZIP_VTRN => {
  title      => 'VZIP (alias)',
  brief      => 'Vector Zip',
  docvars    => 'alias_mnemonic=VZIP instr-class=fpsimd mnemonic=VTRN simdvectorsize=double',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};


1;

