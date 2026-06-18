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


# F2XM1-Compute 2x-1.
ENCODING F2XM1 => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=6 RM=0',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT',
  metadata   => 'isa=X87',
  tags       => 'page=F2XM1',
};


# FABS-Absolute Value.
ENCODING FABS => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=4 RM=1',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|MATH',
  metadata   => 'isa=X87',
  tags       => 'page=FABS',
};


# FBLD-Load Binary Coded Decimal.
ENCODING FBLD => {
  diagram    => 'MOD=MEM MR=1 OP=0xdf REG=4',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|LOAD',
  metadata   => 'isa=X87',
  tags       => 'page=FBLD',
};


# FBSTP-Store BCD Integer and Pop.
ENCODING FBSTP => {
  diagram    => 'MOD=MEM MR=1 OP=0xdf REG=6',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  tags       => 'page=FBSTP',
};


# FCHS-Change Sign.
ENCODING FCHS => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=4 RM=0',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT',
  metadata   => 'isa=X87',
  tags       => 'page=FCHS',
};


# FCMOVcc-Floating-Point Conditional Move.
ENCODING FCMOVB => {
  diagram    => 'MOD=REG MR=1 OP=0xda REG=0',
  iflags     => 'c0=U c1=W c2=U c3=U cf=R',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER',
  metadata   => 'isa=FCMOV',
  tags       => 'page=FCMOVcc',
};

ENCODING FCMOVBE => {
  diagram    => 'MOD=REG MR=1 OP=0xda REG=2',
  iflags     => 'c0=U c1=W c2=U c3=U cf=R zf=R',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER',
  metadata   => 'isa=FCMOV',
  tags       => 'page=FCMOVcc',
};

ENCODING FCMOVE => {
  diagram    => 'MOD=REG MR=1 OP=0xda REG=1',
  iflags     => 'c0=U c1=W c2=U c3=U zf=R',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER',
  metadata   => 'isa=FCMOV',
  tags       => 'page=FCMOVcc',
};

ENCODING FCMOVNB => {
  diagram    => 'MOD=REG MR=1 OP=0xdb REG=0',
  iflags     => 'c0=U c1=W c2=U c3=U cf=R',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER',
  metadata   => 'isa=FCMOV',
  tags       => 'page=FCMOVcc',
};

ENCODING FCMOVNBE => {
  diagram    => 'MOD=REG MR=1 OP=0xdb REG=2',
  iflags     => 'c0=U c1=W c2=U c3=U cf=R zf=R',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER',
  metadata   => 'isa=FCMOV',
  tags       => 'page=FCMOVcc',
};

ENCODING FCMOVNE => {
  diagram    => 'MOD=REG MR=1 OP=0xdb REG=1',
  iflags     => 'c0=U c1=W c2=U c3=U zf=R',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER',
  metadata   => 'isa=FCMOV',
  tags       => 'page=FCMOVcc',
};

ENCODING FCMOVNU => {
  diagram    => 'MOD=REG MR=1 OP=0xdb REG=3',
  iflags     => 'c0=U c1=W c2=U c3=U pf=R',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER',
  metadata   => 'isa=FCMOV',
  tags       => 'page=FCMOVcc',
};

ENCODING FCMOVU => {
  diagram    => 'MOD=REG MR=1 OP=0xda REG=3',
  iflags     => 'c0=U c1=W c2=U c3=U pf=R',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER',
  metadata   => 'isa=FCMOV',
  tags       => 'page=FCMOVcc',
};


# FCOM/FCOMP/FCOMPP-Compare Floating Point Values.
ENCODING FCOMPP => {
  diagram    => 'MOD=REG MR=1 OP=0xde REG=3 RM=1',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT|COMPARISON',
  metadata   => 'isa=X87',
  tags       => 'page=FCOM',
};

ENCODING FCOMP_80m80 => {
  diagram    => 'MOD=REG MR=1 OP=0xde REG=2',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT|COMPARISON',
  metadata   => 'isa=X87',
  tags       => 'page=FCOM',
};

ENCODING FCOMP_80mx_1 => {
  diagram    => 'MR=1 OP=0xd8 REG=3',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT|COMPARISON',
  metadata   => 'isa=X87',
  tags       => 'page=FCOM',
};

ENCODING FCOMP_80mx_2 => {
  diagram    => 'MR=1 OP=0xdc REG=3',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT|COMPARISON',
  metadata   => 'isa=X87',
  tags       => 'page=FCOM',
};

ENCODING FCOM_80mx_1 => {
  diagram    => 'MR=1 OP=0xd8 REG=2',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT|COMPARISON',
  metadata   => 'isa=X87',
  tags       => 'page=FCOM',
};

ENCODING FCOM_80mx_2 => {
  diagram    => 'MR=1 OP=0xdc REG=2',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT|COMPARISON',
  metadata   => 'isa=X87',
  tags       => 'page=FCOM',
};


# FCOS-Cosine.
ENCODING FCOS => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=7 RM=7',
  iflags     => 'c0=U c1=W c2=W c3=U',
  extensions => 'X87',
  categories => 'FLOAT|MATH',
  metadata   => 'isa=X87',
  tags       => 'page=FCOS',
};


# FDECSTP-Decrement Stack-Top Pointer.
ENCODING FDECSTP => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=6 RM=6',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|STACK_TRANSFER',
  metadata   => 'isa=X87',
  tags       => 'page=FDECSTP',
};


# FDISI8087_NOP.
ENCODING FDISI8087_NOP => {
  diagram    => 'MOD=REG MR=1 OP=0xdb REG=4 RM=1',
  extensions => 'X87',
  categories => 'FLOAT|NOP',
  metadata   => 'isa=X87',
  tags       => 'page=FDISI8087_NOP',
};


# FENI8087_NOP.
ENCODING FENI8087_NOP => {
  diagram    => 'MOD=REG MR=1 OP=0xdb REG=4 RM=0',
  extensions => 'X87',
  categories => 'FLOAT|NOP',
  metadata   => 'isa=X87',
  tags       => 'page=FENI8087_NOP',
};


# FFREE-Free Floating-Point Register.
ENCODING FFREE => {
  diagram    => 'MOD=REG MR=1 OP=0xdd REG=0',
  iflags     => 'c0=U c1=U c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT',
  metadata   => 'isa=X87',
  tags       => 'page=FFREE',
};


# FFREEP.
ENCODING FFREEP => {
  diagram    => 'MOD=REG MR=1 OP=0xdf REG=0',
  iflags     => 'c0=U c1=U c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT',
  metadata   => 'isa=X87',
  tags       => 'page=FFREEP',
};


# FICOM/FICOMP-Compare Integer.
ENCODING FICOMP_80md => {
  diagram    => 'MOD=MEM MR=1 OP=0xda REG=3',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT|COMPARISON',
  metadata   => 'isa=X87',
  tags       => 'page=FICOM',
};

ENCODING FICOMP_80mw => {
  diagram    => 'MOD=MEM MR=1 OP=0xde REG=3',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT|COMPARISON',
  metadata   => 'isa=X87',
  tags       => 'page=FICOM',
};

ENCODING FICOM_80md => {
  diagram    => 'MOD=MEM MR=1 OP=0xda REG=2',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT|COMPARISON',
  metadata   => 'isa=X87',
  tags       => 'page=FICOM',
};

ENCODING FICOM_80mw => {
  diagram    => 'MOD=MEM MR=1 OP=0xde REG=2',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT|COMPARISON',
  metadata   => 'isa=X87',
  tags       => 'page=FICOM',
};


# FILD-Load Integer.
ENCODING FILD_80md => {
  diagram    => 'MOD=MEM MR=1 OP=0xdb REG=0',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|LOAD',
  metadata   => 'isa=X87',
  tags       => 'page=FILD',
};

ENCODING FILD_80mq => {
  diagram    => 'MOD=MEM MR=1 OP=0xdf REG=5',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|LOAD',
  metadata   => 'isa=X87',
  tags       => 'page=FILD',
};

ENCODING FILD_80mw => {
  diagram    => 'MOD=MEM MR=1 OP=0xdf REG=0',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|LOAD',
  metadata   => 'isa=X87',
  tags       => 'page=FILD',
};


# FINCSTP-Increment Stack-Top Pointer.
ENCODING FINCSTP => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=6 RM=7',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|STACK_TRANSFER',
  metadata   => 'isa=X87',
  tags       => 'page=FINCSTP',
};


# FIST/FISTP-Store Integer.
ENCODING FISTP_md80 => {
  diagram    => 'MOD=MEM MR=1 OP=0xdb REG=3',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  tags       => 'page=FIST',
};

ENCODING FISTP_mq80 => {
  diagram    => 'MOD=MEM MR=1 OP=0xdf REG=7',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  tags       => 'page=FIST',
};

ENCODING FISTP_mw80 => {
  diagram    => 'MOD=MEM MR=1 OP=0xdf REG=3',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  tags       => 'page=FIST',
};

ENCODING FIST_md80 => {
  diagram    => 'MOD=MEM MR=1 OP=0xdb REG=2',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  tags       => 'page=FIST',
};

ENCODING FIST_mw80 => {
  diagram    => 'MOD=MEM MR=1 OP=0xdf REG=2',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  tags       => 'page=FIST',
};


# FLD-Load Floating Point Value.
ENCODING FLD_80m80 => {
  diagram    => 'MOD=MEM MR=1 OP=0xdb REG=5',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|LOAD',
  metadata   => 'isa=X87',
  tags       => 'page=FLD',
};

ENCODING FLD_80mq => {
  diagram    => 'MOD=MEM MR=1 OP=0xdd REG=0',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|LOAD',
  metadata   => 'isa=X87',
  tags       => 'page=FLD',
};

ENCODING FLD_80mx => {
  diagram    => 'MR=1 OP=0xd9 REG=0',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|LOAD',
  metadata   => 'isa=X87',
  tags       => 'page=FLD',
};


# FLDCW-Load x87 FPU Control Word.
ENCODING FLDCW => {
  diagram    => 'MOD=MEM MR=1 OP=0xd9 REG=5',
  iflags     => 'c0=U c1=U c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|LOAD',
  metadata   => 'isa=X87',
  tags       => 'page=FLDCW',
};


# FLDENV-Load x87 FPU Environment.
ENCODING FLDENV => {
  diagram    => 'MOD=MEM MR=1 OP=0xd9 REG=4',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|LOAD',
  metadata   => 'isa=X87',
  tags       => 'page=FLDENV',
};


# FLD1/FLDL2T/FLDL2E/FLDPI/FLDLG2/FLDLN2/FLDZ-Load Constant.
ENCODING FLDx => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=5',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|LOAD',
  metadata   => 'isa=X87',
  tags       => 'page=FLDx',
};


# FNOP-No Operation.
ENCODING FNOP => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=2 RM=0',
  extensions => 'X87',
  categories => 'FLOAT|NOP',
  metadata   => 'isa=X87',
  tags       => 'page=FNOP',
};


# FPATAN-Partial Arctangent.
ENCODING FPATAN => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=6 RM=3',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|MATH',
  metadata   => 'isa=X87',
  tags       => 'page=FPATAN',
};


# FPREM-Partial Remainder.
ENCODING FPREM => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=7 RM=0',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT',
  metadata   => 'isa=X87',
  tags       => 'page=FPREM',
};


# FPREM1-Partial Remainder.
ENCODING FPREM1 => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=6 RM=5',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT',
  metadata   => 'isa=X87',
  tags       => 'page=FPREM1',
};


# FPTAN-Partial Tangent.
ENCODING FPTAN => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=6 RM=2',
  iflags     => 'c0=U c1=W c2=W c3=U',
  extensions => 'X87',
  categories => 'FLOAT|MATH',
  metadata   => 'isa=X87',
  tags       => 'page=FPTAN',
};


# FRNDINT-Round to Integer.
ENCODING FRNDINT => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=7 RM=4',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT',
  metadata   => 'isa=X87',
  tags       => 'page=FRNDINT',
};


# FRSTOR-Restore x87 FPU State.
ENCODING FRSTOR => {
  diagram    => 'MOD=MEM MR=1 OP=0xdd REG=4',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  docvars    => 'X87_MMX_STATE_W=1',
  tags       => 'page=FRSTOR',
};


# FSAVE/FNSAVE-Store x87 FPU State.
ENCODING FNSAVE => {
  diagram    => 'MOD=MEM MR=1 OP=0xdd REG=6',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  docvars    => 'X87_MMX_STATE_R=1',
  tags       => 'page=FSAVE',
};


# FSCALE-Scale.
ENCODING FSCALE => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=7 RM=5',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT',
  metadata   => 'isa=X87',
  tags       => 'page=FSCALE',
};


# FSETPM287_NOP.
ENCODING FSETPM287_NOP => {
  diagram    => 'MOD=REG MR=1 OP=0xdb REG=4 RM=4',
  extensions => 'X87',
  categories => 'FLOAT|NOP',
  metadata   => 'isa=X87',
  tags       => 'page=FSETPM287_NOP',
};


# FSIN-Sine.
ENCODING FSIN => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=7 RM=6',
  iflags     => 'c0=U c1=W c2=W c3=U',
  extensions => 'X87',
  categories => 'FLOAT|MATH',
  metadata   => 'isa=X87',
  tags       => 'page=FSIN',
};


# FSINCOS-Sine and Cosine.
ENCODING FSINCOS => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=7 RM=3',
  iflags     => 'c0=U c1=W c2=W c3=U',
  extensions => 'X87',
  categories => 'FLOAT|MATH',
  metadata   => 'isa=X87',
  tags       => 'page=FSINCOS',
};


# FSQRT-Square Root.
ENCODING FSQRT => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=7 RM=2',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|MATH',
  metadata   => 'isa=X87',
  tags       => 'page=FSQRT',
};


# FST/FSTP-Store Floating Point Value.
ENCODING FSTP_m8080_1 => {
  diagram    => 'MOD=MEM MR=1 OP=0xdb REG=7',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  tags       => 'page=FST',
};

ENCODING FSTP_m8080_2 => {
  diagram    => 'MOD=REG MR=1 OP=0xdf REG=2',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  tags       => 'page=FST',
};

ENCODING FSTP_m8080_3 => {
  diagram    => 'MOD=REG MR=1 OP=0xdf REG=3',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  tags       => 'page=FST',
};

ENCODING FSTP_md80 => {
  diagram    => 'MOD=MEM MR=1 OP=0xd9 REG=3',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  tags       => 'page=FST',
};

ENCODING FSTP_mxxx => {
  diagram    => 'MR=1 OP=0xdd REG=3',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  tags       => 'page=FST',
};

ENCODING FST_md80 => {
  diagram    => 'MOD=MEM MR=1 OP=0xd9 REG=2',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  tags       => 'page=FST',
};

ENCODING FST_mxxx => {
  diagram    => 'MR=1 OP=0xdd REG=2',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  tags       => 'page=FST',
};


# FSTPNCE.
ENCODING FSTPNCE => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=3',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT',
  metadata   => 'isa=X87',
  tags       => 'page=FSTPNCE',
};


# FTST-TEST.
ENCODING FTST => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=4 RM=4',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT',
  metadata   => 'isa=X87',
  tags       => 'page=FTST',
};


# FUCOM/FUCOMP/FUCOMPP-Unordered Compare Floating Point Values.
ENCODING FUCOM => {
  diagram    => 'MOD=REG MR=1 OP=0xdd REG=4',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT|COMPARISON',
  metadata   => 'isa=X87',
  tags       => 'page=FUCOM',
};

ENCODING FUCOMP => {
  diagram    => 'MOD=REG MR=1 OP=0xdd REG=5',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT|COMPARISON',
  metadata   => 'isa=X87',
  tags       => 'page=FUCOM',
};

ENCODING FUCOMPP => {
  diagram    => 'MOD=REG MR=1 OP=0xda REG=5 RM=1',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT|COMPARISON',
  metadata   => 'isa=X87',
  tags       => 'page=FUCOM',
};


# FXAM-Examine Floating-Point.
ENCODING FXAM => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=4 RM=5',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT',
  metadata   => 'isa=X87',
  tags       => 'page=FXAM',
};


# FXCH-Exchange Register Contents.
ENCODING FXCH_80m80_1 => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=1',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER',
  metadata   => 'isa=X87',
  tags       => 'page=FXCH',
};

ENCODING FXCH_80m80_2 => {
  diagram    => 'MOD=REG MR=1 OP=0xdd REG=1',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER',
  metadata   => 'isa=X87',
  tags       => 'page=FXCH',
};

ENCODING FXCH_80m80_3 => {
  diagram    => 'MOD=REG MR=1 OP=0xdf REG=1',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER',
  metadata   => 'isa=X87',
  tags       => 'page=FXCH',
};


# FXTRACT-Extract Exponent and Significand.
ENCODING FXTRACT => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=6 RM=4',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT',
  metadata   => 'isa=X87',
  tags       => 'page=FXTRACT',
};


# FYL2X-Compute y * log2x.
ENCODING FYL2X => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=6 RM=1',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|MATH',
  metadata   => 'isa=X87',
  tags       => 'page=FYL2X',
};


# FYL2XP1-Compute y * log2(x +1).
ENCODING FYL2XP1 => {
  diagram    => 'MOD=REG MR=1 OP=0xd9 REG=7 RM=1',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|MATH',
  metadata   => 'isa=X87',
  tags       => 'page=FYL2XP1',
};


# FADD/FADDP/FIADD-Add.
ENCODING FADDP => {
  diagram    => 'MOD=REG MR=1 OP=0xde REG=0',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxADD',
};

ENCODING FADD_80mx => {
  diagram    => 'MR=1 OP=0xd8 REG=0',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxADD',
};

ENCODING FADD_xxxx => {
  diagram    => 'MR=1 OP=0xdc REG=0',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxADD',
};

ENCODING FIADD_80md => {
  diagram    => 'MOD=MEM MR=1 OP=0xda REG=0',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxADD',
};

ENCODING FIADD_80mw => {
  diagram    => 'MOD=MEM MR=1 OP=0xde REG=0',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxADD',
};


# FCLEX/FNCLEX-Clear Exceptions.
ENCODING FNCLEX => {
  diagram    => 'MOD=REG MR=1 OP=0xdb REG=4 RM=2',
  iflags     => 'c0=U c1=U c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|EXCEPTION',
  metadata   => 'isa=X87',
  tags       => 'page=FxCLEX',
};


# FCOMI/FCOMIP/ FUCOMI/FUCOMIP-Compare Floating Point Values and Set EFLAGS.
ENCODING FCOMI => {
  diagram    => 'MOD=REG MR=1 OP=0xdb REG=6',
  iflags     => 'af=Z c1=W cf=W of=Z pf=W sf=Z zf=W',
  extensions => 'X87',
  categories => 'FLOAT|COMPARISON',
  metadata   => 'isa=PPRO',
  tags       => 'page=FxCOMI',
};

ENCODING FCOMIP => {
  diagram    => 'MOD=REG MR=1 OP=0xdf REG=6',
  iflags     => 'af=Z c1=W cf=W of=Z pf=W sf=Z zf=W',
  extensions => 'X87',
  categories => 'FLOAT|COMPARISON',
  metadata   => 'isa=PPRO',
  tags       => 'page=FxCOMI',
};

ENCODING FUCOMI => {
  diagram    => 'MOD=REG MR=1 OP=0xdb REG=5',
  iflags     => 'af=Z c1=W cf=W of=Z pf=W sf=Z zf=W',
  extensions => 'X87',
  categories => 'FLOAT|COMPARISON',
  metadata   => 'isa=PPRO',
  tags       => 'page=FxCOMI',
};

ENCODING FUCOMIP => {
  diagram    => 'MOD=REG MR=1 OP=0xdf REG=5',
  iflags     => 'af=Z c1=W cf=W of=Z pf=W sf=Z zf=W',
  extensions => 'X87',
  categories => 'FLOAT|COMPARISON',
  metadata   => 'isa=PPRO',
  tags       => 'page=FxCOMI',
};


# FDIV/FDIVP/FIDIV-Divide.
ENCODING FDIVP => {
  diagram    => 'MOD=REG MR=1 OP=0xde REG=7',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxDIV',
};

ENCODING FDIV_80mq => {
  diagram    => 'MOD=MEM MR=1 OP=0xdc REG=6',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxDIV',
};

ENCODING FDIV_80mx => {
  diagram    => 'MR=1 OP=0xd8 REG=6',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxDIV',
};

ENCODING FDIV_m8080 => {
  diagram    => 'MOD=REG MR=1 OP=0xdc REG=7',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxDIV',
};

ENCODING FIDIV_80md => {
  diagram    => 'MOD=MEM MR=1 OP=0xda REG=6',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxDIV',
};

ENCODING FIDIV_80mw => {
  diagram    => 'MOD=MEM MR=1 OP=0xde REG=6',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxDIV',
};


# FDIVR/FDIVRP/FIDIVR-Reverse Divide.
ENCODING FDIVRP => {
  diagram    => 'MOD=REG MR=1 OP=0xde REG=6',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxDIVR',
};

ENCODING FDIVR_80mq => {
  diagram    => 'MOD=MEM MR=1 OP=0xdc REG=7',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxDIVR',
};

ENCODING FDIVR_80mx => {
  diagram    => 'MR=1 OP=0xd8 REG=7',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxDIVR',
};

ENCODING FDIVR_m8080 => {
  diagram    => 'MOD=REG MR=1 OP=0xdc REG=6',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxDIVR',
};

ENCODING FIDIVR_80md => {
  diagram    => 'MOD=MEM MR=1 OP=0xda REG=7',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxDIVR',
};

ENCODING FIDIVR_80mw => {
  diagram    => 'MOD=MEM MR=1 OP=0xde REG=7',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxDIVR',
};


# FINIT/FNINIT-Initialize Floating-Point Unit.
ENCODING FNINIT => {
  diagram    => 'MOD=REG MR=1 OP=0xdb REG=4 RM=3',
  iflags     => 'c0=W c1=W c2=W c3=W',
  extensions => 'X87',
  categories => 'FLOAT',
  metadata   => 'isa=X87',
  docvars    => 'X87_MMX_STATE_W=1',
  tags       => 'page=FxINIT',
};


# FMUL/FMULP/FIMUL-Multiply.
ENCODING FIMUL_80md => {
  diagram    => 'MOD=MEM MR=1 OP=0xda REG=1',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxMUL',
};

ENCODING FIMUL_80mw => {
  diagram    => 'MOD=MEM MR=1 OP=0xde REG=1',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxMUL',
};

ENCODING FMULP => {
  diagram    => 'MOD=REG MR=1 OP=0xde REG=1',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxMUL',
};

ENCODING FMUL_80mx => {
  diagram    => 'MR=1 OP=0xd8 REG=1',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxMUL',
};

ENCODING FMUL_xxxx => {
  diagram    => 'MR=1 OP=0xdc REG=1',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxMUL',
};


# FSTCW/FNSTCW-Store x87 FPU Control Word.
ENCODING FNSTCW => {
  diagram    => 'MOD=MEM MR=1 OP=0xd9 REG=7',
  iflags     => 'c0=U c1=U c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  tags       => 'page=FxSTCW',
};


# FSTENV/FNSTENV-Store x87 FPU Environment.
ENCODING FNSTENV => {
  diagram    => 'MOD=MEM MR=1 OP=0xd9 REG=6',
  iflags     => 'c0=U c1=U c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  tags       => 'page=FxSTENV',
};


# FSTSW/FNSTSW-Store x87 FPU Status Word.
ENCODING FNSTSW_mw => {
  diagram    => 'MOD=MEM MR=1 OP=0xdd REG=7',
  iflags     => 'c0=U c1=U c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  tags       => 'page=FxSTSW',
};

ENCODING FNSTSW_w => {
  diagram    => 'MOD=REG MR=1 OP=0xdf REG=4 RM=0',
  iflags     => 'c0=U c1=U c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|DATA_TRANSFER|STORE',
  metadata   => 'isa=X87',
  tags       => 'page=FxSTSW',
};


# FSUB/FSUBP/FISUB-Subtract.
ENCODING FISUB_80md => {
  diagram    => 'MOD=MEM MR=1 OP=0xda REG=4',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxSUB',
};

ENCODING FISUB_80mw => {
  diagram    => 'MOD=MEM MR=1 OP=0xde REG=4',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxSUB',
};

ENCODING FSUBP => {
  diagram    => 'MOD=REG MR=1 OP=0xde REG=5',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxSUB',
};

ENCODING FSUB_80mq => {
  diagram    => 'MOD=MEM MR=1 OP=0xdc REG=4',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxSUB',
};

ENCODING FSUB_80mx => {
  diagram    => 'MR=1 OP=0xd8 REG=4',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxSUB',
};

ENCODING FSUB_m8080 => {
  diagram    => 'MOD=REG MR=1 OP=0xdc REG=5',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxSUB',
};


# FSUBR/FSUBRP/FISUBR-Reverse Subtract.
ENCODING FISUBR_80md => {
  diagram    => 'MOD=MEM MR=1 OP=0xda REG=5',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxSUBR',
};

ENCODING FISUBR_80mw => {
  diagram    => 'MOD=MEM MR=1 OP=0xde REG=5',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxSUBR',
};

ENCODING FSUBRP => {
  diagram    => 'MOD=REG MR=1 OP=0xde REG=4',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxSUBR',
};

ENCODING FSUBR_80mq => {
  diagram    => 'MOD=MEM MR=1 OP=0xdc REG=5',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxSUBR',
};

ENCODING FSUBR_80mx => {
  diagram    => 'MR=1 OP=0xd8 REG=5',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxSUBR',
};

ENCODING FSUBR_m8080 => {
  diagram    => 'MOD=REG MR=1 OP=0xdc REG=4',
  iflags     => 'c0=U c1=W c2=U c3=U',
  extensions => 'X87',
  categories => 'FLOAT|ARITHMETIC',
  metadata   => 'isa=X87',
  tags       => 'page=FxSUBR',
};


# WAIT/FWAIT-Wait.
ENCODING FWAIT => {
  diagram    => 'OP=0x9b',
  extensions => 'X87',
  categories => 'FLOAT',
  metadata   => 'isa=X87',
  tags       => 'page=WAIT',
};

