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


# sve_crypto_binary_const: SVE2 crypto constructive binary operations.
# A64: ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b11110 op:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_crypto_binary_const', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' RAX1    VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D} ', 'NONE ', 'rax1_z_zz_    ', 'extensions=SVE2SHA3 '];
T[' SM4EKEY VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S} ', 'NONE ', 'sm4ekey_z_zz_ ', 'extensions=SVE2SM4  '];



# sve_crypto_binary_dest: SVE2 crypto destructive binary operations.
# A64: ig0=0b01000101 size:u=0bxx ig1=0b10001 op:u=0bx ig2=0b11100 o2:u=0bx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_crypto_binary_dest', 'A64');

DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' AESD VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'NONE ', 'aesd_z_zz_ ', 'extensions=SVE2AES '];
T[' AESE VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'NONE ', 'aese_z_zz_ ', 'extensions=SVE2AES '];
T[' SM4E VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'NONE ', 'sm4e_z_zz_ ', 'extensions=SVE2SM4 '];



# sve_crypto_unary: SVE2 crypto unary operations.
# A64: ig0=0b01000101 size:u=0bxx ig1=0b10000011100 op:u=0bx ig2=0b00000 Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_crypto_unary', 'A64');

DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;

T[' AESIMC VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} ', 'NONE ', 'aesimc_z_z_ ', 'extensions=SVE2AES '];
T[' AESMC  VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} ', 'NONE ', 'aesmc_z_z_  ', 'extensions=SVE2AES '];



# sve_fp_2op_i_p_zds: SVE floating-point arithmetic with immediate (predicated).
# A64: ig0=0b01100101 size:u=0bxx ig1=0b011 opc:u=0bxxx ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_fp_2op_i_p_zds', 'A64');

DEF Pg     => PRDREG('Pg', 'Pg', 'read')                  ;
DEF Zdn    => SVEREG('Zdn', 'Zdn', 'read|write')          ;
DEF const  => FPIMM('const', 'i1', table => 'tbl_fpimm0') ;
DEF const2 => FPIMM('const', 'i1', table => 'tbl_fpimm1') ;
DEF const3 => FPIMM('const', 'i1', table => 'tbl_fpimm2') ;

T[' FADD   VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} FPIMM:f16=$const  ', 'size=0b01 ', 'fadd_z_p_zs_   ', 'extensions=SVE '];
T[' FADD   VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} FPIMM:f32=$const  ', 'size=0b10 ', 'fadd_z_p_zs_   ', 'extensions=SVE '];
T[' FADD   VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} FPIMM:f64=$const  ', 'size=0b11 ', 'fadd_z_p_zs_   ', 'extensions=SVE '];
T[' FMAX   VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} FPIMM:f16=$const2 ', 'size=0b01 ', 'fmax_z_p_zs_   ', 'extensions=SVE '];
T[' FMAX   VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} FPIMM:f32=$const2 ', 'size=0b10 ', 'fmax_z_p_zs_   ', 'extensions=SVE '];
T[' FMAX   VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} FPIMM:f64=$const2 ', 'size=0b11 ', 'fmax_z_p_zs_   ', 'extensions=SVE '];
T[' FMAXNM VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} FPIMM:f16=$const2 ', 'size=0b01 ', 'fmaxnm_z_p_zs_ ', 'extensions=SVE '];
T[' FMAXNM VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} FPIMM:f32=$const2 ', 'size=0b10 ', 'fmaxnm_z_p_zs_ ', 'extensions=SVE '];
T[' FMAXNM VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} FPIMM:f64=$const2 ', 'size=0b11 ', 'fmaxnm_z_p_zs_ ', 'extensions=SVE '];
T[' FMIN   VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} FPIMM:f16=$const2 ', 'size=0b01 ', 'fmin_z_p_zs_   ', 'extensions=SVE '];
T[' FMIN   VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} FPIMM:f32=$const2 ', 'size=0b10 ', 'fmin_z_p_zs_   ', 'extensions=SVE '];
T[' FMIN   VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} FPIMM:f64=$const2 ', 'size=0b11 ', 'fmin_z_p_zs_   ', 'extensions=SVE '];
T[' FMINNM VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} FPIMM:f16=$const2 ', 'size=0b01 ', 'fminnm_z_p_zs_ ', 'extensions=SVE '];
T[' FMINNM VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} FPIMM:f32=$const2 ', 'size=0b10 ', 'fminnm_z_p_zs_ ', 'extensions=SVE '];
T[' FMINNM VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} FPIMM:f64=$const2 ', 'size=0b11 ', 'fminnm_z_p_zs_ ', 'extensions=SVE '];
T[' FMUL   VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} FPIMM:f16=$const3 ', 'size=0b01 ', 'fmul_z_p_zs_   ', 'extensions=SVE '];
T[' FMUL   VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} FPIMM:f32=$const3 ', 'size=0b10 ', 'fmul_z_p_zs_   ', 'extensions=SVE '];
T[' FMUL   VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} FPIMM:f64=$const3 ', 'size=0b11 ', 'fmul_z_p_zs_   ', 'extensions=SVE '];
T[' FSUB   VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} FPIMM:f16=$const  ', 'size=0b01 ', 'fsub_z_p_zs_   ', 'extensions=SVE '];
T[' FSUB   VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} FPIMM:f32=$const  ', 'size=0b10 ', 'fsub_z_p_zs_   ', 'extensions=SVE '];
T[' FSUB   VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} FPIMM:f64=$const  ', 'size=0b11 ', 'fsub_z_p_zs_   ', 'extensions=SVE '];
T[' FSUBR  VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} FPIMM:f16=$const  ', 'size=0b01 ', 'fsubr_z_p_zs_  ', 'extensions=SVE '];
T[' FSUBR  VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} FPIMM:f32=$const  ', 'size=0b10 ', 'fsubr_z_p_zs_  ', 'extensions=SVE '];
T[' FSUBR  VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} FPIMM:f64=$const  ', 'size=0b11 ', 'fsubr_z_p_zs_  ', 'extensions=SVE '];



# sve_fp_2op_p_pd: SVE floating-point compare with zero.
# A64: ig0=0b01100101 size:u=0bxx ig1=0b0100 eq:u=0bx lt:u=0bx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx ne:u=0bx Pd:u=0bxxxx
HINT('ICLASS', 'sve_fp_2op_p_pd', 'A64');

DEF Pd => PRDREG('Pd', 'Pd', 'write') ;
DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' FCMEQ VREG={REG:f16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:f16=$Zn VDS=H} FPIMM:f32=0.0 ', 'size=0b01 ', 'fcmeq_p_p_z0_ ', 'extensions=SVE '];
T[' FCMEQ VREG={REG:f32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:f32=$Zn VDS=S} FPIMM:f32=0.0 ', 'size=0b10 ', 'fcmeq_p_p_z0_ ', 'extensions=SVE '];
T[' FCMEQ VREG={REG:f64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:f64=$Zn VDS=D} FPIMM:f32=0.0 ', 'size=0b11 ', 'fcmeq_p_p_z0_ ', 'extensions=SVE '];
T[' FCMGT VREG={REG:f16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:f16=$Zn VDS=H} FPIMM:f32=0.0 ', 'size=0b01 ', 'fcmgt_p_p_z0_ ', 'extensions=SVE '];
T[' FCMGT VREG={REG:f32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:f32=$Zn VDS=S} FPIMM:f32=0.0 ', 'size=0b10 ', 'fcmgt_p_p_z0_ ', 'extensions=SVE '];
T[' FCMGT VREG={REG:f64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:f64=$Zn VDS=D} FPIMM:f32=0.0 ', 'size=0b11 ', 'fcmgt_p_p_z0_ ', 'extensions=SVE '];
T[' FCMGE VREG={REG:f16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:f16=$Zn VDS=H} FPIMM:f32=0.0 ', 'size=0b01 ', 'fcmge_p_p_z0_ ', 'extensions=SVE '];
T[' FCMGE VREG={REG:f32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:f32=$Zn VDS=S} FPIMM:f32=0.0 ', 'size=0b10 ', 'fcmge_p_p_z0_ ', 'extensions=SVE '];
T[' FCMGE VREG={REG:f64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:f64=$Zn VDS=D} FPIMM:f32=0.0 ', 'size=0b11 ', 'fcmge_p_p_z0_ ', 'extensions=SVE '];
T[' FCMLT VREG={REG:f16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:f16=$Zn VDS=H} FPIMM:f32=0.0 ', 'size=0b01 ', 'fcmlt_p_p_z0_ ', 'extensions=SVE '];
T[' FCMLT VREG={REG:f32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:f32=$Zn VDS=S} FPIMM:f32=0.0 ', 'size=0b10 ', 'fcmlt_p_p_z0_ ', 'extensions=SVE '];
T[' FCMLT VREG={REG:f64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:f64=$Zn VDS=D} FPIMM:f32=0.0 ', 'size=0b11 ', 'fcmlt_p_p_z0_ ', 'extensions=SVE '];
T[' FCMLE VREG={REG:f16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:f16=$Zn VDS=H} FPIMM:f32=0.0 ', 'size=0b01 ', 'fcmle_p_p_z0_ ', 'extensions=SVE '];
T[' FCMLE VREG={REG:f32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:f32=$Zn VDS=S} FPIMM:f32=0.0 ', 'size=0b10 ', 'fcmle_p_p_z0_ ', 'extensions=SVE '];
T[' FCMLE VREG={REG:f64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:f64=$Zn VDS=D} FPIMM:f32=0.0 ', 'size=0b11 ', 'fcmle_p_p_z0_ ', 'extensions=SVE '];
T[' FCMNE VREG={REG:f16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:f16=$Zn VDS=H} FPIMM:f32=0.0 ', 'size=0b01 ', 'fcmne_p_p_z0_ ', 'extensions=SVE '];
T[' FCMNE VREG={REG:f32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:f32=$Zn VDS=S} FPIMM:f32=0.0 ', 'size=0b10 ', 'fcmne_p_p_z0_ ', 'extensions=SVE '];
T[' FCMNE VREG={REG:f64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:f64=$Zn VDS=D} FPIMM:f32=0.0 ', 'size=0b11 ', 'fcmne_p_p_z0_ ', 'extensions=SVE '];



# sve_fp_2op_p_vd: SVE floating-point serial reduction (predicated).
# A64: ig0=0b01100101 size:u=0bxx ig1=0b011 opc:u=0bxxx ig2=0b001 Pg:u=0bxxx Zm:u=0bxxxxx Vdn:u=0bxxxxx
HINT('ICLASS', 'sve_fp_2op_p_vd', 'A64');

DEF Ddn => SIMD64('Ddn', 'Vdn', 'read|write') ;
DEF Hdn => SIMD16('Hdn', 'Vdn', 'read|write') ;
DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Sdn => SIMD32('Sdn', 'Vdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' FADDA REG:f16=$Hdn REG=$Pg REG:f16=$Hdn VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fadda_v_p_z_ ', 'extensions=SVE '];
T[' FADDA REG:f32=$Sdn REG=$Pg REG:f32=$Sdn VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fadda_v_p_z_ ', 'extensions=SVE '];
T[' FADDA REG:f64=$Ddn REG=$Pg REG:f64=$Ddn VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fadda_v_p_z_ ', 'extensions=SVE '];



# sve_fp_2op_p_zd_a: SVE floating-point round to integral value.
# A64: ig0=0b01100101 size:u=0bxx ig1=0b000 opc:u=0bxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_fp_2op_p_zd_a', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' FRINTI VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'frinti_z_p_z_ ', 'extensions=SVE '];
T[' FRINTI VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'frinti_z_p_z_ ', 'extensions=SVE '];
T[' FRINTI VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'frinti_z_p_z_ ', 'extensions=SVE '];
T[' FRINTX VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'frintx_z_p_z_ ', 'extensions=SVE '];
T[' FRINTX VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'frintx_z_p_z_ ', 'extensions=SVE '];
T[' FRINTX VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'frintx_z_p_z_ ', 'extensions=SVE '];
T[' FRINTA VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'frinta_z_p_z_ ', 'extensions=SVE '];
T[' FRINTA VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'frinta_z_p_z_ ', 'extensions=SVE '];
T[' FRINTA VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'frinta_z_p_z_ ', 'extensions=SVE '];
T[' FRINTN VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'frintn_z_p_z_ ', 'extensions=SVE '];
T[' FRINTN VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'frintn_z_p_z_ ', 'extensions=SVE '];
T[' FRINTN VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'frintn_z_p_z_ ', 'extensions=SVE '];
T[' FRINTZ VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'frintz_z_p_z_ ', 'extensions=SVE '];
T[' FRINTZ VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'frintz_z_p_z_ ', 'extensions=SVE '];
T[' FRINTZ VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'frintz_z_p_z_ ', 'extensions=SVE '];
T[' FRINTM VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'frintm_z_p_z_ ', 'extensions=SVE '];
T[' FRINTM VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'frintm_z_p_z_ ', 'extensions=SVE '];
T[' FRINTM VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'frintm_z_p_z_ ', 'extensions=SVE '];
T[' FRINTP VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'frintp_z_p_z_ ', 'extensions=SVE '];
T[' FRINTP VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'frintp_z_p_z_ ', 'extensions=SVE '];
T[' FRINTP VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'frintp_z_p_z_ ', 'extensions=SVE '];



# sve_fp_2op_p_zd_b_0: SVE floating-point convert precision.
# A64: ig0=0b01100101 opc:u=0bxx ig1=0b0010 opc2:u=0bxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_fp_2op_p_zd_b_0', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' BFCVT VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg VREG={REG:i32=$Zn VDS=S} ', 'NONE ', 'bfcvt_z_p_z_s2bf ', 'arch_variant=ARMv8v2 extensions=BF16|SVE '];
T[' FCVT  VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'NONE ', 'fcvt_z_p_z_h2s   ', 'extensions=SVE                           '];
T[' FCVT  VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'NONE ', 'fcvt_z_p_z_h2d   ', 'extensions=SVE                           '];
T[' FCVT  VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'NONE ', 'fcvt_z_p_z_s2h   ', 'extensions=SVE                           '];
T[' FCVT  VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'NONE ', 'fcvt_z_p_z_s2d   ', 'extensions=SVE                           '];
T[' FCVT  VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'NONE ', 'fcvt_z_p_z_d2h   ', 'extensions=SVE                           '];
T[' FCVT  VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'NONE ', 'fcvt_z_p_z_d2s   ', 'extensions=SVE                           '];
T[' FCVTX VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'NONE ', 'fcvtx_z_p_z_d2s  ', 'extensions=SVE2                          '];



# sve_fp_2op_p_zd_b_1: SVE floating-point unary operations.
# A64: ig0=0b01100101 size:u=0bxx ig1=0b0011 opc:u=0bxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_fp_2op_p_zd_b_1', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' FRECPX VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'frecpx_z_p_z_ ', 'extensions=SVE '];
T[' FRECPX VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'frecpx_z_p_z_ ', 'extensions=SVE '];
T[' FRECPX VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'frecpx_z_p_z_ ', 'extensions=SVE '];
T[' FSQRT  VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'fsqrt_z_p_z_  ', 'extensions=SVE '];
T[' FSQRT  VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'fsqrt_z_p_z_  ', 'extensions=SVE '];
T[' FSQRT  VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'fsqrt_z_p_z_  ', 'extensions=SVE '];



# sve_fp_2op_p_zd_c: SVE integer convert to floating-point.
# A64: ig0=0b01100101 opc:u=0bxx ig1=0b010 opc2:u=0bxx U:u=0bx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_fp_2op_p_zd_c', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' SCVTF VREG={REG:s16=$Zd VDS=H} PREG:M=$Pg VREG={REG:s16=$Zn VDS=H} ', 'NONE ', 'scvtf_z_p_z_h2fp16 ', 'extensions=SVE '];
T[' SCVTF VREG={REG:s16=$Zd VDS=H} PREG:M=$Pg VREG={REG:s32=$Zn VDS=S} ', 'NONE ', 'scvtf_z_p_z_w2fp16 ', 'extensions=SVE '];
T[' SCVTF VREG={REG:s32=$Zd VDS=S} PREG:M=$Pg VREG={REG:s32=$Zn VDS=S} ', 'NONE ', 'scvtf_z_p_z_w2s    ', 'extensions=SVE '];
T[' SCVTF VREG={REG:s64=$Zd VDS=D} PREG:M=$Pg VREG={REG:s32=$Zn VDS=S} ', 'NONE ', 'scvtf_z_p_z_w2d    ', 'extensions=SVE '];
T[' SCVTF VREG={REG:s16=$Zd VDS=H} PREG:M=$Pg VREG={REG:s64=$Zn VDS=D} ', 'NONE ', 'scvtf_z_p_z_x2fp16 ', 'extensions=SVE '];
T[' SCVTF VREG={REG:s32=$Zd VDS=S} PREG:M=$Pg VREG={REG:s64=$Zn VDS=D} ', 'NONE ', 'scvtf_z_p_z_x2s    ', 'extensions=SVE '];
T[' SCVTF VREG={REG:s64=$Zd VDS=D} PREG:M=$Pg VREG={REG:s64=$Zn VDS=D} ', 'NONE ', 'scvtf_z_p_z_x2d    ', 'extensions=SVE '];
T[' UCVTF VREG={REG:u16=$Zd VDS=H} PREG:M=$Pg VREG={REG:u16=$Zn VDS=H} ', 'NONE ', 'ucvtf_z_p_z_h2fp16 ', 'extensions=SVE '];
T[' UCVTF VREG={REG:u16=$Zd VDS=H} PREG:M=$Pg VREG={REG:u32=$Zn VDS=S} ', 'NONE ', 'ucvtf_z_p_z_w2fp16 ', 'extensions=SVE '];
T[' UCVTF VREG={REG:u32=$Zd VDS=S} PREG:M=$Pg VREG={REG:u32=$Zn VDS=S} ', 'NONE ', 'ucvtf_z_p_z_w2s    ', 'extensions=SVE '];
T[' UCVTF VREG={REG:u64=$Zd VDS=D} PREG:M=$Pg VREG={REG:u32=$Zn VDS=S} ', 'NONE ', 'ucvtf_z_p_z_w2d    ', 'extensions=SVE '];
T[' UCVTF VREG={REG:u16=$Zd VDS=H} PREG:M=$Pg VREG={REG:u64=$Zn VDS=D} ', 'NONE ', 'ucvtf_z_p_z_x2fp16 ', 'extensions=SVE '];
T[' UCVTF VREG={REG:u32=$Zd VDS=S} PREG:M=$Pg VREG={REG:u64=$Zn VDS=D} ', 'NONE ', 'ucvtf_z_p_z_x2s    ', 'extensions=SVE '];
T[' UCVTF VREG={REG:u64=$Zd VDS=D} PREG:M=$Pg VREG={REG:u64=$Zn VDS=D} ', 'NONE ', 'ucvtf_z_p_z_x2d    ', 'extensions=SVE '];



# sve_fp_2op_p_zd_d: SVE floating-point convert to integer.
# A64: ig0=0b01100101 opc:u=0bxx ig1=0b011 opc2:u=0bxx U:u=0bx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_fp_2op_p_zd_d', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' FCVTZS VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'NONE      ', 'fcvtzs_z_p_z_fp162h ', 'extensions=SVE  '];
T[' FCVTZS VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'NONE      ', 'fcvtzs_z_p_z_fp162w ', 'extensions=SVE  '];
T[' FCVTZS VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'NONE      ', 'fcvtzs_z_p_z_fp162x ', 'extensions=SVE  '];
T[' FCVTZS VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'NONE      ', 'fcvtzs_z_p_z_s2w    ', 'extensions=SVE  '];
T[' FCVTZS VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'NONE      ', 'fcvtzs_z_p_z_s2x    ', 'extensions=SVE  '];
T[' FCVTZS VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'NONE      ', 'fcvtzs_z_p_z_d2w    ', 'extensions=SVE  '];
T[' FCVTZS VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'NONE      ', 'fcvtzs_z_p_z_d2x    ', 'extensions=SVE  '];
T[' FCVTZU VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'NONE      ', 'fcvtzu_z_p_z_fp162h ', 'extensions=SVE  '];
T[' FCVTZU VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'NONE      ', 'fcvtzu_z_p_z_fp162w ', 'extensions=SVE  '];
T[' FCVTZU VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'NONE      ', 'fcvtzu_z_p_z_fp162x ', 'extensions=SVE  '];
T[' FCVTZU VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'NONE      ', 'fcvtzu_z_p_z_s2w    ', 'extensions=SVE  '];
T[' FCVTZU VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'NONE      ', 'fcvtzu_z_p_z_s2x    ', 'extensions=SVE  '];
T[' FCVTZU VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'NONE      ', 'fcvtzu_z_p_z_d2w    ', 'extensions=SVE  '];
T[' FCVTZU VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'NONE      ', 'fcvtzu_z_p_z_d2x    ', 'extensions=SVE  '];
T[' FLOGB  VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'flogb_z_p_z_        ', 'extensions=SVE2 '];
T[' FLOGB  VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'flogb_z_p_z_        ', 'extensions=SVE2 '];
T[' FLOGB  VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'flogb_z_p_z_        ', 'extensions=SVE2 '];



# sve_fp_2op_p_zds: SVE floating-point arithmetic (predicated).
# A64: ig0=0b01100101 size:u=0bxx ig1=0b00 opc:u=0bxxxx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_fp_2op_p_zds', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' FABD   VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fabd_z_p_zz_   ', 'extensions=SVE '];
T[' FABD   VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fabd_z_p_zz_   ', 'extensions=SVE '];
T[' FABD   VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fabd_z_p_zz_   ', 'extensions=SVE '];
T[' FADD   VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fadd_z_p_zz_   ', 'extensions=SVE '];
T[' FADD   VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fadd_z_p_zz_   ', 'extensions=SVE '];
T[' FADD   VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fadd_z_p_zz_   ', 'extensions=SVE '];
T[' FDIV   VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fdiv_z_p_zz_   ', 'extensions=SVE '];
T[' FDIV   VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fdiv_z_p_zz_   ', 'extensions=SVE '];
T[' FDIV   VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fdiv_z_p_zz_   ', 'extensions=SVE '];
T[' FDIVR  VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fdivr_z_p_zz_  ', 'extensions=SVE '];
T[' FDIVR  VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fdivr_z_p_zz_  ', 'extensions=SVE '];
T[' FDIVR  VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fdivr_z_p_zz_  ', 'extensions=SVE '];
T[' FMAX   VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fmax_z_p_zz_   ', 'extensions=SVE '];
T[' FMAX   VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fmax_z_p_zz_   ', 'extensions=SVE '];
T[' FMAX   VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fmax_z_p_zz_   ', 'extensions=SVE '];
T[' FMAXNM VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fmaxnm_z_p_zz_ ', 'extensions=SVE '];
T[' FMAXNM VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fmaxnm_z_p_zz_ ', 'extensions=SVE '];
T[' FMAXNM VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fmaxnm_z_p_zz_ ', 'extensions=SVE '];
T[' FMIN   VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fmin_z_p_zz_   ', 'extensions=SVE '];
T[' FMIN   VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fmin_z_p_zz_   ', 'extensions=SVE '];
T[' FMIN   VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fmin_z_p_zz_   ', 'extensions=SVE '];
T[' FMINNM VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fminnm_z_p_zz_ ', 'extensions=SVE '];
T[' FMINNM VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fminnm_z_p_zz_ ', 'extensions=SVE '];
T[' FMINNM VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fminnm_z_p_zz_ ', 'extensions=SVE '];
T[' FMUL   VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fmul_z_p_zz_   ', 'extensions=SVE '];
T[' FMUL   VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fmul_z_p_zz_   ', 'extensions=SVE '];
T[' FMUL   VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fmul_z_p_zz_   ', 'extensions=SVE '];
T[' FMULX  VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fmulx_z_p_zz_  ', 'extensions=SVE '];
T[' FMULX  VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fmulx_z_p_zz_  ', 'extensions=SVE '];
T[' FMULX  VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fmulx_z_p_zz_  ', 'extensions=SVE '];
T[' FSCALE VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fscale_z_p_zz_ ', 'extensions=SVE '];
T[' FSCALE VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fscale_z_p_zz_ ', 'extensions=SVE '];
T[' FSCALE VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fscale_z_p_zz_ ', 'extensions=SVE '];
T[' FSUB   VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fsub_z_p_zz_   ', 'extensions=SVE '];
T[' FSUB   VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fsub_z_p_zz_   ', 'extensions=SVE '];
T[' FSUB   VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fsub_z_p_zz_   ', 'extensions=SVE '];
T[' FSUBR  VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fsubr_z_p_zz_  ', 'extensions=SVE '];
T[' FSUBR  VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fsubr_z_p_zz_  ', 'extensions=SVE '];
T[' FSUBR  VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fsubr_z_p_zz_  ', 'extensions=SVE '];



# sve_fp_2op_u_zd: SVE floating-point reciprocal estimate (unpredicated).
# A64: ig0=0b01100101 size:u=0bxx ig1=0b001 opc:u=0bxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_fp_2op_u_zd', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' FRECPE  VREG={REG:f16=$Zd VDS=H} VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'frecpe_z_z_  ', 'extensions=SVE '];
T[' FRECPE  VREG={REG:f32=$Zd VDS=S} VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'frecpe_z_z_  ', 'extensions=SVE '];
T[' FRECPE  VREG={REG:f64=$Zd VDS=D} VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'frecpe_z_z_  ', 'extensions=SVE '];
T[' FRSQRTE VREG={REG:f16=$Zd VDS=H} VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'frsqrte_z_z_ ', 'extensions=SVE '];
T[' FRSQRTE VREG={REG:f32=$Zd VDS=S} VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'frsqrte_z_z_ ', 'extensions=SVE '];
T[' FRSQRTE VREG={REG:f64=$Zd VDS=D} VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'frsqrte_z_z_ ', 'extensions=SVE '];



# sve_fp_3op_p_pd: SVE floating-point compare vectors.
# A64: ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op:u=0bx ig2=0b1 o2:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx o3:u=0bx Pd:u=0bxxxx
HINT('ICLASS', 'sve_fp_3op_p_pd', 'A64');

DEF Pd => PRDREG('Pd', 'Pd', 'write') ;
DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' FACGT VREG={REG:f16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'facgt_p_p_zz_       ', 'extensions=SVE '];
T[' FACGT VREG={REG:f32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'facgt_p_p_zz_       ', 'extensions=SVE '];
T[' FACGT VREG={REG:f64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'facgt_p_p_zz_       ', 'extensions=SVE '];
T[' FACGE VREG={REG:f16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'facge_p_p_zz_       ', 'extensions=SVE '];
T[' FACGE VREG={REG:f32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'facge_p_p_zz_       ', 'extensions=SVE '];
T[' FACGE VREG={REG:f64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'facge_p_p_zz_       ', 'extensions=SVE '];
T[' FCMEQ VREG={REG:f16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fcmeq_p_p_zz_       ', 'extensions=SVE '];
T[' FCMEQ VREG={REG:f32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fcmeq_p_p_zz_       ', 'extensions=SVE '];
T[' FCMEQ VREG={REG:f64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fcmeq_p_p_zz_       ', 'extensions=SVE '];
T[' FCMGT VREG={REG:f16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fcmgt_p_p_zz_       ', 'extensions=SVE '];
T[' FCMGT VREG={REG:f32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fcmgt_p_p_zz_       ', 'extensions=SVE '];
T[' FCMGT VREG={REG:f64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fcmgt_p_p_zz_       ', 'extensions=SVE '];
T[' FCMGE VREG={REG:f16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fcmge_p_p_zz_       ', 'extensions=SVE '];
T[' FCMGE VREG={REG:f32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fcmge_p_p_zz_       ', 'extensions=SVE '];
T[' FCMGE VREG={REG:f64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fcmge_p_p_zz_       ', 'extensions=SVE '];
T[' FCMNE VREG={REG:f16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fcmne_p_p_zz_       ', 'extensions=SVE '];
T[' FCMNE VREG={REG:f32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fcmne_p_p_zz_       ', 'extensions=SVE '];
T[' FCMNE VREG={REG:f64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fcmne_p_p_zz_       ', 'extensions=SVE '];
T[' FCMUO VREG={REG:f16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fcmuo_p_p_zz_       ', 'extensions=SVE '];
T[' FCMUO VREG={REG:f32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fcmuo_p_p_zz_       ', 'extensions=SVE '];
T[' FCMUO VREG={REG:f64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fcmuo_p_p_zz_       ', 'extensions=SVE '];
T[' FACLE VREG={REG:f16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:f16=$Zm VDS=H} VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'FACLE_facge_p_p_zz_ ', 'extensions=SVE '];
T[' FACLE VREG={REG:f32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:f32=$Zm VDS=S} VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'FACLE_facge_p_p_zz_ ', 'extensions=SVE '];
T[' FACLE VREG={REG:f64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:f64=$Zm VDS=D} VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'FACLE_facge_p_p_zz_ ', 'extensions=SVE '];
T[' FACLT VREG={REG:f16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:f16=$Zm VDS=H} VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'FACLT_facgt_p_p_zz_ ', 'extensions=SVE '];
T[' FACLT VREG={REG:f32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:f32=$Zm VDS=S} VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'FACLT_facgt_p_p_zz_ ', 'extensions=SVE '];
T[' FACLT VREG={REG:f64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:f64=$Zm VDS=D} VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'FACLT_facgt_p_p_zz_ ', 'extensions=SVE '];
T[' FCMLE VREG={REG:f16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:f16=$Zm VDS=H} VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'FCMLE_fcmge_p_p_zz_ ', 'extensions=SVE '];
T[' FCMLE VREG={REG:f32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:f32=$Zm VDS=S} VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'FCMLE_fcmge_p_p_zz_ ', 'extensions=SVE '];
T[' FCMLE VREG={REG:f64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:f64=$Zm VDS=D} VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'FCMLE_fcmge_p_p_zz_ ', 'extensions=SVE '];
T[' FCMLT VREG={REG:f16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:f16=$Zm VDS=H} VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'FCMLT_fcmgt_p_p_zz_ ', 'extensions=SVE '];
T[' FCMLT VREG={REG:f32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:f32=$Zm VDS=S} VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'FCMLT_fcmgt_p_p_zz_ ', 'extensions=SVE '];
T[' FCMLT VREG={REG:f64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:f64=$Zm VDS=D} VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'FCMLT_fcmgt_p_p_zz_ ', 'extensions=SVE '];



# sve_fp_3op_p_zds_a: SVE floating-point multiply-accumulate writing addend.
# A64: ig0=0b01100101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 opc:u=0bxx Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_fp_3op_p_zds_a', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' FMLA  VREG={REG:f16=$Zda VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fmla_z_p_zzz_  ', 'extensions=SVE '];
T[' FMLA  VREG={REG:f32=$Zda VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fmla_z_p_zzz_  ', 'extensions=SVE '];
T[' FMLA  VREG={REG:f64=$Zda VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fmla_z_p_zzz_  ', 'extensions=SVE '];
T[' FMLS  VREG={REG:f16=$Zda VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fmls_z_p_zzz_  ', 'extensions=SVE '];
T[' FMLS  VREG={REG:f32=$Zda VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fmls_z_p_zzz_  ', 'extensions=SVE '];
T[' FMLS  VREG={REG:f64=$Zda VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fmls_z_p_zzz_  ', 'extensions=SVE '];
T[' FNMLA VREG={REG:f16=$Zda VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fnmla_z_p_zzz_ ', 'extensions=SVE '];
T[' FNMLA VREG={REG:f32=$Zda VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fnmla_z_p_zzz_ ', 'extensions=SVE '];
T[' FNMLA VREG={REG:f64=$Zda VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fnmla_z_p_zzz_ ', 'extensions=SVE '];
T[' FNMLS VREG={REG:f16=$Zda VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fnmls_z_p_zzz_ ', 'extensions=SVE '];
T[' FNMLS VREG={REG:f32=$Zda VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fnmls_z_p_zzz_ ', 'extensions=SVE '];
T[' FNMLS VREG={REG:f64=$Zda VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fnmls_z_p_zzz_ ', 'extensions=SVE '];



# sve_fp_3op_p_zds_b: SVE floating-point multiply-accumulate writing multiplicand.
# A64: ig0=0b01100101 size:u=0bxx ig1=0b1 Za:u=0bxxxxx ig2=0b1 opc:u=0bxx Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_fp_3op_p_zds_b', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Za  => SVEREG('Za', 'Za', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' FMAD  VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zm VDS=H} VREG={REG:f16=$Za VDS=H} ', 'size=0b01 ', 'fmad_z_p_zzz_  ', 'extensions=SVE '];
T[' FMAD  VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zm VDS=S} VREG={REG:f32=$Za VDS=S} ', 'size=0b10 ', 'fmad_z_p_zzz_  ', 'extensions=SVE '];
T[' FMAD  VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zm VDS=D} VREG={REG:f64=$Za VDS=D} ', 'size=0b11 ', 'fmad_z_p_zzz_  ', 'extensions=SVE '];
T[' FMSB  VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zm VDS=H} VREG={REG:f16=$Za VDS=H} ', 'size=0b01 ', 'fmsb_z_p_zzz_  ', 'extensions=SVE '];
T[' FMSB  VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zm VDS=S} VREG={REG:f32=$Za VDS=S} ', 'size=0b10 ', 'fmsb_z_p_zzz_  ', 'extensions=SVE '];
T[' FMSB  VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zm VDS=D} VREG={REG:f64=$Za VDS=D} ', 'size=0b11 ', 'fmsb_z_p_zzz_  ', 'extensions=SVE '];
T[' FNMAD VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zm VDS=H} VREG={REG:f16=$Za VDS=H} ', 'size=0b01 ', 'fnmad_z_p_zzz_ ', 'extensions=SVE '];
T[' FNMAD VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zm VDS=S} VREG={REG:f32=$Za VDS=S} ', 'size=0b10 ', 'fnmad_z_p_zzz_ ', 'extensions=SVE '];
T[' FNMAD VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zm VDS=D} VREG={REG:f64=$Za VDS=D} ', 'size=0b11 ', 'fnmad_z_p_zzz_ ', 'extensions=SVE '];
T[' FNMSB VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zm VDS=H} VREG={REG:f16=$Za VDS=H} ', 'size=0b01 ', 'fnmsb_z_p_zzz_ ', 'extensions=SVE '];
T[' FNMSB VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zm VDS=S} VREG={REG:f32=$Za VDS=S} ', 'size=0b10 ', 'fnmsb_z_p_zzz_ ', 'extensions=SVE '];
T[' FNMSB VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zm VDS=D} VREG={REG:f64=$Za VDS=D} ', 'size=0b11 ', 'fnmsb_z_p_zzz_ ', 'extensions=SVE '];



# sve_fp_3op_u_zd: SVE floating-point arithmetic (unpredicated).
# A64: ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b000 opc:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_fp_3op_u_zd', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' FADD    VREG={REG:f16=$Zd VDS=H} VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fadd_z_zz_    ', 'extensions=SVE '];
T[' FADD    VREG={REG:f32=$Zd VDS=S} VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fadd_z_zz_    ', 'extensions=SVE '];
T[' FADD    VREG={REG:f64=$Zd VDS=D} VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fadd_z_zz_    ', 'extensions=SVE '];
T[' FMUL    VREG={REG:f16=$Zd VDS=H} VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fmul_z_zz_    ', 'extensions=SVE '];
T[' FMUL    VREG={REG:f32=$Zd VDS=S} VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fmul_z_zz_    ', 'extensions=SVE '];
T[' FMUL    VREG={REG:f64=$Zd VDS=D} VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fmul_z_zz_    ', 'extensions=SVE '];
T[' FRECPS  VREG={REG:f16=$Zd VDS=H} VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'frecps_z_zz_  ', 'extensions=SVE '];
T[' FRECPS  VREG={REG:f32=$Zd VDS=S} VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'frecps_z_zz_  ', 'extensions=SVE '];
T[' FRECPS  VREG={REG:f64=$Zd VDS=D} VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'frecps_z_zz_  ', 'extensions=SVE '];
T[' FRSQRTS VREG={REG:f16=$Zd VDS=H} VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'frsqrts_z_zz_ ', 'extensions=SVE '];
T[' FRSQRTS VREG={REG:f32=$Zd VDS=S} VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'frsqrts_z_zz_ ', 'extensions=SVE '];
T[' FRSQRTS VREG={REG:f64=$Zd VDS=D} VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'frsqrts_z_zz_ ', 'extensions=SVE '];
T[' FSUB    VREG={REG:f16=$Zd VDS=H} VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fsub_z_zz_    ', 'extensions=SVE '];
T[' FSUB    VREG={REG:f32=$Zd VDS=S} VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fsub_z_zz_    ', 'extensions=SVE '];
T[' FSUB    VREG={REG:f64=$Zd VDS=D} VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fsub_z_zz_    ', 'extensions=SVE '];
T[' FTSMUL  VREG={REG:f16=$Zd VDS=H} VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'ftsmul_z_zz_  ', 'extensions=SVE '];
T[' FTSMUL  VREG={REG:f32=$Zd VDS=S} VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'ftsmul_z_zz_  ', 'extensions=SVE '];
T[' FTSMUL  VREG={REG:f64=$Zd VDS=D} VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'ftsmul_z_zz_  ', 'extensions=SVE '];



# sve_fp_fast_red: SVE floating-point recursive reduction.
# A64: ig0=0b01100101 size:u=0bxx ig1=0b000 opc:u=0bxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx
HINT('ICLASS', 'sve_fp_fast_red', 'A64');

DEF Dd => SIMD64('Dd', 'Vd', 'write') ;
DEF Hd => SIMD16('Hd', 'Vd', 'write') ;
DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Sd => SIMD32('Sd', 'Vd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' FADDV   REG:f16=$Hd REG=$Pg VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'faddv_v_p_z_   ', 'extensions=SVE '];
T[' FADDV   REG:f32=$Sd REG=$Pg VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'faddv_v_p_z_   ', 'extensions=SVE '];
T[' FADDV   REG:f64=$Dd REG=$Pg VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'faddv_v_p_z_   ', 'extensions=SVE '];
T[' FMAXNMV REG:f16=$Hd REG=$Pg VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'fmaxnmv_v_p_z_ ', 'extensions=SVE '];
T[' FMAXNMV REG:f32=$Sd REG=$Pg VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'fmaxnmv_v_p_z_ ', 'extensions=SVE '];
T[' FMAXNMV REG:f64=$Dd REG=$Pg VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'fmaxnmv_v_p_z_ ', 'extensions=SVE '];
T[' FMAXV   REG:f16=$Hd REG=$Pg VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'fmaxv_v_p_z_   ', 'extensions=SVE '];
T[' FMAXV   REG:f32=$Sd REG=$Pg VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'fmaxv_v_p_z_   ', 'extensions=SVE '];
T[' FMAXV   REG:f64=$Dd REG=$Pg VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'fmaxv_v_p_z_   ', 'extensions=SVE '];
T[' FMINNMV REG:f16=$Hd REG=$Pg VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'fminnmv_v_p_z_ ', 'extensions=SVE '];
T[' FMINNMV REG:f32=$Sd REG=$Pg VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'fminnmv_v_p_z_ ', 'extensions=SVE '];
T[' FMINNMV REG:f64=$Dd REG=$Pg VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'fminnmv_v_p_z_ ', 'extensions=SVE '];
T[' FMINV   REG:f16=$Hd REG=$Pg VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'fminv_v_p_z_   ', 'extensions=SVE '];
T[' FMINV   REG:f32=$Sd REG=$Pg VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'fminv_v_p_z_   ', 'extensions=SVE '];
T[' FMINV   REG:f64=$Dd REG=$Pg VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'fminv_v_p_z_   ', 'extensions=SVE '];



# sve_fp_fcadd: SVE floating-point complex add (predicated).
# A64: ig0=0b01100100 size:u=0bxx ig1=0b00000 rot:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_fp_fcadd', 'A64');

DEF Pg    => PRDREG('Pg', 'Pg', 'read')               ;
DEF Zdn   => SVEREG('Zdn', 'Zdn', 'read|write')       ;
DEF Zm    => SVEREG('Zm', 'Zm', 'read')               ;
DEF const => IMM('const', 'rot', table => 'tbl_imm4') ;

T[' FCADD VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} IMM:i16=$const ', 'size=0b01 ', 'fcadd_z_p_zz_ ', 'extensions=SVE '];
T[' FCADD VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} IMM:i32=$const ', 'size=0b10 ', 'fcadd_z_p_zz_ ', 'extensions=SVE '];
T[' FCADD VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} IMM:i64=$const ', 'size=0b11 ', 'fcadd_z_p_zz_ ', 'extensions=SVE '];



# sve_fp_fcmla: SVE floating-point complex multiply-add (predicated).
# A64: ig0=0b01100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 rot:u=0bxx Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_fp_fcmla', 'A64');

DEF Pg    => PRDREG('Pg', 'Pg', 'read')               ;
DEF Zda   => SVEREG('Zda', 'Zda', 'read|write')       ;
DEF Zm    => SVEREG('Zm', 'Zm', 'read')               ;
DEF Zn    => SVEREG('Zn', 'Zn', 'read')               ;
DEF const => IMM('const', 'rot', table => 'tbl_imm0') ;

T[' FCMLA VREG={REG:f16=$Zda VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} IMM:i16=$const ', 'size=0b01 ', 'fcmla_z_p_zzz_ ', 'extensions=SVE '];
T[' FCMLA VREG={REG:f32=$Zda VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} IMM:i32=$const ', 'size=0b10 ', 'fcmla_z_p_zzz_ ', 'extensions=SVE '];
T[' FCMLA VREG={REG:f64=$Zda VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} IMM:i64=$const ', 'size=0b11 ', 'fcmla_z_p_zzz_ ', 'extensions=SVE '];



# sve_fp_fcmla_by_indexed_elem: SVE floating-point complex multiply-add (indexed).
# A64: ig0=0b01100100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b0001 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_fp_fcmla_by_indexed_elem', 'A64');

DEF Zda   => SVEREG('Zda', 'Zda', 'read|write')       ;
DEF Zm    => SVEREG('Zm', 'Zm', 'read')               ;
DEF Zn    => SVEREG('Zn', 'Zn', 'read')               ;
DEF const => IMM('const', 'rot', table => 'tbl_imm0') ;

T[' FCMLA VREG={REG:f16=$Zda VDS=H} VREG={REG:f16=$Zn VDS=H} EREG={REG:f16=$Zm VDS=H IDX={ENCODEDIN=i2}} IMM:i16=$const ', 'NONE ', 'fcmla_z_zzzi_h ', 'extensions=SVE '];
T[' FCMLA VREG={REG:f32=$Zda VDS=S} VREG={REG:f32=$Zn VDS=S} EREG={REG:f32=$Zm VDS=S IDX={ENCODEDIN=i1}} IMM:i32=$const ', 'NONE ', 'fcmla_z_zzzi_s ', 'extensions=SVE '];



# sve_fp_fcvt2: SVE floating-point convert precision odd elements.
# A64: ig0=0b01100100 opc:u=0bxx ig1=0b0010 opc2:u=0bxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_fp_fcvt2', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' BFCVTNT VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg VREG={REG:i32=$Zn VDS=S} ', 'NONE ', 'bfcvtnt_z_p_z_s2bf ', 'arch_variant=ARMv8v2 extensions=BF16|SVE '];
T[' FCVTLT  VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H} ', 'NONE ', 'fcvtlt_z_p_z_h2s   ', 'extensions=SVE2                          '];
T[' FCVTLT  VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'NONE ', 'fcvtlt_z_p_z_s2d   ', 'extensions=SVE2                          '];
T[' FCVTNT  VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S} ', 'NONE ', 'fcvtnt_z_p_z_s2h   ', 'extensions=SVE2                          '];
T[' FCVTNT  VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'NONE ', 'fcvtnt_z_p_z_d2s   ', 'extensions=SVE2                          '];
T[' FCVTXNT VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D} ', 'NONE ', 'fcvtxnt_z_p_z_d2s  ', 'extensions=SVE2                          '];



# sve_fp_fdot: SVE BFloat16 floating-point dot product.
# A64: ig0=0b011001000 op:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b100000 Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_fp_fdot', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' BFDOT VREG={REG:i32=$Zda VDS=S} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H} ', 'NONE ', 'bfdot_z_zzz_ ', 'arch_variant=ARMv8v2 extensions=BF16|SVE '];



# sve_fp_fdot_by_indexed_elem: SVE BFloat16 floating-point dot product (indexed).
# A64: ig0=0b011001000 op:u=0bx ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b010000 Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_fp_fdot_by_indexed_elem', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' BFDOT VREG={REG:i32=$Zda VDS=S} VREG={REG:i16=$Zn VDS=H} EREG={REG:i16=$Zm VDS=H IDX={ENCODEDIN=i2}} ', 'NONE ', 'bfdot_z_zzzi_ ', 'arch_variant=ARMv8v2 extensions=BF16|SVE '];



# sve_fp_fma_by_indexed_elem: SVE floating-point multiply-add (indexed).
# A64: ig0=0b01100100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b00000 op:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_fp_fma_by_indexed_elem', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' FMLA VREG={REG:f16=$Zda VDS=H} VREG={REG:f16=$Zn VDS=H} EREG={REG:f16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'fmla_z_zzzi_h ', 'extensions=SVE '];
T[' FMLA VREG={REG:f32=$Zda VDS=S} VREG={REG:f32=$Zn VDS=S} EREG={REG:f32=$Zm VDS=S IDX={ENCODEDIN=i2}}      ', 'NONE ', 'fmla_z_zzzi_s ', 'extensions=SVE '];
T[' FMLA VREG={REG:f64=$Zda VDS=D} VREG={REG:f64=$Zn VDS=D} EREG={REG:f64=$Zm VDS=D IDX={ENCODEDIN=i1}}      ', 'NONE ', 'fmla_z_zzzi_d ', 'extensions=SVE '];
T[' FMLS VREG={REG:f16=$Zda VDS=H} VREG={REG:f16=$Zn VDS=H} EREG={REG:f16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'fmls_z_zzzi_h ', 'extensions=SVE '];
T[' FMLS VREG={REG:f32=$Zda VDS=S} VREG={REG:f32=$Zn VDS=S} EREG={REG:f32=$Zm VDS=S IDX={ENCODEDIN=i2}}      ', 'NONE ', 'fmls_z_zzzi_s ', 'extensions=SVE '];
T[' FMLS VREG={REG:f64=$Zda VDS=D} VREG={REG:f64=$Zn VDS=D} EREG={REG:f64=$Zm VDS=D IDX={ENCODEDIN=i1}}      ', 'NONE ', 'fmls_z_zzzi_d ', 'extensions=SVE '];



# sve_fp_fma_long: SVE floating-point multiply-add long.
# A64: ig0=0b011001001 o2:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b10 op:u=0bx ig3=0b00 T:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_fp_fma_long', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' BFMLALB VREG={REG:i32=$Zda VDS=S} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H} ', 'NONE ', 'bfmlalb_z_zzz_ ', 'arch_variant=ARMv8v2 extensions=BF16|SVE '];
T[' BFMLALT VREG={REG:i32=$Zda VDS=S} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H} ', 'NONE ', 'bfmlalt_z_zzz_ ', 'arch_variant=ARMv8v2 extensions=BF16|SVE '];
T[' FMLALB  VREG={REG:f32=$Zda VDS=S} VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'NONE ', 'fmlalb_z_zzz_  ', 'extensions=SVE2                          '];
T[' FMLALT  VREG={REG:f32=$Zda VDS=S} VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'NONE ', 'fmlalt_z_zzz_  ', 'extensions=SVE2                          '];
T[' FMLSLB  VREG={REG:f32=$Zda VDS=S} VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'NONE ', 'fmlslb_z_zzz_  ', 'extensions=SVE2                          '];
T[' FMLSLT  VREG={REG:f32=$Zda VDS=S} VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'NONE ', 'fmlslt_z_zzz_  ', 'extensions=SVE2                          '];



# sve_fp_fma_long_by_indexed_elem: SVE floating-point multiply-add long (indexed).
# A64: ig0=0b011001001 o2:u=0bx ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b01 op:u=0bx ig3=0b0 i3l:u=0bx T:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_fp_fma_long_by_indexed_elem', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' BFMLALB VREG={REG:i32=$Zda VDS=S} VREG={REG:i16=$Zn VDS=H} EREG={REG:i16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'bfmlalb_z_zzzi_ ', 'arch_variant=ARMv8v2 extensions=BF16|SVE '];
T[' BFMLALT VREG={REG:i32=$Zda VDS=S} VREG={REG:i16=$Zn VDS=H} EREG={REG:i16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'bfmlalt_z_zzzi_ ', 'arch_variant=ARMv8v2 extensions=BF16|SVE '];
T[' FMLALB  VREG={REG:f32=$Zda VDS=S} VREG={REG:f16=$Zn VDS=H} EREG={REG:f16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'fmlalb_z_zzzi_s ', 'extensions=SVE2                          '];
T[' FMLALT  VREG={REG:f32=$Zda VDS=S} VREG={REG:f16=$Zn VDS=H} EREG={REG:f16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'fmlalt_z_zzzi_s ', 'extensions=SVE2                          '];
T[' FMLSLB  VREG={REG:f32=$Zda VDS=S} VREG={REG:f16=$Zn VDS=H} EREG={REG:f16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'fmlslb_z_zzzi_s ', 'extensions=SVE2                          '];
T[' FMLSLT  VREG={REG:f32=$Zda VDS=S} VREG={REG:f16=$Zn VDS=H} EREG={REG:f16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'fmlslt_z_zzzi_s ', 'extensions=SVE2                          '];



# sve_fp_fmmla: SVE floating point matrix multiply accumulate.
# A64: ig0=0b01100100 opc:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b111001 Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_fp_fmmla', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' BFMMLA VREG={REG:i32=$Zda VDS=S} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H} ', 'NONE ', 'bfmmla_z_zzz_ ', 'arch_variant=ARMv8v2 extensions=BF16|SVE '];
T[' FMMLA  VREG={REG:f32=$Zda VDS=S} VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'NONE ', 'fmmla_z_zzz_s ', 'extensions=SVEFP32MATMUL                 '];
T[' FMMLA  VREG={REG:f64=$Zda VDS=D} VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'NONE ', 'fmmla_z_zzz_d ', 'extensions=SVEFP64MATMUL                 '];



# sve_fp_fmul_by_indexed_elem: SVE floating-point multiply (indexed).
# A64: ig0=0b01100100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b001000 Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_fp_fmul_by_indexed_elem', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' FMUL VREG={REG:f16=$Zd VDS=H} VREG={REG:f16=$Zn VDS=H} EREG={REG:f16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'fmul_z_zzi_h ', 'extensions=SVE '];
T[' FMUL VREG={REG:f32=$Zd VDS=S} VREG={REG:f32=$Zn VDS=S} EREG={REG:f32=$Zm VDS=S IDX={ENCODEDIN=i2}}      ', 'NONE ', 'fmul_z_zzi_s ', 'extensions=SVE '];
T[' FMUL VREG={REG:f64=$Zd VDS=D} VREG={REG:f64=$Zn VDS=D} EREG={REG:f64=$Zm VDS=D IDX={ENCODEDIN=i1}}      ', 'NONE ', 'fmul_z_zzi_d ', 'extensions=SVE '];



# sve_fp_ftmad: SVE floating-point trig multiply-add coefficient.
# A64: ig0=0b01100101 size:u=0bxx ig1=0b010 imm3:u=0bxxx ig2=0b100000 Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_fp_ftmad', 'A64');

DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write')  ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')          ;
DEF imm => IMM('imm', 'imm3', range => (0, 7)) ;

T[' FTMAD VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} IMM:u8=$imm ', 'size=0b01 ', 'ftmad_z_zzi_ ', 'extensions=SVE '];
T[' FTMAD VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} IMM:u8=$imm ', 'size=0b10 ', 'ftmad_z_zzi_ ', 'extensions=SVE '];
T[' FTMAD VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} IMM:u8=$imm ', 'size=0b11 ', 'ftmad_z_zzi_ ', 'extensions=SVE '];



# sve_fp_pairwise: SVE2 floating-point pairwise operations.
# A64: ig0=0b01100100 size:u=0bxx ig1=0b010 opc:u=0bxxx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_fp_pairwise', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' FADDP   VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'faddp_z_p_zz_   ', 'extensions=SVE2 '];
T[' FADDP   VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'faddp_z_p_zz_   ', 'extensions=SVE2 '];
T[' FADDP   VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'faddp_z_p_zz_   ', 'extensions=SVE2 '];
T[' FMAXNMP VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fmaxnmp_z_p_zz_ ', 'extensions=SVE2 '];
T[' FMAXNMP VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fmaxnmp_z_p_zz_ ', 'extensions=SVE2 '];
T[' FMAXNMP VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fmaxnmp_z_p_zz_ ', 'extensions=SVE2 '];
T[' FMAXP   VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fmaxp_z_p_zz_   ', 'extensions=SVE2 '];
T[' FMAXP   VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fmaxp_z_p_zz_   ', 'extensions=SVE2 '];
T[' FMAXP   VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fmaxp_z_p_zz_   ', 'extensions=SVE2 '];
T[' FMINNMP VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fminnmp_z_p_zz_ ', 'extensions=SVE2 '];
T[' FMINNMP VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fminnmp_z_p_zz_ ', 'extensions=SVE2 '];
T[' FMINNMP VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fminnmp_z_p_zz_ ', 'extensions=SVE2 '];
T[' FMINP   VREG={REG:f16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:f16=$Zdn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'fminp_z_p_zz_   ', 'extensions=SVE2 '];
T[' FMINP   VREG={REG:f32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:f32=$Zdn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'fminp_z_p_zz_   ', 'extensions=SVE2 '];
T[' FMINP   VREG={REG:f64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:f64=$Zdn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'fminp_z_p_zz_   ', 'extensions=SVE2 '];



# sve_int_arith_imm0: SVE integer add/subtract immediate (unpredicated).
# A64: ig0=0b00100101 size:u=0bxx ig1=0b100 opc:u=0bxxx ig2=0b11 sh:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_arith_imm0', 'A64');

DEF Zdn     => SVEREG('Zdn', 'Zdn', 'read|write')                                     ;
DEF imm     => IMM('imm', 'imm8', range => (0, 255))                                  ;
DEF shifter => SHIFTER('shifter', 'sh', table => 'tbl_shifter1', default => 'LSL #0') ;

T[' ADD   VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter} ', 'size=0b00 ', 'add_z_zi_   ', 'extensions=SVE '];
T[' ADD   VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zdn VDS=H} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b01 ', 'add_z_zi_   ', 'extensions=SVE '];
T[' ADD   VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zdn VDS=S} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b10 ', 'add_z_zi_   ', 'extensions=SVE '];
T[' ADD   VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b11 ', 'add_z_zi_   ', 'extensions=SVE '];
T[' SQADD VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zdn   VDS=B} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter} ', 'size=0b00 ', 'sqadd_z_zi_ ', 'extensions=SVE '];
T[' SQADD VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zdn VDS=H} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b01 ', 'sqadd_z_zi_ ', 'extensions=SVE '];
T[' SQADD VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zdn VDS=S} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b10 ', 'sqadd_z_zi_ ', 'extensions=SVE '];
T[' SQADD VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zdn VDS=D} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b11 ', 'sqadd_z_zi_ ', 'extensions=SVE '];
T[' SQSUB VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zdn   VDS=B} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter} ', 'size=0b00 ', 'sqsub_z_zi_ ', 'extensions=SVE '];
T[' SQSUB VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zdn VDS=H} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b01 ', 'sqsub_z_zi_ ', 'extensions=SVE '];
T[' SQSUB VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zdn VDS=S} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b10 ', 'sqsub_z_zi_ ', 'extensions=SVE '];
T[' SQSUB VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zdn VDS=D} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b11 ', 'sqsub_z_zi_ ', 'extensions=SVE '];
T[' SUB   VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter} ', 'size=0b00 ', 'sub_z_zi_   ', 'extensions=SVE '];
T[' SUB   VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zdn VDS=H} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b01 ', 'sub_z_zi_   ', 'extensions=SVE '];
T[' SUB   VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zdn VDS=S} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b10 ', 'sub_z_zi_   ', 'extensions=SVE '];
T[' SUB   VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b11 ', 'sub_z_zi_   ', 'extensions=SVE '];
T[' SUBR  VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter} ', 'size=0b00 ', 'subr_z_zi_  ', 'extensions=SVE '];
T[' SUBR  VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zdn VDS=H} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b01 ', 'subr_z_zi_  ', 'extensions=SVE '];
T[' SUBR  VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zdn VDS=S} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b10 ', 'subr_z_zi_  ', 'extensions=SVE '];
T[' SUBR  VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b11 ', 'subr_z_zi_  ', 'extensions=SVE '];
T[' UQADD VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zdn   VDS=B} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter} ', 'size=0b00 ', 'uqadd_z_zi_ ', 'extensions=SVE '];
T[' UQADD VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zdn VDS=H} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b01 ', 'uqadd_z_zi_ ', 'extensions=SVE '];
T[' UQADD VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zdn VDS=S} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b10 ', 'uqadd_z_zi_ ', 'extensions=SVE '];
T[' UQADD VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zdn VDS=D} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b11 ', 'uqadd_z_zi_ ', 'extensions=SVE '];
T[' UQSUB VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zdn   VDS=B} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter} ', 'size=0b00 ', 'uqsub_z_zi_ ', 'extensions=SVE '];
T[' UQSUB VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zdn VDS=H} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b01 ', 'uqsub_z_zi_ ', 'extensions=SVE '];
T[' UQSUB VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zdn VDS=S} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b10 ', 'uqsub_z_zi_ ', 'extensions=SVE '];
T[' UQSUB VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zdn VDS=D} IMMSH={IMM:u8=$imm SHIFTER:OPT=$shifter}   ', 'size=0b11 ', 'uqsub_z_zi_ ', 'extensions=SVE '];



# sve_int_arith_imm1: SVE integer min/max immediate (unpredicated).
# A64: ig0=0b00100101 size:u=0bxx ig1=0b101 opc:u=0bxxx ig2=0b11 o2:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_arith_imm1', 'A64');

DEF Zdn  => SVEREG('Zdn', 'Zdn', 'read|write')                             ;
DEF imm  => IMM('imm', 'imm8', signpos => 'imm8<7>', range => (-128, 127)) ;
DEF imm2 => IMM('imm', 'imm8', range => (0, 255))                          ;

T[' SMAX VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zdn   VDS=B} IMM:s8=$imm  ', 'size=0b00 ', 'smax_z_zi_ ', 'extensions=SVE '];
T[' SMAX VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zdn VDS=H} IMM:s8=$imm    ', 'size=0b01 ', 'smax_z_zi_ ', 'extensions=SVE '];
T[' SMAX VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zdn VDS=S} IMM:s8=$imm    ', 'size=0b10 ', 'smax_z_zi_ ', 'extensions=SVE '];
T[' SMAX VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zdn VDS=D} IMM:s8=$imm    ', 'size=0b11 ', 'smax_z_zi_ ', 'extensions=SVE '];
T[' SMIN VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zdn   VDS=B} IMM:s8=$imm  ', 'size=0b00 ', 'smin_z_zi_ ', 'extensions=SVE '];
T[' SMIN VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zdn VDS=H} IMM:s8=$imm    ', 'size=0b01 ', 'smin_z_zi_ ', 'extensions=SVE '];
T[' SMIN VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zdn VDS=S} IMM:s8=$imm    ', 'size=0b10 ', 'smin_z_zi_ ', 'extensions=SVE '];
T[' SMIN VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zdn VDS=D} IMM:s8=$imm    ', 'size=0b11 ', 'smin_z_zi_ ', 'extensions=SVE '];
T[' UMAX VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zdn   VDS=B} IMM:u8=$imm2 ', 'size=0b00 ', 'umax_z_zi_ ', 'extensions=SVE '];
T[' UMAX VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zdn VDS=H} IMM:u8=$imm2   ', 'size=0b01 ', 'umax_z_zi_ ', 'extensions=SVE '];
T[' UMAX VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zdn VDS=S} IMM:u8=$imm2   ', 'size=0b10 ', 'umax_z_zi_ ', 'extensions=SVE '];
T[' UMAX VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zdn VDS=D} IMM:u8=$imm2   ', 'size=0b11 ', 'umax_z_zi_ ', 'extensions=SVE '];
T[' UMIN VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zdn   VDS=B} IMM:u8=$imm2 ', 'size=0b00 ', 'umin_z_zi_ ', 'extensions=SVE '];
T[' UMIN VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zdn VDS=H} IMM:u8=$imm2   ', 'size=0b01 ', 'umin_z_zi_ ', 'extensions=SVE '];
T[' UMIN VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zdn VDS=S} IMM:u8=$imm2   ', 'size=0b10 ', 'umin_z_zi_ ', 'extensions=SVE '];
T[' UMIN VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zdn VDS=D} IMM:u8=$imm2   ', 'size=0b11 ', 'umin_z_zi_ ', 'extensions=SVE '];



# sve_int_arith_imm2: SVE integer multiply immediate (unpredicated).
# A64: ig0=0b00100101 size:u=0bxx ig1=0b110 opc:u=0bxxx ig2=0b11 o2:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_arith_imm2', 'A64');

DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write')                             ;
DEF imm => IMM('imm', 'imm8', signpos => 'imm8<7>', range => (-128, 127)) ;

T[' MUL VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:s8=$imm ', 'size=0b00 ', 'mul_z_zi_ ', 'extensions=SVE '];
T[' MUL VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zdn VDS=H} IMM:s8=$imm   ', 'size=0b01 ', 'mul_z_zi_ ', 'extensions=SVE '];
T[' MUL VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zdn VDS=S} IMM:s8=$imm   ', 'size=0b10 ', 'mul_z_zi_ ', 'extensions=SVE '];
T[' MUL VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} IMM:s8=$imm   ', 'size=0b11 ', 'mul_z_zi_ ', 'extensions=SVE '];



# sve_int_arith_vl: SVE stack frame adjustment.
# A64: ig0=0b000001000 op:u=0bx ig1=0b1 Rn:u=0bxxxxx ig2=0b01010 imm6:u=0bxxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'sve_int_arith_vl', 'A64');

DEF XdSP => GPR64('Xd', 'Rd', 'write|SP')                                ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                                 ;
DEF imm  => IMM('imm', 'imm6', signpos => 'imm6<5>', range => (-32, 31)) ;

T[' ADDPL REG=$XdSP REG=$XnSP IMM:s8=$imm ', 'NONE ', 'addpl_r_ri_ ', 'extensions=SVE '];
T[' ADDVL REG=$XdSP REG=$XnSP IMM:s8=$imm ', 'NONE ', 'addvl_r_ri_ ', 'extensions=SVE '];



# sve_int_bin_cons_arit_0: SVE integer add/subtract vectors (unpredicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b000 opc:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_bin_cons_arit_0', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' ADD   VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'add_z_zz_   ', 'extensions=SVE '];
T[' ADD   VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'add_z_zz_   ', 'extensions=SVE '];
T[' ADD   VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'add_z_zz_   ', 'extensions=SVE '];
T[' ADD   VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'add_z_zz_   ', 'extensions=SVE '];
T[' SQADD VREG={REG:s8=$Zd   VDS=B} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'sqadd_z_zz_ ', 'extensions=SVE '];
T[' SQADD VREG={REG:s16=$Zd VDS=H} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'sqadd_z_zz_ ', 'extensions=SVE '];
T[' SQADD VREG={REG:s32=$Zd VDS=S} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'sqadd_z_zz_ ', 'extensions=SVE '];
T[' SQADD VREG={REG:s64=$Zd VDS=D} VREG={REG:s64=$Zn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'sqadd_z_zz_ ', 'extensions=SVE '];
T[' SQSUB VREG={REG:s8=$Zd   VDS=B} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'sqsub_z_zz_ ', 'extensions=SVE '];
T[' SQSUB VREG={REG:s16=$Zd VDS=H} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'sqsub_z_zz_ ', 'extensions=SVE '];
T[' SQSUB VREG={REG:s32=$Zd VDS=S} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'sqsub_z_zz_ ', 'extensions=SVE '];
T[' SQSUB VREG={REG:s64=$Zd VDS=D} VREG={REG:s64=$Zn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'sqsub_z_zz_ ', 'extensions=SVE '];
T[' SUB   VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'sub_z_zz_   ', 'extensions=SVE '];
T[' SUB   VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'sub_z_zz_   ', 'extensions=SVE '];
T[' SUB   VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'sub_z_zz_   ', 'extensions=SVE '];
T[' SUB   VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'sub_z_zz_   ', 'extensions=SVE '];
T[' UQADD VREG={REG:u8=$Zd   VDS=B} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'uqadd_z_zz_ ', 'extensions=SVE '];
T[' UQADD VREG={REG:u16=$Zd VDS=H} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'uqadd_z_zz_ ', 'extensions=SVE '];
T[' UQADD VREG={REG:u32=$Zd VDS=S} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'uqadd_z_zz_ ', 'extensions=SVE '];
T[' UQADD VREG={REG:u64=$Zd VDS=D} VREG={REG:u64=$Zn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'uqadd_z_zz_ ', 'extensions=SVE '];
T[' UQSUB VREG={REG:u8=$Zd   VDS=B} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'uqsub_z_zz_ ', 'extensions=SVE '];
T[' UQSUB VREG={REG:u16=$Zd VDS=H} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'uqsub_z_zz_ ', 'extensions=SVE '];
T[' UQSUB VREG={REG:u32=$Zd VDS=S} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'uqsub_z_zz_ ', 'extensions=SVE '];
T[' UQSUB VREG={REG:u64=$Zd VDS=D} VREG={REG:u64=$Zn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'uqsub_z_zz_ ', 'extensions=SVE '];



# sve_int_bin_cons_log: SVE bitwise logical operations (unpredicated).
# A64: ig0=0b00000100 opc:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_bin_cons_log', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' AND VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D} ', 'NONE ', 'and_z_zz_     ', 'extensions=SVE '];
T[' BIC VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D} ', 'NONE ', 'bic_z_zz_     ', 'extensions=SVE '];
T[' EOR VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D} ', 'NONE ', 'eor_z_zz_     ', 'extensions=SVE '];
T[' ORR VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D} ', 'NONE ', 'orr_z_zz_     ', 'extensions=SVE '];
T[' MOV VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D}                          ', 'NONE ', 'MOV_orr_z_zz_ ', 'extensions=SVE '];



# sve_int_bin_cons_misc_0_a: SVE address generation.
# A64: ig0=0b00000100 opc:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b1010 msz:u=0bxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_bin_cons_misc_0_a', 'A64');

DEF Zd      => SVEREG('Zd', 'Zd', 'write')                  ;
DEF Zm      => SVEREG('Zm', 'Zm', 'read')                   ;
DEF Zn      => SVEREG('Zn', 'Zn', 'read')                   ;
DEF amount0 => IMM('amount', 'msz', table => 'tbl_imm9')    ;
DEF mod     => EXTEND('mod', 'msz', table => 'tbl_extend4') ;

T[' ADR VREG={REG:i32=$Zd VDS=S} MEM:OFF:r:i32={VBASE={REG=$Zn VDS=S} SZ=v32x32 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER:OPT={EXTEND=$mod AMOUNT:i32=$amount0}}} ', 'sz=0b0 ', 'adr_z_az_sd_same_scaled ', 'extensions=SVE '];
T[' ADR VREG={REG:i64=$Zd VDS=D} MEM:OFF:r:i64={VBASE={REG=$Zn VDS=D} SZ=v64x64 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER:OPT={EXTEND=$mod AMOUNT:i64=$amount0}}} ', 'sz=0b1 ', 'adr_z_az_sd_same_scaled ', 'extensions=SVE '];
T[' ADR VREG={REG:i64=$Zd VDS=D} MEM:OFF:r:i64={VBASE={REG=$Zn VDS=D} SZ=v64x64 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=SXTW AMOUNT:i64=$amount0}}}     ', 'NONE   ', 'adr_z_az_d_s32_scaled   ', 'extensions=SVE '];
T[' ADR VREG={REG:i64=$Zd VDS=D} MEM:OFF:r:i64={VBASE={REG=$Zn VDS=D} SZ=v64x64 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=UXTW AMOUNT:i64=$amount0}}}     ', 'NONE   ', 'adr_z_az_d_u32_scaled   ', 'extensions=SVE '];



# sve_int_bin_cons_misc_0_b: SVE floating-point trig select coefficient.
# A64: ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b10110 op:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_bin_cons_misc_0_b', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' FTSSEL VREG={REG:f16=$Zd VDS=H} VREG={REG:f16=$Zn VDS=H} VREG={REG:f16=$Zm VDS=H} ', 'size=0b01 ', 'ftssel_z_zz_ ', 'extensions=SVE '];
T[' FTSSEL VREG={REG:f32=$Zd VDS=S} VREG={REG:f32=$Zn VDS=S} VREG={REG:f32=$Zm VDS=S} ', 'size=0b10 ', 'ftssel_z_zz_ ', 'extensions=SVE '];
T[' FTSSEL VREG={REG:f64=$Zd VDS=D} VREG={REG:f64=$Zn VDS=D} VREG={REG:f64=$Zm VDS=D} ', 'size=0b11 ', 'ftssel_z_zz_ ', 'extensions=SVE '];



# sve_int_bin_cons_misc_0_c: SVE floating-point exponential accelerator.
# A64: ig0=0b00000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b101110 Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_bin_cons_misc_0_c', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' FEXPA VREG={REG:f16=$Zd VDS=H} VREG={REG:f16=$Zn VDS=H} ', 'size=0b01 ', 'fexpa_z_z_ ', 'extensions=SVE '];
T[' FEXPA VREG={REG:f32=$Zd VDS=S} VREG={REG:f32=$Zn VDS=S} ', 'size=0b10 ', 'fexpa_z_z_ ', 'extensions=SVE '];
T[' FEXPA VREG={REG:f64=$Zd VDS=D} VREG={REG:f64=$Zn VDS=D} ', 'size=0b11 ', 'fexpa_z_z_ ', 'extensions=SVE '];



# sve_int_bin_cons_misc_0_d: SVE constructive prefix (unpredicated).
# A64: ig0=0b00000100 opc:u=0bxx ig1=0b1 opc2:u=0bxxxxx ig2=0b101111 Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_bin_cons_misc_0_d', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' MOVPRFX REG=$Zd REG=$Zn ', 'NONE ', 'movprfx_z_z_ ', 'extensions=SVE '];



# sve_int_bin_cons_shift_a: SVE bitwise shift by wide elements (unpredicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b1000 opc:u=0bxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_bin_cons_shift_a', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' ASR VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i64=$Zm VDS=D} ', 'size=0b00 ', 'asr_z_zw_ ', 'extensions=SVE '];
T[' ASR VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b01 ', 'asr_z_zw_ ', 'extensions=SVE '];
T[' ASR VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b10 ', 'asr_z_zw_ ', 'extensions=SVE '];
T[' LSL VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i64=$Zm VDS=D} ', 'size=0b00 ', 'lsl_z_zw_ ', 'extensions=SVE '];
T[' LSL VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b01 ', 'lsl_z_zw_ ', 'extensions=SVE '];
T[' LSL VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b10 ', 'lsl_z_zw_ ', 'extensions=SVE '];
T[' LSR VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i64=$Zm VDS=D} ', 'size=0b00 ', 'lsr_z_zw_ ', 'extensions=SVE '];
T[' LSR VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b01 ', 'lsr_z_zw_ ', 'extensions=SVE '];
T[' LSR VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b10 ', 'lsr_z_zw_ ', 'extensions=SVE '];



# sve_int_bin_cons_shift_b: SVE bitwise shift by immediate (unpredicated).
# A64: ig0=0b00000100 tszh:u=0bxx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b1001 opc:u=0bxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_bin_cons_shift_b', 'A64');

DEF Zd     => SVEREG('Zd', 'Zd', 'write')                      ;
DEF Zn     => SVEREG('Zn', 'Zn', 'read')                       ;
DEF const  => IMM('const', 'tszh:tszl:imm3', range => (1, 8))  ;
DEF const2 => IMM('const', 'tszh:tszl:imm3', range => (1, 16)) ;
DEF const3 => IMM('const', 'tszh:tszl:imm3', range => (1, 32)) ;
DEF const4 => IMM('const', 'tszh:tszl:imm3', range => (1, 64)) ;
DEF const5 => IMM('const', 'tszh:tszl:imm3', range => (0, 7))  ;
DEF const6 => IMM('const', 'tszh:tszl:imm3', range => (0, 15)) ;
DEF const7 => IMM('const', 'tszh:tszl:imm3', range => (0, 31)) ;
DEF const8 => IMM('const', 'tszh:tszl:imm3', range => (0, 63)) ;

T[' ASR VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} IMM:u8=$const  ', 'tszh=0b00 tszl=0b01 ', 'asr_z_zi_ ', 'extensions=SVE '];
T[' ASR VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} IMM:u8=$const2   ', 'tszh=0b00 tszl=0b1x ', 'asr_z_zi_ ', 'extensions=SVE '];
T[' ASR VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} IMM:u8=$const3   ', 'tszh=0b01           ', 'asr_z_zi_ ', 'extensions=SVE '];
T[' ASR VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} IMM:u8=$const4   ', 'tszh=0b1x           ', 'asr_z_zi_ ', 'extensions=SVE '];
T[' LSL VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} IMM:u8=$const5 ', 'tszh=0b00 tszl=0b01 ', 'lsl_z_zi_ ', 'extensions=SVE '];
T[' LSL VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} IMM:u8=$const6   ', 'tszh=0b00 tszl=0b1x ', 'lsl_z_zi_ ', 'extensions=SVE '];
T[' LSL VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} IMM:u8=$const7   ', 'tszh=0b01           ', 'lsl_z_zi_ ', 'extensions=SVE '];
T[' LSL VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} IMM:u8=$const8   ', 'tszh=0b1x           ', 'lsl_z_zi_ ', 'extensions=SVE '];
T[' LSR VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} IMM:u8=$const  ', 'tszh=0b00 tszl=0b01 ', 'lsr_z_zi_ ', 'extensions=SVE '];
T[' LSR VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} IMM:u8=$const2   ', 'tszh=0b00 tszl=0b1x ', 'lsr_z_zi_ ', 'extensions=SVE '];
T[' LSR VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} IMM:u8=$const3   ', 'tszh=0b01           ', 'lsr_z_zi_ ', 'extensions=SVE '];
T[' LSR VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} IMM:u8=$const4   ', 'tszh=0b1x           ', 'lsr_z_zi_ ', 'extensions=SVE '];



# sve_int_bin_pred_arit_0: SVE integer add/subtract vectors (predicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b000 opc:u=0bxxx ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_bin_pred_arit_0', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' ADD  VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'add_z_p_zz_  ', 'extensions=SVE '];
T[' ADD  VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'add_z_p_zz_  ', 'extensions=SVE '];
T[' ADD  VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'add_z_p_zz_  ', 'extensions=SVE '];
T[' ADD  VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'add_z_p_zz_  ', 'extensions=SVE '];
T[' SUB  VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'sub_z_p_zz_  ', 'extensions=SVE '];
T[' SUB  VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'sub_z_p_zz_  ', 'extensions=SVE '];
T[' SUB  VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'sub_z_p_zz_  ', 'extensions=SVE '];
T[' SUB  VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'sub_z_p_zz_  ', 'extensions=SVE '];
T[' SUBR VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'subr_z_p_zz_ ', 'extensions=SVE '];
T[' SUBR VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'subr_z_p_zz_ ', 'extensions=SVE '];
T[' SUBR VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'subr_z_p_zz_ ', 'extensions=SVE '];
T[' SUBR VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'subr_z_p_zz_ ', 'extensions=SVE '];



# sve_int_bin_pred_arit_1: SVE integer min/max/difference (predicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b001 opc:u=0bxx U:u=0bx ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_bin_pred_arit_1', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' SABD VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'sabd_z_p_zz_ ', 'extensions=SVE '];
T[' SABD VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'sabd_z_p_zz_ ', 'extensions=SVE '];
T[' SABD VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'sabd_z_p_zz_ ', 'extensions=SVE '];
T[' SABD VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'sabd_z_p_zz_ ', 'extensions=SVE '];
T[' SMAX VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'smax_z_p_zz_ ', 'extensions=SVE '];
T[' SMAX VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'smax_z_p_zz_ ', 'extensions=SVE '];
T[' SMAX VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'smax_z_p_zz_ ', 'extensions=SVE '];
T[' SMAX VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'smax_z_p_zz_ ', 'extensions=SVE '];
T[' SMIN VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'smin_z_p_zz_ ', 'extensions=SVE '];
T[' SMIN VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'smin_z_p_zz_ ', 'extensions=SVE '];
T[' SMIN VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'smin_z_p_zz_ ', 'extensions=SVE '];
T[' SMIN VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'smin_z_p_zz_ ', 'extensions=SVE '];
T[' UABD VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'uabd_z_p_zz_ ', 'extensions=SVE '];
T[' UABD VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'uabd_z_p_zz_ ', 'extensions=SVE '];
T[' UABD VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'uabd_z_p_zz_ ', 'extensions=SVE '];
T[' UABD VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'uabd_z_p_zz_ ', 'extensions=SVE '];
T[' UMAX VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'umax_z_p_zz_ ', 'extensions=SVE '];
T[' UMAX VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'umax_z_p_zz_ ', 'extensions=SVE '];
T[' UMAX VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'umax_z_p_zz_ ', 'extensions=SVE '];
T[' UMAX VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'umax_z_p_zz_ ', 'extensions=SVE '];
T[' UMIN VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'umin_z_p_zz_ ', 'extensions=SVE '];
T[' UMIN VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'umin_z_p_zz_ ', 'extensions=SVE '];
T[' UMIN VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'umin_z_p_zz_ ', 'extensions=SVE '];
T[' UMIN VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'umin_z_p_zz_ ', 'extensions=SVE '];



# sve_int_bin_pred_arit_2: SVE integer multiply vectors (predicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b0100 H:u=0bx U:u=0bx ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_bin_pred_arit_2', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' MUL   VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'mul_z_p_zz_   ', 'extensions=SVE '];
T[' MUL   VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'mul_z_p_zz_   ', 'extensions=SVE '];
T[' MUL   VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'mul_z_p_zz_   ', 'extensions=SVE '];
T[' MUL   VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'mul_z_p_zz_   ', 'extensions=SVE '];
T[' SMULH VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'smulh_z_p_zz_ ', 'extensions=SVE '];
T[' SMULH VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'smulh_z_p_zz_ ', 'extensions=SVE '];
T[' SMULH VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'smulh_z_p_zz_ ', 'extensions=SVE '];
T[' SMULH VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'smulh_z_p_zz_ ', 'extensions=SVE '];
T[' UMULH VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'umulh_z_p_zz_ ', 'extensions=SVE '];
T[' UMULH VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'umulh_z_p_zz_ ', 'extensions=SVE '];
T[' UMULH VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'umulh_z_p_zz_ ', 'extensions=SVE '];
T[' UMULH VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'umulh_z_p_zz_ ', 'extensions=SVE '];



# sve_int_bin_pred_div: SVE integer divide vectors (predicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b0101 R:u=0bx U:u=0bx ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_bin_pred_div', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' SDIV  VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S} ', 'size=0bx0 ', 'sdiv_z_p_zz_  ', 'extensions=SVE '];
T[' SDIV  VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D} ', 'size=0bx1 ', 'sdiv_z_p_zz_  ', 'extensions=SVE '];
T[' SDIVR VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S} ', 'size=0bx0 ', 'sdivr_z_p_zz_ ', 'extensions=SVE '];
T[' SDIVR VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D} ', 'size=0bx1 ', 'sdivr_z_p_zz_ ', 'extensions=SVE '];
T[' UDIV  VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S} ', 'size=0bx0 ', 'udiv_z_p_zz_  ', 'extensions=SVE '];
T[' UDIV  VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D} ', 'size=0bx1 ', 'udiv_z_p_zz_  ', 'extensions=SVE '];
T[' UDIVR VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S} ', 'size=0bx0 ', 'udivr_z_p_zz_ ', 'extensions=SVE '];
T[' UDIVR VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D} ', 'size=0bx1 ', 'udivr_z_p_zz_ ', 'extensions=SVE '];



# sve_int_bin_pred_log: SVE bitwise logical operations (predicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b011 opc:u=0bxxx ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_bin_pred_log', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' AND VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'and_z_p_zz_ ', 'extensions=SVE '];
T[' AND VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'and_z_p_zz_ ', 'extensions=SVE '];
T[' AND VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'and_z_p_zz_ ', 'extensions=SVE '];
T[' AND VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'and_z_p_zz_ ', 'extensions=SVE '];
T[' BIC VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'bic_z_p_zz_ ', 'extensions=SVE '];
T[' BIC VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'bic_z_p_zz_ ', 'extensions=SVE '];
T[' BIC VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'bic_z_p_zz_ ', 'extensions=SVE '];
T[' BIC VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'bic_z_p_zz_ ', 'extensions=SVE '];
T[' EOR VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'eor_z_p_zz_ ', 'extensions=SVE '];
T[' EOR VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'eor_z_p_zz_ ', 'extensions=SVE '];
T[' EOR VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'eor_z_p_zz_ ', 'extensions=SVE '];
T[' EOR VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'eor_z_p_zz_ ', 'extensions=SVE '];
T[' ORR VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'orr_z_p_zz_ ', 'extensions=SVE '];
T[' ORR VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'orr_z_p_zz_ ', 'extensions=SVE '];
T[' ORR VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'orr_z_p_zz_ ', 'extensions=SVE '];
T[' ORR VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'orr_z_p_zz_ ', 'extensions=SVE '];



# sve_int_bin_pred_shift_0: SVE bitwise shift by immediate (predicated).
# A64: ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc:u=0bxx L:u=0bx U:u=0bx ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_bin_pred_shift_0', 'A64');

DEF Pg     => PRDREG('Pg', 'Pg', 'read')                       ;
DEF Zdn    => SVEREG('Zdn', 'Zdn', 'read|write')               ;
DEF const  => IMM('const', 'tszh:tszl:imm3', range => (1, 8))  ;
DEF const2 => IMM('const', 'tszh:tszl:imm3', range => (1, 16)) ;
DEF const3 => IMM('const', 'tszh:tszl:imm3', range => (1, 32)) ;
DEF const4 => IMM('const', 'tszh:tszl:imm3', range => (1, 64)) ;
DEF const5 => IMM('const', 'tszh:tszl:imm3', range => (0, 7))  ;
DEF const6 => IMM('const', 'tszh:tszl:imm3', range => (0, 15)) ;
DEF const7 => IMM('const', 'tszh:tszl:imm3', range => (0, 31)) ;
DEF const8 => IMM('const', 'tszh:tszl:imm3', range => (0, 63)) ;

T[' ASR    VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} IMM:u8=$const  ', 'tszh=0b00 tszl=0b01 ', 'asr_z_p_zi_    ', 'extensions=SVE  '];
T[' ASR    VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} IMM:u8=$const2   ', 'tszh=0b00 tszl=0b1x ', 'asr_z_p_zi_    ', 'extensions=SVE  '];
T[' ASR    VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} IMM:u8=$const3   ', 'tszh=0b01           ', 'asr_z_p_zi_    ', 'extensions=SVE  '];
T[' ASR    VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} IMM:u8=$const4   ', 'tszh=0b1x           ', 'asr_z_p_zi_    ', 'extensions=SVE  '];
T[' ASRD   VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} IMM:u8=$const  ', 'tszh=0b00 tszl=0b01 ', 'asrd_z_p_zi_   ', 'extensions=SVE  '];
T[' ASRD   VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} IMM:u8=$const2   ', 'tszh=0b00 tszl=0b1x ', 'asrd_z_p_zi_   ', 'extensions=SVE  '];
T[' ASRD   VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} IMM:u8=$const3   ', 'tszh=0b01           ', 'asrd_z_p_zi_   ', 'extensions=SVE  '];
T[' ASRD   VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} IMM:u8=$const4   ', 'tszh=0b1x           ', 'asrd_z_p_zi_   ', 'extensions=SVE  '];
T[' LSL    VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} IMM:u8=$const5 ', 'tszh=0b00 tszl=0b01 ', 'lsl_z_p_zi_    ', 'extensions=SVE  '];
T[' LSL    VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} IMM:u8=$const6   ', 'tszh=0b00 tszl=0b1x ', 'lsl_z_p_zi_    ', 'extensions=SVE  '];
T[' LSL    VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} IMM:u8=$const7   ', 'tszh=0b01           ', 'lsl_z_p_zi_    ', 'extensions=SVE  '];
T[' LSL    VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} IMM:u8=$const8   ', 'tszh=0b1x           ', 'lsl_z_p_zi_    ', 'extensions=SVE  '];
T[' LSR    VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} IMM:u8=$const  ', 'tszh=0b00 tszl=0b01 ', 'lsr_z_p_zi_    ', 'extensions=SVE  '];
T[' LSR    VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} IMM:u8=$const2   ', 'tszh=0b00 tszl=0b1x ', 'lsr_z_p_zi_    ', 'extensions=SVE  '];
T[' LSR    VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} IMM:u8=$const3   ', 'tszh=0b01           ', 'lsr_z_p_zi_    ', 'extensions=SVE  '];
T[' LSR    VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} IMM:u8=$const4   ', 'tszh=0b1x           ', 'lsr_z_p_zi_    ', 'extensions=SVE  '];
T[' SQSHL  VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} IMM:u8=$const5 ', 'tszh=0b00 tszl=0b01 ', 'sqshl_z_p_zi_  ', 'extensions=SVE2 '];
T[' SQSHL  VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} IMM:u8=$const6   ', 'tszh=0b00 tszl=0b1x ', 'sqshl_z_p_zi_  ', 'extensions=SVE2 '];
T[' SQSHL  VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} IMM:u8=$const7   ', 'tszh=0b01           ', 'sqshl_z_p_zi_  ', 'extensions=SVE2 '];
T[' SQSHL  VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} IMM:u8=$const8   ', 'tszh=0b1x           ', 'sqshl_z_p_zi_  ', 'extensions=SVE2 '];
T[' SQSHLU VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} IMM:u8=$const5 ', 'tszh=0b00 tszl=0b01 ', 'sqshlu_z_p_zi_ ', 'extensions=SVE2 '];
T[' SQSHLU VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} IMM:u8=$const6   ', 'tszh=0b00 tszl=0b1x ', 'sqshlu_z_p_zi_ ', 'extensions=SVE2 '];
T[' SQSHLU VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} IMM:u8=$const7   ', 'tszh=0b01           ', 'sqshlu_z_p_zi_ ', 'extensions=SVE2 '];
T[' SQSHLU VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} IMM:u8=$const8   ', 'tszh=0b1x           ', 'sqshlu_z_p_zi_ ', 'extensions=SVE2 '];
T[' SRSHR  VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} IMM:u8=$const  ', 'tszh=0b00 tszl=0b01 ', 'srshr_z_p_zi_  ', 'extensions=SVE2 '];
T[' SRSHR  VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} IMM:u8=$const2   ', 'tszh=0b00 tszl=0b1x ', 'srshr_z_p_zi_  ', 'extensions=SVE2 '];
T[' SRSHR  VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} IMM:u8=$const3   ', 'tszh=0b01           ', 'srshr_z_p_zi_  ', 'extensions=SVE2 '];
T[' SRSHR  VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} IMM:u8=$const4   ', 'tszh=0b1x           ', 'srshr_z_p_zi_  ', 'extensions=SVE2 '];
T[' UQSHL  VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} IMM:u8=$const5 ', 'tszh=0b00 tszl=0b01 ', 'uqshl_z_p_zi_  ', 'extensions=SVE2 '];
T[' UQSHL  VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} IMM:u8=$const6   ', 'tszh=0b00 tszl=0b1x ', 'uqshl_z_p_zi_  ', 'extensions=SVE2 '];
T[' UQSHL  VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} IMM:u8=$const7   ', 'tszh=0b01           ', 'uqshl_z_p_zi_  ', 'extensions=SVE2 '];
T[' UQSHL  VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} IMM:u8=$const8   ', 'tszh=0b1x           ', 'uqshl_z_p_zi_  ', 'extensions=SVE2 '];
T[' URSHR  VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} IMM:u8=$const  ', 'tszh=0b00 tszl=0b01 ', 'urshr_z_p_zi_  ', 'extensions=SVE2 '];
T[' URSHR  VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} IMM:u8=$const2   ', 'tszh=0b00 tszl=0b1x ', 'urshr_z_p_zi_  ', 'extensions=SVE2 '];
T[' URSHR  VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} IMM:u8=$const3   ', 'tszh=0b01           ', 'urshr_z_p_zi_  ', 'extensions=SVE2 '];
T[' URSHR  VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} IMM:u8=$const4   ', 'tszh=0b1x           ', 'urshr_z_p_zi_  ', 'extensions=SVE2 '];



# sve_int_bin_pred_shift_1: SVE bitwise shift by vector (predicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b010 R:u=0bx L:u=0bx U:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_bin_pred_shift_1', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' ASR  VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'asr_z_p_zz_  ', 'extensions=SVE '];
T[' ASR  VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'asr_z_p_zz_  ', 'extensions=SVE '];
T[' ASR  VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'asr_z_p_zz_  ', 'extensions=SVE '];
T[' ASR  VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'asr_z_p_zz_  ', 'extensions=SVE '];
T[' ASRR VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'asrr_z_p_zz_ ', 'extensions=SVE '];
T[' ASRR VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'asrr_z_p_zz_ ', 'extensions=SVE '];
T[' ASRR VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'asrr_z_p_zz_ ', 'extensions=SVE '];
T[' ASRR VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'asrr_z_p_zz_ ', 'extensions=SVE '];
T[' LSL  VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'lsl_z_p_zz_  ', 'extensions=SVE '];
T[' LSL  VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'lsl_z_p_zz_  ', 'extensions=SVE '];
T[' LSL  VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'lsl_z_p_zz_  ', 'extensions=SVE '];
T[' LSL  VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'lsl_z_p_zz_  ', 'extensions=SVE '];
T[' LSLR VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'lslr_z_p_zz_ ', 'extensions=SVE '];
T[' LSLR VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'lslr_z_p_zz_ ', 'extensions=SVE '];
T[' LSLR VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'lslr_z_p_zz_ ', 'extensions=SVE '];
T[' LSLR VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'lslr_z_p_zz_ ', 'extensions=SVE '];
T[' LSR  VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'lsr_z_p_zz_  ', 'extensions=SVE '];
T[' LSR  VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'lsr_z_p_zz_  ', 'extensions=SVE '];
T[' LSR  VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'lsr_z_p_zz_  ', 'extensions=SVE '];
T[' LSR  VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'lsr_z_p_zz_  ', 'extensions=SVE '];
T[' LSRR VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'lsrr_z_p_zz_ ', 'extensions=SVE '];
T[' LSRR VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'lsrr_z_p_zz_ ', 'extensions=SVE '];
T[' LSRR VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'lsrr_z_p_zz_ ', 'extensions=SVE '];
T[' LSRR VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'lsrr_z_p_zz_ ', 'extensions=SVE '];



# sve_int_bin_pred_shift_2: SVE bitwise shift by wide elements (predicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b011 R:u=0bx L:u=0bx U:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_bin_pred_shift_2', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' ASR VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i64=$Zm VDS=D} ', 'size=0b00 ', 'asr_z_p_zw_ ', 'extensions=SVE '];
T[' ASR VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b01 ', 'asr_z_p_zw_ ', 'extensions=SVE '];
T[' ASR VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b10 ', 'asr_z_p_zw_ ', 'extensions=SVE '];
T[' LSL VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i64=$Zm VDS=D} ', 'size=0b00 ', 'lsl_z_p_zw_ ', 'extensions=SVE '];
T[' LSL VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b01 ', 'lsl_z_p_zw_ ', 'extensions=SVE '];
T[' LSL VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b10 ', 'lsl_z_p_zw_ ', 'extensions=SVE '];
T[' LSR VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i64=$Zm VDS=D} ', 'size=0b00 ', 'lsr_z_p_zw_ ', 'extensions=SVE '];
T[' LSR VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b01 ', 'lsr_z_p_zw_ ', 'extensions=SVE '];
T[' LSR VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b10 ', 'lsr_z_p_zw_ ', 'extensions=SVE '];



# sve_int_break: SVE partition break condition.
# A64: ig0=0b00100101 B:u=0bx S:u=0bx ig1=0b01000001 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx M:u=0bx Pd:u=0bxxxx
HINT('ICLASS', 'sve_int_break', 'A64');

DEF Pd => PRDREG('Pd', 'Pd', 'write') ;
DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Pn => PRDREG('Pn', 'Pn', 'read')  ;

T[' BRKA  VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} ', 'M=0b0 ', 'brka_p_p_p_   ', 'extensions=SVE '];
T[' BRKA  VREG={REG:i8=$Pd   VDS=B} PREG:M=$Pg VREG={REG:i8=$Pn   VDS=B} ', 'M=0b1 ', 'brka_p_p_p_   ', 'extensions=SVE '];
T[' BRKAS VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} ', 'NONE  ', 'brkas_p_p_p_z ', 'extensions=SVE '];
T[' BRKB  VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} ', 'M=0b0 ', 'brkb_p_p_p_   ', 'extensions=SVE '];
T[' BRKB  VREG={REG:i8=$Pd   VDS=B} PREG:M=$Pg VREG={REG:i8=$Pn   VDS=B} ', 'M=0b1 ', 'brkb_p_p_p_   ', 'extensions=SVE '];
T[' BRKBS VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} ', 'NONE  ', 'brkbs_p_p_p_z ', 'extensions=SVE '];



# sve_int_brkn: SVE propagate break to next partition.
# A64: ig0=0b001001010 S:u=0bx ig1=0b01100001 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx ig3=0b0 Pdm:u=0bxxxx
HINT('ICLASS', 'sve_int_brkn', 'A64');

DEF Pdm => PRDREG('Pdm', 'Pdm', 'read|write') ;
DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Pn  => PRDREG('Pn', 'Pn', 'read')         ;

T[' BRKN  VREG={REG:i8=$Pdm   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pdm   VDS=B} ', 'NONE ', 'brkn_p_p_pp_  ', 'extensions=SVE '];
T[' BRKNS VREG={REG:i8=$Pdm   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pdm   VDS=B} ', 'NONE ', 'brkns_p_p_pp_ ', 'extensions=SVE '];



# sve_int_brkp: SVE propagate break from previous partition.
# A64: ig0=0b00100101 op:u=0bx S:u=0bx ig1=0b00 Pm:u=0bxxxx ig2=0b11 Pg:u=0bxxxx ig3=0b0 Pn:u=0bxxxx B:u=0bx Pd:u=0bxxxx
HINT('ICLASS', 'sve_int_brkp', 'A64');

DEF Pd => PRDREG('Pd', 'Pd', 'write') ;
DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Pm => PRDREG('Pm', 'Pm', 'read')  ;
DEF Pn => PRDREG('Pn', 'Pn', 'read')  ;

T[' BRKPA  VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'brkpa_p_p_pp_  ', 'extensions=SVE '];
T[' BRKPAS VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'brkpas_p_p_pp_ ', 'extensions=SVE '];
T[' BRKPB  VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'brkpb_p_p_pp_  ', 'extensions=SVE '];
T[' BRKPBS VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'brkpbs_p_p_pp_ ', 'extensions=SVE '];



# sve_int_cmp_0: SVE integer compare vectors.
# A64: ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op:u=0bx ig2=0b0 o2:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx ne:u=0bx Pd:u=0bxxxx
HINT('ICLASS', 'sve_int_cmp_0', 'A64');

DEF Pd => PRDREG('Pd', 'Pd', 'write') ;
DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' CMPEQ VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i64=$Zm VDS=D}  ', 'size=0b00 ', 'cmpeq_p_p_zw_       ', 'extensions=SVE '];
T[' CMPEQ VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b01 ', 'cmpeq_p_p_zw_       ', 'extensions=SVE '];
T[' CMPEQ VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b10 ', 'cmpeq_p_p_zw_       ', 'extensions=SVE '];
T[' CMPNE VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i64=$Zm VDS=D}  ', 'size=0b00 ', 'cmpne_p_p_zw_       ', 'extensions=SVE '];
T[' CMPNE VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b01 ', 'cmpne_p_p_zw_       ', 'extensions=SVE '];
T[' CMPNE VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b10 ', 'cmpne_p_p_zw_       ', 'extensions=SVE '];
T[' CMPEQ VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'cmpeq_p_p_zz_       ', 'extensions=SVE '];
T[' CMPEQ VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'cmpeq_p_p_zz_       ', 'extensions=SVE '];
T[' CMPEQ VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'cmpeq_p_p_zz_       ', 'extensions=SVE '];
T[' CMPEQ VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'cmpeq_p_p_zz_       ', 'extensions=SVE '];
T[' CMPGT VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'cmpgt_p_p_zz_       ', 'extensions=SVE '];
T[' CMPGT VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'cmpgt_p_p_zz_       ', 'extensions=SVE '];
T[' CMPGT VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'cmpgt_p_p_zz_       ', 'extensions=SVE '];
T[' CMPGT VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'cmpgt_p_p_zz_       ', 'extensions=SVE '];
T[' CMPGE VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'cmpge_p_p_zz_       ', 'extensions=SVE '];
T[' CMPGE VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'cmpge_p_p_zz_       ', 'extensions=SVE '];
T[' CMPGE VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'cmpge_p_p_zz_       ', 'extensions=SVE '];
T[' CMPGE VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'cmpge_p_p_zz_       ', 'extensions=SVE '];
T[' CMPHI VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'cmphi_p_p_zz_       ', 'extensions=SVE '];
T[' CMPHI VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'cmphi_p_p_zz_       ', 'extensions=SVE '];
T[' CMPHI VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'cmphi_p_p_zz_       ', 'extensions=SVE '];
T[' CMPHI VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'cmphi_p_p_zz_       ', 'extensions=SVE '];
T[' CMPHS VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'cmphs_p_p_zz_       ', 'extensions=SVE '];
T[' CMPHS VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'cmphs_p_p_zz_       ', 'extensions=SVE '];
T[' CMPHS VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'cmphs_p_p_zz_       ', 'extensions=SVE '];
T[' CMPHS VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'cmphs_p_p_zz_       ', 'extensions=SVE '];
T[' CMPNE VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'cmpne_p_p_zz_       ', 'extensions=SVE '];
T[' CMPNE VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'cmpne_p_p_zz_       ', 'extensions=SVE '];
T[' CMPNE VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'cmpne_p_p_zz_       ', 'extensions=SVE '];
T[' CMPNE VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'cmpne_p_p_zz_       ', 'extensions=SVE '];
T[' CMPLE VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zm   VDS=B} VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'CMPLE_cmpge_p_p_zz_ ', 'extensions=SVE '];
T[' CMPLE VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zm VDS=H} VREG={REG:i16=$Zn VDS=H}    ', 'size=0b01 ', 'CMPLE_cmpge_p_p_zz_ ', 'extensions=SVE '];
T[' CMPLE VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zm VDS=S} VREG={REG:i32=$Zn VDS=S}    ', 'size=0b10 ', 'CMPLE_cmpge_p_p_zz_ ', 'extensions=SVE '];
T[' CMPLE VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zm VDS=D} VREG={REG:i64=$Zn VDS=D}    ', 'size=0b11 ', 'CMPLE_cmpge_p_p_zz_ ', 'extensions=SVE '];
T[' CMPLO VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zm   VDS=B} VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'CMPLO_cmphi_p_p_zz_ ', 'extensions=SVE '];
T[' CMPLO VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zm VDS=H} VREG={REG:i16=$Zn VDS=H}    ', 'size=0b01 ', 'CMPLO_cmphi_p_p_zz_ ', 'extensions=SVE '];
T[' CMPLO VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zm VDS=S} VREG={REG:i32=$Zn VDS=S}    ', 'size=0b10 ', 'CMPLO_cmphi_p_p_zz_ ', 'extensions=SVE '];
T[' CMPLO VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zm VDS=D} VREG={REG:i64=$Zn VDS=D}    ', 'size=0b11 ', 'CMPLO_cmphi_p_p_zz_ ', 'extensions=SVE '];
T[' CMPLS VREG={REG:s8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:s8=$Zm   VDS=B} VREG={REG:s8=$Zn   VDS=B} ', 'size=0b00 ', 'CMPLS_cmphs_p_p_zz_ ', 'extensions=SVE '];
T[' CMPLS VREG={REG:s16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:s16=$Zm VDS=H} VREG={REG:s16=$Zn VDS=H}    ', 'size=0b01 ', 'CMPLS_cmphs_p_p_zz_ ', 'extensions=SVE '];
T[' CMPLS VREG={REG:s32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:s32=$Zm VDS=S} VREG={REG:s32=$Zn VDS=S}    ', 'size=0b10 ', 'CMPLS_cmphs_p_p_zz_ ', 'extensions=SVE '];
T[' CMPLS VREG={REG:s64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:s64=$Zm VDS=D} VREG={REG:s64=$Zn VDS=D}    ', 'size=0b11 ', 'CMPLS_cmphs_p_p_zz_ ', 'extensions=SVE '];
T[' CMPLT VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zm   VDS=B} VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'CMPLT_cmpgt_p_p_zz_ ', 'extensions=SVE '];
T[' CMPLT VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zm VDS=H} VREG={REG:i16=$Zn VDS=H}    ', 'size=0b01 ', 'CMPLT_cmpgt_p_p_zz_ ', 'extensions=SVE '];
T[' CMPLT VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zm VDS=S} VREG={REG:i32=$Zn VDS=S}    ', 'size=0b10 ', 'CMPLT_cmpgt_p_p_zz_ ', 'extensions=SVE '];
T[' CMPLT VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zm VDS=D} VREG={REG:i64=$Zn VDS=D}    ', 'size=0b11 ', 'CMPLT_cmpgt_p_p_zz_ ', 'extensions=SVE '];



# sve_int_cmp_1: SVE integer compare with wide elements.
# A64: ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U:u=0bx ig2=0b1 lt:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx ne:u=0bx Pd:u=0bxxxx
HINT('ICLASS', 'sve_int_cmp_1', 'A64');

DEF Pd => PRDREG('Pd', 'Pd', 'write') ;
DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' CMPGT VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i64=$Zm VDS=D} ', 'size=0b00 ', 'cmpgt_p_p_zw_ ', 'extensions=SVE '];
T[' CMPGT VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b01 ', 'cmpgt_p_p_zw_ ', 'extensions=SVE '];
T[' CMPGT VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b10 ', 'cmpgt_p_p_zw_ ', 'extensions=SVE '];
T[' CMPGE VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i64=$Zm VDS=D} ', 'size=0b00 ', 'cmpge_p_p_zw_ ', 'extensions=SVE '];
T[' CMPGE VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b01 ', 'cmpge_p_p_zw_ ', 'extensions=SVE '];
T[' CMPGE VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b10 ', 'cmpge_p_p_zw_ ', 'extensions=SVE '];
T[' CMPHI VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i64=$Zm VDS=D} ', 'size=0b00 ', 'cmphi_p_p_zw_ ', 'extensions=SVE '];
T[' CMPHI VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b01 ', 'cmphi_p_p_zw_ ', 'extensions=SVE '];
T[' CMPHI VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b10 ', 'cmphi_p_p_zw_ ', 'extensions=SVE '];
T[' CMPHS VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i64=$Zm VDS=D} ', 'size=0b00 ', 'cmphs_p_p_zw_ ', 'extensions=SVE '];
T[' CMPHS VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b01 ', 'cmphs_p_p_zw_ ', 'extensions=SVE '];
T[' CMPHS VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b10 ', 'cmphs_p_p_zw_ ', 'extensions=SVE '];
T[' CMPLT VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i64=$Zm VDS=D} ', 'size=0b00 ', 'cmplt_p_p_zw_ ', 'extensions=SVE '];
T[' CMPLT VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b01 ', 'cmplt_p_p_zw_ ', 'extensions=SVE '];
T[' CMPLT VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b10 ', 'cmplt_p_p_zw_ ', 'extensions=SVE '];
T[' CMPLE VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i64=$Zm VDS=D} ', 'size=0b00 ', 'cmple_p_p_zw_ ', 'extensions=SVE '];
T[' CMPLE VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b01 ', 'cmple_p_p_zw_ ', 'extensions=SVE '];
T[' CMPLE VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b10 ', 'cmple_p_p_zw_ ', 'extensions=SVE '];
T[' CMPLO VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i64=$Zm VDS=D} ', 'size=0b00 ', 'cmplo_p_p_zw_ ', 'extensions=SVE '];
T[' CMPLO VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b01 ', 'cmplo_p_p_zw_ ', 'extensions=SVE '];
T[' CMPLO VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b10 ', 'cmplo_p_p_zw_ ', 'extensions=SVE '];
T[' CMPLS VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i64=$Zm VDS=D} ', 'size=0b00 ', 'cmpls_p_p_zw_ ', 'extensions=SVE '];
T[' CMPLS VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b01 ', 'cmpls_p_p_zw_ ', 'extensions=SVE '];
T[' CMPLS VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i64=$Zm VDS=D}   ', 'size=0b10 ', 'cmpls_p_p_zw_ ', 'extensions=SVE '];



# sve_int_count: SVE element count.
# A64: ig0=0b00000100 size:u=0bxx ig1=0b10 imm4:u=0bxxxx ig2=0b11100 op:u=0bx pattern:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'sve_int_count', 'A64');

DEF Xd      => GPR64('Xd', 'Rd', 'write')                                              ;
DEF amount0 => IMM('amount', 'imm4', range => (1, 16), default => 1)                   ;
DEF pattern => PATTERN('pattern', 'pattern', table => 'tbl_pattern', default => 'ALL') ;

T[' CNTB REG=$Xd PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'cntb_r_s_ ', 'extensions=SVE '];
T[' CNTD REG=$Xd PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'cntd_r_s_ ', 'extensions=SVE '];
T[' CNTH REG=$Xd PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'cnth_r_s_ ', 'extensions=SVE '];
T[' CNTW REG=$Xd PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'cntw_r_s_ ', 'extensions=SVE '];



# sve_int_count_r: SVE inc/dec register by predicate count.
# A64: ig0=0b00100101 size:u=0bxx ig1=0b1011 op:u=0bx D:u=0bx ig2=0b10001 opc2:u=0bxx Pm:u=0bxxxx Rdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_count_r', 'A64');

DEF Pm  => PRDREG('Pm', 'Pm', 'read')        ;
DEF Xdn => GPR64('Xdn', 'Rdn', 'read|write') ;

T[' DECP REG=$Xdn VREG={REG:i8=$Pm   VDS=B} ', 'size=0b00 ', 'decp_r_p_r_ ', 'extensions=SVE '];
T[' DECP REG=$Xdn VREG={REG:i16=$Pm VDS=H}  ', 'size=0b01 ', 'decp_r_p_r_ ', 'extensions=SVE '];
T[' DECP REG=$Xdn VREG={REG:i32=$Pm VDS=S}  ', 'size=0b10 ', 'decp_r_p_r_ ', 'extensions=SVE '];
T[' DECP REG=$Xdn VREG={REG:i64=$Pm VDS=D}  ', 'size=0b11 ', 'decp_r_p_r_ ', 'extensions=SVE '];
T[' INCP REG=$Xdn VREG={REG:i8=$Pm   VDS=B} ', 'size=0b00 ', 'incp_r_p_r_ ', 'extensions=SVE '];
T[' INCP REG=$Xdn VREG={REG:i16=$Pm VDS=H}  ', 'size=0b01 ', 'incp_r_p_r_ ', 'extensions=SVE '];
T[' INCP REG=$Xdn VREG={REG:i32=$Pm VDS=S}  ', 'size=0b10 ', 'incp_r_p_r_ ', 'extensions=SVE '];
T[' INCP REG=$Xdn VREG={REG:i64=$Pm VDS=D}  ', 'size=0b11 ', 'incp_r_p_r_ ', 'extensions=SVE '];



# sve_int_count_r_sat: SVE saturating inc/dec register by predicate count.
# A64: ig0=0b00100101 size:u=0bxx ig1=0b1010 D:u=0bx U:u=0bx ig2=0b10001 sf:u=0bx op:u=0bx Pm:u=0bxxxx Rdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_count_r_sat', 'A64');

DEF Pm  => PRDREG('Pm', 'Pm', 'read')        ;
DEF Wdn => GPR32('Wdn', 'Rdn', 'read|write') ;
DEF Xdn => GPR64('Xdn', 'Rdn', 'read|write') ;

T[' SQDECP REG:s64=$Xdn VREG={REG:s8=$Pm   VDS=B} REG:s32=$Wdn ', 'size=0b00 ', 'sqdecp_r_p_r_sx ', 'extensions=SVE '];
T[' SQDECP REG:s64=$Xdn VREG={REG:s16=$Pm VDS=H} REG:s32=$Wdn  ', 'size=0b01 ', 'sqdecp_r_p_r_sx ', 'extensions=SVE '];
T[' SQDECP REG:s64=$Xdn VREG={REG:s32=$Pm VDS=S} REG:s32=$Wdn  ', 'size=0b10 ', 'sqdecp_r_p_r_sx ', 'extensions=SVE '];
T[' SQDECP REG:s64=$Xdn VREG={REG:s64=$Pm VDS=D} REG:s32=$Wdn  ', 'size=0b11 ', 'sqdecp_r_p_r_sx ', 'extensions=SVE '];
T[' SQDECP REG:s64=$Xdn VREG={REG:s8=$Pm   VDS=B}              ', 'size=0b00 ', 'sqdecp_r_p_r_x  ', 'extensions=SVE '];
T[' SQDECP REG:s64=$Xdn VREG={REG:s16=$Pm VDS=H}               ', 'size=0b01 ', 'sqdecp_r_p_r_x  ', 'extensions=SVE '];
T[' SQDECP REG:s64=$Xdn VREG={REG:s32=$Pm VDS=S}               ', 'size=0b10 ', 'sqdecp_r_p_r_x  ', 'extensions=SVE '];
T[' SQDECP REG:s64=$Xdn VREG={REG:s64=$Pm VDS=D}               ', 'size=0b11 ', 'sqdecp_r_p_r_x  ', 'extensions=SVE '];
T[' SQINCP REG:s64=$Xdn VREG={REG:s8=$Pm   VDS=B} REG:s32=$Wdn ', 'size=0b00 ', 'sqincp_r_p_r_sx ', 'extensions=SVE '];
T[' SQINCP REG:s64=$Xdn VREG={REG:s16=$Pm VDS=H} REG:s32=$Wdn  ', 'size=0b01 ', 'sqincp_r_p_r_sx ', 'extensions=SVE '];
T[' SQINCP REG:s64=$Xdn VREG={REG:s32=$Pm VDS=S} REG:s32=$Wdn  ', 'size=0b10 ', 'sqincp_r_p_r_sx ', 'extensions=SVE '];
T[' SQINCP REG:s64=$Xdn VREG={REG:s64=$Pm VDS=D} REG:s32=$Wdn  ', 'size=0b11 ', 'sqincp_r_p_r_sx ', 'extensions=SVE '];
T[' SQINCP REG:s64=$Xdn VREG={REG:s8=$Pm   VDS=B}              ', 'size=0b00 ', 'sqincp_r_p_r_x  ', 'extensions=SVE '];
T[' SQINCP REG:s64=$Xdn VREG={REG:s16=$Pm VDS=H}               ', 'size=0b01 ', 'sqincp_r_p_r_x  ', 'extensions=SVE '];
T[' SQINCP REG:s64=$Xdn VREG={REG:s32=$Pm VDS=S}               ', 'size=0b10 ', 'sqincp_r_p_r_x  ', 'extensions=SVE '];
T[' SQINCP REG:s64=$Xdn VREG={REG:s64=$Pm VDS=D}               ', 'size=0b11 ', 'sqincp_r_p_r_x  ', 'extensions=SVE '];
T[' UQDECP REG:u32=$Wdn VREG={REG:u8=$Pm   VDS=B}              ', 'size=0b00 ', 'uqdecp_r_p_r_uw ', 'extensions=SVE '];
T[' UQDECP REG:u32=$Wdn VREG={REG:u16=$Pm VDS=H}               ', 'size=0b01 ', 'uqdecp_r_p_r_uw ', 'extensions=SVE '];
T[' UQDECP REG:u32=$Wdn VREG={REG:u32=$Pm VDS=S}               ', 'size=0b10 ', 'uqdecp_r_p_r_uw ', 'extensions=SVE '];
T[' UQDECP REG:u32=$Wdn VREG={REG:u64=$Pm VDS=D}               ', 'size=0b11 ', 'uqdecp_r_p_r_uw ', 'extensions=SVE '];
T[' UQDECP REG:u64=$Xdn VREG={REG:u8=$Pm   VDS=B}              ', 'size=0b00 ', 'uqdecp_r_p_r_x  ', 'extensions=SVE '];
T[' UQDECP REG:u64=$Xdn VREG={REG:u16=$Pm VDS=H}               ', 'size=0b01 ', 'uqdecp_r_p_r_x  ', 'extensions=SVE '];
T[' UQDECP REG:u64=$Xdn VREG={REG:u32=$Pm VDS=S}               ', 'size=0b10 ', 'uqdecp_r_p_r_x  ', 'extensions=SVE '];
T[' UQDECP REG:u64=$Xdn VREG={REG:u64=$Pm VDS=D}               ', 'size=0b11 ', 'uqdecp_r_p_r_x  ', 'extensions=SVE '];
T[' UQINCP REG:u32=$Wdn VREG={REG:u8=$Pm   VDS=B}              ', 'size=0b00 ', 'uqincp_r_p_r_uw ', 'extensions=SVE '];
T[' UQINCP REG:u32=$Wdn VREG={REG:u16=$Pm VDS=H}               ', 'size=0b01 ', 'uqincp_r_p_r_uw ', 'extensions=SVE '];
T[' UQINCP REG:u32=$Wdn VREG={REG:u32=$Pm VDS=S}               ', 'size=0b10 ', 'uqincp_r_p_r_uw ', 'extensions=SVE '];
T[' UQINCP REG:u32=$Wdn VREG={REG:u64=$Pm VDS=D}               ', 'size=0b11 ', 'uqincp_r_p_r_uw ', 'extensions=SVE '];
T[' UQINCP REG:u64=$Xdn VREG={REG:u8=$Pm   VDS=B}              ', 'size=0b00 ', 'uqincp_r_p_r_x  ', 'extensions=SVE '];
T[' UQINCP REG:u64=$Xdn VREG={REG:u16=$Pm VDS=H}               ', 'size=0b01 ', 'uqincp_r_p_r_x  ', 'extensions=SVE '];
T[' UQINCP REG:u64=$Xdn VREG={REG:u32=$Pm VDS=S}               ', 'size=0b10 ', 'uqincp_r_p_r_x  ', 'extensions=SVE '];
T[' UQINCP REG:u64=$Xdn VREG={REG:u64=$Pm VDS=D}               ', 'size=0b11 ', 'uqincp_r_p_r_x  ', 'extensions=SVE '];



# sve_int_count_v: SVE inc/dec vector by predicate count.
# A64: ig0=0b00100101 size:u=0bxx ig1=0b1011 op:u=0bx D:u=0bx ig2=0b10000 opc2:u=0bxx Pm:u=0bxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_count_v', 'A64');

DEF Pm  => PRDREG('Pm', 'Pm', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;

T[' DECP VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Pm VDS=H} ', 'size=0b01 ', 'decp_z_p_z_ ', 'extensions=SVE '];
T[' DECP VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Pm VDS=S} ', 'size=0b10 ', 'decp_z_p_z_ ', 'extensions=SVE '];
T[' DECP VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Pm VDS=D} ', 'size=0b11 ', 'decp_z_p_z_ ', 'extensions=SVE '];
T[' INCP VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Pm VDS=H} ', 'size=0b01 ', 'incp_z_p_z_ ', 'extensions=SVE '];
T[' INCP VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Pm VDS=S} ', 'size=0b10 ', 'incp_z_p_z_ ', 'extensions=SVE '];
T[' INCP VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Pm VDS=D} ', 'size=0b11 ', 'incp_z_p_z_ ', 'extensions=SVE '];



# sve_int_count_v_sat: SVE saturating inc/dec vector by predicate count.
# A64: ig0=0b00100101 size:u=0bxx ig1=0b1010 D:u=0bx U:u=0bx ig2=0b10000 opc:u=0bxx Pm:u=0bxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_count_v_sat', 'A64');

DEF Pm  => PRDREG('Pm', 'Pm', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;

T[' SQDECP VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Pm VDS=H} ', 'size=0b01 ', 'sqdecp_z_p_z_ ', 'extensions=SVE '];
T[' SQDECP VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Pm VDS=S} ', 'size=0b10 ', 'sqdecp_z_p_z_ ', 'extensions=SVE '];
T[' SQDECP VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Pm VDS=D} ', 'size=0b11 ', 'sqdecp_z_p_z_ ', 'extensions=SVE '];
T[' SQINCP VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Pm VDS=H} ', 'size=0b01 ', 'sqincp_z_p_z_ ', 'extensions=SVE '];
T[' SQINCP VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Pm VDS=S} ', 'size=0b10 ', 'sqincp_z_p_z_ ', 'extensions=SVE '];
T[' SQINCP VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Pm VDS=D} ', 'size=0b11 ', 'sqincp_z_p_z_ ', 'extensions=SVE '];
T[' UQDECP VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Pm VDS=H} ', 'size=0b01 ', 'uqdecp_z_p_z_ ', 'extensions=SVE '];
T[' UQDECP VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Pm VDS=S} ', 'size=0b10 ', 'uqdecp_z_p_z_ ', 'extensions=SVE '];
T[' UQDECP VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Pm VDS=D} ', 'size=0b11 ', 'uqdecp_z_p_z_ ', 'extensions=SVE '];
T[' UQINCP VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Pm VDS=H} ', 'size=0b01 ', 'uqincp_z_p_z_ ', 'extensions=SVE '];
T[' UQINCP VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Pm VDS=S} ', 'size=0b10 ', 'uqincp_z_p_z_ ', 'extensions=SVE '];
T[' UQINCP VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Pm VDS=D} ', 'size=0b11 ', 'uqincp_z_p_z_ ', 'extensions=SVE '];



# sve_int_countvlv0: SVE saturating inc/dec vector by element count.
# A64: ig0=0b00000100 size:u=0bxx ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D:u=0bx U:u=0bx pattern:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_countvlv0', 'A64');

DEF Zdn     => SVEREG('Zdn', 'Zdn', 'read|write')                                      ;
DEF amount0 => IMM('amount', 'imm4', range => (1, 16), default => 1)                   ;
DEF pattern => PATTERN('pattern', 'pattern', table => 'tbl_pattern', default => 'ALL') ;

T[' SQDECD VREG={REG:s64=$Zdn VDS=D} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'sqdecd_z_zs_ ', 'extensions=SVE '];
T[' SQDECH VREG={REG:s16=$Zdn VDS=H} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'sqdech_z_zs_ ', 'extensions=SVE '];
T[' SQDECW VREG={REG:s32=$Zdn VDS=S} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'sqdecw_z_zs_ ', 'extensions=SVE '];
T[' SQINCD VREG={REG:s64=$Zdn VDS=D} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'sqincd_z_zs_ ', 'extensions=SVE '];
T[' SQINCH VREG={REG:s16=$Zdn VDS=H} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'sqinch_z_zs_ ', 'extensions=SVE '];
T[' SQINCW VREG={REG:s32=$Zdn VDS=S} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'sqincw_z_zs_ ', 'extensions=SVE '];
T[' UQDECD VREG={REG:u64=$Zdn VDS=D} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'uqdecd_z_zs_ ', 'extensions=SVE '];
T[' UQDECH VREG={REG:u16=$Zdn VDS=H} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'uqdech_z_zs_ ', 'extensions=SVE '];
T[' UQDECW VREG={REG:u32=$Zdn VDS=S} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'uqdecw_z_zs_ ', 'extensions=SVE '];
T[' UQINCD VREG={REG:u64=$Zdn VDS=D} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'uqincd_z_zs_ ', 'extensions=SVE '];
T[' UQINCH VREG={REG:u16=$Zdn VDS=H} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'uqinch_z_zs_ ', 'extensions=SVE '];
T[' UQINCW VREG={REG:u32=$Zdn VDS=S} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'uqincw_z_zs_ ', 'extensions=SVE '];



# sve_int_countvlv1: SVE inc/dec vector by element count.
# A64: ig0=0b00000100 size:u=0bxx ig1=0b11 imm4:u=0bxxxx ig2=0b11000 D:u=0bx pattern:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_countvlv1', 'A64');

DEF Zdn     => SVEREG('Zdn', 'Zdn', 'read|write')                                      ;
DEF amount0 => IMM('amount', 'imm4', range => (1, 16), default => 1)                   ;
DEF pattern => PATTERN('pattern', 'pattern', table => 'tbl_pattern', default => 'ALL') ;

T[' DECD VREG={REG:i64=$Zdn VDS=D} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'decd_z_zs_ ', 'extensions=SVE '];
T[' DECH VREG={REG:i16=$Zdn VDS=H} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'dech_z_zs_ ', 'extensions=SVE '];
T[' DECW VREG={REG:i32=$Zdn VDS=S} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'decw_z_zs_ ', 'extensions=SVE '];
T[' INCD VREG={REG:i64=$Zdn VDS=D} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'incd_z_zs_ ', 'extensions=SVE '];
T[' INCH VREG={REG:i16=$Zdn VDS=H} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'inch_z_zs_ ', 'extensions=SVE '];
T[' INCW VREG={REG:i32=$Zdn VDS=S} PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'incw_z_zs_ ', 'extensions=SVE '];



# sve_int_cterm: SVE conditionally terminate scalars.
# A64: ig0=0b00 ig1=0b100101 op:u=0bx sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b001000 Rn:u=0bxxxxx ne:u=0bx ig4=0b0 ig5=0b0 ig6=0b00
HINT('ICLASS', 'sve_int_cterm', 'A64');

DEF Wm => GPR32('Wm', 'Rm', 'read') ;
DEF Wn => GPR32('Wn', 'Rn', 'read') ;
DEF Xm => GPR64('Xm', 'Rm', 'read') ;
DEF Xn => GPR64('Xn', 'Rn', 'read') ;

T[' CTERMEQ REG=$Wn REG=$Wm ', 'sz=0b0 ', 'ctermeq_rr_ ', 'extensions=SVE '];
T[' CTERMEQ REG=$Xn REG=$Xm ', 'sz=0b1 ', 'ctermeq_rr_ ', 'extensions=SVE '];
T[' CTERMNE REG=$Wn REG=$Wm ', 'sz=0b0 ', 'ctermne_rr_ ', 'extensions=SVE '];
T[' CTERMNE REG=$Xn REG=$Xm ', 'sz=0b1 ', 'ctermne_rr_ ', 'extensions=SVE '];



# sve_int_dup_fpimm: SVE broadcast floating-point immediate (unpredicated).
# A64: ig0=0b00100101 size:u=0bxx ig1=0b111 opc:u=0bxx ig2=0b111 o2:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_dup_fpimm', 'A64');

DEF Zd    => SVEREG('Zd', 'Zd', 'write') ;
DEF const => IMM('const', 'imm8')        ;

T[' FDUP VREG={REG:f16=$Zd VDS=H} IMM:i16=$const ', 'size=0b01 ', 'fdup_z_i_      ', 'extensions=SVE '];
T[' FDUP VREG={REG:f32=$Zd VDS=S} IMM:i32=$const ', 'size=0b10 ', 'fdup_z_i_      ', 'extensions=SVE '];
T[' FDUP VREG={REG:f64=$Zd VDS=D} IMM:i64=$const ', 'size=0b11 ', 'fdup_z_i_      ', 'extensions=SVE '];
T[' FMOV VREG={REG:f16=$Zd VDS=H} IMM:i16=$const ', 'size=0b01 ', 'FMOV_fdup_z_i_ ', 'extensions=SVE '];
T[' FMOV VREG={REG:f32=$Zd VDS=S} IMM:i32=$const ', 'size=0b10 ', 'FMOV_fdup_z_i_ ', 'extensions=SVE '];
T[' FMOV VREG={REG:f64=$Zd VDS=D} IMM:i64=$const ', 'size=0b11 ', 'FMOV_fdup_z_i_ ', 'extensions=SVE '];



# sve_int_dup_fpimm_pred: SVE copy floating-point immediate (predicated).
# A64: ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b110 imm8:u=0bxxxxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_dup_fpimm_pred', 'A64');

DEF Pg    => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zd    => SVEREG('Zd', 'Zd', 'write') ;
DEF const => IMM('const', 'imm8')        ;

T[' FCPY VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg IMM:i16=$const ', 'size=0b01 ', 'fcpy_z_p_i_      ', 'extensions=SVE '];
T[' FCPY VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg IMM:i32=$const ', 'size=0b10 ', 'fcpy_z_p_i_      ', 'extensions=SVE '];
T[' FCPY VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg IMM:i64=$const ', 'size=0b11 ', 'fcpy_z_p_i_      ', 'extensions=SVE '];
T[' FMOV VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg IMM:i16=$const ', 'size=0b01 ', 'FMOV_fcpy_z_p_i_ ', 'extensions=SVE '];
T[' FMOV VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg IMM:i32=$const ', 'size=0b10 ', 'FMOV_fcpy_z_p_i_ ', 'extensions=SVE '];
T[' FMOV VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg IMM:i64=$const ', 'size=0b11 ', 'FMOV_fcpy_z_p_i_ ', 'extensions=SVE '];



# sve_int_dup_imm: SVE broadcast integer immediate (unpredicated).
# A64: ig0=0b00100101 size:u=0bxx ig1=0b111 opc:u=0bxx ig2=0b011 sh:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_dup_imm', 'A64');

DEF Zd      => SVEREG('Zd', 'Zd', 'write')                                            ;
DEF imm     => IMM('imm', 'imm8', signpos => 'imm8<7>', range => (-128, 127))         ;
DEF shifter => SHIFTER('shifter', 'sh', table => 'tbl_shifter1', default => 'LSL #0') ;

T[' DUP  VREG={REG:i8=$Zd   VDS=B} IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter} ', 'size=0b00 ', 'dup_z_i_      ', 'extensions=SVE '];
T[' DUP  VREG={REG:i16=$Zd VDS=H} IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b01 ', 'dup_z_i_      ', 'extensions=SVE '];
T[' DUP  VREG={REG:i32=$Zd VDS=S} IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b10 ', 'dup_z_i_      ', 'extensions=SVE '];
T[' DUP  VREG={REG:i64=$Zd VDS=D} IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b11 ', 'dup_z_i_      ', 'extensions=SVE '];
T[' FMOV VREG={REG:f16=$Zd VDS=H} FPIMM:f32=0.0                             ', 'size=0b01 ', 'FMOV_dup_z_i_ ', 'extensions=SVE '];
T[' FMOV VREG={REG:f32=$Zd VDS=S} FPIMM:f32=0.0                             ', 'size=0b10 ', 'FMOV_dup_z_i_ ', 'extensions=SVE '];
T[' FMOV VREG={REG:f64=$Zd VDS=D} FPIMM:f32=0.0                             ', 'size=0b11 ', 'FMOV_dup_z_i_ ', 'extensions=SVE '];
T[' MOV  VREG={REG:i8=$Zd   VDS=B} IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter} ', 'size=0b00 ', 'MOV_dup_z_i_  ', 'extensions=SVE '];
T[' MOV  VREG={REG:i16=$Zd VDS=H} IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b01 ', 'MOV_dup_z_i_  ', 'extensions=SVE '];
T[' MOV  VREG={REG:i32=$Zd VDS=S} IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b10 ', 'MOV_dup_z_i_  ', 'extensions=SVE '];
T[' MOV  VREG={REG:i64=$Zd VDS=D} IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b11 ', 'MOV_dup_z_i_  ', 'extensions=SVE '];



# sve_int_dup_imm_pred: SVE copy integer immediate (predicated).
# A64: ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b0 M:u=0bx sh:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_dup_imm_pred', 'A64');

DEF Pg      => PRDREG('Pg', 'Pg', 'read')                                             ;
DEF Zd      => SVEREG('Zd', 'Zd', 'write')                                            ;
DEF imm     => IMM('imm', 'imm8', signpos => 'imm8<7>', range => (-128, 127))         ;
DEF shifter => SHIFTER('shifter', 'sh', table => 'tbl_shifter1', default => 'LSL #0') ;

T[' CPY  VREG={REG:i8=$Zd   VDS=B} PREG:Z=$Pg IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter} ', 'size=0b00 ', 'cpy_z_o_i_      ', 'extensions=SVE '];
T[' CPY  VREG={REG:i16=$Zd VDS=H} PREG:Z=$Pg IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b01 ', 'cpy_z_o_i_      ', 'extensions=SVE '];
T[' CPY  VREG={REG:i32=$Zd VDS=S} PREG:Z=$Pg IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b10 ', 'cpy_z_o_i_      ', 'extensions=SVE '];
T[' CPY  VREG={REG:i64=$Zd VDS=D} PREG:Z=$Pg IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b11 ', 'cpy_z_o_i_      ', 'extensions=SVE '];
T[' CPY  VREG={REG:i8=$Zd   VDS=B} PREG:M=$Pg IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter} ', 'size=0b00 ', 'cpy_z_p_i_      ', 'extensions=SVE '];
T[' CPY  VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b01 ', 'cpy_z_p_i_      ', 'extensions=SVE '];
T[' CPY  VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b10 ', 'cpy_z_p_i_      ', 'extensions=SVE '];
T[' CPY  VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b11 ', 'cpy_z_p_i_      ', 'extensions=SVE '];
T[' FMOV VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg FPIMM:f32=0.0                             ', 'size=0b01 ', 'FMOV_cpy_z_p_i_ ', 'extensions=SVE '];
T[' FMOV VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg FPIMM:f32=0.0                             ', 'size=0b10 ', 'FMOV_cpy_z_p_i_ ', 'extensions=SVE '];
T[' FMOV VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg FPIMM:f32=0.0                             ', 'size=0b11 ', 'FMOV_cpy_z_p_i_ ', 'extensions=SVE '];
T[' MOV  VREG={REG:i8=$Zd   VDS=B} PREG:Z=$Pg IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter} ', 'size=0b00 ', 'MOV_cpy_z_o_i_  ', 'extensions=SVE '];
T[' MOV  VREG={REG:i16=$Zd VDS=H} PREG:Z=$Pg IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b01 ', 'MOV_cpy_z_o_i_  ', 'extensions=SVE '];
T[' MOV  VREG={REG:i32=$Zd VDS=S} PREG:Z=$Pg IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b10 ', 'MOV_cpy_z_o_i_  ', 'extensions=SVE '];
T[' MOV  VREG={REG:i64=$Zd VDS=D} PREG:Z=$Pg IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b11 ', 'MOV_cpy_z_o_i_  ', 'extensions=SVE '];
T[' MOV  VREG={REG:i8=$Zd   VDS=B} PREG:M=$Pg IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter} ', 'size=0b00 ', 'MOV_cpy_z_p_i_  ', 'extensions=SVE '];
T[' MOV  VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b01 ', 'MOV_cpy_z_p_i_  ', 'extensions=SVE '];
T[' MOV  VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b10 ', 'MOV_cpy_z_p_i_  ', 'extensions=SVE '];
T[' MOV  VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg IMMSH={IMM:s8=$imm SHIFTER:OPT=$shifter}  ', 'size=0b11 ', 'MOV_cpy_z_p_i_  ', 'extensions=SVE '];



# sve_int_dup_mask_imm: SVE broadcast bitmask immediate.
# A64: ig0=0b00000101110000 imm13:u=0bxxxxxxxxxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_dup_mask_imm', 'A64');

DEF Zd   => SVEREG('Zd', 'Zd', 'write') ;
DEF aimm => IMM('aimm', 'imm13')        ;

T[' DUPM VREG={REG:i32=$Zd VDS=S} IMM:i32=$aimm ', 'imm13=0b0xxxxxx0xxxxx ', 'dupm_z_i_     ', 'extensions=SVE '];
T[' DUPM VREG={REG:i16=$Zd VDS=H} IMM:i16=$aimm ', 'imm13=0b0xxxxxx10xxxx ', 'dupm_z_i_     ', 'extensions=SVE '];
T[' DUPM VREG={REG:i8=$Zd   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx110xxx ', 'dupm_z_i_     ', 'extensions=SVE '];
T[' DUPM VREG={REG:i8=$Zd   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx1110xx ', 'dupm_z_i_     ', 'extensions=SVE '];
T[' DUPM VREG={REG:i8=$Zd   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx11110x ', 'dupm_z_i_     ', 'extensions=SVE '];
T[' DUPM VREG={REG:i64=$Zd VDS=D} IMM:i64=$aimm ', 'imm13=0b1xxxxxxxxxxxx ', 'dupm_z_i_     ', 'extensions=SVE '];
T[' MOV  VREG={REG:i32=$Zd VDS=S} IMM:i32=$aimm ', 'imm13=0b0xxxxxx0xxxxx ', 'MOV_dupm_z_i_ ', 'extensions=SVE '];
T[' MOV  VREG={REG:i16=$Zd VDS=H} IMM:i16=$aimm ', 'imm13=0b0xxxxxx10xxxx ', 'MOV_dupm_z_i_ ', 'extensions=SVE '];
T[' MOV  VREG={REG:i8=$Zd   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx110xxx ', 'MOV_dupm_z_i_ ', 'extensions=SVE '];
T[' MOV  VREG={REG:i8=$Zd   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx1110xx ', 'MOV_dupm_z_i_ ', 'extensions=SVE '];
T[' MOV  VREG={REG:i8=$Zd   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx11110x ', 'MOV_dupm_z_i_ ', 'extensions=SVE '];
T[' MOV  VREG={REG:i64=$Zd VDS=D} IMM:i64=$aimm ', 'imm13=0b1xxxxxxxxxxxx ', 'MOV_dupm_z_i_ ', 'extensions=SVE '];



# sve_int_index_ii: SVE index generation (immediate start, immediate increment).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b1 imm5b:u=0bxxxxx ig2=0b010000 imm5:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_index_ii', 'A64');

DEF Zd   => SVEREG('Zd', 'Zd', 'write')                                     ;
DEF imm  => IMM('imm1', 'imm5', signpos => 'imm5<4>', range => (-16, 15))   ;
DEF imm2 => IMM('imm2', 'imm5b', signpos => 'imm5b<4>', range => (-16, 15)) ;

T[' INDEX VREG={REG:i8=$Zd   VDS=B} IMM:s8=$imm IMM:s8=$imm2 ', 'size=0b00 ', 'index_z_ii_ ', 'extensions=SVE '];
T[' INDEX VREG={REG:i16=$Zd VDS=H} IMM:s8=$imm IMM:s8=$imm2  ', 'size=0b01 ', 'index_z_ii_ ', 'extensions=SVE '];
T[' INDEX VREG={REG:i32=$Zd VDS=S} IMM:s8=$imm IMM:s8=$imm2  ', 'size=0b10 ', 'index_z_ii_ ', 'extensions=SVE '];
T[' INDEX VREG={REG:i64=$Zd VDS=D} IMM:s8=$imm IMM:s8=$imm2  ', 'size=0b11 ', 'index_z_ii_ ', 'extensions=SVE '];



# sve_int_index_ir: SVE index generation (immediate start, register increment).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b010010 imm5:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_index_ir', 'A64');

DEF Wm  => GPR32('Wm', 'Rm', 'read')                                    ;
DEF Xm  => GPR64('Xm', 'Rm', 'read')                                    ;
DEF Zd  => SVEREG('Zd', 'Zd', 'write')                                  ;
DEF imm => IMM('imm', 'imm5', signpos => 'imm5<4>', range => (-16, 15)) ;

T[' INDEX VREG={REG:i8=$Zd   VDS=B} IMM:s8=$imm REG=$Wm ', 'size=0b00 ', 'index_z_ir_ ', 'extensions=SVE '];
T[' INDEX VREG={REG:i16=$Zd VDS=H} IMM:s8=$imm REG=$Wm  ', 'size=0b01 ', 'index_z_ir_ ', 'extensions=SVE '];
T[' INDEX VREG={REG:i32=$Zd VDS=S} IMM:s8=$imm REG=$Wm  ', 'size=0b10 ', 'index_z_ir_ ', 'extensions=SVE '];
T[' INDEX VREG={REG:i64=$Zd VDS=D} IMM:s8=$imm REG=$Xm  ', 'size=0b11 ', 'index_z_ir_ ', 'extensions=SVE '];



# sve_int_index_ri: SVE index generation (register start, immediate increment).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b1 imm5:u=0bxxxxx ig2=0b010001 Rn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_index_ri', 'A64');

DEF Wn  => GPR32('Wn', 'Rn', 'read')                                    ;
DEF Xn  => GPR64('Xn', 'Rn', 'read')                                    ;
DEF Zd  => SVEREG('Zd', 'Zd', 'write')                                  ;
DEF imm => IMM('imm', 'imm5', signpos => 'imm5<4>', range => (-16, 15)) ;

T[' INDEX VREG={REG:i8=$Zd   VDS=B} REG=$Wn IMM:s8=$imm ', 'size=0b00 ', 'index_z_ri_ ', 'extensions=SVE '];
T[' INDEX VREG={REG:i16=$Zd VDS=H} REG=$Wn IMM:s8=$imm  ', 'size=0b01 ', 'index_z_ri_ ', 'extensions=SVE '];
T[' INDEX VREG={REG:i32=$Zd VDS=S} REG=$Wn IMM:s8=$imm  ', 'size=0b10 ', 'index_z_ri_ ', 'extensions=SVE '];
T[' INDEX VREG={REG:i64=$Zd VDS=D} REG=$Xn IMM:s8=$imm  ', 'size=0b11 ', 'index_z_ri_ ', 'extensions=SVE '];



# sve_int_index_rr: SVE index generation (register start, register increment).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b010011 Rn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_index_rr', 'A64');

DEF Wm => GPR32('Wm', 'Rm', 'read')   ;
DEF Wn => GPR32('Wn', 'Rn', 'read')   ;
DEF Xm => GPR64('Xm', 'Rm', 'read')   ;
DEF Xn => GPR64('Xn', 'Rn', 'read')   ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;

T[' INDEX VREG={REG:i8=$Zd   VDS=B} REG=$Wn REG=$Wm ', 'size=0b00 ', 'index_z_rr_ ', 'extensions=SVE '];
T[' INDEX VREG={REG:i16=$Zd VDS=H} REG=$Wn REG=$Wm  ', 'size=0b01 ', 'index_z_rr_ ', 'extensions=SVE '];
T[' INDEX VREG={REG:i32=$Zd VDS=S} REG=$Wn REG=$Wm  ', 'size=0b10 ', 'index_z_rr_ ', 'extensions=SVE '];
T[' INDEX VREG={REG:i64=$Zd VDS=D} REG=$Xn REG=$Xm  ', 'size=0b11 ', 'index_z_rr_ ', 'extensions=SVE '];



# sve_int_log_imm: SVE bitwise logical with immediate (unpredicated).
# A64: ig0=0b00000101 opc:u!=0b11 ig1=0b0000 imm13:u=0bxxxxxxxxxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_log_imm', 'A64');

DEF Zdn  => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF aimm => IMM('aimm', 'imm13')               ;

T[' AND VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zdn VDS=S} IMM:i32=$aimm  ', 'imm13=0b0xxxxxx0xxxxx ', 'and_z_zi_     ', 'extensions=SVE '];
T[' AND VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zdn VDS=H} IMM:i16=$aimm  ', 'imm13=0b0xxxxxx10xxxx ', 'and_z_zi_     ', 'extensions=SVE '];
T[' AND VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx110xxx ', 'and_z_zi_     ', 'extensions=SVE '];
T[' AND VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx1110xx ', 'and_z_zi_     ', 'extensions=SVE '];
T[' AND VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx11110x ', 'and_z_zi_     ', 'extensions=SVE '];
T[' AND VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} IMM:i64=$aimm  ', 'imm13=0b1xxxxxxxxxxxx ', 'and_z_zi_     ', 'extensions=SVE '];
T[' EOR VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zdn VDS=S} IMM:i32=$aimm  ', 'imm13=0b0xxxxxx0xxxxx ', 'eor_z_zi_     ', 'extensions=SVE '];
T[' EOR VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zdn VDS=H} IMM:i16=$aimm  ', 'imm13=0b0xxxxxx10xxxx ', 'eor_z_zi_     ', 'extensions=SVE '];
T[' EOR VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx110xxx ', 'eor_z_zi_     ', 'extensions=SVE '];
T[' EOR VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx1110xx ', 'eor_z_zi_     ', 'extensions=SVE '];
T[' EOR VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx11110x ', 'eor_z_zi_     ', 'extensions=SVE '];
T[' EOR VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} IMM:i64=$aimm  ', 'imm13=0b1xxxxxxxxxxxx ', 'eor_z_zi_     ', 'extensions=SVE '];
T[' ORR VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zdn VDS=S} IMM:i32=$aimm  ', 'imm13=0b0xxxxxx0xxxxx ', 'orr_z_zi_     ', 'extensions=SVE '];
T[' ORR VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zdn VDS=H} IMM:i16=$aimm  ', 'imm13=0b0xxxxxx10xxxx ', 'orr_z_zi_     ', 'extensions=SVE '];
T[' ORR VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx110xxx ', 'orr_z_zi_     ', 'extensions=SVE '];
T[' ORR VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx1110xx ', 'orr_z_zi_     ', 'extensions=SVE '];
T[' ORR VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx11110x ', 'orr_z_zi_     ', 'extensions=SVE '];
T[' ORR VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} IMM:i64=$aimm  ', 'imm13=0b1xxxxxxxxxxxx ', 'orr_z_zi_     ', 'extensions=SVE '];
T[' BIC VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zdn VDS=S} IMM:i32=$aimm  ', 'imm13=0b0xxxxxx0xxxxx ', 'BIC_and_z_zi_ ', 'extensions=SVE '];
T[' BIC VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zdn VDS=H} IMM:i16=$aimm  ', 'imm13=0b0xxxxxx10xxxx ', 'BIC_and_z_zi_ ', 'extensions=SVE '];
T[' BIC VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx110xxx ', 'BIC_and_z_zi_ ', 'extensions=SVE '];
T[' BIC VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx1110xx ', 'BIC_and_z_zi_ ', 'extensions=SVE '];
T[' BIC VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx11110x ', 'BIC_and_z_zi_ ', 'extensions=SVE '];
T[' BIC VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} IMM:i64=$aimm  ', 'imm13=0b1xxxxxxxxxxxx ', 'BIC_and_z_zi_ ', 'extensions=SVE '];
T[' EON VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zdn VDS=S} IMM:i32=$aimm  ', 'imm13=0b0xxxxxx0xxxxx ', 'EON_eor_z_zi_ ', 'extensions=SVE '];
T[' EON VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zdn VDS=H} IMM:i16=$aimm  ', 'imm13=0b0xxxxxx10xxxx ', 'EON_eor_z_zi_ ', 'extensions=SVE '];
T[' EON VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx110xxx ', 'EON_eor_z_zi_ ', 'extensions=SVE '];
T[' EON VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx1110xx ', 'EON_eor_z_zi_ ', 'extensions=SVE '];
T[' EON VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx11110x ', 'EON_eor_z_zi_ ', 'extensions=SVE '];
T[' EON VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} IMM:i64=$aimm  ', 'imm13=0b1xxxxxxxxxxxx ', 'EON_eor_z_zi_ ', 'extensions=SVE '];
T[' ORN VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zdn VDS=S} IMM:i32=$aimm  ', 'imm13=0b0xxxxxx0xxxxx ', 'ORN_orr_z_zi_ ', 'extensions=SVE '];
T[' ORN VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zdn VDS=H} IMM:i16=$aimm  ', 'imm13=0b0xxxxxx10xxxx ', 'ORN_orr_z_zi_ ', 'extensions=SVE '];
T[' ORN VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx110xxx ', 'ORN_orr_z_zi_ ', 'extensions=SVE '];
T[' ORN VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx1110xx ', 'ORN_orr_z_zi_ ', 'extensions=SVE '];
T[' ORN VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} IMM:i8=$aimm ', 'imm13=0b0xxxxxx11110x ', 'ORN_orr_z_zi_ ', 'extensions=SVE '];
T[' ORN VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} IMM:i64=$aimm  ', 'imm13=0b1xxxxxxxxxxxx ', 'ORN_orr_z_zi_ ', 'extensions=SVE '];



# sve_int_mladdsub_vvv_pred: SVE integer multiply-add writing multiplicand (predicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b11 op:u=0bx Pg:u=0bxxx Za:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_mladdsub_vvv_pred', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Za  => SVEREG('Za', 'Za', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' MAD VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zm   VDS=B} VREG={REG:i8=$Za   VDS=B} ', 'size=0b00 ', 'mad_z_p_zzz_ ', 'extensions=SVE '];
T[' MAD VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zm VDS=H} VREG={REG:i16=$Za VDS=H}    ', 'size=0b01 ', 'mad_z_p_zzz_ ', 'extensions=SVE '];
T[' MAD VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zm VDS=S} VREG={REG:i32=$Za VDS=S}    ', 'size=0b10 ', 'mad_z_p_zzz_ ', 'extensions=SVE '];
T[' MAD VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zm VDS=D} VREG={REG:i64=$Za VDS=D}    ', 'size=0b11 ', 'mad_z_p_zzz_ ', 'extensions=SVE '];
T[' MSB VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zm   VDS=B} VREG={REG:i8=$Za   VDS=B} ', 'size=0b00 ', 'msb_z_p_zzz_ ', 'extensions=SVE '];
T[' MSB VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zm VDS=H} VREG={REG:i16=$Za VDS=H}    ', 'size=0b01 ', 'msb_z_p_zzz_ ', 'extensions=SVE '];
T[' MSB VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zm VDS=S} VREG={REG:i32=$Za VDS=S}    ', 'size=0b10 ', 'msb_z_p_zzz_ ', 'extensions=SVE '];
T[' MSB VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zm VDS=D} VREG={REG:i64=$Za VDS=D}    ', 'size=0b11 ', 'msb_z_p_zzz_ ', 'extensions=SVE '];



# sve_int_mlas_vvv_pred: SVE integer multiply-accumulate writing addend (predicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b01 op:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_int_mlas_vvv_pred', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' MLA VREG={REG:i8=$Zda   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'mla_z_p_zzz_ ', 'extensions=SVE '];
T[' MLA VREG={REG:i16=$Zda VDS=H} PREG:M=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'mla_z_p_zzz_ ', 'extensions=SVE '];
T[' MLA VREG={REG:i32=$Zda VDS=S} PREG:M=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'mla_z_p_zzz_ ', 'extensions=SVE '];
T[' MLA VREG={REG:i64=$Zda VDS=D} PREG:M=$Pg VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'mla_z_p_zzz_ ', 'extensions=SVE '];
T[' MLS VREG={REG:i8=$Zda   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'mls_z_p_zzz_ ', 'extensions=SVE '];
T[' MLS VREG={REG:i16=$Zda VDS=H} PREG:M=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'mls_z_p_zzz_ ', 'extensions=SVE '];
T[' MLS VREG={REG:i32=$Zda VDS=S} PREG:M=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'mls_z_p_zzz_ ', 'extensions=SVE '];
T[' MLS VREG={REG:i64=$Zda VDS=D} PREG:M=$Pg VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'mls_z_p_zzz_ ', 'extensions=SVE '];



# sve_int_movprfx_pred: SVE constructive prefix (predicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b010 opc:u=0bxx M:u=0bx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_movprfx_pred', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' MOVPRFX VREG={REG:i8=$Zd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 M=0b0 ', 'movprfx_z_p_z_ ', 'extensions=SVE '];
T[' MOVPRFX VREG={REG:i8=$Zd   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 M=0b1 ', 'movprfx_z_p_z_ ', 'extensions=SVE '];
T[' MOVPRFX VREG={REG:i16=$Zd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H}   ', 'size=0b01 M=0b0 ', 'movprfx_z_p_z_ ', 'extensions=SVE '];
T[' MOVPRFX VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg VREG={REG:i16=$Zn VDS=H}   ', 'size=0b01 M=0b1 ', 'movprfx_z_p_z_ ', 'extensions=SVE '];
T[' MOVPRFX VREG={REG:i32=$Zd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S}   ', 'size=0b10 M=0b0 ', 'movprfx_z_p_z_ ', 'extensions=SVE '];
T[' MOVPRFX VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg VREG={REG:i32=$Zn VDS=S}   ', 'size=0b10 M=0b1 ', 'movprfx_z_p_z_ ', 'extensions=SVE '];
T[' MOVPRFX VREG={REG:i64=$Zd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D}   ', 'size=0b11 M=0b0 ', 'movprfx_z_p_z_ ', 'extensions=SVE '];
T[' MOVPRFX VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg VREG={REG:i64=$Zn VDS=D}   ', 'size=0b11 M=0b1 ', 'movprfx_z_p_z_ ', 'extensions=SVE '];



# sve_int_mul_b: SVE2 integer multiply vectors (unpredicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0110 opc:u=0bxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_mul_b', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' MUL   VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'mul_z_zz_   ', 'extensions=SVE2 '];
T[' MUL   VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'mul_z_zz_   ', 'extensions=SVE2 '];
T[' MUL   VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'mul_z_zz_   ', 'extensions=SVE2 '];
T[' MUL   VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'mul_z_zz_   ', 'extensions=SVE2 '];
T[' PMUL  VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'NONE      ', 'pmul_z_zz_  ', 'extensions=SVE2 '];
T[' SMULH VREG={REG:s8=$Zd   VDS=B} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'smulh_z_zz_ ', 'extensions=SVE2 '];
T[' SMULH VREG={REG:s16=$Zd VDS=H} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'smulh_z_zz_ ', 'extensions=SVE2 '];
T[' SMULH VREG={REG:s32=$Zd VDS=S} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'smulh_z_zz_ ', 'extensions=SVE2 '];
T[' SMULH VREG={REG:s64=$Zd VDS=D} VREG={REG:s64=$Zn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'smulh_z_zz_ ', 'extensions=SVE2 '];
T[' UMULH VREG={REG:u8=$Zd   VDS=B} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'umulh_z_zz_ ', 'extensions=SVE2 '];
T[' UMULH VREG={REG:u16=$Zd VDS=H} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'umulh_z_zz_ ', 'extensions=SVE2 '];
T[' UMULH VREG={REG:u32=$Zd VDS=S} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'umulh_z_zz_ ', 'extensions=SVE2 '];
T[' UMULH VREG={REG:u64=$Zd VDS=D} VREG={REG:u64=$Zn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'umulh_z_zz_ ', 'extensions=SVE2 '];



# sve_int_pcount_pred: SVE predicate count.
# A64: ig0=0b00100101 size:u=0bxx ig1=0b100 opc:u=0bxxx ig2=0b10 Pg:u=0bxxxx o2:u=0bx Pn:u=0bxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'sve_int_pcount_pred', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read') ;
DEF Pn => PRDREG('Pn', 'Pn', 'read') ;
DEF Xd => GPR64('Xd', 'Rd', 'write') ;

T[' CNTP REG=$Xd REG=$Pg VREG={REG:i8=$Pn   VDS=B} ', 'size=0b00 ', 'cntp_r_p_p_ ', 'extensions=SVE '];
T[' CNTP REG=$Xd REG=$Pg VREG={REG:i16=$Pn VDS=H}  ', 'size=0b01 ', 'cntp_r_p_p_ ', 'extensions=SVE '];
T[' CNTP REG=$Xd REG=$Pg VREG={REG:i32=$Pn VDS=S}  ', 'size=0b10 ', 'cntp_r_p_p_ ', 'extensions=SVE '];
T[' CNTP REG=$Xd REG=$Pg VREG={REG:i64=$Pn VDS=D}  ', 'size=0b11 ', 'cntp_r_p_p_ ', 'extensions=SVE '];



# sve_int_perm_bin_long_perm_zz: SVE permute vector segments.
# A64: ig0=0b000001011 op:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b000 opc2:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_bin_long_perm_zz', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' TRN1 VREG={REG:i128=$Zd VDS=Q} VREG={REG:i128=$Zn VDS=Q} VREG={REG:i128=$Zm VDS=Q} ', 'NONE ', 'trn1_z_zz_q ', 'extensions=SVEFP64MATMUL '];
T[' TRN2 VREG={REG:i128=$Zd VDS=Q} VREG={REG:i128=$Zn VDS=Q} VREG={REG:i128=$Zm VDS=Q} ', 'NONE ', 'trn2_z_zz_q ', 'extensions=SVEFP64MATMUL '];
T[' UZP1 VREG={REG:i128=$Zd VDS=Q} VREG={REG:i128=$Zn VDS=Q} VREG={REG:i128=$Zm VDS=Q} ', 'NONE ', 'uzp1_z_zz_q ', 'extensions=SVEFP64MATMUL '];
T[' UZP2 VREG={REG:i128=$Zd VDS=Q} VREG={REG:i128=$Zn VDS=Q} VREG={REG:i128=$Zm VDS=Q} ', 'NONE ', 'uzp2_z_zz_q ', 'extensions=SVEFP64MATMUL '];
T[' ZIP2 VREG={REG:i128=$Zd VDS=Q} VREG={REG:i128=$Zn VDS=Q} VREG={REG:i128=$Zm VDS=Q} ', 'NONE ', 'zip2_z_zz_q ', 'extensions=SVEFP64MATMUL '];
T[' ZIP1 VREG={REG:i128=$Zd VDS=Q} VREG={REG:i128=$Zn VDS=Q} VREG={REG:i128=$Zm VDS=Q} ', 'NONE ', 'zip1_z_zz_q ', 'extensions=SVEFP64MATMUL '];



# sve_int_perm_bin_perm_pp: SVE permute predicate elements.
# A64: ig0=0b00000101 size:u=0bxx ig1=0b10 Pm:u=0bxxxx ig2=0b010 opc:u=0bxx H:u=0bx ig3=0b0 Pn:u=0bxxxx ig4=0b0 Pd:u=0bxxxx
HINT('ICLASS', 'sve_int_perm_bin_perm_pp', 'A64');

DEF Pd => PRDREG('Pd', 'Pd', 'write') ;
DEF Pm => PRDREG('Pm', 'Pm', 'read')  ;
DEF Pn => PRDREG('Pn', 'Pn', 'read')  ;

T[' TRN1 VREG={REG:i8=$Pd   VDS=B} VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'size=0b00 ', 'trn1_p_pp_ ', 'extensions=SVE '];
T[' TRN1 VREG={REG:i16=$Pd VDS=H} VREG={REG:i16=$Pn VDS=H} VREG={REG:i16=$Pm VDS=H}    ', 'size=0b01 ', 'trn1_p_pp_ ', 'extensions=SVE '];
T[' TRN1 VREG={REG:i32=$Pd VDS=S} VREG={REG:i32=$Pn VDS=S} VREG={REG:i32=$Pm VDS=S}    ', 'size=0b10 ', 'trn1_p_pp_ ', 'extensions=SVE '];
T[' TRN1 VREG={REG:i64=$Pd VDS=D} VREG={REG:i64=$Pn VDS=D} VREG={REG:i64=$Pm VDS=D}    ', 'size=0b11 ', 'trn1_p_pp_ ', 'extensions=SVE '];
T[' TRN2 VREG={REG:i8=$Pd   VDS=B} VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'size=0b00 ', 'trn2_p_pp_ ', 'extensions=SVE '];
T[' TRN2 VREG={REG:i16=$Pd VDS=H} VREG={REG:i16=$Pn VDS=H} VREG={REG:i16=$Pm VDS=H}    ', 'size=0b01 ', 'trn2_p_pp_ ', 'extensions=SVE '];
T[' TRN2 VREG={REG:i32=$Pd VDS=S} VREG={REG:i32=$Pn VDS=S} VREG={REG:i32=$Pm VDS=S}    ', 'size=0b10 ', 'trn2_p_pp_ ', 'extensions=SVE '];
T[' TRN2 VREG={REG:i64=$Pd VDS=D} VREG={REG:i64=$Pn VDS=D} VREG={REG:i64=$Pm VDS=D}    ', 'size=0b11 ', 'trn2_p_pp_ ', 'extensions=SVE '];
T[' UZP1 VREG={REG:i8=$Pd   VDS=B} VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'size=0b00 ', 'uzp1_p_pp_ ', 'extensions=SVE '];
T[' UZP1 VREG={REG:i16=$Pd VDS=H} VREG={REG:i16=$Pn VDS=H} VREG={REG:i16=$Pm VDS=H}    ', 'size=0b01 ', 'uzp1_p_pp_ ', 'extensions=SVE '];
T[' UZP1 VREG={REG:i32=$Pd VDS=S} VREG={REG:i32=$Pn VDS=S} VREG={REG:i32=$Pm VDS=S}    ', 'size=0b10 ', 'uzp1_p_pp_ ', 'extensions=SVE '];
T[' UZP1 VREG={REG:i64=$Pd VDS=D} VREG={REG:i64=$Pn VDS=D} VREG={REG:i64=$Pm VDS=D}    ', 'size=0b11 ', 'uzp1_p_pp_ ', 'extensions=SVE '];
T[' UZP2 VREG={REG:i8=$Pd   VDS=B} VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'size=0b00 ', 'uzp2_p_pp_ ', 'extensions=SVE '];
T[' UZP2 VREG={REG:i16=$Pd VDS=H} VREG={REG:i16=$Pn VDS=H} VREG={REG:i16=$Pm VDS=H}    ', 'size=0b01 ', 'uzp2_p_pp_ ', 'extensions=SVE '];
T[' UZP2 VREG={REG:i32=$Pd VDS=S} VREG={REG:i32=$Pn VDS=S} VREG={REG:i32=$Pm VDS=S}    ', 'size=0b10 ', 'uzp2_p_pp_ ', 'extensions=SVE '];
T[' UZP2 VREG={REG:i64=$Pd VDS=D} VREG={REG:i64=$Pn VDS=D} VREG={REG:i64=$Pm VDS=D}    ', 'size=0b11 ', 'uzp2_p_pp_ ', 'extensions=SVE '];
T[' ZIP2 VREG={REG:i8=$Pd   VDS=B} VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'size=0b00 ', 'zip2_p_pp_ ', 'extensions=SVE '];
T[' ZIP2 VREG={REG:i16=$Pd VDS=H} VREG={REG:i16=$Pn VDS=H} VREG={REG:i16=$Pm VDS=H}    ', 'size=0b01 ', 'zip2_p_pp_ ', 'extensions=SVE '];
T[' ZIP2 VREG={REG:i32=$Pd VDS=S} VREG={REG:i32=$Pn VDS=S} VREG={REG:i32=$Pm VDS=S}    ', 'size=0b10 ', 'zip2_p_pp_ ', 'extensions=SVE '];
T[' ZIP2 VREG={REG:i64=$Pd VDS=D} VREG={REG:i64=$Pn VDS=D} VREG={REG:i64=$Pm VDS=D}    ', 'size=0b11 ', 'zip2_p_pp_ ', 'extensions=SVE '];
T[' ZIP1 VREG={REG:i8=$Pd   VDS=B} VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'size=0b00 ', 'zip1_p_pp_ ', 'extensions=SVE '];
T[' ZIP1 VREG={REG:i16=$Pd VDS=H} VREG={REG:i16=$Pn VDS=H} VREG={REG:i16=$Pm VDS=H}    ', 'size=0b01 ', 'zip1_p_pp_ ', 'extensions=SVE '];
T[' ZIP1 VREG={REG:i32=$Pd VDS=S} VREG={REG:i32=$Pn VDS=S} VREG={REG:i32=$Pm VDS=S}    ', 'size=0b10 ', 'zip1_p_pp_ ', 'extensions=SVE '];
T[' ZIP1 VREG={REG:i64=$Pd VDS=D} VREG={REG:i64=$Pn VDS=D} VREG={REG:i64=$Pm VDS=D}    ', 'size=0b11 ', 'zip1_p_pp_ ', 'extensions=SVE '];



# sve_int_perm_bin_perm_zz: SVE permute vector elements.
# A64: ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 opc:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_bin_perm_zz', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' TRN1 VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'trn1_z_zz_ ', 'extensions=SVE '];
T[' TRN1 VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'trn1_z_zz_ ', 'extensions=SVE '];
T[' TRN1 VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'trn1_z_zz_ ', 'extensions=SVE '];
T[' TRN1 VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'trn1_z_zz_ ', 'extensions=SVE '];
T[' TRN2 VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'trn2_z_zz_ ', 'extensions=SVE '];
T[' TRN2 VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'trn2_z_zz_ ', 'extensions=SVE '];
T[' TRN2 VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'trn2_z_zz_ ', 'extensions=SVE '];
T[' TRN2 VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'trn2_z_zz_ ', 'extensions=SVE '];
T[' UZP1 VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'uzp1_z_zz_ ', 'extensions=SVE '];
T[' UZP1 VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'uzp1_z_zz_ ', 'extensions=SVE '];
T[' UZP1 VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'uzp1_z_zz_ ', 'extensions=SVE '];
T[' UZP1 VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'uzp1_z_zz_ ', 'extensions=SVE '];
T[' UZP2 VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'uzp2_z_zz_ ', 'extensions=SVE '];
T[' UZP2 VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'uzp2_z_zz_ ', 'extensions=SVE '];
T[' UZP2 VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'uzp2_z_zz_ ', 'extensions=SVE '];
T[' UZP2 VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'uzp2_z_zz_ ', 'extensions=SVE '];
T[' ZIP2 VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'zip2_z_zz_ ', 'extensions=SVE '];
T[' ZIP2 VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'zip2_z_zz_ ', 'extensions=SVE '];
T[' ZIP2 VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'zip2_z_zz_ ', 'extensions=SVE '];
T[' ZIP2 VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'zip2_z_zz_ ', 'extensions=SVE '];
T[' ZIP1 VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'zip1_z_zz_ ', 'extensions=SVE '];
T[' ZIP1 VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'zip1_z_zz_ ', 'extensions=SVE '];
T[' ZIP1 VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'zip1_z_zz_ ', 'extensions=SVE '];
T[' ZIP1 VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'zip1_z_zz_ ', 'extensions=SVE '];



# sve_int_perm_clast_rz: SVE conditionally extract element to general register.
# A64: ig0=0b00000101 size:u=0bxx ig1=0b11000 B:u=0bx ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Rdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_clast_rz', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')        ;
DEF Wdn => GPR32('Wdn', 'Rdn', 'read|write') ;
DEF Xdn => GPR64('Xdn', 'Rdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')        ;

T[' CLASTA REG=$Wdn REG=$Pg REG=$Wdn VREG={REG:i16=$Zm VDS=H}  ', 'size=0b01 ', 'clasta_r_p_z_ ', 'extensions=SVE '];
T[' CLASTA REG=$Wdn REG=$Pg REG=$Wdn VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'clasta_r_p_z_ ', 'extensions=SVE '];
T[' CLASTA REG=$Wdn REG=$Pg REG=$Wdn VREG={REG:i32=$Zm VDS=S}  ', 'size=0b10 ', 'clasta_r_p_z_ ', 'extensions=SVE '];
T[' CLASTA REG=$Xdn REG=$Pg REG=$Xdn VREG={REG:i64=$Zm VDS=D}  ', 'size=0b11 ', 'clasta_r_p_z_ ', 'extensions=SVE '];
T[' CLASTB REG=$Wdn REG=$Pg REG=$Wdn VREG={REG:i16=$Zm VDS=H}  ', 'size=0b01 ', 'clastb_r_p_z_ ', 'extensions=SVE '];
T[' CLASTB REG=$Wdn REG=$Pg REG=$Wdn VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'clastb_r_p_z_ ', 'extensions=SVE '];
T[' CLASTB REG=$Wdn REG=$Pg REG=$Wdn VREG={REG:i32=$Zm VDS=S}  ', 'size=0b10 ', 'clastb_r_p_z_ ', 'extensions=SVE '];
T[' CLASTB REG=$Xdn REG=$Pg REG=$Xdn VREG={REG:i64=$Zm VDS=D}  ', 'size=0b11 ', 'clastb_r_p_z_ ', 'extensions=SVE '];



# sve_int_perm_clast_vz: SVE conditionally extract element to SIMD&FP scalar.
# A64: ig0=0b00000101 size:u=0bxx ig1=0b10101 B:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Vdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_clast_vz', 'A64');

DEF Bdn => SIMD8('Bdn', 'Vdn', 'read|write')  ;
DEF Ddn => SIMD64('Ddn', 'Vdn', 'read|write') ;
DEF Hdn => SIMD16('Hdn', 'Vdn', 'read|write') ;
DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Sdn => SIMD32('Sdn', 'Vdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' CLASTA REG:i8=$Bdn  REG=$Pg REG:i8=$Bdn  VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'clasta_v_p_z_ ', 'extensions=SVE '];
T[' CLASTA REG:i16=$Hdn REG=$Pg REG:i16=$Hdn VREG={REG:i16=$Zm VDS=H}  ', 'size=0b01 ', 'clasta_v_p_z_ ', 'extensions=SVE '];
T[' CLASTA REG:i32=$Sdn REG=$Pg REG:i32=$Sdn VREG={REG:i32=$Zm VDS=S}  ', 'size=0b10 ', 'clasta_v_p_z_ ', 'extensions=SVE '];
T[' CLASTA REG:i64=$Ddn REG=$Pg REG:i64=$Ddn VREG={REG:i64=$Zm VDS=D}  ', 'size=0b11 ', 'clasta_v_p_z_ ', 'extensions=SVE '];
T[' CLASTB REG:i8=$Bdn  REG=$Pg REG:i8=$Bdn  VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'clastb_v_p_z_ ', 'extensions=SVE '];
T[' CLASTB REG:i16=$Hdn REG=$Pg REG:i16=$Hdn VREG={REG:i16=$Zm VDS=H}  ', 'size=0b01 ', 'clastb_v_p_z_ ', 'extensions=SVE '];
T[' CLASTB REG:i32=$Sdn REG=$Pg REG:i32=$Sdn VREG={REG:i32=$Zm VDS=S}  ', 'size=0b10 ', 'clastb_v_p_z_ ', 'extensions=SVE '];
T[' CLASTB REG:i64=$Ddn REG=$Pg REG:i64=$Ddn VREG={REG:i64=$Zm VDS=D}  ', 'size=0b11 ', 'clastb_v_p_z_ ', 'extensions=SVE '];



# sve_int_perm_clast_zz: SVE conditionally broadcast element to vector.
# A64: ig0=0b00000101 size:u=0bxx ig1=0b10100 B:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_clast_zz', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' CLASTA VREG={REG:i8=$Zdn   VDS=B} REG=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'clasta_z_p_zz_ ', 'extensions=SVE '];
T[' CLASTA VREG={REG:i16=$Zdn VDS=H} REG=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'clasta_z_p_zz_ ', 'extensions=SVE '];
T[' CLASTA VREG={REG:i32=$Zdn VDS=S} REG=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'clasta_z_p_zz_ ', 'extensions=SVE '];
T[' CLASTA VREG={REG:i64=$Zdn VDS=D} REG=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'clasta_z_p_zz_ ', 'extensions=SVE '];
T[' CLASTB VREG={REG:i8=$Zdn   VDS=B} REG=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'clastb_z_p_zz_ ', 'extensions=SVE '];
T[' CLASTB VREG={REG:i16=$Zdn VDS=H} REG=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'clastb_z_p_zz_ ', 'extensions=SVE '];
T[' CLASTB VREG={REG:i32=$Zdn VDS=S} REG=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'clastb_z_p_zz_ ', 'extensions=SVE '];
T[' CLASTB VREG={REG:i64=$Zdn VDS=D} REG=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'clastb_z_p_zz_ ', 'extensions=SVE '];



# sve_int_perm_compact: SVE compress active elements.
# A64: ig0=0b00000101 size:u=0bxx ig1=0b100001100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_compact', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' COMPACT VREG={REG:i32=$Zd VDS=S} REG=$Pg VREG={REG:i32=$Zn VDS=S} ', 'size=0bx0 ', 'compact_z_p_z_ ', 'extensions=SVE '];
T[' COMPACT VREG={REG:i64=$Zd VDS=D} REG=$Pg VREG={REG:i64=$Zn VDS=D} ', 'size=0bx1 ', 'compact_z_p_z_ ', 'extensions=SVE '];



# sve_int_perm_cpy_r: SVE copy general register to vector (predicated).
# A64: ig0=0b00000101 size:u=0bxx ig1=0b101000101 Pg:u=0bxxx Rn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_cpy_r', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')   ;
DEF WnSP => GPR32('Wn', 'Rn', 'read|SP') ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP') ;
DEF Zd   => SVEREG('Zd', 'Zd', 'write')  ;

T[' CPY VREG={REG:i8=$Zd   VDS=B} PREG:M=$Pg REG=$WnSP ', 'size=0b00 ', 'cpy_z_p_r_     ', 'extensions=SVE '];
T[' CPY VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg REG=$WnSP  ', 'size=0b01 ', 'cpy_z_p_r_     ', 'extensions=SVE '];
T[' CPY VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg REG=$WnSP  ', 'size=0b10 ', 'cpy_z_p_r_     ', 'extensions=SVE '];
T[' CPY VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg REG=$XnSP  ', 'size=0b11 ', 'cpy_z_p_r_     ', 'extensions=SVE '];
T[' MOV VREG={REG:i8=$Zd   VDS=B} PREG:M=$Pg REG=$WnSP ', 'size=0b00 ', 'MOV_cpy_z_p_r_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg REG=$WnSP  ', 'size=0b01 ', 'MOV_cpy_z_p_r_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg REG=$WnSP  ', 'size=0b10 ', 'MOV_cpy_z_p_r_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg REG=$XnSP  ', 'size=0b11 ', 'MOV_cpy_z_p_r_ ', 'extensions=SVE '];



# sve_int_perm_cpy_v: SVE copy SIMD&FP scalar register to vector (predicated).
# A64: ig0=0b00000101 size:u=0bxx ig1=0b100000100 Pg:u=0bxxx Vn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_cpy_v', 'A64');

DEF Bn => SIMD8('Bn', 'Vn', 'read')   ;
DEF Dn => SIMD64('Dn', 'Vn', 'read')  ;
DEF Hn => SIMD16('Hn', 'Vn', 'read')  ;
DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Sn => SIMD32('Sn', 'Vn', 'read')  ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;

T[' CPY VREG={REG:i8=$Zd   VDS=B} PREG:M=$Pg REG:i8=$Bn  ', 'size=0b00 ', 'cpy_z_p_v_     ', 'extensions=SVE '];
T[' CPY VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg REG:i16=$Hn  ', 'size=0b01 ', 'cpy_z_p_v_     ', 'extensions=SVE '];
T[' CPY VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg REG:i32=$Sn  ', 'size=0b10 ', 'cpy_z_p_v_     ', 'extensions=SVE '];
T[' CPY VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg REG:i64=$Dn  ', 'size=0b11 ', 'cpy_z_p_v_     ', 'extensions=SVE '];
T[' MOV VREG={REG:i8=$Zd   VDS=B} PREG:M=$Pg REG:i8=$Bn  ', 'size=0b00 ', 'MOV_cpy_z_p_v_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg REG:i16=$Hn  ', 'size=0b01 ', 'MOV_cpy_z_p_v_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg REG:i32=$Sn  ', 'size=0b10 ', 'MOV_cpy_z_p_v_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg REG:i64=$Dn  ', 'size=0b11 ', 'MOV_cpy_z_p_v_ ', 'extensions=SVE '];



# sve_int_perm_dup_i: SVE broadcast indexed element.
# A64: ig0=0b00000101 imm2:u=0bxx ig1=0b1 tsz:u=0bxxxxx ig2=0b001000 Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_dup_i', 'A64');

DEF Bn => SIMD8('Bn', 'Zn', 'read')   ;
DEF Dn => SIMD64('Dn', 'Zn', 'read')  ;
DEF Hn => SIMD16('Hn', 'Zn', 'read')  ;
DEF Qn => SIMD128('Qn', 'Zn', 'read') ;
DEF Sn => SIMD32('Sn', 'Zn', 'read')  ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' DUP VREG={REG:i8=$Zd   VDS=B} EREG={REG:i8=$Zn   VDS=B IDX={ENCODEDIN=imm2:tsz}} ', 'tsz=0bxxxx1 ', 'dup_z_zi_     ', 'extensions=SVE '];
T[' DUP VREG={REG:i16=$Zd VDS=H} EREG={REG:i16=$Zn VDS=H IDX={ENCODEDIN=imm2:tsz}}   ', 'tsz=0bxxx10 ', 'dup_z_zi_     ', 'extensions=SVE '];
T[' DUP VREG={REG:i32=$Zd VDS=S} EREG={REG:i32=$Zn VDS=S IDX={ENCODEDIN=imm2:tsz}}   ', 'tsz=0bxx100 ', 'dup_z_zi_     ', 'extensions=SVE '];
T[' DUP VREG={REG:i64=$Zd VDS=D} EREG={REG:i64=$Zn VDS=D IDX={ENCODEDIN=imm2:tsz}}   ', 'tsz=0bx1000 ', 'dup_z_zi_     ', 'extensions=SVE '];
T[' DUP VREG={REG:i128=$Zd VDS=Q} EREG={REG:i128=$Zn VDS=Q IDX={ENCODEDIN=imm2:tsz}} ', 'tsz=0b10000 ', 'dup_z_zi_     ', 'extensions=SVE '];
T[' MOV VREG={REG:i8=$Zd   VDS=B} EREG={REG:i8=$Zn   VDS=B IDX={ENCODEDIN=imm2:tsz}} ', 'tsz=0bxxxx1 ', 'MOV_dup_z_zi_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i16=$Zd VDS=H} EREG={REG:i16=$Zn VDS=H IDX={ENCODEDIN=imm2:tsz}}   ', 'tsz=0bxxx10 ', 'MOV_dup_z_zi_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i32=$Zd VDS=S} EREG={REG:i32=$Zn VDS=S IDX={ENCODEDIN=imm2:tsz}}   ', 'tsz=0bxx100 ', 'MOV_dup_z_zi_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i64=$Zd VDS=D} EREG={REG:i64=$Zn VDS=D IDX={ENCODEDIN=imm2:tsz}}   ', 'tsz=0bx1000 ', 'MOV_dup_z_zi_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i128=$Zd VDS=Q} EREG={REG:i128=$Zn VDS=Q IDX={ENCODEDIN=imm2:tsz}} ', 'tsz=0b10000 ', 'MOV_dup_z_zi_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i8=$Zd   VDS=B} REG:i8=$Bn                                         ', 'tsz=0bxxxx1 ', 'MOV_dup_z_zi_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i16=$Zd VDS=H} REG:i16=$Hn                                         ', 'tsz=0bxxx10 ', 'MOV_dup_z_zi_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i32=$Zd VDS=S} REG:i32=$Sn                                         ', 'tsz=0bxx100 ', 'MOV_dup_z_zi_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i64=$Zd VDS=D} REG:i64=$Dn                                         ', 'tsz=0bx1000 ', 'MOV_dup_z_zi_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i128=$Zd VDS=Q} REG:i128=$Qn                                       ', 'tsz=0b10000 ', 'MOV_dup_z_zi_ ', 'extensions=SVE '];



# sve_int_perm_dup_r: SVE broadcast general register.
# A64: ig0=0b00000101 size:u=0bxx ig1=0b100000001110 Rn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_dup_r', 'A64');

DEF WnSP => GPR32('Wn', 'Rn', 'read|SP') ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP') ;
DEF Zd   => SVEREG('Zd', 'Zd', 'write')  ;

T[' DUP VREG={REG:i8=$Zd   VDS=B} REG=$WnSP ', 'size=0b00 ', 'dup_z_r_     ', 'extensions=SVE '];
T[' DUP VREG={REG:i16=$Zd VDS=H} REG=$WnSP  ', 'size=0b01 ', 'dup_z_r_     ', 'extensions=SVE '];
T[' DUP VREG={REG:i32=$Zd VDS=S} REG=$WnSP  ', 'size=0b10 ', 'dup_z_r_     ', 'extensions=SVE '];
T[' DUP VREG={REG:i64=$Zd VDS=D} REG=$XnSP  ', 'size=0b11 ', 'dup_z_r_     ', 'extensions=SVE '];
T[' MOV VREG={REG:i8=$Zd   VDS=B} REG=$WnSP ', 'size=0b00 ', 'MOV_dup_z_r_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i16=$Zd VDS=H} REG=$WnSP  ', 'size=0b01 ', 'MOV_dup_z_r_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i32=$Zd VDS=S} REG=$WnSP  ', 'size=0b10 ', 'MOV_dup_z_r_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i64=$Zd VDS=D} REG=$XnSP  ', 'size=0b11 ', 'MOV_dup_z_r_ ', 'extensions=SVE '];



# sve_int_perm_extract_i: SVE extract vector (immediate offset, destructive).
# A64: ig0=0b00000101001 imm8h:u=0bxxxxx ig1=0b000 imm8l:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_extract_i', 'A64');

DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write')           ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')                   ;
DEF imm => IMM('imm', 'imm8h:imm8l', range => (0, 255)) ;

T[' EXT VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} IMM:u8=$imm ', 'NONE ', 'ext_z_zi_des ', 'extensions=SVE '];



# sve_int_perm_insrs: SVE insert general register.
# A64: ig0=0b00000101 size:u=0bxx ig1=0b100100001110 Rm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_insrs', 'A64');

DEF Wm  => GPR32('Wm', 'Rm', 'read')          ;
DEF Xm  => GPR64('Xm', 'Rm', 'read')          ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;

T[' INSR VREG={REG:i8=$Zdn   VDS=B} REG=$Wm ', 'size=0b00 ', 'insr_z_r_ ', 'extensions=SVE '];
T[' INSR VREG={REG:i16=$Zdn VDS=H} REG=$Wm  ', 'size=0b01 ', 'insr_z_r_ ', 'extensions=SVE '];
T[' INSR VREG={REG:i32=$Zdn VDS=S} REG=$Wm  ', 'size=0b10 ', 'insr_z_r_ ', 'extensions=SVE '];
T[' INSR VREG={REG:i64=$Zdn VDS=D} REG=$Xm  ', 'size=0b11 ', 'insr_z_r_ ', 'extensions=SVE '];



# sve_int_perm_insrv: SVE insert SIMD&FP scalar register.
# A64: ig0=0b00000101 size:u=0bxx ig1=0b110100001110 Vm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_insrv', 'A64');

DEF Bm  => SIMD8('Bm', 'Vm', 'read')          ;
DEF Dm  => SIMD64('Dm', 'Vm', 'read')         ;
DEF Hm  => SIMD16('Hm', 'Vm', 'read')         ;
DEF Sm  => SIMD32('Sm', 'Vm', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;

T[' INSR VREG={REG:i8=$Zdn   VDS=B} REG:i8=$Bm  ', 'size=0b00 ', 'insr_z_v_ ', 'extensions=SVE '];
T[' INSR VREG={REG:i16=$Zdn VDS=H} REG:i16=$Hm  ', 'size=0b01 ', 'insr_z_v_ ', 'extensions=SVE '];
T[' INSR VREG={REG:i32=$Zdn VDS=S} REG:i32=$Sm  ', 'size=0b10 ', 'insr_z_v_ ', 'extensions=SVE '];
T[' INSR VREG={REG:i64=$Zdn VDS=D} REG:i64=$Dm  ', 'size=0b11 ', 'insr_z_v_ ', 'extensions=SVE '];



# sve_int_perm_last_r: SVE extract element to general register.
# A64: ig0=0b00000101 size:u=0bxx ig1=0b10000 B:u=0bx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_last_r', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read') ;
DEF Wd => GPR32('Wd', 'Rd', 'write') ;
DEF Xd => GPR64('Xd', 'Rd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read') ;

T[' LASTA REG=$Wd REG=$Pg VREG={REG:i16=$Zn VDS=H}  ', 'size=0b01 ', 'lasta_r_p_z_ ', 'extensions=SVE '];
T[' LASTA REG=$Wd REG=$Pg VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'lasta_r_p_z_ ', 'extensions=SVE '];
T[' LASTA REG=$Wd REG=$Pg VREG={REG:i32=$Zn VDS=S}  ', 'size=0b10 ', 'lasta_r_p_z_ ', 'extensions=SVE '];
T[' LASTA REG=$Xd REG=$Pg VREG={REG:i64=$Zn VDS=D}  ', 'size=0b11 ', 'lasta_r_p_z_ ', 'extensions=SVE '];
T[' LASTB REG=$Wd REG=$Pg VREG={REG:i16=$Zn VDS=H}  ', 'size=0b01 ', 'lastb_r_p_z_ ', 'extensions=SVE '];
T[' LASTB REG=$Wd REG=$Pg VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'lastb_r_p_z_ ', 'extensions=SVE '];
T[' LASTB REG=$Wd REG=$Pg VREG={REG:i32=$Zn VDS=S}  ', 'size=0b10 ', 'lastb_r_p_z_ ', 'extensions=SVE '];
T[' LASTB REG=$Xd REG=$Pg VREG={REG:i64=$Zn VDS=D}  ', 'size=0b11 ', 'lastb_r_p_z_ ', 'extensions=SVE '];



# sve_int_perm_last_v: SVE extract element to SIMD&FP scalar register.
# A64: ig0=0b00000101 size:u=0bxx ig1=0b10001 B:u=0bx ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_last_v', 'A64');

DEF Bd => SIMD8('Bd', 'Vd', 'write')  ;
DEF Dd => SIMD64('Dd', 'Vd', 'write') ;
DEF Hd => SIMD16('Hd', 'Vd', 'write') ;
DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Sd => SIMD32('Sd', 'Vd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' LASTA REG:i8=$Bd  REG=$Pg VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'lasta_v_p_z_ ', 'extensions=SVE '];
T[' LASTA REG:i16=$Hd REG=$Pg VREG={REG:i16=$Zn VDS=H}  ', 'size=0b01 ', 'lasta_v_p_z_ ', 'extensions=SVE '];
T[' LASTA REG:i32=$Sd REG=$Pg VREG={REG:i32=$Zn VDS=S}  ', 'size=0b10 ', 'lasta_v_p_z_ ', 'extensions=SVE '];
T[' LASTA REG:i64=$Dd REG=$Pg VREG={REG:i64=$Zn VDS=D}  ', 'size=0b11 ', 'lasta_v_p_z_ ', 'extensions=SVE '];
T[' LASTB REG:i8=$Bd  REG=$Pg VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'lastb_v_p_z_ ', 'extensions=SVE '];
T[' LASTB REG:i16=$Hd REG=$Pg VREG={REG:i16=$Zn VDS=H}  ', 'size=0b01 ', 'lastb_v_p_z_ ', 'extensions=SVE '];
T[' LASTB REG:i32=$Sd REG=$Pg VREG={REG:i32=$Zn VDS=S}  ', 'size=0b10 ', 'lastb_v_p_z_ ', 'extensions=SVE '];
T[' LASTB REG:i64=$Dd REG=$Pg VREG={REG:i64=$Zn VDS=D}  ', 'size=0b11 ', 'lastb_v_p_z_ ', 'extensions=SVE '];



# sve_int_perm_punpk: SVE unpack predicate elements.
# A64: ig0=0b000001010011000 H:u=0bx ig1=0b010000 ig2=0b0 Pn:u=0bxxxx ig3=0b0 Pd:u=0bxxxx
HINT('ICLASS', 'sve_int_perm_punpk', 'A64');

DEF Pd => PRDREG('Pd', 'Pd', 'write') ;
DEF Pn => PRDREG('Pn', 'Pn', 'read')  ;

T[' PUNPKHI VREG={REG:i16=$Pd VDS=H} VREG={REG:i8=$Pn   VDS=B} ', 'NONE ', 'punpkhi_p_p_ ', 'extensions=SVE '];
T[' PUNPKLO VREG={REG:i16=$Pd VDS=H} VREG={REG:i8=$Pn   VDS=B} ', 'NONE ', 'punpklo_p_p_ ', 'extensions=SVE '];



# sve_int_perm_rev: SVE reverse within elements.
# A64: ig0=0b00000101 size:u=0bxx ig1=0b1001 opc:u=0bxx ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_rev', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' RBIT VREG={REG:i8=$Zd   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'rbit_z_p_z_ ', 'extensions=SVE '];
T[' RBIT VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg VREG={REG:i16=$Zn VDS=H}   ', 'size=0b01 ', 'rbit_z_p_z_ ', 'extensions=SVE '];
T[' RBIT VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg VREG={REG:i32=$Zn VDS=S}   ', 'size=0b10 ', 'rbit_z_p_z_ ', 'extensions=SVE '];
T[' RBIT VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg VREG={REG:i64=$Zn VDS=D}   ', 'size=0b11 ', 'rbit_z_p_z_ ', 'extensions=SVE '];
T[' REVB VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg VREG={REG:i16=$Zn VDS=H}   ', 'size=0b01 ', 'revb_z_z_   ', 'extensions=SVE '];
T[' REVB VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg VREG={REG:i32=$Zn VDS=S}   ', 'size=0b10 ', 'revb_z_z_   ', 'extensions=SVE '];
T[' REVB VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg VREG={REG:i64=$Zn VDS=D}   ', 'size=0b11 ', 'revb_z_z_   ', 'extensions=SVE '];
T[' REVH VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg VREG={REG:i32=$Zn VDS=S}   ', 'size=0bx0 ', 'revh_z_z_   ', 'extensions=SVE '];
T[' REVH VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg VREG={REG:i64=$Zn VDS=D}   ', 'size=0bx1 ', 'revh_z_z_   ', 'extensions=SVE '];
T[' REVW VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg VREG={REG:i64=$Zn VDS=D}   ', 'NONE      ', 'revw_z_z_   ', 'extensions=SVE '];



# sve_int_perm_reverse_p: SVE reverse predicate elements.
# A64: ig0=0b00000101 size:u=0bxx ig1=0b110100010000 ig2=0b0 Pn:u=0bxxxx ig3=0b0 Pd:u=0bxxxx
HINT('ICLASS', 'sve_int_perm_reverse_p', 'A64');

DEF Pd => PRDREG('Pd', 'Pd', 'write') ;
DEF Pn => PRDREG('Pn', 'Pn', 'read')  ;

T[' REV VREG={REG:i8=$Pd   VDS=B} VREG={REG:i8=$Pn   VDS=B} ', 'size=0b00 ', 'rev_p_p_ ', 'extensions=SVE '];
T[' REV VREG={REG:i16=$Pd VDS=H} VREG={REG:i16=$Pn VDS=H}   ', 'size=0b01 ', 'rev_p_p_ ', 'extensions=SVE '];
T[' REV VREG={REG:i32=$Pd VDS=S} VREG={REG:i32=$Pn VDS=S}   ', 'size=0b10 ', 'rev_p_p_ ', 'extensions=SVE '];
T[' REV VREG={REG:i64=$Pd VDS=D} VREG={REG:i64=$Pn VDS=D}   ', 'size=0b11 ', 'rev_p_p_ ', 'extensions=SVE '];



# sve_int_perm_reverse_z: SVE reverse vector elements.
# A64: ig0=0b00000101 size:u=0bxx ig1=0b111000001110 Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_reverse_z', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' REV VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'rev_z_z_ ', 'extensions=SVE '];
T[' REV VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H}   ', 'size=0b01 ', 'rev_z_z_ ', 'extensions=SVE '];
T[' REV VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S}   ', 'size=0b10 ', 'rev_z_z_ ', 'extensions=SVE '];
T[' REV VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D}   ', 'size=0b11 ', 'rev_z_z_ ', 'extensions=SVE '];



# sve_int_perm_splice: SVE vector splice (destructive).
# A64: ig0=0b00000101 size:u=0bxx ig1=0b101100100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_splice', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' SPLICE VREG={REG:i8=$Zdn   VDS=B} REG=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'splice_z_p_zz_des ', 'extensions=SVE '];
T[' SPLICE VREG={REG:i16=$Zdn VDS=H} REG=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'splice_z_p_zz_des ', 'extensions=SVE '];
T[' SPLICE VREG={REG:i32=$Zdn VDS=S} REG=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'splice_z_p_zz_des ', 'extensions=SVE '];
T[' SPLICE VREG={REG:i64=$Zdn VDS=D} REG=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'splice_z_p_zz_des ', 'extensions=SVE '];



# sve_int_perm_tbl: SVE table lookup.
# A64: ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_tbl', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' TBL VREG={REG:i8=$Zd   VDS=B} VLIST={VBASE={REG:i8=$Zn   VDS=B} COUNT=1} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'tbl_z_zz_1 ', 'extensions=SVE '];
T[' TBL VREG={REG:i16=$Zd VDS=H} VLIST={VBASE={REG:i16=$Zn VDS=H} COUNT=1} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'tbl_z_zz_1 ', 'extensions=SVE '];
T[' TBL VREG={REG:i32=$Zd VDS=S} VLIST={VBASE={REG:i32=$Zn VDS=S} COUNT=1} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'tbl_z_zz_1 ', 'extensions=SVE '];
T[' TBL VREG={REG:i64=$Zd VDS=D} VLIST={VBASE={REG:i64=$Zn VDS=D} COUNT=1} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'tbl_z_zz_1 ', 'extensions=SVE '];



# sve_int_perm_tbl_3src: SVE table lookup (three sources).
# A64: ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b00101 op:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_tbl_3src', 'A64');

DEF Zd  => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn  => SVEREG('Zn1', 'Zn', 'read') ;
DEF Zn2 => SVEREG('Zn', 'Zn', 'read')  ;

T[' TBL VREG={REG:i8=$Zd   VDS=B} VLIST={VBASE={REG:i8=$Zn   VDS=B} COUNT=2} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'tbl_z_zz_2 ', 'extensions=SVE2 '];
T[' TBL VREG={REG:i16=$Zd VDS=H} VLIST={VBASE={REG:i16=$Zn VDS=H} COUNT=2} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'tbl_z_zz_2 ', 'extensions=SVE2 '];
T[' TBL VREG={REG:i32=$Zd VDS=S} VLIST={VBASE={REG:i32=$Zn VDS=S} COUNT=2} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'tbl_z_zz_2 ', 'extensions=SVE2 '];
T[' TBL VREG={REG:i64=$Zd VDS=D} VLIST={VBASE={REG:i64=$Zn VDS=D} COUNT=2} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'tbl_z_zz_2 ', 'extensions=SVE2 '];
T[' TBX VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn2   VDS=B} VREG={REG:i8=$Zm   VDS=B}                 ', 'size=0b00 ', 'tbx_z_zz_  ', 'extensions=SVE2 '];
T[' TBX VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn2 VDS=H} VREG={REG:i16=$Zm VDS=H}                    ', 'size=0b01 ', 'tbx_z_zz_  ', 'extensions=SVE2 '];
T[' TBX VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn2 VDS=S} VREG={REG:i32=$Zm VDS=S}                    ', 'size=0b10 ', 'tbx_z_zz_  ', 'extensions=SVE2 '];
T[' TBX VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn2 VDS=D} VREG={REG:i64=$Zm VDS=D}                    ', 'size=0b11 ', 'tbx_z_zz_  ', 'extensions=SVE2 '];



# sve_int_perm_unpk: SVE unpack vector elements.
# A64: ig0=0b00000101 size:u=0bxx ig1=0b1100 U:u=0bx H:u=0bx ig2=0b001110 Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_perm_unpk', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' SUNPKHI VREG={REG:s16=$Zd VDS=H} VREG={REG:s8=$Zn   VDS=B} ', 'size=0b01 ', 'sunpkhi_z_z_ ', 'extensions=SVE '];
T[' SUNPKHI VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H}  ', 'size=0b10 ', 'sunpkhi_z_z_ ', 'extensions=SVE '];
T[' SUNPKHI VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S}  ', 'size=0b11 ', 'sunpkhi_z_z_ ', 'extensions=SVE '];
T[' SUNPKLO VREG={REG:s16=$Zd VDS=H} VREG={REG:s8=$Zn   VDS=B} ', 'size=0b01 ', 'sunpklo_z_z_ ', 'extensions=SVE '];
T[' SUNPKLO VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H}  ', 'size=0b10 ', 'sunpklo_z_z_ ', 'extensions=SVE '];
T[' SUNPKLO VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S}  ', 'size=0b11 ', 'sunpklo_z_z_ ', 'extensions=SVE '];
T[' UUNPKHI VREG={REG:u16=$Zd VDS=H} VREG={REG:u8=$Zn   VDS=B} ', 'size=0b01 ', 'uunpkhi_z_z_ ', 'extensions=SVE '];
T[' UUNPKHI VREG={REG:u32=$Zd VDS=S} VREG={REG:u16=$Zn VDS=H}  ', 'size=0b10 ', 'uunpkhi_z_z_ ', 'extensions=SVE '];
T[' UUNPKHI VREG={REG:u64=$Zd VDS=D} VREG={REG:u32=$Zn VDS=S}  ', 'size=0b11 ', 'uunpkhi_z_z_ ', 'extensions=SVE '];
T[' UUNPKLO VREG={REG:u16=$Zd VDS=H} VREG={REG:u8=$Zn   VDS=B} ', 'size=0b01 ', 'uunpklo_z_z_ ', 'extensions=SVE '];
T[' UUNPKLO VREG={REG:u32=$Zd VDS=S} VREG={REG:u16=$Zn VDS=H}  ', 'size=0b10 ', 'uunpklo_z_z_ ', 'extensions=SVE '];
T[' UUNPKLO VREG={REG:u64=$Zd VDS=D} VREG={REG:u32=$Zn VDS=S}  ', 'size=0b11 ', 'uunpklo_z_z_ ', 'extensions=SVE '];



# sve_int_pfalse: SVE predicate zero.
# A64: ig0=0b00100101 op:u=0bx S:u=0bx ig1=0b011000111001 ig2=0b000000 Pd:u=0bxxxx
HINT('ICLASS', 'sve_int_pfalse', 'A64');

DEF Pd => PRDREG('Pd', 'Pd', 'write') ;

T[' PFALSE VREG={REG:i8=$Pd   VDS=B} ', 'NONE ', 'pfalse_p_ ', 'extensions=SVE '];



# sve_int_pfirst: SVE predicate first active.
# A64: ig0=0b00100101 op:u=0bx S:u=0bx ig1=0b011000110000 ig2=0b0 Pg:u=0bxxxx ig3=0b0 Pdn:u=0bxxxx
HINT('ICLASS', 'sve_int_pfirst', 'A64');

DEF Pdn => PRDREG('Pdn', 'Pdn', 'read|write') ;
DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;

T[' PFIRST VREG={REG:i8=$Pdn   VDS=B} REG=$Pg VREG={REG:i8=$Pdn   VDS=B} ', 'NONE ', 'pfirst_p_p_p_ ', 'extensions=SVE '];



# sve_int_pnext: SVE predicate next active.
# A64: ig0=0b00100101 size:u=0bxx ig1=0b011001110001 ig2=0b0 Pg:u=0bxxxx ig3=0b0 Pdn:u=0bxxxx
HINT('ICLASS', 'sve_int_pnext', 'A64');

DEF Pdn => PRDREG('Pdn', 'Pdn', 'read|write') ;
DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;

T[' PNEXT VREG={REG:i8=$Pdn   VDS=B} REG=$Pg VREG={REG:i8=$Pdn   VDS=B} ', 'size=0b00 ', 'pnext_p_p_p_ ', 'extensions=SVE '];
T[' PNEXT VREG={REG:i16=$Pdn VDS=H} REG=$Pg VREG={REG:i16=$Pdn VDS=H}   ', 'size=0b01 ', 'pnext_p_p_p_ ', 'extensions=SVE '];
T[' PNEXT VREG={REG:i32=$Pdn VDS=S} REG=$Pg VREG={REG:i32=$Pdn VDS=S}   ', 'size=0b10 ', 'pnext_p_p_p_ ', 'extensions=SVE '];
T[' PNEXT VREG={REG:i64=$Pdn VDS=D} REG=$Pg VREG={REG:i64=$Pdn VDS=D}   ', 'size=0b11 ', 'pnext_p_p_p_ ', 'extensions=SVE '];



# sve_int_pred_log: SVE predicate logical operations.
# A64: ig0=0b00100101 op:u=0bx S:u=0bx ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2:u=0bx Pn:u=0bxxxx o3:u=0bx Pd:u=0bxxxx
HINT('ICLASS', 'sve_int_pred_log', 'A64');

DEF Pd => PRDREG('Pd', 'Pd', 'write') ;
DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Pm => PRDREG('Pm', 'Pm', 'read')  ;
DEF Pn => PRDREG('Pn', 'Pn', 'read')  ;

T[' AND   VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'and_p_p_pp_z       ', 'extensions=SVE '];
T[' ANDS  VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'ands_p_p_pp_z      ', 'extensions=SVE '];
T[' BIC   VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'bic_p_p_pp_z       ', 'extensions=SVE '];
T[' BICS  VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'bics_p_p_pp_z      ', 'extensions=SVE '];
T[' EOR   VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'eor_p_p_pp_z       ', 'extensions=SVE '];
T[' EORS  VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'eors_p_p_pp_z      ', 'extensions=SVE '];
T[' NAND  VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'nand_p_p_pp_z      ', 'extensions=SVE '];
T[' NANDS VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'nands_p_p_pp_z     ', 'extensions=SVE '];
T[' NOR   VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'nor_p_p_pp_z       ', 'extensions=SVE '];
T[' NORS  VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'nors_p_p_pp_z      ', 'extensions=SVE '];
T[' ORN   VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'orn_p_p_pp_z       ', 'extensions=SVE '];
T[' ORNS  VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'orns_p_p_pp_z      ', 'extensions=SVE '];
T[' ORR   VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'orr_p_p_pp_z       ', 'extensions=SVE '];
T[' ORRS  VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B} ', 'NONE ', 'orrs_p_p_pp_z      ', 'extensions=SVE '];
T[' SEL   VREG={REG:i8=$Pd   VDS=B} REG=$Pg VREG={REG:i8=$Pn   VDS=B} VREG={REG:i8=$Pm   VDS=B}    ', 'NONE ', 'sel_p_p_pp_        ', 'extensions=SVE '];
T[' MOV   VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B}                           ', 'NONE ', 'MOV_and_p_p_pp_z   ', 'extensions=SVE '];
T[' MOV   VREG={REG:i8=$Pd   VDS=B} VREG={REG:i8=$Pn   VDS=B}                                      ', 'NONE ', 'MOV_orr_p_p_pp_z   ', 'extensions=SVE '];
T[' MOV   VREG={REG:i8=$Pd   VDS=B} PREG:M=$Pg VREG={REG:i8=$Pn   VDS=B}                           ', 'NONE ', 'MOV_sel_p_p_pp_    ', 'extensions=SVE '];
T[' MOVS  VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B}                           ', 'NONE ', 'MOVS_ands_p_p_pp_z ', 'extensions=SVE '];
T[' MOVS  VREG={REG:i8=$Pd   VDS=B} VREG={REG:i8=$Pn   VDS=B}                                      ', 'NONE ', 'MOVS_orrs_p_p_pp_z ', 'extensions=SVE '];
T[' NOT   VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B}                           ', 'NONE ', 'NOT_eor_p_p_pp_z   ', 'extensions=SVE '];
T[' NOTS  VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Pn   VDS=B}                           ', 'NONE ', 'NOTS_eors_p_p_pp_z ', 'extensions=SVE '];



# sve_int_pred_pattern_a: SVE inc/dec register by element count.
# A64: ig0=0b00000100 size:u=0bxx ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D:u=0bx pattern:u=0bxxxxx Rdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_pred_pattern_a', 'A64');

DEF Xdn     => GPR64('Xdn', 'Rdn', 'read|write')                                       ;
DEF amount0 => IMM('amount', 'imm4', range => (1, 16), default => 1)                   ;
DEF pattern => PATTERN('pattern', 'pattern', table => 'tbl_pattern', default => 'ALL') ;

T[' DECB REG=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'decb_r_rs_ ', 'extensions=SVE '];
T[' DECD REG=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'decd_r_rs_ ', 'extensions=SVE '];
T[' DECH REG=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'dech_r_rs_ ', 'extensions=SVE '];
T[' DECW REG=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'decw_r_rs_ ', 'extensions=SVE '];
T[' INCB REG=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'incb_r_rs_ ', 'extensions=SVE '];
T[' INCD REG=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'incd_r_rs_ ', 'extensions=SVE '];
T[' INCH REG=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'inch_r_rs_ ', 'extensions=SVE '];
T[' INCW REG=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'incw_r_rs_ ', 'extensions=SVE '];



# sve_int_pred_pattern_b: SVE saturating inc/dec register by element count.
# A64: ig0=0b00000100 size:u=0bxx ig1=0b1 sf:u=0bx imm4:u=0bxxxx ig2=0b1111 D:u=0bx U:u=0bx pattern:u=0bxxxxx Rdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_pred_pattern_b', 'A64');

DEF Wdn     => GPR32('Wdn', 'Rdn', 'read|write')                                       ;
DEF Xdn     => GPR64('Xdn', 'Rdn', 'read|write')                                       ;
DEF amount0 => IMM('amount', 'imm4', range => (1, 16), default => 1)                   ;
DEF pattern => PATTERN('pattern', 'pattern', table => 'tbl_pattern', default => 'ALL') ;

T[' SQDECB REG:s64=$Xdn REG:s32=$Wdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'sqdecb_r_rs_sx ', 'extensions=SVE '];
T[' SQDECB REG:s64=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'sqdecb_r_rs_x  ', 'extensions=SVE '];
T[' SQDECD REG:s64=$Xdn REG:s32=$Wdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'sqdecd_r_rs_sx ', 'extensions=SVE '];
T[' SQDECD REG:s64=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'sqdecd_r_rs_x  ', 'extensions=SVE '];
T[' SQDECH REG:s64=$Xdn REG:s32=$Wdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'sqdech_r_rs_sx ', 'extensions=SVE '];
T[' SQDECH REG:s64=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'sqdech_r_rs_x  ', 'extensions=SVE '];
T[' SQDECW REG:s64=$Xdn REG:s32=$Wdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'sqdecw_r_rs_sx ', 'extensions=SVE '];
T[' SQDECW REG:s64=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'sqdecw_r_rs_x  ', 'extensions=SVE '];
T[' SQINCB REG:s64=$Xdn REG:s32=$Wdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'sqincb_r_rs_sx ', 'extensions=SVE '];
T[' SQINCB REG:s64=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'sqincb_r_rs_x  ', 'extensions=SVE '];
T[' SQINCD REG:s64=$Xdn REG:s32=$Wdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'sqincd_r_rs_sx ', 'extensions=SVE '];
T[' SQINCD REG:s64=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'sqincd_r_rs_x  ', 'extensions=SVE '];
T[' SQINCH REG:s64=$Xdn REG:s32=$Wdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'sqinch_r_rs_sx ', 'extensions=SVE '];
T[' SQINCH REG:s64=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'sqinch_r_rs_x  ', 'extensions=SVE '];
T[' SQINCW REG:s64=$Xdn REG:s32=$Wdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}} ', 'NONE ', 'sqincw_r_rs_sx ', 'extensions=SVE '];
T[' SQINCW REG:s64=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'sqincw_r_rs_x  ', 'extensions=SVE '];
T[' UQDECB REG:u32=$Wdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'uqdecb_r_rs_uw ', 'extensions=SVE '];
T[' UQDECB REG:u64=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'uqdecb_r_rs_x  ', 'extensions=SVE '];
T[' UQDECD REG:u32=$Wdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'uqdecd_r_rs_uw ', 'extensions=SVE '];
T[' UQDECD REG:u64=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'uqdecd_r_rs_x  ', 'extensions=SVE '];
T[' UQDECH REG:u32=$Wdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'uqdech_r_rs_uw ', 'extensions=SVE '];
T[' UQDECH REG:u64=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'uqdech_r_rs_x  ', 'extensions=SVE '];
T[' UQDECW REG:u32=$Wdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'uqdecw_r_rs_uw ', 'extensions=SVE '];
T[' UQDECW REG:u64=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'uqdecw_r_rs_x  ', 'extensions=SVE '];
T[' UQINCB REG:u32=$Wdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'uqincb_r_rs_uw ', 'extensions=SVE '];
T[' UQINCB REG:u64=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'uqincb_r_rs_x  ', 'extensions=SVE '];
T[' UQINCD REG:u32=$Wdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'uqincd_r_rs_uw ', 'extensions=SVE '];
T[' UQINCD REG:u64=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'uqincd_r_rs_x  ', 'extensions=SVE '];
T[' UQINCH REG:u32=$Wdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'uqinch_r_rs_uw ', 'extensions=SVE '];
T[' UQINCH REG:u64=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'uqinch_r_rs_x  ', 'extensions=SVE '];
T[' UQINCW REG:u32=$Wdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'uqincw_r_rs_uw ', 'extensions=SVE '];
T[' UQINCW REG:u64=$Xdn PATTERNSH:OPT={PATTERN=$pattern SHIFTER:OPT={SHIFT=MUL AMOUNT:u8=$amount0}}              ', 'NONE ', 'uqincw_r_rs_x  ', 'extensions=SVE '];



# sve_int_ptest: SVE predicate test.
# A64: ig0=0b00100101 op:u=0bx S:u=0bx ig1=0b01000011 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx ig3=0b0 opc2:u=0bxxxx
HINT('ICLASS', 'sve_int_ptest', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read') ;
DEF Pn => PRDREG('Pn', 'Pn', 'read') ;

T[' PTEST REG=$Pg VREG={REG:i8=$Pn   VDS=B} ', 'NONE ', 'ptest_p_p_ ', 'extensions=SVE '];



# sve_int_ptrue: SVE predicate initialize.
# A64: ig0=0b00100101 size:u=0bxx ig1=0b01100 S:u=0bx ig2=0b111000 pattern:u=0bxxxxx ig3=0b0 Pd:u=0bxxxx
HINT('ICLASS', 'sve_int_ptrue', 'A64');

DEF Pd      => PRDREG('Pd', 'Pd', 'write')                                             ;
DEF pattern => PATTERN('pattern', 'pattern', table => 'tbl_pattern', default => 'ALL') ;

T[' PTRUE  VREG={REG:i8=$Pd   VDS=B} PATTERN:OPT=$pattern ', 'size=0b00 ', 'ptrue_p_s_  ', 'extensions=SVE '];
T[' PTRUE  VREG={REG:i16=$Pd VDS=H} PATTERN:OPT=$pattern  ', 'size=0b01 ', 'ptrue_p_s_  ', 'extensions=SVE '];
T[' PTRUE  VREG={REG:i32=$Pd VDS=S} PATTERN:OPT=$pattern  ', 'size=0b10 ', 'ptrue_p_s_  ', 'extensions=SVE '];
T[' PTRUE  VREG={REG:i64=$Pd VDS=D} PATTERN:OPT=$pattern  ', 'size=0b11 ', 'ptrue_p_s_  ', 'extensions=SVE '];
T[' PTRUES VREG={REG:i8=$Pd   VDS=B} PATTERN:OPT=$pattern ', 'size=0b00 ', 'ptrues_p_s_ ', 'extensions=SVE '];
T[' PTRUES VREG={REG:i16=$Pd VDS=H} PATTERN:OPT=$pattern  ', 'size=0b01 ', 'ptrues_p_s_ ', 'extensions=SVE '];
T[' PTRUES VREG={REG:i32=$Pd VDS=S} PATTERN:OPT=$pattern  ', 'size=0b10 ', 'ptrues_p_s_ ', 'extensions=SVE '];
T[' PTRUES VREG={REG:i64=$Pd VDS=D} PATTERN:OPT=$pattern  ', 'size=0b11 ', 'ptrues_p_s_ ', 'extensions=SVE '];



# sve_int_rdffr: SVE predicate read from FFR (predicated).
# A64: ig0=0b00100101 op:u=0bx S:u=0bx ig1=0b011000111100 ig2=0b0 Pg:u=0bxxxx ig3=0b0 Pd:u=0bxxxx
HINT('ICLASS', 'sve_int_rdffr', 'A64');

DEF Pd => PRDREG('Pd', 'Pd', 'write') ;
DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;

T[' RDFFR  VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg ', 'NONE ', 'rdffr_p_p_f_  ', 'extensions=SVE '];
T[' RDFFRS VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg ', 'NONE ', 'rdffrs_p_p_f_ ', 'extensions=SVE '];



# sve_int_rdffr_2: SVE predicate read from FFR (unpredicated).
# A64: ig0=0b00100101 op:u=0bx S:u=0bx ig1=0b011001111100 ig2=0b000000 Pd:u=0bxxxx
HINT('ICLASS', 'sve_int_rdffr_2', 'A64');

DEF Pd => PRDREG('Pd', 'Pd', 'write') ;

T[' RDFFR VREG={REG:i8=$Pd   VDS=B} ', 'NONE ', 'rdffr_p_f_ ', 'extensions=SVE '];



# sve_int_read_vl_a: SVE stack frame size.
# A64: ig0=0b000001001 op:u=0bx ig1=0b1 opc2:u=0bxxxxx ig2=0b01010 imm6:u=0bxxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'sve_int_read_vl_a', 'A64');

DEF Xd  => GPR64('Xd', 'Rd', 'write')                                   ;
DEF imm => IMM('imm', 'imm6', signpos => 'imm6<5>', range => (-32, 31)) ;

T[' RDVL REG=$Xd IMM:s8=$imm ', 'NONE ', 'rdvl_r_i_ ', 'extensions=SVE '];



# sve_int_reduce_0: SVE integer add reduction (predicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b000 opc:u=0bxx U:u=0bx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx
HINT('ICLASS', 'sve_int_reduce_0', 'A64');

DEF Dd => SIMD64('Dd', 'Vd', 'write') ;
DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' SADDV REG:s64=$Dd REG=$Pg VREG={REG:s8=$Zn   VDS=B} ', 'size=0b00 ', 'saddv_r_p_z_ ', 'extensions=SVE '];
T[' SADDV REG:s64=$Dd REG=$Pg VREG={REG:s16=$Zn VDS=H}  ', 'size=0b01 ', 'saddv_r_p_z_ ', 'extensions=SVE '];
T[' SADDV REG:s64=$Dd REG=$Pg VREG={REG:s32=$Zn VDS=S}  ', 'size=0b10 ', 'saddv_r_p_z_ ', 'extensions=SVE '];
T[' UADDV REG:u64=$Dd REG=$Pg VREG={REG:u8=$Zn   VDS=B} ', 'size=0b00 ', 'uaddv_r_p_z_ ', 'extensions=SVE '];
T[' UADDV REG:u64=$Dd REG=$Pg VREG={REG:u16=$Zn VDS=H}  ', 'size=0b01 ', 'uaddv_r_p_z_ ', 'extensions=SVE '];
T[' UADDV REG:u64=$Dd REG=$Pg VREG={REG:u32=$Zn VDS=S}  ', 'size=0b10 ', 'uaddv_r_p_z_ ', 'extensions=SVE '];
T[' UADDV REG:u64=$Dd REG=$Pg VREG={REG:u64=$Zn VDS=D}  ', 'size=0b11 ', 'uaddv_r_p_z_ ', 'extensions=SVE '];



# sve_int_reduce_1: SVE integer min/max reduction (predicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b001 opc:u=0bxx U:u=0bx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx
HINT('ICLASS', 'sve_int_reduce_1', 'A64');

DEF Bd => SIMD8('Bd', 'Vd', 'write')  ;
DEF Dd => SIMD64('Dd', 'Vd', 'write') ;
DEF Hd => SIMD16('Hd', 'Vd', 'write') ;
DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Sd => SIMD32('Sd', 'Vd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' SMAXV REG:s8=$Bd  REG=$Pg VREG={REG:s8=$Zn   VDS=B} ', 'size=0b00 ', 'smaxv_r_p_z_ ', 'extensions=SVE '];
T[' SMAXV REG:s16=$Hd REG=$Pg VREG={REG:s16=$Zn VDS=H}  ', 'size=0b01 ', 'smaxv_r_p_z_ ', 'extensions=SVE '];
T[' SMAXV REG:s32=$Sd REG=$Pg VREG={REG:s32=$Zn VDS=S}  ', 'size=0b10 ', 'smaxv_r_p_z_ ', 'extensions=SVE '];
T[' SMAXV REG:s64=$Dd REG=$Pg VREG={REG:s64=$Zn VDS=D}  ', 'size=0b11 ', 'smaxv_r_p_z_ ', 'extensions=SVE '];
T[' SMINV REG:s8=$Bd  REG=$Pg VREG={REG:s8=$Zn   VDS=B} ', 'size=0b00 ', 'sminv_r_p_z_ ', 'extensions=SVE '];
T[' SMINV REG:s16=$Hd REG=$Pg VREG={REG:s16=$Zn VDS=H}  ', 'size=0b01 ', 'sminv_r_p_z_ ', 'extensions=SVE '];
T[' SMINV REG:s32=$Sd REG=$Pg VREG={REG:s32=$Zn VDS=S}  ', 'size=0b10 ', 'sminv_r_p_z_ ', 'extensions=SVE '];
T[' SMINV REG:s64=$Dd REG=$Pg VREG={REG:s64=$Zn VDS=D}  ', 'size=0b11 ', 'sminv_r_p_z_ ', 'extensions=SVE '];
T[' UMAXV REG:u8=$Bd  REG=$Pg VREG={REG:u8=$Zn   VDS=B} ', 'size=0b00 ', 'umaxv_r_p_z_ ', 'extensions=SVE '];
T[' UMAXV REG:u16=$Hd REG=$Pg VREG={REG:u16=$Zn VDS=H}  ', 'size=0b01 ', 'umaxv_r_p_z_ ', 'extensions=SVE '];
T[' UMAXV REG:u32=$Sd REG=$Pg VREG={REG:u32=$Zn VDS=S}  ', 'size=0b10 ', 'umaxv_r_p_z_ ', 'extensions=SVE '];
T[' UMAXV REG:u64=$Dd REG=$Pg VREG={REG:u64=$Zn VDS=D}  ', 'size=0b11 ', 'umaxv_r_p_z_ ', 'extensions=SVE '];
T[' UMINV REG:u8=$Bd  REG=$Pg VREG={REG:u8=$Zn   VDS=B} ', 'size=0b00 ', 'uminv_r_p_z_ ', 'extensions=SVE '];
T[' UMINV REG:u16=$Hd REG=$Pg VREG={REG:u16=$Zn VDS=H}  ', 'size=0b01 ', 'uminv_r_p_z_ ', 'extensions=SVE '];
T[' UMINV REG:u32=$Sd REG=$Pg VREG={REG:u32=$Zn VDS=S}  ', 'size=0b10 ', 'uminv_r_p_z_ ', 'extensions=SVE '];
T[' UMINV REG:u64=$Dd REG=$Pg VREG={REG:u64=$Zn VDS=D}  ', 'size=0b11 ', 'uminv_r_p_z_ ', 'extensions=SVE '];



# sve_int_reduce_2: SVE bitwise logical reduction (predicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b011 opc:u=0bxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx
HINT('ICLASS', 'sve_int_reduce_2', 'A64');

DEF Bd => SIMD8('Bd', 'Vd', 'write')  ;
DEF Dd => SIMD64('Dd', 'Vd', 'write') ;
DEF Hd => SIMD16('Hd', 'Vd', 'write') ;
DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Sd => SIMD32('Sd', 'Vd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' ANDV REG:i8=$Bd  REG=$Pg VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'andv_r_p_z_ ', 'extensions=SVE '];
T[' ANDV REG:i16=$Hd REG=$Pg VREG={REG:i16=$Zn VDS=H}  ', 'size=0b01 ', 'andv_r_p_z_ ', 'extensions=SVE '];
T[' ANDV REG:i32=$Sd REG=$Pg VREG={REG:i32=$Zn VDS=S}  ', 'size=0b10 ', 'andv_r_p_z_ ', 'extensions=SVE '];
T[' ANDV REG:i64=$Dd REG=$Pg VREG={REG:i64=$Zn VDS=D}  ', 'size=0b11 ', 'andv_r_p_z_ ', 'extensions=SVE '];
T[' EORV REG:i8=$Bd  REG=$Pg VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'eorv_r_p_z_ ', 'extensions=SVE '];
T[' EORV REG:i16=$Hd REG=$Pg VREG={REG:i16=$Zn VDS=H}  ', 'size=0b01 ', 'eorv_r_p_z_ ', 'extensions=SVE '];
T[' EORV REG:i32=$Sd REG=$Pg VREG={REG:i32=$Zn VDS=S}  ', 'size=0b10 ', 'eorv_r_p_z_ ', 'extensions=SVE '];
T[' EORV REG:i64=$Dd REG=$Pg VREG={REG:i64=$Zn VDS=D}  ', 'size=0b11 ', 'eorv_r_p_z_ ', 'extensions=SVE '];
T[' ORV  REG:i8=$Bd  REG=$Pg VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'orv_r_p_z_  ', 'extensions=SVE '];
T[' ORV  REG:i16=$Hd REG=$Pg VREG={REG:i16=$Zn VDS=H}  ', 'size=0b01 ', 'orv_r_p_z_  ', 'extensions=SVE '];
T[' ORV  REG:i32=$Sd REG=$Pg VREG={REG:i32=$Zn VDS=S}  ', 'size=0b10 ', 'orv_r_p_z_  ', 'extensions=SVE '];
T[' ORV  REG:i64=$Dd REG=$Pg VREG={REG:i64=$Zn VDS=D}  ', 'size=0b11 ', 'orv_r_p_z_  ', 'extensions=SVE '];



# sve_int_rotate_imm: sve_int_rotate_imm.
# A64: ig0=0b00000100 tszh:u=0bxx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b001101 Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_rotate_imm', 'A64');

DEF Zdn    => SVEREG('Zdn', 'Zdn', 'read|write')               ;
DEF Zm     => SVEREG('Zm', 'Zm', 'read')                       ;
DEF const  => IMM('const', 'tszh:tszl:imm3', range => (1, 8))  ;
DEF const2 => IMM('const', 'tszh:tszl:imm3', range => (1, 16)) ;
DEF const3 => IMM('const', 'tszh:tszl:imm3', range => (1, 32)) ;
DEF const4 => IMM('const', 'tszh:tszl:imm3', range => (1, 64)) ;

T[' XAR VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} IMM:u8=$const ', 'tszh=0b00 tszl=0b01 ', 'xar_z_zzi_ ', 'extensions=SVE2 '];
T[' XAR VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H} IMM:u8=$const2   ', 'tszh=0b00 tszl=0b1x ', 'xar_z_zzi_ ', 'extensions=SVE2 '];
T[' XAR VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S} IMM:u8=$const3   ', 'tszh=0b01           ', 'xar_z_zzi_ ', 'extensions=SVE2 '];
T[' XAR VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D} IMM:u8=$const4   ', 'tszh=0b1x           ', 'xar_z_zzi_ ', 'extensions=SVE2 '];



# sve_int_scmp_vi: SVE integer compare with signed immediate.
# A64: ig0=0b00100101 size:u=0bxx ig1=0b0 imm5:u=0bxxxxx op:u=0bx ig2=0b0 o2:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx ne:u=0bx Pd:u=0bxxxx
HINT('ICLASS', 'sve_int_scmp_vi', 'A64');

DEF Pd  => PRDREG('Pd', 'Pd', 'write')                                  ;
DEF Pg  => PRDREG('Pg', 'Pg', 'read')                                   ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')                                   ;
DEF imm => IMM('imm', 'imm5', signpos => 'imm5<4>', range => (-16, 15)) ;

T[' CMPEQ VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} IMM:s8=$imm ', 'size=0b00 ', 'cmpeq_p_p_zi_ ', 'extensions=SVE '];
T[' CMPEQ VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} IMM:s8=$imm   ', 'size=0b01 ', 'cmpeq_p_p_zi_ ', 'extensions=SVE '];
T[' CMPEQ VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} IMM:s8=$imm   ', 'size=0b10 ', 'cmpeq_p_p_zi_ ', 'extensions=SVE '];
T[' CMPEQ VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D} IMM:s8=$imm   ', 'size=0b11 ', 'cmpeq_p_p_zi_ ', 'extensions=SVE '];
T[' CMPGT VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} IMM:s8=$imm ', 'size=0b00 ', 'cmpgt_p_p_zi_ ', 'extensions=SVE '];
T[' CMPGT VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} IMM:s8=$imm   ', 'size=0b01 ', 'cmpgt_p_p_zi_ ', 'extensions=SVE '];
T[' CMPGT VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} IMM:s8=$imm   ', 'size=0b10 ', 'cmpgt_p_p_zi_ ', 'extensions=SVE '];
T[' CMPGT VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D} IMM:s8=$imm   ', 'size=0b11 ', 'cmpgt_p_p_zi_ ', 'extensions=SVE '];
T[' CMPGE VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} IMM:s8=$imm ', 'size=0b00 ', 'cmpge_p_p_zi_ ', 'extensions=SVE '];
T[' CMPGE VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} IMM:s8=$imm   ', 'size=0b01 ', 'cmpge_p_p_zi_ ', 'extensions=SVE '];
T[' CMPGE VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} IMM:s8=$imm   ', 'size=0b10 ', 'cmpge_p_p_zi_ ', 'extensions=SVE '];
T[' CMPGE VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D} IMM:s8=$imm   ', 'size=0b11 ', 'cmpge_p_p_zi_ ', 'extensions=SVE '];
T[' CMPLT VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} IMM:s8=$imm ', 'size=0b00 ', 'cmplt_p_p_zi_ ', 'extensions=SVE '];
T[' CMPLT VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} IMM:s8=$imm   ', 'size=0b01 ', 'cmplt_p_p_zi_ ', 'extensions=SVE '];
T[' CMPLT VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} IMM:s8=$imm   ', 'size=0b10 ', 'cmplt_p_p_zi_ ', 'extensions=SVE '];
T[' CMPLT VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D} IMM:s8=$imm   ', 'size=0b11 ', 'cmplt_p_p_zi_ ', 'extensions=SVE '];
T[' CMPLE VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} IMM:s8=$imm ', 'size=0b00 ', 'cmple_p_p_zi_ ', 'extensions=SVE '];
T[' CMPLE VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} IMM:s8=$imm   ', 'size=0b01 ', 'cmple_p_p_zi_ ', 'extensions=SVE '];
T[' CMPLE VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} IMM:s8=$imm   ', 'size=0b10 ', 'cmple_p_p_zi_ ', 'extensions=SVE '];
T[' CMPLE VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D} IMM:s8=$imm   ', 'size=0b11 ', 'cmple_p_p_zi_ ', 'extensions=SVE '];
T[' CMPNE VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} IMM:s8=$imm ', 'size=0b00 ', 'cmpne_p_p_zi_ ', 'extensions=SVE '];
T[' CMPNE VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} IMM:s8=$imm   ', 'size=0b01 ', 'cmpne_p_p_zi_ ', 'extensions=SVE '];
T[' CMPNE VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} IMM:s8=$imm   ', 'size=0b10 ', 'cmpne_p_p_zi_ ', 'extensions=SVE '];
T[' CMPNE VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D} IMM:s8=$imm   ', 'size=0b11 ', 'cmpne_p_p_zi_ ', 'extensions=SVE '];



# sve_int_sel_vvv: SVE select vector elements (predicated).
# A64: ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b11 Pg:u=0bxxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_sel_vvv', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' SEL VREG={REG:i8=$Zd   VDS=B} REG=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'sel_z_p_zz_     ', 'extensions=SVE '];
T[' SEL VREG={REG:i16=$Zd VDS=H} REG=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'sel_z_p_zz_     ', 'extensions=SVE '];
T[' SEL VREG={REG:i32=$Zd VDS=S} REG=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'sel_z_p_zz_     ', 'extensions=SVE '];
T[' SEL VREG={REG:i64=$Zd VDS=D} REG=$Pg VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'sel_z_p_zz_     ', 'extensions=SVE '];
T[' MOV VREG={REG:i8=$Zd   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zn   VDS=B}                        ', 'size=0b00 ', 'MOV_sel_z_p_zz_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg VREG={REG:i16=$Zn VDS=H}                          ', 'size=0b01 ', 'MOV_sel_z_p_zz_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg VREG={REG:i32=$Zn VDS=S}                          ', 'size=0b10 ', 'MOV_sel_z_p_zz_ ', 'extensions=SVE '];
T[' MOV VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg VREG={REG:i64=$Zn VDS=D}                          ', 'size=0b11 ', 'MOV_sel_z_p_zz_ ', 'extensions=SVE '];



# sve_int_setffr: SVE FFR initialise.
# A64: ig0=0b00 ig1=0b100101 opc:u=0bxx ig2=0b10 ig3=0b1100100100 ig4=0b000000 ig5=0b0 ig6=0b0 ig7=0b00
HINT('ICLASS', 'sve_int_setffr', 'A64');


T[' SETFFR  ', 'NONE ', 'setffr_f_ ', 'extensions=SVE '];



# sve_int_sqdmulh: SVE2 signed saturating doubling multiply high (unpredicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b01110 R:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_sqdmulh', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' SQDMULH  VREG={REG:s8=$Zd   VDS=B} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'sqdmulh_z_zz_  ', 'extensions=SVE2 '];
T[' SQDMULH  VREG={REG:s16=$Zd VDS=H} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'sqdmulh_z_zz_  ', 'extensions=SVE2 '];
T[' SQDMULH  VREG={REG:s32=$Zd VDS=S} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'sqdmulh_z_zz_  ', 'extensions=SVE2 '];
T[' SQDMULH  VREG={REG:s64=$Zd VDS=D} VREG={REG:s64=$Zn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'sqdmulh_z_zz_  ', 'extensions=SVE2 '];
T[' SQRDMULH VREG={REG:s8=$Zd   VDS=B} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'sqrdmulh_z_zz_ ', 'extensions=SVE2 '];
T[' SQRDMULH VREG={REG:s16=$Zd VDS=H} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'sqrdmulh_z_zz_ ', 'extensions=SVE2 '];
T[' SQRDMULH VREG={REG:s32=$Zd VDS=S} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'sqrdmulh_z_zz_ ', 'extensions=SVE2 '];
T[' SQRDMULH VREG={REG:s64=$Zd VDS=D} VREG={REG:s64=$Zn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'sqrdmulh_z_zz_ ', 'extensions=SVE2 '];



# sve_int_tern_log: SVE2 bitwise ternary operations.
# A64: ig0=0b00000100 opc:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b00111 o2:u=0bx Zk:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_int_tern_log', 'A64');

DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zk  => SVEREG('Zk', 'Zk', 'read')         ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' BCAX  VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D} VREG={REG:i64=$Zk VDS=D} ', 'NONE ', 'bcax_z_zzz_  ', 'extensions=SVE2 '];
T[' BSL1N VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D} VREG={REG:i64=$Zk VDS=D} ', 'NONE ', 'bsl1n_z_zzz_ ', 'extensions=SVE2 '];
T[' BSL2N VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D} VREG={REG:i64=$Zk VDS=D} ', 'NONE ', 'bsl2n_z_zzz_ ', 'extensions=SVE2 '];
T[' BSL   VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D} VREG={REG:i64=$Zk VDS=D} ', 'NONE ', 'bsl_z_zzz_   ', 'extensions=SVE2 '];
T[' EOR3  VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D} VREG={REG:i64=$Zk VDS=D} ', 'NONE ', 'eor3_z_zzz_  ', 'extensions=SVE2 '];
T[' NBSL  VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D} VREG={REG:i64=$Zk VDS=D} ', 'NONE ', 'nbsl_z_zzz_  ', 'extensions=SVE2 '];



# sve_int_ucmp_vi: SVE integer compare with unsigned immediate.
# A64: ig0=0b00100100 size:u=0bxx ig1=0b1 imm7:u=0bxxxxxxx lt:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx ne:u=0bx Pd:u=0bxxxx
HINT('ICLASS', 'sve_int_ucmp_vi', 'A64');

DEF Pd  => PRDREG('Pd', 'Pd', 'write')           ;
DEF Pg  => PRDREG('Pg', 'Pg', 'read')            ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')            ;
DEF imm => IMM('imm', 'imm7', range => (0, 127)) ;

T[' CMPHI VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} IMM:u8=$imm ', 'size=0b00 ', 'cmphi_p_p_zi_ ', 'extensions=SVE '];
T[' CMPHI VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} IMM:u8=$imm   ', 'size=0b01 ', 'cmphi_p_p_zi_ ', 'extensions=SVE '];
T[' CMPHI VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} IMM:u8=$imm   ', 'size=0b10 ', 'cmphi_p_p_zi_ ', 'extensions=SVE '];
T[' CMPHI VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D} IMM:u8=$imm   ', 'size=0b11 ', 'cmphi_p_p_zi_ ', 'extensions=SVE '];
T[' CMPHS VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} IMM:u8=$imm ', 'size=0b00 ', 'cmphs_p_p_zi_ ', 'extensions=SVE '];
T[' CMPHS VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} IMM:u8=$imm   ', 'size=0b01 ', 'cmphs_p_p_zi_ ', 'extensions=SVE '];
T[' CMPHS VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} IMM:u8=$imm   ', 'size=0b10 ', 'cmphs_p_p_zi_ ', 'extensions=SVE '];
T[' CMPHS VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D} IMM:u8=$imm   ', 'size=0b11 ', 'cmphs_p_p_zi_ ', 'extensions=SVE '];
T[' CMPLO VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} IMM:u8=$imm ', 'size=0b00 ', 'cmplo_p_p_zi_ ', 'extensions=SVE '];
T[' CMPLO VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} IMM:u8=$imm   ', 'size=0b01 ', 'cmplo_p_p_zi_ ', 'extensions=SVE '];
T[' CMPLO VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} IMM:u8=$imm   ', 'size=0b10 ', 'cmplo_p_p_zi_ ', 'extensions=SVE '];
T[' CMPLO VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D} IMM:u8=$imm   ', 'size=0b11 ', 'cmplo_p_p_zi_ ', 'extensions=SVE '];
T[' CMPLS VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} IMM:u8=$imm ', 'size=0b00 ', 'cmpls_p_p_zi_ ', 'extensions=SVE '];
T[' CMPLS VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} IMM:u8=$imm   ', 'size=0b01 ', 'cmpls_p_p_zi_ ', 'extensions=SVE '];
T[' CMPLS VREG={REG:i32=$Pd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} IMM:u8=$imm   ', 'size=0b10 ', 'cmpls_p_p_zi_ ', 'extensions=SVE '];
T[' CMPLS VREG={REG:i64=$Pd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D} IMM:u8=$imm   ', 'size=0b11 ', 'cmpls_p_p_zi_ ', 'extensions=SVE '];



# sve_int_un_pred_arit_0: SVE integer unary operations (predicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b010 opc:u=0bxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_un_pred_arit_0', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' ABS  VREG={REG:i8=$Zd   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'abs_z_p_z_  ', 'extensions=SVE '];
T[' ABS  VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg VREG={REG:i16=$Zn VDS=H}   ', 'size=0b01 ', 'abs_z_p_z_  ', 'extensions=SVE '];
T[' ABS  VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg VREG={REG:i32=$Zn VDS=S}   ', 'size=0b10 ', 'abs_z_p_z_  ', 'extensions=SVE '];
T[' ABS  VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg VREG={REG:i64=$Zn VDS=D}   ', 'size=0b11 ', 'abs_z_p_z_  ', 'extensions=SVE '];
T[' NEG  VREG={REG:i8=$Zd   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'neg_z_p_z_  ', 'extensions=SVE '];
T[' NEG  VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg VREG={REG:i16=$Zn VDS=H}   ', 'size=0b01 ', 'neg_z_p_z_  ', 'extensions=SVE '];
T[' NEG  VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg VREG={REG:i32=$Zn VDS=S}   ', 'size=0b10 ', 'neg_z_p_z_  ', 'extensions=SVE '];
T[' NEG  VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg VREG={REG:i64=$Zn VDS=D}   ', 'size=0b11 ', 'neg_z_p_z_  ', 'extensions=SVE '];
T[' SXTB VREG={REG:s16=$Zd VDS=H} PREG:M=$Pg VREG={REG:s16=$Zn VDS=H}   ', 'size=0b01 ', 'sxtb_z_p_z_ ', 'extensions=SVE '];
T[' SXTB VREG={REG:s32=$Zd VDS=S} PREG:M=$Pg VREG={REG:s32=$Zn VDS=S}   ', 'size=0b10 ', 'sxtb_z_p_z_ ', 'extensions=SVE '];
T[' SXTB VREG={REG:s64=$Zd VDS=D} PREG:M=$Pg VREG={REG:s64=$Zn VDS=D}   ', 'size=0b11 ', 'sxtb_z_p_z_ ', 'extensions=SVE '];
T[' SXTH VREG={REG:s32=$Zd VDS=S} PREG:M=$Pg VREG={REG:s32=$Zn VDS=S}   ', 'size=0bx0 ', 'sxth_z_p_z_ ', 'extensions=SVE '];
T[' SXTH VREG={REG:s64=$Zd VDS=D} PREG:M=$Pg VREG={REG:s64=$Zn VDS=D}   ', 'size=0bx1 ', 'sxth_z_p_z_ ', 'extensions=SVE '];
T[' SXTW VREG={REG:s64=$Zd VDS=D} PREG:M=$Pg VREG={REG:s64=$Zn VDS=D}   ', 'NONE      ', 'sxtw_z_p_z_ ', 'extensions=SVE '];
T[' UXTB VREG={REG:u16=$Zd VDS=H} PREG:M=$Pg VREG={REG:u16=$Zn VDS=H}   ', 'size=0b01 ', 'uxtb_z_p_z_ ', 'extensions=SVE '];
T[' UXTB VREG={REG:u32=$Zd VDS=S} PREG:M=$Pg VREG={REG:u32=$Zn VDS=S}   ', 'size=0b10 ', 'uxtb_z_p_z_ ', 'extensions=SVE '];
T[' UXTB VREG={REG:u64=$Zd VDS=D} PREG:M=$Pg VREG={REG:u64=$Zn VDS=D}   ', 'size=0b11 ', 'uxtb_z_p_z_ ', 'extensions=SVE '];
T[' UXTH VREG={REG:u32=$Zd VDS=S} PREG:M=$Pg VREG={REG:u32=$Zn VDS=S}   ', 'size=0bx0 ', 'uxth_z_p_z_ ', 'extensions=SVE '];
T[' UXTH VREG={REG:u64=$Zd VDS=D} PREG:M=$Pg VREG={REG:u64=$Zn VDS=D}   ', 'size=0bx1 ', 'uxth_z_p_z_ ', 'extensions=SVE '];
T[' UXTW VREG={REG:u64=$Zd VDS=D} PREG:M=$Pg VREG={REG:u64=$Zn VDS=D}   ', 'NONE      ', 'uxtw_z_p_z_ ', 'extensions=SVE '];



# sve_int_un_pred_arit_1: SVE bitwise unary operations (predicated).
# A64: ig0=0b00000100 size:u=0bxx ig1=0b011 opc:u=0bxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_int_un_pred_arit_1', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' CLS  VREG={REG:i8=$Zd   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'cls_z_p_z_  ', 'extensions=SVE '];
T[' CLS  VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg VREG={REG:i16=$Zn VDS=H}   ', 'size=0b01 ', 'cls_z_p_z_  ', 'extensions=SVE '];
T[' CLS  VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg VREG={REG:i32=$Zn VDS=S}   ', 'size=0b10 ', 'cls_z_p_z_  ', 'extensions=SVE '];
T[' CLS  VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg VREG={REG:i64=$Zn VDS=D}   ', 'size=0b11 ', 'cls_z_p_z_  ', 'extensions=SVE '];
T[' CLZ  VREG={REG:i8=$Zd   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'clz_z_p_z_  ', 'extensions=SVE '];
T[' CLZ  VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg VREG={REG:i16=$Zn VDS=H}   ', 'size=0b01 ', 'clz_z_p_z_  ', 'extensions=SVE '];
T[' CLZ  VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg VREG={REG:i32=$Zn VDS=S}   ', 'size=0b10 ', 'clz_z_p_z_  ', 'extensions=SVE '];
T[' CLZ  VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg VREG={REG:i64=$Zn VDS=D}   ', 'size=0b11 ', 'clz_z_p_z_  ', 'extensions=SVE '];
T[' CNOT VREG={REG:i8=$Zd   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'cnot_z_p_z_ ', 'extensions=SVE '];
T[' CNOT VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg VREG={REG:i16=$Zn VDS=H}   ', 'size=0b01 ', 'cnot_z_p_z_ ', 'extensions=SVE '];
T[' CNOT VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg VREG={REG:i32=$Zn VDS=S}   ', 'size=0b10 ', 'cnot_z_p_z_ ', 'extensions=SVE '];
T[' CNOT VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg VREG={REG:i64=$Zn VDS=D}   ', 'size=0b11 ', 'cnot_z_p_z_ ', 'extensions=SVE '];
T[' CNT  VREG={REG:i8=$Zd   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'cnt_z_p_z_  ', 'extensions=SVE '];
T[' CNT  VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg VREG={REG:i16=$Zn VDS=H}   ', 'size=0b01 ', 'cnt_z_p_z_  ', 'extensions=SVE '];
T[' CNT  VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg VREG={REG:i32=$Zn VDS=S}   ', 'size=0b10 ', 'cnt_z_p_z_  ', 'extensions=SVE '];
T[' CNT  VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg VREG={REG:i64=$Zn VDS=D}   ', 'size=0b11 ', 'cnt_z_p_z_  ', 'extensions=SVE '];
T[' FABS VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H}   ', 'size=0b01 ', 'fabs_z_p_z_ ', 'extensions=SVE '];
T[' FABS VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S}   ', 'size=0b10 ', 'fabs_z_p_z_ ', 'extensions=SVE '];
T[' FABS VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D}   ', 'size=0b11 ', 'fabs_z_p_z_ ', 'extensions=SVE '];
T[' FNEG VREG={REG:f16=$Zd VDS=H} PREG:M=$Pg VREG={REG:f16=$Zn VDS=H}   ', 'size=0b01 ', 'fneg_z_p_z_ ', 'extensions=SVE '];
T[' FNEG VREG={REG:f32=$Zd VDS=S} PREG:M=$Pg VREG={REG:f32=$Zn VDS=S}   ', 'size=0b10 ', 'fneg_z_p_z_ ', 'extensions=SVE '];
T[' FNEG VREG={REG:f64=$Zd VDS=D} PREG:M=$Pg VREG={REG:f64=$Zn VDS=D}   ', 'size=0b11 ', 'fneg_z_p_z_ ', 'extensions=SVE '];
T[' NOT  VREG={REG:i8=$Zd   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zn   VDS=B} ', 'size=0b00 ', 'not_z_p_z_  ', 'extensions=SVE '];
T[' NOT  VREG={REG:i16=$Zd VDS=H} PREG:M=$Pg VREG={REG:i16=$Zn VDS=H}   ', 'size=0b01 ', 'not_z_p_z_  ', 'extensions=SVE '];
T[' NOT  VREG={REG:i32=$Zd VDS=S} PREG:M=$Pg VREG={REG:i32=$Zn VDS=S}   ', 'size=0b10 ', 'not_z_p_z_  ', 'extensions=SVE '];
T[' NOT  VREG={REG:i64=$Zd VDS=D} PREG:M=$Pg VREG={REG:i64=$Zn VDS=D}   ', 'size=0b11 ', 'not_z_p_z_  ', 'extensions=SVE '];



# sve_int_while_rr: SVE integer compare scalar count and limit.
# A64: ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U:u=0bx lt:u=0bx Rn:u=0bxxxxx eq:u=0bx Pd:u=0bxxxx
HINT('ICLASS', 'sve_int_while_rr', 'A64');

DEF Pd => PRDREG('Pd', 'Pd', 'write') ;
DEF Wm => GPR32('Wm', 'Rm', 'read')   ;
DEF Wn => GPR32('Wn', 'Rn', 'read')   ;
DEF Xm => GPR64('Xm', 'Rm', 'read')   ;
DEF Xn => GPR64('Xn', 'Rn', 'read')   ;

T[' WHILEGE VREG={REG:i8=$Pd   VDS=B} REG=$Wn REG=$Wm         ', 'size=0b00 sf=0b0 ', 'whilege_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEGE VREG={REG:i8=$Pd   VDS=B} REG=$Xn REG=$Xm         ', 'size=0b00 sf=0b1 ', 'whilege_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEGE VREG={REG:i16=$Pd VDS=H} REG=$Wn REG=$Wm          ', 'size=0b01 sf=0b0 ', 'whilege_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEGE VREG={REG:i16=$Pd VDS=H} REG=$Xn REG=$Xm          ', 'size=0b01 sf=0b1 ', 'whilege_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEGE VREG={REG:i32=$Pd VDS=S} REG=$Wn REG=$Wm          ', 'size=0b10 sf=0b0 ', 'whilege_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEGE VREG={REG:i32=$Pd VDS=S} REG=$Xn REG=$Xm          ', 'size=0b10 sf=0b1 ', 'whilege_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEGE VREG={REG:i64=$Pd VDS=D} REG=$Wn REG=$Wm          ', 'size=0b11 sf=0b0 ', 'whilege_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEGE VREG={REG:i64=$Pd VDS=D} REG=$Xn REG=$Xm          ', 'size=0b11 sf=0b1 ', 'whilege_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEGT VREG={REG:i8=$Pd   VDS=B} REG=$Wn REG=$Wm         ', 'size=0b00 sf=0b0 ', 'whilegt_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEGT VREG={REG:i8=$Pd   VDS=B} REG=$Xn REG=$Xm         ', 'size=0b00 sf=0b1 ', 'whilegt_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEGT VREG={REG:i16=$Pd VDS=H} REG=$Wn REG=$Wm          ', 'size=0b01 sf=0b0 ', 'whilegt_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEGT VREG={REG:i16=$Pd VDS=H} REG=$Xn REG=$Xm          ', 'size=0b01 sf=0b1 ', 'whilegt_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEGT VREG={REG:i32=$Pd VDS=S} REG=$Wn REG=$Wm          ', 'size=0b10 sf=0b0 ', 'whilegt_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEGT VREG={REG:i32=$Pd VDS=S} REG=$Xn REG=$Xm          ', 'size=0b10 sf=0b1 ', 'whilegt_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEGT VREG={REG:i64=$Pd VDS=D} REG=$Wn REG=$Wm          ', 'size=0b11 sf=0b0 ', 'whilegt_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEGT VREG={REG:i64=$Pd VDS=D} REG=$Xn REG=$Xm          ', 'size=0b11 sf=0b1 ', 'whilegt_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEHI VREG={REG:i8=$Pd   VDS=B} REG=$Wn REG=$Wm         ', 'size=0b00 sf=0b0 ', 'whilehi_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEHI VREG={REG:i8=$Pd   VDS=B} REG=$Xn REG=$Xm         ', 'size=0b00 sf=0b1 ', 'whilehi_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEHI VREG={REG:i16=$Pd VDS=H} REG=$Wn REG=$Wm          ', 'size=0b01 sf=0b0 ', 'whilehi_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEHI VREG={REG:i16=$Pd VDS=H} REG=$Xn REG=$Xm          ', 'size=0b01 sf=0b1 ', 'whilehi_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEHI VREG={REG:i32=$Pd VDS=S} REG=$Wn REG=$Wm          ', 'size=0b10 sf=0b0 ', 'whilehi_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEHI VREG={REG:i32=$Pd VDS=S} REG=$Xn REG=$Xm          ', 'size=0b10 sf=0b1 ', 'whilehi_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEHI VREG={REG:i64=$Pd VDS=D} REG=$Wn REG=$Wm          ', 'size=0b11 sf=0b0 ', 'whilehi_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEHI VREG={REG:i64=$Pd VDS=D} REG=$Xn REG=$Xm          ', 'size=0b11 sf=0b1 ', 'whilehi_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEHS VREG={REG:s8=$Pd   VDS=B} REG:s32=$Wn REG:s32=$Wm ', 'size=0b00 sf=0b0 ', 'whilehs_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEHS VREG={REG:s8=$Pd   VDS=B} REG:s64=$Xn REG:s64=$Xm ', 'size=0b00 sf=0b1 ', 'whilehs_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEHS VREG={REG:s16=$Pd VDS=H} REG:s32=$Wn REG:s32=$Wm  ', 'size=0b01 sf=0b0 ', 'whilehs_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEHS VREG={REG:s16=$Pd VDS=H} REG:s64=$Xn REG:s64=$Xm  ', 'size=0b01 sf=0b1 ', 'whilehs_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEHS VREG={REG:s32=$Pd VDS=S} REG:s32=$Wn REG:s32=$Wm  ', 'size=0b10 sf=0b0 ', 'whilehs_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEHS VREG={REG:s32=$Pd VDS=S} REG:s64=$Xn REG:s64=$Xm  ', 'size=0b10 sf=0b1 ', 'whilehs_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEHS VREG={REG:s64=$Pd VDS=D} REG:s32=$Wn REG:s32=$Wm  ', 'size=0b11 sf=0b0 ', 'whilehs_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEHS VREG={REG:s64=$Pd VDS=D} REG:s64=$Xn REG:s64=$Xm  ', 'size=0b11 sf=0b1 ', 'whilehs_p_p_rr_ ', 'extensions=SVE2 '];
T[' WHILELE VREG={REG:i8=$Pd   VDS=B} REG=$Wn REG=$Wm         ', 'size=0b00 sf=0b0 ', 'whilele_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELE VREG={REG:i8=$Pd   VDS=B} REG=$Xn REG=$Xm         ', 'size=0b00 sf=0b1 ', 'whilele_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELE VREG={REG:i16=$Pd VDS=H} REG=$Wn REG=$Wm          ', 'size=0b01 sf=0b0 ', 'whilele_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELE VREG={REG:i16=$Pd VDS=H} REG=$Xn REG=$Xm          ', 'size=0b01 sf=0b1 ', 'whilele_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELE VREG={REG:i32=$Pd VDS=S} REG=$Wn REG=$Wm          ', 'size=0b10 sf=0b0 ', 'whilele_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELE VREG={REG:i32=$Pd VDS=S} REG=$Xn REG=$Xm          ', 'size=0b10 sf=0b1 ', 'whilele_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELE VREG={REG:i64=$Pd VDS=D} REG=$Wn REG=$Wm          ', 'size=0b11 sf=0b0 ', 'whilele_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELE VREG={REG:i64=$Pd VDS=D} REG=$Xn REG=$Xm          ', 'size=0b11 sf=0b1 ', 'whilele_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELO VREG={REG:i8=$Pd   VDS=B} REG=$Wn REG=$Wm         ', 'size=0b00 sf=0b0 ', 'whilelo_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELO VREG={REG:i8=$Pd   VDS=B} REG=$Xn REG=$Xm         ', 'size=0b00 sf=0b1 ', 'whilelo_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELO VREG={REG:i16=$Pd VDS=H} REG=$Wn REG=$Wm          ', 'size=0b01 sf=0b0 ', 'whilelo_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELO VREG={REG:i16=$Pd VDS=H} REG=$Xn REG=$Xm          ', 'size=0b01 sf=0b1 ', 'whilelo_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELO VREG={REG:i32=$Pd VDS=S} REG=$Wn REG=$Wm          ', 'size=0b10 sf=0b0 ', 'whilelo_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELO VREG={REG:i32=$Pd VDS=S} REG=$Xn REG=$Xm          ', 'size=0b10 sf=0b1 ', 'whilelo_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELO VREG={REG:i64=$Pd VDS=D} REG=$Wn REG=$Wm          ', 'size=0b11 sf=0b0 ', 'whilelo_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELO VREG={REG:i64=$Pd VDS=D} REG=$Xn REG=$Xm          ', 'size=0b11 sf=0b1 ', 'whilelo_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELS VREG={REG:s8=$Pd   VDS=B} REG:s32=$Wn REG:s32=$Wm ', 'size=0b00 sf=0b0 ', 'whilels_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELS VREG={REG:s8=$Pd   VDS=B} REG:s64=$Xn REG:s64=$Xm ', 'size=0b00 sf=0b1 ', 'whilels_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELS VREG={REG:s16=$Pd VDS=H} REG:s32=$Wn REG:s32=$Wm  ', 'size=0b01 sf=0b0 ', 'whilels_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELS VREG={REG:s16=$Pd VDS=H} REG:s64=$Xn REG:s64=$Xm  ', 'size=0b01 sf=0b1 ', 'whilels_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELS VREG={REG:s32=$Pd VDS=S} REG:s32=$Wn REG:s32=$Wm  ', 'size=0b10 sf=0b0 ', 'whilels_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELS VREG={REG:s32=$Pd VDS=S} REG:s64=$Xn REG:s64=$Xm  ', 'size=0b10 sf=0b1 ', 'whilels_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELS VREG={REG:s64=$Pd VDS=D} REG:s32=$Wn REG:s32=$Wm  ', 'size=0b11 sf=0b0 ', 'whilels_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELS VREG={REG:s64=$Pd VDS=D} REG:s64=$Xn REG:s64=$Xm  ', 'size=0b11 sf=0b1 ', 'whilels_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELT VREG={REG:i8=$Pd   VDS=B} REG=$Wn REG=$Wm         ', 'size=0b00 sf=0b0 ', 'whilelt_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELT VREG={REG:i8=$Pd   VDS=B} REG=$Xn REG=$Xm         ', 'size=0b00 sf=0b1 ', 'whilelt_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELT VREG={REG:i16=$Pd VDS=H} REG=$Wn REG=$Wm          ', 'size=0b01 sf=0b0 ', 'whilelt_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELT VREG={REG:i16=$Pd VDS=H} REG=$Xn REG=$Xm          ', 'size=0b01 sf=0b1 ', 'whilelt_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELT VREG={REG:i32=$Pd VDS=S} REG=$Wn REG=$Wm          ', 'size=0b10 sf=0b0 ', 'whilelt_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELT VREG={REG:i32=$Pd VDS=S} REG=$Xn REG=$Xm          ', 'size=0b10 sf=0b1 ', 'whilelt_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELT VREG={REG:i64=$Pd VDS=D} REG=$Wn REG=$Wm          ', 'size=0b11 sf=0b0 ', 'whilelt_p_p_rr_ ', 'extensions=SVE  '];
T[' WHILELT VREG={REG:i64=$Pd VDS=D} REG=$Xn REG=$Xm          ', 'size=0b11 sf=0b1 ', 'whilelt_p_p_rr_ ', 'extensions=SVE  '];



# sve_int_whilenc: SVE pointer conflict compare.
# A64: ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b001100 Rn:u=0bxxxxx rw:u=0bx Pd:u=0bxxxx
HINT('ICLASS', 'sve_int_whilenc', 'A64');

DEF Pd => PRDREG('Pd', 'Pd', 'write') ;
DEF Xm => GPR64('Xm', 'Rm', 'read')   ;
DEF Xn => GPR64('Xn', 'Rn', 'read')   ;

T[' WHILERW VREG={REG:i8=$Pd   VDS=B} REG=$Xn REG=$Xm ', 'size=0b00 ', 'whilerw_p_rr_ ', 'extensions=SVE2 '];
T[' WHILERW VREG={REG:i16=$Pd VDS=H} REG=$Xn REG=$Xm  ', 'size=0b01 ', 'whilerw_p_rr_ ', 'extensions=SVE2 '];
T[' WHILERW VREG={REG:i32=$Pd VDS=S} REG=$Xn REG=$Xm  ', 'size=0b10 ', 'whilerw_p_rr_ ', 'extensions=SVE2 '];
T[' WHILERW VREG={REG:i64=$Pd VDS=D} REG=$Xn REG=$Xm  ', 'size=0b11 ', 'whilerw_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEWR VREG={REG:i8=$Pd   VDS=B} REG=$Xn REG=$Xm ', 'size=0b00 ', 'whilewr_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEWR VREG={REG:i16=$Pd VDS=H} REG=$Xn REG=$Xm  ', 'size=0b01 ', 'whilewr_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEWR VREG={REG:i32=$Pd VDS=S} REG=$Xn REG=$Xm  ', 'size=0b10 ', 'whilewr_p_rr_ ', 'extensions=SVE2 '];
T[' WHILEWR VREG={REG:i64=$Pd VDS=D} REG=$Xn REG=$Xm  ', 'size=0b11 ', 'whilewr_p_rr_ ', 'extensions=SVE2 '];



# sve_int_wrffr: SVE FFR write from predicate.
# A64: ig0=0b00 ig1=0b100101 opc:u=0bxx ig2=0b10 ig3=0b1000100100 ig4=0b0 Pn:u=0bxxxx ig5=0b0 ig6=0b0 ig7=0b0 ig8=0b00
HINT('ICLASS', 'sve_int_wrffr', 'A64');

DEF Pn => PRDREG('Pn', 'Pn', 'read') ;

T[' WRFFR VREG={REG:i8=$Pn   VDS=B} ', 'NONE ', 'wrffr_f_p_ ', 'extensions=SVE '];



# sve_intx_aba: SVE2 integer absolute difference and accumulate.
# A64: ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b11111 U:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_aba', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' SABA VREG={REG:s8=$Zda   VDS=B} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'saba_z_zzz_ ', 'extensions=SVE2 '];
T[' SABA VREG={REG:s16=$Zda VDS=H} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'saba_z_zzz_ ', 'extensions=SVE2 '];
T[' SABA VREG={REG:s32=$Zda VDS=S} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'saba_z_zzz_ ', 'extensions=SVE2 '];
T[' SABA VREG={REG:s64=$Zda VDS=D} VREG={REG:s64=$Zn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'saba_z_zzz_ ', 'extensions=SVE2 '];
T[' UABA VREG={REG:u8=$Zda   VDS=B} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'uaba_z_zzz_ ', 'extensions=SVE2 '];
T[' UABA VREG={REG:u16=$Zda VDS=H} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'uaba_z_zzz_ ', 'extensions=SVE2 '];
T[' UABA VREG={REG:u32=$Zda VDS=S} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'uaba_z_zzz_ ', 'extensions=SVE2 '];
T[' UABA VREG={REG:u64=$Zda VDS=D} VREG={REG:u64=$Zn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'uaba_z_zzz_ ', 'extensions=SVE2 '];



# sve_intx_aba_long: SVE2 integer absolute difference and accumulate long.
# A64: ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1100 U:u=0bx T:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_aba_long', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' SABALB VREG={REG:s16=$Zda VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'sabalb_z_zzz_ ', 'extensions=SVE2 '];
T[' SABALB VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'sabalb_z_zzz_ ', 'extensions=SVE2 '];
T[' SABALB VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'sabalb_z_zzz_ ', 'extensions=SVE2 '];
T[' SABALT VREG={REG:s16=$Zda VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'sabalt_z_zzz_ ', 'extensions=SVE2 '];
T[' SABALT VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'sabalt_z_zzz_ ', 'extensions=SVE2 '];
T[' SABALT VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'sabalt_z_zzz_ ', 'extensions=SVE2 '];
T[' UABALB VREG={REG:u16=$Zda VDS=H} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'uabalb_z_zzz_ ', 'extensions=SVE2 '];
T[' UABALB VREG={REG:u32=$Zda VDS=S} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}   ', 'size=0b10 ', 'uabalb_z_zzz_ ', 'extensions=SVE2 '];
T[' UABALB VREG={REG:u64=$Zda VDS=D} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}   ', 'size=0b11 ', 'uabalb_z_zzz_ ', 'extensions=SVE2 '];
T[' UABALT VREG={REG:u16=$Zda VDS=H} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'uabalt_z_zzz_ ', 'extensions=SVE2 '];
T[' UABALT VREG={REG:u32=$Zda VDS=S} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}   ', 'size=0b10 ', 'uabalt_z_zzz_ ', 'extensions=SVE2 '];
T[' UABALT VREG={REG:u64=$Zda VDS=D} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}   ', 'size=0b11 ', 'uabalt_z_zzz_ ', 'extensions=SVE2 '];



# sve_intx_accumulate_long_pairs: SVE2 integer pairwise add and accumulate long.
# A64: ig0=0b01000100 size:u=0bxx ig1=0b00010 U:u=0bx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_accumulate_long_pairs', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' SADALP VREG={REG:s16=$Zda VDS=H} PREG:M=$Pg VREG={REG:s8=$Zn   VDS=B} ', 'size=0b01 ', 'sadalp_z_p_z_ ', 'extensions=SVE2 '];
T[' SADALP VREG={REG:s32=$Zda VDS=S} PREG:M=$Pg VREG={REG:s16=$Zn VDS=H}  ', 'size=0b10 ', 'sadalp_z_p_z_ ', 'extensions=SVE2 '];
T[' SADALP VREG={REG:s64=$Zda VDS=D} PREG:M=$Pg VREG={REG:s32=$Zn VDS=S}  ', 'size=0b11 ', 'sadalp_z_p_z_ ', 'extensions=SVE2 '];
T[' UADALP VREG={REG:u16=$Zda VDS=H} PREG:M=$Pg VREG={REG:u8=$Zn   VDS=B} ', 'size=0b01 ', 'uadalp_z_p_z_ ', 'extensions=SVE2 '];
T[' UADALP VREG={REG:u32=$Zda VDS=S} PREG:M=$Pg VREG={REG:u16=$Zn VDS=H}  ', 'size=0b10 ', 'uadalp_z_p_z_ ', 'extensions=SVE2 '];
T[' UADALP VREG={REG:u64=$Zda VDS=D} PREG:M=$Pg VREG={REG:u32=$Zn VDS=S}  ', 'size=0b11 ', 'uadalp_z_p_z_ ', 'extensions=SVE2 '];



# sve_intx_adc_long: SVE2 integer add/subtract long with carry.
# A64: ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b11010 T:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_adc_long', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' ADCLB VREG={REG:i32=$Zda VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S} ', 'sz=0b0 ', 'adclb_z_zzz_ ', 'extensions=SVE2 '];
T[' ADCLB VREG={REG:i64=$Zda VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D} ', 'sz=0b1 ', 'adclb_z_zzz_ ', 'extensions=SVE2 '];
T[' ADCLT VREG={REG:i32=$Zda VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S} ', 'sz=0b0 ', 'adclt_z_zzz_ ', 'extensions=SVE2 '];
T[' ADCLT VREG={REG:i64=$Zda VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D} ', 'sz=0b1 ', 'adclt_z_zzz_ ', 'extensions=SVE2 '];
T[' SBCLB VREG={REG:i32=$Zda VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S} ', 'sz=0b0 ', 'sbclb_z_zzz_ ', 'extensions=SVE2 '];
T[' SBCLB VREG={REG:i64=$Zda VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D} ', 'sz=0b1 ', 'sbclb_z_zzz_ ', 'extensions=SVE2 '];
T[' SBCLT VREG={REG:i32=$Zda VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S} ', 'sz=0b0 ', 'sbclt_z_zzz_ ', 'extensions=SVE2 '];
T[' SBCLT VREG={REG:i64=$Zda VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D} ', 'sz=0b1 ', 'sbclt_z_zzz_ ', 'extensions=SVE2 '];



# sve_intx_arith_binary_pairs: SVE2 integer pairwise arithmetic.
# A64: ig0=0b01000100 size:u=0bxx ig1=0b010 opc:u=0bxx U:u=0bx ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_intx_arith_binary_pairs', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' ADDP  VREG={REG:i8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'addp_z_p_zz_  ', 'extensions=SVE2 '];
T[' ADDP  VREG={REG:i16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'addp_z_p_zz_  ', 'extensions=SVE2 '];
T[' ADDP  VREG={REG:i32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'addp_z_p_zz_  ', 'extensions=SVE2 '];
T[' ADDP  VREG={REG:i64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'addp_z_p_zz_  ', 'extensions=SVE2 '];
T[' SMAXP VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'smaxp_z_p_zz_ ', 'extensions=SVE2 '];
T[' SMAXP VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'smaxp_z_p_zz_ ', 'extensions=SVE2 '];
T[' SMAXP VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'smaxp_z_p_zz_ ', 'extensions=SVE2 '];
T[' SMAXP VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'smaxp_z_p_zz_ ', 'extensions=SVE2 '];
T[' SMINP VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'sminp_z_p_zz_ ', 'extensions=SVE2 '];
T[' SMINP VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'sminp_z_p_zz_ ', 'extensions=SVE2 '];
T[' SMINP VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'sminp_z_p_zz_ ', 'extensions=SVE2 '];
T[' SMINP VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'sminp_z_p_zz_ ', 'extensions=SVE2 '];
T[' UMAXP VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'umaxp_z_p_zz_ ', 'extensions=SVE2 '];
T[' UMAXP VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'umaxp_z_p_zz_ ', 'extensions=SVE2 '];
T[' UMAXP VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'umaxp_z_p_zz_ ', 'extensions=SVE2 '];
T[' UMAXP VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'umaxp_z_p_zz_ ', 'extensions=SVE2 '];
T[' UMINP VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'uminp_z_p_zz_ ', 'extensions=SVE2 '];
T[' UMINP VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'uminp_z_p_zz_ ', 'extensions=SVE2 '];
T[' UMINP VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'uminp_z_p_zz_ ', 'extensions=SVE2 '];
T[' UMINP VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'uminp_z_p_zz_ ', 'extensions=SVE2 '];



# sve_intx_arith_narrow: SVE2 integer add/subtract narrow high part.
# A64: ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S:u=0bx R:u=0bx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_arith_narrow', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' ADDHNB  VREG={REG:i8=$Zd   VDS=B} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H} ', 'size=0b01 ', 'addhnb_z_zz_  ', 'extensions=SVE2 '];
T[' ADDHNB  VREG={REG:i16=$Zd VDS=H} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}  ', 'size=0b10 ', 'addhnb_z_zz_  ', 'extensions=SVE2 '];
T[' ADDHNB  VREG={REG:i32=$Zd VDS=S} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}  ', 'size=0b11 ', 'addhnb_z_zz_  ', 'extensions=SVE2 '];
T[' ADDHNT  VREG={REG:i8=$Zd   VDS=B} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H} ', 'size=0b01 ', 'addhnt_z_zz_  ', 'extensions=SVE2 '];
T[' ADDHNT  VREG={REG:i16=$Zd VDS=H} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}  ', 'size=0b10 ', 'addhnt_z_zz_  ', 'extensions=SVE2 '];
T[' ADDHNT  VREG={REG:i32=$Zd VDS=S} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}  ', 'size=0b11 ', 'addhnt_z_zz_  ', 'extensions=SVE2 '];
T[' RADDHNB VREG={REG:i8=$Zd   VDS=B} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H} ', 'size=0b01 ', 'raddhnb_z_zz_ ', 'extensions=SVE2 '];
T[' RADDHNB VREG={REG:i16=$Zd VDS=H} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}  ', 'size=0b10 ', 'raddhnb_z_zz_ ', 'extensions=SVE2 '];
T[' RADDHNB VREG={REG:i32=$Zd VDS=S} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}  ', 'size=0b11 ', 'raddhnb_z_zz_ ', 'extensions=SVE2 '];
T[' RADDHNT VREG={REG:i8=$Zd   VDS=B} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H} ', 'size=0b01 ', 'raddhnt_z_zz_ ', 'extensions=SVE2 '];
T[' RADDHNT VREG={REG:i16=$Zd VDS=H} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}  ', 'size=0b10 ', 'raddhnt_z_zz_ ', 'extensions=SVE2 '];
T[' RADDHNT VREG={REG:i32=$Zd VDS=S} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}  ', 'size=0b11 ', 'raddhnt_z_zz_ ', 'extensions=SVE2 '];
T[' RSUBHNB VREG={REG:i8=$Zd   VDS=B} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H} ', 'size=0b01 ', 'rsubhnb_z_zz_ ', 'extensions=SVE2 '];
T[' RSUBHNB VREG={REG:i16=$Zd VDS=H} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}  ', 'size=0b10 ', 'rsubhnb_z_zz_ ', 'extensions=SVE2 '];
T[' RSUBHNB VREG={REG:i32=$Zd VDS=S} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}  ', 'size=0b11 ', 'rsubhnb_z_zz_ ', 'extensions=SVE2 '];
T[' RSUBHNT VREG={REG:i8=$Zd   VDS=B} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H} ', 'size=0b01 ', 'rsubhnt_z_zz_ ', 'extensions=SVE2 '];
T[' RSUBHNT VREG={REG:i16=$Zd VDS=H} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}  ', 'size=0b10 ', 'rsubhnt_z_zz_ ', 'extensions=SVE2 '];
T[' RSUBHNT VREG={REG:i32=$Zd VDS=S} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}  ', 'size=0b11 ', 'rsubhnt_z_zz_ ', 'extensions=SVE2 '];
T[' SUBHNB  VREG={REG:i8=$Zd   VDS=B} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H} ', 'size=0b01 ', 'subhnb_z_zz_  ', 'extensions=SVE2 '];
T[' SUBHNB  VREG={REG:i16=$Zd VDS=H} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}  ', 'size=0b10 ', 'subhnb_z_zz_  ', 'extensions=SVE2 '];
T[' SUBHNB  VREG={REG:i32=$Zd VDS=S} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}  ', 'size=0b11 ', 'subhnb_z_zz_  ', 'extensions=SVE2 '];
T[' SUBHNT  VREG={REG:i8=$Zd   VDS=B} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H} ', 'size=0b01 ', 'subhnt_z_zz_  ', 'extensions=SVE2 '];
T[' SUBHNT  VREG={REG:i16=$Zd VDS=H} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}  ', 'size=0b10 ', 'subhnt_z_zz_  ', 'extensions=SVE2 '];
T[' SUBHNT  VREG={REG:i32=$Zd VDS=S} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}  ', 'size=0b11 ', 'subhnt_z_zz_  ', 'extensions=SVE2 '];



# sve_intx_bin_pred_shift_sat_round: SVE2 saturating/rounding bitwise shift left (predicated).
# A64: ig0=0b01000100 size:u=0bxx ig1=0b00 Q:u=0bx R:u=0bx N:u=0bx U:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_intx_bin_pred_shift_sat_round', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' SQRSHL  VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'sqrshl_z_p_zz_  ', 'extensions=SVE2 '];
T[' SQRSHL  VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'sqrshl_z_p_zz_  ', 'extensions=SVE2 '];
T[' SQRSHL  VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'sqrshl_z_p_zz_  ', 'extensions=SVE2 '];
T[' SQRSHL  VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'sqrshl_z_p_zz_  ', 'extensions=SVE2 '];
T[' SQRSHLR VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'sqrshlr_z_p_zz_ ', 'extensions=SVE2 '];
T[' SQRSHLR VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'sqrshlr_z_p_zz_ ', 'extensions=SVE2 '];
T[' SQRSHLR VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'sqrshlr_z_p_zz_ ', 'extensions=SVE2 '];
T[' SQRSHLR VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'sqrshlr_z_p_zz_ ', 'extensions=SVE2 '];
T[' SQSHL   VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'sqshl_z_p_zz_   ', 'extensions=SVE2 '];
T[' SQSHL   VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'sqshl_z_p_zz_   ', 'extensions=SVE2 '];
T[' SQSHL   VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'sqshl_z_p_zz_   ', 'extensions=SVE2 '];
T[' SQSHL   VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'sqshl_z_p_zz_   ', 'extensions=SVE2 '];
T[' SQSHLR  VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'sqshlr_z_p_zz_  ', 'extensions=SVE2 '];
T[' SQSHLR  VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'sqshlr_z_p_zz_  ', 'extensions=SVE2 '];
T[' SQSHLR  VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'sqshlr_z_p_zz_  ', 'extensions=SVE2 '];
T[' SQSHLR  VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'sqshlr_z_p_zz_  ', 'extensions=SVE2 '];
T[' SRSHL   VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'srshl_z_p_zz_   ', 'extensions=SVE2 '];
T[' SRSHL   VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'srshl_z_p_zz_   ', 'extensions=SVE2 '];
T[' SRSHL   VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'srshl_z_p_zz_   ', 'extensions=SVE2 '];
T[' SRSHL   VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'srshl_z_p_zz_   ', 'extensions=SVE2 '];
T[' SRSHLR  VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'srshlr_z_p_zz_  ', 'extensions=SVE2 '];
T[' SRSHLR  VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'srshlr_z_p_zz_  ', 'extensions=SVE2 '];
T[' SRSHLR  VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'srshlr_z_p_zz_  ', 'extensions=SVE2 '];
T[' SRSHLR  VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'srshlr_z_p_zz_  ', 'extensions=SVE2 '];
T[' UQRSHL  VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'uqrshl_z_p_zz_  ', 'extensions=SVE2 '];
T[' UQRSHL  VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'uqrshl_z_p_zz_  ', 'extensions=SVE2 '];
T[' UQRSHL  VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'uqrshl_z_p_zz_  ', 'extensions=SVE2 '];
T[' UQRSHL  VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'uqrshl_z_p_zz_  ', 'extensions=SVE2 '];
T[' UQRSHLR VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'uqrshlr_z_p_zz_ ', 'extensions=SVE2 '];
T[' UQRSHLR VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'uqrshlr_z_p_zz_ ', 'extensions=SVE2 '];
T[' UQRSHLR VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'uqrshlr_z_p_zz_ ', 'extensions=SVE2 '];
T[' UQRSHLR VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'uqrshlr_z_p_zz_ ', 'extensions=SVE2 '];
T[' UQSHL   VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'uqshl_z_p_zz_   ', 'extensions=SVE2 '];
T[' UQSHL   VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'uqshl_z_p_zz_   ', 'extensions=SVE2 '];
T[' UQSHL   VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'uqshl_z_p_zz_   ', 'extensions=SVE2 '];
T[' UQSHL   VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'uqshl_z_p_zz_   ', 'extensions=SVE2 '];
T[' UQSHLR  VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'uqshlr_z_p_zz_  ', 'extensions=SVE2 '];
T[' UQSHLR  VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'uqshlr_z_p_zz_  ', 'extensions=SVE2 '];
T[' UQSHLR  VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'uqshlr_z_p_zz_  ', 'extensions=SVE2 '];
T[' UQSHLR  VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'uqshlr_z_p_zz_  ', 'extensions=SVE2 '];
T[' URSHL   VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'urshl_z_p_zz_   ', 'extensions=SVE2 '];
T[' URSHL   VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'urshl_z_p_zz_   ', 'extensions=SVE2 '];
T[' URSHL   VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'urshl_z_p_zz_   ', 'extensions=SVE2 '];
T[' URSHL   VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'urshl_z_p_zz_   ', 'extensions=SVE2 '];
T[' URSHLR  VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'urshlr_z_p_zz_  ', 'extensions=SVE2 '];
T[' URSHLR  VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'urshlr_z_p_zz_  ', 'extensions=SVE2 '];
T[' URSHLR  VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'urshlr_z_p_zz_  ', 'extensions=SVE2 '];
T[' URSHLR  VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'urshlr_z_p_zz_  ', 'extensions=SVE2 '];



# sve_intx_cadd: SVE2 complex integer add.
# A64: ig0=0b01000101 size:u=0bxx ig1=0b00000 op:u=0bx ig2=0b11011 rot:u=0bx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_intx_cadd', 'A64');

DEF Zdn   => SVEREG('Zdn', 'Zdn', 'read|write')       ;
DEF Zm    => SVEREG('Zm', 'Zm', 'read')               ;
DEF const => IMM('const', 'rot', table => 'tbl_imm4') ;

T[' CADD   VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} IMM:i8=$const ', 'size=0b00 ', 'cadd_z_zz_   ', 'extensions=SVE2 '];
T[' CADD   VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H} IMM:i16=$const   ', 'size=0b01 ', 'cadd_z_zz_   ', 'extensions=SVE2 '];
T[' CADD   VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S} IMM:i32=$const   ', 'size=0b10 ', 'cadd_z_zz_   ', 'extensions=SVE2 '];
T[' CADD   VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D} IMM:i64=$const   ', 'size=0b11 ', 'cadd_z_zz_   ', 'extensions=SVE2 '];
T[' SQCADD VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zdn   VDS=B} VREG={REG:i8=$Zm   VDS=B} IMM:i8=$const ', 'size=0b00 ', 'sqcadd_z_zz_ ', 'extensions=SVE2 '];
T[' SQCADD VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zdn VDS=H} VREG={REG:i16=$Zm VDS=H} IMM:i16=$const   ', 'size=0b01 ', 'sqcadd_z_zz_ ', 'extensions=SVE2 '];
T[' SQCADD VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zdn VDS=S} VREG={REG:i32=$Zm VDS=S} IMM:i32=$const   ', 'size=0b10 ', 'sqcadd_z_zz_ ', 'extensions=SVE2 '];
T[' SQCADD VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zdn VDS=D} VREG={REG:i64=$Zm VDS=D} IMM:i64=$const   ', 'size=0b11 ', 'sqcadd_z_zz_ ', 'extensions=SVE2 '];



# sve_intx_cdot: SVE2 complex integer dot product.
# A64: ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0001 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_cdot', 'A64');

DEF Zda   => SVEREG('Zda', 'Zda', 'read|write')       ;
DEF Zm    => SVEREG('Zm', 'Zm', 'read')               ;
DEF Zn    => SVEREG('Zn', 'Zn', 'read')               ;
DEF const => IMM('const', 'rot', table => 'tbl_imm0') ;

T[' CDOT VREG={REG:i32=$Zda VDS=S} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} IMM:i8=$const ', 'size=0bx0 ', 'cdot_z_zzz_ ', 'extensions=SVE2 '];
T[' CDOT VREG={REG:i64=$Zda VDS=D} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H} IMM:i16=$const  ', 'size=0bx1 ', 'cdot_z_zzz_ ', 'extensions=SVE2 '];



# sve_intx_cdot_by_indexed_elem: SVE2 complex integer dot product (indexed).
# A64: ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b0100 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_cdot_by_indexed_elem', 'A64');

DEF Zda   => SVEREG('Zda', 'Zda', 'read|write')       ;
DEF Zm    => SVEREG('Zm', 'Zm', 'read')               ;
DEF Zn    => SVEREG('Zn', 'Zn', 'read')               ;
DEF const => IMM('const', 'rot', table => 'tbl_imm0') ;

T[' CDOT VREG={REG:i32=$Zda VDS=S} VREG={REG:i8=$Zn   VDS=B} EREG={REG:i8=$Zm   VDS=B IDX={ENCODEDIN=i2}} IMM:i8=$const ', 'NONE ', 'cdot_z_zzzi_s ', 'extensions=SVE2 '];
T[' CDOT VREG={REG:i64=$Zda VDS=D} VREG={REG:i16=$Zn VDS=H} EREG={REG:i16=$Zm VDS=H IDX={ENCODEDIN=i1}} IMM:i16=$const  ', 'NONE ', 'cdot_z_zzzi_d ', 'extensions=SVE2 '];



# sve_intx_clong: SVE2 integer add/subtract interleaved long.
# A64: ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1000 S:u=0bx tb:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_clong', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' SADDLBT VREG={REG:s16=$Zd VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'saddlbt_z_zz_ ', 'extensions=SVE2 '];
T[' SADDLBT VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'saddlbt_z_zz_ ', 'extensions=SVE2 '];
T[' SADDLBT VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'saddlbt_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBLBT VREG={REG:s16=$Zd VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'ssublbt_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBLBT VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'ssublbt_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBLBT VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'ssublbt_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBLTB VREG={REG:s16=$Zd VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'ssubltb_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBLTB VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'ssubltb_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBLTB VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'ssubltb_z_zz_ ', 'extensions=SVE2 '];



# sve_intx_cmla: SVE2 complex integer multiply-add.
# A64: ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b001 op:u=0bx rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_cmla', 'A64');

DEF Zda   => SVEREG('Zda', 'Zda', 'read|write')       ;
DEF Zm    => SVEREG('Zm', 'Zm', 'read')               ;
DEF Zn    => SVEREG('Zn', 'Zn', 'read')               ;
DEF const => IMM('const', 'rot', table => 'tbl_imm0') ;

T[' CMLA      VREG={REG:i8=$Zda   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} IMM:i8=$const ', 'size=0b00 ', 'cmla_z_zzz_      ', 'extensions=SVE2 '];
T[' CMLA      VREG={REG:i16=$Zda VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H} IMM:i16=$const   ', 'size=0b01 ', 'cmla_z_zzz_      ', 'extensions=SVE2 '];
T[' CMLA      VREG={REG:i32=$Zda VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S} IMM:i32=$const   ', 'size=0b10 ', 'cmla_z_zzz_      ', 'extensions=SVE2 '];
T[' CMLA      VREG={REG:i64=$Zda VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D} IMM:i64=$const   ', 'size=0b11 ', 'cmla_z_zzz_      ', 'extensions=SVE2 '];
T[' SQRDCMLAH VREG={REG:i8=$Zda   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} IMM:i8=$const ', 'size=0b00 ', 'sqrdcmlah_z_zzz_ ', 'extensions=SVE2 '];
T[' SQRDCMLAH VREG={REG:i16=$Zda VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H} IMM:i16=$const   ', 'size=0b01 ', 'sqrdcmlah_z_zzz_ ', 'extensions=SVE2 '];
T[' SQRDCMLAH VREG={REG:i32=$Zda VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S} IMM:i32=$const   ', 'size=0b10 ', 'sqrdcmlah_z_zzz_ ', 'extensions=SVE2 '];
T[' SQRDCMLAH VREG={REG:i64=$Zda VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D} IMM:i64=$const   ', 'size=0b11 ', 'sqrdcmlah_z_zzz_ ', 'extensions=SVE2 '];



# sve_intx_cmla_by_indexed_elem: SVE2 complex integer multiply-add (indexed).
# A64: ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b0110 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_cmla_by_indexed_elem', 'A64');

DEF Zda   => SVEREG('Zda', 'Zda', 'read|write')       ;
DEF Zm    => SVEREG('Zm', 'Zm', 'read')               ;
DEF Zn    => SVEREG('Zn', 'Zn', 'read')               ;
DEF const => IMM('const', 'rot', table => 'tbl_imm0') ;

T[' CMLA VREG={REG:i16=$Zda VDS=H} VREG={REG:i16=$Zn VDS=H} EREG={REG:i16=$Zm VDS=H IDX={ENCODEDIN=i2}} IMM:i16=$const ', 'NONE ', 'cmla_z_zzzi_h ', 'extensions=SVE2 '];
T[' CMLA VREG={REG:i32=$Zda VDS=S} VREG={REG:i32=$Zn VDS=S} EREG={REG:i32=$Zm VDS=S IDX={ENCODEDIN=i1}} IMM:i32=$const ', 'NONE ', 'cmla_z_zzzi_s ', 'extensions=SVE2 '];



# sve_intx_cons_arith_long: SVE2 integer add/subtract long.
# A64: ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op:u=0bx S:u=0bx U:u=0bx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_cons_arith_long', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' SABDLB VREG={REG:s16=$Zd VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'sabdlb_z_zz_ ', 'extensions=SVE2 '];
T[' SABDLB VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'sabdlb_z_zz_ ', 'extensions=SVE2 '];
T[' SABDLB VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'sabdlb_z_zz_ ', 'extensions=SVE2 '];
T[' SABDLT VREG={REG:s16=$Zd VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'sabdlt_z_zz_ ', 'extensions=SVE2 '];
T[' SABDLT VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'sabdlt_z_zz_ ', 'extensions=SVE2 '];
T[' SABDLT VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'sabdlt_z_zz_ ', 'extensions=SVE2 '];
T[' SADDLB VREG={REG:s16=$Zd VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'saddlb_z_zz_ ', 'extensions=SVE2 '];
T[' SADDLB VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'saddlb_z_zz_ ', 'extensions=SVE2 '];
T[' SADDLB VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'saddlb_z_zz_ ', 'extensions=SVE2 '];
T[' SADDLT VREG={REG:s16=$Zd VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'saddlt_z_zz_ ', 'extensions=SVE2 '];
T[' SADDLT VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'saddlt_z_zz_ ', 'extensions=SVE2 '];
T[' SADDLT VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'saddlt_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBLB VREG={REG:s16=$Zd VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'ssublb_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBLB VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'ssublb_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBLB VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'ssublb_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBLT VREG={REG:s16=$Zd VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'ssublt_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBLT VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'ssublt_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBLT VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'ssublt_z_zz_ ', 'extensions=SVE2 '];
T[' UABDLB VREG={REG:u16=$Zd VDS=H} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'uabdlb_z_zz_ ', 'extensions=SVE2 '];
T[' UABDLB VREG={REG:u32=$Zd VDS=S} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}   ', 'size=0b10 ', 'uabdlb_z_zz_ ', 'extensions=SVE2 '];
T[' UABDLB VREG={REG:u64=$Zd VDS=D} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}   ', 'size=0b11 ', 'uabdlb_z_zz_ ', 'extensions=SVE2 '];
T[' UABDLT VREG={REG:u16=$Zd VDS=H} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'uabdlt_z_zz_ ', 'extensions=SVE2 '];
T[' UABDLT VREG={REG:u32=$Zd VDS=S} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}   ', 'size=0b10 ', 'uabdlt_z_zz_ ', 'extensions=SVE2 '];
T[' UABDLT VREG={REG:u64=$Zd VDS=D} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}   ', 'size=0b11 ', 'uabdlt_z_zz_ ', 'extensions=SVE2 '];
T[' UADDLB VREG={REG:u16=$Zd VDS=H} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'uaddlb_z_zz_ ', 'extensions=SVE2 '];
T[' UADDLB VREG={REG:u32=$Zd VDS=S} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}   ', 'size=0b10 ', 'uaddlb_z_zz_ ', 'extensions=SVE2 '];
T[' UADDLB VREG={REG:u64=$Zd VDS=D} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}   ', 'size=0b11 ', 'uaddlb_z_zz_ ', 'extensions=SVE2 '];
T[' UADDLT VREG={REG:u16=$Zd VDS=H} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'uaddlt_z_zz_ ', 'extensions=SVE2 '];
T[' UADDLT VREG={REG:u32=$Zd VDS=S} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}   ', 'size=0b10 ', 'uaddlt_z_zz_ ', 'extensions=SVE2 '];
T[' UADDLT VREG={REG:u64=$Zd VDS=D} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}   ', 'size=0b11 ', 'uaddlt_z_zz_ ', 'extensions=SVE2 '];
T[' USUBLB VREG={REG:u16=$Zd VDS=H} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'usublb_z_zz_ ', 'extensions=SVE2 '];
T[' USUBLB VREG={REG:u32=$Zd VDS=S} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}   ', 'size=0b10 ', 'usublb_z_zz_ ', 'extensions=SVE2 '];
T[' USUBLB VREG={REG:u64=$Zd VDS=D} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}   ', 'size=0b11 ', 'usublb_z_zz_ ', 'extensions=SVE2 '];
T[' USUBLT VREG={REG:u16=$Zd VDS=H} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'usublt_z_zz_ ', 'extensions=SVE2 '];
T[' USUBLT VREG={REG:u32=$Zd VDS=S} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}   ', 'size=0b10 ', 'usublt_z_zz_ ', 'extensions=SVE2 '];
T[' USUBLT VREG={REG:u64=$Zd VDS=D} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}   ', 'size=0b11 ', 'usublt_z_zz_ ', 'extensions=SVE2 '];



# sve_intx_cons_arith_wide: SVE2 integer add/subtract wide.
# A64: ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S:u=0bx U:u=0bx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_cons_arith_wide', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' SADDWB VREG={REG:s16=$Zd VDS=H} VREG={REG:s16=$Zn VDS=H} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'saddwb_z_zz_ ', 'extensions=SVE2 '];
T[' SADDWB VREG={REG:s32=$Zd VDS=S} VREG={REG:s32=$Zn VDS=S} VREG={REG:s16=$Zm VDS=H}  ', 'size=0b10 ', 'saddwb_z_zz_ ', 'extensions=SVE2 '];
T[' SADDWB VREG={REG:s64=$Zd VDS=D} VREG={REG:s64=$Zn VDS=D} VREG={REG:s32=$Zm VDS=S}  ', 'size=0b11 ', 'saddwb_z_zz_ ', 'extensions=SVE2 '];
T[' SADDWT VREG={REG:s16=$Zd VDS=H} VREG={REG:s16=$Zn VDS=H} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'saddwt_z_zz_ ', 'extensions=SVE2 '];
T[' SADDWT VREG={REG:s32=$Zd VDS=S} VREG={REG:s32=$Zn VDS=S} VREG={REG:s16=$Zm VDS=H}  ', 'size=0b10 ', 'saddwt_z_zz_ ', 'extensions=SVE2 '];
T[' SADDWT VREG={REG:s64=$Zd VDS=D} VREG={REG:s64=$Zn VDS=D} VREG={REG:s32=$Zm VDS=S}  ', 'size=0b11 ', 'saddwt_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBWB VREG={REG:s16=$Zd VDS=H} VREG={REG:s16=$Zn VDS=H} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'ssubwb_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBWB VREG={REG:s32=$Zd VDS=S} VREG={REG:s32=$Zn VDS=S} VREG={REG:s16=$Zm VDS=H}  ', 'size=0b10 ', 'ssubwb_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBWB VREG={REG:s64=$Zd VDS=D} VREG={REG:s64=$Zn VDS=D} VREG={REG:s32=$Zm VDS=S}  ', 'size=0b11 ', 'ssubwb_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBWT VREG={REG:s16=$Zd VDS=H} VREG={REG:s16=$Zn VDS=H} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'ssubwt_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBWT VREG={REG:s32=$Zd VDS=S} VREG={REG:s32=$Zn VDS=S} VREG={REG:s16=$Zm VDS=H}  ', 'size=0b10 ', 'ssubwt_z_zz_ ', 'extensions=SVE2 '];
T[' SSUBWT VREG={REG:s64=$Zd VDS=D} VREG={REG:s64=$Zn VDS=D} VREG={REG:s32=$Zm VDS=S}  ', 'size=0b11 ', 'ssubwt_z_zz_ ', 'extensions=SVE2 '];
T[' UADDWB VREG={REG:u16=$Zd VDS=H} VREG={REG:u16=$Zn VDS=H} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'uaddwb_z_zz_ ', 'extensions=SVE2 '];
T[' UADDWB VREG={REG:u32=$Zd VDS=S} VREG={REG:u32=$Zn VDS=S} VREG={REG:u16=$Zm VDS=H}  ', 'size=0b10 ', 'uaddwb_z_zz_ ', 'extensions=SVE2 '];
T[' UADDWB VREG={REG:u64=$Zd VDS=D} VREG={REG:u64=$Zn VDS=D} VREG={REG:u32=$Zm VDS=S}  ', 'size=0b11 ', 'uaddwb_z_zz_ ', 'extensions=SVE2 '];
T[' UADDWT VREG={REG:u16=$Zd VDS=H} VREG={REG:u16=$Zn VDS=H} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'uaddwt_z_zz_ ', 'extensions=SVE2 '];
T[' UADDWT VREG={REG:u32=$Zd VDS=S} VREG={REG:u32=$Zn VDS=S} VREG={REG:u16=$Zm VDS=H}  ', 'size=0b10 ', 'uaddwt_z_zz_ ', 'extensions=SVE2 '];
T[' UADDWT VREG={REG:u64=$Zd VDS=D} VREG={REG:u64=$Zn VDS=D} VREG={REG:u32=$Zm VDS=S}  ', 'size=0b11 ', 'uaddwt_z_zz_ ', 'extensions=SVE2 '];
T[' USUBWB VREG={REG:u16=$Zd VDS=H} VREG={REG:u16=$Zn VDS=H} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'usubwb_z_zz_ ', 'extensions=SVE2 '];
T[' USUBWB VREG={REG:u32=$Zd VDS=S} VREG={REG:u32=$Zn VDS=S} VREG={REG:u16=$Zm VDS=H}  ', 'size=0b10 ', 'usubwb_z_zz_ ', 'extensions=SVE2 '];
T[' USUBWB VREG={REG:u64=$Zd VDS=D} VREG={REG:u64=$Zn VDS=D} VREG={REG:u32=$Zm VDS=S}  ', 'size=0b11 ', 'usubwb_z_zz_ ', 'extensions=SVE2 '];
T[' USUBWT VREG={REG:u16=$Zd VDS=H} VREG={REG:u16=$Zn VDS=H} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'usubwt_z_zz_ ', 'extensions=SVE2 '];
T[' USUBWT VREG={REG:u32=$Zd VDS=S} VREG={REG:u32=$Zn VDS=S} VREG={REG:u16=$Zm VDS=H}  ', 'size=0b10 ', 'usubwt_z_zz_ ', 'extensions=SVE2 '];
T[' USUBWT VREG={REG:u64=$Zd VDS=D} VREG={REG:u64=$Zn VDS=D} VREG={REG:u32=$Zm VDS=S}  ', 'size=0b11 ', 'usubwt_z_zz_ ', 'extensions=SVE2 '];



# sve_intx_cons_mul_long: SVE2 integer multiply long.
# A64: ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op:u=0bx U:u=0bx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_cons_mul_long', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' PMULLB   VREG={REG:i128=$Zd VDS=Q} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}  ', 'size=0b00 ', 'pmullb_z_zz_   ', 'extensions=SVE2|SVE2PMULL128 '];
T[' PMULLB   VREG={REG:i16=$Zd VDS=H} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b01 ', 'pmullb_z_zz_   ', 'extensions=SVE2|SVE2PMULL128 '];
T[' PMULLB   VREG={REG:i64=$Zd VDS=D} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}   ', 'size=0b11 ', 'pmullb_z_zz_   ', 'extensions=SVE2|SVE2PMULL128 '];
T[' PMULLT   VREG={REG:i128=$Zd VDS=Q} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}  ', 'size=0b00 ', 'pmullt_z_zz_   ', 'extensions=SVE2|SVE2PMULL128 '];
T[' PMULLT   VREG={REG:i16=$Zd VDS=H} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b01 ', 'pmullt_z_zz_   ', 'extensions=SVE2|SVE2PMULL128 '];
T[' PMULLT   VREG={REG:i64=$Zd VDS=D} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}   ', 'size=0b11 ', 'pmullt_z_zz_   ', 'extensions=SVE2|SVE2PMULL128 '];
T[' SMULLB   VREG={REG:s16=$Zd VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'smullb_z_zz_   ', 'extensions=SVE2              '];
T[' SMULLB   VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'smullb_z_zz_   ', 'extensions=SVE2              '];
T[' SMULLB   VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'smullb_z_zz_   ', 'extensions=SVE2              '];
T[' SMULLT   VREG={REG:s16=$Zd VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'smullt_z_zz_   ', 'extensions=SVE2              '];
T[' SMULLT   VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'smullt_z_zz_   ', 'extensions=SVE2              '];
T[' SMULLT   VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'smullt_z_zz_   ', 'extensions=SVE2              '];
T[' SQDMULLB VREG={REG:s16=$Zd VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'sqdmullb_z_zz_ ', 'extensions=SVE2              '];
T[' SQDMULLB VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'sqdmullb_z_zz_ ', 'extensions=SVE2              '];
T[' SQDMULLB VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'sqdmullb_z_zz_ ', 'extensions=SVE2              '];
T[' SQDMULLT VREG={REG:s16=$Zd VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'sqdmullt_z_zz_ ', 'extensions=SVE2              '];
T[' SQDMULLT VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'sqdmullt_z_zz_ ', 'extensions=SVE2              '];
T[' SQDMULLT VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'sqdmullt_z_zz_ ', 'extensions=SVE2              '];
T[' UMULLB   VREG={REG:u16=$Zd VDS=H} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'umullb_z_zz_   ', 'extensions=SVE2              '];
T[' UMULLB   VREG={REG:u32=$Zd VDS=S} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}   ', 'size=0b10 ', 'umullb_z_zz_   ', 'extensions=SVE2              '];
T[' UMULLB   VREG={REG:u64=$Zd VDS=D} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}   ', 'size=0b11 ', 'umullb_z_zz_   ', 'extensions=SVE2              '];
T[' UMULLT   VREG={REG:u16=$Zd VDS=H} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'umullt_z_zz_   ', 'extensions=SVE2              '];
T[' UMULLT   VREG={REG:u32=$Zd VDS=S} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}   ', 'size=0b10 ', 'umullt_z_zz_   ', 'extensions=SVE2              '];
T[' UMULLT   VREG={REG:u64=$Zd VDS=D} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}   ', 'size=0b11 ', 'umullt_z_zz_   ', 'extensions=SVE2              '];



# sve_intx_dot: SVE integer dot product (unpredicated).
# A64: ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00000 U:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_dot', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' SDOT VREG={REG:s32=$Zda VDS=S} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0bx0 ', 'sdot_z_zzz_ ', 'extensions=SVE '];
T[' SDOT VREG={REG:s64=$Zda VDS=D} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0bx1 ', 'sdot_z_zzz_ ', 'extensions=SVE '];
T[' UDOT VREG={REG:u32=$Zda VDS=S} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0bx0 ', 'udot_z_zzz_ ', 'extensions=SVE '];
T[' UDOT VREG={REG:u64=$Zda VDS=D} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}   ', 'size=0bx1 ', 'udot_z_zzz_ ', 'extensions=SVE '];



# sve_intx_dot_by_indexed_elem: SVE integer dot product (indexed).
# A64: ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b00000 U:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_dot_by_indexed_elem', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' SDOT VREG={REG:s32=$Zda VDS=S} VREG={REG:s8=$Zn   VDS=B} EREG={REG:s8=$Zm   VDS=B IDX={ENCODEDIN=i2}} ', 'NONE ', 'sdot_z_zzzi_s ', 'extensions=SVE '];
T[' SDOT VREG={REG:s64=$Zda VDS=D} VREG={REG:s16=$Zn VDS=H} EREG={REG:s16=$Zm VDS=H IDX={ENCODEDIN=i1}}   ', 'NONE ', 'sdot_z_zzzi_d ', 'extensions=SVE '];
T[' UDOT VREG={REG:u32=$Zda VDS=S} VREG={REG:u8=$Zn   VDS=B} EREG={REG:u8=$Zm   VDS=B IDX={ENCODEDIN=i2}} ', 'NONE ', 'udot_z_zzzi_s ', 'extensions=SVE '];
T[' UDOT VREG={REG:u64=$Zda VDS=D} VREG={REG:u16=$Zn VDS=H} EREG={REG:u16=$Zm VDS=H IDX={ENCODEDIN=i1}}   ', 'NONE ', 'udot_z_zzzi_d ', 'extensions=SVE '];



# sve_intx_eorx: SVE2 bitwise exclusive-or interleaved.
# A64: ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b10010 tb:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_eorx', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' EORBT VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'eorbt_z_zz_ ', 'extensions=SVE2 '];
T[' EORBT VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'eorbt_z_zz_ ', 'extensions=SVE2 '];
T[' EORBT VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'eorbt_z_zz_ ', 'extensions=SVE2 '];
T[' EORBT VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'eorbt_z_zz_ ', 'extensions=SVE2 '];
T[' EORTB VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'eortb_z_zz_ ', 'extensions=SVE2 '];
T[' EORTB VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'eortb_z_zz_ ', 'extensions=SVE2 '];
T[' EORTB VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'eortb_z_zz_ ', 'extensions=SVE2 '];
T[' EORTB VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'eortb_z_zz_ ', 'extensions=SVE2 '];



# sve_intx_extract_narrow: SVE2 saturating extract narrow.
# A64: ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx ig2=0b000010 opc:u=0bxx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_extract_narrow', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' SQXTNB  VREG={REG:s8=$Zd   VDS=B} VREG={REG:s16=$Zn VDS=H} ', 'tszh=0b0 tszl=0b01 ', 'sqxtnb_z_zz_  ', 'extensions=SVE2 '];
T[' SQXTNB  VREG={REG:s16=$Zd VDS=H} VREG={REG:s32=$Zn VDS=S}  ', 'tszh=0b0 tszl=0b10 ', 'sqxtnb_z_zz_  ', 'extensions=SVE2 '];
T[' SQXTNB  VREG={REG:s32=$Zd VDS=S} VREG={REG:s64=$Zn VDS=D}  ', 'tszh=0b1 tszl=0b00 ', 'sqxtnb_z_zz_  ', 'extensions=SVE2 '];
T[' SQXTNT  VREG={REG:s8=$Zd   VDS=B} VREG={REG:s16=$Zn VDS=H} ', 'tszh=0b0 tszl=0b01 ', 'sqxtnt_z_zz_  ', 'extensions=SVE2 '];
T[' SQXTNT  VREG={REG:s16=$Zd VDS=H} VREG={REG:s32=$Zn VDS=S}  ', 'tszh=0b0 tszl=0b10 ', 'sqxtnt_z_zz_  ', 'extensions=SVE2 '];
T[' SQXTNT  VREG={REG:s32=$Zd VDS=S} VREG={REG:s64=$Zn VDS=D}  ', 'tszh=0b1 tszl=0b00 ', 'sqxtnt_z_zz_  ', 'extensions=SVE2 '];
T[' SQXTUNB VREG={REG:u8=$Zd   VDS=B} VREG={REG:u16=$Zn VDS=H} ', 'tszh=0b0 tszl=0b01 ', 'sqxtunb_z_zz_ ', 'extensions=SVE2 '];
T[' SQXTUNB VREG={REG:u16=$Zd VDS=H} VREG={REG:u32=$Zn VDS=S}  ', 'tszh=0b0 tszl=0b10 ', 'sqxtunb_z_zz_ ', 'extensions=SVE2 '];
T[' SQXTUNB VREG={REG:u32=$Zd VDS=S} VREG={REG:u64=$Zn VDS=D}  ', 'tszh=0b1 tszl=0b00 ', 'sqxtunb_z_zz_ ', 'extensions=SVE2 '];
T[' SQXTUNT VREG={REG:u8=$Zd   VDS=B} VREG={REG:u16=$Zn VDS=H} ', 'tszh=0b0 tszl=0b01 ', 'sqxtunt_z_zz_ ', 'extensions=SVE2 '];
T[' SQXTUNT VREG={REG:u16=$Zd VDS=H} VREG={REG:u32=$Zn VDS=S}  ', 'tszh=0b0 tszl=0b10 ', 'sqxtunt_z_zz_ ', 'extensions=SVE2 '];
T[' SQXTUNT VREG={REG:u32=$Zd VDS=S} VREG={REG:u64=$Zn VDS=D}  ', 'tszh=0b1 tszl=0b00 ', 'sqxtunt_z_zz_ ', 'extensions=SVE2 '];
T[' UQXTNB  VREG={REG:u8=$Zd   VDS=B} VREG={REG:u16=$Zn VDS=H} ', 'tszh=0b0 tszl=0b01 ', 'uqxtnb_z_zz_  ', 'extensions=SVE2 '];
T[' UQXTNB  VREG={REG:u16=$Zd VDS=H} VREG={REG:u32=$Zn VDS=S}  ', 'tszh=0b0 tszl=0b10 ', 'uqxtnb_z_zz_  ', 'extensions=SVE2 '];
T[' UQXTNB  VREG={REG:u32=$Zd VDS=S} VREG={REG:u64=$Zn VDS=D}  ', 'tszh=0b1 tszl=0b00 ', 'uqxtnb_z_zz_  ', 'extensions=SVE2 '];
T[' UQXTNT  VREG={REG:u8=$Zd   VDS=B} VREG={REG:u16=$Zn VDS=H} ', 'tszh=0b0 tszl=0b01 ', 'uqxtnt_z_zz_  ', 'extensions=SVE2 '];
T[' UQXTNT  VREG={REG:u16=$Zd VDS=H} VREG={REG:u32=$Zn VDS=S}  ', 'tszh=0b0 tszl=0b10 ', 'uqxtnt_z_zz_  ', 'extensions=SVE2 '];
T[' UQXTNT  VREG={REG:u32=$Zd VDS=S} VREG={REG:u64=$Zn VDS=D}  ', 'tszh=0b1 tszl=0b00 ', 'uqxtnt_z_zz_  ', 'extensions=SVE2 '];



# sve_intx_histcnt: SVE2 histogram generation (vector).
# A64: ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_histcnt', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' HISTCNT VREG={REG:i32=$Zd VDS=S} PREG:Z=$Pg VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S} ', 'size=0bx0 ', 'histcnt_z_p_zz_ ', 'extensions=SVE2 '];
T[' HISTCNT VREG={REG:i64=$Zd VDS=D} PREG:Z=$Pg VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D} ', 'size=0bx1 ', 'histcnt_z_p_zz_ ', 'extensions=SVE2 '];



# sve_intx_histseg: SVE2 histogram generation  (segment).
# A64: ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b101000 Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_histseg', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' HISTSEG VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'NONE ', 'histseg_z_zz_ ', 'extensions=SVE2 '];



# sve_intx_match: SVE2 character match.
# A64: ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx op:u=0bx Pd:u=0bxxxx
HINT('ICLASS', 'sve_intx_match', 'A64');

DEF Pd => PRDREG('Pd', 'Pd', 'write') ;
DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' MATCH  VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0bx0 ', 'match_p_p_zz_  ', 'extensions=SVE2 '];
T[' MATCH  VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0bx1 ', 'match_p_p_zz_  ', 'extensions=SVE2 '];
T[' NMATCH VREG={REG:i8=$Pd   VDS=B} PREG:Z=$Pg VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0bx0 ', 'nmatch_p_p_zz_ ', 'extensions=SVE2 '];
T[' NMATCH VREG={REG:i16=$Pd VDS=H} PREG:Z=$Pg VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0bx1 ', 'nmatch_p_p_zz_ ', 'extensions=SVE2 '];



# sve_intx_mixed_dot: SVE mixed sign dot product.
# A64: ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011110 Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_mixed_dot', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' USDOT VREG={REG:u32=$Zda VDS=S} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'NONE ', 'usdot_z_zzz_s ', 'arch_variant=ARMv8v2 extensions=INT8MATMUL|SVE '];



# sve_intx_mixed_dot_by_indexed_elem: SVE mixed sign dot product (indexed).
# A64: ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b00011 U:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_mixed_dot_by_indexed_elem', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' SUDOT VREG={REG:u32=$Zda VDS=S} VREG={REG:u8=$Zn   VDS=B} EREG={REG:u8=$Zm   VDS=B IDX={ENCODEDIN=i2}} ', 'NONE ', 'sudot_z_zzzi_s ', 'arch_variant=ARMv8v2 extensions=INT8MATMUL|SVE '];
T[' USDOT VREG={REG:u32=$Zda VDS=S} VREG={REG:u8=$Zn   VDS=B} EREG={REG:u8=$Zm   VDS=B IDX={ENCODEDIN=i2}} ', 'NONE ', 'usdot_z_zzzi_s ', 'arch_variant=ARMv8v2 extensions=INT8MATMUL|SVE '];



# sve_intx_mla_by_indexed_elem: SVE2 integer multiply-add (indexed).
# A64: ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b00001 S:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_mla_by_indexed_elem', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' MLA VREG={REG:i16=$Zda VDS=H} VREG={REG:i16=$Zn VDS=H} EREG={REG:i16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'mla_z_zzzi_h ', 'extensions=SVE2 '];
T[' MLA VREG={REG:i32=$Zda VDS=S} VREG={REG:i32=$Zn VDS=S} EREG={REG:i32=$Zm VDS=S IDX={ENCODEDIN=i2}}      ', 'NONE ', 'mla_z_zzzi_s ', 'extensions=SVE2 '];
T[' MLA VREG={REG:i64=$Zda VDS=D} VREG={REG:i64=$Zn VDS=D} EREG={REG:i64=$Zm VDS=D IDX={ENCODEDIN=i1}}      ', 'NONE ', 'mla_z_zzzi_d ', 'extensions=SVE2 '];
T[' MLS VREG={REG:i16=$Zda VDS=H} VREG={REG:i16=$Zn VDS=H} EREG={REG:i16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'mls_z_zzzi_h ', 'extensions=SVE2 '];
T[' MLS VREG={REG:i32=$Zda VDS=S} VREG={REG:i32=$Zn VDS=S} EREG={REG:i32=$Zm VDS=S IDX={ENCODEDIN=i2}}      ', 'NONE ', 'mls_z_zzzi_s ', 'extensions=SVE2 '];
T[' MLS VREG={REG:i64=$Zda VDS=D} VREG={REG:i64=$Zn VDS=D} EREG={REG:i64=$Zm VDS=D IDX={ENCODEDIN=i1}}      ', 'NONE ', 'mls_z_zzzi_d ', 'extensions=SVE2 '];



# sve_intx_mla_long_by_indexed_elem: SVE2 integer multiply-add long (indexed).
# A64: ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b10 S:u=0bx U:u=0bx il:u=0bx T:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_mla_long_by_indexed_elem', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' SMLALB VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} EREG={REG:s16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'smlalb_z_zzzi_s ', 'extensions=SVE2 '];
T[' SMLALB VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} EREG={REG:s32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'smlalb_z_zzzi_d ', 'extensions=SVE2 '];
T[' SMLALT VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} EREG={REG:s16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'smlalt_z_zzzi_s ', 'extensions=SVE2 '];
T[' SMLALT VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} EREG={REG:s32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'smlalt_z_zzzi_d ', 'extensions=SVE2 '];
T[' SMLSLB VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} EREG={REG:s16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'smlslb_z_zzzi_s ', 'extensions=SVE2 '];
T[' SMLSLB VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} EREG={REG:s32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'smlslb_z_zzzi_d ', 'extensions=SVE2 '];
T[' SMLSLT VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} EREG={REG:s16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'smlslt_z_zzzi_s ', 'extensions=SVE2 '];
T[' SMLSLT VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} EREG={REG:s32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'smlslt_z_zzzi_d ', 'extensions=SVE2 '];
T[' UMLALB VREG={REG:u32=$Zda VDS=S} VREG={REG:u16=$Zn VDS=H} EREG={REG:u16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'umlalb_z_zzzi_s ', 'extensions=SVE2 '];
T[' UMLALB VREG={REG:u64=$Zda VDS=D} VREG={REG:u32=$Zn VDS=S} EREG={REG:u32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'umlalb_z_zzzi_d ', 'extensions=SVE2 '];
T[' UMLALT VREG={REG:u32=$Zda VDS=S} VREG={REG:u16=$Zn VDS=H} EREG={REG:u16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'umlalt_z_zzzi_s ', 'extensions=SVE2 '];
T[' UMLALT VREG={REG:u64=$Zda VDS=D} VREG={REG:u32=$Zn VDS=S} EREG={REG:u32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'umlalt_z_zzzi_d ', 'extensions=SVE2 '];
T[' UMLSLB VREG={REG:u32=$Zda VDS=S} VREG={REG:u16=$Zn VDS=H} EREG={REG:u16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'umlslb_z_zzzi_s ', 'extensions=SVE2 '];
T[' UMLSLB VREG={REG:u64=$Zda VDS=D} VREG={REG:u32=$Zn VDS=S} EREG={REG:u32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'umlslb_z_zzzi_d ', 'extensions=SVE2 '];
T[' UMLSLT VREG={REG:u32=$Zda VDS=S} VREG={REG:u16=$Zn VDS=H} EREG={REG:u16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'umlslt_z_zzzi_s ', 'extensions=SVE2 '];
T[' UMLSLT VREG={REG:u64=$Zda VDS=D} VREG={REG:u32=$Zn VDS=S} EREG={REG:u32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'umlslt_z_zzzi_d ', 'extensions=SVE2 '];



# sve_intx_mlal_long: SVE2 integer multiply-add long.
# A64: ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S:u=0bx U:u=0bx T:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_mlal_long', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' SMLALB VREG={REG:s16=$Zda VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'smlalb_z_zzz_ ', 'extensions=SVE2 '];
T[' SMLALB VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'smlalb_z_zzz_ ', 'extensions=SVE2 '];
T[' SMLALB VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'smlalb_z_zzz_ ', 'extensions=SVE2 '];
T[' SMLALT VREG={REG:s16=$Zda VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'smlalt_z_zzz_ ', 'extensions=SVE2 '];
T[' SMLALT VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'smlalt_z_zzz_ ', 'extensions=SVE2 '];
T[' SMLALT VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'smlalt_z_zzz_ ', 'extensions=SVE2 '];
T[' SMLSLB VREG={REG:s16=$Zda VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'smlslb_z_zzz_ ', 'extensions=SVE2 '];
T[' SMLSLB VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'smlslb_z_zzz_ ', 'extensions=SVE2 '];
T[' SMLSLB VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'smlslb_z_zzz_ ', 'extensions=SVE2 '];
T[' SMLSLT VREG={REG:s16=$Zda VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'smlslt_z_zzz_ ', 'extensions=SVE2 '];
T[' SMLSLT VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'smlslt_z_zzz_ ', 'extensions=SVE2 '];
T[' SMLSLT VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'smlslt_z_zzz_ ', 'extensions=SVE2 '];
T[' UMLALB VREG={REG:u16=$Zda VDS=H} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'umlalb_z_zzz_ ', 'extensions=SVE2 '];
T[' UMLALB VREG={REG:u32=$Zda VDS=S} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}   ', 'size=0b10 ', 'umlalb_z_zzz_ ', 'extensions=SVE2 '];
T[' UMLALB VREG={REG:u64=$Zda VDS=D} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}   ', 'size=0b11 ', 'umlalb_z_zzz_ ', 'extensions=SVE2 '];
T[' UMLALT VREG={REG:u16=$Zda VDS=H} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'umlalt_z_zzz_ ', 'extensions=SVE2 '];
T[' UMLALT VREG={REG:u32=$Zda VDS=S} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}   ', 'size=0b10 ', 'umlalt_z_zzz_ ', 'extensions=SVE2 '];
T[' UMLALT VREG={REG:u64=$Zda VDS=D} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}   ', 'size=0b11 ', 'umlalt_z_zzz_ ', 'extensions=SVE2 '];
T[' UMLSLB VREG={REG:u16=$Zda VDS=H} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'umlslb_z_zzz_ ', 'extensions=SVE2 '];
T[' UMLSLB VREG={REG:u32=$Zda VDS=S} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}   ', 'size=0b10 ', 'umlslb_z_zzz_ ', 'extensions=SVE2 '];
T[' UMLSLB VREG={REG:u64=$Zda VDS=D} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}   ', 'size=0b11 ', 'umlslb_z_zzz_ ', 'extensions=SVE2 '];
T[' UMLSLT VREG={REG:u16=$Zda VDS=H} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b01 ', 'umlslt_z_zzz_ ', 'extensions=SVE2 '];
T[' UMLSLT VREG={REG:u32=$Zda VDS=S} VREG={REG:u16=$Zn VDS=H} VREG={REG:u16=$Zm VDS=H}   ', 'size=0b10 ', 'umlslt_z_zzz_ ', 'extensions=SVE2 '];
T[' UMLSLT VREG={REG:u64=$Zda VDS=D} VREG={REG:u32=$Zn VDS=S} VREG={REG:u32=$Zm VDS=S}   ', 'size=0b11 ', 'umlslt_z_zzz_ ', 'extensions=SVE2 '];



# sve_intx_mmla: SVE integer matrix multiply accumulate.
# A64: ig0=0b01000101 uns:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b100110 Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_mmla', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' SMMLA  VREG={REG:s32=$Zda VDS=S} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'NONE ', 'smmla_z_zzz_  ', 'arch_variant=ARMv8v2 extensions=INT8MATMUL|SVE '];
T[' UMMLA  VREG={REG:u32=$Zda VDS=S} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'NONE ', 'ummla_z_zzz_  ', 'arch_variant=ARMv8v2 extensions=INT8MATMUL|SVE '];
T[' USMMLA VREG={REG:u32=$Zda VDS=S} VREG={REG:u8=$Zn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'NONE ', 'usmmla_z_zzz_ ', 'arch_variant=ARMv8v2 extensions=INT8MATMUL|SVE '];



# sve_intx_mul_by_indexed_elem: SVE2 integer multiply (indexed).
# A64: ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b111110 Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_mul_by_indexed_elem', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' MUL VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} EREG={REG:i16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'mul_z_zzi_h ', 'extensions=SVE2 '];
T[' MUL VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} EREG={REG:i32=$Zm VDS=S IDX={ENCODEDIN=i2}}      ', 'NONE ', 'mul_z_zzi_s ', 'extensions=SVE2 '];
T[' MUL VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} EREG={REG:i64=$Zm VDS=D IDX={ENCODEDIN=i1}}      ', 'NONE ', 'mul_z_zzi_d ', 'extensions=SVE2 '];



# sve_intx_mul_long_by_indexed_elem: SVE2 integer multiply long (indexed).
# A64: ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b110 U:u=0bx il:u=0bx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_mul_long_by_indexed_elem', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' SMULLB VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} EREG={REG:s16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'smullb_z_zzi_s ', 'extensions=SVE2 '];
T[' SMULLB VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} EREG={REG:s32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'smullb_z_zzi_d ', 'extensions=SVE2 '];
T[' SMULLT VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} EREG={REG:s16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'smullt_z_zzi_s ', 'extensions=SVE2 '];
T[' SMULLT VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} EREG={REG:s32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'smullt_z_zzi_d ', 'extensions=SVE2 '];
T[' UMULLB VREG={REG:u32=$Zd VDS=S} VREG={REG:u16=$Zn VDS=H} EREG={REG:u16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'umullb_z_zzi_s ', 'extensions=SVE2 '];
T[' UMULLB VREG={REG:u64=$Zd VDS=D} VREG={REG:u32=$Zn VDS=S} EREG={REG:u32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'umullb_z_zzi_d ', 'extensions=SVE2 '];
T[' UMULLT VREG={REG:u32=$Zd VDS=S} VREG={REG:u16=$Zn VDS=H} EREG={REG:u16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'umullt_z_zzi_s ', 'extensions=SVE2 '];
T[' UMULLT VREG={REG:u64=$Zd VDS=D} VREG={REG:u32=$Zn VDS=S} EREG={REG:u32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'umullt_z_zzi_d ', 'extensions=SVE2 '];



# sve_intx_perm_bit: SVE2 bitwise permute.
# A64: ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1011 opc:u=0bxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_perm_bit', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' BDEP VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'bdep_z_zz_ ', 'extensions=SVE2BITPERM '];
T[' BDEP VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'bdep_z_zz_ ', 'extensions=SVE2BITPERM '];
T[' BDEP VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'bdep_z_zz_ ', 'extensions=SVE2BITPERM '];
T[' BDEP VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'bdep_z_zz_ ', 'extensions=SVE2BITPERM '];
T[' BEXT VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'bext_z_zz_ ', 'extensions=SVE2BITPERM '];
T[' BEXT VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'bext_z_zz_ ', 'extensions=SVE2BITPERM '];
T[' BEXT VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'bext_z_zz_ ', 'extensions=SVE2BITPERM '];
T[' BEXT VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'bext_z_zz_ ', 'extensions=SVE2BITPERM '];
T[' BGRP VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} VREG={REG:i8=$Zm   VDS=B} ', 'size=0b00 ', 'bgrp_z_zz_ ', 'extensions=SVE2BITPERM '];
T[' BGRP VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} VREG={REG:i16=$Zm VDS=H}    ', 'size=0b01 ', 'bgrp_z_zz_ ', 'extensions=SVE2BITPERM '];
T[' BGRP VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} VREG={REG:i32=$Zm VDS=S}    ', 'size=0b10 ', 'bgrp_z_zz_ ', 'extensions=SVE2BITPERM '];
T[' BGRP VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} VREG={REG:i64=$Zm VDS=D}    ', 'size=0b11 ', 'bgrp_z_zz_ ', 'extensions=SVE2BITPERM '];



# sve_intx_perm_extract_i: SVE2 extract vector (immediate offset, constructive).
# A64: ig0=0b00000101011 imm8h:u=0bxxxxx ig1=0b000 imm8l:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_perm_extract_i', 'A64');

DEF Zd  => SVEREG('Zd', 'Zd', 'write')                  ;
DEF Zn  => SVEREG('Zn1', 'Zn', 'read')                  ;
DEF imm => IMM('imm', 'imm8h:imm8l', range => (0, 255)) ;

T[' EXT VREG={REG:i8=$Zd   VDS=B} VLIST={VBASE={REG:i8=$Zn   VDS=B} COUNT=2} IMM:u8=$imm ', 'NONE ', 'ext_z_zi_con ', 'extensions=SVE2 '];



# sve_intx_perm_splice: SVE2 vector splice (constructive).
# A64: ig0=0b00000101 size:u=0bxx ig1=0b101101100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_perm_splice', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn1', 'Zn', 'read') ;

T[' SPLICE VREG={REG:i8=$Zd   VDS=B} REG=$Pg VLIST={VBASE={REG:i8=$Zn   VDS=B} COUNT=2} ', 'size=0b00 ', 'splice_z_p_zz_con ', 'extensions=SVE2 '];
T[' SPLICE VREG={REG:i16=$Zd VDS=H} REG=$Pg VLIST={VBASE={REG:i16=$Zn VDS=H} COUNT=2}   ', 'size=0b01 ', 'splice_z_p_zz_con ', 'extensions=SVE2 '];
T[' SPLICE VREG={REG:i32=$Zd VDS=S} REG=$Pg VLIST={VBASE={REG:i32=$Zn VDS=S} COUNT=2}   ', 'size=0b10 ', 'splice_z_p_zz_con ', 'extensions=SVE2 '];
T[' SPLICE VREG={REG:i64=$Zd VDS=D} REG=$Pg VLIST={VBASE={REG:i64=$Zn VDS=D} COUNT=2}   ', 'size=0b11 ', 'splice_z_p_zz_con ', 'extensions=SVE2 '];



# sve_intx_pred_arith_binary: SVE2 integer halving add/subtract (predicated).
# A64: ig0=0b01000100 size:u=0bxx ig1=0b010 R:u=0bx S:u=0bx U:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_intx_pred_arith_binary', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' SHADD  VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'shadd_z_p_zz_  ', 'extensions=SVE2 '];
T[' SHADD  VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'shadd_z_p_zz_  ', 'extensions=SVE2 '];
T[' SHADD  VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'shadd_z_p_zz_  ', 'extensions=SVE2 '];
T[' SHADD  VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'shadd_z_p_zz_  ', 'extensions=SVE2 '];
T[' SHSUB  VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'shsub_z_p_zz_  ', 'extensions=SVE2 '];
T[' SHSUB  VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'shsub_z_p_zz_  ', 'extensions=SVE2 '];
T[' SHSUB  VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'shsub_z_p_zz_  ', 'extensions=SVE2 '];
T[' SHSUB  VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'shsub_z_p_zz_  ', 'extensions=SVE2 '];
T[' SHSUBR VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'shsubr_z_p_zz_ ', 'extensions=SVE2 '];
T[' SHSUBR VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'shsubr_z_p_zz_ ', 'extensions=SVE2 '];
T[' SHSUBR VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'shsubr_z_p_zz_ ', 'extensions=SVE2 '];
T[' SHSUBR VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'shsubr_z_p_zz_ ', 'extensions=SVE2 '];
T[' SRHADD VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'srhadd_z_p_zz_ ', 'extensions=SVE2 '];
T[' SRHADD VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'srhadd_z_p_zz_ ', 'extensions=SVE2 '];
T[' SRHADD VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'srhadd_z_p_zz_ ', 'extensions=SVE2 '];
T[' SRHADD VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'srhadd_z_p_zz_ ', 'extensions=SVE2 '];
T[' UHADD  VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'uhadd_z_p_zz_  ', 'extensions=SVE2 '];
T[' UHADD  VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'uhadd_z_p_zz_  ', 'extensions=SVE2 '];
T[' UHADD  VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'uhadd_z_p_zz_  ', 'extensions=SVE2 '];
T[' UHADD  VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'uhadd_z_p_zz_  ', 'extensions=SVE2 '];
T[' UHSUB  VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'uhsub_z_p_zz_  ', 'extensions=SVE2 '];
T[' UHSUB  VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'uhsub_z_p_zz_  ', 'extensions=SVE2 '];
T[' UHSUB  VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'uhsub_z_p_zz_  ', 'extensions=SVE2 '];
T[' UHSUB  VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'uhsub_z_p_zz_  ', 'extensions=SVE2 '];
T[' UHSUBR VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'uhsubr_z_p_zz_ ', 'extensions=SVE2 '];
T[' UHSUBR VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'uhsubr_z_p_zz_ ', 'extensions=SVE2 '];
T[' UHSUBR VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'uhsubr_z_p_zz_ ', 'extensions=SVE2 '];
T[' UHSUBR VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'uhsubr_z_p_zz_ ', 'extensions=SVE2 '];
T[' URHADD VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'urhadd_z_p_zz_ ', 'extensions=SVE2 '];
T[' URHADD VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'urhadd_z_p_zz_ ', 'extensions=SVE2 '];
T[' URHADD VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'urhadd_z_p_zz_ ', 'extensions=SVE2 '];
T[' URHADD VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'urhadd_z_p_zz_ ', 'extensions=SVE2 '];



# sve_intx_pred_arith_binary_sat: SVE2 saturating add/subtract.
# A64: ig0=0b01000100 size:u=0bxx ig1=0b011 op:u=0bx S:u=0bx U:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx
HINT('ICLASS', 'sve_intx_pred_arith_binary_sat', 'A64');

DEF Pg  => PRDREG('Pg', 'Pg', 'read')         ;
DEF Zdn => SVEREG('Zdn', 'Zdn', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;

T[' SQADD  VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'sqadd_z_p_zz_  ', 'extensions=SVE2 '];
T[' SQADD  VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'sqadd_z_p_zz_  ', 'extensions=SVE2 '];
T[' SQADD  VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'sqadd_z_p_zz_  ', 'extensions=SVE2 '];
T[' SQADD  VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'sqadd_z_p_zz_  ', 'extensions=SVE2 '];
T[' SQSUB  VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'sqsub_z_p_zz_  ', 'extensions=SVE2 '];
T[' SQSUB  VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'sqsub_z_p_zz_  ', 'extensions=SVE2 '];
T[' SQSUB  VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'sqsub_z_p_zz_  ', 'extensions=SVE2 '];
T[' SQSUB  VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'sqsub_z_p_zz_  ', 'extensions=SVE2 '];
T[' SQSUBR VREG={REG:s8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zdn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'sqsubr_z_p_zz_ ', 'extensions=SVE2 '];
T[' SQSUBR VREG={REG:s16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:s16=$Zdn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'sqsubr_z_p_zz_ ', 'extensions=SVE2 '];
T[' SQSUBR VREG={REG:s32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:s32=$Zdn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'sqsubr_z_p_zz_ ', 'extensions=SVE2 '];
T[' SQSUBR VREG={REG:s64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:s64=$Zdn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'sqsubr_z_p_zz_ ', 'extensions=SVE2 '];
T[' SUQADD VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'suqadd_z_p_zz_ ', 'extensions=SVE2 '];
T[' SUQADD VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'suqadd_z_p_zz_ ', 'extensions=SVE2 '];
T[' SUQADD VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'suqadd_z_p_zz_ ', 'extensions=SVE2 '];
T[' SUQADD VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'suqadd_z_p_zz_ ', 'extensions=SVE2 '];
T[' UQADD  VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'uqadd_z_p_zz_  ', 'extensions=SVE2 '];
T[' UQADD  VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'uqadd_z_p_zz_  ', 'extensions=SVE2 '];
T[' UQADD  VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'uqadd_z_p_zz_  ', 'extensions=SVE2 '];
T[' UQADD  VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'uqadd_z_p_zz_  ', 'extensions=SVE2 '];
T[' UQSUB  VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'uqsub_z_p_zz_  ', 'extensions=SVE2 '];
T[' UQSUB  VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'uqsub_z_p_zz_  ', 'extensions=SVE2 '];
T[' UQSUB  VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'uqsub_z_p_zz_  ', 'extensions=SVE2 '];
T[' UQSUB  VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'uqsub_z_p_zz_  ', 'extensions=SVE2 '];
T[' UQSUBR VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'uqsubr_z_p_zz_ ', 'extensions=SVE2 '];
T[' UQSUBR VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'uqsubr_z_p_zz_ ', 'extensions=SVE2 '];
T[' UQSUBR VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'uqsubr_z_p_zz_ ', 'extensions=SVE2 '];
T[' UQSUBR VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'uqsubr_z_p_zz_ ', 'extensions=SVE2 '];
T[' USQADD VREG={REG:u8=$Zdn   VDS=B} PREG:M=$Pg VREG={REG:u8=$Zdn   VDS=B} VREG={REG:u8=$Zm   VDS=B} ', 'size=0b00 ', 'usqadd_z_p_zz_ ', 'extensions=SVE2 '];
T[' USQADD VREG={REG:u16=$Zdn VDS=H} PREG:M=$Pg VREG={REG:u16=$Zdn VDS=H} VREG={REG:u16=$Zm VDS=H}    ', 'size=0b01 ', 'usqadd_z_p_zz_ ', 'extensions=SVE2 '];
T[' USQADD VREG={REG:u32=$Zdn VDS=S} PREG:M=$Pg VREG={REG:u32=$Zdn VDS=S} VREG={REG:u32=$Zm VDS=S}    ', 'size=0b10 ', 'usqadd_z_p_zz_ ', 'extensions=SVE2 '];
T[' USQADD VREG={REG:u64=$Zdn VDS=D} PREG:M=$Pg VREG={REG:u64=$Zdn VDS=D} VREG={REG:u64=$Zm VDS=D}    ', 'size=0b11 ', 'usqadd_z_p_zz_ ', 'extensions=SVE2 '];



# sve_intx_pred_arith_unary: SVE2 integer unary operations (predicated).
# A64: ig0=0b01000100 size:u=0bxx ig1=0b00 Q:u=0bx ig2=0b0 opc:u=0bxx ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_pred_arith_unary', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')  ;
DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' SQABS   VREG={REG:s8=$Zd   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zn   VDS=B} ', 'size=0b00 ', 'sqabs_z_p_z_   ', 'extensions=SVE2 '];
T[' SQABS   VREG={REG:s16=$Zd VDS=H} PREG:M=$Pg VREG={REG:s16=$Zn VDS=H}   ', 'size=0b01 ', 'sqabs_z_p_z_   ', 'extensions=SVE2 '];
T[' SQABS   VREG={REG:s32=$Zd VDS=S} PREG:M=$Pg VREG={REG:s32=$Zn VDS=S}   ', 'size=0b10 ', 'sqabs_z_p_z_   ', 'extensions=SVE2 '];
T[' SQABS   VREG={REG:s64=$Zd VDS=D} PREG:M=$Pg VREG={REG:s64=$Zn VDS=D}   ', 'size=0b11 ', 'sqabs_z_p_z_   ', 'extensions=SVE2 '];
T[' SQNEG   VREG={REG:s8=$Zd   VDS=B} PREG:M=$Pg VREG={REG:s8=$Zn   VDS=B} ', 'size=0b00 ', 'sqneg_z_p_z_   ', 'extensions=SVE2 '];
T[' SQNEG   VREG={REG:s16=$Zd VDS=H} PREG:M=$Pg VREG={REG:s16=$Zn VDS=H}   ', 'size=0b01 ', 'sqneg_z_p_z_   ', 'extensions=SVE2 '];
T[' SQNEG   VREG={REG:s32=$Zd VDS=S} PREG:M=$Pg VREG={REG:s32=$Zn VDS=S}   ', 'size=0b10 ', 'sqneg_z_p_z_   ', 'extensions=SVE2 '];
T[' SQNEG   VREG={REG:s64=$Zd VDS=D} PREG:M=$Pg VREG={REG:s64=$Zn VDS=D}   ', 'size=0b11 ', 'sqneg_z_p_z_   ', 'extensions=SVE2 '];
T[' URECPE  VREG={REG:u32=$Zd VDS=S} PREG:M=$Pg VREG={REG:u32=$Zn VDS=S}   ', 'NONE      ', 'urecpe_z_p_z_  ', 'extensions=SVE2 '];
T[' URSQRTE VREG={REG:u32=$Zd VDS=S} PREG:M=$Pg VREG={REG:u32=$Zn VDS=S}   ', 'NONE      ', 'ursqrte_z_p_z_ ', 'extensions=SVE2 '];



# sve_intx_qdmla_long_by_indexed_elem: SVE2 saturating multiply-add (indexed).
# A64: ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b001 S:u=0bx il:u=0bx T:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_qdmla_long_by_indexed_elem', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' SQDMLALB VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} EREG={REG:s16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'sqdmlalb_z_zzzi_s ', 'extensions=SVE2 '];
T[' SQDMLALB VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} EREG={REG:s32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'sqdmlalb_z_zzzi_d ', 'extensions=SVE2 '];
T[' SQDMLALT VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} EREG={REG:s16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'sqdmlalt_z_zzzi_s ', 'extensions=SVE2 '];
T[' SQDMLALT VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} EREG={REG:s32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'sqdmlalt_z_zzzi_d ', 'extensions=SVE2 '];
T[' SQDMLSLB VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} EREG={REG:s16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'sqdmlslb_z_zzzi_s ', 'extensions=SVE2 '];
T[' SQDMLSLB VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} EREG={REG:s32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'sqdmlslb_z_zzzi_d ', 'extensions=SVE2 '];
T[' SQDMLSLT VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} EREG={REG:s16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'sqdmlslt_z_zzzi_s ', 'extensions=SVE2 '];
T[' SQDMLSLT VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} EREG={REG:s32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'sqdmlslt_z_zzzi_d ', 'extensions=SVE2 '];



# sve_intx_qdmlal_long: SVE2 saturating  multiply-add long.
# A64: ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0110 S:u=0bx T:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_qdmlal_long', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' SQDMLALB VREG={REG:s16=$Zda VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'sqdmlalb_z_zzz_ ', 'extensions=SVE2 '];
T[' SQDMLALB VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'sqdmlalb_z_zzz_ ', 'extensions=SVE2 '];
T[' SQDMLALB VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'sqdmlalb_z_zzz_ ', 'extensions=SVE2 '];
T[' SQDMLALT VREG={REG:s16=$Zda VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'sqdmlalt_z_zzz_ ', 'extensions=SVE2 '];
T[' SQDMLALT VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'sqdmlalt_z_zzz_ ', 'extensions=SVE2 '];
T[' SQDMLALT VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'sqdmlalt_z_zzz_ ', 'extensions=SVE2 '];
T[' SQDMLSLB VREG={REG:s16=$Zda VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'sqdmlslb_z_zzz_ ', 'extensions=SVE2 '];
T[' SQDMLSLB VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'sqdmlslb_z_zzz_ ', 'extensions=SVE2 '];
T[' SQDMLSLB VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'sqdmlslb_z_zzz_ ', 'extensions=SVE2 '];
T[' SQDMLSLT VREG={REG:s16=$Zda VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'sqdmlslt_z_zzz_ ', 'extensions=SVE2 '];
T[' SQDMLSLT VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'sqdmlslt_z_zzz_ ', 'extensions=SVE2 '];
T[' SQDMLSLT VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'sqdmlslt_z_zzz_ ', 'extensions=SVE2 '];



# sve_intx_qdmlalbt: SVE2 saturating multiply-add interleaved long.
# A64: ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00001 S:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_qdmlalbt', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' SQDMLALBT VREG={REG:s16=$Zda VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'sqdmlalbt_z_zzz_ ', 'extensions=SVE2 '];
T[' SQDMLALBT VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'sqdmlalbt_z_zzz_ ', 'extensions=SVE2 '];
T[' SQDMLALBT VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'sqdmlalbt_z_zzz_ ', 'extensions=SVE2 '];
T[' SQDMLSLBT VREG={REG:s16=$Zda VDS=H} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b01 ', 'sqdmlslbt_z_zzz_ ', 'extensions=SVE2 '];
T[' SQDMLSLBT VREG={REG:s32=$Zda VDS=S} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}   ', 'size=0b10 ', 'sqdmlslbt_z_zzz_ ', 'extensions=SVE2 '];
T[' SQDMLSLBT VREG={REG:s64=$Zda VDS=D} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}   ', 'size=0b11 ', 'sqdmlslbt_z_zzz_ ', 'extensions=SVE2 '];



# sve_intx_qdmul_long_by_indexed_elem: SVE2 saturating multiply (indexed).
# A64: ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b1110 il:u=0bx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_qdmul_long_by_indexed_elem', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' SQDMULLB VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} EREG={REG:s16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'sqdmullb_z_zzi_s ', 'extensions=SVE2 '];
T[' SQDMULLB VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} EREG={REG:s32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'sqdmullb_z_zzi_d ', 'extensions=SVE2 '];
T[' SQDMULLT VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} EREG={REG:s16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'sqdmullt_z_zzi_s ', 'extensions=SVE2 '];
T[' SQDMULLT VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} EREG={REG:s32=$Zm VDS=S IDX={ENCODEDIN=i2h:i2l}} ', 'NONE ', 'sqdmullt_z_zzi_d ', 'extensions=SVE2 '];



# sve_intx_qdmulh_by_indexed_elem: SVE2 saturating  multiply high (indexed).
# A64: ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b11110 R:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_qdmulh_by_indexed_elem', 'A64');

DEF Zd => SVEREG('Zd', 'Zd', 'write') ;
DEF Zm => SVEREG('Zm', 'Zm', 'read')  ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')  ;

T[' SQDMULH  VREG={REG:s16=$Zd VDS=H} VREG={REG:s16=$Zn VDS=H} EREG={REG:s16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'sqdmulh_z_zzi_h  ', 'extensions=SVE2 '];
T[' SQDMULH  VREG={REG:s32=$Zd VDS=S} VREG={REG:s32=$Zn VDS=S} EREG={REG:s32=$Zm VDS=S IDX={ENCODEDIN=i2}}      ', 'NONE ', 'sqdmulh_z_zzi_s  ', 'extensions=SVE2 '];
T[' SQDMULH  VREG={REG:s64=$Zd VDS=D} VREG={REG:s64=$Zn VDS=D} EREG={REG:s64=$Zm VDS=D IDX={ENCODEDIN=i1}}      ', 'NONE ', 'sqdmulh_z_zzi_d  ', 'extensions=SVE2 '];
T[' SQRDMULH VREG={REG:s16=$Zd VDS=H} VREG={REG:s16=$Zn VDS=H} EREG={REG:s16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'sqrdmulh_z_zzi_h ', 'extensions=SVE2 '];
T[' SQRDMULH VREG={REG:s32=$Zd VDS=S} VREG={REG:s32=$Zn VDS=S} EREG={REG:s32=$Zm VDS=S IDX={ENCODEDIN=i2}}      ', 'NONE ', 'sqrdmulh_z_zzi_s ', 'extensions=SVE2 '];
T[' SQRDMULH VREG={REG:s64=$Zd VDS=D} VREG={REG:s64=$Zn VDS=D} EREG={REG:s64=$Zm VDS=D IDX={ENCODEDIN=i1}}      ', 'NONE ', 'sqrdmulh_z_zzi_d ', 'extensions=SVE2 '];



# sve_intx_qrdcmla_by_indexed_elem: SVE2 complex saturating multiply-add (indexed).
# A64: ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b0111 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_qrdcmla_by_indexed_elem', 'A64');

DEF Zda   => SVEREG('Zda', 'Zda', 'read|write')       ;
DEF Zm    => SVEREG('Zm', 'Zm', 'read')               ;
DEF Zn    => SVEREG('Zn', 'Zn', 'read')               ;
DEF const => IMM('const', 'rot', table => 'tbl_imm0') ;

T[' SQRDCMLAH VREG={REG:i16=$Zda VDS=H} VREG={REG:i16=$Zn VDS=H} EREG={REG:i16=$Zm VDS=H IDX={ENCODEDIN=i2}} IMM:i16=$const ', 'NONE ', 'sqrdcmlah_z_zzzi_h ', 'extensions=SVE2 '];
T[' SQRDCMLAH VREG={REG:i32=$Zda VDS=S} VREG={REG:i32=$Zn VDS=S} EREG={REG:i32=$Zm VDS=S IDX={ENCODEDIN=i1}} IMM:i32=$const ', 'NONE ', 'sqrdcmlah_z_zzzi_s ', 'extensions=SVE2 '];



# sve_intx_qrdmlah: SVE2 saturating multiply-add high.
# A64: ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b01110 S:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_qrdmlah', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' SQRDMLAH VREG={REG:s8=$Zda   VDS=B} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'sqrdmlah_z_zzz_ ', 'extensions=SVE2 '];
T[' SQRDMLAH VREG={REG:s16=$Zda VDS=H} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'sqrdmlah_z_zzz_ ', 'extensions=SVE2 '];
T[' SQRDMLAH VREG={REG:s32=$Zda VDS=S} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'sqrdmlah_z_zzz_ ', 'extensions=SVE2 '];
T[' SQRDMLAH VREG={REG:s64=$Zda VDS=D} VREG={REG:s64=$Zn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'sqrdmlah_z_zzz_ ', 'extensions=SVE2 '];
T[' SQRDMLSH VREG={REG:s8=$Zda   VDS=B} VREG={REG:s8=$Zn   VDS=B} VREG={REG:s8=$Zm   VDS=B} ', 'size=0b00 ', 'sqrdmlsh_z_zzz_ ', 'extensions=SVE2 '];
T[' SQRDMLSH VREG={REG:s16=$Zda VDS=H} VREG={REG:s16=$Zn VDS=H} VREG={REG:s16=$Zm VDS=H}    ', 'size=0b01 ', 'sqrdmlsh_z_zzz_ ', 'extensions=SVE2 '];
T[' SQRDMLSH VREG={REG:s32=$Zda VDS=S} VREG={REG:s32=$Zn VDS=S} VREG={REG:s32=$Zm VDS=S}    ', 'size=0b10 ', 'sqrdmlsh_z_zzz_ ', 'extensions=SVE2 '];
T[' SQRDMLSH VREG={REG:s64=$Zda VDS=D} VREG={REG:s64=$Zn VDS=D} VREG={REG:s64=$Zm VDS=D}    ', 'size=0b11 ', 'sqrdmlsh_z_zzz_ ', 'extensions=SVE2 '];



# sve_intx_qrdmlah_by_indexed_elem: SVE2 saturating multiply-add high (indexed).
# A64: ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b00010 S:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_qrdmlah_by_indexed_elem', 'A64');

DEF Zda => SVEREG('Zda', 'Zda', 'read|write') ;
DEF Zm  => SVEREG('Zm', 'Zm', 'read')         ;
DEF Zn  => SVEREG('Zn', 'Zn', 'read')         ;

T[' SQRDMLAH VREG={REG:s16=$Zda VDS=H} VREG={REG:s16=$Zn VDS=H} EREG={REG:s16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'sqrdmlah_z_zzzi_h ', 'extensions=SVE2 '];
T[' SQRDMLAH VREG={REG:s32=$Zda VDS=S} VREG={REG:s32=$Zn VDS=S} EREG={REG:s32=$Zm VDS=S IDX={ENCODEDIN=i2}}      ', 'NONE ', 'sqrdmlah_z_zzzi_s ', 'extensions=SVE2 '];
T[' SQRDMLAH VREG={REG:s64=$Zda VDS=D} VREG={REG:s64=$Zn VDS=D} EREG={REG:s64=$Zm VDS=D IDX={ENCODEDIN=i1}}      ', 'NONE ', 'sqrdmlah_z_zzzi_d ', 'extensions=SVE2 '];
T[' SQRDMLSH VREG={REG:s16=$Zda VDS=H} VREG={REG:s16=$Zn VDS=H} EREG={REG:s16=$Zm VDS=H IDX={ENCODEDIN=i3h:i3l}} ', 'NONE ', 'sqrdmlsh_z_zzzi_h ', 'extensions=SVE2 '];
T[' SQRDMLSH VREG={REG:s32=$Zda VDS=S} VREG={REG:s32=$Zn VDS=S} EREG={REG:s32=$Zm VDS=S IDX={ENCODEDIN=i2}}      ', 'NONE ', 'sqrdmlsh_z_zzzi_s ', 'extensions=SVE2 '];
T[' SQRDMLSH VREG={REG:s64=$Zda VDS=D} VREG={REG:s64=$Zn VDS=D} EREG={REG:s64=$Zm VDS=D IDX={ENCODEDIN=i1}}      ', 'NONE ', 'sqrdmlsh_z_zzzi_d ', 'extensions=SVE2 '];



# sve_intx_shift_insert: SVE2 bitwise shift and insert.
# A64: ig0=0b01000101 tszh:u=0bxx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b11110 op:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_shift_insert', 'A64');

DEF Zd     => SVEREG('Zd', 'Zd', 'write')                      ;
DEF Zn     => SVEREG('Zn', 'Zn', 'read')                       ;
DEF const  => IMM('const', 'tszh:tszl:imm3', range => (0, 7))  ;
DEF const2 => IMM('const', 'tszh:tszl:imm3', range => (0, 15)) ;
DEF const3 => IMM('const', 'tszh:tszl:imm3', range => (0, 31)) ;
DEF const4 => IMM('const', 'tszh:tszl:imm3', range => (0, 63)) ;
DEF const5 => IMM('const', 'tszh:tszl:imm3', range => (1, 8))  ;
DEF const6 => IMM('const', 'tszh:tszl:imm3', range => (1, 16)) ;
DEF const7 => IMM('const', 'tszh:tszl:imm3', range => (1, 32)) ;
DEF const8 => IMM('const', 'tszh:tszl:imm3', range => (1, 64)) ;

T[' SLI VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} IMM:u8=$const  ', 'tszh=0b00 tszl=0b01 ', 'sli_z_zzi_ ', 'extensions=SVE2 '];
T[' SLI VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} IMM:u8=$const2   ', 'tszh=0b00 tszl=0b1x ', 'sli_z_zzi_ ', 'extensions=SVE2 '];
T[' SLI VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} IMM:u8=$const3   ', 'tszh=0b01           ', 'sli_z_zzi_ ', 'extensions=SVE2 '];
T[' SLI VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} IMM:u8=$const4   ', 'tszh=0b1x           ', 'sli_z_zzi_ ', 'extensions=SVE2 '];
T[' SRI VREG={REG:i8=$Zd   VDS=B} VREG={REG:i8=$Zn   VDS=B} IMM:u8=$const5 ', 'tszh=0b00 tszl=0b01 ', 'sri_z_zzi_ ', 'extensions=SVE2 '];
T[' SRI VREG={REG:i16=$Zd VDS=H} VREG={REG:i16=$Zn VDS=H} IMM:u8=$const6   ', 'tszh=0b00 tszl=0b1x ', 'sri_z_zzi_ ', 'extensions=SVE2 '];
T[' SRI VREG={REG:i32=$Zd VDS=S} VREG={REG:i32=$Zn VDS=S} IMM:u8=$const7   ', 'tszh=0b01           ', 'sri_z_zzi_ ', 'extensions=SVE2 '];
T[' SRI VREG={REG:i64=$Zd VDS=D} VREG={REG:i64=$Zn VDS=D} IMM:u8=$const8   ', 'tszh=0b1x           ', 'sri_z_zzi_ ', 'extensions=SVE2 '];



# sve_intx_shift_long: SVE2 bitwise shift left long.
# A64: ig0=0b010001010 tszh:u=0bx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1010 U:u=0bx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_shift_long', 'A64');

DEF Zd     => SVEREG('Zd', 'Zd', 'write')                      ;
DEF Zn     => SVEREG('Zn', 'Zn', 'read')                       ;
DEF const  => IMM('const', 'tszh:tszl:imm3', range => (0, 15)) ;
DEF const2 => IMM('const', 'tszh:tszl:imm3', range => (0, 31)) ;
DEF const3 => IMM('const', 'tszh:tszl:imm3', range => (0, 63)) ;

T[' SSHLLB VREG={REG:s16=$Zd VDS=H} VREG={REG:s8=$Zn   VDS=B} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'sshllb_z_zi_ ', 'extensions=SVE2 '];
T[' SSHLLB VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'sshllb_z_zi_ ', 'extensions=SVE2 '];
T[' SSHLLB VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} IMM:u8=$const3 ', 'tszh=0b1           ', 'sshllb_z_zi_ ', 'extensions=SVE2 '];
T[' SSHLLT VREG={REG:s16=$Zd VDS=H} VREG={REG:s8=$Zn   VDS=B} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'sshllt_z_zi_ ', 'extensions=SVE2 '];
T[' SSHLLT VREG={REG:s32=$Zd VDS=S} VREG={REG:s16=$Zn VDS=H} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'sshllt_z_zi_ ', 'extensions=SVE2 '];
T[' SSHLLT VREG={REG:s64=$Zd VDS=D} VREG={REG:s32=$Zn VDS=S} IMM:u8=$const3 ', 'tszh=0b1           ', 'sshllt_z_zi_ ', 'extensions=SVE2 '];
T[' USHLLB VREG={REG:u16=$Zd VDS=H} VREG={REG:u8=$Zn   VDS=B} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'ushllb_z_zi_ ', 'extensions=SVE2 '];
T[' USHLLB VREG={REG:u32=$Zd VDS=S} VREG={REG:u16=$Zn VDS=H} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'ushllb_z_zi_ ', 'extensions=SVE2 '];
T[' USHLLB VREG={REG:u64=$Zd VDS=D} VREG={REG:u32=$Zn VDS=S} IMM:u8=$const3 ', 'tszh=0b1           ', 'ushllb_z_zi_ ', 'extensions=SVE2 '];
T[' USHLLT VREG={REG:u16=$Zd VDS=H} VREG={REG:u8=$Zn   VDS=B} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'ushllt_z_zi_ ', 'extensions=SVE2 '];
T[' USHLLT VREG={REG:u32=$Zd VDS=S} VREG={REG:u16=$Zn VDS=H} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'ushllt_z_zi_ ', 'extensions=SVE2 '];
T[' USHLLT VREG={REG:u64=$Zd VDS=D} VREG={REG:u32=$Zn VDS=S} IMM:u8=$const3 ', 'tszh=0b1           ', 'ushllt_z_zi_ ', 'extensions=SVE2 '];



# sve_intx_shift_narrow: SVE2 bitwise shift right narrow.
# A64: ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op:u=0bx U:u=0bx R:u=0bx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx
HINT('ICLASS', 'sve_intx_shift_narrow', 'A64');

DEF Zd     => SVEREG('Zd', 'Zd', 'write')                      ;
DEF Zn     => SVEREG('Zn', 'Zn', 'read')                       ;
DEF const  => IMM('const', 'tszh:tszl:imm3', range => (1, 8))  ;
DEF const2 => IMM('const', 'tszh:tszl:imm3', range => (1, 16)) ;
DEF const3 => IMM('const', 'tszh:tszl:imm3', range => (1, 32)) ;

T[' RSHRNB    VREG={REG:i8=$Zd   VDS=B} VREG={REG:i16=$Zn VDS=H} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'rshrnb_z_zi_    ', 'extensions=SVE2 '];
T[' RSHRNB    VREG={REG:i16=$Zd VDS=H} VREG={REG:i32=$Zn VDS=S} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'rshrnb_z_zi_    ', 'extensions=SVE2 '];
T[' RSHRNB    VREG={REG:i32=$Zd VDS=S} VREG={REG:i64=$Zn VDS=D} IMM:u8=$const3 ', 'tszh=0b1           ', 'rshrnb_z_zi_    ', 'extensions=SVE2 '];
T[' RSHRNT    VREG={REG:i8=$Zd   VDS=B} VREG={REG:i16=$Zn VDS=H} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'rshrnt_z_zi_    ', 'extensions=SVE2 '];
T[' RSHRNT    VREG={REG:i16=$Zd VDS=H} VREG={REG:i32=$Zn VDS=S} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'rshrnt_z_zi_    ', 'extensions=SVE2 '];
T[' RSHRNT    VREG={REG:i32=$Zd VDS=S} VREG={REG:i64=$Zn VDS=D} IMM:u8=$const3 ', 'tszh=0b1           ', 'rshrnt_z_zi_    ', 'extensions=SVE2 '];
T[' SHRNB     VREG={REG:i8=$Zd   VDS=B} VREG={REG:i16=$Zn VDS=H} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'shrnb_z_zi_     ', 'extensions=SVE2 '];
T[' SHRNB     VREG={REG:i16=$Zd VDS=H} VREG={REG:i32=$Zn VDS=S} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'shrnb_z_zi_     ', 'extensions=SVE2 '];
T[' SHRNB     VREG={REG:i32=$Zd VDS=S} VREG={REG:i64=$Zn VDS=D} IMM:u8=$const3 ', 'tszh=0b1           ', 'shrnb_z_zi_     ', 'extensions=SVE2 '];
T[' SHRNT     VREG={REG:i8=$Zd   VDS=B} VREG={REG:i16=$Zn VDS=H} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'shrnt_z_zi_     ', 'extensions=SVE2 '];
T[' SHRNT     VREG={REG:i16=$Zd VDS=H} VREG={REG:i32=$Zn VDS=S} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'shrnt_z_zi_     ', 'extensions=SVE2 '];
T[' SHRNT     VREG={REG:i32=$Zd VDS=S} VREG={REG:i64=$Zn VDS=D} IMM:u8=$const3 ', 'tszh=0b1           ', 'shrnt_z_zi_     ', 'extensions=SVE2 '];
T[' SQRSHRNB  VREG={REG:s8=$Zd   VDS=B} VREG={REG:s16=$Zn VDS=H} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'sqrshrnb_z_zi_  ', 'extensions=SVE2 '];
T[' SQRSHRNB  VREG={REG:s16=$Zd VDS=H} VREG={REG:s32=$Zn VDS=S} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'sqrshrnb_z_zi_  ', 'extensions=SVE2 '];
T[' SQRSHRNB  VREG={REG:s32=$Zd VDS=S} VREG={REG:s64=$Zn VDS=D} IMM:u8=$const3 ', 'tszh=0b1           ', 'sqrshrnb_z_zi_  ', 'extensions=SVE2 '];
T[' SQRSHRNT  VREG={REG:s8=$Zd   VDS=B} VREG={REG:s16=$Zn VDS=H} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'sqrshrnt_z_zi_  ', 'extensions=SVE2 '];
T[' SQRSHRNT  VREG={REG:s16=$Zd VDS=H} VREG={REG:s32=$Zn VDS=S} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'sqrshrnt_z_zi_  ', 'extensions=SVE2 '];
T[' SQRSHRNT  VREG={REG:s32=$Zd VDS=S} VREG={REG:s64=$Zn VDS=D} IMM:u8=$const3 ', 'tszh=0b1           ', 'sqrshrnt_z_zi_  ', 'extensions=SVE2 '];
T[' SQRSHRUNB VREG={REG:u8=$Zd   VDS=B} VREG={REG:u16=$Zn VDS=H} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'sqrshrunb_z_zi_ ', 'extensions=SVE2 '];
T[' SQRSHRUNB VREG={REG:u16=$Zd VDS=H} VREG={REG:u32=$Zn VDS=S} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'sqrshrunb_z_zi_ ', 'extensions=SVE2 '];
T[' SQRSHRUNB VREG={REG:u32=$Zd VDS=S} VREG={REG:u64=$Zn VDS=D} IMM:u8=$const3 ', 'tszh=0b1           ', 'sqrshrunb_z_zi_ ', 'extensions=SVE2 '];
T[' SQRSHRUNT VREG={REG:u8=$Zd   VDS=B} VREG={REG:u16=$Zn VDS=H} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'sqrshrunt_z_zi_ ', 'extensions=SVE2 '];
T[' SQRSHRUNT VREG={REG:u16=$Zd VDS=H} VREG={REG:u32=$Zn VDS=S} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'sqrshrunt_z_zi_ ', 'extensions=SVE2 '];
T[' SQRSHRUNT VREG={REG:u32=$Zd VDS=S} VREG={REG:u64=$Zn VDS=D} IMM:u8=$const3 ', 'tszh=0b1           ', 'sqrshrunt_z_zi_ ', 'extensions=SVE2 '];
T[' SQSHRNB   VREG={REG:s8=$Zd   VDS=B} VREG={REG:s16=$Zn VDS=H} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'sqshrnb_z_zi_   ', 'extensions=SVE2 '];
T[' SQSHRNB   VREG={REG:s16=$Zd VDS=H} VREG={REG:s32=$Zn VDS=S} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'sqshrnb_z_zi_   ', 'extensions=SVE2 '];
T[' SQSHRNB   VREG={REG:s32=$Zd VDS=S} VREG={REG:s64=$Zn VDS=D} IMM:u8=$const3 ', 'tszh=0b1           ', 'sqshrnb_z_zi_   ', 'extensions=SVE2 '];
T[' SQSHRNT   VREG={REG:s8=$Zd   VDS=B} VREG={REG:s16=$Zn VDS=H} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'sqshrnt_z_zi_   ', 'extensions=SVE2 '];
T[' SQSHRNT   VREG={REG:s16=$Zd VDS=H} VREG={REG:s32=$Zn VDS=S} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'sqshrnt_z_zi_   ', 'extensions=SVE2 '];
T[' SQSHRNT   VREG={REG:s32=$Zd VDS=S} VREG={REG:s64=$Zn VDS=D} IMM:u8=$const3 ', 'tszh=0b1           ', 'sqshrnt_z_zi_   ', 'extensions=SVE2 '];
T[' SQSHRUNB  VREG={REG:u8=$Zd   VDS=B} VREG={REG:u16=$Zn VDS=H} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'sqshrunb_z_zi_  ', 'extensions=SVE2 '];
T[' SQSHRUNB  VREG={REG:u16=$Zd VDS=H} VREG={REG:u32=$Zn VDS=S} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'sqshrunb_z_zi_  ', 'extensions=SVE2 '];
T[' SQSHRUNB  VREG={REG:u32=$Zd VDS=S} VREG={REG:u64=$Zn VDS=D} IMM:u8=$const3 ', 'tszh=0b1           ', 'sqshrunb_z_zi_  ', 'extensions=SVE2 '];
T[' SQSHRUNT  VREG={REG:u8=$Zd   VDS=B} VREG={REG:u16=$Zn VDS=H} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'sqshrunt_z_zi_  ', 'extensions=SVE2 '];
T[' SQSHRUNT  VREG={REG:u16=$Zd VDS=H} VREG={REG:u32=$Zn VDS=S} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'sqshrunt_z_zi_  ', 'extensions=SVE2 '];
T[' SQSHRUNT  VREG={REG:u32=$Zd VDS=S} VREG={REG:u64=$Zn VDS=D} IMM:u8=$const3 ', 'tszh=0b1           ', 'sqshrunt_z_zi_  ', 'extensions=SVE2 '];
T[' UQRSHRNB  VREG={REG:u8=$Zd   VDS=B} VREG={REG:u16=$Zn VDS=H} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'uqrshrnb_z_zi_  ', 'extensions=SVE2 '];
T[' UQRSHRNB  VREG={REG:u16=$Zd VDS=H} VREG={REG:u32=$Zn VDS=S} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'uqrshrnb_z_zi_  ', 'extensions=SVE2 '];
T[' UQRSHRNB  VREG={REG:u32=$Zd VDS=S} VREG={REG:u64=$Zn VDS=D} IMM:u8=$const3 ', 'tszh=0b1           ', 'uqrshrnb_z_zi_  ', 'extensions=SVE2 '];
T[' UQRSHRNT  VREG={REG:u8=$Zd   VDS=B} VREG={REG:u16=$Zn VDS=H} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'uqrshrnt_z_zi_  ', 'extensions=SVE2 '];
T[' UQRSHRNT  VREG={REG:u16=$Zd VDS=H} VREG={REG:u32=$Zn VDS=S} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'uqrshrnt_z_zi_  ', 'extensions=SVE2 '];
T[' UQRSHRNT  VREG={REG:u32=$Zd VDS=S} VREG={REG:u64=$Zn VDS=D} IMM:u8=$const3 ', 'tszh=0b1           ', 'uqrshrnt_z_zi_  ', 'extensions=SVE2 '];
T[' UQSHRNB   VREG={REG:u8=$Zd   VDS=B} VREG={REG:u16=$Zn VDS=H} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'uqshrnb_z_zi_   ', 'extensions=SVE2 '];
T[' UQSHRNB   VREG={REG:u16=$Zd VDS=H} VREG={REG:u32=$Zn VDS=S} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'uqshrnb_z_zi_   ', 'extensions=SVE2 '];
T[' UQSHRNB   VREG={REG:u32=$Zd VDS=S} VREG={REG:u64=$Zn VDS=D} IMM:u8=$const3 ', 'tszh=0b1           ', 'uqshrnb_z_zi_   ', 'extensions=SVE2 '];
T[' UQSHRNT   VREG={REG:u8=$Zd   VDS=B} VREG={REG:u16=$Zn VDS=H} IMM:u8=$const ', 'tszh=0b0 tszl=0b01 ', 'uqshrnt_z_zi_   ', 'extensions=SVE2 '];
T[' UQSHRNT   VREG={REG:u16=$Zd VDS=H} VREG={REG:u32=$Zn VDS=S} IMM:u8=$const2 ', 'tszh=0b0 tszl=0b1x ', 'uqshrnt_z_zi_   ', 'extensions=SVE2 '];
T[' UQSHRNT   VREG={REG:u32=$Zd VDS=S} VREG={REG:u64=$Zn VDS=D} IMM:u8=$const3 ', 'tszh=0b1           ', 'uqshrnt_z_zi_   ', 'extensions=SVE2 '];



# sve_intx_sra: SVE2 bitwise shift right and accumulate.
# A64: ig0=0b01000101 tszh:u=0bxx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1110 R:u=0bx U:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx
HINT('ICLASS', 'sve_intx_sra', 'A64');

DEF Zda    => SVEREG('Zda', 'Zda', 'read|write')               ;
DEF Zn     => SVEREG('Zn', 'Zn', 'read')                       ;
DEF const  => IMM('const', 'tszh:tszl:imm3', range => (1, 8))  ;
DEF const2 => IMM('const', 'tszh:tszl:imm3', range => (1, 16)) ;
DEF const3 => IMM('const', 'tszh:tszl:imm3', range => (1, 32)) ;
DEF const4 => IMM('const', 'tszh:tszl:imm3', range => (1, 64)) ;

T[' SRSRA VREG={REG:s8=$Zda   VDS=B} VREG={REG:s8=$Zn   VDS=B} IMM:u8=$const ', 'tszh=0b00 tszl=0b01 ', 'srsra_z_zi_ ', 'extensions=SVE2 '];
T[' SRSRA VREG={REG:s16=$Zda VDS=H} VREG={REG:s16=$Zn VDS=H} IMM:u8=$const2  ', 'tszh=0b00 tszl=0b1x ', 'srsra_z_zi_ ', 'extensions=SVE2 '];
T[' SRSRA VREG={REG:s32=$Zda VDS=S} VREG={REG:s32=$Zn VDS=S} IMM:u8=$const3  ', 'tszh=0b01           ', 'srsra_z_zi_ ', 'extensions=SVE2 '];
T[' SRSRA VREG={REG:s64=$Zda VDS=D} VREG={REG:s64=$Zn VDS=D} IMM:u8=$const4  ', 'tszh=0b1x           ', 'srsra_z_zi_ ', 'extensions=SVE2 '];
T[' SSRA  VREG={REG:s8=$Zda   VDS=B} VREG={REG:s8=$Zn   VDS=B} IMM:u8=$const ', 'tszh=0b00 tszl=0b01 ', 'ssra_z_zi_  ', 'extensions=SVE2 '];
T[' SSRA  VREG={REG:s16=$Zda VDS=H} VREG={REG:s16=$Zn VDS=H} IMM:u8=$const2  ', 'tszh=0b00 tszl=0b1x ', 'ssra_z_zi_  ', 'extensions=SVE2 '];
T[' SSRA  VREG={REG:s32=$Zda VDS=S} VREG={REG:s32=$Zn VDS=S} IMM:u8=$const3  ', 'tszh=0b01           ', 'ssra_z_zi_  ', 'extensions=SVE2 '];
T[' SSRA  VREG={REG:s64=$Zda VDS=D} VREG={REG:s64=$Zn VDS=D} IMM:u8=$const4  ', 'tszh=0b1x           ', 'ssra_z_zi_  ', 'extensions=SVE2 '];
T[' URSRA VREG={REG:u8=$Zda   VDS=B} VREG={REG:u8=$Zn   VDS=B} IMM:u8=$const ', 'tszh=0b00 tszl=0b01 ', 'ursra_z_zi_ ', 'extensions=SVE2 '];
T[' URSRA VREG={REG:u16=$Zda VDS=H} VREG={REG:u16=$Zn VDS=H} IMM:u8=$const2  ', 'tszh=0b00 tszl=0b1x ', 'ursra_z_zi_ ', 'extensions=SVE2 '];
T[' URSRA VREG={REG:u32=$Zda VDS=S} VREG={REG:u32=$Zn VDS=S} IMM:u8=$const3  ', 'tszh=0b01           ', 'ursra_z_zi_ ', 'extensions=SVE2 '];
T[' URSRA VREG={REG:u64=$Zda VDS=D} VREG={REG:u64=$Zn VDS=D} IMM:u8=$const4  ', 'tszh=0b1x           ', 'ursra_z_zi_ ', 'extensions=SVE2 '];
T[' USRA  VREG={REG:u8=$Zda   VDS=B} VREG={REG:u8=$Zn   VDS=B} IMM:u8=$const ', 'tszh=0b00 tszl=0b01 ', 'usra_z_zi_  ', 'extensions=SVE2 '];
T[' USRA  VREG={REG:u16=$Zda VDS=H} VREG={REG:u16=$Zn VDS=H} IMM:u8=$const2  ', 'tszh=0b00 tszl=0b1x ', 'usra_z_zi_  ', 'extensions=SVE2 '];
T[' USRA  VREG={REG:u32=$Zda VDS=S} VREG={REG:u32=$Zn VDS=S} IMM:u8=$const3  ', 'tszh=0b01           ', 'usra_z_zi_  ', 'extensions=SVE2 '];
T[' USRA  VREG={REG:u64=$Zda VDS=D} VREG={REG:u64=$Zn VDS=D} IMM:u8=$const4  ', 'tszh=0b1x           ', 'usra_z_zi_  ', 'extensions=SVE2 '];



# sve_mem_32b_fill: SVE load vector register.
# A64: ig0=0b1000010110 imm9h:u=0bxxxxxx ig1=0b010 imm9l:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_32b_fill', 'A64');

DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                                                         ;
DEF Zt   => SVEREG('Zt', 'Zt')                                                                   ;
DEF imm  => IMM('imm', 'imm9h:imm9l', signpos => 'imm9h<5>', range => (-256, 255), default => 0) ;

T[' LDR REG:w=$Zt MEM:OFF:r:i8={ BASE=$XnSP SZ=vl IMMOFFSH:OPT={IMM:s16=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ldr_z_bi_ ', 'extensions=SVE '];



# sve_mem_32b_gld_sv_a: SVE 32-bit gather load halfwords (scalar plus 32-bit scaled offsets).
# A64: ig0=0b100001001 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U:u=0bx ff:u=0bx Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_32b_gld_sv_a', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')                  ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                ;
DEF Zm   => SVEREG('Zm', 'Zm', 'read')                  ;
DEF Zt   => SVEREG('Zt', 'Zt')                          ;
DEF mod  => EXTEND('mod', 'xs', table => 'tbl_extend5') ;

T[' LD1H    VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v32x16 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod AMOUNT:u8=1}}}  ', 'NONE ', 'ld1h_z_p_bz_s_x32_scaled    ', 'extensions=SVE '];
T[' LD1SH   VLIST={VBASE={REG:w:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v32x16 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod AMOUNT:u8=1}}}  ', 'NONE ', 'ld1sh_z_p_bz_s_x32_scaled   ', 'extensions=SVE '];
T[' LDFF1H  VLIST={VBASE={REG:rw:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v32x16 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod AMOUNT:u8=1}}} ', 'NONE ', 'ldff1h_z_p_bz_s_x32_scaled  ', 'extensions=SVE '];
T[' LDFF1SH VLIST={VBASE={REG:rw:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v32x16 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod AMOUNT:u8=1}}} ', 'NONE ', 'ldff1sh_z_p_bz_s_x32_scaled ', 'extensions=SVE '];



# sve_mem_32b_gld_sv_b: SVE 32-bit gather load words (scalar plus 32-bit scaled offsets).
# A64: ig0=0b100001010 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U:u=0bx ff:u=0bx Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_32b_gld_sv_b', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')                  ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                ;
DEF Zm   => SVEREG('Zm', 'Zm', 'read')                  ;
DEF Zt   => SVEREG('Zt', 'Zt')                          ;
DEF mod  => EXTEND('mod', 'xs', table => 'tbl_extend5') ;

T[' LD1W   VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v32x32 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod AMOUNT:u8=2}}}  ', 'NONE ', 'ld1w_z_p_bz_s_x32_scaled   ', 'extensions=SVE '];
T[' LDFF1W VLIST={VBASE={REG:rw:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v32x32 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod AMOUNT:u8=2}}} ', 'NONE ', 'ldff1w_z_p_bz_s_x32_scaled ', 'extensions=SVE '];



# sve_mem_32b_gld_vi: SVE 32-bit gather load (vector plus immediate).
# A64: ig0=0b1000010 msz:u=0bxx ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U:u=0bx ff:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_32b_gld_vi', 'A64');

DEF Pg    => PRDREG('Pg', 'Pg', 'read')                                      ;
DEF Zn    => SVEREG('Zn', 'Zn', 'read')                                      ;
DEF Zt    => SVEREG('Zt', 'Zt')                                              ;
DEF imm   => IMM('imm', 'imm5', range => (0, 31), default => 0)              ;
DEF immx2 => IMM('imm', 'imm5', scale => 2, range => (0, 62), default => 0)  ;
DEF immx4 => IMM('imm', 'imm5', scale => 4, range => (0, 124), default => 0) ;

T[' LD1B    VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ VBASE={REG=$Zn VDS=S} SZ=v32x8 IMMOFF:OPT:u8=$imm}     ', 'NONE ', 'ld1b_z_p_ai_s    ', 'extensions=SVE '];
T[' LD1H    VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={VBASE={REG=$Zn VDS=S} SZ=v32x16 IMMOFF:OPT:u8=$immx2}  ', 'NONE ', 'ld1h_z_p_ai_s    ', 'extensions=SVE '];
T[' LD1SB   VLIST={VBASE={REG:w:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ VBASE={REG=$Zn VDS=S} SZ=v32x8 IMMOFF:OPT:u8=$imm}     ', 'NONE ', 'ld1sb_z_p_ai_s   ', 'extensions=SVE '];
T[' LD1SH   VLIST={VBASE={REG:w:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={VBASE={REG=$Zn VDS=S} SZ=v32x16 IMMOFF:OPT:u8=$immx2}  ', 'NONE ', 'ld1sh_z_p_ai_s   ', 'extensions=SVE '];
T[' LD1W    VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={VBASE={REG=$Zn VDS=S} SZ=v32x32 IMMOFF:OPT:u8=$immx4}  ', 'NONE ', 'ld1w_z_p_ai_s    ', 'extensions=SVE '];
T[' LDFF1B  VLIST={VBASE={REG:rw:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ VBASE={REG=$Zn VDS=S} SZ=v32x8 IMMOFF:OPT:u8=$imm}    ', 'NONE ', 'ldff1b_z_p_ai_s  ', 'extensions=SVE '];
T[' LDFF1H  VLIST={VBASE={REG:rw:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={VBASE={REG=$Zn VDS=S} SZ=v32x16 IMMOFF:OPT:u8=$immx2} ', 'NONE ', 'ldff1h_z_p_ai_s  ', 'extensions=SVE '];
T[' LDFF1SB VLIST={VBASE={REG:rw:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ VBASE={REG=$Zn VDS=S} SZ=v32x8 IMMOFF:OPT:u8=$imm}    ', 'NONE ', 'ldff1sb_z_p_ai_s ', 'extensions=SVE '];
T[' LDFF1SH VLIST={VBASE={REG:rw:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={VBASE={REG=$Zn VDS=S} SZ=v32x16 IMMOFF:OPT:u8=$immx2} ', 'NONE ', 'ldff1sh_z_p_ai_s ', 'extensions=SVE '];
T[' LDFF1W  VLIST={VBASE={REG:rw:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={VBASE={REG=$Zn VDS=S} SZ=v32x32 IMMOFF:OPT:u8=$immx4} ', 'NONE ', 'ldff1w_z_p_ai_s  ', 'extensions=SVE '];



# sve_mem_32b_gld_vs: SVE 32-bit gather load (scalar plus 32-bit unscaled offsets).
# A64: ig0=0b1000010 opc:u!=0b11 xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U:u=0bx ff:u=0bx Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_32b_gld_vs', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')                  ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                ;
DEF Zm   => SVEREG('Zm', 'Zm', 'read')                  ;
DEF Zt   => SVEREG('Zt', 'Zt')                          ;
DEF mod  => EXTEND('mod', 'xs', table => 'tbl_extend5') ;

T[' LD1B    VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v32x8 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod}}}   ', 'NONE ', 'ld1b_z_p_bz_s_x32_unscaled    ', 'extensions=SVE '];
T[' LD1H    VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v32x16 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod}}}  ', 'NONE ', 'ld1h_z_p_bz_s_x32_unscaled    ', 'extensions=SVE '];
T[' LD1SB   VLIST={VBASE={REG:w:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v32x8 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod}}}   ', 'NONE ', 'ld1sb_z_p_bz_s_x32_unscaled   ', 'extensions=SVE '];
T[' LD1SH   VLIST={VBASE={REG:w:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v32x16 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod}}}  ', 'NONE ', 'ld1sh_z_p_bz_s_x32_unscaled   ', 'extensions=SVE '];
T[' LD1W    VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v32x32 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod}}}  ', 'NONE ', 'ld1w_z_p_bz_s_x32_unscaled    ', 'extensions=SVE '];
T[' LDFF1B  VLIST={VBASE={REG:rw:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v32x8 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod}}}  ', 'NONE ', 'ldff1b_z_p_bz_s_x32_unscaled  ', 'extensions=SVE '];
T[' LDFF1H  VLIST={VBASE={REG:rw:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v32x16 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod}}} ', 'NONE ', 'ldff1h_z_p_bz_s_x32_unscaled  ', 'extensions=SVE '];
T[' LDFF1SB VLIST={VBASE={REG:rw:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v32x8 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod}}}  ', 'NONE ', 'ldff1sb_z_p_bz_s_x32_unscaled ', 'extensions=SVE '];
T[' LDFF1SH VLIST={VBASE={REG:rw:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v32x16 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod}}} ', 'NONE ', 'ldff1sh_z_p_bz_s_x32_unscaled ', 'extensions=SVE '];
T[' LDFF1W  VLIST={VBASE={REG:rw:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v32x32 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod}}} ', 'NONE ', 'ldff1w_z_p_bz_s_x32_unscaled  ', 'extensions=SVE '];



# sve_mem_32b_gldnt_vs: SVE2 32-bit gather non-temporal load (scalar plus 32-bit unscaled offsets).
# A64: ig0=0b1000010 msz:u=0bxx ig1=0b00 Rm:u=0bxxxxx ig2=0b10 U:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_32b_gldnt_vs', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')                  ;
DEF Xm => GPR64('Xm', 'Rm', default => 'XZR', 'read') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')                  ;
DEF Zt => SVEREG('Zt', 'Zt')                          ;

T[' LDNT1B  VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ VBASE={REG=$Zn VDS=S} SZ=v32x8 REGOFF=$Xm}  ', 'NONE ', 'ldnt1b_z_p_ar_s_x32_unscaled  ', 'extensions=SVE2 '];
T[' LDNT1H  VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={VBASE={REG=$Zn VDS=S} SZ=v32x16 REGOFF=$Xm} ', 'NONE ', 'ldnt1h_z_p_ar_s_x32_unscaled  ', 'extensions=SVE2 '];
T[' LDNT1SB VLIST={VBASE={REG:w:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ VBASE={REG=$Zn VDS=S} SZ=v32x8 REGOFF=$Xm}  ', 'NONE ', 'ldnt1sb_z_p_ar_s_x32_unscaled ', 'extensions=SVE2 '];
T[' LDNT1SH VLIST={VBASE={REG:w:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={VBASE={REG=$Zn VDS=S} SZ=v32x16 REGOFF=$Xm} ', 'NONE ', 'ldnt1sh_z_p_ar_s_x32_unscaled ', 'extensions=SVE2 '];
T[' LDNT1W  VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={VBASE={REG=$Zn VDS=S} SZ=v32x32 REGOFF=$Xm} ', 'NONE ', 'ldnt1w_z_p_ar_s_x32_unscaled  ', 'extensions=SVE2 '];



# sve_mem_32b_pfill: SVE load predicate register.
# A64: ig0=0b1000010110 imm9h:u=0bxxxxxx ig1=0b000 imm9l:u=0bxxx Rn:u=0bxxxxx ig2=0b0 Pt:u=0bxxxx
HINT('ICLASS', 'sve_mem_32b_pfill', 'A64');

DEF Pt   => PRDREG('Pt', 'Pt')                                                                   ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                                                         ;
DEF imm  => IMM('imm', 'imm9h:imm9l', signpos => 'imm9h<5>', range => (-256, 255), default => 0) ;

T[' LDR REG:w=$Pt MEM:OFF:r:i8={ BASE=$XnSP SZ=pl IMMOFFSH:OPT={IMM:s16=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ldr_p_bi_ ', 'extensions=SVE '];



# sve_mem_32b_prfm_sv: SVE 32-bit gather prefetch (scalar plus 32-bit scaled offsets).
# A64: ig0=0b100001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz:u=0bxx Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx
HINT('ICLASS', 'sve_mem_32b_prfm_sv', 'A64');

DEF Pg    => PRDREG('Pg', 'Pg', 'read')                      ;
DEF XnSP  => GPR64('Xn', 'Rn', 'read|SP')                    ;
DEF Zm    => SVEREG('Zm', 'Zm', 'read')                      ;
DEF mod   => EXTEND('mod', 'xs', table => 'tbl_extend5')     ;
DEF prfop => PRF_OP('prfop', 'prfop', table => 'tbl_prf_op') ;

T[' PRFB PRF_OP=$prfop REG=$Pg MEM:OFF:r:i8={ BASE=$XnSP SZ=v8x8 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod}}}               ', 'NONE ', 'prfb_i_p_bz_s_x32_scaled ', 'extensions=SVE '];
T[' PRFD PRF_OP=$prfop REG=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod AMOUNT:u8=3}}} ', 'NONE ', 'prfd_i_p_bz_s_x32_scaled ', 'extensions=SVE '];
T[' PRFH PRF_OP=$prfop REG=$Pg MEM:OFF:r:i16={BASE=$XnSP SZ=v16x16 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod AMOUNT:u8=1}}} ', 'NONE ', 'prfh_i_p_bz_s_x32_scaled ', 'extensions=SVE '];
T[' PRFW PRF_OP=$prfop REG=$Pg MEM:OFF:r:i32={BASE=$XnSP SZ=v32x32 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod AMOUNT:u8=2}}} ', 'NONE ', 'prfw_i_p_bz_s_x32_scaled ', 'extensions=SVE '];



# sve_mem_32b_prfm_vi: SVE 32-bit gather prefetch (vector plus immediate).
# A64: ig0=0b1000010 msz:u=0bxx ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx
HINT('ICLASS', 'sve_mem_32b_prfm_vi', 'A64');

DEF Pg    => PRDREG('Pg', 'Pg', 'read')                                      ;
DEF Zn    => SVEREG('Zn', 'Zn', 'read')                                      ;
DEF imm   => IMM('imm', 'imm5', range => (0, 31), default => 0)              ;
DEF immx2 => IMM('imm', 'imm5', scale => 2, range => (0, 62), default => 0)  ;
DEF immx4 => IMM('imm', 'imm5', scale => 4, range => (0, 124), default => 0) ;
DEF immx8 => IMM('imm', 'imm5', scale => 8, range => (0, 248), default => 0) ;
DEF prfop => PRF_OP('prfop', 'prfop', table => 'tbl_prf_op')                 ;

T[' PRFB PRF_OP=$prfop REG=$Pg MEM:OFF:r:i8={ VBASE={REG=$Zn VDS=S} SZ=v8x8 IMMOFF:OPT:u8=$imm}     ', 'NONE ', 'prfb_i_p_ai_s ', 'extensions=SVE '];
T[' PRFD PRF_OP=$prfop REG=$Pg MEM:OFF:r:i64={VBASE={REG=$Zn VDS=S} SZ=v64x64 IMMOFF:OPT:u8=$immx8} ', 'NONE ', 'prfd_i_p_ai_s ', 'extensions=SVE '];
T[' PRFH PRF_OP=$prfop REG=$Pg MEM:OFF:r:i16={VBASE={REG=$Zn VDS=S} SZ=v16x16 IMMOFF:OPT:u8=$immx2} ', 'NONE ', 'prfh_i_p_ai_s ', 'extensions=SVE '];
T[' PRFW PRF_OP=$prfop REG=$Pg MEM:OFF:r:i32={VBASE={REG=$Zn VDS=S} SZ=v32x32 IMMOFF:OPT:u8=$immx4} ', 'NONE ', 'prfw_i_p_ai_s ', 'extensions=SVE '];



# sve_mem_64b_gld_sv: SVE 64-bit gather load (scalar plus 32-bit unpacked scaled offsets).
# A64: ig0=0b1100010 opc:u!=0b00 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U:u=0bx ff:u=0bx Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_64b_gld_sv', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')                  ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                ;
DEF Zm   => SVEREG('Zm', 'Zm', 'read')                  ;
DEF Zt   => SVEREG('Zt', 'Zt')                          ;
DEF mod  => EXTEND('mod', 'xs', table => 'tbl_extend5') ;

T[' LD1D    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod AMOUNT:u8=3}}}  ', 'NONE ', 'ld1d_z_p_bz_d_x32_scaled    ', 'extensions=SVE '];
T[' LD1H    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v64x16 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod AMOUNT:u8=1}}}  ', 'NONE ', 'ld1h_z_p_bz_d_x32_scaled    ', 'extensions=SVE '];
T[' LD1SH   VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v64x16 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod AMOUNT:u8=1}}}  ', 'NONE ', 'ld1sh_z_p_bz_d_x32_scaled   ', 'extensions=SVE '];
T[' LD1SW   VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s32={BASE=$XnSP SZ=v64x32 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod AMOUNT:u8=2}}}  ', 'NONE ', 'ld1sw_z_p_bz_d_x32_scaled   ', 'extensions=SVE '];
T[' LD1W    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v64x32 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod AMOUNT:u8=2}}}  ', 'NONE ', 'ld1w_z_p_bz_d_x32_scaled    ', 'extensions=SVE '];
T[' LDFF1D  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod AMOUNT:u8=3}}} ', 'NONE ', 'ldff1d_z_p_bz_d_x32_scaled  ', 'extensions=SVE '];
T[' LDFF1H  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v64x16 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod AMOUNT:u8=1}}} ', 'NONE ', 'ldff1h_z_p_bz_d_x32_scaled  ', 'extensions=SVE '];
T[' LDFF1SH VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v64x16 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod AMOUNT:u8=1}}} ', 'NONE ', 'ldff1sh_z_p_bz_d_x32_scaled ', 'extensions=SVE '];
T[' LDFF1SW VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s32={BASE=$XnSP SZ=v64x32 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod AMOUNT:u8=2}}} ', 'NONE ', 'ldff1sw_z_p_bz_d_x32_scaled ', 'extensions=SVE '];
T[' LDFF1W  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v64x32 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod AMOUNT:u8=2}}} ', 'NONE ', 'ldff1w_z_p_bz_d_x32_scaled  ', 'extensions=SVE '];



# sve_mem_64b_gld_sv2: SVE 64-bit gather load (scalar plus 64-bit scaled offsets).
# A64: ig0=0b1100010 opc:u!=0b00 ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U:u=0bx ff:u=0bx Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_64b_gld_sv2', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')   ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP') ;
DEF Zm   => SVEREG('Zm', 'Zm', 'read')   ;
DEF Zt   => SVEREG('Zt', 'Zt')           ;

T[' LD1D    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={SHIFT=LSL AMOUNT:u8=3}}}  ', 'NONE ', 'ld1d_z_p_bz_d_64_scaled    ', 'extensions=SVE '];
T[' LD1H    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v64x16 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={SHIFT=LSL AMOUNT:u8=1}}}  ', 'NONE ', 'ld1h_z_p_bz_d_64_scaled    ', 'extensions=SVE '];
T[' LD1SH   VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v64x16 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={SHIFT=LSL AMOUNT:u8=1}}}  ', 'NONE ', 'ld1sh_z_p_bz_d_64_scaled   ', 'extensions=SVE '];
T[' LD1SW   VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s32={BASE=$XnSP SZ=v64x32 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={SHIFT=LSL AMOUNT:u8=2}}}  ', 'NONE ', 'ld1sw_z_p_bz_d_64_scaled   ', 'extensions=SVE '];
T[' LD1W    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v64x32 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={SHIFT=LSL AMOUNT:u8=2}}}  ', 'NONE ', 'ld1w_z_p_bz_d_64_scaled    ', 'extensions=SVE '];
T[' LDFF1D  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={SHIFT=LSL AMOUNT:u8=3}}} ', 'NONE ', 'ldff1d_z_p_bz_d_64_scaled  ', 'extensions=SVE '];
T[' LDFF1H  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v64x16 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'ldff1h_z_p_bz_d_64_scaled  ', 'extensions=SVE '];
T[' LDFF1SH VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v64x16 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'ldff1sh_z_p_bz_d_64_scaled ', 'extensions=SVE '];
T[' LDFF1SW VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s32={BASE=$XnSP SZ=v64x32 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'ldff1sw_z_p_bz_d_64_scaled ', 'extensions=SVE '];
T[' LDFF1W  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v64x32 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'ldff1w_z_p_bz_d_64_scaled  ', 'extensions=SVE '];



# sve_mem_64b_gld_vi: SVE 64-bit gather load (vector plus immediate).
# A64: ig0=0b1100010 msz:u=0bxx ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U:u=0bx ff:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_64b_gld_vi', 'A64');

DEF Pg    => PRDREG('Pg', 'Pg', 'read')                                      ;
DEF Zn    => SVEREG('Zn', 'Zn', 'read')                                      ;
DEF Zt    => SVEREG('Zt', 'Zt')                                              ;
DEF imm   => IMM('imm', 'imm5', range => (0, 31), default => 0)              ;
DEF immx2 => IMM('imm', 'imm5', scale => 2, range => (0, 62), default => 0)  ;
DEF immx4 => IMM('imm', 'imm5', scale => 4, range => (0, 124), default => 0) ;
DEF immx8 => IMM('imm', 'imm5', scale => 8, range => (0, 248), default => 0) ;

T[' LD1B    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ VBASE={REG=$Zn VDS=D} SZ=v64x8 IMMOFF:OPT:u8=$imm}     ', 'NONE ', 'ld1b_z_p_ai_d    ', 'extensions=SVE '];
T[' LD1D    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={VBASE={REG=$Zn VDS=D} SZ=v64x64 IMMOFF:OPT:u8=$immx8}  ', 'NONE ', 'ld1d_z_p_ai_d    ', 'extensions=SVE '];
T[' LD1H    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={VBASE={REG=$Zn VDS=D} SZ=v64x16 IMMOFF:OPT:u8=$immx2}  ', 'NONE ', 'ld1h_z_p_ai_d    ', 'extensions=SVE '];
T[' LD1SB   VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ VBASE={REG=$Zn VDS=D} SZ=v64x8 IMMOFF:OPT:u8=$imm}     ', 'NONE ', 'ld1sb_z_p_ai_d   ', 'extensions=SVE '];
T[' LD1SH   VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={VBASE={REG=$Zn VDS=D} SZ=v64x16 IMMOFF:OPT:u8=$immx2}  ', 'NONE ', 'ld1sh_z_p_ai_d   ', 'extensions=SVE '];
T[' LD1SW   VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s32={VBASE={REG=$Zn VDS=D} SZ=v64x32 IMMOFF:OPT:u8=$immx4}  ', 'NONE ', 'ld1sw_z_p_ai_d   ', 'extensions=SVE '];
T[' LD1W    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={VBASE={REG=$Zn VDS=D} SZ=v64x32 IMMOFF:OPT:u8=$immx4}  ', 'NONE ', 'ld1w_z_p_ai_d    ', 'extensions=SVE '];
T[' LDFF1B  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ VBASE={REG=$Zn VDS=D} SZ=v64x8 IMMOFF:OPT:u8=$imm}    ', 'NONE ', 'ldff1b_z_p_ai_d  ', 'extensions=SVE '];
T[' LDFF1D  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={VBASE={REG=$Zn VDS=D} SZ=v64x64 IMMOFF:OPT:u8=$immx8} ', 'NONE ', 'ldff1d_z_p_ai_d  ', 'extensions=SVE '];
T[' LDFF1H  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={VBASE={REG=$Zn VDS=D} SZ=v64x16 IMMOFF:OPT:u8=$immx2} ', 'NONE ', 'ldff1h_z_p_ai_d  ', 'extensions=SVE '];
T[' LDFF1SB VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ VBASE={REG=$Zn VDS=D} SZ=v64x8 IMMOFF:OPT:u8=$imm}    ', 'NONE ', 'ldff1sb_z_p_ai_d ', 'extensions=SVE '];
T[' LDFF1SH VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={VBASE={REG=$Zn VDS=D} SZ=v64x16 IMMOFF:OPT:u8=$immx2} ', 'NONE ', 'ldff1sh_z_p_ai_d ', 'extensions=SVE '];
T[' LDFF1SW VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s32={VBASE={REG=$Zn VDS=D} SZ=v64x32 IMMOFF:OPT:u8=$immx4} ', 'NONE ', 'ldff1sw_z_p_ai_d ', 'extensions=SVE '];
T[' LDFF1W  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={VBASE={REG=$Zn VDS=D} SZ=v64x32 IMMOFF:OPT:u8=$immx4} ', 'NONE ', 'ldff1w_z_p_ai_d  ', 'extensions=SVE '];



# sve_mem_64b_gld_vs: SVE 64-bit gather load (scalar plus unpacked 32-bit unscaled offsets).
# A64: ig0=0b1100010 msz:u=0bxx xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U:u=0bx ff:u=0bx Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_64b_gld_vs', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')                  ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                ;
DEF Zm   => SVEREG('Zm', 'Zm', 'read')                  ;
DEF Zt   => SVEREG('Zt', 'Zt')                          ;
DEF mod  => EXTEND('mod', 'xs', table => 'tbl_extend5') ;

T[' LD1B    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v64x8 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}}   ', 'NONE ', 'ld1b_z_p_bz_d_x32_unscaled    ', 'extensions=SVE '];
T[' LD1D    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}}  ', 'NONE ', 'ld1d_z_p_bz_d_x32_unscaled    ', 'extensions=SVE '];
T[' LD1H    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v64x16 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}}  ', 'NONE ', 'ld1h_z_p_bz_d_x32_unscaled    ', 'extensions=SVE '];
T[' LD1SB   VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v64x8 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}}   ', 'NONE ', 'ld1sb_z_p_bz_d_x32_unscaled   ', 'extensions=SVE '];
T[' LD1SH   VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v64x16 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}}  ', 'NONE ', 'ld1sh_z_p_bz_d_x32_unscaled   ', 'extensions=SVE '];
T[' LD1SW   VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s32={BASE=$XnSP SZ=v64x32 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}}  ', 'NONE ', 'ld1sw_z_p_bz_d_x32_unscaled   ', 'extensions=SVE '];
T[' LD1W    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v64x32 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}}  ', 'NONE ', 'ld1w_z_p_bz_d_x32_unscaled    ', 'extensions=SVE '];
T[' LDFF1B  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v64x8 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}}  ', 'NONE ', 'ldff1b_z_p_bz_d_x32_unscaled  ', 'extensions=SVE '];
T[' LDFF1D  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}} ', 'NONE ', 'ldff1d_z_p_bz_d_x32_unscaled  ', 'extensions=SVE '];
T[' LDFF1H  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v64x16 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}} ', 'NONE ', 'ldff1h_z_p_bz_d_x32_unscaled  ', 'extensions=SVE '];
T[' LDFF1SB VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v64x8 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}}  ', 'NONE ', 'ldff1sb_z_p_bz_d_x32_unscaled ', 'extensions=SVE '];
T[' LDFF1SH VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v64x16 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}} ', 'NONE ', 'ldff1sh_z_p_bz_d_x32_unscaled ', 'extensions=SVE '];
T[' LDFF1SW VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s32={BASE=$XnSP SZ=v64x32 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}} ', 'NONE ', 'ldff1sw_z_p_bz_d_x32_unscaled ', 'extensions=SVE '];
T[' LDFF1W  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v64x32 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}} ', 'NONE ', 'ldff1w_z_p_bz_d_x32_unscaled  ', 'extensions=SVE '];



# sve_mem_64b_gld_vs2: SVE 64-bit gather load (scalar plus 64-bit unscaled offsets).
# A64: ig0=0b1100010 msz:u=0bxx ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U:u=0bx ff:u=0bx Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_64b_gld_vs2', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')   ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP') ;
DEF Zm   => SVEREG('Zm', 'Zm', 'read')   ;
DEF Zt   => SVEREG('Zt', 'Zt')           ;

T[' LD1B    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v64x8 VREGOFF={REG=$Zm VDS=D}}   ', 'NONE ', 'ld1b_z_p_bz_d_64_unscaled    ', 'extensions=SVE '];
T[' LD1D    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 VREGOFF={REG=$Zm VDS=D}}  ', 'NONE ', 'ld1d_z_p_bz_d_64_unscaled    ', 'extensions=SVE '];
T[' LD1H    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v64x16 VREGOFF={REG=$Zm VDS=D}}  ', 'NONE ', 'ld1h_z_p_bz_d_64_unscaled    ', 'extensions=SVE '];
T[' LD1SB   VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v64x8 VREGOFF={REG=$Zm VDS=D}}   ', 'NONE ', 'ld1sb_z_p_bz_d_64_unscaled   ', 'extensions=SVE '];
T[' LD1SH   VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v64x16 VREGOFF={REG=$Zm VDS=D}}  ', 'NONE ', 'ld1sh_z_p_bz_d_64_unscaled   ', 'extensions=SVE '];
T[' LD1SW   VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s32={BASE=$XnSP SZ=v64x32 VREGOFF={REG=$Zm VDS=D}}  ', 'NONE ', 'ld1sw_z_p_bz_d_64_unscaled   ', 'extensions=SVE '];
T[' LD1W    VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v64x32 VREGOFF={REG=$Zm VDS=D}}  ', 'NONE ', 'ld1w_z_p_bz_d_64_unscaled    ', 'extensions=SVE '];
T[' LDFF1B  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v64x8 VREGOFF={REG=$Zm VDS=D}}  ', 'NONE ', 'ldff1b_z_p_bz_d_64_unscaled  ', 'extensions=SVE '];
T[' LDFF1D  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 VREGOFF={REG=$Zm VDS=D}} ', 'NONE ', 'ldff1d_z_p_bz_d_64_unscaled  ', 'extensions=SVE '];
T[' LDFF1H  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v64x16 VREGOFF={REG=$Zm VDS=D}} ', 'NONE ', 'ldff1h_z_p_bz_d_64_unscaled  ', 'extensions=SVE '];
T[' LDFF1SB VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v64x8 VREGOFF={REG=$Zm VDS=D}}  ', 'NONE ', 'ldff1sb_z_p_bz_d_64_unscaled ', 'extensions=SVE '];
T[' LDFF1SH VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v64x16 VREGOFF={REG=$Zm VDS=D}} ', 'NONE ', 'ldff1sh_z_p_bz_d_64_unscaled ', 'extensions=SVE '];
T[' LDFF1SW VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s32={BASE=$XnSP SZ=v64x32 VREGOFF={REG=$Zm VDS=D}} ', 'NONE ', 'ldff1sw_z_p_bz_d_64_unscaled ', 'extensions=SVE '];
T[' LDFF1W  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v64x32 VREGOFF={REG=$Zm VDS=D}} ', 'NONE ', 'ldff1w_z_p_bz_d_64_unscaled  ', 'extensions=SVE '];



# sve_mem_64b_gldnt_vs: SVE2 64-bit gather non-temporal load (scalar plus unpacked 32-bit unscaled offsets).
# A64: ig0=0b1100010 msz:u=0bxx ig1=0b00 Rm:u=0bxxxxx ig2=0b1 U:u=0bx ig3=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_64b_gldnt_vs', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')                  ;
DEF Xm => GPR64('Xm', 'Rm', default => 'XZR', 'read') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')                  ;
DEF Zt => SVEREG('Zt', 'Zt')                          ;

T[' LDNT1B  VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ VBASE={REG=$Zn VDS=D} SZ=v64x8 REGOFF=$Xm}  ', 'NONE ', 'ldnt1b_z_p_ar_d_64_unscaled  ', 'extensions=SVE2 '];
T[' LDNT1D  VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u64={VBASE={REG=$Zn VDS=D} SZ=v64x64 REGOFF=$Xm} ', 'NONE ', 'ldnt1d_z_p_ar_d_64_unscaled  ', 'extensions=SVE2 '];
T[' LDNT1H  VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={VBASE={REG=$Zn VDS=D} SZ=v64x16 REGOFF=$Xm} ', 'NONE ', 'ldnt1h_z_p_ar_d_64_unscaled  ', 'extensions=SVE2 '];
T[' LDNT1SB VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ VBASE={REG=$Zn VDS=D} SZ=v64x8 REGOFF=$Xm}  ', 'NONE ', 'ldnt1sb_z_p_ar_d_64_unscaled ', 'extensions=SVE2 '];
T[' LDNT1SH VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={VBASE={REG=$Zn VDS=D} SZ=v64x16 REGOFF=$Xm} ', 'NONE ', 'ldnt1sh_z_p_ar_d_64_unscaled ', 'extensions=SVE2 '];
T[' LDNT1SW VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s32={VBASE={REG=$Zn VDS=D} SZ=v64x32 REGOFF=$Xm} ', 'NONE ', 'ldnt1sw_z_p_ar_d_64_unscaled ', 'extensions=SVE2 '];
T[' LDNT1W  VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={VBASE={REG=$Zn VDS=D} SZ=v64x32 REGOFF=$Xm} ', 'NONE ', 'ldnt1w_z_p_ar_d_64_unscaled  ', 'extensions=SVE2 '];



# sve_mem_64b_prfm_sv: SVE 64-bit gather prefetch (scalar plus unpacked 32-bit scaled offsets).
# A64: ig0=0b110001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz:u=0bxx Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx
HINT('ICLASS', 'sve_mem_64b_prfm_sv', 'A64');

DEF Pg    => PRDREG('Pg', 'Pg', 'read')                      ;
DEF XnSP  => GPR64('Xn', 'Rn', 'read|SP')                    ;
DEF Zm    => SVEREG('Zm', 'Zm', 'read')                      ;
DEF mod   => EXTEND('mod', 'xs', table => 'tbl_extend5')     ;
DEF prfop => PRF_OP('prfop', 'prfop', table => 'tbl_prf_op') ;

T[' PRFB PRF_OP=$prfop REG=$Pg MEM:OFF:r:i8={ BASE=$XnSP SZ=v8x8 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}}               ', 'NONE ', 'prfb_i_p_bz_d_x32_scaled ', 'extensions=SVE '];
T[' PRFD PRF_OP=$prfop REG=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod AMOUNT:u8=3}}} ', 'NONE ', 'prfd_i_p_bz_d_x32_scaled ', 'extensions=SVE '];
T[' PRFH PRF_OP=$prfop REG=$Pg MEM:OFF:r:i16={BASE=$XnSP SZ=v16x16 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod AMOUNT:u8=1}}} ', 'NONE ', 'prfh_i_p_bz_d_x32_scaled ', 'extensions=SVE '];
T[' PRFW PRF_OP=$prfop REG=$Pg MEM:OFF:r:i32={BASE=$XnSP SZ=v32x32 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod AMOUNT:u8=2}}} ', 'NONE ', 'prfw_i_p_bz_d_x32_scaled ', 'extensions=SVE '];



# sve_mem_64b_prfm_sv2: SVE 64-bit gather prefetch (scalar plus 64-bit scaled offsets).
# A64: ig0=0b11000100011 Zm:u=0bxxxxx ig1=0b1 msz:u=0bxx Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx
HINT('ICLASS', 'sve_mem_64b_prfm_sv2', 'A64');

DEF Pg    => PRDREG('Pg', 'Pg', 'read')                      ;
DEF XnSP  => GPR64('Xn', 'Rn', 'read|SP')                    ;
DEF Zm    => SVEREG('Zm', 'Zm', 'read')                      ;
DEF prfop => PRF_OP('prfop', 'prfop', table => 'tbl_prf_op') ;

T[' PRFB PRF_OP=$prfop REG=$Pg MEM:OFF:r:i8={ BASE=$XnSP SZ=v8x8 VREGOFF={REG=$Zm VDS=D}}                                            ', 'NONE ', 'prfb_i_p_bz_d_64_scaled ', 'extensions=SVE '];
T[' PRFD PRF_OP=$prfop REG=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={SHIFT=LSL AMOUNT:u8=3}}} ', 'NONE ', 'prfd_i_p_bz_d_64_scaled ', 'extensions=SVE '];
T[' PRFH PRF_OP=$prfop REG=$Pg MEM:OFF:r:i16={BASE=$XnSP SZ=v16x16 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'prfh_i_p_bz_d_64_scaled ', 'extensions=SVE '];
T[' PRFW PRF_OP=$prfop REG=$Pg MEM:OFF:r:i32={BASE=$XnSP SZ=v32x32 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'prfw_i_p_bz_d_64_scaled ', 'extensions=SVE '];



# sve_mem_64b_prfm_vi: SVE 64-bit gather prefetch (vector plus immediate).
# A64: ig0=0b1100010 msz:u=0bxx ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx
HINT('ICLASS', 'sve_mem_64b_prfm_vi', 'A64');

DEF Pg    => PRDREG('Pg', 'Pg', 'read')                                      ;
DEF Zn    => SVEREG('Zn', 'Zn', 'read')                                      ;
DEF imm   => IMM('imm', 'imm5', range => (0, 31), default => 0)              ;
DEF immx2 => IMM('imm', 'imm5', scale => 2, range => (0, 62), default => 0)  ;
DEF immx4 => IMM('imm', 'imm5', scale => 4, range => (0, 124), default => 0) ;
DEF immx8 => IMM('imm', 'imm5', scale => 8, range => (0, 248), default => 0) ;
DEF prfop => PRF_OP('prfop', 'prfop', table => 'tbl_prf_op')                 ;

T[' PRFB PRF_OP=$prfop REG=$Pg MEM:OFF:r:i8={ VBASE={REG=$Zn VDS=D} SZ=v8x8 IMMOFF:OPT:u8=$imm}     ', 'NONE ', 'prfb_i_p_ai_d ', 'extensions=SVE '];
T[' PRFD PRF_OP=$prfop REG=$Pg MEM:OFF:r:i64={VBASE={REG=$Zn VDS=D} SZ=v64x64 IMMOFF:OPT:u8=$immx8} ', 'NONE ', 'prfd_i_p_ai_d ', 'extensions=SVE '];
T[' PRFH PRF_OP=$prfop REG=$Pg MEM:OFF:r:i16={VBASE={REG=$Zn VDS=D} SZ=v16x16 IMMOFF:OPT:u8=$immx2} ', 'NONE ', 'prfh_i_p_ai_d ', 'extensions=SVE '];
T[' PRFW PRF_OP=$prfop REG=$Pg MEM:OFF:r:i32={VBASE={REG=$Zn VDS=D} SZ=v32x32 IMMOFF:OPT:u8=$immx4} ', 'NONE ', 'prfw_i_p_ai_d ', 'extensions=SVE '];



# sve_mem_cld_si: SVE contiguous load (scalar plus immediate).
# A64: ig0=0b1010010 dtype:u=0bxxxx ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_cld_si', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')                                               ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                                             ;
DEF Zt   => SVEREG('Zt', 'Zt')                                                       ;
DEF imm  => IMM('imm', 'imm4', signpos => 'imm4<3>', range => (-8, 7), default => 0) ;

T[' LD1B  VLIST={VBASE={REG:w:i8=$Zt   VDS=B} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v8x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ld1b_z_p_bi_u8   ', 'extensions=SVE '];
T[' LD1B  VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v16x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ld1b_z_p_bi_u16  ', 'extensions=SVE '];
T[' LD1B  VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v32x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ld1b_z_p_bi_u32  ', 'extensions=SVE '];
T[' LD1B  VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v64x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ld1b_z_p_bi_u64  ', 'extensions=SVE '];
T[' LD1D  VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld1d_z_p_bi_u64  ', 'extensions=SVE '];
T[' LD1H  VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v16x16 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld1h_z_p_bi_u16  ', 'extensions=SVE '];
T[' LD1H  VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v32x16 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld1h_z_p_bi_u32  ', 'extensions=SVE '];
T[' LD1H  VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v64x16 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld1h_z_p_bi_u64  ', 'extensions=SVE '];
T[' LD1SB VLIST={VBASE={REG:w:s16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v16x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ld1sb_z_p_bi_s16 ', 'extensions=SVE '];
T[' LD1SB VLIST={VBASE={REG:w:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v32x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ld1sb_z_p_bi_s32 ', 'extensions=SVE '];
T[' LD1SB VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v64x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ld1sb_z_p_bi_s64 ', 'extensions=SVE '];
T[' LD1SH VLIST={VBASE={REG:w:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v32x16 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld1sh_z_p_bi_s32 ', 'extensions=SVE '];
T[' LD1SH VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v64x16 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld1sh_z_p_bi_s64 ', 'extensions=SVE '];
T[' LD1SW VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s32={BASE=$XnSP SZ=v64x32 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld1sw_z_p_bi_s64 ', 'extensions=SVE '];
T[' LD1W  VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v32x32 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld1w_z_p_bi_u32  ', 'extensions=SVE '];
T[' LD1W  VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v64x32 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld1w_z_p_bi_u64  ', 'extensions=SVE '];



# sve_mem_cld_ss: SVE contiguous load (scalar plus scalar).
# A64: ig0=0b1010010 dtype:u=0bxxxx Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_cld_ss', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')   ;
DEF Xm   => GPR64('Xm', 'Rm', 'read')    ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP') ;
DEF Zt   => SVEREG('Zt', 'Zt')           ;

T[' LD1B  VLIST={VBASE={REG:w:i8=$Zt   VDS=B} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v8x8 REGOFF=$Xm}                                          ', 'NONE ', 'ld1b_z_p_br_u8   ', 'extensions=SVE '];
T[' LD1B  VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v16x8 REGOFF=$Xm}                                          ', 'NONE ', 'ld1b_z_p_br_u16  ', 'extensions=SVE '];
T[' LD1B  VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v32x8 REGOFF=$Xm}                                          ', 'NONE ', 'ld1b_z_p_br_u32  ', 'extensions=SVE '];
T[' LD1B  VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v64x8 REGOFF=$Xm}                                          ', 'NONE ', 'ld1b_z_p_br_u64  ', 'extensions=SVE '];
T[' LD1D  VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=3}}} ', 'NONE ', 'ld1d_z_p_br_u64  ', 'extensions=SVE '];
T[' LD1H  VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v16x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'ld1h_z_p_br_u16  ', 'extensions=SVE '];
T[' LD1H  VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v32x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'ld1h_z_p_br_u32  ', 'extensions=SVE '];
T[' LD1H  VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v64x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'ld1h_z_p_br_u64  ', 'extensions=SVE '];
T[' LD1SB VLIST={VBASE={REG:w:s16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v16x8 REGOFF=$Xm}                                          ', 'NONE ', 'ld1sb_z_p_br_s16 ', 'extensions=SVE '];
T[' LD1SB VLIST={VBASE={REG:w:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v32x8 REGOFF=$Xm}                                          ', 'NONE ', 'ld1sb_z_p_br_s32 ', 'extensions=SVE '];
T[' LD1SB VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v64x8 REGOFF=$Xm}                                          ', 'NONE ', 'ld1sb_z_p_br_s64 ', 'extensions=SVE '];
T[' LD1SH VLIST={VBASE={REG:w:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v32x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'ld1sh_z_p_br_s32 ', 'extensions=SVE '];
T[' LD1SH VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v64x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'ld1sh_z_p_br_s64 ', 'extensions=SVE '];
T[' LD1SW VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s32={BASE=$XnSP SZ=v64x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'ld1sw_z_p_br_s64 ', 'extensions=SVE '];
T[' LD1W  VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v32x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'ld1w_z_p_br_u32  ', 'extensions=SVE '];
T[' LD1W  VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v64x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'ld1w_z_p_br_u64  ', 'extensions=SVE '];



# sve_mem_cldff_ss: SVE contiguous first-fault load (scalar plus scalar).
# A64: ig0=0b1010010 dtype:u=0bxxxx Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_cldff_ss', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')                  ;
DEF Xm   => GPR64('Xm', 'Rm', default => 'XZR', 'read') ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                ;
DEF Zt   => SVEREG('Zt', 'Zt')                          ;

T[' LDFF1B  VLIST={VBASE={REG:rw:i8=$Zt   VDS=B} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v8x8 REGOFF=$Xm}                                          ', 'NONE ', 'ldff1b_z_p_br_u8   ', 'extensions=SVE '];
T[' LDFF1B  VLIST={VBASE={REG:rw:i16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v16x8 REGOFF=$Xm}                                          ', 'NONE ', 'ldff1b_z_p_br_u16  ', 'extensions=SVE '];
T[' LDFF1B  VLIST={VBASE={REG:rw:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v32x8 REGOFF=$Xm}                                          ', 'NONE ', 'ldff1b_z_p_br_u32  ', 'extensions=SVE '];
T[' LDFF1B  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v64x8 REGOFF=$Xm}                                          ', 'NONE ', 'ldff1b_z_p_br_u64  ', 'extensions=SVE '];
T[' LDFF1D  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=3}}} ', 'NONE ', 'ldff1d_z_p_br_u64  ', 'extensions=SVE '];
T[' LDFF1H  VLIST={VBASE={REG:rw:i16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v16x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'ldff1h_z_p_br_u16  ', 'extensions=SVE '];
T[' LDFF1H  VLIST={VBASE={REG:rw:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v32x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'ldff1h_z_p_br_u32  ', 'extensions=SVE '];
T[' LDFF1H  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v64x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'ldff1h_z_p_br_u64  ', 'extensions=SVE '];
T[' LDFF1SB VLIST={VBASE={REG:rw:s16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v16x8 REGOFF=$Xm}                                          ', 'NONE ', 'ldff1sb_z_p_br_s16 ', 'extensions=SVE '];
T[' LDFF1SB VLIST={VBASE={REG:rw:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v32x8 REGOFF=$Xm}                                          ', 'NONE ', 'ldff1sb_z_p_br_s32 ', 'extensions=SVE '];
T[' LDFF1SB VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v64x8 REGOFF=$Xm}                                          ', 'NONE ', 'ldff1sb_z_p_br_s64 ', 'extensions=SVE '];
T[' LDFF1SH VLIST={VBASE={REG:rw:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v32x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'ldff1sh_z_p_br_s32 ', 'extensions=SVE '];
T[' LDFF1SH VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v64x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'ldff1sh_z_p_br_s64 ', 'extensions=SVE '];
T[' LDFF1SW VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s32={BASE=$XnSP SZ=v64x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'ldff1sw_z_p_br_s64 ', 'extensions=SVE '];
T[' LDFF1W  VLIST={VBASE={REG:rw:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v32x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'ldff1w_z_p_br_u32  ', 'extensions=SVE '];
T[' LDFF1W  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v64x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'ldff1w_z_p_br_u64  ', 'extensions=SVE '];



# sve_mem_cldnf_si: SVE contiguous non-fault load (scalar plus immediate).
# A64: ig0=0b1010010 dtype:u=0bxxxx ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_cldnf_si', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')                                               ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                                             ;
DEF Zt   => SVEREG('Zt', 'Zt')                                                       ;
DEF imm  => IMM('imm', 'imm4', signpos => 'imm4<3>', range => (-8, 7), default => 0) ;

T[' LDNF1B  VLIST={VBASE={REG:rw:i8=$Zt   VDS=B} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v8x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ldnf1b_z_p_bi_u8   ', 'extensions=SVE '];
T[' LDNF1B  VLIST={VBASE={REG:rw:i16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v16x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ldnf1b_z_p_bi_u16  ', 'extensions=SVE '];
T[' LDNF1B  VLIST={VBASE={REG:rw:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v32x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ldnf1b_z_p_bi_u32  ', 'extensions=SVE '];
T[' LDNF1B  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=v64x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ldnf1b_z_p_bi_u64  ', 'extensions=SVE '];
T[' LDNF1D  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ldnf1d_z_p_bi_u64  ', 'extensions=SVE '];
T[' LDNF1H  VLIST={VBASE={REG:rw:i16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v16x16 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ldnf1h_z_p_bi_u16  ', 'extensions=SVE '];
T[' LDNF1H  VLIST={VBASE={REG:rw:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v32x16 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ldnf1h_z_p_bi_u32  ', 'extensions=SVE '];
T[' LDNF1H  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=v64x16 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ldnf1h_z_p_bi_u64  ', 'extensions=SVE '];
T[' LDNF1SB VLIST={VBASE={REG:rw:s16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v16x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ldnf1sb_z_p_bi_s16 ', 'extensions=SVE '];
T[' LDNF1SB VLIST={VBASE={REG:rw:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v32x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ldnf1sb_z_p_bi_s32 ', 'extensions=SVE '];
T[' LDNF1SB VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=v64x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ldnf1sb_z_p_bi_s64 ', 'extensions=SVE '];
T[' LDNF1SH VLIST={VBASE={REG:rw:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v32x16 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ldnf1sh_z_p_bi_s32 ', 'extensions=SVE '];
T[' LDNF1SH VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=v64x16 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ldnf1sh_z_p_bi_s64 ', 'extensions=SVE '];
T[' LDNF1SW VLIST={VBASE={REG:rw:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s32={BASE=$XnSP SZ=v64x32 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ldnf1sw_z_p_bi_s64 ', 'extensions=SVE '];
T[' LDNF1W  VLIST={VBASE={REG:rw:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v32x32 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ldnf1w_z_p_bi_u32  ', 'extensions=SVE '];
T[' LDNF1W  VLIST={VBASE={REG:rw:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=v64x32 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ldnf1w_z_p_bi_u64  ', 'extensions=SVE '];



# sve_mem_cldnt_si: SVE contiguous non-temporal load (scalar plus immediate).
# A64: ig0=0b1010010 msz:u=0bxx ig1=0b000 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_cldnt_si', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')                                               ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                                             ;
DEF Zt   => SVEREG('Zt', 'Zt')                                                       ;
DEF imm  => IMM('imm', 'imm4', signpos => 'imm4<3>', range => (-8, 7), default => 0) ;

T[' LDNT1B VLIST={VBASE={REG:w:i8=$Zt   VDS=B} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i8={ BASE=$XnSP SZ=v8x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ldnt1b_z_p_bi_contiguous ', 'extensions=SVE '];
T[' LDNT1D VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ldnt1d_z_p_bi_contiguous ', 'extensions=SVE '];
T[' LDNT1H VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i16={BASE=$XnSP SZ=v16x16 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ldnt1h_z_p_bi_contiguous ', 'extensions=SVE '];
T[' LDNT1W VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i32={BASE=$XnSP SZ=v32x32 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ldnt1w_z_p_bi_contiguous ', 'extensions=SVE '];



# sve_mem_cldnt_ss: SVE contiguous non-temporal load (scalar plus scalar).
# A64: ig0=0b1010010 msz:u=0bxx ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_cldnt_ss', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')   ;
DEF Xm   => GPR64('Xm', 'Rm', 'read')    ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP') ;
DEF Zt   => SVEREG('Zt', 'Zt')           ;

T[' LDNT1B VLIST={VBASE={REG:w:i8=$Zt   VDS=B} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i8={ BASE=$XnSP SZ=v8x8 REGOFF=$Xm}                                          ', 'NONE ', 'ldnt1b_z_p_br_contiguous ', 'extensions=SVE '];
T[' LDNT1D VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=3}}} ', 'NONE ', 'ldnt1d_z_p_br_contiguous ', 'extensions=SVE '];
T[' LDNT1H VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i16={BASE=$XnSP SZ=v16x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'ldnt1h_z_p_br_contiguous ', 'extensions=SVE '];
T[' LDNT1W VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i32={BASE=$XnSP SZ=v32x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'ldnt1w_z_p_br_contiguous ', 'extensions=SVE '];



# sve_mem_cst_si: SVE contiguous store (scalar plus immediate).
# A64: ig0=0b1110010 msz:u=0bxx size:u=0bxx ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_cst_si', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')                                               ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                                             ;
DEF Zt   => SVEREG('Zt', 'Zt')                                                       ;
DEF imm  => IMM('imm', 'imm4', signpos => 'imm4<3>', range => (-8, 7), default => 0) ;

T[' ST1B VLIST={VBASE={REG:r:i8=$Zt   VDS=B} COUNT=1} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'size=0b00 ', 'st1b_z_p_bi_ ', 'extensions=SVE '];
T[' ST1B VLIST={VBASE={REG:r:i16=$Zt VDS=H} COUNT=1} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'size=0b01 ', 'st1b_z_p_bi_ ', 'extensions=SVE '];
T[' ST1B VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'size=0b10 ', 'st1b_z_p_bi_ ', 'extensions=SVE '];
T[' ST1B VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'size=0b11 ', 'st1b_z_p_bi_ ', 'extensions=SVE '];
T[' ST1D VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i64={BASE=$XnSP SZ=1x64 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE      ', 'st1d_z_p_bi_ ', 'extensions=SVE '];
T[' ST1H VLIST={VBASE={REG:r:i16=$Zt VDS=H} COUNT=1} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x16 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'size=0b01 ', 'st1h_z_p_bi_ ', 'extensions=SVE '];
T[' ST1H VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x16 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'size=0b10 ', 'st1h_z_p_bi_ ', 'extensions=SVE '];
T[' ST1H VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x16 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'size=0b11 ', 'st1h_z_p_bi_ ', 'extensions=SVE '];
T[' ST1W VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x32 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'size=0bx0 ', 'st1w_z_p_bi_ ', 'extensions=SVE '];
T[' ST1W VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x32 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'size=0bx1 ', 'st1w_z_p_bi_ ', 'extensions=SVE '];



# sve_mem_cst_ss: SVE contiguous store (scalar plus scalar).
# A64: ig0=0b1110010 opc:u!=0b110 o2:u=0bx Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_cst_ss', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')   ;
DEF Xm   => GPR64('Xm', 'Rm', 'read')    ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP') ;
DEF Zt   => SVEREG('Zt', 'Zt')           ;

T[' ST1B VLIST={VBASE={REG:r:i8=$Zt   VDS=B} COUNT=1} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x8 REGOFF=$Xm}                                         ', 'size=0b00 ', 'st1b_z_p_br_ ', 'extensions=SVE '];
T[' ST1B VLIST={VBASE={REG:r:i16=$Zt VDS=H} COUNT=1} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x8 REGOFF=$Xm}                                          ', 'size=0b01 ', 'st1b_z_p_br_ ', 'extensions=SVE '];
T[' ST1B VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x8 REGOFF=$Xm}                                          ', 'size=0b10 ', 'st1b_z_p_br_ ', 'extensions=SVE '];
T[' ST1B VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x8 REGOFF=$Xm}                                          ', 'size=0b11 ', 'st1b_z_p_br_ ', 'extensions=SVE '];
T[' ST1D VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i64={BASE=$XnSP SZ=1x64 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=3}}} ', 'NONE      ', 'st1d_z_p_br_ ', 'extensions=SVE '];
T[' ST1H VLIST={VBASE={REG:r:i16=$Zt VDS=H} COUNT=1} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'size=0b01 ', 'st1h_z_p_br_ ', 'extensions=SVE '];
T[' ST1H VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'size=0b10 ', 'st1h_z_p_br_ ', 'extensions=SVE '];
T[' ST1H VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'size=0b11 ', 'st1h_z_p_br_ ', 'extensions=SVE '];
T[' ST1W VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'size=0bx0 ', 'st1w_z_p_br_ ', 'extensions=SVE '];
T[' ST1W VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'size=0bx1 ', 'st1w_z_p_br_ ', 'extensions=SVE '];



# sve_mem_cstnt_si: SVE contiguous non-temporal store (scalar plus immediate).
# A64: ig0=0b1110010 msz:u=0bxx ig1=0b001 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_cstnt_si', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')                                               ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                                             ;
DEF Zt   => SVEREG('Zt', 'Zt')                                                       ;
DEF imm  => IMM('imm', 'imm4', signpos => 'imm4<3>', range => (-8, 7), default => 0) ;

T[' STNT1B VLIST={VBASE={REG:r:i8=$Zt   VDS=B} COUNT=1} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'stnt1b_z_p_bi_contiguous ', 'extensions=SVE '];
T[' STNT1D VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i64={BASE=$XnSP SZ=1x64 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'stnt1d_z_p_bi_contiguous ', 'extensions=SVE '];
T[' STNT1H VLIST={VBASE={REG:r:i16=$Zt VDS=H} COUNT=1} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x16 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'stnt1h_z_p_bi_contiguous ', 'extensions=SVE '];
T[' STNT1W VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x32 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'stnt1w_z_p_bi_contiguous ', 'extensions=SVE '];



# sve_mem_cstnt_ss: SVE contiguous non-temporal store (scalar plus scalar).
# A64: ig0=0b1110010 msz:u=0bxx ig1=0b00 Rm:u=0bxxxxx ig2=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_cstnt_ss', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')   ;
DEF Xm   => GPR64('Xm', 'Rm', 'read')    ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP') ;
DEF Zt   => SVEREG('Zt', 'Zt')           ;

T[' STNT1B VLIST={VBASE={REG:r:i8=$Zt   VDS=B} COUNT=1} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x8 REGOFF=$Xm}                                         ', 'NONE ', 'stnt1b_z_p_br_contiguous ', 'extensions=SVE '];
T[' STNT1D VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i64={BASE=$XnSP SZ=1x64 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=3}}} ', 'NONE ', 'stnt1d_z_p_br_contiguous ', 'extensions=SVE '];
T[' STNT1H VLIST={VBASE={REG:r:i16=$Zt VDS=H} COUNT=1} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'stnt1h_z_p_br_contiguous ', 'extensions=SVE '];
T[' STNT1W VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'stnt1w_z_p_br_contiguous ', 'extensions=SVE '];



# sve_mem_eld_si: SVE load multiple structures (scalar plus immediate).
# A64: ig0=0b1010010 msz:u=0bxx opc:u!=0b00 ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_eld_si', 'A64');

DEF Pg    => PRDREG('Pg', 'Pg', 'read')                                                             ;
DEF XnSP  => GPR64('Xn', 'Rn', 'read|SP')                                                           ;
DEF Zt    => SVEREG('Zt1', 'Zt')                                                                    ;
DEF immx2 => IMM('imm', 'imm4', scale => 2, signpos => 'imm4<3>', range => (-16, 14), default => 0) ;
DEF immx3 => IMM('imm', 'imm4', scale => 3, signpos => 'imm4<3>', range => (-24, 21), default => 0) ;
DEF immx4 => IMM('imm', 'imm4', scale => 4, signpos => 'imm4<3>', range => (-32, 28), default => 0) ;

T[' LD2B VLIST={VBASE={REG:w:i8=$Zt   VDS=B} COUNT=2} PREG:Z=$Pg MEM:OFF:r:i8={ BASE=$XnSP SZ=v8x2x8 IMMOFFSH:OPT={IMM:s8=$immx2 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ld2b_z_p_bi_contiguous ', 'extensions=SVE '];
T[' LD2D VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=2} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x2x64 IMMOFFSH:OPT={IMM:s8=$immx2 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld2d_z_p_bi_contiguous ', 'extensions=SVE '];
T[' LD2H VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=2} PREG:Z=$Pg MEM:OFF:r:i16={BASE=$XnSP SZ=v16x2x16 IMMOFFSH:OPT={IMM:s8=$immx2 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld2h_z_p_bi_contiguous ', 'extensions=SVE '];
T[' LD2W VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=2} PREG:Z=$Pg MEM:OFF:r:i32={BASE=$XnSP SZ=v32x2x32 IMMOFFSH:OPT={IMM:s8=$immx2 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld2w_z_p_bi_contiguous ', 'extensions=SVE '];
T[' LD3B VLIST={VBASE={REG:w:i8=$Zt   VDS=B} COUNT=3} PREG:Z=$Pg MEM:OFF:r:i8={ BASE=$XnSP SZ=v8x3x8 IMMOFFSH:OPT={IMM:s8=$immx3 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ld3b_z_p_bi_contiguous ', 'extensions=SVE '];
T[' LD3D VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=3} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x3x64 IMMOFFSH:OPT={IMM:s8=$immx3 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld3d_z_p_bi_contiguous ', 'extensions=SVE '];
T[' LD3H VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=3} PREG:Z=$Pg MEM:OFF:r:i16={BASE=$XnSP SZ=v16x3x16 IMMOFFSH:OPT={IMM:s8=$immx3 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld3h_z_p_bi_contiguous ', 'extensions=SVE '];
T[' LD3W VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=3} PREG:Z=$Pg MEM:OFF:r:i32={BASE=$XnSP SZ=v32x3x32 IMMOFFSH:OPT={IMM:s8=$immx3 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld3w_z_p_bi_contiguous ', 'extensions=SVE '];
T[' LD4B VLIST={VBASE={REG:w:i8=$Zt   VDS=B} COUNT=4} PREG:Z=$Pg MEM:OFF:r:i8={ BASE=$XnSP SZ=v8x4x8 IMMOFFSH:OPT={IMM:s8=$immx4 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}  ', 'NONE ', 'ld4b_z_p_bi_contiguous ', 'extensions=SVE '];
T[' LD4D VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=4} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x4x64 IMMOFFSH:OPT={IMM:s8=$immx4 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld4d_z_p_bi_contiguous ', 'extensions=SVE '];
T[' LD4H VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=4} PREG:Z=$Pg MEM:OFF:r:i16={BASE=$XnSP SZ=v16x4x16 IMMOFFSH:OPT={IMM:s8=$immx4 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld4h_z_p_bi_contiguous ', 'extensions=SVE '];
T[' LD4W VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=4} PREG:Z=$Pg MEM:OFF:r:i32={BASE=$XnSP SZ=v32x4x32 IMMOFFSH:OPT={IMM:s8=$immx4 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'ld4w_z_p_bi_contiguous ', 'extensions=SVE '];



# sve_mem_eld_ss: SVE load multiple structures (scalar plus scalar).
# A64: ig0=0b1010010 msz:u=0bxx opc:u!=0b00 Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_eld_ss', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')   ;
DEF Xm   => GPR64('Xm', 'Rm', 'read')    ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP') ;
DEF Zt   => SVEREG('Zt1', 'Zt')          ;

T[' LD2B VLIST={VBASE={REG:w:i8=$Zt   VDS=B} COUNT=2} PREG:Z=$Pg MEM:OFF:r:i8={ BASE=$XnSP SZ=v8x2x8 REGOFF=$Xm}                                          ', 'NONE ', 'ld2b_z_p_br_contiguous ', 'extensions=SVE '];
T[' LD2D VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=2} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x2x64 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=3}}} ', 'NONE ', 'ld2d_z_p_br_contiguous ', 'extensions=SVE '];
T[' LD2H VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=2} PREG:Z=$Pg MEM:OFF:r:i16={BASE=$XnSP SZ=v16x2x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'ld2h_z_p_br_contiguous ', 'extensions=SVE '];
T[' LD2W VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=2} PREG:Z=$Pg MEM:OFF:r:i32={BASE=$XnSP SZ=v32x2x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'ld2w_z_p_br_contiguous ', 'extensions=SVE '];
T[' LD3B VLIST={VBASE={REG:w:i8=$Zt   VDS=B} COUNT=3} PREG:Z=$Pg MEM:OFF:r:i8={ BASE=$XnSP SZ=v8x3x8 REGOFF=$Xm}                                          ', 'NONE ', 'ld3b_z_p_br_contiguous ', 'extensions=SVE '];
T[' LD3D VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=3} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x3x64 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=3}}} ', 'NONE ', 'ld3d_z_p_br_contiguous ', 'extensions=SVE '];
T[' LD3H VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=3} PREG:Z=$Pg MEM:OFF:r:i16={BASE=$XnSP SZ=v16x3x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'ld3h_z_p_br_contiguous ', 'extensions=SVE '];
T[' LD3W VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=3} PREG:Z=$Pg MEM:OFF:r:i32={BASE=$XnSP SZ=v32x3x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'ld3w_z_p_br_contiguous ', 'extensions=SVE '];
T[' LD4B VLIST={VBASE={REG:w:i8=$Zt   VDS=B} COUNT=4} PREG:Z=$Pg MEM:OFF:r:i8={ BASE=$XnSP SZ=v8x4x8 REGOFF=$Xm}                                          ', 'NONE ', 'ld4b_z_p_br_contiguous ', 'extensions=SVE '];
T[' LD4D VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=4} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x4x64 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=3}}} ', 'NONE ', 'ld4d_z_p_br_contiguous ', 'extensions=SVE '];
T[' LD4H VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=4} PREG:Z=$Pg MEM:OFF:r:i16={BASE=$XnSP SZ=v16x4x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'ld4h_z_p_br_contiguous ', 'extensions=SVE '];
T[' LD4W VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=4} PREG:Z=$Pg MEM:OFF:r:i32={BASE=$XnSP SZ=v32x4x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'ld4w_z_p_br_contiguous ', 'extensions=SVE '];



# sve_mem_est_si: SVE store multiple structures (scalar plus immediate).
# A64: ig0=0b1110010 msz:u=0bxx opc:u!=0b00 ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_est_si', 'A64');

DEF Pg    => PRDREG('Pg', 'Pg', 'read')                                                             ;
DEF XnSP  => GPR64('Xn', 'Rn', 'read|SP')                                                           ;
DEF Zt    => SVEREG('Zt1', 'Zt')                                                                    ;
DEF immx2 => IMM('imm', 'imm4', scale => 2, signpos => 'imm4<3>', range => (-16, 14), default => 0) ;
DEF immx3 => IMM('imm', 'imm4', scale => 3, signpos => 'imm4<3>', range => (-24, 21), default => 0) ;
DEF immx4 => IMM('imm', 'imm4', scale => 4, signpos => 'imm4<3>', range => (-32, 28), default => 0) ;

T[' ST2B VLIST={VBASE={REG:r:i8=$Zt   VDS=B} COUNT=2} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x2x8 IMMOFFSH:OPT={IMM:s8=$immx2 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'st2b_z_p_bi_contiguous ', 'extensions=SVE '];
T[' ST2D VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=2} REG=$Pg MEM:OFF:w:i64={BASE=$XnSP SZ=1x2x64 IMMOFFSH:OPT={IMM:s8=$immx2 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'st2d_z_p_bi_contiguous ', 'extensions=SVE '];
T[' ST2H VLIST={VBASE={REG:r:i16=$Zt VDS=H} COUNT=2} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x2x16 IMMOFFSH:OPT={IMM:s8=$immx2 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'st2h_z_p_bi_contiguous ', 'extensions=SVE '];
T[' ST2W VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=2} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x2x32 IMMOFFSH:OPT={IMM:s8=$immx2 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'st2w_z_p_bi_contiguous ', 'extensions=SVE '];
T[' ST3B VLIST={VBASE={REG:r:i8=$Zt   VDS=B} COUNT=3} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x3x8 IMMOFFSH:OPT={IMM:s8=$immx3 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'st3b_z_p_bi_contiguous ', 'extensions=SVE '];
T[' ST3D VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=3} REG=$Pg MEM:OFF:w:i64={BASE=$XnSP SZ=1x3x64 IMMOFFSH:OPT={IMM:s8=$immx3 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'st3d_z_p_bi_contiguous ', 'extensions=SVE '];
T[' ST3H VLIST={VBASE={REG:r:i16=$Zt VDS=H} COUNT=3} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x3x16 IMMOFFSH:OPT={IMM:s8=$immx3 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'st3h_z_p_bi_contiguous ', 'extensions=SVE '];
T[' ST3W VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=3} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x3x32 IMMOFFSH:OPT={IMM:s8=$immx3 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'st3w_z_p_bi_contiguous ', 'extensions=SVE '];
T[' ST4B VLIST={VBASE={REG:r:i8=$Zt   VDS=B} COUNT=4} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x4x8 IMMOFFSH:OPT={IMM:s8=$immx4 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'st4b_z_p_bi_contiguous ', 'extensions=SVE '];
T[' ST4D VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=4} REG=$Pg MEM:OFF:w:i64={BASE=$XnSP SZ=1x4x64 IMMOFFSH:OPT={IMM:s8=$immx4 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'st4d_z_p_bi_contiguous ', 'extensions=SVE '];
T[' ST4H VLIST={VBASE={REG:r:i16=$Zt VDS=H} COUNT=4} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x4x16 IMMOFFSH:OPT={IMM:s8=$immx4 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'st4h_z_p_bi_contiguous ', 'extensions=SVE '];
T[' ST4W VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=4} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x4x32 IMMOFFSH:OPT={IMM:s8=$immx4 SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'st4w_z_p_bi_contiguous ', 'extensions=SVE '];



# sve_mem_est_ss: SVE store multiple structures (scalar plus scalar).
# A64: ig0=0b1110010 msz:u=0bxx opc:u!=0b00 Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_est_ss', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')   ;
DEF Xm   => GPR64('Xm', 'Rm', 'read')    ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP') ;
DEF Zt   => SVEREG('Zt1', 'Zt')          ;

T[' ST2B VLIST={VBASE={REG:r:i8=$Zt   VDS=B} COUNT=2} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x2x8 REGOFF=$Xm}                                         ', 'NONE ', 'st2b_z_p_br_contiguous ', 'extensions=SVE '];
T[' ST2D VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=2} REG=$Pg MEM:OFF:w:i64={BASE=$XnSP SZ=1x2x64 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=3}}} ', 'NONE ', 'st2d_z_p_br_contiguous ', 'extensions=SVE '];
T[' ST2H VLIST={VBASE={REG:r:i16=$Zt VDS=H} COUNT=2} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x2x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'st2h_z_p_br_contiguous ', 'extensions=SVE '];
T[' ST2W VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=2} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x2x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'st2w_z_p_br_contiguous ', 'extensions=SVE '];
T[' ST3B VLIST={VBASE={REG:r:i8=$Zt   VDS=B} COUNT=3} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x3x8 REGOFF=$Xm}                                         ', 'NONE ', 'st3b_z_p_br_contiguous ', 'extensions=SVE '];
T[' ST3D VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=3} REG=$Pg MEM:OFF:w:i64={BASE=$XnSP SZ=1x3x64 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=3}}} ', 'NONE ', 'st3d_z_p_br_contiguous ', 'extensions=SVE '];
T[' ST3H VLIST={VBASE={REG:r:i16=$Zt VDS=H} COUNT=3} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x3x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'st3h_z_p_br_contiguous ', 'extensions=SVE '];
T[' ST3W VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=3} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x3x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'st3w_z_p_br_contiguous ', 'extensions=SVE '];
T[' ST4B VLIST={VBASE={REG:r:i8=$Zt   VDS=B} COUNT=4} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x4x8 REGOFF=$Xm}                                         ', 'NONE ', 'st4b_z_p_br_contiguous ', 'extensions=SVE '];
T[' ST4D VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=4} REG=$Pg MEM:OFF:w:i64={BASE=$XnSP SZ=1x4x64 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=3}}} ', 'NONE ', 'st4d_z_p_br_contiguous ', 'extensions=SVE '];
T[' ST4H VLIST={VBASE={REG:r:i16=$Zt VDS=H} COUNT=4} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x4x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'st4h_z_p_br_contiguous ', 'extensions=SVE '];
T[' ST4W VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=4} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x4x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'st4w_z_p_br_contiguous ', 'extensions=SVE '];



# sve_mem_ld_dup: SVE load and broadcast element.
# A64: ig0=0b1000010 dtypeh:u=0bxx ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel:u=0bxx Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_ld_dup', 'A64');

DEF Pg    => PRDREG('Pg', 'Pg', 'read')                                      ;
DEF XnSP  => GPR64('Xn', 'Rn', 'read|SP')                                    ;
DEF Zt    => SVEREG('Zt', 'Zt')                                              ;
DEF imm   => IMM('imm', 'imm6', range => (0, 63), default => 0)              ;
DEF immx2 => IMM('imm', 'imm6', scale => 2, range => (0, 126), default => 0) ;
DEF immx4 => IMM('imm', 'imm6', scale => 4, range => (0, 252), default => 0) ;
DEF immx8 => IMM('imm', 'imm6', scale => 8, range => (0, 504), default => 0) ;

T[' LD1RB  VLIST={VBASE={REG:w:i8=$Zt   VDS=B} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=$imm}   ', 'NONE ', 'ld1rb_z_p_bi_u8   ', 'extensions=SVE '];
T[' LD1RB  VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=$imm}    ', 'NONE ', 'ld1rb_z_p_bi_u16  ', 'extensions=SVE '];
T[' LD1RB  VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=$imm}    ', 'NONE ', 'ld1rb_z_p_bi_u32  ', 'extensions=SVE '];
T[' LD1RB  VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=$imm}    ', 'NONE ', 'ld1rb_z_p_bi_u64  ', 'extensions=SVE '];
T[' LD1RD  VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:u16=$immx8} ', 'NONE ', 'ld1rd_z_p_bi_u64  ', 'extensions=SVE '];
T[' LD1RH  VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=$immx2}  ', 'NONE ', 'ld1rh_z_p_bi_u16  ', 'extensions=SVE '];
T[' LD1RH  VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=$immx2}  ', 'NONE ', 'ld1rh_z_p_bi_u32  ', 'extensions=SVE '];
T[' LD1RH  VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=$immx2}  ', 'NONE ', 'ld1rh_z_p_bi_u64  ', 'extensions=SVE '];
T[' LD1RSB VLIST={VBASE={REG:w:s16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=$imm}    ', 'NONE ', 'ld1rsb_z_p_bi_s16 ', 'extensions=SVE '];
T[' LD1RSB VLIST={VBASE={REG:w:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=$imm}    ', 'NONE ', 'ld1rsb_z_p_bi_s32 ', 'extensions=SVE '];
T[' LD1RSB VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=$imm}    ', 'NONE ', 'ld1rsb_z_p_bi_s64 ', 'extensions=SVE '];
T[' LD1RSH VLIST={VBASE={REG:w:s32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=$immx2}  ', 'NONE ', 'ld1rsh_z_p_bi_s32 ', 'extensions=SVE '];
T[' LD1RSH VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=$immx2}  ', 'NONE ', 'ld1rsh_z_p_bi_s64 ', 'extensions=SVE '];
T[' LD1RSW VLIST={VBASE={REG:w:s64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:s32={BASE=$XnSP SZ=32 IMMOFF:OPT:u8=$immx4}  ', 'NONE ', 'ld1rsw_z_p_bi_s64 ', 'extensions=SVE '];
T[' LD1RW  VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=32 IMMOFF:OPT:u8=$immx4}  ', 'NONE ', 'ld1rw_z_p_bi_u32  ', 'extensions=SVE '];
T[' LD1RW  VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:u32={BASE=$XnSP SZ=32 IMMOFF:OPT:u8=$immx4}  ', 'NONE ', 'ld1rw_z_p_bi_u64  ', 'extensions=SVE '];



# sve_mem_ldqr_si: SVE load and broadcast quadword (scalar plus immediate).
# A64: ig0=0b1010010 msz:u=0bxx ssz:u=0bxx ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_ldqr_si', 'A64');

DEF Pg     => PRDREG('Pg', 'Pg', 'read')                                                                ;
DEF XnSP   => GPR64('Xn', 'Rn', 'read|SP')                                                              ;
DEF Zt     => SVEREG('Zt', 'Zt')                                                                        ;
DEF immx16 => IMM('imm', 'imm4', scale => 16, signpos => 'imm4<3>', range => (-128, 112), default => 0) ;
DEF immx32 => IMM('imm', 'imm4', scale => 32, signpos => 'imm4<3>', range => (-256, 224), default => 0) ;

T[' LD1ROB VLIST={VBASE={REG:w:i8=$Zt   VDS=B} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i8={ BASE=$XnSP SZ=32x8 IMMOFF:OPT:s16=$immx32} ', 'NONE ', 'ld1rob_z_p_bi_u8  ', 'extensions=SVEFP64MATMUL '];
T[' LD1ROD VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=4x64 IMMOFF:OPT:s16=$immx32}  ', 'NONE ', 'ld1rod_z_p_bi_u64 ', 'extensions=SVEFP64MATMUL '];
T[' LD1ROH VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i16={BASE=$XnSP SZ=16x16 IMMOFF:OPT:s16=$immx32} ', 'NONE ', 'ld1roh_z_p_bi_u16 ', 'extensions=SVEFP64MATMUL '];
T[' LD1ROW VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i32={BASE=$XnSP SZ=8x32 IMMOFF:OPT:s16=$immx32}  ', 'NONE ', 'ld1row_z_p_bi_u32 ', 'extensions=SVEFP64MATMUL '];
T[' LD1RQB VLIST={VBASE={REG:w:i8=$Zt   VDS=B} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i8={ BASE=$XnSP SZ=16x8 IMMOFF:OPT:s8=$immx16}  ', 'NONE ', 'ld1rqb_z_p_bi_u8  ', 'extensions=SVE           '];
T[' LD1RQD VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=2x64 IMMOFF:OPT:s8=$immx16}   ', 'NONE ', 'ld1rqd_z_p_bi_u64 ', 'extensions=SVE           '];
T[' LD1RQH VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i16={BASE=$XnSP SZ=8x16 IMMOFF:OPT:s8=$immx16}   ', 'NONE ', 'ld1rqh_z_p_bi_u16 ', 'extensions=SVE           '];
T[' LD1RQW VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i32={BASE=$XnSP SZ=4x32 IMMOFF:OPT:s8=$immx16}   ', 'NONE ', 'ld1rqw_z_p_bi_u32 ', 'extensions=SVE           '];



# sve_mem_ldqr_ss: SVE load and broadcast quadword (scalar plus scalar).
# A64: ig0=0b1010010 msz:u=0bxx ssz:u=0bxx Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_ldqr_ss', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')   ;
DEF Xm   => GPR64('Xm', 'Rm', 'read')    ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP') ;
DEF Zt   => SVEREG('Zt', 'Zt')           ;

T[' LD1ROB VLIST={VBASE={REG:w:i8=$Zt   VDS=B} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i8={ BASE=$XnSP SZ=32x8 REGOFF=$Xm}                                         ', 'NONE ', 'ld1rob_z_p_br_contiguous ', 'extensions=SVEFP64MATMUL '];
T[' LD1ROD VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=4x64 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=3}}}  ', 'NONE ', 'ld1rod_z_p_br_contiguous ', 'extensions=SVEFP64MATMUL '];
T[' LD1ROH VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i16={BASE=$XnSP SZ=16x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'ld1roh_z_p_br_contiguous ', 'extensions=SVEFP64MATMUL '];
T[' LD1ROW VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i32={BASE=$XnSP SZ=8x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}}  ', 'NONE ', 'ld1row_z_p_br_contiguous ', 'extensions=SVEFP64MATMUL '];
T[' LD1RQB VLIST={VBASE={REG:w:i8=$Zt   VDS=B} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i8={ BASE=$XnSP SZ=16x8 REGOFF=$Xm}                                         ', 'NONE ', 'ld1rqb_z_p_br_contiguous ', 'extensions=SVE           '];
T[' LD1RQD VLIST={VBASE={REG:w:i64=$Zt VDS=D} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=2x64 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=3}}}  ', 'NONE ', 'ld1rqd_z_p_br_contiguous ', 'extensions=SVE           '];
T[' LD1RQH VLIST={VBASE={REG:w:i16=$Zt VDS=H} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i16={BASE=$XnSP SZ=8x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}}  ', 'NONE ', 'ld1rqh_z_p_br_contiguous ', 'extensions=SVE           '];
T[' LD1RQW VLIST={VBASE={REG:w:i32=$Zt VDS=S} COUNT=1} PREG:Z=$Pg MEM:OFF:r:i32={BASE=$XnSP SZ=4x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}}  ', 'NONE ', 'ld1rqw_z_p_br_contiguous ', 'extensions=SVE           '];



# sve_mem_prfm_si: SVE contiguous prefetch (scalar plus immediate).
# A64: ig0=0b1000010111 imm6:u=0bxxxxxx ig1=0b0 msz:u=0bxx Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx
HINT('ICLASS', 'sve_mem_prfm_si', 'A64');

DEF Pg    => PRDREG('Pg', 'Pg', 'read')                                                 ;
DEF XnSP  => GPR64('Xn', 'Rn', 'read|SP')                                               ;
DEF imm   => IMM('imm', 'imm6', signpos => 'imm6<5>', range => (-32, 31), default => 0) ;
DEF prfop => PRF_OP('prfop', 'prfop', table => 'tbl_prf_op')                            ;

T[' PRFB PRF_OP=$prfop REG=$Pg MEM:OFF:r:i8={ BASE=$XnSP SZ=v8x8 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}}   ', 'NONE ', 'prfb_i_p_bi_s ', 'extensions=SVE '];
T[' PRFD PRF_OP=$prfop REG=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'prfd_i_p_bi_s ', 'extensions=SVE '];
T[' PRFH PRF_OP=$prfop REG=$Pg MEM:OFF:r:i16={BASE=$XnSP SZ=v16x16 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'prfh_i_p_bi_s ', 'extensions=SVE '];
T[' PRFW PRF_OP=$prfop REG=$Pg MEM:OFF:r:i32={BASE=$XnSP SZ=v32x32 IMMOFFSH:OPT={IMM:s8=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'prfw_i_p_bi_s ', 'extensions=SVE '];



# sve_mem_prfm_ss: SVE contiguous prefetch (scalar plus scalar).
# A64: ig0=0b1000010 msz:u=0bxx ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx
HINT('ICLASS', 'sve_mem_prfm_ss', 'A64');

DEF Pg    => PRDREG('Pg', 'Pg', 'read')                      ;
DEF Xm    => GPR64('Xm', 'Rm', 'read')                       ;
DEF XnSP  => GPR64('Xn', 'Rn', 'read|SP')                    ;
DEF prfop => PRF_OP('prfop', 'prfop', table => 'tbl_prf_op') ;

T[' PRFB PRF_OP=$prfop REG=$Pg MEM:OFF:r:i8={ BASE=$XnSP SZ=v8x8 REGOFF=$Xm}                                           ', 'NONE ', 'prfb_i_p_br_s ', 'extensions=SVE '];
T[' PRFD PRF_OP=$prfop REG=$Pg MEM:OFF:r:i64={BASE=$XnSP SZ=v64x64 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=3}}} ', 'NONE ', 'prfd_i_p_br_s ', 'extensions=SVE '];
T[' PRFH PRF_OP=$prfop REG=$Pg MEM:OFF:r:i16={BASE=$XnSP SZ=v16x16 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'prfh_i_p_br_s ', 'extensions=SVE '];
T[' PRFW PRF_OP=$prfop REG=$Pg MEM:OFF:r:i32={BASE=$XnSP SZ=v32x32 REGOFFSH={REG=$Xm SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'prfw_i_p_br_s ', 'extensions=SVE '];



# sve_mem_pspill: SVE store predicate register.
# A64: ig0=0b1110010110 imm9h:u=0bxxxxxx ig1=0b000 imm9l:u=0bxxx Rn:u=0bxxxxx ig2=0b0 Pt:u=0bxxxx
HINT('ICLASS', 'sve_mem_pspill', 'A64');

DEF Pt   => PRDREG('Pt', 'Pt')                                                                   ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                                                         ;
DEF imm  => IMM('imm', 'imm9h:imm9l', signpos => 'imm9h<5>', range => (-256, 255), default => 0) ;

T[' STR REG:r=$Pt MEM:OFF:w:i8={ BASE=$XnSP SZ=pl IMMOFFSH:OPT={IMM:s16=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'str_p_bi_ ', 'extensions=SVE '];



# sve_mem_spill: SVE store vector register.
# A64: ig0=0b1110010110 imm9h:u=0bxxxxxx ig1=0b010 imm9l:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_spill', 'A64');

DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                                                         ;
DEF Zt   => SVEREG('Zt', 'Zt')                                                                   ;
DEF imm  => IMM('imm', 'imm9h:imm9l', signpos => 'imm9h<5>', range => (-256, 255), default => 0) ;

T[' STR REG:r=$Zt MEM:OFF:w:i8={ BASE=$XnSP SZ=vl IMMOFFSH:OPT={IMM:s16=$imm SHIFTER={SHIFT=MUL AMOUNT:u32=VL}}} ', 'NONE ', 'str_z_bi_ ', 'extensions=SVE '];



# sve_mem_sst_sv2: SVE 64-bit scatter store (scalar plus 64-bit scaled offsets).
# A64: ig0=0b1110010 msz:u=0bxx ig1=0b01 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_sst_sv2', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')   ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP') ;
DEF Zm   => SVEREG('Zm', 'Zm', 'read')   ;
DEF Zt   => SVEREG('Zt', 'Zt')           ;

T[' ST1D VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i64={BASE=$XnSP SZ=1x64 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={SHIFT=LSL AMOUNT:u8=3}}} ', 'NONE ', 'st1d_z_p_bz_d_64_scaled ', 'extensions=SVE '];
T[' ST1H VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x16 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'st1h_z_p_bz_d_64_scaled ', 'extensions=SVE '];
T[' ST1W VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x32 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={SHIFT=LSL AMOUNT:u8=2}}} ', 'NONE ', 'st1w_z_p_bz_d_64_scaled ', 'extensions=SVE '];



# sve_mem_sst_sv_a: SVE 64-bit scatter store (scalar plus unpacked 32-bit scaled offsets).
# A64: ig0=0b1110010 msz:u=0bxx ig1=0b01 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_sst_sv_a', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')                  ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                ;
DEF Zm   => SVEREG('Zm', 'Zm', 'read')                  ;
DEF Zt   => SVEREG('Zt', 'Zt')                          ;
DEF mod  => EXTEND('mod', 'xs', table => 'tbl_extend5') ;

T[' ST1D VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i64={BASE=$XnSP SZ=1x64 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod AMOUNT:u8=3}}} ', 'NONE ', 'st1d_z_p_bz_d_x32_scaled ', 'extensions=SVE '];
T[' ST1H VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x16 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod AMOUNT:u8=1}}} ', 'NONE ', 'st1h_z_p_bz_d_x32_scaled ', 'extensions=SVE '];
T[' ST1W VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x32 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod AMOUNT:u8=2}}} ', 'NONE ', 'st1w_z_p_bz_d_x32_scaled ', 'extensions=SVE '];



# sve_mem_sst_sv_b: SVE 32-bit scatter store (scalar plus 32-bit scaled offsets).
# A64: ig0=0b1110010 msz:u=0bxx ig1=0b11 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_sst_sv_b', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')                  ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                ;
DEF Zm   => SVEREG('Zm', 'Zm', 'read')                  ;
DEF Zt   => SVEREG('Zt', 'Zt')                          ;
DEF mod  => EXTEND('mod', 'xs', table => 'tbl_extend5') ;

T[' ST1H VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x16 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod AMOUNT:u8=1}}} ', 'NONE ', 'st1h_z_p_bz_s_x32_scaled ', 'extensions=SVE '];
T[' ST1W VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x32 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod AMOUNT:u8=2}}} ', 'NONE ', 'st1w_z_p_bz_s_x32_scaled ', 'extensions=SVE '];



# sve_mem_sst_vi_a: SVE 64-bit scatter store (vector plus immediate).
# A64: ig0=0b1110010 msz:u=0bxx ig1=0b10 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_sst_vi_a', 'A64');

DEF Pg    => PRDREG('Pg', 'Pg', 'read')                                      ;
DEF Zn    => SVEREG('Zn', 'Zn', 'read')                                      ;
DEF Zt    => SVEREG('Zt', 'Zt')                                              ;
DEF imm   => IMM('imm', 'imm5', range => (0, 31), default => 0)              ;
DEF immx2 => IMM('imm', 'imm5', scale => 2, range => (0, 62), default => 0)  ;
DEF immx4 => IMM('imm', 'imm5', scale => 4, range => (0, 124), default => 0) ;
DEF immx8 => IMM('imm', 'imm5', scale => 8, range => (0, 248), default => 0) ;

T[' ST1B VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i8={ VBASE={REG=$Zn VDS=D} SZ=1x8 IMMOFF:OPT:u8=$imm}    ', 'NONE ', 'st1b_z_p_ai_d ', 'extensions=SVE '];
T[' ST1D VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i64={VBASE={REG=$Zn VDS=D} SZ=1x64 IMMOFF:OPT:u8=$immx8} ', 'NONE ', 'st1d_z_p_ai_d ', 'extensions=SVE '];
T[' ST1H VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i16={VBASE={REG=$Zn VDS=D} SZ=1x16 IMMOFF:OPT:u8=$immx2} ', 'NONE ', 'st1h_z_p_ai_d ', 'extensions=SVE '];
T[' ST1W VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i32={VBASE={REG=$Zn VDS=D} SZ=1x32 IMMOFF:OPT:u8=$immx4} ', 'NONE ', 'st1w_z_p_ai_d ', 'extensions=SVE '];



# sve_mem_sst_vi_b: SVE 32-bit scatter store (vector plus immediate).
# A64: ig0=0b1110010 msz:u=0bxx ig1=0b11 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_sst_vi_b', 'A64');

DEF Pg    => PRDREG('Pg', 'Pg', 'read')                                      ;
DEF Zn    => SVEREG('Zn', 'Zn', 'read')                                      ;
DEF Zt    => SVEREG('Zt', 'Zt')                                              ;
DEF imm   => IMM('imm', 'imm5', range => (0, 31), default => 0)              ;
DEF immx2 => IMM('imm', 'imm5', scale => 2, range => (0, 62), default => 0)  ;
DEF immx4 => IMM('imm', 'imm5', scale => 4, range => (0, 124), default => 0) ;

T[' ST1B VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i8={ VBASE={REG=$Zn VDS=S} SZ=1x8 IMMOFF:OPT:u8=$imm}    ', 'NONE ', 'st1b_z_p_ai_s ', 'extensions=SVE '];
T[' ST1H VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i16={VBASE={REG=$Zn VDS=S} SZ=1x16 IMMOFF:OPT:u8=$immx2} ', 'NONE ', 'st1h_z_p_ai_s ', 'extensions=SVE '];
T[' ST1W VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i32={VBASE={REG=$Zn VDS=S} SZ=1x32 IMMOFF:OPT:u8=$immx4} ', 'NONE ', 'st1w_z_p_ai_s ', 'extensions=SVE '];



# sve_mem_sst_vs2: SVE 64-bit scatter store (scalar plus 64-bit unscaled offsets).
# A64: ig0=0b1110010 msz:u=0bxx ig1=0b00 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_sst_vs2', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')   ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP') ;
DEF Zm   => SVEREG('Zm', 'Zm', 'read')   ;
DEF Zt   => SVEREG('Zt', 'Zt')           ;

T[' ST1B VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x8 VREGOFF={REG=$Zm VDS=D}}  ', 'NONE ', 'st1b_z_p_bz_d_64_unscaled ', 'extensions=SVE '];
T[' ST1D VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i64={BASE=$XnSP SZ=1x64 VREGOFF={REG=$Zm VDS=D}} ', 'NONE ', 'st1d_z_p_bz_d_64_unscaled ', 'extensions=SVE '];
T[' ST1H VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x16 VREGOFF={REG=$Zm VDS=D}} ', 'NONE ', 'st1h_z_p_bz_d_64_unscaled ', 'extensions=SVE '];
T[' ST1W VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x32 VREGOFF={REG=$Zm VDS=D}} ', 'NONE ', 'st1w_z_p_bz_d_64_unscaled ', 'extensions=SVE '];



# sve_mem_sst_vs_a: SVE 64-bit scatter store (scalar plus unpacked 32-bit unscaled offsets).
# A64: ig0=0b1110010 msz:u=0bxx ig1=0b00 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_sst_vs_a', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')                  ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                ;
DEF Zm   => SVEREG('Zm', 'Zm', 'read')                  ;
DEF Zt   => SVEREG('Zt', 'Zt')                          ;
DEF mod  => EXTEND('mod', 'xs', table => 'tbl_extend5') ;

T[' ST1B VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x8 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}}  ', 'NONE ', 'st1b_z_p_bz_d_x32_unscaled ', 'extensions=SVE '];
T[' ST1D VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i64={BASE=$XnSP SZ=1x64 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}} ', 'NONE ', 'st1d_z_p_bz_d_x32_unscaled ', 'extensions=SVE '];
T[' ST1H VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x16 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}} ', 'NONE ', 'st1h_z_p_bz_d_x32_unscaled ', 'extensions=SVE '];
T[' ST1W VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x32 VREGOFFSH={VREG={REG=$Zm VDS=D} SHIFTER={EXTEND=$mod}}} ', 'NONE ', 'st1w_z_p_bz_d_x32_unscaled ', 'extensions=SVE '];



# sve_mem_sst_vs_b: SVE 32-bit scatter store (scalar plus 32-bit unscaled offsets).
# A64: ig0=0b1110010 msz:u=0bxx ig1=0b10 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_sst_vs_b', 'A64');

DEF Pg   => PRDREG('Pg', 'Pg', 'read')                  ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                ;
DEF Zm   => SVEREG('Zm', 'Zm', 'read')                  ;
DEF Zt   => SVEREG('Zt', 'Zt')                          ;
DEF mod  => EXTEND('mod', 'xs', table => 'tbl_extend5') ;

T[' ST1B VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i8={ BASE=$XnSP SZ=1x8 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod}}}  ', 'NONE ', 'st1b_z_p_bz_s_x32_unscaled ', 'extensions=SVE '];
T[' ST1H VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i16={BASE=$XnSP SZ=1x16 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod}}} ', 'NONE ', 'st1h_z_p_bz_s_x32_unscaled ', 'extensions=SVE '];
T[' ST1W VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i32={BASE=$XnSP SZ=1x32 VREGOFFSH={VREG={REG=$Zm VDS=S} SHIFTER={EXTEND=$mod}}} ', 'NONE ', 'st1w_z_p_bz_s_x32_unscaled ', 'extensions=SVE '];



# sve_mem_sstnt_32b_vs: SVE2 32-bit scatter non-temporal store (vector plus scalar).
# A64: ig0=0b1110010 msz:u=0bxx ig1=0b10 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_sstnt_32b_vs', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')                  ;
DEF Xm => GPR64('Xm', 'Rm', default => 'XZR', 'read') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')                  ;
DEF Zt => SVEREG('Zt', 'Zt')                          ;

T[' STNT1B VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i8={ VBASE={REG=$Zn VDS=S} SZ=1x8 REGOFF=$Xm}  ', 'NONE ', 'stnt1b_z_p_ar_s_x32_unscaled ', 'extensions=SVE2 '];
T[' STNT1H VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i16={VBASE={REG=$Zn VDS=S} SZ=1x16 REGOFF=$Xm} ', 'NONE ', 'stnt1h_z_p_ar_s_x32_unscaled ', 'extensions=SVE2 '];
T[' STNT1W VLIST={VBASE={REG:r:i32=$Zt VDS=S} COUNT=1} REG=$Pg MEM:OFF:w:i32={VBASE={REG=$Zn VDS=S} SZ=1x32 REGOFF=$Xm} ', 'NONE ', 'stnt1w_z_p_ar_s_x32_unscaled ', 'extensions=SVE2 '];



# sve_mem_sstnt_64b_vs: SVE2 64-bit scatter non-temporal store (vector plus scalar).
# A64: ig0=0b1110010 msz:u=0bxx ig1=0b00 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx
HINT('ICLASS', 'sve_mem_sstnt_64b_vs', 'A64');

DEF Pg => PRDREG('Pg', 'Pg', 'read')                  ;
DEF Xm => GPR64('Xm', 'Rm', default => 'XZR', 'read') ;
DEF Zn => SVEREG('Zn', 'Zn', 'read')                  ;
DEF Zt => SVEREG('Zt', 'Zt')                          ;

T[' STNT1B VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i8={ VBASE={REG=$Zn VDS=D} SZ=1x8 REGOFF=$Xm}  ', 'NONE ', 'stnt1b_z_p_ar_d_64_unscaled ', 'extensions=SVE2 '];
T[' STNT1D VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i64={VBASE={REG=$Zn VDS=D} SZ=1x64 REGOFF=$Xm} ', 'NONE ', 'stnt1d_z_p_ar_d_64_unscaled ', 'extensions=SVE2 '];
T[' STNT1H VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i16={VBASE={REG=$Zn VDS=D} SZ=1x16 REGOFF=$Xm} ', 'NONE ', 'stnt1h_z_p_ar_d_64_unscaled ', 'extensions=SVE2 '];
T[' STNT1W VLIST={VBASE={REG:r:i64=$Zt VDS=D} COUNT=1} REG=$Pg MEM:OFF:w:i32={VBASE={REG=$Zn VDS=D} SZ=1x32 REGOFF=$Xm} ', 'NONE ', 'stnt1w_z_p_ar_d_64_unscaled ', 'extensions=SVE2 '];



1;

