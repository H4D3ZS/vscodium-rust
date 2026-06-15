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


# asimdall: Advanced SIMD across lanes.
# A64: ig0=0b0 Q:u=0bx U:u=0bx ig1=0b01110 size:u=0bxx ig2=0b11000 opcode:u=0bxxxxx ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asimdall', 'A64');

DEF Bd => SIMD8('Bd', 'Rd', 'write')  ;
DEF Dd => SIMD64('Dd', 'Rd', 'write') ;
DEF Hd => SIMD16('Hd', 'Rd', 'write') ;
DEF Sd => SIMD32('Sd', 'Rd', 'write') ;
DEF Vn => VECREG('Vn', 'Rn', 'read')  ;

T[' ADDV    REG:i8=$Bd  VREG={REG:i8=$Vn   VDT=8B } ', 'Q=0b0 size=0b00 ', 'ADDV_asimdall_only       ', '                                     '];
T[' ADDV    REG:i8=$Bd  VREG={REG:i8=$Vn   VDT=16B} ', 'Q=0b1 size=0b00 ', 'ADDV_asimdall_only       ', '                                     '];
T[' ADDV    REG:i16=$Hd VREG={REG:i16=$Vn VDT=4H }  ', 'Q=0b0 size=0b01 ', 'ADDV_asimdall_only       ', '                                     '];
T[' ADDV    REG:i16=$Hd VREG={REG:i16=$Vn VDT=8H }  ', 'Q=0b1 size=0b01 ', 'ADDV_asimdall_only       ', '                                     '];
T[' ADDV    REG:i32=$Sd VREG={REG:i32=$Vn VDT=4S }  ', 'Q=0b1 size=0b10 ', 'ADDV_asimdall_only       ', '                                     '];
T[' FMAXNMV REG:f16=$Hd VREG={REG:f16=$Vn VDT=4H }  ', 'Q=0b0           ', 'FMAXNMV_asimdall_only_H  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMAXNMV REG:f16=$Hd VREG={REG:f16=$Vn VDT=8H }  ', 'Q=0b1           ', 'FMAXNMV_asimdall_only_H  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMAXNMV REG:f32=$Sd VREG={REG:f32=$Vn VDT=4S }  ', 'Q=0b1 sz=0b0    ', 'FMAXNMV_asimdall_only_SD ', '                                     '];
T[' FMAXV   REG:f16=$Hd VREG={REG:f16=$Vn VDT=4H }  ', 'Q=0b0           ', 'FMAXV_asimdall_only_H    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMAXV   REG:f16=$Hd VREG={REG:f16=$Vn VDT=8H }  ', 'Q=0b1           ', 'FMAXV_asimdall_only_H    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMAXV   REG:f32=$Sd VREG={REG:f32=$Vn VDT=4S }  ', 'Q=0b1 sz=0b0    ', 'FMAXV_asimdall_only_SD   ', '                                     '];
T[' FMINNMV REG:f16=$Hd VREG={REG:f16=$Vn VDT=4H }  ', 'Q=0b0           ', 'FMINNMV_asimdall_only_H  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMINNMV REG:f16=$Hd VREG={REG:f16=$Vn VDT=8H }  ', 'Q=0b1           ', 'FMINNMV_asimdall_only_H  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMINNMV REG:f32=$Sd VREG={REG:f32=$Vn VDT=4S }  ', 'Q=0b1 sz=0b0    ', 'FMINNMV_asimdall_only_SD ', '                                     '];
T[' FMINV   REG:f16=$Hd VREG={REG:f16=$Vn VDT=4H }  ', 'Q=0b0           ', 'FMINV_asimdall_only_H    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMINV   REG:f16=$Hd VREG={REG:f16=$Vn VDT=8H }  ', 'Q=0b1           ', 'FMINV_asimdall_only_H    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMINV   REG:f32=$Sd VREG={REG:f32=$Vn VDT=4S }  ', 'Q=0b1 sz=0b0    ', 'FMINV_asimdall_only_SD   ', '                                     '];
T[' SADDLV  REG:s16=$Hd VREG={REG:s8=$Vn   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SADDLV_asimdall_only     ', '                                     '];
T[' SADDLV  REG:s16=$Hd VREG={REG:s8=$Vn   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SADDLV_asimdall_only     ', '                                     '];
T[' SADDLV  REG:s32=$Sd VREG={REG:s16=$Vn VDT=4H }  ', 'Q=0b0 size=0b01 ', 'SADDLV_asimdall_only     ', '                                     '];
T[' SADDLV  REG:s32=$Sd VREG={REG:s16=$Vn VDT=8H }  ', 'Q=0b1 size=0b01 ', 'SADDLV_asimdall_only     ', '                                     '];
T[' SADDLV  REG:s64=$Dd VREG={REG:s32=$Vn VDT=4S }  ', 'Q=0b1 size=0b10 ', 'SADDLV_asimdall_only     ', '                                     '];
T[' SMAXV   REG:s8=$Bd  VREG={REG:s8=$Vn   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SMAXV_asimdall_only      ', '                                     '];
T[' SMAXV   REG:s8=$Bd  VREG={REG:s8=$Vn   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SMAXV_asimdall_only      ', '                                     '];
T[' SMAXV   REG:s16=$Hd VREG={REG:s16=$Vn VDT=4H }  ', 'Q=0b0 size=0b01 ', 'SMAXV_asimdall_only      ', '                                     '];
T[' SMAXV   REG:s16=$Hd VREG={REG:s16=$Vn VDT=8H }  ', 'Q=0b1 size=0b01 ', 'SMAXV_asimdall_only      ', '                                     '];
T[' SMAXV   REG:s32=$Sd VREG={REG:s32=$Vn VDT=4S }  ', 'Q=0b1 size=0b10 ', 'SMAXV_asimdall_only      ', '                                     '];
T[' SMINV   REG:s8=$Bd  VREG={REG:s8=$Vn   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SMINV_asimdall_only      ', '                                     '];
T[' SMINV   REG:s8=$Bd  VREG={REG:s8=$Vn   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SMINV_asimdall_only      ', '                                     '];
T[' SMINV   REG:s16=$Hd VREG={REG:s16=$Vn VDT=4H }  ', 'Q=0b0 size=0b01 ', 'SMINV_asimdall_only      ', '                                     '];
T[' SMINV   REG:s16=$Hd VREG={REG:s16=$Vn VDT=8H }  ', 'Q=0b1 size=0b01 ', 'SMINV_asimdall_only      ', '                                     '];
T[' SMINV   REG:s32=$Sd VREG={REG:s32=$Vn VDT=4S }  ', 'Q=0b1 size=0b10 ', 'SMINV_asimdall_only      ', '                                     '];
T[' UADDLV  REG:u16=$Hd VREG={REG:u8=$Vn   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UADDLV_asimdall_only     ', '                                     '];
T[' UADDLV  REG:u16=$Hd VREG={REG:u8=$Vn   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UADDLV_asimdall_only     ', '                                     '];
T[' UADDLV  REG:u32=$Sd VREG={REG:u16=$Vn VDT=4H }  ', 'Q=0b0 size=0b01 ', 'UADDLV_asimdall_only     ', '                                     '];
T[' UADDLV  REG:u32=$Sd VREG={REG:u16=$Vn VDT=8H }  ', 'Q=0b1 size=0b01 ', 'UADDLV_asimdall_only     ', '                                     '];
T[' UADDLV  REG:u64=$Dd VREG={REG:u32=$Vn VDT=4S }  ', 'Q=0b1 size=0b10 ', 'UADDLV_asimdall_only     ', '                                     '];
T[' UMAXV   REG:u8=$Bd  VREG={REG:u8=$Vn   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UMAXV_asimdall_only      ', '                                     '];
T[' UMAXV   REG:u8=$Bd  VREG={REG:u8=$Vn   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UMAXV_asimdall_only      ', '                                     '];
T[' UMAXV   REG:u16=$Hd VREG={REG:u16=$Vn VDT=4H }  ', 'Q=0b0 size=0b01 ', 'UMAXV_asimdall_only      ', '                                     '];
T[' UMAXV   REG:u16=$Hd VREG={REG:u16=$Vn VDT=8H }  ', 'Q=0b1 size=0b01 ', 'UMAXV_asimdall_only      ', '                                     '];
T[' UMAXV   REG:u32=$Sd VREG={REG:u32=$Vn VDT=4S }  ', 'Q=0b1 size=0b10 ', 'UMAXV_asimdall_only      ', '                                     '];
T[' UMINV   REG:u8=$Bd  VREG={REG:u8=$Vn   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UMINV_asimdall_only      ', '                                     '];
T[' UMINV   REG:u8=$Bd  VREG={REG:u8=$Vn   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UMINV_asimdall_only      ', '                                     '];
T[' UMINV   REG:u16=$Hd VREG={REG:u16=$Vn VDT=4H }  ', 'Q=0b0 size=0b01 ', 'UMINV_asimdall_only      ', '                                     '];
T[' UMINV   REG:u16=$Hd VREG={REG:u16=$Vn VDT=8H }  ', 'Q=0b1 size=0b01 ', 'UMINV_asimdall_only      ', '                                     '];
T[' UMINV   REG:u32=$Sd VREG={REG:u32=$Vn VDT=4S }  ', 'Q=0b1 size=0b10 ', 'UMINV_asimdall_only      ', '                                     '];



# asimddiff: Advanced SIMD three different.
# A64: ig0=0b0 Q:u=0bx U:u=0bx ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode:u=0bxxxx ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asimddiff', 'A64');

DEF Vd => VECREG('Vd', 'Rd', 'write') ;
DEF Vm => VECREG('Vm', 'Rm', 'read')  ;
DEF Vn => VECREG('Vn', 'Rn', 'read')  ;

T[' ADDHN    VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }  ', 'Q=0b0 size=0b00 ', 'ADDHN_asimddiff_N   ', '                       '];
T[' ADDHN    VREG={REG:i16=$Vd VDT=4H } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }   ', 'Q=0b0 size=0b01 ', 'ADDHN_asimddiff_N   ', '                       '];
T[' ADDHN    VREG={REG:i32=$Vd VDT=2S } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }   ', 'Q=0b0 size=0b10 ', 'ADDHN_asimddiff_N   ', '                       '];
T[' ADDHN2   VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }  ', 'Q=0b1 size=0b00 ', 'ADDHN_asimddiff_N   ', '                       '];
T[' ADDHN2   VREG={REG:i16=$Vd VDT=8H } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }   ', 'Q=0b1 size=0b01 ', 'ADDHN_asimddiff_N   ', '                       '];
T[' ADDHN2   VREG={REG:i32=$Vd VDT=4S } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }   ', 'Q=0b1 size=0b10 ', 'ADDHN_asimddiff_N   ', '                       '];
T[' PMULL    VREG={REG:i16=$Vd VDT=8H } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'PMULL_asimddiff_L   ', 'extensions=BIT128PMULL '];
T[' PMULL    VREG={REG:i128=$Vd VDT=1Q } VREG={REG:i64=$Vn VDT=1D } VREG={REG:i64=$Vm VDT=1D }  ', 'Q=0b0 size=0b11 ', 'PMULL_asimddiff_L   ', 'extensions=BIT128PMULL '];
T[' PMULL2   VREG={REG:i16=$Vd VDT=8H } VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'PMULL_asimddiff_L   ', 'extensions=BIT128PMULL '];
T[' PMULL2   VREG={REG:i128=$Vd VDT=1Q } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }  ', 'Q=0b1 size=0b11 ', 'PMULL_asimddiff_L   ', 'extensions=BIT128PMULL '];
T[' RADDHN   VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }  ', 'Q=0b0 size=0b00 ', 'RADDHN_asimddiff_N  ', '                       '];
T[' RADDHN   VREG={REG:i16=$Vd VDT=4H } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }   ', 'Q=0b0 size=0b01 ', 'RADDHN_asimddiff_N  ', '                       '];
T[' RADDHN   VREG={REG:i32=$Vd VDT=2S } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }   ', 'Q=0b0 size=0b10 ', 'RADDHN_asimddiff_N  ', '                       '];
T[' RADDHN2  VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }  ', 'Q=0b1 size=0b00 ', 'RADDHN_asimddiff_N  ', '                       '];
T[' RADDHN2  VREG={REG:i16=$Vd VDT=8H } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }   ', 'Q=0b1 size=0b01 ', 'RADDHN_asimddiff_N  ', '                       '];
T[' RADDHN2  VREG={REG:i32=$Vd VDT=4S } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }   ', 'Q=0b1 size=0b10 ', 'RADDHN_asimddiff_N  ', '                       '];
T[' RSUBHN   VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }  ', 'Q=0b0 size=0b00 ', 'RSUBHN_asimddiff_N  ', '                       '];
T[' RSUBHN   VREG={REG:i16=$Vd VDT=4H } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }   ', 'Q=0b0 size=0b01 ', 'RSUBHN_asimddiff_N  ', '                       '];
T[' RSUBHN   VREG={REG:i32=$Vd VDT=2S } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }   ', 'Q=0b0 size=0b10 ', 'RSUBHN_asimddiff_N  ', '                       '];
T[' RSUBHN2  VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }  ', 'Q=0b1 size=0b00 ', 'RSUBHN_asimddiff_N  ', '                       '];
T[' RSUBHN2  VREG={REG:i16=$Vd VDT=8H } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }   ', 'Q=0b1 size=0b01 ', 'RSUBHN_asimddiff_N  ', '                       '];
T[' RSUBHN2  VREG={REG:i32=$Vd VDT=4S } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }   ', 'Q=0b1 size=0b10 ', 'RSUBHN_asimddiff_N  ', '                       '];
T[' SABAL    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SABAL_asimddiff_L   ', '                       '];
T[' SABAL    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'SABAL_asimddiff_L   ', '                       '];
T[' SABAL    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'SABAL_asimddiff_L   ', '                       '];
T[' SABAL2   VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SABAL_asimddiff_L   ', '                       '];
T[' SABAL2   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'SABAL_asimddiff_L   ', '                       '];
T[' SABAL2   VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'SABAL_asimddiff_L   ', '                       '];
T[' SABDL    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SABDL_asimddiff_L   ', '                       '];
T[' SABDL    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'SABDL_asimddiff_L   ', '                       '];
T[' SABDL    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'SABDL_asimddiff_L   ', '                       '];
T[' SABDL2   VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SABDL_asimddiff_L   ', '                       '];
T[' SABDL2   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'SABDL_asimddiff_L   ', '                       '];
T[' SABDL2   VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'SABDL_asimddiff_L   ', '                       '];
T[' SADDL    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SADDL_asimddiff_L   ', '                       '];
T[' SADDL    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'SADDL_asimddiff_L   ', '                       '];
T[' SADDL    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'SADDL_asimddiff_L   ', '                       '];
T[' SADDL2   VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SADDL_asimddiff_L   ', '                       '];
T[' SADDL2   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'SADDL_asimddiff_L   ', '                       '];
T[' SADDL2   VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'SADDL_asimddiff_L   ', '                       '];
T[' SADDW    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s8=$Vm   VDT=8B }  ', 'Q=0b0 size=0b00 ', 'SADDW_asimddiff_W   ', '                       '];
T[' SADDW    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'SADDW_asimddiff_W   ', '                       '];
T[' SADDW    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D } VREG={REG:s32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'SADDW_asimddiff_W   ', '                       '];
T[' SADDW2   VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s8=$Vm   VDT=16B}  ', 'Q=0b1 size=0b00 ', 'SADDW_asimddiff_W   ', '                       '];
T[' SADDW2   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'SADDW_asimddiff_W   ', '                       '];
T[' SADDW2   VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D } VREG={REG:s32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'SADDW_asimddiff_W   ', '                       '];
T[' SMLAL    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SMLAL_asimddiff_L   ', '                       '];
T[' SMLAL    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'SMLAL_asimddiff_L   ', '                       '];
T[' SMLAL    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'SMLAL_asimddiff_L   ', '                       '];
T[' SMLAL2   VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SMLAL_asimddiff_L   ', '                       '];
T[' SMLAL2   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'SMLAL_asimddiff_L   ', '                       '];
T[' SMLAL2   VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'SMLAL_asimddiff_L   ', '                       '];
T[' SMLSL    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SMLSL_asimddiff_L   ', '                       '];
T[' SMLSL    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'SMLSL_asimddiff_L   ', '                       '];
T[' SMLSL    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'SMLSL_asimddiff_L   ', '                       '];
T[' SMLSL2   VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SMLSL_asimddiff_L   ', '                       '];
T[' SMLSL2   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'SMLSL_asimddiff_L   ', '                       '];
T[' SMLSL2   VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'SMLSL_asimddiff_L   ', '                       '];
T[' SMULL    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SMULL_asimddiff_L   ', '                       '];
T[' SMULL    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'SMULL_asimddiff_L   ', '                       '];
T[' SMULL    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'SMULL_asimddiff_L   ', '                       '];
T[' SMULL2   VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SMULL_asimddiff_L   ', '                       '];
T[' SMULL2   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'SMULL_asimddiff_L   ', '                       '];
T[' SMULL2   VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'SMULL_asimddiff_L   ', '                       '];
T[' SQDMLAL  VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'SQDMLAL_asimddiff_L ', '                       '];
T[' SQDMLAL  VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'SQDMLAL_asimddiff_L ', '                       '];
T[' SQDMLAL2 VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'SQDMLAL_asimddiff_L ', '                       '];
T[' SQDMLAL2 VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'SQDMLAL_asimddiff_L ', '                       '];
T[' SQDMLSL  VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'SQDMLSL_asimddiff_L ', '                       '];
T[' SQDMLSL  VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'SQDMLSL_asimddiff_L ', '                       '];
T[' SQDMLSL2 VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'SQDMLSL_asimddiff_L ', '                       '];
T[' SQDMLSL2 VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'SQDMLSL_asimddiff_L ', '                       '];
T[' SQDMULL  VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'SQDMULL_asimddiff_L ', '                       '];
T[' SQDMULL  VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'SQDMULL_asimddiff_L ', '                       '];
T[' SQDMULL2 VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'SQDMULL_asimddiff_L ', '                       '];
T[' SQDMULL2 VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'SQDMULL_asimddiff_L ', '                       '];
T[' SSUBL    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SSUBL_asimddiff_L   ', '                       '];
T[' SSUBL    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'SSUBL_asimddiff_L   ', '                       '];
T[' SSUBL    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'SSUBL_asimddiff_L   ', '                       '];
T[' SSUBL2   VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SSUBL_asimddiff_L   ', '                       '];
T[' SSUBL2   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'SSUBL_asimddiff_L   ', '                       '];
T[' SSUBL2   VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'SSUBL_asimddiff_L   ', '                       '];
T[' SSUBW    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s8=$Vm   VDT=8B }  ', 'Q=0b0 size=0b00 ', 'SSUBW_asimddiff_W   ', '                       '];
T[' SSUBW    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'SSUBW_asimddiff_W   ', '                       '];
T[' SSUBW    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D } VREG={REG:s32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'SSUBW_asimddiff_W   ', '                       '];
T[' SSUBW2   VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s8=$Vm   VDT=16B}  ', 'Q=0b1 size=0b00 ', 'SSUBW_asimddiff_W   ', '                       '];
T[' SSUBW2   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'SSUBW_asimddiff_W   ', '                       '];
T[' SSUBW2   VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D } VREG={REG:s32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'SSUBW_asimddiff_W   ', '                       '];
T[' SUBHN    VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }  ', 'Q=0b0 size=0b00 ', 'SUBHN_asimddiff_N   ', '                       '];
T[' SUBHN    VREG={REG:i16=$Vd VDT=4H } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }   ', 'Q=0b0 size=0b01 ', 'SUBHN_asimddiff_N   ', '                       '];
T[' SUBHN    VREG={REG:i32=$Vd VDT=2S } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }   ', 'Q=0b0 size=0b10 ', 'SUBHN_asimddiff_N   ', '                       '];
T[' SUBHN2   VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }  ', 'Q=0b1 size=0b00 ', 'SUBHN_asimddiff_N   ', '                       '];
T[' SUBHN2   VREG={REG:i16=$Vd VDT=8H } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }   ', 'Q=0b1 size=0b01 ', 'SUBHN_asimddiff_N   ', '                       '];
T[' SUBHN2   VREG={REG:i32=$Vd VDT=4S } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }   ', 'Q=0b1 size=0b10 ', 'SUBHN_asimddiff_N   ', '                       '];
T[' UABAL    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UABAL_asimddiff_L   ', '                       '];
T[' UABAL    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'UABAL_asimddiff_L   ', '                       '];
T[' UABAL    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'UABAL_asimddiff_L   ', '                       '];
T[' UABAL2   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UABAL_asimddiff_L   ', '                       '];
T[' UABAL2   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'UABAL_asimddiff_L   ', '                       '];
T[' UABAL2   VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'UABAL_asimddiff_L   ', '                       '];
T[' UABDL    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UABDL_asimddiff_L   ', '                       '];
T[' UABDL    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'UABDL_asimddiff_L   ', '                       '];
T[' UABDL    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'UABDL_asimddiff_L   ', '                       '];
T[' UABDL2   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UABDL_asimddiff_L   ', '                       '];
T[' UABDL2   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'UABDL_asimddiff_L   ', '                       '];
T[' UABDL2   VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'UABDL_asimddiff_L   ', '                       '];
T[' UADDL    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UADDL_asimddiff_L   ', '                       '];
T[' UADDL    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'UADDL_asimddiff_L   ', '                       '];
T[' UADDL    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'UADDL_asimddiff_L   ', '                       '];
T[' UADDL2   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UADDL_asimddiff_L   ', '                       '];
T[' UADDL2   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'UADDL_asimddiff_L   ', '                       '];
T[' UADDL2   VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'UADDL_asimddiff_L   ', '                       '];
T[' UADDW    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u8=$Vm   VDT=8B }  ', 'Q=0b0 size=0b00 ', 'UADDW_asimddiff_W   ', '                       '];
T[' UADDW    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'UADDW_asimddiff_W   ', '                       '];
T[' UADDW    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D } VREG={REG:u32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'UADDW_asimddiff_W   ', '                       '];
T[' UADDW2   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u8=$Vm   VDT=16B}  ', 'Q=0b1 size=0b00 ', 'UADDW_asimddiff_W   ', '                       '];
T[' UADDW2   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'UADDW_asimddiff_W   ', '                       '];
T[' UADDW2   VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D } VREG={REG:u32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'UADDW_asimddiff_W   ', '                       '];
T[' UMLAL    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UMLAL_asimddiff_L   ', '                       '];
T[' UMLAL    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'UMLAL_asimddiff_L   ', '                       '];
T[' UMLAL    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'UMLAL_asimddiff_L   ', '                       '];
T[' UMLAL2   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UMLAL_asimddiff_L   ', '                       '];
T[' UMLAL2   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'UMLAL_asimddiff_L   ', '                       '];
T[' UMLAL2   VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'UMLAL_asimddiff_L   ', '                       '];
T[' UMLSL    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UMLSL_asimddiff_L   ', '                       '];
T[' UMLSL    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'UMLSL_asimddiff_L   ', '                       '];
T[' UMLSL    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'UMLSL_asimddiff_L   ', '                       '];
T[' UMLSL2   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UMLSL_asimddiff_L   ', '                       '];
T[' UMLSL2   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'UMLSL_asimddiff_L   ', '                       '];
T[' UMLSL2   VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'UMLSL_asimddiff_L   ', '                       '];
T[' UMULL    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UMULL_asimddiff_L   ', '                       '];
T[' UMULL    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'UMULL_asimddiff_L   ', '                       '];
T[' UMULL    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'UMULL_asimddiff_L   ', '                       '];
T[' UMULL2   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UMULL_asimddiff_L   ', '                       '];
T[' UMULL2   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'UMULL_asimddiff_L   ', '                       '];
T[' UMULL2   VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'UMULL_asimddiff_L   ', '                       '];
T[' USUBL    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'USUBL_asimddiff_L   ', '                       '];
T[' USUBL    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'USUBL_asimddiff_L   ', '                       '];
T[' USUBL    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'USUBL_asimddiff_L   ', '                       '];
T[' USUBL2   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'USUBL_asimddiff_L   ', '                       '];
T[' USUBL2   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'USUBL_asimddiff_L   ', '                       '];
T[' USUBL2   VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'USUBL_asimddiff_L   ', '                       '];
T[' USUBW    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u8=$Vm   VDT=8B }  ', 'Q=0b0 size=0b00 ', 'USUBW_asimddiff_W   ', '                       '];
T[' USUBW    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u16=$Vm VDT=4H }   ', 'Q=0b0 size=0b01 ', 'USUBW_asimddiff_W   ', '                       '];
T[' USUBW    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D } VREG={REG:u32=$Vm VDT=2S }   ', 'Q=0b0 size=0b10 ', 'USUBW_asimddiff_W   ', '                       '];
T[' USUBW2   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u8=$Vm   VDT=16B}  ', 'Q=0b1 size=0b00 ', 'USUBW_asimddiff_W   ', '                       '];
T[' USUBW2   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u16=$Vm VDT=8H }   ', 'Q=0b1 size=0b01 ', 'USUBW_asimddiff_W   ', '                       '];
T[' USUBW2   VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D } VREG={REG:u32=$Vm VDT=4S }   ', 'Q=0b1 size=0b10 ', 'USUBW_asimddiff_W   ', '                       '];



# asimdelem: Advanced SIMD vector x indexed element.
# A64: ig0=0b0 Q:u=0bx U:u=0bx ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode:u=0bxxxx H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asimdelem', 'A64');

DEF Vd     => VECREG('Vd', 'Rd', 'write')               ;
DEF Vm     => VECREG('Vm', 'M:Rm', 'read')              ;
DEF Vm2    => VECREG('Vm', 'Rm', 'read')                ;
DEF Vn     => VECREG('Vn', 'Rn', 'read')                ;
DEF rotate => IMM('rotate', 'rot', table => 'tbl_imm0') ;

T[' BFDOT    VREG={REG:i32=$Vd VDT=2S } VREG={REG:i16=$Vn VDT=4H } EREG={REG:i16=$Vm VDT=2H  IDX={ENCODEDIN=H:L}}                ', 'Q=0b0              ', 'BFDOT_asimdelem_E    ', 'arch_variant=ARMv8v6 extensions=BF16                    '];
T[' BFDOT    VREG={REG:i32=$Vd VDT=4S } VREG={REG:i16=$Vn VDT=8H } EREG={REG:i16=$Vm VDT=2H  IDX={ENCODEDIN=H:L}}                ', 'Q=0b1              ', 'BFDOT_asimdelem_E    ', 'arch_variant=ARMv8v6 extensions=BF16                    '];
T[' BFMLALB  VREG={REG:i32=$Vd VDT=4S } VREG={REG:i16=$Vn VDT=8H } EREG={REG:i16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0              ', 'BFMLAL_asimdelem_F   ', 'arch_variant=ARMv8v6 extensions=BF16                    '];
T[' BFMLALT  VREG={REG:i32=$Vd VDT=4S } VREG={REG:i16=$Vn VDT=8H } EREG={REG:i16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1              ', 'BFMLAL_asimdelem_F   ', 'arch_variant=ARMv8v6 extensions=BF16                    '];
T[' FCMLA    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } EREG={REG:f16=$Vm VDS=H IDX={ENCODEDIN=size}} IMM:i16=$rotate ', 'Q=0b0              ', 'FCMLA_asimdelem_C_H  ', 'arch_variant=ARMv8v3 extensions=FCADD|FP16              '];
T[' FCMLA    VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } EREG={REG:f16=$Vm VDS=H IDX={ENCODEDIN=size}} IMM:i16=$rotate ', 'Q=0b1              ', 'FCMLA_asimdelem_C_H  ', 'arch_variant=ARMv8v3 extensions=FCADD|FP16              '];
T[' FCMLA    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } EREG={REG:f32=$Vm VDS=S IDX={ENCODEDIN=size}} IMM:i32=$rotate ', 'Q=0b1              ', 'FCMLA_asimdelem_C_S  ', 'arch_variant=ARMv8v3 extensions=FCADD|FP16              '];
T[' FMLA     VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } EREG={REG:f16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0              ', 'FMLA_asimdelem_RH_H  ', 'arch_variant=ARMv8v2 extensions=FP16                    '];
T[' FMLA     VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } EREG={REG:f16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1              ', 'FMLA_asimdelem_RH_H  ', 'arch_variant=ARMv8v2 extensions=FP16                    '];
T[' FMLA     VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } EREG={REG:f32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 sz=0b0       ', 'FMLA_asimdelem_R_SD  ', '                                                        '];
T[' FMLA     VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } EREG={REG:f32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 sz=0b0       ', 'FMLA_asimdelem_R_SD  ', '                                                        '];
T[' FMLA     VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } EREG={REG:f64=$Vm VDS=D IDX={ENCODEDIN=H}}                    ', 'Q=0b1 sz=0b1 L=0b0 ', 'FMLA_asimdelem_R_SD  ', '                                                        '];
T[' FMLAL    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f16=$Vn VDT=2H } EREG={REG:f16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0              ', 'FMLAL_asimdelem_LH   ', 'arch_variant=ARMv8v2 extensions=FP16MULNOROUNDINGTOFP32 '];
T[' FMLAL    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f16=$Vn VDT=4H } EREG={REG:f16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1              ', 'FMLAL_asimdelem_LH   ', 'arch_variant=ARMv8v2 extensions=FP16MULNOROUNDINGTOFP32 '];
T[' FMLAL2   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f16=$Vn VDT=2H } EREG={REG:f16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0              ', 'FMLAL2_asimdelem_LH  ', 'arch_variant=ARMv8v2 extensions=FP16MULNOROUNDINGTOFP32 '];
T[' FMLAL2   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f16=$Vn VDT=4H } EREG={REG:f16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1              ', 'FMLAL2_asimdelem_LH  ', 'arch_variant=ARMv8v2 extensions=FP16MULNOROUNDINGTOFP32 '];
T[' FMLS     VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } EREG={REG:f16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0              ', 'FMLS_asimdelem_RH_H  ', 'arch_variant=ARMv8v2 extensions=FP16                    '];
T[' FMLS     VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } EREG={REG:f16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1              ', 'FMLS_asimdelem_RH_H  ', 'arch_variant=ARMv8v2 extensions=FP16                    '];
T[' FMLS     VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } EREG={REG:f32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 sz=0b0       ', 'FMLS_asimdelem_R_SD  ', '                                                        '];
T[' FMLS     VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } EREG={REG:f32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 sz=0b0       ', 'FMLS_asimdelem_R_SD  ', '                                                        '];
T[' FMLS     VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } EREG={REG:f64=$Vm VDS=D IDX={ENCODEDIN=H}}                    ', 'Q=0b1 sz=0b1 L=0b0 ', 'FMLS_asimdelem_R_SD  ', '                                                        '];
T[' FMLSL    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f16=$Vn VDT=2H } EREG={REG:f16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0              ', 'FMLSL_asimdelem_LH   ', 'arch_variant=ARMv8v2 extensions=FP16MULNOROUNDINGTOFP32 '];
T[' FMLSL    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f16=$Vn VDT=4H } EREG={REG:f16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1              ', 'FMLSL_asimdelem_LH   ', 'arch_variant=ARMv8v2 extensions=FP16MULNOROUNDINGTOFP32 '];
T[' FMLSL2   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f16=$Vn VDT=2H } EREG={REG:f16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0              ', 'FMLSL2_asimdelem_LH  ', 'arch_variant=ARMv8v2 extensions=FP16MULNOROUNDINGTOFP32 '];
T[' FMLSL2   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f16=$Vn VDT=4H } EREG={REG:f16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1              ', 'FMLSL2_asimdelem_LH  ', 'arch_variant=ARMv8v2 extensions=FP16MULNOROUNDINGTOFP32 '];
T[' FMUL     VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } EREG={REG:f16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0              ', 'FMUL_asimdelem_RH_H  ', 'arch_variant=ARMv8v2 extensions=FP16                    '];
T[' FMUL     VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } EREG={REG:f16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1              ', 'FMUL_asimdelem_RH_H  ', 'arch_variant=ARMv8v2 extensions=FP16                    '];
T[' FMUL     VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } EREG={REG:f32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 sz=0b0       ', 'FMUL_asimdelem_R_SD  ', '                                                        '];
T[' FMUL     VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } EREG={REG:f32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 sz=0b0       ', 'FMUL_asimdelem_R_SD  ', '                                                        '];
T[' FMUL     VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } EREG={REG:f64=$Vm VDS=D IDX={ENCODEDIN=H}}                    ', 'Q=0b1 sz=0b1 L=0b0 ', 'FMUL_asimdelem_R_SD  ', '                                                        '];
T[' FMULX    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } EREG={REG:f16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0              ', 'FMULX_asimdelem_RH_H ', 'arch_variant=ARMv8v2 extensions=FP16                    '];
T[' FMULX    VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } EREG={REG:f16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1              ', 'FMULX_asimdelem_RH_H ', 'arch_variant=ARMv8v2 extensions=FP16                    '];
T[' FMULX    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } EREG={REG:f32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 sz=0b0       ', 'FMULX_asimdelem_R_SD ', '                                                        '];
T[' FMULX    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } EREG={REG:f32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 sz=0b0       ', 'FMULX_asimdelem_R_SD ', '                                                        '];
T[' FMULX    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } EREG={REG:f64=$Vm VDS=D IDX={ENCODEDIN=H}}                    ', 'Q=0b1 sz=0b1 L=0b0 ', 'FMULX_asimdelem_R_SD ', '                                                        '];
T[' MLA      VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } EREG={REG:i16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0 size=0b01    ', 'MLA_asimdelem_R      ', '                                                        '];
T[' MLA      VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } EREG={REG:i16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1 size=0b01    ', 'MLA_asimdelem_R      ', '                                                        '];
T[' MLA      VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } EREG={REG:i32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 size=0b10    ', 'MLA_asimdelem_R      ', '                                                        '];
T[' MLA      VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } EREG={REG:i32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 size=0b10    ', 'MLA_asimdelem_R      ', '                                                        '];
T[' MLS      VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } EREG={REG:i16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0 size=0b01    ', 'MLS_asimdelem_R      ', '                                                        '];
T[' MLS      VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } EREG={REG:i16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1 size=0b01    ', 'MLS_asimdelem_R      ', '                                                        '];
T[' MLS      VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } EREG={REG:i32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 size=0b10    ', 'MLS_asimdelem_R      ', '                                                        '];
T[' MLS      VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } EREG={REG:i32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 size=0b10    ', 'MLS_asimdelem_R      ', '                                                        '];
T[' MUL      VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } EREG={REG:i16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0 size=0b01    ', 'MUL_asimdelem_R      ', '                                                        '];
T[' MUL      VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } EREG={REG:i16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1 size=0b01    ', 'MUL_asimdelem_R      ', '                                                        '];
T[' MUL      VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } EREG={REG:i32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 size=0b10    ', 'MUL_asimdelem_R      ', '                                                        '];
T[' MUL      VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } EREG={REG:i32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 size=0b10    ', 'MUL_asimdelem_R      ', '                                                        '];
T[' SDOT     VREG={REG:s32=$Vd VDT=2S } VREG={REG:s8=$Vn   VDT=8B } EREG={REG:s8=$Vm   VDT=4B  IDX={ENCODEDIN=H:L}}              ', 'Q=0b0              ', 'SDOT_asimdelem_D     ', 'arch_variant=ARMv8v2 extensions=DOTP                    '];
T[' SDOT     VREG={REG:s32=$Vd VDT=4S } VREG={REG:s8=$Vn   VDT=16B} EREG={REG:s8=$Vm   VDT=4B  IDX={ENCODEDIN=H:L}}              ', 'Q=0b1              ', 'SDOT_asimdelem_D     ', 'arch_variant=ARMv8v2 extensions=DOTP                    '];
T[' SMLAL    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0 size=0b01    ', 'SMLAL_asimdelem_L    ', '                                                        '];
T[' SMLAL    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 size=0b10    ', 'SMLAL_asimdelem_L    ', '                                                        '];
T[' SMLAL2   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1 size=0b01    ', 'SMLAL_asimdelem_L    ', '                                                        '];
T[' SMLAL2   VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 size=0b10    ', 'SMLAL_asimdelem_L    ', '                                                        '];
T[' SMLSL    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0 size=0b01    ', 'SMLSL_asimdelem_L    ', '                                                        '];
T[' SMLSL    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 size=0b10    ', 'SMLSL_asimdelem_L    ', '                                                        '];
T[' SMLSL2   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1 size=0b01    ', 'SMLSL_asimdelem_L    ', '                                                        '];
T[' SMLSL2   VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 size=0b10    ', 'SMLSL_asimdelem_L    ', '                                                        '];
T[' SMULL    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0 size=0b01    ', 'SMULL_asimdelem_L    ', '                                                        '];
T[' SMULL    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 size=0b10    ', 'SMULL_asimdelem_L    ', '                                                        '];
T[' SMULL2   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1 size=0b01    ', 'SMULL_asimdelem_L    ', '                                                        '];
T[' SMULL2   VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 size=0b10    ', 'SMULL_asimdelem_L    ', '                                                        '];
T[' SQDMLAL  VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0 size=0b01    ', 'SQDMLAL_asimdelem_L  ', '                                                        '];
T[' SQDMLAL  VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 size=0b10    ', 'SQDMLAL_asimdelem_L  ', '                                                        '];
T[' SQDMLAL2 VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1 size=0b01    ', 'SQDMLAL_asimdelem_L  ', '                                                        '];
T[' SQDMLAL2 VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 size=0b10    ', 'SQDMLAL_asimdelem_L  ', '                                                        '];
T[' SQDMLSL  VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0 size=0b01    ', 'SQDMLSL_asimdelem_L  ', '                                                        '];
T[' SQDMLSL  VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 size=0b10    ', 'SQDMLSL_asimdelem_L  ', '                                                        '];
T[' SQDMLSL2 VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1 size=0b01    ', 'SQDMLSL_asimdelem_L  ', '                                                        '];
T[' SQDMLSL2 VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 size=0b10    ', 'SQDMLSL_asimdelem_L  ', '                                                        '];
T[' SQDMULH  VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0 size=0b01    ', 'SQDMULH_asimdelem_R  ', '                                                        '];
T[' SQDMULH  VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1 size=0b01    ', 'SQDMULH_asimdelem_R  ', '                                                        '];
T[' SQDMULH  VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 size=0b10    ', 'SQDMULH_asimdelem_R  ', '                                                        '];
T[' SQDMULH  VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 size=0b10    ', 'SQDMULH_asimdelem_R  ', '                                                        '];
T[' SQDMULL  VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0 size=0b01    ', 'SQDMULL_asimdelem_L  ', '                                                        '];
T[' SQDMULL  VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 size=0b10    ', 'SQDMULL_asimdelem_L  ', '                                                        '];
T[' SQDMULL2 VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1 size=0b01    ', 'SQDMULL_asimdelem_L  ', '                                                        '];
T[' SQDMULL2 VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 size=0b10    ', 'SQDMULL_asimdelem_L  ', '                                                        '];
T[' SQRDMLAH VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0 size=0b01    ', 'SQRDMLAH_asimdelem_R ', 'arch_variant=ARMv8v1 extensions=QRDMLAH                 '];
T[' SQRDMLAH VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1 size=0b01    ', 'SQRDMLAH_asimdelem_R ', 'arch_variant=ARMv8v1 extensions=QRDMLAH                 '];
T[' SQRDMLAH VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 size=0b10    ', 'SQRDMLAH_asimdelem_R ', 'arch_variant=ARMv8v1 extensions=QRDMLAH                 '];
T[' SQRDMLAH VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 size=0b10    ', 'SQRDMLAH_asimdelem_R ', 'arch_variant=ARMv8v1 extensions=QRDMLAH                 '];
T[' SQRDMLSH VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0 size=0b01    ', 'SQRDMLSH_asimdelem_R ', 'arch_variant=ARMv8v1 extensions=QRDMLAH                 '];
T[' SQRDMLSH VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1 size=0b01    ', 'SQRDMLSH_asimdelem_R ', 'arch_variant=ARMv8v1 extensions=QRDMLAH                 '];
T[' SQRDMLSH VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 size=0b10    ', 'SQRDMLSH_asimdelem_R ', 'arch_variant=ARMv8v1 extensions=QRDMLAH                 '];
T[' SQRDMLSH VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 size=0b10    ', 'SQRDMLSH_asimdelem_R ', 'arch_variant=ARMv8v1 extensions=QRDMLAH                 '];
T[' SQRDMULH VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0 size=0b01    ', 'SQRDMULH_asimdelem_R ', '                                                        '];
T[' SQRDMULH VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } EREG={REG:s16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1 size=0b01    ', 'SQRDMULH_asimdelem_R ', '                                                        '];
T[' SQRDMULH VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 size=0b10    ', 'SQRDMULH_asimdelem_R ', '                                                        '];
T[' SQRDMULH VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } EREG={REG:s32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 size=0b10    ', 'SQRDMULH_asimdelem_R ', '                                                        '];
T[' SUDOT    VREG={REG:u32=$Vd VDT=2S } VREG={REG:u8=$Vn   VDT=8B } EREG={REG:u8=$Vm   VDT=4B  IDX={ENCODEDIN=H:L}}              ', 'Q=0b0              ', 'SUDOT_asimdelem_D    ', 'arch_variant=ARMv8v6 extensions=INT8MATMUL              '];
T[' SUDOT    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u8=$Vn   VDT=16B} EREG={REG:u8=$Vm   VDT=4B  IDX={ENCODEDIN=H:L}}              ', 'Q=0b1              ', 'SUDOT_asimdelem_D    ', 'arch_variant=ARMv8v6 extensions=INT8MATMUL              '];
T[' UDOT     VREG={REG:u32=$Vd VDT=2S } VREG={REG:u8=$Vn   VDT=8B } EREG={REG:u8=$Vm   VDT=4B  IDX={ENCODEDIN=H:L}}              ', 'Q=0b0              ', 'UDOT_asimdelem_D     ', 'arch_variant=ARMv8v2 extensions=DOTP                    '];
T[' UDOT     VREG={REG:u32=$Vd VDT=4S } VREG={REG:u8=$Vn   VDT=16B} EREG={REG:u8=$Vm   VDT=4B  IDX={ENCODEDIN=H:L}}              ', 'Q=0b1              ', 'UDOT_asimdelem_D     ', 'arch_variant=ARMv8v2 extensions=DOTP                    '];
T[' UMLAL    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=4H } EREG={REG:u16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0 size=0b01    ', 'UMLAL_asimdelem_L    ', '                                                        '];
T[' UMLAL    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=2S } EREG={REG:u32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 size=0b10    ', 'UMLAL_asimdelem_L    ', '                                                        '];
T[' UMLAL2   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=8H } EREG={REG:u16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1 size=0b01    ', 'UMLAL_asimdelem_L    ', '                                                        '];
T[' UMLAL2   VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=4S } EREG={REG:u32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 size=0b10    ', 'UMLAL_asimdelem_L    ', '                                                        '];
T[' UMLSL    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=4H } EREG={REG:u16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0 size=0b01    ', 'UMLSL_asimdelem_L    ', '                                                        '];
T[' UMLSL    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=2S } EREG={REG:u32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 size=0b10    ', 'UMLSL_asimdelem_L    ', '                                                        '];
T[' UMLSL2   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=8H } EREG={REG:u16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1 size=0b01    ', 'UMLSL_asimdelem_L    ', '                                                        '];
T[' UMLSL2   VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=4S } EREG={REG:u32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 size=0b10    ', 'UMLSL_asimdelem_L    ', '                                                        '];
T[' UMULL    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=4H } EREG={REG:u16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b0 size=0b01    ', 'UMULL_asimdelem_L    ', '                                                        '];
T[' UMULL    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=2S } EREG={REG:u32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b0 size=0b10    ', 'UMULL_asimdelem_L    ', '                                                        '];
T[' UMULL2   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=8H } EREG={REG:u16=$Vm2 VDS=H IDX={ENCODEDIN=H:L:M}}               ', 'Q=0b1 size=0b01    ', 'UMULL_asimdelem_L    ', '                                                        '];
T[' UMULL2   VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=4S } EREG={REG:u32=$Vm VDS=S IDX={ENCODEDIN=H:L}}                  ', 'Q=0b1 size=0b10    ', 'UMULL_asimdelem_L    ', '                                                        '];
T[' USDOT    VREG={REG:u32=$Vd VDT=2S } VREG={REG:u8=$Vn   VDT=8B } EREG={REG:u8=$Vm   VDT=4B  IDX={ENCODEDIN=H:L}}              ', 'Q=0b0              ', 'USDOT_asimdelem_D    ', 'arch_variant=ARMv8v6 extensions=INT8MATMUL              '];
T[' USDOT    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u8=$Vn   VDT=16B} EREG={REG:u8=$Vm   VDT=4B  IDX={ENCODEDIN=H:L}}              ', 'Q=0b1              ', 'USDOT_asimdelem_D    ', 'arch_variant=ARMv8v6 extensions=INT8MATMUL              '];



# asimdext: Advanced SIMD extract.
# A64: ig0=0b0 Q:u=0bx ig1=0b101110 op2:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 imm4:u=0bxxxx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asimdext', 'A64');

DEF Vd   => VECREG('Vd', 'Rd', 'write') ;
DEF Vm   => VECREG('Vm', 'Rm', 'read')  ;
DEF Vn   => VECREG('Vn', 'Rn', 'read')  ;
DEF imm  => IMM('imm', 'imm4<2:0>')     ;
DEF imm2 => IMM('imm', 'imm4')          ;

T[' EXT VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } IMM:i8=$imm  ', 'Q=0b0 imm4=0b0xxx ', 'EXT_asimdext_only ', ' '];
T[' EXT VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} IMM:i8=$imm2 ', 'Q=0b1             ', 'EXT_asimdext_only ', ' '];



# asimdimm: Advanced SIMD modified immediate.
# A64: ig0=0b0 Q:u=0bx op:u=0bx ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0bxxxx o2:u=0bx ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx
HINT('ICLASS', 'asimdimm', 'A64');

DEF Dd      => SIMD64('Dd', 'Rd', 'write')                      ;
DEF Vd      => VECREG('Vd', 'Rd', 'write')                      ;
DEF advimm  => IMM('advimm', 'op:cmode:a:b:c:d:e:f:g:h')        ;
DEF amount0 => IMM('amount', 'cmode<1>', table => 'tbl_imm1')   ;
DEF amount2 => IMM('amount', 'cmode<2:1>', table => 'tbl_imm2') ;
DEF amount3 => IMM('amount', 'cmode<0>', table => 'tbl_imm3')   ;
DEF imm     => IMM('imm8', 'a:b:c:d:e:f:g:h')                   ;
DEF vfpimm  => FPIMM('vfpimm', 'a:b:c:d:e:f:g:h')               ;

T[' BIC  VREG={REG:i16=$Vd VDT=4H } IMMSH={IMM:i16=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i16=$amount0}} ', 'Q=0b0 ', 'BIC_asimdimm_L_hl  ', '                                     '];
T[' BIC  VREG={REG:i16=$Vd VDT=8H } IMMSH={IMM:i16=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i16=$amount0}} ', 'Q=0b1 ', 'BIC_asimdimm_L_hl  ', '                                     '];
T[' BIC  VREG={REG:i32=$Vd VDT=2S } IMMSH={IMM:i32=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount2}} ', 'Q=0b0 ', 'BIC_asimdimm_L_sl  ', '                                     '];
T[' BIC  VREG={REG:i32=$Vd VDT=4S } IMMSH={IMM:i32=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount2}} ', 'Q=0b1 ', 'BIC_asimdimm_L_sl  ', '                                     '];
T[' FMOV VREG={REG:f16=$Vd VDT=4H } FPIMM:f16=$vfpimm                                                ', 'Q=0b0 ', 'FMOV_asimdimm_H_h  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMOV VREG={REG:f16=$Vd VDT=8H } FPIMM:f16=$vfpimm                                                ', 'Q=0b1 ', 'FMOV_asimdimm_H_h  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMOV VREG={REG:f32=$Vd VDT=2S } IMM:f32=$advimm                                                  ', 'Q=0b0 ', 'FMOV_asimdimm_S_s  ', '                                     '];
T[' FMOV VREG={REG:f32=$Vd VDT=4S } IMM:f32=$advimm                                                  ', 'Q=0b1 ', 'FMOV_asimdimm_S_s  ', '                                     '];
T[' FMOV VREG={REG:f64=$Vd VDT=2D } IMM:f64=$advimm                                                  ', 'NONE  ', 'FMOV_asimdimm_D2_d ', '                                     '];
T[' MOVI VREG={REG:i8=$Vd   VDT=8B } IMMSH={IMM:i8=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:u8=0}}         ', 'Q=0b0 ', 'MOVI_asimdimm_N_b  ', '                                     '];
T[' MOVI VREG={REG:i8=$Vd   VDT=16B} IMMSH={IMM:i8=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:u8=0}}         ', 'Q=0b1 ', 'MOVI_asimdimm_N_b  ', '                                     '];
T[' MOVI VREG={REG:i16=$Vd VDT=4H } IMMSH={IMM:i16=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i16=$amount0}} ', 'Q=0b0 ', 'MOVI_asimdimm_L_hl ', '                                     '];
T[' MOVI VREG={REG:i16=$Vd VDT=8H } IMMSH={IMM:i16=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i16=$amount0}} ', 'Q=0b1 ', 'MOVI_asimdimm_L_hl ', '                                     '];
T[' MOVI VREG={REG:i32=$Vd VDT=2S } IMMSH={IMM:i32=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount2}} ', 'Q=0b0 ', 'MOVI_asimdimm_L_sl ', '                                     '];
T[' MOVI VREG={REG:i32=$Vd VDT=4S } IMMSH={IMM:i32=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount2}} ', 'Q=0b1 ', 'MOVI_asimdimm_L_sl ', '                                     '];
T[' MOVI VREG={REG:i32=$Vd VDT=2S } IMMSH={IMM:i32=$imm SHIFTER={SHIFT=MSL AMOUNT:i32=$amount3}}     ', 'Q=0b0 ', 'MOVI_asimdimm_M_sm ', '                                     '];
T[' MOVI VREG={REG:i32=$Vd VDT=4S } IMMSH={IMM:i32=$imm SHIFTER={SHIFT=MSL AMOUNT:i32=$amount3}}     ', 'Q=0b1 ', 'MOVI_asimdimm_M_sm ', '                                     '];
T[' MOVI REG:i64=$Dd IMM:i64=$advimm                                                                 ', 'NONE  ', 'MOVI_asimdimm_D_ds ', '                                     '];
T[' MOVI VREG={REG:i64=$Vd VDT=2D } IMM:i64=$advimm                                                  ', 'NONE  ', 'MOVI_asimdimm_D2_d ', '                                     '];
T[' MVNI VREG={REG:i16=$Vd VDT=4H } IMMSH={IMM:i16=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i16=$amount0}} ', 'Q=0b0 ', 'MVNI_asimdimm_L_hl ', '                                     '];
T[' MVNI VREG={REG:i16=$Vd VDT=8H } IMMSH={IMM:i16=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i16=$amount0}} ', 'Q=0b1 ', 'MVNI_asimdimm_L_hl ', '                                     '];
T[' MVNI VREG={REG:i32=$Vd VDT=2S } IMMSH={IMM:i32=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount2}} ', 'Q=0b0 ', 'MVNI_asimdimm_L_sl ', '                                     '];
T[' MVNI VREG={REG:i32=$Vd VDT=4S } IMMSH={IMM:i32=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount2}} ', 'Q=0b1 ', 'MVNI_asimdimm_L_sl ', '                                     '];
T[' MVNI VREG={REG:i32=$Vd VDT=2S } IMMSH={IMM:i32=$imm SHIFTER={SHIFT=MSL AMOUNT:i32=$amount3}}     ', 'Q=0b0 ', 'MVNI_asimdimm_M_sm ', '                                     '];
T[' MVNI VREG={REG:i32=$Vd VDT=4S } IMMSH={IMM:i32=$imm SHIFTER={SHIFT=MSL AMOUNT:i32=$amount3}}     ', 'Q=0b1 ', 'MVNI_asimdimm_M_sm ', '                                     '];
T[' ORR  VREG={REG:i16=$Vd VDT=4H } IMMSH={IMM:i16=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i16=$amount0}} ', 'Q=0b0 ', 'ORR_asimdimm_L_hl  ', '                                     '];
T[' ORR  VREG={REG:i16=$Vd VDT=8H } IMMSH={IMM:i16=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i16=$amount0}} ', 'Q=0b1 ', 'ORR_asimdimm_L_hl  ', '                                     '];
T[' ORR  VREG={REG:i32=$Vd VDT=2S } IMMSH={IMM:i32=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount2}} ', 'Q=0b0 ', 'ORR_asimdimm_L_sl  ', '                                     '];
T[' ORR  VREG={REG:i32=$Vd VDT=4S } IMMSH={IMM:i32=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount2}} ', 'Q=0b1 ', 'ORR_asimdimm_L_sl  ', '                                     '];



# asimdins: Advanced SIMD copy.
# A64: ig0=0b0 Q:u=0bx op:u=0bx ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4:u=0bxxxx ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asimdins', 'A64');

DEF Vd => VECREG('Vd', 'Rd', 'write') ;
DEF Vn => VECREG('Vn', 'Rn', 'read')  ;
DEF Wd => GPR32('Wd', 'Rd', 'write')  ;
DEF Wn => GPR32('Wn', 'Rn', 'read')   ;
DEF Xd => GPR64('Xd', 'Rd', 'write')  ;
DEF Xn => GPR64('Xn', 'Rn', 'read')   ;

T[' DUP  VREG={REG:i8=$Vd   VDT=8B } EREG={REG:i8=$Vn   VDS=B IDX={ENCODEDIN=imm5<4:1>}}                         ', 'Q=0b0 imm5=0bxxxx1 ', 'DUP_asimdins_DV_v     ', ' '];
T[' DUP  VREG={REG:i8=$Vd   VDT=16B} EREG={REG:i8=$Vn   VDS=B IDX={ENCODEDIN=imm5<4:1>}}                         ', 'Q=0b1 imm5=0bxxxx1 ', 'DUP_asimdins_DV_v     ', ' '];
T[' DUP  VREG={REG:i16=$Vd VDT=4H } EREG={REG:i16=$Vn VDS=H IDX={ENCODEDIN=imm5<4:2>}}                           ', 'Q=0b0 imm5=0bxxx10 ', 'DUP_asimdins_DV_v     ', ' '];
T[' DUP  VREG={REG:i16=$Vd VDT=8H } EREG={REG:i16=$Vn VDS=H IDX={ENCODEDIN=imm5<4:2>}}                           ', 'Q=0b1 imm5=0bxxx10 ', 'DUP_asimdins_DV_v     ', ' '];
T[' DUP  VREG={REG:i32=$Vd VDT=2S } EREG={REG:i32=$Vn VDS=S IDX={ENCODEDIN=imm5<4:3>}}                           ', 'Q=0b0 imm5=0bxx100 ', 'DUP_asimdins_DV_v     ', ' '];
T[' DUP  VREG={REG:i32=$Vd VDT=4S } EREG={REG:i32=$Vn VDS=S IDX={ENCODEDIN=imm5<4:3>}}                           ', 'Q=0b1 imm5=0bxx100 ', 'DUP_asimdins_DV_v     ', ' '];
T[' DUP  VREG={REG:i64=$Vd VDT=2D } EREG={REG:i64=$Vn VDS=D IDX={ENCODEDIN=imm5<4>}}                             ', 'Q=0b1 imm5=0bx1000 ', 'DUP_asimdins_DV_v     ', ' '];
T[' DUP  VREG={REG:i8=$Vd   VDT=8B } REG=$Wn                                                                     ', 'Q=0b0 imm5=0bxxxx1 ', 'DUP_asimdins_DR_r     ', ' '];
T[' DUP  VREG={REG:i8=$Vd   VDT=16B} REG=$Wn                                                                     ', 'Q=0b1 imm5=0bxxxx1 ', 'DUP_asimdins_DR_r     ', ' '];
T[' DUP  VREG={REG:i16=$Vd VDT=4H } REG=$Wn                                                                      ', 'Q=0b0 imm5=0bxxx10 ', 'DUP_asimdins_DR_r     ', ' '];
T[' DUP  VREG={REG:i16=$Vd VDT=8H } REG=$Wn                                                                      ', 'Q=0b1 imm5=0bxxx10 ', 'DUP_asimdins_DR_r     ', ' '];
T[' DUP  VREG={REG:i32=$Vd VDT=2S } REG=$Wn                                                                      ', 'Q=0b0 imm5=0bxx100 ', 'DUP_asimdins_DR_r     ', ' '];
T[' DUP  VREG={REG:i32=$Vd VDT=4S } REG=$Wn                                                                      ', 'Q=0b1 imm5=0bxx100 ', 'DUP_asimdins_DR_r     ', ' '];
T[' DUP  VREG={REG:i64=$Vd VDT=2D } REG=$Xn                                                                      ', 'Q=0b1 imm5=0bx1000 ', 'DUP_asimdins_DR_r     ', ' '];
T[' INS  EREG={REG:i8=$Vd   VDS=B IDX={ENCODEDIN=imm5<4:1>}} EREG={REG:i8=$Vn   VDS=B IDX={ENCODEDIN=imm4<3:0>}} ', 'imm5=0bxxxx1       ', 'INS_asimdins_IV_v     ', ' '];
T[' INS  EREG={REG:i16=$Vd VDS=H IDX={ENCODEDIN=imm5<4:2>}} EREG={REG:i16=$Vn VDS=H IDX={ENCODEDIN=imm4<3:1>}}   ', 'imm5=0bxxx10       ', 'INS_asimdins_IV_v     ', ' '];
T[' INS  EREG={REG:i32=$Vd VDS=S IDX={ENCODEDIN=imm5<4:3>}} EREG={REG:i32=$Vn VDS=S IDX={ENCODEDIN=imm4<3:2>}}   ', 'imm5=0bxx100       ', 'INS_asimdins_IV_v     ', ' '];
T[' INS  EREG={REG:i64=$Vd VDS=D IDX={ENCODEDIN=imm5<4>}} EREG={REG:i64=$Vn VDS=D IDX={ENCODEDIN=imm4<3>}}       ', 'imm5=0bx1000       ', 'INS_asimdins_IV_v     ', ' '];
T[' INS  EREG={REG:i8=$Vd   VDS=B IDX={ENCODEDIN=imm5<4:1>}} REG=$Wn                                             ', 'imm5=0bxxxx1       ', 'INS_asimdins_IR_r     ', ' '];
T[' INS  EREG={REG:i16=$Vd VDS=H IDX={ENCODEDIN=imm5<4:2>}} REG=$Wn                                              ', 'imm5=0bxxx10       ', 'INS_asimdins_IR_r     ', ' '];
T[' INS  EREG={REG:i32=$Vd VDS=S IDX={ENCODEDIN=imm5<4:3>}} REG=$Wn                                              ', 'imm5=0bxx100       ', 'INS_asimdins_IR_r     ', ' '];
T[' INS  EREG={REG:i64=$Vd VDS=D IDX={ENCODEDIN=imm5<4>}} REG=$Xn                                                ', 'imm5=0bx1000       ', 'INS_asimdins_IR_r     ', ' '];
T[' SMOV REG:s32=$Wd EREG={REG:s8=$Vn   VDS=B IDX={ENCODEDIN=imm5<4:1>}}                                         ', 'imm5=0bxxxx1       ', 'SMOV_asimdins_W_w     ', ' '];
T[' SMOV REG:s32=$Wd EREG={REG:s16=$Vn VDS=H IDX={ENCODEDIN=imm5<4:2>}}                                          ', 'imm5=0bxxx10       ', 'SMOV_asimdins_W_w     ', ' '];
T[' SMOV REG:s64=$Xd EREG={REG:s8=$Vn   VDS=B IDX={ENCODEDIN=imm5<4:1>}}                                         ', 'imm5=0bxxxx1       ', 'SMOV_asimdins_X_x     ', ' '];
T[' SMOV REG:s64=$Xd EREG={REG:s16=$Vn VDS=H IDX={ENCODEDIN=imm5<4:2>}}                                          ', 'imm5=0bxxx10       ', 'SMOV_asimdins_X_x     ', ' '];
T[' SMOV REG:s64=$Xd EREG={REG:s32=$Vn VDS=S IDX={ENCODEDIN=imm5<4:3>}}                                          ', 'imm5=0bxx100       ', 'SMOV_asimdins_X_x     ', ' '];
T[' UMOV REG:u32=$Wd EREG={REG:u8=$Vn   VDS=B IDX={ENCODEDIN=imm5<4:1>}}                                         ', 'imm5=0bxxxx1       ', 'UMOV_asimdins_W_w     ', ' '];
T[' UMOV REG:u32=$Wd EREG={REG:u16=$Vn VDS=H IDX={ENCODEDIN=imm5<4:2>}}                                          ', 'imm5=0bxxx10       ', 'UMOV_asimdins_W_w     ', ' '];
T[' UMOV REG:u32=$Wd EREG={REG:u32=$Vn VDS=S IDX={ENCODEDIN=imm5<4:3>}}                                          ', 'imm5=0bxx100       ', 'UMOV_asimdins_W_w     ', ' '];
T[' UMOV REG:u64=$Xd EREG={REG:u64=$Vn VDS=D IDX={ENCODEDIN=imm5<4>}}                                            ', 'NONE               ', 'UMOV_asimdins_X_x     ', ' '];
T[' MOV  EREG={REG:i8=$Vd   VDS=B IDX={ENCODEDIN=imm5<4:1>}} EREG={REG:i8=$Vn   VDS=B IDX={ENCODEDIN=imm4<3:0>}} ', 'imm5=0bxxxx1       ', 'MOV_INS_asimdins_IV_v ', ' '];
T[' MOV  EREG={REG:i16=$Vd VDS=H IDX={ENCODEDIN=imm5<4:2>}} EREG={REG:i16=$Vn VDS=H IDX={ENCODEDIN=imm4<3:1>}}   ', 'imm5=0bxxx10       ', 'MOV_INS_asimdins_IV_v ', ' '];
T[' MOV  EREG={REG:i32=$Vd VDS=S IDX={ENCODEDIN=imm5<4:3>}} EREG={REG:i32=$Vn VDS=S IDX={ENCODEDIN=imm4<3:2>}}   ', 'imm5=0bxx100       ', 'MOV_INS_asimdins_IV_v ', ' '];
T[' MOV  EREG={REG:i64=$Vd VDS=D IDX={ENCODEDIN=imm5<4>}} EREG={REG:i64=$Vn VDS=D IDX={ENCODEDIN=imm4<3>}}       ', 'imm5=0bx1000       ', 'MOV_INS_asimdins_IV_v ', ' '];
T[' MOV  EREG={REG:i8=$Vd   VDS=B IDX={ENCODEDIN=imm5<4:1>}} REG=$Wn                                             ', 'imm5=0bxxxx1       ', 'MOV_INS_asimdins_IR_r ', ' '];
T[' MOV  EREG={REG:i16=$Vd VDS=H IDX={ENCODEDIN=imm5<4:2>}} REG=$Wn                                              ', 'imm5=0bxxx10       ', 'MOV_INS_asimdins_IR_r ', ' '];
T[' MOV  EREG={REG:i32=$Vd VDS=S IDX={ENCODEDIN=imm5<4:3>}} REG=$Wn                                              ', 'imm5=0bxx100       ', 'MOV_INS_asimdins_IR_r ', ' '];
T[' MOV  EREG={REG:i64=$Vd VDS=D IDX={ENCODEDIN=imm5<4>}} REG=$Xn                                                ', 'imm5=0bx1000       ', 'MOV_INS_asimdins_IR_r ', ' '];
T[' MOV  REG=$Wd EREG={REG:i32=$Vn VDS=S IDX={ENCODEDIN=imm5<4:3>}}                                              ', 'NONE               ', 'MOV_UMOV_asimdins_W_w ', ' '];
T[' MOV  REG=$Xd EREG={REG:i64=$Vn VDS=D IDX={ENCODEDIN=imm5<4>}}                                                ', 'NONE               ', 'MOV_UMOV_asimdins_X_x ', ' '];



# asimdmisc: Advanced SIMD two-register miscellaneous.
# A64: ig0=0b0 Q:u=0bx U:u=0bx ig1=0b01110 size:u=0bxx ig2=0b10000 opcode:u=0bxxxxx ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asimdmisc', 'A64');

DEF Vd => VECREG('Vd', 'Rd', 'write') ;
DEF Vn => VECREG('Vn', 'Rn', 'read')  ;

T[' ABS      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B }             ', 'Q=0b0 size=0b00 ', 'ABS_asimdmisc_R      ', '                                      '];
T[' ABS      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B}             ', 'Q=0b1 size=0b00 ', 'ABS_asimdmisc_R      ', '                                      '];
T[' ABS      VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H }               ', 'Q=0b0 size=0b01 ', 'ABS_asimdmisc_R      ', '                                      '];
T[' ABS      VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H }               ', 'Q=0b1 size=0b01 ', 'ABS_asimdmisc_R      ', '                                      '];
T[' ABS      VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S }               ', 'Q=0b0 size=0b10 ', 'ABS_asimdmisc_R      ', '                                      '];
T[' ABS      VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S }               ', 'Q=0b1 size=0b10 ', 'ABS_asimdmisc_R      ', '                                      '];
T[' ABS      VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D }               ', 'Q=0b1 size=0b11 ', 'ABS_asimdmisc_R      ', '                                      '];
T[' BFCVTN   VREG={REG:i16=$Vd VDT=4H } VREG={REG:i32=$Vn VDT=4S }               ', 'Q=0b0           ', 'BFCVTN_asimdmisc_4S  ', 'arch_variant=ARMv8v6 extensions=BF16  '];
T[' BFCVTN2  VREG={REG:i16=$Vd VDT=8H } VREG={REG:i32=$Vn VDT=4S }               ', 'Q=0b1           ', 'BFCVTN_asimdmisc_4S  ', 'arch_variant=ARMv8v6 extensions=BF16  '];
T[' CLS      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B }             ', 'Q=0b0 size=0b00 ', 'CLS_asimdmisc_R      ', '                                      '];
T[' CLS      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B}             ', 'Q=0b1 size=0b00 ', 'CLS_asimdmisc_R      ', '                                      '];
T[' CLS      VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H }               ', 'Q=0b0 size=0b01 ', 'CLS_asimdmisc_R      ', '                                      '];
T[' CLS      VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H }               ', 'Q=0b1 size=0b01 ', 'CLS_asimdmisc_R      ', '                                      '];
T[' CLS      VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S }               ', 'Q=0b0 size=0b10 ', 'CLS_asimdmisc_R      ', '                                      '];
T[' CLS      VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S }               ', 'Q=0b1 size=0b10 ', 'CLS_asimdmisc_R      ', '                                      '];
T[' CLZ      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B }             ', 'Q=0b0 size=0b00 ', 'CLZ_asimdmisc_R      ', '                                      '];
T[' CLZ      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B}             ', 'Q=0b1 size=0b00 ', 'CLZ_asimdmisc_R      ', '                                      '];
T[' CLZ      VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H }               ', 'Q=0b0 size=0b01 ', 'CLZ_asimdmisc_R      ', '                                      '];
T[' CLZ      VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H }               ', 'Q=0b1 size=0b01 ', 'CLZ_asimdmisc_R      ', '                                      '];
T[' CLZ      VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S }               ', 'Q=0b0 size=0b10 ', 'CLZ_asimdmisc_R      ', '                                      '];
T[' CLZ      VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S }               ', 'Q=0b1 size=0b10 ', 'CLZ_asimdmisc_R      ', '                                      '];
T[' CMEQ     VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } IMM:u8=0    ', 'Q=0b0 size=0b00 ', 'CMEQ_asimdmisc_Z     ', '                                      '];
T[' CMEQ     VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} IMM:u8=0    ', 'Q=0b1 size=0b00 ', 'CMEQ_asimdmisc_Z     ', '                                      '];
T[' CMEQ     VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } IMM:u8=0      ', 'Q=0b0 size=0b01 ', 'CMEQ_asimdmisc_Z     ', '                                      '];
T[' CMEQ     VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } IMM:u8=0      ', 'Q=0b1 size=0b01 ', 'CMEQ_asimdmisc_Z     ', '                                      '];
T[' CMEQ     VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } IMM:u8=0      ', 'Q=0b0 size=0b10 ', 'CMEQ_asimdmisc_Z     ', '                                      '];
T[' CMEQ     VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } IMM:u8=0      ', 'Q=0b1 size=0b10 ', 'CMEQ_asimdmisc_Z     ', '                                      '];
T[' CMEQ     VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } IMM:u8=0      ', 'Q=0b1 size=0b11 ', 'CMEQ_asimdmisc_Z     ', '                                      '];
T[' CMGE     VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } IMM:u8=0    ', 'Q=0b0 size=0b00 ', 'CMGE_asimdmisc_Z     ', '                                      '];
T[' CMGE     VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} IMM:u8=0    ', 'Q=0b1 size=0b00 ', 'CMGE_asimdmisc_Z     ', '                                      '];
T[' CMGE     VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } IMM:u8=0      ', 'Q=0b0 size=0b01 ', 'CMGE_asimdmisc_Z     ', '                                      '];
T[' CMGE     VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } IMM:u8=0      ', 'Q=0b1 size=0b01 ', 'CMGE_asimdmisc_Z     ', '                                      '];
T[' CMGE     VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } IMM:u8=0      ', 'Q=0b0 size=0b10 ', 'CMGE_asimdmisc_Z     ', '                                      '];
T[' CMGE     VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } IMM:u8=0      ', 'Q=0b1 size=0b10 ', 'CMGE_asimdmisc_Z     ', '                                      '];
T[' CMGE     VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } IMM:u8=0      ', 'Q=0b1 size=0b11 ', 'CMGE_asimdmisc_Z     ', '                                      '];
T[' CMGT     VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } IMM:u8=0    ', 'Q=0b0 size=0b00 ', 'CMGT_asimdmisc_Z     ', '                                      '];
T[' CMGT     VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} IMM:u8=0    ', 'Q=0b1 size=0b00 ', 'CMGT_asimdmisc_Z     ', '                                      '];
T[' CMGT     VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } IMM:u8=0      ', 'Q=0b0 size=0b01 ', 'CMGT_asimdmisc_Z     ', '                                      '];
T[' CMGT     VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } IMM:u8=0      ', 'Q=0b1 size=0b01 ', 'CMGT_asimdmisc_Z     ', '                                      '];
T[' CMGT     VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } IMM:u8=0      ', 'Q=0b0 size=0b10 ', 'CMGT_asimdmisc_Z     ', '                                      '];
T[' CMGT     VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } IMM:u8=0      ', 'Q=0b1 size=0b10 ', 'CMGT_asimdmisc_Z     ', '                                      '];
T[' CMGT     VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } IMM:u8=0      ', 'Q=0b1 size=0b11 ', 'CMGT_asimdmisc_Z     ', '                                      '];
T[' CMLE     VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } IMM:u8=0    ', 'Q=0b0 size=0b00 ', 'CMLE_asimdmisc_Z     ', '                                      '];
T[' CMLE     VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} IMM:u8=0    ', 'Q=0b1 size=0b00 ', 'CMLE_asimdmisc_Z     ', '                                      '];
T[' CMLE     VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } IMM:u8=0      ', 'Q=0b0 size=0b01 ', 'CMLE_asimdmisc_Z     ', '                                      '];
T[' CMLE     VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } IMM:u8=0      ', 'Q=0b1 size=0b01 ', 'CMLE_asimdmisc_Z     ', '                                      '];
T[' CMLE     VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } IMM:u8=0      ', 'Q=0b0 size=0b10 ', 'CMLE_asimdmisc_Z     ', '                                      '];
T[' CMLE     VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } IMM:u8=0      ', 'Q=0b1 size=0b10 ', 'CMLE_asimdmisc_Z     ', '                                      '];
T[' CMLE     VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } IMM:u8=0      ', 'Q=0b1 size=0b11 ', 'CMLE_asimdmisc_Z     ', '                                      '];
T[' CMLT     VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } IMM:u8=0    ', 'Q=0b0 size=0b00 ', 'CMLT_asimdmisc_Z     ', '                                      '];
T[' CMLT     VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} IMM:u8=0    ', 'Q=0b1 size=0b00 ', 'CMLT_asimdmisc_Z     ', '                                      '];
T[' CMLT     VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } IMM:u8=0      ', 'Q=0b0 size=0b01 ', 'CMLT_asimdmisc_Z     ', '                                      '];
T[' CMLT     VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } IMM:u8=0      ', 'Q=0b1 size=0b01 ', 'CMLT_asimdmisc_Z     ', '                                      '];
T[' CMLT     VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } IMM:u8=0      ', 'Q=0b0 size=0b10 ', 'CMLT_asimdmisc_Z     ', '                                      '];
T[' CMLT     VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } IMM:u8=0      ', 'Q=0b1 size=0b10 ', 'CMLT_asimdmisc_Z     ', '                                      '];
T[' CMLT     VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } IMM:u8=0      ', 'Q=0b1 size=0b11 ', 'CMLT_asimdmisc_Z     ', '                                      '];
T[' CNT      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B }             ', 'Q=0b0 size=0b00 ', 'CNT_asimdmisc_R      ', '                                      '];
T[' CNT      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B}             ', 'Q=0b1 size=0b00 ', 'CNT_asimdmisc_R      ', '                                      '];
T[' FABS     VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FABS_asimdmisc_R     ', '                                      '];
T[' FABS     VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FABS_asimdmisc_R     ', '                                      '];
T[' FABS     VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FABS_asimdmisc_R     ', '                                      '];
T[' FCMEQ    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } FPIMM:f32=0.0 ', 'Q=0b0 sz=0b0    ', 'FCMEQ_asimdmisc_FZ   ', '                                      '];
T[' FCMEQ    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } FPIMM:f32=0.0 ', 'Q=0b1 sz=0b0    ', 'FCMEQ_asimdmisc_FZ   ', '                                      '];
T[' FCMEQ    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } FPIMM:f32=0.0 ', 'Q=0b1 sz=0b1    ', 'FCMEQ_asimdmisc_FZ   ', '                                      '];
T[' FCMGE    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } FPIMM:f32=0.0 ', 'Q=0b0 sz=0b0    ', 'FCMGE_asimdmisc_FZ   ', '                                      '];
T[' FCMGE    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } FPIMM:f32=0.0 ', 'Q=0b1 sz=0b0    ', 'FCMGE_asimdmisc_FZ   ', '                                      '];
T[' FCMGE    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } FPIMM:f32=0.0 ', 'Q=0b1 sz=0b1    ', 'FCMGE_asimdmisc_FZ   ', '                                      '];
T[' FCMGT    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } FPIMM:f32=0.0 ', 'Q=0b0 sz=0b0    ', 'FCMGT_asimdmisc_FZ   ', '                                      '];
T[' FCMGT    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } FPIMM:f32=0.0 ', 'Q=0b1 sz=0b0    ', 'FCMGT_asimdmisc_FZ   ', '                                      '];
T[' FCMGT    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } FPIMM:f32=0.0 ', 'Q=0b1 sz=0b1    ', 'FCMGT_asimdmisc_FZ   ', '                                      '];
T[' FCMLE    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } FPIMM:f32=0.0 ', 'Q=0b0 sz=0b0    ', 'FCMLE_asimdmisc_FZ   ', '                                      '];
T[' FCMLE    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } FPIMM:f32=0.0 ', 'Q=0b1 sz=0b0    ', 'FCMLE_asimdmisc_FZ   ', '                                      '];
T[' FCMLE    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } FPIMM:f32=0.0 ', 'Q=0b1 sz=0b1    ', 'FCMLE_asimdmisc_FZ   ', '                                      '];
T[' FCMLT    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } FPIMM:f32=0.0 ', 'Q=0b0 sz=0b0    ', 'FCMLT_asimdmisc_FZ   ', '                                      '];
T[' FCMLT    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } FPIMM:f32=0.0 ', 'Q=0b1 sz=0b0    ', 'FCMLT_asimdmisc_FZ   ', '                                      '];
T[' FCMLT    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } FPIMM:f32=0.0 ', 'Q=0b1 sz=0b1    ', 'FCMLT_asimdmisc_FZ   ', '                                      '];
T[' FCVTAS   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FCVTAS_asimdmisc_R   ', '                                      '];
T[' FCVTAS   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FCVTAS_asimdmisc_R   ', '                                      '];
T[' FCVTAS   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FCVTAS_asimdmisc_R   ', '                                      '];
T[' FCVTAU   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FCVTAU_asimdmisc_R   ', '                                      '];
T[' FCVTAU   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FCVTAU_asimdmisc_R   ', '                                      '];
T[' FCVTAU   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FCVTAU_asimdmisc_R   ', '                                      '];
T[' FCVTL    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 sz=0b0    ', 'FCVTL_asimdmisc_L    ', '                                      '];
T[' FCVTL    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b1    ', 'FCVTL_asimdmisc_L    ', '                                      '];
T[' FCVTL2   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 sz=0b0    ', 'FCVTL_asimdmisc_L    ', '                                      '];
T[' FCVTL2   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b1    ', 'FCVTL_asimdmisc_L    ', '                                      '];
T[' FCVTMS   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FCVTMS_asimdmisc_R   ', '                                      '];
T[' FCVTMS   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FCVTMS_asimdmisc_R   ', '                                      '];
T[' FCVTMS   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FCVTMS_asimdmisc_R   ', '                                      '];
T[' FCVTMU   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FCVTMU_asimdmisc_R   ', '                                      '];
T[' FCVTMU   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FCVTMU_asimdmisc_R   ', '                                      '];
T[' FCVTMU   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FCVTMU_asimdmisc_R   ', '                                      '];
T[' FCVTN    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b0 sz=0b0    ', 'FCVTN_asimdmisc_N    ', '                                      '];
T[' FCVTN    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b0 sz=0b1    ', 'FCVTN_asimdmisc_N    ', '                                      '];
T[' FCVTN2   VREG={REG:f16=$Vd VDT=8H } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FCVTN_asimdmisc_N    ', '                                      '];
T[' FCVTN2   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FCVTN_asimdmisc_N    ', '                                      '];
T[' FCVTNS   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FCVTNS_asimdmisc_R   ', '                                      '];
T[' FCVTNS   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FCVTNS_asimdmisc_R   ', '                                      '];
T[' FCVTNS   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FCVTNS_asimdmisc_R   ', '                                      '];
T[' FCVTNU   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FCVTNU_asimdmisc_R   ', '                                      '];
T[' FCVTNU   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FCVTNU_asimdmisc_R   ', '                                      '];
T[' FCVTNU   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FCVTNU_asimdmisc_R   ', '                                      '];
T[' FCVTPS   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FCVTPS_asimdmisc_R   ', '                                      '];
T[' FCVTPS   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FCVTPS_asimdmisc_R   ', '                                      '];
T[' FCVTPS   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FCVTPS_asimdmisc_R   ', '                                      '];
T[' FCVTPU   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FCVTPU_asimdmisc_R   ', '                                      '];
T[' FCVTPU   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FCVTPU_asimdmisc_R   ', '                                      '];
T[' FCVTPU   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FCVTPU_asimdmisc_R   ', '                                      '];
T[' FCVTXN   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b0 sz=0b1    ', 'FCVTXN_asimdmisc_N   ', '                                      '];
T[' FCVTXN2  VREG={REG:f32=$Vd VDT=4S } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FCVTXN_asimdmisc_N   ', '                                      '];
T[' FCVTZS   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FCVTZS_asimdmisc_R   ', '                                      '];
T[' FCVTZS   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FCVTZS_asimdmisc_R   ', '                                      '];
T[' FCVTZS   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FCVTZS_asimdmisc_R   ', '                                      '];
T[' FCVTZU   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FCVTZU_asimdmisc_R   ', '                                      '];
T[' FCVTZU   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FCVTZU_asimdmisc_R   ', '                                      '];
T[' FCVTZU   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FCVTZU_asimdmisc_R   ', '                                      '];
T[' FNEG     VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FNEG_asimdmisc_R     ', '                                      '];
T[' FNEG     VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FNEG_asimdmisc_R     ', '                                      '];
T[' FNEG     VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FNEG_asimdmisc_R     ', '                                      '];
T[' FRECPE   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FRECPE_asimdmisc_R   ', '                                      '];
T[' FRECPE   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FRECPE_asimdmisc_R   ', '                                      '];
T[' FRECPE   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FRECPE_asimdmisc_R   ', '                                      '];
T[' FRINT32X VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FRINT32X_asimdmisc_R ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT32X VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FRINT32X_asimdmisc_R ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT32X VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FRINT32X_asimdmisc_R ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT32Z VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FRINT32Z_asimdmisc_R ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT32Z VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FRINT32Z_asimdmisc_R ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT32Z VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FRINT32Z_asimdmisc_R ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT64X VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FRINT64X_asimdmisc_R ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT64X VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FRINT64X_asimdmisc_R ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT64X VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FRINT64X_asimdmisc_R ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT64Z VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FRINT64Z_asimdmisc_R ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT64Z VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FRINT64Z_asimdmisc_R ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT64Z VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FRINT64Z_asimdmisc_R ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINTA   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FRINTA_asimdmisc_R   ', '                                      '];
T[' FRINTA   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FRINTA_asimdmisc_R   ', '                                      '];
T[' FRINTA   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FRINTA_asimdmisc_R   ', '                                      '];
T[' FRINTI   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FRINTI_asimdmisc_R   ', '                                      '];
T[' FRINTI   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FRINTI_asimdmisc_R   ', '                                      '];
T[' FRINTI   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FRINTI_asimdmisc_R   ', '                                      '];
T[' FRINTM   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FRINTM_asimdmisc_R   ', '                                      '];
T[' FRINTM   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FRINTM_asimdmisc_R   ', '                                      '];
T[' FRINTM   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FRINTM_asimdmisc_R   ', '                                      '];
T[' FRINTN   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FRINTN_asimdmisc_R   ', '                                      '];
T[' FRINTN   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FRINTN_asimdmisc_R   ', '                                      '];
T[' FRINTN   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FRINTN_asimdmisc_R   ', '                                      '];
T[' FRINTP   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FRINTP_asimdmisc_R   ', '                                      '];
T[' FRINTP   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FRINTP_asimdmisc_R   ', '                                      '];
T[' FRINTP   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FRINTP_asimdmisc_R   ', '                                      '];
T[' FRINTX   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FRINTX_asimdmisc_R   ', '                                      '];
T[' FRINTX   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FRINTX_asimdmisc_R   ', '                                      '];
T[' FRINTX   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FRINTX_asimdmisc_R   ', '                                      '];
T[' FRINTZ   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FRINTZ_asimdmisc_R   ', '                                      '];
T[' FRINTZ   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FRINTZ_asimdmisc_R   ', '                                      '];
T[' FRINTZ   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FRINTZ_asimdmisc_R   ', '                                      '];
T[' FRSQRTE  VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FRSQRTE_asimdmisc_R  ', '                                      '];
T[' FRSQRTE  VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FRSQRTE_asimdmisc_R  ', '                                      '];
T[' FRSQRTE  VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FRSQRTE_asimdmisc_R  ', '                                      '];
T[' FSQRT    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'FSQRT_asimdmisc_R    ', '                                      '];
T[' FSQRT    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'FSQRT_asimdmisc_R    ', '                                      '];
T[' FSQRT    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'FSQRT_asimdmisc_R    ', '                                      '];
T[' NEG      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B }             ', 'Q=0b0 size=0b00 ', 'NEG_asimdmisc_R      ', '                                      '];
T[' NEG      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B}             ', 'Q=0b1 size=0b00 ', 'NEG_asimdmisc_R      ', '                                      '];
T[' NEG      VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H }               ', 'Q=0b0 size=0b01 ', 'NEG_asimdmisc_R      ', '                                      '];
T[' NEG      VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H }               ', 'Q=0b1 size=0b01 ', 'NEG_asimdmisc_R      ', '                                      '];
T[' NEG      VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S }               ', 'Q=0b0 size=0b10 ', 'NEG_asimdmisc_R      ', '                                      '];
T[' NEG      VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S }               ', 'Q=0b1 size=0b10 ', 'NEG_asimdmisc_R      ', '                                      '];
T[' NEG      VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D }               ', 'Q=0b1 size=0b11 ', 'NEG_asimdmisc_R      ', '                                      '];
T[' NOT      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B }             ', 'Q=0b0           ', 'NOT_asimdmisc_R      ', '                                      '];
T[' NOT      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B}             ', 'Q=0b1           ', 'NOT_asimdmisc_R      ', '                                      '];
T[' RBIT     VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B }             ', 'Q=0b0           ', 'RBIT_asimdmisc_R     ', '                                      '];
T[' RBIT     VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B}             ', 'Q=0b1           ', 'RBIT_asimdmisc_R     ', '                                      '];
T[' REV16    VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B }             ', 'Q=0b0 size=0b00 ', 'REV16_asimdmisc_R    ', '                                      '];
T[' REV16    VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B}             ', 'Q=0b1 size=0b00 ', 'REV16_asimdmisc_R    ', '                                      '];
T[' REV32    VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B }             ', 'Q=0b0 size=0b00 ', 'REV32_asimdmisc_R    ', '                                      '];
T[' REV32    VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B}             ', 'Q=0b1 size=0b00 ', 'REV32_asimdmisc_R    ', '                                      '];
T[' REV32    VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H }               ', 'Q=0b0 size=0b01 ', 'REV32_asimdmisc_R    ', '                                      '];
T[' REV32    VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H }               ', 'Q=0b1 size=0b01 ', 'REV32_asimdmisc_R    ', '                                      '];
T[' REV64    VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B }             ', 'Q=0b0 size=0b00 ', 'REV64_asimdmisc_R    ', '                                      '];
T[' REV64    VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B}             ', 'Q=0b1 size=0b00 ', 'REV64_asimdmisc_R    ', '                                      '];
T[' REV64    VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H }               ', 'Q=0b0 size=0b01 ', 'REV64_asimdmisc_R    ', '                                      '];
T[' REV64    VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H }               ', 'Q=0b1 size=0b01 ', 'REV64_asimdmisc_R    ', '                                      '];
T[' REV64    VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S }               ', 'Q=0b0 size=0b10 ', 'REV64_asimdmisc_R    ', '                                      '];
T[' REV64    VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S }               ', 'Q=0b1 size=0b10 ', 'REV64_asimdmisc_R    ', '                                      '];
T[' SADALP   VREG={REG:s16=$Vd VDT=4H } VREG={REG:s8=$Vn   VDT=8B }              ', 'Q=0b0 size=0b00 ', 'SADALP_asimdmisc_P   ', '                                      '];
T[' SADALP   VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=16B}              ', 'Q=0b1 size=0b00 ', 'SADALP_asimdmisc_P   ', '                                      '];
T[' SADALP   VREG={REG:s32=$Vd VDT=2S } VREG={REG:s16=$Vn VDT=4H }               ', 'Q=0b0 size=0b01 ', 'SADALP_asimdmisc_P   ', '                                      '];
T[' SADALP   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H }               ', 'Q=0b1 size=0b01 ', 'SADALP_asimdmisc_P   ', '                                      '];
T[' SADALP   VREG={REG:s64=$Vd VDT=1D } VREG={REG:s32=$Vn VDT=2S }               ', 'Q=0b0 size=0b10 ', 'SADALP_asimdmisc_P   ', '                                      '];
T[' SADALP   VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S }               ', 'Q=0b1 size=0b10 ', 'SADALP_asimdmisc_P   ', '                                      '];
T[' SADDLP   VREG={REG:s16=$Vd VDT=4H } VREG={REG:s8=$Vn   VDT=8B }              ', 'Q=0b0 size=0b00 ', 'SADDLP_asimdmisc_P   ', '                                      '];
T[' SADDLP   VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=16B}              ', 'Q=0b1 size=0b00 ', 'SADDLP_asimdmisc_P   ', '                                      '];
T[' SADDLP   VREG={REG:s32=$Vd VDT=2S } VREG={REG:s16=$Vn VDT=4H }               ', 'Q=0b0 size=0b01 ', 'SADDLP_asimdmisc_P   ', '                                      '];
T[' SADDLP   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H }               ', 'Q=0b1 size=0b01 ', 'SADDLP_asimdmisc_P   ', '                                      '];
T[' SADDLP   VREG={REG:s64=$Vd VDT=1D } VREG={REG:s32=$Vn VDT=2S }               ', 'Q=0b0 size=0b10 ', 'SADDLP_asimdmisc_P   ', '                                      '];
T[' SADDLP   VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S }               ', 'Q=0b1 size=0b10 ', 'SADDLP_asimdmisc_P   ', '                                      '];
T[' SCVTF    VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'SCVTF_asimdmisc_R    ', '                                      '];
T[' SCVTF    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'SCVTF_asimdmisc_R    ', '                                      '];
T[' SCVTF    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'SCVTF_asimdmisc_R    ', '                                      '];
T[' SHLL     VREG={REG:i16=$Vd VDT=8H } VREG={REG:i8=$Vn   VDT=8B } IMM:u8=8     ', 'Q=0b0 size=0b00 ', 'SHLL_asimdmisc_S     ', '                                      '];
T[' SHLL     VREG={REG:i32=$Vd VDT=4S } VREG={REG:i16=$Vn VDT=4H } IMM:u8=16     ', 'Q=0b0 size=0b01 ', 'SHLL_asimdmisc_S     ', '                                      '];
T[' SHLL     VREG={REG:i64=$Vd VDT=2D } VREG={REG:i32=$Vn VDT=2S } IMM:u8=32     ', 'Q=0b0 size=0b10 ', 'SHLL_asimdmisc_S     ', '                                      '];
T[' SHLL2    VREG={REG:i16=$Vd VDT=8H } VREG={REG:i8=$Vn   VDT=16B} IMM:u8=8     ', 'Q=0b1 size=0b00 ', 'SHLL_asimdmisc_S     ', '                                      '];
T[' SHLL2    VREG={REG:i32=$Vd VDT=4S } VREG={REG:i16=$Vn VDT=8H } IMM:u8=16     ', 'Q=0b1 size=0b01 ', 'SHLL_asimdmisc_S     ', '                                      '];
T[' SHLL2    VREG={REG:i64=$Vd VDT=2D } VREG={REG:i32=$Vn VDT=4S } IMM:u8=32     ', 'Q=0b1 size=0b10 ', 'SHLL_asimdmisc_S     ', '                                      '];
T[' SQABS    VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B }             ', 'Q=0b0 size=0b00 ', 'SQABS_asimdmisc_R    ', '                                      '];
T[' SQABS    VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B}             ', 'Q=0b1 size=0b00 ', 'SQABS_asimdmisc_R    ', '                                      '];
T[' SQABS    VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H }               ', 'Q=0b0 size=0b01 ', 'SQABS_asimdmisc_R    ', '                                      '];
T[' SQABS    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H }               ', 'Q=0b1 size=0b01 ', 'SQABS_asimdmisc_R    ', '                                      '];
T[' SQABS    VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S }               ', 'Q=0b0 size=0b10 ', 'SQABS_asimdmisc_R    ', '                                      '];
T[' SQABS    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S }               ', 'Q=0b1 size=0b10 ', 'SQABS_asimdmisc_R    ', '                                      '];
T[' SQABS    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D }               ', 'Q=0b1 size=0b11 ', 'SQABS_asimdmisc_R    ', '                                      '];
T[' SQNEG    VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B }             ', 'Q=0b0 size=0b00 ', 'SQNEG_asimdmisc_R    ', '                                      '];
T[' SQNEG    VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B}             ', 'Q=0b1 size=0b00 ', 'SQNEG_asimdmisc_R    ', '                                      '];
T[' SQNEG    VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H }               ', 'Q=0b0 size=0b01 ', 'SQNEG_asimdmisc_R    ', '                                      '];
T[' SQNEG    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H }               ', 'Q=0b1 size=0b01 ', 'SQNEG_asimdmisc_R    ', '                                      '];
T[' SQNEG    VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S }               ', 'Q=0b0 size=0b10 ', 'SQNEG_asimdmisc_R    ', '                                      '];
T[' SQNEG    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S }               ', 'Q=0b1 size=0b10 ', 'SQNEG_asimdmisc_R    ', '                                      '];
T[' SQNEG    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D }               ', 'Q=0b1 size=0b11 ', 'SQNEG_asimdmisc_R    ', '                                      '];
T[' SQXTN    VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s16=$Vn VDT=8H }              ', 'Q=0b0 size=0b00 ', 'SQXTN_asimdmisc_N    ', '                                      '];
T[' SQXTN    VREG={REG:s16=$Vd VDT=4H } VREG={REG:s32=$Vn VDT=4S }               ', 'Q=0b0 size=0b01 ', 'SQXTN_asimdmisc_N    ', '                                      '];
T[' SQXTN    VREG={REG:s32=$Vd VDT=2S } VREG={REG:s64=$Vn VDT=2D }               ', 'Q=0b0 size=0b10 ', 'SQXTN_asimdmisc_N    ', '                                      '];
T[' SQXTN2   VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s16=$Vn VDT=8H }              ', 'Q=0b1 size=0b00 ', 'SQXTN_asimdmisc_N    ', '                                      '];
T[' SQXTN2   VREG={REG:s16=$Vd VDT=8H } VREG={REG:s32=$Vn VDT=4S }               ', 'Q=0b1 size=0b01 ', 'SQXTN_asimdmisc_N    ', '                                      '];
T[' SQXTN2   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s64=$Vn VDT=2D }               ', 'Q=0b1 size=0b10 ', 'SQXTN_asimdmisc_N    ', '                                      '];
T[' SQXTUN   VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u16=$Vn VDT=8H }              ', 'Q=0b0 size=0b00 ', 'SQXTUN_asimdmisc_N   ', '                                      '];
T[' SQXTUN   VREG={REG:u16=$Vd VDT=4H } VREG={REG:u32=$Vn VDT=4S }               ', 'Q=0b0 size=0b01 ', 'SQXTUN_asimdmisc_N   ', '                                      '];
T[' SQXTUN   VREG={REG:u32=$Vd VDT=2S } VREG={REG:u64=$Vn VDT=2D }               ', 'Q=0b0 size=0b10 ', 'SQXTUN_asimdmisc_N   ', '                                      '];
T[' SQXTUN2  VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u16=$Vn VDT=8H }              ', 'Q=0b1 size=0b00 ', 'SQXTUN_asimdmisc_N   ', '                                      '];
T[' SQXTUN2  VREG={REG:u16=$Vd VDT=8H } VREG={REG:u32=$Vn VDT=4S }               ', 'Q=0b1 size=0b01 ', 'SQXTUN_asimdmisc_N   ', '                                      '];
T[' SQXTUN2  VREG={REG:u32=$Vd VDT=4S } VREG={REG:u64=$Vn VDT=2D }               ', 'Q=0b1 size=0b10 ', 'SQXTUN_asimdmisc_N   ', '                                      '];
T[' SUQADD   VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B }             ', 'Q=0b0 size=0b00 ', 'SUQADD_asimdmisc_R   ', '                                      '];
T[' SUQADD   VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B}             ', 'Q=0b1 size=0b00 ', 'SUQADD_asimdmisc_R   ', '                                      '];
T[' SUQADD   VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H }               ', 'Q=0b0 size=0b01 ', 'SUQADD_asimdmisc_R   ', '                                      '];
T[' SUQADD   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H }               ', 'Q=0b1 size=0b01 ', 'SUQADD_asimdmisc_R   ', '                                      '];
T[' SUQADD   VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S }               ', 'Q=0b0 size=0b10 ', 'SUQADD_asimdmisc_R   ', '                                      '];
T[' SUQADD   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S }               ', 'Q=0b1 size=0b10 ', 'SUQADD_asimdmisc_R   ', '                                      '];
T[' SUQADD   VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D }               ', 'Q=0b1 size=0b11 ', 'SUQADD_asimdmisc_R   ', '                                      '];
T[' UADALP   VREG={REG:u16=$Vd VDT=4H } VREG={REG:u8=$Vn   VDT=8B }              ', 'Q=0b0 size=0b00 ', 'UADALP_asimdmisc_P   ', '                                      '];
T[' UADALP   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=16B}              ', 'Q=0b1 size=0b00 ', 'UADALP_asimdmisc_P   ', '                                      '];
T[' UADALP   VREG={REG:u32=$Vd VDT=2S } VREG={REG:u16=$Vn VDT=4H }               ', 'Q=0b0 size=0b01 ', 'UADALP_asimdmisc_P   ', '                                      '];
T[' UADALP   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=8H }               ', 'Q=0b1 size=0b01 ', 'UADALP_asimdmisc_P   ', '                                      '];
T[' UADALP   VREG={REG:u64=$Vd VDT=1D } VREG={REG:u32=$Vn VDT=2S }               ', 'Q=0b0 size=0b10 ', 'UADALP_asimdmisc_P   ', '                                      '];
T[' UADALP   VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=4S }               ', 'Q=0b1 size=0b10 ', 'UADALP_asimdmisc_P   ', '                                      '];
T[' UADDLP   VREG={REG:u16=$Vd VDT=4H } VREG={REG:u8=$Vn   VDT=8B }              ', 'Q=0b0 size=0b00 ', 'UADDLP_asimdmisc_P   ', '                                      '];
T[' UADDLP   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=16B}              ', 'Q=0b1 size=0b00 ', 'UADDLP_asimdmisc_P   ', '                                      '];
T[' UADDLP   VREG={REG:u32=$Vd VDT=2S } VREG={REG:u16=$Vn VDT=4H }               ', 'Q=0b0 size=0b01 ', 'UADDLP_asimdmisc_P   ', '                                      '];
T[' UADDLP   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=8H }               ', 'Q=0b1 size=0b01 ', 'UADDLP_asimdmisc_P   ', '                                      '];
T[' UADDLP   VREG={REG:u64=$Vd VDT=1D } VREG={REG:u32=$Vn VDT=2S }               ', 'Q=0b0 size=0b10 ', 'UADDLP_asimdmisc_P   ', '                                      '];
T[' UADDLP   VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=4S }               ', 'Q=0b1 size=0b10 ', 'UADDLP_asimdmisc_P   ', '                                      '];
T[' UCVTF    VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'UCVTF_asimdmisc_R    ', '                                      '];
T[' UCVTF    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'UCVTF_asimdmisc_R    ', '                                      '];
T[' UCVTF    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D }               ', 'Q=0b1 sz=0b1    ', 'UCVTF_asimdmisc_R    ', '                                      '];
T[' UQXTN    VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u16=$Vn VDT=8H }              ', 'Q=0b0 size=0b00 ', 'UQXTN_asimdmisc_N    ', '                                      '];
T[' UQXTN    VREG={REG:u16=$Vd VDT=4H } VREG={REG:u32=$Vn VDT=4S }               ', 'Q=0b0 size=0b01 ', 'UQXTN_asimdmisc_N    ', '                                      '];
T[' UQXTN    VREG={REG:u32=$Vd VDT=2S } VREG={REG:u64=$Vn VDT=2D }               ', 'Q=0b0 size=0b10 ', 'UQXTN_asimdmisc_N    ', '                                      '];
T[' UQXTN2   VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u16=$Vn VDT=8H }              ', 'Q=0b1 size=0b00 ', 'UQXTN_asimdmisc_N    ', '                                      '];
T[' UQXTN2   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u32=$Vn VDT=4S }               ', 'Q=0b1 size=0b01 ', 'UQXTN_asimdmisc_N    ', '                                      '];
T[' UQXTN2   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u64=$Vn VDT=2D }               ', 'Q=0b1 size=0b10 ', 'UQXTN_asimdmisc_N    ', '                                      '];
T[' URECPE   VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'URECPE_asimdmisc_R   ', '                                      '];
T[' URECPE   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'URECPE_asimdmisc_R   ', '                                      '];
T[' URSQRTE  VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S }               ', 'Q=0b0 sz=0b0    ', 'URSQRTE_asimdmisc_R  ', '                                      '];
T[' URSQRTE  VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S }               ', 'Q=0b1 sz=0b0    ', 'URSQRTE_asimdmisc_R  ', '                                      '];
T[' USQADD   VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B }             ', 'Q=0b0 size=0b00 ', 'USQADD_asimdmisc_R   ', '                                      '];
T[' USQADD   VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B}             ', 'Q=0b1 size=0b00 ', 'USQADD_asimdmisc_R   ', '                                      '];
T[' USQADD   VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H }               ', 'Q=0b0 size=0b01 ', 'USQADD_asimdmisc_R   ', '                                      '];
T[' USQADD   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H }               ', 'Q=0b1 size=0b01 ', 'USQADD_asimdmisc_R   ', '                                      '];
T[' USQADD   VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S }               ', 'Q=0b0 size=0b10 ', 'USQADD_asimdmisc_R   ', '                                      '];
T[' USQADD   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S }               ', 'Q=0b1 size=0b10 ', 'USQADD_asimdmisc_R   ', '                                      '];
T[' USQADD   VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D }               ', 'Q=0b1 size=0b11 ', 'USQADD_asimdmisc_R   ', '                                      '];
T[' XTN      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i16=$Vn VDT=8H }              ', 'Q=0b0 size=0b00 ', 'XTN_asimdmisc_N      ', '                                      '];
T[' XTN      VREG={REG:i16=$Vd VDT=4H } VREG={REG:i32=$Vn VDT=4S }               ', 'Q=0b0 size=0b01 ', 'XTN_asimdmisc_N      ', '                                      '];
T[' XTN      VREG={REG:i32=$Vd VDT=2S } VREG={REG:i64=$Vn VDT=2D }               ', 'Q=0b0 size=0b10 ', 'XTN_asimdmisc_N      ', '                                      '];
T[' XTN2     VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i16=$Vn VDT=8H }              ', 'Q=0b1 size=0b00 ', 'XTN_asimdmisc_N      ', '                                      '];
T[' XTN2     VREG={REG:i16=$Vd VDT=8H } VREG={REG:i32=$Vn VDT=4S }               ', 'Q=0b1 size=0b01 ', 'XTN_asimdmisc_N      ', '                                      '];
T[' XTN2     VREG={REG:i32=$Vd VDT=4S } VREG={REG:i64=$Vn VDT=2D }               ', 'Q=0b1 size=0b10 ', 'XTN_asimdmisc_N      ', '                                      '];
T[' MVN      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B }             ', 'Q=0b0           ', 'MVN_NOT_asimdmisc_R  ', '                                      '];
T[' MVN      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B}             ', 'Q=0b1           ', 'MVN_NOT_asimdmisc_R  ', '                                      '];



# asimdmiscfp16: Advanced SIMD two-register miscellaneous (FP16).
# A64: ig0=0b0 Q:u=0bx U:u=0bx ig1=0b01110 a:u=0bx ig2=0b111100 opcode:u=0bxxxxx ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asimdmiscfp16', 'A64');

DEF Vd => VECREG('Vd', 'Rd', 'write') ;
DEF Vn => VECREG('Vn', 'Rn', 'read')  ;

T[' FABS    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FABS_asimdmiscfp16_R    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FABS    VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FABS_asimdmiscfp16_R    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMEQ   VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } FPIMM:f32=0.0 ', 'Q=0b0 ', 'FCMEQ_asimdmiscfp16_FZ  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMEQ   VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } FPIMM:f32=0.0 ', 'Q=0b1 ', 'FCMEQ_asimdmiscfp16_FZ  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMGE   VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } FPIMM:f32=0.0 ', 'Q=0b0 ', 'FCMGE_asimdmiscfp16_FZ  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMGE   VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } FPIMM:f32=0.0 ', 'Q=0b1 ', 'FCMGE_asimdmiscfp16_FZ  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMGT   VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } FPIMM:f32=0.0 ', 'Q=0b0 ', 'FCMGT_asimdmiscfp16_FZ  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMGT   VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } FPIMM:f32=0.0 ', 'Q=0b1 ', 'FCMGT_asimdmiscfp16_FZ  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMLE   VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } FPIMM:f32=0.0 ', 'Q=0b0 ', 'FCMLE_asimdmiscfp16_FZ  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMLE   VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } FPIMM:f32=0.0 ', 'Q=0b1 ', 'FCMLE_asimdmiscfp16_FZ  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMLT   VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } FPIMM:f32=0.0 ', 'Q=0b0 ', 'FCMLT_asimdmiscfp16_FZ  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMLT   VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } FPIMM:f32=0.0 ', 'Q=0b1 ', 'FCMLT_asimdmiscfp16_FZ  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTAS  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FCVTAS_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTAS  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FCVTAS_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTAU  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FCVTAU_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTAU  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FCVTAU_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTMS  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FCVTMS_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTMS  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FCVTMS_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTMU  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FCVTMU_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTMU  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FCVTMU_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTNS  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FCVTNS_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTNS  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FCVTNS_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTNU  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FCVTNU_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTNU  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FCVTNU_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTPS  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FCVTPS_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTPS  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FCVTPS_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTPU  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FCVTPU_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTPU  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FCVTPU_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTZS  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FCVTZS_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTZS  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FCVTZS_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTZU  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FCVTZU_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTZU  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FCVTZU_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FNEG    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FNEG_asimdmiscfp16_R    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FNEG    VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FNEG_asimdmiscfp16_R    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRECPE  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FRECPE_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRECPE  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FRECPE_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRINTA  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FRINTA_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRINTA  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FRINTA_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRINTI  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FRINTI_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRINTI  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FRINTI_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRINTM  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FRINTM_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRINTM  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FRINTM_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRINTN  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FRINTN_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRINTN  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FRINTN_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRINTP  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FRINTP_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRINTP  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FRINTP_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRINTX  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FRINTX_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRINTX  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FRINTX_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRINTZ  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FRINTZ_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRINTZ  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FRINTZ_asimdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRSQRTE VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FRSQRTE_asimdmiscfp16_R ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRSQRTE VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FRSQRTE_asimdmiscfp16_R ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FSQRT   VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H }               ', 'Q=0b0 ', 'FSQRT_asimdmiscfp16_R   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FSQRT   VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H }               ', 'Q=0b1 ', 'FSQRT_asimdmiscfp16_R   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' SCVTF   VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H }               ', 'Q=0b0 ', 'SCVTF_asimdmiscfp16_R   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' SCVTF   VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H }               ', 'Q=0b1 ', 'SCVTF_asimdmiscfp16_R   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' UCVTF   VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H }               ', 'Q=0b0 ', 'UCVTF_asimdmiscfp16_R   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' UCVTF   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H }               ', 'Q=0b1 ', 'UCVTF_asimdmiscfp16_R   ', 'arch_variant=ARMv8v2 extensions=FP16 '];



# asimdperm: Advanced SIMD permute.
# A64: ig0=0b0 Q:u=0bx ig1=0b001110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 opcode:u=0bxxx ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asimdperm', 'A64');

DEF Vd => VECREG('Vd', 'Rd', 'write') ;
DEF Vm => VECREG('Vm', 'Rm', 'read')  ;
DEF Vn => VECREG('Vn', 'Rn', 'read')  ;

T[' TRN1 VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'TRN1_asimdperm_only ', ' '];
T[' TRN1 VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'TRN1_asimdperm_only ', ' '];
T[' TRN1 VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'TRN1_asimdperm_only ', ' '];
T[' TRN1 VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'TRN1_asimdperm_only ', ' '];
T[' TRN1 VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } VREG={REG:i32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'TRN1_asimdperm_only ', ' '];
T[' TRN1 VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'TRN1_asimdperm_only ', ' '];
T[' TRN1 VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'TRN1_asimdperm_only ', ' '];
T[' TRN2 VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'TRN2_asimdperm_only ', ' '];
T[' TRN2 VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'TRN2_asimdperm_only ', ' '];
T[' TRN2 VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'TRN2_asimdperm_only ', ' '];
T[' TRN2 VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'TRN2_asimdperm_only ', ' '];
T[' TRN2 VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } VREG={REG:i32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'TRN2_asimdperm_only ', ' '];
T[' TRN2 VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'TRN2_asimdperm_only ', ' '];
T[' TRN2 VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'TRN2_asimdperm_only ', ' '];
T[' UZP1 VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UZP1_asimdperm_only ', ' '];
T[' UZP1 VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UZP1_asimdperm_only ', ' '];
T[' UZP1 VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'UZP1_asimdperm_only ', ' '];
T[' UZP1 VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'UZP1_asimdperm_only ', ' '];
T[' UZP1 VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } VREG={REG:i32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'UZP1_asimdperm_only ', ' '];
T[' UZP1 VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'UZP1_asimdperm_only ', ' '];
T[' UZP1 VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'UZP1_asimdperm_only ', ' '];
T[' UZP2 VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UZP2_asimdperm_only ', ' '];
T[' UZP2 VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UZP2_asimdperm_only ', ' '];
T[' UZP2 VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'UZP2_asimdperm_only ', ' '];
T[' UZP2 VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'UZP2_asimdperm_only ', ' '];
T[' UZP2 VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } VREG={REG:i32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'UZP2_asimdperm_only ', ' '];
T[' UZP2 VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'UZP2_asimdperm_only ', ' '];
T[' UZP2 VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'UZP2_asimdperm_only ', ' '];
T[' ZIP1 VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'ZIP1_asimdperm_only ', ' '];
T[' ZIP1 VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'ZIP1_asimdperm_only ', ' '];
T[' ZIP1 VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'ZIP1_asimdperm_only ', ' '];
T[' ZIP1 VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'ZIP1_asimdperm_only ', ' '];
T[' ZIP1 VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } VREG={REG:i32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'ZIP1_asimdperm_only ', ' '];
T[' ZIP1 VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'ZIP1_asimdperm_only ', ' '];
T[' ZIP1 VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'ZIP1_asimdperm_only ', ' '];
T[' ZIP2 VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'ZIP2_asimdperm_only ', ' '];
T[' ZIP2 VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'ZIP2_asimdperm_only ', ' '];
T[' ZIP2 VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'ZIP2_asimdperm_only ', ' '];
T[' ZIP2 VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'ZIP2_asimdperm_only ', ' '];
T[' ZIP2 VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } VREG={REG:i32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'ZIP2_asimdperm_only ', ' '];
T[' ZIP2 VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'ZIP2_asimdperm_only ', ' '];
T[' ZIP2 VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'ZIP2_asimdperm_only ', ' '];



# asimdsame: Advanced SIMD three same.
# A64: ig0=0b0 Q:u=0bx U:u=0bx ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode:u=0bxxxxx ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asimdsame', 'A64');

DEF Vd => VECREG('Vd', 'Rd', 'write') ;
DEF Vm => VECREG('Vm', 'Rm', 'read')  ;
DEF Vn => VECREG('Vn', 'Rn', 'read')  ;

T[' ADD      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'ADD_asimdsame_only      ', '                                                        '];
T[' ADD      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'ADD_asimdsame_only      ', '                                                        '];
T[' ADD      VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'ADD_asimdsame_only      ', '                                                        '];
T[' ADD      VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'ADD_asimdsame_only      ', '                                                        '];
T[' ADD      VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } VREG={REG:i32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'ADD_asimdsame_only      ', '                                                        '];
T[' ADD      VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'ADD_asimdsame_only      ', '                                                        '];
T[' ADD      VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'ADD_asimdsame_only      ', '                                                        '];
T[' ADDP     VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'ADDP_asimdsame_only     ', '                                                        '];
T[' ADDP     VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'ADDP_asimdsame_only     ', '                                                        '];
T[' ADDP     VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'ADDP_asimdsame_only     ', '                                                        '];
T[' ADDP     VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'ADDP_asimdsame_only     ', '                                                        '];
T[' ADDP     VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } VREG={REG:i32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'ADDP_asimdsame_only     ', '                                                        '];
T[' ADDP     VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'ADDP_asimdsame_only     ', '                                                        '];
T[' ADDP     VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'ADDP_asimdsame_only     ', '                                                        '];
T[' AND      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0           ', 'AND_asimdsame_only      ', '                                                        '];
T[' AND      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1           ', 'AND_asimdsame_only      ', '                                                        '];
T[' BIC      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0           ', 'BIC_asimdsame_only      ', '                                                        '];
T[' BIC      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1           ', 'BIC_asimdsame_only      ', '                                                        '];
T[' BIF      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0           ', 'BIF_asimdsame_only      ', '                                                        '];
T[' BIF      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1           ', 'BIF_asimdsame_only      ', '                                                        '];
T[' BIT      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0           ', 'BIT_asimdsame_only      ', '                                                        '];
T[' BIT      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1           ', 'BIT_asimdsame_only      ', '                                                        '];
T[' BSL      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0           ', 'BSL_asimdsame_only      ', '                                                        '];
T[' BSL      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1           ', 'BSL_asimdsame_only      ', '                                                        '];
T[' CMEQ     VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'CMEQ_asimdsame_only     ', '                                                        '];
T[' CMEQ     VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'CMEQ_asimdsame_only     ', '                                                        '];
T[' CMEQ     VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'CMEQ_asimdsame_only     ', '                                                        '];
T[' CMEQ     VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'CMEQ_asimdsame_only     ', '                                                        '];
T[' CMEQ     VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } VREG={REG:i32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'CMEQ_asimdsame_only     ', '                                                        '];
T[' CMEQ     VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'CMEQ_asimdsame_only     ', '                                                        '];
T[' CMEQ     VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'CMEQ_asimdsame_only     ', '                                                        '];
T[' CMGE     VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'CMGE_asimdsame_only     ', '                                                        '];
T[' CMGE     VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'CMGE_asimdsame_only     ', '                                                        '];
T[' CMGE     VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'CMGE_asimdsame_only     ', '                                                        '];
T[' CMGE     VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'CMGE_asimdsame_only     ', '                                                        '];
T[' CMGE     VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } VREG={REG:i32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'CMGE_asimdsame_only     ', '                                                        '];
T[' CMGE     VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'CMGE_asimdsame_only     ', '                                                        '];
T[' CMGE     VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'CMGE_asimdsame_only     ', '                                                        '];
T[' CMGT     VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'CMGT_asimdsame_only     ', '                                                        '];
T[' CMGT     VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'CMGT_asimdsame_only     ', '                                                        '];
T[' CMGT     VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'CMGT_asimdsame_only     ', '                                                        '];
T[' CMGT     VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'CMGT_asimdsame_only     ', '                                                        '];
T[' CMGT     VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } VREG={REG:i32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'CMGT_asimdsame_only     ', '                                                        '];
T[' CMGT     VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'CMGT_asimdsame_only     ', '                                                        '];
T[' CMGT     VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'CMGT_asimdsame_only     ', '                                                        '];
T[' CMHI     VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'CMHI_asimdsame_only     ', '                                                        '];
T[' CMHI     VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'CMHI_asimdsame_only     ', '                                                        '];
T[' CMHI     VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'CMHI_asimdsame_only     ', '                                                        '];
T[' CMHI     VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'CMHI_asimdsame_only     ', '                                                        '];
T[' CMHI     VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } VREG={REG:i32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'CMHI_asimdsame_only     ', '                                                        '];
T[' CMHI     VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'CMHI_asimdsame_only     ', '                                                        '];
T[' CMHI     VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'CMHI_asimdsame_only     ', '                                                        '];
T[' CMHS     VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'CMHS_asimdsame_only     ', '                                                        '];
T[' CMHS     VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'CMHS_asimdsame_only     ', '                                                        '];
T[' CMHS     VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'CMHS_asimdsame_only     ', '                                                        '];
T[' CMHS     VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'CMHS_asimdsame_only     ', '                                                        '];
T[' CMHS     VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'CMHS_asimdsame_only     ', '                                                        '];
T[' CMHS     VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'CMHS_asimdsame_only     ', '                                                        '];
T[' CMHS     VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D } VREG={REG:s64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'CMHS_asimdsame_only     ', '                                                        '];
T[' CMTST    VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'CMTST_asimdsame_only    ', '                                                        '];
T[' CMTST    VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'CMTST_asimdsame_only    ', '                                                        '];
T[' CMTST    VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'CMTST_asimdsame_only    ', '                                                        '];
T[' CMTST    VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'CMTST_asimdsame_only    ', '                                                        '];
T[' CMTST    VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } VREG={REG:i32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'CMTST_asimdsame_only    ', '                                                        '];
T[' CMTST    VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'CMTST_asimdsame_only    ', '                                                        '];
T[' CMTST    VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'CMTST_asimdsame_only    ', '                                                        '];
T[' EOR      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0           ', 'EOR_asimdsame_only      ', '                                                        '];
T[' EOR      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1           ', 'EOR_asimdsame_only      ', '                                                        '];
T[' FABD     VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FABD_asimdsame_only     ', '                                                        '];
T[' FABD     VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FABD_asimdsame_only     ', '                                                        '];
T[' FABD     VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FABD_asimdsame_only     ', '                                                        '];
T[' FACGE    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FACGE_asimdsame_only    ', '                                                        '];
T[' FACGE    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FACGE_asimdsame_only    ', '                                                        '];
T[' FACGE    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FACGE_asimdsame_only    ', '                                                        '];
T[' FACGT    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FACGT_asimdsame_only    ', '                                                        '];
T[' FACGT    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FACGT_asimdsame_only    ', '                                                        '];
T[' FACGT    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FACGT_asimdsame_only    ', '                                                        '];
T[' FADD     VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FADD_asimdsame_only     ', '                                                        '];
T[' FADD     VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FADD_asimdsame_only     ', '                                                        '];
T[' FADD     VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FADD_asimdsame_only     ', '                                                        '];
T[' FADDP    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FADDP_asimdsame_only    ', '                                                        '];
T[' FADDP    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FADDP_asimdsame_only    ', '                                                        '];
T[' FADDP    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FADDP_asimdsame_only    ', '                                                        '];
T[' FCMEQ    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FCMEQ_asimdsame_only    ', '                                                        '];
T[' FCMEQ    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FCMEQ_asimdsame_only    ', '                                                        '];
T[' FCMEQ    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FCMEQ_asimdsame_only    ', '                                                        '];
T[' FCMGE    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FCMGE_asimdsame_only    ', '                                                        '];
T[' FCMGE    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FCMGE_asimdsame_only    ', '                                                        '];
T[' FCMGE    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FCMGE_asimdsame_only    ', '                                                        '];
T[' FCMGT    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FCMGT_asimdsame_only    ', '                                                        '];
T[' FCMGT    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FCMGT_asimdsame_only    ', '                                                        '];
T[' FCMGT    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FCMGT_asimdsame_only    ', '                                                        '];
T[' FDIV     VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FDIV_asimdsame_only     ', '                                                        '];
T[' FDIV     VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FDIV_asimdsame_only     ', '                                                        '];
T[' FDIV     VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FDIV_asimdsame_only     ', '                                                        '];
T[' FMAX     VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FMAX_asimdsame_only     ', '                                                        '];
T[' FMAX     VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FMAX_asimdsame_only     ', '                                                        '];
T[' FMAX     VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FMAX_asimdsame_only     ', '                                                        '];
T[' FMAXNM   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FMAXNM_asimdsame_only   ', '                                                        '];
T[' FMAXNM   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FMAXNM_asimdsame_only   ', '                                                        '];
T[' FMAXNM   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FMAXNM_asimdsame_only   ', '                                                        '];
T[' FMAXNMP  VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FMAXNMP_asimdsame_only  ', '                                                        '];
T[' FMAXNMP  VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FMAXNMP_asimdsame_only  ', '                                                        '];
T[' FMAXNMP  VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FMAXNMP_asimdsame_only  ', '                                                        '];
T[' FMAXP    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FMAXP_asimdsame_only    ', '                                                        '];
T[' FMAXP    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FMAXP_asimdsame_only    ', '                                                        '];
T[' FMAXP    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FMAXP_asimdsame_only    ', '                                                        '];
T[' FMIN     VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FMIN_asimdsame_only     ', '                                                        '];
T[' FMIN     VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FMIN_asimdsame_only     ', '                                                        '];
T[' FMIN     VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FMIN_asimdsame_only     ', '                                                        '];
T[' FMINNM   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FMINNM_asimdsame_only   ', '                                                        '];
T[' FMINNM   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FMINNM_asimdsame_only   ', '                                                        '];
T[' FMINNM   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FMINNM_asimdsame_only   ', '                                                        '];
T[' FMINNMP  VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FMINNMP_asimdsame_only  ', '                                                        '];
T[' FMINNMP  VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FMINNMP_asimdsame_only  ', '                                                        '];
T[' FMINNMP  VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FMINNMP_asimdsame_only  ', '                                                        '];
T[' FMINP    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FMINP_asimdsame_only    ', '                                                        '];
T[' FMINP    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FMINP_asimdsame_only    ', '                                                        '];
T[' FMINP    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FMINP_asimdsame_only    ', '                                                        '];
T[' FMLA     VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FMLA_asimdsame_only     ', '                                                        '];
T[' FMLA     VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FMLA_asimdsame_only     ', '                                                        '];
T[' FMLA     VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FMLA_asimdsame_only     ', '                                                        '];
T[' FMLAL    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f16=$Vn VDT=2H } VREG={REG:f16=$Vm VDT=2H }    ', 'Q=0b0           ', 'FMLAL_asimdsame_F       ', 'arch_variant=ARMv8v2 extensions=FP16MULNOROUNDINGTOFP32 '];
T[' FMLAL    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H }    ', 'Q=0b1           ', 'FMLAL_asimdsame_F       ', 'arch_variant=ARMv8v2 extensions=FP16MULNOROUNDINGTOFP32 '];
T[' FMLAL2   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f16=$Vn VDT=2H } VREG={REG:f16=$Vm VDT=2H }    ', 'Q=0b0           ', 'FMLAL2_asimdsame_F      ', 'arch_variant=ARMv8v2 extensions=FP16MULNOROUNDINGTOFP32 '];
T[' FMLAL2   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H }    ', 'Q=0b1           ', 'FMLAL2_asimdsame_F      ', 'arch_variant=ARMv8v2 extensions=FP16MULNOROUNDINGTOFP32 '];
T[' FMLS     VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FMLS_asimdsame_only     ', '                                                        '];
T[' FMLS     VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FMLS_asimdsame_only     ', '                                                        '];
T[' FMLS     VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FMLS_asimdsame_only     ', '                                                        '];
T[' FMLSL    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f16=$Vn VDT=2H } VREG={REG:f16=$Vm VDT=2H }    ', 'Q=0b0           ', 'FMLSL_asimdsame_F       ', 'arch_variant=ARMv8v2 extensions=FP16MULNOROUNDINGTOFP32 '];
T[' FMLSL    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H }    ', 'Q=0b1           ', 'FMLSL_asimdsame_F       ', 'arch_variant=ARMv8v2 extensions=FP16MULNOROUNDINGTOFP32 '];
T[' FMLSL2   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f16=$Vn VDT=2H } VREG={REG:f16=$Vm VDT=2H }    ', 'Q=0b0           ', 'FMLSL2_asimdsame_F      ', 'arch_variant=ARMv8v2 extensions=FP16MULNOROUNDINGTOFP32 '];
T[' FMLSL2   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H }    ', 'Q=0b1           ', 'FMLSL2_asimdsame_F      ', 'arch_variant=ARMv8v2 extensions=FP16MULNOROUNDINGTOFP32 '];
T[' FMUL     VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FMUL_asimdsame_only     ', '                                                        '];
T[' FMUL     VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FMUL_asimdsame_only     ', '                                                        '];
T[' FMUL     VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FMUL_asimdsame_only     ', '                                                        '];
T[' FMULX    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FMULX_asimdsame_only    ', '                                                        '];
T[' FMULX    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FMULX_asimdsame_only    ', '                                                        '];
T[' FMULX    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FMULX_asimdsame_only    ', '                                                        '];
T[' FRECPS   VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FRECPS_asimdsame_only   ', '                                                        '];
T[' FRECPS   VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FRECPS_asimdsame_only   ', '                                                        '];
T[' FRECPS   VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FRECPS_asimdsame_only   ', '                                                        '];
T[' FRSQRTS  VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FRSQRTS_asimdsame_only  ', '                                                        '];
T[' FRSQRTS  VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FRSQRTS_asimdsame_only  ', '                                                        '];
T[' FRSQRTS  VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FRSQRTS_asimdsame_only  ', '                                                        '];
T[' FSUB     VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S }    ', 'Q=0b0 sz=0b0    ', 'FSUB_asimdsame_only     ', '                                                        '];
T[' FSUB     VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S }    ', 'Q=0b1 sz=0b0    ', 'FSUB_asimdsame_only     ', '                                                        '];
T[' FSUB     VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D }    ', 'Q=0b1 sz=0b1    ', 'FSUB_asimdsame_only     ', '                                                        '];
T[' MLA      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'MLA_asimdsame_only      ', '                                                        '];
T[' MLA      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'MLA_asimdsame_only      ', '                                                        '];
T[' MLA      VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'MLA_asimdsame_only      ', '                                                        '];
T[' MLA      VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'MLA_asimdsame_only      ', '                                                        '];
T[' MLA      VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } VREG={REG:i32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'MLA_asimdsame_only      ', '                                                        '];
T[' MLA      VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'MLA_asimdsame_only      ', '                                                        '];
T[' MLS      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'MLS_asimdsame_only      ', '                                                        '];
T[' MLS      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'MLS_asimdsame_only      ', '                                                        '];
T[' MLS      VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'MLS_asimdsame_only      ', '                                                        '];
T[' MLS      VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'MLS_asimdsame_only      ', '                                                        '];
T[' MLS      VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } VREG={REG:i32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'MLS_asimdsame_only      ', '                                                        '];
T[' MLS      VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'MLS_asimdsame_only      ', '                                                        '];
T[' MUL      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'MUL_asimdsame_only      ', '                                                        '];
T[' MUL      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'MUL_asimdsame_only      ', '                                                        '];
T[' MUL      VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'MUL_asimdsame_only      ', '                                                        '];
T[' MUL      VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'MUL_asimdsame_only      ', '                                                        '];
T[' MUL      VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } VREG={REG:i32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'MUL_asimdsame_only      ', '                                                        '];
T[' MUL      VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'MUL_asimdsame_only      ', '                                                        '];
T[' ORN      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0           ', 'ORN_asimdsame_only      ', '                                                        '];
T[' ORN      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1           ', 'ORN_asimdsame_only      ', '                                                        '];
T[' ORR      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0           ', 'ORR_asimdsame_only      ', '                                                        '];
T[' ORR      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1           ', 'ORR_asimdsame_only      ', '                                                        '];
T[' PMUL     VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'PMUL_asimdsame_only     ', '                                                        '];
T[' PMUL     VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'PMUL_asimdsame_only     ', '                                                        '];
T[' SABA     VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SABA_asimdsame_only     ', '                                                        '];
T[' SABA     VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SABA_asimdsame_only     ', '                                                        '];
T[' SABA     VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SABA_asimdsame_only     ', '                                                        '];
T[' SABA     VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SABA_asimdsame_only     ', '                                                        '];
T[' SABA     VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SABA_asimdsame_only     ', '                                                        '];
T[' SABA     VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SABA_asimdsame_only     ', '                                                        '];
T[' SABD     VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SABD_asimdsame_only     ', '                                                        '];
T[' SABD     VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SABD_asimdsame_only     ', '                                                        '];
T[' SABD     VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SABD_asimdsame_only     ', '                                                        '];
T[' SABD     VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SABD_asimdsame_only     ', '                                                        '];
T[' SABD     VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SABD_asimdsame_only     ', '                                                        '];
T[' SABD     VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SABD_asimdsame_only     ', '                                                        '];
T[' SHADD    VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SHADD_asimdsame_only    ', '                                                        '];
T[' SHADD    VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SHADD_asimdsame_only    ', '                                                        '];
T[' SHADD    VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SHADD_asimdsame_only    ', '                                                        '];
T[' SHADD    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SHADD_asimdsame_only    ', '                                                        '];
T[' SHADD    VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SHADD_asimdsame_only    ', '                                                        '];
T[' SHADD    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SHADD_asimdsame_only    ', '                                                        '];
T[' SHSUB    VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SHSUB_asimdsame_only    ', '                                                        '];
T[' SHSUB    VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SHSUB_asimdsame_only    ', '                                                        '];
T[' SHSUB    VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SHSUB_asimdsame_only    ', '                                                        '];
T[' SHSUB    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SHSUB_asimdsame_only    ', '                                                        '];
T[' SHSUB    VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SHSUB_asimdsame_only    ', '                                                        '];
T[' SHSUB    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SHSUB_asimdsame_only    ', '                                                        '];
T[' SMAX     VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SMAX_asimdsame_only     ', '                                                        '];
T[' SMAX     VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SMAX_asimdsame_only     ', '                                                        '];
T[' SMAX     VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SMAX_asimdsame_only     ', '                                                        '];
T[' SMAX     VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SMAX_asimdsame_only     ', '                                                        '];
T[' SMAX     VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SMAX_asimdsame_only     ', '                                                        '];
T[' SMAX     VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SMAX_asimdsame_only     ', '                                                        '];
T[' SMAXP    VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SMAXP_asimdsame_only    ', '                                                        '];
T[' SMAXP    VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SMAXP_asimdsame_only    ', '                                                        '];
T[' SMAXP    VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SMAXP_asimdsame_only    ', '                                                        '];
T[' SMAXP    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SMAXP_asimdsame_only    ', '                                                        '];
T[' SMAXP    VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SMAXP_asimdsame_only    ', '                                                        '];
T[' SMAXP    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SMAXP_asimdsame_only    ', '                                                        '];
T[' SMIN     VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SMIN_asimdsame_only     ', '                                                        '];
T[' SMIN     VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SMIN_asimdsame_only     ', '                                                        '];
T[' SMIN     VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SMIN_asimdsame_only     ', '                                                        '];
T[' SMIN     VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SMIN_asimdsame_only     ', '                                                        '];
T[' SMIN     VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SMIN_asimdsame_only     ', '                                                        '];
T[' SMIN     VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SMIN_asimdsame_only     ', '                                                        '];
T[' SMINP    VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SMINP_asimdsame_only    ', '                                                        '];
T[' SMINP    VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SMINP_asimdsame_only    ', '                                                        '];
T[' SMINP    VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SMINP_asimdsame_only    ', '                                                        '];
T[' SMINP    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SMINP_asimdsame_only    ', '                                                        '];
T[' SMINP    VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SMINP_asimdsame_only    ', '                                                        '];
T[' SMINP    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SMINP_asimdsame_only    ', '                                                        '];
T[' SQADD    VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SQADD_asimdsame_only    ', '                                                        '];
T[' SQADD    VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SQADD_asimdsame_only    ', '                                                        '];
T[' SQADD    VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SQADD_asimdsame_only    ', '                                                        '];
T[' SQADD    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SQADD_asimdsame_only    ', '                                                        '];
T[' SQADD    VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SQADD_asimdsame_only    ', '                                                        '];
T[' SQADD    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SQADD_asimdsame_only    ', '                                                        '];
T[' SQADD    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D } VREG={REG:s64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'SQADD_asimdsame_only    ', '                                                        '];
T[' SQDMULH  VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SQDMULH_asimdsame_only  ', '                                                        '];
T[' SQDMULH  VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SQDMULH_asimdsame_only  ', '                                                        '];
T[' SQDMULH  VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SQDMULH_asimdsame_only  ', '                                                        '];
T[' SQDMULH  VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SQDMULH_asimdsame_only  ', '                                                        '];
T[' SQRDMULH VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SQRDMULH_asimdsame_only ', '                                                        '];
T[' SQRDMULH VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SQRDMULH_asimdsame_only ', '                                                        '];
T[' SQRDMULH VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SQRDMULH_asimdsame_only ', '                                                        '];
T[' SQRDMULH VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SQRDMULH_asimdsame_only ', '                                                        '];
T[' SQRSHL   VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SQRSHL_asimdsame_only   ', '                                                        '];
T[' SQRSHL   VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SQRSHL_asimdsame_only   ', '                                                        '];
T[' SQRSHL   VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SQRSHL_asimdsame_only   ', '                                                        '];
T[' SQRSHL   VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SQRSHL_asimdsame_only   ', '                                                        '];
T[' SQRSHL   VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SQRSHL_asimdsame_only   ', '                                                        '];
T[' SQRSHL   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SQRSHL_asimdsame_only   ', '                                                        '];
T[' SQRSHL   VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D } VREG={REG:s64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'SQRSHL_asimdsame_only   ', '                                                        '];
T[' SQSHL    VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SQSHL_asimdsame_only    ', '                                                        '];
T[' SQSHL    VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SQSHL_asimdsame_only    ', '                                                        '];
T[' SQSHL    VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SQSHL_asimdsame_only    ', '                                                        '];
T[' SQSHL    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SQSHL_asimdsame_only    ', '                                                        '];
T[' SQSHL    VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SQSHL_asimdsame_only    ', '                                                        '];
T[' SQSHL    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SQSHL_asimdsame_only    ', '                                                        '];
T[' SQSHL    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D } VREG={REG:s64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'SQSHL_asimdsame_only    ', '                                                        '];
T[' SQSUB    VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SQSUB_asimdsame_only    ', '                                                        '];
T[' SQSUB    VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SQSUB_asimdsame_only    ', '                                                        '];
T[' SQSUB    VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SQSUB_asimdsame_only    ', '                                                        '];
T[' SQSUB    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SQSUB_asimdsame_only    ', '                                                        '];
T[' SQSUB    VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SQSUB_asimdsame_only    ', '                                                        '];
T[' SQSUB    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SQSUB_asimdsame_only    ', '                                                        '];
T[' SQSUB    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D } VREG={REG:s64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'SQSUB_asimdsame_only    ', '                                                        '];
T[' SRHADD   VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SRHADD_asimdsame_only   ', '                                                        '];
T[' SRHADD   VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SRHADD_asimdsame_only   ', '                                                        '];
T[' SRHADD   VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SRHADD_asimdsame_only   ', '                                                        '];
T[' SRHADD   VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SRHADD_asimdsame_only   ', '                                                        '];
T[' SRHADD   VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SRHADD_asimdsame_only   ', '                                                        '];
T[' SRHADD   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SRHADD_asimdsame_only   ', '                                                        '];
T[' SRSHL    VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SRSHL_asimdsame_only    ', '                                                        '];
T[' SRSHL    VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SRSHL_asimdsame_only    ', '                                                        '];
T[' SRSHL    VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SRSHL_asimdsame_only    ', '                                                        '];
T[' SRSHL    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SRSHL_asimdsame_only    ', '                                                        '];
T[' SRSHL    VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SRSHL_asimdsame_only    ', '                                                        '];
T[' SRSHL    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SRSHL_asimdsame_only    ', '                                                        '];
T[' SRSHL    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D } VREG={REG:s64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'SRSHL_asimdsame_only    ', '                                                        '];
T[' SSHL     VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SSHL_asimdsame_only     ', '                                                        '];
T[' SSHL     VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SSHL_asimdsame_only     ', '                                                        '];
T[' SSHL     VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SSHL_asimdsame_only     ', '                                                        '];
T[' SSHL     VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SSHL_asimdsame_only     ', '                                                        '];
T[' SSHL     VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SSHL_asimdsame_only     ', '                                                        '];
T[' SSHL     VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SSHL_asimdsame_only     ', '                                                        '];
T[' SSHL     VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D } VREG={REG:s64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'SSHL_asimdsame_only     ', '                                                        '];
T[' SUB      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'SUB_asimdsame_only      ', '                                                        '];
T[' SUB      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'SUB_asimdsame_only      ', '                                                        '];
T[' SUB      VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'SUB_asimdsame_only      ', '                                                        '];
T[' SUB      VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'SUB_asimdsame_only      ', '                                                        '];
T[' SUB      VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } VREG={REG:i32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'SUB_asimdsame_only      ', '                                                        '];
T[' SUB      VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'SUB_asimdsame_only      ', '                                                        '];
T[' SUB      VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'SUB_asimdsame_only      ', '                                                        '];
T[' UABA     VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UABA_asimdsame_only     ', '                                                        '];
T[' UABA     VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UABA_asimdsame_only     ', '                                                        '];
T[' UABA     VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'UABA_asimdsame_only     ', '                                                        '];
T[' UABA     VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'UABA_asimdsame_only     ', '                                                        '];
T[' UABA     VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'UABA_asimdsame_only     ', '                                                        '];
T[' UABA     VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'UABA_asimdsame_only     ', '                                                        '];
T[' UABD     VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UABD_asimdsame_only     ', '                                                        '];
T[' UABD     VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UABD_asimdsame_only     ', '                                                        '];
T[' UABD     VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'UABD_asimdsame_only     ', '                                                        '];
T[' UABD     VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'UABD_asimdsame_only     ', '                                                        '];
T[' UABD     VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'UABD_asimdsame_only     ', '                                                        '];
T[' UABD     VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'UABD_asimdsame_only     ', '                                                        '];
T[' UHADD    VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UHADD_asimdsame_only    ', '                                                        '];
T[' UHADD    VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UHADD_asimdsame_only    ', '                                                        '];
T[' UHADD    VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'UHADD_asimdsame_only    ', '                                                        '];
T[' UHADD    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'UHADD_asimdsame_only    ', '                                                        '];
T[' UHADD    VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'UHADD_asimdsame_only    ', '                                                        '];
T[' UHADD    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'UHADD_asimdsame_only    ', '                                                        '];
T[' UHSUB    VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UHSUB_asimdsame_only    ', '                                                        '];
T[' UHSUB    VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UHSUB_asimdsame_only    ', '                                                        '];
T[' UHSUB    VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'UHSUB_asimdsame_only    ', '                                                        '];
T[' UHSUB    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'UHSUB_asimdsame_only    ', '                                                        '];
T[' UHSUB    VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'UHSUB_asimdsame_only    ', '                                                        '];
T[' UHSUB    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'UHSUB_asimdsame_only    ', '                                                        '];
T[' UMAX     VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UMAX_asimdsame_only     ', '                                                        '];
T[' UMAX     VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UMAX_asimdsame_only     ', '                                                        '];
T[' UMAX     VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'UMAX_asimdsame_only     ', '                                                        '];
T[' UMAX     VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'UMAX_asimdsame_only     ', '                                                        '];
T[' UMAX     VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'UMAX_asimdsame_only     ', '                                                        '];
T[' UMAX     VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'UMAX_asimdsame_only     ', '                                                        '];
T[' UMAXP    VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UMAXP_asimdsame_only    ', '                                                        '];
T[' UMAXP    VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UMAXP_asimdsame_only    ', '                                                        '];
T[' UMAXP    VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'UMAXP_asimdsame_only    ', '                                                        '];
T[' UMAXP    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'UMAXP_asimdsame_only    ', '                                                        '];
T[' UMAXP    VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'UMAXP_asimdsame_only    ', '                                                        '];
T[' UMAXP    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'UMAXP_asimdsame_only    ', '                                                        '];
T[' UMIN     VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UMIN_asimdsame_only     ', '                                                        '];
T[' UMIN     VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UMIN_asimdsame_only     ', '                                                        '];
T[' UMIN     VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'UMIN_asimdsame_only     ', '                                                        '];
T[' UMIN     VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'UMIN_asimdsame_only     ', '                                                        '];
T[' UMIN     VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'UMIN_asimdsame_only     ', '                                                        '];
T[' UMIN     VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'UMIN_asimdsame_only     ', '                                                        '];
T[' UMINP    VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UMINP_asimdsame_only    ', '                                                        '];
T[' UMINP    VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UMINP_asimdsame_only    ', '                                                        '];
T[' UMINP    VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'UMINP_asimdsame_only    ', '                                                        '];
T[' UMINP    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'UMINP_asimdsame_only    ', '                                                        '];
T[' UMINP    VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'UMINP_asimdsame_only    ', '                                                        '];
T[' UMINP    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'UMINP_asimdsame_only    ', '                                                        '];
T[' UQADD    VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UQADD_asimdsame_only    ', '                                                        '];
T[' UQADD    VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UQADD_asimdsame_only    ', '                                                        '];
T[' UQADD    VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'UQADD_asimdsame_only    ', '                                                        '];
T[' UQADD    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'UQADD_asimdsame_only    ', '                                                        '];
T[' UQADD    VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'UQADD_asimdsame_only    ', '                                                        '];
T[' UQADD    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'UQADD_asimdsame_only    ', '                                                        '];
T[' UQADD    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D } VREG={REG:u64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'UQADD_asimdsame_only    ', '                                                        '];
T[' UQRSHL   VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UQRSHL_asimdsame_only   ', '                                                        '];
T[' UQRSHL   VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UQRSHL_asimdsame_only   ', '                                                        '];
T[' UQRSHL   VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'UQRSHL_asimdsame_only   ', '                                                        '];
T[' UQRSHL   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'UQRSHL_asimdsame_only   ', '                                                        '];
T[' UQRSHL   VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'UQRSHL_asimdsame_only   ', '                                                        '];
T[' UQRSHL   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'UQRSHL_asimdsame_only   ', '                                                        '];
T[' UQRSHL   VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D } VREG={REG:u64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'UQRSHL_asimdsame_only   ', '                                                        '];
T[' UQSHL    VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UQSHL_asimdsame_only    ', '                                                        '];
T[' UQSHL    VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UQSHL_asimdsame_only    ', '                                                        '];
T[' UQSHL    VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'UQSHL_asimdsame_only    ', '                                                        '];
T[' UQSHL    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'UQSHL_asimdsame_only    ', '                                                        '];
T[' UQSHL    VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'UQSHL_asimdsame_only    ', '                                                        '];
T[' UQSHL    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'UQSHL_asimdsame_only    ', '                                                        '];
T[' UQSHL    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D } VREG={REG:u64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'UQSHL_asimdsame_only    ', '                                                        '];
T[' UQSUB    VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'UQSUB_asimdsame_only    ', '                                                        '];
T[' UQSUB    VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'UQSUB_asimdsame_only    ', '                                                        '];
T[' UQSUB    VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'UQSUB_asimdsame_only    ', '                                                        '];
T[' UQSUB    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'UQSUB_asimdsame_only    ', '                                                        '];
T[' UQSUB    VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'UQSUB_asimdsame_only    ', '                                                        '];
T[' UQSUB    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'UQSUB_asimdsame_only    ', '                                                        '];
T[' UQSUB    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D } VREG={REG:u64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'UQSUB_asimdsame_only    ', '                                                        '];
T[' URHADD   VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'URHADD_asimdsame_only   ', '                                                        '];
T[' URHADD   VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'URHADD_asimdsame_only   ', '                                                        '];
T[' URHADD   VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'URHADD_asimdsame_only   ', '                                                        '];
T[' URHADD   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'URHADD_asimdsame_only   ', '                                                        '];
T[' URHADD   VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'URHADD_asimdsame_only   ', '                                                        '];
T[' URHADD   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'URHADD_asimdsame_only   ', '                                                        '];
T[' URSHL    VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'URSHL_asimdsame_only    ', '                                                        '];
T[' URSHL    VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'URSHL_asimdsame_only    ', '                                                        '];
T[' URSHL    VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'URSHL_asimdsame_only    ', '                                                        '];
T[' URSHL    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'URSHL_asimdsame_only    ', '                                                        '];
T[' URSHL    VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'URSHL_asimdsame_only    ', '                                                        '];
T[' URSHL    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'URSHL_asimdsame_only    ', '                                                        '];
T[' URSHL    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D } VREG={REG:u64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'URSHL_asimdsame_only    ', '                                                        '];
T[' USHL     VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B } ', 'Q=0b0 size=0b00 ', 'USHL_asimdsame_only     ', '                                                        '];
T[' USHL     VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B} ', 'Q=0b1 size=0b00 ', 'USHL_asimdsame_only     ', '                                                        '];
T[' USHL     VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } VREG={REG:u16=$Vm VDT=4H }    ', 'Q=0b0 size=0b01 ', 'USHL_asimdsame_only     ', '                                                        '];
T[' USHL     VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } VREG={REG:u16=$Vm VDT=8H }    ', 'Q=0b1 size=0b01 ', 'USHL_asimdsame_only     ', '                                                        '];
T[' USHL     VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } VREG={REG:u32=$Vm VDT=2S }    ', 'Q=0b0 size=0b10 ', 'USHL_asimdsame_only     ', '                                                        '];
T[' USHL     VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } VREG={REG:u32=$Vm VDT=4S }    ', 'Q=0b1 size=0b10 ', 'USHL_asimdsame_only     ', '                                                        '];
T[' USHL     VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D } VREG={REG:u64=$Vm VDT=2D }    ', 'Q=0b1 size=0b11 ', 'USHL_asimdsame_only     ', '                                                        '];
T[' MOV      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B }                             ', 'Q=0b0           ', 'MOV_ORR_asimdsame_only  ', '                                                        '];
T[' MOV      VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B}                             ', 'Q=0b1           ', 'MOV_ORR_asimdsame_only  ', '                                                        '];



# asimdsame2: Advanced SIMD three-register extension.
# A64: ig0=0b0 Q:u=0bx U:u=0bx ig1=0b01110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 opcode:u=0bxxxx ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asimdsame2', 'A64');

DEF Vd      => VECREG('Vd', 'Rd', 'write')               ;
DEF Vm      => VECREG('Vm', 'Rm', 'read')                ;
DEF Vn      => VECREG('Vn', 'Rn', 'read')                ;
DEF rotate  => IMM('rotate', 'rot', table => 'tbl_imm4') ;
DEF rotate2 => IMM('rotate', 'rot', table => 'tbl_imm0') ;

T[' BFDOT    VREG={REG:i32=$Vd VDT=2S } VREG={REG:i16=$Vn VDT=4H } VREG={REG:i16=$Vm VDT=4H }                  ', 'Q=0b0           ', 'BFDOT_asimdsame2_D       ', 'arch_variant=ARMv8v6 extensions=BF16       '];
T[' BFDOT    VREG={REG:i32=$Vd VDT=4S } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }                  ', 'Q=0b1           ', 'BFDOT_asimdsame2_D       ', 'arch_variant=ARMv8v6 extensions=BF16       '];
T[' BFMLALB  VREG={REG:i32=$Vd VDT=4S } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }                  ', 'Q=0b0           ', 'BFMLAL_asimdsame2_F_     ', 'arch_variant=ARMv8v6 extensions=BF16       '];
T[' BFMLALT  VREG={REG:i32=$Vd VDT=4S } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }                  ', 'Q=0b1           ', 'BFMLAL_asimdsame2_F_     ', 'arch_variant=ARMv8v6 extensions=BF16       '];
T[' BFMMLA   VREG={REG:i32=$Vd VDT=4S } VREG={REG:i16=$Vn VDT=8H } VREG={REG:i16=$Vm VDT=8H }                  ', 'NONE            ', 'BFMMLA_asimdsame2_E      ', 'arch_variant=ARMv8v6 extensions=BF16       '];
T[' FCADD    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } IMM:i16=$rotate  ', 'Q=0b0 size=0b01 ', 'FCADD_asimdsame2_C       ', 'arch_variant=ARMv8v3 extensions=FCADD|FP16 '];
T[' FCADD    VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } IMM:i16=$rotate  ', 'Q=0b1 size=0b01 ', 'FCADD_asimdsame2_C       ', 'arch_variant=ARMv8v3 extensions=FCADD|FP16 '];
T[' FCADD    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S } IMM:i32=$rotate  ', 'Q=0b0 size=0b10 ', 'FCADD_asimdsame2_C       ', 'arch_variant=ARMv8v3 extensions=FCADD|FP16 '];
T[' FCADD    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S } IMM:i32=$rotate  ', 'Q=0b1 size=0b10 ', 'FCADD_asimdsame2_C       ', 'arch_variant=ARMv8v3 extensions=FCADD|FP16 '];
T[' FCADD    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D } IMM:i64=$rotate  ', 'Q=0b1 size=0b11 ', 'FCADD_asimdsame2_C       ', 'arch_variant=ARMv8v3 extensions=FCADD|FP16 '];
T[' FCMLA    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } IMM:i16=$rotate2 ', 'Q=0b0 size=0b01 ', 'FCMLA_asimdsame2_C       ', 'arch_variant=ARMv8v3 extensions=FCADD|FP16 '];
T[' FCMLA    VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } IMM:i16=$rotate2 ', 'Q=0b1 size=0b01 ', 'FCMLA_asimdsame2_C       ', 'arch_variant=ARMv8v3 extensions=FCADD|FP16 '];
T[' FCMLA    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } VREG={REG:f32=$Vm VDT=2S } IMM:i32=$rotate2 ', 'Q=0b0 size=0b10 ', 'FCMLA_asimdsame2_C       ', 'arch_variant=ARMv8v3 extensions=FCADD|FP16 '];
T[' FCMLA    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } VREG={REG:f32=$Vm VDT=4S } IMM:i32=$rotate2 ', 'Q=0b1 size=0b10 ', 'FCMLA_asimdsame2_C       ', 'arch_variant=ARMv8v3 extensions=FCADD|FP16 '];
T[' FCMLA    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } VREG={REG:f64=$Vm VDT=2D } IMM:i64=$rotate2 ', 'Q=0b1 size=0b11 ', 'FCMLA_asimdsame2_C       ', 'arch_variant=ARMv8v3 extensions=FCADD|FP16 '];
T[' SDOT     VREG={REG:s32=$Vd VDT=2S } VREG={REG:s8=$Vn   VDT=8B } VREG={REG:s8=$Vm   VDT=8B }                ', 'Q=0b0           ', 'SDOT_asimdsame2_D        ', 'arch_variant=ARMv8v2 extensions=DOTP       '];
T[' SDOT     VREG={REG:s32=$Vd VDT=4S } VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B}                ', 'Q=0b1           ', 'SDOT_asimdsame2_D        ', 'arch_variant=ARMv8v2 extensions=DOTP       '];
T[' SMMLA    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s8=$Vn   VDT=16B} VREG={REG:s8=$Vm   VDT=16B}                ', 'NONE            ', 'SMMLA_asimdsame2_G       ', 'arch_variant=ARMv8v6 extensions=INT8MATMUL '];
T[' SQRDMLAH VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }                  ', 'Q=0b0 size=0b01 ', 'SQRDMLAH_asimdsame2_only ', 'arch_variant=ARMv8v1 extensions=QRDMLAH    '];
T[' SQRDMLAH VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }                  ', 'Q=0b1 size=0b01 ', 'SQRDMLAH_asimdsame2_only ', 'arch_variant=ARMv8v1 extensions=QRDMLAH    '];
T[' SQRDMLAH VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }                  ', 'Q=0b0 size=0b10 ', 'SQRDMLAH_asimdsame2_only ', 'arch_variant=ARMv8v1 extensions=QRDMLAH    '];
T[' SQRDMLAH VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }                  ', 'Q=0b1 size=0b10 ', 'SQRDMLAH_asimdsame2_only ', 'arch_variant=ARMv8v1 extensions=QRDMLAH    '];
T[' SQRDMLSH VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } VREG={REG:s16=$Vm VDT=4H }                  ', 'Q=0b0 size=0b01 ', 'SQRDMLSH_asimdsame2_only ', 'arch_variant=ARMv8v1 extensions=QRDMLAH    '];
T[' SQRDMLSH VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } VREG={REG:s16=$Vm VDT=8H }                  ', 'Q=0b1 size=0b01 ', 'SQRDMLSH_asimdsame2_only ', 'arch_variant=ARMv8v1 extensions=QRDMLAH    '];
T[' SQRDMLSH VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } VREG={REG:s32=$Vm VDT=2S }                  ', 'Q=0b0 size=0b10 ', 'SQRDMLSH_asimdsame2_only ', 'arch_variant=ARMv8v1 extensions=QRDMLAH    '];
T[' SQRDMLSH VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } VREG={REG:s32=$Vm VDT=4S }                  ', 'Q=0b1 size=0b10 ', 'SQRDMLSH_asimdsame2_only ', 'arch_variant=ARMv8v1 extensions=QRDMLAH    '];
T[' UDOT     VREG={REG:u32=$Vd VDT=2S } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B }                ', 'Q=0b0           ', 'UDOT_asimdsame2_D        ', 'arch_variant=ARMv8v2 extensions=DOTP       '];
T[' UDOT     VREG={REG:u32=$Vd VDT=4S } VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B}                ', 'Q=0b1           ', 'UDOT_asimdsame2_D        ', 'arch_variant=ARMv8v2 extensions=DOTP       '];
T[' UMMLA    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B}                ', 'NONE            ', 'UMMLA_asimdsame2_G       ', 'arch_variant=ARMv8v6 extensions=INT8MATMUL '];
T[' USDOT    VREG={REG:u32=$Vd VDT=2S } VREG={REG:u8=$Vn   VDT=8B } VREG={REG:u8=$Vm   VDT=8B }                ', 'Q=0b0           ', 'USDOT_asimdsame2_D       ', 'arch_variant=ARMv8v6 extensions=INT8MATMUL '];
T[' USDOT    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B}                ', 'Q=0b1           ', 'USDOT_asimdsame2_D       ', 'arch_variant=ARMv8v6 extensions=INT8MATMUL '];
T[' USMMLA   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u8=$Vn   VDT=16B} VREG={REG:u8=$Vm   VDT=16B}                ', 'NONE            ', 'USMMLA_asimdsame2_G      ', 'arch_variant=ARMv8v6 extensions=INT8MATMUL '];



# asimdsamefp16: Advanced SIMD three same (FP16).
# A64: ig0=0b0 Q:u=0bx U:u=0bx ig1=0b01110 a:u=0bx ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode:u=0bxxx ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asimdsamefp16', 'A64');

DEF Vd => VECREG('Vd', 'Rd', 'write') ;
DEF Vm => VECREG('Vm', 'Rm', 'read')  ;
DEF Vn => VECREG('Vn', 'Rn', 'read')  ;

T[' FABD    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FABD_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FABD    VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FABD_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FACGE   VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FACGE_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FACGE   VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FACGE_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FACGT   VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FACGT_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FACGT   VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FACGT_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FADD    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FADD_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FADD    VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FADD_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FADDP   VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FADDP_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FADDP   VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FADDP_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMEQ   VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FCMEQ_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMEQ   VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FCMEQ_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMGE   VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FCMGE_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMGE   VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FCMGE_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMGT   VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FCMGT_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMGT   VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FCMGT_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FDIV    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FDIV_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FDIV    VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FDIV_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMAX    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FMAX_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMAX    VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FMAX_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMAXNM  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FMAXNM_asimdsamefp16_only  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMAXNM  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FMAXNM_asimdsamefp16_only  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMAXNMP VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FMAXNMP_asimdsamefp16_only ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMAXNMP VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FMAXNMP_asimdsamefp16_only ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMAXP   VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FMAXP_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMAXP   VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FMAXP_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMIN    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FMIN_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMIN    VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FMIN_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMINNM  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FMINNM_asimdsamefp16_only  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMINNM  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FMINNM_asimdsamefp16_only  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMINNMP VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FMINNMP_asimdsamefp16_only ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMINNMP VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FMINNMP_asimdsamefp16_only ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMINP   VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FMINP_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMINP   VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FMINP_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMLA    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FMLA_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMLA    VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FMLA_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMLS    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FMLS_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMLS    VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FMLS_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMUL    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FMUL_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMUL    VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FMUL_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMULX   VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FMULX_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMULX   VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FMULX_asimdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRECPS  VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FRECPS_asimdsamefp16_only  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRECPS  VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FRECPS_asimdsamefp16_only  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRSQRTS VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FRSQRTS_asimdsamefp16_only ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRSQRTS VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FRSQRTS_asimdsamefp16_only ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FSUB    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } VREG={REG:f16=$Vm VDT=4H } ', 'Q=0b0 ', 'FSUB_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FSUB    VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } VREG={REG:f16=$Vm VDT=8H } ', 'Q=0b1 ', 'FSUB_asimdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];



# asimdshf: Advanced SIMD shift by immediate.
# A64: ig0=0b0 Q:u=0bx U:u=0bx ig1=0b011110 immh:u!=0b0000 immb:u=0bxxx opcode:u=0bxxxxx ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asimdshf', 'A64');

DEF Vd           => VECREG('Vd', 'Rd', 'write')                                    ;
DEF Vn           => VECREG('Vn', 'Rn', 'read')                                     ;
DEF shift_left   => IMM('shift_left', 'immh:immb', range => (0, 7), arg0 => 8)     ;
DEF shift_left2  => IMM('shift_left', 'immh:immb', range => (0, 15), arg0 => 16)   ;
DEF shift_left3  => IMM('shift_left', 'immh:immb', range => (0, 31), arg0 => 32)   ;
DEF shift_left4  => IMM('shift_left', 'immh:immb', range => (0, 63), arg0 => 64)   ;
DEF shift_right  => IMM('shift_right', 'immh:immb', range => (1, 16), arg0 => 32)  ;
DEF shift_right2 => IMM('shift_right', 'immh:immb', range => (1, 32), arg0 => 64)  ;
DEF shift_right3 => IMM('shift_right', 'immh:immb', range => (1, 64), arg0 => 128) ;
DEF shift_right4 => IMM('shift_right', 'immh:immb', range => (1, 8), arg0 => 16)   ;

T[' FCVTZS    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'FCVTZS_asimdshf_C     ', 'extensions=FP16 '];
T[' FCVTZS    VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'FCVTZS_asimdshf_C     ', 'extensions=FP16 '];
T[' FCVTZS    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'FCVTZS_asimdshf_C     ', 'extensions=FP16 '];
T[' FCVTZS    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'FCVTZS_asimdshf_C     ', 'extensions=FP16 '];
T[' FCVTZS    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } IMM:u8=$shift_right3   ', 'Q=0b1 immh=0b1xxx ', 'FCVTZS_asimdshf_C     ', 'extensions=FP16 '];
T[' FCVTZU    VREG={REG:f16=$Vd VDT=4H } VREG={REG:f16=$Vn VDT=4H } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'FCVTZU_asimdshf_C     ', 'extensions=FP16 '];
T[' FCVTZU    VREG={REG:f16=$Vd VDT=8H } VREG={REG:f16=$Vn VDT=8H } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'FCVTZU_asimdshf_C     ', 'extensions=FP16 '];
T[' FCVTZU    VREG={REG:f32=$Vd VDT=2S } VREG={REG:f32=$Vn VDT=2S } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'FCVTZU_asimdshf_C     ', 'extensions=FP16 '];
T[' FCVTZU    VREG={REG:f32=$Vd VDT=4S } VREG={REG:f32=$Vn VDT=4S } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'FCVTZU_asimdshf_C     ', 'extensions=FP16 '];
T[' FCVTZU    VREG={REG:f64=$Vd VDT=2D } VREG={REG:f64=$Vn VDT=2D } IMM:u8=$shift_right3   ', 'Q=0b1 immh=0b1xxx ', 'FCVTZU_asimdshf_C     ', 'extensions=FP16 '];
T[' RSHRN     VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i16=$Vn VDT=8H } IMM:u8=$shift_right4  ', 'Q=0b0 immh=0b0001 ', 'RSHRN_asimdshf_N      ', '                '];
T[' RSHRN     VREG={REG:i16=$Vd VDT=4H } VREG={REG:i32=$Vn VDT=4S } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'RSHRN_asimdshf_N      ', '                '];
T[' RSHRN     VREG={REG:i32=$Vd VDT=2S } VREG={REG:i64=$Vn VDT=2D } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'RSHRN_asimdshf_N      ', '                '];
T[' RSHRN2    VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i16=$Vn VDT=8H } IMM:u8=$shift_right4  ', 'Q=0b1 immh=0b0001 ', 'RSHRN_asimdshf_N      ', '                '];
T[' RSHRN2    VREG={REG:i16=$Vd VDT=8H } VREG={REG:i32=$Vn VDT=4S } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'RSHRN_asimdshf_N      ', '                '];
T[' RSHRN2    VREG={REG:i32=$Vd VDT=4S } VREG={REG:i64=$Vn VDT=2D } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'RSHRN_asimdshf_N      ', '                '];
T[' SCVTF     VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'SCVTF_asimdshf_C      ', 'extensions=FP16 '];
T[' SCVTF     VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'SCVTF_asimdshf_C      ', 'extensions=FP16 '];
T[' SCVTF     VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'SCVTF_asimdshf_C      ', 'extensions=FP16 '];
T[' SCVTF     VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'SCVTF_asimdshf_C      ', 'extensions=FP16 '];
T[' SCVTF     VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D } IMM:u8=$shift_right3   ', 'Q=0b1 immh=0b1xxx ', 'SCVTF_asimdshf_C      ', 'extensions=FP16 '];
T[' SHL       VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } IMM:u8=$shift_left   ', 'Q=0b0 immh=0b0001 ', 'SHL_asimdshf_R        ', '                '];
T[' SHL       VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} IMM:u8=$shift_left   ', 'Q=0b1 immh=0b0001 ', 'SHL_asimdshf_R        ', '                '];
T[' SHL       VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } IMM:u8=$shift_left2    ', 'Q=0b0 immh=0b001x ', 'SHL_asimdshf_R        ', '                '];
T[' SHL       VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } IMM:u8=$shift_left2    ', 'Q=0b1 immh=0b001x ', 'SHL_asimdshf_R        ', '                '];
T[' SHL       VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } IMM:u8=$shift_left3    ', 'Q=0b0 immh=0b01xx ', 'SHL_asimdshf_R        ', '                '];
T[' SHL       VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } IMM:u8=$shift_left3    ', 'Q=0b1 immh=0b01xx ', 'SHL_asimdshf_R        ', '                '];
T[' SHL       VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } IMM:u8=$shift_left4    ', 'Q=0b1 immh=0b1xxx ', 'SHL_asimdshf_R        ', '                '];
T[' SHRN      VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i16=$Vn VDT=8H } IMM:u8=$shift_right4  ', 'Q=0b0 immh=0b0001 ', 'SHRN_asimdshf_N       ', '                '];
T[' SHRN      VREG={REG:i16=$Vd VDT=4H } VREG={REG:i32=$Vn VDT=4S } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'SHRN_asimdshf_N       ', '                '];
T[' SHRN      VREG={REG:i32=$Vd VDT=2S } VREG={REG:i64=$Vn VDT=2D } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'SHRN_asimdshf_N       ', '                '];
T[' SHRN2     VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i16=$Vn VDT=8H } IMM:u8=$shift_right4  ', 'Q=0b1 immh=0b0001 ', 'SHRN_asimdshf_N       ', '                '];
T[' SHRN2     VREG={REG:i16=$Vd VDT=8H } VREG={REG:i32=$Vn VDT=4S } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'SHRN_asimdshf_N       ', '                '];
T[' SHRN2     VREG={REG:i32=$Vd VDT=4S } VREG={REG:i64=$Vn VDT=2D } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'SHRN_asimdshf_N       ', '                '];
T[' SLI       VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } IMM:u8=$shift_left   ', 'Q=0b0 immh=0b0001 ', 'SLI_asimdshf_R        ', '                '];
T[' SLI       VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} IMM:u8=$shift_left   ', 'Q=0b1 immh=0b0001 ', 'SLI_asimdshf_R        ', '                '];
T[' SLI       VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } IMM:u8=$shift_left2    ', 'Q=0b0 immh=0b001x ', 'SLI_asimdshf_R        ', '                '];
T[' SLI       VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } IMM:u8=$shift_left2    ', 'Q=0b1 immh=0b001x ', 'SLI_asimdshf_R        ', '                '];
T[' SLI       VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } IMM:u8=$shift_left3    ', 'Q=0b0 immh=0b01xx ', 'SLI_asimdshf_R        ', '                '];
T[' SLI       VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } IMM:u8=$shift_left3    ', 'Q=0b1 immh=0b01xx ', 'SLI_asimdshf_R        ', '                '];
T[' SLI       VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } IMM:u8=$shift_left4    ', 'Q=0b1 immh=0b1xxx ', 'SLI_asimdshf_R        ', '                '];
T[' SQRSHRN   VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s16=$Vn VDT=8H } IMM:u8=$shift_right4  ', 'Q=0b0 immh=0b0001 ', 'SQRSHRN_asimdshf_N    ', '                '];
T[' SQRSHRN   VREG={REG:s16=$Vd VDT=4H } VREG={REG:s32=$Vn VDT=4S } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'SQRSHRN_asimdshf_N    ', '                '];
T[' SQRSHRN   VREG={REG:s32=$Vd VDT=2S } VREG={REG:s64=$Vn VDT=2D } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'SQRSHRN_asimdshf_N    ', '                '];
T[' SQRSHRN2  VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s16=$Vn VDT=8H } IMM:u8=$shift_right4  ', 'Q=0b1 immh=0b0001 ', 'SQRSHRN_asimdshf_N    ', '                '];
T[' SQRSHRN2  VREG={REG:s16=$Vd VDT=8H } VREG={REG:s32=$Vn VDT=4S } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'SQRSHRN_asimdshf_N    ', '                '];
T[' SQRSHRN2  VREG={REG:s32=$Vd VDT=4S } VREG={REG:s64=$Vn VDT=2D } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'SQRSHRN_asimdshf_N    ', '                '];
T[' SQRSHRUN  VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u16=$Vn VDT=8H } IMM:u8=$shift_right4  ', 'Q=0b0 immh=0b0001 ', 'SQRSHRUN_asimdshf_N   ', '                '];
T[' SQRSHRUN  VREG={REG:u16=$Vd VDT=4H } VREG={REG:u32=$Vn VDT=4S } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'SQRSHRUN_asimdshf_N   ', '                '];
T[' SQRSHRUN  VREG={REG:u32=$Vd VDT=2S } VREG={REG:u64=$Vn VDT=2D } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'SQRSHRUN_asimdshf_N   ', '                '];
T[' SQRSHRUN2 VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u16=$Vn VDT=8H } IMM:u8=$shift_right4  ', 'Q=0b1 immh=0b0001 ', 'SQRSHRUN_asimdshf_N   ', '                '];
T[' SQRSHRUN2 VREG={REG:u16=$Vd VDT=8H } VREG={REG:u32=$Vn VDT=4S } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'SQRSHRUN_asimdshf_N   ', '                '];
T[' SQRSHRUN2 VREG={REG:u32=$Vd VDT=4S } VREG={REG:u64=$Vn VDT=2D } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'SQRSHRUN_asimdshf_N   ', '                '];
T[' SQSHL     VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } IMM:u8=$shift_left   ', 'Q=0b0 immh=0b0001 ', 'SQSHL_asimdshf_R      ', '                '];
T[' SQSHL     VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} IMM:u8=$shift_left   ', 'Q=0b1 immh=0b0001 ', 'SQSHL_asimdshf_R      ', '                '];
T[' SQSHL     VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } IMM:u8=$shift_left2    ', 'Q=0b0 immh=0b001x ', 'SQSHL_asimdshf_R      ', '                '];
T[' SQSHL     VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } IMM:u8=$shift_left2    ', 'Q=0b1 immh=0b001x ', 'SQSHL_asimdshf_R      ', '                '];
T[' SQSHL     VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } IMM:u8=$shift_left3    ', 'Q=0b0 immh=0b01xx ', 'SQSHL_asimdshf_R      ', '                '];
T[' SQSHL     VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } IMM:u8=$shift_left3    ', 'Q=0b1 immh=0b01xx ', 'SQSHL_asimdshf_R      ', '                '];
T[' SQSHL     VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D } IMM:u8=$shift_left4    ', 'Q=0b1 immh=0b1xxx ', 'SQSHL_asimdshf_R      ', '                '];
T[' SQSHLU    VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } IMM:u8=$shift_left   ', 'Q=0b0 immh=0b0001 ', 'SQSHLU_asimdshf_R     ', '                '];
T[' SQSHLU    VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} IMM:u8=$shift_left   ', 'Q=0b1 immh=0b0001 ', 'SQSHLU_asimdshf_R     ', '                '];
T[' SQSHLU    VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } IMM:u8=$shift_left2    ', 'Q=0b0 immh=0b001x ', 'SQSHLU_asimdshf_R     ', '                '];
T[' SQSHLU    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } IMM:u8=$shift_left2    ', 'Q=0b1 immh=0b001x ', 'SQSHLU_asimdshf_R     ', '                '];
T[' SQSHLU    VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } IMM:u8=$shift_left3    ', 'Q=0b0 immh=0b01xx ', 'SQSHLU_asimdshf_R     ', '                '];
T[' SQSHLU    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } IMM:u8=$shift_left3    ', 'Q=0b1 immh=0b01xx ', 'SQSHLU_asimdshf_R     ', '                '];
T[' SQSHLU    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D } IMM:u8=$shift_left4    ', 'Q=0b1 immh=0b1xxx ', 'SQSHLU_asimdshf_R     ', '                '];
T[' SQSHRN    VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s16=$Vn VDT=8H } IMM:u8=$shift_right4  ', 'Q=0b0 immh=0b0001 ', 'SQSHRN_asimdshf_N     ', '                '];
T[' SQSHRN    VREG={REG:s16=$Vd VDT=4H } VREG={REG:s32=$Vn VDT=4S } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'SQSHRN_asimdshf_N     ', '                '];
T[' SQSHRN    VREG={REG:s32=$Vd VDT=2S } VREG={REG:s64=$Vn VDT=2D } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'SQSHRN_asimdshf_N     ', '                '];
T[' SQSHRN2   VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s16=$Vn VDT=8H } IMM:u8=$shift_right4  ', 'Q=0b1 immh=0b0001 ', 'SQSHRN_asimdshf_N     ', '                '];
T[' SQSHRN2   VREG={REG:s16=$Vd VDT=8H } VREG={REG:s32=$Vn VDT=4S } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'SQSHRN_asimdshf_N     ', '                '];
T[' SQSHRN2   VREG={REG:s32=$Vd VDT=4S } VREG={REG:s64=$Vn VDT=2D } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'SQSHRN_asimdshf_N     ', '                '];
T[' SQSHRUN   VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u16=$Vn VDT=8H } IMM:u8=$shift_right4  ', 'Q=0b0 immh=0b0001 ', 'SQSHRUN_asimdshf_N    ', '                '];
T[' SQSHRUN   VREG={REG:u16=$Vd VDT=4H } VREG={REG:u32=$Vn VDT=4S } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'SQSHRUN_asimdshf_N    ', '                '];
T[' SQSHRUN   VREG={REG:u32=$Vd VDT=2S } VREG={REG:u64=$Vn VDT=2D } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'SQSHRUN_asimdshf_N    ', '                '];
T[' SQSHRUN2  VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u16=$Vn VDT=8H } IMM:u8=$shift_right4  ', 'Q=0b1 immh=0b0001 ', 'SQSHRUN_asimdshf_N    ', '                '];
T[' SQSHRUN2  VREG={REG:u16=$Vd VDT=8H } VREG={REG:u32=$Vn VDT=4S } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'SQSHRUN_asimdshf_N    ', '                '];
T[' SQSHRUN2  VREG={REG:u32=$Vd VDT=4S } VREG={REG:u64=$Vn VDT=2D } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'SQSHRUN_asimdshf_N    ', '                '];
T[' SRI       VREG={REG:i8=$Vd   VDT=8B } VREG={REG:i8=$Vn   VDT=8B } IMM:u8=$shift_right4 ', 'Q=0b0 immh=0b0001 ', 'SRI_asimdshf_R        ', '                '];
T[' SRI       VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} IMM:u8=$shift_right4 ', 'Q=0b1 immh=0b0001 ', 'SRI_asimdshf_R        ', '                '];
T[' SRI       VREG={REG:i16=$Vd VDT=4H } VREG={REG:i16=$Vn VDT=4H } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'SRI_asimdshf_R        ', '                '];
T[' SRI       VREG={REG:i16=$Vd VDT=8H } VREG={REG:i16=$Vn VDT=8H } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'SRI_asimdshf_R        ', '                '];
T[' SRI       VREG={REG:i32=$Vd VDT=2S } VREG={REG:i32=$Vn VDT=2S } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'SRI_asimdshf_R        ', '                '];
T[' SRI       VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'SRI_asimdshf_R        ', '                '];
T[' SRI       VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } IMM:u8=$shift_right3   ', 'Q=0b1 immh=0b1xxx ', 'SRI_asimdshf_R        ', '                '];
T[' SRSHR     VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } IMM:u8=$shift_right4 ', 'Q=0b0 immh=0b0001 ', 'SRSHR_asimdshf_R      ', '                '];
T[' SRSHR     VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} IMM:u8=$shift_right4 ', 'Q=0b1 immh=0b0001 ', 'SRSHR_asimdshf_R      ', '                '];
T[' SRSHR     VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'SRSHR_asimdshf_R      ', '                '];
T[' SRSHR     VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'SRSHR_asimdshf_R      ', '                '];
T[' SRSHR     VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'SRSHR_asimdshf_R      ', '                '];
T[' SRSHR     VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'SRSHR_asimdshf_R      ', '                '];
T[' SRSHR     VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D } IMM:u8=$shift_right3   ', 'Q=0b1 immh=0b1xxx ', 'SRSHR_asimdshf_R      ', '                '];
T[' SRSRA     VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } IMM:u8=$shift_right4 ', 'Q=0b0 immh=0b0001 ', 'SRSRA_asimdshf_R      ', '                '];
T[' SRSRA     VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} IMM:u8=$shift_right4 ', 'Q=0b1 immh=0b0001 ', 'SRSRA_asimdshf_R      ', '                '];
T[' SRSRA     VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'SRSRA_asimdshf_R      ', '                '];
T[' SRSRA     VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'SRSRA_asimdshf_R      ', '                '];
T[' SRSRA     VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'SRSRA_asimdshf_R      ', '                '];
T[' SRSRA     VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'SRSRA_asimdshf_R      ', '                '];
T[' SRSRA     VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D } IMM:u8=$shift_right3   ', 'Q=0b1 immh=0b1xxx ', 'SRSRA_asimdshf_R      ', '                '];
T[' SSHLL     VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=8B } IMM:u8=$shift_left    ', 'Q=0b0 immh=0b0001 ', 'SSHLL_asimdshf_L      ', '                '];
T[' SSHLL     VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H } IMM:u8=$shift_left2    ', 'Q=0b0 immh=0b001x ', 'SSHLL_asimdshf_L      ', '                '];
T[' SSHLL     VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S } IMM:u8=$shift_left3    ', 'Q=0b0 immh=0b01xx ', 'SSHLL_asimdshf_L      ', '                '];
T[' SSHLL2    VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=16B} IMM:u8=$shift_left    ', 'Q=0b1 immh=0b0001 ', 'SSHLL_asimdshf_L      ', '                '];
T[' SSHLL2    VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H } IMM:u8=$shift_left2    ', 'Q=0b1 immh=0b001x ', 'SSHLL_asimdshf_L      ', '                '];
T[' SSHLL2    VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S } IMM:u8=$shift_left3    ', 'Q=0b1 immh=0b01xx ', 'SSHLL_asimdshf_L      ', '                '];
T[' SSHR      VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } IMM:u8=$shift_right4 ', 'Q=0b0 immh=0b0001 ', 'SSHR_asimdshf_R       ', '                '];
T[' SSHR      VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} IMM:u8=$shift_right4 ', 'Q=0b1 immh=0b0001 ', 'SSHR_asimdshf_R       ', '                '];
T[' SSHR      VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'SSHR_asimdshf_R       ', '                '];
T[' SSHR      VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'SSHR_asimdshf_R       ', '                '];
T[' SSHR      VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'SSHR_asimdshf_R       ', '                '];
T[' SSHR      VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'SSHR_asimdshf_R       ', '                '];
T[' SSHR      VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D } IMM:u8=$shift_right3   ', 'Q=0b1 immh=0b1xxx ', 'SSHR_asimdshf_R       ', '                '];
T[' SSRA      VREG={REG:s8=$Vd   VDT=8B } VREG={REG:s8=$Vn   VDT=8B } IMM:u8=$shift_right4 ', 'Q=0b0 immh=0b0001 ', 'SSRA_asimdshf_R       ', '                '];
T[' SSRA      VREG={REG:s8=$Vd   VDT=16B} VREG={REG:s8=$Vn   VDT=16B} IMM:u8=$shift_right4 ', 'Q=0b1 immh=0b0001 ', 'SSRA_asimdshf_R       ', '                '];
T[' SSRA      VREG={REG:s16=$Vd VDT=4H } VREG={REG:s16=$Vn VDT=4H } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'SSRA_asimdshf_R       ', '                '];
T[' SSRA      VREG={REG:s16=$Vd VDT=8H } VREG={REG:s16=$Vn VDT=8H } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'SSRA_asimdshf_R       ', '                '];
T[' SSRA      VREG={REG:s32=$Vd VDT=2S } VREG={REG:s32=$Vn VDT=2S } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'SSRA_asimdshf_R       ', '                '];
T[' SSRA      VREG={REG:s32=$Vd VDT=4S } VREG={REG:s32=$Vn VDT=4S } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'SSRA_asimdshf_R       ', '                '];
T[' SSRA      VREG={REG:s64=$Vd VDT=2D } VREG={REG:s64=$Vn VDT=2D } IMM:u8=$shift_right3   ', 'Q=0b1 immh=0b1xxx ', 'SSRA_asimdshf_R       ', '                '];
T[' UCVTF     VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'UCVTF_asimdshf_C      ', 'extensions=FP16 '];
T[' UCVTF     VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'UCVTF_asimdshf_C      ', 'extensions=FP16 '];
T[' UCVTF     VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'UCVTF_asimdshf_C      ', 'extensions=FP16 '];
T[' UCVTF     VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'UCVTF_asimdshf_C      ', 'extensions=FP16 '];
T[' UCVTF     VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D } IMM:u8=$shift_right3   ', 'Q=0b1 immh=0b1xxx ', 'UCVTF_asimdshf_C      ', 'extensions=FP16 '];
T[' UQRSHRN   VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u16=$Vn VDT=8H } IMM:u8=$shift_right4  ', 'Q=0b0 immh=0b0001 ', 'UQRSHRN_asimdshf_N    ', '                '];
T[' UQRSHRN   VREG={REG:u16=$Vd VDT=4H } VREG={REG:u32=$Vn VDT=4S } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'UQRSHRN_asimdshf_N    ', '                '];
T[' UQRSHRN   VREG={REG:u32=$Vd VDT=2S } VREG={REG:u64=$Vn VDT=2D } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'UQRSHRN_asimdshf_N    ', '                '];
T[' UQRSHRN2  VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u16=$Vn VDT=8H } IMM:u8=$shift_right4  ', 'Q=0b1 immh=0b0001 ', 'UQRSHRN_asimdshf_N    ', '                '];
T[' UQRSHRN2  VREG={REG:u16=$Vd VDT=8H } VREG={REG:u32=$Vn VDT=4S } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'UQRSHRN_asimdshf_N    ', '                '];
T[' UQRSHRN2  VREG={REG:u32=$Vd VDT=4S } VREG={REG:u64=$Vn VDT=2D } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'UQRSHRN_asimdshf_N    ', '                '];
T[' UQSHL     VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } IMM:u8=$shift_left   ', 'Q=0b0 immh=0b0001 ', 'UQSHL_asimdshf_R      ', '                '];
T[' UQSHL     VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} IMM:u8=$shift_left   ', 'Q=0b1 immh=0b0001 ', 'UQSHL_asimdshf_R      ', '                '];
T[' UQSHL     VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } IMM:u8=$shift_left2    ', 'Q=0b0 immh=0b001x ', 'UQSHL_asimdshf_R      ', '                '];
T[' UQSHL     VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } IMM:u8=$shift_left2    ', 'Q=0b1 immh=0b001x ', 'UQSHL_asimdshf_R      ', '                '];
T[' UQSHL     VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } IMM:u8=$shift_left3    ', 'Q=0b0 immh=0b01xx ', 'UQSHL_asimdshf_R      ', '                '];
T[' UQSHL     VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } IMM:u8=$shift_left3    ', 'Q=0b1 immh=0b01xx ', 'UQSHL_asimdshf_R      ', '                '];
T[' UQSHL     VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D } IMM:u8=$shift_left4    ', 'Q=0b1 immh=0b1xxx ', 'UQSHL_asimdshf_R      ', '                '];
T[' UQSHRN    VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u16=$Vn VDT=8H } IMM:u8=$shift_right4  ', 'Q=0b0 immh=0b0001 ', 'UQSHRN_asimdshf_N     ', '                '];
T[' UQSHRN    VREG={REG:u16=$Vd VDT=4H } VREG={REG:u32=$Vn VDT=4S } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'UQSHRN_asimdshf_N     ', '                '];
T[' UQSHRN    VREG={REG:u32=$Vd VDT=2S } VREG={REG:u64=$Vn VDT=2D } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'UQSHRN_asimdshf_N     ', '                '];
T[' UQSHRN2   VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u16=$Vn VDT=8H } IMM:u8=$shift_right4  ', 'Q=0b1 immh=0b0001 ', 'UQSHRN_asimdshf_N     ', '                '];
T[' UQSHRN2   VREG={REG:u16=$Vd VDT=8H } VREG={REG:u32=$Vn VDT=4S } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'UQSHRN_asimdshf_N     ', '                '];
T[' UQSHRN2   VREG={REG:u32=$Vd VDT=4S } VREG={REG:u64=$Vn VDT=2D } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'UQSHRN_asimdshf_N     ', '                '];
T[' URSHR     VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } IMM:u8=$shift_right4 ', 'Q=0b0 immh=0b0001 ', 'URSHR_asimdshf_R      ', '                '];
T[' URSHR     VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} IMM:u8=$shift_right4 ', 'Q=0b1 immh=0b0001 ', 'URSHR_asimdshf_R      ', '                '];
T[' URSHR     VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'URSHR_asimdshf_R      ', '                '];
T[' URSHR     VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'URSHR_asimdshf_R      ', '                '];
T[' URSHR     VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'URSHR_asimdshf_R      ', '                '];
T[' URSHR     VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'URSHR_asimdshf_R      ', '                '];
T[' URSHR     VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D } IMM:u8=$shift_right3   ', 'Q=0b1 immh=0b1xxx ', 'URSHR_asimdshf_R      ', '                '];
T[' URSRA     VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } IMM:u8=$shift_right4 ', 'Q=0b0 immh=0b0001 ', 'URSRA_asimdshf_R      ', '                '];
T[' URSRA     VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} IMM:u8=$shift_right4 ', 'Q=0b1 immh=0b0001 ', 'URSRA_asimdshf_R      ', '                '];
T[' URSRA     VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'URSRA_asimdshf_R      ', '                '];
T[' URSRA     VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'URSRA_asimdshf_R      ', '                '];
T[' URSRA     VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'URSRA_asimdshf_R      ', '                '];
T[' URSRA     VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'URSRA_asimdshf_R      ', '                '];
T[' URSRA     VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D } IMM:u8=$shift_right3   ', 'Q=0b1 immh=0b1xxx ', 'URSRA_asimdshf_R      ', '                '];
T[' USHLL     VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=8B } IMM:u8=$shift_left    ', 'Q=0b0 immh=0b0001 ', 'USHLL_asimdshf_L      ', '                '];
T[' USHLL     VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=4H } IMM:u8=$shift_left2    ', 'Q=0b0 immh=0b001x ', 'USHLL_asimdshf_L      ', '                '];
T[' USHLL     VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=2S } IMM:u8=$shift_left3    ', 'Q=0b0 immh=0b01xx ', 'USHLL_asimdshf_L      ', '                '];
T[' USHLL2    VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=16B} IMM:u8=$shift_left    ', 'Q=0b1 immh=0b0001 ', 'USHLL_asimdshf_L      ', '                '];
T[' USHLL2    VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=8H } IMM:u8=$shift_left2    ', 'Q=0b1 immh=0b001x ', 'USHLL_asimdshf_L      ', '                '];
T[' USHLL2    VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=4S } IMM:u8=$shift_left3    ', 'Q=0b1 immh=0b01xx ', 'USHLL_asimdshf_L      ', '                '];
T[' USHR      VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } IMM:u8=$shift_right4 ', 'Q=0b0 immh=0b0001 ', 'USHR_asimdshf_R       ', '                '];
T[' USHR      VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} IMM:u8=$shift_right4 ', 'Q=0b1 immh=0b0001 ', 'USHR_asimdshf_R       ', '                '];
T[' USHR      VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'USHR_asimdshf_R       ', '                '];
T[' USHR      VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'USHR_asimdshf_R       ', '                '];
T[' USHR      VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'USHR_asimdshf_R       ', '                '];
T[' USHR      VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'USHR_asimdshf_R       ', '                '];
T[' USHR      VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D } IMM:u8=$shift_right3   ', 'Q=0b1 immh=0b1xxx ', 'USHR_asimdshf_R       ', '                '];
T[' USRA      VREG={REG:u8=$Vd   VDT=8B } VREG={REG:u8=$Vn   VDT=8B } IMM:u8=$shift_right4 ', 'Q=0b0 immh=0b0001 ', 'USRA_asimdshf_R       ', '                '];
T[' USRA      VREG={REG:u8=$Vd   VDT=16B} VREG={REG:u8=$Vn   VDT=16B} IMM:u8=$shift_right4 ', 'Q=0b1 immh=0b0001 ', 'USRA_asimdshf_R       ', '                '];
T[' USRA      VREG={REG:u16=$Vd VDT=4H } VREG={REG:u16=$Vn VDT=4H } IMM:u8=$shift_right    ', 'Q=0b0 immh=0b001x ', 'USRA_asimdshf_R       ', '                '];
T[' USRA      VREG={REG:u16=$Vd VDT=8H } VREG={REG:u16=$Vn VDT=8H } IMM:u8=$shift_right    ', 'Q=0b1 immh=0b001x ', 'USRA_asimdshf_R       ', '                '];
T[' USRA      VREG={REG:u32=$Vd VDT=2S } VREG={REG:u32=$Vn VDT=2S } IMM:u8=$shift_right2   ', 'Q=0b0 immh=0b01xx ', 'USRA_asimdshf_R       ', '                '];
T[' USRA      VREG={REG:u32=$Vd VDT=4S } VREG={REG:u32=$Vn VDT=4S } IMM:u8=$shift_right2   ', 'Q=0b1 immh=0b01xx ', 'USRA_asimdshf_R       ', '                '];
T[' USRA      VREG={REG:u64=$Vd VDT=2D } VREG={REG:u64=$Vn VDT=2D } IMM:u8=$shift_right3   ', 'Q=0b1 immh=0b1xxx ', 'USRA_asimdshf_R       ', '                '];
T[' SXTL      VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=8B }                       ', 'Q=0b0 immh=0b0001 ', 'SXTL_SSHLL_asimdshf_L ', '                '];
T[' SXTL      VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=4H }                        ', 'Q=0b0 immh=0b001x ', 'SXTL_SSHLL_asimdshf_L ', '                '];
T[' SXTL      VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=2S }                        ', 'Q=0b0 immh=0b01xx ', 'SXTL_SSHLL_asimdshf_L ', '                '];
T[' SXTL2     VREG={REG:s16=$Vd VDT=8H } VREG={REG:s8=$Vn   VDT=16B}                       ', 'Q=0b1 immh=0b0001 ', 'SXTL_SSHLL_asimdshf_L ', '                '];
T[' SXTL2     VREG={REG:s32=$Vd VDT=4S } VREG={REG:s16=$Vn VDT=8H }                        ', 'Q=0b1 immh=0b001x ', 'SXTL_SSHLL_asimdshf_L ', '                '];
T[' SXTL2     VREG={REG:s64=$Vd VDT=2D } VREG={REG:s32=$Vn VDT=4S }                        ', 'Q=0b1 immh=0b01xx ', 'SXTL_SSHLL_asimdshf_L ', '                '];
T[' UXTL      VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=8B }                       ', 'Q=0b0 immh=0b0001 ', 'UXTL_USHLL_asimdshf_L ', '                '];
T[' UXTL      VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=4H }                        ', 'Q=0b0 immh=0b001x ', 'UXTL_USHLL_asimdshf_L ', '                '];
T[' UXTL      VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=2S }                        ', 'Q=0b0 immh=0b01xx ', 'UXTL_USHLL_asimdshf_L ', '                '];
T[' UXTL2     VREG={REG:u16=$Vd VDT=8H } VREG={REG:u8=$Vn   VDT=16B}                       ', 'Q=0b1 immh=0b0001 ', 'UXTL_USHLL_asimdshf_L ', '                '];
T[' UXTL2     VREG={REG:u32=$Vd VDT=4S } VREG={REG:u16=$Vn VDT=8H }                        ', 'Q=0b1 immh=0b001x ', 'UXTL_USHLL_asimdshf_L ', '                '];
T[' UXTL2     VREG={REG:u64=$Vd VDT=2D } VREG={REG:u32=$Vn VDT=4S }                        ', 'Q=0b1 immh=0b01xx ', 'UXTL_USHLL_asimdshf_L ', '                '];



# asimdtbl: Advanced SIMD table lookup.
# A64: ig0=0b0 Q:u=0bx ig1=0b001110 op2:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 len:u=0bxx op:u=0bx ig4=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asimdtbl', 'A64');

DEF Vd => VECREG('Vd', 'Rd', 'write') ;
DEF Vm => VECREG('Vm', 'Rm', 'read')  ;
DEF Vn => VECREG('Vn', 'Rn', 'read')  ;

T[' TBL VREG={REG:i8=$Vd   VDT=8B } VLIST={VBASE={REG:i8=$Vn   VDT=16B} COUNT=2} VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 ', 'TBL_asimdtbl_L2_2 ', ' '];
T[' TBL VREG={REG:i8=$Vd   VDT=16B} VLIST={VBASE={REG:i8=$Vn   VDT=16B} COUNT=2} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 ', 'TBL_asimdtbl_L2_2 ', ' '];
T[' TBL VREG={REG:i8=$Vd   VDT=8B } VLIST={VBASE={REG:i8=$Vn   VDT=16B} COUNT=3} VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 ', 'TBL_asimdtbl_L3_3 ', ' '];
T[' TBL VREG={REG:i8=$Vd   VDT=16B} VLIST={VBASE={REG:i8=$Vn   VDT=16B} COUNT=3} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 ', 'TBL_asimdtbl_L3_3 ', ' '];
T[' TBL VREG={REG:i8=$Vd   VDT=8B } VLIST={VBASE={REG:i8=$Vn   VDT=16B} COUNT=4} VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 ', 'TBL_asimdtbl_L4_4 ', ' '];
T[' TBL VREG={REG:i8=$Vd   VDT=16B} VLIST={VBASE={REG:i8=$Vn   VDT=16B} COUNT=4} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 ', 'TBL_asimdtbl_L4_4 ', ' '];
T[' TBL VREG={REG:i8=$Vd   VDT=8B } VLIST={VBASE={REG:i8=$Vn   VDT=16B} COUNT=1} VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 ', 'TBL_asimdtbl_L1_1 ', ' '];
T[' TBL VREG={REG:i8=$Vd   VDT=16B} VLIST={VBASE={REG:i8=$Vn   VDT=16B} COUNT=1} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 ', 'TBL_asimdtbl_L1_1 ', ' '];
T[' TBX VREG={REG:i8=$Vd   VDT=8B } VLIST={VBASE={REG:i8=$Vn   VDT=16B} COUNT=2} VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 ', 'TBX_asimdtbl_L2_2 ', ' '];
T[' TBX VREG={REG:i8=$Vd   VDT=16B} VLIST={VBASE={REG:i8=$Vn   VDT=16B} COUNT=2} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 ', 'TBX_asimdtbl_L2_2 ', ' '];
T[' TBX VREG={REG:i8=$Vd   VDT=8B } VLIST={VBASE={REG:i8=$Vn   VDT=16B} COUNT=3} VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 ', 'TBX_asimdtbl_L3_3 ', ' '];
T[' TBX VREG={REG:i8=$Vd   VDT=16B} VLIST={VBASE={REG:i8=$Vn   VDT=16B} COUNT=3} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 ', 'TBX_asimdtbl_L3_3 ', ' '];
T[' TBX VREG={REG:i8=$Vd   VDT=8B } VLIST={VBASE={REG:i8=$Vn   VDT=16B} COUNT=4} VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 ', 'TBX_asimdtbl_L4_4 ', ' '];
T[' TBX VREG={REG:i8=$Vd   VDT=16B} VLIST={VBASE={REG:i8=$Vn   VDT=16B} COUNT=4} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 ', 'TBX_asimdtbl_L4_4 ', ' '];
T[' TBX VREG={REG:i8=$Vd   VDT=8B } VLIST={VBASE={REG:i8=$Vn   VDT=16B} COUNT=1} VREG={REG:i8=$Vm   VDT=8B } ', 'Q=0b0 ', 'TBX_asimdtbl_L1_1 ', ' '];
T[' TBX VREG={REG:i8=$Vd   VDT=16B} VLIST={VBASE={REG:i8=$Vn   VDT=16B} COUNT=1} VREG={REG:i8=$Vm   VDT=16B} ', 'Q=0b1 ', 'TBX_asimdtbl_L1_1 ', ' '];



# asisddiff: Advanced SIMD scalar three different.
# A64: ig0=0b01 U:u=0bx ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode:u=0bxxxx ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asisddiff', 'A64');

DEF Dd => SIMD64('Dd', 'Rd', 'write') ;
DEF Hm => SIMD16('Hm', 'Rm', 'read')  ;
DEF Hn => SIMD16('Hn', 'Rn', 'read')  ;
DEF Sd => SIMD32('Sd', 'Rd', 'write') ;
DEF Sm => SIMD32('Sm', 'Rm', 'read')  ;
DEF Sn => SIMD32('Sn', 'Rn', 'read')  ;

T[' SQDMLAL REG:s32=$Sd REG:s16=$Hn REG:s16=$Hm ', 'size=0b01 ', 'SQDMLAL_asisddiff_only ', ' '];
T[' SQDMLAL REG:s64=$Dd REG:s32=$Sn REG:s32=$Sm ', 'size=0b10 ', 'SQDMLAL_asisddiff_only ', ' '];
T[' SQDMLSL REG:s32=$Sd REG:s16=$Hn REG:s16=$Hm ', 'size=0b01 ', 'SQDMLSL_asisddiff_only ', ' '];
T[' SQDMLSL REG:s64=$Dd REG:s32=$Sn REG:s32=$Sm ', 'size=0b10 ', 'SQDMLSL_asisddiff_only ', ' '];
T[' SQDMULL REG:s32=$Sd REG:s16=$Hn REG:s16=$Hm ', 'size=0b01 ', 'SQDMULL_asisddiff_only ', ' '];
T[' SQDMULL REG:s64=$Dd REG:s32=$Sn REG:s32=$Sm ', 'size=0b10 ', 'SQDMULL_asisddiff_only ', ' '];



# asisdelem: Advanced SIMD scalar x indexed element.
# A64: ig0=0b01 U:u=0bx ig1=0b11111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode:u=0bxxxx H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asisdelem', 'A64');

DEF Dd  => SIMD64('Dd', 'Rd', 'write')  ;
DEF Dn  => SIMD64('Dn', 'Rn', 'read')   ;
DEF Hd  => SIMD16('Hd', 'Rd', 'write')  ;
DEF Hn  => SIMD16('Hn', 'Rn', 'read')   ;
DEF Sd  => SIMD32('Sd', 'Rd', 'write')  ;
DEF Sn  => SIMD32('Sn', 'Rn', 'read')   ;
DEF Vm  => VECREG('Vm', 'Rm', 'read')   ;
DEF Vm2 => VECREG('Vm', 'M:Rm', 'read') ;

T[' FMLA     REG:f16=$Hd REG:f16=$Hn EREG={REG:f16=$Vm VDS=H IDX={ENCODEDIN=H:L:M}} ', 'NONE         ', 'FMLA_asisdelem_RH_H  ', 'arch_variant=ARMv8v2 extensions=FP16    '];
T[' FMLA     REG:f32=$Sd REG:f32=$Sn EREG={REG:f32=$Vm2 VDS=S IDX={ENCODEDIN=H:L}}  ', 'sz=0b0       ', 'FMLA_asisdelem_R_SD  ', '                                        '];
T[' FMLA     REG:f64=$Dd REG:f64=$Dn EREG={REG:f64=$Vm2 VDS=D IDX={ENCODEDIN=H}}    ', 'sz=0b1 L=0b0 ', 'FMLA_asisdelem_R_SD  ', '                                        '];
T[' FMLS     REG:f16=$Hd REG:f16=$Hn EREG={REG:f16=$Vm VDS=H IDX={ENCODEDIN=H:L:M}} ', 'NONE         ', 'FMLS_asisdelem_RH_H  ', 'arch_variant=ARMv8v2 extensions=FP16    '];
T[' FMLS     REG:f32=$Sd REG:f32=$Sn EREG={REG:f32=$Vm2 VDS=S IDX={ENCODEDIN=H:L}}  ', 'sz=0b0       ', 'FMLS_asisdelem_R_SD  ', '                                        '];
T[' FMLS     REG:f64=$Dd REG:f64=$Dn EREG={REG:f64=$Vm2 VDS=D IDX={ENCODEDIN=H}}    ', 'sz=0b1 L=0b0 ', 'FMLS_asisdelem_R_SD  ', '                                        '];
T[' FMUL     REG:f16=$Hd REG:f16=$Hn EREG={REG:f16=$Vm VDS=H IDX={ENCODEDIN=H:L:M}} ', 'NONE         ', 'FMUL_asisdelem_RH_H  ', 'arch_variant=ARMv8v2 extensions=FP16    '];
T[' FMUL     REG:f32=$Sd REG:f32=$Sn EREG={REG:f32=$Vm2 VDS=S IDX={ENCODEDIN=H:L}}  ', 'sz=0b0       ', 'FMUL_asisdelem_R_SD  ', '                                        '];
T[' FMUL     REG:f64=$Dd REG:f64=$Dn EREG={REG:f64=$Vm2 VDS=D IDX={ENCODEDIN=H}}    ', 'sz=0b1 L=0b0 ', 'FMUL_asisdelem_R_SD  ', '                                        '];
T[' FMULX    REG:f16=$Hd REG:f16=$Hn EREG={REG:f16=$Vm VDS=H IDX={ENCODEDIN=H:L:M}} ', 'NONE         ', 'FMULX_asisdelem_RH_H ', 'arch_variant=ARMv8v2 extensions=FP16    '];
T[' FMULX    REG:f32=$Sd REG:f32=$Sn EREG={REG:f32=$Vm2 VDS=S IDX={ENCODEDIN=H:L}}  ', 'sz=0b0       ', 'FMULX_asisdelem_R_SD ', '                                        '];
T[' FMULX    REG:f64=$Dd REG:f64=$Dn EREG={REG:f64=$Vm2 VDS=D IDX={ENCODEDIN=H}}    ', 'sz=0b1 L=0b0 ', 'FMULX_asisdelem_R_SD ', '                                        '];
T[' SQDMLAL  REG:s32=$Sd REG:s16=$Hn EREG={REG:s16=$Vm VDS=H IDX={ENCODEDIN=H:L:M}} ', 'size=0b01    ', 'SQDMLAL_asisdelem_L  ', '                                        '];
T[' SQDMLAL  REG:s64=$Dd REG:s32=$Sn EREG={REG:s32=$Vm2 VDS=S IDX={ENCODEDIN=H:L}}  ', 'size=0b10    ', 'SQDMLAL_asisdelem_L  ', '                                        '];
T[' SQDMLSL  REG:s32=$Sd REG:s16=$Hn EREG={REG:s16=$Vm VDS=H IDX={ENCODEDIN=H:L:M}} ', 'size=0b01    ', 'SQDMLSL_asisdelem_L  ', '                                        '];
T[' SQDMLSL  REG:s64=$Dd REG:s32=$Sn EREG={REG:s32=$Vm2 VDS=S IDX={ENCODEDIN=H:L}}  ', 'size=0b10    ', 'SQDMLSL_asisdelem_L  ', '                                        '];
T[' SQDMULH  REG:s16=$Hd REG:s16=$Hn EREG={REG:s16=$Vm VDS=H IDX={ENCODEDIN=H:L:M}} ', 'size=0b01    ', 'SQDMULH_asisdelem_R  ', '                                        '];
T[' SQDMULH  REG:s32=$Sd REG:s32=$Sn EREG={REG:s32=$Vm2 VDS=S IDX={ENCODEDIN=H:L}}  ', 'size=0b10    ', 'SQDMULH_asisdelem_R  ', '                                        '];
T[' SQDMULL  REG:s32=$Sd REG:s16=$Hn EREG={REG:s16=$Vm VDS=H IDX={ENCODEDIN=H:L:M}} ', 'size=0b01    ', 'SQDMULL_asisdelem_L  ', '                                        '];
T[' SQDMULL  REG:s64=$Dd REG:s32=$Sn EREG={REG:s32=$Vm2 VDS=S IDX={ENCODEDIN=H:L}}  ', 'size=0b10    ', 'SQDMULL_asisdelem_L  ', '                                        '];
T[' SQRDMLAH REG:s16=$Hd REG:s16=$Hn EREG={REG:s16=$Vm VDS=H IDX={ENCODEDIN=H:L:M}} ', 'size=0b01    ', 'SQRDMLAH_asisdelem_R ', 'arch_variant=ARMv8v1 extensions=QRDMLAH '];
T[' SQRDMLAH REG:s32=$Sd REG:s32=$Sn EREG={REG:s32=$Vm2 VDS=S IDX={ENCODEDIN=H:L}}  ', 'size=0b10    ', 'SQRDMLAH_asisdelem_R ', 'arch_variant=ARMv8v1 extensions=QRDMLAH '];
T[' SQRDMLSH REG:s16=$Hd REG:s16=$Hn EREG={REG:s16=$Vm VDS=H IDX={ENCODEDIN=H:L:M}} ', 'size=0b01    ', 'SQRDMLSH_asisdelem_R ', 'arch_variant=ARMv8v1 extensions=QRDMLAH '];
T[' SQRDMLSH REG:s32=$Sd REG:s32=$Sn EREG={REG:s32=$Vm2 VDS=S IDX={ENCODEDIN=H:L}}  ', 'size=0b10    ', 'SQRDMLSH_asisdelem_R ', 'arch_variant=ARMv8v1 extensions=QRDMLAH '];
T[' SQRDMULH REG:s16=$Hd REG:s16=$Hn EREG={REG:s16=$Vm VDS=H IDX={ENCODEDIN=H:L:M}} ', 'size=0b01    ', 'SQRDMULH_asisdelem_R ', '                                        '];
T[' SQRDMULH REG:s32=$Sd REG:s32=$Sn EREG={REG:s32=$Vm2 VDS=S IDX={ENCODEDIN=H:L}}  ', 'size=0b10    ', 'SQRDMULH_asisdelem_R ', '                                        '];



# asisdlse: Advanced SIMD load/store multiple structures.
# A64: ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0bx ig2=0b000000 opcode:u=0bxxxx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'asisdlse', 'A64');

DEF Vt   => VECREG('Vt', 'Rt')           ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP') ;

T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=1} MEM:r:i8={ BASE=$XnSP SZ=1x8x1x8}  ', 'Q=0b0 size=0b00 ', 'LD1_asisdlse_R1_1v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=1} MEM:r:i8={ BASE=$XnSP SZ=1x16x1x8} ', 'Q=0b1 size=0b00 ', 'LD1_asisdlse_R1_1v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=1} MEM:r:i16={BASE=$XnSP SZ=1x4x1x16}  ', 'Q=0b0 size=0b01 ', 'LD1_asisdlse_R1_1v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=1} MEM:r:i16={BASE=$XnSP SZ=1x8x1x16}  ', 'Q=0b1 size=0b01 ', 'LD1_asisdlse_R1_1v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=1} MEM:r:i32={BASE=$XnSP SZ=1x2x1x32}  ', 'Q=0b0 size=0b10 ', 'LD1_asisdlse_R1_1v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=1} MEM:r:i32={BASE=$XnSP SZ=1x4x1x32}  ', 'Q=0b1 size=0b10 ', 'LD1_asisdlse_R1_1v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=1} MEM:r:i64={BASE=$XnSP SZ=1x1x1x64}  ', 'Q=0b0 size=0b11 ', 'LD1_asisdlse_R1_1v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=1} MEM:r:i64={BASE=$XnSP SZ=1x2x1x64}  ', 'Q=0b1 size=0b11 ', 'LD1_asisdlse_R1_1v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=2} MEM:r:i8={ BASE=$XnSP SZ=2x8x1x8}  ', 'Q=0b0 size=0b00 ', 'LD1_asisdlse_R2_2v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=2} MEM:r:i8={ BASE=$XnSP SZ=2x16x1x8} ', 'Q=0b1 size=0b00 ', 'LD1_asisdlse_R2_2v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=2} MEM:r:i16={BASE=$XnSP SZ=2x4x1x16}  ', 'Q=0b0 size=0b01 ', 'LD1_asisdlse_R2_2v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=2} MEM:r:i16={BASE=$XnSP SZ=2x8x1x16}  ', 'Q=0b1 size=0b01 ', 'LD1_asisdlse_R2_2v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=2} MEM:r:i32={BASE=$XnSP SZ=2x2x1x32}  ', 'Q=0b0 size=0b10 ', 'LD1_asisdlse_R2_2v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=2} MEM:r:i32={BASE=$XnSP SZ=2x4x1x32}  ', 'Q=0b1 size=0b10 ', 'LD1_asisdlse_R2_2v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=2} MEM:r:i64={BASE=$XnSP SZ=2x1x1x64}  ', 'Q=0b0 size=0b11 ', 'LD1_asisdlse_R2_2v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=2} MEM:r:i64={BASE=$XnSP SZ=2x2x1x64}  ', 'Q=0b1 size=0b11 ', 'LD1_asisdlse_R2_2v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=3} MEM:r:i8={ BASE=$XnSP SZ=3x8x1x8}  ', 'Q=0b0 size=0b00 ', 'LD1_asisdlse_R3_3v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=3} MEM:r:i8={ BASE=$XnSP SZ=3x16x1x8} ', 'Q=0b1 size=0b00 ', 'LD1_asisdlse_R3_3v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=3} MEM:r:i16={BASE=$XnSP SZ=3x4x1x16}  ', 'Q=0b0 size=0b01 ', 'LD1_asisdlse_R3_3v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=3} MEM:r:i16={BASE=$XnSP SZ=3x8x1x16}  ', 'Q=0b1 size=0b01 ', 'LD1_asisdlse_R3_3v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=3} MEM:r:i32={BASE=$XnSP SZ=3x2x1x32}  ', 'Q=0b0 size=0b10 ', 'LD1_asisdlse_R3_3v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=3} MEM:r:i32={BASE=$XnSP SZ=3x4x1x32}  ', 'Q=0b1 size=0b10 ', 'LD1_asisdlse_R3_3v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=3} MEM:r:i64={BASE=$XnSP SZ=3x1x1x64}  ', 'Q=0b0 size=0b11 ', 'LD1_asisdlse_R3_3v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=3} MEM:r:i64={BASE=$XnSP SZ=3x2x1x64}  ', 'Q=0b1 size=0b11 ', 'LD1_asisdlse_R3_3v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=4} MEM:r:i8={ BASE=$XnSP SZ=4x8x1x8}  ', 'Q=0b0 size=0b00 ', 'LD1_asisdlse_R4_4v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=4} MEM:r:i8={ BASE=$XnSP SZ=4x16x1x8} ', 'Q=0b1 size=0b00 ', 'LD1_asisdlse_R4_4v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=4} MEM:r:i16={BASE=$XnSP SZ=4x4x1x16}  ', 'Q=0b0 size=0b01 ', 'LD1_asisdlse_R4_4v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=4} MEM:r:i16={BASE=$XnSP SZ=4x8x1x16}  ', 'Q=0b1 size=0b01 ', 'LD1_asisdlse_R4_4v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=4} MEM:r:i32={BASE=$XnSP SZ=4x2x1x32}  ', 'Q=0b0 size=0b10 ', 'LD1_asisdlse_R4_4v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=4} MEM:r:i32={BASE=$XnSP SZ=4x4x1x32}  ', 'Q=0b1 size=0b10 ', 'LD1_asisdlse_R4_4v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=4} MEM:r:i64={BASE=$XnSP SZ=4x1x1x64}  ', 'Q=0b0 size=0b11 ', 'LD1_asisdlse_R4_4v ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=4} MEM:r:i64={BASE=$XnSP SZ=4x2x1x64}  ', 'Q=0b1 size=0b11 ', 'LD1_asisdlse_R4_4v ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=2} MEM:r:i8={ BASE=$XnSP SZ=1x8x2x8}  ', 'Q=0b0 size=0b00 ', 'LD2_asisdlse_R2    ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=2} MEM:r:i8={ BASE=$XnSP SZ=1x16x2x8} ', 'Q=0b1 size=0b00 ', 'LD2_asisdlse_R2    ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=2} MEM:r:i16={BASE=$XnSP SZ=1x4x2x16}  ', 'Q=0b0 size=0b01 ', 'LD2_asisdlse_R2    ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=2} MEM:r:i16={BASE=$XnSP SZ=1x8x2x16}  ', 'Q=0b1 size=0b01 ', 'LD2_asisdlse_R2    ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=2} MEM:r:i32={BASE=$XnSP SZ=1x2x2x32}  ', 'Q=0b0 size=0b10 ', 'LD2_asisdlse_R2    ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=2} MEM:r:i32={BASE=$XnSP SZ=1x4x2x32}  ', 'Q=0b1 size=0b10 ', 'LD2_asisdlse_R2    ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=2} MEM:r:i64={BASE=$XnSP SZ=1x2x2x64}  ', 'Q=0b1 size=0b11 ', 'LD2_asisdlse_R2    ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=3} MEM:r:i8={ BASE=$XnSP SZ=1x8x3x8}  ', 'Q=0b0 size=0b00 ', 'LD3_asisdlse_R3    ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=3} MEM:r:i8={ BASE=$XnSP SZ=1x16x3x8} ', 'Q=0b1 size=0b00 ', 'LD3_asisdlse_R3    ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=3} MEM:r:i16={BASE=$XnSP SZ=1x4x3x16}  ', 'Q=0b0 size=0b01 ', 'LD3_asisdlse_R3    ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=3} MEM:r:i16={BASE=$XnSP SZ=1x8x3x16}  ', 'Q=0b1 size=0b01 ', 'LD3_asisdlse_R3    ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=3} MEM:r:i32={BASE=$XnSP SZ=1x2x3x32}  ', 'Q=0b0 size=0b10 ', 'LD3_asisdlse_R3    ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=3} MEM:r:i32={BASE=$XnSP SZ=1x4x3x32}  ', 'Q=0b1 size=0b10 ', 'LD3_asisdlse_R3    ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=3} MEM:r:i64={BASE=$XnSP SZ=1x2x3x64}  ', 'Q=0b1 size=0b11 ', 'LD3_asisdlse_R3    ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=4} MEM:r:i8={ BASE=$XnSP SZ=1x8x4x8}  ', 'Q=0b0 size=0b00 ', 'LD4_asisdlse_R4    ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=4} MEM:r:i8={ BASE=$XnSP SZ=1x16x4x8} ', 'Q=0b1 size=0b00 ', 'LD4_asisdlse_R4    ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=4} MEM:r:i16={BASE=$XnSP SZ=1x4x4x16}  ', 'Q=0b0 size=0b01 ', 'LD4_asisdlse_R4    ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=4} MEM:r:i16={BASE=$XnSP SZ=1x8x4x16}  ', 'Q=0b1 size=0b01 ', 'LD4_asisdlse_R4    ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=4} MEM:r:i32={BASE=$XnSP SZ=1x2x4x32}  ', 'Q=0b0 size=0b10 ', 'LD4_asisdlse_R4    ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=4} MEM:r:i32={BASE=$XnSP SZ=1x4x4x32}  ', 'Q=0b1 size=0b10 ', 'LD4_asisdlse_R4    ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=4} MEM:r:i64={BASE=$XnSP SZ=1x2x4x64}  ', 'Q=0b1 size=0b11 ', 'LD4_asisdlse_R4    ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=1} MEM:w:i8={ BASE=$XnSP SZ=1x8x1x8}  ', 'Q=0b0 size=0b00 ', 'ST1_asisdlse_R1_1v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=1} MEM:w:i8={ BASE=$XnSP SZ=1x16x1x8} ', 'Q=0b1 size=0b00 ', 'ST1_asisdlse_R1_1v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=1} MEM:w:i16={BASE=$XnSP SZ=1x4x1x16}  ', 'Q=0b0 size=0b01 ', 'ST1_asisdlse_R1_1v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=1} MEM:w:i16={BASE=$XnSP SZ=1x8x1x16}  ', 'Q=0b1 size=0b01 ', 'ST1_asisdlse_R1_1v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=1} MEM:w:i32={BASE=$XnSP SZ=1x2x1x32}  ', 'Q=0b0 size=0b10 ', 'ST1_asisdlse_R1_1v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=1} MEM:w:i32={BASE=$XnSP SZ=1x4x1x32}  ', 'Q=0b1 size=0b10 ', 'ST1_asisdlse_R1_1v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=1D } COUNT=1} MEM:w:i64={BASE=$XnSP SZ=1x1x1x64}  ', 'Q=0b0 size=0b11 ', 'ST1_asisdlse_R1_1v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=1} MEM:w:i64={BASE=$XnSP SZ=1x2x1x64}  ', 'Q=0b1 size=0b11 ', 'ST1_asisdlse_R1_1v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=2} MEM:w:i8={ BASE=$XnSP SZ=2x8x1x8}  ', 'Q=0b0 size=0b00 ', 'ST1_asisdlse_R2_2v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=2} MEM:w:i8={ BASE=$XnSP SZ=2x16x1x8} ', 'Q=0b1 size=0b00 ', 'ST1_asisdlse_R2_2v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=2} MEM:w:i16={BASE=$XnSP SZ=2x4x1x16}  ', 'Q=0b0 size=0b01 ', 'ST1_asisdlse_R2_2v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=2} MEM:w:i16={BASE=$XnSP SZ=2x8x1x16}  ', 'Q=0b1 size=0b01 ', 'ST1_asisdlse_R2_2v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=2} MEM:w:i32={BASE=$XnSP SZ=2x2x1x32}  ', 'Q=0b0 size=0b10 ', 'ST1_asisdlse_R2_2v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=2} MEM:w:i32={BASE=$XnSP SZ=2x4x1x32}  ', 'Q=0b1 size=0b10 ', 'ST1_asisdlse_R2_2v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=1D } COUNT=2} MEM:w:i64={BASE=$XnSP SZ=2x1x1x64}  ', 'Q=0b0 size=0b11 ', 'ST1_asisdlse_R2_2v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=2} MEM:w:i64={BASE=$XnSP SZ=2x2x1x64}  ', 'Q=0b1 size=0b11 ', 'ST1_asisdlse_R2_2v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=3} MEM:w:i8={ BASE=$XnSP SZ=3x8x1x8}  ', 'Q=0b0 size=0b00 ', 'ST1_asisdlse_R3_3v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=3} MEM:w:i8={ BASE=$XnSP SZ=3x16x1x8} ', 'Q=0b1 size=0b00 ', 'ST1_asisdlse_R3_3v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=3} MEM:w:i16={BASE=$XnSP SZ=3x4x1x16}  ', 'Q=0b0 size=0b01 ', 'ST1_asisdlse_R3_3v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=3} MEM:w:i16={BASE=$XnSP SZ=3x8x1x16}  ', 'Q=0b1 size=0b01 ', 'ST1_asisdlse_R3_3v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=3} MEM:w:i32={BASE=$XnSP SZ=3x2x1x32}  ', 'Q=0b0 size=0b10 ', 'ST1_asisdlse_R3_3v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=3} MEM:w:i32={BASE=$XnSP SZ=3x4x1x32}  ', 'Q=0b1 size=0b10 ', 'ST1_asisdlse_R3_3v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=1D } COUNT=3} MEM:w:i64={BASE=$XnSP SZ=3x1x1x64}  ', 'Q=0b0 size=0b11 ', 'ST1_asisdlse_R3_3v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=3} MEM:w:i64={BASE=$XnSP SZ=3x2x1x64}  ', 'Q=0b1 size=0b11 ', 'ST1_asisdlse_R3_3v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=4} MEM:w:i8={ BASE=$XnSP SZ=4x8x1x8}  ', 'Q=0b0 size=0b00 ', 'ST1_asisdlse_R4_4v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=4} MEM:w:i8={ BASE=$XnSP SZ=4x16x1x8} ', 'Q=0b1 size=0b00 ', 'ST1_asisdlse_R4_4v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=4} MEM:w:i16={BASE=$XnSP SZ=4x4x1x16}  ', 'Q=0b0 size=0b01 ', 'ST1_asisdlse_R4_4v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=4} MEM:w:i16={BASE=$XnSP SZ=4x8x1x16}  ', 'Q=0b1 size=0b01 ', 'ST1_asisdlse_R4_4v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=4} MEM:w:i32={BASE=$XnSP SZ=4x2x1x32}  ', 'Q=0b0 size=0b10 ', 'ST1_asisdlse_R4_4v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=4} MEM:w:i32={BASE=$XnSP SZ=4x4x1x32}  ', 'Q=0b1 size=0b10 ', 'ST1_asisdlse_R4_4v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=1D } COUNT=4} MEM:w:i64={BASE=$XnSP SZ=4x1x1x64}  ', 'Q=0b0 size=0b11 ', 'ST1_asisdlse_R4_4v ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=4} MEM:w:i64={BASE=$XnSP SZ=4x2x1x64}  ', 'Q=0b1 size=0b11 ', 'ST1_asisdlse_R4_4v ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=2} MEM:w:i8={ BASE=$XnSP SZ=1x8x2x8}  ', 'Q=0b0 size=0b00 ', 'ST2_asisdlse_R2    ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=2} MEM:w:i8={ BASE=$XnSP SZ=1x16x2x8} ', 'Q=0b1 size=0b00 ', 'ST2_asisdlse_R2    ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=2} MEM:w:i16={BASE=$XnSP SZ=1x4x2x16}  ', 'Q=0b0 size=0b01 ', 'ST2_asisdlse_R2    ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=2} MEM:w:i16={BASE=$XnSP SZ=1x8x2x16}  ', 'Q=0b1 size=0b01 ', 'ST2_asisdlse_R2    ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=2} MEM:w:i32={BASE=$XnSP SZ=1x2x2x32}  ', 'Q=0b0 size=0b10 ', 'ST2_asisdlse_R2    ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=2} MEM:w:i32={BASE=$XnSP SZ=1x4x2x32}  ', 'Q=0b1 size=0b10 ', 'ST2_asisdlse_R2    ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=2} MEM:w:i64={BASE=$XnSP SZ=1x2x2x64}  ', 'Q=0b1 size=0b11 ', 'ST2_asisdlse_R2    ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=3} MEM:w:i8={ BASE=$XnSP SZ=1x8x3x8}  ', 'Q=0b0 size=0b00 ', 'ST3_asisdlse_R3    ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=3} MEM:w:i8={ BASE=$XnSP SZ=1x16x3x8} ', 'Q=0b1 size=0b00 ', 'ST3_asisdlse_R3    ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=3} MEM:w:i16={BASE=$XnSP SZ=1x4x3x16}  ', 'Q=0b0 size=0b01 ', 'ST3_asisdlse_R3    ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=3} MEM:w:i16={BASE=$XnSP SZ=1x8x3x16}  ', 'Q=0b1 size=0b01 ', 'ST3_asisdlse_R3    ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=3} MEM:w:i32={BASE=$XnSP SZ=1x2x3x32}  ', 'Q=0b0 size=0b10 ', 'ST3_asisdlse_R3    ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=3} MEM:w:i32={BASE=$XnSP SZ=1x4x3x32}  ', 'Q=0b1 size=0b10 ', 'ST3_asisdlse_R3    ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=3} MEM:w:i64={BASE=$XnSP SZ=1x2x3x64}  ', 'Q=0b1 size=0b11 ', 'ST3_asisdlse_R3    ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=4} MEM:w:i8={ BASE=$XnSP SZ=1x8x4x8}  ', 'Q=0b0 size=0b00 ', 'ST4_asisdlse_R4    ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=4} MEM:w:i8={ BASE=$XnSP SZ=1x16x4x8} ', 'Q=0b1 size=0b00 ', 'ST4_asisdlse_R4    ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=4} MEM:w:i16={BASE=$XnSP SZ=1x4x4x16}  ', 'Q=0b0 size=0b01 ', 'ST4_asisdlse_R4    ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=4} MEM:w:i16={BASE=$XnSP SZ=1x8x4x16}  ', 'Q=0b1 size=0b01 ', 'ST4_asisdlse_R4    ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=4} MEM:w:i32={BASE=$XnSP SZ=1x2x4x32}  ', 'Q=0b0 size=0b10 ', 'ST4_asisdlse_R4    ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=4} MEM:w:i32={BASE=$XnSP SZ=1x4x4x32}  ', 'Q=0b1 size=0b10 ', 'ST4_asisdlse_R4    ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=4} MEM:w:i64={BASE=$XnSP SZ=1x2x4x64}  ', 'Q=0b1 size=0b11 ', 'ST4_asisdlse_R4    ', ' '];



# asisdlsep: Advanced SIMD load/store multiple structures (post-indexed).
# A64: ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0bx ig2=0b0 Rm:u=0bxxxxx opcode:u=0bxxxx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'asisdlsep', 'A64');

DEF Vt     => VECREG('Vt', 'Rt')              ;
DEF Xmnxzr => GPR64('Xm', 'Rm', 'read|noxzr') ;
DEF XnSP   => GPR64('Xn', 'Rn', 'read|SP')    ;

T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=1} MEM:PST:r:i8={ BASE=$XnSP SZ=1x8x1x8 IMMOFF:u8=8}     ', 'Q=0b0 size=0b00 ', 'LD1_asisdlsep_I1_i1 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=1} MEM:PST:r:i8={ BASE=$XnSP SZ=1x16x1x8 IMMOFF:u8=16}   ', 'Q=0b1 size=0b00 ', 'LD1_asisdlsep_I1_i1 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=1} MEM:PST:r:i16={BASE=$XnSP SZ=1x4x1x16 IMMOFF:u8=8}     ', 'Q=0b0 size=0b01 ', 'LD1_asisdlsep_I1_i1 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=1} MEM:PST:r:i16={BASE=$XnSP SZ=1x8x1x16 IMMOFF:u8=16}    ', 'Q=0b1 size=0b01 ', 'LD1_asisdlsep_I1_i1 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=1} MEM:PST:r:i32={BASE=$XnSP SZ=1x2x1x32 IMMOFF:u8=8}     ', 'Q=0b0 size=0b10 ', 'LD1_asisdlsep_I1_i1 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=1} MEM:PST:r:i32={BASE=$XnSP SZ=1x4x1x32 IMMOFF:u8=16}    ', 'Q=0b1 size=0b10 ', 'LD1_asisdlsep_I1_i1 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=1} MEM:PST:r:i64={BASE=$XnSP SZ=1x1x1x64 IMMOFF:u8=8}     ', 'Q=0b0 size=0b11 ', 'LD1_asisdlsep_I1_i1 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=1} MEM:PST:r:i64={BASE=$XnSP SZ=1x2x1x64 IMMOFF:u8=16}    ', 'Q=0b1 size=0b11 ', 'LD1_asisdlsep_I1_i1 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=1} MEM:PST:r:i8={ BASE=$XnSP SZ=1x8x1x8 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b00 ', 'LD1_asisdlsep_R1_r1 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=1} MEM:PST:r:i8={ BASE=$XnSP SZ=1x16x1x8 REGOFF=$Xmnxzr} ', 'Q=0b1 size=0b00 ', 'LD1_asisdlsep_R1_r1 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=1} MEM:PST:r:i16={BASE=$XnSP SZ=1x4x1x16 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b01 ', 'LD1_asisdlsep_R1_r1 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=1} MEM:PST:r:i16={BASE=$XnSP SZ=1x8x1x16 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b01 ', 'LD1_asisdlsep_R1_r1 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=1} MEM:PST:r:i32={BASE=$XnSP SZ=1x2x1x32 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b10 ', 'LD1_asisdlsep_R1_r1 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=1} MEM:PST:r:i32={BASE=$XnSP SZ=1x4x1x32 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b10 ', 'LD1_asisdlsep_R1_r1 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=1} MEM:PST:r:i64={BASE=$XnSP SZ=1x1x1x64 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b11 ', 'LD1_asisdlsep_R1_r1 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=1} MEM:PST:r:i64={BASE=$XnSP SZ=1x2x1x64 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b11 ', 'LD1_asisdlsep_R1_r1 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=2} MEM:PST:r:i8={ BASE=$XnSP SZ=2x8x1x8 IMMOFF:u8=16}    ', 'Q=0b0 size=0b00 ', 'LD1_asisdlsep_I2_i2 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=2} MEM:PST:r:i8={ BASE=$XnSP SZ=2x16x1x8 IMMOFF:u8=32}   ', 'Q=0b1 size=0b00 ', 'LD1_asisdlsep_I2_i2 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=2} MEM:PST:r:i16={BASE=$XnSP SZ=2x4x1x16 IMMOFF:u8=16}    ', 'Q=0b0 size=0b01 ', 'LD1_asisdlsep_I2_i2 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=2} MEM:PST:r:i16={BASE=$XnSP SZ=2x8x1x16 IMMOFF:u8=32}    ', 'Q=0b1 size=0b01 ', 'LD1_asisdlsep_I2_i2 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=2} MEM:PST:r:i32={BASE=$XnSP SZ=2x2x1x32 IMMOFF:u8=16}    ', 'Q=0b0 size=0b10 ', 'LD1_asisdlsep_I2_i2 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=2} MEM:PST:r:i32={BASE=$XnSP SZ=2x4x1x32 IMMOFF:u8=32}    ', 'Q=0b1 size=0b10 ', 'LD1_asisdlsep_I2_i2 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=2} MEM:PST:r:i64={BASE=$XnSP SZ=2x1x1x64 IMMOFF:u8=16}    ', 'Q=0b0 size=0b11 ', 'LD1_asisdlsep_I2_i2 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=2} MEM:PST:r:i64={BASE=$XnSP SZ=2x2x1x64 IMMOFF:u8=32}    ', 'Q=0b1 size=0b11 ', 'LD1_asisdlsep_I2_i2 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=2} MEM:PST:r:i8={ BASE=$XnSP SZ=2x8x1x8 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b00 ', 'LD1_asisdlsep_R2_r2 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=2} MEM:PST:r:i8={ BASE=$XnSP SZ=2x16x1x8 REGOFF=$Xmnxzr} ', 'Q=0b1 size=0b00 ', 'LD1_asisdlsep_R2_r2 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=2} MEM:PST:r:i16={BASE=$XnSP SZ=2x4x1x16 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b01 ', 'LD1_asisdlsep_R2_r2 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=2} MEM:PST:r:i16={BASE=$XnSP SZ=2x8x1x16 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b01 ', 'LD1_asisdlsep_R2_r2 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=2} MEM:PST:r:i32={BASE=$XnSP SZ=2x2x1x32 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b10 ', 'LD1_asisdlsep_R2_r2 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=2} MEM:PST:r:i32={BASE=$XnSP SZ=2x4x1x32 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b10 ', 'LD1_asisdlsep_R2_r2 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=2} MEM:PST:r:i64={BASE=$XnSP SZ=2x1x1x64 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b11 ', 'LD1_asisdlsep_R2_r2 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=2} MEM:PST:r:i64={BASE=$XnSP SZ=2x2x1x64 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b11 ', 'LD1_asisdlsep_R2_r2 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=3} MEM:PST:r:i8={ BASE=$XnSP SZ=3x8x1x8 IMMOFF:u8=24}    ', 'Q=0b0 size=0b00 ', 'LD1_asisdlsep_I3_i3 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=3} MEM:PST:r:i8={ BASE=$XnSP SZ=3x16x1x8 IMMOFF:u8=48}   ', 'Q=0b1 size=0b00 ', 'LD1_asisdlsep_I3_i3 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=3} MEM:PST:r:i16={BASE=$XnSP SZ=3x4x1x16 IMMOFF:u8=24}    ', 'Q=0b0 size=0b01 ', 'LD1_asisdlsep_I3_i3 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=3} MEM:PST:r:i16={BASE=$XnSP SZ=3x8x1x16 IMMOFF:u8=48}    ', 'Q=0b1 size=0b01 ', 'LD1_asisdlsep_I3_i3 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=3} MEM:PST:r:i32={BASE=$XnSP SZ=3x2x1x32 IMMOFF:u8=24}    ', 'Q=0b0 size=0b10 ', 'LD1_asisdlsep_I3_i3 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=3} MEM:PST:r:i32={BASE=$XnSP SZ=3x4x1x32 IMMOFF:u8=48}    ', 'Q=0b1 size=0b10 ', 'LD1_asisdlsep_I3_i3 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=3} MEM:PST:r:i64={BASE=$XnSP SZ=3x1x1x64 IMMOFF:u8=24}    ', 'Q=0b0 size=0b11 ', 'LD1_asisdlsep_I3_i3 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=3} MEM:PST:r:i64={BASE=$XnSP SZ=3x2x1x64 IMMOFF:u8=48}    ', 'Q=0b1 size=0b11 ', 'LD1_asisdlsep_I3_i3 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=3} MEM:PST:r:i8={ BASE=$XnSP SZ=3x8x1x8 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b00 ', 'LD1_asisdlsep_R3_r3 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=3} MEM:PST:r:i8={ BASE=$XnSP SZ=3x16x1x8 REGOFF=$Xmnxzr} ', 'Q=0b1 size=0b00 ', 'LD1_asisdlsep_R3_r3 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=3} MEM:PST:r:i16={BASE=$XnSP SZ=3x4x1x16 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b01 ', 'LD1_asisdlsep_R3_r3 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=3} MEM:PST:r:i16={BASE=$XnSP SZ=3x8x1x16 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b01 ', 'LD1_asisdlsep_R3_r3 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=3} MEM:PST:r:i32={BASE=$XnSP SZ=3x2x1x32 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b10 ', 'LD1_asisdlsep_R3_r3 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=3} MEM:PST:r:i32={BASE=$XnSP SZ=3x4x1x32 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b10 ', 'LD1_asisdlsep_R3_r3 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=3} MEM:PST:r:i64={BASE=$XnSP SZ=3x1x1x64 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b11 ', 'LD1_asisdlsep_R3_r3 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=3} MEM:PST:r:i64={BASE=$XnSP SZ=3x2x1x64 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b11 ', 'LD1_asisdlsep_R3_r3 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=4} MEM:PST:r:i8={ BASE=$XnSP SZ=4x8x1x8 IMMOFF:u8=32}    ', 'Q=0b0 size=0b00 ', 'LD1_asisdlsep_I4_i4 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=4} MEM:PST:r:i8={ BASE=$XnSP SZ=4x16x1x8 IMMOFF:u8=64}   ', 'Q=0b1 size=0b00 ', 'LD1_asisdlsep_I4_i4 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=4} MEM:PST:r:i16={BASE=$XnSP SZ=4x4x1x16 IMMOFF:u8=32}    ', 'Q=0b0 size=0b01 ', 'LD1_asisdlsep_I4_i4 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=4} MEM:PST:r:i16={BASE=$XnSP SZ=4x8x1x16 IMMOFF:u8=64}    ', 'Q=0b1 size=0b01 ', 'LD1_asisdlsep_I4_i4 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=4} MEM:PST:r:i32={BASE=$XnSP SZ=4x2x1x32 IMMOFF:u8=32}    ', 'Q=0b0 size=0b10 ', 'LD1_asisdlsep_I4_i4 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=4} MEM:PST:r:i32={BASE=$XnSP SZ=4x4x1x32 IMMOFF:u8=64}    ', 'Q=0b1 size=0b10 ', 'LD1_asisdlsep_I4_i4 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=4} MEM:PST:r:i64={BASE=$XnSP SZ=4x1x1x64 IMMOFF:u8=32}    ', 'Q=0b0 size=0b11 ', 'LD1_asisdlsep_I4_i4 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=4} MEM:PST:r:i64={BASE=$XnSP SZ=4x2x1x64 IMMOFF:u8=64}    ', 'Q=0b1 size=0b11 ', 'LD1_asisdlsep_I4_i4 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=4} MEM:PST:r:i8={ BASE=$XnSP SZ=4x8x1x8 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b00 ', 'LD1_asisdlsep_R4_r4 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=4} MEM:PST:r:i8={ BASE=$XnSP SZ=4x16x1x8 REGOFF=$Xmnxzr} ', 'Q=0b1 size=0b00 ', 'LD1_asisdlsep_R4_r4 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=4} MEM:PST:r:i16={BASE=$XnSP SZ=4x4x1x16 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b01 ', 'LD1_asisdlsep_R4_r4 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=4} MEM:PST:r:i16={BASE=$XnSP SZ=4x8x1x16 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b01 ', 'LD1_asisdlsep_R4_r4 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=4} MEM:PST:r:i32={BASE=$XnSP SZ=4x2x1x32 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b10 ', 'LD1_asisdlsep_R4_r4 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=4} MEM:PST:r:i32={BASE=$XnSP SZ=4x4x1x32 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b10 ', 'LD1_asisdlsep_R4_r4 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=4} MEM:PST:r:i64={BASE=$XnSP SZ=4x1x1x64 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b11 ', 'LD1_asisdlsep_R4_r4 ', ' '];
T[' LD1 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=4} MEM:PST:r:i64={BASE=$XnSP SZ=4x2x1x64 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b11 ', 'LD1_asisdlsep_R4_r4 ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=2} MEM:PST:r:i8={ BASE=$XnSP SZ=1x8x2x8 IMMOFF:u8=16}    ', 'Q=0b0 size=0b00 ', 'LD2_asisdlsep_I2_i  ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=2} MEM:PST:r:i8={ BASE=$XnSP SZ=1x16x2x8 IMMOFF:u8=32}   ', 'Q=0b1 size=0b00 ', 'LD2_asisdlsep_I2_i  ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=2} MEM:PST:r:i16={BASE=$XnSP SZ=1x4x2x16 IMMOFF:u8=16}    ', 'Q=0b0 size=0b01 ', 'LD2_asisdlsep_I2_i  ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=2} MEM:PST:r:i16={BASE=$XnSP SZ=1x8x2x16 IMMOFF:u8=32}    ', 'Q=0b1 size=0b01 ', 'LD2_asisdlsep_I2_i  ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=2} MEM:PST:r:i32={BASE=$XnSP SZ=1x2x2x32 IMMOFF:u8=16}    ', 'Q=0b0 size=0b10 ', 'LD2_asisdlsep_I2_i  ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=2} MEM:PST:r:i32={BASE=$XnSP SZ=1x4x2x32 IMMOFF:u8=32}    ', 'Q=0b1 size=0b10 ', 'LD2_asisdlsep_I2_i  ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=2} MEM:PST:r:i64={BASE=$XnSP SZ=1x2x2x64 IMMOFF:u8=32}    ', 'Q=0b1 size=0b11 ', 'LD2_asisdlsep_I2_i  ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=2} MEM:PST:r:i8={ BASE=$XnSP SZ=1x8x2x8 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b00 ', 'LD2_asisdlsep_R2_r  ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=2} MEM:PST:r:i8={ BASE=$XnSP SZ=1x16x2x8 REGOFF=$Xmnxzr} ', 'Q=0b1 size=0b00 ', 'LD2_asisdlsep_R2_r  ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=2} MEM:PST:r:i16={BASE=$XnSP SZ=1x4x2x16 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b01 ', 'LD2_asisdlsep_R2_r  ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=2} MEM:PST:r:i16={BASE=$XnSP SZ=1x8x2x16 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b01 ', 'LD2_asisdlsep_R2_r  ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=2} MEM:PST:r:i32={BASE=$XnSP SZ=1x2x2x32 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b10 ', 'LD2_asisdlsep_R2_r  ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=2} MEM:PST:r:i32={BASE=$XnSP SZ=1x4x2x32 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b10 ', 'LD2_asisdlsep_R2_r  ', ' '];
T[' LD2 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=2} MEM:PST:r:i64={BASE=$XnSP SZ=1x2x2x64 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b11 ', 'LD2_asisdlsep_R2_r  ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=3} MEM:PST:r:i8={ BASE=$XnSP SZ=1x8x3x8 IMMOFF:u8=24}    ', 'Q=0b0 size=0b00 ', 'LD3_asisdlsep_I3_i  ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=3} MEM:PST:r:i8={ BASE=$XnSP SZ=1x16x3x8 IMMOFF:u8=48}   ', 'Q=0b1 size=0b00 ', 'LD3_asisdlsep_I3_i  ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=3} MEM:PST:r:i16={BASE=$XnSP SZ=1x4x3x16 IMMOFF:u8=24}    ', 'Q=0b0 size=0b01 ', 'LD3_asisdlsep_I3_i  ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=3} MEM:PST:r:i16={BASE=$XnSP SZ=1x8x3x16 IMMOFF:u8=48}    ', 'Q=0b1 size=0b01 ', 'LD3_asisdlsep_I3_i  ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=3} MEM:PST:r:i32={BASE=$XnSP SZ=1x2x3x32 IMMOFF:u8=24}    ', 'Q=0b0 size=0b10 ', 'LD3_asisdlsep_I3_i  ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=3} MEM:PST:r:i32={BASE=$XnSP SZ=1x4x3x32 IMMOFF:u8=48}    ', 'Q=0b1 size=0b10 ', 'LD3_asisdlsep_I3_i  ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=3} MEM:PST:r:i64={BASE=$XnSP SZ=1x2x3x64 IMMOFF:u8=48}    ', 'Q=0b1 size=0b11 ', 'LD3_asisdlsep_I3_i  ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=3} MEM:PST:r:i8={ BASE=$XnSP SZ=1x8x3x8 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b00 ', 'LD3_asisdlsep_R3_r  ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=3} MEM:PST:r:i8={ BASE=$XnSP SZ=1x16x3x8 REGOFF=$Xmnxzr} ', 'Q=0b1 size=0b00 ', 'LD3_asisdlsep_R3_r  ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=3} MEM:PST:r:i16={BASE=$XnSP SZ=1x4x3x16 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b01 ', 'LD3_asisdlsep_R3_r  ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=3} MEM:PST:r:i16={BASE=$XnSP SZ=1x8x3x16 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b01 ', 'LD3_asisdlsep_R3_r  ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=3} MEM:PST:r:i32={BASE=$XnSP SZ=1x2x3x32 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b10 ', 'LD3_asisdlsep_R3_r  ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=3} MEM:PST:r:i32={BASE=$XnSP SZ=1x4x3x32 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b10 ', 'LD3_asisdlsep_R3_r  ', ' '];
T[' LD3 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=3} MEM:PST:r:i64={BASE=$XnSP SZ=1x2x3x64 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b11 ', 'LD3_asisdlsep_R3_r  ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=4} MEM:PST:r:i8={ BASE=$XnSP SZ=1x8x4x8 IMMOFF:u8=32}    ', 'Q=0b0 size=0b00 ', 'LD4_asisdlsep_I4_i  ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=4} MEM:PST:r:i8={ BASE=$XnSP SZ=1x16x4x8 IMMOFF:u8=64}   ', 'Q=0b1 size=0b00 ', 'LD4_asisdlsep_I4_i  ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=4} MEM:PST:r:i16={BASE=$XnSP SZ=1x4x4x16 IMMOFF:u8=32}    ', 'Q=0b0 size=0b01 ', 'LD4_asisdlsep_I4_i  ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=4} MEM:PST:r:i16={BASE=$XnSP SZ=1x8x4x16 IMMOFF:u8=64}    ', 'Q=0b1 size=0b01 ', 'LD4_asisdlsep_I4_i  ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=4} MEM:PST:r:i32={BASE=$XnSP SZ=1x2x4x32 IMMOFF:u8=32}    ', 'Q=0b0 size=0b10 ', 'LD4_asisdlsep_I4_i  ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=4} MEM:PST:r:i32={BASE=$XnSP SZ=1x4x4x32 IMMOFF:u8=64}    ', 'Q=0b1 size=0b10 ', 'LD4_asisdlsep_I4_i  ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=4} MEM:PST:r:i64={BASE=$XnSP SZ=1x2x4x64 IMMOFF:u8=64}    ', 'Q=0b1 size=0b11 ', 'LD4_asisdlsep_I4_i  ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=4} MEM:PST:r:i8={ BASE=$XnSP SZ=1x8x4x8 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b00 ', 'LD4_asisdlsep_R4_r  ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=4} MEM:PST:r:i8={ BASE=$XnSP SZ=1x16x4x8 REGOFF=$Xmnxzr} ', 'Q=0b1 size=0b00 ', 'LD4_asisdlsep_R4_r  ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=4} MEM:PST:r:i16={BASE=$XnSP SZ=1x4x4x16 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b01 ', 'LD4_asisdlsep_R4_r  ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=4} MEM:PST:r:i16={BASE=$XnSP SZ=1x8x4x16 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b01 ', 'LD4_asisdlsep_R4_r  ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=4} MEM:PST:r:i32={BASE=$XnSP SZ=1x2x4x32 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b10 ', 'LD4_asisdlsep_R4_r  ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=4} MEM:PST:r:i32={BASE=$XnSP SZ=1x4x4x32 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b10 ', 'LD4_asisdlsep_R4_r  ', ' '];
T[' LD4 VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=4} MEM:PST:r:i64={BASE=$XnSP SZ=1x2x4x64 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b11 ', 'LD4_asisdlsep_R4_r  ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=1} MEM:PST:w:i8={ BASE=$XnSP SZ=1x8x1x8 IMMOFF:u8=8}     ', 'Q=0b0 size=0b00 ', 'ST1_asisdlsep_I1_i1 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=1} MEM:PST:w:i8={ BASE=$XnSP SZ=1x16x1x8 IMMOFF:u8=16}   ', 'Q=0b1 size=0b00 ', 'ST1_asisdlsep_I1_i1 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=1} MEM:PST:w:i16={BASE=$XnSP SZ=1x4x1x16 IMMOFF:u8=8}     ', 'Q=0b0 size=0b01 ', 'ST1_asisdlsep_I1_i1 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=1} MEM:PST:w:i16={BASE=$XnSP SZ=1x8x1x16 IMMOFF:u8=16}    ', 'Q=0b1 size=0b01 ', 'ST1_asisdlsep_I1_i1 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=1} MEM:PST:w:i32={BASE=$XnSP SZ=1x2x1x32 IMMOFF:u8=8}     ', 'Q=0b0 size=0b10 ', 'ST1_asisdlsep_I1_i1 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=1} MEM:PST:w:i32={BASE=$XnSP SZ=1x4x1x32 IMMOFF:u8=16}    ', 'Q=0b1 size=0b10 ', 'ST1_asisdlsep_I1_i1 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=1D } COUNT=1} MEM:PST:w:i64={BASE=$XnSP SZ=1x1x1x64 IMMOFF:u8=8}     ', 'Q=0b0 size=0b11 ', 'ST1_asisdlsep_I1_i1 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=1} MEM:PST:w:i64={BASE=$XnSP SZ=1x2x1x64 IMMOFF:u8=16}    ', 'Q=0b1 size=0b11 ', 'ST1_asisdlsep_I1_i1 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=1} MEM:PST:w:i8={ BASE=$XnSP SZ=1x8x1x8 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b00 ', 'ST1_asisdlsep_R1_r1 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=1} MEM:PST:w:i8={ BASE=$XnSP SZ=1x16x1x8 REGOFF=$Xmnxzr} ', 'Q=0b1 size=0b00 ', 'ST1_asisdlsep_R1_r1 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=1} MEM:PST:w:i16={BASE=$XnSP SZ=1x4x1x16 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b01 ', 'ST1_asisdlsep_R1_r1 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=1} MEM:PST:w:i16={BASE=$XnSP SZ=1x8x1x16 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b01 ', 'ST1_asisdlsep_R1_r1 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=1} MEM:PST:w:i32={BASE=$XnSP SZ=1x2x1x32 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b10 ', 'ST1_asisdlsep_R1_r1 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=1} MEM:PST:w:i32={BASE=$XnSP SZ=1x4x1x32 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b10 ', 'ST1_asisdlsep_R1_r1 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=1D } COUNT=1} MEM:PST:w:i64={BASE=$XnSP SZ=1x1x1x64 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b11 ', 'ST1_asisdlsep_R1_r1 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=1} MEM:PST:w:i64={BASE=$XnSP SZ=1x2x1x64 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b11 ', 'ST1_asisdlsep_R1_r1 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=2} MEM:PST:w:i8={ BASE=$XnSP SZ=2x8x1x8 IMMOFF:u8=16}    ', 'Q=0b0 size=0b00 ', 'ST1_asisdlsep_I2_i2 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=2} MEM:PST:w:i8={ BASE=$XnSP SZ=2x16x1x8 IMMOFF:u8=32}   ', 'Q=0b1 size=0b00 ', 'ST1_asisdlsep_I2_i2 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=2} MEM:PST:w:i16={BASE=$XnSP SZ=2x4x1x16 IMMOFF:u8=16}    ', 'Q=0b0 size=0b01 ', 'ST1_asisdlsep_I2_i2 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=2} MEM:PST:w:i16={BASE=$XnSP SZ=2x8x1x16 IMMOFF:u8=32}    ', 'Q=0b1 size=0b01 ', 'ST1_asisdlsep_I2_i2 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=2} MEM:PST:w:i32={BASE=$XnSP SZ=2x2x1x32 IMMOFF:u8=16}    ', 'Q=0b0 size=0b10 ', 'ST1_asisdlsep_I2_i2 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=2} MEM:PST:w:i32={BASE=$XnSP SZ=2x4x1x32 IMMOFF:u8=32}    ', 'Q=0b1 size=0b10 ', 'ST1_asisdlsep_I2_i2 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=1D } COUNT=2} MEM:PST:w:i64={BASE=$XnSP SZ=2x1x1x64 IMMOFF:u8=16}    ', 'Q=0b0 size=0b11 ', 'ST1_asisdlsep_I2_i2 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=2} MEM:PST:w:i64={BASE=$XnSP SZ=2x2x1x64 IMMOFF:u8=32}    ', 'Q=0b1 size=0b11 ', 'ST1_asisdlsep_I2_i2 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=2} MEM:PST:w:i8={ BASE=$XnSP SZ=2x8x1x8 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b00 ', 'ST1_asisdlsep_R2_r2 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=2} MEM:PST:w:i8={ BASE=$XnSP SZ=2x16x1x8 REGOFF=$Xmnxzr} ', 'Q=0b1 size=0b00 ', 'ST1_asisdlsep_R2_r2 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=2} MEM:PST:w:i16={BASE=$XnSP SZ=2x4x1x16 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b01 ', 'ST1_asisdlsep_R2_r2 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=2} MEM:PST:w:i16={BASE=$XnSP SZ=2x8x1x16 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b01 ', 'ST1_asisdlsep_R2_r2 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=2} MEM:PST:w:i32={BASE=$XnSP SZ=2x2x1x32 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b10 ', 'ST1_asisdlsep_R2_r2 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=2} MEM:PST:w:i32={BASE=$XnSP SZ=2x4x1x32 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b10 ', 'ST1_asisdlsep_R2_r2 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=1D } COUNT=2} MEM:PST:w:i64={BASE=$XnSP SZ=2x1x1x64 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b11 ', 'ST1_asisdlsep_R2_r2 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=2} MEM:PST:w:i64={BASE=$XnSP SZ=2x2x1x64 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b11 ', 'ST1_asisdlsep_R2_r2 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=3} MEM:PST:w:i8={ BASE=$XnSP SZ=3x8x1x8 IMMOFF:u8=24}    ', 'Q=0b0 size=0b00 ', 'ST1_asisdlsep_I3_i3 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=3} MEM:PST:w:i8={ BASE=$XnSP SZ=3x16x1x8 IMMOFF:u8=48}   ', 'Q=0b1 size=0b00 ', 'ST1_asisdlsep_I3_i3 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=3} MEM:PST:w:i16={BASE=$XnSP SZ=3x4x1x16 IMMOFF:u8=24}    ', 'Q=0b0 size=0b01 ', 'ST1_asisdlsep_I3_i3 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=3} MEM:PST:w:i16={BASE=$XnSP SZ=3x8x1x16 IMMOFF:u8=48}    ', 'Q=0b1 size=0b01 ', 'ST1_asisdlsep_I3_i3 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=3} MEM:PST:w:i32={BASE=$XnSP SZ=3x2x1x32 IMMOFF:u8=24}    ', 'Q=0b0 size=0b10 ', 'ST1_asisdlsep_I3_i3 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=3} MEM:PST:w:i32={BASE=$XnSP SZ=3x4x1x32 IMMOFF:u8=48}    ', 'Q=0b1 size=0b10 ', 'ST1_asisdlsep_I3_i3 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=1D } COUNT=3} MEM:PST:w:i64={BASE=$XnSP SZ=3x1x1x64 IMMOFF:u8=24}    ', 'Q=0b0 size=0b11 ', 'ST1_asisdlsep_I3_i3 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=3} MEM:PST:w:i64={BASE=$XnSP SZ=3x2x1x64 IMMOFF:u8=48}    ', 'Q=0b1 size=0b11 ', 'ST1_asisdlsep_I3_i3 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=3} MEM:PST:w:i8={ BASE=$XnSP SZ=3x8x1x8 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b00 ', 'ST1_asisdlsep_R3_r3 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=3} MEM:PST:w:i8={ BASE=$XnSP SZ=3x16x1x8 REGOFF=$Xmnxzr} ', 'Q=0b1 size=0b00 ', 'ST1_asisdlsep_R3_r3 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=3} MEM:PST:w:i16={BASE=$XnSP SZ=3x4x1x16 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b01 ', 'ST1_asisdlsep_R3_r3 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=3} MEM:PST:w:i16={BASE=$XnSP SZ=3x8x1x16 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b01 ', 'ST1_asisdlsep_R3_r3 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=3} MEM:PST:w:i32={BASE=$XnSP SZ=3x2x1x32 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b10 ', 'ST1_asisdlsep_R3_r3 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=3} MEM:PST:w:i32={BASE=$XnSP SZ=3x4x1x32 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b10 ', 'ST1_asisdlsep_R3_r3 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=1D } COUNT=3} MEM:PST:w:i64={BASE=$XnSP SZ=3x1x1x64 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b11 ', 'ST1_asisdlsep_R3_r3 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=3} MEM:PST:w:i64={BASE=$XnSP SZ=3x2x1x64 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b11 ', 'ST1_asisdlsep_R3_r3 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=4} MEM:PST:w:i8={ BASE=$XnSP SZ=4x8x1x8 IMMOFF:u8=32}    ', 'Q=0b0 size=0b00 ', 'ST1_asisdlsep_I4_i4 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=4} MEM:PST:w:i8={ BASE=$XnSP SZ=4x16x1x8 IMMOFF:u8=64}   ', 'Q=0b1 size=0b00 ', 'ST1_asisdlsep_I4_i4 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=4} MEM:PST:w:i16={BASE=$XnSP SZ=4x4x1x16 IMMOFF:u8=32}    ', 'Q=0b0 size=0b01 ', 'ST1_asisdlsep_I4_i4 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=4} MEM:PST:w:i16={BASE=$XnSP SZ=4x8x1x16 IMMOFF:u8=64}    ', 'Q=0b1 size=0b01 ', 'ST1_asisdlsep_I4_i4 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=4} MEM:PST:w:i32={BASE=$XnSP SZ=4x2x1x32 IMMOFF:u8=32}    ', 'Q=0b0 size=0b10 ', 'ST1_asisdlsep_I4_i4 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=4} MEM:PST:w:i32={BASE=$XnSP SZ=4x4x1x32 IMMOFF:u8=64}    ', 'Q=0b1 size=0b10 ', 'ST1_asisdlsep_I4_i4 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=1D } COUNT=4} MEM:PST:w:i64={BASE=$XnSP SZ=4x1x1x64 IMMOFF:u8=32}    ', 'Q=0b0 size=0b11 ', 'ST1_asisdlsep_I4_i4 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=4} MEM:PST:w:i64={BASE=$XnSP SZ=4x2x1x64 IMMOFF:u8=64}    ', 'Q=0b1 size=0b11 ', 'ST1_asisdlsep_I4_i4 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=4} MEM:PST:w:i8={ BASE=$XnSP SZ=4x8x1x8 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b00 ', 'ST1_asisdlsep_R4_r4 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=4} MEM:PST:w:i8={ BASE=$XnSP SZ=4x16x1x8 REGOFF=$Xmnxzr} ', 'Q=0b1 size=0b00 ', 'ST1_asisdlsep_R4_r4 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=4} MEM:PST:w:i16={BASE=$XnSP SZ=4x4x1x16 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b01 ', 'ST1_asisdlsep_R4_r4 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=4} MEM:PST:w:i16={BASE=$XnSP SZ=4x8x1x16 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b01 ', 'ST1_asisdlsep_R4_r4 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=4} MEM:PST:w:i32={BASE=$XnSP SZ=4x2x1x32 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b10 ', 'ST1_asisdlsep_R4_r4 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=4} MEM:PST:w:i32={BASE=$XnSP SZ=4x4x1x32 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b10 ', 'ST1_asisdlsep_R4_r4 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=1D } COUNT=4} MEM:PST:w:i64={BASE=$XnSP SZ=4x1x1x64 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b11 ', 'ST1_asisdlsep_R4_r4 ', ' '];
T[' ST1 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=4} MEM:PST:w:i64={BASE=$XnSP SZ=4x2x1x64 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b11 ', 'ST1_asisdlsep_R4_r4 ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=2} MEM:PST:w:i8={ BASE=$XnSP SZ=1x8x2x8 IMMOFF:u8=16}    ', 'Q=0b0 size=0b00 ', 'ST2_asisdlsep_I2_i  ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=2} MEM:PST:w:i8={ BASE=$XnSP SZ=1x16x2x8 IMMOFF:u8=32}   ', 'Q=0b1 size=0b00 ', 'ST2_asisdlsep_I2_i  ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=2} MEM:PST:w:i16={BASE=$XnSP SZ=1x4x2x16 IMMOFF:u8=16}    ', 'Q=0b0 size=0b01 ', 'ST2_asisdlsep_I2_i  ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=2} MEM:PST:w:i16={BASE=$XnSP SZ=1x8x2x16 IMMOFF:u8=32}    ', 'Q=0b1 size=0b01 ', 'ST2_asisdlsep_I2_i  ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=2} MEM:PST:w:i32={BASE=$XnSP SZ=1x2x2x32 IMMOFF:u8=16}    ', 'Q=0b0 size=0b10 ', 'ST2_asisdlsep_I2_i  ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=2} MEM:PST:w:i32={BASE=$XnSP SZ=1x4x2x32 IMMOFF:u8=32}    ', 'Q=0b1 size=0b10 ', 'ST2_asisdlsep_I2_i  ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=2} MEM:PST:w:i64={BASE=$XnSP SZ=1x2x2x64 IMMOFF:u8=32}    ', 'Q=0b1 size=0b11 ', 'ST2_asisdlsep_I2_i  ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=2} MEM:PST:w:i8={ BASE=$XnSP SZ=1x8x2x8 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b00 ', 'ST2_asisdlsep_R2_r  ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=2} MEM:PST:w:i8={ BASE=$XnSP SZ=1x16x2x8 REGOFF=$Xmnxzr} ', 'Q=0b1 size=0b00 ', 'ST2_asisdlsep_R2_r  ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=2} MEM:PST:w:i16={BASE=$XnSP SZ=1x4x2x16 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b01 ', 'ST2_asisdlsep_R2_r  ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=2} MEM:PST:w:i16={BASE=$XnSP SZ=1x8x2x16 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b01 ', 'ST2_asisdlsep_R2_r  ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=2} MEM:PST:w:i32={BASE=$XnSP SZ=1x2x2x32 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b10 ', 'ST2_asisdlsep_R2_r  ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=2} MEM:PST:w:i32={BASE=$XnSP SZ=1x4x2x32 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b10 ', 'ST2_asisdlsep_R2_r  ', ' '];
T[' ST2 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=2} MEM:PST:w:i64={BASE=$XnSP SZ=1x2x2x64 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b11 ', 'ST2_asisdlsep_R2_r  ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=3} MEM:PST:w:i8={ BASE=$XnSP SZ=1x8x3x8 IMMOFF:u8=24}    ', 'Q=0b0 size=0b00 ', 'ST3_asisdlsep_I3_i  ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=3} MEM:PST:w:i8={ BASE=$XnSP SZ=1x16x3x8 IMMOFF:u8=48}   ', 'Q=0b1 size=0b00 ', 'ST3_asisdlsep_I3_i  ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=3} MEM:PST:w:i16={BASE=$XnSP SZ=1x4x3x16 IMMOFF:u8=24}    ', 'Q=0b0 size=0b01 ', 'ST3_asisdlsep_I3_i  ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=3} MEM:PST:w:i16={BASE=$XnSP SZ=1x8x3x16 IMMOFF:u8=48}    ', 'Q=0b1 size=0b01 ', 'ST3_asisdlsep_I3_i  ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=3} MEM:PST:w:i32={BASE=$XnSP SZ=1x2x3x32 IMMOFF:u8=24}    ', 'Q=0b0 size=0b10 ', 'ST3_asisdlsep_I3_i  ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=3} MEM:PST:w:i32={BASE=$XnSP SZ=1x4x3x32 IMMOFF:u8=48}    ', 'Q=0b1 size=0b10 ', 'ST3_asisdlsep_I3_i  ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=3} MEM:PST:w:i64={BASE=$XnSP SZ=1x2x3x64 IMMOFF:u8=48}    ', 'Q=0b1 size=0b11 ', 'ST3_asisdlsep_I3_i  ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=3} MEM:PST:w:i8={ BASE=$XnSP SZ=1x8x3x8 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b00 ', 'ST3_asisdlsep_R3_r  ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=3} MEM:PST:w:i8={ BASE=$XnSP SZ=1x16x3x8 REGOFF=$Xmnxzr} ', 'Q=0b1 size=0b00 ', 'ST3_asisdlsep_R3_r  ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=3} MEM:PST:w:i16={BASE=$XnSP SZ=1x4x3x16 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b01 ', 'ST3_asisdlsep_R3_r  ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=3} MEM:PST:w:i16={BASE=$XnSP SZ=1x8x3x16 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b01 ', 'ST3_asisdlsep_R3_r  ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=3} MEM:PST:w:i32={BASE=$XnSP SZ=1x2x3x32 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b10 ', 'ST3_asisdlsep_R3_r  ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=3} MEM:PST:w:i32={BASE=$XnSP SZ=1x4x3x32 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b10 ', 'ST3_asisdlsep_R3_r  ', ' '];
T[' ST3 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=3} MEM:PST:w:i64={BASE=$XnSP SZ=1x2x3x64 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b11 ', 'ST3_asisdlsep_R3_r  ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=4} MEM:PST:w:i8={ BASE=$XnSP SZ=1x8x4x8 IMMOFF:u8=32}    ', 'Q=0b0 size=0b00 ', 'ST4_asisdlsep_I4_i  ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=4} MEM:PST:w:i8={ BASE=$XnSP SZ=1x16x4x8 IMMOFF:u8=64}   ', 'Q=0b1 size=0b00 ', 'ST4_asisdlsep_I4_i  ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=4} MEM:PST:w:i16={BASE=$XnSP SZ=1x4x4x16 IMMOFF:u8=32}    ', 'Q=0b0 size=0b01 ', 'ST4_asisdlsep_I4_i  ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=4} MEM:PST:w:i16={BASE=$XnSP SZ=1x8x4x16 IMMOFF:u8=64}    ', 'Q=0b1 size=0b01 ', 'ST4_asisdlsep_I4_i  ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=4} MEM:PST:w:i32={BASE=$XnSP SZ=1x2x4x32 IMMOFF:u8=32}    ', 'Q=0b0 size=0b10 ', 'ST4_asisdlsep_I4_i  ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=4} MEM:PST:w:i32={BASE=$XnSP SZ=1x4x4x32 IMMOFF:u8=64}    ', 'Q=0b1 size=0b10 ', 'ST4_asisdlsep_I4_i  ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=4} MEM:PST:w:i64={BASE=$XnSP SZ=1x2x4x64 IMMOFF:u8=64}    ', 'Q=0b1 size=0b11 ', 'ST4_asisdlsep_I4_i  ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i8=$Vt   VDT=8B } COUNT=4} MEM:PST:w:i8={ BASE=$XnSP SZ=1x8x4x8 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b00 ', 'ST4_asisdlsep_R4_r  ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i8=$Vt   VDT=16B} COUNT=4} MEM:PST:w:i8={ BASE=$XnSP SZ=1x16x4x8 REGOFF=$Xmnxzr} ', 'Q=0b1 size=0b00 ', 'ST4_asisdlsep_R4_r  ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i16=$Vt VDT=4H } COUNT=4} MEM:PST:w:i16={BASE=$XnSP SZ=1x4x4x16 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b01 ', 'ST4_asisdlsep_R4_r  ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i16=$Vt VDT=8H } COUNT=4} MEM:PST:w:i16={BASE=$XnSP SZ=1x8x4x16 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b01 ', 'ST4_asisdlsep_R4_r  ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i32=$Vt VDT=2S } COUNT=4} MEM:PST:w:i32={BASE=$XnSP SZ=1x2x4x32 REGOFF=$Xmnxzr}  ', 'Q=0b0 size=0b10 ', 'ST4_asisdlsep_R4_r  ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i32=$Vt VDT=4S } COUNT=4} MEM:PST:w:i32={BASE=$XnSP SZ=1x4x4x32 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b10 ', 'ST4_asisdlsep_R4_r  ', ' '];
T[' ST4 VLIST={VBASE={REG:r:i64=$Vt VDT=2D } COUNT=4} MEM:PST:w:i64={BASE=$XnSP SZ=1x2x4x64 REGOFF=$Xmnxzr}  ', 'Q=0b1 size=0b11 ', 'ST4_asisdlsep_R4_r  ', ' '];



# asisdlso: Advanced SIMD load/store single structure.
# A64: ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0bx R:u=0bx ig2=0b00000 opcode:u=0bxxx S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'asisdlso', 'A64');

DEF Vt   => VECREG('Vt', 'Rt')           ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP') ;

T[' LD1  ELIST={EBASE={REG:w:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=1} MEM:r:i8={ BASE=$XnSP SZ=1x8}    ', 'NONE            ', 'LD1_asisdlso_B1_1b ', ' '];
T[' LD1  ELIST={EBASE={REG:w:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=1} MEM:r:i16={BASE=$XnSP SZ=1x16} ', 'NONE            ', 'LD1_asisdlso_H1_1h ', ' '];
T[' LD1  ELIST={EBASE={REG:w:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=1} MEM:r:i32={BASE=$XnSP SZ=1x32}         ', 'NONE            ', 'LD1_asisdlso_S1_1s ', ' '];
T[' LD1  ELIST={EBASE={REG:w:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=1} MEM:r:i64={BASE=$XnSP SZ=1x64}           ', 'NONE            ', 'LD1_asisdlso_D1_1d ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=1} MEM:r:i8={ BASE=$XnSP SZ=1x8}                           ', 'Q=0b0 size=0b00 ', 'LD1R_asisdlso_R1   ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=1} MEM:r:i8={ BASE=$XnSP SZ=1x8}                           ', 'Q=0b1 size=0b00 ', 'LD1R_asisdlso_R1   ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=1} MEM:r:i16={BASE=$XnSP SZ=1x16}                           ', 'Q=0b0 size=0b01 ', 'LD1R_asisdlso_R1   ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=1} MEM:r:i16={BASE=$XnSP SZ=1x16}                           ', 'Q=0b1 size=0b01 ', 'LD1R_asisdlso_R1   ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=1} MEM:r:i32={BASE=$XnSP SZ=1x32}                           ', 'Q=0b0 size=0b10 ', 'LD1R_asisdlso_R1   ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=1} MEM:r:i32={BASE=$XnSP SZ=1x32}                           ', 'Q=0b1 size=0b10 ', 'LD1R_asisdlso_R1   ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=1} MEM:r:i64={BASE=$XnSP SZ=1x64}                           ', 'Q=0b0 size=0b11 ', 'LD1R_asisdlso_R1   ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=1} MEM:r:i64={BASE=$XnSP SZ=1x64}                           ', 'Q=0b1 size=0b11 ', 'LD1R_asisdlso_R1   ', ' '];
T[' LD2  ELIST={EBASE={REG:w:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=2} MEM:r:i8={ BASE=$XnSP SZ=2x8}    ', 'NONE            ', 'LD2_asisdlso_B2_2b ', ' '];
T[' LD2  ELIST={EBASE={REG:w:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=2} MEM:r:i16={BASE=$XnSP SZ=2x16} ', 'NONE            ', 'LD2_asisdlso_H2_2h ', ' '];
T[' LD2  ELIST={EBASE={REG:w:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=2} MEM:r:i32={BASE=$XnSP SZ=2x32}         ', 'NONE            ', 'LD2_asisdlso_S2_2s ', ' '];
T[' LD2  ELIST={EBASE={REG:w:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=2} MEM:r:i64={BASE=$XnSP SZ=2x64}           ', 'NONE            ', 'LD2_asisdlso_D2_2d ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=2} MEM:r:i8={ BASE=$XnSP SZ=2x8}                           ', 'Q=0b0 size=0b00 ', 'LD2R_asisdlso_R2   ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=2} MEM:r:i8={ BASE=$XnSP SZ=2x8}                           ', 'Q=0b1 size=0b00 ', 'LD2R_asisdlso_R2   ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=2} MEM:r:i16={BASE=$XnSP SZ=2x16}                           ', 'Q=0b0 size=0b01 ', 'LD2R_asisdlso_R2   ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=2} MEM:r:i16={BASE=$XnSP SZ=2x16}                           ', 'Q=0b1 size=0b01 ', 'LD2R_asisdlso_R2   ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=2} MEM:r:i32={BASE=$XnSP SZ=2x32}                           ', 'Q=0b0 size=0b10 ', 'LD2R_asisdlso_R2   ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=2} MEM:r:i32={BASE=$XnSP SZ=2x32}                           ', 'Q=0b1 size=0b10 ', 'LD2R_asisdlso_R2   ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=2} MEM:r:i64={BASE=$XnSP SZ=2x64}                           ', 'Q=0b0 size=0b11 ', 'LD2R_asisdlso_R2   ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=2} MEM:r:i64={BASE=$XnSP SZ=2x64}                           ', 'Q=0b1 size=0b11 ', 'LD2R_asisdlso_R2   ', ' '];
T[' LD3  ELIST={EBASE={REG:w:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=3} MEM:r:i8={ BASE=$XnSP SZ=3x8}    ', 'NONE            ', 'LD3_asisdlso_B3_3b ', ' '];
T[' LD3  ELIST={EBASE={REG:w:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=3} MEM:r:i16={BASE=$XnSP SZ=3x16} ', 'NONE            ', 'LD3_asisdlso_H3_3h ', ' '];
T[' LD3  ELIST={EBASE={REG:w:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=3} MEM:r:i32={BASE=$XnSP SZ=3x32}         ', 'NONE            ', 'LD3_asisdlso_S3_3s ', ' '];
T[' LD3  ELIST={EBASE={REG:w:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=3} MEM:r:i64={BASE=$XnSP SZ=3x64}           ', 'NONE            ', 'LD3_asisdlso_D3_3d ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=3} MEM:r:i8={ BASE=$XnSP SZ=3x8}                           ', 'Q=0b0 size=0b00 ', 'LD3R_asisdlso_R3   ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=3} MEM:r:i8={ BASE=$XnSP SZ=3x8}                           ', 'Q=0b1 size=0b00 ', 'LD3R_asisdlso_R3   ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=3} MEM:r:i16={BASE=$XnSP SZ=3x16}                           ', 'Q=0b0 size=0b01 ', 'LD3R_asisdlso_R3   ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=3} MEM:r:i16={BASE=$XnSP SZ=3x16}                           ', 'Q=0b1 size=0b01 ', 'LD3R_asisdlso_R3   ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=3} MEM:r:i32={BASE=$XnSP SZ=3x32}                           ', 'Q=0b0 size=0b10 ', 'LD3R_asisdlso_R3   ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=3} MEM:r:i32={BASE=$XnSP SZ=3x32}                           ', 'Q=0b1 size=0b10 ', 'LD3R_asisdlso_R3   ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=3} MEM:r:i64={BASE=$XnSP SZ=3x64}                           ', 'Q=0b0 size=0b11 ', 'LD3R_asisdlso_R3   ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=3} MEM:r:i64={BASE=$XnSP SZ=3x64}                           ', 'Q=0b1 size=0b11 ', 'LD3R_asisdlso_R3   ', ' '];
T[' LD4  ELIST={EBASE={REG:w:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=4} MEM:r:i8={ BASE=$XnSP SZ=4x8}    ', 'NONE            ', 'LD4_asisdlso_B4_4b ', ' '];
T[' LD4  ELIST={EBASE={REG:w:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=4} MEM:r:i16={BASE=$XnSP SZ=4x16} ', 'NONE            ', 'LD4_asisdlso_H4_4h ', ' '];
T[' LD4  ELIST={EBASE={REG:w:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=4} MEM:r:i32={BASE=$XnSP SZ=4x32}         ', 'NONE            ', 'LD4_asisdlso_S4_4s ', ' '];
T[' LD4  ELIST={EBASE={REG:w:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=4} MEM:r:i64={BASE=$XnSP SZ=4x64}           ', 'NONE            ', 'LD4_asisdlso_D4_4d ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=4} MEM:r:i8={ BASE=$XnSP SZ=4x8}                           ', 'Q=0b0 size=0b00 ', 'LD4R_asisdlso_R4   ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=4} MEM:r:i8={ BASE=$XnSP SZ=4x8}                           ', 'Q=0b1 size=0b00 ', 'LD4R_asisdlso_R4   ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=4} MEM:r:i16={BASE=$XnSP SZ=4x16}                           ', 'Q=0b0 size=0b01 ', 'LD4R_asisdlso_R4   ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=4} MEM:r:i16={BASE=$XnSP SZ=4x16}                           ', 'Q=0b1 size=0b01 ', 'LD4R_asisdlso_R4   ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=4} MEM:r:i32={BASE=$XnSP SZ=4x32}                           ', 'Q=0b0 size=0b10 ', 'LD4R_asisdlso_R4   ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=4} MEM:r:i32={BASE=$XnSP SZ=4x32}                           ', 'Q=0b1 size=0b10 ', 'LD4R_asisdlso_R4   ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=4} MEM:r:i64={BASE=$XnSP SZ=4x64}                           ', 'Q=0b0 size=0b11 ', 'LD4R_asisdlso_R4   ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=4} MEM:r:i64={BASE=$XnSP SZ=4x64}                           ', 'Q=0b1 size=0b11 ', 'LD4R_asisdlso_R4   ', ' '];
T[' ST1  ELIST={EBASE={REG:r:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=1} MEM:w:i8={ BASE=$XnSP SZ=1x8}    ', 'NONE            ', 'ST1_asisdlso_B1_1b ', ' '];
T[' ST1  ELIST={EBASE={REG:r:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=1} MEM:w:i16={BASE=$XnSP SZ=1x16} ', 'NONE            ', 'ST1_asisdlso_H1_1h ', ' '];
T[' ST1  ELIST={EBASE={REG:r:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=1} MEM:w:i32={BASE=$XnSP SZ=1x32}         ', 'NONE            ', 'ST1_asisdlso_S1_1s ', ' '];
T[' ST1  ELIST={EBASE={REG:r:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=1} MEM:w:i64={BASE=$XnSP SZ=1x64}           ', 'NONE            ', 'ST1_asisdlso_D1_1d ', ' '];
T[' ST2  ELIST={EBASE={REG:r:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=2} MEM:w:i8={ BASE=$XnSP SZ=2x8}    ', 'NONE            ', 'ST2_asisdlso_B2_2b ', ' '];
T[' ST2  ELIST={EBASE={REG:r:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=2} MEM:w:i16={BASE=$XnSP SZ=2x16} ', 'NONE            ', 'ST2_asisdlso_H2_2h ', ' '];
T[' ST2  ELIST={EBASE={REG:r:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=2} MEM:w:i32={BASE=$XnSP SZ=2x32}         ', 'NONE            ', 'ST2_asisdlso_S2_2s ', ' '];
T[' ST2  ELIST={EBASE={REG:r:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=2} MEM:w:i64={BASE=$XnSP SZ=2x64}           ', 'NONE            ', 'ST2_asisdlso_D2_2d ', ' '];
T[' ST3  ELIST={EBASE={REG:r:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=3} MEM:w:i8={ BASE=$XnSP SZ=3x8}    ', 'NONE            ', 'ST3_asisdlso_B3_3b ', ' '];
T[' ST3  ELIST={EBASE={REG:r:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=3} MEM:w:i16={BASE=$XnSP SZ=3x16} ', 'NONE            ', 'ST3_asisdlso_H3_3h ', ' '];
T[' ST3  ELIST={EBASE={REG:r:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=3} MEM:w:i32={BASE=$XnSP SZ=3x32}         ', 'NONE            ', 'ST3_asisdlso_S3_3s ', ' '];
T[' ST3  ELIST={EBASE={REG:r:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=3} MEM:w:i64={BASE=$XnSP SZ=3x64}           ', 'NONE            ', 'ST3_asisdlso_D3_3d ', ' '];
T[' ST4  ELIST={EBASE={REG:r:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=4} MEM:w:i8={ BASE=$XnSP SZ=4x8}    ', 'NONE            ', 'ST4_asisdlso_B4_4b ', ' '];
T[' ST4  ELIST={EBASE={REG:r:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=4} MEM:w:i16={BASE=$XnSP SZ=4x16} ', 'NONE            ', 'ST4_asisdlso_H4_4h ', ' '];
T[' ST4  ELIST={EBASE={REG:r:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=4} MEM:w:i32={BASE=$XnSP SZ=4x32}         ', 'NONE            ', 'ST4_asisdlso_S4_4s ', ' '];
T[' ST4  ELIST={EBASE={REG:r:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=4} MEM:w:i64={BASE=$XnSP SZ=4x64}           ', 'NONE            ', 'ST4_asisdlso_D4_4d ', ' '];



# asisdlsop: Advanced SIMD load/store single structure (post-indexed).
# A64: ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0bx R:u=0bx Rm:u=0bxxxxx opcode:u=0bxxx S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'asisdlsop', 'A64');

DEF Vt     => VECREG('Vt', 'Rt')              ;
DEF Xmnxzr => GPR64('Xm', 'Rm', 'read|noxzr') ;
DEF XnSP   => GPR64('Xn', 'Rn', 'read|SP')    ;

T[' LD1  ELIST={EBASE={REG:w:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=1} MEM:PST:r:i8={ BASE=$XnSP SZ=1x8 IMMOFF:u8=1}       ', 'NONE            ', 'LD1_asisdlsop_B1_i1b  ', ' '];
T[' LD1  ELIST={EBASE={REG:w:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=1} MEM:PST:r:i8={ BASE=$XnSP SZ=1x8 REGOFF=$Xmnxzr}    ', 'NONE            ', 'LD1_asisdlsop_BX1_r1b ', ' '];
T[' LD1  ELIST={EBASE={REG:w:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=1} MEM:PST:r:i16={BASE=$XnSP SZ=1x16 IMMOFF:u8=2}    ', 'NONE            ', 'LD1_asisdlsop_H1_i1h  ', ' '];
T[' LD1  ELIST={EBASE={REG:w:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=1} MEM:PST:r:i16={BASE=$XnSP SZ=1x16 REGOFF=$Xmnxzr} ', 'NONE            ', 'LD1_asisdlsop_HX1_r1h ', ' '];
T[' LD1  ELIST={EBASE={REG:w:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=1} MEM:PST:r:i32={BASE=$XnSP SZ=1x32 IMMOFF:u8=4}            ', 'NONE            ', 'LD1_asisdlsop_S1_i1s  ', ' '];
T[' LD1  ELIST={EBASE={REG:w:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=1} MEM:PST:r:i32={BASE=$XnSP SZ=1x32 REGOFF=$Xmnxzr}         ', 'NONE            ', 'LD1_asisdlsop_SX1_r1s ', ' '];
T[' LD1  ELIST={EBASE={REG:w:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=1} MEM:PST:r:i64={BASE=$XnSP SZ=1x64 IMMOFF:u8=8}              ', 'NONE            ', 'LD1_asisdlsop_D1_i1d  ', ' '];
T[' LD1  ELIST={EBASE={REG:w:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=1} MEM:PST:r:i64={BASE=$XnSP SZ=1x64 REGOFF=$Xmnxzr}           ', 'NONE            ', 'LD1_asisdlsop_DX1_r1d ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=1} MEM:PST:r:i8={ BASE=$XnSP SZ=1x8 IMMOFF:u8=1}                              ', 'Q=0b0 size=0b00 ', 'LD1R_asisdlsop_R1_i   ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=1} MEM:PST:r:i8={ BASE=$XnSP SZ=1x8 IMMOFF:u8=1}                              ', 'Q=0b1 size=0b00 ', 'LD1R_asisdlsop_R1_i   ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=1} MEM:PST:r:i16={BASE=$XnSP SZ=1x16 IMMOFF:u8=2}                              ', 'Q=0b0 size=0b01 ', 'LD1R_asisdlsop_R1_i   ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=1} MEM:PST:r:i16={BASE=$XnSP SZ=1x16 IMMOFF:u8=2}                              ', 'Q=0b1 size=0b01 ', 'LD1R_asisdlsop_R1_i   ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=1} MEM:PST:r:i32={BASE=$XnSP SZ=1x32 IMMOFF:u8=4}                              ', 'Q=0b0 size=0b10 ', 'LD1R_asisdlsop_R1_i   ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=1} MEM:PST:r:i32={BASE=$XnSP SZ=1x32 IMMOFF:u8=4}                              ', 'Q=0b1 size=0b10 ', 'LD1R_asisdlsop_R1_i   ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=1} MEM:PST:r:i64={BASE=$XnSP SZ=1x64 IMMOFF:u8=8}                              ', 'Q=0b0 size=0b11 ', 'LD1R_asisdlsop_R1_i   ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=1} MEM:PST:r:i64={BASE=$XnSP SZ=1x64 IMMOFF:u8=8}                              ', 'Q=0b1 size=0b11 ', 'LD1R_asisdlsop_R1_i   ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=1} MEM:PST:r:i8={ BASE=$XnSP SZ=1x8 REGOFF=$Xmnxzr}                           ', 'Q=0b0 size=0b00 ', 'LD1R_asisdlsop_RX1_r  ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=1} MEM:PST:r:i8={ BASE=$XnSP SZ=1x8 REGOFF=$Xmnxzr}                           ', 'Q=0b1 size=0b00 ', 'LD1R_asisdlsop_RX1_r  ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=1} MEM:PST:r:i16={BASE=$XnSP SZ=1x16 REGOFF=$Xmnxzr}                           ', 'Q=0b0 size=0b01 ', 'LD1R_asisdlsop_RX1_r  ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=1} MEM:PST:r:i16={BASE=$XnSP SZ=1x16 REGOFF=$Xmnxzr}                           ', 'Q=0b1 size=0b01 ', 'LD1R_asisdlsop_RX1_r  ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=1} MEM:PST:r:i32={BASE=$XnSP SZ=1x32 REGOFF=$Xmnxzr}                           ', 'Q=0b0 size=0b10 ', 'LD1R_asisdlsop_RX1_r  ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=1} MEM:PST:r:i32={BASE=$XnSP SZ=1x32 REGOFF=$Xmnxzr}                           ', 'Q=0b1 size=0b10 ', 'LD1R_asisdlsop_RX1_r  ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=1} MEM:PST:r:i64={BASE=$XnSP SZ=1x64 REGOFF=$Xmnxzr}                           ', 'Q=0b0 size=0b11 ', 'LD1R_asisdlsop_RX1_r  ', ' '];
T[' LD1R VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=1} MEM:PST:r:i64={BASE=$XnSP SZ=1x64 REGOFF=$Xmnxzr}                           ', 'Q=0b1 size=0b11 ', 'LD1R_asisdlsop_RX1_r  ', ' '];
T[' LD2  ELIST={EBASE={REG:w:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=2} MEM:PST:r:i8={ BASE=$XnSP SZ=2x8 IMMOFF:u8=2}       ', 'NONE            ', 'LD2_asisdlsop_B2_i2b  ', ' '];
T[' LD2  ELIST={EBASE={REG:w:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=2} MEM:PST:r:i8={ BASE=$XnSP SZ=2x8 REGOFF=$Xmnxzr}    ', 'NONE            ', 'LD2_asisdlsop_BX2_r2b ', ' '];
T[' LD2  ELIST={EBASE={REG:w:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=2} MEM:PST:r:i16={BASE=$XnSP SZ=2x16 IMMOFF:u8=4}    ', 'NONE            ', 'LD2_asisdlsop_H2_i2h  ', ' '];
T[' LD2  ELIST={EBASE={REG:w:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=2} MEM:PST:r:i16={BASE=$XnSP SZ=2x16 REGOFF=$Xmnxzr} ', 'NONE            ', 'LD2_asisdlsop_HX2_r2h ', ' '];
T[' LD2  ELIST={EBASE={REG:w:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=2} MEM:PST:r:i32={BASE=$XnSP SZ=2x32 IMMOFF:u8=8}            ', 'NONE            ', 'LD2_asisdlsop_S2_i2s  ', ' '];
T[' LD2  ELIST={EBASE={REG:w:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=2} MEM:PST:r:i32={BASE=$XnSP SZ=2x32 REGOFF=$Xmnxzr}         ', 'NONE            ', 'LD2_asisdlsop_SX2_r2s ', ' '];
T[' LD2  ELIST={EBASE={REG:w:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=2} MEM:PST:r:i64={BASE=$XnSP SZ=2x64 IMMOFF:u8=16}             ', 'NONE            ', 'LD2_asisdlsop_D2_i2d  ', ' '];
T[' LD2  ELIST={EBASE={REG:w:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=2} MEM:PST:r:i64={BASE=$XnSP SZ=2x64 REGOFF=$Xmnxzr}           ', 'NONE            ', 'LD2_asisdlsop_DX2_r2d ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=2} MEM:PST:r:i8={ BASE=$XnSP SZ=2x8 IMMOFF:u8=2}                              ', 'Q=0b0 size=0b00 ', 'LD2R_asisdlsop_R2_i   ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=2} MEM:PST:r:i8={ BASE=$XnSP SZ=2x8 IMMOFF:u8=2}                              ', 'Q=0b1 size=0b00 ', 'LD2R_asisdlsop_R2_i   ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=2} MEM:PST:r:i16={BASE=$XnSP SZ=2x16 IMMOFF:u8=4}                              ', 'Q=0b0 size=0b01 ', 'LD2R_asisdlsop_R2_i   ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=2} MEM:PST:r:i16={BASE=$XnSP SZ=2x16 IMMOFF:u8=4}                              ', 'Q=0b1 size=0b01 ', 'LD2R_asisdlsop_R2_i   ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=2} MEM:PST:r:i32={BASE=$XnSP SZ=2x32 IMMOFF:u8=8}                              ', 'Q=0b0 size=0b10 ', 'LD2R_asisdlsop_R2_i   ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=2} MEM:PST:r:i32={BASE=$XnSP SZ=2x32 IMMOFF:u8=8}                              ', 'Q=0b1 size=0b10 ', 'LD2R_asisdlsop_R2_i   ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=2} MEM:PST:r:i64={BASE=$XnSP SZ=2x64 IMMOFF:u8=16}                             ', 'Q=0b0 size=0b11 ', 'LD2R_asisdlsop_R2_i   ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=2} MEM:PST:r:i64={BASE=$XnSP SZ=2x64 IMMOFF:u8=16}                             ', 'Q=0b1 size=0b11 ', 'LD2R_asisdlsop_R2_i   ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=2} MEM:PST:r:i8={ BASE=$XnSP SZ=2x8 REGOFF=$Xmnxzr}                           ', 'Q=0b0 size=0b00 ', 'LD2R_asisdlsop_RX2_r  ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=2} MEM:PST:r:i8={ BASE=$XnSP SZ=2x8 REGOFF=$Xmnxzr}                           ', 'Q=0b1 size=0b00 ', 'LD2R_asisdlsop_RX2_r  ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=2} MEM:PST:r:i16={BASE=$XnSP SZ=2x16 REGOFF=$Xmnxzr}                           ', 'Q=0b0 size=0b01 ', 'LD2R_asisdlsop_RX2_r  ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=2} MEM:PST:r:i16={BASE=$XnSP SZ=2x16 REGOFF=$Xmnxzr}                           ', 'Q=0b1 size=0b01 ', 'LD2R_asisdlsop_RX2_r  ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=2} MEM:PST:r:i32={BASE=$XnSP SZ=2x32 REGOFF=$Xmnxzr}                           ', 'Q=0b0 size=0b10 ', 'LD2R_asisdlsop_RX2_r  ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=2} MEM:PST:r:i32={BASE=$XnSP SZ=2x32 REGOFF=$Xmnxzr}                           ', 'Q=0b1 size=0b10 ', 'LD2R_asisdlsop_RX2_r  ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=2} MEM:PST:r:i64={BASE=$XnSP SZ=2x64 REGOFF=$Xmnxzr}                           ', 'Q=0b0 size=0b11 ', 'LD2R_asisdlsop_RX2_r  ', ' '];
T[' LD2R VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=2} MEM:PST:r:i64={BASE=$XnSP SZ=2x64 REGOFF=$Xmnxzr}                           ', 'Q=0b1 size=0b11 ', 'LD2R_asisdlsop_RX2_r  ', ' '];
T[' LD3  ELIST={EBASE={REG:w:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=3} MEM:PST:r:i8={ BASE=$XnSP SZ=3x8 IMMOFF:u8=3}       ', 'NONE            ', 'LD3_asisdlsop_B3_i3b  ', ' '];
T[' LD3  ELIST={EBASE={REG:w:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=3} MEM:PST:r:i8={ BASE=$XnSP SZ=3x8 REGOFF=$Xmnxzr}    ', 'NONE            ', 'LD3_asisdlsop_BX3_r3b ', ' '];
T[' LD3  ELIST={EBASE={REG:w:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=3} MEM:PST:r:i16={BASE=$XnSP SZ=3x16 IMMOFF:u8=6}    ', 'NONE            ', 'LD3_asisdlsop_H3_i3h  ', ' '];
T[' LD3  ELIST={EBASE={REG:w:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=3} MEM:PST:r:i16={BASE=$XnSP SZ=3x16 REGOFF=$Xmnxzr} ', 'NONE            ', 'LD3_asisdlsop_HX3_r3h ', ' '];
T[' LD3  ELIST={EBASE={REG:w:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=3} MEM:PST:r:i32={BASE=$XnSP SZ=3x32 IMMOFF:u8=12}           ', 'NONE            ', 'LD3_asisdlsop_S3_i3s  ', ' '];
T[' LD3  ELIST={EBASE={REG:w:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=3} MEM:PST:r:i32={BASE=$XnSP SZ=3x32 REGOFF=$Xmnxzr}         ', 'NONE            ', 'LD3_asisdlsop_SX3_r3s ', ' '];
T[' LD3  ELIST={EBASE={REG:w:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=3} MEM:PST:r:i64={BASE=$XnSP SZ=3x64 IMMOFF:u8=24}             ', 'NONE            ', 'LD3_asisdlsop_D3_i3d  ', ' '];
T[' LD3  ELIST={EBASE={REG:w:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=3} MEM:PST:r:i64={BASE=$XnSP SZ=3x64 REGOFF=$Xmnxzr}           ', 'NONE            ', 'LD3_asisdlsop_DX3_r3d ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=3} MEM:PST:r:i8={ BASE=$XnSP SZ=3x8 IMMOFF:u8=3}                              ', 'Q=0b0 size=0b00 ', 'LD3R_asisdlsop_R3_i   ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=3} MEM:PST:r:i8={ BASE=$XnSP SZ=3x8 IMMOFF:u8=3}                              ', 'Q=0b1 size=0b00 ', 'LD3R_asisdlsop_R3_i   ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=3} MEM:PST:r:i16={BASE=$XnSP SZ=3x16 IMMOFF:u8=6}                              ', 'Q=0b0 size=0b01 ', 'LD3R_asisdlsop_R3_i   ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=3} MEM:PST:r:i16={BASE=$XnSP SZ=3x16 IMMOFF:u8=6}                              ', 'Q=0b1 size=0b01 ', 'LD3R_asisdlsop_R3_i   ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=3} MEM:PST:r:i32={BASE=$XnSP SZ=3x32 IMMOFF:u8=12}                             ', 'Q=0b0 size=0b10 ', 'LD3R_asisdlsop_R3_i   ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=3} MEM:PST:r:i32={BASE=$XnSP SZ=3x32 IMMOFF:u8=12}                             ', 'Q=0b1 size=0b10 ', 'LD3R_asisdlsop_R3_i   ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=3} MEM:PST:r:i64={BASE=$XnSP SZ=3x64 IMMOFF:u8=24}                             ', 'Q=0b0 size=0b11 ', 'LD3R_asisdlsop_R3_i   ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=3} MEM:PST:r:i64={BASE=$XnSP SZ=3x64 IMMOFF:u8=24}                             ', 'Q=0b1 size=0b11 ', 'LD3R_asisdlsop_R3_i   ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=3} MEM:PST:r:i8={ BASE=$XnSP SZ=3x8 REGOFF=$Xmnxzr}                           ', 'Q=0b0 size=0b00 ', 'LD3R_asisdlsop_RX3_r  ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=3} MEM:PST:r:i8={ BASE=$XnSP SZ=3x8 REGOFF=$Xmnxzr}                           ', 'Q=0b1 size=0b00 ', 'LD3R_asisdlsop_RX3_r  ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=3} MEM:PST:r:i16={BASE=$XnSP SZ=3x16 REGOFF=$Xmnxzr}                           ', 'Q=0b0 size=0b01 ', 'LD3R_asisdlsop_RX3_r  ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=3} MEM:PST:r:i16={BASE=$XnSP SZ=3x16 REGOFF=$Xmnxzr}                           ', 'Q=0b1 size=0b01 ', 'LD3R_asisdlsop_RX3_r  ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=3} MEM:PST:r:i32={BASE=$XnSP SZ=3x32 REGOFF=$Xmnxzr}                           ', 'Q=0b0 size=0b10 ', 'LD3R_asisdlsop_RX3_r  ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=3} MEM:PST:r:i32={BASE=$XnSP SZ=3x32 REGOFF=$Xmnxzr}                           ', 'Q=0b1 size=0b10 ', 'LD3R_asisdlsop_RX3_r  ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=3} MEM:PST:r:i64={BASE=$XnSP SZ=3x64 REGOFF=$Xmnxzr}                           ', 'Q=0b0 size=0b11 ', 'LD3R_asisdlsop_RX3_r  ', ' '];
T[' LD3R VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=3} MEM:PST:r:i64={BASE=$XnSP SZ=3x64 REGOFF=$Xmnxzr}                           ', 'Q=0b1 size=0b11 ', 'LD3R_asisdlsop_RX3_r  ', ' '];
T[' LD4  ELIST={EBASE={REG:w:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=4} MEM:PST:r:i8={ BASE=$XnSP SZ=4x8 IMMOFF:u8=4}       ', 'NONE            ', 'LD4_asisdlsop_B4_i4b  ', ' '];
T[' LD4  ELIST={EBASE={REG:w:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=4} MEM:PST:r:i8={ BASE=$XnSP SZ=4x8 REGOFF=$Xmnxzr}    ', 'NONE            ', 'LD4_asisdlsop_BX4_r4b ', ' '];
T[' LD4  ELIST={EBASE={REG:w:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=4} MEM:PST:r:i16={BASE=$XnSP SZ=4x16 IMMOFF:u8=8}    ', 'NONE            ', 'LD4_asisdlsop_H4_i4h  ', ' '];
T[' LD4  ELIST={EBASE={REG:w:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=4} MEM:PST:r:i16={BASE=$XnSP SZ=4x16 REGOFF=$Xmnxzr} ', 'NONE            ', 'LD4_asisdlsop_HX4_r4h ', ' '];
T[' LD4  ELIST={EBASE={REG:w:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=4} MEM:PST:r:i32={BASE=$XnSP SZ=4x32 IMMOFF:u8=16}           ', 'NONE            ', 'LD4_asisdlsop_S4_i4s  ', ' '];
T[' LD4  ELIST={EBASE={REG:w:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=4} MEM:PST:r:i32={BASE=$XnSP SZ=4x32 REGOFF=$Xmnxzr}         ', 'NONE            ', 'LD4_asisdlsop_SX4_r4s ', ' '];
T[' LD4  ELIST={EBASE={REG:w:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=4} MEM:PST:r:i64={BASE=$XnSP SZ=4x64 IMMOFF:u8=32}             ', 'NONE            ', 'LD4_asisdlsop_D4_i4d  ', ' '];
T[' LD4  ELIST={EBASE={REG:w:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=4} MEM:PST:r:i64={BASE=$XnSP SZ=4x64 REGOFF=$Xmnxzr}           ', 'NONE            ', 'LD4_asisdlsop_DX4_r4d ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=4} MEM:PST:r:i8={ BASE=$XnSP SZ=4x8 IMMOFF:u8=4}                              ', 'Q=0b0 size=0b00 ', 'LD4R_asisdlsop_R4_i   ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=4} MEM:PST:r:i8={ BASE=$XnSP SZ=4x8 IMMOFF:u8=4}                              ', 'Q=0b1 size=0b00 ', 'LD4R_asisdlsop_R4_i   ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=4} MEM:PST:r:i16={BASE=$XnSP SZ=4x16 IMMOFF:u8=8}                              ', 'Q=0b0 size=0b01 ', 'LD4R_asisdlsop_R4_i   ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=4} MEM:PST:r:i16={BASE=$XnSP SZ=4x16 IMMOFF:u8=8}                              ', 'Q=0b1 size=0b01 ', 'LD4R_asisdlsop_R4_i   ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=4} MEM:PST:r:i32={BASE=$XnSP SZ=4x32 IMMOFF:u8=16}                             ', 'Q=0b0 size=0b10 ', 'LD4R_asisdlsop_R4_i   ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=4} MEM:PST:r:i32={BASE=$XnSP SZ=4x32 IMMOFF:u8=16}                             ', 'Q=0b1 size=0b10 ', 'LD4R_asisdlsop_R4_i   ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=4} MEM:PST:r:i64={BASE=$XnSP SZ=4x64 IMMOFF:u8=32}                             ', 'Q=0b0 size=0b11 ', 'LD4R_asisdlsop_R4_i   ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=4} MEM:PST:r:i64={BASE=$XnSP SZ=4x64 IMMOFF:u8=32}                             ', 'Q=0b1 size=0b11 ', 'LD4R_asisdlsop_R4_i   ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i8=$Vt   VDT=8B } COUNT=4} MEM:PST:r:i8={ BASE=$XnSP SZ=4x8 REGOFF=$Xmnxzr}                           ', 'Q=0b0 size=0b00 ', 'LD4R_asisdlsop_RX4_r  ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i8=$Vt   VDT=16B} COUNT=4} MEM:PST:r:i8={ BASE=$XnSP SZ=4x8 REGOFF=$Xmnxzr}                           ', 'Q=0b1 size=0b00 ', 'LD4R_asisdlsop_RX4_r  ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i16=$Vt VDT=4H } COUNT=4} MEM:PST:r:i16={BASE=$XnSP SZ=4x16 REGOFF=$Xmnxzr}                           ', 'Q=0b0 size=0b01 ', 'LD4R_asisdlsop_RX4_r  ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i16=$Vt VDT=8H } COUNT=4} MEM:PST:r:i16={BASE=$XnSP SZ=4x16 REGOFF=$Xmnxzr}                           ', 'Q=0b1 size=0b01 ', 'LD4R_asisdlsop_RX4_r  ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i32=$Vt VDT=2S } COUNT=4} MEM:PST:r:i32={BASE=$XnSP SZ=4x32 REGOFF=$Xmnxzr}                           ', 'Q=0b0 size=0b10 ', 'LD4R_asisdlsop_RX4_r  ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i32=$Vt VDT=4S } COUNT=4} MEM:PST:r:i32={BASE=$XnSP SZ=4x32 REGOFF=$Xmnxzr}                           ', 'Q=0b1 size=0b10 ', 'LD4R_asisdlsop_RX4_r  ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i64=$Vt VDT=1D } COUNT=4} MEM:PST:r:i64={BASE=$XnSP SZ=4x64 REGOFF=$Xmnxzr}                           ', 'Q=0b0 size=0b11 ', 'LD4R_asisdlsop_RX4_r  ', ' '];
T[' LD4R VLIST={VBASE={REG:w:i64=$Vt VDT=2D } COUNT=4} MEM:PST:r:i64={BASE=$XnSP SZ=4x64 REGOFF=$Xmnxzr}                           ', 'Q=0b1 size=0b11 ', 'LD4R_asisdlsop_RX4_r  ', ' '];
T[' ST1  ELIST={EBASE={REG:r:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=1} MEM:PST:w:i8={ BASE=$XnSP SZ=1x8 IMMOFF:u8=1}       ', 'NONE            ', 'ST1_asisdlsop_B1_i1b  ', ' '];
T[' ST1  ELIST={EBASE={REG:r:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=1} MEM:PST:w:i8={ BASE=$XnSP SZ=1x8 REGOFF=$Xmnxzr}    ', 'NONE            ', 'ST1_asisdlsop_BX1_r1b ', ' '];
T[' ST1  ELIST={EBASE={REG:r:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=1} MEM:PST:w:i16={BASE=$XnSP SZ=1x16 IMMOFF:u8=2}    ', 'NONE            ', 'ST1_asisdlsop_H1_i1h  ', ' '];
T[' ST1  ELIST={EBASE={REG:r:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=1} MEM:PST:w:i16={BASE=$XnSP SZ=1x16 REGOFF=$Xmnxzr} ', 'NONE            ', 'ST1_asisdlsop_HX1_r1h ', ' '];
T[' ST1  ELIST={EBASE={REG:r:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=1} MEM:PST:w:i32={BASE=$XnSP SZ=1x32 IMMOFF:u8=4}            ', 'NONE            ', 'ST1_asisdlsop_S1_i1s  ', ' '];
T[' ST1  ELIST={EBASE={REG:r:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=1} MEM:PST:w:i32={BASE=$XnSP SZ=1x32 REGOFF=$Xmnxzr}         ', 'NONE            ', 'ST1_asisdlsop_SX1_r1s ', ' '];
T[' ST1  ELIST={EBASE={REG:r:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=1} MEM:PST:w:i64={BASE=$XnSP SZ=1x64 IMMOFF:u8=8}              ', 'NONE            ', 'ST1_asisdlsop_D1_i1d  ', ' '];
T[' ST1  ELIST={EBASE={REG:r:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=1} MEM:PST:w:i64={BASE=$XnSP SZ=1x64 REGOFF=$Xmnxzr}           ', 'NONE            ', 'ST1_asisdlsop_DX1_r1d ', ' '];
T[' ST2  ELIST={EBASE={REG:r:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=2} MEM:PST:w:i8={ BASE=$XnSP SZ=2x8 IMMOFF:u8=2}       ', 'NONE            ', 'ST2_asisdlsop_B2_i2b  ', ' '];
T[' ST2  ELIST={EBASE={REG:r:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=2} MEM:PST:w:i8={ BASE=$XnSP SZ=2x8 REGOFF=$Xmnxzr}    ', 'NONE            ', 'ST2_asisdlsop_BX2_r2b ', ' '];
T[' ST2  ELIST={EBASE={REG:r:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=2} MEM:PST:w:i16={BASE=$XnSP SZ=2x16 IMMOFF:u8=4}    ', 'NONE            ', 'ST2_asisdlsop_H2_i2h  ', ' '];
T[' ST2  ELIST={EBASE={REG:r:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=2} MEM:PST:w:i16={BASE=$XnSP SZ=2x16 REGOFF=$Xmnxzr} ', 'NONE            ', 'ST2_asisdlsop_HX2_r2h ', ' '];
T[' ST2  ELIST={EBASE={REG:r:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=2} MEM:PST:w:i32={BASE=$XnSP SZ=2x32 IMMOFF:u8=8}            ', 'NONE            ', 'ST2_asisdlsop_S2_i2s  ', ' '];
T[' ST2  ELIST={EBASE={REG:r:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=2} MEM:PST:w:i32={BASE=$XnSP SZ=2x32 REGOFF=$Xmnxzr}         ', 'NONE            ', 'ST2_asisdlsop_SX2_r2s ', ' '];
T[' ST2  ELIST={EBASE={REG:r:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=2} MEM:PST:w:i64={BASE=$XnSP SZ=2x64 IMMOFF:u8=16}             ', 'NONE            ', 'ST2_asisdlsop_D2_i2d  ', ' '];
T[' ST2  ELIST={EBASE={REG:r:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=2} MEM:PST:w:i64={BASE=$XnSP SZ=2x64 REGOFF=$Xmnxzr}           ', 'NONE            ', 'ST2_asisdlsop_DX2_r2d ', ' '];
T[' ST3  ELIST={EBASE={REG:r:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=3} MEM:PST:w:i8={ BASE=$XnSP SZ=3x8 IMMOFF:u8=3}       ', 'NONE            ', 'ST3_asisdlsop_B3_i3b  ', ' '];
T[' ST3  ELIST={EBASE={REG:r:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=3} MEM:PST:w:i8={ BASE=$XnSP SZ=3x8 REGOFF=$Xmnxzr}    ', 'NONE            ', 'ST3_asisdlsop_BX3_r3b ', ' '];
T[' ST3  ELIST={EBASE={REG:r:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=3} MEM:PST:w:i16={BASE=$XnSP SZ=3x16 IMMOFF:u8=6}    ', 'NONE            ', 'ST3_asisdlsop_H3_i3h  ', ' '];
T[' ST3  ELIST={EBASE={REG:r:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=3} MEM:PST:w:i16={BASE=$XnSP SZ=3x16 REGOFF=$Xmnxzr} ', 'NONE            ', 'ST3_asisdlsop_HX3_r3h ', ' '];
T[' ST3  ELIST={EBASE={REG:r:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=3} MEM:PST:w:i32={BASE=$XnSP SZ=3x32 IMMOFF:u8=12}           ', 'NONE            ', 'ST3_asisdlsop_S3_i3s  ', ' '];
T[' ST3  ELIST={EBASE={REG:r:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=3} MEM:PST:w:i32={BASE=$XnSP SZ=3x32 REGOFF=$Xmnxzr}         ', 'NONE            ', 'ST3_asisdlsop_SX3_r3s ', ' '];
T[' ST3  ELIST={EBASE={REG:r:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=3} MEM:PST:w:i64={BASE=$XnSP SZ=3x64 IMMOFF:u8=24}             ', 'NONE            ', 'ST3_asisdlsop_D3_i3d  ', ' '];
T[' ST3  ELIST={EBASE={REG:r:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=3} MEM:PST:w:i64={BASE=$XnSP SZ=3x64 REGOFF=$Xmnxzr}           ', 'NONE            ', 'ST3_asisdlsop_DX3_r3d ', ' '];
T[' ST4  ELIST={EBASE={REG:r:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=4} MEM:PST:w:i8={ BASE=$XnSP SZ=4x8 IMMOFF:u8=4}       ', 'NONE            ', 'ST4_asisdlsop_B4_i4b  ', ' '];
T[' ST4  ELIST={EBASE={REG:r:i8=$Vt   VDS=B IDX={ENCODEDIN=Q:S:size}} COUNT=4} MEM:PST:w:i8={ BASE=$XnSP SZ=4x8 REGOFF=$Xmnxzr}    ', 'NONE            ', 'ST4_asisdlsop_BX4_r4b ', ' '];
T[' ST4  ELIST={EBASE={REG:r:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=4} MEM:PST:w:i16={BASE=$XnSP SZ=4x16 IMMOFF:u8=8}    ', 'NONE            ', 'ST4_asisdlsop_H4_i4h  ', ' '];
T[' ST4  ELIST={EBASE={REG:r:i16=$Vt VDS=H IDX={ENCODEDIN=Q:S:size<1>}} COUNT=4} MEM:PST:w:i16={BASE=$XnSP SZ=4x16 REGOFF=$Xmnxzr} ', 'NONE            ', 'ST4_asisdlsop_HX4_r4h ', ' '];
T[' ST4  ELIST={EBASE={REG:r:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=4} MEM:PST:w:i32={BASE=$XnSP SZ=4x32 IMMOFF:u8=16}           ', 'NONE            ', 'ST4_asisdlsop_S4_i4s  ', ' '];
T[' ST4  ELIST={EBASE={REG:r:i32=$Vt VDS=S IDX={ENCODEDIN=Q:S}} COUNT=4} MEM:PST:w:i32={BASE=$XnSP SZ=4x32 REGOFF=$Xmnxzr}         ', 'NONE            ', 'ST4_asisdlsop_SX4_r4s ', ' '];
T[' ST4  ELIST={EBASE={REG:r:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=4} MEM:PST:w:i64={BASE=$XnSP SZ=4x64 IMMOFF:u8=32}             ', 'NONE            ', 'ST4_asisdlsop_D4_i4d  ', ' '];
T[' ST4  ELIST={EBASE={REG:r:i64=$Vt VDS=D IDX={ENCODEDIN=Q}} COUNT=4} MEM:PST:w:i64={BASE=$XnSP SZ=4x64 REGOFF=$Xmnxzr}           ', 'NONE            ', 'ST4_asisdlsop_DX4_r4d ', ' '];



# asisdmisc: Advanced SIMD scalar two-register miscellaneous.
# A64: ig0=0b01 U:u=0bx ig1=0b11110 size:u=0bxx ig2=0b10000 opcode:u=0bxxxxx ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asisdmisc', 'A64');

DEF Bd => SIMD8('Bd', 'Rd', 'write')  ;
DEF Bn => SIMD8('Bn', 'Rn', 'read')   ;
DEF Dd => SIMD64('Dd', 'Rd', 'write') ;
DEF Dn => SIMD64('Dn', 'Rn', 'read')  ;
DEF Hd => SIMD16('Hd', 'Rd', 'write') ;
DEF Hn => SIMD16('Hn', 'Rn', 'read')  ;
DEF Sd => SIMD32('Sd', 'Rd', 'write') ;
DEF Sn => SIMD32('Sn', 'Rn', 'read')  ;

T[' ABS     REG:i64=$Dd REG:i64=$Dn               ', 'size=0b11 ', 'ABS_asisdmisc_R     ', ' '];
T[' CMEQ    REG:i64=$Dd REG:i64=$Dn IMM:u8=0      ', 'size=0b11 ', 'CMEQ_asisdmisc_Z    ', ' '];
T[' CMGE    REG:i64=$Dd REG:i64=$Dn IMM:u8=0      ', 'size=0b11 ', 'CMGE_asisdmisc_Z    ', ' '];
T[' CMGT    REG:i64=$Dd REG:i64=$Dn IMM:u8=0      ', 'size=0b11 ', 'CMGT_asisdmisc_Z    ', ' '];
T[' CMLE    REG:i64=$Dd REG:i64=$Dn IMM:u8=0      ', 'size=0b11 ', 'CMLE_asisdmisc_Z    ', ' '];
T[' CMLT    REG:i64=$Dd REG:i64=$Dn IMM:u8=0      ', 'size=0b11 ', 'CMLT_asisdmisc_Z    ', ' '];
T[' FCMEQ   REG:f32=$Sd REG:f32=$Sn FPIMM:f32=0.0 ', 'sz=0b0    ', 'FCMEQ_asisdmisc_FZ  ', ' '];
T[' FCMEQ   REG:f64=$Dd REG:f64=$Dn FPIMM:f32=0.0 ', 'sz=0b1    ', 'FCMEQ_asisdmisc_FZ  ', ' '];
T[' FCMGE   REG:f32=$Sd REG:f32=$Sn FPIMM:f32=0.0 ', 'sz=0b0    ', 'FCMGE_asisdmisc_FZ  ', ' '];
T[' FCMGE   REG:f64=$Dd REG:f64=$Dn FPIMM:f32=0.0 ', 'sz=0b1    ', 'FCMGE_asisdmisc_FZ  ', ' '];
T[' FCMGT   REG:f32=$Sd REG:f32=$Sn FPIMM:f32=0.0 ', 'sz=0b0    ', 'FCMGT_asisdmisc_FZ  ', ' '];
T[' FCMGT   REG:f64=$Dd REG:f64=$Dn FPIMM:f32=0.0 ', 'sz=0b1    ', 'FCMGT_asisdmisc_FZ  ', ' '];
T[' FCMLE   REG:f32=$Sd REG:f32=$Sn FPIMM:f32=0.0 ', 'sz=0b0    ', 'FCMLE_asisdmisc_FZ  ', ' '];
T[' FCMLE   REG:f64=$Dd REG:f64=$Dn FPIMM:f32=0.0 ', 'sz=0b1    ', 'FCMLE_asisdmisc_FZ  ', ' '];
T[' FCMLT   REG:f32=$Sd REG:f32=$Sn FPIMM:f32=0.0 ', 'sz=0b0    ', 'FCMLT_asisdmisc_FZ  ', ' '];
T[' FCMLT   REG:f64=$Dd REG:f64=$Dn FPIMM:f32=0.0 ', 'sz=0b1    ', 'FCMLT_asisdmisc_FZ  ', ' '];
T[' FCVTAS  REG:f32=$Sd REG:f32=$Sn               ', 'sz=0b0    ', 'FCVTAS_asisdmisc_R  ', ' '];
T[' FCVTAS  REG:f64=$Dd REG:f64=$Dn               ', 'sz=0b1    ', 'FCVTAS_asisdmisc_R  ', ' '];
T[' FCVTAU  REG:f32=$Sd REG:f32=$Sn               ', 'sz=0b0    ', 'FCVTAU_asisdmisc_R  ', ' '];
T[' FCVTAU  REG:f64=$Dd REG:f64=$Dn               ', 'sz=0b1    ', 'FCVTAU_asisdmisc_R  ', ' '];
T[' FCVTMS  REG:f32=$Sd REG:f32=$Sn               ', 'sz=0b0    ', 'FCVTMS_asisdmisc_R  ', ' '];
T[' FCVTMS  REG:f64=$Dd REG:f64=$Dn               ', 'sz=0b1    ', 'FCVTMS_asisdmisc_R  ', ' '];
T[' FCVTMU  REG:f32=$Sd REG:f32=$Sn               ', 'sz=0b0    ', 'FCVTMU_asisdmisc_R  ', ' '];
T[' FCVTMU  REG:f64=$Dd REG:f64=$Dn               ', 'sz=0b1    ', 'FCVTMU_asisdmisc_R  ', ' '];
T[' FCVTNS  REG:f32=$Sd REG:f32=$Sn               ', 'sz=0b0    ', 'FCVTNS_asisdmisc_R  ', ' '];
T[' FCVTNS  REG:f64=$Dd REG:f64=$Dn               ', 'sz=0b1    ', 'FCVTNS_asisdmisc_R  ', ' '];
T[' FCVTNU  REG:f32=$Sd REG:f32=$Sn               ', 'sz=0b0    ', 'FCVTNU_asisdmisc_R  ', ' '];
T[' FCVTNU  REG:f64=$Dd REG:f64=$Dn               ', 'sz=0b1    ', 'FCVTNU_asisdmisc_R  ', ' '];
T[' FCVTPS  REG:f32=$Sd REG:f32=$Sn               ', 'sz=0b0    ', 'FCVTPS_asisdmisc_R  ', ' '];
T[' FCVTPS  REG:f64=$Dd REG:f64=$Dn               ', 'sz=0b1    ', 'FCVTPS_asisdmisc_R  ', ' '];
T[' FCVTPU  REG:f32=$Sd REG:f32=$Sn               ', 'sz=0b0    ', 'FCVTPU_asisdmisc_R  ', ' '];
T[' FCVTPU  REG:f64=$Dd REG:f64=$Dn               ', 'sz=0b1    ', 'FCVTPU_asisdmisc_R  ', ' '];
T[' FCVTXN  REG:f32=$Sd REG:f64=$Dn               ', 'sz=0b1    ', 'FCVTXN_asisdmisc_N  ', ' '];
T[' FCVTZS  REG:f32=$Sd REG:f32=$Sn               ', 'sz=0b0    ', 'FCVTZS_asisdmisc_R  ', ' '];
T[' FCVTZS  REG:f64=$Dd REG:f64=$Dn               ', 'sz=0b1    ', 'FCVTZS_asisdmisc_R  ', ' '];
T[' FCVTZU  REG:f32=$Sd REG:f32=$Sn               ', 'sz=0b0    ', 'FCVTZU_asisdmisc_R  ', ' '];
T[' FCVTZU  REG:f64=$Dd REG:f64=$Dn               ', 'sz=0b1    ', 'FCVTZU_asisdmisc_R  ', ' '];
T[' FRECPE  REG:f32=$Sd REG:f32=$Sn               ', 'sz=0b0    ', 'FRECPE_asisdmisc_R  ', ' '];
T[' FRECPE  REG:f64=$Dd REG:f64=$Dn               ', 'sz=0b1    ', 'FRECPE_asisdmisc_R  ', ' '];
T[' FRECPX  REG:f32=$Sd REG:f32=$Sn               ', 'sz=0b0    ', 'FRECPX_asisdmisc_R  ', ' '];
T[' FRECPX  REG:f64=$Dd REG:f64=$Dn               ', 'sz=0b1    ', 'FRECPX_asisdmisc_R  ', ' '];
T[' FRSQRTE REG:f32=$Sd REG:f32=$Sn               ', 'sz=0b0    ', 'FRSQRTE_asisdmisc_R ', ' '];
T[' FRSQRTE REG:f64=$Dd REG:f64=$Dn               ', 'sz=0b1    ', 'FRSQRTE_asisdmisc_R ', ' '];
T[' NEG     REG:i64=$Dd REG:i64=$Dn               ', 'size=0b11 ', 'NEG_asisdmisc_R     ', ' '];
T[' SCVTF   REG:s32=$Sd REG:s32=$Sn               ', 'sz=0b0    ', 'SCVTF_asisdmisc_R   ', ' '];
T[' SCVTF   REG:s64=$Dd REG:s64=$Dn               ', 'sz=0b1    ', 'SCVTF_asisdmisc_R   ', ' '];
T[' SQABS   REG:s8=$Bd  REG:s8=$Bn                ', 'size=0b00 ', 'SQABS_asisdmisc_R   ', ' '];
T[' SQABS   REG:s16=$Hd REG:s16=$Hn               ', 'size=0b01 ', 'SQABS_asisdmisc_R   ', ' '];
T[' SQABS   REG:s32=$Sd REG:s32=$Sn               ', 'size=0b10 ', 'SQABS_asisdmisc_R   ', ' '];
T[' SQABS   REG:s64=$Dd REG:s64=$Dn               ', 'size=0b11 ', 'SQABS_asisdmisc_R   ', ' '];
T[' SQNEG   REG:s8=$Bd  REG:s8=$Bn                ', 'size=0b00 ', 'SQNEG_asisdmisc_R   ', ' '];
T[' SQNEG   REG:s16=$Hd REG:s16=$Hn               ', 'size=0b01 ', 'SQNEG_asisdmisc_R   ', ' '];
T[' SQNEG   REG:s32=$Sd REG:s32=$Sn               ', 'size=0b10 ', 'SQNEG_asisdmisc_R   ', ' '];
T[' SQNEG   REG:s64=$Dd REG:s64=$Dn               ', 'size=0b11 ', 'SQNEG_asisdmisc_R   ', ' '];
T[' SQXTN   REG:s8=$Bd  REG:s16=$Hn               ', 'size=0b00 ', 'SQXTN_asisdmisc_N   ', ' '];
T[' SQXTN   REG:s16=$Hd REG:s32=$Sn               ', 'size=0b01 ', 'SQXTN_asisdmisc_N   ', ' '];
T[' SQXTN   REG:s32=$Sd REG:s64=$Dn               ', 'size=0b10 ', 'SQXTN_asisdmisc_N   ', ' '];
T[' SQXTUN  REG:u8=$Bd  REG:u16=$Hn               ', 'size=0b00 ', 'SQXTUN_asisdmisc_N  ', ' '];
T[' SQXTUN  REG:u16=$Hd REG:u32=$Sn               ', 'size=0b01 ', 'SQXTUN_asisdmisc_N  ', ' '];
T[' SQXTUN  REG:u32=$Sd REG:u64=$Dn               ', 'size=0b10 ', 'SQXTUN_asisdmisc_N  ', ' '];
T[' SUQADD  REG:u8=$Bd  REG:u8=$Bn                ', 'size=0b00 ', 'SUQADD_asisdmisc_R  ', ' '];
T[' SUQADD  REG:u16=$Hd REG:u16=$Hn               ', 'size=0b01 ', 'SUQADD_asisdmisc_R  ', ' '];
T[' SUQADD  REG:u32=$Sd REG:u32=$Sn               ', 'size=0b10 ', 'SUQADD_asisdmisc_R  ', ' '];
T[' SUQADD  REG:u64=$Dd REG:u64=$Dn               ', 'size=0b11 ', 'SUQADD_asisdmisc_R  ', ' '];
T[' UCVTF   REG:u32=$Sd REG:u32=$Sn               ', 'sz=0b0    ', 'UCVTF_asisdmisc_R   ', ' '];
T[' UCVTF   REG:u64=$Dd REG:u64=$Dn               ', 'sz=0b1    ', 'UCVTF_asisdmisc_R   ', ' '];
T[' UQXTN   REG:u8=$Bd  REG:u16=$Hn               ', 'size=0b00 ', 'UQXTN_asisdmisc_N   ', ' '];
T[' UQXTN   REG:u16=$Hd REG:u32=$Sn               ', 'size=0b01 ', 'UQXTN_asisdmisc_N   ', ' '];
T[' UQXTN   REG:u32=$Sd REG:u64=$Dn               ', 'size=0b10 ', 'UQXTN_asisdmisc_N   ', ' '];
T[' USQADD  REG:u8=$Bd  REG:u8=$Bn                ', 'size=0b00 ', 'USQADD_asisdmisc_R  ', ' '];
T[' USQADD  REG:u16=$Hd REG:u16=$Hn               ', 'size=0b01 ', 'USQADD_asisdmisc_R  ', ' '];
T[' USQADD  REG:u32=$Sd REG:u32=$Sn               ', 'size=0b10 ', 'USQADD_asisdmisc_R  ', ' '];
T[' USQADD  REG:u64=$Dd REG:u64=$Dn               ', 'size=0b11 ', 'USQADD_asisdmisc_R  ', ' '];



# asisdmiscfp16: Advanced SIMD scalar two-register miscellaneous FP16.
# A64: ig0=0b01 U:u=0bx ig1=0b11110 a:u=0bx ig2=0b111100 opcode:u=0bxxxxx ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asisdmiscfp16', 'A64');

DEF Hd => SIMD16('Hd', 'Rd', 'write') ;
DEF Hn => SIMD16('Hn', 'Rn', 'read')  ;

T[' FCMEQ   REG:f16=$Hd REG:f16=$Hn FPIMM:f32=0.0 ', 'NONE ', 'FCMEQ_asisdmiscfp16_FZ  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMGE   REG:f16=$Hd REG:f16=$Hn FPIMM:f32=0.0 ', 'NONE ', 'FCMGE_asisdmiscfp16_FZ  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMGT   REG:f16=$Hd REG:f16=$Hn FPIMM:f32=0.0 ', 'NONE ', 'FCMGT_asisdmiscfp16_FZ  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMLE   REG:f16=$Hd REG:f16=$Hn FPIMM:f32=0.0 ', 'NONE ', 'FCMLE_asisdmiscfp16_FZ  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMLT   REG:f16=$Hd REG:f16=$Hn FPIMM:f32=0.0 ', 'NONE ', 'FCMLT_asisdmiscfp16_FZ  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTAS  REG:f16=$Hd REG:f16=$Hn               ', 'NONE ', 'FCVTAS_asisdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTAU  REG:f16=$Hd REG:f16=$Hn               ', 'NONE ', 'FCVTAU_asisdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTMS  REG:f16=$Hd REG:f16=$Hn               ', 'NONE ', 'FCVTMS_asisdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTMU  REG:f16=$Hd REG:f16=$Hn               ', 'NONE ', 'FCVTMU_asisdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTNS  REG:f16=$Hd REG:f16=$Hn               ', 'NONE ', 'FCVTNS_asisdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTNU  REG:f16=$Hd REG:f16=$Hn               ', 'NONE ', 'FCVTNU_asisdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTPS  REG:f16=$Hd REG:f16=$Hn               ', 'NONE ', 'FCVTPS_asisdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTPU  REG:f16=$Hd REG:f16=$Hn               ', 'NONE ', 'FCVTPU_asisdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTZS  REG:f16=$Hd REG:f16=$Hn               ', 'NONE ', 'FCVTZS_asisdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTZU  REG:f16=$Hd REG:f16=$Hn               ', 'NONE ', 'FCVTZU_asisdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRECPE  REG:f16=$Hd REG:f16=$Hn               ', 'NONE ', 'FRECPE_asisdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRECPX  REG:f16=$Hd REG:f16=$Hn               ', 'NONE ', 'FRECPX_asisdmiscfp16_R  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRSQRTE REG:f16=$Hd REG:f16=$Hn               ', 'NONE ', 'FRSQRTE_asisdmiscfp16_R ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' SCVTF   REG:s16=$Hd REG:s16=$Hn               ', 'NONE ', 'SCVTF_asisdmiscfp16_R   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' UCVTF   REG:u16=$Hd REG:u16=$Hn               ', 'NONE ', 'UCVTF_asisdmiscfp16_R   ', 'arch_variant=ARMv8v2 extensions=FP16 '];



# asisdone: Advanced SIMD scalar copy.
# A64: ig0=0b01 op:u=0bx ig1=0b11110000 imm5:u=0bxxxxx ig2=0b0 imm4:u=0bxxxx ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asisdone', 'A64');

DEF Bd => SIMD8('Bd', 'Rd', 'write')  ;
DEF Dd => SIMD64('Dd', 'Rd', 'write') ;
DEF Hd => SIMD16('Hd', 'Rd', 'write') ;
DEF Sd => SIMD32('Sd', 'Rd', 'write') ;
DEF Vn => VECREG('Vn', 'Rn', 'read')  ;

T[' DUP REG:i8=$Bd  EREG={REG:i8=$Vn   VDS=B IDX={ENCODEDIN=imm5<4:1>}} ', 'imm5=0bxxxx1 ', 'DUP_asisdone_only     ', ' '];
T[' DUP REG:i16=$Hd EREG={REG:i16=$Vn VDS=H IDX={ENCODEDIN=imm5<4:2>}}  ', 'imm5=0bxxx10 ', 'DUP_asisdone_only     ', ' '];
T[' DUP REG:i32=$Sd EREG={REG:i32=$Vn VDS=S IDX={ENCODEDIN=imm5<4:3>}}  ', 'imm5=0bxx100 ', 'DUP_asisdone_only     ', ' '];
T[' DUP REG:i64=$Dd EREG={REG:i64=$Vn VDS=D IDX={ENCODEDIN=imm5<4>}}    ', 'imm5=0bx1000 ', 'DUP_asisdone_only     ', ' '];
T[' MOV REG:i8=$Bd  EREG={REG:i8=$Vn   VDS=B IDX={ENCODEDIN=imm5<4:1>}} ', 'imm5=0bxxxx1 ', 'MOV_DUP_asisdone_only ', ' '];
T[' MOV REG:i16=$Hd EREG={REG:i16=$Vn VDS=H IDX={ENCODEDIN=imm5<4:2>}}  ', 'imm5=0bxxx10 ', 'MOV_DUP_asisdone_only ', ' '];
T[' MOV REG:i32=$Sd EREG={REG:i32=$Vn VDS=S IDX={ENCODEDIN=imm5<4:3>}}  ', 'imm5=0bxx100 ', 'MOV_DUP_asisdone_only ', ' '];
T[' MOV REG:i64=$Dd EREG={REG:i64=$Vn VDS=D IDX={ENCODEDIN=imm5<4>}}    ', 'imm5=0bx1000 ', 'MOV_DUP_asisdone_only ', ' '];



# asisdpair: Advanced SIMD scalar pairwise.
# A64: ig0=0b01 U:u=0bx ig1=0b11110 size:u=0bxx ig2=0b11000 opcode:u=0bxxxxx ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asisdpair', 'A64');

DEF Dd => SIMD64('Dd', 'Rd', 'write') ;
DEF Hd => SIMD16('Hd', 'Rd', 'write') ;
DEF Sd => SIMD32('Sd', 'Rd', 'write') ;
DEF Vn => VECREG('Vn', 'Rn', 'read')  ;

T[' ADDP    REG:i64=$Dd VREG={REG:i64=$Vn VDT=2D } ', 'size=0b11 ', 'ADDP_asisdpair_only       ', '                                     '];
T[' FADDP   REG:f16=$Hd VREG={REG:f16=$Vn VDT=2H } ', 'NONE      ', 'FADDP_asisdpair_only_H    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FADDP   REG:f32=$Sd VREG={REG:f32=$Vn VDT=2S } ', 'sz=0b0    ', 'FADDP_asisdpair_only_SD   ', '                                     '];
T[' FADDP   REG:f64=$Dd VREG={REG:f64=$Vn VDT=2D } ', 'sz=0b1    ', 'FADDP_asisdpair_only_SD   ', '                                     '];
T[' FMAXNMP REG:f16=$Hd VREG={REG:f16=$Vn VDT=2H } ', 'NONE      ', 'FMAXNMP_asisdpair_only_H  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMAXNMP REG:f32=$Sd VREG={REG:f32=$Vn VDT=2S } ', 'sz=0b0    ', 'FMAXNMP_asisdpair_only_SD ', '                                     '];
T[' FMAXNMP REG:f64=$Dd VREG={REG:f64=$Vn VDT=2D } ', 'sz=0b1    ', 'FMAXNMP_asisdpair_only_SD ', '                                     '];
T[' FMAXP   REG:f16=$Hd VREG={REG:f16=$Vn VDT=2H } ', 'NONE      ', 'FMAXP_asisdpair_only_H    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMAXP   REG:f32=$Sd VREG={REG:f32=$Vn VDT=2S } ', 'sz=0b0    ', 'FMAXP_asisdpair_only_SD   ', '                                     '];
T[' FMAXP   REG:f64=$Dd VREG={REG:f64=$Vn VDT=2D } ', 'sz=0b1    ', 'FMAXP_asisdpair_only_SD   ', '                                     '];
T[' FMINNMP REG:f16=$Hd VREG={REG:f16=$Vn VDT=2H } ', 'NONE      ', 'FMINNMP_asisdpair_only_H  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMINNMP REG:f32=$Sd VREG={REG:f32=$Vn VDT=2S } ', 'sz=0b0    ', 'FMINNMP_asisdpair_only_SD ', '                                     '];
T[' FMINNMP REG:f64=$Dd VREG={REG:f64=$Vn VDT=2D } ', 'sz=0b1    ', 'FMINNMP_asisdpair_only_SD ', '                                     '];
T[' FMINP   REG:f16=$Hd VREG={REG:f16=$Vn VDT=2H } ', 'NONE      ', 'FMINP_asisdpair_only_H    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMINP   REG:f32=$Sd VREG={REG:f32=$Vn VDT=2S } ', 'sz=0b0    ', 'FMINP_asisdpair_only_SD   ', '                                     '];
T[' FMINP   REG:f64=$Dd VREG={REG:f64=$Vn VDT=2D } ', 'sz=0b1    ', 'FMINP_asisdpair_only_SD   ', '                                     '];



# asisdsame: Advanced SIMD scalar three same.
# A64: ig0=0b01 U:u=0bx ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode:u=0bxxxxx ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asisdsame', 'A64');

DEF Bd => SIMD8('Bd', 'Rd', 'write')  ;
DEF Bm => SIMD8('Bm', 'Rm', 'read')   ;
DEF Bn => SIMD8('Bn', 'Rn', 'read')   ;
DEF Dd => SIMD64('Dd', 'Rd', 'write') ;
DEF Dm => SIMD64('Dm', 'Rm', 'read')  ;
DEF Dn => SIMD64('Dn', 'Rn', 'read')  ;
DEF Hd => SIMD16('Hd', 'Rd', 'write') ;
DEF Hm => SIMD16('Hm', 'Rm', 'read')  ;
DEF Hn => SIMD16('Hn', 'Rn', 'read')  ;
DEF Sd => SIMD32('Sd', 'Rd', 'write') ;
DEF Sm => SIMD32('Sm', 'Rm', 'read')  ;
DEF Sn => SIMD32('Sn', 'Rn', 'read')  ;

T[' ADD      REG:i64=$Dd REG:i64=$Dn REG:i64=$Dm ', 'size=0b11 ', 'ADD_asisdsame_only      ', ' '];
T[' CMEQ     REG:i64=$Dd REG:i64=$Dn REG:i64=$Dm ', 'size=0b11 ', 'CMEQ_asisdsame_only     ', ' '];
T[' CMGE     REG:i64=$Dd REG:i64=$Dn REG:i64=$Dm ', 'size=0b11 ', 'CMGE_asisdsame_only     ', ' '];
T[' CMGT     REG:i64=$Dd REG:i64=$Dn REG:i64=$Dm ', 'size=0b11 ', 'CMGT_asisdsame_only     ', ' '];
T[' CMHI     REG:i64=$Dd REG:i64=$Dn REG:i64=$Dm ', 'size=0b11 ', 'CMHI_asisdsame_only     ', ' '];
T[' CMHS     REG:s64=$Dd REG:s64=$Dn REG:s64=$Dm ', 'size=0b11 ', 'CMHS_asisdsame_only     ', ' '];
T[' CMTST    REG:i64=$Dd REG:i64=$Dn REG:i64=$Dm ', 'size=0b11 ', 'CMTST_asisdsame_only    ', ' '];
T[' FABD     REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'sz=0b0    ', 'FABD_asisdsame_only     ', ' '];
T[' FABD     REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'sz=0b1    ', 'FABD_asisdsame_only     ', ' '];
T[' FACGE    REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'sz=0b0    ', 'FACGE_asisdsame_only    ', ' '];
T[' FACGE    REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'sz=0b1    ', 'FACGE_asisdsame_only    ', ' '];
T[' FACGT    REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'sz=0b0    ', 'FACGT_asisdsame_only    ', ' '];
T[' FACGT    REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'sz=0b1    ', 'FACGT_asisdsame_only    ', ' '];
T[' FCMEQ    REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'sz=0b0    ', 'FCMEQ_asisdsame_only    ', ' '];
T[' FCMEQ    REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'sz=0b1    ', 'FCMEQ_asisdsame_only    ', ' '];
T[' FCMGE    REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'sz=0b0    ', 'FCMGE_asisdsame_only    ', ' '];
T[' FCMGE    REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'sz=0b1    ', 'FCMGE_asisdsame_only    ', ' '];
T[' FCMGT    REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'sz=0b0    ', 'FCMGT_asisdsame_only    ', ' '];
T[' FCMGT    REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'sz=0b1    ', 'FCMGT_asisdsame_only    ', ' '];
T[' FMULX    REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'sz=0b0    ', 'FMULX_asisdsame_only    ', ' '];
T[' FMULX    REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'sz=0b1    ', 'FMULX_asisdsame_only    ', ' '];
T[' FRECPS   REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'sz=0b0    ', 'FRECPS_asisdsame_only   ', ' '];
T[' FRECPS   REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'sz=0b1    ', 'FRECPS_asisdsame_only   ', ' '];
T[' FRSQRTS  REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'sz=0b0    ', 'FRSQRTS_asisdsame_only  ', ' '];
T[' FRSQRTS  REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'sz=0b1    ', 'FRSQRTS_asisdsame_only  ', ' '];
T[' SQADD    REG:s8=$Bd  REG:s8=$Bn  REG:s8=$Bm  ', 'size=0b00 ', 'SQADD_asisdsame_only    ', ' '];
T[' SQADD    REG:s16=$Hd REG:s16=$Hn REG:s16=$Hm ', 'size=0b01 ', 'SQADD_asisdsame_only    ', ' '];
T[' SQADD    REG:s32=$Sd REG:s32=$Sn REG:s32=$Sm ', 'size=0b10 ', 'SQADD_asisdsame_only    ', ' '];
T[' SQADD    REG:s64=$Dd REG:s64=$Dn REG:s64=$Dm ', 'size=0b11 ', 'SQADD_asisdsame_only    ', ' '];
T[' SQDMULH  REG:s16=$Hd REG:s16=$Hn REG:s16=$Hm ', 'size=0b01 ', 'SQDMULH_asisdsame_only  ', ' '];
T[' SQDMULH  REG:s32=$Sd REG:s32=$Sn REG:s32=$Sm ', 'size=0b10 ', 'SQDMULH_asisdsame_only  ', ' '];
T[' SQRDMULH REG:s16=$Hd REG:s16=$Hn REG:s16=$Hm ', 'size=0b01 ', 'SQRDMULH_asisdsame_only ', ' '];
T[' SQRDMULH REG:s32=$Sd REG:s32=$Sn REG:s32=$Sm ', 'size=0b10 ', 'SQRDMULH_asisdsame_only ', ' '];
T[' SQRSHL   REG:s8=$Bd  REG:s8=$Bn  REG:s8=$Bm  ', 'size=0b00 ', 'SQRSHL_asisdsame_only   ', ' '];
T[' SQRSHL   REG:s16=$Hd REG:s16=$Hn REG:s16=$Hm ', 'size=0b01 ', 'SQRSHL_asisdsame_only   ', ' '];
T[' SQRSHL   REG:s32=$Sd REG:s32=$Sn REG:s32=$Sm ', 'size=0b10 ', 'SQRSHL_asisdsame_only   ', ' '];
T[' SQRSHL   REG:s64=$Dd REG:s64=$Dn REG:s64=$Dm ', 'size=0b11 ', 'SQRSHL_asisdsame_only   ', ' '];
T[' SQSHL    REG:s8=$Bd  REG:s8=$Bn  REG:s8=$Bm  ', 'size=0b00 ', 'SQSHL_asisdsame_only    ', ' '];
T[' SQSHL    REG:s16=$Hd REG:s16=$Hn REG:s16=$Hm ', 'size=0b01 ', 'SQSHL_asisdsame_only    ', ' '];
T[' SQSHL    REG:s32=$Sd REG:s32=$Sn REG:s32=$Sm ', 'size=0b10 ', 'SQSHL_asisdsame_only    ', ' '];
T[' SQSHL    REG:s64=$Dd REG:s64=$Dn REG:s64=$Dm ', 'size=0b11 ', 'SQSHL_asisdsame_only    ', ' '];
T[' SQSUB    REG:s8=$Bd  REG:s8=$Bn  REG:s8=$Bm  ', 'size=0b00 ', 'SQSUB_asisdsame_only    ', ' '];
T[' SQSUB    REG:s16=$Hd REG:s16=$Hn REG:s16=$Hm ', 'size=0b01 ', 'SQSUB_asisdsame_only    ', ' '];
T[' SQSUB    REG:s32=$Sd REG:s32=$Sn REG:s32=$Sm ', 'size=0b10 ', 'SQSUB_asisdsame_only    ', ' '];
T[' SQSUB    REG:s64=$Dd REG:s64=$Dn REG:s64=$Dm ', 'size=0b11 ', 'SQSUB_asisdsame_only    ', ' '];
T[' SRSHL    REG:s64=$Dd REG:s64=$Dn REG:s64=$Dm ', 'size=0b11 ', 'SRSHL_asisdsame_only    ', ' '];
T[' SSHL     REG:s64=$Dd REG:s64=$Dn REG:s64=$Dm ', 'size=0b11 ', 'SSHL_asisdsame_only     ', ' '];
T[' SUB      REG:i64=$Dd REG:i64=$Dn REG:i64=$Dm ', 'size=0b11 ', 'SUB_asisdsame_only      ', ' '];
T[' UQADD    REG:u8=$Bd  REG:u8=$Bn  REG:u8=$Bm  ', 'size=0b00 ', 'UQADD_asisdsame_only    ', ' '];
T[' UQADD    REG:u16=$Hd REG:u16=$Hn REG:u16=$Hm ', 'size=0b01 ', 'UQADD_asisdsame_only    ', ' '];
T[' UQADD    REG:u32=$Sd REG:u32=$Sn REG:u32=$Sm ', 'size=0b10 ', 'UQADD_asisdsame_only    ', ' '];
T[' UQADD    REG:u64=$Dd REG:u64=$Dn REG:u64=$Dm ', 'size=0b11 ', 'UQADD_asisdsame_only    ', ' '];
T[' UQRSHL   REG:u8=$Bd  REG:u8=$Bn  REG:u8=$Bm  ', 'size=0b00 ', 'UQRSHL_asisdsame_only   ', ' '];
T[' UQRSHL   REG:u16=$Hd REG:u16=$Hn REG:u16=$Hm ', 'size=0b01 ', 'UQRSHL_asisdsame_only   ', ' '];
T[' UQRSHL   REG:u32=$Sd REG:u32=$Sn REG:u32=$Sm ', 'size=0b10 ', 'UQRSHL_asisdsame_only   ', ' '];
T[' UQRSHL   REG:u64=$Dd REG:u64=$Dn REG:u64=$Dm ', 'size=0b11 ', 'UQRSHL_asisdsame_only   ', ' '];
T[' UQSHL    REG:u8=$Bd  REG:u8=$Bn  REG:u8=$Bm  ', 'size=0b00 ', 'UQSHL_asisdsame_only    ', ' '];
T[' UQSHL    REG:u16=$Hd REG:u16=$Hn REG:u16=$Hm ', 'size=0b01 ', 'UQSHL_asisdsame_only    ', ' '];
T[' UQSHL    REG:u32=$Sd REG:u32=$Sn REG:u32=$Sm ', 'size=0b10 ', 'UQSHL_asisdsame_only    ', ' '];
T[' UQSHL    REG:u64=$Dd REG:u64=$Dn REG:u64=$Dm ', 'size=0b11 ', 'UQSHL_asisdsame_only    ', ' '];
T[' UQSUB    REG:u8=$Bd  REG:u8=$Bn  REG:u8=$Bm  ', 'size=0b00 ', 'UQSUB_asisdsame_only    ', ' '];
T[' UQSUB    REG:u16=$Hd REG:u16=$Hn REG:u16=$Hm ', 'size=0b01 ', 'UQSUB_asisdsame_only    ', ' '];
T[' UQSUB    REG:u32=$Sd REG:u32=$Sn REG:u32=$Sm ', 'size=0b10 ', 'UQSUB_asisdsame_only    ', ' '];
T[' UQSUB    REG:u64=$Dd REG:u64=$Dn REG:u64=$Dm ', 'size=0b11 ', 'UQSUB_asisdsame_only    ', ' '];
T[' URSHL    REG:u64=$Dd REG:u64=$Dn REG:u64=$Dm ', 'size=0b11 ', 'URSHL_asisdsame_only    ', ' '];
T[' USHL     REG:u64=$Dd REG:u64=$Dn REG:u64=$Dm ', 'size=0b11 ', 'USHL_asisdsame_only     ', ' '];



# asisdsame2: Advanced SIMD scalar three same extra.
# A64: ig0=0b01 U:u=0bx ig1=0b11110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 opcode:u=0bxxxx ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asisdsame2', 'A64');

DEF Hd => SIMD16('Hd', 'Rd', 'write') ;
DEF Hm => SIMD16('Hm', 'Rm', 'read')  ;
DEF Hn => SIMD16('Hn', 'Rn', 'read')  ;
DEF Sd => SIMD32('Sd', 'Rd', 'write') ;
DEF Sm => SIMD32('Sm', 'Rm', 'read')  ;
DEF Sn => SIMD32('Sn', 'Rn', 'read')  ;

T[' SQRDMLAH REG:s16=$Hd REG:s16=$Hn REG:s16=$Hm ', 'size=0b01 ', 'SQRDMLAH_asisdsame2_only ', 'arch_variant=ARMv8v1 extensions=QRDMLAH '];
T[' SQRDMLAH REG:s32=$Sd REG:s32=$Sn REG:s32=$Sm ', 'size=0b10 ', 'SQRDMLAH_asisdsame2_only ', 'arch_variant=ARMv8v1 extensions=QRDMLAH '];
T[' SQRDMLSH REG:s16=$Hd REG:s16=$Hn REG:s16=$Hm ', 'size=0b01 ', 'SQRDMLSH_asisdsame2_only ', 'arch_variant=ARMv8v1 extensions=QRDMLAH '];
T[' SQRDMLSH REG:s32=$Sd REG:s32=$Sn REG:s32=$Sm ', 'size=0b10 ', 'SQRDMLSH_asisdsame2_only ', 'arch_variant=ARMv8v1 extensions=QRDMLAH '];



# asisdsamefp16: Advanced SIMD scalar three same FP16.
# A64: ig0=0b01 U:u=0bx ig1=0b11110 a:u=0bx ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode:u=0bxxx ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asisdsamefp16', 'A64');

DEF Hd => SIMD16('Hd', 'Rd', 'write') ;
DEF Hm => SIMD16('Hm', 'Rm', 'read')  ;
DEF Hn => SIMD16('Hn', 'Rn', 'read')  ;

T[' FABD    REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FABD_asisdsamefp16_only    ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FACGE   REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FACGE_asisdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FACGT   REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FACGT_asisdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMEQ   REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FCMEQ_asisdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMGE   REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FCMGE_asisdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMGT   REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FCMGT_asisdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMULX   REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FMULX_asisdsamefp16_only   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRECPS  REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FRECPS_asisdsamefp16_only  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FRSQRTS REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FRSQRTS_asisdsamefp16_only ', 'arch_variant=ARMv8v2 extensions=FP16 '];



# asisdshf: Advanced SIMD scalar shift by immediate.
# A64: ig0=0b01 U:u=0bx ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx opcode:u=0bxxxxx ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'asisdshf', 'A64');

DEF Bd           => SIMD8('Bd', 'Rd', 'write')                                     ;
DEF Bn           => SIMD8('Bn', 'Rn', 'read')                                      ;
DEF Dd           => SIMD64('Dd', 'Rd', 'write')                                    ;
DEF Dn           => SIMD64('Dn', 'Rn', 'read')                                     ;
DEF Hd           => SIMD16('Hd', 'Rd', 'write')                                    ;
DEF Hn           => SIMD16('Hn', 'Rn', 'read')                                     ;
DEF Sd           => SIMD32('Sd', 'Rd', 'write')                                    ;
DEF Sn           => SIMD32('Sn', 'Rn', 'read')                                     ;
DEF shift_left   => IMM('shift_left', 'immh:immb', range => (0, 63), arg0 => 64)   ;
DEF shift_left2  => IMM('shift_left', 'immh:immb', range => (0, 7), arg0 => 8)     ;
DEF shift_left3  => IMM('shift_left', 'immh:immb', range => (0, 15), arg0 => 16)   ;
DEF shift_left4  => IMM('shift_left', 'immh:immb', range => (0, 31), arg0 => 32)   ;
DEF shift_right  => IMM('shift_right', 'immh:immb', range => (1, 16), arg0 => 32)  ;
DEF shift_right2 => IMM('shift_right', 'immh:immb', range => (1, 32), arg0 => 64)  ;
DEF shift_right3 => IMM('shift_right', 'immh:immb', range => (1, 64), arg0 => 128) ;
DEF shift_right4 => IMM('shift_right', 'immh:immb', range => (1, 8), arg0 => 16)   ;

T[' FCVTZS   REG:f16=$Hd REG:f16=$Hn IMM:u8=$shift_right  ', 'immh=0b001x ', 'FCVTZS_asisdshf_C   ', 'extensions=FP16 '];
T[' FCVTZS   REG:f32=$Sd REG:f32=$Sn IMM:u8=$shift_right2 ', 'immh=0b01xx ', 'FCVTZS_asisdshf_C   ', 'extensions=FP16 '];
T[' FCVTZS   REG:f64=$Dd REG:f64=$Dn IMM:u8=$shift_right3 ', 'immh=0b1xxx ', 'FCVTZS_asisdshf_C   ', 'extensions=FP16 '];
T[' FCVTZU   REG:f16=$Hd REG:f16=$Hn IMM:u8=$shift_right  ', 'immh=0b001x ', 'FCVTZU_asisdshf_C   ', 'extensions=FP16 '];
T[' FCVTZU   REG:f32=$Sd REG:f32=$Sn IMM:u8=$shift_right2 ', 'immh=0b01xx ', 'FCVTZU_asisdshf_C   ', 'extensions=FP16 '];
T[' FCVTZU   REG:f64=$Dd REG:f64=$Dn IMM:u8=$shift_right3 ', 'immh=0b1xxx ', 'FCVTZU_asisdshf_C   ', 'extensions=FP16 '];
T[' SCVTF    REG:s16=$Hd REG:s16=$Hn IMM:u8=$shift_right  ', 'immh=0b001x ', 'SCVTF_asisdshf_C    ', 'extensions=FP16 '];
T[' SCVTF    REG:s32=$Sd REG:s32=$Sn IMM:u8=$shift_right2 ', 'immh=0b01xx ', 'SCVTF_asisdshf_C    ', 'extensions=FP16 '];
T[' SCVTF    REG:s64=$Dd REG:s64=$Dn IMM:u8=$shift_right3 ', 'immh=0b1xxx ', 'SCVTF_asisdshf_C    ', 'extensions=FP16 '];
T[' SHL      REG:i64=$Dd REG:i64=$Dn IMM:u8=$shift_left   ', 'immh=0b1xxx ', 'SHL_asisdshf_R      ', '                '];
T[' SLI      REG:i64=$Dd REG:i64=$Dn IMM:u8=$shift_left   ', 'immh=0b1xxx ', 'SLI_asisdshf_R      ', '                '];
T[' SQRSHRN  REG:s8=$Bd  REG:s16=$Hn IMM:u8=$shift_right4 ', 'immh=0b0001 ', 'SQRSHRN_asisdshf_N  ', '                '];
T[' SQRSHRN  REG:s16=$Hd REG:s32=$Sn IMM:u8=$shift_right  ', 'immh=0b001x ', 'SQRSHRN_asisdshf_N  ', '                '];
T[' SQRSHRN  REG:s32=$Sd REG:s64=$Dn IMM:u8=$shift_right2 ', 'immh=0b01xx ', 'SQRSHRN_asisdshf_N  ', '                '];
T[' SQRSHRUN REG:u8=$Bd  REG:u16=$Hn IMM:u8=$shift_right4 ', 'immh=0b0001 ', 'SQRSHRUN_asisdshf_N ', '                '];
T[' SQRSHRUN REG:u16=$Hd REG:u32=$Sn IMM:u8=$shift_right  ', 'immh=0b001x ', 'SQRSHRUN_asisdshf_N ', '                '];
T[' SQRSHRUN REG:u32=$Sd REG:u64=$Dn IMM:u8=$shift_right2 ', 'immh=0b01xx ', 'SQRSHRUN_asisdshf_N ', '                '];
T[' SQSHL    REG:s8=$Bd  REG:s8=$Bn  IMM:u8=$shift_left2  ', 'immh=0b0001 ', 'SQSHL_asisdshf_R    ', '                '];
T[' SQSHL    REG:s16=$Hd REG:s16=$Hn IMM:u8=$shift_left3  ', 'immh=0b001x ', 'SQSHL_asisdshf_R    ', '                '];
T[' SQSHL    REG:s32=$Sd REG:s32=$Sn IMM:u8=$shift_left4  ', 'immh=0b01xx ', 'SQSHL_asisdshf_R    ', '                '];
T[' SQSHL    REG:s64=$Dd REG:s64=$Dn IMM:u8=$shift_left   ', 'immh=0b1xxx ', 'SQSHL_asisdshf_R    ', '                '];
T[' SQSHLU   REG:u8=$Bd  REG:u8=$Bn  IMM:u8=$shift_left2  ', 'immh=0b0001 ', 'SQSHLU_asisdshf_R   ', '                '];
T[' SQSHLU   REG:u16=$Hd REG:u16=$Hn IMM:u8=$shift_left3  ', 'immh=0b001x ', 'SQSHLU_asisdshf_R   ', '                '];
T[' SQSHLU   REG:u32=$Sd REG:u32=$Sn IMM:u8=$shift_left4  ', 'immh=0b01xx ', 'SQSHLU_asisdshf_R   ', '                '];
T[' SQSHLU   REG:u64=$Dd REG:u64=$Dn IMM:u8=$shift_left   ', 'immh=0b1xxx ', 'SQSHLU_asisdshf_R   ', '                '];
T[' SQSHRN   REG:s8=$Bd  REG:s16=$Hn IMM:u8=$shift_right4 ', 'immh=0b0001 ', 'SQSHRN_asisdshf_N   ', '                '];
T[' SQSHRN   REG:s16=$Hd REG:s32=$Sn IMM:u8=$shift_right  ', 'immh=0b001x ', 'SQSHRN_asisdshf_N   ', '                '];
T[' SQSHRN   REG:s32=$Sd REG:s64=$Dn IMM:u8=$shift_right2 ', 'immh=0b01xx ', 'SQSHRN_asisdshf_N   ', '                '];
T[' SQSHRUN  REG:u8=$Bd  REG:u16=$Hn IMM:u8=$shift_right4 ', 'immh=0b0001 ', 'SQSHRUN_asisdshf_N  ', '                '];
T[' SQSHRUN  REG:u16=$Hd REG:u32=$Sn IMM:u8=$shift_right  ', 'immh=0b001x ', 'SQSHRUN_asisdshf_N  ', '                '];
T[' SQSHRUN  REG:u32=$Sd REG:u64=$Dn IMM:u8=$shift_right2 ', 'immh=0b01xx ', 'SQSHRUN_asisdshf_N  ', '                '];
T[' SRI      REG:i64=$Dd REG:i64=$Dn IMM:u8=$shift_right3 ', 'immh=0b1xxx ', 'SRI_asisdshf_R      ', '                '];
T[' SRSHR    REG:s64=$Dd REG:s64=$Dn IMM:u8=$shift_right3 ', 'immh=0b1xxx ', 'SRSHR_asisdshf_R    ', '                '];
T[' SRSRA    REG:s64=$Dd REG:s64=$Dn IMM:u8=$shift_right3 ', 'immh=0b1xxx ', 'SRSRA_asisdshf_R    ', '                '];
T[' SSHR     REG:s64=$Dd REG:s64=$Dn IMM:u8=$shift_right3 ', 'immh=0b1xxx ', 'SSHR_asisdshf_R     ', '                '];
T[' SSRA     REG:s64=$Dd REG:s64=$Dn IMM:u8=$shift_right3 ', 'immh=0b1xxx ', 'SSRA_asisdshf_R     ', '                '];
T[' UCVTF    REG:u16=$Hd REG:u16=$Hn IMM:u8=$shift_right  ', 'immh=0b001x ', 'UCVTF_asisdshf_C    ', 'extensions=FP16 '];
T[' UCVTF    REG:u32=$Sd REG:u32=$Sn IMM:u8=$shift_right2 ', 'immh=0b01xx ', 'UCVTF_asisdshf_C    ', 'extensions=FP16 '];
T[' UCVTF    REG:u64=$Dd REG:u64=$Dn IMM:u8=$shift_right3 ', 'immh=0b1xxx ', 'UCVTF_asisdshf_C    ', 'extensions=FP16 '];
T[' UQRSHRN  REG:u8=$Bd  REG:u16=$Hn IMM:u8=$shift_right4 ', 'immh=0b0001 ', 'UQRSHRN_asisdshf_N  ', '                '];
T[' UQRSHRN  REG:u16=$Hd REG:u32=$Sn IMM:u8=$shift_right  ', 'immh=0b001x ', 'UQRSHRN_asisdshf_N  ', '                '];
T[' UQRSHRN  REG:u32=$Sd REG:u64=$Dn IMM:u8=$shift_right2 ', 'immh=0b01xx ', 'UQRSHRN_asisdshf_N  ', '                '];
T[' UQSHL    REG:u8=$Bd  REG:u8=$Bn  IMM:u8=$shift_left2  ', 'immh=0b0001 ', 'UQSHL_asisdshf_R    ', '                '];
T[' UQSHL    REG:u16=$Hd REG:u16=$Hn IMM:u8=$shift_left3  ', 'immh=0b001x ', 'UQSHL_asisdshf_R    ', '                '];
T[' UQSHL    REG:u32=$Sd REG:u32=$Sn IMM:u8=$shift_left4  ', 'immh=0b01xx ', 'UQSHL_asisdshf_R    ', '                '];
T[' UQSHL    REG:u64=$Dd REG:u64=$Dn IMM:u8=$shift_left   ', 'immh=0b1xxx ', 'UQSHL_asisdshf_R    ', '                '];
T[' UQSHRN   REG:u8=$Bd  REG:u16=$Hn IMM:u8=$shift_right4 ', 'immh=0b0001 ', 'UQSHRN_asisdshf_N   ', '                '];
T[' UQSHRN   REG:u16=$Hd REG:u32=$Sn IMM:u8=$shift_right  ', 'immh=0b001x ', 'UQSHRN_asisdshf_N   ', '                '];
T[' UQSHRN   REG:u32=$Sd REG:u64=$Dn IMM:u8=$shift_right2 ', 'immh=0b01xx ', 'UQSHRN_asisdshf_N   ', '                '];
T[' URSHR    REG:u64=$Dd REG:u64=$Dn IMM:u8=$shift_right3 ', 'immh=0b1xxx ', 'URSHR_asisdshf_R    ', '                '];
T[' URSRA    REG:u64=$Dd REG:u64=$Dn IMM:u8=$shift_right3 ', 'immh=0b1xxx ', 'URSRA_asisdshf_R    ', '                '];
T[' USHR     REG:u64=$Dd REG:u64=$Dn IMM:u8=$shift_right3 ', 'immh=0b1xxx ', 'USHR_asisdshf_R     ', '                '];
T[' USRA     REG:u64=$Dd REG:u64=$Dn IMM:u8=$shift_right3 ', 'immh=0b1xxx ', 'USRA_asisdshf_R     ', '                '];



# crypto3_imm2: Cryptographic three-register, imm2.
# A64: ig0=0b11001110010 Rm:u=0bxxxxx ig1=0b10 imm2:u=0bxx opcode:u=0bxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'crypto3_imm2', 'A64');

DEF Vd => VECREG('Vd', 'Rd', 'write') ;
DEF Vm => VECREG('Vm', 'Rm', 'read')  ;
DEF Vn => VECREG('Vn', 'Rn', 'read')  ;

T[' SM3TT1A VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } EREG={REG:i32=$Vm VDS=S IDX={ENCODEDIN=imm2}} ', 'NONE ', 'SM3TT1A_VVV4_crypto3_imm2 ', 'arch_variant=ARMv8v2 extensions=SM3 '];
T[' SM3TT1B VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } EREG={REG:i32=$Vm VDS=S IDX={ENCODEDIN=imm2}} ', 'NONE ', 'SM3TT1B_VVV4_crypto3_imm2 ', 'arch_variant=ARMv8v2 extensions=SM3 '];
T[' SM3TT2A VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } EREG={REG:i32=$Vm VDS=S IDX={ENCODEDIN=imm2}} ', 'NONE ', 'SM3TT2A_VVV4_crypto3_imm2 ', 'arch_variant=ARMv8v2 extensions=SM3 '];
T[' SM3TT2B VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } EREG={REG:i32=$Vm VDS=S IDX={ENCODEDIN=imm2}} ', 'NONE ', 'SM3TT2B_VVV_crypto3_imm2  ', 'arch_variant=ARMv8v2 extensions=SM3 '];



# crypto3_imm6: Cryptographic three-register, imm6.
# A64: ig0=0b11001110100 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'crypto3_imm6', 'A64');

DEF Vd  => VECREG('Vd', 'Rd', 'write') ;
DEF Vm  => VECREG('Vm', 'Rm', 'read')  ;
DEF Vn  => VECREG('Vn', 'Rn', 'read')  ;
DEF imm => IMM('imm6', 'imm6')         ;

T[' XAR VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D } IMM:i64=$imm ', 'NONE ', 'XAR_VVV2_crypto3_imm6 ', 'arch_variant=ARMv8v2 extensions=SHA3 '];



# crypto4: Cryptographic four-register.
# A64: ig0=0b110011100 Op0:u=0bxx Rm:u=0bxxxxx ig1=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'crypto4', 'A64');

DEF Va => VECREG('Va', 'Ra', 'read')  ;
DEF Vd => VECREG('Vd', 'Rd', 'write') ;
DEF Vm => VECREG('Vm', 'Rm', 'read')  ;
DEF Vn => VECREG('Vn', 'Rn', 'read')  ;

T[' BCAX   VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} VREG={REG:i8=$Va   VDT=16B} ', 'NONE ', 'BCAX_VVV16_crypto4  ', 'arch_variant=ARMv8v2 extensions=SHA3 '];
T[' EOR3   VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} VREG={REG:i8=$Vm   VDT=16B} VREG={REG:i8=$Va   VDT=16B} ', 'NONE ', 'EOR3_VVV16_crypto4  ', 'arch_variant=ARMv8v2 extensions=SHA3 '];
T[' SM3SS1 VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S } VREG={REG:i32=$Va VDT=4S }     ', 'NONE ', 'SM3SS1_VVV4_crypto4 ', 'arch_variant=ARMv8v2 extensions=SM3  '];



# cryptoaes: Cryptographic AES.
# A64: ig0=0b01001110 size:u=0bxx ig1=0b10100 opcode:u=0bxxxxx ig2=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'cryptoaes', 'A64');

DEF Vd => VECREG('Vd', 'Rd', 'write') ;
DEF Vn => VECREG('Vn', 'Rn', 'read')  ;

T[' AESD   VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} ', 'NONE ', 'AESD_B_cryptoaes   ', 'extensions=AES '];
T[' AESE   VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} ', 'NONE ', 'AESE_B_cryptoaes   ', 'extensions=AES '];
T[' AESIMC VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} ', 'NONE ', 'AESIMC_B_cryptoaes ', 'extensions=AES '];
T[' AESMC  VREG={REG:i8=$Vd   VDT=16B} VREG={REG:i8=$Vn   VDT=16B} ', 'NONE ', 'AESMC_B_cryptoaes  ', 'extensions=AES '];



# cryptosha2: Cryptographic two-register SHA.
# A64: ig0=0b01011110 size:u=0bxx ig1=0b10100 opcode:u=0bxxxxx ig2=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'cryptosha2', 'A64');

DEF Sd => SIMD32('Sd', 'Rd', 'write') ;
DEF Sn => SIMD32('Sn', 'Rn', 'read')  ;
DEF Vd => VECREG('Vd', 'Rd', 'write') ;
DEF Vn => VECREG('Vn', 'Rn', 'read')  ;

T[' SHA1H     REG:i32=$Sd REG:i32=$Sn                               ', 'NONE ', 'SHA1H_SS_cryptosha2     ', 'extensions=SHA1   '];
T[' SHA1SU1   VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } ', 'NONE ', 'SHA1SU1_VV_cryptosha2   ', 'extensions=SHA1   '];
T[' SHA256SU0 VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } ', 'NONE ', 'SHA256SU0_VV_cryptosha2 ', 'extensions=SHA256 '];



# cryptosha3: Cryptographic three-register SHA.
# A64: ig0=0b01011110 size:u=0bxx ig1=0b0 Rm:u=0bxxxxx ig2=0b0 opcode:u=0bxxx ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'cryptosha3', 'A64');

DEF Qd => SIMD128('Qd', 'Rd', 'write') ;
DEF Qn => SIMD128('Qn', 'Rn', 'read')  ;
DEF Sn => SIMD32('Sn', 'Rn', 'read')   ;
DEF Vd => VECREG('Vd', 'Rd', 'write')  ;
DEF Vm => VECREG('Vm', 'Rm', 'read')   ;
DEF Vn => VECREG('Vn', 'Rn', 'read')   ;

T[' SHA1C     REG:i128=$Qd REG:i32=$Sn VREG={REG:i32=$Vm VDT=4S }                              ', 'NONE ', 'SHA1C_QSV_cryptosha3     ', 'extensions=SHA1   '];
T[' SHA1M     REG:i128=$Qd REG:i32=$Sn VREG={REG:i32=$Vm VDT=4S }                              ', 'NONE ', 'SHA1M_QSV_cryptosha3     ', 'extensions=SHA1   '];
T[' SHA1P     REG:i128=$Qd REG:i32=$Sn VREG={REG:i32=$Vm VDT=4S }                              ', 'NONE ', 'SHA1P_QSV_cryptosha3     ', 'extensions=SHA1   '];
T[' SHA1SU0   VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S } ', 'NONE ', 'SHA1SU0_VVV_cryptosha3   ', 'extensions=SHA1   '];
T[' SHA256H2  REG:i128=$Qd REG:i128=$Qn VREG={REG:i32=$Vm VDT=4S }                             ', 'NONE ', 'SHA256H2_QQV_cryptosha3  ', 'extensions=SHA256 '];
T[' SHA256H   REG:i128=$Qd REG:i128=$Qn VREG={REG:i32=$Vm VDT=4S }                             ', 'NONE ', 'SHA256H_QQV_cryptosha3   ', 'extensions=SHA256 '];
T[' SHA256SU1 VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S } ', 'NONE ', 'SHA256SU1_VVV_cryptosha3 ', 'extensions=SHA256 '];



# cryptosha512_2: Cryptographic two-register SHA 512.
# A64: ig0=0b11001110110000001000 opcode:u=0bxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'cryptosha512_2', 'A64');

DEF Vd => VECREG('Vd', 'Rd', 'write') ;
DEF Vn => VECREG('Vn', 'Rn', 'read')  ;

T[' SHA512SU0 VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } ', 'NONE ', 'SHA512SU0_VV2_cryptosha512_2 ', 'arch_variant=ARMv8v2 extensions=SHA512 '];
T[' SM4E      VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } ', 'NONE ', 'SM4E_VV4_cryptosha512_2      ', 'arch_variant=ARMv8v2 extensions=SM4    '];



# cryptosha512_3: Cryptographic three-register SHA 512.
# A64: ig0=0b11001110011 Rm:u=0bxxxxx ig1=0b1 O:u=0bx ig2=0b00 opcode:u=0bxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'cryptosha512_3', 'A64');

DEF Qd => SIMD128('Qd', 'Rd', 'write') ;
DEF Qn => SIMD128('Qn', 'Rn', 'read')  ;
DEF Vd => VECREG('Vd', 'Rd', 'write')  ;
DEF Vm => VECREG('Vm', 'Rm', 'read')   ;
DEF Vn => VECREG('Vn', 'Rn', 'read')   ;

T[' RAX1      VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D } ', 'NONE ', 'RAX1_VVV2_cryptosha512_3      ', 'arch_variant=ARMv8v2 extensions=SHA3   '];
T[' SHA512H2  REG:i128=$Qd REG:i128=$Qn VREG={REG:i64=$Vm VDT=2D }                             ', 'NONE ', 'SHA512H2_QQV_cryptosha512_3   ', 'arch_variant=ARMv8v2 extensions=SHA512 '];
T[' SHA512H   REG:i128=$Qd REG:i128=$Qn VREG={REG:i64=$Vm VDT=2D }                             ', 'NONE ', 'SHA512H_QQV_cryptosha512_3    ', 'arch_variant=ARMv8v2 extensions=SHA512 '];
T[' SHA512SU1 VREG={REG:i64=$Vd VDT=2D } VREG={REG:i64=$Vn VDT=2D } VREG={REG:i64=$Vm VDT=2D } ', 'NONE ', 'SHA512SU1_VVV2_cryptosha512_3 ', 'arch_variant=ARMv8v2 extensions=SHA512 '];
T[' SM3PARTW1 VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S } ', 'NONE ', 'SM3PARTW1_VVV4_cryptosha512_3 ', 'arch_variant=ARMv8v2 extensions=SM3    '];
T[' SM3PARTW2 VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S } ', 'NONE ', 'SM3PARTW2_VVV4_cryptosha512_3 ', 'arch_variant=ARMv8v2 extensions=SM3    '];
T[' SM4EKEY   VREG={REG:i32=$Vd VDT=4S } VREG={REG:i32=$Vn VDT=4S } VREG={REG:i32=$Vm VDT=4S } ', 'NONE ', 'SM4EKEY_VVV4_cryptosha512_3   ', 'arch_variant=ARMv8v2 extensions=SM4    '];



# float2fix: Conversion between floating-point and fixed-point.
# A64: sf:u=0bx ig0=0b0 S:u=0bx ig1=0b11110 ptype:u=0bxx ig2=0b0 rmode:u=0bxx opcode:u=0bxxx scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'float2fix', 'A64');

DEF Dd           => SIMD64('Dd', 'Rd', 'write')                               ;
DEF Dn           => SIMD64('Dn', 'Rn', 'read')                                ;
DEF Hd           => SIMD16('Hd', 'Rd', 'write')                               ;
DEF Hn           => SIMD16('Hn', 'Rn', 'read')                                ;
DEF Sd           => SIMD32('Sd', 'Rd', 'write')                               ;
DEF Sn           => SIMD32('Sn', 'Rn', 'read')                                ;
DEF Wd           => GPR32('Wd', 'Rd', 'write')                                ;
DEF Wn           => GPR32('Wn', 'Rn', 'read')                                 ;
DEF Xd           => GPR64('Xd', 'Rd', 'write')                                ;
DEF Xn           => GPR64('Xn', 'Rn', 'read')                                 ;
DEF fbits_right  => IMM('fbits_right', 'scale', range => (1, 32), arg0 => 64) ;
DEF fbits_right2 => IMM('fbits_right', 'scale', range => (1, 64), arg0 => 64) ;

T[' FCVTZS REG:s32=$Wd REG:f16=$Hn IMM:u8=$fbits_right  ', 'NONE ', 'FCVTZS_32H_float2fix ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTZS REG:s64=$Xd REG:f16=$Hn IMM:u8=$fbits_right2 ', 'NONE ', 'FCVTZS_64H_float2fix ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTZS REG:s32=$Wd REG:f32=$Sn IMM:u8=$fbits_right  ', 'NONE ', 'FCVTZS_32S_float2fix ', 'extensions=FP16                      '];
T[' FCVTZS REG:s64=$Xd REG:f32=$Sn IMM:u8=$fbits_right2 ', 'NONE ', 'FCVTZS_64S_float2fix ', 'extensions=FP16                      '];
T[' FCVTZS REG:s32=$Wd REG:f64=$Dn IMM:u8=$fbits_right  ', 'NONE ', 'FCVTZS_32D_float2fix ', 'extensions=FP16                      '];
T[' FCVTZS REG:s64=$Xd REG:f64=$Dn IMM:u8=$fbits_right2 ', 'NONE ', 'FCVTZS_64D_float2fix ', 'extensions=FP16                      '];
T[' FCVTZU REG:u32=$Wd REG:f16=$Hn IMM:u8=$fbits_right  ', 'NONE ', 'FCVTZU_32H_float2fix ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTZU REG:u64=$Xd REG:f16=$Hn IMM:u8=$fbits_right2 ', 'NONE ', 'FCVTZU_64H_float2fix ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCVTZU REG:u32=$Wd REG:f32=$Sn IMM:u8=$fbits_right  ', 'NONE ', 'FCVTZU_32S_float2fix ', 'extensions=FP16                      '];
T[' FCVTZU REG:u64=$Xd REG:f32=$Sn IMM:u8=$fbits_right2 ', 'NONE ', 'FCVTZU_64S_float2fix ', 'extensions=FP16                      '];
T[' FCVTZU REG:u32=$Wd REG:f64=$Dn IMM:u8=$fbits_right  ', 'NONE ', 'FCVTZU_32D_float2fix ', 'extensions=FP16                      '];
T[' FCVTZU REG:u64=$Xd REG:f64=$Dn IMM:u8=$fbits_right2 ', 'NONE ', 'FCVTZU_64D_float2fix ', 'extensions=FP16                      '];
T[' SCVTF  REG:s16=$Hd REG:s32=$Wn IMM:u8=$fbits_right  ', 'NONE ', 'SCVTF_H32_float2fix  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' SCVTF  REG:s32=$Sd REG:s32=$Wn IMM:u8=$fbits_right  ', 'NONE ', 'SCVTF_S32_float2fix  ', 'extensions=FP16                      '];
T[' SCVTF  REG:s64=$Dd REG:s32=$Wn IMM:u8=$fbits_right  ', 'NONE ', 'SCVTF_D32_float2fix  ', 'extensions=FP16                      '];
T[' SCVTF  REG:s16=$Hd REG:s64=$Xn IMM:u8=$fbits_right2 ', 'NONE ', 'SCVTF_H64_float2fix  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' SCVTF  REG:s32=$Sd REG:s64=$Xn IMM:u8=$fbits_right2 ', 'NONE ', 'SCVTF_S64_float2fix  ', 'extensions=FP16                      '];
T[' SCVTF  REG:s64=$Dd REG:s64=$Xn IMM:u8=$fbits_right2 ', 'NONE ', 'SCVTF_D64_float2fix  ', 'extensions=FP16                      '];
T[' UCVTF  REG:u16=$Hd REG:u32=$Wn IMM:u8=$fbits_right  ', 'NONE ', 'UCVTF_H32_float2fix  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' UCVTF  REG:u32=$Sd REG:u32=$Wn IMM:u8=$fbits_right  ', 'NONE ', 'UCVTF_S32_float2fix  ', 'extensions=FP16                      '];
T[' UCVTF  REG:u64=$Dd REG:u32=$Wn IMM:u8=$fbits_right  ', 'NONE ', 'UCVTF_D32_float2fix  ', 'extensions=FP16                      '];
T[' UCVTF  REG:u16=$Hd REG:u64=$Xn IMM:u8=$fbits_right2 ', 'NONE ', 'UCVTF_H64_float2fix  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' UCVTF  REG:u32=$Sd REG:u64=$Xn IMM:u8=$fbits_right2 ', 'NONE ', 'UCVTF_S64_float2fix  ', 'extensions=FP16                      '];
T[' UCVTF  REG:u64=$Dd REG:u64=$Xn IMM:u8=$fbits_right2 ', 'NONE ', 'UCVTF_D64_float2fix  ', 'extensions=FP16                      '];



# float2int: Conversion between floating-point and integer.
# A64: sf:u=0bx ig0=0b0 S:u=0bx ig1=0b11110 ptype:u=0bxx ig2=0b1 rmode:u=0bxx opcode:u=0bxxx ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'float2int', 'A64');

DEF Dd => SIMD64('Dd', 'Rd', 'write') ;
DEF Dn => SIMD64('Dn', 'Rn', 'read')  ;
DEF Hd => SIMD16('Hd', 'Rd', 'write') ;
DEF Hn => SIMD16('Hn', 'Rn', 'read')  ;
DEF Sd => SIMD32('Sd', 'Rd', 'write') ;
DEF Sn => SIMD32('Sn', 'Rn', 'read')  ;
DEF Vd => VECREG('Vd', 'Rd', 'write') ;
DEF Vn => VECREG('Vn', 'Rn', 'read')  ;
DEF Wd => GPR32('Wd', 'Rd', 'write')  ;
DEF Wn => GPR32('Wn', 'Rn', 'read')   ;
DEF Xd => GPR64('Xd', 'Rd', 'write')  ;
DEF Xn => GPR64('Xn', 'Rn', 'read')   ;

T[' FCVTAS  REG:s32=$Wd REG:f16=$Hn                ', 'NONE ', 'FCVTAS_32H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTAS  REG:s64=$Xd REG:f16=$Hn                ', 'NONE ', 'FCVTAS_64H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTAS  REG:s32=$Wd REG:f32=$Sn                ', 'NONE ', 'FCVTAS_32S_float2int  ', 'extensions=FP16                              '];
T[' FCVTAS  REG:s64=$Xd REG:f32=$Sn                ', 'NONE ', 'FCVTAS_64S_float2int  ', 'extensions=FP16                              '];
T[' FCVTAS  REG:s32=$Wd REG:f64=$Dn                ', 'NONE ', 'FCVTAS_32D_float2int  ', 'extensions=FP16                              '];
T[' FCVTAS  REG:s64=$Xd REG:f64=$Dn                ', 'NONE ', 'FCVTAS_64D_float2int  ', 'extensions=FP16                              '];
T[' FCVTAU  REG:u32=$Wd REG:f16=$Hn                ', 'NONE ', 'FCVTAU_32H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTAU  REG:u64=$Xd REG:f16=$Hn                ', 'NONE ', 'FCVTAU_64H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTAU  REG:u32=$Wd REG:f32=$Sn                ', 'NONE ', 'FCVTAU_32S_float2int  ', 'extensions=FP16                              '];
T[' FCVTAU  REG:u64=$Xd REG:f32=$Sn                ', 'NONE ', 'FCVTAU_64S_float2int  ', 'extensions=FP16                              '];
T[' FCVTAU  REG:u32=$Wd REG:f64=$Dn                ', 'NONE ', 'FCVTAU_32D_float2int  ', 'extensions=FP16                              '];
T[' FCVTAU  REG:u64=$Xd REG:f64=$Dn                ', 'NONE ', 'FCVTAU_64D_float2int  ', 'extensions=FP16                              '];
T[' FCVTMS  REG:s32=$Wd REG:f16=$Hn                ', 'NONE ', 'FCVTMS_32H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTMS  REG:s64=$Xd REG:f16=$Hn                ', 'NONE ', 'FCVTMS_64H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTMS  REG:s32=$Wd REG:f32=$Sn                ', 'NONE ', 'FCVTMS_32S_float2int  ', 'extensions=FP16                              '];
T[' FCVTMS  REG:s64=$Xd REG:f32=$Sn                ', 'NONE ', 'FCVTMS_64S_float2int  ', 'extensions=FP16                              '];
T[' FCVTMS  REG:s32=$Wd REG:f64=$Dn                ', 'NONE ', 'FCVTMS_32D_float2int  ', 'extensions=FP16                              '];
T[' FCVTMS  REG:s64=$Xd REG:f64=$Dn                ', 'NONE ', 'FCVTMS_64D_float2int  ', 'extensions=FP16                              '];
T[' FCVTMU  REG:u32=$Wd REG:f16=$Hn                ', 'NONE ', 'FCVTMU_32H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTMU  REG:u64=$Xd REG:f16=$Hn                ', 'NONE ', 'FCVTMU_64H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTMU  REG:u32=$Wd REG:f32=$Sn                ', 'NONE ', 'FCVTMU_32S_float2int  ', 'extensions=FP16                              '];
T[' FCVTMU  REG:u64=$Xd REG:f32=$Sn                ', 'NONE ', 'FCVTMU_64S_float2int  ', 'extensions=FP16                              '];
T[' FCVTMU  REG:u32=$Wd REG:f64=$Dn                ', 'NONE ', 'FCVTMU_32D_float2int  ', 'extensions=FP16                              '];
T[' FCVTMU  REG:u64=$Xd REG:f64=$Dn                ', 'NONE ', 'FCVTMU_64D_float2int  ', 'extensions=FP16                              '];
T[' FCVTNS  REG:s32=$Wd REG:f16=$Hn                ', 'NONE ', 'FCVTNS_32H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTNS  REG:s64=$Xd REG:f16=$Hn                ', 'NONE ', 'FCVTNS_64H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTNS  REG:s32=$Wd REG:f32=$Sn                ', 'NONE ', 'FCVTNS_32S_float2int  ', 'extensions=FP16                              '];
T[' FCVTNS  REG:s64=$Xd REG:f32=$Sn                ', 'NONE ', 'FCVTNS_64S_float2int  ', 'extensions=FP16                              '];
T[' FCVTNS  REG:s32=$Wd REG:f64=$Dn                ', 'NONE ', 'FCVTNS_32D_float2int  ', 'extensions=FP16                              '];
T[' FCVTNS  REG:s64=$Xd REG:f64=$Dn                ', 'NONE ', 'FCVTNS_64D_float2int  ', 'extensions=FP16                              '];
T[' FCVTNU  REG:u32=$Wd REG:f16=$Hn                ', 'NONE ', 'FCVTNU_32H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTNU  REG:u64=$Xd REG:f16=$Hn                ', 'NONE ', 'FCVTNU_64H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTNU  REG:u32=$Wd REG:f32=$Sn                ', 'NONE ', 'FCVTNU_32S_float2int  ', 'extensions=FP16                              '];
T[' FCVTNU  REG:u64=$Xd REG:f32=$Sn                ', 'NONE ', 'FCVTNU_64S_float2int  ', 'extensions=FP16                              '];
T[' FCVTNU  REG:u32=$Wd REG:f64=$Dn                ', 'NONE ', 'FCVTNU_32D_float2int  ', 'extensions=FP16                              '];
T[' FCVTNU  REG:u64=$Xd REG:f64=$Dn                ', 'NONE ', 'FCVTNU_64D_float2int  ', 'extensions=FP16                              '];
T[' FCVTPS  REG:s32=$Wd REG:f16=$Hn                ', 'NONE ', 'FCVTPS_32H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTPS  REG:s64=$Xd REG:f16=$Hn                ', 'NONE ', 'FCVTPS_64H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTPS  REG:s32=$Wd REG:f32=$Sn                ', 'NONE ', 'FCVTPS_32S_float2int  ', 'extensions=FP16                              '];
T[' FCVTPS  REG:s64=$Xd REG:f32=$Sn                ', 'NONE ', 'FCVTPS_64S_float2int  ', 'extensions=FP16                              '];
T[' FCVTPS  REG:s32=$Wd REG:f64=$Dn                ', 'NONE ', 'FCVTPS_32D_float2int  ', 'extensions=FP16                              '];
T[' FCVTPS  REG:s64=$Xd REG:f64=$Dn                ', 'NONE ', 'FCVTPS_64D_float2int  ', 'extensions=FP16                              '];
T[' FCVTPU  REG:u32=$Wd REG:f16=$Hn                ', 'NONE ', 'FCVTPU_32H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTPU  REG:u64=$Xd REG:f16=$Hn                ', 'NONE ', 'FCVTPU_64H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTPU  REG:u32=$Wd REG:f32=$Sn                ', 'NONE ', 'FCVTPU_32S_float2int  ', 'extensions=FP16                              '];
T[' FCVTPU  REG:u64=$Xd REG:f32=$Sn                ', 'NONE ', 'FCVTPU_64S_float2int  ', 'extensions=FP16                              '];
T[' FCVTPU  REG:u32=$Wd REG:f64=$Dn                ', 'NONE ', 'FCVTPU_32D_float2int  ', 'extensions=FP16                              '];
T[' FCVTPU  REG:u64=$Xd REG:f64=$Dn                ', 'NONE ', 'FCVTPU_64D_float2int  ', 'extensions=FP16                              '];
T[' FCVTZS  REG:s32=$Wd REG:f16=$Hn                ', 'NONE ', 'FCVTZS_32H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTZS  REG:s64=$Xd REG:f16=$Hn                ', 'NONE ', 'FCVTZS_64H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTZS  REG:s32=$Wd REG:f32=$Sn                ', 'NONE ', 'FCVTZS_32S_float2int  ', 'extensions=FP16                              '];
T[' FCVTZS  REG:s64=$Xd REG:f32=$Sn                ', 'NONE ', 'FCVTZS_64S_float2int  ', 'extensions=FP16                              '];
T[' FCVTZS  REG:s32=$Wd REG:f64=$Dn                ', 'NONE ', 'FCVTZS_32D_float2int  ', 'extensions=FP16                              '];
T[' FCVTZS  REG:s64=$Xd REG:f64=$Dn                ', 'NONE ', 'FCVTZS_64D_float2int  ', 'extensions=FP16                              '];
T[' FCVTZU  REG:u32=$Wd REG:f16=$Hn                ', 'NONE ', 'FCVTZU_32H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTZU  REG:u64=$Xd REG:f16=$Hn                ', 'NONE ', 'FCVTZU_64H_float2int  ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' FCVTZU  REG:u32=$Wd REG:f32=$Sn                ', 'NONE ', 'FCVTZU_32S_float2int  ', 'extensions=FP16                              '];
T[' FCVTZU  REG:u64=$Xd REG:f32=$Sn                ', 'NONE ', 'FCVTZU_64S_float2int  ', 'extensions=FP16                              '];
T[' FCVTZU  REG:u32=$Wd REG:f64=$Dn                ', 'NONE ', 'FCVTZU_32D_float2int  ', 'extensions=FP16                              '];
T[' FCVTZU  REG:u64=$Xd REG:f64=$Dn                ', 'NONE ', 'FCVTZU_64D_float2int  ', 'extensions=FP16                              '];
T[' FJCVTZS REG:s32=$Wd REG:f64=$Dn                ', 'NONE ', 'FJCVTZS_32D_float2int ', 'arch_variant=ARMv8v3 extensions=FJCVTZS      '];
T[' FMOV    REG=$Wd REG:f16=$Hn                    ', 'NONE ', 'FMOV_32H_float2int    ', 'arch_variant=ARMv8v2 extensions=FJCVTZS|FP16 '];
T[' FMOV    REG=$Xd REG:f16=$Hn                    ', 'NONE ', 'FMOV_64H_float2int    ', 'arch_variant=ARMv8v2 extensions=FJCVTZS|FP16 '];
T[' FMOV    REG:f16=$Hd REG=$Wn                    ', 'NONE ', 'FMOV_H32_float2int    ', 'arch_variant=ARMv8v2 extensions=FJCVTZS|FP16 '];
T[' FMOV    REG:f32=$Sd REG=$Wn                    ', 'NONE ', 'FMOV_S32_float2int    ', 'arch_variant=ARMv8v3 extensions=FJCVTZS|FP16 '];
T[' FMOV    REG=$Wd REG:f32=$Sn                    ', 'NONE ', 'FMOV_32S_float2int    ', 'arch_variant=ARMv8v3 extensions=FJCVTZS|FP16 '];
T[' FMOV    REG:f16=$Hd REG=$Xn                    ', 'NONE ', 'FMOV_H64_float2int    ', 'arch_variant=ARMv8v2 extensions=FJCVTZS|FP16 '];
T[' FMOV    REG:f64=$Dd REG=$Xn                    ', 'NONE ', 'FMOV_D64_float2int    ', 'arch_variant=ARMv8v3 extensions=FJCVTZS|FP16 '];
T[' FMOV    EREG={REG:f64=$Vd VDS=D IDX=1} REG=$Xn ', 'NONE ', 'FMOV_V64I_float2int   ', 'arch_variant=ARMv8v3 extensions=FJCVTZS|FP16 '];
T[' FMOV    REG=$Xd REG:f64=$Dn                    ', 'NONE ', 'FMOV_64D_float2int    ', 'arch_variant=ARMv8v3 extensions=FJCVTZS|FP16 '];
T[' FMOV    REG=$Xd EREG={REG:f64=$Vn VDS=D IDX=1} ', 'NONE ', 'FMOV_64VX_float2int   ', 'arch_variant=ARMv8v3 extensions=FJCVTZS|FP16 '];
T[' SCVTF   REG:s16=$Hd REG:s32=$Wn                ', 'NONE ', 'SCVTF_H32_float2int   ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' SCVTF   REG:s32=$Sd REG:s32=$Wn                ', 'NONE ', 'SCVTF_S32_float2int   ', 'extensions=FP16                              '];
T[' SCVTF   REG:s64=$Dd REG:s32=$Wn                ', 'NONE ', 'SCVTF_D32_float2int   ', 'extensions=FP16                              '];
T[' SCVTF   REG:s16=$Hd REG:s64=$Xn                ', 'NONE ', 'SCVTF_H64_float2int   ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' SCVTF   REG:s32=$Sd REG:s64=$Xn                ', 'NONE ', 'SCVTF_S64_float2int   ', 'extensions=FP16                              '];
T[' SCVTF   REG:s64=$Dd REG:s64=$Xn                ', 'NONE ', 'SCVTF_D64_float2int   ', 'extensions=FP16                              '];
T[' UCVTF   REG:u16=$Hd REG:u32=$Wn                ', 'NONE ', 'UCVTF_H32_float2int   ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' UCVTF   REG:u32=$Sd REG:u32=$Wn                ', 'NONE ', 'UCVTF_S32_float2int   ', 'extensions=FP16                              '];
T[' UCVTF   REG:u64=$Dd REG:u32=$Wn                ', 'NONE ', 'UCVTF_D32_float2int   ', 'extensions=FP16                              '];
T[' UCVTF   REG:u16=$Hd REG:u64=$Xn                ', 'NONE ', 'UCVTF_H64_float2int   ', 'arch_variant=ARMv8v2 extensions=FP16         '];
T[' UCVTF   REG:u32=$Sd REG:u64=$Xn                ', 'NONE ', 'UCVTF_S64_float2int   ', 'extensions=FP16                              '];
T[' UCVTF   REG:u64=$Dd REG:u64=$Xn                ', 'NONE ', 'UCVTF_D64_float2int   ', 'extensions=FP16                              '];



# floatccmp: Floating-point conditional compare.
# A64: M:u=0bx ig0=0b0 S:u=0bx ig1=0b11110 ptype:u=0bxx ig2=0b1 Rm:u=0bxxxxx cond:u=0bxxxx ig3=0b01 Rn:u=0bxxxxx op:u=0bx nzcv:u=0bxxxx
HINT('ICLASS', 'floatccmp', 'A64');

DEF Dm   => SIMD64('Dm', 'Rm', 'read')            ;
DEF Dn   => SIMD64('Dn', 'Rn', 'read')            ;
DEF Hm   => SIMD16('Hm', 'Rm', 'read')            ;
DEF Hn   => SIMD16('Hn', 'Rn', 'read')            ;
DEF Sm   => SIMD32('Sm', 'Rm', 'read')            ;
DEF Sn   => SIMD32('Sn', 'Rn', 'read')            ;
DEF cond => COND('cond', 'cond')                  ;
DEF nzcv => IMM('nzcv', 'nzcv', range => (0, 15)) ;

T[' FCCMP  REG:f16=$Hn REG:f16=$Hm IMM:u8=$nzcv COND=$cond ', 'NONE ', 'FCCMP_H_floatccmp  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCCMP  REG:f32=$Sn REG:f32=$Sm IMM:u8=$nzcv COND=$cond ', 'NONE ', 'FCCMP_S_floatccmp  ', 'extensions=FP16                      '];
T[' FCCMP  REG:f64=$Dn REG:f64=$Dm IMM:u8=$nzcv COND=$cond ', 'NONE ', 'FCCMP_D_floatccmp  ', 'extensions=FP16                      '];
T[' FCCMPE REG:f16=$Hn REG:f16=$Hm IMM:u8=$nzcv COND=$cond ', 'NONE ', 'FCCMPE_H_floatccmp ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCCMPE REG:f32=$Sn REG:f32=$Sm IMM:u8=$nzcv COND=$cond ', 'NONE ', 'FCCMPE_S_floatccmp ', 'extensions=FP16                      '];
T[' FCCMPE REG:f64=$Dn REG:f64=$Dm IMM:u8=$nzcv COND=$cond ', 'NONE ', 'FCCMPE_D_floatccmp ', 'extensions=FP16                      '];



# floatcmp: Floating-point compare.
# A64: M:u=0bx ig0=0b0 S:u=0bx ig1=0b11110 ptype:u=0bxx ig2=0b1 Rm:u=0bxxxxx op:u=0bxx ig3=0b1000 Rn:u=0bxxxxx opcode2:u=0bxxxxx
HINT('ICLASS', 'floatcmp', 'A64');

DEF Dm => SIMD64('Dm', 'Rm', 'read') ;
DEF Dn => SIMD64('Dn', 'Rn', 'read') ;
DEF Hm => SIMD16('Hm', 'Rm', 'read') ;
DEF Hn => SIMD16('Hn', 'Rn', 'read') ;
DEF Sm => SIMD32('Sm', 'Rm', 'read') ;
DEF Sn => SIMD32('Sn', 'Rn', 'read') ;

T[' FCMP  REG:f16=$Hn REG:f16=$Hm   ', 'NONE ', 'FCMP_H_floatcmp   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMP  REG:f16=$Hn FPIMM:f32=0.0 ', 'NONE ', 'FCMP_HZ_floatcmp  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMP  REG:f32=$Sn REG:f32=$Sm   ', 'NONE ', 'FCMP_S_floatcmp   ', 'extensions=FP16                      '];
T[' FCMP  REG:f32=$Sn FPIMM:f32=0.0 ', 'NONE ', 'FCMP_SZ_floatcmp  ', 'extensions=FP16                      '];
T[' FCMP  REG:f64=$Dn REG:f64=$Dm   ', 'NONE ', 'FCMP_D_floatcmp   ', 'extensions=FP16                      '];
T[' FCMP  REG:f64=$Dn FPIMM:f32=0.0 ', 'NONE ', 'FCMP_DZ_floatcmp  ', 'extensions=FP16                      '];
T[' FCMPE REG:f16=$Hn REG:f16=$Hm   ', 'NONE ', 'FCMPE_H_floatcmp  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMPE REG:f16=$Hn FPIMM:f32=0.0 ', 'NONE ', 'FCMPE_HZ_floatcmp ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCMPE REG:f32=$Sn REG:f32=$Sm   ', 'NONE ', 'FCMPE_S_floatcmp  ', 'extensions=FP16                      '];
T[' FCMPE REG:f32=$Sn FPIMM:f32=0.0 ', 'NONE ', 'FCMPE_SZ_floatcmp ', 'extensions=FP16                      '];
T[' FCMPE REG:f64=$Dn REG:f64=$Dm   ', 'NONE ', 'FCMPE_D_floatcmp  ', 'extensions=FP16                      '];
T[' FCMPE REG:f64=$Dn FPIMM:f32=0.0 ', 'NONE ', 'FCMPE_DZ_floatcmp ', 'extensions=FP16                      '];



# floatdp1: Floating-point data-processing (1 source).
# A64: M:u=0bx ig0=0b0 S:u=0bx ig1=0b11110 ptype:u=0bxx ig2=0b1 opcode:u=0bxxxxxx ig3=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'floatdp1', 'A64');

DEF Dd => SIMD64('Dd', 'Rd', 'write') ;
DEF Dn => SIMD64('Dn', 'Rn', 'read')  ;
DEF Hd => SIMD16('Hd', 'Rd', 'write') ;
DEF Hn => SIMD16('Hn', 'Rn', 'read')  ;
DEF Sd => SIMD32('Sd', 'Rd', 'write') ;
DEF Sn => SIMD32('Sn', 'Rn', 'read')  ;

T[' BFCVT    REG:i16=$Hd REG:i32=$Sn ', 'NONE ', 'BFCVT_BS_floatdp1   ', 'arch_variant=ARMv8v6 extensions=BF16  '];
T[' FABS     REG:f16=$Hd REG:f16=$Hn ', 'NONE ', 'FABS_H_floatdp1     ', 'arch_variant=ARMv8v2 extensions=FP16  '];
T[' FABS     REG:f32=$Sd REG:f32=$Sn ', 'NONE ', 'FABS_S_floatdp1     ', 'extensions=FP16                       '];
T[' FABS     REG:f64=$Dd REG:f64=$Dn ', 'NONE ', 'FABS_D_floatdp1     ', 'extensions=FP16                       '];
T[' FCVT     REG:f32=$Sd REG:f16=$Hn ', 'NONE ', 'FCVT_SH_floatdp1    ', '                                      '];
T[' FCVT     REG:f64=$Dd REG:f16=$Hn ', 'NONE ', 'FCVT_DH_floatdp1    ', '                                      '];
T[' FCVT     REG:f16=$Hd REG:f32=$Sn ', 'NONE ', 'FCVT_HS_floatdp1    ', '                                      '];
T[' FCVT     REG:f64=$Dd REG:f32=$Sn ', 'NONE ', 'FCVT_DS_floatdp1    ', '                                      '];
T[' FCVT     REG:f16=$Hd REG:f64=$Dn ', 'NONE ', 'FCVT_HD_floatdp1    ', '                                      '];
T[' FCVT     REG:f32=$Sd REG:f64=$Dn ', 'NONE ', 'FCVT_SD_floatdp1    ', '                                      '];
T[' FMOV     REG:f16=$Hd REG:f16=$Hn ', 'NONE ', 'FMOV_H_floatdp1     ', 'arch_variant=ARMv8v2 extensions=FP16  '];
T[' FMOV     REG:f32=$Sd REG:f32=$Sn ', 'NONE ', 'FMOV_S_floatdp1     ', 'extensions=FP16                       '];
T[' FMOV     REG:f64=$Dd REG:f64=$Dn ', 'NONE ', 'FMOV_D_floatdp1     ', 'extensions=FP16                       '];
T[' FNEG     REG:f16=$Hd REG:f16=$Hn ', 'NONE ', 'FNEG_H_floatdp1     ', 'arch_variant=ARMv8v2 extensions=FP16  '];
T[' FNEG     REG:f32=$Sd REG:f32=$Sn ', 'NONE ', 'FNEG_S_floatdp1     ', 'extensions=FP16                       '];
T[' FNEG     REG:f64=$Dd REG:f64=$Dn ', 'NONE ', 'FNEG_D_floatdp1     ', 'extensions=FP16                       '];
T[' FRINT32X REG:f32=$Sd REG:f32=$Sn ', 'NONE ', 'FRINT32X_S_floatdp1 ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT32X REG:f64=$Dd REG:f64=$Dn ', 'NONE ', 'FRINT32X_D_floatdp1 ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT32Z REG:f32=$Sd REG:f32=$Sn ', 'NONE ', 'FRINT32Z_S_floatdp1 ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT32Z REG:f64=$Dd REG:f64=$Dn ', 'NONE ', 'FRINT32Z_D_floatdp1 ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT64X REG:f32=$Sd REG:f32=$Sn ', 'NONE ', 'FRINT64X_S_floatdp1 ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT64X REG:f64=$Dd REG:f64=$Dn ', 'NONE ', 'FRINT64X_D_floatdp1 ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT64Z REG:f32=$Sd REG:f32=$Sn ', 'NONE ', 'FRINT64Z_S_floatdp1 ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINT64Z REG:f64=$Dd REG:f64=$Dn ', 'NONE ', 'FRINT64Z_D_floatdp1 ', 'arch_variant=ARMv8v5 extensions=FRINT '];
T[' FRINTA   REG:f16=$Hd REG:f16=$Hn ', 'NONE ', 'FRINTA_H_floatdp1   ', 'arch_variant=ARMv8v2 extensions=FP16  '];
T[' FRINTA   REG:f32=$Sd REG:f32=$Sn ', 'NONE ', 'FRINTA_S_floatdp1   ', 'extensions=FP16                       '];
T[' FRINTA   REG:f64=$Dd REG:f64=$Dn ', 'NONE ', 'FRINTA_D_floatdp1   ', 'extensions=FP16                       '];
T[' FRINTI   REG:f16=$Hd REG:f16=$Hn ', 'NONE ', 'FRINTI_H_floatdp1   ', 'arch_variant=ARMv8v2 extensions=FP16  '];
T[' FRINTI   REG:f32=$Sd REG:f32=$Sn ', 'NONE ', 'FRINTI_S_floatdp1   ', 'extensions=FP16                       '];
T[' FRINTI   REG:f64=$Dd REG:f64=$Dn ', 'NONE ', 'FRINTI_D_floatdp1   ', 'extensions=FP16                       '];
T[' FRINTM   REG:f16=$Hd REG:f16=$Hn ', 'NONE ', 'FRINTM_H_floatdp1   ', 'arch_variant=ARMv8v2 extensions=FP16  '];
T[' FRINTM   REG:f32=$Sd REG:f32=$Sn ', 'NONE ', 'FRINTM_S_floatdp1   ', 'extensions=FP16                       '];
T[' FRINTM   REG:f64=$Dd REG:f64=$Dn ', 'NONE ', 'FRINTM_D_floatdp1   ', 'extensions=FP16                       '];
T[' FRINTN   REG:f16=$Hd REG:f16=$Hn ', 'NONE ', 'FRINTN_H_floatdp1   ', 'arch_variant=ARMv8v2 extensions=FP16  '];
T[' FRINTN   REG:f32=$Sd REG:f32=$Sn ', 'NONE ', 'FRINTN_S_floatdp1   ', 'extensions=FP16                       '];
T[' FRINTN   REG:f64=$Dd REG:f64=$Dn ', 'NONE ', 'FRINTN_D_floatdp1   ', 'extensions=FP16                       '];
T[' FRINTP   REG:f16=$Hd REG:f16=$Hn ', 'NONE ', 'FRINTP_H_floatdp1   ', 'arch_variant=ARMv8v2 extensions=FP16  '];
T[' FRINTP   REG:f32=$Sd REG:f32=$Sn ', 'NONE ', 'FRINTP_S_floatdp1   ', 'extensions=FP16                       '];
T[' FRINTP   REG:f64=$Dd REG:f64=$Dn ', 'NONE ', 'FRINTP_D_floatdp1   ', 'extensions=FP16                       '];
T[' FRINTX   REG:f16=$Hd REG:f16=$Hn ', 'NONE ', 'FRINTX_H_floatdp1   ', 'arch_variant=ARMv8v2 extensions=FP16  '];
T[' FRINTX   REG:f32=$Sd REG:f32=$Sn ', 'NONE ', 'FRINTX_S_floatdp1   ', 'extensions=FP16                       '];
T[' FRINTX   REG:f64=$Dd REG:f64=$Dn ', 'NONE ', 'FRINTX_D_floatdp1   ', 'extensions=FP16                       '];
T[' FRINTZ   REG:f16=$Hd REG:f16=$Hn ', 'NONE ', 'FRINTZ_H_floatdp1   ', 'arch_variant=ARMv8v2 extensions=FP16  '];
T[' FRINTZ   REG:f32=$Sd REG:f32=$Sn ', 'NONE ', 'FRINTZ_S_floatdp1   ', 'extensions=FP16                       '];
T[' FRINTZ   REG:f64=$Dd REG:f64=$Dn ', 'NONE ', 'FRINTZ_D_floatdp1   ', 'extensions=FP16                       '];
T[' FSQRT    REG:f16=$Hd REG:f16=$Hn ', 'NONE ', 'FSQRT_H_floatdp1    ', 'arch_variant=ARMv8v2 extensions=FP16  '];
T[' FSQRT    REG:f32=$Sd REG:f32=$Sn ', 'NONE ', 'FSQRT_S_floatdp1    ', 'extensions=FP16                       '];
T[' FSQRT    REG:f64=$Dd REG:f64=$Dn ', 'NONE ', 'FSQRT_D_floatdp1    ', 'extensions=FP16                       '];



# floatdp2: Floating-point data-processing (2 source).
# A64: M:u=0bx ig0=0b0 S:u=0bx ig1=0b11110 ptype:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode:u=0bxxxx ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'floatdp2', 'A64');

DEF Dd => SIMD64('Dd', 'Rd', 'write') ;
DEF Dm => SIMD64('Dm', 'Rm', 'read')  ;
DEF Dn => SIMD64('Dn', 'Rn', 'read')  ;
DEF Hd => SIMD16('Hd', 'Rd', 'write') ;
DEF Hm => SIMD16('Hm', 'Rm', 'read')  ;
DEF Hn => SIMD16('Hn', 'Rn', 'read')  ;
DEF Sd => SIMD32('Sd', 'Rd', 'write') ;
DEF Sm => SIMD32('Sm', 'Rm', 'read')  ;
DEF Sn => SIMD32('Sn', 'Rn', 'read')  ;

T[' FADD   REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FADD_H_floatdp2   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FADD   REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'NONE ', 'FADD_S_floatdp2   ', 'extensions=FP16                      '];
T[' FADD   REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'NONE ', 'FADD_D_floatdp2   ', 'extensions=FP16                      '];
T[' FDIV   REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FDIV_H_floatdp2   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FDIV   REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'NONE ', 'FDIV_S_floatdp2   ', 'extensions=FP16                      '];
T[' FDIV   REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'NONE ', 'FDIV_D_floatdp2   ', 'extensions=FP16                      '];
T[' FMAX   REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FMAX_H_floatdp2   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMAX   REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'NONE ', 'FMAX_S_floatdp2   ', 'extensions=FP16                      '];
T[' FMAX   REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'NONE ', 'FMAX_D_floatdp2   ', 'extensions=FP16                      '];
T[' FMAXNM REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FMAXNM_H_floatdp2 ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMAXNM REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'NONE ', 'FMAXNM_S_floatdp2 ', 'extensions=FP16                      '];
T[' FMAXNM REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'NONE ', 'FMAXNM_D_floatdp2 ', 'extensions=FP16                      '];
T[' FMIN   REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FMIN_H_floatdp2   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMIN   REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'NONE ', 'FMIN_S_floatdp2   ', 'extensions=FP16                      '];
T[' FMIN   REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'NONE ', 'FMIN_D_floatdp2   ', 'extensions=FP16                      '];
T[' FMINNM REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FMINNM_H_floatdp2 ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMINNM REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'NONE ', 'FMINNM_S_floatdp2 ', 'extensions=FP16                      '];
T[' FMINNM REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'NONE ', 'FMINNM_D_floatdp2 ', 'extensions=FP16                      '];
T[' FMUL   REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FMUL_H_floatdp2   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMUL   REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'NONE ', 'FMUL_S_floatdp2   ', 'extensions=FP16                      '];
T[' FMUL   REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'NONE ', 'FMUL_D_floatdp2   ', 'extensions=FP16                      '];
T[' FNMUL  REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FNMUL_H_floatdp2  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FNMUL  REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'NONE ', 'FNMUL_S_floatdp2  ', 'extensions=FP16                      '];
T[' FNMUL  REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'NONE ', 'FNMUL_D_floatdp2  ', 'extensions=FP16                      '];
T[' FSUB   REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm ', 'NONE ', 'FSUB_H_floatdp2   ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FSUB   REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm ', 'NONE ', 'FSUB_S_floatdp2   ', 'extensions=FP16                      '];
T[' FSUB   REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm ', 'NONE ', 'FSUB_D_floatdp2   ', 'extensions=FP16                      '];



# floatdp3: Floating-point data-processing (3 source).
# A64: M:u=0bx ig0=0b0 S:u=0bx ig1=0b11111 ptype:u=0bxx o1:u=0bx Rm:u=0bxxxxx o0:u=0bx Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'floatdp3', 'A64');

DEF Da => SIMD64('Da', 'Ra', 'read')  ;
DEF Dd => SIMD64('Dd', 'Rd', 'write') ;
DEF Dm => SIMD64('Dm', 'Rm', 'read')  ;
DEF Dn => SIMD64('Dn', 'Rn', 'read')  ;
DEF Ha => SIMD16('Ha', 'Ra', 'read')  ;
DEF Hd => SIMD16('Hd', 'Rd', 'write') ;
DEF Hm => SIMD16('Hm', 'Rm', 'read')  ;
DEF Hn => SIMD16('Hn', 'Rn', 'read')  ;
DEF Sa => SIMD32('Sa', 'Ra', 'read')  ;
DEF Sd => SIMD32('Sd', 'Rd', 'write') ;
DEF Sm => SIMD32('Sm', 'Rm', 'read')  ;
DEF Sn => SIMD32('Sn', 'Rn', 'read')  ;

T[' FMADD  REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm REG:f16=$Ha ', 'NONE ', 'FMADD_H_floatdp3  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMADD  REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm REG:f32=$Sa ', 'NONE ', 'FMADD_S_floatdp3  ', 'extensions=FP16                      '];
T[' FMADD  REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm REG:f64=$Da ', 'NONE ', 'FMADD_D_floatdp3  ', 'extensions=FP16                      '];
T[' FMSUB  REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm REG:f16=$Ha ', 'NONE ', 'FMSUB_H_floatdp3  ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMSUB  REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm REG:f32=$Sa ', 'NONE ', 'FMSUB_S_floatdp3  ', 'extensions=FP16                      '];
T[' FMSUB  REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm REG:f64=$Da ', 'NONE ', 'FMSUB_D_floatdp3  ', 'extensions=FP16                      '];
T[' FNMADD REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm REG:f16=$Ha ', 'NONE ', 'FNMADD_H_floatdp3 ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FNMADD REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm REG:f32=$Sa ', 'NONE ', 'FNMADD_S_floatdp3 ', 'extensions=FP16                      '];
T[' FNMADD REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm REG:f64=$Da ', 'NONE ', 'FNMADD_D_floatdp3 ', 'extensions=FP16                      '];
T[' FNMSUB REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm REG:f16=$Ha ', 'NONE ', 'FNMSUB_H_floatdp3 ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FNMSUB REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm REG:f32=$Sa ', 'NONE ', 'FNMSUB_S_floatdp3 ', 'extensions=FP16                      '];
T[' FNMSUB REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm REG:f64=$Da ', 'NONE ', 'FNMSUB_D_floatdp3 ', 'extensions=FP16                      '];



# floatimm: Floating-point immediate.
# A64: M:u=0bx ig0=0b0 S:u=0bx ig1=0b11110 ptype:u=0bxx ig2=0b1 imm8:u=0bxxxxxxxx ig3=0b100 imm5:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'floatimm', 'A64');

DEF Dd     => SIMD64('Dd', 'Rd', 'write') ;
DEF Hd     => SIMD16('Hd', 'Rd', 'write') ;
DEF Sd     => SIMD32('Sd', 'Rd', 'write') ;
DEF vfpimm => FPIMM('vfpimm', 'imm8')     ;

T[' FMOV REG:f16=$Hd FPIMM:f16=$vfpimm ', 'NONE ', 'FMOV_H_floatimm ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FMOV REG:f32=$Sd FPIMM:f32=$vfpimm ', 'NONE ', 'FMOV_S_floatimm ', 'extensions=FP16                      '];
T[' FMOV REG:f64=$Dd FPIMM:f64=$vfpimm ', 'NONE ', 'FMOV_D_floatimm ', 'extensions=FP16                      '];



# floatsel: Floating-point conditional select.
# A64: M:u=0bx ig0=0b0 S:u=0bx ig1=0b11110 ptype:u=0bxx ig2=0b1 Rm:u=0bxxxxx cond:u=0bxxxx ig3=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'floatsel', 'A64');

DEF Dd   => SIMD64('Dd', 'Rd', 'write') ;
DEF Dm   => SIMD64('Dm', 'Rm', 'read')  ;
DEF Dn   => SIMD64('Dn', 'Rn', 'read')  ;
DEF Hd   => SIMD16('Hd', 'Rd', 'write') ;
DEF Hm   => SIMD16('Hm', 'Rm', 'read')  ;
DEF Hn   => SIMD16('Hn', 'Rn', 'read')  ;
DEF Sd   => SIMD32('Sd', 'Rd', 'write') ;
DEF Sm   => SIMD32('Sm', 'Rm', 'read')  ;
DEF Sn   => SIMD32('Sn', 'Rn', 'read')  ;
DEF cond => COND('cond', 'cond')        ;

T[' FCSEL REG:f16=$Hd REG:f16=$Hn REG:f16=$Hm COND=$cond ', 'NONE ', 'FCSEL_H_floatsel ', 'arch_variant=ARMv8v2 extensions=FP16 '];
T[' FCSEL REG:f32=$Sd REG:f32=$Sn REG:f32=$Sm COND=$cond ', 'NONE ', 'FCSEL_S_floatsel ', 'extensions=FP16                      '];
T[' FCSEL REG:f64=$Dd REG:f64=$Dn REG:f64=$Dm COND=$cond ', 'NONE ', 'FCSEL_D_floatsel ', 'extensions=FP16                      '];



# ldst_immpost: Load/store register (immediate post-indexed).
# A64: size:u=0bxx ig0=0b111 V:u=0bx ig1=0b00 opc:u=0bxx ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'ldst_immpost', 'A64');

DEF Bt   => SIMD8('Bt', 'Rt')                                               ;
DEF Dt   => SIMD64('Dt', 'Rt')                                              ;
DEF Ht   => SIMD16('Ht', 'Rt')                                              ;
DEF Qt   => SIMD128('Qt', 'Rt')                                             ;
DEF St   => SIMD32('St', 'Rt')                                              ;
DEF Wt   => GPR32('Wt', 'Rt')                                               ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                                    ;
DEF Xt   => GPR64('Xt', 'Rt')                                               ;
DEF simm => IMM('simm', 'imm9', signpos => 'imm9<8>', range => (-256, 255)) ;

T[' LDR   REG:w:i8=$Bt  MEM:PST:r:i8={ BASE=$XnSP SZ=8  IMMOFF:s16=$simm}    ', 'NONE ', 'LDR_B_ldst_immpost    ', ' '];
T[' LDR   REG:w:i16=$Ht MEM:PST:r:f16={BASE=$XnSP SZ=16 IMMOFF:s16=$simm}    ', 'NONE ', 'LDR_H_ldst_immpost    ', ' '];
T[' LDR   REG:w:i32=$St MEM:PST:r:f32={BASE=$XnSP SZ=32 IMMOFF:s16=$simm}    ', 'NONE ', 'LDR_S_ldst_immpost    ', ' '];
T[' LDR   REG:w:i64=$Dt MEM:PST:r:f64={BASE=$XnSP SZ=64 IMMOFF:s16=$simm}    ', 'NONE ', 'LDR_D_ldst_immpost    ', ' '];
T[' LDR   REG:w:i128=$Qt MEM:PST:r:i128={BASE=$XnSP SZ=128 IMMOFF:s16=$simm} ', 'NONE ', 'LDR_Q_ldst_immpost    ', ' '];
T[' LDR   REG:w=$Wt MEM:PST:r:i32={BASE=$XnSP SZ=32 IMMOFF:s16=$simm}        ', 'NONE ', 'LDR_32_ldst_immpost   ', ' '];
T[' LDR   REG:w=$Xt MEM:PST:r:i64={BASE=$XnSP SZ=64 IMMOFF:s16=$simm}        ', 'NONE ', 'LDR_64_ldst_immpost   ', ' '];
T[' LDRB  REG:w=$Wt MEM:PST:r:i8={ BASE=$XnSP SZ=8  IMMOFF:s16=$simm}        ', 'NONE ', 'LDRB_32_ldst_immpost  ', ' '];
T[' LDRH  REG:w=$Wt MEM:PST:r:i16={BASE=$XnSP SZ=16 IMMOFF:s16=$simm}        ', 'NONE ', 'LDRH_32_ldst_immpost  ', ' '];
T[' LDRSB REG:w:s32=$Wt MEM:PST:r:s8={ BASE=$XnSP SZ=8  IMMOFF:s16=$simm}    ', 'NONE ', 'LDRSB_32_ldst_immpost ', ' '];
T[' LDRSB REG:w:s64=$Xt MEM:PST:r:s8={ BASE=$XnSP SZ=8  IMMOFF:s16=$simm}    ', 'NONE ', 'LDRSB_64_ldst_immpost ', ' '];
T[' LDRSH REG:w:s32=$Wt MEM:PST:r:s16={BASE=$XnSP SZ=16 IMMOFF:s16=$simm}    ', 'NONE ', 'LDRSH_32_ldst_immpost ', ' '];
T[' LDRSH REG:w:s64=$Xt MEM:PST:r:s16={BASE=$XnSP SZ=16 IMMOFF:s16=$simm}    ', 'NONE ', 'LDRSH_64_ldst_immpost ', ' '];
T[' LDRSW REG:w:s64=$Xt MEM:PST:r:s32={BASE=$XnSP SZ=32 IMMOFF:s16=$simm}    ', 'NONE ', 'LDRSW_64_ldst_immpost ', ' '];
T[' STR   REG:r:i8=$Bt  MEM:PST:w:i8={ BASE=$XnSP SZ=8  IMMOFF:s16=$simm}    ', 'NONE ', 'STR_B_ldst_immpost    ', ' '];
T[' STR   REG:r:i16=$Ht MEM:PST:w:f16={BASE=$XnSP SZ=16 IMMOFF:s16=$simm}    ', 'NONE ', 'STR_H_ldst_immpost    ', ' '];
T[' STR   REG:r:i32=$St MEM:PST:w:f32={BASE=$XnSP SZ=32 IMMOFF:s16=$simm}    ', 'NONE ', 'STR_S_ldst_immpost    ', ' '];
T[' STR   REG:r:i64=$Dt MEM:PST:w:f64={BASE=$XnSP SZ=64 IMMOFF:s16=$simm}    ', 'NONE ', 'STR_D_ldst_immpost    ', ' '];
T[' STR   REG:r:i128=$Qt MEM:PST:w:i128={BASE=$XnSP SZ=128 IMMOFF:s16=$simm} ', 'NONE ', 'STR_Q_ldst_immpost    ', ' '];
T[' STR   REG:r=$Wt MEM:PST:w:i32={BASE=$XnSP SZ=32 IMMOFF:s16=$simm}        ', 'NONE ', 'STR_32_ldst_immpost   ', ' '];
T[' STR   REG:r=$Xt MEM:PST:w:i64={BASE=$XnSP SZ=64 IMMOFF:s16=$simm}        ', 'NONE ', 'STR_64_ldst_immpost   ', ' '];
T[' STRB  REG:r=$Wt MEM:PST:w:i8={ BASE=$XnSP SZ=8  IMMOFF:s16=$simm}        ', 'NONE ', 'STRB_32_ldst_immpost  ', ' '];
T[' STRH  REG:r=$Wt MEM:PST:w:i16={BASE=$XnSP SZ=16 IMMOFF:s16=$simm}        ', 'NONE ', 'STRH_32_ldst_immpost  ', ' '];



# ldst_immpre: Load/store register (immediate pre-indexed).
# A64: size:u=0bxx ig0=0b111 V:u=0bx ig1=0b00 opc:u=0bxx ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'ldst_immpre', 'A64');

DEF Bt   => SIMD8('Bt', 'Rt')                                               ;
DEF Dt   => SIMD64('Dt', 'Rt')                                              ;
DEF Ht   => SIMD16('Ht', 'Rt')                                              ;
DEF Qt   => SIMD128('Qt', 'Rt')                                             ;
DEF St   => SIMD32('St', 'Rt')                                              ;
DEF Wt   => GPR32('Wt', 'Rt')                                               ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                                    ;
DEF Xt   => GPR64('Xt', 'Rt')                                               ;
DEF simm => IMM('simm', 'imm9', signpos => 'imm9<8>', range => (-256, 255)) ;

T[' LDR   REG:w:i8=$Bt  MEM:PRE:r:i8={ BASE=$XnSP SZ=8  IMMOFF:s16=$simm}    ', 'NONE ', 'LDR_B_ldst_immpre    ', ' '];
T[' LDR   REG:w:i16=$Ht MEM:PRE:r:f16={BASE=$XnSP SZ=16 IMMOFF:s16=$simm}    ', 'NONE ', 'LDR_H_ldst_immpre    ', ' '];
T[' LDR   REG:w:i32=$St MEM:PRE:r:f32={BASE=$XnSP SZ=32 IMMOFF:s16=$simm}    ', 'NONE ', 'LDR_S_ldst_immpre    ', ' '];
T[' LDR   REG:w:i64=$Dt MEM:PRE:r:f64={BASE=$XnSP SZ=64 IMMOFF:s16=$simm}    ', 'NONE ', 'LDR_D_ldst_immpre    ', ' '];
T[' LDR   REG:w:i128=$Qt MEM:PRE:r:i128={BASE=$XnSP SZ=128 IMMOFF:s16=$simm} ', 'NONE ', 'LDR_Q_ldst_immpre    ', ' '];
T[' LDR   REG:w=$Wt MEM:PRE:r:i32={BASE=$XnSP SZ=32 IMMOFF:s16=$simm}        ', 'NONE ', 'LDR_32_ldst_immpre   ', ' '];
T[' LDR   REG:w=$Xt MEM:PRE:r:i64={BASE=$XnSP SZ=64 IMMOFF:s16=$simm}        ', 'NONE ', 'LDR_64_ldst_immpre   ', ' '];
T[' LDRB  REG:w=$Wt MEM:PRE:r:i8={ BASE=$XnSP SZ=8  IMMOFF:s16=$simm}        ', 'NONE ', 'LDRB_32_ldst_immpre  ', ' '];
T[' LDRH  REG:w=$Wt MEM:PRE:r:i16={BASE=$XnSP SZ=16 IMMOFF:s16=$simm}        ', 'NONE ', 'LDRH_32_ldst_immpre  ', ' '];
T[' LDRSB REG:w:s32=$Wt MEM:PRE:r:s8={ BASE=$XnSP SZ=8  IMMOFF:s16=$simm}    ', 'NONE ', 'LDRSB_32_ldst_immpre ', ' '];
T[' LDRSB REG:w:s64=$Xt MEM:PRE:r:s8={ BASE=$XnSP SZ=8  IMMOFF:s16=$simm}    ', 'NONE ', 'LDRSB_64_ldst_immpre ', ' '];
T[' LDRSH REG:w:s32=$Wt MEM:PRE:r:s16={BASE=$XnSP SZ=16 IMMOFF:s16=$simm}    ', 'NONE ', 'LDRSH_32_ldst_immpre ', ' '];
T[' LDRSH REG:w:s64=$Xt MEM:PRE:r:s16={BASE=$XnSP SZ=16 IMMOFF:s16=$simm}    ', 'NONE ', 'LDRSH_64_ldst_immpre ', ' '];
T[' LDRSW REG:w:s64=$Xt MEM:PRE:r:s32={BASE=$XnSP SZ=32 IMMOFF:s16=$simm}    ', 'NONE ', 'LDRSW_64_ldst_immpre ', ' '];
T[' STR   REG:r:i8=$Bt  MEM:PRE:w:i8={ BASE=$XnSP SZ=8  IMMOFF:s16=$simm}    ', 'NONE ', 'STR_B_ldst_immpre    ', ' '];
T[' STR   REG:r:i16=$Ht MEM:PRE:w:f16={BASE=$XnSP SZ=16 IMMOFF:s16=$simm}    ', 'NONE ', 'STR_H_ldst_immpre    ', ' '];
T[' STR   REG:r:i32=$St MEM:PRE:w:f32={BASE=$XnSP SZ=32 IMMOFF:s16=$simm}    ', 'NONE ', 'STR_S_ldst_immpre    ', ' '];
T[' STR   REG:r:i64=$Dt MEM:PRE:w:f64={BASE=$XnSP SZ=64 IMMOFF:s16=$simm}    ', 'NONE ', 'STR_D_ldst_immpre    ', ' '];
T[' STR   REG:r:i128=$Qt MEM:PRE:w:i128={BASE=$XnSP SZ=128 IMMOFF:s16=$simm} ', 'NONE ', 'STR_Q_ldst_immpre    ', ' '];
T[' STR   REG:r=$Wt MEM:PRE:w:i32={BASE=$XnSP SZ=32 IMMOFF:s16=$simm}        ', 'NONE ', 'STR_32_ldst_immpre   ', ' '];
T[' STR   REG:r=$Xt MEM:PRE:w:i64={BASE=$XnSP SZ=64 IMMOFF:s16=$simm}        ', 'NONE ', 'STR_64_ldst_immpre   ', ' '];
T[' STRB  REG:r=$Wt MEM:PRE:w:i8={ BASE=$XnSP SZ=8  IMMOFF:s16=$simm}        ', 'NONE ', 'STRB_32_ldst_immpre  ', ' '];
T[' STRH  REG:r=$Wt MEM:PRE:w:i16={BASE=$XnSP SZ=16 IMMOFF:s16=$simm}        ', 'NONE ', 'STRH_32_ldst_immpre  ', ' '];



# ldst_pos: Load/store register (unsigned immediate).
# A64: size:u=0bxx ig0=0b111 V:u=0bx ig1=0b01 opc:u=0bxx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'ldst_pos', 'A64');

DEF Bt      => SIMD8('Bt', 'Rt')                                                    ;
DEF Dt      => SIMD64('Dt', 'Rt')                                                   ;
DEF Ht      => SIMD16('Ht', 'Rt')                                                   ;
DEF Qt      => SIMD128('Qt', 'Rt')                                                  ;
DEF St      => SIMD32('St', 'Rt')                                                   ;
DEF Wt      => GPR32('Wt', 'Rt')                                                    ;
DEF XnSP    => GPR64('Xn', 'Rn', 'read|SP')                                         ;
DEF Xt      => GPR64('Xt', 'Rt')                                                    ;
DEF pimm    => IMM('pimm', 'imm12', range => (0, 4095), default => 0)               ;
DEF pimmx16 => IMM('pimm', 'imm12', scale => 16, range => (0, 65520), default => 0) ;
DEF pimmx2  => IMM('pimm', 'imm12', scale => 2, range => (0, 8190), default => 0)   ;
DEF pimmx4  => IMM('pimm', 'imm12', scale => 4, range => (0, 16380), default => 0)  ;
DEF pimmx8  => IMM('pimm', 'imm12', scale => 8, range => (0, 32760), default => 0)  ;
DEF prfop   => PRF_OP('prfop', 'Rt')                                                ;

T[' LDR   REG:w:i8=$Bt  MEM:OFF:r:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u16=$pimm}       ', 'NONE ', 'LDR_B_ldst_pos    ', ' '];
T[' LDR   REG:w:i16=$Ht MEM:OFF:r:f16={BASE=$XnSP SZ=16 IMMOFF:OPT:u16=$pimmx2}     ', 'NONE ', 'LDR_H_ldst_pos    ', ' '];
T[' LDR   REG:w:i32=$St MEM:OFF:r:f32={BASE=$XnSP SZ=32 IMMOFF:OPT:u16=$pimmx4}     ', 'NONE ', 'LDR_S_ldst_pos    ', ' '];
T[' LDR   REG:w:i64=$Dt MEM:OFF:r:f64={BASE=$XnSP SZ=64 IMMOFF:OPT:u16=$pimmx8}     ', 'NONE ', 'LDR_D_ldst_pos    ', ' '];
T[' LDR   REG:w:i128=$Qt MEM:OFF:r:i128={BASE=$XnSP SZ=128 IMMOFF:OPT:u16=$pimmx16} ', 'NONE ', 'LDR_Q_ldst_pos    ', ' '];
T[' LDR   REG:w=$Wt MEM:OFF:r:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:u16=$pimmx4}         ', 'NONE ', 'LDR_32_ldst_pos   ', ' '];
T[' LDR   REG:w=$Xt MEM:OFF:r:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:u16=$pimmx8}         ', 'NONE ', 'LDR_64_ldst_pos   ', ' '];
T[' LDRB  REG:w=$Wt MEM:OFF:r:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u16=$pimm}           ', 'NONE ', 'LDRB_32_ldst_pos  ', ' '];
T[' LDRH  REG:w=$Wt MEM:OFF:r:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:u16=$pimmx2}         ', 'NONE ', 'LDRH_32_ldst_pos  ', ' '];
T[' LDRSB REG:w:s32=$Wt MEM:OFF:r:s8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u16=$pimm}       ', 'NONE ', 'LDRSB_32_ldst_pos ', ' '];
T[' LDRSB REG:w:s64=$Xt MEM:OFF:r:s8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u16=$pimm}       ', 'NONE ', 'LDRSB_64_ldst_pos ', ' '];
T[' LDRSH REG:w:s32=$Wt MEM:OFF:r:s16={BASE=$XnSP SZ=16 IMMOFF:OPT:u16=$pimmx2}     ', 'NONE ', 'LDRSH_32_ldst_pos ', ' '];
T[' LDRSH REG:w:s64=$Xt MEM:OFF:r:s16={BASE=$XnSP SZ=16 IMMOFF:OPT:u16=$pimmx2}     ', 'NONE ', 'LDRSH_64_ldst_pos ', ' '];
T[' LDRSW REG:w:s64=$Xt MEM:OFF:r:s32={BASE=$XnSP SZ=32 IMMOFF:OPT:u16=$pimmx4}     ', 'NONE ', 'LDRSW_64_ldst_pos ', ' '];
T[' PRFM  PRF_OP=$prfop MEM:OFF:r:i0={BASE=$XnSP SZ=0  IMMOFF:OPT:u16=$pimmx8}      ', 'NONE ', 'PRFM_P_ldst_pos   ', ' '];
T[' STR   REG:r:i8=$Bt  MEM:OFF:w:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u16=$pimm}       ', 'NONE ', 'STR_B_ldst_pos    ', ' '];
T[' STR   REG:r:i16=$Ht MEM:OFF:w:f16={BASE=$XnSP SZ=16 IMMOFF:OPT:u16=$pimmx2}     ', 'NONE ', 'STR_H_ldst_pos    ', ' '];
T[' STR   REG:r:i32=$St MEM:OFF:w:f32={BASE=$XnSP SZ=32 IMMOFF:OPT:u16=$pimmx4}     ', 'NONE ', 'STR_S_ldst_pos    ', ' '];
T[' STR   REG:r:i64=$Dt MEM:OFF:w:f64={BASE=$XnSP SZ=64 IMMOFF:OPT:u16=$pimmx8}     ', 'NONE ', 'STR_D_ldst_pos    ', ' '];
T[' STR   REG:r:i128=$Qt MEM:OFF:w:i128={BASE=$XnSP SZ=128 IMMOFF:OPT:u16=$pimmx16} ', 'NONE ', 'STR_Q_ldst_pos    ', ' '];
T[' STR   REG:r=$Wt MEM:OFF:w:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:u16=$pimmx4}         ', 'NONE ', 'STR_32_ldst_pos   ', ' '];
T[' STR   REG:r=$Xt MEM:OFF:w:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:u16=$pimmx8}         ', 'NONE ', 'STR_64_ldst_pos   ', ' '];
T[' STRB  REG:r=$Wt MEM:OFF:w:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u16=$pimm}           ', 'NONE ', 'STRB_32_ldst_pos  ', ' '];
T[' STRH  REG:r=$Wt MEM:OFF:w:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:u16=$pimmx2}         ', 'NONE ', 'STRH_32_ldst_pos  ', ' '];



# ldst_regoff: Load/store register (register offset).
# A64: size:u=0bxx ig0=0b111 V:u=0bx ig1=0b00 opc:u=0bxx ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'ldst_regoff', 'A64');

DEF Bt      => SIMD8('Bt', 'Rt')                                                    ;
DEF Dt      => SIMD64('Dt', 'Rt')                                                   ;
DEF Ht      => SIMD16('Ht', 'Rt')                                                   ;
DEF Qt      => SIMD128('Qt', 'Rt')                                                  ;
DEF St      => SIMD32('St', 'Rt')                                                   ;
DEF Wm      => GPR32('Wm', 'Rm', 'read')                                            ;
DEF Wt      => GPR32('Wt', 'Rt')                                                    ;
DEF Xm      => GPR64('Xm', 'Rm', 'read')                                            ;
DEF XnSP    => GPR64('Xn', 'Rn', 'read|SP')                                         ;
DEF Xt      => GPR64('Xt', 'Rt')                                                    ;
DEF amount0 => IMM('amount', 'S')                                                   ;
DEF amount2 => IMM('amount', 'S', table => 'tbl_imm5')                              ;
DEF amount3 => IMM('amount', 'S', table => 'tbl_imm6')                              ;
DEF amount4 => IMM('amount', 'S', table => 'tbl_imm7')                              ;
DEF amount5 => IMM('amount', 'S', table => 'tbl_imm8')                              ;
DEF extend0 => EXTEND('extend', 'option', table => 'tbl_extend2')                   ;
DEF extend2 => EXTEND('extend', 'option', table => 'tbl_extend3', default => 'LSL') ;
DEF prfop   => PRF_OP('prfop', 'Rt')                                                ;

T[' LDR   REG:w:i8=$Bt  MEM:OFF:r:i8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Wm SHIFTER={EXTEND=$extend0 AMOUNT:i8=$amount0 }}}         ', 'option=0bxx0 ', 'LDR_B_ldst_regoff      ', ' '];
T[' LDR   REG:w:i8=$Bt  MEM:OFF:r:i8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Xm SHIFTER={EXTEND=$extend0 AMOUNT:i8=$amount0 }}}         ', 'option=0bxx1 ', 'LDR_B_ldst_regoff      ', ' '];
T[' LDR   REG:w:i8=$Bt  MEM:OFF:r:i8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Xm SHIFTER:OPT={SHIFT=LSL AMOUNT:i8=$amount0 }}}           ', 'NONE         ', 'LDR_BL_ldst_regoff     ', ' '];
T[' LDR   REG:w:i16=$Ht MEM:OFF:r:f16={BASE=$XnSP SZ=16 REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i16=$amount2}}}     ', 'option=0bxx0 ', 'LDR_H_ldst_regoff      ', ' '];
T[' LDR   REG:w:i16=$Ht MEM:OFF:r:f16={BASE=$XnSP SZ=16 REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i16=$amount2}}}     ', 'option=0bxx1 ', 'LDR_H_ldst_regoff      ', ' '];
T[' LDR   REG:w:i32=$St MEM:OFF:r:f32={BASE=$XnSP SZ=32 REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i32=$amount3}}}     ', 'option=0bxx0 ', 'LDR_S_ldst_regoff      ', ' '];
T[' LDR   REG:w:i32=$St MEM:OFF:r:f32={BASE=$XnSP SZ=32 REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i32=$amount3}}}     ', 'option=0bxx1 ', 'LDR_S_ldst_regoff      ', ' '];
T[' LDR   REG:w:i64=$Dt MEM:OFF:r:f64={BASE=$XnSP SZ=64 REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i64=$amount4}}}     ', 'option=0bxx0 ', 'LDR_D_ldst_regoff      ', ' '];
T[' LDR   REG:w:i64=$Dt MEM:OFF:r:f64={BASE=$XnSP SZ=64 REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i64=$amount4}}}     ', 'option=0bxx1 ', 'LDR_D_ldst_regoff      ', ' '];
T[' LDR   REG:w:i128=$Qt MEM:OFF:r:i128={BASE=$XnSP SZ=128 REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i128=$amount5}}} ', 'option=0bxx0 ', 'LDR_Q_ldst_regoff      ', ' '];
T[' LDR   REG:w:i128=$Qt MEM:OFF:r:i128={BASE=$XnSP SZ=128 REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i128=$amount5}}} ', 'option=0bxx1 ', 'LDR_Q_ldst_regoff      ', ' '];
T[' LDR   REG:w=$Wt MEM:OFF:r:i32={BASE=$XnSP SZ=32 REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i32=$amount3}}}         ', 'option=0bxx0 ', 'LDR_32_ldst_regoff     ', ' '];
T[' LDR   REG:w=$Wt MEM:OFF:r:i32={BASE=$XnSP SZ=32 REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i32=$amount3}}}         ', 'option=0bxx1 ', 'LDR_32_ldst_regoff     ', ' '];
T[' LDR   REG:w=$Xt MEM:OFF:r:i64={BASE=$XnSP SZ=64 REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i64=$amount4}}}         ', 'option=0bxx0 ', 'LDR_64_ldst_regoff     ', ' '];
T[' LDR   REG:w=$Xt MEM:OFF:r:i64={BASE=$XnSP SZ=64 REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i64=$amount4}}}         ', 'option=0bxx1 ', 'LDR_64_ldst_regoff     ', ' '];
T[' LDRB  REG:w=$Wt MEM:OFF:r:i8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Wm SHIFTER={EXTEND=$extend0 AMOUNT:i32=$amount0}}}             ', 'option=0bxx0 ', 'LDRB_32B_ldst_regoff   ', ' '];
T[' LDRB  REG:w=$Wt MEM:OFF:r:i8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Xm SHIFTER={EXTEND=$extend0 AMOUNT:i32=$amount0}}}             ', 'option=0bxx1 ', 'LDRB_32B_ldst_regoff   ', ' '];
T[' LDRB  REG:w=$Wt MEM:OFF:r:i8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Xm SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount0}}}               ', 'NONE         ', 'LDRB_32BL_ldst_regoff  ', ' '];
T[' LDRH  REG:w=$Wt MEM:OFF:r:i16={BASE=$XnSP SZ=16 REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i32=$amount2}}}         ', 'option=0bxx0 ', 'LDRH_32_ldst_regoff    ', ' '];
T[' LDRH  REG:w=$Wt MEM:OFF:r:i16={BASE=$XnSP SZ=16 REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i32=$amount2}}}         ', 'option=0bxx1 ', 'LDRH_32_ldst_regoff    ', ' '];
T[' LDRSB REG:w:s32=$Wt MEM:OFF:r:s8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Wm SHIFTER={EXTEND=$extend0 AMOUNT:i32=$amount0}}}         ', 'option=0bxx0 ', 'LDRSB_32B_ldst_regoff  ', ' '];
T[' LDRSB REG:w:s32=$Wt MEM:OFF:r:s8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Xm SHIFTER={EXTEND=$extend0 AMOUNT:i32=$amount0}}}         ', 'option=0bxx1 ', 'LDRSB_32B_ldst_regoff  ', ' '];
T[' LDRSB REG:w:s32=$Wt MEM:OFF:r:s8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Xm SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount0}}}           ', 'NONE         ', 'LDRSB_32BL_ldst_regoff ', ' '];
T[' LDRSB REG:w:s64=$Xt MEM:OFF:r:s8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Wm SHIFTER={EXTEND=$extend0 AMOUNT:i64=$amount0}}}         ', 'option=0bxx0 ', 'LDRSB_64B_ldst_regoff  ', ' '];
T[' LDRSB REG:w:s64=$Xt MEM:OFF:r:s8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Xm SHIFTER={EXTEND=$extend0 AMOUNT:i64=$amount0}}}         ', 'option=0bxx1 ', 'LDRSB_64B_ldst_regoff  ', ' '];
T[' LDRSB REG:w:s64=$Xt MEM:OFF:r:s8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Xm SHIFTER:OPT={SHIFT=LSL AMOUNT:i64=$amount0}}}           ', 'NONE         ', 'LDRSB_64BL_ldst_regoff ', ' '];
T[' LDRSH REG:w:s32=$Wt MEM:OFF:r:s16={BASE=$XnSP SZ=16 REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i32=$amount2}}}     ', 'option=0bxx0 ', 'LDRSH_32_ldst_regoff   ', ' '];
T[' LDRSH REG:w:s32=$Wt MEM:OFF:r:s16={BASE=$XnSP SZ=16 REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i32=$amount2}}}     ', 'option=0bxx1 ', 'LDRSH_32_ldst_regoff   ', ' '];
T[' LDRSH REG:w:s64=$Xt MEM:OFF:r:s16={BASE=$XnSP SZ=16 REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i64=$amount2}}}     ', 'option=0bxx0 ', 'LDRSH_64_ldst_regoff   ', ' '];
T[' LDRSH REG:w:s64=$Xt MEM:OFF:r:s16={BASE=$XnSP SZ=16 REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i64=$amount2}}}     ', 'option=0bxx1 ', 'LDRSH_64_ldst_regoff   ', ' '];
T[' LDRSW REG:w:s64=$Xt MEM:OFF:r:s32={BASE=$XnSP SZ=32 REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i64=$amount3}}}     ', 'option=0bxx0 ', 'LDRSW_64_ldst_regoff   ', ' '];
T[' LDRSW REG:w:s64=$Xt MEM:OFF:r:s32={BASE=$XnSP SZ=32 REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i64=$amount3}}}     ', 'option=0bxx1 ', 'LDRSW_64_ldst_regoff   ', ' '];
T[' PRFM  PRF_OP=$prfop MEM:OFF:r:i0={BASE=$XnSP SZ=0  REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i32=$amount4}}}      ', 'option=0bxx0 ', 'PRFM_P_ldst_regoff     ', ' '];
T[' PRFM  PRF_OP=$prfop MEM:OFF:r:i0={BASE=$XnSP SZ=0  REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i32=$amount4}}}      ', 'option=0bxx1 ', 'PRFM_P_ldst_regoff     ', ' '];
T[' STR   REG:r:i8=$Bt  MEM:OFF:w:i8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Wm SHIFTER={EXTEND=$extend0 AMOUNT:i8=$amount0 }}}         ', 'option=0bxx0 ', 'STR_B_ldst_regoff      ', ' '];
T[' STR   REG:r:i8=$Bt  MEM:OFF:w:i8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Xm SHIFTER={EXTEND=$extend0 AMOUNT:i8=$amount0 }}}         ', 'option=0bxx1 ', 'STR_B_ldst_regoff      ', ' '];
T[' STR   REG:r:i8=$Bt  MEM:OFF:w:i8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Xm SHIFTER:OPT={SHIFT=LSL AMOUNT:i8=$amount0 }}}           ', 'NONE         ', 'STR_BL_ldst_regoff     ', ' '];
T[' STR   REG:r:i16=$Ht MEM:OFF:w:f16={BASE=$XnSP SZ=16 REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i16=$amount2}}}     ', 'option=0bxx0 ', 'STR_H_ldst_regoff      ', ' '];
T[' STR   REG:r:i16=$Ht MEM:OFF:w:f16={BASE=$XnSP SZ=16 REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i16=$amount2}}}     ', 'option=0bxx1 ', 'STR_H_ldst_regoff      ', ' '];
T[' STR   REG:r:i32=$St MEM:OFF:w:f32={BASE=$XnSP SZ=32 REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i32=$amount3}}}     ', 'option=0bxx0 ', 'STR_S_ldst_regoff      ', ' '];
T[' STR   REG:r:i32=$St MEM:OFF:w:f32={BASE=$XnSP SZ=32 REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i32=$amount3}}}     ', 'option=0bxx1 ', 'STR_S_ldst_regoff      ', ' '];
T[' STR   REG:r:i64=$Dt MEM:OFF:w:f64={BASE=$XnSP SZ=64 REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i64=$amount4}}}     ', 'option=0bxx0 ', 'STR_D_ldst_regoff      ', ' '];
T[' STR   REG:r:i64=$Dt MEM:OFF:w:f64={BASE=$XnSP SZ=64 REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i64=$amount4}}}     ', 'option=0bxx1 ', 'STR_D_ldst_regoff      ', ' '];
T[' STR   REG:r:i128=$Qt MEM:OFF:w:i128={BASE=$XnSP SZ=128 REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i128=$amount5}}} ', 'option=0bxx0 ', 'STR_Q_ldst_regoff      ', ' '];
T[' STR   REG:r:i128=$Qt MEM:OFF:w:i128={BASE=$XnSP SZ=128 REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i128=$amount5}}} ', 'option=0bxx1 ', 'STR_Q_ldst_regoff      ', ' '];
T[' STR   REG:r=$Wt MEM:OFF:w:i32={BASE=$XnSP SZ=32 REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i32=$amount3}}}         ', 'option=0bxx0 ', 'STR_32_ldst_regoff     ', ' '];
T[' STR   REG:r=$Wt MEM:OFF:w:i32={BASE=$XnSP SZ=32 REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i32=$amount3}}}         ', 'option=0bxx1 ', 'STR_32_ldst_regoff     ', ' '];
T[' STR   REG:r=$Xt MEM:OFF:w:i64={BASE=$XnSP SZ=64 REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i64=$amount4}}}         ', 'option=0bxx0 ', 'STR_64_ldst_regoff     ', ' '];
T[' STR   REG:r=$Xt MEM:OFF:w:i64={BASE=$XnSP SZ=64 REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i64=$amount4}}}         ', 'option=0bxx1 ', 'STR_64_ldst_regoff     ', ' '];
T[' STRB  REG:r=$Wt MEM:OFF:w:i8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Wm SHIFTER={EXTEND=$extend0 AMOUNT:i32=$amount0}}}             ', 'option=0bxx0 ', 'STRB_32B_ldst_regoff   ', ' '];
T[' STRB  REG:r=$Wt MEM:OFF:w:i8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Xm SHIFTER={EXTEND=$extend0 AMOUNT:i32=$amount0}}}             ', 'option=0bxx1 ', 'STRB_32B_ldst_regoff   ', ' '];
T[' STRB  REG:r=$Wt MEM:OFF:w:i8={ BASE=$XnSP SZ=8  REGOFFSH={REG=$Xm SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount0}}}               ', 'NONE         ', 'STRB_32BL_ldst_regoff  ', ' '];
T[' STRH  REG:r=$Wt MEM:OFF:w:i16={BASE=$XnSP SZ=16 REGOFFSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i32=$amount2}}}         ', 'option=0bxx0 ', 'STRH_32_ldst_regoff    ', ' '];
T[' STRH  REG:r=$Wt MEM:OFF:w:i16={BASE=$XnSP SZ=16 REGOFFSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:i32=$amount2}}}         ', 'option=0bxx1 ', 'STRH_32_ldst_regoff    ', ' '];



# ldst_unscaled: Load/store register (unscaled immediate).
# A64: size:u=0bxx ig0=0b111 V:u=0bx ig1=0b00 opc:u=0bxx ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'ldst_unscaled', 'A64');

DEF Bt    => SIMD8('Bt', 'Rt')                                                             ;
DEF Dt    => SIMD64('Dt', 'Rt')                                                            ;
DEF Ht    => SIMD16('Ht', 'Rt')                                                            ;
DEF Qt    => SIMD128('Qt', 'Rt')                                                           ;
DEF St    => SIMD32('St', 'Rt')                                                            ;
DEF Wt    => GPR32('Wt', 'Rt')                                                             ;
DEF XnSP  => GPR64('Xn', 'Rn', 'read|SP')                                                  ;
DEF Xt    => GPR64('Xt', 'Rt')                                                             ;
DEF prfop => PRF_OP('prfop', 'Rt')                                                         ;
DEF simm  => IMM('simm', 'imm9', signpos => 'imm9<8>', range => (-256, 255), default => 0) ;

T[' LDUR   REG:w:i8=$Bt  MEM:OFF:r:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:s16=$simm}    ', 'NONE ', 'LDUR_B_ldst_unscaled    ', ' '];
T[' LDUR   REG:w:i16=$Ht MEM:OFF:r:f16={BASE=$XnSP SZ=16 IMMOFF:OPT:s16=$simm}    ', 'NONE ', 'LDUR_H_ldst_unscaled    ', ' '];
T[' LDUR   REG:w:i32=$St MEM:OFF:r:f32={BASE=$XnSP SZ=32 IMMOFF:OPT:s16=$simm}    ', 'NONE ', 'LDUR_S_ldst_unscaled    ', ' '];
T[' LDUR   REG:w:i64=$Dt MEM:OFF:r:f64={BASE=$XnSP SZ=64 IMMOFF:OPT:s16=$simm}    ', 'NONE ', 'LDUR_D_ldst_unscaled    ', ' '];
T[' LDUR   REG:w:i128=$Qt MEM:OFF:r:i128={BASE=$XnSP SZ=128 IMMOFF:OPT:s16=$simm} ', 'NONE ', 'LDUR_Q_ldst_unscaled    ', ' '];
T[' LDUR   REG:w=$Wt MEM:OFF:r:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:s16=$simm}        ', 'NONE ', 'LDUR_32_ldst_unscaled   ', ' '];
T[' LDUR   REG:w=$Xt MEM:OFF:r:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:s16=$simm}        ', 'NONE ', 'LDUR_64_ldst_unscaled   ', ' '];
T[' LDURB  REG:w=$Wt MEM:OFF:r:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:s16=$simm}        ', 'NONE ', 'LDURB_32_ldst_unscaled  ', ' '];
T[' LDURH  REG:w=$Wt MEM:OFF:r:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:s16=$simm}        ', 'NONE ', 'LDURH_32_ldst_unscaled  ', ' '];
T[' LDURSB REG:w:s32=$Wt MEM:OFF:r:s8={ BASE=$XnSP SZ=8  IMMOFF:OPT:s16=$simm}    ', 'NONE ', 'LDURSB_32_ldst_unscaled ', ' '];
T[' LDURSB REG:w:s64=$Xt MEM:OFF:r:s8={ BASE=$XnSP SZ=8  IMMOFF:OPT:s16=$simm}    ', 'NONE ', 'LDURSB_64_ldst_unscaled ', ' '];
T[' LDURSH REG:w:s32=$Wt MEM:OFF:r:s16={BASE=$XnSP SZ=16 IMMOFF:OPT:s16=$simm}    ', 'NONE ', 'LDURSH_32_ldst_unscaled ', ' '];
T[' LDURSH REG:w:s64=$Xt MEM:OFF:r:s16={BASE=$XnSP SZ=16 IMMOFF:OPT:s16=$simm}    ', 'NONE ', 'LDURSH_64_ldst_unscaled ', ' '];
T[' LDURSW REG:w:s64=$Xt MEM:OFF:r:s32={BASE=$XnSP SZ=32 IMMOFF:OPT:s16=$simm}    ', 'NONE ', 'LDURSW_64_ldst_unscaled ', ' '];
T[' PRFUM  PRF_OP=$prfop MEM:OFF:r:i0={BASE=$XnSP SZ=0  IMMOFF:OPT:s16=$simm}     ', 'NONE ', 'PRFUM_P_ldst_unscaled   ', ' '];
T[' STUR   REG:r:i8=$Bt  MEM:OFF:w:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:s16=$simm}    ', 'NONE ', 'STUR_B_ldst_unscaled    ', ' '];
T[' STUR   REG:r:i16=$Ht MEM:OFF:w:f16={BASE=$XnSP SZ=16 IMMOFF:OPT:s16=$simm}    ', 'NONE ', 'STUR_H_ldst_unscaled    ', ' '];
T[' STUR   REG:r:i32=$St MEM:OFF:w:f32={BASE=$XnSP SZ=32 IMMOFF:OPT:s16=$simm}    ', 'NONE ', 'STUR_S_ldst_unscaled    ', ' '];
T[' STUR   REG:r:i64=$Dt MEM:OFF:w:f64={BASE=$XnSP SZ=64 IMMOFF:OPT:s16=$simm}    ', 'NONE ', 'STUR_D_ldst_unscaled    ', ' '];
T[' STUR   REG:r:i128=$Qt MEM:OFF:w:i128={BASE=$XnSP SZ=128 IMMOFF:OPT:s16=$simm} ', 'NONE ', 'STUR_Q_ldst_unscaled    ', ' '];
T[' STUR   REG:r=$Wt MEM:OFF:w:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:s16=$simm}        ', 'NONE ', 'STUR_32_ldst_unscaled   ', ' '];
T[' STUR   REG:r=$Xt MEM:OFF:w:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:s16=$simm}        ', 'NONE ', 'STUR_64_ldst_unscaled   ', ' '];
T[' STURB  REG:r=$Wt MEM:OFF:w:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:s16=$simm}        ', 'NONE ', 'STURB_32_ldst_unscaled  ', ' '];
T[' STURH  REG:r=$Wt MEM:OFF:w:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:s16=$simm}        ', 'NONE ', 'STURH_32_ldst_unscaled  ', ' '];



# ldstnapair_offs: Load/store no-allocate pair (offset).
# A64: opc:u=0bxx ig0=0b101 V:u=0bx ig1=0b000 L:u=0bx imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'ldstnapair_offs', 'A64');

DEF Dt     => SIMD64('Dt1', 'Rt')                                                                         ;
DEF Dt2    => SIMD64('Dt2', 'Rt2')                                                                        ;
DEF Qt     => SIMD128('Qt1', 'Rt')                                                                        ;
DEF Qt2    => SIMD128('Qt2', 'Rt2')                                                                       ;
DEF St     => SIMD32('St1', 'Rt')                                                                         ;
DEF St2    => SIMD32('St2', 'Rt2')                                                                        ;
DEF Wt     => GPR32('Wt1', 'Rt')                                                                          ;
DEF Wt2    => GPR32('Wt2', 'Rt2')                                                                         ;
DEF XnSP   => GPR64('Xn', 'Rn', 'read|SP')                                                                ;
DEF Xt     => GPR64('Xt1', 'Rt')                                                                          ;
DEF Xt2    => GPR64('Xt2', 'Rt2')                                                                         ;
DEF immx16 => IMM('imm', 'imm7', scale => 16, signpos => 'imm7<6>', range => (-1024, 1008), default => 0) ;
DEF immx4  => IMM('imm', 'imm7', scale => 4, signpos => 'imm7<6>', range => (-256, 252), default => 0)    ;
DEF immx8  => IMM('imm', 'imm7', scale => 8, signpos => 'imm7<6>', range => (-512, 504), default => 0)    ;

T[' LDNP REG:w:i32=$St REG:w:i32=$St2 MEM:OFF:r:f32={BASE=$XnSP SZ=2x32 IMMOFF:OPT:s16=$immx4}      ', 'NONE ', 'LDNP_S_ldstnapair_offs  ', ' '];
T[' LDNP REG:w:i64=$Dt REG:w:i64=$Dt2 MEM:OFF:r:f64={BASE=$XnSP SZ=2x64 IMMOFF:OPT:s16=$immx8}      ', 'NONE ', 'LDNP_D_ldstnapair_offs  ', ' '];
T[' LDNP REG:w:i128=$Qt REG:w:i128=$Qt2 MEM:OFF:r:i128={BASE=$XnSP SZ=2x128 IMMOFF:OPT:s16=$immx16} ', 'NONE ', 'LDNP_Q_ldstnapair_offs  ', ' '];
T[' LDNP REG:w=$Wt REG:w=$Wt2 MEM:OFF:r:i32={BASE=$XnSP SZ=2x32 IMMOFF:OPT:s16=$immx4}              ', 'NONE ', 'LDNP_32_ldstnapair_offs ', ' '];
T[' LDNP REG:w=$Xt REG:w=$Xt2 MEM:OFF:r:i64={BASE=$XnSP SZ=2x64 IMMOFF:OPT:s16=$immx8}              ', 'NONE ', 'LDNP_64_ldstnapair_offs ', ' '];
T[' STNP REG:r:i32=$St REG:r:i32=$St2 MEM:OFF:w:f32={BASE=$XnSP SZ=2x32 IMMOFF:OPT:s16=$immx4}      ', 'NONE ', 'STNP_S_ldstnapair_offs  ', ' '];
T[' STNP REG:r:i64=$Dt REG:r:i64=$Dt2 MEM:OFF:w:f64={BASE=$XnSP SZ=2x64 IMMOFF:OPT:s16=$immx8}      ', 'NONE ', 'STNP_D_ldstnapair_offs  ', ' '];
T[' STNP REG:r:i128=$Qt REG:r:i128=$Qt2 MEM:OFF:w:i128={BASE=$XnSP SZ=2x128 IMMOFF:OPT:s16=$immx16} ', 'NONE ', 'STNP_Q_ldstnapair_offs  ', ' '];
T[' STNP REG:r=$Wt REG:r=$Wt2 MEM:OFF:w:i32={BASE=$XnSP SZ=2x32 IMMOFF:OPT:s16=$immx4}              ', 'NONE ', 'STNP_32_ldstnapair_offs ', ' '];
T[' STNP REG:r=$Xt REG:r=$Xt2 MEM:OFF:w:i64={BASE=$XnSP SZ=2x64 IMMOFF:OPT:s16=$immx8}              ', 'NONE ', 'STNP_64_ldstnapair_offs ', ' '];



# ldstpair_off: Load/store register pair (offset).
# A64: opc:u=0bxx ig0=0b101 V:u=0bx ig1=0b010 L:u=0bx imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'ldstpair_off', 'A64');

DEF Dt       => SIMD64('Dt1', 'Rt')                                                                           ;
DEF Dt2      => SIMD64('Dt2', 'Rt2')                                                                          ;
DEF Qt       => SIMD128('Qt1', 'Rt')                                                                          ;
DEF Qt2      => SIMD128('Qt2', 'Rt2')                                                                         ;
DEF St       => SIMD32('St1', 'Rt')                                                                           ;
DEF St2      => SIMD32('St2', 'Rt2')                                                                          ;
DEF Wt       => GPR32('Wt1', 'Rt')                                                                            ;
DEF Wt2      => GPR32('Wt2', 'Rt2')                                                                           ;
DEF XnSP     => GPR64('Xn', 'Rn', 'read|SP')                                                                  ;
DEF XnSP2    => GPR64('Xn', 'Xn', 'read|SP')                                                                  ;
DEF Xt       => GPR64('Xt1', 'Rt')                                                                            ;
DEF Xt2      => GPR64('Xt2', 'Rt2')                                                                           ;
DEF Xt3      => GPR64('Xt1', 'Xt')                                                                            ;
DEF Xt4      => GPR64('Xt2', 'Xt2')                                                                           ;
DEF immx16   => IMM('imm', 'imm7', scale => 16, signpos => 'imm7<6>', range => (-1024, 1008), default => 0)   ;
DEF immx16v2 => IMM('imm', 'simm7', scale => 16, signpos => 'simm7<6>', range => (-1024, 1008), default => 0) ;
DEF immx4    => IMM('imm', 'imm7', scale => 4, signpos => 'imm7<6>', range => (-256, 252), default => 0)      ;
DEF immx8    => IMM('imm', 'imm7', scale => 8, signpos => 'imm7<6>', range => (-512, 504), default => 0)      ;

T[' LDP   REG:w:i32=$St REG:w:i32=$St2 MEM:OFF:r:f32={BASE=$XnSP SZ=2x32 IMMOFF:OPT:s16=$immx4}      ', 'NONE ', 'LDP_S_ldstpair_off    ', '                     '];
T[' LDP   REG:w:i64=$Dt REG:w:i64=$Dt2 MEM:OFF:r:f64={BASE=$XnSP SZ=2x64 IMMOFF:OPT:s16=$immx8}      ', 'NONE ', 'LDP_D_ldstpair_off    ', '                     '];
T[' LDP   REG:w:i128=$Qt REG:w:i128=$Qt2 MEM:OFF:r:i128={BASE=$XnSP SZ=2x128 IMMOFF:OPT:s16=$immx16} ', 'NONE ', 'LDP_Q_ldstpair_off    ', '                     '];
T[' LDP   REG:w=$Wt REG:w=$Wt2 MEM:OFF:r:i32={BASE=$XnSP SZ=2x32 IMMOFF:OPT:s16=$immx4}              ', 'NONE ', 'LDP_32_ldstpair_off   ', '                     '];
T[' LDP   REG:w=$Xt REG:w=$Xt2 MEM:OFF:r:i64={BASE=$XnSP SZ=2x64 IMMOFF:OPT:s16=$immx8}              ', 'NONE ', 'LDP_64_ldstpair_off   ', '                     '];
T[' LDPSW REG:w:s64=$Xt REG:w:s64=$Xt2 MEM:OFF:r:s32={BASE=$XnSP SZ=2x32 IMMOFF:OPT:s16=$immx4}      ', 'NONE ', 'LDPSW_64_ldstpair_off ', '                     '];
T[' STGP  REG:r=$Xt3 REG:r=$Xt4 MEM:OFF:w:i64={BASE=$XnSP2 SZ=2x64 IMMOFF:OPT:s16=$immx16v2}         ', 'NONE ', 'STGP_64_ldstpair_off  ', 'arch_variant=ARMv8v5 '];
T[' STP   REG:r:i32=$St REG:r:i32=$St2 MEM:OFF:w:f32={BASE=$XnSP SZ=2x32 IMMOFF:OPT:s16=$immx4}      ', 'NONE ', 'STP_S_ldstpair_off    ', '                     '];
T[' STP   REG:r:i64=$Dt REG:r:i64=$Dt2 MEM:OFF:w:f64={BASE=$XnSP SZ=2x64 IMMOFF:OPT:s16=$immx8}      ', 'NONE ', 'STP_D_ldstpair_off    ', '                     '];
T[' STP   REG:r:i128=$Qt REG:r:i128=$Qt2 MEM:OFF:w:i128={BASE=$XnSP SZ=2x128 IMMOFF:OPT:s16=$immx16} ', 'NONE ', 'STP_Q_ldstpair_off    ', '                     '];
T[' STP   REG:r=$Wt REG:r=$Wt2 MEM:OFF:w:i32={BASE=$XnSP SZ=2x32 IMMOFF:OPT:s16=$immx4}              ', 'NONE ', 'STP_32_ldstpair_off   ', '                     '];
T[' STP   REG:r=$Xt REG:r=$Xt2 MEM:OFF:w:i64={BASE=$XnSP SZ=2x64 IMMOFF:OPT:s16=$immx8}              ', 'NONE ', 'STP_64_ldstpair_off   ', '                     '];



# ldstpair_post: Load/store register pair (post-indexed).
# A64: opc:u=0bxx ig0=0b101 V:u=0bx ig1=0b001 L:u=0bx imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'ldstpair_post', 'A64');

DEF Dt       => SIMD64('Dt1', 'Rt')                                                             ;
DEF Dt2      => SIMD64('Dt2', 'Rt2')                                                            ;
DEF Qt       => SIMD128('Qt1', 'Rt')                                                            ;
DEF Qt2      => SIMD128('Qt2', 'Rt2')                                                           ;
DEF St       => SIMD32('St1', 'Rt')                                                             ;
DEF St2      => SIMD32('St2', 'Rt2')                                                            ;
DEF Wt       => GPR32('Wt1', 'Rt')                                                              ;
DEF Wt2      => GPR32('Wt2', 'Rt2')                                                             ;
DEF XnSP     => GPR64('Xn', 'Rn', 'read|SP')                                                    ;
DEF XnSP2    => GPR64('Xn', 'Xn', 'read|SP')                                                    ;
DEF Xt       => GPR64('Xt1', 'Rt')                                                              ;
DEF Xt2      => GPR64('Xt2', 'Rt2')                                                             ;
DEF Xt3      => GPR64('Xt1', 'Xt')                                                              ;
DEF Xt4      => GPR64('Xt2', 'Xt2')                                                             ;
DEF immx16   => IMM('imm', 'imm7', scale => 16, signpos => 'imm7<6>', range => (-1024, 1008))   ;
DEF immx16v2 => IMM('imm', 'simm7', scale => 16, signpos => 'simm7<6>', range => (-1024, 1008)) ;
DEF immx4    => IMM('imm', 'imm7', scale => 4, signpos => 'imm7<6>', range => (-256, 252))      ;
DEF immx8    => IMM('imm', 'imm7', scale => 8, signpos => 'imm7<6>', range => (-512, 504))      ;

T[' LDP   REG:w:i32=$St REG:w:i32=$St2 MEM:PST:r:f32={BASE=$XnSP SZ=2x32 IMMOFF:s16=$immx4}      ', 'NONE ', 'LDP_S_ldstpair_post    ', '                     '];
T[' LDP   REG:w:i64=$Dt REG:w:i64=$Dt2 MEM:PST:r:f64={BASE=$XnSP SZ=2x64 IMMOFF:s16=$immx8}      ', 'NONE ', 'LDP_D_ldstpair_post    ', '                     '];
T[' LDP   REG:w:i128=$Qt REG:w:i128=$Qt2 MEM:PST:r:i128={BASE=$XnSP SZ=2x128 IMMOFF:s16=$immx16} ', 'NONE ', 'LDP_Q_ldstpair_post    ', '                     '];
T[' LDP   REG:w=$Wt REG:w=$Wt2 MEM:PST:r:i32={BASE=$XnSP SZ=2x32 IMMOFF:s16=$immx4}              ', 'NONE ', 'LDP_32_ldstpair_post   ', '                     '];
T[' LDP   REG:w=$Xt REG:w=$Xt2 MEM:PST:r:i64={BASE=$XnSP SZ=2x64 IMMOFF:s16=$immx8}              ', 'NONE ', 'LDP_64_ldstpair_post   ', '                     '];
T[' LDPSW REG:w:s64=$Xt REG:w:s64=$Xt2 MEM:PST:r:s32={BASE=$XnSP SZ=2x32 IMMOFF:s16=$immx4}      ', 'NONE ', 'LDPSW_64_ldstpair_post ', '                     '];
T[' STGP  REG:r=$Xt3 REG:r=$Xt4 MEM:PST:w:i64={BASE=$XnSP2 SZ=2x64 IMMOFF:s16=$immx16v2}         ', 'NONE ', 'STGP_64_ldstpair_post  ', 'arch_variant=ARMv8v5 '];
T[' STP   REG:r:i32=$St REG:r:i32=$St2 MEM:PST:w:f32={BASE=$XnSP SZ=2x32 IMMOFF:s16=$immx4}      ', 'NONE ', 'STP_S_ldstpair_post    ', '                     '];
T[' STP   REG:r:i64=$Dt REG:r:i64=$Dt2 MEM:PST:w:f64={BASE=$XnSP SZ=2x64 IMMOFF:s16=$immx8}      ', 'NONE ', 'STP_D_ldstpair_post    ', '                     '];
T[' STP   REG:r:i128=$Qt REG:r:i128=$Qt2 MEM:PST:w:i128={BASE=$XnSP SZ=2x128 IMMOFF:s16=$immx16} ', 'NONE ', 'STP_Q_ldstpair_post    ', '                     '];
T[' STP   REG:r=$Wt REG:r=$Wt2 MEM:PST:w:i32={BASE=$XnSP SZ=2x32 IMMOFF:s16=$immx4}              ', 'NONE ', 'STP_32_ldstpair_post   ', '                     '];
T[' STP   REG:r=$Xt REG:r=$Xt2 MEM:PST:w:i64={BASE=$XnSP SZ=2x64 IMMOFF:s16=$immx8}              ', 'NONE ', 'STP_64_ldstpair_post   ', '                     '];



# ldstpair_pre: Load/store register pair (pre-indexed).
# A64: opc:u=0bxx ig0=0b101 V:u=0bx ig1=0b011 L:u=0bx imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'ldstpair_pre', 'A64');

DEF Dt       => SIMD64('Dt1', 'Rt')                                                             ;
DEF Dt2      => SIMD64('Dt2', 'Rt2')                                                            ;
DEF Qt       => SIMD128('Qt1', 'Rt')                                                            ;
DEF Qt2      => SIMD128('Qt2', 'Rt2')                                                           ;
DEF St       => SIMD32('St1', 'Rt')                                                             ;
DEF St2      => SIMD32('St2', 'Rt2')                                                            ;
DEF Wt       => GPR32('Wt1', 'Rt')                                                              ;
DEF Wt2      => GPR32('Wt2', 'Rt2')                                                             ;
DEF XnSP     => GPR64('Xn', 'Rn', 'read|SP')                                                    ;
DEF XnSP2    => GPR64('Xn', 'Xn', 'read|SP')                                                    ;
DEF Xt       => GPR64('Xt1', 'Rt')                                                              ;
DEF Xt2      => GPR64('Xt2', 'Rt2')                                                             ;
DEF Xt3      => GPR64('Xt1', 'Xt')                                                              ;
DEF Xt4      => GPR64('Xt2', 'Xt2')                                                             ;
DEF immx16   => IMM('imm', 'imm7', scale => 16, signpos => 'imm7<6>', range => (-1024, 1008))   ;
DEF immx16v2 => IMM('imm', 'simm7', scale => 16, signpos => 'simm7<6>', range => (-1024, 1008)) ;
DEF immx4    => IMM('imm', 'imm7', scale => 4, signpos => 'imm7<6>', range => (-256, 252))      ;
DEF immx8    => IMM('imm', 'imm7', scale => 8, signpos => 'imm7<6>', range => (-512, 504))      ;

T[' LDP   REG:w:i32=$St REG:w:i32=$St2 MEM:PRE:r:f32={BASE=$XnSP SZ=2x32 IMMOFF:s16=$immx4}      ', 'NONE ', 'LDP_S_ldstpair_pre    ', '                     '];
T[' LDP   REG:w:i64=$Dt REG:w:i64=$Dt2 MEM:PRE:r:f64={BASE=$XnSP SZ=2x64 IMMOFF:s16=$immx8}      ', 'NONE ', 'LDP_D_ldstpair_pre    ', '                     '];
T[' LDP   REG:w:i128=$Qt REG:w:i128=$Qt2 MEM:PRE:r:i128={BASE=$XnSP SZ=2x128 IMMOFF:s16=$immx16} ', 'NONE ', 'LDP_Q_ldstpair_pre    ', '                     '];
T[' LDP   REG:w=$Wt REG:w=$Wt2 MEM:PRE:r:i32={BASE=$XnSP SZ=2x32 IMMOFF:s16=$immx4}              ', 'NONE ', 'LDP_32_ldstpair_pre   ', '                     '];
T[' LDP   REG:w=$Xt REG:w=$Xt2 MEM:PRE:r:i64={BASE=$XnSP SZ=2x64 IMMOFF:s16=$immx8}              ', 'NONE ', 'LDP_64_ldstpair_pre   ', '                     '];
T[' LDPSW REG:w:s64=$Xt REG:w:s64=$Xt2 MEM:PRE:r:s32={BASE=$XnSP SZ=2x32 IMMOFF:s16=$immx4}      ', 'NONE ', 'LDPSW_64_ldstpair_pre ', '                     '];
T[' STGP  REG:r=$Xt3 REG:r=$Xt4 MEM:PRE:w:i64={BASE=$XnSP2 SZ=2x64 IMMOFF:s16=$immx16v2}         ', 'NONE ', 'STGP_64_ldstpair_pre  ', 'arch_variant=ARMv8v5 '];
T[' STP   REG:r:i32=$St REG:r:i32=$St2 MEM:PRE:w:f32={BASE=$XnSP SZ=2x32 IMMOFF:s16=$immx4}      ', 'NONE ', 'STP_S_ldstpair_pre    ', '                     '];
T[' STP   REG:r:i64=$Dt REG:r:i64=$Dt2 MEM:PRE:w:f64={BASE=$XnSP SZ=2x64 IMMOFF:s16=$immx8}      ', 'NONE ', 'STP_D_ldstpair_pre    ', '                     '];
T[' STP   REG:r:i128=$Qt REG:r:i128=$Qt2 MEM:PRE:w:i128={BASE=$XnSP SZ=2x128 IMMOFF:s16=$immx16} ', 'NONE ', 'STP_Q_ldstpair_pre    ', '                     '];
T[' STP   REG:r=$Wt REG:r=$Wt2 MEM:PRE:w:i32={BASE=$XnSP SZ=2x32 IMMOFF:s16=$immx4}              ', 'NONE ', 'STP_32_ldstpair_pre   ', '                     '];
T[' STP   REG:r=$Xt REG:r=$Xt2 MEM:PRE:w:i64={BASE=$XnSP SZ=2x64 IMMOFF:s16=$immx8}              ', 'NONE ', 'STP_64_ldstpair_pre   ', '                     '];



# loadlit: Load register (literal).
# A64: opc:u=0bxx ig0=0b011 V:u=0bx ig1=0b00 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'loadlit', 'A64');

DEF Dt      => SIMD64('Dt', 'Rt')                                                                        ;
DEF Qt      => SIMD128('Qt', 'Rt')                                                                       ;
DEF St      => SIMD32('St', 'Rt')                                                                        ;
DEF Wt      => GPR32('Wt', 'Rt')                                                                         ;
DEF Xt      => GPR64('Xt', 'Rt')                                                                         ;
DEF labelx4 => LABEL('label', 'imm19', scale => 4, signpos => 'imm19<18>', range => (-1000000, 1000000)) ;
DEF prfop   => PRF_OP('prfop', 'Rt')                                                                     ;

T[' LDR   REG:w:i32=$St LABEL:s64=$labelx4  ', 'NONE ', 'LDR_S_loadlit    ', 'may_load=1 '];
T[' LDR   REG:w:i64=$Dt LABEL:s64=$labelx4  ', 'NONE ', 'LDR_D_loadlit    ', 'may_load=1 '];
T[' LDR   REG:w:i128=$Qt LABEL:s64=$labelx4 ', 'NONE ', 'LDR_Q_loadlit    ', 'may_load=1 '];
T[' LDR   REG:w=$Wt LABEL:s64=$labelx4      ', 'NONE ', 'LDR_32_loadlit   ', 'may_load=1 '];
T[' LDR   REG:w=$Xt LABEL:s64=$labelx4      ', 'NONE ', 'LDR_64_loadlit   ', 'may_load=1 '];
T[' LDRSW REG:w:s64=$Xt LABEL:s64=$labelx4  ', 'NONE ', 'LDRSW_64_loadlit ', 'may_load=1 '];
T[' PRFM  PRF_OP=$prfop LABEL:s64=$labelx4  ', 'NONE ', 'PRFM_P_loadlit   ', '           '];



1;

