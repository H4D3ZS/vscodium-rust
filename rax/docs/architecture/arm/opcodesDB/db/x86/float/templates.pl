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
T['F2XM1 REG:SUPP:rw:f80=ST0', 'NONE', 'F2XM1', ''];

# FABS-Absolute Value.
T['FABS REG:SUPP:rw:f80=ST0', 'NONE', 'FABS', ''];

# FBLD-Load Binary Coded Decimal.
T['FBLD REG:w:f80=ST0 MEM:r:bcd80=$mem80', 'NONE', 'FBLD', ''];

# FBSTP-Store BCD Integer and Pop.
T['FBSTP MEM:w:bcd80=$mem80 REG:r:f80=ST0', 'NONE', 'FBSTP', ''];

# FCHS-Change Sign.
T['FCHS REG:SUPP:rw:f80=ST0', 'NONE', 'FCHS', ''];

# FCMOVcc-Floating-Point Conditional Move.
T['FCMOVB   REG:cw:f80=ST0 REG:r:f80=$FPm', 'NONE', 'FCMOVB  ', ''];
T['FCMOVBE  REG:cw:f80=ST0 REG:r:f80=$FPm', 'NONE', 'FCMOVBE ', ''];
T['FCMOVE   REG:cw:f80=ST0 REG:r:f80=$FPm', 'NONE', 'FCMOVE  ', ''];
T['FCMOVNB  REG:cw:f80=ST0 REG:r:f80=$FPm', 'NONE', 'FCMOVNB ', ''];
T['FCMOVNBE REG:cw:f80=ST0 REG:r:f80=$FPm', 'NONE', 'FCMOVNBE', ''];
T['FCMOVNE  REG:cw:f80=ST0 REG:r:f80=$FPm', 'NONE', 'FCMOVNE ', ''];
T['FCMOVNU  REG:cw:f80=ST0 REG:r:f80=$FPm', 'NONE', 'FCMOVNU ', ''];
T['FCMOVU   REG:cw:f80=ST0 REG:r:f80=$FPm', 'NONE', 'FCMOVU  ', ''];

# FCOM/FCOMP/FCOMPP-Compare Floating Point Values.
T['FCOMPP REG:SUPP:r:f80=ST0 REG:SUPP:r:f80=ST1', 'NONE   ', 'FCOMPP      ', ''];
T['FCOMP  REG:r:f80=ST0      REG:r:f80=$FPm    ', 'NONE   ', 'FCOMP_80m80 ', ''];
T['FCOMP  REG:r:f80=ST0      REG:r:f80=$FPm    ', 'MOD=REG', 'FCOMP_80mx_1', ''];
T['FCOMP  REG:r:f80=ST0      MEM:r:f32=$mem32  ', 'MOD=MEM', 'FCOMP_80mx_1', ''];
T['FCOMP  REG:r:f80=ST0      REG:r:f80=$FPm    ', 'MOD=REG', 'FCOMP_80mx_2', ''];
T['FCOMP  REG:r:f80=ST0      MEM:r:f64=$mem64  ', 'MOD=MEM', 'FCOMP_80mx_2', ''];
T['FCOM   REG:r:f80=ST0      REG:r:f80=$FPm    ', 'MOD=REG', 'FCOM_80mx_1 ', ''];
T['FCOM   REG:r:f80=ST0      MEM:r:f32=$mem32  ', 'MOD=MEM', 'FCOM_80mx_1 ', ''];
T['FCOM   REG:r:f80=ST0      REG:r:f80=$FPm    ', 'MOD=REG', 'FCOM_80mx_2 ', ''];
T['FCOM   REG:r:f80=ST0      MEM:r:f64=$mem64  ', 'MOD=MEM', 'FCOM_80mx_2 ', ''];

# FCOS-Cosine.
T['FCOS REG:SUPP:rw:f80=ST0', 'NONE', 'FCOS', ''];

# FDECSTP-Decrement Stack-Top Pointer.
T['FDECSTP', 'NONE', 'FDECSTP', ''];

# FDISI8087_NOP.
T['FDISI8087_NOP', 'NONE', 'FDISI8087_NOP', ''];

# FENI8087_NOP.
T['FENI8087_NOP', 'NONE', 'FENI8087_NOP', ''];

# FFREE-Free Floating-Point Register.
T['FFREE REG:r:f80=$FPm', 'NONE', 'FFREE', ''];

# FFREEP.
T['FFREEP REG:r:f80=$FPm', 'NONE', 'FFREEP', ''];

# FICOM/FICOMP-Compare Integer.
T['FICOMP REG:r:f80=ST0 MEM:r:s32=$mem32', 'NONE', 'FICOMP_80md', ''];
T['FICOMP REG:r:f80=ST0 MEM:r:s16=$mem16', 'NONE', 'FICOMP_80mw', ''];
T['FICOM  REG:r:f80=ST0 MEM:r:s32=$mem32', 'NONE', 'FICOM_80md ', ''];
T['FICOM  REG:r:f80=ST0 MEM:r:s16=$mem16', 'NONE', 'FICOM_80mw ', ''];

# FILD-Load Integer.
T['FILD REG:w:f80=ST0 MEM:r:s32=$mem32', 'NONE', 'FILD_80md', ''];
T['FILD REG:w:f80=ST0 MEM:r:s64=$mem64', 'NONE', 'FILD_80mq', ''];
T['FILD REG:w:f80=ST0 MEM:r:s16=$mem16', 'NONE', 'FILD_80mw', ''];

# FINCSTP-Increment Stack-Top Pointer.
T['FINCSTP', 'NONE', 'FINCSTP', ''];

# FIST/FISTP-Store Integer.
T['FISTP MEM:w:s32=$mem32 REG:r:f80=ST0', 'NONE', 'FISTP_md80', ''];
T['FISTP MEM:w:s64=$mem64 REG:r:f80=ST0', 'NONE', 'FISTP_mq80', ''];
T['FISTP MEM:w:s16=$mem16 REG:r:f80=ST0', 'NONE', 'FISTP_mw80', ''];
T['FIST  MEM:w:s32=$mem32 REG:r:f80=ST0', 'NONE', 'FIST_md80 ', ''];
T['FIST  MEM:w:s16=$mem16 REG:r:f80=ST0', 'NONE', 'FIST_mw80 ', ''];

# FLD-Load Floating Point Value.
T['FLD REG:w:f80=ST0 MEM:r:f80=$mem80', 'NONE   ', 'FLD_80m80', ''];
T['FLD REG:w:f80=ST0 MEM:r:f64=$mem64', 'NONE   ', 'FLD_80mq ', ''];
T['FLD REG:w:f80=ST0 REG:r:f80=$FPm  ', 'MOD=REG', 'FLD_80mx ', ''];
T['FLD REG:w:f80=ST0 MEM:r:f32=$mem32', 'MOD=MEM', 'FLD_80mx ', ''];

# FLDCW-Load x87 FPU Control Word.
T['FLDCW MEM:r=$mem16', 'NONE', 'FLDCW', ''];

# FLDENV-Load x87 FPU Environment.
T['FLDENV MEM:r=$mem112', 'OSZ=16', 'FLDENV', ''];
T['FLDENV MEM:r=$mem224', 'OSZ=Y ', 'FLDENV', ''];

# FLD1/FLDL2T/FLDL2E/FLDPI/FLDLG2/FLDLN2/FLDZ-Load Constant.
T['FLD1   REG:SUPP:w:f80=ST0', 'RM=0', 'FLDx', ''];
T['FLDL2E REG:SUPP:w:f80=ST0', 'RM=2', 'FLDx', ''];
T['FLDL2T REG:SUPP:w:f80=ST0', 'RM=1', 'FLDx', ''];
T['FLDLG2 REG:SUPP:w:f80=ST0', 'RM=4', 'FLDx', ''];
T['FLDLN2 REG:SUPP:w:f80=ST0', 'RM=5', 'FLDx', ''];
T['FLDPI  REG:SUPP:w:f80=ST0', 'RM=3', 'FLDx', ''];
T['FLDZ   REG:SUPP:w:f80=ST0', 'RM=6', 'FLDx', ''];

# FNOP-No Operation.
T['FNOP', 'NONE', 'FNOP', ''];

# FPATAN-Partial Arctangent.
T['FPATAN REG:SUPP:r:f80=ST0 REG:SUPP:rw:f80=ST1', 'NONE', 'FPATAN', ''];

# FPREM-Partial Remainder.
T['FPREM REG:SUPP:rw:f80=ST0 REG:SUPP:r:f80=ST1', 'NONE', 'FPREM', ''];

# FPREM1-Partial Remainder.
T['FPREM1 REG:SUPP:rw:f80=ST0 REG:SUPP:r:f80=ST1', 'NONE', 'FPREM1', ''];

# FPTAN-Partial Tangent.
T['FPTAN REG:SUPP:rw:f80=ST0 REG:SUPP:w:f80=ST1', 'NONE', 'FPTAN', ''];

# FRNDINT-Round to Integer.
T['FRNDINT REG:SUPP:rw:f80=ST0', 'NONE', 'FRNDINT', ''];

# FRSTOR-Restore x87 FPU State.
T['FRSTOR MEM:r=$mem752', 'OSZ=16', 'FRSTOR', ''];
T['FRSTOR MEM:r=$mem864', 'OSZ=Y ', 'FRSTOR', ''];

# FSAVE/FNSAVE-Store x87 FPU State.
T['FNSAVE MEM:w=$mem752', 'OSZ=16', 'FNSAVE', ''];
T['FNSAVE MEM:w=$mem864', 'OSZ=Y ', 'FNSAVE', ''];

# FSCALE-Scale.
T['FSCALE REG:SUPP:rw:f80=ST0 REG:SUPP:r:f80=ST1', 'NONE', 'FSCALE', ''];

# FSETPM287_NOP.
T['FSETPM287_NOP', 'NONE', 'FSETPM287_NOP', ''];

# FSIN-Sine.
T['FSIN REG:SUPP:rw:f80=ST0', 'NONE', 'FSIN', ''];

# FSINCOS-Sine and Cosine.
T['FSINCOS REG:SUPP:rw:f80=ST0 REG:SUPP:w:f80=ST1', 'NONE', 'FSINCOS', ''];

# FSQRT-Square Root.
T['FSQRT REG:SUPP:rw:f80=ST0', 'NONE', 'FSQRT', ''];

# FST/FSTP-Store Floating Point Value.
T['FSTP MEM:w:f80=$mem80 REG:r:f80=ST0', 'NONE   ', 'FSTP_m8080_1', ''];
T['FSTP REG:w:f80=$FPm   REG:r:f80=ST0', 'NONE   ', 'FSTP_m8080_2', ''];
T['FSTP REG:w:f80=$FPm   REG:r:f80=ST0', 'NONE   ', 'FSTP_m8080_3', ''];
T['FSTP MEM:w:f32=$mem32 REG:r:f80=ST0', 'NONE   ', 'FSTP_md80   ', ''];
T['FSTP REG:w:f80=$FPm   REG:r:f80=ST0', 'MOD=REG', 'FSTP_mxxx   ', ''];
T['FSTP MEM:w:f64=$mem64 REG:r:f80=ST0', 'MOD=MEM', 'FSTP_mxxx   ', ''];
T['FST  MEM:w:f32=$mem32 REG:r:f80=ST0', 'NONE   ', 'FST_md80    ', ''];
T['FST  REG:w:f80=$FPm   REG:r:f80=ST0', 'MOD=REG', 'FST_mxxx    ', ''];
T['FST  MEM:w:f64=$mem64 REG:r:f80=ST0', 'MOD=MEM', 'FST_mxxx    ', ''];

# FSTPNCE.
T['FSTPNCE REG:w:f80=$FPm REG:r:f80=ST0', 'NONE', 'FSTPNCE', ''];

# FTST-TEST.
T['FTST REG:SUPP:rw:f80=ST0', 'NONE', 'FTST', ''];

# FUCOM/FUCOMP/FUCOMPP-Unordered Compare Floating Point Values.
T['FUCOM   REG:r:f80=ST0      REG:r:f80=$FPm    ', 'NONE', 'FUCOM  ', ''];
T['FUCOMP  REG:r:f80=ST0      REG:r:f80=$FPm    ', 'NONE', 'FUCOMP ', ''];
T['FUCOMPP REG:SUPP:r:f80=ST0 REG:SUPP:r:f80=ST1', 'NONE', 'FUCOMPP', ''];

# FXAM-Examine Floating-Point.
T['FXAM REG:SUPP:rw:f80=ST0', 'NONE', 'FXAM', ''];

# FXCH-Exchange Register Contents.
T['FXCH REG:rw:f80=ST0 REG:rw:f80=$FPm', 'NONE', 'FXCH_80m80_1', ''];
T['FXCH REG:rw:f80=ST0 REG:rw:f80=$FPm', 'NONE', 'FXCH_80m80_2', ''];
T['FXCH REG:rw:f80=ST0 REG:rw:f80=$FPm', 'NONE', 'FXCH_80m80_3', ''];

# FXTRACT-Extract Exponent and Significand.
T['FXTRACT REG:SUPP:rw:f80=ST0 REG:SUPP:w:f80=ST1', 'NONE', 'FXTRACT', ''];

# FYL2X-Compute y * log2x.
T['FYL2X REG:SUPP:r:f80=ST0 REG:SUPP:rw:f80=ST1', 'NONE', 'FYL2X', ''];

# FYL2XP1-Compute y * log2(x +1).
T['FYL2XP1 REG:SUPP:r:f80=ST0 REG:SUPP:rw:f80=ST1', 'NONE', 'FYL2XP1', ''];

# FADD/FADDP/FIADD-Add.
T['FADDP REG:rw:f80=$FPm REG:r:f80=ST0   ', 'NONE   ', 'FADDP     ', ''];
T['FADD  REG:rw:f80=ST0  REG:r:f80=$FPm  ', 'MOD=REG', 'FADD_80mx ', ''];
T['FADD  REG:rw:f80=ST0  MEM:r:f32=$mem32', 'MOD=MEM', 'FADD_80mx ', ''];
T['FADD  REG:rw:f80=$FPm REG:r:f80=ST0   ', 'MOD=REG', 'FADD_xxxx ', ''];
T['FADD  REG:rw:f80=ST0  MEM:r:f64=$mem64', 'MOD=MEM', 'FADD_xxxx ', ''];
T['FIADD REG:rw:f80=ST0  MEM:r:s32=$mem32', 'NONE   ', 'FIADD_80md', ''];
T['FIADD REG:rw:f80=ST0  MEM:r:s16=$mem16', 'NONE   ', 'FIADD_80mw', ''];

# FCLEX/FNCLEX-Clear Exceptions.
T['FNCLEX', 'NONE', 'FNCLEX', ''];

# FCOMI/FCOMIP/ FUCOMI/FUCOMIP-Compare Floating Point Values and Set EFLAGS.
T['FCOMI   REG:r:f80=ST0 REG:r:f80=$FPm', 'NONE', 'FCOMI  ', ''];
T['FCOMIP  REG:r:f80=ST0 REG:r:f80=$FPm', 'NONE', 'FCOMIP ', ''];
T['FUCOMI  REG:r:f80=ST0 REG:r:f80=$FPm', 'NONE', 'FUCOMI ', ''];
T['FUCOMIP REG:r:f80=ST0 REG:r:f80=$FPm', 'NONE', 'FUCOMIP', ''];

# FDIV/FDIVP/FIDIV-Divide.
T['FDIVP REG:rw:f80=$FPm REG:r:f80=ST0   ', 'NONE   ', 'FDIVP     ', ''];
T['FDIV  REG:rw:f80=ST0  MEM:r:f64=$mem64', 'NONE   ', 'FDIV_80mq ', ''];
T['FDIV  REG:rw:f80=ST0  REG:r:f80=$FPm  ', 'MOD=REG', 'FDIV_80mx ', ''];
T['FDIV  REG:rw:f80=ST0  MEM:r:f32=$mem32', 'MOD=MEM', 'FDIV_80mx ', ''];
T['FDIV  REG:rw:f80=$FPm REG:r:f80=ST0   ', 'NONE   ', 'FDIV_m8080', ''];
T['FIDIV REG:rw:f80=ST0  MEM:r:s32=$mem32', 'NONE   ', 'FIDIV_80md', ''];
T['FIDIV REG:rw:f80=ST0  MEM:r:s16=$mem16', 'NONE   ', 'FIDIV_80mw', ''];

# FDIVR/FDIVRP/FIDIVR-Reverse Divide.
T['FDIVRP REG:rw:f80=$FPm REG:r:f80=ST0   ', 'NONE   ', 'FDIVRP     ', ''];
T['FDIVR  REG:rw:f80=ST0  MEM:r:f64=$mem64', 'NONE   ', 'FDIVR_80mq ', ''];
T['FDIVR  REG:rw:f80=ST0  REG:r:f80=$FPm  ', 'MOD=REG', 'FDIVR_80mx ', ''];
T['FDIVR  REG:rw:f80=ST0  MEM:r:f32=$mem32', 'MOD=MEM', 'FDIVR_80mx ', ''];
T['FDIVR  REG:rw:f80=$FPm REG:r:f80=ST0   ', 'NONE   ', 'FDIVR_m8080', ''];
T['FIDIVR REG:rw:f80=ST0  MEM:r:s32=$mem32', 'NONE   ', 'FIDIVR_80md', ''];
T['FIDIVR REG:rw:f80=ST0  MEM:r:s16=$mem16', 'NONE   ', 'FIDIVR_80mw', ''];

# FINIT/FNINIT-Initialize Floating-Point Unit.
T['FNINIT', 'NONE', 'FNINIT', ''];

# FMUL/FMULP/FIMUL-Multiply.
T['FIMUL REG:rw:f80=ST0  MEM:r:s32=$mem32', 'NONE   ', 'FIMUL_80md', ''];
T['FIMUL REG:rw:f80=ST0  MEM:r:s16=$mem16', 'NONE   ', 'FIMUL_80mw', ''];
T['FMULP REG:rw:f80=$FPm REG:r:f80=ST0   ', 'NONE   ', 'FMULP     ', ''];
T['FMUL  REG:rw:f80=ST0  REG:r:f80=$FPm  ', 'MOD=REG', 'FMUL_80mx ', ''];
T['FMUL  REG:rw:f80=ST0  MEM:r:f32=$mem32', 'MOD=MEM', 'FMUL_80mx ', ''];
T['FMUL  REG:rw:f80=$FPm REG:r:f80=ST0   ', 'MOD=REG', 'FMUL_xxxx ', ''];
T['FMUL  REG:rw:f80=ST0  MEM:r:f64=$mem64', 'MOD=MEM', 'FMUL_xxxx ', ''];

# FSTCW/FNSTCW-Store x87 FPU Control Word.
T['FNSTCW MEM:w=$mem16', 'NONE', 'FNSTCW', ''];

# FSTENV/FNSTENV-Store x87 FPU Environment.
T['FNSTENV MEM:w=$mem112', 'OSZ=16', 'FNSTENV', ''];
T['FNSTENV MEM:w=$mem224', 'OSZ=Y ', 'FNSTENV', ''];

# FSTSW/FNSTSW-Store x87 FPU Status Word.
T['FNSTSW MEM:w=$mem16', 'NONE', 'FNSTSW_mw', ''];
T['FNSTSW REG:w=AX    ', 'NONE', 'FNSTSW_w ', ''];

# FSUB/FSUBP/FISUB-Subtract.
T['FISUB REG:rw:f80=ST0  MEM:r:s32=$mem32', 'NONE   ', 'FISUB_80md', ''];
T['FISUB REG:rw:f80=ST0  MEM:r:s16=$mem16', 'NONE   ', 'FISUB_80mw', ''];
T['FSUBP REG:rw:f80=$FPm REG:r:f80=ST0   ', 'NONE   ', 'FSUBP     ', ''];
T['FSUB  REG:rw:f80=ST0  MEM:r:f64=$mem64', 'NONE   ', 'FSUB_80mq ', ''];
T['FSUB  REG:rw:f80=ST0  REG:r:f80=$FPm  ', 'MOD=REG', 'FSUB_80mx ', ''];
T['FSUB  REG:rw:f80=ST0  MEM:r:f32=$mem32', 'MOD=MEM', 'FSUB_80mx ', ''];
T['FSUB  REG:rw:f80=$FPm REG:r:f80=ST0   ', 'NONE   ', 'FSUB_m8080', ''];

# FSUBR/FSUBRP/FISUBR-Reverse Subtract.
T['FISUBR REG:rw:f80=ST0  MEM:r:s32=$mem32', 'NONE   ', 'FISUBR_80md', ''];
T['FISUBR REG:rw:f80=ST0  MEM:r:s16=$mem16', 'NONE   ', 'FISUBR_80mw', ''];
T['FSUBRP REG:rw:f80=$FPm REG:r:f80=ST0   ', 'NONE   ', 'FSUBRP     ', ''];
T['FSUBR  REG:rw:f80=ST0  MEM:r:f64=$mem64', 'NONE   ', 'FSUBR_80mq ', ''];
T['FSUBR  REG:rw:f80=ST0  REG:r:f80=$FPm  ', 'MOD=REG', 'FSUBR_80mx ', ''];
T['FSUBR  REG:rw:f80=ST0  MEM:r:f32=$mem32', 'MOD=MEM', 'FSUBR_80mx ', ''];
T['FSUBR  REG:rw:f80=$FPm REG:r:f80=ST0   ', 'NONE   ', 'FSUBR_m8080', ''];

# WAIT/FWAIT-Wait.
T['FWAIT', 'NONE', 'FWAIT', ''];
