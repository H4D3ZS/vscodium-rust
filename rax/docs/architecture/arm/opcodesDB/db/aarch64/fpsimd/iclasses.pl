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

ICLASS asimdall => {
  title      => 'Advanced SIMD across lanes',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0bx ig1=0b01110 size:u=0bxx ig2=0b11000 opcode:u=0bxxxxx ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asimddiff => {
  title      => 'Advanced SIMD three different',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0bx ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode:u=0bxxxx ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asimdelem => {
  title      => 'Advanced SIMD vector x indexed element',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0bx ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode:u=0bxxxx H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asimdext => {
  title      => 'Advanced SIMD extract',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b101110 op2:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 imm4:u=0bxxxx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asimdimm => {
  title      => 'Advanced SIMD modified immediate',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0bx ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0bxxxx o2:u=0bx ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asimdins => {
  title      => 'Advanced SIMD copy',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0bx ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4:u=0bxxxx ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asimdmisc => {
  title      => 'Advanced SIMD two-register miscellaneous',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0bx ig1=0b01110 size:u=0bxx ig2=0b10000 opcode:u=0bxxxxx ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asimdmiscfp16 => {
  title      => 'Advanced SIMD two-register miscellaneous (FP16)',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0bx ig1=0b01110 a:u=0bx ig2=0b111100 opcode:u=0bxxxxx ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asimdperm => {
  title      => 'Advanced SIMD permute',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 opcode:u=0bxxx ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asimdsame => {
  title      => 'Advanced SIMD three same',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0bx ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode:u=0bxxxxx ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asimdsame2 => {
  title      => 'Advanced SIMD three-register extension',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0bx ig1=0b01110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 opcode:u=0bxxxx ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asimdsamefp16 => {
  title      => 'Advanced SIMD three same (FP16)',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0bx ig1=0b01110 a:u=0bx ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode:u=0bxxx ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asimdshf => {
  title      => 'Advanced SIMD shift by immediate',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0bx ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode:u=0bxxxxx ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asimdtbl => {
  title      => 'Advanced SIMD table lookup',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 op2:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 len:u=0bxx op:u=0bx ig4=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asisddiff => {
  title      => 'Advanced SIMD scalar three different',
  diagram    => 'ig0=0b01 U:u=0bx ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode:u=0bxxxx ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asisdelem => {
  title      => 'Advanced SIMD scalar x indexed element',
  diagram    => 'ig0=0b01 U:u=0bx ig1=0b11111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode:u=0bxxxx H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asisdlse => {
  title      => 'Advanced SIMD load/store multiple structures',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0bx ig2=0b000000 opcode:u=0bxxxx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst',
};

ICLASS asisdlsep => {
  title      => 'Advanced SIMD load/store multiple structures (post-indexed)',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0bx ig2=0b0 Rm:u=0bxxxxx opcode:u=0bxxxx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst',
};

ICLASS asisdlso => {
  title      => 'Advanced SIMD load/store single structure',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0bx R:u=0bx ig2=0b00000 opcode:u=0bxxx S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst',
};

ICLASS asisdlsop => {
  title      => 'Advanced SIMD load/store single structure (post-indexed)',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0bx R:u=0bx Rm:u=0bxxxxx opcode:u=0bxxx S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst',
};

ICLASS asisdmisc => {
  title      => 'Advanced SIMD scalar two-register miscellaneous',
  diagram    => 'ig0=0b01 U:u=0bx ig1=0b11110 size:u=0bxx ig2=0b10000 opcode:u=0bxxxxx ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asisdmiscfp16 => {
  title      => 'Advanced SIMD scalar two-register miscellaneous FP16',
  diagram    => 'ig0=0b01 U:u=0bx ig1=0b11110 a:u=0bx ig2=0b111100 opcode:u=0bxxxxx ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asisdone => {
  title      => 'Advanced SIMD scalar copy',
  diagram    => 'ig0=0b01 op:u=0bx ig1=0b11110000 imm5:u=0bxxxxx ig2=0b0 imm4:u=0bxxxx ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asisdpair => {
  title      => 'Advanced SIMD scalar pairwise',
  diagram    => 'ig0=0b01 U:u=0bx ig1=0b11110 size:u=0bxx ig2=0b11000 opcode:u=0bxxxxx ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asisdsame => {
  title      => 'Advanced SIMD scalar three same',
  diagram    => 'ig0=0b01 U:u=0bx ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode:u=0bxxxxx ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asisdsame2 => {
  title      => 'Advanced SIMD scalar three same extra',
  diagram    => 'ig0=0b01 U:u=0bx ig1=0b11110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 opcode:u=0bxxxx ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asisdsamefp16 => {
  title      => 'Advanced SIMD scalar three same FP16',
  diagram    => 'ig0=0b01 U:u=0bx ig1=0b11110 a:u=0bx ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode:u=0bxxx ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS asisdshf => {
  title      => 'Advanced SIMD scalar shift by immediate',
  diagram    => 'ig0=0b01 U:u=0bx ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx opcode:u=0bxxxxx ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS crypto3_imm2 => {
  title      => 'Cryptographic three-register, imm2',
  diagram    => 'ig0=0b11001110010 Rm:u=0bxxxxx ig1=0b10 imm2:u=0bxx opcode:u=0bxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS crypto3_imm6 => {
  title      => 'Cryptographic three-register, imm6',
  diagram    => 'ig0=0b11001110100 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS crypto4 => {
  title      => 'Cryptographic four-register',
  diagram    => 'ig0=0b110011100 Op0:u=0bxx Rm:u=0bxxxxx ig1=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS cryptoaes => {
  title      => 'Cryptographic AES',
  diagram    => 'ig0=0b01001110 size:u=0bxx ig1=0b10100 opcode:u=0bxxxxx ig2=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS cryptosha2 => {
  title      => 'Cryptographic two-register SHA',
  diagram    => 'ig0=0b01011110 size:u=0bxx ig1=0b10100 opcode:u=0bxxxxx ig2=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS cryptosha3 => {
  title      => 'Cryptographic three-register SHA',
  diagram    => 'ig0=0b01011110 size:u=0bxx ig1=0b0 Rm:u=0bxxxxx ig2=0b0 opcode:u=0bxxx ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS cryptosha512_2 => {
  title      => 'Cryptographic two-register SHA 512',
  diagram    => 'ig0=0b11001110110000001000 opcode:u=0bxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS cryptosha512_3 => {
  title      => 'Cryptographic three-register SHA 512',
  diagram    => 'ig0=0b11001110011 Rm:u=0bxxxxx ig1=0b1 O:u=0bx ig2=0b00 opcode:u=0bxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS float2fix => {
  title      => 'Conversion between floating-point and fixed-point',
  diagram    => 'sf:u=0bx ig0=0b0 S:u=0bx ig1=0b11110 ptype:u=0bxx ig2=0b0 rmode:u=0bxx opcode:u=0bxxx scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS float2int => {
  title      => 'Conversion between floating-point and integer',
  diagram    => 'sf:u=0bx ig0=0b0 S:u=0bx ig1=0b11110 ptype:u=0bxx ig2=0b1 rmode:u=0bxx opcode:u=0bxxx ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS floatccmp => {
  title      => 'Floating-point conditional compare',
  diagram    => 'M:u=0bx ig0=0b0 S:u=0bx ig1=0b11110 ptype:u=0bxx ig2=0b1 Rm:u=0bxxxxx cond:u=0bxxxx ig3=0b01 Rn:u=0bxxxxx op:u=0bx nzcv:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS floatcmp => {
  title      => 'Floating-point compare',
  diagram    => 'M:u=0bx ig0=0b0 S:u=0bx ig1=0b11110 ptype:u=0bxx ig2=0b1 Rm:u=0bxxxxx op:u=0bxx ig3=0b1000 Rn:u=0bxxxxx opcode2:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS floatdp1 => {
  title      => 'Floating-point data-processing (1 source)',
  diagram    => 'M:u=0bx ig0=0b0 S:u=0bx ig1=0b11110 ptype:u=0bxx ig2=0b1 opcode:u=0bxxxxxx ig3=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS floatdp2 => {
  title      => 'Floating-point data-processing (2 source)',
  diagram    => 'M:u=0bx ig0=0b0 S:u=0bx ig1=0b11110 ptype:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode:u=0bxxxx ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS floatdp3 => {
  title      => 'Floating-point data-processing (3 source)',
  diagram    => 'M:u=0bx ig0=0b0 S:u=0bx ig1=0b11111 ptype:u=0bxx o1:u=0bx Rm:u=0bxxxxx o0:u=0bx Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS floatimm => {
  title      => 'Floating-point immediate',
  diagram    => 'M:u=0bx ig0=0b0 S:u=0bx ig1=0b11110 ptype:u=0bxx ig2=0b1 imm8:u=0bxxxxxxxx ig3=0b100 imm5:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS floatsel => {
  title      => 'Floating-point conditional select',
  diagram    => 'M:u=0bx ig0=0b0 S:u=0bx ig1=0b11110 ptype:u=0bxx ig2=0b1 Rm:u=0bxxxxx cond:u=0bxxxx ig3=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp',
};

ICLASS ldst_immpost => {
  title      => 'Load/store register (immediate post-indexed)',
  diagram    => 'size:u=0bxx ig0=0b111 V:u=0bx ig1=0b00 opc:u=0bxx ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst',
};

ICLASS ldst_immpre => {
  title      => 'Load/store register (immediate pre-indexed)',
  diagram    => 'size:u=0bxx ig0=0b111 V:u=0bx ig1=0b00 opc:u=0bxx ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst',
};

ICLASS ldst_pos => {
  title      => 'Load/store register (unsigned immediate)',
  diagram    => 'size:u=0bxx ig0=0b111 V:u=0bx ig1=0b01 opc:u=0bxx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst',
};

ICLASS ldst_regoff => {
  title      => 'Load/store register (register offset)',
  diagram    => 'size:u=0bxx ig0=0b111 V:u=0bx ig1=0b00 opc:u=0bxx ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst',
};

ICLASS ldst_unscaled => {
  title      => 'Load/store register (unscaled immediate)',
  diagram    => 'size:u=0bxx ig0=0b111 V:u=0bx ig1=0b00 opc:u=0bxx ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst',
};

ICLASS ldstnapair_offs => {
  title      => 'Load/store no-allocate pair (offset)',
  diagram    => 'opc:u=0bxx ig0=0b101 V:u=0bx ig1=0b000 L:u=0bx imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst',
};

ICLASS ldstpair_off => {
  title      => 'Load/store register pair (offset)',
  diagram    => 'opc:u=0bxx ig0=0b101 V:u=0bx ig1=0b010 L:u=0bx imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst',
};

ICLASS ldstpair_post => {
  title      => 'Load/store register pair (post-indexed)',
  diagram    => 'opc:u=0bxx ig0=0b101 V:u=0bx ig1=0b001 L:u=0bx imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst',
};

ICLASS ldstpair_pre => {
  title      => 'Load/store register pair (pre-indexed)',
  diagram    => 'opc:u=0bxx ig0=0b101 V:u=0bx ig1=0b011 L:u=0bx imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst',
};

ICLASS loadlit => {
  title      => 'Load register (literal)',
  diagram    => 'opc:u=0bxx ig0=0b011 V:u=0bx ig1=0b00 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst',
};


1;

