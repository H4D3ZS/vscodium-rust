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
T['VADDPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VADDPD_ronomo', '        '];
T['VADDPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VADDPD_ronomo', 'tuple=FV'];
T['VADDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VADDPD_rxnxmx', '        '];
T['VADDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VADDPD_rxnxmx', '        '];
T['VADDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VADDPD_rxnxmx', '        '];
T['VADDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VADDPD_rxnxmx', '        '];
T['VADDPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VADDPD_rynymy', '        '];
T['VADDPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VADDPD_rynymy', 'tuple=FV'];
T['VADDPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VADDPD_rznzmz', '        '];
T['VADDPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VADDPD_rznzmz', '        '];
T['VADDPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VADDPD_rznzmz', 'tuple=FV'];

# ADDPS-Add Packed Single-Precision Floating-Point Values.
T['VADDPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VADDPS_ronomo', '        '];
T['VADDPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VADDPS_ronomo', 'tuple=FV'];
T['VADDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VADDPS_rxnxmx', '        '];
T['VADDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VADDPS_rxnxmx', '        '];
T['VADDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VADDPS_rxnxmx', '        '];
T['VADDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VADDPS_rxnxmx', '        '];
T['VADDPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VADDPS_rynymy', '        '];
T['VADDPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VADDPS_rynymy', 'tuple=FV'];
T['VADDPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VADDPS_rznzmz', '        '];
T['VADDPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VADDPS_rznzmz', '        '];
T['VADDPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VADDPS_rznzmz', 'tuple=FV'];

# ADDSD-Add Scalar Double-Precision Floating-Point Values.
T['VADDSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VADDSD_ronomq', '         '];
T['VADDSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VADDSD_ronomq', '         '];
T['VADDSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VADDSD_ronomx', '         '];
T['VADDSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VADDSD_ronomx', '         '];
T['VADDSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VADDSD_ronomx', 'tuple=T1S'];

# ADDSS-Add Scalar Single-Precision Floating-Point Values.
T['VADDSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn REG:r:f32=$XMMm                         ', 'MOD=REG             ', 'VADDSS_ronomd', '         '];
T['VADDSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem32                        ', 'MOD=MEM             ', 'VADDSS_ronomd', '         '];
T['VADDSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VADDSS_ronomx', '         '];
T['VADDSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VADDSS_ronomx', '         '];
T['VADDSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VADDSS_ronomx', 'tuple=T1S'];

# ADDSUBPD-Packed Double-FP Add/Subtract.
T['VADDSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm ', 'MOD=REG VL=128', 'VADDSUBPD', ''];
T['VADDSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm ', 'MOD=REG VL=256', 'VADDSUBPD', ''];
T['VADDSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128', 'MOD=MEM VL=128', 'VADDSUBPD', ''];
T['VADDSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256', 'MOD=MEM VL=256', 'VADDSUBPD', ''];

# ADDSUBPS-Packed Single-FP Add/Subtract.
T['VADDSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm ', 'MOD=REG VL=128', 'VADDSUBPS', ''];
T['VADDSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm ', 'MOD=REG VL=256', 'VADDSUBPS', ''];
T['VADDSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128', 'MOD=MEM VL=128', 'VADDSUBPS', ''];
T['VADDSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256', 'MOD=MEM VL=256', 'VADDSUBPS', ''];

# AESDEC-Perform One Round of an AES Decryption Flow.
T['VAESDEC VREG:w:s32=$XMMr  VREG:r:s32=$XMMn  VREG:r:s32=$XMMm  ', 'MOD=REG', 'VAESDEC_ronomo_1', '         '];
T['VAESDEC VREG:w:s32=$XMMr  VREG:r:s32=$XMMn  MEM:r:s32=$mem128 ', 'MOD=MEM', 'VAESDEC_ronomo_1', '         '];
T['VAESDEC REG:w:u128=$XMMr  REG:r:u128=$XMMn  REG:r:u128=$XMMm  ', 'MOD=REG', 'VAESDEC_ronomo_2', '         '];
T['VAESDEC REG:w:u128=$XMMr  REG:r:u128=$XMMn  MEM:r:u128=$mem128', 'MOD=MEM', 'VAESDEC_ronomo_2', 'tuple=FVM'];
T['VAESDEC VREG:w:u128=$YMMr VREG:r:u128=$YMMn VREG:r:u128=$YMMm ', 'MOD=REG', 'VAESDEC_rynymy_1', '         '];
T['VAESDEC VREG:w:u128=$YMMr VREG:r:u128=$YMMn MEM:r:u128=$mem256', 'MOD=MEM', 'VAESDEC_rynymy_1', '         '];
T['VAESDEC VREG:w:u128=$YMMr VREG:r:u128=$YMMn VREG:r:u128=$YMMm ', 'MOD=REG', 'VAESDEC_rynymy_2', '         '];
T['VAESDEC VREG:w:u128=$YMMr VREG:r:u128=$YMMn MEM:r:u128=$mem256', 'MOD=MEM', 'VAESDEC_rynymy_2', 'tuple=FVM'];
T['VAESDEC VREG:w:u128=$ZMMr VREG:r:u128=$ZMMn VREG:r:u128=$ZMMm ', 'MOD=REG', 'VAESDEC_rznzmz  ', '         '];
T['VAESDEC VREG:w:u128=$ZMMr VREG:r:u128=$ZMMn MEM:r:u128=$mem512', 'MOD=MEM', 'VAESDEC_rznzmz  ', 'tuple=FVM'];

# AESDECLAST-Perform Last Round of an AES Decryption Flow.
T['VAESDECLAST VREG:w:s32=$XMMr  VREG:r:s32=$XMMn  VREG:r:s32=$XMMm  ', 'MOD=REG', 'VAESDECLAST_ronomo_1', '         '];
T['VAESDECLAST VREG:w:s32=$XMMr  VREG:r:s32=$XMMn  MEM:r:s32=$mem128 ', 'MOD=MEM', 'VAESDECLAST_ronomo_1', '         '];
T['VAESDECLAST REG:w:u128=$XMMr  REG:r:u128=$XMMn  REG:r:u128=$XMMm  ', 'MOD=REG', 'VAESDECLAST_ronomo_2', '         '];
T['VAESDECLAST REG:w:u128=$XMMr  REG:r:u128=$XMMn  MEM:r:u128=$mem128', 'MOD=MEM', 'VAESDECLAST_ronomo_2', 'tuple=FVM'];
T['VAESDECLAST VREG:w:u128=$YMMr VREG:r:u128=$YMMn VREG:r:u128=$YMMm ', 'MOD=REG', 'VAESDECLAST_rynymy_1', '         '];
T['VAESDECLAST VREG:w:u128=$YMMr VREG:r:u128=$YMMn MEM:r:u128=$mem256', 'MOD=MEM', 'VAESDECLAST_rynymy_1', '         '];
T['VAESDECLAST VREG:w:u128=$YMMr VREG:r:u128=$YMMn VREG:r:u128=$YMMm ', 'MOD=REG', 'VAESDECLAST_rynymy_2', '         '];
T['VAESDECLAST VREG:w:u128=$YMMr VREG:r:u128=$YMMn MEM:r:u128=$mem256', 'MOD=MEM', 'VAESDECLAST_rynymy_2', 'tuple=FVM'];
T['VAESDECLAST VREG:w:u128=$ZMMr VREG:r:u128=$ZMMn VREG:r:u128=$ZMMm ', 'MOD=REG', 'VAESDECLAST_rznzmz  ', '         '];
T['VAESDECLAST VREG:w:u128=$ZMMr VREG:r:u128=$ZMMn MEM:r:u128=$mem512', 'MOD=MEM', 'VAESDECLAST_rznzmz  ', 'tuple=FVM'];

# AESENC-Perform One Round of an AES Encryption Flow.
T['VAESENC VREG:w:s32=$XMMr  VREG:r:s32=$XMMn  VREG:r:s32=$XMMm  ', 'MOD=REG', 'VAESENC_ronomo_1', '         '];
T['VAESENC VREG:w:s32=$XMMr  VREG:r:s32=$XMMn  MEM:r:s32=$mem128 ', 'MOD=MEM', 'VAESENC_ronomo_1', '         '];
T['VAESENC REG:w:u128=$XMMr  REG:r:u128=$XMMn  REG:r:u128=$XMMm  ', 'MOD=REG', 'VAESENC_ronomo_2', '         '];
T['VAESENC REG:w:u128=$XMMr  REG:r:u128=$XMMn  MEM:r:u128=$mem128', 'MOD=MEM', 'VAESENC_ronomo_2', 'tuple=FVM'];
T['VAESENC VREG:w:u128=$YMMr VREG:r:u128=$YMMn VREG:r:u128=$YMMm ', 'MOD=REG', 'VAESENC_rynymy_1', '         '];
T['VAESENC VREG:w:u128=$YMMr VREG:r:u128=$YMMn MEM:r:u128=$mem256', 'MOD=MEM', 'VAESENC_rynymy_1', '         '];
T['VAESENC VREG:w:u128=$YMMr VREG:r:u128=$YMMn VREG:r:u128=$YMMm ', 'MOD=REG', 'VAESENC_rynymy_2', '         '];
T['VAESENC VREG:w:u128=$YMMr VREG:r:u128=$YMMn MEM:r:u128=$mem256', 'MOD=MEM', 'VAESENC_rynymy_2', 'tuple=FVM'];
T['VAESENC VREG:w:u128=$ZMMr VREG:r:u128=$ZMMn VREG:r:u128=$ZMMm ', 'MOD=REG', 'VAESENC_rznzmz  ', '         '];
T['VAESENC VREG:w:u128=$ZMMr VREG:r:u128=$ZMMn MEM:r:u128=$mem512', 'MOD=MEM', 'VAESENC_rznzmz  ', 'tuple=FVM'];

# AESENCLAST-Perform Last Round of an AES Encryption Flow.
T['VAESENCLAST VREG:w:s32=$XMMr  VREG:r:s32=$XMMn  VREG:r:s32=$XMMm  ', 'MOD=REG', 'VAESENCLAST_ronomo_1', '         '];
T['VAESENCLAST VREG:w:s32=$XMMr  VREG:r:s32=$XMMn  MEM:r:s32=$mem128 ', 'MOD=MEM', 'VAESENCLAST_ronomo_1', '         '];
T['VAESENCLAST REG:w:u128=$XMMr  REG:r:u128=$XMMn  REG:r:u128=$XMMm  ', 'MOD=REG', 'VAESENCLAST_ronomo_2', '         '];
T['VAESENCLAST REG:w:u128=$XMMr  REG:r:u128=$XMMn  MEM:r:u128=$mem128', 'MOD=MEM', 'VAESENCLAST_ronomo_2', 'tuple=FVM'];
T['VAESENCLAST VREG:w:u128=$YMMr VREG:r:u128=$YMMn VREG:r:u128=$YMMm ', 'MOD=REG', 'VAESENCLAST_rynymy_1', '         '];
T['VAESENCLAST VREG:w:u128=$YMMr VREG:r:u128=$YMMn MEM:r:u128=$mem256', 'MOD=MEM', 'VAESENCLAST_rynymy_1', '         '];
T['VAESENCLAST VREG:w:u128=$YMMr VREG:r:u128=$YMMn VREG:r:u128=$YMMm ', 'MOD=REG', 'VAESENCLAST_rynymy_2', '         '];
T['VAESENCLAST VREG:w:u128=$YMMr VREG:r:u128=$YMMn MEM:r:u128=$mem256', 'MOD=MEM', 'VAESENCLAST_rynymy_2', 'tuple=FVM'];
T['VAESENCLAST VREG:w:u128=$ZMMr VREG:r:u128=$ZMMn VREG:r:u128=$ZMMm ', 'MOD=REG', 'VAESENCLAST_rznzmz  ', '         '];
T['VAESENCLAST VREG:w:u128=$ZMMr VREG:r:u128=$ZMMn MEM:r:u128=$mem512', 'MOD=MEM', 'VAESENCLAST_rznzmz  ', 'tuple=FVM'];

# AESIMC-Perform the AES InvMixColumn Transformation.
T['VAESIMC VREG:w:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'VAESIMC', ''];
T['VAESIMC VREG:w:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'VAESIMC', ''];

# AESKEYGENASSIST-AES Round Key Generation Assist.
T['VAESKEYGENASSIST VREG:w:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'VAESKEYGENASSIST', ''];
T['VAESKEYGENASSIST VREG:w:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'VAESKEYGENASSIST', ''];

# ANDNPD-Bitwise Logical AND NOT of Packed Double Precision Floating-Point Values.
T['VANDNPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG  ', 'VANDNPD_ronomo', '        '];
T['VANDNPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM       ', 'VANDNPD_ronomo', 'tuple=FV'];
T['VANDNPD VREG:w:u64=$XMMr VREG:r:u64=$XMMn VREG:r:u64=$XMMm                              ', 'MOD=REG VL=128', 'VANDNPD_rxnxmx', '        '];
T['VANDNPD VREG:w:u64=$YMMr VREG:r:u64=$YMMn VREG:r:u64=$YMMm                              ', 'MOD=REG VL=256', 'VANDNPD_rxnxmx', '        '];
T['VANDNPD VREG:w:u64=$XMMr VREG:r:u64=$XMMn MEM:r:u64=$mem128                             ', 'MOD=MEM VL=128', 'VANDNPD_rxnxmx', '        '];
T['VANDNPD VREG:w:u64=$YMMr VREG:r:u64=$YMMn MEM:r:u64=$mem256                             ', 'MOD=MEM VL=256', 'VANDNPD_rxnxmx', '        '];
T['VANDNPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG  ', 'VANDNPD_rynymy', '        '];
T['VANDNPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM       ', 'VANDNPD_rynymy', 'tuple=FV'];
T['VANDNPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG  ', 'VANDNPD_rznzmz', '        '];
T['VANDNPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM       ', 'VANDNPD_rznzmz', 'tuple=FV'];

# ANDNPS-Bitwise Logical AND NOT of Packed Single Precision Floating-Point Values.
T['VANDNPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG  ', 'VANDNPS_ronomo', '        '];
T['VANDNPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM       ', 'VANDNPS_ronomo', 'tuple=FV'];
T['VANDNPS VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm                              ', 'MOD=REG VL=128', 'VANDNPS_rxnxmx', '        '];
T['VANDNPS VREG:w:s32=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm                              ', 'MOD=REG VL=256', 'VANDNPS_rxnxmx', '        '];
T['VANDNPS VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128                             ', 'MOD=MEM VL=128', 'VANDNPS_rxnxmx', '        '];
T['VANDNPS VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256                             ', 'MOD=MEM VL=256', 'VANDNPS_rxnxmx', '        '];
T['VANDNPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG  ', 'VANDNPS_rynymy', '        '];
T['VANDNPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM       ', 'VANDNPS_rynymy', 'tuple=FV'];
T['VANDNPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG  ', 'VANDNPS_rznzmz', '        '];
T['VANDNPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM       ', 'VANDNPS_rznzmz', 'tuple=FV'];

# ANDPD-Bitwise Logical AND of Packed Double Precision Floating-Point Values.
T['VANDPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG  ', 'VANDPD_ronomo', '        '];
T['VANDPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM       ', 'VANDPD_ronomo', 'tuple=FV'];
T['VANDPD VREG:w:u64=$XMMr VREG:r:u64=$XMMn VREG:r:u64=$XMMm                              ', 'MOD=REG VL=128', 'VANDPD_rxnxmx', '        '];
T['VANDPD VREG:w:u64=$YMMr VREG:r:u64=$YMMn VREG:r:u64=$YMMm                              ', 'MOD=REG VL=256', 'VANDPD_rxnxmx', '        '];
T['VANDPD VREG:w:u64=$XMMr VREG:r:u64=$XMMn MEM:r:u64=$mem128                             ', 'MOD=MEM VL=128', 'VANDPD_rxnxmx', '        '];
T['VANDPD VREG:w:u64=$YMMr VREG:r:u64=$YMMn MEM:r:u64=$mem256                             ', 'MOD=MEM VL=256', 'VANDPD_rxnxmx', '        '];
T['VANDPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG  ', 'VANDPD_rynymy', '        '];
T['VANDPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM       ', 'VANDPD_rynymy', 'tuple=FV'];
T['VANDPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG  ', 'VANDPD_rznzmz', '        '];
T['VANDPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM       ', 'VANDPD_rznzmz', 'tuple=FV'];

# ANDPS-Bitwise Logical AND of Packed Single Precision Floating-Point Values.
T['VANDPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG  ', 'VANDPS_ronomo', '        '];
T['VANDPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM       ', 'VANDPS_ronomo', 'tuple=FV'];
T['VANDPS VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm                              ', 'MOD=REG VL=128', 'VANDPS_rxnxmx', '        '];
T['VANDPS VREG:w:s32=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm                              ', 'MOD=REG VL=256', 'VANDPS_rxnxmx', '        '];
T['VANDPS VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128                             ', 'MOD=MEM VL=128', 'VANDPS_rxnxmx', '        '];
T['VANDPS VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256                             ', 'MOD=MEM VL=256', 'VANDPS_rxnxmx', '        '];
T['VANDPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG  ', 'VANDPS_rynymy', '        '];
T['VANDPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM       ', 'VANDPS_rynymy', 'tuple=FV'];
T['VANDPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG  ', 'VANDPS_rznzmz', '        '];
T['VANDPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM       ', 'VANDPS_rznzmz', 'tuple=FV'];

# BLENDPD-Blend Packed Double Precision Floating-Point Values.
T['VBLENDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm  IMM:u8=$uimm8', 'MOD=REG VL=128', 'VBLENDPD', ''];
T['VBLENDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm  IMM:u8=$uimm8', 'MOD=REG VL=256', 'VBLENDPD', ''];
T['VBLENDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128 IMM:u8=$uimm8', 'MOD=MEM VL=128', 'VBLENDPD', ''];
T['VBLENDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256 IMM:u8=$uimm8', 'MOD=MEM VL=256', 'VBLENDPD', ''];

# BLENDPS-Blend Packed Single Precision Floating-Point Values.
T['VBLENDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm  IMM:u8=$uimm8', 'MOD=REG VL=128', 'VBLENDPS', ''];
T['VBLENDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm  IMM:u8=$uimm8', 'MOD=REG VL=256', 'VBLENDPS', ''];
T['VBLENDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128 IMM:u8=$uimm8', 'MOD=MEM VL=128', 'VBLENDPS', ''];
T['VBLENDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256 IMM:u8=$uimm8', 'MOD=MEM VL=256', 'VBLENDPS', ''];

# BLENDVPD-Variable Blend Packed Double Precision Floating-Point Values.
T['VBLENDVPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm  VREG:r:u64=$XMMis4', 'MOD=REG VL=128', 'VBLENDVPD', ''];
T['VBLENDVPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm  VREG:r:u64=$YMMis4', 'MOD=REG VL=256', 'VBLENDVPD', ''];
T['VBLENDVPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128 VREG:r:u64=$XMMis4', 'MOD=MEM VL=128', 'VBLENDVPD', ''];
T['VBLENDVPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256 VREG:r:u64=$YMMis4', 'MOD=MEM VL=256', 'VBLENDVPD', ''];

# BLENDVPS-Variable Blend Packed Single Precision Floating-Point Values.
T['VBLENDVPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm  VREG:r:u32=$XMMis4', 'MOD=REG VL=128', 'VBLENDVPS', ''];
T['VBLENDVPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm  VREG:r:u32=$YMMis4', 'MOD=REG VL=256', 'VBLENDVPS', ''];
T['VBLENDVPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128 VREG:r:u32=$XMMis4', 'MOD=MEM VL=128', 'VBLENDVPS', ''];
T['VBLENDVPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256 VREG:r:u32=$YMMis4', 'MOD=MEM VL=256', 'VBLENDVPS', ''];

# CMPPD-Compare Packed Double-Precision Floating-Point Values.
T['VCMPPD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm             CC:u8=$cc5', 'BP=0 MOD=REG        ', 'VCMPPD_nomoub  ', '        '];
T['VCMPPD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST CC:u8=$cc5', 'MOD=MEM             ', 'VCMPPD_nomoub  ', 'tuple=FV'];
T['VCMPPD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm             CC:u8=$cc5', 'BP=0 MOD=REG        ', 'VCMPPD_nymyub  ', '        '];
T['VCMPPD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST CC:u8=$cc5', 'MOD=MEM             ', 'VCMPPD_nymyub  ', 'tuple=FV'];
T['VCMPPD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm SAE=$SAE    CC:u8=$cc5', 'BP=1 MOD=REG RVL=512', 'VCMPPD_nzmzub  ', '        '];
T['VCMPPD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm             CC:u8=$cc5', 'BP=0 MOD=REG VL=512 ', 'VCMPPD_nzmzub  ', '        '];
T['VCMPPD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST CC:u8=$cc5', 'MOD=MEM VL=512      ', 'VCMPPD_nzmzub  ', 'tuple=FV'];
T['VCMPPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm  CC:u8=$cc5                             ', 'MOD=REG VL=128      ', 'VCMPPD_rxnxmxub', '        '];
T['VCMPPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm  CC:u8=$cc5                             ', 'MOD=REG VL=256      ', 'VCMPPD_rxnxmxub', '        '];
T['VCMPPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128 CC:u8=$cc5                             ', 'MOD=MEM VL=128      ', 'VCMPPD_rxnxmxub', '        '];
T['VCMPPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256 CC:u8=$cc5                             ', 'MOD=MEM VL=256      ', 'VCMPPD_rxnxmxub', '        '];

# CMPPS-Compare Packed Single-Precision Floating-Point Values.
T['VCMPPS REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm             CC:u8=$cc5', 'BP=0 MOD=REG        ', 'VCMPPS_nomoub  ', '        '];
T['VCMPPS REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST CC:u8=$cc5', 'MOD=MEM             ', 'VCMPPS_nomoub  ', 'tuple=FV'];
T['VCMPPS REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm             CC:u8=$cc5', 'BP=0 MOD=REG        ', 'VCMPPS_nymyub  ', '        '];
T['VCMPPS REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST CC:u8=$cc5', 'MOD=MEM             ', 'VCMPPS_nymyub  ', 'tuple=FV'];
T['VCMPPS REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm SAE=$SAE    CC:u8=$cc5', 'BP=1 MOD=REG RVL=512', 'VCMPPS_nzmzub  ', '        '];
T['VCMPPS REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm             CC:u8=$cc5', 'BP=0 MOD=REG VL=512 ', 'VCMPPS_nzmzub  ', '        '];
T['VCMPPS REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST CC:u8=$cc5', 'MOD=MEM VL=512      ', 'VCMPPS_nzmzub  ', 'tuple=FV'];
T['VCMPPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm  CC:u8=$cc5                             ', 'MOD=REG VL=128      ', 'VCMPPS_rxnxmxub', '        '];
T['VCMPPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm  CC:u8=$cc5                             ', 'MOD=REG VL=256      ', 'VCMPPS_rxnxmxub', '        '];
T['VCMPPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128 CC:u8=$cc5                             ', 'MOD=MEM VL=128      ', 'VCMPPS_rxnxmxub', '        '];
T['VCMPPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256 CC:u8=$cc5                             ', 'MOD=MEM VL=256      ', 'VCMPPS_rxnxmxub', '        '];

# CMPS/CMPSB/CMPSW/CMPSD/CMPSQ-Compare String Operands.
T['VCMPSD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm          CC:u8=$cc5', 'BP=0 MOD=REG        ', 'VCMPSD_nomxub  ', '         '];
T['VCMPSD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm SAE=$SAE CC:u8=$cc5', 'BP=1 MOD=REG RVL=128', 'VCMPSD_nomxub  ', '         '];
T['VCMPSD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f64=$XMMn MEM:r:f64=$mem64          CC:u8=$cc5', 'BP=0 MOD=MEM        ', 'VCMPSD_nomxub  ', 'tuple=T1S'];
T['VCMPSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn REG:r:f64=$XMMm  CC:u8=$cc5                          ', 'MOD=REG             ', 'VCMPSD_ronomqub', '         '];
T['VCMPSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem64 CC:u8=$cc5                          ', 'MOD=MEM             ', 'VCMPSD_ronomqub', '         '];

# CMPSS-Compare Scalar Single-Precision Floating-Point Value.
T['VCMPSS REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm          CC:u8=$cc5', 'BP=0 MOD=REG        ', 'VCMPSS_nomxub  ', '         '];
T['VCMPSS REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm SAE=$SAE CC:u8=$cc5', 'BP=1 MOD=REG RVL=128', 'VCMPSS_nomxub  ', '         '];
T['VCMPSS REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:f32=$XMMn MEM:r:f32=$mem32          CC:u8=$cc5', 'BP=0 MOD=MEM        ', 'VCMPSS_nomxub  ', 'tuple=T1S'];
T['VCMPSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn REG:r:f32=$XMMm  CC:u8=$cc5                          ', 'MOD=REG             ', 'VCMPSS_ronomdub', '         '];
T['VCMPSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem32 CC:u8=$cc5                          ', 'MOD=MEM             ', 'VCMPSS_ronomdub', '         '];

# COMISD-Compare Scalar Ordered Double-Precision Floating-Point Values and Set EFLAGS.
T['VCOMISD VREG:r:f64=$XMMr VREG:r:f64=$XMMm         ', 'BP=0 MOD=REG        ', 'VCOMISD_romx', '         '];
T['VCOMISD VREG:r:f64=$XMMr VREG:r:f64=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128', 'VCOMISD_romx', '         '];
T['VCOMISD VREG:r:f64=$XMMr MEM:r:f64=$mem64         ', 'BP=0 MOD=MEM        ', 'VCOMISD_romx', 'tuple=T1S'];
T['VCOMISD REG:r:f64=$XMMr  REG:r:f64=$XMMm          ', 'MOD=REG             ', 'VCOMISD_rqmq', '         '];
T['VCOMISD REG:r:f64=$XMMr  MEM:r:f64=$mem64         ', 'MOD=MEM             ', 'VCOMISD_rqmq', '         '];

# COMISS-Compare Scalar Ordered Single-Precision Floating-Point Values and Set EFLAGS.
T['VCOMISS REG:r:f32=$XMMr  REG:r:f32=$XMMm          ', 'MOD=REG             ', 'VCOMISS_rdmd', '         '];
T['VCOMISS REG:r:f32=$XMMr  MEM:r:f32=$mem32         ', 'MOD=MEM             ', 'VCOMISS_rdmd', '         '];
T['VCOMISS VREG:r:f32=$XMMr VREG:r:f32=$XMMm         ', 'BP=0 MOD=REG        ', 'VCOMISS_romx', '         '];
T['VCOMISS VREG:r:f32=$XMMr VREG:r:f32=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128', 'VCOMISS_romx', '         '];
T['VCOMISS VREG:r:f32=$XMMr MEM:r:f32=$mem32         ', 'BP=0 MOD=MEM        ', 'VCOMISS_romx', 'tuple=T1S'];

# CVTDQ2PD-Convert Packed Doubleword Integers to Packed Double-Precision Floating-Point Values.
T['VCVTDQ2PD VREG:w:f64=$XMMr PREG:r:Z=$Kk      VREG:r:s32=$XMMm            ', 'BP=0 MOD=REG  ', 'VCVTDQ2PD_romx', '        '];
T['VCVTDQ2PD VREG:w:f64=$XMMr PREG:r:Z=$Kk      MEM:r:s32=$mem64 BCST=$BCST ', 'MOD=MEM       ', 'VCVTDQ2PD_romx', 'tuple=HV'];
T['VCVTDQ2PD VREG:w:f64=$XMMr VREG:r:s32=$XMMm                              ', 'MOD=REG VL=128', 'VCVTDQ2PD_rxmx', '        '];
T['VCVTDQ2PD VREG:w:f64=$YMMr VREG:r:s32=$XMMm                              ', 'MOD=REG VL=256', 'VCVTDQ2PD_rxmx', '        '];
T['VCVTDQ2PD VREG:w:f64=$XMMr MEM:r:s32=$mem64                              ', 'MOD=MEM VL=128', 'VCVTDQ2PD_rxmx', '        '];
T['VCVTDQ2PD VREG:w:f64=$YMMr MEM:r:s32=$mem128                             ', 'MOD=MEM VL=256', 'VCVTDQ2PD_rxmx', '        '];
T['VCVTDQ2PD VREG:w:f64=$YMMr PREG:r:Z=$Kk      VREG:r:s32=$XMMm            ', 'BP=0 MOD=REG  ', 'VCVTDQ2PD_rymo', '        '];
T['VCVTDQ2PD VREG:w:f64=$YMMr PREG:r:Z=$Kk      MEM:r:s32=$mem128 BCST=$BCST', 'MOD=MEM       ', 'VCVTDQ2PD_rymo', 'tuple=HV'];
T['VCVTDQ2PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk      VREG:r:s32=$YMMm            ', 'BP=0 MOD=REG  ', 'VCVTDQ2PD_rzmy', '        '];
T['VCVTDQ2PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk      MEM:r:s32=$mem256 BCST=$BCST', 'MOD=MEM       ', 'VCVTDQ2PD_rzmy', 'tuple=HV'];

# CVTDQ2PS-Convert Packed Doubleword Integers to Packed Single-Precision Floating-Point Values.
T['VCVTDQ2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk      VREG:r:s32=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTDQ2PS_romo', '        '];
T['VCVTDQ2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk      MEM:r:s32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTDQ2PS_romo', 'tuple=FV'];
T['VCVTDQ2PS VREG:w:f32=$XMMr VREG:r:s32=$XMMm                              ', 'MOD=REG VL=128      ', 'VCVTDQ2PS_rxmx', '        '];
T['VCVTDQ2PS VREG:w:f32=$YMMr VREG:r:s32=$YMMm                              ', 'MOD=REG VL=256      ', 'VCVTDQ2PS_rxmx', '        '];
T['VCVTDQ2PS VREG:w:f32=$XMMr MEM:r:s32=$mem128                             ', 'MOD=MEM VL=128      ', 'VCVTDQ2PS_rxmx', '        '];
T['VCVTDQ2PS VREG:w:f32=$YMMr MEM:r:s32=$mem256                             ', 'MOD=MEM VL=256      ', 'VCVTDQ2PS_rxmx', '        '];
T['VCVTDQ2PS VREG:w:f32=$YMMr PREG:r:Z=$Kk      VREG:r:s32=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTDQ2PS_rymy', '        '];
T['VCVTDQ2PS VREG:w:f32=$YMMr PREG:r:Z=$Kk      MEM:r:s32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTDQ2PS_rymy', 'tuple=FV'];
T['VCVTDQ2PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk      VREG:r:s32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VCVTDQ2PS_rzmz', '        '];
T['VCVTDQ2PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk      VREG:r:s32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTDQ2PS_rzmz', '        '];
T['VCVTDQ2PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk      MEM:r:s32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTDQ2PS_rzmz', 'tuple=FV'];

# CVTPD2DQ-Convert Packed Double-Precision Floating-Point Values to Packed Doubleword Integers.
T['VCVTPD2DQ VREG:w:s32=$XMMr PREG:r:Z=$Kk      VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTPD2DQ_romo', '        '];
T['VCVTPD2DQ VREG:w:s32=$XMMr PREG:r:Z=$Kk      MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTPD2DQ_romo', 'tuple=FV'];
T['VCVTPD2DQ VREG:w:s32=$XMMr VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VCVTPD2DQ_romx', '        '];
T['VCVTPD2DQ VREG:w:s32=$XMMr VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VCVTPD2DQ_romx', '        '];
T['VCVTPD2DQ VREG:w:s32=$XMMr MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VCVTPD2DQ_romx', '        '];
T['VCVTPD2DQ VREG:w:s32=$XMMr MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VCVTPD2DQ_romx', '        '];
T['VCVTPD2DQ VREG:w:s32=$XMMr PREG:r:Z=$Kk      VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTPD2DQ_romy', '        '];
T['VCVTPD2DQ VREG:w:s32=$XMMr PREG:r:Z=$Kk      MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTPD2DQ_romy', 'tuple=FV'];
T['VCVTPD2DQ VREG:w:s32=$YMMr PREG:r:Z=$Kk      VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VCVTPD2DQ_rymz', '        '];
T['VCVTPD2DQ VREG:w:s32=$YMMr PREG:r:Z=$Kk      VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTPD2DQ_rymz', '        '];
T['VCVTPD2DQ VREG:w:s32=$YMMr PREG:r:Z=$Kk      MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTPD2DQ_rymz', 'tuple=FV'];

# CVTPD2PS-Convert Packed Double-Precision Floating-Point Values to Packed Single-Precision Floating-Point Values.
T['VCVTPD2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk      VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTPD2PS_romo', '        '];
T['VCVTPD2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk      MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTPD2PS_romo', 'tuple=FV'];
T['VCVTPD2PS VREG:w:f32=$XMMr VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VCVTPD2PS_romx', '        '];
T['VCVTPD2PS VREG:w:f32=$XMMr VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VCVTPD2PS_romx', '        '];
T['VCVTPD2PS VREG:w:f32=$XMMr MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VCVTPD2PS_romx', '        '];
T['VCVTPD2PS VREG:w:f32=$XMMr MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VCVTPD2PS_romx', '        '];
T['VCVTPD2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk      VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTPD2PS_romy', '        '];
T['VCVTPD2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk      MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTPD2PS_romy', 'tuple=FV'];
T['VCVTPD2PS VREG:w:f32=$YMMr PREG:r:Z=$Kk      VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VCVTPD2PS_rymz', '        '];
T['VCVTPD2PS VREG:w:f32=$YMMr PREG:r:Z=$Kk      VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTPD2PS_rymz', '        '];
T['VCVTPD2PS VREG:w:f32=$YMMr PREG:r:Z=$Kk      MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTPD2PS_rymz', 'tuple=FV'];

# CVTPS2DQ-Convert Packed Single-Precision Floating-Point Values to Packed Signed Doubleword Integer Values .3-258.
T['VCVTPS2DQ VREG:w:s32=$XMMr PREG:r:Z=$Kk      VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTPS2DQ_romo', '        '];
T['VCVTPS2DQ VREG:w:s32=$XMMr PREG:r:Z=$Kk      MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTPS2DQ_romo', 'tuple=FV'];
T['VCVTPS2DQ VREG:w:s32=$XMMr VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VCVTPS2DQ_rxmx', '        '];
T['VCVTPS2DQ VREG:w:s32=$YMMr VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VCVTPS2DQ_rxmx', '        '];
T['VCVTPS2DQ VREG:w:s32=$XMMr MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VCVTPS2DQ_rxmx', '        '];
T['VCVTPS2DQ VREG:w:s32=$YMMr MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VCVTPS2DQ_rxmx', '        '];
T['VCVTPS2DQ VREG:w:s32=$YMMr PREG:r:Z=$Kk      VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTPS2DQ_rymy', '        '];
T['VCVTPS2DQ VREG:w:s32=$YMMr PREG:r:Z=$Kk      MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTPS2DQ_rymy', 'tuple=FV'];
T['VCVTPS2DQ VREG:w:s32=$ZMMr PREG:r:Z=$Kk      VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VCVTPS2DQ_rzmz', '        '];
T['VCVTPS2DQ VREG:w:s32=$ZMMr PREG:r:Z=$Kk      VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTPS2DQ_rzmz', '        '];
T['VCVTPS2DQ VREG:w:s32=$ZMMr PREG:r:Z=$Kk      MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTPS2DQ_rzmz', 'tuple=FV'];

# CVTPS2PD-Convert Packed Single-Precision Floating-Point Values to Packed Double-Precision Floating-Point Values.
T['VCVTPS2PD VREG:w:f64=$XMMr PREG:r:Z=$Kk      VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTPS2PD_romx', '        '];
T['VCVTPS2PD VREG:w:f64=$XMMr PREG:r:Z=$Kk      MEM:r:f32=$mem64 BCST=$BCST ', 'MOD=MEM             ', 'VCVTPS2PD_romx', 'tuple=HV'];
T['VCVTPS2PD VREG:w:f64=$XMMr VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VCVTPS2PD_rxmx', '        '];
T['VCVTPS2PD VREG:w:f64=$YMMr VREG:r:f32=$XMMm                              ', 'MOD=REG VL=256      ', 'VCVTPS2PD_rxmx', '        '];
T['VCVTPS2PD VREG:w:f64=$XMMr MEM:r:f32=$mem64                              ', 'MOD=MEM VL=128      ', 'VCVTPS2PD_rxmx', '        '];
T['VCVTPS2PD VREG:w:f64=$YMMr MEM:r:f32=$mem128                             ', 'MOD=MEM VL=256      ', 'VCVTPS2PD_rxmx', '        '];
T['VCVTPS2PD VREG:w:f64=$YMMr PREG:r:Z=$Kk      VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTPS2PD_rymo', '        '];
T['VCVTPS2PD VREG:w:f64=$YMMr PREG:r:Z=$Kk      MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTPS2PD_rymo', 'tuple=HV'];
T['VCVTPS2PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk      VREG:r:f32=$YMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VCVTPS2PD_rzmy', '        '];
T['VCVTPS2PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk      VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTPS2PD_rzmy', '        '];
T['VCVTPS2PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk      MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTPS2PD_rzmy', 'tuple=HV'];

# CVTSD2SI-Convert Scalar Double-Precision Floating-Point Value to Doubleword Integer.
T['VCVTSD2SI REG:w:s32=$GPR32r REG:r:f64=$XMMm        ', 'MOD=REG MODE=64 W=0               ', 'VCVTSD2SI_rxmq', '           '];
T['VCVTSD2SI REG:w:s64=$GPR64r REG:r:f64=$XMMm        ', 'MOD=REG MODE=64 W=1               ', 'VCVTSD2SI_rxmq', '           '];
T['VCVTSD2SI REG:w:s32=$GPR32r MEM:r:f64=$mem64       ', 'MOD=MEM MODE=64 W=0               ', 'VCVTSD2SI_rxmq', '           '];
T['VCVTSD2SI REG:w:s64=$GPR64r MEM:r:f64=$mem64       ', 'MOD=MEM MODE=64 W=1               ', 'VCVTSD2SI_rxmq', '           '];
T['VCVTSD2SI REG:w:s32=$GPR32r REG:r:f64=$XMMm        ', 'MOD=REG MODE=NO64                 ', 'VCVTSD2SI_rxmq', '           '];
T['VCVTSD2SI REG:w:s32=$GPR32r MEM:r:f64=$mem64       ', 'MOD=MEM MODE=NO64                 ', 'VCVTSD2SI_rxmq', '           '];
T['VCVTSD2SI REG:w:s32=$GPR32r VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG W=0                  ', 'VCVTSD2SI_rxmx', '           '];
T['VCVTSD2SI REG:w:s32=$GPR32r VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128 W=0          ', 'VCVTSD2SI_rxmx', '           '];
T['VCVTSD2SI REG:w:s32=$GPR32r MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM W=0                  ', 'VCVTSD2SI_rxmx', 'tuple=T1F64'];
T['VCVTSD2SI REG:w:s64=$GPR64r VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG MODE=64 W=1          ', 'VCVTSD2SI_rxmx', '           '];
T['VCVTSD2SI REG:w:s64=$GPR64r VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG MODE=64 RVL=128 W=1  ', 'VCVTSD2SI_rxmx', '           '];
T['VCVTSD2SI REG:w:s64=$GPR64r MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM MODE=64 W=1          ', 'VCVTSD2SI_rxmx', 'tuple=T1F64'];
T['VCVTSD2SI REG:w:s32=$GPR32r VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG MODE=NO64 W=1        ', 'VCVTSD2SI_rxmx', '           '];
T['VCVTSD2SI REG:w:s32=$GPR32r VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG MODE=NO64 RVL=128 W=1', 'VCVTSD2SI_rxmx', '           '];
T['VCVTSD2SI REG:w:s32=$GPR32r MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM MODE=NO64 W=1        ', 'VCVTSD2SI_rxmx', 'tuple=T1F64'];

# CVTSD2SS-Convert Scalar Double-Precision Floating-Point Value to Scalar Single-Precision Floating-Point Value. .3-267.
T['VCVTSD2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VCVTSD2SS_ronomq', '         '];
T['VCVTSD2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VCVTSD2SS_ronomq', '         '];
T['VCVTSD2SS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VCVTSD2SS_ronomx', '         '];
T['VCVTSD2SS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VCVTSD2SS_ronomx', '         '];
T['VCVTSD2SS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VCVTSD2SS_ronomx', 'tuple=T1S'];

# CVTSI2SD-Convert Doubleword Integer to Scalar Double-Precision Floating-Point Value.
T['VCVTSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        REG:r:s32=$GPR32m', 'MOD=REG W=0          ', 'VCVTSI2SD_ronomd', '         '];
T['VCVTSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        MEM:r:s32=$mem32 ', 'MOD=MEM W=0          ', 'VCVTSI2SD_ronomd', 'tuple=T1S'];
T['VCVTSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        REG:r:s32=$GPR32m', 'MOD=REG MODE=NO64 W=1', 'VCVTSI2SD_ronomd', '         '];
T['VCVTSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        MEM:r:s32=$mem32 ', 'MOD=MEM MODE=NO64 W=1', 'VCVTSI2SD_ronomd', 'tuple=T1S'];
T['VCVTSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        REG:r:s64=$GPR64m', 'BP=0 MOD=REG         ', 'VCVTSI2SD_ronomq', '         '];
T['VCVTSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn RC=$RC REG:r:s64=$GPR64m', 'BP=1 MOD=REG RVL=128 ', 'VCVTSI2SD_ronomq', '         '];
T['VCVTSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        MEM:r:s64=$mem64 ', 'BP=0 MOD=MEM         ', 'VCVTSI2SD_ronomq', 'tuple=T1S'];
T['VCVTSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        REG:r:s32=$GPR32m', 'MOD=REG MODE=64 W=0  ', 'VCVTSI2SD_ronomx', '         '];
T['VCVTSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        REG:r:s64=$GPR64m', 'MOD=REG MODE=64 W=1  ', 'VCVTSI2SD_ronomx', '         '];
T['VCVTSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        MEM:r:s32=$mem32 ', 'MOD=MEM MODE=64 W=0  ', 'VCVTSI2SD_ronomx', '         '];
T['VCVTSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        MEM:r:s64=$mem64 ', 'MOD=MEM MODE=64 W=1  ', 'VCVTSI2SD_ronomx', '         '];
T['VCVTSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        REG:r:s32=$GPR32m', 'MOD=REG MODE=NO64    ', 'VCVTSI2SD_ronomx', '         '];
T['VCVTSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        MEM:r:s32=$mem32 ', 'MOD=MEM MODE=NO64    ', 'VCVTSI2SD_ronomx', '         '];

# CVTSI2SS-Convert Doubleword Integer to Scalar Single-Precision Floating-Point Value.
T['VCVTSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        REG:r:s32=$GPR32m', 'MOD=REG MODE=64 W=0               ', 'VCVTSI2SS_ronomx_1', '         '];
T['VCVTSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        REG:r:s64=$GPR64m', 'MOD=REG MODE=64 W=1               ', 'VCVTSI2SS_ronomx_1', '         '];
T['VCVTSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        MEM:r:s32=$mem32 ', 'MOD=MEM MODE=64 W=0               ', 'VCVTSI2SS_ronomx_1', '         '];
T['VCVTSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        MEM:r:s64=$mem64 ', 'MOD=MEM MODE=64 W=1               ', 'VCVTSI2SS_ronomx_1', '         '];
T['VCVTSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        REG:r:s32=$GPR32m', 'MOD=REG MODE=NO64                 ', 'VCVTSI2SS_ronomx_1', '         '];
T['VCVTSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        MEM:r:s32=$mem32 ', 'MOD=MEM MODE=NO64                 ', 'VCVTSI2SS_ronomx_1', '         '];
T['VCVTSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        REG:r:s32=$GPR32m', 'BP=0 MOD=REG W=0                  ', 'VCVTSI2SS_ronomx_2', '         '];
T['VCVTSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn RC=$RC REG:r:s32=$GPR32m', 'BP=1 MOD=REG RVL=128 W=0          ', 'VCVTSI2SS_ronomx_2', '         '];
T['VCVTSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        MEM:r:s32=$mem32 ', 'BP=0 MOD=MEM W=0                  ', 'VCVTSI2SS_ronomx_2', 'tuple=T1S'];
T['VCVTSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        REG:r:s64=$GPR64m', 'BP=0 MOD=REG MODE=64 W=1          ', 'VCVTSI2SS_ronomx_2', '         '];
T['VCVTSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn RC=$RC REG:r:s64=$GPR64m', 'BP=1 MOD=REG MODE=64 RVL=128 W=1  ', 'VCVTSI2SS_ronomx_2', '         '];
T['VCVTSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        MEM:r:s64=$mem64 ', 'BP=0 MOD=MEM MODE=64 W=1          ', 'VCVTSI2SS_ronomx_2', 'tuple=T1S'];
T['VCVTSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        REG:r:s32=$GPR32m', 'BP=0 MOD=REG MODE=NO64 W=1        ', 'VCVTSI2SS_ronomx_2', '         '];
T['VCVTSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn RC=$RC REG:r:s32=$GPR32m', 'BP=1 MOD=REG MODE=NO64 RVL=128 W=1', 'VCVTSI2SS_ronomx_2', '         '];
T['VCVTSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        MEM:r:s32=$mem32 ', 'BP=0 MOD=MEM MODE=NO64 W=1        ', 'VCVTSI2SS_ronomx_2', 'tuple=T1S'];

# CVTSS2SD-Convert Scalar Single-Precision Floating-Point Value to Scalar Double-Precision Floating-Point Value. .3-273.
T['VCVTSS2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn REG:r:f32=$XMMm                           ', 'MOD=REG             ', 'VCVTSS2SD_ronomd', '         '];
T['VCVTSS2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f32=$mem32                          ', 'MOD=MEM             ', 'VCVTSS2SD_ronomd', '         '];
T['VCVTSS2SD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm         ', 'BP=0 MOD=REG        ', 'VCVTSS2SD_ronomx', '         '];
T['VCVTSS2SD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128', 'VCVTSS2SD_ronomx', '         '];
T['VCVTSS2SD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn MEM:r:f32=$mem32         ', 'BP=0 MOD=MEM        ', 'VCVTSS2SD_ronomx', 'tuple=T1S'];

# CVTSS2SI-Convert Scalar Single-Precision Floating-Point Value to Doubleword Integer.
T['VCVTSS2SI REG:w:s32=$GPR32r REG:r:f32=$XMMm        ', 'MOD=REG MODE=64 W=0               ', 'VCVTSS2SI_rxmd', '           '];
T['VCVTSS2SI REG:w:s64=$GPR64r REG:r:f32=$XMMm        ', 'MOD=REG MODE=64 W=1               ', 'VCVTSS2SI_rxmd', '           '];
T['VCVTSS2SI REG:w:s32=$GPR32r MEM:r:f32=$mem32       ', 'MOD=MEM MODE=64 W=0               ', 'VCVTSS2SI_rxmd', '           '];
T['VCVTSS2SI REG:w:s64=$GPR64r MEM:r:f32=$mem32       ', 'MOD=MEM MODE=64 W=1               ', 'VCVTSS2SI_rxmd', '           '];
T['VCVTSS2SI REG:w:s32=$GPR32r REG:r:f32=$XMMm        ', 'MOD=REG MODE=NO64                 ', 'VCVTSS2SI_rxmd', '           '];
T['VCVTSS2SI REG:w:s32=$GPR32r MEM:r:f32=$mem32       ', 'MOD=MEM MODE=NO64                 ', 'VCVTSS2SI_rxmd', '           '];
T['VCVTSS2SI REG:w:s32=$GPR32r VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG W=0                  ', 'VCVTSS2SI_rxmx', '           '];
T['VCVTSS2SI REG:w:s32=$GPR32r VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128 W=0          ', 'VCVTSS2SI_rxmx', '           '];
T['VCVTSS2SI REG:w:s32=$GPR32r MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM W=0                  ', 'VCVTSS2SI_rxmx', 'tuple=T1F32'];
T['VCVTSS2SI REG:w:s64=$GPR64r VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG MODE=64 W=1          ', 'VCVTSS2SI_rxmx', '           '];
T['VCVTSS2SI REG:w:s64=$GPR64r VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG MODE=64 RVL=128 W=1  ', 'VCVTSS2SI_rxmx', '           '];
T['VCVTSS2SI REG:w:s64=$GPR64r MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM MODE=64 W=1          ', 'VCVTSS2SI_rxmx', 'tuple=T1F32'];
T['VCVTSS2SI REG:w:s32=$GPR32r VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG MODE=NO64 W=1        ', 'VCVTSS2SI_rxmx', '           '];
T['VCVTSS2SI REG:w:s32=$GPR32r VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG MODE=NO64 RVL=128 W=1', 'VCVTSS2SI_rxmx', '           '];
T['VCVTSS2SI REG:w:s32=$GPR32r MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM MODE=NO64 W=1        ', 'VCVTSS2SI_rxmx', 'tuple=T1F32'];

# CVTTPD2DQ-Convert with Truncation Packed Double-Precision Floating-Point Values to Packed Doubleword Integers.
T['VCVTTPD2DQ VREG:w:s32=$XMMr PREG:r:Z=$Kk      VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTTPD2DQ_romo', '        '];
T['VCVTTPD2DQ VREG:w:s32=$XMMr PREG:r:Z=$Kk      MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTTPD2DQ_romo', 'tuple=FV'];
T['VCVTTPD2DQ VREG:w:s32=$XMMr VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VCVTTPD2DQ_romx', '        '];
T['VCVTTPD2DQ VREG:w:s32=$XMMr VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VCVTTPD2DQ_romx', '        '];
T['VCVTTPD2DQ VREG:w:s32=$XMMr MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VCVTTPD2DQ_romx', '        '];
T['VCVTTPD2DQ VREG:w:s32=$XMMr MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VCVTTPD2DQ_romx', '        '];
T['VCVTTPD2DQ VREG:w:s32=$XMMr PREG:r:Z=$Kk      VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTTPD2DQ_romy', '        '];
T['VCVTTPD2DQ VREG:w:s32=$XMMr PREG:r:Z=$Kk      MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTTPD2DQ_romy', 'tuple=FV'];
T['VCVTTPD2DQ VREG:w:s32=$YMMr PREG:r:Z=$Kk      VREG:r:f64=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VCVTTPD2DQ_rymz', '        '];
T['VCVTTPD2DQ VREG:w:s32=$YMMr PREG:r:Z=$Kk      VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTTPD2DQ_rymz', '        '];
T['VCVTTPD2DQ VREG:w:s32=$YMMr PREG:r:Z=$Kk      MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTTPD2DQ_rymz', 'tuple=FV'];

# CVTTPS2DQ-Convert with Truncation Packed Single-Precision Floating-Point Values to Packed Signed Doubleword Integer Values.
T['VCVTTPS2DQ VREG:w:s32=$XMMr PREG:r:Z=$Kk      VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTTPS2DQ_romo', '        '];
T['VCVTTPS2DQ VREG:w:s32=$XMMr PREG:r:Z=$Kk      MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTTPS2DQ_romo', 'tuple=FV'];
T['VCVTTPS2DQ VREG:w:s32=$XMMr VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VCVTTPS2DQ_rxmx', '        '];
T['VCVTTPS2DQ VREG:w:s32=$YMMr VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VCVTTPS2DQ_rxmx', '        '];
T['VCVTTPS2DQ VREG:w:s32=$XMMr MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VCVTTPS2DQ_rxmx', '        '];
T['VCVTTPS2DQ VREG:w:s32=$YMMr MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VCVTTPS2DQ_rxmx', '        '];
T['VCVTTPS2DQ VREG:w:s32=$YMMr PREG:r:Z=$Kk      VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTTPS2DQ_rymy', '        '];
T['VCVTTPS2DQ VREG:w:s32=$YMMr PREG:r:Z=$Kk      MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTTPS2DQ_rymy', 'tuple=FV'];
T['VCVTTPS2DQ VREG:w:s32=$ZMMr PREG:r:Z=$Kk      VREG:r:f32=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VCVTTPS2DQ_rzmz', '        '];
T['VCVTTPS2DQ VREG:w:s32=$ZMMr PREG:r:Z=$Kk      VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTTPS2DQ_rzmz', '        '];
T['VCVTTPS2DQ VREG:w:s32=$ZMMr PREG:r:Z=$Kk      MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTTPS2DQ_rzmz', 'tuple=FV'];

# CVTTSD2SI-Convert with Truncation Scalar Double-Precision Floating-Point Value to Signed Integer.
T['VCVTTSD2SI REG:w:s32=$GPR32r REG:r:f64=$XMMm          ', 'MOD=REG MODE=64 W=0               ', 'VCVTTSD2SI_rxmq', '           '];
T['VCVTTSD2SI REG:w:s64=$GPR64r REG:r:f64=$XMMm          ', 'MOD=REG MODE=64 W=1               ', 'VCVTTSD2SI_rxmq', '           '];
T['VCVTTSD2SI REG:w:s32=$GPR32r MEM:r:f64=$mem64         ', 'MOD=MEM MODE=64 W=0               ', 'VCVTTSD2SI_rxmq', '           '];
T['VCVTTSD2SI REG:w:s64=$GPR64r MEM:r:f64=$mem64         ', 'MOD=MEM MODE=64 W=1               ', 'VCVTTSD2SI_rxmq', '           '];
T['VCVTTSD2SI REG:w:s32=$GPR32r REG:r:f64=$XMMm          ', 'MOD=REG MODE=NO64                 ', 'VCVTTSD2SI_rxmq', '           '];
T['VCVTTSD2SI REG:w:s32=$GPR32r MEM:r:f64=$mem64         ', 'MOD=MEM MODE=NO64                 ', 'VCVTTSD2SI_rxmq', '           '];
T['VCVTTSD2SI REG:w:s32=$GPR32r VREG:r:f64=$XMMm         ', 'BP=0 MOD=REG W=0                  ', 'VCVTTSD2SI_rxmx', '           '];
T['VCVTTSD2SI REG:w:s32=$GPR32r VREG:r:f64=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128 W=0          ', 'VCVTTSD2SI_rxmx', '           '];
T['VCVTTSD2SI REG:w:s32=$GPR32r MEM:r:f64=$mem64         ', 'BP=0 MOD=MEM W=0                  ', 'VCVTTSD2SI_rxmx', 'tuple=T1F64'];
T['VCVTTSD2SI REG:w:s64=$GPR64r VREG:r:f64=$XMMm         ', 'BP=0 MOD=REG MODE=64 W=1          ', 'VCVTTSD2SI_rxmx', '           '];
T['VCVTTSD2SI REG:w:s64=$GPR64r VREG:r:f64=$XMMm SAE=$SAE', 'BP=1 MOD=REG MODE=64 RVL=128 W=1  ', 'VCVTTSD2SI_rxmx', '           '];
T['VCVTTSD2SI REG:w:s64=$GPR64r MEM:r:f64=$mem64         ', 'BP=0 MOD=MEM MODE=64 W=1          ', 'VCVTTSD2SI_rxmx', 'tuple=T1F64'];
T['VCVTTSD2SI REG:w:s32=$GPR32r VREG:r:f64=$XMMm         ', 'BP=0 MOD=REG MODE=NO64 W=1        ', 'VCVTTSD2SI_rxmx', '           '];
T['VCVTTSD2SI REG:w:s32=$GPR32r VREG:r:f64=$XMMm SAE=$SAE', 'BP=1 MOD=REG MODE=NO64 RVL=128 W=1', 'VCVTTSD2SI_rxmx', '           '];
T['VCVTTSD2SI REG:w:s32=$GPR32r MEM:r:f64=$mem64         ', 'BP=0 MOD=MEM MODE=NO64 W=1        ', 'VCVTTSD2SI_rxmx', 'tuple=T1F64'];

# CVTTSS2SI-Convert with Truncation Scalar Single-Precision Floating-Point Value to Integer.
T['VCVTTSS2SI REG:w:s32=$GPR32r REG:r:f32=$XMMm          ', 'MOD=REG MODE=64 W=0               ', 'VCVTTSS2SI_rxmd', '           '];
T['VCVTTSS2SI REG:w:s64=$GPR64r REG:r:f32=$XMMm          ', 'MOD=REG MODE=64 W=1               ', 'VCVTTSS2SI_rxmd', '           '];
T['VCVTTSS2SI REG:w:s32=$GPR32r MEM:r:f32=$mem32         ', 'MOD=MEM MODE=64 W=0               ', 'VCVTTSS2SI_rxmd', '           '];
T['VCVTTSS2SI REG:w:s64=$GPR64r MEM:r:f32=$mem32         ', 'MOD=MEM MODE=64 W=1               ', 'VCVTTSS2SI_rxmd', '           '];
T['VCVTTSS2SI REG:w:s32=$GPR32r REG:r:f32=$XMMm          ', 'MOD=REG MODE=NO64                 ', 'VCVTTSS2SI_rxmd', '           '];
T['VCVTTSS2SI REG:w:s32=$GPR32r MEM:r:f32=$mem32         ', 'MOD=MEM MODE=NO64                 ', 'VCVTTSS2SI_rxmd', '           '];
T['VCVTTSS2SI REG:w:s32=$GPR32r VREG:r:f32=$XMMm         ', 'BP=0 MOD=REG W=0                  ', 'VCVTTSS2SI_rxmx', '           '];
T['VCVTTSS2SI REG:w:s32=$GPR32r VREG:r:f32=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128 W=0          ', 'VCVTTSS2SI_rxmx', '           '];
T['VCVTTSS2SI REG:w:s32=$GPR32r MEM:r:f32=$mem32         ', 'BP=0 MOD=MEM W=0                  ', 'VCVTTSS2SI_rxmx', 'tuple=T1F32'];
T['VCVTTSS2SI REG:w:s64=$GPR64r VREG:r:f32=$XMMm         ', 'BP=0 MOD=REG MODE=64 W=1          ', 'VCVTTSS2SI_rxmx', '           '];
T['VCVTTSS2SI REG:w:s64=$GPR64r VREG:r:f32=$XMMm SAE=$SAE', 'BP=1 MOD=REG MODE=64 RVL=128 W=1  ', 'VCVTTSS2SI_rxmx', '           '];
T['VCVTTSS2SI REG:w:s64=$GPR64r MEM:r:f32=$mem32         ', 'BP=0 MOD=MEM MODE=64 W=1          ', 'VCVTTSS2SI_rxmx', 'tuple=T1F32'];
T['VCVTTSS2SI REG:w:s32=$GPR32r VREG:r:f32=$XMMm         ', 'BP=0 MOD=REG MODE=NO64 W=1        ', 'VCVTTSS2SI_rxmx', '           '];
T['VCVTTSS2SI REG:w:s32=$GPR32r VREG:r:f32=$XMMm SAE=$SAE', 'BP=1 MOD=REG MODE=NO64 RVL=128 W=1', 'VCVTTSS2SI_rxmx', '           '];
T['VCVTTSS2SI REG:w:s32=$GPR32r MEM:r:f32=$mem32         ', 'BP=0 MOD=MEM MODE=NO64 W=1        ', 'VCVTTSS2SI_rxmx', 'tuple=T1F32'];

# DIVPD-Divide Packed Double-Precision Floating-Point Values.
T['VDIVPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VDIVPD_ronomo', '        '];
T['VDIVPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VDIVPD_ronomo', 'tuple=FV'];
T['VDIVPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VDIVPD_rxnxmx', '        '];
T['VDIVPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VDIVPD_rxnxmx', '        '];
T['VDIVPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VDIVPD_rxnxmx', '        '];
T['VDIVPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VDIVPD_rxnxmx', '        '];
T['VDIVPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VDIVPD_rynymy', '        '];
T['VDIVPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VDIVPD_rynymy', 'tuple=FV'];
T['VDIVPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VDIVPD_rznzmz', '        '];
T['VDIVPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VDIVPD_rznzmz', '        '];
T['VDIVPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VDIVPD_rznzmz', 'tuple=FV'];

# DIVPS-Divide Packed Single-Precision Floating-Point Values.
T['VDIVPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VDIVPS_ronomo', '        '];
T['VDIVPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VDIVPS_ronomo', 'tuple=FV'];
T['VDIVPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VDIVPS_rxnxmx', '        '];
T['VDIVPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VDIVPS_rxnxmx', '        '];
T['VDIVPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VDIVPS_rxnxmx', '        '];
T['VDIVPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VDIVPS_rxnxmx', '        '];
T['VDIVPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VDIVPS_rynymy', '        '];
T['VDIVPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VDIVPS_rynymy', 'tuple=FV'];
T['VDIVPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VDIVPS_rznzmz', '        '];
T['VDIVPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VDIVPS_rznzmz', '        '];
T['VDIVPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VDIVPS_rznzmz', 'tuple=FV'];

# DIVSD-Divide Scalar Double-Precision Floating-Point Value.
T['VDIVSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VDIVSD_ronomq', '         '];
T['VDIVSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VDIVSD_ronomq', '         '];
T['VDIVSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VDIVSD_ronomx', '         '];
T['VDIVSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VDIVSD_ronomx', '         '];
T['VDIVSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VDIVSD_ronomx', 'tuple=T1S'];

# DIVSS-Divide Scalar Single-Precision Floating-Point Values.
T['VDIVSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn REG:r:f32=$XMMm                         ', 'MOD=REG             ', 'VDIVSS_ronomd', '         '];
T['VDIVSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem32                        ', 'MOD=MEM             ', 'VDIVSS_ronomd', '         '];
T['VDIVSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VDIVSS_ronomx', '         '];
T['VDIVSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VDIVSS_ronomx', '         '];
T['VDIVSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VDIVSS_ronomx', 'tuple=T1S'];

# DPPD-Dot Product of Packed Double Precision Floating-Point Values.
T['VDPPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'VDPPD', ''];
T['VDPPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'VDPPD', ''];

# DPPS-Dot Product of Packed Single Precision Floating-Point Values.
T['VDPPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm  IMM:u8=$uimm8', 'MOD=REG VL=128', 'VDPPS', ''];
T['VDPPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm  IMM:u8=$uimm8', 'MOD=REG VL=256', 'VDPPS', ''];
T['VDPPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128 IMM:u8=$uimm8', 'MOD=MEM VL=128', 'VDPPS', ''];
T['VDPPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256 IMM:u8=$uimm8', 'MOD=MEM VL=256', 'VDPPS', ''];

# EXTRACTPS-Extract Packed Floating-Point Values.
T['VEXTRACTPS REG:w=$GPR32m     VREG:r:f32=$XMMr IMM:u8=$index', 'MOD=REG', 'VEXTRACTPS_mdroub_1', '         '];
T['VEXTRACTPS MEM:w:f32=$mem32  VREG:r:f32=$XMMr IMM:u8=$index', 'MOD=MEM', 'VEXTRACTPS_mdroub_1', '         '];
T['VEXTRACTPS REG:w:f32=$GPR32m VREG:r:f32=$XMMr IMM:u8=$index', 'MOD=REG', 'VEXTRACTPS_mdroub_2', '         '];
T['VEXTRACTPS MEM:w:f32=$mem32  VREG:r:f32=$XMMr IMM:u8=$index', 'MOD=MEM', 'VEXTRACTPS_mdroub_2', 'tuple=T1S'];

# GF2P8AFFINEINVQB-Galois Field Affine Transformation Inverse.
T['GF2P8AFFINEINVQB  VREG:rw:u8=$XMMr VREG:r:u64=$XMMm  IMM:u8=$uimm8                                               ', 'MOD=REG       ', 'GF2P8AFFINEINVQB          ', '        '];
T['GF2P8AFFINEINVQB  VREG:rw:u8=$XMMr MEM:r:u64=$mem128 IMM:u8=$uimm8                                               ', 'MOD=MEM       ', 'GF2P8AFFINEINVQB          ', '        '];
T['VGF2P8AFFINEINVQB VREG:w:u8=$XMMr  PREG:r:Z=$Kk      VREG:r:u8=$XMMn   VREG:r:u64=$XMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG  ', 'VGF2P8AFFINEINVQB_ronomoub', '        '];
T['VGF2P8AFFINEINVQB VREG:w:u8=$XMMr  PREG:r:Z=$Kk      VREG:r:u8=$XMMn   MEM:r:u64=$mem128 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM       ', 'VGF2P8AFFINEINVQB_ronomoub', 'tuple=FV'];
T['VGF2P8AFFINEINVQB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn   VREG:r:u64=$XMMm  IMM:u8=$uimm8                             ', 'MOD=REG VL=128', 'VGF2P8AFFINEINVQB_rxnxmxub', '        '];
T['VGF2P8AFFINEINVQB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn   VREG:r:u64=$YMMm  IMM:u8=$uimm8                             ', 'MOD=REG VL=256', 'VGF2P8AFFINEINVQB_rxnxmxub', '        '];
T['VGF2P8AFFINEINVQB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn   MEM:r:u64=$mem128 IMM:u8=$uimm8                             ', 'MOD=MEM VL=128', 'VGF2P8AFFINEINVQB_rxnxmxub', '        '];
T['VGF2P8AFFINEINVQB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn   MEM:r:u64=$mem256 IMM:u8=$uimm8                             ', 'MOD=MEM VL=256', 'VGF2P8AFFINEINVQB_rxnxmxub', '        '];
T['VGF2P8AFFINEINVQB VREG:w:u8=$YMMr  PREG:r:Z=$Kk      VREG:r:u8=$YMMn   VREG:r:u64=$YMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG  ', 'VGF2P8AFFINEINVQB_rynymyub', '        '];
T['VGF2P8AFFINEINVQB VREG:w:u8=$YMMr  PREG:r:Z=$Kk      VREG:r:u8=$YMMn   MEM:r:u64=$mem256 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM       ', 'VGF2P8AFFINEINVQB_rynymyub', 'tuple=FV'];
T['VGF2P8AFFINEINVQB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk      VREG:r:u8=$ZMMn   VREG:r:u64=$ZMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG  ', 'VGF2P8AFFINEINVQB_rznzmzub', '        '];
T['VGF2P8AFFINEINVQB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk      VREG:r:u8=$ZMMn   MEM:r:u64=$mem512 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM       ', 'VGF2P8AFFINEINVQB_rznzmzub', 'tuple=FV'];

# GF2P8AFFINEQB-Galois Field Affine Transformation.
T['GF2P8AFFINEQB  VREG:rw:u8=$XMMr VREG:r:u64=$XMMm  IMM:u8=$uimm8                                               ', 'MOD=REG       ', 'GF2P8AFFINEQB          ', '        '];
T['GF2P8AFFINEQB  VREG:rw:u8=$XMMr MEM:r:u64=$mem128 IMM:u8=$uimm8                                               ', 'MOD=MEM       ', 'GF2P8AFFINEQB          ', '        '];
T['VGF2P8AFFINEQB VREG:w:u8=$XMMr  PREG:r:Z=$Kk      VREG:r:u8=$XMMn   VREG:r:u64=$XMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG  ', 'VGF2P8AFFINEQB_ronomoub', '        '];
T['VGF2P8AFFINEQB VREG:w:u8=$XMMr  PREG:r:Z=$Kk      VREG:r:u8=$XMMn   MEM:r:u64=$mem128 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM       ', 'VGF2P8AFFINEQB_ronomoub', 'tuple=FV'];
T['VGF2P8AFFINEQB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn   VREG:r:u64=$XMMm  IMM:u8=$uimm8                             ', 'MOD=REG VL=128', 'VGF2P8AFFINEQB_rxnxmxub', '        '];
T['VGF2P8AFFINEQB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn   VREG:r:u64=$YMMm  IMM:u8=$uimm8                             ', 'MOD=REG VL=256', 'VGF2P8AFFINEQB_rxnxmxub', '        '];
T['VGF2P8AFFINEQB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn   MEM:r:u64=$mem128 IMM:u8=$uimm8                             ', 'MOD=MEM VL=128', 'VGF2P8AFFINEQB_rxnxmxub', '        '];
T['VGF2P8AFFINEQB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn   MEM:r:u64=$mem256 IMM:u8=$uimm8                             ', 'MOD=MEM VL=256', 'VGF2P8AFFINEQB_rxnxmxub', '        '];
T['VGF2P8AFFINEQB VREG:w:u8=$YMMr  PREG:r:Z=$Kk      VREG:r:u8=$YMMn   VREG:r:u64=$YMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG  ', 'VGF2P8AFFINEQB_rynymyub', '        '];
T['VGF2P8AFFINEQB VREG:w:u8=$YMMr  PREG:r:Z=$Kk      VREG:r:u8=$YMMn   MEM:r:u64=$mem256 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM       ', 'VGF2P8AFFINEQB_rynymyub', 'tuple=FV'];
T['VGF2P8AFFINEQB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk      VREG:r:u8=$ZMMn   VREG:r:u64=$ZMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG  ', 'VGF2P8AFFINEQB_rznzmzub', '        '];
T['VGF2P8AFFINEQB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk      VREG:r:u8=$ZMMn   MEM:r:u64=$mem512 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM       ', 'VGF2P8AFFINEQB_rznzmzub', 'tuple=FV'];

# GF2P8MULB-Galois Field Multiply Bytes.
T['GF2P8MULB  VREG:rw:u8=$XMMr VREG:r:u8=$XMMm                                   ', 'MOD=REG       ', 'GF2P8MULB        ', '         '];
T['GF2P8MULB  VREG:rw:u8=$XMMr MEM:r:u8=$mem128                                  ', 'MOD=MEM       ', 'GF2P8MULB        ', '         '];
T['VGF2P8MULB VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn  VREG:r:u8=$XMMm ', 'MOD=REG       ', 'VGF2P8MULB_ronomo', '         '];
T['VGF2P8MULB VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn  MEM:r:u8=$mem128', 'MOD=MEM       ', 'VGF2P8MULB_ronomo', 'tuple=FVM'];
T['VGF2P8MULB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  VREG:r:u8=$XMMm                  ', 'MOD=REG VL=128', 'VGF2P8MULB_rxnxmx', '         '];
T['VGF2P8MULB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  VREG:r:u8=$YMMm                  ', 'MOD=REG VL=256', 'VGF2P8MULB_rxnxmx', '         '];
T['VGF2P8MULB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  MEM:r:u8=$mem128                 ', 'MOD=MEM VL=128', 'VGF2P8MULB_rxnxmx', '         '];
T['VGF2P8MULB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  MEM:r:u8=$mem256                 ', 'MOD=MEM VL=256', 'VGF2P8MULB_rxnxmx', '         '];
T['VGF2P8MULB VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn  VREG:r:u8=$YMMm ', 'MOD=REG       ', 'VGF2P8MULB_rynymy', '         '];
T['VGF2P8MULB VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn  MEM:r:u8=$mem256', 'MOD=MEM       ', 'VGF2P8MULB_rynymy', 'tuple=FVM'];
T['VGF2P8MULB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn  VREG:r:u8=$ZMMm ', 'MOD=REG       ', 'VGF2P8MULB_rznzmz', '         '];
T['VGF2P8MULB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn  MEM:r:u8=$mem512', 'MOD=MEM       ', 'VGF2P8MULB_rznzmz', 'tuple=FVM'];

# HADDPD-Packed Double-FP Horizontal Add.
T['VHADDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm ', 'MOD=REG VL=128', 'VHADDPD', ''];
T['VHADDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm ', 'MOD=REG VL=256', 'VHADDPD', ''];
T['VHADDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128', 'MOD=MEM VL=128', 'VHADDPD', ''];
T['VHADDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256', 'MOD=MEM VL=256', 'VHADDPD', ''];

# HADDPS-Packed Single-FP Horizontal Add.
T['VHADDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm ', 'MOD=REG VL=128', 'VHADDPS', ''];
T['VHADDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm ', 'MOD=REG VL=256', 'VHADDPS', ''];
T['VHADDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128', 'MOD=MEM VL=128', 'VHADDPS', ''];
T['VHADDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256', 'MOD=MEM VL=256', 'VHADDPS', ''];

# HSUBPD-Packed Double-FP Horizontal Subtract.
T['VHSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm ', 'MOD=REG VL=128', 'VHSUBPD', ''];
T['VHSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm ', 'MOD=REG VL=256', 'VHSUBPD', ''];
T['VHSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128', 'MOD=MEM VL=128', 'VHSUBPD', ''];
T['VHSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256', 'MOD=MEM VL=256', 'VHSUBPD', ''];

# HSUBPS-Packed Single-FP Horizontal Subtract.
T['VHSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm ', 'MOD=REG VL=128', 'VHSUBPS', ''];
T['VHSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm ', 'MOD=REG VL=256', 'VHSUBPS', ''];
T['VHSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128', 'MOD=MEM VL=128', 'VHSUBPS', ''];
T['VHSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256', 'MOD=MEM VL=256', 'VHSUBPS', ''];

# INSERTPS-Insert Scalar Single-Precision Floating-Point Value.
T['VINSERTPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm IMM:u8=$index', 'MOD=REG', 'VINSERTPS_ronomxub_1', '           '];
T['VINSERTPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem32 IMM:u8=$index', 'MOD=MEM', 'VINSERTPS_ronomxub_1', '           '];
T['VINSERTPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm IMM:u8=$index', 'MOD=REG', 'VINSERTPS_ronomxub_2', '           '];
T['VINSERTPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem32 IMM:u8=$index', 'MOD=MEM', 'VINSERTPS_ronomxub_2', 'tuple=T1F32'];

# KADDW/KADDB/KADDQ/KADDD-ADD Two Masks.
T['KADDB REG:w=$Kr REG:r=$Kn REG:r=$Km', 'P66=1', 'KADDx_V_1', ''];
T['KADDW REG:w=$Kr REG:r=$Kn REG:r=$Km', 'P66=0', 'KADDx_V_1', ''];
T['KADDD REG:w=$Kr REG:r=$Kn REG:r=$Km', 'P66=1', 'KADDx_V_2', ''];
T['KADDQ REG:w=$Kr REG:r=$Kn REG:r=$Km', 'P66=0', 'KADDx_V_2', ''];

# KANDNW/KANDNB/KANDNQ/KANDND-Bitwise Logical AND NOT Masks.
T['KANDNB REG:w=$Kr REG:r=$Kn REG:r=$Km', 'NONE ', 'KANDNB', ''];
T['KANDNW REG:w=$Kr REG:r=$Kn REG:r=$Km', 'NONE ', 'KANDNW', ''];
T['KANDND REG:w=$Kr REG:r=$Kn REG:r=$Km', 'P66=1', 'KANDNx', ''];
T['KANDNQ REG:w=$Kr REG:r=$Kn REG:r=$Km', 'P66=0', 'KANDNx', ''];

# KANDW/KANDB/KANDQ/KANDD-Bitwise Logical AND Masks.
T['KANDB REG:w=$Kr REG:r=$Kn REG:r=$Km', 'NONE ', 'KANDB', ''];
T['KANDW REG:w=$Kr REG:r=$Kn REG:r=$Km', 'NONE ', 'KANDW', ''];
T['KANDD REG:w=$Kr REG:r=$Kn REG:r=$Km', 'P66=1', 'KANDx', ''];
T['KANDQ REG:w=$Kr REG:r=$Kn REG:r=$Km', 'P66=0', 'KANDx', ''];

# KMOVW/KMOVB/KMOVQ/KMOVD-Move from and to Mask Registers.
T['KMOVB REG:w=$Kr         REG:r:u8=$Km     ', 'MOD=REG                ', 'KMOVB_mb_1', ''];
T['KMOVB REG:w=$Kr         MEM:r:u8=$mem8   ', 'MOD=MEM                ', 'KMOVB_mb_1', ''];
T['KMOVB MEM:w:u8=$mem8    REG:r=$Kr        ', 'NONE                   ', 'KMOVB_mb_2', ''];
T['KMOVB REG:w=$Kr         REG:r:u32=$GPR32m', 'NONE                   ', 'KMOVB_md  ', ''];
T['KMOVB REG:w:u32=$GPR32r REG:r=$Km        ', 'NONE                   ', 'KMOVB_rd  ', ''];
T['KMOVW REG:w=$Kr         REG:r:u32=$GPR32m', 'KNC=0 MODE=64          ', 'KMOVW_md  ', ''];
T['KMOVW REG:w=$Kr         REG:r:u32=$GPR32m', 'MODE=NO64              ', 'KMOVW_md  ', ''];
T['KMOVW REG:w=$Kr         REG:r:u16=$Km    ', 'KNC=0 MOD=REG MODE=64  ', 'KMOVW_mw_1', ''];
T['KMOVW REG:w=$Kr         MEM:r:u16=$mem16 ', 'MOD=MEM MODE=64        ', 'KMOVW_mw_1', ''];
T['KMOVW REG:w=$Kr         REG:r:u16=$Km    ', 'MOD=REG MODE=NO64      ', 'KMOVW_mw_1', ''];
T['KMOVW REG:w=$Kr         MEM:r:u16=$mem16 ', 'MOD=MEM MODE=NO64      ', 'KMOVW_mw_1', ''];
T['KMOVW MEM:w:u16=$mem16  REG:r=$Kr        ', 'NONE                   ', 'KMOVW_mw_2', ''];
T['KMOVW REG:w:u32=$GPR32r REG:r=$Km        ', 'KNC=0 MODE=64          ', 'KMOVW_rd  ', ''];
T['KMOVW REG:w:u32=$GPR32r REG:r=$Km        ', 'MODE=NO64              ', 'KMOVW_rd  ', ''];
T['KMOVD REG:w=$Kr         REG:r:u32=$Km    ', 'MOD=REG P66=1          ', 'KMOVx_mx_1', ''];
T['KMOVD REG:w=$Kr         MEM:r:u32=$mem32 ', 'MOD=MEM P66=1          ', 'KMOVx_mx_1', ''];
T['KMOVQ REG:w=$Kr         REG:r:u64=$Km    ', 'MOD=REG MODE=64 P66=0  ', 'KMOVx_mx_1', ''];
T['KMOVQ REG:w=$Kr         MEM:r:u64=$mem64 ', 'MOD=MEM MODE=64 P66=0  ', 'KMOVx_mx_1', ''];
T['KMOVQ REG:w=$Kr         REG:r:u64=$Km    ', 'MOD=REG MODE=NO64 P66=0', 'KMOVx_mx_1', ''];
T['KMOVQ REG:w=$Kr         MEM:r:u64=$mem64 ', 'MOD=MEM MODE=NO64 P66=0', 'KMOVx_mx_1', ''];
T['KMOVD MEM:w:u32=$mem32  REG:r=$Kr        ', 'P66=1                  ', 'KMOVx_mx_2', ''];
T['KMOVQ MEM:w:u64=$mem64  REG:r=$Kr        ', 'P66=0                  ', 'KMOVx_mx_2', ''];
T['KMOVD REG:w=$Kr         REG:r:u32=$GPR32m', 'MODE=64 W=0            ', 'KMOVx_mx_3', ''];
T['KMOVD REG:w=$Kr         REG:r:u32=$GPR32m', 'MODE=NO64              ', 'KMOVx_mx_3', ''];
T['KMOVQ REG:w=$Kr         REG:r:u64=$GPR64m', 'MODE=64 W=1            ', 'KMOVx_mx_3', ''];
T['KMOVD REG:w:u32=$GPR32r REG:r=$Km        ', 'MODE=64 W=0            ', 'KMOVx_rx  ', ''];
T['KMOVD REG:w:u32=$GPR32r REG:r=$Km        ', 'MODE=NO64              ', 'KMOVx_rx  ', ''];
T['KMOVQ REG:w:u64=$GPR64r REG:r=$Km        ', 'MODE=64 W=1            ', 'KMOVx_rx  ', ''];

# KNOTW/KNOTB/KNOTQ/KNOTD-NOT Mask Register.
T['KNOTB REG:w=$Kr REG:r=$Km', 'NONE           ', 'KNOTB', ''];
T['KNOTW REG:w=$Kr REG:r=$Km', 'KNC=0 MODE=64  ', 'KNOTW', ''];
T['KNOTW REG:w=$Kr REG:r=$Km', 'MODE=NO64      ', 'KNOTW', ''];
T['KNOTD REG:w=$Kr REG:r=$Km', 'P66=1          ', 'KNOTx', ''];
T['KNOTQ REG:w=$Kr REG:r=$Km', 'MODE=64 P66=0  ', 'KNOTx', ''];
T['KNOTQ REG:w=$Kr REG:r=$Km', 'MODE=NO64 P66=0', 'KNOTx', ''];

# KORTESTW/KORTESTB/KORTESTQ/KORTESTD-OR Masks And Set Flags.
T['KORTESTB REG:r=$Kr REG:r=$Km', 'NONE           ', 'KORTESTB', ''];
T['KORTESTW REG:r=$Kr REG:r=$Km', 'KNC=0 MODE=64  ', 'KORTESTW', ''];
T['KORTESTW REG:r=$Kr REG:r=$Km', 'MODE=NO64      ', 'KORTESTW', ''];
T['KORTESTD REG:r=$Kr REG:r=$Km', 'P66=1          ', 'KORTESTx', ''];
T['KORTESTQ REG:r=$Kr REG:r=$Km', 'MODE=64 P66=0  ', 'KORTESTx', ''];
T['KORTESTQ REG:r=$Kr REG:r=$Km', 'MODE=NO64 P66=0', 'KORTESTx', ''];

# KORW/KORB/KORQ/KORD-Bitwise Logical OR Masks.
T['KORB REG:w=$Kr REG:r=$Kn REG:r=$Km', 'NONE ', 'KORB', ''];
T['KORW REG:w=$Kr REG:r=$Kn REG:r=$Km', 'NONE ', 'KORW', ''];
T['KORD REG:w=$Kr REG:r=$Kn REG:r=$Km', 'P66=1', 'KORx', ''];
T['KORQ REG:w=$Kr REG:r=$Kn REG:r=$Km', 'P66=0', 'KORx', ''];

# KSHIFTLW/KSHIFTLB/KSHIFTLQ/KSHIFTLD-Shift Left Mask Registers.
T['KSHIFTLB REG:w=$Kr REG:r=$Km SHIFT:r:u8=$shl', 'NONE', 'KSHIFTLB', ''];
T['KSHIFTLW REG:w=$Kr REG:r=$Km SHIFT:r:u8=$shl', 'NONE', 'KSHIFTLW', ''];
T['KSHIFTLD REG:w=$Kr REG:r=$Km SHIFT:r:u8=$shl', 'W=0 ', 'KSHIFTLx', ''];
T['KSHIFTLQ REG:w=$Kr REG:r=$Km SHIFT:r:u8=$shl', 'W=1 ', 'KSHIFTLx', ''];

# KSHIFTRW/KSHIFTRB/KSHIFTRQ/KSHIFTRD-Shift Right Mask Registers.
T['KSHIFTRB REG:w=$Kr REG:r=$Km SHIFT:r:u8=$shr', 'NONE', 'KSHIFTRB', ''];
T['KSHIFTRW REG:w=$Kr REG:r=$Km SHIFT:r:u8=$shr', 'NONE', 'KSHIFTRW', ''];
T['KSHIFTRD REG:w=$Kr REG:r=$Km SHIFT:r:u8=$shr', 'W=0 ', 'KSHIFTRx', ''];
T['KSHIFTRQ REG:w=$Kr REG:r=$Km SHIFT:r:u8=$shr', 'W=1 ', 'KSHIFTRx', ''];

# KTESTW/KTESTB/KTESTQ/KTESTD-Packed Bit Test Masks and Set Flags.
T['KTESTB REG:r=$Kr REG:r=$Km', 'P66=1', 'KTESTx_V_1', ''];
T['KTESTW REG:r=$Kr REG:r=$Km', 'P66=0', 'KTESTx_V_1', ''];
T['KTESTD REG:r=$Kr REG:r=$Km', 'P66=1', 'KTESTx_V_2', ''];
T['KTESTQ REG:r=$Kr REG:r=$Km', 'P66=0', 'KTESTx_V_2', ''];

# KUNPCKBW/KUNPCKWD/KUNPCKDQ-Unpack for Mask Registers.
T['KUNPCKBW REG:w=$Kr REG:r=$Kn REG:r=$Km', 'NONE', 'KUNPCKBW', ''];
T['KUNPCKDQ REG:w=$Kr REG:r=$Kn REG:r=$Km', 'W=1 ', 'KUNPCKxx', ''];
T['KUNPCKWD REG:w=$Kr REG:r=$Kn REG:r=$Km', 'W=0 ', 'KUNPCKxx', ''];

# KXNORW/KXNORB/KXNORQ/KXNORD-Bitwise Logical XNOR Masks.
T['KXNORB REG:w=$Kr REG:r=$Kn REG:r=$Km', 'NONE ', 'KXNORB', ''];
T['KXNORW REG:w=$Kr REG:r=$Kn REG:r=$Km', 'NONE ', 'KXNORW', ''];
T['KXNORD REG:w=$Kr REG:r=$Kn REG:r=$Km', 'P66=1', 'KXNORx', ''];
T['KXNORQ REG:w=$Kr REG:r=$Kn REG:r=$Km', 'P66=0', 'KXNORx', ''];

# KXORW/KXORB/KXORQ/KXORD-Bitwise Logical XOR Masks.
T['KXORB REG:w=$Kr REG:r=$Kn REG:r=$Km', 'NONE ', 'KXORB', ''];
T['KXORW REG:w=$Kr REG:r=$Kn REG:r=$Km', 'NONE ', 'KXORW', ''];
T['KXORD REG:w=$Kr REG:r=$Kn REG:r=$Km', 'P66=1', 'KXORx', ''];
T['KXORQ REG:w=$Kr REG:r=$Kn REG:r=$Km', 'P66=0', 'KXORx', ''];

# LDDQU-Load Unaligned Integer 128 Bits.
T['VLDDQU VREG:w:s32=$XMMr MEM:r:s32=$mem128', 'VL=128', 'VLDDQU', ''];
T['VLDDQU VREG:w:s32=$YMMr MEM:r:s32=$mem256', 'VL=256', 'VLDDQU', ''];

# LDMXCSR-Load MXCSR Register.
T['VLDMXCSR MEM:r:s32=$mem32 REG:SUPP:w=MXCSR', 'NONE', 'VLDMXCSR', ''];

# MASKMOVDQU-Store Selected Bytes of Double Quadword.
T['VMASKMOVDQU VREG:r:u8=$XMMr VREG:r:u8=$XMMm MEM:SUPP:w:u8={SEG:r=DS BASE:r=$DIa SZ=128}', 'NONE', 'VMASKMOVDQU', ''];

# MAXPD-Maximum of Packed Double-Precision Floating-Point Values.
T['VMAXPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VMAXPD_ronomo', '        '];
T['VMAXPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VMAXPD_ronomo', 'tuple=FV'];
T['VMAXPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VMAXPD_rxnxmx', '        '];
T['VMAXPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VMAXPD_rxnxmx', '        '];
T['VMAXPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VMAXPD_rxnxmx', '        '];
T['VMAXPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VMAXPD_rxnxmx', '        '];
T['VMAXPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VMAXPD_rynymy', '        '];
T['VMAXPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VMAXPD_rynymy', 'tuple=FV'];
T['VMAXPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VMAXPD_rznzmz', '        '];
T['VMAXPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VMAXPD_rznzmz', '        '];
T['VMAXPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VMAXPD_rznzmz', 'tuple=FV'];

# MAXPS-Maximum of Packed Single-Precision Floating-Point Values.
T['VMAXPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VMAXPS_ronomo', '        '];
T['VMAXPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VMAXPS_ronomo', 'tuple=FV'];
T['VMAXPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VMAXPS_rxnxmx', '        '];
T['VMAXPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VMAXPS_rxnxmx', '        '];
T['VMAXPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VMAXPS_rxnxmx', '        '];
T['VMAXPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VMAXPS_rxnxmx', '        '];
T['VMAXPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VMAXPS_rynymy', '        '];
T['VMAXPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VMAXPS_rynymy', 'tuple=FV'];
T['VMAXPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VMAXPS_rznzmz', '        '];
T['VMAXPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VMAXPS_rznzmz', '        '];
T['VMAXPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VMAXPS_rznzmz', 'tuple=FV'];

# MAXSD-Return Maximum Scalar Double-Precision Floating-Point Value.
T['VMAXSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn REG:r:f64=$XMMm                           ', 'MOD=REG             ', 'VMAXSD_ronomq', '         '];
T['VMAXSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem64                          ', 'MOD=MEM             ', 'VMAXSD_ronomq', '         '];
T['VMAXSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm         ', 'BP=0 MOD=REG        ', 'VMAXSD_ronomx', '         '];
T['VMAXSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128', 'VMAXSD_ronomx', '         '];
T['VMAXSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn MEM:r:f64=$mem64         ', 'BP=0 MOD=MEM        ', 'VMAXSD_ronomx', 'tuple=T1S'];

# MAXSS-Return Maximum Scalar Single-Precision Floating-Point Value.
T['VMAXSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn REG:r:f32=$XMMm                           ', 'MOD=REG             ', 'VMAXSS_ronomd', '         '];
T['VMAXSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem32                          ', 'MOD=MEM             ', 'VMAXSS_ronomd', '         '];
T['VMAXSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm         ', 'BP=0 MOD=REG        ', 'VMAXSS_ronomx', '         '];
T['VMAXSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128', 'VMAXSS_ronomx', '         '];
T['VMAXSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn MEM:r:f32=$mem32         ', 'BP=0 MOD=MEM        ', 'VMAXSS_ronomx', 'tuple=T1S'];

# MINPD-Minimum of Packed Double-Precision Floating-Point Values.
T['VMINPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VMINPD_ronomo', '        '];
T['VMINPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VMINPD_ronomo', 'tuple=FV'];
T['VMINPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VMINPD_rxnxmx', '        '];
T['VMINPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VMINPD_rxnxmx', '        '];
T['VMINPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VMINPD_rxnxmx', '        '];
T['VMINPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VMINPD_rxnxmx', '        '];
T['VMINPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VMINPD_rynymy', '        '];
T['VMINPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VMINPD_rynymy', 'tuple=FV'];
T['VMINPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VMINPD_rznzmz', '        '];
T['VMINPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VMINPD_rznzmz', '        '];
T['VMINPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VMINPD_rznzmz', 'tuple=FV'];

# MINPS-Minimum of Packed Single-Precision Floating-Point Values.
T['VMINPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VMINPS_ronomo', '        '];
T['VMINPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VMINPS_ronomo', 'tuple=FV'];
T['VMINPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VMINPS_rxnxmx', '        '];
T['VMINPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VMINPS_rxnxmx', '        '];
T['VMINPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VMINPS_rxnxmx', '        '];
T['VMINPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VMINPS_rxnxmx', '        '];
T['VMINPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VMINPS_rynymy', '        '];
T['VMINPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VMINPS_rynymy', 'tuple=FV'];
T['VMINPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VMINPS_rznzmz', '        '];
T['VMINPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VMINPS_rznzmz', '        '];
T['VMINPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VMINPS_rznzmz', 'tuple=FV'];

# MINSD-Return Minimum Scalar Double-Precision Floating-Point Value.
T['VMINSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn REG:r:f64=$XMMm                           ', 'MOD=REG             ', 'VMINSD_ronomq', '         '];
T['VMINSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem64                          ', 'MOD=MEM             ', 'VMINSD_ronomq', '         '];
T['VMINSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm         ', 'BP=0 MOD=REG        ', 'VMINSD_ronomx', '         '];
T['VMINSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128', 'VMINSD_ronomx', '         '];
T['VMINSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn MEM:r:f64=$mem64         ', 'BP=0 MOD=MEM        ', 'VMINSD_ronomx', 'tuple=T1S'];

# MINSS-Return Minimum Scalar Single-Precision Floating-Point Value.
T['VMINSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn REG:r:f32=$XMMm                           ', 'MOD=REG             ', 'VMINSS_ronomd', '         '];
T['VMINSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem32                          ', 'MOD=MEM             ', 'VMINSS_ronomd', '         '];
T['VMINSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm         ', 'BP=0 MOD=REG        ', 'VMINSS_ronomx', '         '];
T['VMINSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128', 'VMINSS_ronomx', '         '];
T['VMINSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn MEM:r:f32=$mem32         ', 'BP=0 MOD=MEM        ', 'VMINSS_ronomx', 'tuple=T1S'];

# MOVAPD-Move Aligned Packed Double-Precision Floating-Point Values.
T['VMOVAPD VREG:w:f64=$XMMm  PREG:r:Z=$Kk      VREG:r:f64=$XMMr ', 'NONE          ', 'VMOVAPD_moro_1', '         '];
T['VMOVAPD MEM:w:f64=$mem128 PREG:r:Z=$Kk      VREG:r:f64=$XMMr ', 'NONE          ', 'VMOVAPD_moro_2', 'tuple=FVM'];
T['VMOVAPD VREG:w:f64=$XMMm  VREG:r:f64=$XMMr                   ', 'MOD=REG VL=128', 'VMOVAPD_mxrx  ', '         '];
T['VMOVAPD VREG:w:f64=$YMMm  VREG:r:f64=$YMMr                   ', 'MOD=REG VL=256', 'VMOVAPD_mxrx  ', '         '];
T['VMOVAPD MEM:w:f64=$mem128 VREG:r:f64=$XMMr                   ', 'MOD=MEM VL=128', 'VMOVAPD_mxrx  ', '         '];
T['VMOVAPD MEM:w:f64=$mem256 VREG:r:f64=$YMMr                   ', 'MOD=MEM VL=256', 'VMOVAPD_mxrx  ', '         '];
T['VMOVAPD VREG:w:f64=$YMMm  PREG:r:Z=$Kk      VREG:r:f64=$YMMr ', 'NONE          ', 'VMOVAPD_myry_1', '         '];
T['VMOVAPD MEM:w:f64=$mem256 PREG:r:Z=$Kk      VREG:r:f64=$YMMr ', 'NONE          ', 'VMOVAPD_myry_2', 'tuple=FVM'];
T['VMOVAPD VREG:w:f64=$ZMMm  PREG:r:Z=$Kk      VREG:r:f64=$ZMMr ', 'NONE          ', 'VMOVAPD_mzrz_1', '         '];
T['VMOVAPD MEM:w:f64=$mem512 PREG:r:Z=$Kk      VREG:r:f64=$ZMMr ', 'NONE          ', 'VMOVAPD_mzrz_2', 'tuple=FVM'];
T['VMOVAPD VREG:w:f64=$XMMr  PREG:r:Z=$Kk      VREG:r:f64=$XMMm ', 'NONE          ', 'VMOVAPD_romo_1', '         '];
T['VMOVAPD VREG:w:f64=$XMMr  PREG:r:Z=$Kk      MEM:r:f64=$mem128', 'NONE          ', 'VMOVAPD_romo_2', 'tuple=FVM'];
T['VMOVAPD VREG:w:f64=$XMMr  VREG:r:f64=$XMMm                   ', 'MOD=REG VL=128', 'VMOVAPD_rxmx  ', '         '];
T['VMOVAPD VREG:w:f64=$YMMr  VREG:r:f64=$YMMm                   ', 'MOD=REG VL=256', 'VMOVAPD_rxmx  ', '         '];
T['VMOVAPD VREG:w:f64=$XMMr  MEM:r:f64=$mem128                  ', 'MOD=MEM VL=128', 'VMOVAPD_rxmx  ', '         '];
T['VMOVAPD VREG:w:f64=$YMMr  MEM:r:f64=$mem256                  ', 'MOD=MEM VL=256', 'VMOVAPD_rxmx  ', '         '];
T['VMOVAPD VREG:w:f64=$YMMr  PREG:r:Z=$Kk      VREG:r:f64=$YMMm ', 'NONE          ', 'VMOVAPD_rymy_1', '         '];
T['VMOVAPD VREG:w:f64=$YMMr  PREG:r:Z=$Kk      MEM:r:f64=$mem256', 'NONE          ', 'VMOVAPD_rymy_2', 'tuple=FVM'];
T['VMOVAPD VREG:w:f64=$ZMMr  PREG:r:Z=$Kk      VREG:r:f64=$ZMMm ', 'NONE          ', 'VMOVAPD_rzmz_1', '         '];
T['VMOVAPD VREG:w:f64=$ZMMr  PREG:r:Z=$Kk      MEM:r:f64=$mem512', 'NONE          ', 'VMOVAPD_rzmz_2', 'tuple=FVM'];

# MOVAPS-Move Aligned Packed Single-Precision Floating-Point Values.
T['VMOVAPS VREG:w:f32=$XMMm  PREG:r:Z=$Kk      VREG:r:f32=$XMMr ', 'NONE          ', 'VMOVAPS_moro_1', '         '];
T['VMOVAPS MEM:w:f32=$mem128 PREG:r:Z=$Kk      VREG:r:f32=$XMMr ', 'NONE          ', 'VMOVAPS_moro_2', 'tuple=FVM'];
T['VMOVAPS VREG:w:f32=$XMMm  VREG:r:f32=$XMMr                   ', 'MOD=REG VL=128', 'VMOVAPS_mxrx  ', '         '];
T['VMOVAPS VREG:w:f32=$YMMm  VREG:r:f32=$YMMr                   ', 'MOD=REG VL=256', 'VMOVAPS_mxrx  ', '         '];
T['VMOVAPS MEM:w:f32=$mem128 VREG:r:f32=$XMMr                   ', 'MOD=MEM VL=128', 'VMOVAPS_mxrx  ', '         '];
T['VMOVAPS MEM:w:f32=$mem256 VREG:r:f32=$YMMr                   ', 'MOD=MEM VL=256', 'VMOVAPS_mxrx  ', '         '];
T['VMOVAPS VREG:w:f32=$YMMm  PREG:r:Z=$Kk      VREG:r:f32=$YMMr ', 'NONE          ', 'VMOVAPS_myry_1', '         '];
T['VMOVAPS MEM:w:f32=$mem256 PREG:r:Z=$Kk      VREG:r:f32=$YMMr ', 'NONE          ', 'VMOVAPS_myry_2', 'tuple=FVM'];
T['VMOVAPS VREG:w:f32=$ZMMm  PREG:r:Z=$Kk      VREG:r:f32=$ZMMr ', 'NONE          ', 'VMOVAPS_mzrz_1', '         '];
T['VMOVAPS MEM:w:f32=$mem512 PREG:r:Z=$Kk      VREG:r:f32=$ZMMr ', 'NONE          ', 'VMOVAPS_mzrz_2', 'tuple=FVM'];
T['VMOVAPS VREG:w:f32=$XMMr  PREG:r:Z=$Kk      VREG:r:f32=$XMMm ', 'NONE          ', 'VMOVAPS_romo_1', '         '];
T['VMOVAPS VREG:w:f32=$XMMr  PREG:r:Z=$Kk      MEM:r:f32=$mem128', 'NONE          ', 'VMOVAPS_romo_2', 'tuple=FVM'];
T['VMOVAPS VREG:w:f32=$XMMr  VREG:r:f32=$XMMm                   ', 'MOD=REG VL=128', 'VMOVAPS_rxmx  ', '         '];
T['VMOVAPS VREG:w:f32=$YMMr  VREG:r:f32=$YMMm                   ', 'MOD=REG VL=256', 'VMOVAPS_rxmx  ', '         '];
T['VMOVAPS VREG:w:f32=$XMMr  MEM:r:f32=$mem128                  ', 'MOD=MEM VL=128', 'VMOVAPS_rxmx  ', '         '];
T['VMOVAPS VREG:w:f32=$YMMr  MEM:r:f32=$mem256                  ', 'MOD=MEM VL=256', 'VMOVAPS_rxmx  ', '         '];
T['VMOVAPS VREG:w:f32=$YMMr  PREG:r:Z=$Kk      VREG:r:f32=$YMMm ', 'NONE          ', 'VMOVAPS_rymy_1', '         '];
T['VMOVAPS VREG:w:f32=$YMMr  PREG:r:Z=$Kk      MEM:r:f32=$mem256', 'NONE          ', 'VMOVAPS_rymy_2', 'tuple=FVM'];
T['VMOVAPS VREG:w:f32=$ZMMr  PREG:r:Z=$Kk      VREG:r:f32=$ZMMm ', 'NONE          ', 'VMOVAPS_rzmz_1', '         '];
T['VMOVAPS VREG:w:f32=$ZMMr  PREG:r:Z=$Kk      MEM:r:f32=$mem512', 'NONE          ', 'VMOVAPS_rzmz_2', 'tuple=FVM'];

# MOVDDUP-Replicate Double FP Values.
T['VMOVDDUP VREG:w:f64=$XMMr PREG:r:Z=$Kk      VREG:r:f64=$XMMm ', 'MOD=REG       ', 'VMOVDDUP_romx', '         '];
T['VMOVDDUP VREG:w:f64=$XMMr PREG:r:Z=$Kk      MEM:r:f64=$mem64 ', 'MOD=MEM       ', 'VMOVDDUP_romx', 'tuple=DUP'];
T['VMOVDDUP VREG:w:f64=$XMMr VREG:r:f64=$XMMm                   ', 'MOD=REG VL=128', 'VMOVDDUP_rxmx', '         '];
T['VMOVDDUP VREG:w:f64=$YMMr VREG:r:f64=$YMMm                   ', 'MOD=REG VL=256', 'VMOVDDUP_rxmx', '         '];
T['VMOVDDUP VREG:w:f64=$XMMr MEM:r:f64=$mem64                   ', 'MOD=MEM VL=128', 'VMOVDDUP_rxmx', '         '];
T['VMOVDDUP VREG:w:f64=$YMMr MEM:r:f64=$mem256                  ', 'MOD=MEM VL=256', 'VMOVDDUP_rxmx', '         '];
T['VMOVDDUP VREG:w:f64=$YMMr PREG:r:Z=$Kk      VREG:r:f64=$YMMm ', 'MOD=REG       ', 'VMOVDDUP_rymy', '         '];
T['VMOVDDUP VREG:w:f64=$YMMr PREG:r:Z=$Kk      MEM:r:f64=$mem256', 'MOD=MEM       ', 'VMOVDDUP_rymy', 'tuple=DUP'];
T['VMOVDDUP VREG:w:f64=$ZMMr PREG:r:Z=$Kk      VREG:r:f64=$ZMMm ', 'MOD=REG       ', 'VMOVDDUP_rzmz', '         '];
T['VMOVDDUP VREG:w:f64=$ZMMr PREG:r:Z=$Kk      MEM:r:f64=$mem512', 'MOD=MEM       ', 'VMOVDDUP_rzmz', 'tuple=DUP'];

# MOVDQA,VMOVDQA32/64-Move Aligned Packed Integer Values.
T['VMOVDQA   VREG:w:s32=$XMMm  VREG:r:s32=$XMMr                   ', 'MOD=REG VL=128', 'VMOVDQA_mxrx    ', '         '];
T['VMOVDQA   VREG:w:s32=$YMMm  VREG:r:s32=$YMMr                   ', 'MOD=REG VL=256', 'VMOVDQA_mxrx    ', '         '];
T['VMOVDQA   MEM:w:s32=$mem128 VREG:r:s32=$XMMr                   ', 'MOD=MEM VL=128', 'VMOVDQA_mxrx    ', '         '];
T['VMOVDQA   MEM:w:s32=$mem256 VREG:r:s32=$YMMr                   ', 'MOD=MEM VL=256', 'VMOVDQA_mxrx    ', '         '];
T['VMOVDQA   VREG:w:s32=$XMMr  VREG:r:s32=$XMMm                   ', 'MOD=REG VL=128', 'VMOVDQA_rxmx    ', '         '];
T['VMOVDQA   VREG:w:s32=$YMMr  VREG:r:s32=$YMMm                   ', 'MOD=REG VL=256', 'VMOVDQA_rxmx    ', '         '];
T['VMOVDQA   VREG:w:s32=$XMMr  MEM:r:s32=$mem128                  ', 'MOD=MEM VL=128', 'VMOVDQA_rxmx    ', '         '];
T['VMOVDQA   VREG:w:s32=$YMMr  MEM:r:s32=$mem256                  ', 'MOD=MEM VL=256', 'VMOVDQA_rxmx    ', '         '];
T['VMOVDQA32 VREG:w:u32=$XMMm  PREG:r:Z=$Kk      VREG:r:u32=$XMMr ', 'W=0           ', 'VMOVDQAxx_moro_1', '         '];
T['VMOVDQA64 VREG:w:u64=$XMMm  PREG:r:Z=$Kk      VREG:r:u64=$XMMr ', 'W=1           ', 'VMOVDQAxx_moro_1', '         '];
T['VMOVDQA32 MEM:w:u32=$mem128 PREG:r:Z=$Kk      VREG:r:u32=$XMMr ', 'W=0           ', 'VMOVDQAxx_moro_2', 'tuple=FVM'];
T['VMOVDQA64 MEM:w:u64=$mem128 PREG:r:Z=$Kk      VREG:r:u64=$XMMr ', 'W=1           ', 'VMOVDQAxx_moro_2', 'tuple=FVM'];
T['VMOVDQA32 VREG:w:u32=$YMMm  PREG:r:Z=$Kk      VREG:r:u32=$YMMr ', 'W=0           ', 'VMOVDQAxx_myry_1', '         '];
T['VMOVDQA64 VREG:w:u64=$YMMm  PREG:r:Z=$Kk      VREG:r:u64=$YMMr ', 'W=1           ', 'VMOVDQAxx_myry_1', '         '];
T['VMOVDQA32 MEM:w:u32=$mem256 PREG:r:Z=$Kk      VREG:r:u32=$YMMr ', 'W=0           ', 'VMOVDQAxx_myry_2', 'tuple=FVM'];
T['VMOVDQA64 MEM:w:u64=$mem256 PREG:r:Z=$Kk      VREG:r:u64=$YMMr ', 'W=1           ', 'VMOVDQAxx_myry_2', 'tuple=FVM'];
T['VMOVDQA32 VREG:w:u32=$ZMMm  PREG:r:Z=$Kk      VREG:r:u32=$ZMMr ', 'W=0           ', 'VMOVDQAxx_mzrz_1', '         '];
T['VMOVDQA64 VREG:w:u64=$ZMMm  PREG:r:Z=$Kk      VREG:r:u64=$ZMMr ', 'W=1           ', 'VMOVDQAxx_mzrz_1', '         '];
T['VMOVDQA32 MEM:w:u32=$mem512 PREG:r:Z=$Kk      VREG:r:u32=$ZMMr ', 'W=0           ', 'VMOVDQAxx_mzrz_2', 'tuple=FVM'];
T['VMOVDQA64 MEM:w:u64=$mem512 PREG:r:Z=$Kk      VREG:r:u64=$ZMMr ', 'W=1           ', 'VMOVDQAxx_mzrz_2', 'tuple=FVM'];
T['VMOVDQA32 VREG:w:u32=$XMMr  PREG:r:Z=$Kk      VREG:r:u32=$XMMm ', 'W=0           ', 'VMOVDQAxx_romo_1', '         '];
T['VMOVDQA64 VREG:w:u64=$XMMr  PREG:r:Z=$Kk      VREG:r:u64=$XMMm ', 'W=1           ', 'VMOVDQAxx_romo_1', '         '];
T['VMOVDQA32 VREG:w:u32=$XMMr  PREG:r:Z=$Kk      MEM:r:u32=$mem128', 'W=0           ', 'VMOVDQAxx_romo_2', 'tuple=FVM'];
T['VMOVDQA64 VREG:w:u64=$XMMr  PREG:r:Z=$Kk      MEM:r:u64=$mem128', 'W=1           ', 'VMOVDQAxx_romo_2', 'tuple=FVM'];
T['VMOVDQA32 VREG:w:u32=$YMMr  PREG:r:Z=$Kk      VREG:r:u32=$YMMm ', 'W=0           ', 'VMOVDQAxx_rymy_1', '         '];
T['VMOVDQA64 VREG:w:u64=$YMMr  PREG:r:Z=$Kk      VREG:r:u64=$YMMm ', 'W=1           ', 'VMOVDQAxx_rymy_1', '         '];
T['VMOVDQA32 VREG:w:u32=$YMMr  PREG:r:Z=$Kk      MEM:r:u32=$mem256', 'W=0           ', 'VMOVDQAxx_rymy_2', 'tuple=FVM'];
T['VMOVDQA64 VREG:w:u64=$YMMr  PREG:r:Z=$Kk      MEM:r:u64=$mem256', 'W=1           ', 'VMOVDQAxx_rymy_2', 'tuple=FVM'];
T['VMOVDQA32 VREG:w:u32=$ZMMr  PREG:r:Z=$Kk      VREG:r:u32=$ZMMm ', 'W=0           ', 'VMOVDQAxx_rzmz_1', '         '];
T['VMOVDQA64 VREG:w:u64=$ZMMr  PREG:r:Z=$Kk      VREG:r:u64=$ZMMm ', 'W=1           ', 'VMOVDQAxx_rzmz_1', '         '];
T['VMOVDQA32 VREG:w:u32=$ZMMr  PREG:r:Z=$Kk      MEM:r:u32=$mem512', 'W=0           ', 'VMOVDQAxx_rzmz_2', 'tuple=FVM'];
T['VMOVDQA64 VREG:w:u64=$ZMMr  PREG:r:Z=$Kk      MEM:r:u64=$mem512', 'W=1           ', 'VMOVDQAxx_rzmz_2', 'tuple=FVM'];

# MOVDQU,VMOVDQU8/16/32/64-Move Unaligned Packed Integer Values.
T['VMOVDQU   VREG:w:s32=$XMMm  VREG:r:s32=$XMMr                   ', 'MOD=REG VL=128', 'VMOVDQU_mxrx  ', '         '];
T['VMOVDQU   VREG:w:s32=$YMMm  VREG:r:s32=$YMMr                   ', 'MOD=REG VL=256', 'VMOVDQU_mxrx  ', '         '];
T['VMOVDQU   MEM:w:s32=$mem128 VREG:r:s32=$XMMr                   ', 'MOD=MEM VL=128', 'VMOVDQU_mxrx  ', '         '];
T['VMOVDQU   MEM:w:s32=$mem256 VREG:r:s32=$YMMr                   ', 'MOD=MEM VL=256', 'VMOVDQU_mxrx  ', '         '];
T['VMOVDQU   VREG:w:s32=$XMMr  VREG:r:s32=$XMMm                   ', 'MOD=REG VL=128', 'VMOVDQU_rxmx  ', '         '];
T['VMOVDQU   VREG:w:s32=$YMMr  VREG:r:s32=$YMMm                   ', 'MOD=REG VL=256', 'VMOVDQU_rxmx  ', '         '];
T['VMOVDQU   VREG:w:s32=$XMMr  MEM:r:s32=$mem128                  ', 'MOD=MEM VL=128', 'VMOVDQU_rxmx  ', '         '];
T['VMOVDQU   VREG:w:s32=$YMMr  MEM:r:s32=$mem256                  ', 'MOD=MEM VL=256', 'VMOVDQU_rxmx  ', '         '];
T['VMOVDQU16 VREG:w:u16=$XMMm  PREG:r:Z=$Kk      VREG:r:u16=$XMMr ', 'MOD=REG W=1   ', 'VMOVDQUx_moro ', '         '];
T['VMOVDQU16 MEM:w:u16=$mem128 PREG:r:Z=$Kk      VREG:r:u16=$XMMr ', 'MOD=MEM W=1   ', 'VMOVDQUx_moro ', 'tuple=FVM'];
T['VMOVDQU8  VREG:w:u8=$XMMm   PREG:r:Z=$Kk      VREG:r:u8=$XMMr  ', 'MOD=REG W=0   ', 'VMOVDQUx_moro ', '         '];
T['VMOVDQU8  MEM:w:u8=$mem128  PREG:r:Z=$Kk      VREG:r:u8=$XMMr  ', 'MOD=MEM W=0   ', 'VMOVDQUx_moro ', 'tuple=FVM'];
T['VMOVDQU16 VREG:w:u16=$YMMm  PREG:r:Z=$Kk      VREG:r:u16=$YMMr ', 'MOD=REG W=1   ', 'VMOVDQUx_myry ', '         '];
T['VMOVDQU16 MEM:w:u16=$mem256 PREG:r:Z=$Kk      VREG:r:u16=$YMMr ', 'MOD=MEM W=1   ', 'VMOVDQUx_myry ', 'tuple=FVM'];
T['VMOVDQU8  VREG:w:u8=$YMMm   PREG:r:Z=$Kk      VREG:r:u8=$YMMr  ', 'MOD=REG W=0   ', 'VMOVDQUx_myry ', '         '];
T['VMOVDQU8  MEM:w:u8=$mem256  PREG:r:Z=$Kk      VREG:r:u8=$YMMr  ', 'MOD=MEM W=0   ', 'VMOVDQUx_myry ', 'tuple=FVM'];
T['VMOVDQU16 VREG:w:u16=$ZMMm  PREG:r:Z=$Kk      VREG:r:u16=$ZMMr ', 'MOD=REG W=1   ', 'VMOVDQUx_mzrz ', '         '];
T['VMOVDQU16 MEM:w:u16=$mem512 PREG:r:Z=$Kk      VREG:r:u16=$ZMMr ', 'MOD=MEM W=1   ', 'VMOVDQUx_mzrz ', 'tuple=FVM'];
T['VMOVDQU8  VREG:w:u8=$ZMMm   PREG:r:Z=$Kk      VREG:r:u8=$ZMMr  ', 'MOD=REG W=0   ', 'VMOVDQUx_mzrz ', '         '];
T['VMOVDQU8  MEM:w:u8=$mem512  PREG:r:Z=$Kk      VREG:r:u8=$ZMMr  ', 'MOD=MEM W=0   ', 'VMOVDQUx_mzrz ', 'tuple=FVM'];
T['VMOVDQU16 VREG:w:u16=$XMMr  PREG:r:Z=$Kk      VREG:r:u16=$XMMm ', 'MOD=REG W=1   ', 'VMOVDQUx_romo ', '         '];
T['VMOVDQU16 VREG:w:u16=$XMMr  PREG:r:Z=$Kk      MEM:r:u16=$mem128', 'MOD=MEM W=1   ', 'VMOVDQUx_romo ', 'tuple=FVM'];
T['VMOVDQU8  VREG:w:u8=$XMMr   PREG:r:Z=$Kk      VREG:r:u8=$XMMm  ', 'MOD=REG W=0   ', 'VMOVDQUx_romo ', '         '];
T['VMOVDQU8  VREG:w:u8=$XMMr   PREG:r:Z=$Kk      MEM:r:u8=$mem128 ', 'MOD=MEM W=0   ', 'VMOVDQUx_romo ', 'tuple=FVM'];
T['VMOVDQU16 VREG:w:u16=$YMMr  PREG:r:Z=$Kk      VREG:r:u16=$YMMm ', 'MOD=REG W=1   ', 'VMOVDQUx_rymy ', '         '];
T['VMOVDQU16 VREG:w:u16=$YMMr  PREG:r:Z=$Kk      MEM:r:u16=$mem256', 'MOD=MEM W=1   ', 'VMOVDQUx_rymy ', 'tuple=FVM'];
T['VMOVDQU8  VREG:w:u8=$YMMr   PREG:r:Z=$Kk      VREG:r:u8=$YMMm  ', 'MOD=REG W=0   ', 'VMOVDQUx_rymy ', '         '];
T['VMOVDQU8  VREG:w:u8=$YMMr   PREG:r:Z=$Kk      MEM:r:u8=$mem256 ', 'MOD=MEM W=0   ', 'VMOVDQUx_rymy ', 'tuple=FVM'];
T['VMOVDQU16 VREG:w:u16=$ZMMr  PREG:r:Z=$Kk      VREG:r:u16=$ZMMm ', 'MOD=REG W=1   ', 'VMOVDQUx_rzmz ', '         '];
T['VMOVDQU16 VREG:w:u16=$ZMMr  PREG:r:Z=$Kk      MEM:r:u16=$mem512', 'MOD=MEM W=1   ', 'VMOVDQUx_rzmz ', 'tuple=FVM'];
T['VMOVDQU8  VREG:w:u8=$ZMMr   PREG:r:Z=$Kk      VREG:r:u8=$ZMMm  ', 'MOD=REG W=0   ', 'VMOVDQUx_rzmz ', '         '];
T['VMOVDQU8  VREG:w:u8=$ZMMr   PREG:r:Z=$Kk      MEM:r:u8=$mem512 ', 'MOD=MEM W=0   ', 'VMOVDQUx_rzmz ', 'tuple=FVM'];
T['VMOVDQU32 VREG:w:u32=$XMMm  PREG:r:Z=$Kk      VREG:r:u32=$XMMr ', 'MOD=REG W=0   ', 'VMOVDQUxx_moro', '         '];
T['VMOVDQU32 MEM:w:u32=$mem128 PREG:r:Z=$Kk      VREG:r:u32=$XMMr ', 'MOD=MEM W=0   ', 'VMOVDQUxx_moro', 'tuple=FVM'];
T['VMOVDQU64 VREG:w:u64=$XMMm  PREG:r:Z=$Kk      VREG:r:u64=$XMMr ', 'MOD=REG W=1   ', 'VMOVDQUxx_moro', '         '];
T['VMOVDQU64 MEM:w:u64=$mem128 PREG:r:Z=$Kk      VREG:r:u64=$XMMr ', 'MOD=MEM W=1   ', 'VMOVDQUxx_moro', 'tuple=FVM'];
T['VMOVDQU32 VREG:w:u32=$YMMm  PREG:r:Z=$Kk      VREG:r:u32=$YMMr ', 'MOD=REG W=0   ', 'VMOVDQUxx_myry', '         '];
T['VMOVDQU32 MEM:w:u32=$mem256 PREG:r:Z=$Kk      VREG:r:u32=$YMMr ', 'MOD=MEM W=0   ', 'VMOVDQUxx_myry', 'tuple=FVM'];
T['VMOVDQU64 VREG:w:u64=$YMMm  PREG:r:Z=$Kk      VREG:r:u64=$YMMr ', 'MOD=REG W=1   ', 'VMOVDQUxx_myry', '         '];
T['VMOVDQU64 MEM:w:u64=$mem256 PREG:r:Z=$Kk      VREG:r:u64=$YMMr ', 'MOD=MEM W=1   ', 'VMOVDQUxx_myry', 'tuple=FVM'];
T['VMOVDQU32 VREG:w:u32=$ZMMm  PREG:r:Z=$Kk      VREG:r:u32=$ZMMr ', 'MOD=REG W=0   ', 'VMOVDQUxx_mzrz', '         '];
T['VMOVDQU32 MEM:w:u32=$mem512 PREG:r:Z=$Kk      VREG:r:u32=$ZMMr ', 'MOD=MEM W=0   ', 'VMOVDQUxx_mzrz', 'tuple=FVM'];
T['VMOVDQU64 VREG:w:u64=$ZMMm  PREG:r:Z=$Kk      VREG:r:u64=$ZMMr ', 'MOD=REG W=1   ', 'VMOVDQUxx_mzrz', '         '];
T['VMOVDQU64 MEM:w:u64=$mem512 PREG:r:Z=$Kk      VREG:r:u64=$ZMMr ', 'MOD=MEM W=1   ', 'VMOVDQUxx_mzrz', 'tuple=FVM'];
T['VMOVDQU32 VREG:w:u32=$XMMr  PREG:r:Z=$Kk      VREG:r:u32=$XMMm ', 'MOD=REG W=0   ', 'VMOVDQUxx_romo', '         '];
T['VMOVDQU32 VREG:w:u32=$XMMr  PREG:r:Z=$Kk      MEM:r:u32=$mem128', 'MOD=MEM W=0   ', 'VMOVDQUxx_romo', 'tuple=FVM'];
T['VMOVDQU64 VREG:w:u64=$XMMr  PREG:r:Z=$Kk      VREG:r:u64=$XMMm ', 'MOD=REG W=1   ', 'VMOVDQUxx_romo', '         '];
T['VMOVDQU64 VREG:w:u64=$XMMr  PREG:r:Z=$Kk      MEM:r:u64=$mem128', 'MOD=MEM W=1   ', 'VMOVDQUxx_romo', 'tuple=FVM'];
T['VMOVDQU32 VREG:w:u32=$YMMr  PREG:r:Z=$Kk      VREG:r:u32=$YMMm ', 'MOD=REG W=0   ', 'VMOVDQUxx_rymy', '         '];
T['VMOVDQU32 VREG:w:u32=$YMMr  PREG:r:Z=$Kk      MEM:r:u32=$mem256', 'MOD=MEM W=0   ', 'VMOVDQUxx_rymy', 'tuple=FVM'];
T['VMOVDQU64 VREG:w:u64=$YMMr  PREG:r:Z=$Kk      VREG:r:u64=$YMMm ', 'MOD=REG W=1   ', 'VMOVDQUxx_rymy', '         '];
T['VMOVDQU64 VREG:w:u64=$YMMr  PREG:r:Z=$Kk      MEM:r:u64=$mem256', 'MOD=MEM W=1   ', 'VMOVDQUxx_rymy', 'tuple=FVM'];
T['VMOVDQU32 VREG:w:u32=$ZMMr  PREG:r:Z=$Kk      VREG:r:u32=$ZMMm ', 'MOD=REG W=0   ', 'VMOVDQUxx_rzmz', '         '];
T['VMOVDQU32 VREG:w:u32=$ZMMr  PREG:r:Z=$Kk      MEM:r:u32=$mem512', 'MOD=MEM W=0   ', 'VMOVDQUxx_rzmz', 'tuple=FVM'];
T['VMOVDQU64 VREG:w:u64=$ZMMr  PREG:r:Z=$Kk      VREG:r:u64=$ZMMm ', 'MOD=REG W=1   ', 'VMOVDQUxx_rzmz', '         '];
T['VMOVDQU64 VREG:w:u64=$ZMMr  PREG:r:Z=$Kk      MEM:r:u64=$mem512', 'MOD=MEM W=1   ', 'VMOVDQUxx_rzmz', 'tuple=FVM'];

# MOVHLPS-Move Packed Single-Precision Floating-Point Values High to Low.
T['VMOVHLPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm', 'NONE', 'VMOVHLPS_ronomo_1', ''];
T['VMOVHLPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm', 'NONE', 'VMOVHLPS_ronomo_2', ''];

# MOVHPD-Move High Packed Double-Precision Floating-Point Value.
T['VMOVHPD MEM:w:f64=$mem64 VREG:r:f64=$XMMr                 ', 'NONE', 'VMOVHPD_mqro_1  ', '         '];
T['VMOVHPD MEM:w:f64=$mem64 VREG:r:f64=$XMMr                 ', 'NONE', 'VMOVHPD_mqro_2  ', 'tuple=T1S'];
T['VMOVHPD VREG:w:f64=$XMMr REG:r:f64=$XMMn  MEM:r:f64=$mem64', 'NONE', 'VMOVHPD_ronqmq_1', '         '];
T['VMOVHPD VREG:w:f64=$XMMr REG:r:f64=$XMMn  MEM:r:f64=$mem64', 'NONE', 'VMOVHPD_ronqmq_2', 'tuple=T1S'];

# MOVHPS-Move High Packed Single-Precision Floating-Point Values.
T['VMOVHPS MEM:w:f32=$mem64 VREG:r:f32=$XMMr                 ', 'NONE', 'VMOVHPS_mqro_1  ', '        '];
T['VMOVHPS MEM:w:f32=$mem64 VREG:r:f32=$XMMr                 ', 'NONE', 'VMOVHPS_mqro_2  ', 'tuple=T2'];
T['VMOVHPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem64', 'NONE', 'VMOVHPS_ronqmq_1', '        '];
T['VMOVHPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem64', 'NONE', 'VMOVHPS_ronqmq_2', 'tuple=T2'];

# MOVLHPS-Move Packed Single-Precision Floating-Point Values Low to High.
T['VMOVLHPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm', 'NONE', 'VMOVLHPS_ronqmq_1', ''];
T['VMOVLHPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm', 'NONE', 'VMOVLHPS_ronqmq_2', ''];

# MOVLPD-Move Low Packed Double-Precision Floating-Point Value.
T['VMOVLPD MEM:w:f64=$mem64 REG:r:f64=$XMMr                  ', 'NONE', 'VMOVLPD_mqrq_1  ', '         '];
T['VMOVLPD MEM:w:f64=$mem64 REG:r:f64=$XMMr                  ', 'NONE', 'VMOVLPD_mqrq_2  ', 'tuple=T1S'];
T['VMOVLPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem64', 'NONE', 'VMOVLPD_ronomq_1', '         '];
T['VMOVLPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem64', 'NONE', 'VMOVLPD_ronomq_2', 'tuple=T1S'];

# MOVLPS-Move Low Packed Single-Precision Floating-Point Values.
T['VMOVLPS MEM:w:f32=$mem64 VREG:r:f32=$XMMr                 ', 'NONE', 'VMOVLPS_mqrq_1  ', '        '];
T['VMOVLPS MEM:w:f32=$mem64 VREG:r:f32=$XMMr                 ', 'NONE', 'VMOVLPS_mqrq_2  ', 'tuple=T2'];
T['VMOVLPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem64', 'NONE', 'VMOVLPS_ronomq_1', '        '];
T['VMOVLPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem64', 'NONE', 'VMOVLPS_ronomq_2', 'tuple=T2'];

# MOVMSKPD-Extract Packed Double-Precision Floating-Point Sign Mask.
T['VMOVMSKPD REG:w:s32=$GPR32r VREG:r:f64=$XMMm', 'VL=128', 'VMOVMSKPD', ''];
T['VMOVMSKPD REG:w:s32=$GPR32r VREG:r:f64=$YMMm', 'VL=256', 'VMOVMSKPD', ''];

# MOVMSKPS-Extract Packed Single-Precision Floating-Point Sign Mask.
T['VMOVMSKPS REG:w:s32=$GPR32r VREG:r:f32=$XMMm', 'VL=128', 'VMOVMSKPS', ''];
T['VMOVMSKPS REG:w:s32=$GPR32r VREG:r:f32=$YMMm', 'VL=256', 'VMOVMSKPS', ''];

# MOVNTDQ-Store Packed Integers Using Non-Temporal Hint.
T['VMOVNTDQ MEM:w:u32=$mem128 VREG:r:u32=$XMMr', 'NONE  ', 'VMOVNTDQ_moro', 'tuple=FVM'];
T['VMOVNTDQ MEM:w:s32=$mem128 VREG:r:s32=$XMMr', 'VL=128', 'VMOVNTDQ_mxrx', '         '];
T['VMOVNTDQ MEM:w:s32=$mem256 VREG:r:s32=$YMMr', 'VL=256', 'VMOVNTDQ_mxrx', '         '];
T['VMOVNTDQ MEM:w:u32=$mem256 VREG:r:u32=$YMMr', 'NONE  ', 'VMOVNTDQ_myry', 'tuple=FVM'];
T['VMOVNTDQ MEM:w:u32=$mem512 VREG:r:u32=$ZMMr', 'NONE  ', 'VMOVNTDQ_mzrz', 'tuple=FVM'];

# MOVNTDQA-Load Double Quadword Non-Temporal Aligned Hint.
T['VMOVNTDQA VREG:w:s32=$XMMr MEM:r:s32=$mem128', 'NONE', 'VMOVNTDQA_romo_1', '         '];
T['VMOVNTDQA VREG:w:u32=$XMMr MEM:r:u32=$mem128', 'NONE', 'VMOVNTDQA_romo_2', 'tuple=FVM'];
T['VMOVNTDQA VREG:w:s32=$YMMr MEM:r:s32=$mem256', 'NONE', 'VMOVNTDQA_rymy_1', '         '];
T['VMOVNTDQA VREG:w:u32=$YMMr MEM:r:u32=$mem256', 'NONE', 'VMOVNTDQA_rymy_2', 'tuple=FVM'];
T['VMOVNTDQA VREG:w:u32=$ZMMr MEM:r:u32=$mem512', 'NONE', 'VMOVNTDQA_rzmz  ', 'tuple=FVM'];

# MOVNTPD-Store Packed Double-Precision Floating-Point Values Using Non-Temporal Hint.
T['VMOVNTPD MEM:w:f64=$mem128 VREG:r:f64=$XMMr', 'NONE  ', 'VMOVNTPD_moro', 'tuple=FVM'];
T['VMOVNTPD MEM:w:f64=$mem128 VREG:r:f64=$XMMr', 'VL=128', 'VMOVNTPD_mxrx', '         '];
T['VMOVNTPD MEM:w:f64=$mem256 VREG:r:f64=$YMMr', 'VL=256', 'VMOVNTPD_mxrx', '         '];
T['VMOVNTPD MEM:w:f64=$mem256 VREG:r:f64=$YMMr', 'NONE  ', 'VMOVNTPD_myry', 'tuple=FVM'];
T['VMOVNTPD MEM:w:f64=$mem512 VREG:r:f64=$ZMMr', 'NONE  ', 'VMOVNTPD_mzrz', 'tuple=FVM'];

# MOVNTPS-Store Packed Single-Precision Floating-Point Values Using Non-Temporal Hint.
T['VMOVNTPS MEM:w:f32=$mem128 VREG:r:f32=$XMMr', 'NONE  ', 'VMOVNTPS_moro', 'tuple=FVM'];
T['VMOVNTPS MEM:w:f32=$mem128 VREG:r:f32=$XMMr', 'VL=128', 'VMOVNTPS_mxrx', '         '];
T['VMOVNTPS MEM:w:f32=$mem256 VREG:r:f32=$YMMr', 'VL=256', 'VMOVNTPS_mxrx', '         '];
T['VMOVNTPS MEM:w:f32=$mem256 VREG:r:f32=$YMMr', 'NONE  ', 'VMOVNTPS_myry', 'tuple=FVM'];
T['VMOVNTPS MEM:w:f32=$mem512 VREG:r:f32=$ZMMr', 'NONE  ', 'VMOVNTPS_mzrz', 'tuple=FVM'];

# MOVQ-Move Quadword.
T['VMOVQ REG:w:u64=$GPR64m VREG:r:u64=$XMMr ', 'MOD=REG', 'VMOVQ_mqro  ', '         '];
T['VMOVQ MEM:w:u64=$mem64  VREG:r:u64=$XMMr ', 'MOD=MEM', 'VMOVQ_mqro  ', 'tuple=T1S'];
T['VMOVQ REG:w:s64=$GPR64m REG:r:s64=$XMMr  ', 'MOD=REG', 'VMOVQ_mqrq  ', '         '];
T['VMOVQ MEM:w:s64=$mem64  REG:r:s64=$XMMr  ', 'MOD=MEM', 'VMOVQ_mqrq  ', '         '];
T['VMOVQ VREG:w:u64=$XMMm  VREG:r:u64=$XMMr ', 'MOD=REG', 'VMOVQ_mxro  ', '         '];
T['VMOVQ MEM:w:u64=$mem64  VREG:r:u64=$XMMr ', 'MOD=MEM', 'VMOVQ_mxro  ', 'tuple=T1S'];
T['VMOVQ VREG:w:s32=$XMMm  REG:r:s64=$XMMr  ', 'MOD=REG', 'VMOVQ_mxrq  ', '         '];
T['VMOVQ MEM:w:s64=$mem64  REG:r:s64=$XMMr  ', 'MOD=MEM', 'VMOVQ_mxrq  ', '         '];
T['VMOVQ VREG:w:s32=$XMMr  REG:r:s64=$GPR64m', 'MOD=REG', 'VMOVQ_romq_1', '         '];
T['VMOVQ VREG:w:s32=$XMMr  MEM:r:s64=$mem64 ', 'MOD=MEM', 'VMOVQ_romq_1', '         '];
T['VMOVQ VREG:w:s32=$XMMr  REG:r:s64=$XMMm  ', 'MOD=REG', 'VMOVQ_romq_2', '         '];
T['VMOVQ VREG:w:s32=$XMMr  MEM:r:s64=$mem64 ', 'MOD=MEM', 'VMOVQ_romq_2', '         '];
T['VMOVQ VREG:w:u64=$XMMr  REG:r:u64=$GPR64m', 'MOD=REG', 'VMOVQ_romq_3', '         '];
T['VMOVQ VREG:w:u64=$XMMr  MEM:r:u64=$mem64 ', 'MOD=MEM', 'VMOVQ_romq_3', 'tuple=T1S'];
T['VMOVQ VREG:w:u64=$XMMr  VREG:r:u64=$XMMm ', 'MOD=REG', 'VMOVQ_romx  ', '         '];
T['VMOVQ VREG:w:u64=$XMMr  MEM:r:u64=$mem64 ', 'MOD=MEM', 'VMOVQ_romx  ', 'tuple=T1S'];

# MOVS/MOVSB/MOVSW/MOVSD/MOVSQ-Move Data from String to String.
T['VMOVSD VREG:w:f64=$XMMm PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMr', 'NONE', 'VMOVSD_monoro', '         '];
T['VMOVSD VREG:w:f64=$XMMm VREG:r:f64=$XMMn REG:r:f64=$XMMr                  ', 'NONE', 'VMOVSD_monorq', '         '];
T['VMOVSD MEM:w:f64=$mem64 PREG:r:Z=$Kk     VREG:r:f64=$XMMr                 ', 'NONE', 'VMOVSD_mqro  ', 'tuple=T1S'];
T['VMOVSD MEM:w:f64=$mem64 REG:r:f64=$XMMr                                   ', 'NONE', 'VMOVSD_mqrq  ', '         '];
T['VMOVSD VREG:w:f64=$XMMr MEM:r:f64=$mem64                                  ', 'NONE', 'VMOVSD_romq_1', '         '];
T['VMOVSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     MEM:r:f64=$mem64                 ', 'NONE', 'VMOVSD_romq_2', 'tuple=T1S'];
T['VMOVSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm', 'NONE', 'VMOVSD_ronomo', '         '];
T['VMOVSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn REG:r:f64=$XMMm                  ', 'NONE', 'VMOVSD_ronomq', '         '];

# MOVSHDUP-Replicate Single FP Values.
T['VMOVSHDUP VREG:w:f32=$XMMr PREG:r:Z=$Kk      VREG:r:f32=$XMMm ', 'MOD=REG       ', 'VMOVSHDUP_romo', '         '];
T['VMOVSHDUP VREG:w:f32=$XMMr PREG:r:Z=$Kk      MEM:r:f32=$mem128', 'MOD=MEM       ', 'VMOVSHDUP_romo', 'tuple=FVM'];
T['VMOVSHDUP VREG:w:f32=$XMMr VREG:r:f32=$XMMm                   ', 'MOD=REG VL=128', 'VMOVSHDUP_rxmx', '         '];
T['VMOVSHDUP VREG:w:f32=$YMMr VREG:r:f32=$YMMm                   ', 'MOD=REG VL=256', 'VMOVSHDUP_rxmx', '         '];
T['VMOVSHDUP VREG:w:f32=$XMMr MEM:r:f32=$mem128                  ', 'MOD=MEM VL=128', 'VMOVSHDUP_rxmx', '         '];
T['VMOVSHDUP VREG:w:f32=$YMMr MEM:r:f32=$mem256                  ', 'MOD=MEM VL=256', 'VMOVSHDUP_rxmx', '         '];
T['VMOVSHDUP VREG:w:f32=$YMMr PREG:r:Z=$Kk      VREG:r:f32=$YMMm ', 'MOD=REG       ', 'VMOVSHDUP_rymy', '         '];
T['VMOVSHDUP VREG:w:f32=$YMMr PREG:r:Z=$Kk      MEM:r:f32=$mem256', 'MOD=MEM       ', 'VMOVSHDUP_rymy', 'tuple=FVM'];
T['VMOVSHDUP VREG:w:f32=$ZMMr PREG:r:Z=$Kk      VREG:r:f32=$ZMMm ', 'MOD=REG       ', 'VMOVSHDUP_rzmz', '         '];
T['VMOVSHDUP VREG:w:f32=$ZMMr PREG:r:Z=$Kk      MEM:r:f32=$mem512', 'MOD=MEM       ', 'VMOVSHDUP_rzmz', 'tuple=FVM'];

# MOVSLDUP-Replicate Single FP Values.
T['VMOVSLDUP VREG:w:f32=$XMMr PREG:r:Z=$Kk      VREG:r:f32=$XMMm ', 'MOD=REG       ', 'VMOVSLDUP_romo', '         '];
T['VMOVSLDUP VREG:w:f32=$XMMr PREG:r:Z=$Kk      MEM:r:f32=$mem128', 'MOD=MEM       ', 'VMOVSLDUP_romo', 'tuple=FVM'];
T['VMOVSLDUP VREG:w:f32=$XMMr VREG:r:f32=$XMMm                   ', 'MOD=REG VL=128', 'VMOVSLDUP_rxmx', '         '];
T['VMOVSLDUP VREG:w:f32=$YMMr VREG:r:f32=$YMMm                   ', 'MOD=REG VL=256', 'VMOVSLDUP_rxmx', '         '];
T['VMOVSLDUP VREG:w:f32=$XMMr MEM:r:f32=$mem128                  ', 'MOD=MEM VL=128', 'VMOVSLDUP_rxmx', '         '];
T['VMOVSLDUP VREG:w:f32=$YMMr MEM:r:f32=$mem256                  ', 'MOD=MEM VL=256', 'VMOVSLDUP_rxmx', '         '];
T['VMOVSLDUP VREG:w:f32=$YMMr PREG:r:Z=$Kk      VREG:r:f32=$YMMm ', 'MOD=REG       ', 'VMOVSLDUP_rymy', '         '];
T['VMOVSLDUP VREG:w:f32=$YMMr PREG:r:Z=$Kk      MEM:r:f32=$mem256', 'MOD=MEM       ', 'VMOVSLDUP_rymy', 'tuple=FVM'];
T['VMOVSLDUP VREG:w:f32=$ZMMr PREG:r:Z=$Kk      VREG:r:f32=$ZMMm ', 'MOD=REG       ', 'VMOVSLDUP_rzmz', '         '];
T['VMOVSLDUP VREG:w:f32=$ZMMr PREG:r:Z=$Kk      MEM:r:f32=$mem512', 'MOD=MEM       ', 'VMOVSLDUP_rzmz', 'tuple=FVM'];

# MOVSS-Move or Merge Scalar Single-Precision Floating-Point Value.
T['VMOVSS MEM:w:f32=$mem32 REG:r:f32=$XMMr                                   ', 'NONE', 'VMOVSS_mdrd  ', '         '];
T['VMOVSS MEM:w:f32=$mem32 PREG:r:Z=$Kk     VREG:r:f32=$XMMr                 ', 'NONE', 'VMOVSS_mdro  ', 'tuple=T1S'];
T['VMOVSS VREG:w:f32=$XMMm VREG:r:f32=$XMMn REG:r:f32=$XMMr                  ', 'NONE', 'VMOVSS_monord', '         '];
T['VMOVSS VREG:w:f32=$XMMm PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMr', 'NONE', 'VMOVSS_monoro', '         '];
T['VMOVSS VREG:w:f32=$XMMr MEM:r:f32=$mem32                                  ', 'NONE', 'VMOVSS_romd_1', '         '];
T['VMOVSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     MEM:r:f32=$mem32                 ', 'NONE', 'VMOVSS_romd_2', 'tuple=T1S'];
T['VMOVSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn REG:r:f32=$XMMm                  ', 'NONE', 'VMOVSS_ronomd', '         '];
T['VMOVSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm', 'NONE', 'VMOVSS_ronomo', '         '];

# MOVUPD-Move Unaligned Packed Double-Precision Floating-Point Values.
T['VMOVUPD VREG:w:f64=$XMMm  PREG:r:Z=$Kk      VREG:r:f64=$XMMr ', 'MOD=REG       ', 'VMOVUPD_moro', '         '];
T['VMOVUPD MEM:w:f64=$mem128 PREG:r:Z=$Kk      VREG:r:f64=$XMMr ', 'MOD=MEM       ', 'VMOVUPD_moro', 'tuple=FVM'];
T['VMOVUPD VREG:w:f64=$XMMm  VREG:r:f64=$XMMr                   ', 'MOD=REG VL=128', 'VMOVUPD_mxrx', '         '];
T['VMOVUPD VREG:w:f64=$YMMm  VREG:r:f64=$YMMr                   ', 'MOD=REG VL=256', 'VMOVUPD_mxrx', '         '];
T['VMOVUPD MEM:w:f64=$mem128 VREG:r:f64=$XMMr                   ', 'MOD=MEM VL=128', 'VMOVUPD_mxrx', '         '];
T['VMOVUPD MEM:w:f64=$mem256 VREG:r:f64=$YMMr                   ', 'MOD=MEM VL=256', 'VMOVUPD_mxrx', '         '];
T['VMOVUPD VREG:w:f64=$YMMm  PREG:r:Z=$Kk      VREG:r:f64=$YMMr ', 'MOD=REG       ', 'VMOVUPD_myry', '         '];
T['VMOVUPD MEM:w:f64=$mem256 PREG:r:Z=$Kk      VREG:r:f64=$YMMr ', 'MOD=MEM       ', 'VMOVUPD_myry', 'tuple=FVM'];
T['VMOVUPD VREG:w:f64=$ZMMm  PREG:r:Z=$Kk      VREG:r:f64=$ZMMr ', 'MOD=REG       ', 'VMOVUPD_mzrz', '         '];
T['VMOVUPD MEM:w:f64=$mem512 PREG:r:Z=$Kk      VREG:r:f64=$ZMMr ', 'MOD=MEM       ', 'VMOVUPD_mzrz', 'tuple=FVM'];
T['VMOVUPD VREG:w:f64=$XMMr  PREG:r:Z=$Kk      VREG:r:f64=$XMMm ', 'MOD=REG       ', 'VMOVUPD_romo', '         '];
T['VMOVUPD VREG:w:f64=$XMMr  PREG:r:Z=$Kk      MEM:r:f64=$mem128', 'MOD=MEM       ', 'VMOVUPD_romo', 'tuple=FVM'];
T['VMOVUPD VREG:w:f64=$XMMr  VREG:r:f64=$XMMm                   ', 'MOD=REG VL=128', 'VMOVUPD_rxmx', '         '];
T['VMOVUPD VREG:w:f64=$YMMr  VREG:r:f64=$YMMm                   ', 'MOD=REG VL=256', 'VMOVUPD_rxmx', '         '];
T['VMOVUPD VREG:w:f64=$XMMr  MEM:r:f64=$mem128                  ', 'MOD=MEM VL=128', 'VMOVUPD_rxmx', '         '];
T['VMOVUPD VREG:w:f64=$YMMr  MEM:r:f64=$mem256                  ', 'MOD=MEM VL=256', 'VMOVUPD_rxmx', '         '];
T['VMOVUPD VREG:w:f64=$YMMr  PREG:r:Z=$Kk      VREG:r:f64=$YMMm ', 'MOD=REG       ', 'VMOVUPD_rymy', '         '];
T['VMOVUPD VREG:w:f64=$YMMr  PREG:r:Z=$Kk      MEM:r:f64=$mem256', 'MOD=MEM       ', 'VMOVUPD_rymy', 'tuple=FVM'];
T['VMOVUPD VREG:w:f64=$ZMMr  PREG:r:Z=$Kk      VREG:r:f64=$ZMMm ', 'MOD=REG       ', 'VMOVUPD_rzmz', '         '];
T['VMOVUPD VREG:w:f64=$ZMMr  PREG:r:Z=$Kk      MEM:r:f64=$mem512', 'MOD=MEM       ', 'VMOVUPD_rzmz', 'tuple=FVM'];

# MOVUPS-Move Unaligned Packed Single-Precision Floating-Point Values.
T['VMOVUPS VREG:w:f32=$XMMm  PREG:r:Z=$Kk      VREG:r:f32=$XMMr ', 'MOD=REG       ', 'VMOVUPS_moro', '         '];
T['VMOVUPS MEM:w:f32=$mem128 PREG:r:Z=$Kk      VREG:r:f32=$XMMr ', 'MOD=MEM       ', 'VMOVUPS_moro', 'tuple=FVM'];
T['VMOVUPS VREG:w:f32=$XMMm  VREG:r:f32=$XMMr                   ', 'MOD=REG VL=128', 'VMOVUPS_mxrx', '         '];
T['VMOVUPS VREG:w:f32=$YMMm  VREG:r:f32=$YMMr                   ', 'MOD=REG VL=256', 'VMOVUPS_mxrx', '         '];
T['VMOVUPS MEM:w:f32=$mem128 VREG:r:f32=$XMMr                   ', 'MOD=MEM VL=128', 'VMOVUPS_mxrx', '         '];
T['VMOVUPS MEM:w:f32=$mem256 VREG:r:f32=$YMMr                   ', 'MOD=MEM VL=256', 'VMOVUPS_mxrx', '         '];
T['VMOVUPS VREG:w:f32=$YMMm  PREG:r:Z=$Kk      VREG:r:f32=$YMMr ', 'MOD=REG       ', 'VMOVUPS_myry', '         '];
T['VMOVUPS MEM:w:f32=$mem256 PREG:r:Z=$Kk      VREG:r:f32=$YMMr ', 'MOD=MEM       ', 'VMOVUPS_myry', 'tuple=FVM'];
T['VMOVUPS VREG:w:f32=$ZMMm  PREG:r:Z=$Kk      VREG:r:f32=$ZMMr ', 'MOD=REG       ', 'VMOVUPS_mzrz', '         '];
T['VMOVUPS MEM:w:f32=$mem512 PREG:r:Z=$Kk      VREG:r:f32=$ZMMr ', 'MOD=MEM       ', 'VMOVUPS_mzrz', 'tuple=FVM'];
T['VMOVUPS VREG:w:f32=$XMMr  PREG:r:Z=$Kk      VREG:r:f32=$XMMm ', 'MOD=REG       ', 'VMOVUPS_romo', '         '];
T['VMOVUPS VREG:w:f32=$XMMr  PREG:r:Z=$Kk      MEM:r:f32=$mem128', 'MOD=MEM       ', 'VMOVUPS_romo', 'tuple=FVM'];
T['VMOVUPS VREG:w:f32=$XMMr  VREG:r:f32=$XMMm                   ', 'MOD=REG VL=128', 'VMOVUPS_rxmx', '         '];
T['VMOVUPS VREG:w:f32=$YMMr  VREG:r:f32=$YMMm                   ', 'MOD=REG VL=256', 'VMOVUPS_rxmx', '         '];
T['VMOVUPS VREG:w:f32=$XMMr  MEM:r:f32=$mem128                  ', 'MOD=MEM VL=128', 'VMOVUPS_rxmx', '         '];
T['VMOVUPS VREG:w:f32=$YMMr  MEM:r:f32=$mem256                  ', 'MOD=MEM VL=256', 'VMOVUPS_rxmx', '         '];
T['VMOVUPS VREG:w:f32=$YMMr  PREG:r:Z=$Kk      VREG:r:f32=$YMMm ', 'MOD=REG       ', 'VMOVUPS_rymy', '         '];
T['VMOVUPS VREG:w:f32=$YMMr  PREG:r:Z=$Kk      MEM:r:f32=$mem256', 'MOD=MEM       ', 'VMOVUPS_rymy', 'tuple=FVM'];
T['VMOVUPS VREG:w:f32=$ZMMr  PREG:r:Z=$Kk      VREG:r:f32=$ZMMm ', 'MOD=REG       ', 'VMOVUPS_rzmz', '         '];
T['VMOVUPS VREG:w:f32=$ZMMr  PREG:r:Z=$Kk      MEM:r:f32=$mem512', 'MOD=MEM       ', 'VMOVUPS_rzmz', 'tuple=FVM'];

# MOVD/MOVQ-Move Doubleword/Move Quadword.
T['VMOVD REG:w:s32=$GPR32m REG:r:s32=$XMMr  ', 'MOD=REG MODE=64 W=0  ', 'VMOVD_mdrd  ', '         '];
T['VMOVD MEM:w:s32=$mem32  REG:r:s32=$XMMr  ', 'MOD=MEM MODE=64 W=0  ', 'VMOVD_mdrd  ', '         '];
T['VMOVD REG:w:s32=$GPR32m REG:r:s32=$XMMr  ', 'MOD=REG MODE=NO64    ', 'VMOVD_mdrd  ', '         '];
T['VMOVD MEM:w:s32=$mem32  REG:r:s32=$XMMr  ', 'MOD=MEM MODE=NO64    ', 'VMOVD_mdrd  ', '         '];
T['VMOVD REG:w:u32=$GPR32m VREG:r:u32=$XMMr ', 'MOD=REG W=0          ', 'VMOVD_mdro  ', '         '];
T['VMOVD MEM:w:u32=$mem32  VREG:r:u32=$XMMr ', 'MOD=MEM W=0          ', 'VMOVD_mdro  ', 'tuple=T1S'];
T['VMOVD REG:w:u32=$GPR32m VREG:r:u32=$XMMr ', 'MOD=REG MODE=NO64 W=1', 'VMOVD_mdro  ', '         '];
T['VMOVD MEM:w:u32=$mem32  VREG:r:u32=$XMMr ', 'MOD=MEM MODE=NO64 W=1', 'VMOVD_mdro  ', 'tuple=T1S'];
T['VMOVD VREG:w:s32=$XMMr  REG:r:s32=$GPR32m', 'MOD=REG MODE=64 W=0  ', 'VMOVD_romd_1', '         '];
T['VMOVD VREG:w:s32=$XMMr  MEM:r:s32=$mem32 ', 'MOD=MEM MODE=64 W=0  ', 'VMOVD_romd_1', '         '];
T['VMOVD VREG:w:s32=$XMMr  REG:r:s32=$GPR32m', 'MOD=REG MODE=NO64    ', 'VMOVD_romd_1', '         '];
T['VMOVD VREG:w:s32=$XMMr  MEM:r:s32=$mem32 ', 'MOD=MEM MODE=NO64    ', 'VMOVD_romd_1', '         '];
T['VMOVD VREG:w:u32=$XMMr  REG:r:u32=$GPR32m', 'MOD=REG W=0          ', 'VMOVD_romd_2', '         '];
T['VMOVD VREG:w:u32=$XMMr  MEM:r:u32=$mem32 ', 'MOD=MEM W=0          ', 'VMOVD_romd_2', 'tuple=T1S'];
T['VMOVD VREG:w:u32=$XMMr  REG:r:u32=$GPR32m', 'MOD=REG MODE=NO64 W=1', 'VMOVD_romd_2', '         '];
T['VMOVD VREG:w:u32=$XMMr  MEM:r:u32=$mem32 ', 'MOD=MEM MODE=NO64 W=1', 'VMOVD_romd_2', 'tuple=T1S'];

# MPSADBW-Compute Multiple Packed Sums of Absolute Difference.
T['VMPSADBW VREG:w:u16=$XMMr VREG:r:u8=$XMMn VREG:r:u8=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'VMPSADBW_ronomoub', ''];
T['VMPSADBW VREG:w:u16=$XMMr VREG:r:u8=$XMMn MEM:r:u8=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'VMPSADBW_ronomoub', ''];
T['VMPSADBW VREG:w:u16=$YMMr VREG:r:u8=$YMMn VREG:r:u8=$YMMm  IMM:u8=$uimm8', 'MOD=REG', 'VMPSADBW_rynymyub', ''];
T['VMPSADBW VREG:w:u16=$YMMr VREG:r:u8=$YMMn MEM:r:u8=$mem256 IMM:u8=$uimm8', 'MOD=MEM', 'VMPSADBW_rynymyub', ''];

# MULPD-Multiply Packed Double-Precision Floating-Point Values.
T['VMULPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VMULPD_ronomo', '        '];
T['VMULPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VMULPD_ronomo', 'tuple=FV'];
T['VMULPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VMULPD_rxnxmx', '        '];
T['VMULPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VMULPD_rxnxmx', '        '];
T['VMULPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VMULPD_rxnxmx', '        '];
T['VMULPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VMULPD_rxnxmx', '        '];
T['VMULPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VMULPD_rynymy', '        '];
T['VMULPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VMULPD_rynymy', 'tuple=FV'];
T['VMULPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VMULPD_rznzmz', '        '];
T['VMULPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VMULPD_rznzmz', '        '];
T['VMULPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VMULPD_rznzmz', 'tuple=FV'];

# MULPS-Multiply Packed Single-Precision Floating-Point Values.
T['VMULPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VMULPS_ronomo', '        '];
T['VMULPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VMULPS_ronomo', 'tuple=FV'];
T['VMULPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VMULPS_rxnxmx', '        '];
T['VMULPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VMULPS_rxnxmx', '        '];
T['VMULPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VMULPS_rxnxmx', '        '];
T['VMULPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VMULPS_rxnxmx', '        '];
T['VMULPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VMULPS_rynymy', '        '];
T['VMULPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VMULPS_rynymy', 'tuple=FV'];
T['VMULPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VMULPS_rznzmz', '        '];
T['VMULPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VMULPS_rznzmz', '        '];
T['VMULPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VMULPS_rznzmz', 'tuple=FV'];

# MULSD-Multiply Scalar Double-Precision Floating-Point Value.
T['VMULSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VMULSD_ronomq', '         '];
T['VMULSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VMULSD_ronomq', '         '];
T['VMULSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VMULSD_ronomx', '         '];
T['VMULSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VMULSD_ronomx', '         '];
T['VMULSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VMULSD_ronomx', 'tuple=T1S'];

# MULSS-Multiply Scalar Single-Precision Floating-Point Values.
T['VMULSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn REG:r:f32=$XMMm                         ', 'MOD=REG             ', 'VMULSS_ronomd', '         '];
T['VMULSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem32                        ', 'MOD=MEM             ', 'VMULSS_ronomd', '         '];
T['VMULSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VMULSS_ronomx', '         '];
T['VMULSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VMULSS_ronomx', '         '];
T['VMULSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VMULSS_ronomx', 'tuple=T1S'];

# ORPD-Bitwise Logical OR of Packed Double Precision Floating-Point Values.
T['VORPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG  ', 'VORPD_ronomo', '        '];
T['VORPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM       ', 'VORPD_ronomo', 'tuple=FV'];
T['VORPD VREG:w:u64=$XMMr VREG:r:u64=$XMMn VREG:r:u64=$XMMm                              ', 'MOD=REG VL=128', 'VORPD_rxnxmx', '        '];
T['VORPD VREG:w:u64=$YMMr VREG:r:u64=$YMMn VREG:r:u64=$YMMm                              ', 'MOD=REG VL=256', 'VORPD_rxnxmx', '        '];
T['VORPD VREG:w:u64=$XMMr VREG:r:u64=$XMMn MEM:r:u64=$mem128                             ', 'MOD=MEM VL=128', 'VORPD_rxnxmx', '        '];
T['VORPD VREG:w:u64=$YMMr VREG:r:u64=$YMMn MEM:r:u64=$mem256                             ', 'MOD=MEM VL=256', 'VORPD_rxnxmx', '        '];
T['VORPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG  ', 'VORPD_rynymy', '        '];
T['VORPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM       ', 'VORPD_rynymy', 'tuple=FV'];
T['VORPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG  ', 'VORPD_rznzmz', '        '];
T['VORPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM       ', 'VORPD_rznzmz', 'tuple=FV'];

# ORPS-Bitwise Logical OR of Packed Single Precision Floating-Point Values.
T['VORPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG  ', 'VORPS_ronomo', '        '];
T['VORPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM       ', 'VORPS_ronomo', 'tuple=FV'];
T['VORPS VREG:w:u32=$XMMr VREG:r:u32=$XMMn VREG:r:u32=$XMMm                              ', 'MOD=REG VL=128', 'VORPS_rxnxmx', '        '];
T['VORPS VREG:w:u32=$YMMr VREG:r:u32=$YMMn VREG:r:u32=$YMMm                              ', 'MOD=REG VL=256', 'VORPS_rxnxmx', '        '];
T['VORPS VREG:w:u32=$XMMr VREG:r:u32=$XMMn MEM:r:u32=$mem128                             ', 'MOD=MEM VL=128', 'VORPS_rxnxmx', '        '];
T['VORPS VREG:w:u32=$YMMr VREG:r:u32=$YMMn MEM:r:u32=$mem256                             ', 'MOD=MEM VL=256', 'VORPS_rxnxmx', '        '];
T['VORPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG  ', 'VORPS_rynymy', '        '];
T['VORPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM       ', 'VORPS_rynymy', 'tuple=FV'];
T['VORPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG  ', 'VORPS_rznzmz', '        '];
T['VORPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM       ', 'VORPS_rznzmz', 'tuple=FV'];

# PABSB/PABSW/PABSD/PABSQ-Packed Absolute Value.
T['VPABSB VREG:w:u8=$XMMr  VREG:r:s8=$XMMm                               ', 'MOD=REG     ', 'VPABSB_romo_1', '         '];
T['VPABSB VREG:w:u8=$XMMr  MEM:r:s8=$mem128                              ', 'MOD=MEM     ', 'VPABSB_romo_1', '         '];
T['VPABSB VREG:w:s8=$XMMr  PREG:r:Z=$Kk      VREG:r:s8=$XMMm             ', 'MOD=REG     ', 'VPABSB_romo_2', '         '];
T['VPABSB VREG:w:s8=$XMMr  PREG:r:Z=$Kk      MEM:r:s8=$mem128            ', 'MOD=MEM     ', 'VPABSB_romo_2', 'tuple=FVM'];
T['VPABSB VREG:w:u8=$YMMr  VREG:r:s8=$YMMm                               ', 'MOD=REG     ', 'VPABSB_rymy_1', '         '];
T['VPABSB VREG:w:u8=$YMMr  MEM:r:s8=$mem256                              ', 'MOD=MEM     ', 'VPABSB_rymy_1', '         '];
T['VPABSB VREG:w:s8=$YMMr  PREG:r:Z=$Kk      VREG:r:s8=$YMMm             ', 'MOD=REG     ', 'VPABSB_rymy_2', '         '];
T['VPABSB VREG:w:s8=$YMMr  PREG:r:Z=$Kk      MEM:r:s8=$mem256            ', 'MOD=MEM     ', 'VPABSB_rymy_2', 'tuple=FVM'];
T['VPABSB VREG:w:s8=$ZMMr  PREG:r:Z=$Kk      VREG:r:s8=$ZMMm             ', 'MOD=REG     ', 'VPABSB_rzmz  ', '         '];
T['VPABSB VREG:w:s8=$ZMMr  PREG:r:Z=$Kk      MEM:r:s8=$mem512            ', 'MOD=MEM     ', 'VPABSB_rzmz  ', 'tuple=FVM'];
T['VPABSD VREG:w:u32=$XMMr VREG:r:s32=$XMMm                              ', 'MOD=REG     ', 'VPABSD_romo_1', '         '];
T['VPABSD VREG:w:u32=$XMMr MEM:r:s32=$mem128                             ', 'MOD=MEM     ', 'VPABSD_romo_1', '         '];
T['VPABSD VREG:w:s32=$XMMr PREG:r:Z=$Kk      VREG:r:s32=$XMMm            ', 'BP=0 MOD=REG', 'VPABSD_romo_2', '         '];
T['VPABSD VREG:w:s32=$XMMr PREG:r:Z=$Kk      MEM:r:s32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPABSD_romo_2', 'tuple=FV '];
T['VPABSD VREG:w:u32=$YMMr VREG:r:s32=$YMMm                              ', 'MOD=REG     ', 'VPABSD_rymy_1', '         '];
T['VPABSD VREG:w:u32=$YMMr MEM:r:s32=$mem256                             ', 'MOD=MEM     ', 'VPABSD_rymy_1', '         '];
T['VPABSD VREG:w:s32=$YMMr PREG:r:Z=$Kk      VREG:r:s32=$YMMm            ', 'BP=0 MOD=REG', 'VPABSD_rymy_2', '         '];
T['VPABSD VREG:w:s32=$YMMr PREG:r:Z=$Kk      MEM:r:s32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPABSD_rymy_2', 'tuple=FV '];
T['VPABSD VREG:w:s32=$ZMMr PREG:r:Z=$Kk      VREG:r:s32=$ZMMm            ', 'BP=0 MOD=REG', 'VPABSD_rzmz  ', '         '];
T['VPABSD VREG:w:s32=$ZMMr PREG:r:Z=$Kk      MEM:r:s32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPABSD_rzmz  ', 'tuple=FV '];
T['VPABSQ VREG:w:s64=$XMMr PREG:r:Z=$Kk      VREG:r:s64=$XMMm            ', 'BP=0 MOD=REG', 'VPABSQ_romo  ', '         '];
T['VPABSQ VREG:w:s64=$XMMr PREG:r:Z=$Kk      MEM:r:s64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPABSQ_romo  ', 'tuple=FV '];
T['VPABSQ VREG:w:s64=$YMMr PREG:r:Z=$Kk      VREG:r:s64=$YMMm            ', 'BP=0 MOD=REG', 'VPABSQ_rymy  ', '         '];
T['VPABSQ VREG:w:s64=$YMMr PREG:r:Z=$Kk      MEM:r:s64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPABSQ_rymy  ', 'tuple=FV '];
T['VPABSQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk      VREG:r:s64=$ZMMm            ', 'BP=0 MOD=REG', 'VPABSQ_rzmz  ', '         '];
T['VPABSQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk      MEM:r:s64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPABSQ_rzmz  ', 'tuple=FV '];
T['VPABSW VREG:w:u16=$XMMr VREG:r:s16=$XMMm                              ', 'MOD=REG     ', 'VPABSW_romo_1', '         '];
T['VPABSW VREG:w:u16=$XMMr MEM:r:s16=$mem128                             ', 'MOD=MEM     ', 'VPABSW_romo_1', '         '];
T['VPABSW VREG:w:s16=$XMMr PREG:r:Z=$Kk      VREG:r:s16=$XMMm            ', 'MOD=REG     ', 'VPABSW_romo_2', '         '];
T['VPABSW VREG:w:s16=$XMMr PREG:r:Z=$Kk      MEM:r:s16=$mem128           ', 'MOD=MEM     ', 'VPABSW_romo_2', 'tuple=FVM'];
T['VPABSW VREG:w:u16=$YMMr VREG:r:s16=$YMMm                              ', 'MOD=REG     ', 'VPABSW_rymy_1', '         '];
T['VPABSW VREG:w:u16=$YMMr MEM:r:s16=$mem256                             ', 'MOD=MEM     ', 'VPABSW_rymy_1', '         '];
T['VPABSW VREG:w:s16=$YMMr PREG:r:Z=$Kk      VREG:r:s16=$YMMm            ', 'MOD=REG     ', 'VPABSW_rymy_2', '         '];
T['VPABSW VREG:w:s16=$YMMr PREG:r:Z=$Kk      MEM:r:s16=$mem256           ', 'MOD=MEM     ', 'VPABSW_rymy_2', 'tuple=FVM'];
T['VPABSW VREG:w:s16=$ZMMr PREG:r:Z=$Kk      VREG:r:s16=$ZMMm            ', 'MOD=REG     ', 'VPABSW_rzmz  ', '         '];
T['VPABSW VREG:w:s16=$ZMMr PREG:r:Z=$Kk      MEM:r:s16=$mem512           ', 'MOD=MEM     ', 'VPABSW_rzmz  ', 'tuple=FVM'];

# PACKSSWB/PACKSSDW-Pack with Signed Saturation.
T['VPACKSSDW VREG:w:s16=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm                              ', 'MOD=REG     ', 'VPACKSSDW_ronomo_1', '         '];
T['VPACKSSDW VREG:w:s16=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128                             ', 'MOD=MEM     ', 'VPACKSSDW_ronomo_1', '         '];
T['VPACKSSDW VREG:w:s16=$XMMr PREG:r:Z=$Kk     VREG:r:s32=$XMMn  VREG:r:s32=$XMMm            ', 'BP=0 MOD=REG', 'VPACKSSDW_ronomo_2', '         '];
T['VPACKSSDW VREG:w:s16=$XMMr PREG:r:Z=$Kk     VREG:r:s32=$XMMn  MEM:r:s32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPACKSSDW_ronomo_2', 'tuple=FV '];
T['VPACKSSDW VREG:w:s16=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm                              ', 'MOD=REG     ', 'VPACKSSDW_rynymy_1', '         '];
T['VPACKSSDW VREG:w:s16=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256                             ', 'MOD=MEM     ', 'VPACKSSDW_rynymy_1', '         '];
T['VPACKSSDW VREG:w:s16=$YMMr PREG:r:Z=$Kk     VREG:r:s32=$YMMn  VREG:r:s32=$YMMm            ', 'BP=0 MOD=REG', 'VPACKSSDW_rynymy_2', '         '];
T['VPACKSSDW VREG:w:s16=$YMMr PREG:r:Z=$Kk     VREG:r:s32=$YMMn  MEM:r:s32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPACKSSDW_rynymy_2', 'tuple=FV '];
T['VPACKSSDW VREG:w:s16=$ZMMr PREG:r:Z=$Kk     VREG:r:s32=$ZMMn  VREG:r:s32=$ZMMm            ', 'BP=0 MOD=REG', 'VPACKSSDW_rznzmz  ', '         '];
T['VPACKSSDW VREG:w:s16=$ZMMr PREG:r:Z=$Kk     VREG:r:s32=$ZMMn  MEM:r:s32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPACKSSDW_rznzmz  ', 'tuple=FV '];
T['VPACKSSWB VREG:w:s8=$XMMr  VREG:r:s16=$XMMn VREG:r:s16=$XMMm                              ', 'MOD=REG     ', 'VPACKSSWB_ronomo_1', '         '];
T['VPACKSSWB VREG:w:s8=$XMMr  VREG:r:s16=$XMMn MEM:r:s16=$mem128                             ', 'MOD=MEM     ', 'VPACKSSWB_ronomo_1', '         '];
T['VPACKSSWB VREG:w:s8=$XMMr  PREG:r:Z=$Kk     VREG:r:s16=$XMMn  VREG:r:s16=$XMMm            ', 'MOD=REG     ', 'VPACKSSWB_ronomo_2', '         '];
T['VPACKSSWB VREG:w:s8=$XMMr  PREG:r:Z=$Kk     VREG:r:s16=$XMMn  MEM:r:s16=$mem128           ', 'MOD=MEM     ', 'VPACKSSWB_ronomo_2', 'tuple=FVM'];
T['VPACKSSWB VREG:w:s8=$YMMr  VREG:r:s16=$YMMn VREG:r:s16=$YMMm                              ', 'MOD=REG     ', 'VPACKSSWB_rynymy_1', '         '];
T['VPACKSSWB VREG:w:s8=$YMMr  VREG:r:s16=$YMMn MEM:r:s16=$mem256                             ', 'MOD=MEM     ', 'VPACKSSWB_rynymy_1', '         '];
T['VPACKSSWB VREG:w:s8=$YMMr  PREG:r:Z=$Kk     VREG:r:s16=$YMMn  VREG:r:s16=$YMMm            ', 'MOD=REG     ', 'VPACKSSWB_rynymy_2', '         '];
T['VPACKSSWB VREG:w:s8=$YMMr  PREG:r:Z=$Kk     VREG:r:s16=$YMMn  MEM:r:s16=$mem256           ', 'MOD=MEM     ', 'VPACKSSWB_rynymy_2', 'tuple=FVM'];
T['VPACKSSWB VREG:w:s8=$ZMMr  PREG:r:Z=$Kk     VREG:r:s16=$ZMMn  VREG:r:s16=$ZMMm            ', 'MOD=REG     ', 'VPACKSSWB_rznzmz  ', '         '];
T['VPACKSSWB VREG:w:s8=$ZMMr  PREG:r:Z=$Kk     VREG:r:s16=$ZMMn  MEM:r:s16=$mem512           ', 'MOD=MEM     ', 'VPACKSSWB_rznzmz  ', 'tuple=FVM'];

# PACKUSDW-Pack with Unsigned Saturation.
T['VPACKUSDW VREG:w:u16=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm                              ', 'MOD=REG     ', 'VPACKUSDW_ronomo_1', '        '];
T['VPACKUSDW VREG:w:u16=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128                             ', 'MOD=MEM     ', 'VPACKUSDW_ronomo_1', '        '];
T['VPACKUSDW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VPACKUSDW_ronomo_2', '        '];
T['VPACKUSDW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPACKUSDW_ronomo_2', 'tuple=FV'];
T['VPACKUSDW VREG:w:u16=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm                              ', 'MOD=REG     ', 'VPACKUSDW_rynymy_1', '        '];
T['VPACKUSDW VREG:w:u16=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256                             ', 'MOD=MEM     ', 'VPACKUSDW_rynymy_1', '        '];
T['VPACKUSDW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VPACKUSDW_rynymy_2', '        '];
T['VPACKUSDW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPACKUSDW_rynymy_2', 'tuple=FV'];
T['VPACKUSDW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VPACKUSDW_rznzmz  ', '        '];
T['VPACKUSDW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPACKUSDW_rznzmz  ', 'tuple=FV'];

# PACKUSWB-Pack with Unsigned Saturation.
T['VPACKUSWB VREG:w:u8=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm                   ', 'MOD=REG', 'VPACKUSWB_ronomo_1', '         '];
T['VPACKUSWB VREG:w:u8=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128                  ', 'MOD=MEM', 'VPACKUSWB_ronomo_1', '         '];
T['VPACKUSWB VREG:w:u8=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm ', 'MOD=REG', 'VPACKUSWB_ronomo_2', '         '];
T['VPACKUSWB VREG:w:u8=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128', 'MOD=MEM', 'VPACKUSWB_ronomo_2', 'tuple=FVM'];
T['VPACKUSWB VREG:w:u8=$YMMr VREG:r:s16=$YMMn VREG:r:s16=$YMMm                   ', 'MOD=REG', 'VPACKUSWB_rynymy_1', '         '];
T['VPACKUSWB VREG:w:u8=$YMMr VREG:r:s16=$YMMn MEM:r:s16=$mem256                  ', 'MOD=MEM', 'VPACKUSWB_rynymy_1', '         '];
T['VPACKUSWB VREG:w:u8=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm ', 'MOD=REG', 'VPACKUSWB_rynymy_2', '         '];
T['VPACKUSWB VREG:w:u8=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256', 'MOD=MEM', 'VPACKUSWB_rynymy_2', 'tuple=FVM'];
T['VPACKUSWB VREG:w:u8=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm ', 'MOD=REG', 'VPACKUSWB_rznzmz  ', '         '];
T['VPACKUSWB VREG:w:u8=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512', 'MOD=MEM', 'VPACKUSWB_rznzmz  ', 'tuple=FVM'];

# PADDSB/PADDSW-Add Packed Signed Integers with Signed Saturation.
T['VPADDSB VREG:w:s8=$XMMr  VREG:r:s8=$XMMn  VREG:r:s8=$XMMm                    ', 'MOD=REG', 'VPADDSB_ronomo_1', '         '];
T['VPADDSB VREG:w:s8=$XMMr  VREG:r:s8=$XMMn  MEM:r:s8=$mem128                   ', 'MOD=MEM', 'VPADDSB_ronomo_1', '         '];
T['VPADDSB VREG:w:s8=$XMMr  PREG:r:Z=$Kk     VREG:r:s8=$XMMn   VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPADDSB_ronomo_2', '         '];
T['VPADDSB VREG:w:s8=$XMMr  PREG:r:Z=$Kk     VREG:r:s8=$XMMn   MEM:r:s8=$mem128 ', 'MOD=MEM', 'VPADDSB_ronomo_2', 'tuple=FVM'];
T['VPADDSB VREG:w:s8=$YMMr  VREG:r:s8=$YMMn  VREG:r:s8=$YMMm                    ', 'MOD=REG', 'VPADDSB_rynymy_1', '         '];
T['VPADDSB VREG:w:s8=$YMMr  VREG:r:s8=$YMMn  MEM:r:s8=$mem256                   ', 'MOD=MEM', 'VPADDSB_rynymy_1', '         '];
T['VPADDSB VREG:w:s8=$YMMr  PREG:r:Z=$Kk     VREG:r:s8=$YMMn   VREG:r:s8=$YMMm  ', 'MOD=REG', 'VPADDSB_rynymy_2', '         '];
T['VPADDSB VREG:w:s8=$YMMr  PREG:r:Z=$Kk     VREG:r:s8=$YMMn   MEM:r:s8=$mem256 ', 'MOD=MEM', 'VPADDSB_rynymy_2', 'tuple=FVM'];
T['VPADDSB VREG:w:s8=$ZMMr  PREG:r:Z=$Kk     VREG:r:s8=$ZMMn   VREG:r:s8=$ZMMm  ', 'MOD=REG', 'VPADDSB_rznzmz  ', '         '];
T['VPADDSB VREG:w:s8=$ZMMr  PREG:r:Z=$Kk     VREG:r:s8=$ZMMn   MEM:r:s8=$mem512 ', 'MOD=MEM', 'VPADDSB_rznzmz  ', 'tuple=FVM'];
T['VPADDSW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm                   ', 'MOD=REG', 'VPADDSW_ronomo_1', '         '];
T['VPADDSW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128                  ', 'MOD=MEM', 'VPADDSW_ronomo_1', '         '];
T['VPADDSW VREG:w:s16=$XMMr PREG:r:Z=$Kk     VREG:r:s16=$XMMn  VREG:r:s16=$XMMm ', 'MOD=REG', 'VPADDSW_ronomo_2', '         '];
T['VPADDSW VREG:w:s16=$XMMr PREG:r:Z=$Kk     VREG:r:s16=$XMMn  MEM:r:s16=$mem128', 'MOD=MEM', 'VPADDSW_ronomo_2', 'tuple=FVM'];
T['VPADDSW VREG:w:s16=$YMMr VREG:r:s16=$YMMn VREG:r:s16=$YMMm                   ', 'MOD=REG', 'VPADDSW_rynymy_1', '         '];
T['VPADDSW VREG:w:s16=$YMMr VREG:r:s16=$YMMn MEM:r:s16=$mem256                  ', 'MOD=MEM', 'VPADDSW_rynymy_1', '         '];
T['VPADDSW VREG:w:s16=$YMMr PREG:r:Z=$Kk     VREG:r:s16=$YMMn  VREG:r:s16=$YMMm ', 'MOD=REG', 'VPADDSW_rynymy_2', '         '];
T['VPADDSW VREG:w:s16=$YMMr PREG:r:Z=$Kk     VREG:r:s16=$YMMn  MEM:r:s16=$mem256', 'MOD=MEM', 'VPADDSW_rynymy_2', 'tuple=FVM'];
T['VPADDSW VREG:w:s16=$ZMMr PREG:r:Z=$Kk     VREG:r:s16=$ZMMn  VREG:r:s16=$ZMMm ', 'MOD=REG', 'VPADDSW_rznzmz  ', '         '];
T['VPADDSW VREG:w:s16=$ZMMr PREG:r:Z=$Kk     VREG:r:s16=$ZMMn  MEM:r:s16=$mem512', 'MOD=MEM', 'VPADDSW_rznzmz  ', 'tuple=FVM'];

# PADDUSB/PADDUSW-Add Packed Unsigned Integers with Unsigned Saturation.
T['VPADDUSB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  VREG:r:u8=$XMMm                    ', 'MOD=REG', 'VPADDUSB_ronomo_1', '         '];
T['VPADDUSB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  MEM:r:u8=$mem128                   ', 'MOD=MEM', 'VPADDUSB_ronomo_1', '         '];
T['VPADDUSB VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   VREG:r:u8=$XMMm  ', 'MOD=REG', 'VPADDUSB_ronomo_2', '         '];
T['VPADDUSB VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   MEM:r:u8=$mem128 ', 'MOD=MEM', 'VPADDUSB_ronomo_2', 'tuple=FVM'];
T['VPADDUSB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  VREG:r:u8=$YMMm                    ', 'MOD=REG', 'VPADDUSB_rynymy_1', '         '];
T['VPADDUSB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  MEM:r:u8=$mem256                   ', 'MOD=MEM', 'VPADDUSB_rynymy_1', '         '];
T['VPADDUSB VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   VREG:r:u8=$YMMm  ', 'MOD=REG', 'VPADDUSB_rynymy_2', '         '];
T['VPADDUSB VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   MEM:r:u8=$mem256 ', 'MOD=MEM', 'VPADDUSB_rynymy_2', 'tuple=FVM'];
T['VPADDUSB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   VREG:r:u8=$ZMMm  ', 'MOD=REG', 'VPADDUSB_rznzmz  ', '         '];
T['VPADDUSB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   MEM:r:u8=$mem512 ', 'MOD=MEM', 'VPADDUSB_rznzmz  ', 'tuple=FVM'];
T['VPADDUSW VREG:w:u16=$XMMr VREG:r:u16=$XMMn VREG:r:u16=$XMMm                   ', 'MOD=REG', 'VPADDUSW_ronomo_1', '         '];
T['VPADDUSW VREG:w:u16=$XMMr VREG:r:u16=$XMMn MEM:r:u16=$mem128                  ', 'MOD=MEM', 'VPADDUSW_ronomo_1', '         '];
T['VPADDUSW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm ', 'MOD=REG', 'VPADDUSW_ronomo_2', '         '];
T['VPADDUSW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128', 'MOD=MEM', 'VPADDUSW_ronomo_2', 'tuple=FVM'];
T['VPADDUSW VREG:w:u16=$YMMr VREG:r:u16=$YMMn VREG:r:u16=$YMMm                   ', 'MOD=REG', 'VPADDUSW_rynymy_1', '         '];
T['VPADDUSW VREG:w:u16=$YMMr VREG:r:u16=$YMMn MEM:r:u16=$mem256                  ', 'MOD=MEM', 'VPADDUSW_rynymy_1', '         '];
T['VPADDUSW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm ', 'MOD=REG', 'VPADDUSW_rynymy_2', '         '];
T['VPADDUSW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256', 'MOD=MEM', 'VPADDUSW_rynymy_2', 'tuple=FVM'];
T['VPADDUSW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm ', 'MOD=REG', 'VPADDUSW_rznzmz  ', '         '];
T['VPADDUSW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512', 'MOD=MEM', 'VPADDUSW_rznzmz  ', 'tuple=FVM'];

# PADDB/PADDW/PADDD/PADDQ-Add Packed Integers.
T['VPADDB VREG:w:s8=$XMMr  VREG:r:s8=$XMMn  VREG:r:s8=$XMMm                               ', 'MOD=REG     ', 'VPADDB_ronomo_1', '         '];
T['VPADDB VREG:w:s8=$XMMr  VREG:r:s8=$XMMn  MEM:r:s8=$mem128                              ', 'MOD=MEM     ', 'VPADDB_ronomo_1', '         '];
T['VPADDB VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   VREG:r:u8=$XMMm             ', 'MOD=REG     ', 'VPADDB_ronomo_2', '         '];
T['VPADDB VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   MEM:r:u8=$mem128            ', 'MOD=MEM     ', 'VPADDB_ronomo_2', 'tuple=FVM'];
T['VPADDB VREG:w:s8=$YMMr  VREG:r:s8=$YMMn  VREG:r:s8=$YMMm                               ', 'MOD=REG     ', 'VPADDB_rynymy_1', '         '];
T['VPADDB VREG:w:s8=$YMMr  VREG:r:s8=$YMMn  MEM:r:s8=$mem256                              ', 'MOD=MEM     ', 'VPADDB_rynymy_1', '         '];
T['VPADDB VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   VREG:r:u8=$YMMm             ', 'MOD=REG     ', 'VPADDB_rynymy_2', '         '];
T['VPADDB VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   MEM:r:u8=$mem256            ', 'MOD=MEM     ', 'VPADDB_rynymy_2', 'tuple=FVM'];
T['VPADDB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   VREG:r:u8=$ZMMm             ', 'MOD=REG     ', 'VPADDB_rznzmz  ', '         '];
T['VPADDB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   MEM:r:u8=$mem512            ', 'MOD=MEM     ', 'VPADDB_rznzmz  ', 'tuple=FVM'];
T['VPADDD VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm                              ', 'MOD=REG     ', 'VPADDD_ronomo_1', '         '];
T['VPADDD VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128                             ', 'MOD=MEM     ', 'VPADDD_ronomo_1', '         '];
T['VPADDD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VPADDD_ronomo_2', '         '];
T['VPADDD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPADDD_ronomo_2', 'tuple=FV '];
T['VPADDD VREG:w:s32=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm                              ', 'MOD=REG     ', 'VPADDD_rynymy_1', '         '];
T['VPADDD VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256                             ', 'MOD=MEM     ', 'VPADDD_rynymy_1', '         '];
T['VPADDD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VPADDD_rynymy_2', '         '];
T['VPADDD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPADDD_rynymy_2', 'tuple=FV '];
T['VPADDD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VPADDD_rznzmz  ', '         '];
T['VPADDD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPADDD_rznzmz  ', 'tuple=FV '];
T['VPADDQ VREG:w:s64=$XMMr VREG:r:s64=$XMMn VREG:r:s64=$XMMm                              ', 'MOD=REG     ', 'VPADDQ_ronomo_1', '         '];
T['VPADDQ VREG:w:s64=$XMMr VREG:r:s64=$XMMn MEM:r:s64=$mem128                             ', 'MOD=MEM     ', 'VPADDQ_ronomo_1', '         '];
T['VPADDQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG', 'VPADDQ_ronomo_2', '         '];
T['VPADDQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPADDQ_ronomo_2', 'tuple=FV '];
T['VPADDQ VREG:w:s64=$YMMr VREG:r:s64=$YMMn VREG:r:s64=$YMMm                              ', 'MOD=REG     ', 'VPADDQ_rynymy_1', '         '];
T['VPADDQ VREG:w:s64=$YMMr VREG:r:s64=$YMMn MEM:r:s64=$mem256                             ', 'MOD=MEM     ', 'VPADDQ_rynymy_1', '         '];
T['VPADDQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG', 'VPADDQ_rynymy_2', '         '];
T['VPADDQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPADDQ_rynymy_2', 'tuple=FV '];
T['VPADDQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG', 'VPADDQ_rznzmz  ', '         '];
T['VPADDQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPADDQ_rznzmz  ', 'tuple=FV '];
T['VPADDW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm                              ', 'MOD=REG     ', 'VPADDW_ronomo_1', '         '];
T['VPADDW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128                             ', 'MOD=MEM     ', 'VPADDW_ronomo_1', '         '];
T['VPADDW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm            ', 'MOD=REG     ', 'VPADDW_ronomo_2', '         '];
T['VPADDW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128           ', 'MOD=MEM     ', 'VPADDW_ronomo_2', 'tuple=FVM'];
T['VPADDW VREG:w:s16=$YMMr VREG:r:s16=$YMMn VREG:r:s16=$YMMm                              ', 'MOD=REG     ', 'VPADDW_rynymy_1', '         '];
T['VPADDW VREG:w:s16=$YMMr VREG:r:s16=$YMMn MEM:r:s16=$mem256                             ', 'MOD=MEM     ', 'VPADDW_rynymy_1', '         '];
T['VPADDW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm            ', 'MOD=REG     ', 'VPADDW_rynymy_2', '         '];
T['VPADDW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256           ', 'MOD=MEM     ', 'VPADDW_rynymy_2', 'tuple=FVM'];
T['VPADDW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm            ', 'MOD=REG     ', 'VPADDW_rznzmz  ', '         '];
T['VPADDW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512           ', 'MOD=MEM     ', 'VPADDW_rznzmz  ', 'tuple=FVM'];

# PALIGNR-Packed Align Right.
T['VPALIGNR VREG:w:u8=$XMMr VREG:r:u8=$XMMn VREG:r:u8=$XMMm  IMM:u8=$uimm8                 ', 'MOD=REG', 'VPALIGNR_ronomoub_1', '         '];
T['VPALIGNR VREG:w:u8=$XMMr VREG:r:u8=$XMMn MEM:r:u8=$mem128 IMM:u8=$uimm8                 ', 'MOD=MEM', 'VPALIGNR_ronomoub_1', '         '];
T['VPALIGNR VREG:w:u8=$XMMr PREG:r:Z=$Kk    VREG:r:u8=$XMMn  VREG:r:u8=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPALIGNR_ronomoub_2', '         '];
T['VPALIGNR VREG:w:u8=$XMMr PREG:r:Z=$Kk    VREG:r:u8=$XMMn  MEM:r:u8=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'VPALIGNR_ronomoub_2', 'tuple=FVM'];
T['VPALIGNR VREG:w:u8=$YMMr VREG:r:u8=$YMMn VREG:r:u8=$YMMm  IMM:u8=$uimm8                 ', 'MOD=REG', 'VPALIGNR_rynymyub_1', '         '];
T['VPALIGNR VREG:w:u8=$YMMr VREG:r:u8=$YMMn MEM:r:u8=$mem256 IMM:u8=$uimm8                 ', 'MOD=MEM', 'VPALIGNR_rynymyub_1', '         '];
T['VPALIGNR VREG:w:u8=$YMMr PREG:r:Z=$Kk    VREG:r:u8=$YMMn  VREG:r:u8=$YMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPALIGNR_rynymyub_2', '         '];
T['VPALIGNR VREG:w:u8=$YMMr PREG:r:Z=$Kk    VREG:r:u8=$YMMn  MEM:r:u8=$mem256 IMM:u8=$uimm8', 'MOD=MEM', 'VPALIGNR_rynymyub_2', 'tuple=FVM'];
T['VPALIGNR VREG:w:u8=$ZMMr PREG:r:Z=$Kk    VREG:r:u8=$ZMMn  VREG:r:u8=$ZMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPALIGNR_rznzmzub  ', '         '];
T['VPALIGNR VREG:w:u8=$ZMMr PREG:r:Z=$Kk    VREG:r:u8=$ZMMn  MEM:r:u8=$mem512 IMM:u8=$uimm8', 'MOD=MEM', 'VPALIGNR_rznzmzub  ', 'tuple=FVM'];

# PAND-Logical AND.
T['VPAND REG:w:u128=$XMMr REG:r:u128=$XMMn REG:r:u128=$XMMm  ', 'MOD=REG', 'VPAND_ronomo', ''];
T['VPAND REG:w:u128=$XMMr REG:r:u128=$XMMn MEM:r:u128=$mem128', 'MOD=MEM', 'VPAND_ronomo', ''];
T['VPAND REG:w:u256=$YMMr REG:r:u256=$YMMn REG:r:u256=$YMMm  ', 'MOD=REG', 'VPAND_rynymy', ''];
T['VPAND REG:w:u256=$YMMr REG:r:u256=$YMMn MEM:r:u256=$mem256', 'MOD=MEM', 'VPAND_rynymy', ''];

# PANDN-Logical AND NOT.
T['VPANDN REG:w:u128=$XMMr REG:r:u128=$XMMn REG:r:u128=$XMMm  ', 'MOD=REG', 'VPANDN_ronomo', ''];
T['VPANDN REG:w:u128=$XMMr REG:r:u128=$XMMn MEM:r:u128=$mem128', 'MOD=MEM', 'VPANDN_ronomo', ''];
T['VPANDN REG:w:u256=$YMMr REG:r:u256=$YMMn REG:r:u256=$YMMm  ', 'MOD=REG', 'VPANDN_rynymy', ''];
T['VPANDN REG:w:u256=$YMMr REG:r:u256=$YMMn MEM:r:u256=$mem256', 'MOD=MEM', 'VPANDN_rynymy', ''];

# PAVGB/PAVGW-Average Packed Integers.
T['VPAVGB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  VREG:r:u8=$XMMm                    ', 'MOD=REG', 'VPAVGB_ronomo_1', '         '];
T['VPAVGB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  MEM:r:u8=$mem128                   ', 'MOD=MEM', 'VPAVGB_ronomo_1', '         '];
T['VPAVGB VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   VREG:r:u8=$XMMm  ', 'MOD=REG', 'VPAVGB_ronomo_2', '         '];
T['VPAVGB VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   MEM:r:u8=$mem128 ', 'MOD=MEM', 'VPAVGB_ronomo_2', 'tuple=FVM'];
T['VPAVGB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  VREG:r:u8=$YMMm                    ', 'MOD=REG', 'VPAVGB_rynymy_1', '         '];
T['VPAVGB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  MEM:r:u8=$mem256                   ', 'MOD=MEM', 'VPAVGB_rynymy_1', '         '];
T['VPAVGB VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   VREG:r:u8=$YMMm  ', 'MOD=REG', 'VPAVGB_rynymy_2', '         '];
T['VPAVGB VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   MEM:r:u8=$mem256 ', 'MOD=MEM', 'VPAVGB_rynymy_2', 'tuple=FVM'];
T['VPAVGB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   VREG:r:u8=$ZMMm  ', 'MOD=REG', 'VPAVGB_rznzmz  ', '         '];
T['VPAVGB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   MEM:r:u8=$mem512 ', 'MOD=MEM', 'VPAVGB_rznzmz  ', 'tuple=FVM'];
T['VPAVGW VREG:w:u16=$XMMr VREG:r:u16=$XMMn VREG:r:u16=$XMMm                   ', 'MOD=REG', 'VPAVGW_ronomo_1', '         '];
T['VPAVGW VREG:w:u16=$XMMr VREG:r:u16=$XMMn MEM:r:u16=$mem128                  ', 'MOD=MEM', 'VPAVGW_ronomo_1', '         '];
T['VPAVGW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm ', 'MOD=REG', 'VPAVGW_ronomo_2', '         '];
T['VPAVGW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128', 'MOD=MEM', 'VPAVGW_ronomo_2', 'tuple=FVM'];
T['VPAVGW VREG:w:u16=$YMMr VREG:r:u16=$YMMn VREG:r:u16=$YMMm                   ', 'MOD=REG', 'VPAVGW_rynymy_1', '         '];
T['VPAVGW VREG:w:u16=$YMMr VREG:r:u16=$YMMn MEM:r:u16=$mem256                  ', 'MOD=MEM', 'VPAVGW_rynymy_1', '         '];
T['VPAVGW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm ', 'MOD=REG', 'VPAVGW_rynymy_2', '         '];
T['VPAVGW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256', 'MOD=MEM', 'VPAVGW_rynymy_2', 'tuple=FVM'];
T['VPAVGW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm ', 'MOD=REG', 'VPAVGW_rznzmz  ', '         '];
T['VPAVGW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512', 'MOD=MEM', 'VPAVGW_rznzmz  ', 'tuple=FVM'];

# PBLENDVB-Variable Blend Packed Bytes.
T['VPBLENDVB VREG:w:s8=$XMMr VREG:r:s8=$XMMn VREG:r:s8=$XMMm  VREG:r:s8=$XMMis4', 'MOD=REG', 'VPBLENDVB_ronomois4o', ''];
T['VPBLENDVB VREG:w:s8=$XMMr VREG:r:s8=$XMMn MEM:r:s8=$mem128 VREG:r:s8=$XMMis4', 'MOD=MEM', 'VPBLENDVB_ronomois4o', ''];
T['VPBLENDVB VREG:w:u8=$YMMr VREG:r:u8=$YMMn VREG:r:u8=$YMMm  VREG:r:u8=$YMMis4', 'MOD=REG', 'VPBLENDVB_rynymyis4y', ''];
T['VPBLENDVB VREG:w:u8=$YMMr VREG:r:u8=$YMMn MEM:r:u8=$mem256 VREG:r:u8=$YMMis4', 'MOD=MEM', 'VPBLENDVB_rynymyis4y', ''];

# PBLENDW-Blend Packed Words.
T['VPBLENDW VREG:w:u16=$XMMr VREG:r:u16=$XMMn VREG:r:u16=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPBLENDW_ronomoub', ''];
T['VPBLENDW VREG:w:u16=$XMMr VREG:r:u16=$XMMn MEM:r:u16=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'VPBLENDW_ronomoub', ''];
T['VPBLENDW VREG:w:u16=$YMMr VREG:r:u16=$YMMn VREG:r:u16=$YMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPBLENDW_rynymyub', ''];
T['VPBLENDW VREG:w:u16=$YMMr VREG:r:u16=$YMMn MEM:r:u16=$mem256 IMM:u8=$uimm8', 'MOD=MEM', 'VPBLENDW_rynymyub', ''];

# PCLMULQDQ-Carry-Less Multiplication Quadword.
T['VPCLMULQDQ REG:w:u128=$XMMr  VREG:r:u64=$XMMn VREG:r:u64=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPCLMULQDQ_ronomoub_1', '         '];
T['VPCLMULQDQ REG:w:u128=$XMMr  VREG:r:u64=$XMMn MEM:r:u64=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'VPCLMULQDQ_ronomoub_1', '         '];
T['VPCLMULQDQ REG:w:u128=$XMMr  VREG:r:u64=$XMMn VREG:r:u64=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPCLMULQDQ_ronomoub_2', '         '];
T['VPCLMULQDQ REG:w:u128=$XMMr  VREG:r:u64=$XMMn MEM:r:u64=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'VPCLMULQDQ_ronomoub_2', 'tuple=FVM'];
T['VPCLMULQDQ VREG:w:u128=$YMMr VREG:r:u64=$YMMn VREG:r:u64=$YMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPCLMULQDQ_rynymyub_2', '         '];
T['VPCLMULQDQ VREG:w:u128=$YMMr VREG:r:u64=$YMMn MEM:r:u64=$mem256 IMM:u8=$uimm8', 'MOD=MEM', 'VPCLMULQDQ_rynymyub_2', 'tuple=FVM'];
T['VPCLMULQDQ VREG:w:u128=$ZMMr VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPCLMULQDQ_rznzmzub  ', '         '];
T['VPCLMULQDQ VREG:w:u128=$ZMMr VREG:r:u64=$ZMMn MEM:r:u64=$mem512 IMM:u8=$uimm8', 'MOD=MEM', 'VPCLMULQDQ_rznzmzub  ', 'tuple=FVM'];

# PCMPEQQ-Compare Packed Qword Data for Equal.
T['VPCMPEQQ REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u64=$XMMn  VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG', 'VPCMPEQQ_nomo  ', '        '];
T['VPCMPEQQ REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u64=$XMMn  MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPCMPEQQ_nomo  ', 'tuple=FV'];
T['VPCMPEQQ REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u64=$YMMn  VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG', 'VPCMPEQQ_nymy  ', '        '];
T['VPCMPEQQ REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u64=$YMMn  MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPCMPEQQ_nymy  ', 'tuple=FV'];
T['VPCMPEQQ REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG', 'VPCMPEQQ_nzmz  ', '        '];
T['VPCMPEQQ REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPCMPEQQ_nzmz  ', 'tuple=FV'];
T['VPCMPEQQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn VREG:r:u64=$XMMm                              ', 'MOD=REG     ', 'VPCMPEQQ_ronomo', '        '];
T['VPCMPEQQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn MEM:r:u64=$mem128                             ', 'MOD=MEM     ', 'VPCMPEQQ_ronomo', '        '];
T['VPCMPEQQ VREG:w:u64=$YMMr VREG:r:u64=$YMMn VREG:r:u64=$YMMm                              ', 'MOD=REG     ', 'VPCMPEQQ_rynymy', '        '];
T['VPCMPEQQ VREG:w:u64=$YMMr VREG:r:u64=$YMMn MEM:r:u64=$mem256                             ', 'MOD=MEM     ', 'VPCMPEQQ_rynymy', '        '];

# PCMPEQB/PCMPEQW/PCMPEQD-Compare Packed Data for Equal.
T['VPCMPEQB REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u8=$XMMn   VREG:r:u8=$XMMm             ', 'MOD=REG     ', 'VPCMPEQB_nomo  ', '         '];
T['VPCMPEQB REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u8=$XMMn   MEM:r:u8=$mem128            ', 'MOD=MEM     ', 'VPCMPEQB_nomo  ', 'tuple=FVM'];
T['VPCMPEQB REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u8=$YMMn   VREG:r:u8=$YMMm             ', 'MOD=REG     ', 'VPCMPEQB_nymy  ', '         '];
T['VPCMPEQB REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u8=$YMMn   MEM:r:u8=$mem256            ', 'MOD=MEM     ', 'VPCMPEQB_nymy  ', 'tuple=FVM'];
T['VPCMPEQB REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   VREG:r:u8=$ZMMm             ', 'MOD=REG     ', 'VPCMPEQB_nzmz  ', '         '];
T['VPCMPEQB REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   MEM:r:u8=$mem512            ', 'MOD=MEM     ', 'VPCMPEQB_nzmz  ', 'tuple=FVM'];
T['VPCMPEQB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  VREG:r:u8=$XMMm                               ', 'MOD=REG     ', 'VPCMPEQB_ronomo', '         '];
T['VPCMPEQB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  MEM:r:u8=$mem128                              ', 'MOD=MEM     ', 'VPCMPEQB_ronomo', '         '];
T['VPCMPEQB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  VREG:r:u8=$YMMm                               ', 'MOD=REG     ', 'VPCMPEQB_rynymy', '         '];
T['VPCMPEQB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  MEM:r:u8=$mem256                              ', 'MOD=MEM     ', 'VPCMPEQB_rynymy', '         '];
T['VPCMPEQD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u32=$XMMn  VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VPCMPEQD_nomo  ', '         '];
T['VPCMPEQD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u32=$XMMn  MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPCMPEQD_nomo  ', 'tuple=FV '];
T['VPCMPEQD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u32=$YMMn  VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VPCMPEQD_nymy  ', '         '];
T['VPCMPEQD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u32=$YMMn  MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPCMPEQD_nymy  ', 'tuple=FV '];
T['VPCMPEQD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VPCMPEQD_nzmz  ', '         '];
T['VPCMPEQD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPCMPEQD_nzmz  ', 'tuple=FV '];
T['VPCMPEQD VREG:w:u32=$XMMr VREG:r:u32=$XMMn VREG:r:u32=$XMMm                              ', 'MOD=REG     ', 'VPCMPEQD_ronomo', '         '];
T['VPCMPEQD VREG:w:u32=$XMMr VREG:r:u32=$XMMn MEM:r:u32=$mem128                             ', 'MOD=MEM     ', 'VPCMPEQD_ronomo', '         '];
T['VPCMPEQD VREG:w:u32=$YMMr VREG:r:u32=$YMMn VREG:r:u32=$YMMm                              ', 'MOD=REG     ', 'VPCMPEQD_rynymy', '         '];
T['VPCMPEQD VREG:w:u32=$YMMr VREG:r:u32=$YMMn MEM:r:u32=$mem256                             ', 'MOD=MEM     ', 'VPCMPEQD_rynymy', '         '];
T['VPCMPEQW REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm            ', 'MOD=REG     ', 'VPCMPEQW_nomo  ', '         '];
T['VPCMPEQW REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128           ', 'MOD=MEM     ', 'VPCMPEQW_nomo  ', 'tuple=FVM'];
T['VPCMPEQW REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm            ', 'MOD=REG     ', 'VPCMPEQW_nymy  ', '         '];
T['VPCMPEQW REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256           ', 'MOD=MEM     ', 'VPCMPEQW_nymy  ', 'tuple=FVM'];
T['VPCMPEQW REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm            ', 'MOD=REG     ', 'VPCMPEQW_nzmz  ', '         '];
T['VPCMPEQW REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512           ', 'MOD=MEM     ', 'VPCMPEQW_nzmz  ', 'tuple=FVM'];
T['VPCMPEQW VREG:w:u16=$XMMr VREG:r:u16=$XMMn VREG:r:u16=$XMMm                              ', 'MOD=REG     ', 'VPCMPEQW_ronomo', '         '];
T['VPCMPEQW VREG:w:u16=$XMMr VREG:r:u16=$XMMn MEM:r:u16=$mem128                             ', 'MOD=MEM     ', 'VPCMPEQW_ronomo', '         '];
T['VPCMPEQW VREG:w:u16=$YMMr VREG:r:u16=$YMMn VREG:r:u16=$YMMm                              ', 'MOD=REG     ', 'VPCMPEQW_rynymy', '         '];
T['VPCMPEQW VREG:w:u16=$YMMr VREG:r:u16=$YMMn MEM:r:u16=$mem256                             ', 'MOD=MEM     ', 'VPCMPEQW_rynymy', '         '];

# PCMPESTRI-Packed Compare Explicit Length Strings, Return Index.
T['VPCMPESTRI VREG:r:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8 REG:SUPP:r=EAX REG:SUPP:r=EDX REG:SUPP:w=ECX', 'MOD=REG MODE=64 W=0', 'VPCMPESTRI', ''];
T['VPCMPESTRI VREG:r:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8 REG:SUPP:r=RAX REG:SUPP:r=RDX REG:SUPP:w=RCX', 'MOD=REG MODE=64 W=1', 'VPCMPESTRI', ''];
T['VPCMPESTRI VREG:r:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8 REG:SUPP:r=EAX REG:SUPP:r=EDX REG:SUPP:w=ECX', 'MOD=MEM MODE=64 W=0', 'VPCMPESTRI', ''];
T['VPCMPESTRI VREG:r:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8 REG:SUPP:r=RAX REG:SUPP:r=RDX REG:SUPP:w=RCX', 'MOD=MEM MODE=64 W=1', 'VPCMPESTRI', ''];
T['VPCMPESTRI VREG:r:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8 REG:SUPP:r=EAX REG:SUPP:r=EDX REG:SUPP:w=ECX', 'MOD=REG MODE=NO64  ', 'VPCMPESTRI', ''];
T['VPCMPESTRI VREG:r:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8 REG:SUPP:r=EAX REG:SUPP:r=EDX REG:SUPP:w=ECX', 'MOD=MEM MODE=NO64  ', 'VPCMPESTRI', ''];

# PCMPESTRM-Packed Compare Explicit Length Strings, Return Mask.
T['VPCMPESTRM VREG:r:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8 REG:SUPP:r=EAX REG:SUPP:r=EDX VREG:SUPP:w:s32=XMM0', 'MOD=REG MODE=64 W=0', 'VPCMPESTRM', ''];
T['VPCMPESTRM VREG:r:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8 REG:SUPP:r=RAX REG:SUPP:r=RDX VREG:SUPP:w:s32=XMM0', 'MOD=REG MODE=64 W=1', 'VPCMPESTRM', ''];
T['VPCMPESTRM VREG:r:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8 REG:SUPP:r=EAX REG:SUPP:r=EDX VREG:SUPP:w:s32=XMM0', 'MOD=MEM MODE=64 W=0', 'VPCMPESTRM', ''];
T['VPCMPESTRM VREG:r:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8 REG:SUPP:r=RAX REG:SUPP:r=RDX VREG:SUPP:w:s32=XMM0', 'MOD=MEM MODE=64 W=1', 'VPCMPESTRM', ''];
T['VPCMPESTRM VREG:r:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8 REG:SUPP:r=EAX REG:SUPP:r=EDX VREG:SUPP:w:s32=XMM0', 'MOD=REG MODE=NO64  ', 'VPCMPESTRM', ''];
T['VPCMPESTRM VREG:r:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8 REG:SUPP:r=EAX REG:SUPP:r=EDX VREG:SUPP:w:s32=XMM0', 'MOD=MEM MODE=NO64  ', 'VPCMPESTRM', ''];

# PCMPGTQ-Compare Packed Data for Greater Than.
T['VPCMPGTQ REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:s64=$XMMn  VREG:r:s64=$XMMm            ', 'BP=0 MOD=REG', 'VPCMPGTQ_nomo  ', '        '];
T['VPCMPGTQ REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:s64=$XMMn  MEM:r:s64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPCMPGTQ_nomo  ', 'tuple=FV'];
T['VPCMPGTQ REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:s64=$YMMn  VREG:r:s64=$YMMm            ', 'BP=0 MOD=REG', 'VPCMPGTQ_nymy  ', '        '];
T['VPCMPGTQ REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:s64=$YMMn  MEM:r:s64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPCMPGTQ_nymy  ', 'tuple=FV'];
T['VPCMPGTQ REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:s64=$ZMMn  VREG:r:s64=$ZMMm            ', 'BP=0 MOD=REG', 'VPCMPGTQ_nzmz  ', '        '];
T['VPCMPGTQ REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:s64=$ZMMn  MEM:r:s64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPCMPGTQ_nzmz  ', 'tuple=FV'];
T['VPCMPGTQ VREG:w:s32=$XMMr VREG:r:s64=$XMMn VREG:r:s64=$XMMm                              ', 'MOD=REG     ', 'VPCMPGTQ_ronomo', '        '];
T['VPCMPGTQ VREG:w:s32=$XMMr VREG:r:s64=$XMMn MEM:r:s64=$mem128                             ', 'MOD=MEM     ', 'VPCMPGTQ_ronomo', '        '];
T['VPCMPGTQ VREG:w:s64=$YMMr VREG:r:s64=$YMMn VREG:r:s64=$YMMm                              ', 'MOD=REG     ', 'VPCMPGTQ_rynymy', '        '];
T['VPCMPGTQ VREG:w:s64=$YMMr VREG:r:s64=$YMMn MEM:r:s64=$mem256                             ', 'MOD=MEM     ', 'VPCMPGTQ_rynymy', '        '];

# PCMPGTB/PCMPGTW/PCMPGTD-Compare Packed Signed Integers for Greater Than.
T['VPCMPGTB REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u8=$XMMn   VREG:r:u8=$XMMm             ', 'MOD=REG     ', 'VPCMPGTB_nomo  ', '         '];
T['VPCMPGTB REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u8=$XMMn   MEM:r:u8=$mem128            ', 'MOD=MEM     ', 'VPCMPGTB_nomo  ', 'tuple=FVM'];
T['VPCMPGTB REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u8=$YMMn   VREG:r:u8=$YMMm             ', 'MOD=REG     ', 'VPCMPGTB_nymy  ', '         '];
T['VPCMPGTB REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u8=$YMMn   MEM:r:u8=$mem256            ', 'MOD=MEM     ', 'VPCMPGTB_nymy  ', 'tuple=FVM'];
T['VPCMPGTB REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   VREG:r:u8=$ZMMm             ', 'MOD=REG     ', 'VPCMPGTB_nzmz  ', '         '];
T['VPCMPGTB REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   MEM:r:u8=$mem512            ', 'MOD=MEM     ', 'VPCMPGTB_nzmz  ', 'tuple=FVM'];
T['VPCMPGTB VREG:w:s8=$XMMr  VREG:r:s8=$XMMn  VREG:r:s8=$XMMm                               ', 'MOD=REG     ', 'VPCMPGTB_ronomo', '         '];
T['VPCMPGTB VREG:w:s8=$XMMr  VREG:r:s8=$XMMn  MEM:r:s8=$mem128                              ', 'MOD=MEM     ', 'VPCMPGTB_ronomo', '         '];
T['VPCMPGTB VREG:w:s8=$YMMr  VREG:r:s8=$YMMn  VREG:r:s8=$YMMm                               ', 'MOD=REG     ', 'VPCMPGTB_rynymy', '         '];
T['VPCMPGTB VREG:w:s8=$YMMr  VREG:r:s8=$YMMn  MEM:r:s8=$mem256                              ', 'MOD=MEM     ', 'VPCMPGTB_rynymy', '         '];
T['VPCMPGTD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:s32=$XMMn  VREG:r:s32=$XMMm            ', 'BP=0 MOD=REG', 'VPCMPGTD_nomo  ', '         '];
T['VPCMPGTD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:s32=$XMMn  MEM:r:s32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPCMPGTD_nomo  ', 'tuple=FV '];
T['VPCMPGTD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:s32=$YMMn  VREG:r:s32=$YMMm            ', 'BP=0 MOD=REG', 'VPCMPGTD_nymy  ', '         '];
T['VPCMPGTD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:s32=$YMMn  MEM:r:s32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPCMPGTD_nymy  ', 'tuple=FV '];
T['VPCMPGTD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:s32=$ZMMn  VREG:r:s32=$ZMMm            ', 'BP=0 MOD=REG', 'VPCMPGTD_nzmz  ', '         '];
T['VPCMPGTD REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:s32=$ZMMn  MEM:r:s32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPCMPGTD_nzmz  ', 'tuple=FV '];
T['VPCMPGTD VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm                              ', 'MOD=REG     ', 'VPCMPGTD_ronomo', '         '];
T['VPCMPGTD VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128                             ', 'MOD=MEM     ', 'VPCMPGTD_ronomo', '         '];
T['VPCMPGTD VREG:w:s32=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm                              ', 'MOD=REG     ', 'VPCMPGTD_rynymy', '         '];
T['VPCMPGTD VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256                             ', 'MOD=MEM     ', 'VPCMPGTD_rynymy', '         '];
T['VPCMPGTW REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm            ', 'MOD=REG     ', 'VPCMPGTW_nomo  ', '         '];
T['VPCMPGTW REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128           ', 'MOD=MEM     ', 'VPCMPGTW_nomo  ', 'tuple=FVM'];
T['VPCMPGTW REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm            ', 'MOD=REG     ', 'VPCMPGTW_nymy  ', '         '];
T['VPCMPGTW REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256           ', 'MOD=MEM     ', 'VPCMPGTW_nymy  ', 'tuple=FVM'];
T['VPCMPGTW REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm            ', 'MOD=REG     ', 'VPCMPGTW_nzmz  ', '         '];
T['VPCMPGTW REG:w=$Kr        PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512           ', 'MOD=MEM     ', 'VPCMPGTW_nzmz  ', 'tuple=FVM'];
T['VPCMPGTW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm                              ', 'MOD=REG     ', 'VPCMPGTW_ronomo', '         '];
T['VPCMPGTW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128                             ', 'MOD=MEM     ', 'VPCMPGTW_ronomo', '         '];
T['VPCMPGTW VREG:w:s16=$YMMr VREG:r:s16=$YMMn VREG:r:s16=$YMMm                              ', 'MOD=REG     ', 'VPCMPGTW_rynymy', '         '];
T['VPCMPGTW VREG:w:s16=$YMMr VREG:r:s16=$YMMn MEM:r:s16=$mem256                             ', 'MOD=MEM     ', 'VPCMPGTW_rynymy', '         '];

# PCMPISTRI-Packed Compare Implicit Length Strings, Return Index.
T['VPCMPISTRI VREG:r:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8 REG:SUPP:w=ECX', 'MOD=REG MODE=64 W=0', 'VPCMPISTRI', ''];
T['VPCMPISTRI VREG:r:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8 REG:SUPP:w=RCX', 'MOD=REG MODE=64 W=1', 'VPCMPISTRI', ''];
T['VPCMPISTRI VREG:r:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8 REG:SUPP:w=ECX', 'MOD=MEM MODE=64 W=0', 'VPCMPISTRI', ''];
T['VPCMPISTRI VREG:r:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8 REG:SUPP:w=RCX', 'MOD=MEM MODE=64 W=1', 'VPCMPISTRI', ''];
T['VPCMPISTRI VREG:r:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8 REG:SUPP:w=ECX', 'MOD=REG MODE=NO64  ', 'VPCMPISTRI', ''];
T['VPCMPISTRI VREG:r:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8 REG:SUPP:w=ECX', 'MOD=MEM MODE=NO64  ', 'VPCMPISTRI', ''];

# PCMPISTRM-Packed Compare Implicit Length Strings, Return Mask.
T['VPCMPISTRM VREG:r:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8 VREG:SUPP:w:s32=XMM0', 'MOD=REG', 'VPCMPISTRM', ''];
T['VPCMPISTRM VREG:r:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8 VREG:SUPP:w:s32=XMM0', 'MOD=MEM', 'VPCMPISTRM', ''];

# PEXTRW-Extract Word.
T['VPEXTRW REG:w:s32=$GPR32m VREG:r:u16=$XMMr IMM:u8=$index', 'MOD=REG', 'VPEXTRW_mxroub_1', '           '];
T['VPEXTRW MEM:w:s16=$mem16  VREG:r:u16=$XMMr IMM:u8=$index', 'MOD=MEM', 'VPEXTRW_mxroub_1', '           '];
T['VPEXTRW REG:w:u16=$GPR32m VREG:r:u16=$XMMr IMM:u8=$index', 'MOD=REG', 'VPEXTRW_mxroub_2', '           '];
T['VPEXTRW MEM:w:u16=$mem16  VREG:r:u16=$XMMr IMM:u8=$index', 'MOD=MEM', 'VPEXTRW_mxroub_2', 'tuple=T1S16'];
T['VPEXTRW REG:w:s32=$GPR32r VREG:r:u16=$XMMm IMM:u8=$index', 'NONE   ', 'VPEXTRW_rdmoub_1', '           '];
T['VPEXTRW REG:w:u16=$GPR32r VREG:r:u16=$XMMm IMM:u8=$index', 'NONE   ', 'VPEXTRW_rdmoub_2', '           '];

# PEXTRB/PEXTRD/PEXTRQ-Extract Byte/Dword/Qword.
T['VPEXTRB REG:w:s32=$GPR32m VREG:r:u8=$XMMr  IMM:u8=$index', 'MOD=REG              ', 'VPEXTRB_mxroub_1', '          '];
T['VPEXTRB MEM:w:u8=$mem8    VREG:r:u8=$XMMr  IMM:u8=$index', 'MOD=MEM              ', 'VPEXTRB_mxroub_1', '          '];
T['VPEXTRB REG:w:u8=$GPR32m  VREG:r:u8=$XMMr  IMM:u8=$index', 'MOD=REG              ', 'VPEXTRB_mxroub_2', '          '];
T['VPEXTRB MEM:w:u8=$mem8    VREG:r:u8=$XMMr  IMM:u8=$index', 'MOD=MEM              ', 'VPEXTRB_mxroub_2', 'tuple=T1S8'];
T['VPEXTRD REG:w:s32=$GPR32m VREG:r:u32=$XMMr IMM:u8=$index', 'MOD=REG MODE=64 W=0  ', 'VPEXTRx_mxroub_1', '          '];
T['VPEXTRD MEM:w:s32=$mem32  VREG:r:u32=$XMMr IMM:u8=$index', 'MOD=MEM MODE=64 W=0  ', 'VPEXTRx_mxroub_1', '          '];
T['VPEXTRD REG:w:s32=$GPR32m VREG:r:u32=$XMMr IMM:u8=$index', 'MOD=REG MODE=NO64    ', 'VPEXTRx_mxroub_1', '          '];
T['VPEXTRD MEM:w:s32=$mem32  VREG:r:u32=$XMMr IMM:u8=$index', 'MOD=MEM MODE=NO64    ', 'VPEXTRx_mxroub_1', '          '];
T['VPEXTRQ REG:w:s64=$GPR64m VREG:r:u64=$XMMr IMM:u8=$index', 'MOD=REG MODE=64 W=1  ', 'VPEXTRx_mxroub_1', '          '];
T['VPEXTRQ MEM:w:s64=$mem64  VREG:r:u64=$XMMr IMM:u8=$index', 'MOD=MEM MODE=64 W=1  ', 'VPEXTRx_mxroub_1', '          '];
T['VPEXTRD REG:w:u32=$GPR32m VREG:r:u32=$XMMr IMM:u8=$index', 'MOD=REG W=0          ', 'VPEXTRx_mxroub_2', '          '];
T['VPEXTRD MEM:w:u32=$mem32  VREG:r:u32=$XMMr IMM:u8=$index', 'MOD=MEM W=0          ', 'VPEXTRx_mxroub_2', 'tuple=T1S '];
T['VPEXTRD REG:w:u32=$GPR32m VREG:r:u32=$XMMr IMM:u8=$index', 'MOD=REG MODE=NO64 W=1', 'VPEXTRx_mxroub_2', '          '];
T['VPEXTRD MEM:w:u32=$mem32  VREG:r:u32=$XMMr IMM:u8=$index', 'MOD=MEM MODE=NO64 W=1', 'VPEXTRx_mxroub_2', 'tuple=T1S '];
T['VPEXTRQ REG:w:u64=$GPR64m VREG:r:u64=$XMMr IMM:u8=$index', 'MOD=REG MODE=64 W=1  ', 'VPEXTRx_mxroub_2', '          '];
T['VPEXTRQ MEM:w:u64=$mem64  VREG:r:u64=$XMMr IMM:u8=$index', 'MOD=MEM MODE=64 W=1  ', 'VPEXTRx_mxroub_2', 'tuple=T1S '];

# PHADDSW-Packed Horizontal Add and Saturate.
T['VPHADDSW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm ', 'MOD=REG', 'VPHADDSW_ronomo', ''];
T['VPHADDSW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128', 'MOD=MEM', 'VPHADDSW_ronomo', ''];
T['VPHADDSW VREG:w:s16=$YMMr VREG:r:s16=$YMMn VREG:r:s16=$YMMm ', 'MOD=REG', 'VPHADDSW_rynymy', ''];
T['VPHADDSW VREG:w:s16=$YMMr VREG:r:s16=$YMMn MEM:r:s16=$mem256', 'MOD=MEM', 'VPHADDSW_rynymy', ''];

# PHADDW/PHADDD-Packed Horizontal Add.
T['VPHADDD VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm ', 'MOD=REG', 'VPHADDD_ronomo', ''];
T['VPHADDD VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128', 'MOD=MEM', 'VPHADDD_ronomo', ''];
T['VPHADDD VREG:w:s32=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm ', 'MOD=REG', 'VPHADDD_rynymy', ''];
T['VPHADDD VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256', 'MOD=MEM', 'VPHADDD_rynymy', ''];
T['VPHADDW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm ', 'MOD=REG', 'VPHADDW_ronomo', ''];
T['VPHADDW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128', 'MOD=MEM', 'VPHADDW_ronomo', ''];
T['VPHADDW VREG:w:s16=$YMMr VREG:r:s16=$YMMn VREG:r:s16=$YMMm ', 'MOD=REG', 'VPHADDW_rynymy', ''];
T['VPHADDW VREG:w:s16=$YMMr VREG:r:s16=$YMMn MEM:r:s16=$mem256', 'MOD=MEM', 'VPHADDW_rynymy', ''];

# PHMINPOSUW-Packed Horizontal Word Minimum.
T['VPHMINPOSUW VREG:w:u16=$XMMr VREG:r:u16=$XMMm ', 'MOD=REG', 'VPHMINPOSUW', ''];
T['VPHMINPOSUW VREG:w:u16=$XMMr MEM:r:u16=$mem128', 'MOD=MEM', 'VPHMINPOSUW', ''];

# PHSUBSW-Packed Horizontal Subtract and Saturate.
T['VPHSUBSW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm ', 'MOD=REG', 'VPHSUBSW_ronomo', ''];
T['VPHSUBSW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128', 'MOD=MEM', 'VPHSUBSW_ronomo', ''];
T['VPHSUBSW VREG:w:s16=$YMMr VREG:r:s16=$YMMn VREG:r:s16=$YMMm ', 'MOD=REG', 'VPHSUBSW_rynymy', ''];
T['VPHSUBSW VREG:w:s16=$YMMr VREG:r:s16=$YMMn MEM:r:s16=$mem256', 'MOD=MEM', 'VPHSUBSW_rynymy', ''];

# PHSUBW/PHSUBD-Packed Horizontal Subtract.
T['VPHSUBD VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm ', 'MOD=REG', 'VPHSUBD_ronomo', ''];
T['VPHSUBD VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128', 'MOD=MEM', 'VPHSUBD_ronomo', ''];
T['VPHSUBD VREG:w:s32=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm ', 'MOD=REG', 'VPHSUBD_rynymy', ''];
T['VPHSUBD VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256', 'MOD=MEM', 'VPHSUBD_rynymy', ''];
T['VPHSUBW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm ', 'MOD=REG', 'VPHSUBW_ronomo', ''];
T['VPHSUBW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128', 'MOD=MEM', 'VPHSUBW_ronomo', ''];
T['VPHSUBW VREG:w:s16=$YMMr VREG:r:s16=$YMMn VREG:r:s16=$YMMm ', 'MOD=REG', 'VPHSUBW_rynymy', ''];
T['VPHSUBW VREG:w:s16=$YMMr VREG:r:s16=$YMMn MEM:r:s16=$mem256', 'MOD=MEM', 'VPHSUBW_rynymy', ''];

# PINSRW-Insert Word.
T['VPINSRW VREG:w:u16=$XMMr VREG:r:u16=$XMMn REG:r:u16=$GPR32m IMM:u8=$index', 'MOD=REG', 'VPINSRW_ronomxub_1', '           '];
T['VPINSRW VREG:w:u16=$XMMr VREG:r:u16=$XMMn MEM:r:u16=$mem16  IMM:u8=$index', 'MOD=MEM', 'VPINSRW_ronomxub_1', '           '];
T['VPINSRW VREG:w:u16=$XMMr VREG:r:u16=$XMMn REG:r:u16=$GPR32m IMM:u8=$index', 'MOD=REG', 'VPINSRW_ronomxub_2', '           '];
T['VPINSRW VREG:w:u16=$XMMr VREG:r:u16=$XMMn MEM:r:u16=$mem16  IMM:u8=$index', 'MOD=MEM', 'VPINSRW_ronomxub_2', 'tuple=T1S16'];

# PINSRB/PINSRD/PINSRQ-Insert Byte/Dword/Qword.
T['VPINSRB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  REG:r:u8=$GPR32m  IMM:u8=$index', 'MOD=REG              ', 'VPINSRB_ronomxub_1', '          '];
T['VPINSRB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  MEM:r:u8=$mem8    IMM:u8=$index', 'MOD=MEM              ', 'VPINSRB_ronomxub_1', '          '];
T['VPINSRB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  REG:r:u8=$GPR32m  IMM:u8=$index', 'MOD=REG              ', 'VPINSRB_ronomxub_2', '          '];
T['VPINSRB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  MEM:r:u8=$mem8    IMM:u8=$index', 'MOD=MEM              ', 'VPINSRB_ronomxub_2', 'tuple=T1S8'];
T['VPINSRD VREG:w:u32=$XMMr VREG:r:u32=$XMMn REG:r:u32=$GPR32m IMM:u8=$index', 'MOD=REG MODE=64 W=0  ', 'VPINSRx_ronomxub_1', '          '];
T['VPINSRD VREG:w:u32=$XMMr VREG:r:u32=$XMMn MEM:r:u32=$mem32  IMM:u8=$index', 'MOD=MEM MODE=64 W=0  ', 'VPINSRx_ronomxub_1', '          '];
T['VPINSRD VREG:w:u32=$XMMr VREG:r:u32=$XMMn REG:r:u32=$GPR32m IMM:u8=$index', 'MOD=REG MODE=NO64    ', 'VPINSRx_ronomxub_1', '          '];
T['VPINSRD VREG:w:u32=$XMMr VREG:r:u32=$XMMn MEM:r:u32=$mem32  IMM:u8=$index', 'MOD=MEM MODE=NO64    ', 'VPINSRx_ronomxub_1', '          '];
T['VPINSRQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn REG:r:u64=$GPR64m IMM:u8=$index', 'MOD=REG MODE=64 W=1  ', 'VPINSRx_ronomxub_1', '          '];
T['VPINSRQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn MEM:r:u64=$mem64  IMM:u8=$index', 'MOD=MEM MODE=64 W=1  ', 'VPINSRx_ronomxub_1', '          '];
T['VPINSRD VREG:w:u32=$XMMr VREG:r:u32=$XMMn REG:r:u32=$GPR32m IMM:u8=$index', 'MOD=REG W=0          ', 'VPINSRx_ronomxub_2', '          '];
T['VPINSRD VREG:w:u32=$XMMr VREG:r:u32=$XMMn MEM:r:u32=$mem32  IMM:u8=$index', 'MOD=MEM W=0          ', 'VPINSRx_ronomxub_2', 'tuple=T1S '];
T['VPINSRD VREG:w:u32=$XMMr VREG:r:u32=$XMMn REG:r:u32=$GPR32m IMM:u8=$index', 'MOD=REG MODE=NO64 W=1', 'VPINSRx_ronomxub_2', '          '];
T['VPINSRD VREG:w:u32=$XMMr VREG:r:u32=$XMMn MEM:r:u32=$mem32  IMM:u8=$index', 'MOD=MEM MODE=NO64 W=1', 'VPINSRx_ronomxub_2', 'tuple=T1S '];
T['VPINSRQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn REG:r:u64=$GPR64m IMM:u8=$index', 'MOD=REG MODE=64 W=1  ', 'VPINSRx_ronomxub_2', '          '];
T['VPINSRQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn MEM:r:u64=$mem64  IMM:u8=$index', 'MOD=MEM MODE=64 W=1  ', 'VPINSRx_ronomxub_2', 'tuple=T1S '];

# PMADDUBSW-Multiply and Add Packed Signed and Unsigned Bytes.
T['VPMADDUBSW VREG:w:s16=$XMMr VREG:r:u8=$XMMn VREG:r:s8=$XMMm                   ', 'MOD=REG', 'VPMADDUBSW_ronomo_1', '         '];
T['VPMADDUBSW VREG:w:s16=$XMMr VREG:r:u8=$XMMn MEM:r:s8=$mem128                  ', 'MOD=MEM', 'VPMADDUBSW_ronomo_1', '         '];
T['VPMADDUBSW VREG:w:s16=$XMMr PREG:r:Z=$Kk    VREG:r:s16=$XMMn VREG:r:s16=$XMMm ', 'MOD=REG', 'VPMADDUBSW_ronomo_2', '         '];
T['VPMADDUBSW VREG:w:s16=$XMMr PREG:r:Z=$Kk    VREG:r:s16=$XMMn MEM:r:s16=$mem128', 'MOD=MEM', 'VPMADDUBSW_ronomo_2', 'tuple=FVM'];
T['VPMADDUBSW VREG:w:s16=$YMMr VREG:r:u8=$YMMn VREG:r:s8=$YMMm                   ', 'MOD=REG', 'VPMADDUBSW_rynymy_1', '         '];
T['VPMADDUBSW VREG:w:s16=$YMMr VREG:r:u8=$YMMn MEM:r:s8=$mem256                  ', 'MOD=MEM', 'VPMADDUBSW_rynymy_1', '         '];
T['VPMADDUBSW VREG:w:s16=$YMMr PREG:r:Z=$Kk    VREG:r:s16=$YMMn VREG:r:s16=$YMMm ', 'MOD=REG', 'VPMADDUBSW_rynymy_2', '         '];
T['VPMADDUBSW VREG:w:s16=$YMMr PREG:r:Z=$Kk    VREG:r:s16=$YMMn MEM:r:s16=$mem256', 'MOD=MEM', 'VPMADDUBSW_rynymy_2', 'tuple=FVM'];
T['VPMADDUBSW VREG:w:s16=$ZMMr PREG:r:Z=$Kk    VREG:r:s16=$ZMMn VREG:r:s16=$ZMMm ', 'MOD=REG', 'VPMADDUBSW_rznzmz  ', '         '];
T['VPMADDUBSW VREG:w:s16=$ZMMr PREG:r:Z=$Kk    VREG:r:s16=$ZMMn MEM:r:s16=$mem512', 'MOD=MEM', 'VPMADDUBSW_rznzmz  ', 'tuple=FVM'];

# PMADDWD-Multiply and Add Packed Integers.
T['VPMADDWD VREG:w:s32=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm                   ', 'MOD=REG', 'VPMADDWD_ronomo_1', '         '];
T['VPMADDWD VREG:w:s32=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128                  ', 'MOD=MEM', 'VPMADDWD_ronomo_1', '         '];
T['VPMADDWD VREG:w:s32=$XMMr PREG:r:Z=$Kk     VREG:r:s16=$XMMn  VREG:r:s16=$XMMm ', 'MOD=REG', 'VPMADDWD_ronomo_2', '         '];
T['VPMADDWD VREG:w:s32=$XMMr PREG:r:Z=$Kk     VREG:r:s16=$XMMn  MEM:r:s16=$mem128', 'MOD=MEM', 'VPMADDWD_ronomo_2', 'tuple=FVM'];
T['VPMADDWD VREG:w:s32=$YMMr VREG:r:s16=$YMMn VREG:r:s16=$YMMm                   ', 'MOD=REG', 'VPMADDWD_rynymy_1', '         '];
T['VPMADDWD VREG:w:s32=$YMMr VREG:r:s16=$YMMn MEM:r:s16=$mem256                  ', 'MOD=MEM', 'VPMADDWD_rynymy_1', '         '];
T['VPMADDWD VREG:w:s32=$YMMr PREG:r:Z=$Kk     VREG:r:s16=$YMMn  VREG:r:s16=$YMMm ', 'MOD=REG', 'VPMADDWD_rynymy_2', '         '];
T['VPMADDWD VREG:w:s32=$YMMr PREG:r:Z=$Kk     VREG:r:s16=$YMMn  MEM:r:s16=$mem256', 'MOD=MEM', 'VPMADDWD_rynymy_2', 'tuple=FVM'];
T['VPMADDWD VREG:w:s32=$ZMMr PREG:r:Z=$Kk     VREG:r:s16=$ZMMn  VREG:r:s16=$ZMMm ', 'MOD=REG', 'VPMADDWD_rznzmz  ', '         '];
T['VPMADDWD VREG:w:s32=$ZMMr PREG:r:Z=$Kk     VREG:r:s16=$ZMMn  MEM:r:s16=$mem512', 'MOD=MEM', 'VPMADDWD_rznzmz  ', 'tuple=FVM'];

# PMAXSB/PMAXSW/PMAXSD/PMAXSQ-Maximum of Packed Signed Integers.
T['VPMAXSB VREG:w:s8=$XMMr  VREG:r:s8=$XMMn  VREG:r:s8=$XMMm                               ', 'MOD=REG         ', 'VPMAXSB_ronomo_1', '         '];
T['VPMAXSB VREG:w:s8=$XMMr  VREG:r:s8=$XMMn  MEM:r:s8=$mem128                              ', 'MOD=MEM         ', 'VPMAXSB_ronomo_1', '         '];
T['VPMAXSB VREG:w:s8=$XMMr  PREG:r:Z=$Kk     VREG:r:s8=$XMMn   VREG:r:s8=$XMMm             ', 'MOD=REG         ', 'VPMAXSB_ronomo_2', '         '];
T['VPMAXSB VREG:w:s8=$XMMr  PREG:r:Z=$Kk     VREG:r:s8=$XMMn   MEM:r:s8=$mem128            ', 'MOD=MEM         ', 'VPMAXSB_ronomo_2', 'tuple=FVM'];
T['VPMAXSB VREG:w:s8=$YMMr  VREG:r:s8=$YMMn  VREG:r:s8=$YMMm                               ', 'MOD=REG         ', 'VPMAXSB_rynymy_1', '         '];
T['VPMAXSB VREG:w:s8=$YMMr  VREG:r:s8=$YMMn  MEM:r:s8=$mem256                              ', 'MOD=MEM         ', 'VPMAXSB_rynymy_1', '         '];
T['VPMAXSB VREG:w:s8=$YMMr  PREG:r:Z=$Kk     VREG:r:s8=$YMMn   VREG:r:s8=$YMMm             ', 'MOD=REG         ', 'VPMAXSB_rynymy_2', '         '];
T['VPMAXSB VREG:w:s8=$YMMr  PREG:r:Z=$Kk     VREG:r:s8=$YMMn   MEM:r:s8=$mem256            ', 'MOD=MEM         ', 'VPMAXSB_rynymy_2', 'tuple=FVM'];
T['VPMAXSB VREG:w:s8=$ZMMr  PREG:r:Z=$Kk     VREG:r:s8=$ZMMn   VREG:r:s8=$ZMMm             ', 'MOD=REG         ', 'VPMAXSB_rznzmz  ', '         '];
T['VPMAXSB VREG:w:s8=$ZMMr  PREG:r:Z=$Kk     VREG:r:s8=$ZMMn   MEM:r:s8=$mem512            ', 'MOD=MEM         ', 'VPMAXSB_rznzmz  ', 'tuple=FVM'];
T['VPMAXSD VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm                              ', 'MOD=REG         ', 'VPMAXSD_ronomo  ', '         '];
T['VPMAXSD VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128                             ', 'MOD=MEM         ', 'VPMAXSD_ronomo  ', '         '];
T['VPMAXSD VREG:w:s32=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm                              ', 'MOD=REG         ', 'VPMAXSD_rynymy  ', '         '];
T['VPMAXSD VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256                             ', 'MOD=MEM         ', 'VPMAXSD_rynymy  ', '         '];
T['VPMAXSW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm                              ', 'MOD=REG         ', 'VPMAXSW_ronomo_1', '         '];
T['VPMAXSW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128                             ', 'MOD=MEM         ', 'VPMAXSW_ronomo_1', '         '];
T['VPMAXSW VREG:w:s16=$XMMr PREG:r:Z=$Kk     VREG:r:s16=$XMMn  VREG:r:s16=$XMMm            ', 'MOD=REG         ', 'VPMAXSW_ronomo_2', '         '];
T['VPMAXSW VREG:w:s16=$XMMr PREG:r:Z=$Kk     VREG:r:s16=$XMMn  MEM:r:s16=$mem128           ', 'MOD=MEM         ', 'VPMAXSW_ronomo_2', 'tuple=FVM'];
T['VPMAXSW VREG:w:s16=$YMMr VREG:r:s16=$YMMn VREG:r:s16=$YMMm                              ', 'MOD=REG         ', 'VPMAXSW_rynymy_1', '         '];
T['VPMAXSW VREG:w:s16=$YMMr VREG:r:s16=$YMMn MEM:r:s16=$mem256                             ', 'MOD=MEM         ', 'VPMAXSW_rynymy_1', '         '];
T['VPMAXSW VREG:w:s16=$YMMr PREG:r:Z=$Kk     VREG:r:s16=$YMMn  VREG:r:s16=$YMMm            ', 'MOD=REG         ', 'VPMAXSW_rynymy_2', '         '];
T['VPMAXSW VREG:w:s16=$YMMr PREG:r:Z=$Kk     VREG:r:s16=$YMMn  MEM:r:s16=$mem256           ', 'MOD=MEM         ', 'VPMAXSW_rynymy_2', 'tuple=FVM'];
T['VPMAXSW VREG:w:s16=$ZMMr PREG:r:Z=$Kk     VREG:r:s16=$ZMMn  VREG:r:s16=$ZMMm            ', 'MOD=REG         ', 'VPMAXSW_rznzmz  ', '         '];
T['VPMAXSW VREG:w:s16=$ZMMr PREG:r:Z=$Kk     VREG:r:s16=$ZMMn  MEM:r:s16=$mem512           ', 'MOD=MEM         ', 'VPMAXSW_rznzmz  ', 'tuple=FVM'];
T['VPMAXSD VREG:w:s32=$XMMr PREG:r:Z=$Kk     VREG:r:s32=$XMMn  VREG:r:s32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPMAXSx_ronomo  ', '         '];
T['VPMAXSD VREG:w:s32=$XMMr PREG:r:Z=$Kk     VREG:r:s32=$XMMn  MEM:r:s32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPMAXSx_ronomo  ', 'tuple=FV '];
T['VPMAXSQ VREG:w:s64=$XMMr PREG:r:Z=$Kk     VREG:r:s64=$XMMn  VREG:r:s64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPMAXSx_ronomo  ', '         '];
T['VPMAXSQ VREG:w:s64=$XMMr PREG:r:Z=$Kk     VREG:r:s64=$XMMn  MEM:r:s64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPMAXSx_ronomo  ', 'tuple=FV '];
T['VPMAXSD VREG:w:s32=$YMMr PREG:r:Z=$Kk     VREG:r:s32=$YMMn  VREG:r:s32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPMAXSx_rynymy  ', '         '];
T['VPMAXSD VREG:w:s32=$YMMr PREG:r:Z=$Kk     VREG:r:s32=$YMMn  MEM:r:s32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPMAXSx_rynymy  ', 'tuple=FV '];
T['VPMAXSQ VREG:w:s64=$YMMr PREG:r:Z=$Kk     VREG:r:s64=$YMMn  VREG:r:s64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPMAXSx_rynymy  ', '         '];
T['VPMAXSQ VREG:w:s64=$YMMr PREG:r:Z=$Kk     VREG:r:s64=$YMMn  MEM:r:s64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPMAXSx_rynymy  ', 'tuple=FV '];
T['VPMAXSD VREG:w:s32=$ZMMr PREG:r:Z=$Kk     VREG:r:s32=$ZMMn  VREG:r:s32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPMAXSx_rznzmz  ', '         '];
T['VPMAXSD VREG:w:s32=$ZMMr PREG:r:Z=$Kk     VREG:r:s32=$ZMMn  MEM:r:s32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPMAXSx_rznzmz  ', 'tuple=FV '];
T['VPMAXSQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk     VREG:r:s64=$ZMMn  VREG:r:s64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPMAXSx_rznzmz  ', '         '];
T['VPMAXSQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk     VREG:r:s64=$ZMMn  MEM:r:s64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPMAXSx_rznzmz  ', 'tuple=FV '];

# PMAXUB/PMAXUW-Maximum of Packed Unsigned Integers.
T['VPMAXUB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  VREG:r:u8=$XMMm                    ', 'MOD=REG', 'VPMAXUB_ronomo_1', '         '];
T['VPMAXUB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  MEM:r:u8=$mem128                   ', 'MOD=MEM', 'VPMAXUB_ronomo_1', '         '];
T['VPMAXUB VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   VREG:r:u8=$XMMm  ', 'MOD=REG', 'VPMAXUB_ronomo_2', '         '];
T['VPMAXUB VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   MEM:r:u8=$mem128 ', 'MOD=MEM', 'VPMAXUB_ronomo_2', 'tuple=FVM'];
T['VPMAXUB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  VREG:r:u8=$YMMm                    ', 'MOD=REG', 'VPMAXUB_rynymy_1', '         '];
T['VPMAXUB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  MEM:r:u8=$mem256                   ', 'MOD=MEM', 'VPMAXUB_rynymy_1', '         '];
T['VPMAXUB VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   VREG:r:u8=$YMMm  ', 'MOD=REG', 'VPMAXUB_rynymy_2', '         '];
T['VPMAXUB VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   MEM:r:u8=$mem256 ', 'MOD=MEM', 'VPMAXUB_rynymy_2', 'tuple=FVM'];
T['VPMAXUB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   VREG:r:u8=$ZMMm  ', 'MOD=REG', 'VPMAXUB_rznzmz  ', '         '];
T['VPMAXUB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   MEM:r:u8=$mem512 ', 'MOD=MEM', 'VPMAXUB_rznzmz  ', 'tuple=FVM'];
T['VPMAXUW VREG:w:u16=$XMMr VREG:r:u16=$XMMn VREG:r:u16=$XMMm                   ', 'MOD=REG', 'VPMAXUW_ronomo_1', '         '];
T['VPMAXUW VREG:w:u16=$XMMr VREG:r:u16=$XMMn MEM:r:u16=$mem128                  ', 'MOD=MEM', 'VPMAXUW_ronomo_1', '         '];
T['VPMAXUW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm ', 'MOD=REG', 'VPMAXUW_ronomo_2', '         '];
T['VPMAXUW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128', 'MOD=MEM', 'VPMAXUW_ronomo_2', 'tuple=FVM'];
T['VPMAXUW VREG:w:u16=$YMMr VREG:r:u16=$YMMn VREG:r:u16=$YMMm                   ', 'MOD=REG', 'VPMAXUW_rynymy_1', '         '];
T['VPMAXUW VREG:w:u16=$YMMr VREG:r:u16=$YMMn MEM:r:u16=$mem256                  ', 'MOD=MEM', 'VPMAXUW_rynymy_1', '         '];
T['VPMAXUW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm ', 'MOD=REG', 'VPMAXUW_rynymy_2', '         '];
T['VPMAXUW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256', 'MOD=MEM', 'VPMAXUW_rynymy_2', 'tuple=FVM'];
T['VPMAXUW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm ', 'MOD=REG', 'VPMAXUW_rznzmz  ', '         '];
T['VPMAXUW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512', 'MOD=MEM', 'VPMAXUW_rznzmz  ', 'tuple=FVM'];

# PMAXUD/PMAXUQ-Maximum of Packed Unsigned Integers.
T['VPMAXUD VREG:w:u32=$XMMr VREG:r:u32=$XMMn VREG:r:u32=$XMMm                              ', 'MOD=REG         ', 'VPMAXUD_ronomo', '        '];
T['VPMAXUD VREG:w:u32=$XMMr VREG:r:u32=$XMMn MEM:r:u32=$mem128                             ', 'MOD=MEM         ', 'VPMAXUD_ronomo', '        '];
T['VPMAXUD VREG:w:u32=$YMMr VREG:r:u32=$YMMn VREG:r:u32=$YMMm                              ', 'MOD=REG         ', 'VPMAXUD_rynymy', '        '];
T['VPMAXUD VREG:w:u32=$YMMr VREG:r:u32=$YMMn MEM:r:u32=$mem256                             ', 'MOD=MEM         ', 'VPMAXUD_rynymy', '        '];
T['VPMAXUD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPMAXUx_ronomo', '        '];
T['VPMAXUD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPMAXUx_ronomo', 'tuple=FV'];
T['VPMAXUQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPMAXUx_ronomo', '        '];
T['VPMAXUQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPMAXUx_ronomo', 'tuple=FV'];
T['VPMAXUD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPMAXUx_rynymy', '        '];
T['VPMAXUD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPMAXUx_rynymy', 'tuple=FV'];
T['VPMAXUQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPMAXUx_rynymy', '        '];
T['VPMAXUQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPMAXUx_rynymy', 'tuple=FV'];
T['VPMAXUD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPMAXUx_rznzmz', '        '];
T['VPMAXUD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPMAXUx_rznzmz', 'tuple=FV'];
T['VPMAXUQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPMAXUx_rznzmz', '        '];
T['VPMAXUQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPMAXUx_rznzmz', 'tuple=FV'];

# PMINSB/PMINSW-Minimum of Packed Signed Integers.
T['VPMINSB VREG:w:s8=$XMMr  VREG:r:s8=$XMMn  VREG:r:s8=$XMMm                    ', 'MOD=REG', 'VPMINSB_ronomo_1', '         '];
T['VPMINSB VREG:w:s8=$XMMr  VREG:r:s8=$XMMn  MEM:r:s8=$mem128                   ', 'MOD=MEM', 'VPMINSB_ronomo_1', '         '];
T['VPMINSB VREG:w:s8=$XMMr  PREG:r:Z=$Kk     VREG:r:s8=$XMMn   VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPMINSB_ronomo_2', '         '];
T['VPMINSB VREG:w:s8=$XMMr  PREG:r:Z=$Kk     VREG:r:s8=$XMMn   MEM:r:s8=$mem128 ', 'MOD=MEM', 'VPMINSB_ronomo_2', 'tuple=FVM'];
T['VPMINSB VREG:w:s8=$YMMr  VREG:r:s8=$YMMn  VREG:r:s8=$YMMm                    ', 'MOD=REG', 'VPMINSB_rynymy_1', '         '];
T['VPMINSB VREG:w:s8=$YMMr  VREG:r:s8=$YMMn  MEM:r:s8=$mem256                   ', 'MOD=MEM', 'VPMINSB_rynymy_1', '         '];
T['VPMINSB VREG:w:s8=$YMMr  PREG:r:Z=$Kk     VREG:r:s8=$YMMn   VREG:r:s8=$YMMm  ', 'MOD=REG', 'VPMINSB_rynymy_2', '         '];
T['VPMINSB VREG:w:s8=$YMMr  PREG:r:Z=$Kk     VREG:r:s8=$YMMn   MEM:r:s8=$mem256 ', 'MOD=MEM', 'VPMINSB_rynymy_2', 'tuple=FVM'];
T['VPMINSB VREG:w:s8=$ZMMr  PREG:r:Z=$Kk     VREG:r:s8=$ZMMn   VREG:r:s8=$ZMMm  ', 'MOD=REG', 'VPMINSB_rznzmz  ', '         '];
T['VPMINSB VREG:w:s8=$ZMMr  PREG:r:Z=$Kk     VREG:r:s8=$ZMMn   MEM:r:s8=$mem512 ', 'MOD=MEM', 'VPMINSB_rznzmz  ', 'tuple=FVM'];
T['VPMINSW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm                   ', 'MOD=REG', 'VPMINSW_ronomo_1', '         '];
T['VPMINSW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128                  ', 'MOD=MEM', 'VPMINSW_ronomo_1', '         '];
T['VPMINSW VREG:w:s16=$XMMr PREG:r:Z=$Kk     VREG:r:s16=$XMMn  VREG:r:s16=$XMMm ', 'MOD=REG', 'VPMINSW_ronomo_2', '         '];
T['VPMINSW VREG:w:s16=$XMMr PREG:r:Z=$Kk     VREG:r:s16=$XMMn  MEM:r:s16=$mem128', 'MOD=MEM', 'VPMINSW_ronomo_2', 'tuple=FVM'];
T['VPMINSW VREG:w:s16=$YMMr VREG:r:s16=$YMMn VREG:r:s16=$YMMm                   ', 'MOD=REG', 'VPMINSW_rynymy_1', '         '];
T['VPMINSW VREG:w:s16=$YMMr VREG:r:s16=$YMMn MEM:r:s16=$mem256                  ', 'MOD=MEM', 'VPMINSW_rynymy_1', '         '];
T['VPMINSW VREG:w:s16=$YMMr PREG:r:Z=$Kk     VREG:r:s16=$YMMn  VREG:r:s16=$YMMm ', 'MOD=REG', 'VPMINSW_rynymy_2', '         '];
T['VPMINSW VREG:w:s16=$YMMr PREG:r:Z=$Kk     VREG:r:s16=$YMMn  MEM:r:s16=$mem256', 'MOD=MEM', 'VPMINSW_rynymy_2', 'tuple=FVM'];
T['VPMINSW VREG:w:s16=$ZMMr PREG:r:Z=$Kk     VREG:r:s16=$ZMMn  VREG:r:s16=$ZMMm ', 'MOD=REG', 'VPMINSW_rznzmz  ', '         '];
T['VPMINSW VREG:w:s16=$ZMMr PREG:r:Z=$Kk     VREG:r:s16=$ZMMn  MEM:r:s16=$mem512', 'MOD=MEM', 'VPMINSW_rznzmz  ', 'tuple=FVM'];

# PMINSD/PMINSQ-Minimum of Packed Signed Integers.
T['VPMINSD VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm                              ', 'MOD=REG         ', 'VPMINSD_ronomo', '        '];
T['VPMINSD VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128                             ', 'MOD=MEM         ', 'VPMINSD_ronomo', '        '];
T['VPMINSD VREG:w:s32=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm                              ', 'MOD=REG         ', 'VPMINSD_rynymy', '        '];
T['VPMINSD VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256                             ', 'MOD=MEM         ', 'VPMINSD_rynymy', '        '];
T['VPMINSD VREG:w:s32=$XMMr PREG:r:Z=$Kk     VREG:r:s32=$XMMn  VREG:r:s32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPMINSx_ronomo', '        '];
T['VPMINSD VREG:w:s32=$XMMr PREG:r:Z=$Kk     VREG:r:s32=$XMMn  MEM:r:s32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPMINSx_ronomo', 'tuple=FV'];
T['VPMINSQ VREG:w:s64=$XMMr PREG:r:Z=$Kk     VREG:r:s64=$XMMn  VREG:r:s64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPMINSx_ronomo', '        '];
T['VPMINSQ VREG:w:s64=$XMMr PREG:r:Z=$Kk     VREG:r:s64=$XMMn  MEM:r:s64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPMINSx_ronomo', 'tuple=FV'];
T['VPMINSD VREG:w:s32=$YMMr PREG:r:Z=$Kk     VREG:r:s32=$YMMn  VREG:r:s32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPMINSx_rynymy', '        '];
T['VPMINSD VREG:w:s32=$YMMr PREG:r:Z=$Kk     VREG:r:s32=$YMMn  MEM:r:s32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPMINSx_rynymy', 'tuple=FV'];
T['VPMINSQ VREG:w:s64=$YMMr PREG:r:Z=$Kk     VREG:r:s64=$YMMn  VREG:r:s64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPMINSx_rynymy', '        '];
T['VPMINSQ VREG:w:s64=$YMMr PREG:r:Z=$Kk     VREG:r:s64=$YMMn  MEM:r:s64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPMINSx_rynymy', 'tuple=FV'];
T['VPMINSD VREG:w:s32=$ZMMr PREG:r:Z=$Kk     VREG:r:s32=$ZMMn  VREG:r:s32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPMINSx_rznzmz', '        '];
T['VPMINSD VREG:w:s32=$ZMMr PREG:r:Z=$Kk     VREG:r:s32=$ZMMn  MEM:r:s32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPMINSx_rznzmz', 'tuple=FV'];
T['VPMINSQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk     VREG:r:s64=$ZMMn  VREG:r:s64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPMINSx_rznzmz', '        '];
T['VPMINSQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk     VREG:r:s64=$ZMMn  MEM:r:s64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPMINSx_rznzmz', 'tuple=FV'];

# PMINUB/PMINUW-Minimum of Packed Unsigned Integers.
T['VPMINUB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  VREG:r:u8=$XMMm                    ', 'MOD=REG', 'VPMINUB_ronomo_1', '         '];
T['VPMINUB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  MEM:r:u8=$mem128                   ', 'MOD=MEM', 'VPMINUB_ronomo_1', '         '];
T['VPMINUB VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   VREG:r:u8=$XMMm  ', 'MOD=REG', 'VPMINUB_ronomo_2', '         '];
T['VPMINUB VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   MEM:r:u8=$mem128 ', 'MOD=MEM', 'VPMINUB_ronomo_2', 'tuple=FVM'];
T['VPMINUB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  VREG:r:u8=$YMMm                    ', 'MOD=REG', 'VPMINUB_rynymy_1', '         '];
T['VPMINUB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  MEM:r:u8=$mem256                   ', 'MOD=MEM', 'VPMINUB_rynymy_1', '         '];
T['VPMINUB VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   VREG:r:u8=$YMMm  ', 'MOD=REG', 'VPMINUB_rynymy_2', '         '];
T['VPMINUB VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   MEM:r:u8=$mem256 ', 'MOD=MEM', 'VPMINUB_rynymy_2', 'tuple=FVM'];
T['VPMINUB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   VREG:r:u8=$ZMMm  ', 'MOD=REG', 'VPMINUB_rznzmz  ', '         '];
T['VPMINUB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   MEM:r:u8=$mem512 ', 'MOD=MEM', 'VPMINUB_rznzmz  ', 'tuple=FVM'];
T['VPMINUW VREG:w:u16=$XMMr VREG:r:u16=$XMMn VREG:r:u16=$XMMm                   ', 'MOD=REG', 'VPMINUW_ronomo_1', '         '];
T['VPMINUW VREG:w:u16=$XMMr VREG:r:u16=$XMMn MEM:r:u16=$mem128                  ', 'MOD=MEM', 'VPMINUW_ronomo_1', '         '];
T['VPMINUW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm ', 'MOD=REG', 'VPMINUW_ronomo_2', '         '];
T['VPMINUW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128', 'MOD=MEM', 'VPMINUW_ronomo_2', 'tuple=FVM'];
T['VPMINUW VREG:w:u16=$YMMr VREG:r:u16=$YMMn VREG:r:u16=$YMMm                   ', 'MOD=REG', 'VPMINUW_rynymy_1', '         '];
T['VPMINUW VREG:w:u16=$YMMr VREG:r:u16=$YMMn MEM:r:u16=$mem256                  ', 'MOD=MEM', 'VPMINUW_rynymy_1', '         '];
T['VPMINUW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm ', 'MOD=REG', 'VPMINUW_rynymy_2', '         '];
T['VPMINUW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256', 'MOD=MEM', 'VPMINUW_rynymy_2', 'tuple=FVM'];
T['VPMINUW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm ', 'MOD=REG', 'VPMINUW_rznzmz  ', '         '];
T['VPMINUW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512', 'MOD=MEM', 'VPMINUW_rznzmz  ', 'tuple=FVM'];

# PMINUD/PMINUQ-Minimum of Packed Unsigned Integers.
T['VPMINUD VREG:w:u32=$XMMr VREG:r:u32=$XMMn VREG:r:u32=$XMMm                              ', 'MOD=REG         ', 'VPMINUD_ronomo', '        '];
T['VPMINUD VREG:w:u32=$XMMr VREG:r:u32=$XMMn MEM:r:u32=$mem128                             ', 'MOD=MEM         ', 'VPMINUD_ronomo', '        '];
T['VPMINUD VREG:w:u32=$YMMr VREG:r:u32=$YMMn VREG:r:u32=$YMMm                              ', 'MOD=REG         ', 'VPMINUD_rynymy', '        '];
T['VPMINUD VREG:w:u32=$YMMr VREG:r:u32=$YMMn MEM:r:u32=$mem256                             ', 'MOD=MEM         ', 'VPMINUD_rynymy', '        '];
T['VPMINUD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPMINUx_ronomo', '        '];
T['VPMINUD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPMINUx_ronomo', 'tuple=FV'];
T['VPMINUQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPMINUx_ronomo', '        '];
T['VPMINUQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPMINUx_ronomo', 'tuple=FV'];
T['VPMINUD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPMINUx_rynymy', '        '];
T['VPMINUD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPMINUx_rynymy', 'tuple=FV'];
T['VPMINUQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPMINUx_rynymy', '        '];
T['VPMINUQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPMINUx_rynymy', 'tuple=FV'];
T['VPMINUD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPMINUx_rznzmz', '        '];
T['VPMINUD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPMINUx_rznzmz', 'tuple=FV'];
T['VPMINUQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPMINUx_rznzmz', '        '];
T['VPMINUQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPMINUx_rznzmz', 'tuple=FV'];

# PMOVMSKB-Move Byte Mask.
T['VPMOVMSKB REG:w:u32=$GPR32r VREG:r:s8=$XMMm', 'NONE', 'VPMOVMSKB_rdmo', ''];
T['VPMOVMSKB REG:w:u32=$GPR32r VREG:r:s8=$YMMm', 'NONE', 'VPMOVMSKB_rdmy', ''];

# PMOVSX-Packed Move with Sign Extend.
T['VPMOVSXBD VREG:w:s32=$XMMr VREG:r:s8=$XMMm                    ', 'MOD=REG', 'VPMOVSXBD_romd  ', '         '];
T['VPMOVSXBD VREG:w:s32=$XMMr MEM:r:s8=$mem32                    ', 'MOD=MEM', 'VPMOVSXBD_romd  ', '         '];
T['VPMOVSXBD VREG:w:s32=$XMMr PREG:r:Z=$Kk      VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPMOVSXBD_romx  ', '         '];
T['VPMOVSXBD VREG:w:s32=$XMMr PREG:r:Z=$Kk      MEM:r:s8=$mem32  ', 'MOD=MEM', 'VPMOVSXBD_romx  ', 'tuple=QVM'];
T['VPMOVSXBD VREG:w:s32=$YMMr VREG:r:s8=$XMMm                    ', 'MOD=REG', 'VPMOVSXBD_rymq  ', '         '];
T['VPMOVSXBD VREG:w:s32=$YMMr MEM:r:s8=$mem64                    ', 'MOD=MEM', 'VPMOVSXBD_rymq  ', '         '];
T['VPMOVSXBD VREG:w:s32=$YMMr PREG:r:Z=$Kk      VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPMOVSXBD_rymx  ', '         '];
T['VPMOVSXBD VREG:w:s32=$YMMr PREG:r:Z=$Kk      MEM:r:s8=$mem64  ', 'MOD=MEM', 'VPMOVSXBD_rymx  ', 'tuple=QVM'];
T['VPMOVSXBD VREG:w:s32=$ZMMr PREG:r:Z=$Kk      VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPMOVSXBD_rzmo  ', '         '];
T['VPMOVSXBD VREG:w:s32=$ZMMr PREG:r:Z=$Kk      MEM:r:s8=$mem128 ', 'MOD=MEM', 'VPMOVSXBD_rzmo  ', 'tuple=QVM'];
T['VPMOVSXBQ VREG:w:s64=$XMMr VREG:r:s8=$XMMm                    ', 'MOD=REG', 'VPMOVSXBQ_romw  ', '         '];
T['VPMOVSXBQ VREG:w:s64=$XMMr MEM:r:s8=$mem16                    ', 'MOD=MEM', 'VPMOVSXBQ_romw  ', '         '];
T['VPMOVSXBQ VREG:w:s64=$XMMr PREG:r:Z=$Kk      VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPMOVSXBQ_romx  ', '         '];
T['VPMOVSXBQ VREG:w:s64=$XMMr PREG:r:Z=$Kk      MEM:r:s8=$mem16  ', 'MOD=MEM', 'VPMOVSXBQ_romx  ', 'tuple=OVM'];
T['VPMOVSXBQ VREG:w:s64=$YMMr VREG:r:s8=$XMMm                    ', 'MOD=REG', 'VPMOVSXBQ_rymd  ', '         '];
T['VPMOVSXBQ VREG:w:s64=$YMMr MEM:r:s8=$mem32                    ', 'MOD=MEM', 'VPMOVSXBQ_rymd  ', '         '];
T['VPMOVSXBQ VREG:w:s64=$YMMr PREG:r:Z=$Kk      VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPMOVSXBQ_rymx  ', '         '];
T['VPMOVSXBQ VREG:w:s64=$YMMr PREG:r:Z=$Kk      MEM:r:s8=$mem32  ', 'MOD=MEM', 'VPMOVSXBQ_rymx  ', 'tuple=OVM'];
T['VPMOVSXBQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk      VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPMOVSXBQ_rzmx  ', '         '];
T['VPMOVSXBQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk      MEM:r:s8=$mem64  ', 'MOD=MEM', 'VPMOVSXBQ_rzmx  ', 'tuple=OVM'];
T['VPMOVSXBW VREG:w:s16=$XMMr VREG:r:s8=$XMMm                    ', 'MOD=REG', 'VPMOVSXBW_romq  ', '         '];
T['VPMOVSXBW VREG:w:s16=$XMMr MEM:r:s8=$mem64                    ', 'MOD=MEM', 'VPMOVSXBW_romq  ', '         '];
T['VPMOVSXBW VREG:w:s16=$XMMr PREG:r:Z=$Kk      VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPMOVSXBW_romx  ', '         '];
T['VPMOVSXBW VREG:w:s16=$XMMr PREG:r:Z=$Kk      MEM:r:s8=$mem64  ', 'MOD=MEM', 'VPMOVSXBW_romx  ', 'tuple=HVM'];
T['VPMOVSXBW VREG:w:s16=$YMMr VREG:r:s8=$XMMm                    ', 'MOD=REG', 'VPMOVSXBW_rymo_1', '         '];
T['VPMOVSXBW VREG:w:s16=$YMMr MEM:r:s8=$mem128                   ', 'MOD=MEM', 'VPMOVSXBW_rymo_1', '         '];
T['VPMOVSXBW VREG:w:s16=$YMMr PREG:r:Z=$Kk      VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPMOVSXBW_rymo_2', '         '];
T['VPMOVSXBW VREG:w:s16=$YMMr PREG:r:Z=$Kk      MEM:r:s8=$mem128 ', 'MOD=MEM', 'VPMOVSXBW_rymo_2', 'tuple=HVM'];
T['VPMOVSXBW VREG:w:s16=$ZMMr PREG:r:Z=$Kk      VREG:r:s8=$YMMm  ', 'MOD=REG', 'VPMOVSXBW_rzmy  ', '         '];
T['VPMOVSXBW VREG:w:s16=$ZMMr PREG:r:Z=$Kk      MEM:r:s8=$mem256 ', 'MOD=MEM', 'VPMOVSXBW_rzmy  ', 'tuple=HVM'];
T['VPMOVSXDQ VREG:w:s64=$XMMr VREG:r:s32=$XMMm                   ', 'MOD=REG', 'VPMOVSXDQ_romq  ', '         '];
T['VPMOVSXDQ VREG:w:s64=$XMMr MEM:r:s32=$mem64                   ', 'MOD=MEM', 'VPMOVSXDQ_romq  ', '         '];
T['VPMOVSXDQ VREG:w:s64=$XMMr PREG:r:Z=$Kk      VREG:r:s32=$XMMm ', 'MOD=REG', 'VPMOVSXDQ_romx  ', '         '];
T['VPMOVSXDQ VREG:w:s64=$XMMr PREG:r:Z=$Kk      MEM:r:s32=$mem64 ', 'MOD=MEM', 'VPMOVSXDQ_romx  ', 'tuple=HVM'];
T['VPMOVSXDQ VREG:w:s64=$YMMr VREG:r:s32=$XMMm                   ', 'MOD=REG', 'VPMOVSXDQ_rymo_1', '         '];
T['VPMOVSXDQ VREG:w:s64=$YMMr MEM:r:s32=$mem128                  ', 'MOD=MEM', 'VPMOVSXDQ_rymo_1', '         '];
T['VPMOVSXDQ VREG:w:s64=$YMMr PREG:r:Z=$Kk      VREG:r:s32=$XMMm ', 'MOD=REG', 'VPMOVSXDQ_rymo_2', '         '];
T['VPMOVSXDQ VREG:w:s64=$YMMr PREG:r:Z=$Kk      MEM:r:s32=$mem128', 'MOD=MEM', 'VPMOVSXDQ_rymo_2', 'tuple=HVM'];
T['VPMOVSXDQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk      VREG:r:s32=$YMMm ', 'MOD=REG', 'VPMOVSXDQ_rzmy  ', '         '];
T['VPMOVSXDQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk      MEM:r:s32=$mem256', 'MOD=MEM', 'VPMOVSXDQ_rzmy  ', 'tuple=HVM'];
T['VPMOVSXWD VREG:w:s32=$XMMr VREG:r:s16=$XMMm                   ', 'MOD=REG', 'VPMOVSXWD_romq  ', '         '];
T['VPMOVSXWD VREG:w:s32=$XMMr MEM:r:s16=$mem64                   ', 'MOD=MEM', 'VPMOVSXWD_romq  ', '         '];
T['VPMOVSXWD VREG:w:s32=$XMMr PREG:r:Z=$Kk      VREG:r:s16=$XMMm ', 'MOD=REG', 'VPMOVSXWD_romx  ', '         '];
T['VPMOVSXWD VREG:w:s32=$XMMr PREG:r:Z=$Kk      MEM:r:s16=$mem64 ', 'MOD=MEM', 'VPMOVSXWD_romx  ', 'tuple=HVM'];
T['VPMOVSXWD VREG:w:s32=$YMMr VREG:r:s16=$XMMm                   ', 'MOD=REG', 'VPMOVSXWD_rymo_1', '         '];
T['VPMOVSXWD VREG:w:s32=$YMMr MEM:r:s16=$mem128                  ', 'MOD=MEM', 'VPMOVSXWD_rymo_1', '         '];
T['VPMOVSXWD VREG:w:s32=$YMMr PREG:r:Z=$Kk      VREG:r:s16=$XMMm ', 'MOD=REG', 'VPMOVSXWD_rymo_2', '         '];
T['VPMOVSXWD VREG:w:s32=$YMMr PREG:r:Z=$Kk      MEM:r:s16=$mem128', 'MOD=MEM', 'VPMOVSXWD_rymo_2', 'tuple=HVM'];
T['VPMOVSXWD VREG:w:s32=$ZMMr PREG:r:Z=$Kk      VREG:r:s16=$YMMm ', 'MOD=REG', 'VPMOVSXWD_rzmy  ', '         '];
T['VPMOVSXWD VREG:w:s32=$ZMMr PREG:r:Z=$Kk      MEM:r:s16=$mem256', 'MOD=MEM', 'VPMOVSXWD_rzmy  ', 'tuple=HVM'];
T['VPMOVSXWQ VREG:w:s64=$XMMr VREG:r:s16=$XMMm                   ', 'MOD=REG', 'VPMOVSXWQ_romd  ', '         '];
T['VPMOVSXWQ VREG:w:s64=$XMMr MEM:r:s16=$mem32                   ', 'MOD=MEM', 'VPMOVSXWQ_romd  ', '         '];
T['VPMOVSXWQ VREG:w:s64=$XMMr PREG:r:Z=$Kk      VREG:r:s16=$XMMm ', 'MOD=REG', 'VPMOVSXWQ_romx  ', '         '];
T['VPMOVSXWQ VREG:w:s64=$XMMr PREG:r:Z=$Kk      MEM:r:s16=$mem32 ', 'MOD=MEM', 'VPMOVSXWQ_romx  ', 'tuple=QVM'];
T['VPMOVSXWQ VREG:w:s64=$YMMr VREG:r:s16=$XMMm                   ', 'MOD=REG', 'VPMOVSXWQ_rymq  ', '         '];
T['VPMOVSXWQ VREG:w:s64=$YMMr MEM:r:s16=$mem64                   ', 'MOD=MEM', 'VPMOVSXWQ_rymq  ', '         '];
T['VPMOVSXWQ VREG:w:s64=$YMMr PREG:r:Z=$Kk      VREG:r:s16=$XMMm ', 'MOD=REG', 'VPMOVSXWQ_rymx  ', '         '];
T['VPMOVSXWQ VREG:w:s64=$YMMr PREG:r:Z=$Kk      MEM:r:s16=$mem64 ', 'MOD=MEM', 'VPMOVSXWQ_rymx  ', 'tuple=QVM'];
T['VPMOVSXWQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk      VREG:r:s16=$XMMm ', 'MOD=REG', 'VPMOVSXWQ_rzmo  ', '         '];
T['VPMOVSXWQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk      MEM:r:s16=$mem128', 'MOD=MEM', 'VPMOVSXWQ_rzmo  ', 'tuple=QVM'];

# PMOVZX-Packed Move with Zero Extend.
T['VPMOVZXBD VREG:w:u32=$XMMr VREG:r:u8=$XMMm                    ', 'MOD=REG', 'VPMOVZXBD_romd  ', '         '];
T['VPMOVZXBD VREG:w:u32=$XMMr MEM:r:u8=$mem32                    ', 'MOD=MEM', 'VPMOVZXBD_romd  ', '         '];
T['VPMOVZXBD VREG:w:s32=$XMMr PREG:r:Z=$Kk      VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPMOVZXBD_romx  ', '         '];
T['VPMOVZXBD VREG:w:s32=$XMMr PREG:r:Z=$Kk      MEM:r:s8=$mem32  ', 'MOD=MEM', 'VPMOVZXBD_romx  ', 'tuple=QVM'];
T['VPMOVZXBD VREG:w:u32=$YMMr VREG:r:u8=$XMMm                    ', 'MOD=REG', 'VPMOVZXBD_rymq  ', '         '];
T['VPMOVZXBD VREG:w:u32=$YMMr MEM:r:u8=$mem64                    ', 'MOD=MEM', 'VPMOVZXBD_rymq  ', '         '];
T['VPMOVZXBD VREG:w:s32=$YMMr PREG:r:Z=$Kk      VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPMOVZXBD_rymx  ', '         '];
T['VPMOVZXBD VREG:w:s32=$YMMr PREG:r:Z=$Kk      MEM:r:s8=$mem64  ', 'MOD=MEM', 'VPMOVZXBD_rymx  ', 'tuple=QVM'];
T['VPMOVZXBD VREG:w:s32=$ZMMr PREG:r:Z=$Kk      VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPMOVZXBD_rzmo  ', '         '];
T['VPMOVZXBD VREG:w:s32=$ZMMr PREG:r:Z=$Kk      MEM:r:s8=$mem128 ', 'MOD=MEM', 'VPMOVZXBD_rzmo  ', 'tuple=QVM'];
T['VPMOVZXBQ VREG:w:u64=$XMMr VREG:r:u8=$XMMm                    ', 'MOD=REG', 'VPMOVZXBQ_romw  ', '         '];
T['VPMOVZXBQ VREG:w:u64=$XMMr MEM:r:u8=$mem16                    ', 'MOD=MEM', 'VPMOVZXBQ_romw  ', '         '];
T['VPMOVZXBQ VREG:w:s64=$XMMr PREG:r:Z=$Kk      VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPMOVZXBQ_romx  ', '         '];
T['VPMOVZXBQ VREG:w:s64=$XMMr PREG:r:Z=$Kk      MEM:r:s8=$mem16  ', 'MOD=MEM', 'VPMOVZXBQ_romx  ', 'tuple=OVM'];
T['VPMOVZXBQ VREG:w:u64=$YMMr VREG:r:u8=$XMMm                    ', 'MOD=REG', 'VPMOVZXBQ_rymd  ', '         '];
T['VPMOVZXBQ VREG:w:u64=$YMMr MEM:r:u8=$mem32                    ', 'MOD=MEM', 'VPMOVZXBQ_rymd  ', '         '];
T['VPMOVZXBQ VREG:w:s64=$YMMr PREG:r:Z=$Kk      VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPMOVZXBQ_rymx  ', '         '];
T['VPMOVZXBQ VREG:w:s64=$YMMr PREG:r:Z=$Kk      MEM:r:s8=$mem32  ', 'MOD=MEM', 'VPMOVZXBQ_rymx  ', 'tuple=OVM'];
T['VPMOVZXBQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk      VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPMOVZXBQ_rzmx  ', '         '];
T['VPMOVZXBQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk      MEM:r:s8=$mem64  ', 'MOD=MEM', 'VPMOVZXBQ_rzmx  ', 'tuple=OVM'];
T['VPMOVZXBW VREG:w:u16=$XMMr VREG:r:u8=$XMMm                    ', 'MOD=REG', 'VPMOVZXBW_romq  ', '         '];
T['VPMOVZXBW VREG:w:u16=$XMMr MEM:r:u8=$mem64                    ', 'MOD=MEM', 'VPMOVZXBW_romq  ', '         '];
T['VPMOVZXBW VREG:w:s16=$XMMr PREG:r:Z=$Kk      VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPMOVZXBW_romx  ', '         '];
T['VPMOVZXBW VREG:w:s16=$XMMr PREG:r:Z=$Kk      MEM:r:s8=$mem64  ', 'MOD=MEM', 'VPMOVZXBW_romx  ', 'tuple=HVM'];
T['VPMOVZXBW VREG:w:u16=$YMMr VREG:r:u8=$XMMm                    ', 'MOD=REG', 'VPMOVZXBW_rymo_1', '         '];
T['VPMOVZXBW VREG:w:u16=$YMMr MEM:r:u8=$mem128                   ', 'MOD=MEM', 'VPMOVZXBW_rymo_1', '         '];
T['VPMOVZXBW VREG:w:s16=$YMMr PREG:r:Z=$Kk      VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPMOVZXBW_rymo_2', '         '];
T['VPMOVZXBW VREG:w:s16=$YMMr PREG:r:Z=$Kk      MEM:r:s8=$mem128 ', 'MOD=MEM', 'VPMOVZXBW_rymo_2', 'tuple=HVM'];
T['VPMOVZXBW VREG:w:s16=$ZMMr PREG:r:Z=$Kk      VREG:r:s8=$YMMm  ', 'MOD=REG', 'VPMOVZXBW_rzmy  ', '         '];
T['VPMOVZXBW VREG:w:s16=$ZMMr PREG:r:Z=$Kk      MEM:r:s8=$mem256 ', 'MOD=MEM', 'VPMOVZXBW_rzmy  ', 'tuple=HVM'];
T['VPMOVZXDQ VREG:w:u64=$XMMr VREG:r:u32=$XMMm                   ', 'MOD=REG', 'VPMOVZXDQ_romq  ', '         '];
T['VPMOVZXDQ VREG:w:u64=$XMMr MEM:r:u32=$mem64                   ', 'MOD=MEM', 'VPMOVZXDQ_romq  ', '         '];
T['VPMOVZXDQ VREG:w:s64=$XMMr PREG:r:Z=$Kk      VREG:r:s32=$XMMm ', 'MOD=REG', 'VPMOVZXDQ_romx  ', '         '];
T['VPMOVZXDQ VREG:w:s64=$XMMr PREG:r:Z=$Kk      MEM:r:s32=$mem64 ', 'MOD=MEM', 'VPMOVZXDQ_romx  ', 'tuple=HVM'];
T['VPMOVZXDQ VREG:w:u64=$YMMr VREG:r:u32=$XMMm                   ', 'MOD=REG', 'VPMOVZXDQ_rymo_1', '         '];
T['VPMOVZXDQ VREG:w:u64=$YMMr MEM:r:u32=$mem128                  ', 'MOD=MEM', 'VPMOVZXDQ_rymo_1', '         '];
T['VPMOVZXDQ VREG:w:s64=$YMMr PREG:r:Z=$Kk      VREG:r:s32=$XMMm ', 'MOD=REG', 'VPMOVZXDQ_rymo_2', '         '];
T['VPMOVZXDQ VREG:w:s64=$YMMr PREG:r:Z=$Kk      MEM:r:s32=$mem128', 'MOD=MEM', 'VPMOVZXDQ_rymo_2', 'tuple=HVM'];
T['VPMOVZXDQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk      VREG:r:s32=$YMMm ', 'MOD=REG', 'VPMOVZXDQ_rzmy  ', '         '];
T['VPMOVZXDQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk      MEM:r:s32=$mem256', 'MOD=MEM', 'VPMOVZXDQ_rzmy  ', 'tuple=HVM'];
T['VPMOVZXWD VREG:w:u32=$XMMr VREG:r:u16=$XMMm                   ', 'MOD=REG', 'VPMOVZXWD_romq  ', '         '];
T['VPMOVZXWD VREG:w:u32=$XMMr MEM:r:u16=$mem64                   ', 'MOD=MEM', 'VPMOVZXWD_romq  ', '         '];
T['VPMOVZXWD VREG:w:s32=$XMMr PREG:r:Z=$Kk      VREG:r:s16=$XMMm ', 'MOD=REG', 'VPMOVZXWD_romx  ', '         '];
T['VPMOVZXWD VREG:w:s32=$XMMr PREG:r:Z=$Kk      MEM:r:s16=$mem64 ', 'MOD=MEM', 'VPMOVZXWD_romx  ', 'tuple=HVM'];
T['VPMOVZXWD VREG:w:u32=$YMMr VREG:r:u16=$XMMm                   ', 'MOD=REG', 'VPMOVZXWD_rymo_1', '         '];
T['VPMOVZXWD VREG:w:u32=$YMMr MEM:r:u16=$mem128                  ', 'MOD=MEM', 'VPMOVZXWD_rymo_1', '         '];
T['VPMOVZXWD VREG:w:s32=$YMMr PREG:r:Z=$Kk      VREG:r:s16=$XMMm ', 'MOD=REG', 'VPMOVZXWD_rymo_2', '         '];
T['VPMOVZXWD VREG:w:s32=$YMMr PREG:r:Z=$Kk      MEM:r:s16=$mem128', 'MOD=MEM', 'VPMOVZXWD_rymo_2', 'tuple=HVM'];
T['VPMOVZXWD VREG:w:s32=$ZMMr PREG:r:Z=$Kk      VREG:r:s16=$YMMm ', 'MOD=REG', 'VPMOVZXWD_rzmy  ', '         '];
T['VPMOVZXWD VREG:w:s32=$ZMMr PREG:r:Z=$Kk      MEM:r:s16=$mem256', 'MOD=MEM', 'VPMOVZXWD_rzmy  ', 'tuple=HVM'];
T['VPMOVZXWQ VREG:w:u64=$XMMr VREG:r:u16=$XMMm                   ', 'MOD=REG', 'VPMOVZXWQ_romd  ', '         '];
T['VPMOVZXWQ VREG:w:u64=$XMMr MEM:r:u16=$mem32                   ', 'MOD=MEM', 'VPMOVZXWQ_romd  ', '         '];
T['VPMOVZXWQ VREG:w:s64=$XMMr PREG:r:Z=$Kk      VREG:r:s16=$XMMm ', 'MOD=REG', 'VPMOVZXWQ_romx  ', '         '];
T['VPMOVZXWQ VREG:w:s64=$XMMr PREG:r:Z=$Kk      MEM:r:s16=$mem32 ', 'MOD=MEM', 'VPMOVZXWQ_romx  ', 'tuple=QVM'];
T['VPMOVZXWQ VREG:w:u64=$YMMr VREG:r:u16=$XMMm                   ', 'MOD=REG', 'VPMOVZXWQ_rymq  ', '         '];
T['VPMOVZXWQ VREG:w:u64=$YMMr MEM:r:u16=$mem64                   ', 'MOD=MEM', 'VPMOVZXWQ_rymq  ', '         '];
T['VPMOVZXWQ VREG:w:s64=$YMMr PREG:r:Z=$Kk      VREG:r:s16=$XMMm ', 'MOD=REG', 'VPMOVZXWQ_rymx  ', '         '];
T['VPMOVZXWQ VREG:w:s64=$YMMr PREG:r:Z=$Kk      MEM:r:s16=$mem64 ', 'MOD=MEM', 'VPMOVZXWQ_rymx  ', 'tuple=QVM'];
T['VPMOVZXWQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk      VREG:r:s16=$XMMm ', 'MOD=REG', 'VPMOVZXWQ_rzmo  ', '         '];
T['VPMOVZXWQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk      MEM:r:s16=$mem128', 'MOD=MEM', 'VPMOVZXWQ_rzmo  ', 'tuple=QVM'];

# PMULDQ-Multiply Packed Doubleword Integers.
T['VPMULDQ VREG:w:s64=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm                              ', 'MOD=REG     ', 'VPMULDQ_ronomo_1', '        '];
T['VPMULDQ VREG:w:s64=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128                             ', 'MOD=MEM     ', 'VPMULDQ_ronomo_1', '        '];
T['VPMULDQ VREG:w:s64=$XMMr PREG:r:Z=$Kk     VREG:r:s64=$XMMn  VREG:r:s64=$XMMm            ', 'BP=0 MOD=REG', 'VPMULDQ_ronomo_2', '        '];
T['VPMULDQ VREG:w:s64=$XMMr PREG:r:Z=$Kk     VREG:r:s64=$XMMn  MEM:r:s64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPMULDQ_ronomo_2', 'tuple=FV'];
T['VPMULDQ VREG:w:s64=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm                              ', 'MOD=REG     ', 'VPMULDQ_rynymy_1', '        '];
T['VPMULDQ VREG:w:s64=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256                             ', 'MOD=MEM     ', 'VPMULDQ_rynymy_1', '        '];
T['VPMULDQ VREG:w:s64=$YMMr PREG:r:Z=$Kk     VREG:r:s64=$YMMn  VREG:r:s64=$YMMm            ', 'BP=0 MOD=REG', 'VPMULDQ_rynymy_2', '        '];
T['VPMULDQ VREG:w:s64=$YMMr PREG:r:Z=$Kk     VREG:r:s64=$YMMn  MEM:r:s64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPMULDQ_rynymy_2', 'tuple=FV'];
T['VPMULDQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk     VREG:r:s64=$ZMMn  VREG:r:s64=$ZMMm            ', 'BP=0 MOD=REG', 'VPMULDQ_rznzmz  ', '        '];
T['VPMULDQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk     VREG:r:s64=$ZMMn  MEM:r:s64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPMULDQ_rznzmz  ', 'tuple=FV'];

# PMULHRSW-Packed Multiply High with Round and Scale.
T['VPMULHRSW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm                   ', 'MOD=REG', 'VPMULHRSW_ronomo_1', '         '];
T['VPMULHRSW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128                  ', 'MOD=MEM', 'VPMULHRSW_ronomo_1', '         '];
T['VPMULHRSW VREG:w:s16=$XMMr PREG:r:Z=$Kk     VREG:r:s16=$XMMn  VREG:r:s16=$XMMm ', 'MOD=REG', 'VPMULHRSW_ronomo_2', '         '];
T['VPMULHRSW VREG:w:s16=$XMMr PREG:r:Z=$Kk     VREG:r:s16=$XMMn  MEM:r:s16=$mem128', 'MOD=MEM', 'VPMULHRSW_ronomo_2', 'tuple=FVM'];
T['VPMULHRSW VREG:w:s16=$YMMr VREG:r:s16=$YMMn VREG:r:s16=$YMMm                   ', 'MOD=REG', 'VPMULHRSW_rynymy_1', '         '];
T['VPMULHRSW VREG:w:s16=$YMMr VREG:r:s16=$YMMn MEM:r:s16=$mem256                  ', 'MOD=MEM', 'VPMULHRSW_rynymy_1', '         '];
T['VPMULHRSW VREG:w:s16=$YMMr PREG:r:Z=$Kk     VREG:r:s16=$YMMn  VREG:r:s16=$YMMm ', 'MOD=REG', 'VPMULHRSW_rynymy_2', '         '];
T['VPMULHRSW VREG:w:s16=$YMMr PREG:r:Z=$Kk     VREG:r:s16=$YMMn  MEM:r:s16=$mem256', 'MOD=MEM', 'VPMULHRSW_rynymy_2', 'tuple=FVM'];
T['VPMULHRSW VREG:w:s16=$ZMMr PREG:r:Z=$Kk     VREG:r:s16=$ZMMn  VREG:r:s16=$ZMMm ', 'MOD=REG', 'VPMULHRSW_rznzmz  ', '         '];
T['VPMULHRSW VREG:w:s16=$ZMMr PREG:r:Z=$Kk     VREG:r:s16=$ZMMn  MEM:r:s16=$mem512', 'MOD=MEM', 'VPMULHRSW_rznzmz  ', 'tuple=FVM'];

# PMULHUW-Multiply Packed Unsigned Integers and Store High Result.
T['VPMULHUW VREG:w:u16=$XMMr VREG:r:u16=$XMMn VREG:r:u16=$XMMm                   ', 'MOD=REG', 'VPMULHUW_ronomo_1', '         '];
T['VPMULHUW VREG:w:u16=$XMMr VREG:r:u16=$XMMn MEM:r:u16=$mem128                  ', 'MOD=MEM', 'VPMULHUW_ronomo_1', '         '];
T['VPMULHUW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm ', 'MOD=REG', 'VPMULHUW_ronomo_2', '         '];
T['VPMULHUW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128', 'MOD=MEM', 'VPMULHUW_ronomo_2', 'tuple=FVM'];
T['VPMULHUW VREG:w:u16=$YMMr VREG:r:u16=$YMMn VREG:r:u16=$YMMm                   ', 'MOD=REG', 'VPMULHUW_rynymy_1', '         '];
T['VPMULHUW VREG:w:u16=$YMMr VREG:r:u16=$YMMn MEM:r:u16=$mem256                  ', 'MOD=MEM', 'VPMULHUW_rynymy_1', '         '];
T['VPMULHUW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm ', 'MOD=REG', 'VPMULHUW_rynymy_2', '         '];
T['VPMULHUW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256', 'MOD=MEM', 'VPMULHUW_rynymy_2', 'tuple=FVM'];
T['VPMULHUW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm ', 'MOD=REG', 'VPMULHUW_rznzmz  ', '         '];
T['VPMULHUW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512', 'MOD=MEM', 'VPMULHUW_rznzmz  ', 'tuple=FVM'];

# PMULHW-Multiply Packed Signed Integers and Store High Result.
T['VPMULHW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm                   ', 'MOD=REG', 'VPMULHW_ronomo_1', '         '];
T['VPMULHW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128                  ', 'MOD=MEM', 'VPMULHW_ronomo_1', '         '];
T['VPMULHW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm ', 'MOD=REG', 'VPMULHW_ronomo_2', '         '];
T['VPMULHW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128', 'MOD=MEM', 'VPMULHW_ronomo_2', 'tuple=FVM'];
T['VPMULHW VREG:w:s16=$YMMr VREG:r:s16=$YMMn VREG:r:s16=$YMMm                   ', 'MOD=REG', 'VPMULHW_rynymy_1', '         '];
T['VPMULHW VREG:w:s16=$YMMr VREG:r:s16=$YMMn MEM:r:s16=$mem256                  ', 'MOD=MEM', 'VPMULHW_rynymy_1', '         '];
T['VPMULHW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm ', 'MOD=REG', 'VPMULHW_rynymy_2', '         '];
T['VPMULHW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256', 'MOD=MEM', 'VPMULHW_rynymy_2', 'tuple=FVM'];
T['VPMULHW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm ', 'MOD=REG', 'VPMULHW_rznzmz  ', '         '];
T['VPMULHW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512', 'MOD=MEM', 'VPMULHW_rznzmz  ', 'tuple=FVM'];

# PMULLW-Multiply Packed Signed Integers and Store Low Result.
T['VPMULLW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm                   ', 'MOD=REG', 'VPMULLW_ronomo_1', '         '];
T['VPMULLW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128                  ', 'MOD=MEM', 'VPMULLW_ronomo_1', '         '];
T['VPMULLW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm ', 'MOD=REG', 'VPMULLW_ronomo_2', '         '];
T['VPMULLW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128', 'MOD=MEM', 'VPMULLW_ronomo_2', 'tuple=FVM'];
T['VPMULLW VREG:w:s16=$YMMr VREG:r:s16=$YMMn VREG:r:s16=$YMMm                   ', 'MOD=REG', 'VPMULLW_rynymy_1', '         '];
T['VPMULLW VREG:w:s16=$YMMr VREG:r:s16=$YMMn MEM:r:s16=$mem256                  ', 'MOD=MEM', 'VPMULLW_rynymy_1', '         '];
T['VPMULLW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm ', 'MOD=REG', 'VPMULLW_rynymy_2', '         '];
T['VPMULLW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256', 'MOD=MEM', 'VPMULLW_rynymy_2', 'tuple=FVM'];
T['VPMULLW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm ', 'MOD=REG', 'VPMULLW_rznzmz  ', '         '];
T['VPMULLW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512', 'MOD=MEM', 'VPMULLW_rznzmz  ', 'tuple=FVM'];

# PMULLD/PMULLQ-Multiply Packed Integers and Store Low Result.
T['VPMULLD VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm                              ', 'MOD=REG     ', 'VPMULLD_ronomo_1', '        '];
T['VPMULLD VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128                             ', 'MOD=MEM     ', 'VPMULLD_ronomo_1', '        '];
T['VPMULLD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VPMULLD_ronomo_2', '        '];
T['VPMULLD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPMULLD_ronomo_2', 'tuple=FV'];
T['VPMULLD VREG:w:s32=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm                              ', 'MOD=REG     ', 'VPMULLD_rynymy_1', '        '];
T['VPMULLD VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256                             ', 'MOD=MEM     ', 'VPMULLD_rynymy_1', '        '];
T['VPMULLD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VPMULLD_rynymy_2', '        '];
T['VPMULLD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPMULLD_rynymy_2', 'tuple=FV'];
T['VPMULLD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VPMULLD_rznzmz  ', '        '];
T['VPMULLD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPMULLD_rznzmz  ', 'tuple=FV'];
T['VPMULLQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG', 'VPMULLQ_ronomo  ', '        '];
T['VPMULLQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPMULLQ_ronomo  ', 'tuple=FV'];
T['VPMULLQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG', 'VPMULLQ_rynymy  ', '        '];
T['VPMULLQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPMULLQ_rynymy  ', 'tuple=FV'];
T['VPMULLQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG', 'VPMULLQ_rznzmz  ', '        '];
T['VPMULLQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPMULLQ_rznzmz  ', 'tuple=FV'];

# PMULUDQ-Multiply Packed Unsigned Doubleword Integers.
T['VPMULUDQ VREG:w:u64=$XMMr VREG:r:u32=$XMMn VREG:r:u32=$XMMm                              ', 'MOD=REG     ', 'VPMULUDQ_ronomo_1', '        '];
T['VPMULUDQ VREG:w:u64=$XMMr VREG:r:u32=$XMMn MEM:r:u32=$mem128                             ', 'MOD=MEM     ', 'VPMULUDQ_ronomo_1', '        '];
T['VPMULUDQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG', 'VPMULUDQ_ronomo_2', '        '];
T['VPMULUDQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPMULUDQ_ronomo_2', 'tuple=FV'];
T['VPMULUDQ VREG:w:u64=$YMMr VREG:r:u32=$YMMn VREG:r:u32=$YMMm                              ', 'MOD=REG     ', 'VPMULUDQ_rynymy_1', '        '];
T['VPMULUDQ VREG:w:u64=$YMMr VREG:r:u32=$YMMn MEM:r:u32=$mem256                             ', 'MOD=MEM     ', 'VPMULUDQ_rynymy_1', '        '];
T['VPMULUDQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG', 'VPMULUDQ_rynymy_2', '        '];
T['VPMULUDQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPMULUDQ_rynymy_2', 'tuple=FV'];
T['VPMULUDQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG', 'VPMULUDQ_rznzmz  ', '        '];
T['VPMULUDQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPMULUDQ_rznzmz  ', 'tuple=FV'];

# POR-Bitwise Logical OR.
T['VPOR REG:w:u128=$XMMr REG:r:u128=$XMMn REG:r:u128=$XMMm  ', 'MOD=REG', 'VPOR_ronomo', ''];
T['VPOR REG:w:u128=$XMMr REG:r:u128=$XMMn MEM:r:u128=$mem128', 'MOD=MEM', 'VPOR_ronomo', ''];
T['VPOR REG:w:u256=$YMMr REG:r:u256=$YMMn REG:r:u256=$YMMm  ', 'MOD=REG', 'VPOR_rynymy', ''];
T['VPOR REG:w:u256=$YMMr REG:r:u256=$YMMn MEM:r:u256=$mem256', 'MOD=MEM', 'VPOR_rynymy', ''];

# PSADBW-Compute Sum of Absolute Differences.
T['VPSADBW VREG:w:u16=$XMMr VREG:r:u8=$XMMn VREG:r:u8=$XMMm ', 'MOD=REG', 'VPSADBW_ronomo_1', '         '];
T['VPSADBW VREG:w:u16=$XMMr VREG:r:u8=$XMMn MEM:r:u8=$mem128', 'MOD=MEM', 'VPSADBW_ronomo_1', '         '];
T['VPSADBW VREG:w:u16=$XMMr VREG:r:u8=$XMMn VREG:r:u8=$XMMm ', 'MOD=REG', 'VPSADBW_ronomo_2', '         '];
T['VPSADBW VREG:w:u16=$XMMr VREG:r:u8=$XMMn MEM:r:u8=$mem128', 'MOD=MEM', 'VPSADBW_ronomo_2', 'tuple=FVM'];
T['VPSADBW VREG:w:u16=$YMMr VREG:r:u8=$YMMn VREG:r:u8=$YMMm ', 'MOD=REG', 'VPSADBW_rynymy_1', '         '];
T['VPSADBW VREG:w:u16=$YMMr VREG:r:u8=$YMMn MEM:r:u8=$mem256', 'MOD=MEM', 'VPSADBW_rynymy_1', '         '];
T['VPSADBW VREG:w:u16=$YMMr VREG:r:u8=$YMMn VREG:r:u8=$YMMm ', 'MOD=REG', 'VPSADBW_rynymy_2', '         '];
T['VPSADBW VREG:w:u16=$YMMr VREG:r:u8=$YMMn MEM:r:u8=$mem256', 'MOD=MEM', 'VPSADBW_rynymy_2', 'tuple=FVM'];
T['VPSADBW VREG:w:u16=$ZMMr VREG:r:u8=$ZMMn VREG:r:u8=$ZMMm ', 'MOD=REG', 'VPSADBW_rznzmz  ', '         '];
T['VPSADBW VREG:w:u16=$ZMMr VREG:r:u8=$ZMMn MEM:r:u8=$mem512', 'MOD=MEM', 'VPSADBW_rznzmz  ', 'tuple=FVM'];

# PSHUFB-Packed Shuffle Bytes.
T['VPSHUFB VREG:w:u8=$XMMr VREG:r:u8=$XMMn VREG:r:u8=$XMMm                  ', 'MOD=REG', 'VPSHUFB_ronomo_1', '         '];
T['VPSHUFB VREG:w:u8=$XMMr VREG:r:u8=$XMMn MEM:r:u8=$mem128                 ', 'MOD=MEM', 'VPSHUFB_ronomo_1', '         '];
T['VPSHUFB VREG:w:u8=$XMMr PREG:r:Z=$Kk    VREG:r:u8=$XMMn  VREG:r:u8=$XMMm ', 'MOD=REG', 'VPSHUFB_ronomo_2', '         '];
T['VPSHUFB VREG:w:u8=$XMMr PREG:r:Z=$Kk    VREG:r:u8=$XMMn  MEM:r:u8=$mem128', 'MOD=MEM', 'VPSHUFB_ronomo_2', 'tuple=FVM'];
T['VPSHUFB VREG:w:u8=$YMMr VREG:r:u8=$YMMn VREG:r:u8=$YMMm                  ', 'MOD=REG', 'VPSHUFB_rynymy_1', '         '];
T['VPSHUFB VREG:w:u8=$YMMr VREG:r:u8=$YMMn MEM:r:u8=$mem256                 ', 'MOD=MEM', 'VPSHUFB_rynymy_1', '         '];
T['VPSHUFB VREG:w:u8=$YMMr PREG:r:Z=$Kk    VREG:r:u8=$YMMn  VREG:r:u8=$YMMm ', 'MOD=REG', 'VPSHUFB_rynymy_2', '         '];
T['VPSHUFB VREG:w:u8=$YMMr PREG:r:Z=$Kk    VREG:r:u8=$YMMn  MEM:r:u8=$mem256', 'MOD=MEM', 'VPSHUFB_rynymy_2', 'tuple=FVM'];
T['VPSHUFB VREG:w:u8=$ZMMr PREG:r:Z=$Kk    VREG:r:u8=$ZMMn  VREG:r:u8=$ZMMm ', 'MOD=REG', 'VPSHUFB_rznzmz  ', '         '];
T['VPSHUFB VREG:w:u8=$ZMMr PREG:r:Z=$Kk    VREG:r:u8=$ZMMn  MEM:r:u8=$mem512', 'MOD=MEM', 'VPSHUFB_rznzmz  ', 'tuple=FVM'];

# PSHUFD-Shuffle Packed Doublewords.
T['VPSHUFD VREG:w:s32=$XMMr VREG:r:s32=$XMMm  ORDER:r:u8=$order                             ', 'MOD=REG     ', 'VPSHUFD_romoub_1', '        '];
T['VPSHUFD VREG:w:s32=$XMMr MEM:r:s32=$mem128 ORDER:r:u8=$order                             ', 'MOD=MEM     ', 'VPSHUFD_romoub_1', '        '];
T['VPSHUFD VREG:w:u32=$XMMr PREG:r:Z=$Kk      VREG:r:u32=$XMMm             ORDER:r:u8=$order', 'BP=0 MOD=REG', 'VPSHUFD_romoub_2', '        '];
T['VPSHUFD VREG:w:u32=$XMMr PREG:r:Z=$Kk      MEM:r:u32=$mem128 BCST=$BCST ORDER:r:u8=$order', 'MOD=MEM     ', 'VPSHUFD_romoub_2', 'tuple=FV'];
T['VPSHUFD VREG:w:u32=$YMMr VREG:r:u32=$YMMm  ORDER:r:u8=$order                             ', 'MOD=REG     ', 'VPSHUFD_rymyub_1', '        '];
T['VPSHUFD VREG:w:u32=$YMMr MEM:r:u32=$mem256 ORDER:r:u8=$order                             ', 'MOD=MEM     ', 'VPSHUFD_rymyub_1', '        '];
T['VPSHUFD VREG:w:u32=$YMMr PREG:r:Z=$Kk      VREG:r:u32=$YMMm             ORDER:r:u8=$order', 'BP=0 MOD=REG', 'VPSHUFD_rymyub_2', '        '];
T['VPSHUFD VREG:w:u32=$YMMr PREG:r:Z=$Kk      MEM:r:u32=$mem256 BCST=$BCST ORDER:r:u8=$order', 'MOD=MEM     ', 'VPSHUFD_rymyub_2', 'tuple=FV'];
T['VPSHUFD VREG:w:u32=$ZMMr PREG:r:Z=$Kk      VREG:r:u32=$ZMMm             ORDER:r:u8=$order', 'BP=0 MOD=REG', 'VPSHUFD_rzmzub_1', '        '];
T['VPSHUFD VREG:w:u32=$ZMMr PREG:r:Z=$Kk      MEM:r:u32=$mem512 BCST=$BCST ORDER:r:u8=$order', 'MOD=MEM     ', 'VPSHUFD_rzmzub_1', 'tuple=FV'];

# PSHUFHW-Shuffle Packed High Words.
T['VPSHUFHW VREG:w:s32=$XMMr VREG:r:s32=$XMMm  ORDER:r:u8=$order                  ', 'MOD=REG', 'VPSHUFHW_romoub_1', '         '];
T['VPSHUFHW VREG:w:s32=$XMMr MEM:r:s32=$mem128 ORDER:r:u8=$order                  ', 'MOD=MEM', 'VPSHUFHW_romoub_1', '         '];
T['VPSHUFHW VREG:w:u16=$XMMr PREG:r:Z=$Kk      VREG:r:u16=$XMMm  ORDER:r:u8=$order', 'MOD=REG', 'VPSHUFHW_romoub_2', '         '];
T['VPSHUFHW VREG:w:u16=$XMMr PREG:r:Z=$Kk      MEM:r:u16=$mem128 ORDER:r:u8=$order', 'MOD=MEM', 'VPSHUFHW_romoub_2', 'tuple=FVM'];
T['VPSHUFHW VREG:w:u16=$YMMr VREG:r:u16=$YMMm  ORDER:r:u8=$order                  ', 'MOD=REG', 'VPSHUFHW_rymyub_1', '         '];
T['VPSHUFHW VREG:w:u16=$YMMr MEM:r:u16=$mem256 ORDER:r:u8=$order                  ', 'MOD=MEM', 'VPSHUFHW_rymyub_1', '         '];
T['VPSHUFHW VREG:w:u16=$YMMr PREG:r:Z=$Kk      VREG:r:u16=$YMMm  ORDER:r:u8=$order', 'MOD=REG', 'VPSHUFHW_rymyub_2', '         '];
T['VPSHUFHW VREG:w:u16=$YMMr PREG:r:Z=$Kk      MEM:r:u16=$mem256 ORDER:r:u8=$order', 'MOD=MEM', 'VPSHUFHW_rymyub_2', 'tuple=FVM'];
T['VPSHUFHW VREG:w:u16=$ZMMr PREG:r:Z=$Kk      VREG:r:u16=$ZMMm  ORDER:r:u8=$order', 'MOD=REG', 'VPSHUFHW_rzmzub  ', '         '];
T['VPSHUFHW VREG:w:u16=$ZMMr PREG:r:Z=$Kk      MEM:r:u16=$mem512 ORDER:r:u8=$order', 'MOD=MEM', 'VPSHUFHW_rzmzub  ', 'tuple=FVM'];

# PSHUFLW-Shuffle Packed Low Words.
T['VPSHUFLW VREG:w:s32=$XMMr VREG:r:s32=$XMMm  ORDER:r:u8=$order                  ', 'MOD=REG', 'VPSHUFLW_romoub_1', '         '];
T['VPSHUFLW VREG:w:s32=$XMMr MEM:r:s32=$mem128 ORDER:r:u8=$order                  ', 'MOD=MEM', 'VPSHUFLW_romoub_1', '         '];
T['VPSHUFLW VREG:w:u16=$XMMr PREG:r:Z=$Kk      VREG:r:u16=$XMMm  ORDER:r:u8=$order', 'MOD=REG', 'VPSHUFLW_romoub_2', '         '];
T['VPSHUFLW VREG:w:u16=$XMMr PREG:r:Z=$Kk      MEM:r:u16=$mem128 ORDER:r:u8=$order', 'MOD=MEM', 'VPSHUFLW_romoub_2', 'tuple=FVM'];
T['VPSHUFLW VREG:w:u16=$YMMr VREG:r:u16=$YMMm  ORDER:r:u8=$order                  ', 'MOD=REG', 'VPSHUFLW_rymyub_1', '         '];
T['VPSHUFLW VREG:w:u16=$YMMr MEM:r:u16=$mem256 ORDER:r:u8=$order                  ', 'MOD=MEM', 'VPSHUFLW_rymyub_1', '         '];
T['VPSHUFLW VREG:w:u16=$YMMr PREG:r:Z=$Kk      VREG:r:u16=$YMMm  ORDER:r:u8=$order', 'MOD=REG', 'VPSHUFLW_rymyub_2', '         '];
T['VPSHUFLW VREG:w:u16=$YMMr PREG:r:Z=$Kk      MEM:r:u16=$mem256 ORDER:r:u8=$order', 'MOD=MEM', 'VPSHUFLW_rymyub_2', 'tuple=FVM'];
T['VPSHUFLW VREG:w:u16=$ZMMr PREG:r:Z=$Kk      VREG:r:u16=$ZMMm  ORDER:r:u8=$order', 'MOD=REG', 'VPSHUFLW_rzmzub  ', '         '];
T['VPSHUFLW VREG:w:u16=$ZMMr PREG:r:Z=$Kk      MEM:r:u16=$mem512 ORDER:r:u8=$order', 'MOD=MEM', 'VPSHUFLW_rzmzub  ', 'tuple=FVM'];

# PSIGNB/PSIGNW/PSIGND-Packed SIGN.
T['VPSIGNB VREG:w:s8=$XMMr  VREG:r:s8=$XMMn  VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPSIGNB_ronomo', ''];
T['VPSIGNB VREG:w:s8=$XMMr  VREG:r:s8=$XMMn  MEM:r:s8=$mem128 ', 'MOD=MEM', 'VPSIGNB_ronomo', ''];
T['VPSIGNB VREG:w:s8=$YMMr  VREG:r:s8=$YMMn  VREG:r:s8=$YMMm  ', 'MOD=REG', 'VPSIGNB_rynymy', ''];
T['VPSIGNB VREG:w:s8=$YMMr  VREG:r:s8=$YMMn  MEM:r:s8=$mem256 ', 'MOD=MEM', 'VPSIGNB_rynymy', ''];
T['VPSIGND VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm ', 'MOD=REG', 'VPSIGND_ronomo', ''];
T['VPSIGND VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128', 'MOD=MEM', 'VPSIGND_ronomo', ''];
T['VPSIGND VREG:w:s32=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm ', 'MOD=REG', 'VPSIGND_rynymy', ''];
T['VPSIGND VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256', 'MOD=MEM', 'VPSIGND_rynymy', ''];
T['VPSIGNW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm ', 'MOD=REG', 'VPSIGNW_ronomo', ''];
T['VPSIGNW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128', 'MOD=MEM', 'VPSIGNW_ronomo', ''];
T['VPSIGNW VREG:w:s16=$YMMr VREG:r:s16=$YMMn VREG:r:s16=$YMMm ', 'MOD=REG', 'VPSIGNW_rynymy', ''];
T['VPSIGNW VREG:w:s16=$YMMr VREG:r:s16=$YMMn MEM:r:s16=$mem256', 'MOD=MEM', 'VPSIGNW_rynymy', ''];

# PSLLDQ-Shift Double Quadword Left Logical.
T['VPSLLDQ REG:w:u128=$XMMn  REG:r:u128=$XMMm  SHIFT:r:u8=$shl', 'NONE   ', 'VPSLLDQ_nomoub_1', '         '];
T['VPSLLDQ VREG:w:u8=$XMMn   VREG:r:u8=$XMMm   SHIFT:r:u8=$shl', 'MOD=REG', 'VPSLLDQ_nomoub_2', '         '];
T['VPSLLDQ VREG:w:u8=$XMMn   MEM:r:u8=$mem128  SHIFT:r:u8=$shl', 'MOD=MEM', 'VPSLLDQ_nomoub_2', 'tuple=FVM'];
T['VPSLLDQ VREG:w:u128=$YMMn VREG:r:u128=$YMMm SHIFT:r:u8=$shl', 'NONE   ', 'VPSLLDQ_nymyub_1', '         '];
T['VPSLLDQ VREG:w:u8=$YMMn   VREG:r:u8=$YMMm   SHIFT:r:u8=$shl', 'MOD=REG', 'VPSLLDQ_nymyub_2', '         '];
T['VPSLLDQ VREG:w:u8=$YMMn   MEM:r:u8=$mem256  SHIFT:r:u8=$shl', 'MOD=MEM', 'VPSLLDQ_nymyub_2', 'tuple=FVM'];
T['VPSLLDQ VREG:w:u8=$ZMMn   VREG:r:u8=$ZMMm   SHIFT:r:u8=$shl', 'MOD=REG', 'VPSLLDQ_nzmzub  ', '         '];
T['VPSLLDQ VREG:w:u8=$ZMMn   MEM:r:u8=$mem512  SHIFT:r:u8=$shl', 'MOD=MEM', 'VPSLLDQ_nzmzub  ', 'tuple=FVM'];

# PSLLW/PSLLD/PSLLQ-Shift Packed Data Left Logical.
T['VPSLLD VREG:w:u32=$XMMn VREG:r:u32=$XMMm SHIFT:r:u8=$shl                               ', 'NONE        ', 'VPSLLD_nomoub_1', '          '];
T['VPSLLD VREG:w:u32=$XMMn PREG:r:Z=$Kk     VREG:r:u32=$XMMm             SHIFT:r:u8=$shl  ', 'BP=0 MOD=REG', 'VPSLLD_nomoub_2', '          '];
T['VPSLLD VREG:w:u32=$XMMn PREG:r:Z=$Kk     MEM:r:u32=$mem128 BCST=$BCST SHIFT:r:u8=$shl  ', 'MOD=MEM     ', 'VPSLLD_nomoub_2', 'tuple=FV  '];
T['VPSLLD VREG:w:u32=$YMMn VREG:r:u32=$YMMm SHIFT:r:u8=$shl                               ', 'NONE        ', 'VPSLLD_nymyub_1', '          '];
T['VPSLLD VREG:w:u32=$YMMn PREG:r:Z=$Kk     VREG:r:u32=$YMMm             SHIFT:r:u8=$shl  ', 'BP=0 MOD=REG', 'VPSLLD_nymyub_2', '          '];
T['VPSLLD VREG:w:u32=$YMMn PREG:r:Z=$Kk     MEM:r:u32=$mem256 BCST=$BCST SHIFT:r:u8=$shl  ', 'MOD=MEM     ', 'VPSLLD_nymyub_2', 'tuple=FV  '];
T['VPSLLD VREG:w:u32=$ZMMn PREG:r:Z=$Kk     VREG:r:u32=$ZMMm             SHIFT:r:u8=$shl  ', 'BP=0 MOD=REG', 'VPSLLD_nzmzub  ', '          '];
T['VPSLLD VREG:w:u32=$ZMMn PREG:r:Z=$Kk     MEM:r:u32=$mem512 BCST=$BCST SHIFT:r:u8=$shl  ', 'MOD=MEM     ', 'VPSLLD_nzmzub  ', 'tuple=FV  '];
T['VPSLLD VREG:w:u32=$XMMr VREG:r:u32=$XMMn VREG:r:u64=$XMMm                              ', 'MOD=REG     ', 'VPSLLD_ronomo_1', '          '];
T['VPSLLD VREG:w:u32=$XMMr VREG:r:u32=$XMMn MEM:r:u64=$mem128                             ', 'MOD=MEM     ', 'VPSLLD_ronomo_1', '          '];
T['VPSLLD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn             VREG:r:u32=$XMMm ', 'MOD=REG     ', 'VPSLLD_ronomo_2', '          '];
T['VPSLLD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn             MEM:r:u32=$mem128', 'MOD=MEM     ', 'VPSLLD_ronomo_2', 'tuple=M128'];
T['VPSLLD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn             VREG:r:u32=$XMMm ', 'MOD=REG     ', 'VPSLLD_rynymo  ', '          '];
T['VPSLLD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn             MEM:r:u32=$mem128', 'MOD=MEM     ', 'VPSLLD_rynymo  ', 'tuple=M128'];
T['VPSLLD VREG:w:u32=$YMMr VREG:r:u32=$YMMn REG:r:u64=$XMMm                               ', 'MOD=REG     ', 'VPSLLD_rynymx  ', '          '];
T['VPSLLD VREG:w:u32=$YMMr VREG:r:u32=$YMMn MEM:r:u64=$mem128                             ', 'MOD=MEM     ', 'VPSLLD_rynymx  ', '          '];
T['VPSLLD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn             VREG:r:u32=$XMMm ', 'MOD=REG     ', 'VPSLLD_rznzmo  ', '          '];
T['VPSLLD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn             MEM:r:u32=$mem128', 'MOD=MEM     ', 'VPSLLD_rznzmo  ', 'tuple=M128'];
T['VPSLLQ VREG:w:u64=$XMMn VREG:r:u64=$XMMm SHIFT:r:u8=$shl                               ', 'NONE        ', 'VPSLLQ_nomoub_1', '          '];
T['VPSLLQ VREG:w:u64=$XMMn PREG:r:Z=$Kk     VREG:r:u64=$XMMm             SHIFT:r:u8=$shl  ', 'BP=0 MOD=REG', 'VPSLLQ_nomoub_2', '          '];
T['VPSLLQ VREG:w:u64=$XMMn PREG:r:Z=$Kk     MEM:r:u64=$mem128 BCST=$BCST SHIFT:r:u8=$shl  ', 'MOD=MEM     ', 'VPSLLQ_nomoub_2', 'tuple=FV  '];
T['VPSLLQ VREG:w:u64=$YMMn VREG:r:u64=$YMMm SHIFT:r:u8=$shl                               ', 'NONE        ', 'VPSLLQ_nymyub_1', '          '];
T['VPSLLQ VREG:w:u64=$YMMn PREG:r:Z=$Kk     VREG:r:u64=$YMMm             SHIFT:r:u8=$shl  ', 'BP=0 MOD=REG', 'VPSLLQ_nymyub_2', '          '];
T['VPSLLQ VREG:w:u64=$YMMn PREG:r:Z=$Kk     MEM:r:u64=$mem256 BCST=$BCST SHIFT:r:u8=$shl  ', 'MOD=MEM     ', 'VPSLLQ_nymyub_2', 'tuple=FV  '];
T['VPSLLQ VREG:w:u64=$ZMMn PREG:r:Z=$Kk     VREG:r:u64=$ZMMm             SHIFT:r:u8=$shl  ', 'BP=0 MOD=REG', 'VPSLLQ_nzmzub  ', '          '];
T['VPSLLQ VREG:w:u64=$ZMMn PREG:r:Z=$Kk     MEM:r:u64=$mem512 BCST=$BCST SHIFT:r:u8=$shl  ', 'MOD=MEM     ', 'VPSLLQ_nzmzub  ', 'tuple=FV  '];
T['VPSLLQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn VREG:r:u64=$XMMm                              ', 'MOD=REG     ', 'VPSLLQ_ronomo_1', '          '];
T['VPSLLQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn MEM:r:u64=$mem128                             ', 'MOD=MEM     ', 'VPSLLQ_ronomo_1', '          '];
T['VPSLLQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn             VREG:r:u64=$XMMm ', 'MOD=REG     ', 'VPSLLQ_ronomo_2', '          '];
T['VPSLLQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn             MEM:r:u64=$mem128', 'MOD=MEM     ', 'VPSLLQ_ronomo_2', 'tuple=M128'];
T['VPSLLQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn             VREG:r:u64=$XMMm ', 'MOD=REG     ', 'VPSLLQ_rynymo  ', '          '];
T['VPSLLQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn             MEM:r:u64=$mem128', 'MOD=MEM     ', 'VPSLLQ_rynymo  ', 'tuple=M128'];
T['VPSLLQ VREG:w:u64=$YMMr VREG:r:u64=$YMMn REG:r:u64=$XMMm                               ', 'MOD=REG     ', 'VPSLLQ_rynymx  ', '          '];
T['VPSLLQ VREG:w:u64=$YMMr VREG:r:u64=$YMMn MEM:r:u64=$mem128                             ', 'MOD=MEM     ', 'VPSLLQ_rynymx  ', '          '];
T['VPSLLQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn             VREG:r:u64=$XMMm ', 'MOD=REG     ', 'VPSLLQ_rznzmo  ', '          '];
T['VPSLLQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn             MEM:r:u64=$mem128', 'MOD=MEM     ', 'VPSLLQ_rznzmo  ', 'tuple=M128'];
T['VPSLLW VREG:w:u16=$XMMn VREG:r:u16=$XMMm SHIFT:r:u8=$shl                               ', 'NONE        ', 'VPSLLW_nomoub_1', '          '];
T['VPSLLW VREG:w:u16=$XMMn PREG:r:Z=$Kk     VREG:r:u16=$XMMm             SHIFT:r:u8=$shl  ', 'MOD=REG     ', 'VPSLLW_nomoub_2', '          '];
T['VPSLLW VREG:w:u16=$XMMn PREG:r:Z=$Kk     MEM:r:u16=$mem128            SHIFT:r:u8=$shl  ', 'MOD=MEM     ', 'VPSLLW_nomoub_2', 'tuple=FVM '];
T['VPSLLW VREG:w:u16=$YMMn VREG:r:u16=$YMMm SHIFT:r:u8=$shl                               ', 'NONE        ', 'VPSLLW_nymyub_1', '          '];
T['VPSLLW VREG:w:u16=$YMMn PREG:r:Z=$Kk     VREG:r:u16=$YMMm             SHIFT:r:u8=$shl  ', 'MOD=REG     ', 'VPSLLW_nymyub_2', '          '];
T['VPSLLW VREG:w:u16=$YMMn PREG:r:Z=$Kk     MEM:r:u16=$mem256            SHIFT:r:u8=$shl  ', 'MOD=MEM     ', 'VPSLLW_nymyub_2', 'tuple=FVM '];
T['VPSLLW VREG:w:u16=$ZMMn PREG:r:Z=$Kk     VREG:r:u16=$ZMMm             SHIFT:r:u8=$shl  ', 'MOD=REG     ', 'VPSLLW_nzmzub  ', '          '];
T['VPSLLW VREG:w:u16=$ZMMn PREG:r:Z=$Kk     MEM:r:u16=$mem512            SHIFT:r:u8=$shl  ', 'MOD=MEM     ', 'VPSLLW_nzmzub  ', 'tuple=FVM '];
T['VPSLLW VREG:w:u16=$XMMr VREG:r:u16=$XMMn VREG:r:u64=$XMMm                              ', 'MOD=REG     ', 'VPSLLW_ronomo_1', '          '];
T['VPSLLW VREG:w:u16=$XMMr VREG:r:u16=$XMMn MEM:r:u64=$mem128                             ', 'MOD=MEM     ', 'VPSLLW_ronomo_1', '          '];
T['VPSLLW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn             VREG:r:u16=$XMMm ', 'MOD=REG     ', 'VPSLLW_ronomo_2', '          '];
T['VPSLLW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn             MEM:r:u16=$mem128', 'MOD=MEM     ', 'VPSLLW_ronomo_2', 'tuple=M128'];
T['VPSLLW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn             VREG:r:u16=$XMMm ', 'MOD=REG     ', 'VPSLLW_rynymo  ', '          '];
T['VPSLLW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn             MEM:r:u16=$mem128', 'MOD=MEM     ', 'VPSLLW_rynymo  ', 'tuple=M128'];
T['VPSLLW VREG:w:u16=$YMMr VREG:r:u16=$YMMn REG:r:u64=$XMMm                               ', 'MOD=REG     ', 'VPSLLW_rynymx  ', '          '];
T['VPSLLW VREG:w:u16=$YMMr VREG:r:u16=$YMMn MEM:r:u64=$mem128                             ', 'MOD=MEM     ', 'VPSLLW_rynymx  ', '          '];
T['VPSLLW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn             VREG:r:u16=$XMMm ', 'MOD=REG     ', 'VPSLLW_rznzmo  ', '          '];
T['VPSLLW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn             MEM:r:u16=$mem128', 'MOD=MEM     ', 'VPSLLW_rznzmo  ', 'tuple=M128'];

# PSRAW/PSRAD/PSRAQ-Shift Packed Data Right Arithmetic.
T['VPSRAD VREG:w:s32=$XMMn VREG:r:s32=$XMMm SHIFT:r:u8=$shr                               ', 'NONE            ', 'VPSRAD_nomoub  ', '          '];
T['VPSRAD VREG:w:s32=$YMMn VREG:r:s32=$YMMm SHIFT:r:u8=$shr                               ', 'NONE            ', 'VPSRAD_nymyub  ', '          '];
T['VPSRAD VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:u64=$XMMm                              ', 'MOD=REG         ', 'VPSRAD_ronomo  ', '          '];
T['VPSRAD VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:u64=$mem128                             ', 'MOD=MEM         ', 'VPSRAD_ronomo  ', '          '];
T['VPSRAD VREG:w:s32=$YMMr VREG:r:s32=$YMMn REG:r:u64=$XMMm                               ', 'MOD=REG         ', 'VPSRAD_rynymx  ', '          '];
T['VPSRAD VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:u64=$mem128                             ', 'MOD=MEM         ', 'VPSRAD_rynymx  ', '          '];
T['VPSRAW VREG:w:s16=$XMMn VREG:r:s16=$XMMm SHIFT:r:u8=$shr                               ', 'NONE            ', 'VPSRAW_nomoub_1', '          '];
T['VPSRAW VREG:w:u16=$XMMn PREG:r:Z=$Kk     VREG:r:u16=$XMMm             SHIFT:r:u8=$shr  ', 'MOD=REG         ', 'VPSRAW_nomoub_2', '          '];
T['VPSRAW VREG:w:u16=$XMMn PREG:r:Z=$Kk     MEM:r:u16=$mem128            SHIFT:r:u8=$shr  ', 'MOD=MEM         ', 'VPSRAW_nomoub_2', 'tuple=FVM '];
T['VPSRAW VREG:w:s16=$YMMn VREG:r:s16=$YMMm SHIFT:r:u8=$shr                               ', 'NONE            ', 'VPSRAW_nymyub_1', '          '];
T['VPSRAW VREG:w:u16=$YMMn PREG:r:Z=$Kk     VREG:r:u16=$YMMm             SHIFT:r:u8=$shr  ', 'MOD=REG         ', 'VPSRAW_nymyub_2', '          '];
T['VPSRAW VREG:w:u16=$YMMn PREG:r:Z=$Kk     MEM:r:u16=$mem256            SHIFT:r:u8=$shr  ', 'MOD=MEM         ', 'VPSRAW_nymyub_2', 'tuple=FVM '];
T['VPSRAW VREG:w:u16=$ZMMn PREG:r:Z=$Kk     VREG:r:u16=$ZMMm             SHIFT:r:u8=$shr  ', 'MOD=REG         ', 'VPSRAW_nzmzub  ', '          '];
T['VPSRAW VREG:w:u16=$ZMMn PREG:r:Z=$Kk     MEM:r:u16=$mem512            SHIFT:r:u8=$shr  ', 'MOD=MEM         ', 'VPSRAW_nzmzub  ', 'tuple=FVM '];
T['VPSRAW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:u64=$XMMm                              ', 'MOD=REG         ', 'VPSRAW_ronomo_1', '          '];
T['VPSRAW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:u64=$mem128                             ', 'MOD=MEM         ', 'VPSRAW_ronomo_1', '          '];
T['VPSRAW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn             VREG:r:u16=$XMMm ', 'MOD=REG         ', 'VPSRAW_ronomo_2', '          '];
T['VPSRAW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn             MEM:r:u16=$mem128', 'MOD=MEM         ', 'VPSRAW_ronomo_2', 'tuple=M128'];
T['VPSRAW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn             VREG:r:u16=$XMMm ', 'MOD=REG         ', 'VPSRAW_rynymo  ', '          '];
T['VPSRAW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn             MEM:r:u16=$mem128', 'MOD=MEM         ', 'VPSRAW_rynymo  ', 'tuple=M128'];
T['VPSRAW VREG:w:s16=$YMMr VREG:r:s16=$YMMn REG:r:u64=$XMMm                               ', 'MOD=REG         ', 'VPSRAW_rynymx  ', '          '];
T['VPSRAW VREG:w:s16=$YMMr VREG:r:s16=$YMMn MEM:r:u64=$mem128                             ', 'MOD=MEM         ', 'VPSRAW_rynymx  ', '          '];
T['VPSRAW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn             VREG:r:u16=$XMMm ', 'MOD=REG         ', 'VPSRAW_rznzmo  ', '          '];
T['VPSRAW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn             MEM:r:u16=$mem128', 'MOD=MEM         ', 'VPSRAW_rznzmo  ', 'tuple=M128'];
T['VPSRAD VREG:w:u32=$XMMn PREG:r:Z=$Kk     VREG:r:u32=$XMMm             SHIFT:r:u8=$shr  ', 'BP=0 MOD=REG W=0', 'VPSRAx_nomoub  ', '          '];
T['VPSRAD VREG:w:u32=$XMMn PREG:r:Z=$Kk     MEM:r:u32=$mem128 BCST=$BCST SHIFT:r:u8=$shr  ', 'MOD=MEM W=0     ', 'VPSRAx_nomoub  ', 'tuple=FV  '];
T['VPSRAQ VREG:w:u64=$XMMn PREG:r:Z=$Kk     VREG:r:u64=$XMMm             SHIFT:r:u8=$shr  ', 'BP=0 MOD=REG W=1', 'VPSRAx_nomoub  ', '          '];
T['VPSRAQ VREG:w:u64=$XMMn PREG:r:Z=$Kk     MEM:r:u64=$mem128 BCST=$BCST SHIFT:r:u8=$shr  ', 'MOD=MEM W=1     ', 'VPSRAx_nomoub  ', 'tuple=FV  '];
T['VPSRAD VREG:w:u32=$YMMn PREG:r:Z=$Kk     VREG:r:u32=$YMMm             SHIFT:r:u8=$shr  ', 'BP=0 MOD=REG W=0', 'VPSRAx_nymyub  ', '          '];
T['VPSRAD VREG:w:u32=$YMMn PREG:r:Z=$Kk     MEM:r:u32=$mem256 BCST=$BCST SHIFT:r:u8=$shr  ', 'MOD=MEM W=0     ', 'VPSRAx_nymyub  ', 'tuple=FV  '];
T['VPSRAQ VREG:w:u64=$YMMn PREG:r:Z=$Kk     VREG:r:u64=$YMMm             SHIFT:r:u8=$shr  ', 'BP=0 MOD=REG W=1', 'VPSRAx_nymyub  ', '          '];
T['VPSRAQ VREG:w:u64=$YMMn PREG:r:Z=$Kk     MEM:r:u64=$mem256 BCST=$BCST SHIFT:r:u8=$shr  ', 'MOD=MEM W=1     ', 'VPSRAx_nymyub  ', 'tuple=FV  '];
T['VPSRAD VREG:w:u32=$ZMMn PREG:r:Z=$Kk     VREG:r:u32=$ZMMm             SHIFT:r:u8=$shr  ', 'BP=0 MOD=REG W=0', 'VPSRAx_nzmzub  ', '          '];
T['VPSRAD VREG:w:u32=$ZMMn PREG:r:Z=$Kk     MEM:r:u32=$mem512 BCST=$BCST SHIFT:r:u8=$shr  ', 'MOD=MEM W=0     ', 'VPSRAx_nzmzub  ', 'tuple=FV  '];
T['VPSRAQ VREG:w:u64=$ZMMn PREG:r:Z=$Kk     VREG:r:u64=$ZMMm             SHIFT:r:u8=$shr  ', 'BP=0 MOD=REG W=1', 'VPSRAx_nzmzub  ', '          '];
T['VPSRAQ VREG:w:u64=$ZMMn PREG:r:Z=$Kk     MEM:r:u64=$mem512 BCST=$BCST SHIFT:r:u8=$shr  ', 'MOD=MEM W=1     ', 'VPSRAx_nzmzub  ', 'tuple=FV  '];
T['VPSRAD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn             VREG:r:u32=$XMMm ', 'MOD=REG W=0     ', 'VPSRAx_ronomo  ', '          '];
T['VPSRAD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn             MEM:r:u32=$mem128', 'MOD=MEM W=0     ', 'VPSRAx_ronomo  ', 'tuple=M128'];
T['VPSRAQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn             VREG:r:u64=$XMMm ', 'MOD=REG W=1     ', 'VPSRAx_ronomo  ', '          '];
T['VPSRAQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn             MEM:r:u64=$mem128', 'MOD=MEM W=1     ', 'VPSRAx_ronomo  ', 'tuple=M128'];
T['VPSRAD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn             VREG:r:u32=$XMMm ', 'MOD=REG W=0     ', 'VPSRAx_rynymo  ', '          '];
T['VPSRAD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn             MEM:r:u32=$mem128', 'MOD=MEM W=0     ', 'VPSRAx_rynymo  ', 'tuple=M128'];
T['VPSRAQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn             VREG:r:u64=$XMMm ', 'MOD=REG W=1     ', 'VPSRAx_rynymo  ', '          '];
T['VPSRAQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn             MEM:r:u64=$mem128', 'MOD=MEM W=1     ', 'VPSRAx_rynymo  ', 'tuple=M128'];
T['VPSRAD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn             VREG:r:u32=$XMMm ', 'MOD=REG W=0     ', 'VPSRAx_rznzmo  ', '          '];
T['VPSRAD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn             MEM:r:u32=$mem128', 'MOD=MEM W=0     ', 'VPSRAx_rznzmo  ', 'tuple=M128'];
T['VPSRAQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn             VREG:r:u64=$XMMm ', 'MOD=REG W=1     ', 'VPSRAx_rznzmo  ', '          '];
T['VPSRAQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn             MEM:r:u64=$mem128', 'MOD=MEM W=1     ', 'VPSRAx_rznzmo  ', 'tuple=M128'];

# PSRLDQ-Shift Double Quadword Right Logical.
T['VPSRLDQ REG:w:u128=$XMMn  REG:r:u128=$XMMm  SHIFT:r:u8=$shr', 'NONE   ', 'VPSRLDQ_nomoub_1', '         '];
T['VPSRLDQ VREG:w:u8=$XMMn   VREG:r:u8=$XMMm   SHIFT:r:u8=$shr', 'MOD=REG', 'VPSRLDQ_nomoub_2', '         '];
T['VPSRLDQ VREG:w:u8=$XMMn   MEM:r:u8=$mem128  SHIFT:r:u8=$shr', 'MOD=MEM', 'VPSRLDQ_nomoub_2', 'tuple=FVM'];
T['VPSRLDQ VREG:w:u128=$YMMn VREG:r:u128=$YMMm SHIFT:r:u8=$shr', 'NONE   ', 'VPSRLDQ_nymyub_1', '         '];
T['VPSRLDQ VREG:w:u8=$YMMn   VREG:r:u8=$YMMm   SHIFT:r:u8=$shr', 'MOD=REG', 'VPSRLDQ_nymyub_2', '         '];
T['VPSRLDQ VREG:w:u8=$YMMn   MEM:r:u8=$mem256  SHIFT:r:u8=$shr', 'MOD=MEM', 'VPSRLDQ_nymyub_2', 'tuple=FVM'];
T['VPSRLDQ VREG:w:u8=$ZMMn   VREG:r:u8=$ZMMm   SHIFT:r:u8=$shr', 'MOD=REG', 'VPSRLDQ_nzmzub  ', '         '];
T['VPSRLDQ VREG:w:u8=$ZMMn   MEM:r:u8=$mem512  SHIFT:r:u8=$shr', 'MOD=MEM', 'VPSRLDQ_nzmzub  ', 'tuple=FVM'];

# PSRLW/PSRLD/PSRLQ-Shift Packed Data Right Logical.
T['VPSRLD VREG:w:u32=$XMMn VREG:r:u32=$XMMm SHIFT:r:u8=$shr                               ', 'NONE        ', 'VPSRLD_nomoub_1', '          '];
T['VPSRLD VREG:w:u32=$XMMn PREG:r:Z=$Kk     VREG:r:u32=$XMMm             SHIFT:r:u8=$shr  ', 'BP=0 MOD=REG', 'VPSRLD_nomoub_2', '          '];
T['VPSRLD VREG:w:u32=$XMMn PREG:r:Z=$Kk     MEM:r:u32=$mem128 BCST=$BCST SHIFT:r:u8=$shr  ', 'MOD=MEM     ', 'VPSRLD_nomoub_2', 'tuple=FV  '];
T['VPSRLD VREG:w:u32=$YMMn VREG:r:u32=$YMMm SHIFT:r:u8=$shr                               ', 'NONE        ', 'VPSRLD_nymyub_1', '          '];
T['VPSRLD VREG:w:u32=$YMMn PREG:r:Z=$Kk     VREG:r:u32=$YMMm             SHIFT:r:u8=$shr  ', 'BP=0 MOD=REG', 'VPSRLD_nymyub_2', '          '];
T['VPSRLD VREG:w:u32=$YMMn PREG:r:Z=$Kk     MEM:r:u32=$mem256 BCST=$BCST SHIFT:r:u8=$shr  ', 'MOD=MEM     ', 'VPSRLD_nymyub_2', 'tuple=FV  '];
T['VPSRLD VREG:w:u32=$ZMMn PREG:r:Z=$Kk     VREG:r:u32=$ZMMm             SHIFT:r:u8=$shr  ', 'BP=0 MOD=REG', 'VPSRLD_nzmzub  ', '          '];
T['VPSRLD VREG:w:u32=$ZMMn PREG:r:Z=$Kk     MEM:r:u32=$mem512 BCST=$BCST SHIFT:r:u8=$shr  ', 'MOD=MEM     ', 'VPSRLD_nzmzub  ', 'tuple=FV  '];
T['VPSRLD VREG:w:u32=$XMMr VREG:r:u32=$XMMn VREG:r:u64=$XMMm                              ', 'MOD=REG     ', 'VPSRLD_ronomo_1', '          '];
T['VPSRLD VREG:w:u32=$XMMr VREG:r:u32=$XMMn MEM:r:u64=$mem128                             ', 'MOD=MEM     ', 'VPSRLD_ronomo_1', '          '];
T['VPSRLD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn             VREG:r:u32=$XMMm ', 'MOD=REG     ', 'VPSRLD_ronomo_2', '          '];
T['VPSRLD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn             MEM:r:u32=$mem128', 'MOD=MEM     ', 'VPSRLD_ronomo_2', 'tuple=M128'];
T['VPSRLD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn             VREG:r:u32=$XMMm ', 'MOD=REG     ', 'VPSRLD_rynymo  ', '          '];
T['VPSRLD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn             MEM:r:u32=$mem128', 'MOD=MEM     ', 'VPSRLD_rynymo  ', 'tuple=M128'];
T['VPSRLD VREG:w:u32=$YMMr VREG:r:u32=$YMMn REG:r:u64=$XMMm                               ', 'MOD=REG     ', 'VPSRLD_rynymx  ', '          '];
T['VPSRLD VREG:w:u32=$YMMr VREG:r:u32=$YMMn MEM:r:u64=$mem128                             ', 'MOD=MEM     ', 'VPSRLD_rynymx  ', '          '];
T['VPSRLD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn             VREG:r:u32=$XMMm ', 'MOD=REG     ', 'VPSRLD_rznzmo  ', '          '];
T['VPSRLD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn             MEM:r:u32=$mem128', 'MOD=MEM     ', 'VPSRLD_rznzmo  ', 'tuple=M128'];
T['VPSRLQ VREG:w:u64=$XMMn VREG:r:u64=$XMMm SHIFT:r:u8=$shr                               ', 'NONE        ', 'VPSRLQ_nomoub_1', '          '];
T['VPSRLQ VREG:w:u64=$XMMn PREG:r:Z=$Kk     VREG:r:u64=$XMMm             SHIFT:r:u8=$shr  ', 'BP=0 MOD=REG', 'VPSRLQ_nomoub_2', '          '];
T['VPSRLQ VREG:w:u64=$XMMn PREG:r:Z=$Kk     MEM:r:u64=$mem128 BCST=$BCST SHIFT:r:u8=$shr  ', 'MOD=MEM     ', 'VPSRLQ_nomoub_2', 'tuple=FV  '];
T['VPSRLQ VREG:w:u64=$YMMn VREG:r:u64=$YMMm SHIFT:r:u8=$shr                               ', 'NONE        ', 'VPSRLQ_nymyub_1', '          '];
T['VPSRLQ VREG:w:u64=$YMMn PREG:r:Z=$Kk     VREG:r:u64=$YMMm             SHIFT:r:u8=$shr  ', 'BP=0 MOD=REG', 'VPSRLQ_nymyub_2', '          '];
T['VPSRLQ VREG:w:u64=$YMMn PREG:r:Z=$Kk     MEM:r:u64=$mem256 BCST=$BCST SHIFT:r:u8=$shr  ', 'MOD=MEM     ', 'VPSRLQ_nymyub_2', 'tuple=FV  '];
T['VPSRLQ VREG:w:u64=$ZMMn PREG:r:Z=$Kk     VREG:r:u64=$ZMMm             SHIFT:r:u8=$shr  ', 'BP=0 MOD=REG', 'VPSRLQ_nzmzub  ', '          '];
T['VPSRLQ VREG:w:u64=$ZMMn PREG:r:Z=$Kk     MEM:r:u64=$mem512 BCST=$BCST SHIFT:r:u8=$shr  ', 'MOD=MEM     ', 'VPSRLQ_nzmzub  ', 'tuple=FV  '];
T['VPSRLQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn VREG:r:u64=$XMMm                              ', 'MOD=REG     ', 'VPSRLQ_ronomo_1', '          '];
T['VPSRLQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn MEM:r:u64=$mem128                             ', 'MOD=MEM     ', 'VPSRLQ_ronomo_1', '          '];
T['VPSRLQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn             VREG:r:u64=$XMMm ', 'MOD=REG     ', 'VPSRLQ_ronomo_2', '          '];
T['VPSRLQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn             MEM:r:u64=$mem128', 'MOD=MEM     ', 'VPSRLQ_ronomo_2', 'tuple=M128'];
T['VPSRLQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn             VREG:r:u64=$XMMm ', 'MOD=REG     ', 'VPSRLQ_rynymo  ', '          '];
T['VPSRLQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn             MEM:r:u64=$mem128', 'MOD=MEM     ', 'VPSRLQ_rynymo  ', 'tuple=M128'];
T['VPSRLQ VREG:w:u64=$YMMr VREG:r:u64=$YMMn REG:r:u64=$XMMm                               ', 'MOD=REG     ', 'VPSRLQ_rynymx  ', '          '];
T['VPSRLQ VREG:w:u64=$YMMr VREG:r:u64=$YMMn MEM:r:u64=$mem128                             ', 'MOD=MEM     ', 'VPSRLQ_rynymx  ', '          '];
T['VPSRLQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn             VREG:r:u64=$XMMm ', 'MOD=REG     ', 'VPSRLQ_rznzmo  ', '          '];
T['VPSRLQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn             MEM:r:u64=$mem128', 'MOD=MEM     ', 'VPSRLQ_rznzmo  ', 'tuple=M128'];
T['VPSRLW VREG:w:u16=$XMMn VREG:r:u16=$XMMm SHIFT:r:u8=$shr                               ', 'NONE        ', 'VPSRLW_nomoub_1', '          '];
T['VPSRLW VREG:w:u16=$XMMn PREG:r:Z=$Kk     VREG:r:u16=$XMMm             SHIFT:r:u8=$shr  ', 'MOD=REG     ', 'VPSRLW_nomoub_2', '          '];
T['VPSRLW VREG:w:u16=$XMMn PREG:r:Z=$Kk     MEM:r:u16=$mem128            SHIFT:r:u8=$shr  ', 'MOD=MEM     ', 'VPSRLW_nomoub_2', 'tuple=FVM '];
T['VPSRLW VREG:w:u16=$YMMn VREG:r:u16=$YMMm SHIFT:r:u8=$shr                               ', 'NONE        ', 'VPSRLW_nymyub_1', '          '];
T['VPSRLW VREG:w:u16=$YMMn PREG:r:Z=$Kk     VREG:r:u16=$YMMm             SHIFT:r:u8=$shr  ', 'MOD=REG     ', 'VPSRLW_nymyub_2', '          '];
T['VPSRLW VREG:w:u16=$YMMn PREG:r:Z=$Kk     MEM:r:u16=$mem256            SHIFT:r:u8=$shr  ', 'MOD=MEM     ', 'VPSRLW_nymyub_2', 'tuple=FVM '];
T['VPSRLW VREG:w:u16=$ZMMn PREG:r:Z=$Kk     VREG:r:u16=$ZMMm             SHIFT:r:u8=$shr  ', 'MOD=REG     ', 'VPSRLW_nzmzub  ', '          '];
T['VPSRLW VREG:w:u16=$ZMMn PREG:r:Z=$Kk     MEM:r:u16=$mem512            SHIFT:r:u8=$shr  ', 'MOD=MEM     ', 'VPSRLW_nzmzub  ', 'tuple=FVM '];
T['VPSRLW VREG:w:u16=$XMMr VREG:r:u16=$XMMn VREG:r:u64=$XMMm                              ', 'MOD=REG     ', 'VPSRLW_ronomo_1', '          '];
T['VPSRLW VREG:w:u16=$XMMr VREG:r:u16=$XMMn MEM:r:u64=$mem128                             ', 'MOD=MEM     ', 'VPSRLW_ronomo_1', '          '];
T['VPSRLW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn             VREG:r:u16=$XMMm ', 'MOD=REG     ', 'VPSRLW_ronomo_2', '          '];
T['VPSRLW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn             MEM:r:u16=$mem128', 'MOD=MEM     ', 'VPSRLW_ronomo_2', 'tuple=M128'];
T['VPSRLW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn             VREG:r:u16=$XMMm ', 'MOD=REG     ', 'VPSRLW_rynymo  ', '          '];
T['VPSRLW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn             MEM:r:u16=$mem128', 'MOD=MEM     ', 'VPSRLW_rynymo  ', 'tuple=M128'];
T['VPSRLW VREG:w:u16=$YMMr VREG:r:u16=$YMMn REG:r:u64=$XMMm                               ', 'MOD=REG     ', 'VPSRLW_rynymx  ', '          '];
T['VPSRLW VREG:w:u16=$YMMr VREG:r:u16=$YMMn MEM:r:u64=$mem128                             ', 'MOD=MEM     ', 'VPSRLW_rynymx  ', '          '];
T['VPSRLW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn             VREG:r:u16=$XMMm ', 'MOD=REG     ', 'VPSRLW_rznzmo  ', '          '];
T['VPSRLW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn             MEM:r:u16=$mem128', 'MOD=MEM     ', 'VPSRLW_rznzmo  ', 'tuple=M128'];

# PSUBQ-Subtract Packed Quadword Integers.
T['VPSUBQ VREG:w:s64=$XMMr VREG:r:s64=$XMMn VREG:r:s64=$XMMm                              ', 'MOD=REG     ', 'VPSUBQ_ronomo_1', '        '];
T['VPSUBQ VREG:w:s64=$XMMr VREG:r:s64=$XMMn MEM:r:s64=$mem128                             ', 'MOD=MEM     ', 'VPSUBQ_ronomo_1', '        '];
T['VPSUBQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG', 'VPSUBQ_ronomo_2', '        '];
T['VPSUBQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPSUBQ_ronomo_2', 'tuple=FV'];
T['VPSUBQ VREG:w:s64=$YMMr VREG:r:s64=$YMMn VREG:r:s64=$YMMm                              ', 'MOD=REG     ', 'VPSUBQ_rynymy_1', '        '];
T['VPSUBQ VREG:w:s64=$YMMr VREG:r:s64=$YMMn MEM:r:s64=$mem256                             ', 'MOD=MEM     ', 'VPSUBQ_rynymy_1', '        '];
T['VPSUBQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG', 'VPSUBQ_rynymy_2', '        '];
T['VPSUBQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPSUBQ_rynymy_2', 'tuple=FV'];
T['VPSUBQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG', 'VPSUBQ_rznzmz  ', '        '];
T['VPSUBQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPSUBQ_rznzmz  ', 'tuple=FV'];

# PSUBSB/PSUBSW-Subtract Packed Signed Integers with Signed Saturation.
T['VPSUBSB VREG:w:s8=$XMMr  VREG:r:s8=$XMMn  VREG:r:s8=$XMMm                    ', 'MOD=REG', 'VPSUBSB_ronomo_1', '         '];
T['VPSUBSB VREG:w:s8=$XMMr  VREG:r:s8=$XMMn  MEM:r:s8=$mem128                   ', 'MOD=MEM', 'VPSUBSB_ronomo_1', '         '];
T['VPSUBSB VREG:w:s8=$XMMr  PREG:r:Z=$Kk     VREG:r:s8=$XMMn   VREG:r:s8=$XMMm  ', 'MOD=REG', 'VPSUBSB_ronomo_2', '         '];
T['VPSUBSB VREG:w:s8=$XMMr  PREG:r:Z=$Kk     VREG:r:s8=$XMMn   MEM:r:s8=$mem128 ', 'MOD=MEM', 'VPSUBSB_ronomo_2', 'tuple=FVM'];
T['VPSUBSB VREG:w:s8=$YMMr  VREG:r:s8=$YMMn  VREG:r:s8=$YMMm                    ', 'MOD=REG', 'VPSUBSB_rynymy_1', '         '];
T['VPSUBSB VREG:w:s8=$YMMr  VREG:r:s8=$YMMn  MEM:r:s8=$mem256                   ', 'MOD=MEM', 'VPSUBSB_rynymy_1', '         '];
T['VPSUBSB VREG:w:s8=$YMMr  PREG:r:Z=$Kk     VREG:r:s8=$YMMn   VREG:r:s8=$YMMm  ', 'MOD=REG', 'VPSUBSB_rynymy_2', '         '];
T['VPSUBSB VREG:w:s8=$YMMr  PREG:r:Z=$Kk     VREG:r:s8=$YMMn   MEM:r:s8=$mem256 ', 'MOD=MEM', 'VPSUBSB_rynymy_2', 'tuple=FVM'];
T['VPSUBSB VREG:w:s8=$ZMMr  PREG:r:Z=$Kk     VREG:r:s8=$ZMMn   VREG:r:s8=$ZMMm  ', 'MOD=REG', 'VPSUBSB_rznzmz  ', '         '];
T['VPSUBSB VREG:w:s8=$ZMMr  PREG:r:Z=$Kk     VREG:r:s8=$ZMMn   MEM:r:s8=$mem512 ', 'MOD=MEM', 'VPSUBSB_rznzmz  ', 'tuple=FVM'];
T['VPSUBSW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm                   ', 'MOD=REG', 'VPSUBSW_ronomo_1', '         '];
T['VPSUBSW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128                  ', 'MOD=MEM', 'VPSUBSW_ronomo_1', '         '];
T['VPSUBSW VREG:w:s16=$XMMr PREG:r:Z=$Kk     VREG:r:s16=$XMMn  VREG:r:s16=$XMMm ', 'MOD=REG', 'VPSUBSW_ronomo_2', '         '];
T['VPSUBSW VREG:w:s16=$XMMr PREG:r:Z=$Kk     VREG:r:s16=$XMMn  MEM:r:s16=$mem128', 'MOD=MEM', 'VPSUBSW_ronomo_2', 'tuple=FVM'];
T['VPSUBSW VREG:w:s16=$YMMr VREG:r:s16=$YMMn VREG:r:s16=$YMMm                   ', 'MOD=REG', 'VPSUBSW_rynymy_1', '         '];
T['VPSUBSW VREG:w:s16=$YMMr VREG:r:s16=$YMMn MEM:r:s16=$mem256                  ', 'MOD=MEM', 'VPSUBSW_rynymy_1', '         '];
T['VPSUBSW VREG:w:s16=$YMMr PREG:r:Z=$Kk     VREG:r:s16=$YMMn  VREG:r:s16=$YMMm ', 'MOD=REG', 'VPSUBSW_rynymy_2', '         '];
T['VPSUBSW VREG:w:s16=$YMMr PREG:r:Z=$Kk     VREG:r:s16=$YMMn  MEM:r:s16=$mem256', 'MOD=MEM', 'VPSUBSW_rynymy_2', 'tuple=FVM'];
T['VPSUBSW VREG:w:s16=$ZMMr PREG:r:Z=$Kk     VREG:r:s16=$ZMMn  VREG:r:s16=$ZMMm ', 'MOD=REG', 'VPSUBSW_rznzmz  ', '         '];
T['VPSUBSW VREG:w:s16=$ZMMr PREG:r:Z=$Kk     VREG:r:s16=$ZMMn  MEM:r:s16=$mem512', 'MOD=MEM', 'VPSUBSW_rznzmz  ', 'tuple=FVM'];

# PSUBUSB/PSUBUSW-Subtract Packed Unsigned Integers with Unsigned Saturation.
T['VPSUBUSB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  VREG:r:u8=$XMMm                    ', 'MOD=REG', 'VPSUBUSB_ronomo_1', '         '];
T['VPSUBUSB VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  MEM:r:u8=$mem128                   ', 'MOD=MEM', 'VPSUBUSB_ronomo_1', '         '];
T['VPSUBUSB VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   VREG:r:u8=$XMMm  ', 'MOD=REG', 'VPSUBUSB_ronomo_2', '         '];
T['VPSUBUSB VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   MEM:r:u8=$mem128 ', 'MOD=MEM', 'VPSUBUSB_ronomo_2', 'tuple=FVM'];
T['VPSUBUSB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  VREG:r:u8=$YMMm                    ', 'MOD=REG', 'VPSUBUSB_rynymy_1', '         '];
T['VPSUBUSB VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  MEM:r:u8=$mem256                   ', 'MOD=MEM', 'VPSUBUSB_rynymy_1', '         '];
T['VPSUBUSB VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   VREG:r:u8=$YMMm  ', 'MOD=REG', 'VPSUBUSB_rynymy_2', '         '];
T['VPSUBUSB VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   MEM:r:u8=$mem256 ', 'MOD=MEM', 'VPSUBUSB_rynymy_2', 'tuple=FVM'];
T['VPSUBUSB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   VREG:r:u8=$ZMMm  ', 'MOD=REG', 'VPSUBUSB_rznzmz  ', '         '];
T['VPSUBUSB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   MEM:r:u8=$mem512 ', 'MOD=MEM', 'VPSUBUSB_rznzmz  ', 'tuple=FVM'];
T['VPSUBUSW VREG:w:u16=$XMMr VREG:r:u16=$XMMn VREG:r:u16=$XMMm                   ', 'MOD=REG', 'VPSUBUSW_ronomo_1', '         '];
T['VPSUBUSW VREG:w:u16=$XMMr VREG:r:u16=$XMMn MEM:r:u16=$mem128                  ', 'MOD=MEM', 'VPSUBUSW_ronomo_1', '         '];
T['VPSUBUSW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm ', 'MOD=REG', 'VPSUBUSW_ronomo_2', '         '];
T['VPSUBUSW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128', 'MOD=MEM', 'VPSUBUSW_ronomo_2', 'tuple=FVM'];
T['VPSUBUSW VREG:w:u16=$YMMr VREG:r:u16=$YMMn VREG:r:u16=$YMMm                   ', 'MOD=REG', 'VPSUBUSW_rynymy_1', '         '];
T['VPSUBUSW VREG:w:u16=$YMMr VREG:r:u16=$YMMn MEM:r:u16=$mem256                  ', 'MOD=MEM', 'VPSUBUSW_rynymy_1', '         '];
T['VPSUBUSW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm ', 'MOD=REG', 'VPSUBUSW_rynymy_2', '         '];
T['VPSUBUSW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256', 'MOD=MEM', 'VPSUBUSW_rynymy_2', 'tuple=FVM'];
T['VPSUBUSW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm ', 'MOD=REG', 'VPSUBUSW_rznzmz  ', '         '];
T['VPSUBUSW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512', 'MOD=MEM', 'VPSUBUSW_rznzmz  ', 'tuple=FVM'];

# PSUBB/PSUBW/PSUBD-Subtract Packed Integers.
T['VPSUBB VREG:w:s8=$XMMr  VREG:r:s8=$XMMn  VREG:r:s8=$XMMm                               ', 'MOD=REG     ', 'VPSUBB_ronomo_1', '         '];
T['VPSUBB VREG:w:s8=$XMMr  VREG:r:s8=$XMMn  MEM:r:s8=$mem128                              ', 'MOD=MEM     ', 'VPSUBB_ronomo_1', '         '];
T['VPSUBB VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   VREG:r:u8=$XMMm             ', 'MOD=REG     ', 'VPSUBB_ronomo_2', '         '];
T['VPSUBB VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   MEM:r:u8=$mem128            ', 'MOD=MEM     ', 'VPSUBB_ronomo_2', 'tuple=FVM'];
T['VPSUBB VREG:w:s8=$YMMr  VREG:r:s8=$YMMn  VREG:r:s8=$YMMm                               ', 'MOD=REG     ', 'VPSUBB_rynymy_1', '         '];
T['VPSUBB VREG:w:s8=$YMMr  VREG:r:s8=$YMMn  MEM:r:s8=$mem256                              ', 'MOD=MEM     ', 'VPSUBB_rynymy_1', '         '];
T['VPSUBB VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   VREG:r:u8=$YMMm             ', 'MOD=REG     ', 'VPSUBB_rynymy_2', '         '];
T['VPSUBB VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   MEM:r:u8=$mem256            ', 'MOD=MEM     ', 'VPSUBB_rynymy_2', 'tuple=FVM'];
T['VPSUBB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   VREG:r:u8=$ZMMm             ', 'MOD=REG     ', 'VPSUBB_rznzmz  ', '         '];
T['VPSUBB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   MEM:r:u8=$mem512            ', 'MOD=MEM     ', 'VPSUBB_rznzmz  ', 'tuple=FVM'];
T['VPSUBD VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm                              ', 'MOD=REG     ', 'VPSUBD_ronomo_1', '         '];
T['VPSUBD VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128                             ', 'MOD=MEM     ', 'VPSUBD_ronomo_1', '         '];
T['VPSUBD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VPSUBD_ronomo_2', '         '];
T['VPSUBD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPSUBD_ronomo_2', 'tuple=FV '];
T['VPSUBD VREG:w:s32=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm                              ', 'MOD=REG     ', 'VPSUBD_rynymy_1', '         '];
T['VPSUBD VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256                             ', 'MOD=MEM     ', 'VPSUBD_rynymy_1', '         '];
T['VPSUBD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VPSUBD_rynymy_2', '         '];
T['VPSUBD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPSUBD_rynymy_2', 'tuple=FV '];
T['VPSUBD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VPSUBD_rznzmz  ', '         '];
T['VPSUBD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPSUBD_rznzmz  ', 'tuple=FV '];
T['VPSUBW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm                              ', 'MOD=REG     ', 'VPSUBW_ronomo_1', '         '];
T['VPSUBW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128                             ', 'MOD=MEM     ', 'VPSUBW_ronomo_1', '         '];
T['VPSUBW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm            ', 'MOD=REG     ', 'VPSUBW_ronomo_2', '         '];
T['VPSUBW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128           ', 'MOD=MEM     ', 'VPSUBW_ronomo_2', 'tuple=FVM'];
T['VPSUBW VREG:w:s16=$YMMr VREG:r:s16=$YMMn VREG:r:s16=$YMMm                              ', 'MOD=REG     ', 'VPSUBW_rynymy_1', '         '];
T['VPSUBW VREG:w:s16=$YMMr VREG:r:s16=$YMMn MEM:r:s16=$mem256                             ', 'MOD=MEM     ', 'VPSUBW_rynymy_1', '         '];
T['VPSUBW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm            ', 'MOD=REG     ', 'VPSUBW_rynymy_2', '         '];
T['VPSUBW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256           ', 'MOD=MEM     ', 'VPSUBW_rynymy_2', 'tuple=FVM'];
T['VPSUBW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm            ', 'MOD=REG     ', 'VPSUBW_rznzmz  ', '         '];
T['VPSUBW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512           ', 'MOD=MEM     ', 'VPSUBW_rznzmz  ', 'tuple=FVM'];

# PTEST-Logical Compare.
T['VPTEST VREG:r:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG VL=128', 'VPTEST', ''];
T['VPTEST VREG:r:s32=$YMMr VREG:r:s32=$YMMm ', 'MOD=REG VL=256', 'VPTEST', ''];
T['VPTEST VREG:r:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM VL=128', 'VPTEST', ''];
T['VPTEST VREG:r:s32=$YMMr MEM:r:s32=$mem256', 'MOD=MEM VL=256', 'VPTEST', ''];

# PUNPCKHBW/PUNPCKHWD/PUNPCKHDQ/PUNPCKHQDQ-Unpack High Data.
T['VPUNPCKHBW  VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  VREG:r:u8=$XMMm                               ', 'MOD=REG     ', 'VPUNPCKHBW_ronomo_1 ', '         '];
T['VPUNPCKHBW  VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  MEM:r:u8=$mem128                              ', 'MOD=MEM     ', 'VPUNPCKHBW_ronomo_1 ', '         '];
T['VPUNPCKHBW  VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   VREG:r:u8=$XMMm             ', 'MOD=REG     ', 'VPUNPCKHBW_ronomo_2 ', '         '];
T['VPUNPCKHBW  VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   MEM:r:u8=$mem128            ', 'MOD=MEM     ', 'VPUNPCKHBW_ronomo_2 ', 'tuple=FVM'];
T['VPUNPCKHBW  VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  VREG:r:u8=$YMMm                               ', 'MOD=REG     ', 'VPUNPCKHBW_rynymy_1 ', '         '];
T['VPUNPCKHBW  VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  MEM:r:u8=$mem256                              ', 'MOD=MEM     ', 'VPUNPCKHBW_rynymy_1 ', '         '];
T['VPUNPCKHBW  VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   VREG:r:u8=$YMMm             ', 'MOD=REG     ', 'VPUNPCKHBW_rynymy_2 ', '         '];
T['VPUNPCKHBW  VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   MEM:r:u8=$mem256            ', 'MOD=MEM     ', 'VPUNPCKHBW_rynymy_2 ', 'tuple=FVM'];
T['VPUNPCKHBW  VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   VREG:r:u8=$ZMMm             ', 'MOD=REG     ', 'VPUNPCKHBW_rznzmz   ', '         '];
T['VPUNPCKHBW  VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   MEM:r:u8=$mem512            ', 'MOD=MEM     ', 'VPUNPCKHBW_rznzmz   ', 'tuple=FVM'];
T['VPUNPCKHDQ  VREG:w:u32=$XMMr VREG:r:u32=$XMMn VREG:r:u32=$XMMm                              ', 'MOD=REG     ', 'VPUNPCKHDQ_ronomo_1 ', '         '];
T['VPUNPCKHDQ  VREG:w:u32=$XMMr VREG:r:u32=$XMMn MEM:r:u32=$mem128                             ', 'MOD=MEM     ', 'VPUNPCKHDQ_ronomo_1 ', '         '];
T['VPUNPCKHDQ  VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VPUNPCKHDQ_ronomo_2 ', '         '];
T['VPUNPCKHDQ  VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPUNPCKHDQ_ronomo_2 ', 'tuple=FV '];
T['VPUNPCKHDQ  VREG:w:u32=$YMMr VREG:r:u32=$YMMn VREG:r:u32=$YMMm                              ', 'MOD=REG     ', 'VPUNPCKHDQ_rynymy_1 ', '         '];
T['VPUNPCKHDQ  VREG:w:u32=$YMMr VREG:r:u32=$YMMn MEM:r:u32=$mem256                             ', 'MOD=MEM     ', 'VPUNPCKHDQ_rynymy_1 ', '         '];
T['VPUNPCKHDQ  VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VPUNPCKHDQ_rynymy_2 ', '         '];
T['VPUNPCKHDQ  VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPUNPCKHDQ_rynymy_2 ', 'tuple=FV '];
T['VPUNPCKHDQ  VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VPUNPCKHDQ_rznzmz   ', '         '];
T['VPUNPCKHDQ  VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPUNPCKHDQ_rznzmz   ', 'tuple=FV '];
T['VPUNPCKHQDQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn VREG:r:u64=$XMMm                              ', 'MOD=REG     ', 'VPUNPCKHQDQ_ronomo_1', '         '];
T['VPUNPCKHQDQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn MEM:r:u64=$mem128                             ', 'MOD=MEM     ', 'VPUNPCKHQDQ_ronomo_1', '         '];
T['VPUNPCKHQDQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG', 'VPUNPCKHQDQ_ronomo_2', '         '];
T['VPUNPCKHQDQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPUNPCKHQDQ_ronomo_2', 'tuple=FV '];
T['VPUNPCKHQDQ VREG:w:u64=$YMMr VREG:r:u64=$YMMn VREG:r:u64=$YMMm                              ', 'MOD=REG     ', 'VPUNPCKHQDQ_rynymy_1', '         '];
T['VPUNPCKHQDQ VREG:w:u64=$YMMr VREG:r:u64=$YMMn MEM:r:u64=$mem256                             ', 'MOD=MEM     ', 'VPUNPCKHQDQ_rynymy_1', '         '];
T['VPUNPCKHQDQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG', 'VPUNPCKHQDQ_rynymy_2', '         '];
T['VPUNPCKHQDQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPUNPCKHQDQ_rynymy_2', 'tuple=FV '];
T['VPUNPCKHQDQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG', 'VPUNPCKHQDQ_rznzmz  ', '         '];
T['VPUNPCKHQDQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPUNPCKHQDQ_rznzmz  ', 'tuple=FV '];
T['VPUNPCKHWD  VREG:w:u16=$XMMr VREG:r:u16=$XMMn VREG:r:u16=$XMMm                              ', 'MOD=REG     ', 'VPUNPCKHWD_ronomo_1 ', '         '];
T['VPUNPCKHWD  VREG:w:u16=$XMMr VREG:r:u16=$XMMn MEM:r:u16=$mem128                             ', 'MOD=MEM     ', 'VPUNPCKHWD_ronomo_1 ', '         '];
T['VPUNPCKHWD  VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm            ', 'MOD=REG     ', 'VPUNPCKHWD_ronomo_2 ', '         '];
T['VPUNPCKHWD  VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128           ', 'MOD=MEM     ', 'VPUNPCKHWD_ronomo_2 ', 'tuple=FVM'];
T['VPUNPCKHWD  VREG:w:u16=$YMMr VREG:r:u16=$YMMn VREG:r:u16=$YMMm                              ', 'MOD=REG     ', 'VPUNPCKHWD_rynymy_1 ', '         '];
T['VPUNPCKHWD  VREG:w:u16=$YMMr VREG:r:u16=$YMMn MEM:r:u16=$mem256                             ', 'MOD=MEM     ', 'VPUNPCKHWD_rynymy_1 ', '         '];
T['VPUNPCKHWD  VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm            ', 'MOD=REG     ', 'VPUNPCKHWD_rynymy_2 ', '         '];
T['VPUNPCKHWD  VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256           ', 'MOD=MEM     ', 'VPUNPCKHWD_rynymy_2 ', 'tuple=FVM'];
T['VPUNPCKHWD  VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm            ', 'MOD=REG     ', 'VPUNPCKHWD_rznzmz   ', '         '];
T['VPUNPCKHWD  VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512           ', 'MOD=MEM     ', 'VPUNPCKHWD_rznzmz   ', 'tuple=FVM'];

# PUNPCKLBW/PUNPCKLWD/PUNPCKLDQ/PUNPCKLQDQ-Unpack Low Data.
T['VPUNPCKLBW  VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  VREG:r:u8=$XMMm                               ', 'MOD=REG     ', 'VPUNPCKLBW_ronomo_1 ', '         '];
T['VPUNPCKLBW  VREG:w:u8=$XMMr  VREG:r:u8=$XMMn  MEM:r:u8=$mem128                              ', 'MOD=MEM     ', 'VPUNPCKLBW_ronomo_1 ', '         '];
T['VPUNPCKLBW  VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   VREG:r:u8=$XMMm             ', 'MOD=REG     ', 'VPUNPCKLBW_ronomo_2 ', '         '];
T['VPUNPCKLBW  VREG:w:u8=$XMMr  PREG:r:Z=$Kk     VREG:r:u8=$XMMn   MEM:r:u8=$mem128            ', 'MOD=MEM     ', 'VPUNPCKLBW_ronomo_2 ', 'tuple=FVM'];
T['VPUNPCKLBW  VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  VREG:r:u8=$YMMm                               ', 'MOD=REG     ', 'VPUNPCKLBW_rynymy_1 ', '         '];
T['VPUNPCKLBW  VREG:w:u8=$YMMr  VREG:r:u8=$YMMn  MEM:r:u8=$mem256                              ', 'MOD=MEM     ', 'VPUNPCKLBW_rynymy_1 ', '         '];
T['VPUNPCKLBW  VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   VREG:r:u8=$YMMm             ', 'MOD=REG     ', 'VPUNPCKLBW_rynymy_2 ', '         '];
T['VPUNPCKLBW  VREG:w:u8=$YMMr  PREG:r:Z=$Kk     VREG:r:u8=$YMMn   MEM:r:u8=$mem256            ', 'MOD=MEM     ', 'VPUNPCKLBW_rynymy_2 ', 'tuple=FVM'];
T['VPUNPCKLBW  VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   VREG:r:u8=$ZMMm             ', 'MOD=REG     ', 'VPUNPCKLBW_rznzmz   ', '         '];
T['VPUNPCKLBW  VREG:w:u8=$ZMMr  PREG:r:Z=$Kk     VREG:r:u8=$ZMMn   MEM:r:u8=$mem512            ', 'MOD=MEM     ', 'VPUNPCKLBW_rznzmz   ', 'tuple=FVM'];
T['VPUNPCKLDQ  VREG:w:u32=$XMMr VREG:r:u32=$XMMn VREG:r:u32=$XMMm                              ', 'MOD=REG     ', 'VPUNPCKLDQ_ronomo_1 ', '         '];
T['VPUNPCKLDQ  VREG:w:u32=$XMMr VREG:r:u32=$XMMn MEM:r:u32=$mem128                             ', 'MOD=MEM     ', 'VPUNPCKLDQ_ronomo_1 ', '         '];
T['VPUNPCKLDQ  VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VPUNPCKLDQ_ronomo_2 ', '         '];
T['VPUNPCKLDQ  VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPUNPCKLDQ_ronomo_2 ', 'tuple=FV '];
T['VPUNPCKLDQ  VREG:w:u32=$YMMr VREG:r:u32=$YMMn VREG:r:u32=$YMMm                              ', 'MOD=REG     ', 'VPUNPCKLDQ_rynymy_1 ', '         '];
T['VPUNPCKLDQ  VREG:w:u32=$YMMr VREG:r:u32=$YMMn MEM:r:u32=$mem256                             ', 'MOD=MEM     ', 'VPUNPCKLDQ_rynymy_1 ', '         '];
T['VPUNPCKLDQ  VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VPUNPCKLDQ_rynymy_2 ', '         '];
T['VPUNPCKLDQ  VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPUNPCKLDQ_rynymy_2 ', 'tuple=FV '];
T['VPUNPCKLDQ  VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VPUNPCKLDQ_rznzmz   ', '         '];
T['VPUNPCKLDQ  VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPUNPCKLDQ_rznzmz   ', 'tuple=FV '];
T['VPUNPCKLQDQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn VREG:r:u64=$XMMm                              ', 'MOD=REG     ', 'VPUNPCKLQDQ_ronomo_1', '         '];
T['VPUNPCKLQDQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn MEM:r:u64=$mem128                             ', 'MOD=MEM     ', 'VPUNPCKLQDQ_ronomo_1', '         '];
T['VPUNPCKLQDQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG', 'VPUNPCKLQDQ_ronomo_2', '         '];
T['VPUNPCKLQDQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPUNPCKLQDQ_ronomo_2', 'tuple=FV '];
T['VPUNPCKLQDQ VREG:w:u64=$YMMr VREG:r:u64=$YMMn VREG:r:u64=$YMMm                              ', 'MOD=REG     ', 'VPUNPCKLQDQ_rynymy_1', '         '];
T['VPUNPCKLQDQ VREG:w:u64=$YMMr VREG:r:u64=$YMMn MEM:r:u64=$mem256                             ', 'MOD=MEM     ', 'VPUNPCKLQDQ_rynymy_1', '         '];
T['VPUNPCKLQDQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG', 'VPUNPCKLQDQ_rynymy_2', '         '];
T['VPUNPCKLQDQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPUNPCKLQDQ_rynymy_2', 'tuple=FV '];
T['VPUNPCKLQDQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG', 'VPUNPCKLQDQ_rznzmz  ', '         '];
T['VPUNPCKLQDQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPUNPCKLQDQ_rznzmz  ', 'tuple=FV '];
T['VPUNPCKLWD  VREG:w:u16=$XMMr VREG:r:u16=$XMMn VREG:r:u16=$XMMm                              ', 'MOD=REG     ', 'VPUNPCKLWD_ronomo_1 ', '         '];
T['VPUNPCKLWD  VREG:w:u16=$XMMr VREG:r:u16=$XMMn MEM:r:u16=$mem128                             ', 'MOD=MEM     ', 'VPUNPCKLWD_ronomo_1 ', '         '];
T['VPUNPCKLWD  VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm            ', 'MOD=REG     ', 'VPUNPCKLWD_ronomo_2 ', '         '];
T['VPUNPCKLWD  VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128           ', 'MOD=MEM     ', 'VPUNPCKLWD_ronomo_2 ', 'tuple=FVM'];
T['VPUNPCKLWD  VREG:w:u16=$YMMr VREG:r:u16=$YMMn VREG:r:u16=$YMMm                              ', 'MOD=REG     ', 'VPUNPCKLWD_rynymy_1 ', '         '];
T['VPUNPCKLWD  VREG:w:u16=$YMMr VREG:r:u16=$YMMn MEM:r:u16=$mem256                             ', 'MOD=MEM     ', 'VPUNPCKLWD_rynymy_1 ', '         '];
T['VPUNPCKLWD  VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm            ', 'MOD=REG     ', 'VPUNPCKLWD_rynymy_2 ', '         '];
T['VPUNPCKLWD  VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256           ', 'MOD=MEM     ', 'VPUNPCKLWD_rynymy_2 ', 'tuple=FVM'];
T['VPUNPCKLWD  VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm            ', 'MOD=REG     ', 'VPUNPCKLWD_rznzmz   ', '         '];
T['VPUNPCKLWD  VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512           ', 'MOD=MEM     ', 'VPUNPCKLWD_rznzmz   ', 'tuple=FVM'];

# PXOR-Logical Exclusive OR.
T['VPXOR REG:w:u128=$XMMr REG:r:u128=$XMMn REG:r:u128=$XMMm  ', 'MOD=REG', 'VPXOR_ronomo', ''];
T['VPXOR REG:w:u128=$XMMr REG:r:u128=$XMMn MEM:r:u128=$mem128', 'MOD=MEM', 'VPXOR_ronomo', ''];
T['VPXOR REG:w:u256=$YMMr REG:r:u256=$YMMn REG:r:u256=$YMMm  ', 'MOD=REG', 'VPXOR_rynymy', ''];
T['VPXOR REG:w:u256=$YMMr REG:r:u256=$YMMn MEM:r:u256=$mem256', 'MOD=MEM', 'VPXOR_rynymy', ''];

# RCPPS-Compute Reciprocals of Packed Single-Precision Floating-Point Values.
T['VRCPPS VREG:w:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG VL=128', 'VRCPPS', ''];
T['VRCPPS VREG:w:f32=$YMMr VREG:r:f32=$YMMm ', 'MOD=REG VL=256', 'VRCPPS', ''];
T['VRCPPS VREG:w:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM VL=128', 'VRCPPS', ''];
T['VRCPPS VREG:w:f32=$YMMr MEM:r:f32=$mem256', 'MOD=MEM VL=256', 'VRCPPS', ''];

# RCPSS-Compute Reciprocal of Scalar Single-Precision Floating-Point Values.
T['VRCPSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn REG:r:f32=$XMMm ', 'MOD=REG', 'VRCPSS', ''];
T['VRCPSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem32', 'MOD=MEM', 'VRCPSS', ''];

# ROUNDPD-Round Packed Double Precision Floating-Point Values.
T['VROUNDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMm  RNDC:r:u8=$rndc2', 'MOD=REG VL=128', 'VROUNDPD', ''];
T['VROUNDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMm  RNDC:r:u8=$rndc2', 'MOD=REG VL=256', 'VROUNDPD', ''];
T['VROUNDPD VREG:w:f64=$XMMr MEM:r:f64=$mem128 RNDC:r:u8=$rndc2', 'MOD=MEM VL=128', 'VROUNDPD', ''];
T['VROUNDPD VREG:w:f64=$YMMr MEM:r:f64=$mem256 RNDC:r:u8=$rndc2', 'MOD=MEM VL=256', 'VROUNDPD', ''];

# ROUNDPS-Round Packed Single Precision Floating-Point Values.
T['VROUNDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMm  RNDC:r:u8=$rndc2', 'MOD=REG VL=128', 'VROUNDPS', ''];
T['VROUNDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMm  RNDC:r:u8=$rndc2', 'MOD=REG VL=256', 'VROUNDPS', ''];
T['VROUNDPS VREG:w:f32=$XMMr MEM:r:f32=$mem128 RNDC:r:u8=$rndc2', 'MOD=MEM VL=128', 'VROUNDPS', ''];
T['VROUNDPS VREG:w:f32=$YMMr MEM:r:f32=$mem256 RNDC:r:u8=$rndc2', 'MOD=MEM VL=256', 'VROUNDPS', ''];

# ROUNDSD-Round Scalar Double Precision Floating-Point Values.
T['VROUNDSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn REG:r:f64=$XMMm  RNDC:r:u8=$rndc2', 'MOD=REG', 'VROUNDSD', ''];
T['VROUNDSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem64 RNDC:r:u8=$rndc2', 'MOD=MEM', 'VROUNDSD', ''];

# ROUNDSS-Round Scalar Single Precision Floating-Point Values.
T['VROUNDSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn REG:r:f32=$XMMm  RNDC:r:u8=$rndc2', 'MOD=REG', 'VROUNDSS', ''];
T['VROUNDSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem32 RNDC:r:u8=$rndc2', 'MOD=MEM', 'VROUNDSS', ''];

# RSQRTPS-Compute Reciprocals of Square Roots of Packed Single-Precision Floating-Point Values.
T['VRSQRTPS VREG:w:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG VL=128', 'VRSQRTPS', ''];
T['VRSQRTPS VREG:w:f32=$YMMr VREG:r:f32=$YMMm ', 'MOD=REG VL=256', 'VRSQRTPS', ''];
T['VRSQRTPS VREG:w:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM VL=128', 'VRSQRTPS', ''];
T['VRSQRTPS VREG:w:f32=$YMMr MEM:r:f32=$mem256', 'MOD=MEM VL=256', 'VRSQRTPS', ''];

# RSQRTSS-Compute Reciprocal of Square Root of Scalar Single-Precision Floating-Point Value.
T['VRSQRTSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn REG:r:f32=$XMMm ', 'MOD=REG', 'VRSQRTSS', ''];
T['VRSQRTSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem32', 'MOD=MEM', 'VRSQRTSS', ''];

# SHUFPD-Packed Interleave Shuffle of Pairs of Double-Precision Floating-Point Values.
T['VSHUFPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm             ORDER:r:u8=$order', 'BP=0 MOD=REG  ', 'VSHUFPD_ronomoub', '        '];
T['VSHUFPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST ORDER:r:u8=$order', 'MOD=MEM       ', 'VSHUFPD_ronomoub', 'tuple=FV'];
T['VSHUFPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm  ORDER:r:u8=$order                             ', 'MOD=REG VL=128', 'VSHUFPD_rxnxmxub', '        '];
T['VSHUFPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm  ORDER:r:u8=$order                             ', 'MOD=REG VL=256', 'VSHUFPD_rxnxmxub', '        '];
T['VSHUFPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128 ORDER:r:u8=$order                             ', 'MOD=MEM VL=128', 'VSHUFPD_rxnxmxub', '        '];
T['VSHUFPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256 ORDER:r:u8=$order                             ', 'MOD=MEM VL=256', 'VSHUFPD_rxnxmxub', '        '];
T['VSHUFPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm             ORDER:r:u8=$order', 'BP=0 MOD=REG  ', 'VSHUFPD_rynymyub', '        '];
T['VSHUFPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST ORDER:r:u8=$order', 'MOD=MEM       ', 'VSHUFPD_rynymyub', 'tuple=FV'];
T['VSHUFPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm             ORDER:r:u8=$order', 'BP=0 MOD=REG  ', 'VSHUFPD_rznzmzub', '        '];
T['VSHUFPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST ORDER:r:u8=$order', 'MOD=MEM       ', 'VSHUFPD_rznzmzub', 'tuple=FV'];

# SHUFPS-Packed Interleave Shuffle of Quadruplets of Single-Precision Floating-Point Values.
T['VSHUFPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm             ORDER:r:u8=$order', 'BP=0 MOD=REG  ', 'VSHUFPS_ronomoub', '        '];
T['VSHUFPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST ORDER:r:u8=$order', 'MOD=MEM       ', 'VSHUFPS_ronomoub', 'tuple=FV'];
T['VSHUFPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm  ORDER:r:u8=$order                             ', 'MOD=REG VL=128', 'VSHUFPS_rxnxmxub', '        '];
T['VSHUFPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm  ORDER:r:u8=$order                             ', 'MOD=REG VL=256', 'VSHUFPS_rxnxmxub', '        '];
T['VSHUFPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128 ORDER:r:u8=$order                             ', 'MOD=MEM VL=128', 'VSHUFPS_rxnxmxub', '        '];
T['VSHUFPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256 ORDER:r:u8=$order                             ', 'MOD=MEM VL=256', 'VSHUFPS_rxnxmxub', '        '];
T['VSHUFPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm             ORDER:r:u8=$order', 'BP=0 MOD=REG  ', 'VSHUFPS_rynymyub', '        '];
T['VSHUFPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST ORDER:r:u8=$order', 'MOD=MEM       ', 'VSHUFPS_rynymyub', 'tuple=FV'];
T['VSHUFPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm             ORDER:r:u8=$order', 'BP=0 MOD=REG  ', 'VSHUFPS_rznzmzub', '        '];
T['VSHUFPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST ORDER:r:u8=$order', 'MOD=MEM       ', 'VSHUFPS_rznzmzub', 'tuple=FV'];

# SQRTPD-Square Root of Double-Precision Floating-Point Values.
T['VSQRTPD VREG:w:f64=$XMMr PREG:r:Z=$Kk      VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VSQRTPD_romo', '        '];
T['VSQRTPD VREG:w:f64=$XMMr PREG:r:Z=$Kk      MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VSQRTPD_romo', 'tuple=FV'];
T['VSQRTPD VREG:w:f64=$XMMr VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VSQRTPD_rxmx', '        '];
T['VSQRTPD VREG:w:f64=$YMMr VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VSQRTPD_rxmx', '        '];
T['VSQRTPD VREG:w:f64=$XMMr MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VSQRTPD_rxmx', '        '];
T['VSQRTPD VREG:w:f64=$YMMr MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VSQRTPD_rxmx', '        '];
T['VSQRTPD VREG:w:f64=$YMMr PREG:r:Z=$Kk      VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VSQRTPD_rymy', '        '];
T['VSQRTPD VREG:w:f64=$YMMr PREG:r:Z=$Kk      MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VSQRTPD_rymy', 'tuple=FV'];
T['VSQRTPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk      VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VSQRTPD_rzmz', '        '];
T['VSQRTPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk      VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VSQRTPD_rzmz', '        '];
T['VSQRTPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk      MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VSQRTPD_rzmz', 'tuple=FV'];

# SQRTPS-Square Root of Single-Precision Floating-Point Values.
T['VSQRTPS VREG:w:f32=$XMMr PREG:r:Z=$Kk      VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VSQRTPS_romo', '        '];
T['VSQRTPS VREG:w:f32=$XMMr PREG:r:Z=$Kk      MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VSQRTPS_romo', 'tuple=FV'];
T['VSQRTPS VREG:w:f32=$XMMr VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VSQRTPS_rxmx', '        '];
T['VSQRTPS VREG:w:f32=$YMMr VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VSQRTPS_rxmx', '        '];
T['VSQRTPS VREG:w:f32=$XMMr MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VSQRTPS_rxmx', '        '];
T['VSQRTPS VREG:w:f32=$YMMr MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VSQRTPS_rxmx', '        '];
T['VSQRTPS VREG:w:f32=$YMMr PREG:r:Z=$Kk      VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VSQRTPS_rymy', '        '];
T['VSQRTPS VREG:w:f32=$YMMr PREG:r:Z=$Kk      MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VSQRTPS_rymy', 'tuple=FV'];
T['VSQRTPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk      VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VSQRTPS_rzmz', '        '];
T['VSQRTPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk      VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VSQRTPS_rzmz', '        '];
T['VSQRTPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk      MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VSQRTPS_rzmz', 'tuple=FV'];

# SQRTSD-Compute Square Root of Scalar Double-Precision Floating-Point Value.
T['VSQRTSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VSQRTSD_ronomq', '         '];
T['VSQRTSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VSQRTSD_ronomq', '         '];
T['VSQRTSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VSQRTSD_ronomx', '         '];
T['VSQRTSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VSQRTSD_ronomx', '         '];
T['VSQRTSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VSQRTSD_ronomx', 'tuple=T1S'];

# SQRTSS-Compute Square Root of Scalar Single-Precision Value.
T['VSQRTSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn REG:r:f32=$XMMm                         ', 'MOD=REG             ', 'VSQRTSS_ronomd', '         '];
T['VSQRTSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem32                        ', 'MOD=MEM             ', 'VSQRTSS_ronomd', '         '];
T['VSQRTSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VSQRTSS_ronomx', '         '];
T['VSQRTSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VSQRTSS_ronomx', '         '];
T['VSQRTSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VSQRTSS_ronomx', 'tuple=T1S'];

# STMXCSR-Store MXCSR Register State.
T['VSTMXCSR MEM:w:s32=$mem32 REG:SUPP:r=MXCSR', 'NONE', 'VSTMXCSR', ''];

# SUBPD-Subtract Packed Double-Precision Floating-Point Values.
T['VSUBPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VSUBPD_ronomo', '        '];
T['VSUBPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VSUBPD_ronomo', 'tuple=FV'];
T['VSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VSUBPD_rxnxmx', '        '];
T['VSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VSUBPD_rxnxmx', '        '];
T['VSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VSUBPD_rxnxmx', '        '];
T['VSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VSUBPD_rxnxmx', '        '];
T['VSUBPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VSUBPD_rynymy', '        '];
T['VSUBPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VSUBPD_rynymy', 'tuple=FV'];
T['VSUBPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VSUBPD_rznzmz', '        '];
T['VSUBPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VSUBPD_rznzmz', '        '];
T['VSUBPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VSUBPD_rznzmz', 'tuple=FV'];

# SUBPS-Subtract Packed Single-Precision Floating-Point Values.
T['VSUBPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VSUBPS_ronomo', '        '];
T['VSUBPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VSUBPS_ronomo', 'tuple=FV'];
T['VSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VSUBPS_rxnxmx', '        '];
T['VSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VSUBPS_rxnxmx', '        '];
T['VSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VSUBPS_rxnxmx', '        '];
T['VSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VSUBPS_rxnxmx', '        '];
T['VSUBPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VSUBPS_rynymy', '        '];
T['VSUBPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VSUBPS_rynymy', 'tuple=FV'];
T['VSUBPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VSUBPS_rznzmz', '        '];
T['VSUBPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VSUBPS_rznzmz', '        '];
T['VSUBPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VSUBPS_rznzmz', 'tuple=FV'];

# SUBSD-Subtract Scalar Double-Precision Floating-Point Value.
T['VSUBSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VSUBSD_ronomq', '         '];
T['VSUBSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VSUBSD_ronomq', '         '];
T['VSUBSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VSUBSD_ronomx', '         '];
T['VSUBSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VSUBSD_ronomx', '         '];
T['VSUBSD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VSUBSD_ronomx', 'tuple=T1S'];

# SUBSS-Subtract Scalar Single-Precision Floating-Point Value.
T['VSUBSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn REG:r:f32=$XMMm                         ', 'MOD=REG             ', 'VSUBSS_ronomd', '         '];
T['VSUBSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem32                        ', 'MOD=MEM             ', 'VSUBSS_ronomd', '         '];
T['VSUBSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VSUBSS_ronomx', '         '];
T['VSUBSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VSUBSS_ronomx', '         '];
T['VSUBSS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VSUBSS_ronomx', 'tuple=T1S'];

# UCOMISD-Unordered Compare Scalar Double-Precision Floating-Point Values and Set EFLAGS.
T['VUCOMISD VREG:r:f64=$XMMr REG:r:f64=$XMMm          ', 'MOD=REG             ', 'VUCOMISD_romq', '         '];
T['VUCOMISD VREG:r:f64=$XMMr MEM:r:f64=$mem64         ', 'MOD=MEM             ', 'VUCOMISD_romq', '         '];
T['VUCOMISD VREG:r:f64=$XMMr VREG:r:f64=$XMMm         ', 'BP=0 MOD=REG        ', 'VUCOMISD_romx', '         '];
T['VUCOMISD VREG:r:f64=$XMMr VREG:r:f64=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128', 'VUCOMISD_romx', '         '];
T['VUCOMISD VREG:r:f64=$XMMr MEM:r:f64=$mem64         ', 'BP=0 MOD=MEM        ', 'VUCOMISD_romx', 'tuple=T1S'];

# UCOMISS-Unordered Compare Scalar Single-Precision Floating-Point Values and Set EFLAGS.
T['VUCOMISS VREG:r:f32=$XMMr REG:r:f32=$XMMm          ', 'MOD=REG             ', 'VUCOMISS_romd', '         '];
T['VUCOMISS VREG:r:f32=$XMMr MEM:r:f32=$mem32         ', 'MOD=MEM             ', 'VUCOMISS_romd', '         '];
T['VUCOMISS VREG:r:f32=$XMMr VREG:r:f32=$XMMm         ', 'BP=0 MOD=REG        ', 'VUCOMISS_romx', '         '];
T['VUCOMISS VREG:r:f32=$XMMr VREG:r:f32=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128', 'VUCOMISS_romx', '         '];
T['VUCOMISS VREG:r:f32=$XMMr MEM:r:f32=$mem32         ', 'BP=0 MOD=MEM        ', 'VUCOMISS_romx', 'tuple=T1S'];

# UNPCKHPD-Unpack and Interleave High Packed Double-Precision Floating-Point Values.
T['VUNPCKHPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG  ', 'VUNPCKHPD_ronomo', '        '];
T['VUNPCKHPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM       ', 'VUNPCKHPD_ronomo', 'tuple=FV'];
T['VUNPCKHPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128', 'VUNPCKHPD_rxnxmx', '        '];
T['VUNPCKHPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256', 'VUNPCKHPD_rxnxmx', '        '];
T['VUNPCKHPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128', 'VUNPCKHPD_rxnxmx', '        '];
T['VUNPCKHPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256', 'VUNPCKHPD_rxnxmx', '        '];
T['VUNPCKHPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG  ', 'VUNPCKHPD_rynymy', '        '];
T['VUNPCKHPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM       ', 'VUNPCKHPD_rynymy', 'tuple=FV'];
T['VUNPCKHPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG  ', 'VUNPCKHPD_rznzmz', '        '];
T['VUNPCKHPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM       ', 'VUNPCKHPD_rznzmz', 'tuple=FV'];

# UNPCKHPS-Unpack and Interleave High Packed Single-Precision Floating-Point Values.
T['VUNPCKHPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG  ', 'VUNPCKHPS_ronomo', '        '];
T['VUNPCKHPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM       ', 'VUNPCKHPS_ronomo', 'tuple=FV'];
T['VUNPCKHPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128', 'VUNPCKHPS_rxnxmx', '        '];
T['VUNPCKHPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256', 'VUNPCKHPS_rxnxmx', '        '];
T['VUNPCKHPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128', 'VUNPCKHPS_rxnxmx', '        '];
T['VUNPCKHPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256', 'VUNPCKHPS_rxnxmx', '        '];
T['VUNPCKHPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG  ', 'VUNPCKHPS_rynymy', '        '];
T['VUNPCKHPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM       ', 'VUNPCKHPS_rynymy', 'tuple=FV'];
T['VUNPCKHPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG  ', 'VUNPCKHPS_rznzmz', '        '];
T['VUNPCKHPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM       ', 'VUNPCKHPS_rznzmz', 'tuple=FV'];

# UNPCKLPD-Unpack and Interleave Low Packed Double-Precision Floating-Point Values.
T['VUNPCKLPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG  ', 'VUNPCKLPD_ronomo', '        '];
T['VUNPCKLPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM       ', 'VUNPCKLPD_ronomo', 'tuple=FV'];
T['VUNPCKLPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128', 'VUNPCKLPD_rxnxmx', '        '];
T['VUNPCKLPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256', 'VUNPCKLPD_rxnxmx', '        '];
T['VUNPCKLPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128', 'VUNPCKLPD_rxnxmx', '        '];
T['VUNPCKLPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256', 'VUNPCKLPD_rxnxmx', '        '];
T['VUNPCKLPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG  ', 'VUNPCKLPD_rynymy', '        '];
T['VUNPCKLPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM       ', 'VUNPCKLPD_rynymy', 'tuple=FV'];
T['VUNPCKLPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG  ', 'VUNPCKLPD_rznzmz', '        '];
T['VUNPCKLPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM       ', 'VUNPCKLPD_rznzmz', 'tuple=FV'];

# UNPCKLPS-Unpack and Interleave Low Packed Single-Precision Floating-Point Values.
T['VUNPCKLPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG  ', 'VUNPCKLPS_ronomo', '        '];
T['VUNPCKLPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM       ', 'VUNPCKLPS_ronomo', 'tuple=FV'];
T['VUNPCKLPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128', 'VUNPCKLPS_rxnxmx', '        '];
T['VUNPCKLPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256', 'VUNPCKLPS_rxnxmx', '        '];
T['VUNPCKLPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128', 'VUNPCKLPS_rxnxmx', '        '];
T['VUNPCKLPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256', 'VUNPCKLPS_rxnxmx', '        '];
T['VUNPCKLPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG  ', 'VUNPCKLPS_rynymy', '        '];
T['VUNPCKLPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM       ', 'VUNPCKLPS_rynymy', 'tuple=FV'];
T['VUNPCKLPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG  ', 'VUNPCKLPS_rznzmz', '        '];
T['VUNPCKLPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM       ', 'VUNPCKLPS_rznzmz', 'tuple=FV'];

# VALIGND/VALIGNQ-Align Doubleword/Quadword Vectors.
T['VALIGND VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn VREG:r:u32=$XMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG W=0', 'VALIGNx_ronomoub', '        '];
T['VALIGND VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM W=0     ', 'VALIGNx_ronomoub', 'tuple=FV'];
T['VALIGNQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG W=1', 'VALIGNx_ronomoub', '        '];
T['VALIGNQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM W=1     ', 'VALIGNx_ronomoub', 'tuple=FV'];
T['VALIGND VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG W=0', 'VALIGNx_rynymyub', '        '];
T['VALIGND VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM W=0     ', 'VALIGNx_rynymyub', 'tuple=FV'];
T['VALIGNQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG W=1', 'VALIGNx_rynymyub', '        '];
T['VALIGNQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM W=1     ', 'VALIGNx_rynymyub', 'tuple=FV'];
T['VALIGND VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG W=0', 'VALIGNx_rznzmzub', '        '];
T['VALIGND VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM W=0     ', 'VALIGNx_rznzmzub', 'tuple=FV'];
T['VALIGNQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG W=1', 'VALIGNx_rznzmzub', '        '];
T['VALIGNQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM W=1     ', 'VALIGNx_rznzmzub', 'tuple=FV'];

# VBLENDMPD/VBLENDMPS-Blend Float64/Float32 Vectors Using an OpMask Control.
T['VBLENDMPD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VBLENDMPx_ronomo', '        '];
T['VBLENDMPD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VBLENDMPx_ronomo', 'tuple=FV'];
T['VBLENDMPS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VBLENDMPx_ronomo', '        '];
T['VBLENDMPS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VBLENDMPx_ronomo', 'tuple=FV'];
T['VBLENDMPD VREG:w:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMn VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VBLENDMPx_rynymy', '        '];
T['VBLENDMPD VREG:w:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMn MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VBLENDMPx_rynymy', 'tuple=FV'];
T['VBLENDMPS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMn VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VBLENDMPx_rynymy', '        '];
T['VBLENDMPS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMn MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VBLENDMPx_rynymy', 'tuple=FV'];
T['VBLENDMPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VBLENDMPx_rznzmz', '        '];
T['VBLENDMPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VBLENDMPx_rznzmz', 'tuple=FV'];
T['VBLENDMPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VBLENDMPx_rznzmz', '        '];
T['VBLENDMPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VBLENDMPx_rznzmz', 'tuple=FV'];

# VBROADCAST-Load with Broadcast Floating-Point Data.
T['VBROADCASTF128  VREG:w:f64=$YMMr MEM:r:f64=$mem128 SBCST:SUPP={FROM=2 TO=4}                                            ', 'NONE   ', 'VBROADCASTF128      ', '           '];
T['VBROADCASTF32X2 VREG:w:f32=$YMMr PREG:r:Z=$Kk                               VREG:r:f32=$XMMm                           ', 'MOD=REG', 'VBROADCASTF32X2_rymx', '           '];
T['VBROADCASTF32X2 VREG:w:f32=$YMMr PREG:r:Z=$Kk                               MEM:r:f32=$mem64 SBCST:SUPP={FROM=2 TO=8}  ', 'MOD=MEM', 'VBROADCASTF32X2_rymx', 'tuple=T2   '];
T['VBROADCASTF32X2 VREG:w:f32=$ZMMr PREG:r:Z=$Kk                               VREG:r:f32=$XMMm                           ', 'MOD=REG', 'VBROADCASTF32X2_rzmx', '           '];
T['VBROADCASTF32X2 VREG:w:f32=$ZMMr PREG:r:Z=$Kk                               MEM:r:f32=$mem64 SBCST:SUPP={FROM=2 TO=16} ', 'MOD=MEM', 'VBROADCASTF32X2_rzmx', 'tuple=T2   '];
T['VBROADCASTF32X4 VREG:w:f32=$YMMr PREG:r:Z=$Kk                               MEM:r:f32=$mem128 SBCST:SUPP={FROM=4 TO=8} ', 'NONE   ', 'VBROADCASTF32X4_rymo', 'tuple=T4   '];
T['VBROADCASTF32X4 VREG:w:f32=$ZMMr PREG:r:Z=$Kk                               MEM:r:f32=$mem128 SBCST:SUPP={FROM=4 TO=16}', 'NONE   ', 'VBROADCASTF32X4_rzmo', 'tuple=T4   '];
T['VBROADCASTF32X8 VREG:w:f32=$ZMMr PREG:r:Z=$Kk                               MEM:r:f32=$mem256 SBCST:SUPP={FROM=8 TO=16}', 'NONE   ', 'VBROADCASTF32X8     ', 'tuple=T8   '];
T['VBROADCASTF64X2 VREG:w:f64=$YMMr PREG:r:Z=$Kk                               MEM:r:f64=$mem128 SBCST:SUPP={FROM=2 TO=4} ', 'NONE   ', 'VBROADCASTF64X2_rymo', 'tuple=T2   '];
T['VBROADCASTF64X2 VREG:w:f64=$ZMMr PREG:r:Z=$Kk                               MEM:r:f64=$mem128 SBCST:SUPP={FROM=2 TO=8} ', 'NONE   ', 'VBROADCASTF64X2_rzmo', 'tuple=T2   '];
T['VBROADCASTF64X4 VREG:w:f64=$ZMMr PREG:r:Z=$Kk                               MEM:r:f64=$mem256 SBCST:SUPP={FROM=4 TO=8} ', 'NONE   ', 'VBROADCASTF64X4_rzmy', 'tuple=T4   '];
T['VBROADCASTSD    VREG:w:f64=$YMMr VREG:r:f64=$XMMm                                                                      ', 'NONE   ', 'VBROADCASTSD_rymo   ', '           '];
T['VBROADCASTSD    VREG:w:f64=$YMMr MEM:r:f64=$mem64 SBCST:SUPP={FROM=1 TO=4}                                             ', 'NONE   ', 'VBROADCASTSD_rymq   ', '           '];
T['VBROADCASTSD    VREG:w:f64=$YMMr PREG:r:Z=$Kk                               VREG:r:f64=$XMMm                           ', 'MOD=REG', 'VBROADCASTSD_rymx   ', '           '];
T['VBROADCASTSD    VREG:w:f64=$YMMr PREG:r:Z=$Kk                               MEM:r:f64=$mem64 SBCST:SUPP={FROM=1 TO=4}  ', 'MOD=MEM', 'VBROADCASTSD_rymx   ', 'tuple=T1F64'];
T['VBROADCASTSD    VREG:w:f64=$ZMMr PREG:r:Z=$Kk                               VREG:r:f64=$XMMm                           ', 'MOD=REG', 'VBROADCASTSD_rzmx   ', '           '];
T['VBROADCASTSD    VREG:w:f64=$ZMMr PREG:r:Z=$Kk                               MEM:r:f64=$mem64 SBCST:SUPP={FROM=1 TO=8}  ', 'MOD=MEM', 'VBROADCASTSD_rzmx   ', 'tuple=T1F64'];
T['VBROADCASTSS    VREG:w:f32=$XMMr PREG:r:Z=$Kk                               VREG:r:f32=$XMMm                           ', 'MOD=REG', 'VBROADCASTSS_romx   ', '           '];
T['VBROADCASTSS    VREG:w:f32=$XMMr PREG:r:Z=$Kk                               MEM:r:f32=$mem32 SBCST:SUPP={FROM=1 TO=4}  ', 'MOD=MEM', 'VBROADCASTSS_romx   ', 'tuple=T1F32'];
T['VBROADCASTSS    VREG:w:f32=$XMMr MEM:r:f32=$mem32 SBCST:SUPP={FROM=1 TO=4}                                             ', 'VL=128 ', 'VBROADCASTSS_rxmd   ', '           '];
T['VBROADCASTSS    VREG:w:f32=$YMMr MEM:r:f32=$mem32 SBCST:SUPP={FROM=1 TO=8}                                             ', 'VL=256 ', 'VBROADCASTSS_rxmd   ', '           '];
T['VBROADCASTSS    VREG:w:f32=$XMMr VREG:r:f32=$XMMm                                                                      ', 'VL=128 ', 'VBROADCASTSS_rxmo   ', '           '];
T['VBROADCASTSS    VREG:w:f32=$YMMr VREG:r:f32=$XMMm                                                                      ', 'VL=256 ', 'VBROADCASTSS_rxmo   ', '           '];
T['VBROADCASTSS    VREG:w:f32=$YMMr PREG:r:Z=$Kk                               VREG:r:f32=$XMMm                           ', 'MOD=REG', 'VBROADCASTSS_rymx   ', '           '];
T['VBROADCASTSS    VREG:w:f32=$YMMr PREG:r:Z=$Kk                               MEM:r:f32=$mem32 SBCST:SUPP={FROM=1 TO=8}  ', 'MOD=MEM', 'VBROADCASTSS_rymx   ', 'tuple=T1F32'];
T['VBROADCASTSS    VREG:w:f32=$ZMMr PREG:r:Z=$Kk                               VREG:r:f32=$XMMm                           ', 'MOD=REG', 'VBROADCASTSS_rzmx   ', '           '];
T['VBROADCASTSS    VREG:w:f32=$ZMMr PREG:r:Z=$Kk                               MEM:r:f32=$mem32 SBCST:SUPP={FROM=1 TO=16} ', 'MOD=MEM', 'VBROADCASTSS_rzmx   ', 'tuple=T1F32'];

# VCOMPRESSPD-Store Sparse Packed Double-Precision Floating-Point Values into Dense Memory.
T['VCOMPRESSPD VREG:w:f64=$XMMm  PREG:r:Z=$Kk VREG:r:f64=$XMMr', 'MOD=REG', 'VCOMPRESSPD_moro', '         '];
T['VCOMPRESSPD MEM:w:f64=$mem128 PREG:r:Z=$Kk VREG:r:f64=$XMMr', 'MOD=MEM', 'VCOMPRESSPD_moro', 'tuple=T1S'];
T['VCOMPRESSPD VREG:w:f64=$YMMm  PREG:r:Z=$Kk VREG:r:f64=$YMMr', 'MOD=REG', 'VCOMPRESSPD_myry', '         '];
T['VCOMPRESSPD MEM:w:f64=$mem256 PREG:r:Z=$Kk VREG:r:f64=$YMMr', 'MOD=MEM', 'VCOMPRESSPD_myry', 'tuple=T1S'];
T['VCOMPRESSPD VREG:w:f64=$ZMMm  PREG:r:Z=$Kk VREG:r:f64=$ZMMr', 'MOD=REG', 'VCOMPRESSPD_mzrz', '         '];
T['VCOMPRESSPD MEM:w:f64=$mem512 PREG:r:Z=$Kk VREG:r:f64=$ZMMr', 'MOD=MEM', 'VCOMPRESSPD_mzrz', 'tuple=T1S'];

# VCOMPRESSPS-Store Sparse Packed Single-Precision Floating-Point Values into Dense Memory.
T['VCOMPRESSPS VREG:w:f32=$XMMm  PREG:r:Z=$Kk VREG:r:f32=$XMMr', 'MOD=REG', 'VCOMPRESSPS_moro', '         '];
T['VCOMPRESSPS MEM:w:f32=$mem128 PREG:r:Z=$Kk VREG:r:f32=$XMMr', 'MOD=MEM', 'VCOMPRESSPS_moro', 'tuple=T1S'];
T['VCOMPRESSPS VREG:w:f32=$YMMm  PREG:r:Z=$Kk VREG:r:f32=$YMMr', 'MOD=REG', 'VCOMPRESSPS_myry', '         '];
T['VCOMPRESSPS MEM:w:f32=$mem256 PREG:r:Z=$Kk VREG:r:f32=$YMMr', 'MOD=MEM', 'VCOMPRESSPS_myry', 'tuple=T1S'];
T['VCOMPRESSPS VREG:w:f32=$ZMMm  PREG:r:Z=$Kk VREG:r:f32=$ZMMr', 'MOD=REG', 'VCOMPRESSPS_mzrz', '         '];
T['VCOMPRESSPS MEM:w:f32=$mem512 PREG:r:Z=$Kk VREG:r:f32=$ZMMr', 'MOD=MEM', 'VCOMPRESSPS_mzrz', 'tuple=T1S'];

# VCVTNE2PS2BF16.
T['VCVTNE2PS2BF16 VREG:w:u16=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG', 'VCVTNE2PS2BF16_ronomo', '        '];
T['VCVTNE2PS2BF16 VREG:w:u16=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VCVTNE2PS2BF16_ronomo', 'tuple=FV'];
T['VCVTNE2PS2BF16 VREG:w:u16=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMn VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG', 'VCVTNE2PS2BF16_rynymy', '        '];
T['VCVTNE2PS2BF16 VREG:w:u16=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMn MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VCVTNE2PS2BF16_rynymy', 'tuple=FV'];
T['VCVTNE2PS2BF16 VREG:w:u16=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG', 'VCVTNE2PS2BF16_rznzmz', '        '];
T['VCVTNE2PS2BF16 VREG:w:u16=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VCVTNE2PS2BF16_rznzmz', 'tuple=FV'];

# VCVTNEPS2BF16.
T['VCVTNEPS2BF16 VREG:w:u16=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG', 'VCVTNEPS2BF16_romo', '        '];
T['VCVTNEPS2BF16 VREG:w:u16=$XMMr PREG:r:Z=$Kk MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VCVTNEPS2BF16_romo', 'tuple=FV'];
T['VCVTNEPS2BF16 VREG:w:u16=$XMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG', 'VCVTNEPS2BF16_romy', '        '];
T['VCVTNEPS2BF16 VREG:w:u16=$XMMr PREG:r:Z=$Kk MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VCVTNEPS2BF16_romy', 'tuple=FV'];
T['VCVTNEPS2BF16 VREG:w:u16=$YMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG', 'VCVTNEPS2BF16_rymz', '        '];
T['VCVTNEPS2BF16 VREG:w:u16=$YMMr PREG:r:Z=$Kk MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VCVTNEPS2BF16_rymz', 'tuple=FV'];

# VCVTPD2QQ-Convert Packed Double-Precision Floating-Point Values to Packed Quadword Integers.
T['VCVTPD2QQ VREG:w:s64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTPD2QQ_romo', '        '];
T['VCVTPD2QQ VREG:w:s64=$XMMr PREG:r:Z=$Kk MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTPD2QQ_romo', 'tuple=FV'];
T['VCVTPD2QQ VREG:w:s64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTPD2QQ_rymy', '        '];
T['VCVTPD2QQ VREG:w:s64=$YMMr PREG:r:Z=$Kk MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTPD2QQ_rymy', 'tuple=FV'];
T['VCVTPD2QQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VCVTPD2QQ_rzmz', '        '];
T['VCVTPD2QQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTPD2QQ_rzmz', '        '];
T['VCVTPD2QQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTPD2QQ_rzmz', 'tuple=FV'];

# VCVTPD2UDQ-Convert Packed Double-Precision Floating-Point Values to Packed Unsigned Doubleword Integers . 5-27.
T['VCVTPD2UDQ VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTPD2UDQ_romo', '        '];
T['VCVTPD2UDQ VREG:w:u32=$XMMr PREG:r:Z=$Kk MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTPD2UDQ_romo', 'tuple=FV'];
T['VCVTPD2UDQ VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTPD2UDQ_romy', '        '];
T['VCVTPD2UDQ VREG:w:u32=$XMMr PREG:r:Z=$Kk MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTPD2UDQ_romy', 'tuple=FV'];
T['VCVTPD2UDQ VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VCVTPD2UDQ_rymz', '        '];
T['VCVTPD2UDQ VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTPD2UDQ_rymz', '        '];
T['VCVTPD2UDQ VREG:w:u32=$YMMr PREG:r:Z=$Kk MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTPD2UDQ_rymz', 'tuple=FV'];

# VCVTPD2UQQ-Convert Packed Double-Precision Floating-Point Values to Packed Unsigned Quadword Integers.
T['VCVTPD2UQQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTPD2UQQ_romo', '        '];
T['VCVTPD2UQQ VREG:w:u64=$XMMr PREG:r:Z=$Kk MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTPD2UQQ_romo', 'tuple=FV'];
T['VCVTPD2UQQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTPD2UQQ_rymy', '        '];
T['VCVTPD2UQQ VREG:w:u64=$YMMr PREG:r:Z=$Kk MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTPD2UQQ_rymy', 'tuple=FV'];
T['VCVTPD2UQQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VCVTPD2UQQ_rzmz', '        '];
T['VCVTPD2UQQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTPD2UQQ_rzmz', '        '];
T['VCVTPD2UQQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTPD2UQQ_rzmz', 'tuple=FV'];

# VCVTPH2PS-Convert 16-bit FP values to Single-Precision FP values.
T['VCVTPH2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f16=$XMMm         ', 'MOD=REG             ', 'VCVTPH2PS_romx', '         '];
T['VCVTPH2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk MEM:r:f16=$mem64         ', 'MOD=MEM             ', 'VCVTPH2PS_romx', 'tuple=HVM'];
T['VCVTPH2PS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:f16=$XMMm         ', 'MOD=REG             ', 'VCVTPH2PS_rymo', '         '];
T['VCVTPH2PS VREG:w:f32=$YMMr PREG:r:Z=$Kk MEM:r:f16=$mem128        ', 'MOD=MEM             ', 'VCVTPH2PS_rymo', 'tuple=HVM'];
T['VCVTPH2PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f16=$YMMm SAE=$SAE', 'BP=1 MOD=REG RVL=512', 'VCVTPH2PS_rzmy', '         '];
T['VCVTPH2PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f16=$YMMm         ', 'BP=0 MOD=REG VL=512 ', 'VCVTPH2PS_rzmy', '         '];
T['VCVTPH2PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk MEM:r:f16=$mem256        ', 'BP=0 MOD=MEM VL=512 ', 'VCVTPH2PS_rzmy', 'tuple=HVM'];

# VCVTPS2PH-Convert Single-Precision FP value to 16-bit FP value.
T['VCVTPS2PH VREG:w:f16=$XMMm  PREG:r:Z=$Kk VREG:r:f32=$XMMr          RNDC:r:u8=$rndc3', 'NONE        ', 'VCVTPS2PH_moroub  ', '         '];
T['VCVTPS2PH VREG:w:f16=$XMMm  PREG:r:Z=$Kk VREG:r:f32=$YMMr          RNDC:r:u8=$rndc3', 'NONE        ', 'VCVTPS2PH_moryub_1', '         '];
T['VCVTPS2PH MEM:w:f16=$mem128 PREG:r:Z=$Kk VREG:r:f32=$YMMr          RNDC:r:u8=$rndc3', 'NONE        ', 'VCVTPS2PH_moryub_2', 'tuple=HVM'];
T['VCVTPS2PH MEM:w:f16=$mem64  PREG:r:Z=$Kk VREG:r:f32=$XMMr          RNDC:r:u8=$rndc3', 'NONE        ', 'VCVTPS2PH_mqroub  ', 'tuple=HVM'];
T['VCVTPS2PH VREG:w:f16=$YMMm  PREG:r:Z=$Kk VREG:r:f32=$ZMMr SAE=$SAE RNDC:r:u8=$rndc3', 'BP=1 RVL=512', 'VCVTPS2PH_myrzub_1', '         '];
T['VCVTPS2PH VREG:w:f16=$YMMm  PREG:r:Z=$Kk VREG:r:f32=$ZMMr          RNDC:r:u8=$rndc3', 'BP=0 VL=512 ', 'VCVTPS2PH_myrzub_1', '         '];
T['VCVTPS2PH MEM:w:f16=$mem256 PREG:r:Z=$Kk VREG:r:f32=$ZMMr          RNDC:r:u8=$rndc3', 'NONE        ', 'VCVTPS2PH_myrzub_2', 'tuple=HVM'];

# VCVTPS2QQ-Convert Packed Single Precision Floating-Point Values to Packed Signed Quadword Integer Values . . 5-43.
T['VCVTPS2QQ VREG:w:s64=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTPS2QQ_romx', '        '];
T['VCVTPS2QQ VREG:w:s64=$XMMr PREG:r:Z=$Kk MEM:r:f32=$mem64 BCST=$BCST ', 'MOD=MEM             ', 'VCVTPS2QQ_romx', 'tuple=HV'];
T['VCVTPS2QQ VREG:w:s64=$YMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTPS2QQ_rymo', '        '];
T['VCVTPS2QQ VREG:w:s64=$YMMr PREG:r:Z=$Kk MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTPS2QQ_rymo', 'tuple=HV'];
T['VCVTPS2QQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VCVTPS2QQ_rzmy', '        '];
T['VCVTPS2QQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTPS2QQ_rzmy', '        '];
T['VCVTPS2QQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTPS2QQ_rzmy', 'tuple=HV'];

# VCVTPS2UDQ-Convert Packed Single-Precision Floating-Point Values to Packed Unsigned Doubleword Integer Values.
T['VCVTPS2UDQ VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTPS2UDQ_romo', '        '];
T['VCVTPS2UDQ VREG:w:u32=$XMMr PREG:r:Z=$Kk MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTPS2UDQ_romo', 'tuple=FV'];
T['VCVTPS2UDQ VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTPS2UDQ_rymy', '        '];
T['VCVTPS2UDQ VREG:w:u32=$YMMr PREG:r:Z=$Kk MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTPS2UDQ_rymy', 'tuple=FV'];
T['VCVTPS2UDQ VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VCVTPS2UDQ_rzmz', '        '];
T['VCVTPS2UDQ VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTPS2UDQ_rzmz', '        '];
T['VCVTPS2UDQ VREG:w:u32=$ZMMr PREG:r:Z=$Kk MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTPS2UDQ_rzmz', 'tuple=FV'];

# VCVTPS2UQQ-Convert Packed Single Precision Floating-Point Values to Packed Unsigned Quadword Integer Values.
T['VCVTPS2UQQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTPS2UQQ_romx', '        '];
T['VCVTPS2UQQ VREG:w:u64=$XMMr PREG:r:Z=$Kk MEM:r:f32=$mem64 BCST=$BCST ', 'MOD=MEM             ', 'VCVTPS2UQQ_romx', 'tuple=HV'];
T['VCVTPS2UQQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTPS2UQQ_rymo', '        '];
T['VCVTPS2UQQ VREG:w:u64=$YMMr PREG:r:Z=$Kk MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTPS2UQQ_rymo', 'tuple=HV'];
T['VCVTPS2UQQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VCVTPS2UQQ_rzmy', '        '];
T['VCVTPS2UQQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTPS2UQQ_rzmy', '        '];
T['VCVTPS2UQQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTPS2UQQ_rzmy', 'tuple=HV'];

# VCVTQQ2PD-Convert Packed Quadword Integers to Packed Double-Precision Floating-Point Values.
T['VCVTQQ2PD VREG:w:s64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTQQ2PD_romo', '        '];
T['VCVTQQ2PD VREG:w:s64=$XMMr PREG:r:Z=$Kk MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTQQ2PD_romo', 'tuple=FV'];
T['VCVTQQ2PD VREG:w:s64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTQQ2PD_rymy', '        '];
T['VCVTQQ2PD VREG:w:s64=$YMMr PREG:r:Z=$Kk MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTQQ2PD_rymy', 'tuple=FV'];
T['VCVTQQ2PD VREG:w:s64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VCVTQQ2PD_rzmz', '        '];
T['VCVTQQ2PD VREG:w:s64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTQQ2PD_rzmz', '        '];
T['VCVTQQ2PD VREG:w:s64=$ZMMr PREG:r:Z=$Kk MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTQQ2PD_rzmz', 'tuple=FV'];

# VCVTQQ2PS-Convert Packed Quadword Integers to Packed Single-Precision Floating-Point Values.
T['VCVTQQ2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTQQ2PS_romo', '        '];
T['VCVTQQ2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTQQ2PS_romo', 'tuple=FV'];
T['VCVTQQ2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTQQ2PS_romy', '        '];
T['VCVTQQ2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTQQ2PS_romy', 'tuple=FV'];
T['VCVTQQ2PS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VCVTQQ2PS_rymz', '        '];
T['VCVTQQ2PS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTQQ2PS_rymz', '        '];
T['VCVTQQ2PS VREG:w:f32=$YMMr PREG:r:Z=$Kk MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTQQ2PS_rymz', 'tuple=FV'];

# VCVTSD2USI-Convert Scalar Double-Precision Floating-Point Value to Unsigned Doubleword Integer.
T['VCVTSD2USI REG:w:u32=$GPR32r VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG W=0                  ', 'VCVTSD2USI', '           '];
T['VCVTSD2USI REG:w:u32=$GPR32r VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128 W=0          ', 'VCVTSD2USI', '           '];
T['VCVTSD2USI REG:w:u32=$GPR32r MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM W=0                  ', 'VCVTSD2USI', 'tuple=T1F64'];
T['VCVTSD2USI REG:w:u64=$GPR64r VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG MODE=64 W=1          ', 'VCVTSD2USI', '           '];
T['VCVTSD2USI REG:w:u64=$GPR64r VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG MODE=64 RVL=128 W=1  ', 'VCVTSD2USI', '           '];
T['VCVTSD2USI REG:w:u64=$GPR64r MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM MODE=64 W=1          ', 'VCVTSD2USI', 'tuple=T1F64'];
T['VCVTSD2USI REG:w:u32=$GPR32r VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG MODE=NO64 W=1        ', 'VCVTSD2USI', '           '];
T['VCVTSD2USI REG:w:u32=$GPR32r VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG MODE=NO64 RVL=128 W=1', 'VCVTSD2USI', '           '];
T['VCVTSD2USI REG:w:u32=$GPR32r MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM MODE=NO64 W=1        ', 'VCVTSD2USI', 'tuple=T1F64'];

# VCVTSS2USI-Convert Scalar Single-Precision Floating-Point Value to Unsigned Doubleword Integer.
T['VCVTSS2USI REG:w:u32=$GPR32r VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG W=0                  ', 'VCVTSS2USI', '           '];
T['VCVTSS2USI REG:w:u32=$GPR32r VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128 W=0          ', 'VCVTSS2USI', '           '];
T['VCVTSS2USI REG:w:u32=$GPR32r MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM W=0                  ', 'VCVTSS2USI', 'tuple=T1F32'];
T['VCVTSS2USI REG:w:u64=$GPR64r VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG MODE=64 W=1          ', 'VCVTSS2USI', '           '];
T['VCVTSS2USI REG:w:u64=$GPR64r VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG MODE=64 RVL=128 W=1  ', 'VCVTSS2USI', '           '];
T['VCVTSS2USI REG:w:u64=$GPR64r MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM MODE=64 W=1          ', 'VCVTSS2USI', 'tuple=T1F32'];
T['VCVTSS2USI REG:w:u32=$GPR32r VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG MODE=NO64 W=1        ', 'VCVTSS2USI', '           '];
T['VCVTSS2USI REG:w:u32=$GPR32r VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG MODE=NO64 RVL=128 W=1', 'VCVTSS2USI', '           '];
T['VCVTSS2USI REG:w:u32=$GPR32r MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM MODE=NO64 W=1        ', 'VCVTSS2USI', 'tuple=T1F32'];

# VCVTTPD2QQ-Convert with Truncation Packed Double-Precision Floating-Point Values to Packed Quadword Integers.
T['VCVTTPD2QQ VREG:w:s64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTTPD2QQ_romo', '        '];
T['VCVTTPD2QQ VREG:w:s64=$XMMr PREG:r:Z=$Kk MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTTPD2QQ_romo', 'tuple=FV'];
T['VCVTTPD2QQ VREG:w:s64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTTPD2QQ_rymy', '        '];
T['VCVTTPD2QQ VREG:w:s64=$YMMr PREG:r:Z=$Kk MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTTPD2QQ_rymy', 'tuple=FV'];
T['VCVTTPD2QQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VCVTTPD2QQ_rzmz', '        '];
T['VCVTTPD2QQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTTPD2QQ_rzmz', '        '];
T['VCVTTPD2QQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTTPD2QQ_rzmz', 'tuple=FV'];

# VCVTTPD2UDQ-Convert with Truncation Packed Double-Precision Floating-Point Values to Packed Unsigned Doubleword Integers.
T['VCVTTPD2UDQ VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTTPD2UDQ_romo', '        '];
T['VCVTTPD2UDQ VREG:w:u32=$XMMr PREG:r:Z=$Kk MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTTPD2UDQ_romo', 'tuple=FV'];
T['VCVTTPD2UDQ VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTTPD2UDQ_romy', '        '];
T['VCVTTPD2UDQ VREG:w:u32=$XMMr PREG:r:Z=$Kk MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTTPD2UDQ_romy', 'tuple=FV'];
T['VCVTTPD2UDQ VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VCVTTPD2UDQ_rymz', '        '];
T['VCVTTPD2UDQ VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTTPD2UDQ_rymz', '        '];
T['VCVTTPD2UDQ VREG:w:u32=$YMMr PREG:r:Z=$Kk MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTTPD2UDQ_rymz', 'tuple=FV'];

# VCVTTPD2UQQ-Convert with Truncation Packed Double-Precision Floating-Point Values to Packed Unsigned Quadword Integers.
T['VCVTTPD2UQQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTTPD2UQQ_romo', '        '];
T['VCVTTPD2UQQ VREG:w:u64=$XMMr PREG:r:Z=$Kk MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTTPD2UQQ_romo', 'tuple=FV'];
T['VCVTTPD2UQQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTTPD2UQQ_rymy', '        '];
T['VCVTTPD2UQQ VREG:w:u64=$YMMr PREG:r:Z=$Kk MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTTPD2UQQ_rymy', 'tuple=FV'];
T['VCVTTPD2UQQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VCVTTPD2UQQ_rzmz', '        '];
T['VCVTTPD2UQQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTTPD2UQQ_rzmz', '        '];
T['VCVTTPD2UQQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTTPD2UQQ_rzmz', 'tuple=FV'];

# VCVTTPS2QQ-Convert with Truncation Packed Single Precision Floating-Point Values to Packed Signed Quadword Integer Values.
T['VCVTTPS2QQ VREG:w:s64=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTTPS2QQ_romx', '        '];
T['VCVTTPS2QQ VREG:w:s64=$XMMr PREG:r:Z=$Kk MEM:r:f32=$mem64 BCST=$BCST ', 'MOD=MEM             ', 'VCVTTPS2QQ_romx', 'tuple=HV'];
T['VCVTTPS2QQ VREG:w:s64=$YMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTTPS2QQ_rymo', '        '];
T['VCVTTPS2QQ VREG:w:s64=$YMMr PREG:r:Z=$Kk MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTTPS2QQ_rymo', 'tuple=HV'];
T['VCVTTPS2QQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VCVTTPS2QQ_rzmy', '        '];
T['VCVTTPS2QQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTTPS2QQ_rzmy', '        '];
T['VCVTTPS2QQ VREG:w:s64=$ZMMr PREG:r:Z=$Kk MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTTPS2QQ_rzmy', 'tuple=HV'];

# VCVTTPS2UDQ-Convert with Truncation Packed Single-Precision Floating-Point Values to Packed Unsigned Doubleword Integer Values.
T['VCVTTPS2UDQ VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTTPS2UDQ_romo', '        '];
T['VCVTTPS2UDQ VREG:w:u32=$XMMr PREG:r:Z=$Kk MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTTPS2UDQ_romo', 'tuple=FV'];
T['VCVTTPS2UDQ VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTTPS2UDQ_rymy', '        '];
T['VCVTTPS2UDQ VREG:w:u32=$YMMr PREG:r:Z=$Kk MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTTPS2UDQ_rymy', 'tuple=FV'];
T['VCVTTPS2UDQ VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VCVTTPS2UDQ_rzmz', '        '];
T['VCVTTPS2UDQ VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTTPS2UDQ_rzmz', '        '];
T['VCVTTPS2UDQ VREG:w:u32=$ZMMr PREG:r:Z=$Kk MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTTPS2UDQ_rzmz', 'tuple=FV'];

# VCVTTPS2UQQ-Convert with Truncation Packed Single Precision Floating-Point Values to Packed Unsigned Quadword Integer Values.
T['VCVTTPS2UQQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTTPS2UQQ_romx', '        '];
T['VCVTTPS2UQQ VREG:w:u64=$XMMr PREG:r:Z=$Kk MEM:r:f32=$mem64 BCST=$BCST ', 'MOD=MEM             ', 'VCVTTPS2UQQ_romx', 'tuple=HV'];
T['VCVTTPS2UQQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTTPS2UQQ_rymo', '        '];
T['VCVTTPS2UQQ VREG:w:u64=$YMMr PREG:r:Z=$Kk MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTTPS2UQQ_rymo', 'tuple=HV'];
T['VCVTTPS2UQQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VCVTTPS2UQQ_rzmy', '        '];
T['VCVTTPS2UQQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTTPS2UQQ_rzmy', '        '];
T['VCVTTPS2UQQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTTPS2UQQ_rzmy', 'tuple=HV'];

# VCVTTSD2USI-Convert with Truncation Scalar Double-Precision Floating-Point Value to Unsigned Integer.
T['VCVTTSD2USI REG:w:u32=$GPR32r VREG:r:f64=$XMMm         ', 'BP=0 MOD=REG W=0                  ', 'VCVTTSD2USI', '           '];
T['VCVTTSD2USI REG:w:u32=$GPR32r VREG:r:f64=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128 W=0          ', 'VCVTTSD2USI', '           '];
T['VCVTTSD2USI REG:w:u32=$GPR32r MEM:r:f64=$mem64         ', 'BP=0 MOD=MEM W=0                  ', 'VCVTTSD2USI', 'tuple=T1F64'];
T['VCVTTSD2USI REG:w:u64=$GPR64r VREG:r:f64=$XMMm         ', 'BP=0 MOD=REG MODE=64 W=1          ', 'VCVTTSD2USI', '           '];
T['VCVTTSD2USI REG:w:u64=$GPR64r VREG:r:f64=$XMMm SAE=$SAE', 'BP=1 MOD=REG MODE=64 RVL=128 W=1  ', 'VCVTTSD2USI', '           '];
T['VCVTTSD2USI REG:w:u64=$GPR64r MEM:r:f64=$mem64         ', 'BP=0 MOD=MEM MODE=64 W=1          ', 'VCVTTSD2USI', 'tuple=T1F64'];
T['VCVTTSD2USI REG:w:u32=$GPR32r VREG:r:f64=$XMMm         ', 'BP=0 MOD=REG MODE=NO64 W=1        ', 'VCVTTSD2USI', '           '];
T['VCVTTSD2USI REG:w:u32=$GPR32r VREG:r:f64=$XMMm SAE=$SAE', 'BP=1 MOD=REG MODE=NO64 RVL=128 W=1', 'VCVTTSD2USI', '           '];
T['VCVTTSD2USI REG:w:u32=$GPR32r MEM:r:f64=$mem64         ', 'BP=0 MOD=MEM MODE=NO64 W=1        ', 'VCVTTSD2USI', 'tuple=T1F64'];

# VCVTTSS2USI-Convert with Truncation Scalar Single-Precision Floating-Point Value to Unsigned Integer.
T['VCVTTSS2USI REG:w:u32=$GPR32r VREG:r:f32=$XMMm         ', 'BP=0 MOD=REG W=0                  ', 'VCVTTSS2USI', '           '];
T['VCVTTSS2USI REG:w:u32=$GPR32r VREG:r:f32=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128 W=0          ', 'VCVTTSS2USI', '           '];
T['VCVTTSS2USI REG:w:u32=$GPR32r MEM:r:f32=$mem32         ', 'BP=0 MOD=MEM W=0                  ', 'VCVTTSS2USI', 'tuple=T1F32'];
T['VCVTTSS2USI REG:w:u64=$GPR64r VREG:r:f32=$XMMm         ', 'BP=0 MOD=REG MODE=64 W=1          ', 'VCVTTSS2USI', '           '];
T['VCVTTSS2USI REG:w:u64=$GPR64r VREG:r:f32=$XMMm SAE=$SAE', 'BP=1 MOD=REG MODE=64 RVL=128 W=1  ', 'VCVTTSS2USI', '           '];
T['VCVTTSS2USI REG:w:u64=$GPR64r MEM:r:f32=$mem32         ', 'BP=0 MOD=MEM MODE=64 W=1          ', 'VCVTTSS2USI', 'tuple=T1F32'];
T['VCVTTSS2USI REG:w:u32=$GPR32r VREG:r:f32=$XMMm         ', 'BP=0 MOD=REG MODE=NO64 W=1        ', 'VCVTTSS2USI', '           '];
T['VCVTTSS2USI REG:w:u32=$GPR32r VREG:r:f32=$XMMm SAE=$SAE', 'BP=1 MOD=REG MODE=NO64 RVL=128 W=1', 'VCVTTSS2USI', '           '];
T['VCVTTSS2USI REG:w:u32=$GPR32r MEM:r:f32=$mem32         ', 'BP=0 MOD=MEM MODE=NO64 W=1        ', 'VCVTTSS2USI', 'tuple=T1F32'];

# VCVTUDQ2PD-Convert Packed Unsigned Doubleword Integers to Packed Double-Precision Floating-Point Values . 5-72.
T['VCVTUDQ2PD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VCVTUDQ2PD_romx', '        '];
T['VCVTUDQ2PD VREG:w:f64=$XMMr PREG:r:Z=$Kk MEM:r:u32=$mem64 BCST=$BCST ', 'MOD=MEM     ', 'VCVTUDQ2PD_romx', 'tuple=HV'];
T['VCVTUDQ2PD VREG:w:f64=$YMMr PREG:r:Z=$Kk VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VCVTUDQ2PD_rymo', '        '];
T['VCVTUDQ2PD VREG:w:f64=$YMMr PREG:r:Z=$Kk MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VCVTUDQ2PD_rymo', 'tuple=HV'];
T['VCVTUDQ2PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VCVTUDQ2PD_rzmy', '        '];
T['VCVTUDQ2PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VCVTUDQ2PD_rzmy', 'tuple=HV'];

# VCVTUDQ2PS-Convert Packed Unsigned Doubleword Integers to Packed Single-Precision Floating-Point Values . . 5-74.
T['VCVTUDQ2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTUDQ2PS_romo', '        '];
T['VCVTUDQ2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTUDQ2PS_romo', 'tuple=FV'];
T['VCVTUDQ2PS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTUDQ2PS_rymy', '        '];
T['VCVTUDQ2PS VREG:w:f32=$YMMr PREG:r:Z=$Kk MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTUDQ2PS_rymy', 'tuple=FV'];
T['VCVTUDQ2PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VCVTUDQ2PS_rzmz', '        '];
T['VCVTUDQ2PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTUDQ2PS_rzmz', '        '];
T['VCVTUDQ2PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTUDQ2PS_rzmz', 'tuple=FV'];

# VCVTUQQ2PD-Convert Packed Unsigned Quadword Integers to Packed Double-Precision Floating-Point Values.
T['VCVTUQQ2PD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTUQQ2PD_romo', '        '];
T['VCVTUQQ2PD VREG:w:f64=$XMMr PREG:r:Z=$Kk MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTUQQ2PD_romo', 'tuple=FV'];
T['VCVTUQQ2PD VREG:w:f64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTUQQ2PD_rymy', '        '];
T['VCVTUQQ2PD VREG:w:f64=$YMMr PREG:r:Z=$Kk MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTUQQ2PD_rymy', 'tuple=FV'];
T['VCVTUQQ2PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VCVTUQQ2PD_rzmz', '        '];
T['VCVTUQQ2PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTUQQ2PD_rzmz', '        '];
T['VCVTUQQ2PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTUQQ2PD_rzmz', 'tuple=FV'];

# VCVTUQQ2PS-Convert Packed Unsigned Quadword Integers to Packed Single-Precision Floating-Point Values.
T['VCVTUQQ2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG        ', 'VCVTUQQ2PS_romo', '        '];
T['VCVTUQQ2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VCVTUQQ2PS_romo', 'tuple=FV'];
T['VCVTUQQ2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG        ', 'VCVTUQQ2PS_romy', '        '];
T['VCVTUQQ2PS VREG:w:f32=$XMMr PREG:r:Z=$Kk MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VCVTUQQ2PS_romy', 'tuple=FV'];
T['VCVTUQQ2PS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VCVTUQQ2PS_rymz', '        '];
T['VCVTUQQ2PS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VCVTUQQ2PS_rymz', '        '];
T['VCVTUQQ2PS VREG:w:f32=$YMMr PREG:r:Z=$Kk MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VCVTUQQ2PS_rymz', 'tuple=FV'];

# VCVTUSI2SD-Convert Unsigned Integer to Scalar Double-Precision Floating-Point Value.
T['VCVTUSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        REG:r:u32=$GPR32m', 'MOD=REG W=0          ', 'VCVTUSI2SD_ronomd', '         '];
T['VCVTUSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        MEM:r:u32=$mem32 ', 'MOD=MEM W=0          ', 'VCVTUSI2SD_ronomd', 'tuple=T1S'];
T['VCVTUSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        REG:r:u32=$GPR32m', 'MOD=REG MODE=NO64 W=1', 'VCVTUSI2SD_ronomd', '         '];
T['VCVTUSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        MEM:r:u32=$mem32 ', 'MOD=MEM MODE=NO64 W=1', 'VCVTUSI2SD_ronomd', 'tuple=T1S'];
T['VCVTUSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        REG:r:u64=$GPR64m', 'BP=0 MOD=REG         ', 'VCVTUSI2SD_ronomq', '         '];
T['VCVTUSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn RC=$RC REG:r:u64=$GPR64m', 'BP=1 MOD=REG RVL=128 ', 'VCVTUSI2SD_ronomq', '         '];
T['VCVTUSI2SD VREG:w:f64=$XMMr VREG:r:f64=$XMMn        MEM:r:u64=$mem64 ', 'BP=0 MOD=MEM         ', 'VCVTUSI2SD_ronomq', 'tuple=T1S'];

# VCVTUSI2SS-Convert Unsigned Integer to Scalar Single-Precision Floating-Point Value.
T['VCVTUSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        REG:r:u32=$GPR32m', 'BP=0 MOD=REG W=0                  ', 'VCVTUSI2SS', '         '];
T['VCVTUSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn RC=$RC REG:r:u32=$GPR32m', 'BP=1 MOD=REG RVL=128 W=0          ', 'VCVTUSI2SS', '         '];
T['VCVTUSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        MEM:r:u32=$mem32 ', 'BP=0 MOD=MEM W=0                  ', 'VCVTUSI2SS', 'tuple=T1S'];
T['VCVTUSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        REG:r:u64=$GPR64m', 'BP=0 MOD=REG MODE=64 W=1          ', 'VCVTUSI2SS', '         '];
T['VCVTUSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn RC=$RC REG:r:u64=$GPR64m', 'BP=1 MOD=REG MODE=64 RVL=128 W=1  ', 'VCVTUSI2SS', '         '];
T['VCVTUSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        MEM:r:u64=$mem64 ', 'BP=0 MOD=MEM MODE=64 W=1          ', 'VCVTUSI2SS', 'tuple=T1S'];
T['VCVTUSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        REG:r:u32=$GPR32m', 'BP=0 MOD=REG MODE=NO64 W=1        ', 'VCVTUSI2SS', '         '];
T['VCVTUSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn RC=$RC REG:r:u32=$GPR32m', 'BP=1 MOD=REG MODE=NO64 RVL=128 W=1', 'VCVTUSI2SS', '         '];
T['VCVTUSI2SS VREG:w:f32=$XMMr VREG:r:f32=$XMMn        MEM:r:u32=$mem32 ', 'BP=0 MOD=MEM MODE=NO64 W=1        ', 'VCVTUSI2SS', 'tuple=T1S'];

# VDBPSADBW-Double Block Packed Sum-Absolute-Differences (SAD) on Unsigned Bytes.
T['VDBPSADBW VREG:w:u16=$XMMr PREG:r:Z=$Kk VREG:r:u8=$XMMn VREG:r:u8=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'VDBPSADBW_ronomoub', '         '];
T['VDBPSADBW VREG:w:u16=$XMMr PREG:r:Z=$Kk VREG:r:u8=$XMMn MEM:r:u8=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'VDBPSADBW_ronomoub', 'tuple=FVM'];
T['VDBPSADBW VREG:w:u16=$YMMr PREG:r:Z=$Kk VREG:r:u8=$YMMn VREG:r:u8=$YMMm  IMM:u8=$uimm8', 'MOD=REG', 'VDBPSADBW_rynymyub', '         '];
T['VDBPSADBW VREG:w:u16=$YMMr PREG:r:Z=$Kk VREG:r:u8=$YMMn MEM:r:u8=$mem256 IMM:u8=$uimm8', 'MOD=MEM', 'VDBPSADBW_rynymyub', 'tuple=FVM'];
T['VDBPSADBW VREG:w:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u8=$ZMMn VREG:r:u8=$ZMMm  IMM:u8=$uimm8', 'MOD=REG', 'VDBPSADBW_rznzmzub', '         '];
T['VDBPSADBW VREG:w:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u8=$ZMMn MEM:r:u8=$mem512 IMM:u8=$uimm8', 'MOD=MEM', 'VDBPSADBW_rznzmzub', 'tuple=FVM'];

# VDPBF16PS.
T['VDPBF16PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VDPBF16PS_ronomo', '        '];
T['VDPBF16PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VDPBF16PS_ronomo', 'tuple=FV'];
T['VDPBF16PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VDPBF16PS_rynymy', '        '];
T['VDPBF16PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VDPBF16PS_rynymy', 'tuple=FV'];
T['VDPBF16PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VDPBF16PS_rznzmz', '        '];
T['VDPBF16PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VDPBF16PS_rznzmz', 'tuple=FV'];

# VEXPANDPD-Load Sparse Packed Double-Precision Floating-Point Values from Dense Memory.
T['VEXPANDPD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMm ', 'MOD=REG', 'VEXPANDPD_romo', '         '];
T['VEXPANDPD VREG:w:f64=$XMMr PREG:r:Z=$Kk MEM:r:f64=$mem128', 'MOD=MEM', 'VEXPANDPD_romo', 'tuple=T1S'];
T['VEXPANDPD VREG:w:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMm ', 'MOD=REG', 'VEXPANDPD_rymy', '         '];
T['VEXPANDPD VREG:w:f64=$YMMr PREG:r:Z=$Kk MEM:r:f64=$mem256', 'MOD=MEM', 'VEXPANDPD_rymy', 'tuple=T1S'];
T['VEXPANDPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm ', 'MOD=REG', 'VEXPANDPD_rzmz', '         '];
T['VEXPANDPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk MEM:r:f64=$mem512', 'MOD=MEM', 'VEXPANDPD_rzmz', 'tuple=T1S'];

# VEXPANDPS-Load Sparse Packed Single-Precision Floating-Point Values from Dense Memory.
T['VEXPANDPS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm ', 'MOD=REG', 'VEXPANDPS_romo', '         '];
T['VEXPANDPS VREG:w:f32=$XMMr PREG:r:Z=$Kk MEM:r:f32=$mem128', 'MOD=MEM', 'VEXPANDPS_romo', 'tuple=T1S'];
T['VEXPANDPS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm ', 'MOD=REG', 'VEXPANDPS_rymy', '         '];
T['VEXPANDPS VREG:w:f32=$YMMr PREG:r:Z=$Kk MEM:r:f32=$mem256', 'MOD=MEM', 'VEXPANDPS_rymy', 'tuple=T1S'];
T['VEXPANDPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm ', 'MOD=REG', 'VEXPANDPS_rzmz', '         '];
T['VEXPANDPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk MEM:r:f32=$mem512', 'MOD=MEM', 'VEXPANDPS_rzmz', 'tuple=T1S'];

# VEXTRACTF128/VEXTRACTF32x4/VEXTRACTF64x2/VEXTRACTF32x8/VEXTRACTF64x4-Extract Packed Floating-Point Values.
T['VEXTRACTF128  VREG:w:f64=$XMMm  VREG:r:f64=$YMMr IMM:u8=$index                 ', 'MOD=REG', 'VEXTRACTF128        ', '        '];
T['VEXTRACTF128  MEM:w:f64=$mem128 VREG:r:f64=$YMMr IMM:u8=$index                 ', 'MOD=MEM', 'VEXTRACTF128        ', '        '];
T['VEXTRACTF32X4 VREG:w:f32=$XMMm  PREG:r:Z=$Kk     VREG:r:f32=$YMMr IMM:u8=$index', 'MOD=REG', 'VEXTRACTF32X4_moryub', '        '];
T['VEXTRACTF32X4 MEM:w:f32=$mem128 PREG:r:Z=$Kk     VREG:r:f32=$YMMr IMM:u8=$index', 'MOD=MEM', 'VEXTRACTF32X4_moryub', 'tuple=T4'];
T['VEXTRACTF32X4 VREG:w:f32=$XMMm  PREG:r:Z=$Kk     VREG:r:f32=$ZMMr IMM:u8=$index', 'MOD=REG', 'VEXTRACTF32X4_morzub', '        '];
T['VEXTRACTF32X4 MEM:w:f32=$mem128 PREG:r:Z=$Kk     VREG:r:f32=$ZMMr IMM:u8=$index', 'MOD=MEM', 'VEXTRACTF32X4_morzub', 'tuple=T4'];
T['VEXTRACTF32X8 VREG:w:f32=$YMMm  PREG:r:Z=$Kk     VREG:r:f32=$ZMMr IMM:u8=$index', 'MOD=REG', 'VEXTRACTF32X8       ', '        '];
T['VEXTRACTF32X8 MEM:w:f32=$mem256 PREG:r:Z=$Kk     VREG:r:f32=$ZMMr IMM:u8=$index', 'MOD=MEM', 'VEXTRACTF32X8       ', 'tuple=T8'];
T['VEXTRACTF64X2 VREG:w:f64=$XMMm  PREG:r:Z=$Kk     VREG:r:f64=$YMMr IMM:u8=$index', 'MOD=REG', 'VEXTRACTF64X2_moryub', '        '];
T['VEXTRACTF64X2 MEM:w:f64=$mem128 PREG:r:Z=$Kk     VREG:r:f64=$YMMr IMM:u8=$index', 'MOD=MEM', 'VEXTRACTF64X2_moryub', 'tuple=T2'];
T['VEXTRACTF64X2 VREG:w:f64=$XMMm  PREG:r:Z=$Kk     VREG:r:f64=$ZMMr IMM:u8=$index', 'MOD=REG', 'VEXTRACTF64X2_morzub', '        '];
T['VEXTRACTF64X2 MEM:w:f64=$mem128 PREG:r:Z=$Kk     VREG:r:f64=$ZMMr IMM:u8=$index', 'MOD=MEM', 'VEXTRACTF64X2_morzub', 'tuple=T2'];
T['VEXTRACTF64X4 VREG:w:f64=$YMMm  PREG:r:Z=$Kk     VREG:r:f64=$ZMMr IMM:u8=$index', 'MOD=REG', 'VEXTRACTF64X4       ', '        '];
T['VEXTRACTF64X4 MEM:w:f64=$mem256 PREG:r:Z=$Kk     VREG:r:f64=$ZMMr IMM:u8=$index', 'MOD=MEM', 'VEXTRACTF64X4       ', 'tuple=T4'];

# VEXTRACTI128/VEXTRACTI32x4/VEXTRACTI64x2/VEXTRACTI32x8/VEXTRACTI64x4-Extract packed Integer Values.
T['VEXTRACTI128  REG:w:u128=$XMMm   VREG:r:u128=$YMMr IMM:u8=$index                 ', 'MOD=REG', 'VEXTRACTI128        ', '        '];
T['VEXTRACTI128  MEM:w:u128=$mem128 VREG:r:u128=$YMMr IMM:u8=$index                 ', 'MOD=MEM', 'VEXTRACTI128        ', '        '];
T['VEXTRACTI32X4 VREG:w:u32=$XMMm   PREG:r:Z=$Kk      VREG:r:u32=$YMMr IMM:u8=$index', 'MOD=REG', 'VEXTRACTI32X4_moryub', '        '];
T['VEXTRACTI32X4 MEM:w:u32=$mem128  PREG:r:Z=$Kk      VREG:r:u32=$YMMr IMM:u8=$index', 'MOD=MEM', 'VEXTRACTI32X4_moryub', 'tuple=T4'];
T['VEXTRACTI32X4 VREG:w:u32=$XMMm   PREG:r:Z=$Kk      VREG:r:u32=$ZMMr IMM:u8=$index', 'MOD=REG', 'VEXTRACTI32X4_morzub', '        '];
T['VEXTRACTI32X4 MEM:w:u32=$mem128  PREG:r:Z=$Kk      VREG:r:u32=$ZMMr IMM:u8=$index', 'MOD=MEM', 'VEXTRACTI32X4_morzub', 'tuple=T4'];
T['VEXTRACTI32X8 VREG:w:u32=$YMMm   PREG:r:Z=$Kk      VREG:r:u32=$ZMMr IMM:u8=$index', 'MOD=REG', 'VEXTRACTI32X8       ', '        '];
T['VEXTRACTI32X8 MEM:w:u32=$mem256  PREG:r:Z=$Kk      VREG:r:u32=$ZMMr IMM:u8=$index', 'MOD=MEM', 'VEXTRACTI32X8       ', 'tuple=T8'];
T['VEXTRACTI64X2 VREG:w:u64=$XMMm   PREG:r:Z=$Kk      VREG:r:u64=$YMMr IMM:u8=$index', 'MOD=REG', 'VEXTRACTI64X2_moryub', '        '];
T['VEXTRACTI64X2 MEM:w:u64=$mem128  PREG:r:Z=$Kk      VREG:r:u64=$YMMr IMM:u8=$index', 'MOD=MEM', 'VEXTRACTI64X2_moryub', 'tuple=T2'];
T['VEXTRACTI64X2 VREG:w:u64=$XMMm   PREG:r:Z=$Kk      VREG:r:u64=$ZMMr IMM:u8=$index', 'MOD=REG', 'VEXTRACTI64X2_morzub', '        '];
T['VEXTRACTI64X2 MEM:w:u64=$mem128  PREG:r:Z=$Kk      VREG:r:u64=$ZMMr IMM:u8=$index', 'MOD=MEM', 'VEXTRACTI64X2_morzub', 'tuple=T2'];
T['VEXTRACTI64X4 VREG:w:u64=$YMMm   PREG:r:Z=$Kk      VREG:r:u64=$ZMMr IMM:u8=$index', 'MOD=REG', 'VEXTRACTI64X4       ', '        '];
T['VEXTRACTI64X4 MEM:w:u64=$mem256  PREG:r:Z=$Kk      VREG:r:u64=$ZMMr IMM:u8=$index', 'MOD=MEM', 'VEXTRACTI64X4       ', 'tuple=T4'];

# VFIXUPIMMPD-Fix Up Special Packed Float64 Values.
T['VFIXUPIMMPD VREG:rw:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm             FEXCPC:r:u8=$fexcpc', 'BP=0 MOD=REG        ', 'VFIXUPIMMPD_ronomoub', '        '];
T['VFIXUPIMMPD VREG:rw:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn MEM:r:f64=$mem128 BCST=$BCST FEXCPC:r:u8=$fexcpc', 'MOD=MEM             ', 'VFIXUPIMMPD_ronomoub', 'tuple=FV'];
T['VFIXUPIMMPD VREG:rw:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMn VREG:r:f64=$YMMm             FEXCPC:r:u8=$fexcpc', 'BP=0 MOD=REG        ', 'VFIXUPIMMPD_rynymyub', '        '];
T['VFIXUPIMMPD VREG:rw:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMn MEM:r:f64=$mem256 BCST=$BCST FEXCPC:r:u8=$fexcpc', 'MOD=MEM             ', 'VFIXUPIMMPD_rynymyub', 'tuple=FV'];
T['VFIXUPIMMPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SAE=$SAE    FEXCPC:r:u8=$fexcpc', 'BP=1 MOD=REG RVL=512', 'VFIXUPIMMPD_rznzmzub', '        '];
T['VFIXUPIMMPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm             FEXCPC:r:u8=$fexcpc', 'BP=0 MOD=REG VL=512 ', 'VFIXUPIMMPD_rznzmzub', '        '];
T['VFIXUPIMMPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r:f64=$mem512 BCST=$BCST FEXCPC:r:u8=$fexcpc', 'MOD=MEM VL=512      ', 'VFIXUPIMMPD_rznzmzub', 'tuple=FV'];

# VFIXUPIMMPS-Fix Up Special Packed Float32 Values.
T['VFIXUPIMMPS VREG:rw:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm             FEXCPC:r:u8=$fexcpc', 'BP=0 MOD=REG        ', 'VFIXUPIMMPS_ronomoub', '        '];
T['VFIXUPIMMPS VREG:rw:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem128 BCST=$BCST FEXCPC:r:u8=$fexcpc', 'MOD=MEM             ', 'VFIXUPIMMPS_ronomoub', 'tuple=FV'];
T['VFIXUPIMMPS VREG:rw:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMn VREG:r:f32=$YMMm             FEXCPC:r:u8=$fexcpc', 'BP=0 MOD=REG        ', 'VFIXUPIMMPS_rynymyub', '        '];
T['VFIXUPIMMPS VREG:rw:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMn MEM:r:f32=$mem256 BCST=$BCST FEXCPC:r:u8=$fexcpc', 'MOD=MEM             ', 'VFIXUPIMMPS_rynymyub', 'tuple=FV'];
T['VFIXUPIMMPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SAE=$SAE    FEXCPC:r:u8=$fexcpc', 'BP=1 MOD=REG RVL=512', 'VFIXUPIMMPS_rznzmzub', '        '];
T['VFIXUPIMMPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm             FEXCPC:r:u8=$fexcpc', 'BP=0 MOD=REG VL=512 ', 'VFIXUPIMMPS_rznzmzub', '        '];
T['VFIXUPIMMPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r:f32=$mem512 BCST=$BCST FEXCPC:r:u8=$fexcpc', 'MOD=MEM VL=512      ', 'VFIXUPIMMPS_rznzmzub', 'tuple=FV'];

# VFIXUPIMMSD-Fix Up Special Scalar Float64 Value.
T['VFIXUPIMMSD VREG:rw:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm          FEXCPC:r:u8=$fexcpc', 'BP=0 MOD=REG        ', 'VFIXUPIMMSD', '         '];
T['VFIXUPIMMSD VREG:rw:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm SAE=$SAE FEXCPC:r:u8=$fexcpc', 'BP=1 MOD=REG RVL=128', 'VFIXUPIMMSD', '         '];
T['VFIXUPIMMSD VREG:rw:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn MEM:r:f64=$mem64          FEXCPC:r:u8=$fexcpc', 'BP=0 MOD=MEM        ', 'VFIXUPIMMSD', 'tuple=T1S'];

# VFIXUPIMMSS-Fix Up Special Scalar Float32 Value.
T['VFIXUPIMMSS VREG:rw:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm          FEXCPC:r:u8=$fexcpc', 'BP=0 MOD=REG        ', 'VFIXUPIMMSS', '         '];
T['VFIXUPIMMSS VREG:rw:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm SAE=$SAE FEXCPC:r:u8=$fexcpc', 'BP=1 MOD=REG RVL=128', 'VFIXUPIMMSS', '         '];
T['VFIXUPIMMSS VREG:rw:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem32          FEXCPC:r:u8=$fexcpc', 'BP=0 MOD=MEM        ', 'VFIXUPIMMSS', 'tuple=T1S'];

# VFMADDSUB132PD/VFMADDSUB213PD/VFMADDSUB231PD-Fused Multiply-Alternating Add/Subtract of Packed Double-Precision Floating-Point Values.
T['VFMADDSUB132PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMADDSUB132PD_ronomo', '        '];
T['VFMADDSUB132PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMADDSUB132PD_ronomo', 'tuple=FV'];
T['VFMADDSUB132PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMADDSUB132PD_rxnxmx', '        '];
T['VFMADDSUB132PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMADDSUB132PD_rxnxmx', '        '];
T['VFMADDSUB132PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMADDSUB132PD_rxnxmx', '        '];
T['VFMADDSUB132PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMADDSUB132PD_rxnxmx', '        '];
T['VFMADDSUB132PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMADDSUB132PD_rynymy', '        '];
T['VFMADDSUB132PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMADDSUB132PD_rynymy', 'tuple=FV'];
T['VFMADDSUB132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMADDSUB132PD_rznzmz', '        '];
T['VFMADDSUB132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMADDSUB132PD_rznzmz', '        '];
T['VFMADDSUB132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMADDSUB132PD_rznzmz', 'tuple=FV'];
T['VFMADDSUB213PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMADDSUB213PD_ronomo', '        '];
T['VFMADDSUB213PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMADDSUB213PD_ronomo', 'tuple=FV'];
T['VFMADDSUB213PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMADDSUB213PD_rxnxmx', '        '];
T['VFMADDSUB213PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMADDSUB213PD_rxnxmx', '        '];
T['VFMADDSUB213PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMADDSUB213PD_rxnxmx', '        '];
T['VFMADDSUB213PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMADDSUB213PD_rxnxmx', '        '];
T['VFMADDSUB213PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMADDSUB213PD_rynymy', '        '];
T['VFMADDSUB213PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMADDSUB213PD_rynymy', 'tuple=FV'];
T['VFMADDSUB213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMADDSUB213PD_rznzmz', '        '];
T['VFMADDSUB213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMADDSUB213PD_rznzmz', '        '];
T['VFMADDSUB213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMADDSUB213PD_rznzmz', 'tuple=FV'];
T['VFMADDSUB231PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMADDSUB231PD_ronomo', '        '];
T['VFMADDSUB231PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMADDSUB231PD_ronomo', 'tuple=FV'];
T['VFMADDSUB231PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMADDSUB231PD_rxnxmx', '        '];
T['VFMADDSUB231PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMADDSUB231PD_rxnxmx', '        '];
T['VFMADDSUB231PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMADDSUB231PD_rxnxmx', '        '];
T['VFMADDSUB231PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMADDSUB231PD_rxnxmx', '        '];
T['VFMADDSUB231PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMADDSUB231PD_rynymy', '        '];
T['VFMADDSUB231PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMADDSUB231PD_rynymy', 'tuple=FV'];
T['VFMADDSUB231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMADDSUB231PD_rznzmz', '        '];
T['VFMADDSUB231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMADDSUB231PD_rznzmz', '        '];
T['VFMADDSUB231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMADDSUB231PD_rznzmz', 'tuple=FV'];

# VFMADDSUB132PS/VFMADDSUB213PS/VFMADDSUB231PS-Fused Multiply-Alternating Add/Subtract of Packed Single-Precision Floating-Point Values.
T['VFMADDSUB132PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMADDSUB132PS_ronomo', '        '];
T['VFMADDSUB132PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMADDSUB132PS_ronomo', 'tuple=FV'];
T['VFMADDSUB132PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMADDSUB132PS_rxnxmx', '        '];
T['VFMADDSUB132PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMADDSUB132PS_rxnxmx', '        '];
T['VFMADDSUB132PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMADDSUB132PS_rxnxmx', '        '];
T['VFMADDSUB132PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMADDSUB132PS_rxnxmx', '        '];
T['VFMADDSUB132PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMADDSUB132PS_rynymy', '        '];
T['VFMADDSUB132PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMADDSUB132PS_rynymy', 'tuple=FV'];
T['VFMADDSUB132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMADDSUB132PS_rznzmz', '        '];
T['VFMADDSUB132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMADDSUB132PS_rznzmz', '        '];
T['VFMADDSUB132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMADDSUB132PS_rznzmz', 'tuple=FV'];
T['VFMADDSUB213PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMADDSUB213PS_ronomo', '        '];
T['VFMADDSUB213PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMADDSUB213PS_ronomo', 'tuple=FV'];
T['VFMADDSUB213PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMADDSUB213PS_rxnxmx', '        '];
T['VFMADDSUB213PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMADDSUB213PS_rxnxmx', '        '];
T['VFMADDSUB213PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMADDSUB213PS_rxnxmx', '        '];
T['VFMADDSUB213PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMADDSUB213PS_rxnxmx', '        '];
T['VFMADDSUB213PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMADDSUB213PS_rynymy', '        '];
T['VFMADDSUB213PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMADDSUB213PS_rynymy', 'tuple=FV'];
T['VFMADDSUB213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMADDSUB213PS_rznzmz', '        '];
T['VFMADDSUB213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMADDSUB213PS_rznzmz', '        '];
T['VFMADDSUB213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMADDSUB213PS_rznzmz', 'tuple=FV'];
T['VFMADDSUB231PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMADDSUB231PS_ronomo', '        '];
T['VFMADDSUB231PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMADDSUB231PS_ronomo', 'tuple=FV'];
T['VFMADDSUB231PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMADDSUB231PS_rxnxmx', '        '];
T['VFMADDSUB231PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMADDSUB231PS_rxnxmx', '        '];
T['VFMADDSUB231PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMADDSUB231PS_rxnxmx', '        '];
T['VFMADDSUB231PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMADDSUB231PS_rxnxmx', '        '];
T['VFMADDSUB231PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMADDSUB231PS_rynymy', '        '];
T['VFMADDSUB231PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMADDSUB231PS_rynymy', 'tuple=FV'];
T['VFMADDSUB231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMADDSUB231PS_rznzmz', '        '];
T['VFMADDSUB231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMADDSUB231PS_rznzmz', '        '];
T['VFMADDSUB231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMADDSUB231PS_rznzmz', 'tuple=FV'];

# VFMADD132PD/VFMADD213PD/VFMADD231PD-Fused Multiply-Add of Packed Double-Precision Floating-Point Values.
T['VFMADD132PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMADD132PD_ronomo', '        '];
T['VFMADD132PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMADD132PD_ronomo', 'tuple=FV'];
T['VFMADD132PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMADD132PD_rxnxmx', '        '];
T['VFMADD132PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMADD132PD_rxnxmx', '        '];
T['VFMADD132PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMADD132PD_rxnxmx', '        '];
T['VFMADD132PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMADD132PD_rxnxmx', '        '];
T['VFMADD132PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMADD132PD_rynymy', '        '];
T['VFMADD132PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMADD132PD_rynymy', 'tuple=FV'];
T['VFMADD132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMADD132PD_rznzmz', '        '];
T['VFMADD132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMADD132PD_rznzmz', '        '];
T['VFMADD132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMADD132PD_rznzmz', 'tuple=FV'];
T['VFMADD213PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMADD213PD_ronomo', '        '];
T['VFMADD213PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMADD213PD_ronomo', 'tuple=FV'];
T['VFMADD213PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMADD213PD_rxnxmx', '        '];
T['VFMADD213PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMADD213PD_rxnxmx', '        '];
T['VFMADD213PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMADD213PD_rxnxmx', '        '];
T['VFMADD213PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMADD213PD_rxnxmx', '        '];
T['VFMADD213PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMADD213PD_rynymy', '        '];
T['VFMADD213PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMADD213PD_rynymy', 'tuple=FV'];
T['VFMADD213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMADD213PD_rznzmz', '        '];
T['VFMADD213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMADD213PD_rznzmz', '        '];
T['VFMADD213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMADD213PD_rznzmz', 'tuple=FV'];
T['VFMADD231PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMADD231PD_ronomo', '        '];
T['VFMADD231PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMADD231PD_ronomo', 'tuple=FV'];
T['VFMADD231PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMADD231PD_rxnxmx', '        '];
T['VFMADD231PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMADD231PD_rxnxmx', '        '];
T['VFMADD231PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMADD231PD_rxnxmx', '        '];
T['VFMADD231PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMADD231PD_rxnxmx', '        '];
T['VFMADD231PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMADD231PD_rynymy', '        '];
T['VFMADD231PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMADD231PD_rynymy', 'tuple=FV'];
T['VFMADD231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMADD231PD_rznzmz', '        '];
T['VFMADD231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMADD231PD_rznzmz', '        '];
T['VFMADD231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMADD231PD_rznzmz', 'tuple=FV'];

# VFMADD132PS/VFMADD213PS/VFMADD231PS-Fused Multiply-Add of Packed Single-Precision Floating-Point Values.
T['VFMADD132PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMADD132PS_ronomo', '        '];
T['VFMADD132PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMADD132PS_ronomo', 'tuple=FV'];
T['VFMADD132PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMADD132PS_rxnxmx', '        '];
T['VFMADD132PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMADD132PS_rxnxmx', '        '];
T['VFMADD132PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMADD132PS_rxnxmx', '        '];
T['VFMADD132PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMADD132PS_rxnxmx', '        '];
T['VFMADD132PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMADD132PS_rynymy', '        '];
T['VFMADD132PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMADD132PS_rynymy', 'tuple=FV'];
T['VFMADD132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMADD132PS_rznzmz', '        '];
T['VFMADD132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMADD132PS_rznzmz', '        '];
T['VFMADD132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMADD132PS_rznzmz', 'tuple=FV'];
T['VFMADD213PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMADD213PS_ronomo', '        '];
T['VFMADD213PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMADD213PS_ronomo', 'tuple=FV'];
T['VFMADD213PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMADD213PS_rxnxmx', '        '];
T['VFMADD213PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMADD213PS_rxnxmx', '        '];
T['VFMADD213PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMADD213PS_rxnxmx', '        '];
T['VFMADD213PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMADD213PS_rxnxmx', '        '];
T['VFMADD213PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMADD213PS_rynymy', '        '];
T['VFMADD213PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMADD213PS_rynymy', 'tuple=FV'];
T['VFMADD213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMADD213PS_rznzmz', '        '];
T['VFMADD213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMADD213PS_rznzmz', '        '];
T['VFMADD213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMADD213PS_rznzmz', 'tuple=FV'];
T['VFMADD231PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMADD231PS_ronomo', '        '];
T['VFMADD231PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMADD231PS_ronomo', 'tuple=FV'];
T['VFMADD231PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMADD231PS_rxnxmx', '        '];
T['VFMADD231PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMADD231PS_rxnxmx', '        '];
T['VFMADD231PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMADD231PS_rxnxmx', '        '];
T['VFMADD231PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMADD231PS_rxnxmx', '        '];
T['VFMADD231PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMADD231PS_rynymy', '        '];
T['VFMADD231PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMADD231PS_rynymy', 'tuple=FV'];
T['VFMADD231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMADD231PS_rznzmz', '        '];
T['VFMADD231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMADD231PS_rznzmz', '        '];
T['VFMADD231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMADD231PS_rznzmz', 'tuple=FV'];

# VFMADD132SD/VFMADD213SD/VFMADD231SD-Fused Multiply-Add of Scalar Double-Precision Floating-Point Values.
T['VFMADD132SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VFMADD132SD_ronomx', '         '];
T['VFMADD132SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFMADD132SD_ronomx', '         '];
T['VFMADD132SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VFMADD132SD_ronomx', 'tuple=T1S'];
T['VFMADD132SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VFMADD132SD_ronqmq', '         '];
T['VFMADD132SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VFMADD132SD_ronqmq', '         '];
T['VFMADD213SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VFMADD213SD_ronomx', '         '];
T['VFMADD213SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFMADD213SD_ronomx', '         '];
T['VFMADD213SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VFMADD213SD_ronomx', 'tuple=T1S'];
T['VFMADD213SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VFMADD213SD_ronqmq', '         '];
T['VFMADD213SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VFMADD213SD_ronqmq', '         '];
T['VFMADD231SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VFMADD231SD_ronomx', '         '];
T['VFMADD231SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFMADD231SD_ronomx', '         '];
T['VFMADD231SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VFMADD231SD_ronomx', 'tuple=T1S'];
T['VFMADD231SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VFMADD231SD_ronqmq', '         '];
T['VFMADD231SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VFMADD231SD_ronqmq', '         '];

# VFMADD132SS/VFMADD213SS/VFMADD231SS-Fused Multiply-Add of Scalar Single-Precision Floating-Point Values.
T['VFMADD132SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn REG:r:f32=$XMMm                         ', 'MOD=REG             ', 'VFMADD132SS_rondmd', '         '];
T['VFMADD132SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn MEM:r:f32=$mem32                        ', 'MOD=MEM             ', 'VFMADD132SS_rondmd', '         '];
T['VFMADD132SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VFMADD132SS_ronomx', '         '];
T['VFMADD132SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFMADD132SS_ronomx', '         '];
T['VFMADD132SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VFMADD132SS_ronomx', 'tuple=T1S'];
T['VFMADD213SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn REG:r:f32=$XMMm                         ', 'MOD=REG             ', 'VFMADD213SS_rondmd', '         '];
T['VFMADD213SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn MEM:r:f32=$mem32                        ', 'MOD=MEM             ', 'VFMADD213SS_rondmd', '         '];
T['VFMADD213SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VFMADD213SS_ronomx', '         '];
T['VFMADD213SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFMADD213SS_ronomx', '         '];
T['VFMADD213SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VFMADD213SS_ronomx', 'tuple=T1S'];
T['VFMADD231SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn REG:r:f32=$XMMm                         ', 'MOD=REG             ', 'VFMADD231SS_rondmd', '         '];
T['VFMADD231SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn MEM:r:f32=$mem32                        ', 'MOD=MEM             ', 'VFMADD231SS_rondmd', '         '];
T['VFMADD231SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VFMADD231SS_ronomx', '         '];
T['VFMADD231SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFMADD231SS_ronomx', '         '];
T['VFMADD231SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VFMADD231SS_ronomx', 'tuple=T1S'];

# VFMSUBADD132PD/VFMSUBADD213PD/VFMSUBADD231PD-Fused Multiply-Alternating Subtract/Add of Packed Double-Precision Floating-Point Values.
T['VFMSUBADD132PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMSUBADD132PD_ronomo', '        '];
T['VFMSUBADD132PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMSUBADD132PD_ronomo', 'tuple=FV'];
T['VFMSUBADD132PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMSUBADD132PD_rxnxmx', '        '];
T['VFMSUBADD132PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMSUBADD132PD_rxnxmx', '        '];
T['VFMSUBADD132PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMSUBADD132PD_rxnxmx', '        '];
T['VFMSUBADD132PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMSUBADD132PD_rxnxmx', '        '];
T['VFMSUBADD132PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMSUBADD132PD_rynymy', '        '];
T['VFMSUBADD132PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMSUBADD132PD_rynymy', 'tuple=FV'];
T['VFMSUBADD132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMSUBADD132PD_rznzmz', '        '];
T['VFMSUBADD132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMSUBADD132PD_rznzmz', '        '];
T['VFMSUBADD132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMSUBADD132PD_rznzmz', 'tuple=FV'];
T['VFMSUBADD213PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMSUBADD213PD_ronomo', '        '];
T['VFMSUBADD213PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMSUBADD213PD_ronomo', 'tuple=FV'];
T['VFMSUBADD213PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMSUBADD213PD_rxnxmx', '        '];
T['VFMSUBADD213PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMSUBADD213PD_rxnxmx', '        '];
T['VFMSUBADD213PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMSUBADD213PD_rxnxmx', '        '];
T['VFMSUBADD213PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMSUBADD213PD_rxnxmx', '        '];
T['VFMSUBADD213PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMSUBADD213PD_rynymy', '        '];
T['VFMSUBADD213PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMSUBADD213PD_rynymy', 'tuple=FV'];
T['VFMSUBADD213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMSUBADD213PD_rznzmz', '        '];
T['VFMSUBADD213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMSUBADD213PD_rznzmz', '        '];
T['VFMSUBADD213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMSUBADD213PD_rznzmz', 'tuple=FV'];
T['VFMSUBADD231PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMSUBADD231PD_ronomo', '        '];
T['VFMSUBADD231PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMSUBADD231PD_ronomo', 'tuple=FV'];
T['VFMSUBADD231PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMSUBADD231PD_rxnxmx', '        '];
T['VFMSUBADD231PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMSUBADD231PD_rxnxmx', '        '];
T['VFMSUBADD231PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMSUBADD231PD_rxnxmx', '        '];
T['VFMSUBADD231PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMSUBADD231PD_rxnxmx', '        '];
T['VFMSUBADD231PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMSUBADD231PD_rynymy', '        '];
T['VFMSUBADD231PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMSUBADD231PD_rynymy', 'tuple=FV'];
T['VFMSUBADD231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMSUBADD231PD_rznzmz', '        '];
T['VFMSUBADD231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMSUBADD231PD_rznzmz', '        '];
T['VFMSUBADD231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMSUBADD231PD_rznzmz', 'tuple=FV'];

# VFMSUBADD132PS/VFMSUBADD213PS/VFMSUBADD231PS-Fused Multiply-Alternating Subtract/Add of Packed Single-Precision Floating-Point Values.
T['VFMSUBADD132PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMSUBADD132PS_ronomo', '        '];
T['VFMSUBADD132PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMSUBADD132PS_ronomo', 'tuple=FV'];
T['VFMSUBADD132PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMSUBADD132PS_rxnxmx', '        '];
T['VFMSUBADD132PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMSUBADD132PS_rxnxmx', '        '];
T['VFMSUBADD132PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMSUBADD132PS_rxnxmx', '        '];
T['VFMSUBADD132PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMSUBADD132PS_rxnxmx', '        '];
T['VFMSUBADD132PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMSUBADD132PS_rynymy', '        '];
T['VFMSUBADD132PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMSUBADD132PS_rynymy', 'tuple=FV'];
T['VFMSUBADD132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMSUBADD132PS_rznzmz', '        '];
T['VFMSUBADD132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMSUBADD132PS_rznzmz', '        '];
T['VFMSUBADD132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMSUBADD132PS_rznzmz', 'tuple=FV'];
T['VFMSUBADD213PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMSUBADD213PS_ronomo', '        '];
T['VFMSUBADD213PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMSUBADD213PS_ronomo', 'tuple=FV'];
T['VFMSUBADD213PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMSUBADD213PS_rxnxmx', '        '];
T['VFMSUBADD213PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMSUBADD213PS_rxnxmx', '        '];
T['VFMSUBADD213PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMSUBADD213PS_rxnxmx', '        '];
T['VFMSUBADD213PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMSUBADD213PS_rxnxmx', '        '];
T['VFMSUBADD213PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMSUBADD213PS_rynymy', '        '];
T['VFMSUBADD213PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMSUBADD213PS_rynymy', 'tuple=FV'];
T['VFMSUBADD213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMSUBADD213PS_rznzmz', '        '];
T['VFMSUBADD213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMSUBADD213PS_rznzmz', '        '];
T['VFMSUBADD213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMSUBADD213PS_rznzmz', 'tuple=FV'];
T['VFMSUBADD231PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMSUBADD231PS_ronomo', '        '];
T['VFMSUBADD231PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMSUBADD231PS_ronomo', 'tuple=FV'];
T['VFMSUBADD231PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMSUBADD231PS_rxnxmx', '        '];
T['VFMSUBADD231PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMSUBADD231PS_rxnxmx', '        '];
T['VFMSUBADD231PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMSUBADD231PS_rxnxmx', '        '];
T['VFMSUBADD231PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMSUBADD231PS_rxnxmx', '        '];
T['VFMSUBADD231PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMSUBADD231PS_rynymy', '        '];
T['VFMSUBADD231PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMSUBADD231PS_rynymy', 'tuple=FV'];
T['VFMSUBADD231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMSUBADD231PS_rznzmz', '        '];
T['VFMSUBADD231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMSUBADD231PS_rznzmz', '        '];
T['VFMSUBADD231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMSUBADD231PS_rznzmz', 'tuple=FV'];

# VFMSUB132PD/VFMSUB213PD/VFMSUB231PD-Fused Multiply-Subtract of Packed Double-Precision Floating-Point Values.
T['VFMSUB132PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMSUB132PD_ronomo', '        '];
T['VFMSUB132PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMSUB132PD_ronomo', 'tuple=FV'];
T['VFMSUB132PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMSUB132PD_rxnxmx', '        '];
T['VFMSUB132PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMSUB132PD_rxnxmx', '        '];
T['VFMSUB132PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMSUB132PD_rxnxmx', '        '];
T['VFMSUB132PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMSUB132PD_rxnxmx', '        '];
T['VFMSUB132PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMSUB132PD_rynymy', '        '];
T['VFMSUB132PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMSUB132PD_rynymy', 'tuple=FV'];
T['VFMSUB132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMSUB132PD_rznzmz', '        '];
T['VFMSUB132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMSUB132PD_rznzmz', '        '];
T['VFMSUB132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMSUB132PD_rznzmz', 'tuple=FV'];
T['VFMSUB213PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMSUB213PD_ronomo', '        '];
T['VFMSUB213PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMSUB213PD_ronomo', 'tuple=FV'];
T['VFMSUB213PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMSUB213PD_rxnxmx', '        '];
T['VFMSUB213PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMSUB213PD_rxnxmx', '        '];
T['VFMSUB213PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMSUB213PD_rxnxmx', '        '];
T['VFMSUB213PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMSUB213PD_rxnxmx', '        '];
T['VFMSUB213PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMSUB213PD_rynymy', '        '];
T['VFMSUB213PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMSUB213PD_rynymy', 'tuple=FV'];
T['VFMSUB213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMSUB213PD_rznzmz', '        '];
T['VFMSUB213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMSUB213PD_rznzmz', '        '];
T['VFMSUB213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMSUB213PD_rznzmz', 'tuple=FV'];
T['VFMSUB231PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMSUB231PD_ronomo', '        '];
T['VFMSUB231PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMSUB231PD_ronomo', 'tuple=FV'];
T['VFMSUB231PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMSUB231PD_rxnxmx', '        '];
T['VFMSUB231PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMSUB231PD_rxnxmx', '        '];
T['VFMSUB231PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMSUB231PD_rxnxmx', '        '];
T['VFMSUB231PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMSUB231PD_rxnxmx', '        '];
T['VFMSUB231PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMSUB231PD_rynymy', '        '];
T['VFMSUB231PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMSUB231PD_rynymy', 'tuple=FV'];
T['VFMSUB231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMSUB231PD_rznzmz', '        '];
T['VFMSUB231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMSUB231PD_rznzmz', '        '];
T['VFMSUB231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMSUB231PD_rznzmz', 'tuple=FV'];

# VFMSUB132PS/VFMSUB213PS/VFMSUB231PS-Fused Multiply-Subtract of Packed Single-Precision Floating-Point Values.
T['VFMSUB132PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMSUB132PS_ronomo', '        '];
T['VFMSUB132PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMSUB132PS_ronomo', 'tuple=FV'];
T['VFMSUB132PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMSUB132PS_rxnxmx', '        '];
T['VFMSUB132PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMSUB132PS_rxnxmx', '        '];
T['VFMSUB132PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMSUB132PS_rxnxmx', '        '];
T['VFMSUB132PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMSUB132PS_rxnxmx', '        '];
T['VFMSUB132PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMSUB132PS_rynymy', '        '];
T['VFMSUB132PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMSUB132PS_rynymy', 'tuple=FV'];
T['VFMSUB132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMSUB132PS_rznzmz', '        '];
T['VFMSUB132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMSUB132PS_rznzmz', '        '];
T['VFMSUB132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMSUB132PS_rznzmz', 'tuple=FV'];
T['VFMSUB213PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMSUB213PS_ronomo', '        '];
T['VFMSUB213PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMSUB213PS_ronomo', 'tuple=FV'];
T['VFMSUB213PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMSUB213PS_rxnxmx', '        '];
T['VFMSUB213PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMSUB213PS_rxnxmx', '        '];
T['VFMSUB213PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMSUB213PS_rxnxmx', '        '];
T['VFMSUB213PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMSUB213PS_rxnxmx', '        '];
T['VFMSUB213PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMSUB213PS_rynymy', '        '];
T['VFMSUB213PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMSUB213PS_rynymy', 'tuple=FV'];
T['VFMSUB213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMSUB213PS_rznzmz', '        '];
T['VFMSUB213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMSUB213PS_rznzmz', '        '];
T['VFMSUB213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMSUB213PS_rznzmz', 'tuple=FV'];
T['VFMSUB231PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFMSUB231PS_ronomo', '        '];
T['VFMSUB231PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFMSUB231PS_ronomo', 'tuple=FV'];
T['VFMSUB231PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFMSUB231PS_rxnxmx', '        '];
T['VFMSUB231PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFMSUB231PS_rxnxmx', '        '];
T['VFMSUB231PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFMSUB231PS_rxnxmx', '        '];
T['VFMSUB231PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFMSUB231PS_rxnxmx', '        '];
T['VFMSUB231PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFMSUB231PS_rynymy', '        '];
T['VFMSUB231PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFMSUB231PS_rynymy', 'tuple=FV'];
T['VFMSUB231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFMSUB231PS_rznzmz', '        '];
T['VFMSUB231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFMSUB231PS_rznzmz', '        '];
T['VFMSUB231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFMSUB231PS_rznzmz', 'tuple=FV'];

# VFMSUB132SD/VFMSUB213SD/VFMSUB231SD-Fused Multiply-Subtract of Scalar Double-Precision Floating-Point Values.
T['VFMSUB132SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VFMSUB132SD_ronomx', '         '];
T['VFMSUB132SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFMSUB132SD_ronomx', '         '];
T['VFMSUB132SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VFMSUB132SD_ronomx', 'tuple=T1S'];
T['VFMSUB132SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VFMSUB132SD_ronqmq', '         '];
T['VFMSUB132SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VFMSUB132SD_ronqmq', '         '];
T['VFMSUB213SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VFMSUB213SD_ronomx', '         '];
T['VFMSUB213SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFMSUB213SD_ronomx', '         '];
T['VFMSUB213SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VFMSUB213SD_ronomx', 'tuple=T1S'];
T['VFMSUB213SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VFMSUB213SD_ronqmq', '         '];
T['VFMSUB213SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VFMSUB213SD_ronqmq', '         '];
T['VFMSUB231SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VFMSUB231SD_ronomx', '         '];
T['VFMSUB231SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFMSUB231SD_ronomx', '         '];
T['VFMSUB231SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VFMSUB231SD_ronomx', 'tuple=T1S'];
T['VFMSUB231SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VFMSUB231SD_ronqmq', '         '];
T['VFMSUB231SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VFMSUB231SD_ronqmq', '         '];

# VFMSUB132SS/VFMSUB213SS/VFMSUB231SS-Fused Multiply-Subtract of Scalar Single-Precision Floating-Point Values.
T['VFMSUB132SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn REG:r:f32=$XMMm                         ', 'MOD=REG             ', 'VFMSUB132SS_rondmd', '         '];
T['VFMSUB132SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn MEM:r:f32=$mem32                        ', 'MOD=MEM             ', 'VFMSUB132SS_rondmd', '         '];
T['VFMSUB132SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VFMSUB132SS_ronomx', '         '];
T['VFMSUB132SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFMSUB132SS_ronomx', '         '];
T['VFMSUB132SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VFMSUB132SS_ronomx', 'tuple=T1S'];
T['VFMSUB213SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn REG:r:f32=$XMMm                         ', 'MOD=REG             ', 'VFMSUB213SS_rondmd', '         '];
T['VFMSUB213SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn MEM:r:f32=$mem32                        ', 'MOD=MEM             ', 'VFMSUB213SS_rondmd', '         '];
T['VFMSUB213SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VFMSUB213SS_ronomx', '         '];
T['VFMSUB213SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFMSUB213SS_ronomx', '         '];
T['VFMSUB213SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VFMSUB213SS_ronomx', 'tuple=T1S'];
T['VFMSUB231SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn REG:r:f32=$XMMm                         ', 'MOD=REG             ', 'VFMSUB231SS_rondmd', '         '];
T['VFMSUB231SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn MEM:r:f32=$mem32                        ', 'MOD=MEM             ', 'VFMSUB231SS_rondmd', '         '];
T['VFMSUB231SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VFMSUB231SS_ronomx', '         '];
T['VFMSUB231SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFMSUB231SS_ronomx', '         '];
T['VFMSUB231SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VFMSUB231SS_ronomx', 'tuple=T1S'];

# VFNMADD132PD/VFNMADD213PD/VFNMADD231PD-Fused Negative Multiply-Add of Packed Double-Precision Floating-Point Values.
T['VFNMADD132PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFNMADD132PD_ronomo', '        '];
T['VFNMADD132PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFNMADD132PD_ronomo', 'tuple=FV'];
T['VFNMADD132PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFNMADD132PD_rxnxmx', '        '];
T['VFNMADD132PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFNMADD132PD_rxnxmx', '        '];
T['VFNMADD132PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFNMADD132PD_rxnxmx', '        '];
T['VFNMADD132PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFNMADD132PD_rxnxmx', '        '];
T['VFNMADD132PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFNMADD132PD_rynymy', '        '];
T['VFNMADD132PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFNMADD132PD_rynymy', 'tuple=FV'];
T['VFNMADD132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFNMADD132PD_rznzmz', '        '];
T['VFNMADD132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFNMADD132PD_rznzmz', '        '];
T['VFNMADD132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFNMADD132PD_rznzmz', 'tuple=FV'];
T['VFNMADD213PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFNMADD213PD_ronomo', '        '];
T['VFNMADD213PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFNMADD213PD_ronomo', 'tuple=FV'];
T['VFNMADD213PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFNMADD213PD_rxnxmx', '        '];
T['VFNMADD213PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFNMADD213PD_rxnxmx', '        '];
T['VFNMADD213PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFNMADD213PD_rxnxmx', '        '];
T['VFNMADD213PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFNMADD213PD_rxnxmx', '        '];
T['VFNMADD213PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFNMADD213PD_rynymy', '        '];
T['VFNMADD213PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFNMADD213PD_rynymy', 'tuple=FV'];
T['VFNMADD213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFNMADD213PD_rznzmz', '        '];
T['VFNMADD213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFNMADD213PD_rznzmz', '        '];
T['VFNMADD213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFNMADD213PD_rznzmz', 'tuple=FV'];
T['VFNMADD231PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFNMADD231PD_ronomo', '        '];
T['VFNMADD231PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFNMADD231PD_ronomo', 'tuple=FV'];
T['VFNMADD231PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFNMADD231PD_rxnxmx', '        '];
T['VFNMADD231PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFNMADD231PD_rxnxmx', '        '];
T['VFNMADD231PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFNMADD231PD_rxnxmx', '        '];
T['VFNMADD231PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFNMADD231PD_rxnxmx', '        '];
T['VFNMADD231PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFNMADD231PD_rynymy', '        '];
T['VFNMADD231PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFNMADD231PD_rynymy', 'tuple=FV'];
T['VFNMADD231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFNMADD231PD_rznzmz', '        '];
T['VFNMADD231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFNMADD231PD_rznzmz', '        '];
T['VFNMADD231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFNMADD231PD_rznzmz', 'tuple=FV'];

# VFNMADD132PS/VFNMADD213PS/VFNMADD231PS-Fused Negative Multiply-Add of Packed Single-Precision Floating-Point Values.
T['VFNMADD132PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFNMADD132PS_ronomo', '        '];
T['VFNMADD132PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFNMADD132PS_ronomo', 'tuple=FV'];
T['VFNMADD132PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFNMADD132PS_rxnxmx', '        '];
T['VFNMADD132PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFNMADD132PS_rxnxmx', '        '];
T['VFNMADD132PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFNMADD132PS_rxnxmx', '        '];
T['VFNMADD132PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFNMADD132PS_rxnxmx', '        '];
T['VFNMADD132PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFNMADD132PS_rynymy', '        '];
T['VFNMADD132PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFNMADD132PS_rynymy', 'tuple=FV'];
T['VFNMADD132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFNMADD132PS_rznzmz', '        '];
T['VFNMADD132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFNMADD132PS_rznzmz', '        '];
T['VFNMADD132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFNMADD132PS_rznzmz', 'tuple=FV'];
T['VFNMADD213PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFNMADD213PS_ronomo', '        '];
T['VFNMADD213PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFNMADD213PS_ronomo', 'tuple=FV'];
T['VFNMADD213PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFNMADD213PS_rxnxmx', '        '];
T['VFNMADD213PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFNMADD213PS_rxnxmx', '        '];
T['VFNMADD213PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFNMADD213PS_rxnxmx', '        '];
T['VFNMADD213PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFNMADD213PS_rxnxmx', '        '];
T['VFNMADD213PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFNMADD213PS_rynymy', '        '];
T['VFNMADD213PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFNMADD213PS_rynymy', 'tuple=FV'];
T['VFNMADD213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFNMADD213PS_rznzmz', '        '];
T['VFNMADD213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFNMADD213PS_rznzmz', '        '];
T['VFNMADD213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFNMADD213PS_rznzmz', 'tuple=FV'];
T['VFNMADD231PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFNMADD231PS_ronomo', '        '];
T['VFNMADD231PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFNMADD231PS_ronomo', 'tuple=FV'];
T['VFNMADD231PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFNMADD231PS_rxnxmx', '        '];
T['VFNMADD231PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFNMADD231PS_rxnxmx', '        '];
T['VFNMADD231PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFNMADD231PS_rxnxmx', '        '];
T['VFNMADD231PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFNMADD231PS_rxnxmx', '        '];
T['VFNMADD231PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFNMADD231PS_rynymy', '        '];
T['VFNMADD231PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFNMADD231PS_rynymy', 'tuple=FV'];
T['VFNMADD231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFNMADD231PS_rznzmz', '        '];
T['VFNMADD231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFNMADD231PS_rznzmz', '        '];
T['VFNMADD231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFNMADD231PS_rznzmz', 'tuple=FV'];

# VFNMADD132SD/VFNMADD213SD/VFNMADD231SD-Fused Negative Multiply-Add of Scalar Double-Precision Floating-Point Values.
T['VFNMADD132SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VFNMADD132SD_ronomx', '         '];
T['VFNMADD132SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFNMADD132SD_ronomx', '         '];
T['VFNMADD132SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VFNMADD132SD_ronomx', 'tuple=T1S'];
T['VFNMADD132SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VFNMADD132SD_ronqmq', '         '];
T['VFNMADD132SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VFNMADD132SD_ronqmq', '         '];
T['VFNMADD213SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VFNMADD213SD_ronomx', '         '];
T['VFNMADD213SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFNMADD213SD_ronomx', '         '];
T['VFNMADD213SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VFNMADD213SD_ronomx', 'tuple=T1S'];
T['VFNMADD213SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VFNMADD213SD_ronqmq', '         '];
T['VFNMADD213SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VFNMADD213SD_ronqmq', '         '];
T['VFNMADD231SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VFNMADD231SD_ronomx', '         '];
T['VFNMADD231SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFNMADD231SD_ronomx', '         '];
T['VFNMADD231SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VFNMADD231SD_ronomx', 'tuple=T1S'];
T['VFNMADD231SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VFNMADD231SD_ronqmq', '         '];
T['VFNMADD231SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VFNMADD231SD_ronqmq', '         '];

# VFNMADD132SS/VFNMADD213SS/VFNMADD231SS-Fused Negative Multiply-Add of Scalar Single-Precision Floating-Point Values.
T['VFNMADD132SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn REG:r:f32=$XMMm                         ', 'MOD=REG             ', 'VFNMADD132SS_rondmd', '         '];
T['VFNMADD132SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn MEM:r:f32=$mem32                        ', 'MOD=MEM             ', 'VFNMADD132SS_rondmd', '         '];
T['VFNMADD132SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VFNMADD132SS_ronomx', '         '];
T['VFNMADD132SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFNMADD132SS_ronomx', '         '];
T['VFNMADD132SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VFNMADD132SS_ronomx', 'tuple=T1S'];
T['VFNMADD213SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn REG:r:f32=$XMMm                         ', 'MOD=REG             ', 'VFNMADD213SS_rondmd', '         '];
T['VFNMADD213SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn MEM:r:f32=$mem32                        ', 'MOD=MEM             ', 'VFNMADD213SS_rondmd', '         '];
T['VFNMADD213SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VFNMADD213SS_ronomx', '         '];
T['VFNMADD213SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFNMADD213SS_ronomx', '         '];
T['VFNMADD213SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VFNMADD213SS_ronomx', 'tuple=T1S'];
T['VFNMADD231SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn REG:r:f32=$XMMm                         ', 'MOD=REG             ', 'VFNMADD231SS_rondmd', '         '];
T['VFNMADD231SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn MEM:r:f32=$mem32                        ', 'MOD=MEM             ', 'VFNMADD231SS_rondmd', '         '];
T['VFNMADD231SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VFNMADD231SS_ronomx', '         '];
T['VFNMADD231SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFNMADD231SS_ronomx', '         '];
T['VFNMADD231SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VFNMADD231SS_ronomx', 'tuple=T1S'];

# VFNMSUB132PD/VFNMSUB213PD/VFNMSUB231PD-Fused Negative Multiply-Subtract of Packed Double-Precision Floating-Point Values.
T['VFNMSUB132PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFNMSUB132PD_ronomo', '        '];
T['VFNMSUB132PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFNMSUB132PD_ronomo', 'tuple=FV'];
T['VFNMSUB132PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFNMSUB132PD_rxnxmx', '        '];
T['VFNMSUB132PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFNMSUB132PD_rxnxmx', '        '];
T['VFNMSUB132PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFNMSUB132PD_rxnxmx', '        '];
T['VFNMSUB132PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFNMSUB132PD_rxnxmx', '        '];
T['VFNMSUB132PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFNMSUB132PD_rynymy', '        '];
T['VFNMSUB132PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFNMSUB132PD_rynymy', 'tuple=FV'];
T['VFNMSUB132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFNMSUB132PD_rznzmz', '        '];
T['VFNMSUB132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFNMSUB132PD_rznzmz', '        '];
T['VFNMSUB132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFNMSUB132PD_rznzmz', 'tuple=FV'];
T['VFNMSUB213PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFNMSUB213PD_ronomo', '        '];
T['VFNMSUB213PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFNMSUB213PD_ronomo', 'tuple=FV'];
T['VFNMSUB213PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFNMSUB213PD_rxnxmx', '        '];
T['VFNMSUB213PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFNMSUB213PD_rxnxmx', '        '];
T['VFNMSUB213PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFNMSUB213PD_rxnxmx', '        '];
T['VFNMSUB213PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFNMSUB213PD_rxnxmx', '        '];
T['VFNMSUB213PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFNMSUB213PD_rynymy', '        '];
T['VFNMSUB213PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFNMSUB213PD_rynymy', 'tuple=FV'];
T['VFNMSUB213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFNMSUB213PD_rznzmz', '        '];
T['VFNMSUB213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFNMSUB213PD_rznzmz', '        '];
T['VFNMSUB213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFNMSUB213PD_rznzmz', 'tuple=FV'];
T['VFNMSUB231PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VFNMSUB231PD_ronomo', '        '];
T['VFNMSUB231PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFNMSUB231PD_ronomo', 'tuple=FV'];
T['VFNMSUB231PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm                              ', 'MOD=REG VL=128      ', 'VFNMSUB231PD_rxnxmx', '        '];
T['VFNMSUB231PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm                              ', 'MOD=REG VL=256      ', 'VFNMSUB231PD_rxnxmx', '        '];
T['VFNMSUB231PD VREG:rw:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128                             ', 'MOD=MEM VL=128      ', 'VFNMSUB231PD_rxnxmx', '        '];
T['VFNMSUB231PD VREG:rw:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256                             ', 'MOD=MEM VL=256      ', 'VFNMSUB231PD_rxnxmx', '        '];
T['VFNMSUB231PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VFNMSUB231PD_rynymy', '        '];
T['VFNMSUB231PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFNMSUB231PD_rynymy', 'tuple=FV'];
T['VFNMSUB231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFNMSUB231PD_rznzmz', '        '];
T['VFNMSUB231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFNMSUB231PD_rznzmz', '        '];
T['VFNMSUB231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFNMSUB231PD_rznzmz', 'tuple=FV'];

# VFNMSUB132PS/VFNMSUB213PS/VFNMSUB231PS-Fused Negative Multiply-Subtract of Packed Single-Precision Floating-Point Values.
T['VFNMSUB132PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFNMSUB132PS_ronomo', '        '];
T['VFNMSUB132PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFNMSUB132PS_ronomo', 'tuple=FV'];
T['VFNMSUB132PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFNMSUB132PS_rxnxmx', '        '];
T['VFNMSUB132PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFNMSUB132PS_rxnxmx', '        '];
T['VFNMSUB132PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFNMSUB132PS_rxnxmx', '        '];
T['VFNMSUB132PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFNMSUB132PS_rxnxmx', '        '];
T['VFNMSUB132PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFNMSUB132PS_rynymy', '        '];
T['VFNMSUB132PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFNMSUB132PS_rynymy', 'tuple=FV'];
T['VFNMSUB132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFNMSUB132PS_rznzmz', '        '];
T['VFNMSUB132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFNMSUB132PS_rznzmz', '        '];
T['VFNMSUB132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFNMSUB132PS_rznzmz', 'tuple=FV'];
T['VFNMSUB213PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFNMSUB213PS_ronomo', '        '];
T['VFNMSUB213PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFNMSUB213PS_ronomo', 'tuple=FV'];
T['VFNMSUB213PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFNMSUB213PS_rxnxmx', '        '];
T['VFNMSUB213PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFNMSUB213PS_rxnxmx', '        '];
T['VFNMSUB213PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFNMSUB213PS_rxnxmx', '        '];
T['VFNMSUB213PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFNMSUB213PS_rxnxmx', '        '];
T['VFNMSUB213PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFNMSUB213PS_rynymy', '        '];
T['VFNMSUB213PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFNMSUB213PS_rynymy', 'tuple=FV'];
T['VFNMSUB213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFNMSUB213PS_rznzmz', '        '];
T['VFNMSUB213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFNMSUB213PS_rznzmz', '        '];
T['VFNMSUB213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFNMSUB213PS_rznzmz', 'tuple=FV'];
T['VFNMSUB231PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VFNMSUB231PS_ronomo', '        '];
T['VFNMSUB231PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VFNMSUB231PS_ronomo', 'tuple=FV'];
T['VFNMSUB231PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm                              ', 'MOD=REG VL=128      ', 'VFNMSUB231PS_rxnxmx', '        '];
T['VFNMSUB231PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG VL=256      ', 'VFNMSUB231PS_rxnxmx', '        '];
T['VFNMSUB231PS VREG:rw:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128                             ', 'MOD=MEM VL=128      ', 'VFNMSUB231PS_rxnxmx', '        '];
T['VFNMSUB231PS VREG:rw:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM VL=256      ', 'VFNMSUB231PS_rxnxmx', '        '];
T['VFNMSUB231PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VFNMSUB231PS_rynymy', '        '];
T['VFNMSUB231PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VFNMSUB231PS_rynymy', 'tuple=FV'];
T['VFNMSUB231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VFNMSUB231PS_rznzmz', '        '];
T['VFNMSUB231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VFNMSUB231PS_rznzmz', '        '];
T['VFNMSUB231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VFNMSUB231PS_rznzmz', 'tuple=FV'];

# VFNMSUB132SD/VFNMSUB213SD/VFNMSUB231SD-Fused Negative Multiply-Subtract of Scalar Double-Precision Floating-Point Values.
T['VFNMSUB132SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VFNMSUB132SD_ronomx', '         '];
T['VFNMSUB132SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFNMSUB132SD_ronomx', '         '];
T['VFNMSUB132SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VFNMSUB132SD_ronomx', 'tuple=T1S'];
T['VFNMSUB132SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VFNMSUB132SD_ronqmq', '         '];
T['VFNMSUB132SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VFNMSUB132SD_ronqmq', '         '];
T['VFNMSUB213SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VFNMSUB213SD_ronomx', '         '];
T['VFNMSUB213SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFNMSUB213SD_ronomx', '         '];
T['VFNMSUB213SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VFNMSUB213SD_ronomx', 'tuple=T1S'];
T['VFNMSUB213SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VFNMSUB213SD_ronqmq', '         '];
T['VFNMSUB213SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VFNMSUB213SD_ronqmq', '         '];
T['VFNMSUB231SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VFNMSUB231SD_ronomx', '         '];
T['VFNMSUB231SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFNMSUB231SD_ronomx', '         '];
T['VFNMSUB231SD VREG:rw:f64=$XMMr PREG:r:Z=$Kk    VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VFNMSUB231SD_ronomx', 'tuple=T1S'];
T['VFNMSUB231SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn REG:r:f64=$XMMm                         ', 'MOD=REG             ', 'VFNMSUB231SD_ronqmq', '         '];
T['VFNMSUB231SD VREG:rw:f64=$XMMr REG:r:f64=$XMMn MEM:r:f64=$mem64                        ', 'MOD=MEM             ', 'VFNMSUB231SD_ronqmq', '         '];

# VFNMSUB132SS/VFNMSUB213SS/VFNMSUB231SS-Fused Negative Multiply-Subtract of Scalar Single-Precision Floating-Point Values.
T['VFNMSUB132SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn REG:r:f32=$XMMm                         ', 'MOD=REG             ', 'VFNMSUB132SS_rondmd', '         '];
T['VFNMSUB132SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn MEM:r:f32=$mem32                        ', 'MOD=MEM             ', 'VFNMSUB132SS_rondmd', '         '];
T['VFNMSUB132SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VFNMSUB132SS_ronomx', '         '];
T['VFNMSUB132SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFNMSUB132SS_ronomx', '         '];
T['VFNMSUB132SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VFNMSUB132SS_ronomx', 'tuple=T1S'];
T['VFNMSUB213SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn REG:r:f32=$XMMm                         ', 'MOD=REG             ', 'VFNMSUB213SS_rondmd', '         '];
T['VFNMSUB213SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn MEM:r:f32=$mem32                        ', 'MOD=MEM             ', 'VFNMSUB213SS_rondmd', '         '];
T['VFNMSUB213SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VFNMSUB213SS_ronomx', '         '];
T['VFNMSUB213SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFNMSUB213SS_ronomx', '         '];
T['VFNMSUB213SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VFNMSUB213SS_ronomx', 'tuple=T1S'];
T['VFNMSUB231SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn REG:r:f32=$XMMm                         ', 'MOD=REG             ', 'VFNMSUB231SS_rondmd', '         '];
T['VFNMSUB231SS VREG:rw:f32=$XMMr REG:r:f32=$XMMn MEM:r:f32=$mem32                        ', 'MOD=MEM             ', 'VFNMSUB231SS_rondmd', '         '];
T['VFNMSUB231SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VFNMSUB231SS_ronomx', '         '];
T['VFNMSUB231SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VFNMSUB231SS_ronomx', '         '];
T['VFNMSUB231SS VREG:rw:f32=$XMMr PREG:r:Z=$Kk    VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VFNMSUB231SS_ronomx', 'tuple=T1S'];

# VFPCLASSPD-Tests Types Of a Packed Float64 Values.
T['VFPCLASSPD REG:w=$Kr PREG:r:Z=$Kk VREG:r:f64=$XMMm             FPCT:r:u8=$fpct', 'BP=0 MOD=REG', 'VFPCLASSPD_moub', '        '];
T['VFPCLASSPD REG:w=$Kr PREG:r:Z=$Kk MEM:r:f64=$mem128 BCST=$BCST FPCT:r:u8=$fpct', 'MOD=MEM     ', 'VFPCLASSPD_moub', 'tuple=FV'];
T['VFPCLASSPD REG:w=$Kr PREG:r:Z=$Kk VREG:r:f64=$YMMm             FPCT:r:u8=$fpct', 'BP=0 MOD=REG', 'VFPCLASSPD_myub', '        '];
T['VFPCLASSPD REG:w=$Kr PREG:r:Z=$Kk MEM:r:f64=$mem256 BCST=$BCST FPCT:r:u8=$fpct', 'MOD=MEM     ', 'VFPCLASSPD_myub', 'tuple=FV'];
T['VFPCLASSPD REG:w=$Kr PREG:r:Z=$Kk VREG:r:f64=$ZMMm             FPCT:r:u8=$fpct', 'BP=0 MOD=REG', 'VFPCLASSPD_mzub', '        '];
T['VFPCLASSPD REG:w=$Kr PREG:r:Z=$Kk MEM:r:f64=$mem512 BCST=$BCST FPCT:r:u8=$fpct', 'MOD=MEM     ', 'VFPCLASSPD_mzub', 'tuple=FV'];

# VFPCLASSPS-Tests Types Of a Packed Float32 Values.
T['VFPCLASSPS REG:w=$Kr PREG:r:Z=$Kk VREG:r:f32=$XMMm             FPCT:r:u8=$fpct', 'BP=0 MOD=REG', 'VFPCLASSPS_moub', '        '];
T['VFPCLASSPS REG:w=$Kr PREG:r:Z=$Kk MEM:r:f32=$mem128 BCST=$BCST FPCT:r:u8=$fpct', 'MOD=MEM     ', 'VFPCLASSPS_moub', 'tuple=FV'];
T['VFPCLASSPS REG:w=$Kr PREG:r:Z=$Kk VREG:r:f32=$YMMm             FPCT:r:u8=$fpct', 'BP=0 MOD=REG', 'VFPCLASSPS_myub', '        '];
T['VFPCLASSPS REG:w=$Kr PREG:r:Z=$Kk MEM:r:f32=$mem256 BCST=$BCST FPCT:r:u8=$fpct', 'MOD=MEM     ', 'VFPCLASSPS_myub', 'tuple=FV'];
T['VFPCLASSPS REG:w=$Kr PREG:r:Z=$Kk VREG:r:f32=$ZMMm             FPCT:r:u8=$fpct', 'BP=0 MOD=REG', 'VFPCLASSPS_mzub', '        '];
T['VFPCLASSPS REG:w=$Kr PREG:r:Z=$Kk MEM:r:f32=$mem512 BCST=$BCST FPCT:r:u8=$fpct', 'MOD=MEM     ', 'VFPCLASSPS_mzub', 'tuple=FV'];

# VFPCLASSSD-Tests Types Of a Scalar Float64 Values.
T['VFPCLASSSD REG:w=$Kr PREG:r:Z=$Kk VREG:r:f64=$XMMm IMM:u8=$uimm8', 'MOD=REG', 'VFPCLASSSD', '         '];
T['VFPCLASSSD REG:w=$Kr PREG:r:Z=$Kk MEM:r:f64=$mem64 IMM:u8=$uimm8', 'MOD=MEM', 'VFPCLASSSD', 'tuple=T1S'];

# VFPCLASSSS-Tests Types Of a Scalar Float32 Values.
T['VFPCLASSSS REG:w=$Kr PREG:r:Z=$Kk VREG:r:f32=$XMMm IMM:u8=$uimm8', 'MOD=REG', 'VFPCLASSSS', '         '];
T['VFPCLASSSS REG:w=$Kr PREG:r:Z=$Kk MEM:r:f32=$mem32 IMM:u8=$uimm8', 'MOD=MEM', 'VFPCLASSSS', 'tuple=T1S'];

# VGATHERDPS/VGATHERDPD-Gather Packed Single, Packed Double with Signed Dword.
T['VGATHERDPD VREG:w:f64=$XMMr   PREG:rw=$Kk      MEM:r:f64=$vmx64 ', 'W=1       ', 'VGATHERDPx_romx  ', 'tuple=T1S'];
T['VGATHERDPS VREG:w:f32=$XMMr   PREG:rw=$Kk      MEM:r:f32=$vmx32 ', 'W=0       ', 'VGATHERDPx_romx  ', 'tuple=T1S'];
T['VGATHERDPD VREG:crw:f64=$XMMr MEM:r:f64=$vmx64 VREG:rw:s64=$XMMn', 'VL=128 W=1', 'VGATHERDPx_rxmxnx', '         '];
T['VGATHERDPD VREG:crw:f64=$YMMr MEM:r:f64=$vmx64 VREG:rw:s64=$YMMn', 'VL=256 W=1', 'VGATHERDPx_rxmxnx', '         '];
T['VGATHERDPS VREG:crw:f32=$XMMr MEM:r:f32=$vmx32 VREG:rw:s32=$XMMn', 'VL=128 W=0', 'VGATHERDPx_rxmxnx', '         '];
T['VGATHERDPS VREG:crw:f32=$YMMr MEM:r:f32=$vmy32 VREG:rw:s32=$YMMn', 'VL=256 W=0', 'VGATHERDPx_rxmxnx', '         '];
T['VGATHERDPD VREG:w:f64=$YMMr   PREG:rw=$Kk      MEM:r:f64=$vmx64 ', 'W=1       ', 'VGATHERDPx_rymx  ', 'tuple=T1S'];
T['VGATHERDPS VREG:w:f32=$YMMr   PREG:rw=$Kk      MEM:r:f32=$vmy32 ', 'W=0       ', 'VGATHERDPx_rymx  ', 'tuple=T1S'];
T['VGATHERDPD VREG:w:f64=$ZMMr   PREG:rw=$Kk      MEM:r:f64=$vmy64 ', 'W=1       ', 'VGATHERDPx_rzmx  ', 'tuple=T1S'];
T['VGATHERDPS VREG:w:f32=$ZMMr   PREG:rw=$Kk      MEM:r:f32=$vmz32 ', 'W=0       ', 'VGATHERDPx_rzmx  ', 'tuple=T1S'];

# VGATHERQPS/VGATHERQPD-Gather Packed Single, Packed Double with Signed Qword Indices.
T['VGATHERQPD VREG:w:f64=$XMMr   PREG:rw=$Kk      MEM:r:f64=$vmx64 ', 'W=1       ', 'VGATHERQPx_romx  ', 'tuple=T1S'];
T['VGATHERQPS VREG:w:f32=$XMMr   PREG:rw=$Kk      MEM:r:f32=$vmx32 ', 'W=0       ', 'VGATHERQPx_romx  ', 'tuple=T1S'];
T['VGATHERQPD VREG:w:f64=$YMMr   PREG:rw=$Kk      MEM:r:f64=$vmy64 ', 'W=1       ', 'VGATHERQPx_rxmx_1', 'tuple=T1S'];
T['VGATHERQPS VREG:w:f32=$XMMr   PREG:rw=$Kk      MEM:r:f32=$vmy32 ', 'W=0       ', 'VGATHERQPx_rxmx_1', 'tuple=T1S'];
T['VGATHERQPD VREG:w:f64=$ZMMr   PREG:rw=$Kk      MEM:r:f64=$vmz64 ', 'W=1       ', 'VGATHERQPx_rxmx_2', 'tuple=T1S'];
T['VGATHERQPS VREG:w:f32=$YMMr   PREG:rw=$Kk      MEM:r:f32=$vmz32 ', 'W=0       ', 'VGATHERQPx_rxmx_2', 'tuple=T1S'];
T['VGATHERQPD VREG:crw:f64=$XMMr MEM:r:f64=$vmx64 VREG:rw:s64=$XMMn', 'VL=128 W=1', 'VGATHERQPx_rxmxnx', '         '];
T['VGATHERQPD VREG:crw:f64=$YMMr MEM:r:f64=$vmy64 VREG:rw:s64=$YMMn', 'VL=256 W=1', 'VGATHERQPx_rxmxnx', '         '];
T['VGATHERQPS VREG:crw:f32=$XMMr MEM:r:f32=$vmx32 VREG:rw:s32=$XMMn', 'VL=128 W=0', 'VGATHERQPx_rxmxnx', '         '];
T['VGATHERQPS VREG:crw:f32=$XMMr MEM:r:f32=$vmy32 VREG:rw:s32=$XMMn', 'VL=256 W=0', 'VGATHERQPx_rxmxnx', '         '];

# VGETEXPPD-Convert Exponents of Packed DP FP Values to DP FP Values.
T['VGETEXPPD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VGETEXPPD_romo', '        '];
T['VGETEXPPD VREG:w:f64=$XMMr PREG:r:Z=$Kk MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VGETEXPPD_romo', 'tuple=FV'];
T['VGETEXPPD VREG:w:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VGETEXPPD_rymy', '        '];
T['VGETEXPPD VREG:w:f64=$YMMr PREG:r:Z=$Kk MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VGETEXPPD_rymy', 'tuple=FV'];
T['VGETEXPPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VGETEXPPD_rzmz', '        '];
T['VGETEXPPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VGETEXPPD_rzmz', '        '];
T['VGETEXPPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VGETEXPPD_rzmz', 'tuple=FV'];

# VGETEXPPS-Convert Exponents of Packed SP FP Values to SP FP Values.
T['VGETEXPPS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VGETEXPPS_romo', '        '];
T['VGETEXPPS VREG:w:f32=$XMMr PREG:r:Z=$Kk MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VGETEXPPS_romo', 'tuple=FV'];
T['VGETEXPPS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VGETEXPPS_rymy', '        '];
T['VGETEXPPS VREG:w:f32=$YMMr PREG:r:Z=$Kk MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VGETEXPPS_rymy', 'tuple=FV'];
T['VGETEXPPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VGETEXPPS_rzmz', '        '];
T['VGETEXPPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VGETEXPPS_rzmz', '        '];
T['VGETEXPPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VGETEXPPS_rzmz', 'tuple=FV'];

# VGETEXPSD-Convert Exponents of Scalar DP FP Values to DP FP Value.
T['VGETEXPSD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm         ', 'BP=0 MOD=REG        ', 'VGETEXPSD', '         '];
T['VGETEXPSD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128', 'VGETEXPSD', '         '];
T['VGETEXPSD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn MEM:r:f64=$mem64         ', 'BP=0 MOD=MEM        ', 'VGETEXPSD', 'tuple=T1S'];

# VGETEXPSS-Convert Exponents of Scalar SP FP Values to SP FP Value.
T['VGETEXPSS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm         ', 'BP=0 MOD=REG        ', 'VGETEXPSS', '         '];
T['VGETEXPSS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128', 'VGETEXPSS', '         '];
T['VGETEXPSS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem32         ', 'BP=0 MOD=MEM        ', 'VGETEXPSS', 'tuple=T1S'];

# VGETMANTPD-Extract Float64 Vector of Normalized Mantissas from Float64 Vector.
T['VGETMANTPD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMm             SIGNC:r:u8=$signc', 'BP=0 MOD=REG        ', 'VGETMANTPD_romoub', '        '];
T['VGETMANTPD VREG:w:f64=$XMMr PREG:r:Z=$Kk MEM:r:f64=$mem128 BCST=$BCST SIGNC:r:u8=$signc', 'MOD=MEM             ', 'VGETMANTPD_romoub', 'tuple=FV'];
T['VGETMANTPD VREG:w:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMm             SIGNC:r:u8=$signc', 'BP=0 MOD=REG        ', 'VGETMANTPD_rymyub', '        '];
T['VGETMANTPD VREG:w:f64=$YMMr PREG:r:Z=$Kk MEM:r:f64=$mem256 BCST=$BCST SIGNC:r:u8=$signc', 'MOD=MEM             ', 'VGETMANTPD_rymyub', 'tuple=FV'];
T['VGETMANTPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SAE=$SAE    SIGNC:r:u8=$signc', 'BP=1 MOD=REG RVL=512', 'VGETMANTPD_rzmzub', '        '];
T['VGETMANTPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm             SIGNC:r:u8=$signc', 'BP=0 MOD=REG VL=512 ', 'VGETMANTPD_rzmzub', '        '];
T['VGETMANTPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk MEM:r:f64=$mem512 BCST=$BCST SIGNC:r:u8=$signc', 'MOD=MEM VL=512      ', 'VGETMANTPD_rzmzub', 'tuple=FV'];

# VGETMANTPS-Extract Float32 Vector of Normalized Mantissas from Float32 Vector.
T['VGETMANTPS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm             SIGNC:r:u8=$signc', 'BP=0 MOD=REG        ', 'VGETMANTPS_romoub', '        '];
T['VGETMANTPS VREG:w:f32=$XMMr PREG:r:Z=$Kk MEM:r:f32=$mem128 BCST=$BCST SIGNC:r:u8=$signc', 'MOD=MEM             ', 'VGETMANTPS_romoub', 'tuple=FV'];
T['VGETMANTPS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm             SIGNC:r:u8=$signc', 'BP=0 MOD=REG        ', 'VGETMANTPS_rymyub', '        '];
T['VGETMANTPS VREG:w:f32=$YMMr PREG:r:Z=$Kk MEM:r:f32=$mem256 BCST=$BCST SIGNC:r:u8=$signc', 'MOD=MEM             ', 'VGETMANTPS_rymyub', 'tuple=FV'];
T['VGETMANTPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SAE=$SAE    SIGNC:r:u8=$signc', 'BP=1 MOD=REG RVL=512', 'VGETMANTPS_rzmzub', '        '];
T['VGETMANTPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm             SIGNC:r:u8=$signc', 'BP=0 MOD=REG VL=512 ', 'VGETMANTPS_rzmzub', '        '];
T['VGETMANTPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk MEM:r:f32=$mem512 BCST=$BCST SIGNC:r:u8=$signc', 'MOD=MEM VL=512      ', 'VGETMANTPS_rzmzub', 'tuple=FV'];

# VGETMANTSD-Extract Float64 of Normalized Mantissas from Float64 Scalar.
T['VGETMANTSD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm          SIGNC:r:u8=$signc', 'BP=0 MOD=REG        ', 'VGETMANTSD', '         '];
T['VGETMANTSD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm SAE=$SAE SIGNC:r:u8=$signc', 'BP=1 MOD=REG RVL=128', 'VGETMANTSD', '         '];
T['VGETMANTSD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn MEM:r:f64=$mem64          SIGNC:r:u8=$signc', 'BP=0 MOD=MEM        ', 'VGETMANTSD', 'tuple=T1S'];

# VGETMANTSS-Extract Float32 Vector of Normalized Mantissa from Float32 Vector.
T['VGETMANTSS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm          SIGNC:r:u8=$signc', 'BP=0 MOD=REG        ', 'VGETMANTSS', '         '];
T['VGETMANTSS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm SAE=$SAE SIGNC:r:u8=$signc', 'BP=1 MOD=REG RVL=128', 'VGETMANTSS', '         '];
T['VGETMANTSS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem32          SIGNC:r:u8=$signc', 'BP=0 MOD=MEM        ', 'VGETMANTSS', 'tuple=T1S'];

# VINSERTF128/VINSERTF32x4/VINSERTF64x2/VINSERTF32x8/VINSERTF64x4-Insert Packed Floating-Point Values.
T['VINSERTF128  VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$XMMm                           IMM:u8=$index                  ', 'MOD=REG', 'VINSERTF128          ', '        '];
T['VINSERTF128  VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem128 SBCST:SUPP={FROM=2 TO=4} IMM:u8=$index                  ', 'MOD=MEM', 'VINSERTF128          ', '        '];
T['VINSERTF32X4 VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn                           VREG:r:f32=$XMMm  IMM:u8=$index', 'MOD=REG', 'VINSERTF32X4_rynymoub', '        '];
T['VINSERTF32X4 VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn                           MEM:r:f32=$mem128 IMM:u8=$index', 'MOD=MEM', 'VINSERTF32X4_rynymoub', 'tuple=T4'];
T['VINSERTF32X4 VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn                           VREG:r:f32=$XMMm  IMM:u8=$index', 'MOD=REG', 'VINSERTF32X4_rznzmoub', '        '];
T['VINSERTF32X4 VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn                           MEM:r:f32=$mem128 IMM:u8=$index', 'MOD=MEM', 'VINSERTF32X4_rznzmoub', 'tuple=T4'];
T['VINSERTF32X8 VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn                           VREG:r:f32=$YMMm  IMM:u8=$index', 'MOD=REG', 'VINSERTF32X8         ', '        '];
T['VINSERTF32X8 VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn                           MEM:r:f32=$mem256 IMM:u8=$index', 'MOD=MEM', 'VINSERTF32X8         ', 'tuple=T8'];
T['VINSERTF64X2 VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn                           VREG:r:f64=$XMMm  IMM:u8=$index', 'MOD=REG', 'VINSERTF64X2_rynymoub', '        '];
T['VINSERTF64X2 VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn                           MEM:r:f64=$mem128 IMM:u8=$index', 'MOD=MEM', 'VINSERTF64X2_rynymoub', 'tuple=T2'];
T['VINSERTF64X2 VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn                           VREG:r:f64=$XMMm  IMM:u8=$index', 'MOD=REG', 'VINSERTF64X2_rznzmoub', '        '];
T['VINSERTF64X2 VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn                           MEM:r:f64=$mem128 IMM:u8=$index', 'MOD=MEM', 'VINSERTF64X2_rznzmoub', 'tuple=T2'];
T['VINSERTF64X4 VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn                           VREG:r:f64=$YMMm  IMM:u8=$index', 'MOD=REG', 'VINSERTF64X4         ', '        '];
T['VINSERTF64X4 VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn                           MEM:r:f64=$mem256 IMM:u8=$index', 'MOD=MEM', 'VINSERTF64X4         ', 'tuple=T4'];

# VINSERTI128/VINSERTI32x4/VINSERTI64x2/VINSERTI32x8/VINSERTI64x4-Insert Packed Integer Values.
T['VINSERTI128  VREG:w:u128=$YMMr VREG:r:u128=$YMMn REG:r:u128=$XMMm   IMM:u8=$index                  ', 'MOD=REG', 'VINSERTI128          ', '        '];
T['VINSERTI128  VREG:w:u128=$YMMr VREG:r:u128=$YMMn MEM:r:u128=$mem128 IMM:u8=$index                  ', 'MOD=MEM', 'VINSERTI128          ', '        '];
T['VINSERTI32X4 VREG:w:u32=$YMMr  PREG:r:Z=$Kk      VREG:r:u32=$YMMn   VREG:r:u32=$XMMm  IMM:u8=$index', 'MOD=REG', 'VINSERTI32X4_rynymoub', '        '];
T['VINSERTI32X4 VREG:w:u32=$YMMr  PREG:r:Z=$Kk      VREG:r:u32=$YMMn   MEM:r:u32=$mem128 IMM:u8=$index', 'MOD=MEM', 'VINSERTI32X4_rynymoub', 'tuple=T4'];
T['VINSERTI32X4 VREG:w:u32=$ZMMr  PREG:r:Z=$Kk      VREG:r:u32=$ZMMn   VREG:r:u32=$XMMm  IMM:u8=$index', 'MOD=REG', 'VINSERTI32X4_rznzmoub', '        '];
T['VINSERTI32X4 VREG:w:u32=$ZMMr  PREG:r:Z=$Kk      VREG:r:u32=$ZMMn   MEM:r:u32=$mem128 IMM:u8=$index', 'MOD=MEM', 'VINSERTI32X4_rznzmoub', 'tuple=T4'];
T['VINSERTI32X8 VREG:w:u32=$ZMMr  PREG:r:Z=$Kk      VREG:r:u32=$ZMMn   VREG:r:u32=$YMMm  IMM:u8=$index', 'MOD=REG', 'VINSERTI32X8         ', '        '];
T['VINSERTI32X8 VREG:w:u32=$ZMMr  PREG:r:Z=$Kk      VREG:r:u32=$ZMMn   MEM:r:u32=$mem256 IMM:u8=$index', 'MOD=MEM', 'VINSERTI32X8         ', 'tuple=T8'];
T['VINSERTI64X2 VREG:w:u64=$YMMr  PREG:r:Z=$Kk      VREG:r:u64=$YMMn   VREG:r:u64=$XMMm  IMM:u8=$index', 'MOD=REG', 'VINSERTI64X2_rynymoub', '        '];
T['VINSERTI64X2 VREG:w:u64=$YMMr  PREG:r:Z=$Kk      VREG:r:u64=$YMMn   MEM:r:u64=$mem128 IMM:u8=$index', 'MOD=MEM', 'VINSERTI64X2_rynymoub', 'tuple=T2'];
T['VINSERTI64X2 VREG:w:u64=$ZMMr  PREG:r:Z=$Kk      VREG:r:u64=$ZMMn   VREG:r:u64=$XMMm  IMM:u8=$index', 'MOD=REG', 'VINSERTI64X2_rznzmoub', '        '];
T['VINSERTI64X2 VREG:w:u64=$ZMMr  PREG:r:Z=$Kk      VREG:r:u64=$ZMMn   MEM:r:u64=$mem128 IMM:u8=$index', 'MOD=MEM', 'VINSERTI64X2_rznzmoub', 'tuple=T2'];
T['VINSERTI64X4 VREG:w:u64=$ZMMr  PREG:r:Z=$Kk      VREG:r:u64=$ZMMn   VREG:r:u64=$YMMm  IMM:u8=$index', 'MOD=REG', 'VINSERTI64X4         ', '        '];
T['VINSERTI64X4 VREG:w:u64=$ZMMr  PREG:r:Z=$Kk      VREG:r:u64=$ZMMn   MEM:r:u64=$mem256 IMM:u8=$index', 'MOD=MEM', 'VINSERTI64X4         ', 'tuple=T4'];

# VMASKMOV-Conditional SIMD Packed Loads and Stores.
T['VMASKMOVPD MEM:w:f64=$mem128 VREG:r:u64=$XMMn VREG:r:f64=$XMMr ', 'VL=128', 'VMASKMOVPD_mxnxrx', ''];
T['VMASKMOVPD MEM:w:f64=$mem256 VREG:r:u64=$YMMn VREG:r:f64=$YMMr ', 'VL=256', 'VMASKMOVPD_mxnxrx', ''];
T['VMASKMOVPD VREG:w:f64=$XMMr  VREG:r:u64=$XMMn MEM:r:f64=$mem128', 'VL=128', 'VMASKMOVPD_rxnxmx', ''];
T['VMASKMOVPD VREG:w:f64=$YMMr  VREG:r:u64=$YMMn MEM:r:f64=$mem256', 'VL=256', 'VMASKMOVPD_rxnxmx', ''];
T['VMASKMOVPS MEM:w:f32=$mem128 VREG:r:s32=$XMMn VREG:r:f32=$XMMr ', 'VL=128', 'VMASKMOVPS_mxnxrx', ''];
T['VMASKMOVPS MEM:w:f32=$mem256 VREG:r:s32=$YMMn VREG:r:f32=$YMMr ', 'VL=256', 'VMASKMOVPS_mxnxrx', ''];
T['VMASKMOVPS VREG:w:f32=$XMMr  VREG:r:s32=$XMMn MEM:r:f32=$mem128', 'VL=128', 'VMASKMOVPS_rxnxmx', ''];
T['VMASKMOVPS VREG:w:f32=$YMMr  VREG:r:s32=$YMMn MEM:r:f32=$mem256', 'VL=256', 'VMASKMOVPS_rxnxmx', ''];

# VP2INTERSECTD.
T['VP2INTERSECTD REG:w=$Kr VREG:r:u32=$XMMn VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VP2INTERSECTD_nomo', '        '];
T['VP2INTERSECTD REG:w=$Kr VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VP2INTERSECTD_nomo', 'tuple=FV'];
T['VP2INTERSECTD REG:w=$Kr VREG:r:u32=$YMMn VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VP2INTERSECTD_nymy', '        '];
T['VP2INTERSECTD REG:w=$Kr VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VP2INTERSECTD_nymy', 'tuple=FV'];
T['VP2INTERSECTD REG:w=$Kr VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VP2INTERSECTD_nzmz', '        '];
T['VP2INTERSECTD REG:w=$Kr VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VP2INTERSECTD_nzmz', 'tuple=FV'];

# VP2INTERSECTQ.
T['VP2INTERSECTQ REG:w=$Kr VREG:r:u64=$XMMn VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG', 'VP2INTERSECTQ_nomo', '        '];
T['VP2INTERSECTQ REG:w=$Kr VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VP2INTERSECTQ_nomo', 'tuple=FV'];
T['VP2INTERSECTQ REG:w=$Kr VREG:r:u64=$YMMn VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG', 'VP2INTERSECTQ_nymy', '        '];
T['VP2INTERSECTQ REG:w=$Kr VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VP2INTERSECTQ_nymy', 'tuple=FV'];
T['VP2INTERSECTQ REG:w=$Kr VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG', 'VP2INTERSECTQ_nzmz', '        '];
T['VP2INTERSECTQ REG:w=$Kr VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VP2INTERSECTQ_nzmz', 'tuple=FV'];

# VPBLENDD-Blend Packed Dwords.
T['VPBLENDD VREG:w:u32=$XMMr VREG:r:u32=$XMMn VREG:r:u32=$XMMm  IMM:u8=$uimm8', 'MOD=REG VL=128', 'VPBLENDD', ''];
T['VPBLENDD VREG:w:u32=$YMMr VREG:r:u32=$YMMn VREG:r:u32=$YMMm  IMM:u8=$uimm8', 'MOD=REG VL=256', 'VPBLENDD', ''];
T['VPBLENDD VREG:w:u32=$XMMr VREG:r:u32=$XMMn MEM:r:u32=$mem128 IMM:u8=$uimm8', 'MOD=MEM VL=128', 'VPBLENDD', ''];
T['VPBLENDD VREG:w:u32=$YMMr VREG:r:u32=$YMMn MEM:r:u32=$mem256 IMM:u8=$uimm8', 'MOD=MEM VL=256', 'VPBLENDD', ''];

# VPBLENDMB/VPBLENDMW-Blend Byte/Word Vectors Using an Opmask Control.
T['VPBLENDMB VREG:w:u8=$XMMr  PREG:r:Z=$Kk VREG:r:u8=$XMMn  VREG:r:u8=$XMMm  ', 'MOD=REG W=0', 'VPBLENDMx_ronomo_1', '         '];
T['VPBLENDMB VREG:w:u8=$XMMr  PREG:r:Z=$Kk VREG:r:u8=$XMMn  MEM:r:u8=$mem128 ', 'MOD=MEM W=0', 'VPBLENDMx_ronomo_1', 'tuple=FVM'];
T['VPBLENDMW VREG:w:u16=$XMMr PREG:r:Z=$Kk VREG:r:u16=$XMMn VREG:r:u16=$XMMm ', 'MOD=REG W=1', 'VPBLENDMx_ronomo_1', '         '];
T['VPBLENDMW VREG:w:u16=$XMMr PREG:r:Z=$Kk VREG:r:u16=$XMMn MEM:r:u16=$mem128', 'MOD=MEM W=1', 'VPBLENDMx_ronomo_1', 'tuple=FVM'];
T['VPBLENDMB VREG:w:u8=$YMMr  PREG:r:Z=$Kk VREG:r:u8=$YMMn  VREG:r:u8=$YMMm  ', 'MOD=REG W=0', 'VPBLENDMx_rynymy_1', '         '];
T['VPBLENDMB VREG:w:u8=$YMMr  PREG:r:Z=$Kk VREG:r:u8=$YMMn  MEM:r:u8=$mem256 ', 'MOD=MEM W=0', 'VPBLENDMx_rynymy_1', 'tuple=FVM'];
T['VPBLENDMW VREG:w:u16=$YMMr PREG:r:Z=$Kk VREG:r:u16=$YMMn VREG:r:u16=$YMMm ', 'MOD=REG W=1', 'VPBLENDMx_rynymy_1', '         '];
T['VPBLENDMW VREG:w:u16=$YMMr PREG:r:Z=$Kk VREG:r:u16=$YMMn MEM:r:u16=$mem256', 'MOD=MEM W=1', 'VPBLENDMx_rynymy_1', 'tuple=FVM'];
T['VPBLENDMB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk VREG:r:u8=$ZMMn  VREG:r:u8=$ZMMm  ', 'MOD=REG W=0', 'VPBLENDMx_rznzmz_1', '         '];
T['VPBLENDMB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk VREG:r:u8=$ZMMn  MEM:r:u8=$mem512 ', 'MOD=MEM W=0', 'VPBLENDMx_rznzmz_1', 'tuple=FVM'];
T['VPBLENDMW VREG:w:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u16=$ZMMn VREG:r:u16=$ZMMm ', 'MOD=REG W=1', 'VPBLENDMx_rznzmz_1', '         '];
T['VPBLENDMW VREG:w:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u16=$ZMMn MEM:r:u16=$mem512', 'MOD=MEM W=1', 'VPBLENDMx_rznzmz_1', 'tuple=FVM'];

# VPBLENDMD/VPBLENDMQ-Blend Int32/Int64 Vectors Using an OpMask Control.
T['VPBLENDMD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPBLENDMx_ronomo_2', '        '];
T['VPBLENDMD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPBLENDMx_ronomo_2', 'tuple=FV'];
T['VPBLENDMQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPBLENDMx_ronomo_2', '        '];
T['VPBLENDMQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPBLENDMx_ronomo_2', 'tuple=FV'];
T['VPBLENDMD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPBLENDMx_rynymy_2', '        '];
T['VPBLENDMD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPBLENDMx_rynymy_2', 'tuple=FV'];
T['VPBLENDMQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPBLENDMx_rynymy_2', '        '];
T['VPBLENDMQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPBLENDMx_rynymy_2', 'tuple=FV'];
T['VPBLENDMD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPBLENDMx_rznzmz_2', '        '];
T['VPBLENDMD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPBLENDMx_rznzmz_2', 'tuple=FV'];
T['VPBLENDMQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPBLENDMx_rznzmz_2', '        '];
T['VPBLENDMQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPBLENDMx_rznzmz_2', 'tuple=FV'];

# VPBROADCAST-Load Integer and Broadcast.
T['VBROADCASTI128  VREG:w:u128=$YMMr MEM:r:u128=$mem128 SBCST:SUPP={FROM=2 TO=4}                                            ', 'NONE          ', 'VBROADCASTI128      ', '           '];
T['VBROADCASTI32X2 VREG:w:u32=$XMMr  PREG:r:Z=$Kk                                VREG:r:u32=$XMMm                           ', 'MOD=REG       ', 'VBROADCASTI32X2_romx', '           '];
T['VBROADCASTI32X2 VREG:w:u32=$XMMr  PREG:r:Z=$Kk                                MEM:r:u32=$mem64 SBCST:SUPP={FROM=2 TO=4}  ', 'MOD=MEM       ', 'VBROADCASTI32X2_romx', 'tuple=T2   '];
T['VBROADCASTI32X2 VREG:w:u32=$YMMr  PREG:r:Z=$Kk                                VREG:r:u32=$XMMm                           ', 'MOD=REG       ', 'VBROADCASTI32X2_rymx', '           '];
T['VBROADCASTI32X2 VREG:w:u32=$YMMr  PREG:r:Z=$Kk                                MEM:r:u32=$mem64 SBCST:SUPP={FROM=2 TO=8}  ', 'MOD=MEM       ', 'VBROADCASTI32X2_rymx', 'tuple=T2   '];
T['VBROADCASTI32X2 VREG:w:u32=$ZMMr  PREG:r:Z=$Kk                                VREG:r:u32=$XMMm                           ', 'MOD=REG       ', 'VBROADCASTI32X2_rzmx', '           '];
T['VBROADCASTI32X2 VREG:w:u32=$ZMMr  PREG:r:Z=$Kk                                MEM:r:u32=$mem64 SBCST:SUPP={FROM=2 TO=16} ', 'MOD=MEM       ', 'VBROADCASTI32X2_rzmx', 'tuple=T2   '];
T['VBROADCASTI32X4 VREG:w:u32=$YMMr  PREG:r:Z=$Kk                                MEM:r:u32=$mem128 SBCST:SUPP={FROM=4 TO=8} ', 'NONE          ', 'VBROADCASTI32X4_rymo', 'tuple=T4   '];
T['VBROADCASTI32X4 VREG:w:u32=$ZMMr  PREG:r:Z=$Kk                                MEM:r:u32=$mem128 SBCST:SUPP={FROM=4 TO=16}', 'NONE          ', 'VBROADCASTI32X4_rzmo', 'tuple=T4   '];
T['VBROADCASTI32X8 VREG:w:u32=$ZMMr  PREG:r:Z=$Kk                                MEM:r:u32=$mem256 SBCST:SUPP={FROM=8 TO=16}', 'NONE          ', 'VBROADCASTI32X8     ', 'tuple=T8   '];
T['VBROADCASTI64X2 VREG:w:u64=$YMMr  PREG:r:Z=$Kk                                MEM:r:u64=$mem128 SBCST:SUPP={FROM=2 TO=4} ', 'NONE          ', 'VBROADCASTI64X2_rymo', 'tuple=T2   '];
T['VBROADCASTI64X2 VREG:w:u64=$ZMMr  PREG:r:Z=$Kk                                MEM:r:u64=$mem128 SBCST:SUPP={FROM=2 TO=8} ', 'NONE          ', 'VBROADCASTI64X2_rzmo', 'tuple=T2   '];
T['VBROADCASTI64X4 VREG:w:u64=$ZMMr  PREG:r:Z=$Kk                                MEM:r:u64=$mem256 SBCST:SUPP={FROM=4 TO=8} ', 'NONE          ', 'VBROADCASTI64X4_rzmy', 'tuple=T4   '];
T['VPBROADCASTB    VREG:w:u8=$XMMr   PREG:r:Z=$Kk                                REG:r:u8=$GPR32m                           ', 'NONE          ', 'VPBROADCASTB_romd   ', '           '];
T['VPBROADCASTB    VREG:w:u8=$XMMr   PREG:r:Z=$Kk                                VREG:r:u8=$XMMm                            ', 'MOD=REG       ', 'VPBROADCASTB_romx   ', '           '];
T['VPBROADCASTB    VREG:w:u8=$XMMr   PREG:r:Z=$Kk                                MEM:r:u8=$mem8 SBCST:SUPP={FROM=1 TO=16}   ', 'MOD=MEM       ', 'VPBROADCASTB_romx   ', 'tuple=T1S8 '];
T['VPBROADCASTB    VREG:w:u8=$XMMr   REG:r:u8=$XMMm                                                                         ', 'MOD=REG VL=128', 'VPBROADCASTB_rxmb   ', '           '];
T['VPBROADCASTB    VREG:w:u8=$YMMr   REG:r:u8=$XMMm                                                                         ', 'MOD=REG VL=256', 'VPBROADCASTB_rxmb   ', '           '];
T['VPBROADCASTB    VREG:w:u8=$XMMr   MEM:r:u8=$mem8 SBCST:SUPP={FROM=1 TO=16}                                               ', 'MOD=MEM VL=128', 'VPBROADCASTB_rxmb   ', '           '];
T['VPBROADCASTB    VREG:w:u8=$YMMr   MEM:r:u8=$mem8 SBCST:SUPP={FROM=1 TO=32}                                               ', 'MOD=MEM VL=256', 'VPBROADCASTB_rxmb   ', '           '];
T['VPBROADCASTB    VREG:w:u8=$YMMr   PREG:r:Z=$Kk                                REG:r:u8=$GPR32m                           ', 'NONE          ', 'VPBROADCASTB_rymd   ', '           '];
T['VPBROADCASTB    VREG:w:u8=$YMMr   PREG:r:Z=$Kk                                VREG:r:u8=$XMMm                            ', 'MOD=REG       ', 'VPBROADCASTB_rymx   ', '           '];
T['VPBROADCASTB    VREG:w:u8=$YMMr   PREG:r:Z=$Kk                                MEM:r:u8=$mem8 SBCST:SUPP={FROM=1 TO=32}   ', 'MOD=MEM       ', 'VPBROADCASTB_rymx   ', 'tuple=T1S8 '];
T['VPBROADCASTB    VREG:w:u8=$ZMMr   PREG:r:Z=$Kk                                REG:r:u8=$GPR32m                           ', 'NONE          ', 'VPBROADCASTB_rzmd   ', '           '];
T['VPBROADCASTB    VREG:w:u8=$ZMMr   PREG:r:Z=$Kk                                VREG:r:u8=$XMMm                            ', 'MOD=REG       ', 'VPBROADCASTB_rzmx   ', '           '];
T['VPBROADCASTB    VREG:w:u8=$ZMMr   PREG:r:Z=$Kk                                MEM:r:u8=$mem8 SBCST:SUPP={FROM=1 TO=64}   ', 'MOD=MEM       ', 'VPBROADCASTB_rzmx   ', 'tuple=T1S8 '];
T['VPBROADCASTD    VREG:w:u32=$XMMr  PREG:r:Z=$Kk                                VREG:r:u32=$XMMm                           ', 'MOD=REG       ', 'VPBROADCASTD_romx   ', '           '];
T['VPBROADCASTD    VREG:w:u32=$XMMr  PREG:r:Z=$Kk                                MEM:r:u32=$mem32 SBCST:SUPP={FROM=1 TO=4}  ', 'MOD=MEM       ', 'VPBROADCASTD_romx   ', 'tuple=T1F32'];
T['VPBROADCASTD    VREG:w:u32=$XMMr  REG:r:u32=$XMMm                                                                        ', 'MOD=REG VL=128', 'VPBROADCASTD_rxmd   ', '           '];
T['VPBROADCASTD    VREG:w:u32=$YMMr  REG:r:u32=$XMMm                                                                        ', 'MOD=REG VL=256', 'VPBROADCASTD_rxmd   ', '           '];
T['VPBROADCASTD    VREG:w:u32=$XMMr  MEM:r:u32=$mem32 SBCST:SUPP={FROM=1 TO=4}                                              ', 'MOD=MEM VL=128', 'VPBROADCASTD_rxmd   ', '           '];
T['VPBROADCASTD    VREG:w:u32=$YMMr  MEM:r:u32=$mem32 SBCST:SUPP={FROM=1 TO=8}                                              ', 'MOD=MEM VL=256', 'VPBROADCASTD_rxmd   ', '           '];
T['VPBROADCASTD    VREG:w:u32=$YMMr  PREG:r:Z=$Kk                                VREG:r:u32=$XMMm                           ', 'MOD=REG       ', 'VPBROADCASTD_rymx   ', '           '];
T['VPBROADCASTD    VREG:w:u32=$YMMr  PREG:r:Z=$Kk                                MEM:r:u32=$mem32 SBCST:SUPP={FROM=1 TO=8}  ', 'MOD=MEM       ', 'VPBROADCASTD_rymx   ', 'tuple=T1F32'];
T['VPBROADCASTD    VREG:w:u32=$ZMMr  PREG:r:Z=$Kk                                VREG:r:u32=$XMMm                           ', 'MOD=REG       ', 'VPBROADCASTD_rzmx   ', '           '];
T['VPBROADCASTD    VREG:w:u32=$ZMMr  PREG:r:Z=$Kk                                MEM:r:u32=$mem32 SBCST:SUPP={FROM=1 TO=16} ', 'MOD=MEM       ', 'VPBROADCASTD_rzmx   ', 'tuple=T1F32'];
T['VPBROADCASTQ    VREG:w:u64=$XMMr  PREG:r:Z=$Kk                                VREG:r:u64=$XMMm                           ', 'MOD=REG       ', 'VPBROADCASTQ_romx   ', '           '];
T['VPBROADCASTQ    VREG:w:u64=$XMMr  PREG:r:Z=$Kk                                MEM:r:u64=$mem64 SBCST:SUPP={FROM=1 TO=2}  ', 'MOD=MEM       ', 'VPBROADCASTQ_romx   ', 'tuple=T1F64'];
T['VPBROADCASTQ    VREG:w:u64=$XMMr  REG:r:u64=$XMMm                                                                        ', 'MOD=REG VL=128', 'VPBROADCASTQ_rxmq   ', '           '];
T['VPBROADCASTQ    VREG:w:u64=$YMMr  REG:r:u64=$XMMm                                                                        ', 'MOD=REG VL=256', 'VPBROADCASTQ_rxmq   ', '           '];
T['VPBROADCASTQ    VREG:w:u64=$XMMr  MEM:r:u64=$mem64 SBCST:SUPP={FROM=1 TO=2}                                              ', 'MOD=MEM VL=128', 'VPBROADCASTQ_rxmq   ', '           '];
T['VPBROADCASTQ    VREG:w:u64=$YMMr  MEM:r:u64=$mem64 SBCST:SUPP={FROM=1 TO=4}                                              ', 'MOD=MEM VL=256', 'VPBROADCASTQ_rxmq   ', '           '];
T['VPBROADCASTQ    VREG:w:u64=$YMMr  PREG:r:Z=$Kk                                VREG:r:u64=$XMMm                           ', 'MOD=REG       ', 'VPBROADCASTQ_rymx   ', '           '];
T['VPBROADCASTQ    VREG:w:u64=$YMMr  PREG:r:Z=$Kk                                MEM:r:u64=$mem64 SBCST:SUPP={FROM=1 TO=4}  ', 'MOD=MEM       ', 'VPBROADCASTQ_rymx   ', 'tuple=T1F64'];
T['VPBROADCASTQ    VREG:w:u64=$ZMMr  PREG:r:Z=$Kk                                VREG:r:u64=$XMMm                           ', 'MOD=REG       ', 'VPBROADCASTQ_rzmx   ', '           '];
T['VPBROADCASTQ    VREG:w:u64=$ZMMr  PREG:r:Z=$Kk                                MEM:r:u64=$mem64 SBCST:SUPP={FROM=1 TO=8}  ', 'MOD=MEM       ', 'VPBROADCASTQ_rzmx   ', 'tuple=T1F64'];
T['VPBROADCASTW    VREG:w:u16=$XMMr  PREG:r:Z=$Kk                                REG:r:u16=$GPR32m                          ', 'NONE          ', 'VPBROADCASTW_romd   ', '           '];
T['VPBROADCASTW    VREG:w:u16=$XMMr  PREG:r:Z=$Kk                                VREG:r:u16=$XMMm                           ', 'MOD=REG       ', 'VPBROADCASTW_romx   ', '           '];
T['VPBROADCASTW    VREG:w:u16=$XMMr  PREG:r:Z=$Kk                                MEM:r:u16=$mem16 SBCST:SUPP={FROM=1 TO=8}  ', 'MOD=MEM       ', 'VPBROADCASTW_romx   ', 'tuple=T1S16'];
T['VPBROADCASTW    VREG:w:u16=$XMMr  REG:r:u16=$XMMm                                                                        ', 'MOD=REG VL=128', 'VPBROADCASTW_rxmw   ', '           '];
T['VPBROADCASTW    VREG:w:u16=$YMMr  REG:r:u16=$XMMm                                                                        ', 'MOD=REG VL=256', 'VPBROADCASTW_rxmw   ', '           '];
T['VPBROADCASTW    VREG:w:u16=$XMMr  MEM:r:u16=$mem16 SBCST:SUPP={FROM=1 TO=8}                                              ', 'MOD=MEM VL=128', 'VPBROADCASTW_rxmw   ', '           '];
T['VPBROADCASTW    VREG:w:u16=$YMMr  MEM:r:u16=$mem16 SBCST:SUPP={FROM=1 TO=16}                                             ', 'MOD=MEM VL=256', 'VPBROADCASTW_rxmw   ', '           '];
T['VPBROADCASTW    VREG:w:u16=$YMMr  PREG:r:Z=$Kk                                REG:r:u16=$GPR32m                          ', 'NONE          ', 'VPBROADCASTW_rymd   ', '           '];
T['VPBROADCASTW    VREG:w:u16=$YMMr  PREG:r:Z=$Kk                                VREG:r:u16=$XMMm                           ', 'MOD=REG       ', 'VPBROADCASTW_rymx   ', '           '];
T['VPBROADCASTW    VREG:w:u16=$YMMr  PREG:r:Z=$Kk                                MEM:r:u16=$mem16 SBCST:SUPP={FROM=1 TO=16} ', 'MOD=MEM       ', 'VPBROADCASTW_rymx   ', 'tuple=T1S16'];
T['VPBROADCASTW    VREG:w:u16=$ZMMr  PREG:r:Z=$Kk                                REG:r:u16=$GPR32m                          ', 'NONE          ', 'VPBROADCASTW_rzmd   ', '           '];
T['VPBROADCASTW    VREG:w:u16=$ZMMr  PREG:r:Z=$Kk                                VREG:r:u16=$XMMm                           ', 'MOD=REG       ', 'VPBROADCASTW_rzmx   ', '           '];
T['VPBROADCASTW    VREG:w:u16=$ZMMr  PREG:r:Z=$Kk                                MEM:r:u16=$mem16 SBCST:SUPP={FROM=1 TO=32} ', 'MOD=MEM       ', 'VPBROADCASTW_rzmx   ', 'tuple=T1S16'];
T['VPBROADCASTD    VREG:w:u32=$XMMr  PREG:r:Z=$Kk                                REG:r:u32=$GPR32m                          ', 'W=0           ', 'VPBROADCASTx_romx   ', '           '];
T['VPBROADCASTD    VREG:w:u32=$XMMr  PREG:r:Z=$Kk                                REG:r:u32=$GPR32m                          ', 'MODE=NO64 W=1 ', 'VPBROADCASTx_romx   ', '           '];
T['VPBROADCASTQ    VREG:w:u64=$XMMr  PREG:r:Z=$Kk                                REG:r:u64=$GPR64m                          ', 'MODE=64 W=1   ', 'VPBROADCASTx_romx   ', '           '];
T['VPBROADCASTD    VREG:w:u32=$YMMr  PREG:r:Z=$Kk                                REG:r:u32=$GPR32m                          ', 'W=0           ', 'VPBROADCASTx_rymx   ', '           '];
T['VPBROADCASTD    VREG:w:u32=$YMMr  PREG:r:Z=$Kk                                REG:r:u32=$GPR32m                          ', 'MODE=NO64 W=1 ', 'VPBROADCASTx_rymx   ', '           '];
T['VPBROADCASTQ    VREG:w:u64=$YMMr  PREG:r:Z=$Kk                                REG:r:u64=$GPR64m                          ', 'MODE=64 W=1   ', 'VPBROADCASTx_rymx   ', '           '];
T['VPBROADCASTD    VREG:w:u32=$ZMMr  PREG:r:Z=$Kk                                REG:r:u32=$GPR32m                          ', 'W=0           ', 'VPBROADCASTx_rzmx   ', '           '];
T['VPBROADCASTD    VREG:w:u32=$ZMMr  PREG:r:Z=$Kk                                REG:r:u32=$GPR32m                          ', 'MODE=NO64 W=1 ', 'VPBROADCASTx_rzmx   ', '           '];
T['VPBROADCASTQ    VREG:w:u64=$ZMMr  PREG:r:Z=$Kk                                REG:r:u64=$GPR64m                          ', 'MODE=64 W=1   ', 'VPBROADCASTx_rzmx   ', '           '];

# VPBROADCASTM-Broadcast Mask to Vector Register.
T['VPBROADCASTMB2Q VREG:w:u64=$XMMr REG:r:u64=$Km', 'NONE', 'VPBROADCASTMB2Q_ro', ''];
T['VPBROADCASTMB2Q VREG:w:u64=$YMMr REG:r:u64=$Km', 'NONE', 'VPBROADCASTMB2Q_ry', ''];
T['VPBROADCASTMB2Q VREG:w:u64=$ZMMr REG:r:u64=$Km', 'NONE', 'VPBROADCASTMB2Q_rz', ''];
T['VPBROADCASTMW2D VREG:w:u32=$XMMr REG:r:u32=$Km', 'NONE', 'VPBROADCASTMW2D_ro', ''];
T['VPBROADCASTMW2D VREG:w:u32=$YMMr REG:r:u32=$Km', 'NONE', 'VPBROADCASTMW2D_ry', ''];
T['VPBROADCASTMW2D VREG:w:u32=$ZMMr REG:r:u32=$Km', 'NONE', 'VPBROADCASTMW2D_rz', ''];

# VPCMPB/VPCMPUB-Compare Packed Byte Values Into Mask.
T['VPCMPB  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s8=$XMMn VREG:r:s8=$XMMm  CC:u8=$cc3', 'MOD=REG', 'VPCMPB_nomoub ', '         '];
T['VPCMPB  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s8=$XMMn MEM:r:s8=$mem128 CC:u8=$cc3', 'MOD=MEM', 'VPCMPB_nomoub ', 'tuple=FVM'];
T['VPCMPB  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s8=$YMMn VREG:r:s8=$YMMm  CC:u8=$cc3', 'MOD=REG', 'VPCMPB_nymyub ', '         '];
T['VPCMPB  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s8=$YMMn MEM:r:s8=$mem256 CC:u8=$cc3', 'MOD=MEM', 'VPCMPB_nymyub ', 'tuple=FVM'];
T['VPCMPB  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s8=$ZMMn VREG:r:s8=$ZMMm  CC:u8=$cc3', 'MOD=REG', 'VPCMPB_nzmzub ', '         '];
T['VPCMPB  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s8=$ZMMn MEM:r:s8=$mem512 CC:u8=$cc3', 'MOD=MEM', 'VPCMPB_nzmzub ', 'tuple=FVM'];
T['VPCMPUB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$XMMn VREG:r:u8=$XMMm  CC:u8=$cc3', 'MOD=REG', 'VPCMPUB_nomoub', '         '];
T['VPCMPUB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$XMMn MEM:r:u8=$mem128 CC:u8=$cc3', 'MOD=MEM', 'VPCMPUB_nomoub', 'tuple=FVM'];
T['VPCMPUB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$YMMn VREG:r:u8=$YMMm  CC:u8=$cc3', 'MOD=REG', 'VPCMPUB_nymyub', '         '];
T['VPCMPUB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$YMMn MEM:r:u8=$mem256 CC:u8=$cc3', 'MOD=MEM', 'VPCMPUB_nymyub', 'tuple=FVM'];
T['VPCMPUB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$ZMMn VREG:r:u8=$ZMMm  CC:u8=$cc3', 'MOD=REG', 'VPCMPUB_nzmzub', '         '];
T['VPCMPUB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$ZMMn MEM:r:u8=$mem512 CC:u8=$cc3', 'MOD=MEM', 'VPCMPUB_nzmzub', 'tuple=FVM'];

# VPCMPD/VPCMPUD-Compare Packed Integer Values into Mask.
T['VPCMPD  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$XMMn VREG:r:s32=$XMMm             CC:u8=$cc3', 'BP=0 MOD=REG', 'VPCMPD_nomoub ', '        '];
T['VPCMPD  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$XMMn MEM:r:s32=$mem128 BCST=$BCST CC:u8=$cc3', 'MOD=MEM     ', 'VPCMPD_nomoub ', 'tuple=FV'];
T['VPCMPD  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$YMMn VREG:r:s32=$YMMm             CC:u8=$cc3', 'BP=0 MOD=REG', 'VPCMPD_nymyub ', '        '];
T['VPCMPD  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$YMMn MEM:r:s32=$mem256 BCST=$BCST CC:u8=$cc3', 'MOD=MEM     ', 'VPCMPD_nymyub ', 'tuple=FV'];
T['VPCMPD  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$ZMMn VREG:r:s32=$ZMMm             CC:u8=$cc3', 'BP=0 MOD=REG', 'VPCMPD_nzmzub ', '        '];
T['VPCMPD  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$ZMMn MEM:r:s32=$mem512 BCST=$BCST CC:u8=$cc3', 'MOD=MEM     ', 'VPCMPD_nzmzub ', 'tuple=FV'];
T['VPCMPUD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$XMMn VREG:r:u32=$XMMm             CC:u8=$cc3', 'BP=0 MOD=REG', 'VPCMPUD_nomoub', '        '];
T['VPCMPUD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST CC:u8=$cc3', 'MOD=MEM     ', 'VPCMPUD_nomoub', 'tuple=FV'];
T['VPCMPUD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm             CC:u8=$cc3', 'BP=0 MOD=REG', 'VPCMPUD_nymyub', '        '];
T['VPCMPUD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST CC:u8=$cc3', 'MOD=MEM     ', 'VPCMPUD_nymyub', 'tuple=FV'];
T['VPCMPUD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm             CC:u8=$cc3', 'BP=0 MOD=REG', 'VPCMPUD_nzmzub', '        '];
T['VPCMPUD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST CC:u8=$cc3', 'MOD=MEM     ', 'VPCMPUD_nzmzub', 'tuple=FV'];

# VPCMPQ/VPCMPUQ-Compare Packed Integer Values into Mask.
T['VPCMPQ  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s64=$XMMn VREG:r:s64=$XMMm             CC:u8=$cc3', 'BP=0 MOD=REG', 'VPCMPQ_nomoub ', '        '];
T['VPCMPQ  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s64=$XMMn MEM:r:s64=$mem128 BCST=$BCST CC:u8=$cc3', 'MOD=MEM     ', 'VPCMPQ_nomoub ', 'tuple=FV'];
T['VPCMPQ  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s64=$YMMn VREG:r:s64=$YMMm             CC:u8=$cc3', 'BP=0 MOD=REG', 'VPCMPQ_nymyub ', '        '];
T['VPCMPQ  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s64=$YMMn MEM:r:s64=$mem256 BCST=$BCST CC:u8=$cc3', 'MOD=MEM     ', 'VPCMPQ_nymyub ', 'tuple=FV'];
T['VPCMPQ  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s64=$ZMMn VREG:r:s64=$ZMMm             CC:u8=$cc3', 'BP=0 MOD=REG', 'VPCMPQ_nzmzub ', '        '];
T['VPCMPQ  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s64=$ZMMn MEM:r:s64=$mem512 BCST=$BCST CC:u8=$cc3', 'MOD=MEM     ', 'VPCMPQ_nzmzub ', 'tuple=FV'];
T['VPCMPUQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm             CC:u8=$cc3', 'BP=0 MOD=REG', 'VPCMPUQ_nomoub', '        '];
T['VPCMPUQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST CC:u8=$cc3', 'MOD=MEM     ', 'VPCMPUQ_nomoub', 'tuple=FV'];
T['VPCMPUQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm             CC:u8=$cc3', 'BP=0 MOD=REG', 'VPCMPUQ_nymyub', '        '];
T['VPCMPUQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST CC:u8=$cc3', 'MOD=MEM     ', 'VPCMPUQ_nymyub', 'tuple=FV'];
T['VPCMPUQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm             CC:u8=$cc3', 'BP=0 MOD=REG', 'VPCMPUQ_nzmzub', '        '];
T['VPCMPUQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST CC:u8=$cc3', 'MOD=MEM     ', 'VPCMPUQ_nzmzub', 'tuple=FV'];

# VPCMPW/VPCMPUW-Compare Packed Word Values Into Mask.
T['VPCMPUW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$XMMn VREG:r:u16=$XMMm  CC:u8=$cc3', 'MOD=REG', 'VPCMPUW_nomoub', '         '];
T['VPCMPUW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$XMMn MEM:r:u16=$mem128 CC:u8=$cc3', 'MOD=MEM', 'VPCMPUW_nomoub', 'tuple=FVM'];
T['VPCMPUW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$YMMn VREG:r:u16=$YMMm  CC:u8=$cc3', 'MOD=REG', 'VPCMPUW_nymyub', '         '];
T['VPCMPUW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$YMMn MEM:r:u16=$mem256 CC:u8=$cc3', 'MOD=MEM', 'VPCMPUW_nymyub', 'tuple=FVM'];
T['VPCMPUW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$ZMMn VREG:r:u16=$ZMMm  CC:u8=$cc3', 'MOD=REG', 'VPCMPUW_nzmzub', '         '];
T['VPCMPUW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$ZMMn MEM:r:u16=$mem512 CC:u8=$cc3', 'MOD=MEM', 'VPCMPUW_nzmzub', 'tuple=FVM'];
T['VPCMPW  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s16=$XMMn VREG:r:s16=$XMMm  CC:u8=$cc3', 'MOD=REG', 'VPCMPW_nomoub ', '         '];
T['VPCMPW  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s16=$XMMn MEM:r:s16=$mem128 CC:u8=$cc3', 'MOD=MEM', 'VPCMPW_nomoub ', 'tuple=FVM'];
T['VPCMPW  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s16=$YMMn VREG:r:s16=$YMMm  CC:u8=$cc3', 'MOD=REG', 'VPCMPW_nymyub ', '         '];
T['VPCMPW  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s16=$YMMn MEM:r:s16=$mem256 CC:u8=$cc3', 'MOD=MEM', 'VPCMPW_nymyub ', 'tuple=FVM'];
T['VPCMPW  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s16=$ZMMn VREG:r:s16=$ZMMm  CC:u8=$cc3', 'MOD=REG', 'VPCMPW_nzmzub ', '         '];
T['VPCMPW  REG:w=$Kr PREG:r:Z=$Kk VREG:r:s16=$ZMMn MEM:r:s16=$mem512 CC:u8=$cc3', 'MOD=MEM', 'VPCMPW_nzmzub ', 'tuple=FVM'];

# VPCOMPRESSB/VCOMPRESSW-Store Sparse Packed Byte/Word Integer Values into Dense Memory/Register.
T['VPCOMPRESSB VREG:w:u8=$XMMm   PREG:r:Z=$Kk VREG:r:u8=$XMMr ', 'MOD=REG W=0', 'VPCOMPRESSx_moro', '           '];
T['VPCOMPRESSB MEM:w:u8=$mem128  PREG:r:Z=$Kk VREG:r:u8=$XMMr ', 'MOD=MEM W=0', 'VPCOMPRESSx_moro', 'tuple=T1S8 '];
T['VPCOMPRESSW VREG:w:u16=$XMMm  PREG:r:Z=$Kk VREG:r:u16=$XMMr', 'MOD=REG W=1', 'VPCOMPRESSx_moro', '           '];
T['VPCOMPRESSW MEM:w:u16=$mem128 PREG:r:Z=$Kk VREG:r:u16=$XMMr', 'MOD=MEM W=1', 'VPCOMPRESSx_moro', 'tuple=T1S16'];
T['VPCOMPRESSB VREG:w:u8=$YMMm   PREG:r:Z=$Kk VREG:r:u8=$YMMr ', 'MOD=REG W=0', 'VPCOMPRESSx_myry', '           '];
T['VPCOMPRESSB MEM:w:u8=$mem256  PREG:r:Z=$Kk VREG:r:u8=$YMMr ', 'MOD=MEM W=0', 'VPCOMPRESSx_myry', 'tuple=T1S8 '];
T['VPCOMPRESSW VREG:w:u16=$YMMm  PREG:r:Z=$Kk VREG:r:u16=$YMMr', 'MOD=REG W=1', 'VPCOMPRESSx_myry', '           '];
T['VPCOMPRESSW MEM:w:u16=$mem256 PREG:r:Z=$Kk VREG:r:u16=$YMMr', 'MOD=MEM W=1', 'VPCOMPRESSx_myry', 'tuple=T1S16'];
T['VPCOMPRESSB VREG:w:u8=$ZMMm   PREG:r:Z=$Kk VREG:r:u8=$ZMMr ', 'MOD=REG W=0', 'VPCOMPRESSx_mzrz', '           '];
T['VPCOMPRESSB MEM:w:u8=$mem512  PREG:r:Z=$Kk VREG:r:u8=$ZMMr ', 'MOD=MEM W=0', 'VPCOMPRESSx_mzrz', 'tuple=T1S8 '];
T['VPCOMPRESSW VREG:w:u16=$ZMMm  PREG:r:Z=$Kk VREG:r:u16=$ZMMr', 'MOD=REG W=1', 'VPCOMPRESSx_mzrz', '           '];
T['VPCOMPRESSW MEM:w:u16=$mem512 PREG:r:Z=$Kk VREG:r:u16=$ZMMr', 'MOD=MEM W=1', 'VPCOMPRESSx_mzrz', 'tuple=T1S16'];

# VPCOMPRESSD-Store Sparse Packed Doubleword Integer Values into Dense Memory/Register.
T['VPCOMPRESSD VREG:w:u32=$XMMm  PREG:r:Z=$Kk VREG:r:u32=$XMMr', 'MOD=REG', 'VPCOMPRESSD_moro', '         '];
T['VPCOMPRESSD MEM:w:u32=$mem128 PREG:r:Z=$Kk VREG:r:u32=$XMMr', 'MOD=MEM', 'VPCOMPRESSD_moro', 'tuple=T1S'];
T['VPCOMPRESSD VREG:w:u32=$YMMm  PREG:r:Z=$Kk VREG:r:u32=$YMMr', 'MOD=REG', 'VPCOMPRESSD_myry', '         '];
T['VPCOMPRESSD MEM:w:u32=$mem256 PREG:r:Z=$Kk VREG:r:u32=$YMMr', 'MOD=MEM', 'VPCOMPRESSD_myry', 'tuple=T1S'];
T['VPCOMPRESSD VREG:w:u32=$ZMMm  PREG:r:Z=$Kk VREG:r:u32=$ZMMr', 'MOD=REG', 'VPCOMPRESSD_mzrz', '         '];
T['VPCOMPRESSD MEM:w:u32=$mem512 PREG:r:Z=$Kk VREG:r:u32=$ZMMr', 'MOD=MEM', 'VPCOMPRESSD_mzrz', 'tuple=T1S'];

# VPCOMPRESSQ-Store Sparse Packed Quadword Integer Values into Dense Memory/Register.
T['VPCOMPRESSQ VREG:w:u64=$XMMm  PREG:r:Z=$Kk VREG:r:u64=$XMMr', 'MOD=REG', 'VPCOMPRESSQ_moro', '         '];
T['VPCOMPRESSQ MEM:w:u64=$mem128 PREG:r:Z=$Kk VREG:r:u64=$XMMr', 'MOD=MEM', 'VPCOMPRESSQ_moro', 'tuple=T1S'];
T['VPCOMPRESSQ VREG:w:u64=$YMMm  PREG:r:Z=$Kk VREG:r:u64=$YMMr', 'MOD=REG', 'VPCOMPRESSQ_myry', '         '];
T['VPCOMPRESSQ MEM:w:u64=$mem256 PREG:r:Z=$Kk VREG:r:u64=$YMMr', 'MOD=MEM', 'VPCOMPRESSQ_myry', 'tuple=T1S'];
T['VPCOMPRESSQ VREG:w:u64=$ZMMm  PREG:r:Z=$Kk VREG:r:u64=$ZMMr', 'MOD=REG', 'VPCOMPRESSQ_mzrz', '         '];
T['VPCOMPRESSQ MEM:w:u64=$mem512 PREG:r:Z=$Kk VREG:r:u64=$ZMMr', 'MOD=MEM', 'VPCOMPRESSQ_mzrz', 'tuple=T1S'];

# VPCONFLICTD/Q-Detect Conflicts Within a Vector of Packed Dword/Qword Values into Dense Memory/ Register. 5-334.
T['VPCONFLICTD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPCONFLICTx_romo', '        '];
T['VPCONFLICTD VREG:w:u32=$XMMr PREG:r:Z=$Kk MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPCONFLICTx_romo', 'tuple=FV'];
T['VPCONFLICTQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPCONFLICTx_romo', '        '];
T['VPCONFLICTQ VREG:w:u64=$XMMr PREG:r:Z=$Kk MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPCONFLICTx_romo', 'tuple=FV'];
T['VPCONFLICTD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPCONFLICTx_rymy', '        '];
T['VPCONFLICTD VREG:w:u32=$YMMr PREG:r:Z=$Kk MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPCONFLICTx_rymy', 'tuple=FV'];
T['VPCONFLICTQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPCONFLICTx_rymy', '        '];
T['VPCONFLICTQ VREG:w:u64=$YMMr PREG:r:Z=$Kk MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPCONFLICTx_rymy', 'tuple=FV'];
T['VPCONFLICTD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPCONFLICTx_rzmz', '        '];
T['VPCONFLICTD VREG:w:u32=$ZMMr PREG:r:Z=$Kk MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPCONFLICTx_rzmz', 'tuple=FV'];
T['VPCONFLICTQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPCONFLICTx_rzmz', '        '];
T['VPCONFLICTQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPCONFLICTx_rzmz', 'tuple=FV'];

# VPDPBUSD-Multiply and Add Unsigned and Signed Bytes.
T['VPDPBUSD VREG:rw:s32=$XMMr PREG:r:Z=$Kk VREG:r:u8=$XMMn VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VPDPBUSD_ronomo', '        '];
T['VPDPBUSD VREG:rw:s32=$XMMr PREG:r:Z=$Kk VREG:r:u8=$XMMn MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPDPBUSD_ronomo', 'tuple=FV'];
T['VPDPBUSD VREG:rw:s32=$YMMr PREG:r:Z=$Kk VREG:r:u8=$YMMn VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VPDPBUSD_rynymy', '        '];
T['VPDPBUSD VREG:rw:s32=$YMMr PREG:r:Z=$Kk VREG:r:u8=$YMMn MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPDPBUSD_rynymy', 'tuple=FV'];
T['VPDPBUSD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:u8=$ZMMn VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VPDPBUSD_rznzmz', '        '];
T['VPDPBUSD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:u8=$ZMMn MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPDPBUSD_rznzmz', 'tuple=FV'];

# VPDPBUSDS-Multiply and Add Unsigned and Signed Bytes with Saturation.
T['VPDPBUSDS VREG:rw:s32=$XMMr PREG:r:Z=$Kk VREG:r:u8=$XMMn VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VPDPBUSDS_ronomo', '        '];
T['VPDPBUSDS VREG:rw:s32=$XMMr PREG:r:Z=$Kk VREG:r:u8=$XMMn MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPDPBUSDS_ronomo', 'tuple=FV'];
T['VPDPBUSDS VREG:rw:s32=$YMMr PREG:r:Z=$Kk VREG:r:u8=$YMMn VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VPDPBUSDS_rynymy', '        '];
T['VPDPBUSDS VREG:rw:s32=$YMMr PREG:r:Z=$Kk VREG:r:u8=$YMMn MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPDPBUSDS_rynymy', 'tuple=FV'];
T['VPDPBUSDS VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:u8=$ZMMn VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VPDPBUSDS_rznzmz', '        '];
T['VPDPBUSDS VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:u8=$ZMMn MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPDPBUSDS_rznzmz', 'tuple=FV'];

# VPDPWSSD-Multiply and Add Signed Word Integers.
T['VPDPWSSD VREG:rw:s32=$XMMr PREG:r:Z=$Kk VREG:r:s16=$XMMn VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VPDPWSSD_ronomo', '        '];
T['VPDPWSSD VREG:rw:s32=$XMMr PREG:r:Z=$Kk VREG:r:s16=$XMMn MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPDPWSSD_ronomo', 'tuple=FV'];
T['VPDPWSSD VREG:rw:s32=$YMMr PREG:r:Z=$Kk VREG:r:s16=$YMMn VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VPDPWSSD_rynymy', '        '];
T['VPDPWSSD VREG:rw:s32=$YMMr PREG:r:Z=$Kk VREG:r:s16=$YMMn MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPDPWSSD_rynymy', 'tuple=FV'];
T['VPDPWSSD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s16=$ZMMn VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VPDPWSSD_rznzmz', '        '];
T['VPDPWSSD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s16=$ZMMn MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPDPWSSD_rznzmz', 'tuple=FV'];

# VPDPWSSDS-Multiply and Add Signed Word Integers with Saturation.
T['VPDPWSSDS VREG:rw:s32=$XMMr PREG:r:Z=$Kk VREG:r:s16=$XMMn VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VPDPWSSDS_ronomo', '        '];
T['VPDPWSSDS VREG:rw:s32=$XMMr PREG:r:Z=$Kk VREG:r:s16=$XMMn MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPDPWSSDS_ronomo', 'tuple=FV'];
T['VPDPWSSDS VREG:rw:s32=$YMMr PREG:r:Z=$Kk VREG:r:s16=$YMMn VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VPDPWSSDS_rynymy', '        '];
T['VPDPWSSDS VREG:rw:s32=$YMMr PREG:r:Z=$Kk VREG:r:s16=$YMMn MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPDPWSSDS_rynymy', 'tuple=FV'];
T['VPDPWSSDS VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s16=$ZMMn VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VPDPWSSDS_rznzmz', '        '];
T['VPDPWSSDS VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s16=$ZMMn MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPDPWSSDS_rznzmz', 'tuple=FV'];

# VPERM2F128-Permute Floating-Point Values.
T['VPERM2F128 VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm  CTL:r:u8=$ctl', 'MOD=REG', 'VPERM2F128', ''];
T['VPERM2F128 VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256 CTL:r:u8=$ctl', 'MOD=MEM', 'VPERM2F128', ''];

# VPERM2I128-Permute Integer Values.
T['VPERM2I128 VREG:w:u128=$YMMr VREG:r:u128=$YMMn VREG:r:u128=$YMMm  CTL:r:u8=$ctl', 'MOD=REG', 'VPERM2I128', ''];
T['VPERM2I128 VREG:w:u128=$YMMr VREG:r:u128=$YMMn MEM:r:u128=$mem256 CTL:r:u8=$ctl', 'MOD=MEM', 'VPERM2I128', ''];

# VPERMB-Permute Packed Bytes Elements.
T['VPERMB VREG:w:u8=$XMMr PREG:r:Z=$Kk VREG:r:u8=$XMMn VREG:r:u8=$XMMm ', 'MOD=REG', 'VPERMB_ronomo', '         '];
T['VPERMB VREG:w:u8=$XMMr PREG:r:Z=$Kk VREG:r:u8=$XMMn MEM:r:u8=$mem128', 'MOD=MEM', 'VPERMB_ronomo', 'tuple=FVM'];
T['VPERMB VREG:w:u8=$YMMr PREG:r:Z=$Kk VREG:r:u8=$YMMn VREG:r:u8=$YMMm ', 'MOD=REG', 'VPERMB_rynymy', '         '];
T['VPERMB VREG:w:u8=$YMMr PREG:r:Z=$Kk VREG:r:u8=$YMMn MEM:r:u8=$mem256', 'MOD=MEM', 'VPERMB_rynymy', 'tuple=FVM'];
T['VPERMB VREG:w:u8=$ZMMr PREG:r:Z=$Kk VREG:r:u8=$ZMMn VREG:r:u8=$ZMMm ', 'MOD=REG', 'VPERMB_rznzmz', '         '];
T['VPERMB VREG:w:u8=$ZMMr PREG:r:Z=$Kk VREG:r:u8=$ZMMn MEM:r:u8=$mem512', 'MOD=MEM', 'VPERMB_rznzmz', 'tuple=FVM'];

# VPERMI2B-Full Permute of Bytes from Two Tables Overwriting the Index.
T['VPERMI2B VREG:rw:u8=$XMMr PREG:r:Z=$Kk VREG:r:u8=$XMMn VREG:r:u8=$XMMm ', 'MOD=REG', 'VPERMI2B_ronomo', '         '];
T['VPERMI2B VREG:rw:u8=$XMMr PREG:r:Z=$Kk VREG:r:u8=$XMMn MEM:r:u8=$mem128', 'MOD=MEM', 'VPERMI2B_ronomo', 'tuple=FVM'];
T['VPERMI2B VREG:rw:u8=$YMMr PREG:r:Z=$Kk VREG:r:u8=$YMMn VREG:r:u8=$YMMm ', 'MOD=REG', 'VPERMI2B_rynymy', '         '];
T['VPERMI2B VREG:rw:u8=$YMMr PREG:r:Z=$Kk VREG:r:u8=$YMMn MEM:r:u8=$mem256', 'MOD=MEM', 'VPERMI2B_rynymy', 'tuple=FVM'];
T['VPERMI2B VREG:rw:u8=$ZMMr PREG:r:Z=$Kk VREG:r:u8=$ZMMn VREG:r:u8=$ZMMm ', 'MOD=REG', 'VPERMI2B_rznzmz', '         '];
T['VPERMI2B VREG:rw:u8=$ZMMr PREG:r:Z=$Kk VREG:r:u8=$ZMMn MEM:r:u8=$mem512', 'MOD=MEM', 'VPERMI2B_rznzmz', 'tuple=FVM'];

# VPERMI2W/D/Q/PS/PD-Full Permute From Two Tables Overwriting the Index.
T['VPERMI2PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPERMI2Px_ronomo', '         '];
T['VPERMI2PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPERMI2Px_ronomo', 'tuple=FV '];
T['VPERMI2PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPERMI2Px_ronomo', '         '];
T['VPERMI2PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPERMI2Px_ronomo', 'tuple=FV '];
T['VPERMI2PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMn VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPERMI2Px_rynymy', '         '];
T['VPERMI2PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMn MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPERMI2Px_rynymy', 'tuple=FV '];
T['VPERMI2PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMn VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPERMI2Px_rynymy', '         '];
T['VPERMI2PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMn MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPERMI2Px_rynymy', 'tuple=FV '];
T['VPERMI2PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPERMI2Px_rznzmz', '         '];
T['VPERMI2PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPERMI2Px_rznzmz', 'tuple=FV '];
T['VPERMI2PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPERMI2Px_rznzmz', '         '];
T['VPERMI2PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPERMI2Px_rznzmz', 'tuple=FV '];
T['VPERMI2W  VREG:rw:u16=$XMMr PREG:r:Z=$Kk VREG:r:u16=$XMMn VREG:r:u16=$XMMm            ', 'MOD=REG         ', 'VPERMI2W_ronomo ', '         '];
T['VPERMI2W  VREG:rw:u16=$XMMr PREG:r:Z=$Kk VREG:r:u16=$XMMn MEM:r:u16=$mem128           ', 'MOD=MEM         ', 'VPERMI2W_ronomo ', 'tuple=FVM'];
T['VPERMI2W  VREG:rw:u16=$YMMr PREG:r:Z=$Kk VREG:r:u16=$YMMn VREG:r:u16=$YMMm            ', 'MOD=REG         ', 'VPERMI2W_rynymy ', '         '];
T['VPERMI2W  VREG:rw:u16=$YMMr PREG:r:Z=$Kk VREG:r:u16=$YMMn MEM:r:u16=$mem256           ', 'MOD=MEM         ', 'VPERMI2W_rynymy ', 'tuple=FVM'];
T['VPERMI2W  VREG:rw:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u16=$ZMMn VREG:r:u16=$ZMMm            ', 'MOD=REG         ', 'VPERMI2W_rznzmz ', '         '];
T['VPERMI2W  VREG:rw:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u16=$ZMMn MEM:r:u16=$mem512           ', 'MOD=MEM         ', 'VPERMI2W_rznzmz ', 'tuple=FVM'];
T['VPERMI2D  VREG:rw:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPERMI2x_ronomo ', '         '];
T['VPERMI2D  VREG:rw:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPERMI2x_ronomo ', 'tuple=FV '];
T['VPERMI2Q  VREG:rw:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPERMI2x_ronomo ', '         '];
T['VPERMI2Q  VREG:rw:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPERMI2x_ronomo ', 'tuple=FV '];
T['VPERMI2D  VREG:rw:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPERMI2x_rynymy ', '         '];
T['VPERMI2D  VREG:rw:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPERMI2x_rynymy ', 'tuple=FV '];
T['VPERMI2Q  VREG:rw:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPERMI2x_rynymy ', '         '];
T['VPERMI2Q  VREG:rw:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPERMI2x_rynymy ', 'tuple=FV '];
T['VPERMI2D  VREG:rw:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPERMI2x_rznzmz ', '         '];
T['VPERMI2D  VREG:rw:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPERMI2x_rznzmz ', 'tuple=FV '];
T['VPERMI2Q  VREG:rw:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPERMI2x_rznzmz ', '         '];
T['VPERMI2Q  VREG:rw:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPERMI2x_rznzmz ', 'tuple=FV '];

# VPERMILPD-Permute In-Lane of Pairs of Double-Precision Floating-Point Values.
T['VPERMILPD VREG:w:f64=$XMMr PREG:r:Z=$Kk      VREG:r:f64=$XMMm             CTL:r:u8=$ctl               ', 'BP=0 MOD=REG  ', 'VPERMILPD_romoub', '        '];
T['VPERMILPD VREG:w:f64=$XMMr PREG:r:Z=$Kk      MEM:r:f64=$mem128 BCST=$BCST CTL:r:u8=$ctl               ', 'MOD=MEM       ', 'VPERMILPD_romoub', 'tuple=FV'];
T['VPERMILPD VREG:w:f64=$XMMr PREG:r:Z=$Kk      VREG:r:f64=$XMMn             VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG  ', 'VPERMILPD_ronomo', '        '];
T['VPERMILPD VREG:w:f64=$XMMr PREG:r:Z=$Kk      VREG:r:f64=$XMMn             MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM       ', 'VPERMILPD_ronomo', 'tuple=FV'];
T['VPERMILPD VREG:w:f64=$XMMr VREG:r:f64=$XMMm  CTL:r:u8=$ctl                                            ', 'MOD=REG VL=128', 'VPERMILPD_rxmxub', '        '];
T['VPERMILPD VREG:w:f64=$YMMr VREG:r:f64=$YMMm  CTL:r:u8=$ctl                                            ', 'MOD=REG VL=256', 'VPERMILPD_rxmxub', '        '];
T['VPERMILPD VREG:w:f64=$XMMr MEM:r:f64=$mem128 CTL:r:u8=$ctl                                            ', 'MOD=MEM VL=128', 'VPERMILPD_rxmxub', '        '];
T['VPERMILPD VREG:w:f64=$YMMr MEM:r:f64=$mem256 CTL:r:u8=$ctl                                            ', 'MOD=MEM VL=256', 'VPERMILPD_rxmxub', '        '];
T['VPERMILPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn  VREG:r:u64=$XMMm                                         ', 'MOD=REG VL=128', 'VPERMILPD_rxnxmx', '        '];
T['VPERMILPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn  VREG:r:u64=$YMMm                                         ', 'MOD=REG VL=256', 'VPERMILPD_rxnxmx', '        '];
T['VPERMILPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn  MEM:r:u64=$mem128                                        ', 'MOD=MEM VL=128', 'VPERMILPD_rxnxmx', '        '];
T['VPERMILPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn  MEM:r:u64=$mem256                                        ', 'MOD=MEM VL=256', 'VPERMILPD_rxnxmx', '        '];
T['VPERMILPD VREG:w:f64=$YMMr PREG:r:Z=$Kk      VREG:r:f64=$YMMm             CTL:r:u8=$ctl               ', 'BP=0 MOD=REG  ', 'VPERMILPD_rymyub', '        '];
T['VPERMILPD VREG:w:f64=$YMMr PREG:r:Z=$Kk      MEM:r:f64=$mem256 BCST=$BCST CTL:r:u8=$ctl               ', 'MOD=MEM       ', 'VPERMILPD_rymyub', 'tuple=FV'];
T['VPERMILPD VREG:w:f64=$YMMr PREG:r:Z=$Kk      VREG:r:f64=$YMMn             VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG  ', 'VPERMILPD_rynymy', '        '];
T['VPERMILPD VREG:w:f64=$YMMr PREG:r:Z=$Kk      VREG:r:f64=$YMMn             MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM       ', 'VPERMILPD_rynymy', 'tuple=FV'];
T['VPERMILPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk      VREG:r:f64=$ZMMm             CTL:r:u8=$ctl               ', 'BP=0 MOD=REG  ', 'VPERMILPD_rzmzub', '        '];
T['VPERMILPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk      MEM:r:f64=$mem512 BCST=$BCST CTL:r:u8=$ctl               ', 'MOD=MEM       ', 'VPERMILPD_rzmzub', 'tuple=FV'];
T['VPERMILPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk      VREG:r:f64=$ZMMn             VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG  ', 'VPERMILPD_rznzmz', '        '];
T['VPERMILPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk      VREG:r:f64=$ZMMn             MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM       ', 'VPERMILPD_rznzmz', 'tuple=FV'];

# VPERMILPS-Permute In-Lane of Quadruples of Single-Precision Floating-Point Values.
T['VPERMILPS VREG:w:f32=$XMMr PREG:r:Z=$Kk      VREG:r:f32=$XMMm             CTL:r:u8=$ctl               ', 'BP=0 MOD=REG  ', 'VPERMILPS_romoub', '        '];
T['VPERMILPS VREG:w:f32=$XMMr PREG:r:Z=$Kk      MEM:r:f32=$mem128 BCST=$BCST CTL:r:u8=$ctl               ', 'MOD=MEM       ', 'VPERMILPS_romoub', 'tuple=FV'];
T['VPERMILPS VREG:w:f32=$XMMr PREG:r:Z=$Kk      VREG:r:f32=$XMMn             VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG  ', 'VPERMILPS_ronomo', '        '];
T['VPERMILPS VREG:w:f32=$XMMr PREG:r:Z=$Kk      VREG:r:f32=$XMMn             MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM       ', 'VPERMILPS_ronomo', 'tuple=FV'];
T['VPERMILPS VREG:w:f32=$XMMr VREG:r:f32=$XMMm  CTL:r:u8=$ctl                                            ', 'MOD=REG VL=128', 'VPERMILPS_rxmxub', '        '];
T['VPERMILPS VREG:w:f32=$YMMr VREG:r:f32=$YMMm  CTL:r:u8=$ctl                                            ', 'MOD=REG VL=256', 'VPERMILPS_rxmxub', '        '];
T['VPERMILPS VREG:w:f32=$XMMr MEM:r:f32=$mem128 CTL:r:u8=$ctl                                            ', 'MOD=MEM VL=128', 'VPERMILPS_rxmxub', '        '];
T['VPERMILPS VREG:w:f32=$YMMr MEM:r:f32=$mem256 CTL:r:u8=$ctl                                            ', 'MOD=MEM VL=256', 'VPERMILPS_rxmxub', '        '];
T['VPERMILPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn  VREG:r:u32=$XMMm                                         ', 'MOD=REG VL=128', 'VPERMILPS_rxnxmx', '        '];
T['VPERMILPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn  VREG:r:u32=$YMMm                                         ', 'MOD=REG VL=256', 'VPERMILPS_rxnxmx', '        '];
T['VPERMILPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn  MEM:r:u32=$mem128                                        ', 'MOD=MEM VL=128', 'VPERMILPS_rxnxmx', '        '];
T['VPERMILPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn  MEM:r:u32=$mem256                                        ', 'MOD=MEM VL=256', 'VPERMILPS_rxnxmx', '        '];
T['VPERMILPS VREG:w:f32=$YMMr PREG:r:Z=$Kk      VREG:r:f32=$YMMm             CTL:r:u8=$ctl               ', 'BP=0 MOD=REG  ', 'VPERMILPS_rymyub', '        '];
T['VPERMILPS VREG:w:f32=$YMMr PREG:r:Z=$Kk      MEM:r:f32=$mem256 BCST=$BCST CTL:r:u8=$ctl               ', 'MOD=MEM       ', 'VPERMILPS_rymyub', 'tuple=FV'];
T['VPERMILPS VREG:w:f32=$YMMr PREG:r:Z=$Kk      VREG:r:f32=$YMMn             VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG  ', 'VPERMILPS_rynymy', '        '];
T['VPERMILPS VREG:w:f32=$YMMr PREG:r:Z=$Kk      VREG:r:f32=$YMMn             MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM       ', 'VPERMILPS_rynymy', 'tuple=FV'];
T['VPERMILPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk      VREG:r:f32=$ZMMm             CTL:r:u8=$ctl               ', 'BP=0 MOD=REG  ', 'VPERMILPS_rzmzub', '        '];
T['VPERMILPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk      MEM:r:f32=$mem512 BCST=$BCST CTL:r:u8=$ctl               ', 'MOD=MEM       ', 'VPERMILPS_rzmzub', 'tuple=FV'];
T['VPERMILPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk      VREG:r:f32=$ZMMn             VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG  ', 'VPERMILPS_rznzmz', '        '];
T['VPERMILPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk      VREG:r:f32=$ZMMn             MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM       ', 'VPERMILPS_rznzmz', 'tuple=FV'];

# VPERMPD-Permute Double-Precision Floating-Point Elements.
T['VPERMPD VREG:w:f64=$YMMr VREG:r:f64=$YMMm  CTL:r:u8=$ctl                                            ', 'MOD=REG     ', 'VPERMPD_rymyub_1', '        '];
T['VPERMPD VREG:w:f64=$YMMr MEM:r:f64=$mem256 CTL:r:u8=$ctl                                            ', 'MOD=MEM     ', 'VPERMPD_rymyub_1', '        '];
T['VPERMPD VREG:w:f64=$YMMr PREG:r:Z=$Kk      VREG:r:f64=$YMMm             CTL:r:u8=$ctl               ', 'BP=0 MOD=REG', 'VPERMPD_rymyub_2', '        '];
T['VPERMPD VREG:w:f64=$YMMr PREG:r:Z=$Kk      MEM:r:f64=$mem256 BCST=$BCST CTL:r:u8=$ctl               ', 'MOD=MEM     ', 'VPERMPD_rymyub_2', 'tuple=FV'];
T['VPERMPD VREG:w:f64=$YMMr PREG:r:Z=$Kk      VREG:r:f64=$YMMn             VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG', 'VPERMPD_rynymy  ', '        '];
T['VPERMPD VREG:w:f64=$YMMr PREG:r:Z=$Kk      VREG:r:f64=$YMMn             MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPERMPD_rynymy  ', 'tuple=FV'];
T['VPERMPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk      VREG:r:f64=$ZMMm             CTL:r:u8=$ctl               ', 'BP=0 MOD=REG', 'VPERMPD_rzmzub  ', '        '];
T['VPERMPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk      MEM:r:f64=$mem512 BCST=$BCST CTL:r:u8=$ctl               ', 'MOD=MEM     ', 'VPERMPD_rzmzub  ', 'tuple=FV'];
T['VPERMPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk      VREG:r:f64=$ZMMn             VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG', 'VPERMPD_rznzmz  ', '        '];
T['VPERMPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk      VREG:r:f64=$ZMMn             MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPERMPD_rznzmz  ', 'tuple=FV'];

# VPERMPS-Permute Single-Precision Floating-Point Elements.
T['VPERMPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm                              ', 'MOD=REG     ', 'VPERMPS_rynymy_1', '        '];
T['VPERMPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256                             ', 'MOD=MEM     ', 'VPERMPS_rynymy_1', '        '];
T['VPERMPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG', 'VPERMPS_rynymy_2', '        '];
T['VPERMPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPERMPS_rynymy_2', 'tuple=FV'];
T['VPERMPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG', 'VPERMPS_rznzmz  ', '        '];
T['VPERMPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPERMPS_rznzmz  ', 'tuple=FV'];

# VPERMQ-Qwords Element Permutation.
T['VPERMQ VREG:w:u64=$YMMr VREG:r:u64=$YMMm  IMM:u8=$uimm8                                            ', 'MOD=REG     ', 'VPERMQ_rymyub_1', '        '];
T['VPERMQ VREG:w:u64=$YMMr MEM:r:u64=$mem256 IMM:u8=$uimm8                                            ', 'MOD=MEM     ', 'VPERMQ_rymyub_1', '        '];
T['VPERMQ VREG:w:u64=$YMMr PREG:r:Z=$Kk      VREG:r:u64=$YMMm             IMM:u8=$uimm8               ', 'BP=0 MOD=REG', 'VPERMQ_rymyub_2', '        '];
T['VPERMQ VREG:w:u64=$YMMr PREG:r:Z=$Kk      MEM:r:u64=$mem256 BCST=$BCST IMM:u8=$uimm8               ', 'MOD=MEM     ', 'VPERMQ_rymyub_2', 'tuple=FV'];
T['VPERMQ VREG:w:u64=$YMMr PREG:r:Z=$Kk      VREG:r:u64=$YMMn             VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG', 'VPERMQ_rynymy  ', '        '];
T['VPERMQ VREG:w:u64=$YMMr PREG:r:Z=$Kk      VREG:r:u64=$YMMn             MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPERMQ_rynymy  ', 'tuple=FV'];
T['VPERMQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk      VREG:r:u64=$ZMMm             IMM:u8=$uimm8               ', 'BP=0 MOD=REG', 'VPERMQ_rzmzub  ', '        '];
T['VPERMQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk      MEM:r:u64=$mem512 BCST=$BCST IMM:u8=$uimm8               ', 'MOD=MEM     ', 'VPERMQ_rzmzub  ', 'tuple=FV'];
T['VPERMQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk      VREG:r:u64=$ZMMn             VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG', 'VPERMQ_rznzmz  ', '        '];
T['VPERMQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk      VREG:r:u64=$ZMMn             MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPERMQ_rznzmz  ', 'tuple=FV'];

# VPERMT2B-Full Permute of Bytes from Two Tables Overwriting a Table.
T['VPERMT2B VREG:rw:u8=$XMMr PREG:r:Z=$Kk VREG:r:u8=$XMMn VREG:r:u8=$XMMm ', 'MOD=REG', 'VPERMT2B_ronomo', '         '];
T['VPERMT2B VREG:rw:u8=$XMMr PREG:r:Z=$Kk VREG:r:u8=$XMMn MEM:r:u8=$mem128', 'MOD=MEM', 'VPERMT2B_ronomo', 'tuple=FVM'];
T['VPERMT2B VREG:rw:u8=$YMMr PREG:r:Z=$Kk VREG:r:u8=$YMMn VREG:r:u8=$YMMm ', 'MOD=REG', 'VPERMT2B_rynymy', '         '];
T['VPERMT2B VREG:rw:u8=$YMMr PREG:r:Z=$Kk VREG:r:u8=$YMMn MEM:r:u8=$mem256', 'MOD=MEM', 'VPERMT2B_rynymy', 'tuple=FVM'];
T['VPERMT2B VREG:rw:u8=$ZMMr PREG:r:Z=$Kk VREG:r:u8=$ZMMn VREG:r:u8=$ZMMm ', 'MOD=REG', 'VPERMT2B_rznzmz', '         '];
T['VPERMT2B VREG:rw:u8=$ZMMr PREG:r:Z=$Kk VREG:r:u8=$ZMMn MEM:r:u8=$mem512', 'MOD=MEM', 'VPERMT2B_rznzmz', 'tuple=FVM'];

# VPERMT2W/D/Q/PS/PD-Full Permute from Two Tables Overwriting one Table.
T['VPERMT2PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPERMT2Px_ronomo', '         '];
T['VPERMT2PD VREG:rw:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPERMT2Px_ronomo', 'tuple=FV '];
T['VPERMT2PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPERMT2Px_ronomo', '         '];
T['VPERMT2PS VREG:rw:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPERMT2Px_ronomo', 'tuple=FV '];
T['VPERMT2PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMn VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPERMT2Px_rynymy', '         '];
T['VPERMT2PD VREG:rw:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMn MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPERMT2Px_rynymy', 'tuple=FV '];
T['VPERMT2PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMn VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPERMT2Px_rynymy', '         '];
T['VPERMT2PS VREG:rw:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMn MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPERMT2Px_rynymy', 'tuple=FV '];
T['VPERMT2PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPERMT2Px_rznzmz', '         '];
T['VPERMT2PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPERMT2Px_rznzmz', 'tuple=FV '];
T['VPERMT2PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPERMT2Px_rznzmz', '         '];
T['VPERMT2PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPERMT2Px_rznzmz', 'tuple=FV '];
T['VPERMT2W  VREG:rw:u16=$XMMr PREG:r:Z=$Kk VREG:r:u16=$XMMn VREG:r:u16=$XMMm            ', 'MOD=REG         ', 'VPERMT2W_ronomo ', '         '];
T['VPERMT2W  VREG:rw:u16=$XMMr PREG:r:Z=$Kk VREG:r:u16=$XMMn MEM:r:u16=$mem128           ', 'MOD=MEM         ', 'VPERMT2W_ronomo ', 'tuple=FVM'];
T['VPERMT2W  VREG:rw:u16=$YMMr PREG:r:Z=$Kk VREG:r:u16=$YMMn VREG:r:u16=$YMMm            ', 'MOD=REG         ', 'VPERMT2W_rynymy ', '         '];
T['VPERMT2W  VREG:rw:u16=$YMMr PREG:r:Z=$Kk VREG:r:u16=$YMMn MEM:r:u16=$mem256           ', 'MOD=MEM         ', 'VPERMT2W_rynymy ', 'tuple=FVM'];
T['VPERMT2W  VREG:rw:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u16=$ZMMn VREG:r:u16=$ZMMm            ', 'MOD=REG         ', 'VPERMT2W_rznzmz ', '         '];
T['VPERMT2W  VREG:rw:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u16=$ZMMn MEM:r:u16=$mem512           ', 'MOD=MEM         ', 'VPERMT2W_rznzmz ', 'tuple=FVM'];
T['VPERMT2D  VREG:rw:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPERMT2x_ronomo ', '         '];
T['VPERMT2D  VREG:rw:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPERMT2x_ronomo ', 'tuple=FV '];
T['VPERMT2Q  VREG:rw:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPERMT2x_ronomo ', '         '];
T['VPERMT2Q  VREG:rw:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPERMT2x_ronomo ', 'tuple=FV '];
T['VPERMT2D  VREG:rw:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPERMT2x_rynymy ', '         '];
T['VPERMT2D  VREG:rw:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPERMT2x_rynymy ', 'tuple=FV '];
T['VPERMT2Q  VREG:rw:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPERMT2x_rynymy ', '         '];
T['VPERMT2Q  VREG:rw:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPERMT2x_rynymy ', 'tuple=FV '];
T['VPERMT2D  VREG:rw:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPERMT2x_rznzmz ', '         '];
T['VPERMT2D  VREG:rw:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPERMT2x_rznzmz ', 'tuple=FV '];
T['VPERMT2Q  VREG:rw:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPERMT2x_rznzmz ', '         '];
T['VPERMT2Q  VREG:rw:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPERMT2x_rznzmz ', 'tuple=FV '];

# VPERMD/VPERMW-Permute Packed Doublewords/Words Elements.
T['VPERMD VREG:w:u32=$YMMr VREG:r:u32=$YMMn VREG:r:u32=$YMMm                              ', 'MOD=REG     ', 'VPERMD_rynymy_1', '         '];
T['VPERMD VREG:w:u32=$YMMr VREG:r:u32=$YMMn MEM:r:u32=$mem256                             ', 'MOD=MEM     ', 'VPERMD_rynymy_1', '         '];
T['VPERMD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VPERMD_rynymy_2', '         '];
T['VPERMD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPERMD_rynymy_2', 'tuple=FV '];
T['VPERMD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VPERMD_rznzmz_1', '         '];
T['VPERMD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPERMD_rznzmz_1', 'tuple=FV '];
T['VPERMW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm            ', 'MOD=REG     ', 'VPERMW_ronomo  ', '         '];
T['VPERMW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128           ', 'MOD=MEM     ', 'VPERMW_ronomo  ', 'tuple=FVM'];
T['VPERMW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm            ', 'MOD=REG     ', 'VPERMW_rynymy  ', '         '];
T['VPERMW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256           ', 'MOD=MEM     ', 'VPERMW_rynymy  ', 'tuple=FVM'];
T['VPERMW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm            ', 'MOD=REG     ', 'VPERMW_rznzmz  ', '         '];
T['VPERMW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512           ', 'MOD=MEM     ', 'VPERMW_rznzmz  ', 'tuple=FVM'];

# VPEXPANDD-Load Sparse Packed Doubleword Integer Values from Dense Memory / Register.
T['VPEXPANDD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMm ', 'MOD=REG', 'VPEXPANDD_romo', '         '];
T['VPEXPANDD VREG:w:u32=$XMMr PREG:r:Z=$Kk MEM:r:u32=$mem128', 'MOD=MEM', 'VPEXPANDD_romo', 'tuple=T1S'];
T['VPEXPANDD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMm ', 'MOD=REG', 'VPEXPANDD_rymy', '         '];
T['VPEXPANDD VREG:w:u32=$YMMr PREG:r:Z=$Kk MEM:r:u32=$mem256', 'MOD=MEM', 'VPEXPANDD_rymy', 'tuple=T1S'];
T['VPEXPANDD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMm ', 'MOD=REG', 'VPEXPANDD_rzmz', '         '];
T['VPEXPANDD VREG:w:u32=$ZMMr PREG:r:Z=$Kk MEM:r:u32=$mem512', 'MOD=MEM', 'VPEXPANDD_rzmz', 'tuple=T1S'];

# VPEXPANDQ-Load Sparse Packed Quadword Integer Values from Dense Memory / Register.
T['VPEXPANDQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMm ', 'MOD=REG', 'VPEXPANDQ_romo', '         '];
T['VPEXPANDQ VREG:w:u64=$XMMr PREG:r:Z=$Kk MEM:r:u64=$mem128', 'MOD=MEM', 'VPEXPANDQ_romo', 'tuple=T1S'];
T['VPEXPANDQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMm ', 'MOD=REG', 'VPEXPANDQ_rymy', '         '];
T['VPEXPANDQ VREG:w:u64=$YMMr PREG:r:Z=$Kk MEM:r:u64=$mem256', 'MOD=MEM', 'VPEXPANDQ_rymy', 'tuple=T1S'];
T['VPEXPANDQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMm ', 'MOD=REG', 'VPEXPANDQ_rzmz', '         '];
T['VPEXPANDQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk MEM:r:u64=$mem512', 'MOD=MEM', 'VPEXPANDQ_rzmz', 'tuple=T1S'];

# VPEXPANDB/VPEXPANDW-Expand Byte/Word Values.
T['VPEXPANDB VREG:w:u8=$XMMr  PREG:r:Z=$Kk VREG:r:u8=$XMMm  ', 'MOD=REG W=0', 'VPEXPANDx_romo', '           '];
T['VPEXPANDB VREG:w:u8=$XMMr  PREG:r:Z=$Kk MEM:r:u8=$mem128 ', 'MOD=MEM W=0', 'VPEXPANDx_romo', 'tuple=T1S8 '];
T['VPEXPANDW VREG:w:u16=$XMMr PREG:r:Z=$Kk VREG:r:u16=$XMMm ', 'MOD=REG W=1', 'VPEXPANDx_romo', '           '];
T['VPEXPANDW VREG:w:u16=$XMMr PREG:r:Z=$Kk MEM:r:u16=$mem128', 'MOD=MEM W=1', 'VPEXPANDx_romo', 'tuple=T1S16'];
T['VPEXPANDB VREG:w:u8=$YMMr  PREG:r:Z=$Kk VREG:r:u8=$YMMm  ', 'MOD=REG W=0', 'VPEXPANDx_rymy', '           '];
T['VPEXPANDB VREG:w:u8=$YMMr  PREG:r:Z=$Kk MEM:r:u8=$mem256 ', 'MOD=MEM W=0', 'VPEXPANDx_rymy', 'tuple=T1S8 '];
T['VPEXPANDW VREG:w:u16=$YMMr PREG:r:Z=$Kk VREG:r:u16=$YMMm ', 'MOD=REG W=1', 'VPEXPANDx_rymy', '           '];
T['VPEXPANDW VREG:w:u16=$YMMr PREG:r:Z=$Kk MEM:r:u16=$mem256', 'MOD=MEM W=1', 'VPEXPANDx_rymy', 'tuple=T1S16'];
T['VPEXPANDB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk VREG:r:u8=$ZMMm  ', 'MOD=REG W=0', 'VPEXPANDx_rzmz', '           '];
T['VPEXPANDB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk MEM:r:u8=$mem512 ', 'MOD=MEM W=0', 'VPEXPANDx_rzmz', 'tuple=T1S8 '];
T['VPEXPANDW VREG:w:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u16=$ZMMm ', 'MOD=REG W=1', 'VPEXPANDx_rzmz', '           '];
T['VPEXPANDW VREG:w:u16=$ZMMr PREG:r:Z=$Kk MEM:r:u16=$mem512', 'MOD=MEM W=1', 'VPEXPANDx_rzmz', 'tuple=T1S16'];

# VPGATHERDD/VPGATHERDQ-Gather Packed Dword, Packed Qword with Signed Dword Indices.
T['VPGATHERDD VREG:w:u32=$XMMr   PREG:rw=$Kk      MEM:r:u32=$vmx32 ', 'W=0       ', 'VPGATHERDx_romx  ', 'tuple=T1S'];
T['VPGATHERDQ VREG:w:u64=$XMMr   PREG:rw=$Kk      MEM:r:u64=$vmx64 ', 'W=1       ', 'VPGATHERDx_romx  ', 'tuple=T1S'];
T['VPGATHERDD VREG:crw:u32=$XMMr MEM:r:u32=$vmx32 VREG:rw:s32=$XMMn', 'VL=128 W=0', 'VPGATHERDx_rxmxnx', '         '];
T['VPGATHERDD VREG:crw:u32=$YMMr MEM:r:u32=$vmy32 VREG:rw:s32=$YMMn', 'VL=256 W=0', 'VPGATHERDx_rxmxnx', '         '];
T['VPGATHERDQ VREG:crw:u64=$XMMr MEM:r:u64=$vmx64 VREG:rw:s64=$XMMn', 'VL=128 W=1', 'VPGATHERDx_rxmxnx', '         '];
T['VPGATHERDQ VREG:crw:u64=$YMMr MEM:r:u64=$vmx64 VREG:rw:s64=$YMMn', 'VL=256 W=1', 'VPGATHERDx_rxmxnx', '         '];
T['VPGATHERDD VREG:w:u32=$YMMr   PREG:rw=$Kk      MEM:r:u32=$vmy32 ', 'W=0       ', 'VPGATHERDx_rymx  ', 'tuple=T1S'];
T['VPGATHERDQ VREG:w:u64=$YMMr   PREG:rw=$Kk      MEM:r:u64=$vmx64 ', 'W=1       ', 'VPGATHERDx_rymx  ', 'tuple=T1S'];
T['VPGATHERDD VREG:w:u32=$ZMMr   PREG:rw=$Kk      MEM:r:u32=$vmz32 ', 'W=0       ', 'VPGATHERDx_rzmx  ', 'tuple=T1S'];
T['VPGATHERDQ VREG:w:u64=$ZMMr   PREG:rw=$Kk      MEM:r:u64=$vmy64 ', 'W=1       ', 'VPGATHERDx_rzmx  ', 'tuple=T1S'];

# VPGATHERQD/VPGATHERQQ-Gather Packed Dword, Packed Qword with Signed Qword Indices.
T['VPGATHERQD VREG:w:u32=$XMMr   PREG:rw=$Kk      MEM:r:u32=$vmx32 ', 'W=0       ', 'VPGATHERQx_romx  ', 'tuple=T1S'];
T['VPGATHERQQ VREG:w:u64=$XMMr   PREG:rw=$Kk      MEM:r:u64=$vmx64 ', 'W=1       ', 'VPGATHERQx_romx  ', 'tuple=T1S'];
T['VPGATHERQD VREG:w:u32=$XMMr   PREG:rw=$Kk      MEM:r:u32=$vmy32 ', 'W=0       ', 'VPGATHERQx_rxmx_1', 'tuple=T1S'];
T['VPGATHERQQ VREG:w:u64=$YMMr   PREG:rw=$Kk      MEM:r:u64=$vmy64 ', 'W=1       ', 'VPGATHERQx_rxmx_1', 'tuple=T1S'];
T['VPGATHERQD VREG:w:u32=$YMMr   PREG:rw=$Kk      MEM:r:u32=$vmz32 ', 'W=0       ', 'VPGATHERQx_rxmx_2', 'tuple=T1S'];
T['VPGATHERQQ VREG:w:u64=$ZMMr   PREG:rw=$Kk      MEM:r:u64=$vmz64 ', 'W=1       ', 'VPGATHERQx_rxmx_2', 'tuple=T1S'];
T['VPGATHERQD VREG:crw:u32=$XMMr MEM:r:u32=$vmx32 VREG:rw:s32=$XMMn', 'VL=128 W=0', 'VPGATHERQx_rxmxnx', '         '];
T['VPGATHERQD VREG:crw:u32=$XMMr MEM:r:u32=$vmy32 VREG:rw:s32=$XMMn', 'VL=256 W=0', 'VPGATHERQx_rxmxnx', '         '];
T['VPGATHERQQ VREG:crw:u64=$XMMr MEM:r:u64=$vmx64 VREG:rw:s64=$XMMn', 'VL=128 W=1', 'VPGATHERQx_rxmxnx', '         '];
T['VPGATHERQQ VREG:crw:u64=$YMMr MEM:r:u64=$vmy64 VREG:rw:s64=$YMMn', 'VL=256 W=1', 'VPGATHERQx_rxmxnx', '         '];

# VPLZCNTD/Q-Count the Number of Leading Zero Bits for Packed Dword, Packed Qword Values.
T['VPLZCNTD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPLZCNTx_romo', '        '];
T['VPLZCNTD VREG:w:u32=$XMMr PREG:r:Z=$Kk MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPLZCNTx_romo', 'tuple=FV'];
T['VPLZCNTQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPLZCNTx_romo', '        '];
T['VPLZCNTQ VREG:w:u64=$XMMr PREG:r:Z=$Kk MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPLZCNTx_romo', 'tuple=FV'];
T['VPLZCNTD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPLZCNTx_rymy', '        '];
T['VPLZCNTD VREG:w:u32=$YMMr PREG:r:Z=$Kk MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPLZCNTx_rymy', 'tuple=FV'];
T['VPLZCNTQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPLZCNTx_rymy', '        '];
T['VPLZCNTQ VREG:w:u64=$YMMr PREG:r:Z=$Kk MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPLZCNTx_rymy', 'tuple=FV'];
T['VPLZCNTD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPLZCNTx_rzmz', '        '];
T['VPLZCNTD VREG:w:u32=$ZMMr PREG:r:Z=$Kk MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPLZCNTx_rzmz', 'tuple=FV'];
T['VPLZCNTQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPLZCNTx_rzmz', '        '];
T['VPLZCNTQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPLZCNTx_rzmz', 'tuple=FV'];

# VPMADD52HUQ-Packed Multiply of Unsigned 52-bit Unsigned Integers and Add High 52-bit Products to 64-bit Accumulators.
T['VPMADD52HUQ VREG:rw:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG', 'VPMADD52HUQ_ronomo', '        '];
T['VPMADD52HUQ VREG:rw:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPMADD52HUQ_ronomo', 'tuple=FV'];
T['VPMADD52HUQ VREG:rw:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG', 'VPMADD52HUQ_rynymy', '        '];
T['VPMADD52HUQ VREG:rw:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPMADD52HUQ_rynymy', 'tuple=FV'];
T['VPMADD52HUQ VREG:rw:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG', 'VPMADD52HUQ_rznzmz', '        '];
T['VPMADD52HUQ VREG:rw:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPMADD52HUQ_rznzmz', 'tuple=FV'];

# VPMADD52LUQ-Packed Multiply of Unsigned 52-bit Integers and Add the Low 52-bit Products to Qword Accumulators.
T['VPMADD52LUQ VREG:rw:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG', 'VPMADD52LUQ_ronomo', '        '];
T['VPMADD52LUQ VREG:rw:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPMADD52LUQ_ronomo', 'tuple=FV'];
T['VPMADD52LUQ VREG:rw:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG', 'VPMADD52LUQ_rynymy', '        '];
T['VPMADD52LUQ VREG:rw:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPMADD52LUQ_rynymy', 'tuple=FV'];
T['VPMADD52LUQ VREG:rw:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG', 'VPMADD52LUQ_rznzmz', '        '];
T['VPMADD52LUQ VREG:rw:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPMADD52LUQ_rznzmz', 'tuple=FV'];

# VPMASKMOV-Conditional SIMD Integer Packed Loads and Stores.
T['VPMASKMOVD MEM:w:u32=$mem128 VREG:r:u32=$XMMn VREG:r:u32=$XMMr ', 'VL=128 W=0', 'VPMASKMOVx_mxnxrx', ''];
T['VPMASKMOVD MEM:w:u32=$mem256 VREG:r:u32=$YMMn VREG:r:u32=$YMMr ', 'VL=256 W=0', 'VPMASKMOVx_mxnxrx', ''];
T['VPMASKMOVQ MEM:w:u64=$mem128 VREG:r:u64=$XMMn VREG:r:u64=$XMMr ', 'VL=128 W=1', 'VPMASKMOVx_mxnxrx', ''];
T['VPMASKMOVQ MEM:w:u64=$mem256 VREG:r:u64=$YMMn VREG:r:u64=$YMMr ', 'VL=256 W=1', 'VPMASKMOVx_mxnxrx', ''];
T['VPMASKMOVD VREG:w:u32=$XMMr  VREG:r:u32=$XMMn MEM:r:u32=$mem128', 'VL=128 W=0', 'VPMASKMOVx_rxnxmx', ''];
T['VPMASKMOVD VREG:w:u32=$YMMr  VREG:r:u32=$YMMn MEM:r:u32=$mem256', 'VL=256 W=0', 'VPMASKMOVx_rxnxmx', ''];
T['VPMASKMOVQ VREG:w:u64=$XMMr  VREG:r:u64=$XMMn MEM:r:u64=$mem128', 'VL=128 W=1', 'VPMASKMOVx_rxnxmx', ''];
T['VPMASKMOVQ VREG:w:u64=$YMMr  VREG:r:u64=$YMMn MEM:r:u64=$mem256', 'VL=256 W=1', 'VPMASKMOVx_rxnxmx', ''];

# VPMOVM2B/VPMOVM2W/VPMOVM2D/VPMOVM2Q-Convert a Mask Register to a Vector Register.
T['VPMOVM2B VREG:w:u8=$XMMr  REG:r=$Km', 'W=0', 'VPMOVM2x_ro_1', ''];
T['VPMOVM2W VREG:w:u16=$XMMr REG:r=$Km', 'W=1', 'VPMOVM2x_ro_1', ''];
T['VPMOVM2D VREG:w:u32=$XMMr REG:r=$Km', 'W=0', 'VPMOVM2x_ro_2', ''];
T['VPMOVM2Q VREG:w:u64=$XMMr REG:r=$Km', 'W=1', 'VPMOVM2x_ro_2', ''];
T['VPMOVM2B VREG:w:u8=$YMMr  REG:r=$Km', 'W=0', 'VPMOVM2x_ry_1', ''];
T['VPMOVM2W VREG:w:u16=$YMMr REG:r=$Km', 'W=1', 'VPMOVM2x_ry_1', ''];
T['VPMOVM2D VREG:w:u32=$YMMr REG:r=$Km', 'W=0', 'VPMOVM2x_ry_2', ''];
T['VPMOVM2Q VREG:w:u64=$YMMr REG:r=$Km', 'W=1', 'VPMOVM2x_ry_2', ''];
T['VPMOVM2B VREG:w:u8=$ZMMr  REG:r=$Km', 'W=0', 'VPMOVM2x_rz_1', ''];
T['VPMOVM2W VREG:w:u16=$ZMMr REG:r=$Km', 'W=1', 'VPMOVM2x_rz_1', ''];
T['VPMOVM2D VREG:w:u32=$ZMMr REG:r=$Km', 'W=0', 'VPMOVM2x_rz_2', ''];
T['VPMOVM2Q VREG:w:u64=$ZMMr REG:r=$Km', 'W=1', 'VPMOVM2x_rz_2', ''];

# VPMOVB2M/VPMOVW2M/VPMOVD2M/VPMOVQ2M-Convert a Vector Register to a Mask.
T['VPMOVB2M REG:w=$Kr VREG:r:u8=$XMMm ', 'W=0', 'VPMOVx2M_mo_1', ''];
T['VPMOVW2M REG:w=$Kr VREG:r:u16=$XMMm', 'W=1', 'VPMOVx2M_mo_1', ''];
T['VPMOVD2M REG:w=$Kr VREG:r:u32=$XMMm', 'W=0', 'VPMOVx2M_mo_2', ''];
T['VPMOVQ2M REG:w=$Kr VREG:r:u64=$XMMm', 'W=1', 'VPMOVx2M_mo_2', ''];
T['VPMOVB2M REG:w=$Kr VREG:r:u8=$YMMm ', 'W=0', 'VPMOVx2M_my_1', ''];
T['VPMOVW2M REG:w=$Kr VREG:r:u16=$YMMm', 'W=1', 'VPMOVx2M_my_1', ''];
T['VPMOVD2M REG:w=$Kr VREG:r:u32=$YMMm', 'W=0', 'VPMOVx2M_my_2', ''];
T['VPMOVQ2M REG:w=$Kr VREG:r:u64=$YMMm', 'W=1', 'VPMOVx2M_my_2', ''];
T['VPMOVB2M REG:w=$Kr VREG:r:u8=$ZMMm ', 'W=0', 'VPMOVx2M_mz_1', ''];
T['VPMOVW2M REG:w=$Kr VREG:r:u16=$ZMMm', 'W=1', 'VPMOVx2M_mz_1', ''];
T['VPMOVD2M REG:w=$Kr VREG:r:u32=$ZMMm', 'W=0', 'VPMOVx2M_mz_2', ''];
T['VPMOVQ2M REG:w=$Kr VREG:r:u64=$ZMMm', 'W=1', 'VPMOVx2M_mz_2', ''];

# VPMOVDB/VPMOVSDB/VPMOVUSDB-Down Convert DWord to Byte.
T['VPMOVDB   MEM:w:u8=$mem32  PREG:r:Z=$Kk VREG:r:u32=$XMMr', 'NONE', 'VPMOVDB_mdro    ', 'tuple=QVM'];
T['VPMOVDB   VREG:w:u8=$XMMm  PREG:r:Z=$Kk VREG:r:u32=$XMMr', 'NONE', 'VPMOVDB_moro    ', '         '];
T['VPMOVDB   VREG:w:u8=$XMMm  PREG:r:Z=$Kk VREG:r:u32=$YMMr', 'NONE', 'VPMOVDB_mory    ', '         '];
T['VPMOVDB   VREG:w:u8=$XMMm  PREG:r:Z=$Kk VREG:r:u32=$ZMMr', 'NONE', 'VPMOVDB_morz_1  ', '         '];
T['VPMOVDB   MEM:w:u8=$mem128 PREG:r:Z=$Kk VREG:r:u32=$ZMMr', 'NONE', 'VPMOVDB_morz_2  ', 'tuple=QVM'];
T['VPMOVDB   MEM:w:u8=$mem64  PREG:r:Z=$Kk VREG:r:u32=$YMMr', 'NONE', 'VPMOVDB_mqry    ', 'tuple=QVM'];
T['VPMOVSDB  MEM:w:s8=$mem32  PREG:r:Z=$Kk VREG:r:s32=$XMMr', 'NONE', 'VPMOVSDB_mdro   ', 'tuple=QVM'];
T['VPMOVSDB  VREG:w:s8=$XMMm  PREG:r:Z=$Kk VREG:r:s32=$XMMr', 'NONE', 'VPMOVSDB_moro   ', '         '];
T['VPMOVSDB  VREG:w:s8=$XMMm  PREG:r:Z=$Kk VREG:r:s32=$YMMr', 'NONE', 'VPMOVSDB_mory   ', '         '];
T['VPMOVSDB  VREG:w:s8=$XMMm  PREG:r:Z=$Kk VREG:r:s32=$ZMMr', 'NONE', 'VPMOVSDB_morz_1 ', '         '];
T['VPMOVSDB  MEM:w:s8=$mem128 PREG:r:Z=$Kk VREG:r:s32=$ZMMr', 'NONE', 'VPMOVSDB_morz_2 ', 'tuple=QVM'];
T['VPMOVSDB  MEM:w:s8=$mem64  PREG:r:Z=$Kk VREG:r:s32=$YMMr', 'NONE', 'VPMOVSDB_mqry   ', 'tuple=QVM'];
T['VPMOVUSDB MEM:w:u8=$mem32  PREG:r:Z=$Kk VREG:r:u32=$XMMr', 'NONE', 'VPMOVUSDB_mdro  ', 'tuple=QVM'];
T['VPMOVUSDB VREG:w:u8=$XMMm  PREG:r:Z=$Kk VREG:r:u32=$XMMr', 'NONE', 'VPMOVUSDB_moro  ', '         '];
T['VPMOVUSDB VREG:w:u8=$XMMm  PREG:r:Z=$Kk VREG:r:u32=$YMMr', 'NONE', 'VPMOVUSDB_mory  ', '         '];
T['VPMOVUSDB VREG:w:u8=$XMMm  PREG:r:Z=$Kk VREG:r:u32=$ZMMr', 'NONE', 'VPMOVUSDB_morz_1', '         '];
T['VPMOVUSDB MEM:w:u8=$mem128 PREG:r:Z=$Kk VREG:r:u32=$ZMMr', 'NONE', 'VPMOVUSDB_morz_2', 'tuple=QVM'];
T['VPMOVUSDB MEM:w:u8=$mem64  PREG:r:Z=$Kk VREG:r:u32=$YMMr', 'NONE', 'VPMOVUSDB_mqry  ', 'tuple=QVM'];

# VPMOVDW/VPMOVSDW/VPMOVUSDW-Down Convert DWord to Word.
T['VPMOVDW   VREG:w:u16=$XMMm  PREG:r:Z=$Kk VREG:r:u32=$XMMr', 'NONE', 'VPMOVDW_moro    ', '         '];
T['VPMOVDW   VREG:w:u16=$XMMm  PREG:r:Z=$Kk VREG:r:u32=$YMMr', 'NONE', 'VPMOVDW_mory_1  ', '         '];
T['VPMOVDW   MEM:w:u16=$mem128 PREG:r:Z=$Kk VREG:r:u32=$YMMr', 'NONE', 'VPMOVDW_mory_2  ', 'tuple=HVM'];
T['VPMOVDW   MEM:w:u16=$mem64  PREG:r:Z=$Kk VREG:r:u32=$XMMr', 'NONE', 'VPMOVDW_mqro    ', 'tuple=HVM'];
T['VPMOVDW   VREG:w:u16=$YMMm  PREG:r:Z=$Kk VREG:r:u32=$ZMMr', 'NONE', 'VPMOVDW_myrz_1  ', '         '];
T['VPMOVDW   MEM:w:u16=$mem256 PREG:r:Z=$Kk VREG:r:u32=$ZMMr', 'NONE', 'VPMOVDW_myrz_2  ', 'tuple=HVM'];
T['VPMOVSDW  VREG:w:s16=$XMMm  PREG:r:Z=$Kk VREG:r:s32=$XMMr', 'NONE', 'VPMOVSDW_moro   ', '         '];
T['VPMOVSDW  VREG:w:s16=$XMMm  PREG:r:Z=$Kk VREG:r:s32=$YMMr', 'NONE', 'VPMOVSDW_mory_1 ', '         '];
T['VPMOVSDW  MEM:w:s16=$mem128 PREG:r:Z=$Kk VREG:r:s32=$YMMr', 'NONE', 'VPMOVSDW_mory_2 ', 'tuple=HVM'];
T['VPMOVSDW  MEM:w:s16=$mem64  PREG:r:Z=$Kk VREG:r:s32=$XMMr', 'NONE', 'VPMOVSDW_mqro   ', 'tuple=HVM'];
T['VPMOVSDW  VREG:w:s16=$YMMm  PREG:r:Z=$Kk VREG:r:s32=$ZMMr', 'NONE', 'VPMOVSDW_myrz_1 ', '         '];
T['VPMOVSDW  MEM:w:s16=$mem256 PREG:r:Z=$Kk VREG:r:s32=$ZMMr', 'NONE', 'VPMOVSDW_myrz_2 ', 'tuple=HVM'];
T['VPMOVUSDW VREG:w:u16=$XMMm  PREG:r:Z=$Kk VREG:r:u32=$XMMr', 'NONE', 'VPMOVUSDW_moro  ', '         '];
T['VPMOVUSDW VREG:w:u16=$XMMm  PREG:r:Z=$Kk VREG:r:u32=$YMMr', 'NONE', 'VPMOVUSDW_mory_1', '         '];
T['VPMOVUSDW MEM:w:u16=$mem128 PREG:r:Z=$Kk VREG:r:u32=$YMMr', 'NONE', 'VPMOVUSDW_mory_2', 'tuple=HVM'];
T['VPMOVUSDW MEM:w:u16=$mem64  PREG:r:Z=$Kk VREG:r:u32=$XMMr', 'NONE', 'VPMOVUSDW_mqro  ', 'tuple=HVM'];
T['VPMOVUSDW VREG:w:u16=$YMMm  PREG:r:Z=$Kk VREG:r:u32=$ZMMr', 'NONE', 'VPMOVUSDW_myrz_1', '         '];
T['VPMOVUSDW MEM:w:u16=$mem256 PREG:r:Z=$Kk VREG:r:u32=$ZMMr', 'NONE', 'VPMOVUSDW_myrz_2', 'tuple=HVM'];

# VPMOVQB/VPMOVSQB/VPMOVUSQB-Down Convert QWord to Byte.
T['VPMOVQB   MEM:w:u8=$mem32 PREG:r:Z=$Kk VREG:r:u64=$YMMr', 'NONE', 'VPMOVQB_mdry  ', 'tuple=OVM'];
T['VPMOVQB   VREG:w:u8=$XMMm PREG:r:Z=$Kk VREG:r:u64=$XMMr', 'NONE', 'VPMOVQB_moro  ', '         '];
T['VPMOVQB   VREG:w:u8=$XMMm PREG:r:Z=$Kk VREG:r:u64=$YMMr', 'NONE', 'VPMOVQB_mory  ', '         '];
T['VPMOVQB   VREG:w:u8=$XMMm PREG:r:Z=$Kk VREG:r:u64=$ZMMr', 'NONE', 'VPMOVQB_morz  ', '         '];
T['VPMOVQB   MEM:w:u8=$mem64 PREG:r:Z=$Kk VREG:r:u64=$ZMMr', 'NONE', 'VPMOVQB_mqrz  ', 'tuple=OVM'];
T['VPMOVQB   MEM:w:u8=$mem16 PREG:r:Z=$Kk VREG:r:u64=$XMMr', 'NONE', 'VPMOVQB_mwro  ', 'tuple=OVM'];
T['VPMOVSQB  MEM:w:s8=$mem32 PREG:r:Z=$Kk VREG:r:s64=$YMMr', 'NONE', 'VPMOVSQB_mdry ', 'tuple=OVM'];
T['VPMOVSQB  VREG:w:s8=$XMMm PREG:r:Z=$Kk VREG:r:s64=$XMMr', 'NONE', 'VPMOVSQB_moro ', '         '];
T['VPMOVSQB  VREG:w:s8=$XMMm PREG:r:Z=$Kk VREG:r:s64=$YMMr', 'NONE', 'VPMOVSQB_mory ', '         '];
T['VPMOVSQB  VREG:w:s8=$XMMm PREG:r:Z=$Kk VREG:r:s64=$ZMMr', 'NONE', 'VPMOVSQB_morz ', '         '];
T['VPMOVSQB  MEM:w:s8=$mem64 PREG:r:Z=$Kk VREG:r:s64=$ZMMr', 'NONE', 'VPMOVSQB_mqrz ', 'tuple=OVM'];
T['VPMOVSQB  MEM:w:s8=$mem16 PREG:r:Z=$Kk VREG:r:s64=$XMMr', 'NONE', 'VPMOVSQB_mwro ', 'tuple=OVM'];
T['VPMOVUSQB MEM:w:u8=$mem32 PREG:r:Z=$Kk VREG:r:u64=$YMMr', 'NONE', 'VPMOVUSQB_mdry', 'tuple=OVM'];
T['VPMOVUSQB VREG:w:u8=$XMMm PREG:r:Z=$Kk VREG:r:u64=$XMMr', 'NONE', 'VPMOVUSQB_moro', '         '];
T['VPMOVUSQB VREG:w:u8=$XMMm PREG:r:Z=$Kk VREG:r:u64=$YMMr', 'NONE', 'VPMOVUSQB_mory', '         '];
T['VPMOVUSQB VREG:w:u8=$XMMm PREG:r:Z=$Kk VREG:r:u64=$ZMMr', 'NONE', 'VPMOVUSQB_morz', '         '];
T['VPMOVUSQB MEM:w:u8=$mem64 PREG:r:Z=$Kk VREG:r:u64=$ZMMr', 'NONE', 'VPMOVUSQB_mqrz', 'tuple=OVM'];
T['VPMOVUSQB MEM:w:u8=$mem16 PREG:r:Z=$Kk VREG:r:u64=$XMMr', 'NONE', 'VPMOVUSQB_mwro', 'tuple=OVM'];

# VPMOVQD/VPMOVSQD/VPMOVUSQD-Down Convert QWord to DWord.
T['VPMOVQD   VREG:w:u32=$XMMm  PREG:r:Z=$Kk VREG:r:u64=$XMMr', 'NONE', 'VPMOVQD_moro    ', '         '];
T['VPMOVQD   VREG:w:u32=$XMMm  PREG:r:Z=$Kk VREG:r:u64=$YMMr', 'NONE', 'VPMOVQD_mory_1  ', '         '];
T['VPMOVQD   MEM:w:u32=$mem128 PREG:r:Z=$Kk VREG:r:u64=$YMMr', 'NONE', 'VPMOVQD_mory_2  ', 'tuple=HVM'];
T['VPMOVQD   MEM:w:u32=$mem64  PREG:r:Z=$Kk VREG:r:u64=$XMMr', 'NONE', 'VPMOVQD_mqro    ', 'tuple=HVM'];
T['VPMOVQD   VREG:w:u32=$YMMm  PREG:r:Z=$Kk VREG:r:u64=$ZMMr', 'NONE', 'VPMOVQD_myrz_1  ', '         '];
T['VPMOVQD   MEM:w:u32=$mem256 PREG:r:Z=$Kk VREG:r:u64=$ZMMr', 'NONE', 'VPMOVQD_myrz_2  ', 'tuple=HVM'];
T['VPMOVSQD  VREG:w:s32=$XMMm  PREG:r:Z=$Kk VREG:r:s64=$XMMr', 'NONE', 'VPMOVSQD_moro   ', '         '];
T['VPMOVSQD  VREG:w:s32=$XMMm  PREG:r:Z=$Kk VREG:r:s64=$YMMr', 'NONE', 'VPMOVSQD_mory_1 ', '         '];
T['VPMOVSQD  MEM:w:s32=$mem128 PREG:r:Z=$Kk VREG:r:s64=$YMMr', 'NONE', 'VPMOVSQD_mory_2 ', 'tuple=HVM'];
T['VPMOVSQD  MEM:w:s32=$mem64  PREG:r:Z=$Kk VREG:r:s64=$XMMr', 'NONE', 'VPMOVSQD_mqro   ', 'tuple=HVM'];
T['VPMOVSQD  VREG:w:s32=$YMMm  PREG:r:Z=$Kk VREG:r:s64=$ZMMr', 'NONE', 'VPMOVSQD_myrz_1 ', '         '];
T['VPMOVSQD  MEM:w:s32=$mem256 PREG:r:Z=$Kk VREG:r:s64=$ZMMr', 'NONE', 'VPMOVSQD_myrz_2 ', 'tuple=HVM'];
T['VPMOVUSQD VREG:w:u32=$XMMm  PREG:r:Z=$Kk VREG:r:u64=$XMMr', 'NONE', 'VPMOVUSQD_moro  ', '         '];
T['VPMOVUSQD VREG:w:u32=$XMMm  PREG:r:Z=$Kk VREG:r:u64=$YMMr', 'NONE', 'VPMOVUSQD_mory_1', '         '];
T['VPMOVUSQD MEM:w:u32=$mem128 PREG:r:Z=$Kk VREG:r:u64=$YMMr', 'NONE', 'VPMOVUSQD_mory_2', 'tuple=HVM'];
T['VPMOVUSQD MEM:w:u32=$mem64  PREG:r:Z=$Kk VREG:r:u64=$XMMr', 'NONE', 'VPMOVUSQD_mqro  ', 'tuple=HVM'];
T['VPMOVUSQD VREG:w:u32=$YMMm  PREG:r:Z=$Kk VREG:r:u64=$ZMMr', 'NONE', 'VPMOVUSQD_myrz_1', '         '];
T['VPMOVUSQD MEM:w:u32=$mem256 PREG:r:Z=$Kk VREG:r:u64=$ZMMr', 'NONE', 'VPMOVUSQD_myrz_2', 'tuple=HVM'];

# VPMOVQW/VPMOVSQW/VPMOVUSQW-Down Convert QWord to Word.
T['VPMOVQW   MEM:w:u16=$mem32  PREG:r:Z=$Kk VREG:r:u64=$XMMr', 'NONE', 'VPMOVQW_mdro    ', 'tuple=QVM'];
T['VPMOVQW   VREG:w:u16=$XMMm  PREG:r:Z=$Kk VREG:r:u64=$XMMr', 'NONE', 'VPMOVQW_moro    ', '         '];
T['VPMOVQW   VREG:w:u16=$XMMm  PREG:r:Z=$Kk VREG:r:u64=$YMMr', 'NONE', 'VPMOVQW_mory    ', '         '];
T['VPMOVQW   VREG:w:u16=$XMMm  PREG:r:Z=$Kk VREG:r:u64=$ZMMr', 'NONE', 'VPMOVQW_morz_1  ', '         '];
T['VPMOVQW   MEM:w:u16=$mem128 PREG:r:Z=$Kk VREG:r:u64=$ZMMr', 'NONE', 'VPMOVQW_morz_2  ', 'tuple=QVM'];
T['VPMOVQW   MEM:w:u16=$mem64  PREG:r:Z=$Kk VREG:r:u64=$YMMr', 'NONE', 'VPMOVQW_mqry    ', 'tuple=QVM'];
T['VPMOVSQW  MEM:w:s16=$mem32  PREG:r:Z=$Kk VREG:r:s64=$XMMr', 'NONE', 'VPMOVSQW_mdro   ', 'tuple=QVM'];
T['VPMOVSQW  VREG:w:s16=$XMMm  PREG:r:Z=$Kk VREG:r:s64=$XMMr', 'NONE', 'VPMOVSQW_moro   ', '         '];
T['VPMOVSQW  VREG:w:s16=$XMMm  PREG:r:Z=$Kk VREG:r:s64=$YMMr', 'NONE', 'VPMOVSQW_mory   ', '         '];
T['VPMOVSQW  VREG:w:s16=$XMMm  PREG:r:Z=$Kk VREG:r:s64=$ZMMr', 'NONE', 'VPMOVSQW_morz_1 ', '         '];
T['VPMOVSQW  MEM:w:s16=$mem128 PREG:r:Z=$Kk VREG:r:s64=$ZMMr', 'NONE', 'VPMOVSQW_morz_2 ', 'tuple=QVM'];
T['VPMOVSQW  MEM:w:s16=$mem64  PREG:r:Z=$Kk VREG:r:s64=$YMMr', 'NONE', 'VPMOVSQW_mqry   ', 'tuple=QVM'];
T['VPMOVUSQW MEM:w:u16=$mem32  PREG:r:Z=$Kk VREG:r:u64=$XMMr', 'NONE', 'VPMOVUSQW_mdro  ', 'tuple=QVM'];
T['VPMOVUSQW VREG:w:u16=$XMMm  PREG:r:Z=$Kk VREG:r:u64=$XMMr', 'NONE', 'VPMOVUSQW_moro  ', '         '];
T['VPMOVUSQW VREG:w:u16=$XMMm  PREG:r:Z=$Kk VREG:r:u64=$YMMr', 'NONE', 'VPMOVUSQW_mory  ', '         '];
T['VPMOVUSQW VREG:w:u16=$XMMm  PREG:r:Z=$Kk VREG:r:u64=$ZMMr', 'NONE', 'VPMOVUSQW_morz_1', '         '];
T['VPMOVUSQW MEM:w:u16=$mem128 PREG:r:Z=$Kk VREG:r:u64=$ZMMr', 'NONE', 'VPMOVUSQW_morz_2', 'tuple=QVM'];
T['VPMOVUSQW MEM:w:u16=$mem64  PREG:r:Z=$Kk VREG:r:u64=$YMMr', 'NONE', 'VPMOVUSQW_mqry  ', 'tuple=QVM'];

# VPMOVWB/VPMOVSWB/VPMOVUSWB-Down Convert Word to Byte.
T['VPMOVSWB  VREG:w:s8=$XMMm  PREG:r:Z=$Kk VREG:r:s16=$XMMr', 'NONE', 'VPMOVSWB_moro   ', '         '];
T['VPMOVSWB  VREG:w:s8=$XMMm  PREG:r:Z=$Kk VREG:r:s16=$YMMr', 'NONE', 'VPMOVSWB_mory_1 ', '         '];
T['VPMOVSWB  MEM:w:s8=$mem128 PREG:r:Z=$Kk VREG:r:s16=$YMMr', 'NONE', 'VPMOVSWB_mory_2 ', 'tuple=HVM'];
T['VPMOVSWB  MEM:w:s8=$mem64  PREG:r:Z=$Kk VREG:r:s16=$XMMr', 'NONE', 'VPMOVSWB_mqro   ', 'tuple=HVM'];
T['VPMOVSWB  VREG:w:s8=$YMMm  PREG:r:Z=$Kk VREG:r:s16=$ZMMr', 'NONE', 'VPMOVSWB_myrz_1 ', '         '];
T['VPMOVSWB  MEM:w:s8=$mem256 PREG:r:Z=$Kk VREG:r:s16=$ZMMr', 'NONE', 'VPMOVSWB_myrz_2 ', 'tuple=HVM'];
T['VPMOVUSWB VREG:w:u8=$XMMm  PREG:r:Z=$Kk VREG:r:u16=$XMMr', 'NONE', 'VPMOVUSWB_moro  ', '         '];
T['VPMOVUSWB VREG:w:u8=$XMMm  PREG:r:Z=$Kk VREG:r:u16=$YMMr', 'NONE', 'VPMOVUSWB_mory_1', '         '];
T['VPMOVUSWB MEM:w:u8=$mem128 PREG:r:Z=$Kk VREG:r:u16=$YMMr', 'NONE', 'VPMOVUSWB_mory_2', 'tuple=HVM'];
T['VPMOVUSWB MEM:w:u8=$mem64  PREG:r:Z=$Kk VREG:r:u16=$XMMr', 'NONE', 'VPMOVUSWB_mqro  ', 'tuple=HVM'];
T['VPMOVUSWB VREG:w:u8=$YMMm  PREG:r:Z=$Kk VREG:r:u16=$ZMMr', 'NONE', 'VPMOVUSWB_myrz_1', '         '];
T['VPMOVUSWB MEM:w:u8=$mem256 PREG:r:Z=$Kk VREG:r:u16=$ZMMr', 'NONE', 'VPMOVUSWB_myrz_2', 'tuple=HVM'];
T['VPMOVWB   VREG:w:u8=$XMMm  PREG:r:Z=$Kk VREG:r:u16=$XMMr', 'NONE', 'VPMOVWB_moro    ', '         '];
T['VPMOVWB   VREG:w:u8=$XMMm  PREG:r:Z=$Kk VREG:r:u16=$YMMr', 'NONE', 'VPMOVWB_mory_1  ', '         '];
T['VPMOVWB   MEM:w:u8=$mem128 PREG:r:Z=$Kk VREG:r:u16=$YMMr', 'NONE', 'VPMOVWB_mory_2  ', 'tuple=HVM'];
T['VPMOVWB   MEM:w:u8=$mem64  PREG:r:Z=$Kk VREG:r:u16=$XMMr', 'NONE', 'VPMOVWB_mqro    ', 'tuple=HVM'];
T['VPMOVWB   VREG:w:u8=$YMMm  PREG:r:Z=$Kk VREG:r:u16=$ZMMr', 'NONE', 'VPMOVWB_myrz_1  ', '         '];
T['VPMOVWB   MEM:w:u8=$mem256 PREG:r:Z=$Kk VREG:r:u16=$ZMMr', 'NONE', 'VPMOVWB_myrz_2  ', 'tuple=HVM'];

# VPMULTISHIFTQB.
T['VPMULTISHIFTQB VREG:w:u8=$XMMr PREG:r:Z=$Kk VREG:r:u8=$XMMn VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG', 'VPMULTISHIFTQB_ronomo', '        '];
T['VPMULTISHIFTQB VREG:w:u8=$XMMr PREG:r:Z=$Kk VREG:r:u8=$XMMn MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPMULTISHIFTQB_ronomo', 'tuple=FV'];
T['VPMULTISHIFTQB VREG:w:u8=$YMMr PREG:r:Z=$Kk VREG:r:u8=$YMMn VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG', 'VPMULTISHIFTQB_rynymy', '        '];
T['VPMULTISHIFTQB VREG:w:u8=$YMMr PREG:r:Z=$Kk VREG:r:u8=$YMMn MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPMULTISHIFTQB_rynymy', 'tuple=FV'];
T['VPMULTISHIFTQB VREG:w:u8=$ZMMr PREG:r:Z=$Kk VREG:r:u8=$ZMMn VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG', 'VPMULTISHIFTQB_rznzmz', '        '];
T['VPMULTISHIFTQB VREG:w:u8=$ZMMr PREG:r:Z=$Kk VREG:r:u8=$ZMMn MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPMULTISHIFTQB_rznzmz', 'tuple=FV'];

# VPOPCNT-Return the Count of Number of Bits Set to 1 in BYTE/WORD/DWORD/QWORD.
T['VPOPCNTB VREG:w:u8=$XMMr  PREG:r:Z=$Kk VREG:r:u8=$XMMm             ', 'MOD=REG W=0     ', 'VPOPCNTx_romo_1', '         '];
T['VPOPCNTB VREG:w:u8=$XMMr  PREG:r:Z=$Kk MEM:r:u8=$mem128            ', 'MOD=MEM W=0     ', 'VPOPCNTx_romo_1', 'tuple=FVM'];
T['VPOPCNTW VREG:w:u16=$XMMr PREG:r:Z=$Kk VREG:r:u16=$XMMm            ', 'MOD=REG W=1     ', 'VPOPCNTx_romo_1', '         '];
T['VPOPCNTW VREG:w:u16=$XMMr PREG:r:Z=$Kk MEM:r:u16=$mem128           ', 'MOD=MEM W=1     ', 'VPOPCNTx_romo_1', 'tuple=FVM'];
T['VPOPCNTD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPOPCNTx_romo_2', '         '];
T['VPOPCNTD VREG:w:u32=$XMMr PREG:r:Z=$Kk MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPOPCNTx_romo_2', 'tuple=FV '];
T['VPOPCNTQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPOPCNTx_romo_2', '         '];
T['VPOPCNTQ VREG:w:u64=$XMMr PREG:r:Z=$Kk MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPOPCNTx_romo_2', 'tuple=FV '];
T['VPOPCNTB VREG:w:u8=$YMMr  PREG:r:Z=$Kk VREG:r:u8=$YMMm             ', 'MOD=REG W=0     ', 'VPOPCNTx_rymy_1', '         '];
T['VPOPCNTB VREG:w:u8=$YMMr  PREG:r:Z=$Kk MEM:r:u8=$mem256            ', 'MOD=MEM W=0     ', 'VPOPCNTx_rymy_1', 'tuple=FVM'];
T['VPOPCNTW VREG:w:u16=$YMMr PREG:r:Z=$Kk VREG:r:u16=$YMMm            ', 'MOD=REG W=1     ', 'VPOPCNTx_rymy_1', '         '];
T['VPOPCNTW VREG:w:u16=$YMMr PREG:r:Z=$Kk MEM:r:u16=$mem256           ', 'MOD=MEM W=1     ', 'VPOPCNTx_rymy_1', 'tuple=FVM'];
T['VPOPCNTD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPOPCNTx_rymy_2', '         '];
T['VPOPCNTD VREG:w:u32=$YMMr PREG:r:Z=$Kk MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPOPCNTx_rymy_2', 'tuple=FV '];
T['VPOPCNTQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPOPCNTx_rymy_2', '         '];
T['VPOPCNTQ VREG:w:u64=$YMMr PREG:r:Z=$Kk MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPOPCNTx_rymy_2', 'tuple=FV '];
T['VPOPCNTB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk VREG:r:u8=$ZMMm             ', 'MOD=REG W=0     ', 'VPOPCNTx_rzmz_1', '         '];
T['VPOPCNTB VREG:w:u8=$ZMMr  PREG:r:Z=$Kk MEM:r:u8=$mem512            ', 'MOD=MEM W=0     ', 'VPOPCNTx_rzmz_1', 'tuple=FVM'];
T['VPOPCNTW VREG:w:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u16=$ZMMm            ', 'MOD=REG W=1     ', 'VPOPCNTx_rzmz_1', '         '];
T['VPOPCNTW VREG:w:u16=$ZMMr PREG:r:Z=$Kk MEM:r:u16=$mem512           ', 'MOD=MEM W=1     ', 'VPOPCNTx_rzmz_1', 'tuple=FVM'];
T['VPOPCNTD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPOPCNTx_rzmz_2', '         '];
T['VPOPCNTD VREG:w:u32=$ZMMr PREG:r:Z=$Kk MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPOPCNTx_rzmz_2', 'tuple=FV '];
T['VPOPCNTQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPOPCNTx_rzmz_2', '         '];
T['VPOPCNTQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPOPCNTx_rzmz_2', 'tuple=FV '];

# VPROLD/VPROLVD/VPROLQ/VPROLVQ-Bit Rotate Left.
T['VPROLVD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn             VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPROLVx_ronomo', '        '];
T['VPROLVD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn             MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPROLVx_ronomo', 'tuple=FV'];
T['VPROLVQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn             VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPROLVx_ronomo', '        '];
T['VPROLVQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn             MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPROLVx_ronomo', 'tuple=FV'];
T['VPROLVD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn             VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPROLVx_rynymy', '        '];
T['VPROLVD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn             MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPROLVx_rynymy', 'tuple=FV'];
T['VPROLVQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn             VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPROLVx_rynymy', '        '];
T['VPROLVQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn             MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPROLVx_rynymy', 'tuple=FV'];
T['VPROLVD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn             VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPROLVx_rznzmz', '        '];
T['VPROLVD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn             MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPROLVx_rznzmz', 'tuple=FV'];
T['VPROLVQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn             VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPROLVx_rznzmz', '        '];
T['VPROLVQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn             MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPROLVx_rznzmz', 'tuple=FV'];
T['VPROLD  VREG:w:u32=$XMMn PREG:r:Z=$Kk VREG:r:u32=$XMMm             ROTATE:r:u8=$rol            ', 'BP=0 MOD=REG W=0', 'VPROLx_nomoub ', '        '];
T['VPROLD  VREG:w:u32=$XMMn PREG:r:Z=$Kk MEM:r:u32=$mem128 BCST=$BCST ROTATE:r:u8=$rol            ', 'MOD=MEM W=0     ', 'VPROLx_nomoub ', 'tuple=FV'];
T['VPROLQ  VREG:w:u64=$XMMn PREG:r:Z=$Kk VREG:r:u64=$XMMm             ROTATE:r:u8=$rol            ', 'BP=0 MOD=REG W=1', 'VPROLx_nomoub ', '        '];
T['VPROLQ  VREG:w:u64=$XMMn PREG:r:Z=$Kk MEM:r:u64=$mem128 BCST=$BCST ROTATE:r:u8=$rol            ', 'MOD=MEM W=1     ', 'VPROLx_nomoub ', 'tuple=FV'];
T['VPROLD  VREG:w:u32=$YMMn PREG:r:Z=$Kk VREG:r:u32=$YMMm             ROTATE:r:u8=$rol            ', 'BP=0 MOD=REG W=0', 'VPROLx_nymyub ', '        '];
T['VPROLD  VREG:w:u32=$YMMn PREG:r:Z=$Kk MEM:r:u32=$mem256 BCST=$BCST ROTATE:r:u8=$rol            ', 'MOD=MEM W=0     ', 'VPROLx_nymyub ', 'tuple=FV'];
T['VPROLQ  VREG:w:u64=$YMMn PREG:r:Z=$Kk VREG:r:u64=$YMMm             ROTATE:r:u8=$rol            ', 'BP=0 MOD=REG W=1', 'VPROLx_nymyub ', '        '];
T['VPROLQ  VREG:w:u64=$YMMn PREG:r:Z=$Kk MEM:r:u64=$mem256 BCST=$BCST ROTATE:r:u8=$rol            ', 'MOD=MEM W=1     ', 'VPROLx_nymyub ', 'tuple=FV'];
T['VPROLD  VREG:w:u32=$ZMMn PREG:r:Z=$Kk VREG:r:u32=$ZMMm             ROTATE:r:u8=$rol            ', 'BP=0 MOD=REG W=0', 'VPROLx_nzmzub ', '        '];
T['VPROLD  VREG:w:u32=$ZMMn PREG:r:Z=$Kk MEM:r:u32=$mem512 BCST=$BCST ROTATE:r:u8=$rol            ', 'MOD=MEM W=0     ', 'VPROLx_nzmzub ', 'tuple=FV'];
T['VPROLQ  VREG:w:u64=$ZMMn PREG:r:Z=$Kk VREG:r:u64=$ZMMm             ROTATE:r:u8=$rol            ', 'BP=0 MOD=REG W=1', 'VPROLx_nzmzub ', '        '];
T['VPROLQ  VREG:w:u64=$ZMMn PREG:r:Z=$Kk MEM:r:u64=$mem512 BCST=$BCST ROTATE:r:u8=$rol            ', 'MOD=MEM W=1     ', 'VPROLx_nzmzub ', 'tuple=FV'];

# VPRORD/VPRORVD/VPRORQ/VPRORVQ-Bit Rotate Right.
T['VPRORVD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn             VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPRORVx_ronomo', '        '];
T['VPRORVD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn             MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPRORVx_ronomo', 'tuple=FV'];
T['VPRORVQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn             VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPRORVx_ronomo', '        '];
T['VPRORVQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn             MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPRORVx_ronomo', 'tuple=FV'];
T['VPRORVD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn             VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPRORVx_rynymy', '        '];
T['VPRORVD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn             MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPRORVx_rynymy', 'tuple=FV'];
T['VPRORVQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn             VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPRORVx_rynymy', '        '];
T['VPRORVQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn             MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPRORVx_rynymy', 'tuple=FV'];
T['VPRORVD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn             VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPRORVx_rznzmz', '        '];
T['VPRORVD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn             MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPRORVx_rznzmz', 'tuple=FV'];
T['VPRORVQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn             VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPRORVx_rznzmz', '        '];
T['VPRORVQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn             MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPRORVx_rznzmz', 'tuple=FV'];
T['VPRORD  VREG:w:u32=$XMMn PREG:r:Z=$Kk VREG:r:u32=$XMMm             ROTATE:r:u8=$ror            ', 'BP=0 MOD=REG W=0', 'VPRORx_nomoub ', '        '];
T['VPRORD  VREG:w:u32=$XMMn PREG:r:Z=$Kk MEM:r:u32=$mem128 BCST=$BCST ROTATE:r:u8=$ror            ', 'MOD=MEM W=0     ', 'VPRORx_nomoub ', 'tuple=FV'];
T['VPRORQ  VREG:w:u64=$XMMn PREG:r:Z=$Kk VREG:r:u64=$XMMm             ROTATE:r:u8=$ror            ', 'BP=0 MOD=REG W=1', 'VPRORx_nomoub ', '        '];
T['VPRORQ  VREG:w:u64=$XMMn PREG:r:Z=$Kk MEM:r:u64=$mem128 BCST=$BCST ROTATE:r:u8=$ror            ', 'MOD=MEM W=1     ', 'VPRORx_nomoub ', 'tuple=FV'];
T['VPRORD  VREG:w:u32=$YMMn PREG:r:Z=$Kk VREG:r:u32=$YMMm             ROTATE:r:u8=$ror            ', 'BP=0 MOD=REG W=0', 'VPRORx_nymyub ', '        '];
T['VPRORD  VREG:w:u32=$YMMn PREG:r:Z=$Kk MEM:r:u32=$mem256 BCST=$BCST ROTATE:r:u8=$ror            ', 'MOD=MEM W=0     ', 'VPRORx_nymyub ', 'tuple=FV'];
T['VPRORQ  VREG:w:u64=$YMMn PREG:r:Z=$Kk VREG:r:u64=$YMMm             ROTATE:r:u8=$ror            ', 'BP=0 MOD=REG W=1', 'VPRORx_nymyub ', '        '];
T['VPRORQ  VREG:w:u64=$YMMn PREG:r:Z=$Kk MEM:r:u64=$mem256 BCST=$BCST ROTATE:r:u8=$ror            ', 'MOD=MEM W=1     ', 'VPRORx_nymyub ', 'tuple=FV'];
T['VPRORD  VREG:w:u32=$ZMMn PREG:r:Z=$Kk VREG:r:u32=$ZMMm             ROTATE:r:u8=$ror            ', 'BP=0 MOD=REG W=0', 'VPRORx_nzmzub ', '        '];
T['VPRORD  VREG:w:u32=$ZMMn PREG:r:Z=$Kk MEM:r:u32=$mem512 BCST=$BCST ROTATE:r:u8=$ror            ', 'MOD=MEM W=0     ', 'VPRORx_nzmzub ', 'tuple=FV'];
T['VPRORQ  VREG:w:u64=$ZMMn PREG:r:Z=$Kk VREG:r:u64=$ZMMm             ROTATE:r:u8=$ror            ', 'BP=0 MOD=REG W=1', 'VPRORx_nzmzub ', '        '];
T['VPRORQ  VREG:w:u64=$ZMMn PREG:r:Z=$Kk MEM:r:u64=$mem512 BCST=$BCST ROTATE:r:u8=$ror            ', 'MOD=MEM W=1     ', 'VPRORx_nzmzub ', 'tuple=FV'];

# VPSCATTERDD/VPSCATTERDQ/VPSCATTERQD/VPSCATTERQQ-Scatter Packed Dword, Packed Qword with Signed Dword, Signed Qword Indices.
T['VPSCATTERDD MEM:w:u32=$vmx32 PREG:rw=$Kk VREG:r:u32=$XMMr', 'W=0', 'VPSCATTERDx_mxro  ', 'tuple=T1S'];
T['VPSCATTERDQ MEM:w:u64=$vmx64 PREG:rw=$Kk VREG:r:u64=$XMMr', 'W=1', 'VPSCATTERDx_mxro  ', 'tuple=T1S'];
T['VPSCATTERDD MEM:w:u32=$vmy32 PREG:rw=$Kk VREG:r:u32=$YMMr', 'W=0', 'VPSCATTERDx_mxry  ', 'tuple=T1S'];
T['VPSCATTERDQ MEM:w:u64=$vmx64 PREG:rw=$Kk VREG:r:u64=$YMMr', 'W=1', 'VPSCATTERDx_mxry  ', 'tuple=T1S'];
T['VPSCATTERDD MEM:w:u32=$vmz32 PREG:rw=$Kk VREG:r:u32=$ZMMr', 'W=0', 'VPSCATTERDx_mxrz  ', 'tuple=T1S'];
T['VPSCATTERDQ MEM:w:u64=$vmy64 PREG:rw=$Kk VREG:r:u64=$ZMMr', 'W=1', 'VPSCATTERDx_mxrz  ', 'tuple=T1S'];
T['VPSCATTERQD MEM:w:u32=$vmx32 PREG:rw=$Kk VREG:r:u32=$XMMr', 'W=0', 'VPSCATTERQx_mxro  ', 'tuple=T1S'];
T['VPSCATTERQQ MEM:w:u64=$vmx64 PREG:rw=$Kk VREG:r:u64=$XMMr', 'W=1', 'VPSCATTERQx_mxro  ', 'tuple=T1S'];
T['VPSCATTERQD MEM:w:u32=$vmy32 PREG:rw=$Kk VREG:r:u32=$XMMr', 'W=0', 'VPSCATTERQx_mxrx_1', 'tuple=T1S'];
T['VPSCATTERQQ MEM:w:u64=$vmy64 PREG:rw=$Kk VREG:r:u64=$YMMr', 'W=1', 'VPSCATTERQx_mxrx_1', 'tuple=T1S'];
T['VPSCATTERQD MEM:w:u32=$vmz32 PREG:rw=$Kk VREG:r:u32=$YMMr', 'W=0', 'VPSCATTERQx_mxrx_2', 'tuple=T1S'];
T['VPSCATTERQQ MEM:w:u64=$vmz64 PREG:rw=$Kk VREG:r:u64=$ZMMr', 'W=1', 'VPSCATTERQx_mxrx_2', 'tuple=T1S'];

# VPSHLD-Concatenate and Shift Packed Data Left Logical.
T['VPSHLDW VREG:w:u16=$XMMr PREG:r:Z=$Kk VREG:r:u16=$XMMn VREG:r:u16=$XMMm             SHIFT:r:u8=$shl', 'MOD=REG         ', 'VPSHLDW_ronomoub', '         '];
T['VPSHLDW VREG:w:u16=$XMMr PREG:r:Z=$Kk VREG:r:u16=$XMMn MEM:r:u16=$mem128            SHIFT:r:u8=$shl', 'MOD=MEM         ', 'VPSHLDW_ronomoub', 'tuple=FVM'];
T['VPSHLDW VREG:w:u16=$YMMr PREG:r:Z=$Kk VREG:r:u16=$YMMn VREG:r:u16=$YMMm             SHIFT:r:u8=$shl', 'MOD=REG         ', 'VPSHLDW_rynymyub', '         '];
T['VPSHLDW VREG:w:u16=$YMMr PREG:r:Z=$Kk VREG:r:u16=$YMMn MEM:r:u16=$mem256            SHIFT:r:u8=$shl', 'MOD=MEM         ', 'VPSHLDW_rynymyub', 'tuple=FVM'];
T['VPSHLDW VREG:w:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u16=$ZMMn VREG:r:u16=$ZMMm             SHIFT:r:u8=$shl', 'MOD=REG         ', 'VPSHLDW_rznzmzub', '         '];
T['VPSHLDW VREG:w:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u16=$ZMMn MEM:r:u16=$mem512            SHIFT:r:u8=$shl', 'MOD=MEM         ', 'VPSHLDW_rznzmzub', 'tuple=FVM'];
T['VPSHLDD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn VREG:r:u32=$XMMm             SHIFT:r:u8=$shl', 'BP=0 MOD=REG W=0', 'VPSHLDx_ronomoub', '         '];
T['VPSHLDD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST SHIFT:r:u8=$shl', 'MOD=MEM W=0     ', 'VPSHLDx_ronomoub', 'tuple=FV '];
T['VPSHLDQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm             SHIFT:r:u8=$shl', 'BP=0 MOD=REG W=1', 'VPSHLDx_ronomoub', '         '];
T['VPSHLDQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST SHIFT:r:u8=$shl', 'MOD=MEM W=1     ', 'VPSHLDx_ronomoub', 'tuple=FV '];
T['VPSHLDD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm             SHIFT:r:u8=$shl', 'BP=0 MOD=REG W=0', 'VPSHLDx_rynymyub', '         '];
T['VPSHLDD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST SHIFT:r:u8=$shl', 'MOD=MEM W=0     ', 'VPSHLDx_rynymyub', 'tuple=FV '];
T['VPSHLDQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm             SHIFT:r:u8=$shl', 'BP=0 MOD=REG W=1', 'VPSHLDx_rynymyub', '         '];
T['VPSHLDQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST SHIFT:r:u8=$shl', 'MOD=MEM W=1     ', 'VPSHLDx_rynymyub', 'tuple=FV '];
T['VPSHLDD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm             SHIFT:r:u8=$shl', 'BP=0 MOD=REG W=0', 'VPSHLDx_rznzmzub', '         '];
T['VPSHLDD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST SHIFT:r:u8=$shl', 'MOD=MEM W=0     ', 'VPSHLDx_rznzmzub', 'tuple=FV '];
T['VPSHLDQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm             SHIFT:r:u8=$shl', 'BP=0 MOD=REG W=1', 'VPSHLDx_rznzmzub', '         '];
T['VPSHLDQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST SHIFT:r:u8=$shl', 'MOD=MEM W=1     ', 'VPSHLDx_rznzmzub', 'tuple=FV '];

# VPSHLDV-Concatenate and Variable Shift Packed Data Left Logical.
T['VPSHLDVW VREG:rw:u16=$XMMr PREG:r:Z=$Kk VREG:r:u16=$XMMn VREG:r:u16=$XMMm            ', 'MOD=REG         ', 'VPSHLDVW_ronomo', '         '];
T['VPSHLDVW VREG:rw:u16=$XMMr PREG:r:Z=$Kk VREG:r:u16=$XMMn MEM:r:u16=$mem128           ', 'MOD=MEM         ', 'VPSHLDVW_ronomo', 'tuple=FVM'];
T['VPSHLDVW VREG:rw:u16=$YMMr PREG:r:Z=$Kk VREG:r:u16=$YMMn VREG:r:u16=$YMMm            ', 'MOD=REG         ', 'VPSHLDVW_rynymy', '         '];
T['VPSHLDVW VREG:rw:u16=$YMMr PREG:r:Z=$Kk VREG:r:u16=$YMMn MEM:r:u16=$mem256           ', 'MOD=MEM         ', 'VPSHLDVW_rynymy', 'tuple=FVM'];
T['VPSHLDVW VREG:rw:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u16=$ZMMn VREG:r:u16=$ZMMm            ', 'MOD=REG         ', 'VPSHLDVW_rznzmz', '         '];
T['VPSHLDVW VREG:rw:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u16=$ZMMn MEM:r:u16=$mem512           ', 'MOD=MEM         ', 'VPSHLDVW_rznzmz', 'tuple=FVM'];
T['VPSHLDVD VREG:rw:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPSHLDVx_ronomo', '         '];
T['VPSHLDVD VREG:rw:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPSHLDVx_ronomo', 'tuple=FV '];
T['VPSHLDVQ VREG:rw:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPSHLDVx_ronomo', '         '];
T['VPSHLDVQ VREG:rw:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPSHLDVx_ronomo', 'tuple=FV '];
T['VPSHLDVD VREG:rw:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPSHLDVx_rynymy', '         '];
T['VPSHLDVD VREG:rw:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPSHLDVx_rynymy', 'tuple=FV '];
T['VPSHLDVQ VREG:rw:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPSHLDVx_rynymy', '         '];
T['VPSHLDVQ VREG:rw:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPSHLDVx_rynymy', 'tuple=FV '];
T['VPSHLDVD VREG:rw:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPSHLDVx_rznzmz', '         '];
T['VPSHLDVD VREG:rw:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPSHLDVx_rznzmz', 'tuple=FV '];
T['VPSHLDVQ VREG:rw:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPSHLDVx_rznzmz', '         '];
T['VPSHLDVQ VREG:rw:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPSHLDVx_rznzmz', 'tuple=FV '];

# VPSHRD-Concatenate and Shift Packed Data Right Logical.
T['VPSHRDW VREG:w:u16=$XMMr PREG:r:Z=$Kk VREG:r:u16=$XMMn VREG:r:u16=$XMMm             SHIFT:r:u8=$shr', 'MOD=REG         ', 'VPSHRDW_ronomoub', '         '];
T['VPSHRDW VREG:w:u16=$XMMr PREG:r:Z=$Kk VREG:r:u16=$XMMn MEM:r:u16=$mem128            SHIFT:r:u8=$shr', 'MOD=MEM         ', 'VPSHRDW_ronomoub', 'tuple=FVM'];
T['VPSHRDW VREG:w:u16=$YMMr PREG:r:Z=$Kk VREG:r:u16=$YMMn VREG:r:u16=$YMMm             SHIFT:r:u8=$shr', 'MOD=REG         ', 'VPSHRDW_rynymyub', '         '];
T['VPSHRDW VREG:w:u16=$YMMr PREG:r:Z=$Kk VREG:r:u16=$YMMn MEM:r:u16=$mem256            SHIFT:r:u8=$shr', 'MOD=MEM         ', 'VPSHRDW_rynymyub', 'tuple=FVM'];
T['VPSHRDW VREG:w:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u16=$ZMMn VREG:r:u16=$ZMMm             SHIFT:r:u8=$shr', 'MOD=REG         ', 'VPSHRDW_rznzmzub', '         '];
T['VPSHRDW VREG:w:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u16=$ZMMn MEM:r:u16=$mem512            SHIFT:r:u8=$shr', 'MOD=MEM         ', 'VPSHRDW_rznzmzub', 'tuple=FVM'];
T['VPSHRDD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn VREG:r:u32=$XMMm             SHIFT:r:u8=$shr', 'BP=0 MOD=REG W=0', 'VPSHRDx_ronomoub', '         '];
T['VPSHRDD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST SHIFT:r:u8=$shr', 'MOD=MEM W=0     ', 'VPSHRDx_ronomoub', 'tuple=FV '];
T['VPSHRDQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm             SHIFT:r:u8=$shr', 'BP=0 MOD=REG W=1', 'VPSHRDx_ronomoub', '         '];
T['VPSHRDQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST SHIFT:r:u8=$shr', 'MOD=MEM W=1     ', 'VPSHRDx_ronomoub', 'tuple=FV '];
T['VPSHRDD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm             SHIFT:r:u8=$shr', 'BP=0 MOD=REG W=0', 'VPSHRDx_rynymyub', '         '];
T['VPSHRDD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST SHIFT:r:u8=$shr', 'MOD=MEM W=0     ', 'VPSHRDx_rynymyub', 'tuple=FV '];
T['VPSHRDQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm             SHIFT:r:u8=$shr', 'BP=0 MOD=REG W=1', 'VPSHRDx_rynymyub', '         '];
T['VPSHRDQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST SHIFT:r:u8=$shr', 'MOD=MEM W=1     ', 'VPSHRDx_rynymyub', 'tuple=FV '];
T['VPSHRDD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm             SHIFT:r:u8=$shr', 'BP=0 MOD=REG W=0', 'VPSHRDx_rznzmzub', '         '];
T['VPSHRDD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST SHIFT:r:u8=$shr', 'MOD=MEM W=0     ', 'VPSHRDx_rznzmzub', 'tuple=FV '];
T['VPSHRDQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm             SHIFT:r:u8=$shr', 'BP=0 MOD=REG W=1', 'VPSHRDx_rznzmzub', '         '];
T['VPSHRDQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST SHIFT:r:u8=$shr', 'MOD=MEM W=1     ', 'VPSHRDx_rznzmzub', 'tuple=FV '];

# VPSHRDV-Concatenate and Variable Shift Packed Data Right Logical.
T['VPSHRDVW VREG:rw:u16=$XMMr PREG:r:Z=$Kk VREG:r:u16=$XMMn VREG:r:u16=$XMMm            ', 'MOD=REG         ', 'VPSHRDVW_ronomo', '         '];
T['VPSHRDVW VREG:rw:u16=$XMMr PREG:r:Z=$Kk VREG:r:u16=$XMMn MEM:r:u16=$mem128           ', 'MOD=MEM         ', 'VPSHRDVW_ronomo', 'tuple=FVM'];
T['VPSHRDVW VREG:rw:u16=$YMMr PREG:r:Z=$Kk VREG:r:u16=$YMMn VREG:r:u16=$YMMm            ', 'MOD=REG         ', 'VPSHRDVW_rynymy', '         '];
T['VPSHRDVW VREG:rw:u16=$YMMr PREG:r:Z=$Kk VREG:r:u16=$YMMn MEM:r:u16=$mem256           ', 'MOD=MEM         ', 'VPSHRDVW_rynymy', 'tuple=FVM'];
T['VPSHRDVW VREG:rw:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u16=$ZMMn VREG:r:u16=$ZMMm            ', 'MOD=REG         ', 'VPSHRDVW_rznzmz', '         '];
T['VPSHRDVW VREG:rw:u16=$ZMMr PREG:r:Z=$Kk VREG:r:u16=$ZMMn MEM:r:u16=$mem512           ', 'MOD=MEM         ', 'VPSHRDVW_rznzmz', 'tuple=FVM'];
T['VPSHRDVD VREG:rw:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPSHRDVx_ronomo', '         '];
T['VPSHRDVD VREG:rw:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPSHRDVx_ronomo', 'tuple=FV '];
T['VPSHRDVQ VREG:rw:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPSHRDVx_ronomo', '         '];
T['VPSHRDVQ VREG:rw:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPSHRDVx_ronomo', 'tuple=FV '];
T['VPSHRDVD VREG:rw:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPSHRDVx_rynymy', '         '];
T['VPSHRDVD VREG:rw:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPSHRDVx_rynymy', 'tuple=FV '];
T['VPSHRDVQ VREG:rw:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPSHRDVx_rynymy', '         '];
T['VPSHRDVQ VREG:rw:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPSHRDVx_rynymy', 'tuple=FV '];
T['VPSHRDVD VREG:rw:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPSHRDVx_rznzmz', '         '];
T['VPSHRDVD VREG:rw:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPSHRDVx_rznzmz', 'tuple=FV '];
T['VPSHRDVQ VREG:rw:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPSHRDVx_rznzmz', '         '];
T['VPSHRDVQ VREG:rw:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPSHRDVx_rznzmz', 'tuple=FV '];

# VPSHUFBITQMB-Shuffle Bits from Quadword Elements Using Byte Indexes into Mask.
T['VPSHUFBITQMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u8=$XMMm ', 'MOD=REG', 'VPSHUFBITQMB_nomo', '         '];
T['VPSHUFBITQMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u8=$mem128', 'MOD=MEM', 'VPSHUFBITQMB_nomo', 'tuple=FVM'];
T['VPSHUFBITQMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u8=$YMMm ', 'MOD=REG', 'VPSHUFBITQMB_nymy', '         '];
T['VPSHUFBITQMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u8=$mem256', 'MOD=MEM', 'VPSHUFBITQMB_nymy', 'tuple=FVM'];
T['VPSHUFBITQMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u8=$ZMMm ', 'MOD=REG', 'VPSHUFBITQMB_nzmz', '         '];
T['VPSHUFBITQMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u8=$mem512', 'MOD=MEM', 'VPSHUFBITQMB_nzmz', 'tuple=FVM'];

# VPSLLVW/VPSLLVD/VPSLLVQ-Variable Bit Shift Left Logical.
T['VPSLLVW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm            ', 'MOD=REG           ', 'VPSLLVW_ronomo', '         '];
T['VPSLLVW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128           ', 'MOD=MEM           ', 'VPSLLVW_ronomo', 'tuple=FVM'];
T['VPSLLVW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm            ', 'MOD=REG           ', 'VPSLLVW_rynymy', '         '];
T['VPSLLVW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256           ', 'MOD=MEM           ', 'VPSLLVW_rynymy', 'tuple=FVM'];
T['VPSLLVW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm            ', 'MOD=REG           ', 'VPSLLVW_rznzmz', '         '];
T['VPSLLVW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512           ', 'MOD=MEM           ', 'VPSLLVW_rznzmz', 'tuple=FVM'];
T['VPSLLVD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG W=0  ', 'VPSLLVx_ronomo', '         '];
T['VPSLLVD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM W=0       ', 'VPSLLVx_ronomo', 'tuple=FV '];
T['VPSLLVQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG W=1  ', 'VPSLLVx_ronomo', '         '];
T['VPSLLVQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM W=1       ', 'VPSLLVx_ronomo', 'tuple=FV '];
T['VPSLLVD VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm                              ', 'MOD=REG VL=128 W=0', 'VPSLLVx_rxnxmx', '         '];
T['VPSLLVD VREG:w:s32=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm                              ', 'MOD=REG VL=256 W=0', 'VPSLLVx_rxnxmx', '         '];
T['VPSLLVD VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128                             ', 'MOD=MEM VL=128 W=0', 'VPSLLVx_rxnxmx', '         '];
T['VPSLLVD VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256                             ', 'MOD=MEM VL=256 W=0', 'VPSLLVx_rxnxmx', '         '];
T['VPSLLVQ VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm                              ', 'MOD=REG VL=128 W=1', 'VPSLLVx_rxnxmx', '         '];
T['VPSLLVQ VREG:w:s32=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm                              ', 'MOD=REG VL=256 W=1', 'VPSLLVx_rxnxmx', '         '];
T['VPSLLVQ VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128                             ', 'MOD=MEM VL=128 W=1', 'VPSLLVx_rxnxmx', '         '];
T['VPSLLVQ VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256                             ', 'MOD=MEM VL=256 W=1', 'VPSLLVx_rxnxmx', '         '];
T['VPSLLVD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG W=0  ', 'VPSLLVx_rynymy', '         '];
T['VPSLLVD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM W=0       ', 'VPSLLVx_rynymy', 'tuple=FV '];
T['VPSLLVQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG W=1  ', 'VPSLLVx_rynymy', '         '];
T['VPSLLVQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM W=1       ', 'VPSLLVx_rynymy', 'tuple=FV '];
T['VPSLLVD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG W=0  ', 'VPSLLVx_rznzmz', '         '];
T['VPSLLVD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM W=0       ', 'VPSLLVx_rznzmz', 'tuple=FV '];
T['VPSLLVQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG W=1  ', 'VPSLLVx_rznzmz', '         '];
T['VPSLLVQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM W=1       ', 'VPSLLVx_rznzmz', 'tuple=FV '];

# VPSRAVW/VPSRAVD/VPSRAVQ-Variable Bit Shift Right Arithmetic.
T['VPSRAVD VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm                              ', 'MOD=REG VL=128  ', 'VPSRAVD_rxnxmx', '         '];
T['VPSRAVD VREG:w:s32=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm                              ', 'MOD=REG VL=256  ', 'VPSRAVD_rxnxmx', '         '];
T['VPSRAVD VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128                             ', 'MOD=MEM VL=128  ', 'VPSRAVD_rxnxmx', '         '];
T['VPSRAVD VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256                             ', 'MOD=MEM VL=256  ', 'VPSRAVD_rxnxmx', '         '];
T['VPSRAVW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm            ', 'MOD=REG         ', 'VPSRAVW_ronomo', '         '];
T['VPSRAVW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128           ', 'MOD=MEM         ', 'VPSRAVW_ronomo', 'tuple=FVM'];
T['VPSRAVW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm            ', 'MOD=REG         ', 'VPSRAVW_rynymy', '         '];
T['VPSRAVW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256           ', 'MOD=MEM         ', 'VPSRAVW_rynymy', 'tuple=FVM'];
T['VPSRAVW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm            ', 'MOD=REG         ', 'VPSRAVW_rznzmz', '         '];
T['VPSRAVW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512           ', 'MOD=MEM         ', 'VPSRAVW_rznzmz', 'tuple=FVM'];
T['VPSRAVD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPSRAVx_ronomo', '         '];
T['VPSRAVD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPSRAVx_ronomo', 'tuple=FV '];
T['VPSRAVQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPSRAVx_ronomo', '         '];
T['VPSRAVQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPSRAVx_ronomo', 'tuple=FV '];
T['VPSRAVD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPSRAVx_rynymy', '         '];
T['VPSRAVD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPSRAVx_rynymy', 'tuple=FV '];
T['VPSRAVQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPSRAVx_rynymy', '         '];
T['VPSRAVQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPSRAVx_rynymy', 'tuple=FV '];
T['VPSRAVD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPSRAVx_rznzmz', '         '];
T['VPSRAVD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPSRAVx_rznzmz', 'tuple=FV '];
T['VPSRAVQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPSRAVx_rznzmz', '         '];
T['VPSRAVQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPSRAVx_rznzmz', 'tuple=FV '];

# VPSRLVW/VPSRLVD/VPSRLVQ-Variable Bit Shift Right Logical.
T['VPSRLVW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  VREG:r:u16=$XMMm            ', 'MOD=REG           ', 'VPSRLVW_ronomo', '         '];
T['VPSRLVW VREG:w:u16=$XMMr PREG:r:Z=$Kk     VREG:r:u16=$XMMn  MEM:r:u16=$mem128           ', 'MOD=MEM           ', 'VPSRLVW_ronomo', 'tuple=FVM'];
T['VPSRLVW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  VREG:r:u16=$YMMm            ', 'MOD=REG           ', 'VPSRLVW_rynymy', '         '];
T['VPSRLVW VREG:w:u16=$YMMr PREG:r:Z=$Kk     VREG:r:u16=$YMMn  MEM:r:u16=$mem256           ', 'MOD=MEM           ', 'VPSRLVW_rynymy', 'tuple=FVM'];
T['VPSRLVW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  VREG:r:u16=$ZMMm            ', 'MOD=REG           ', 'VPSRLVW_rznzmz', '         '];
T['VPSRLVW VREG:w:u16=$ZMMr PREG:r:Z=$Kk     VREG:r:u16=$ZMMn  MEM:r:u16=$mem512           ', 'MOD=MEM           ', 'VPSRLVW_rznzmz', 'tuple=FVM'];
T['VPSRLVD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG W=0  ', 'VPSRLVx_ronomo', '         '];
T['VPSRLVD VREG:w:u32=$XMMr PREG:r:Z=$Kk     VREG:r:u32=$XMMn  MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM W=0       ', 'VPSRLVx_ronomo', 'tuple=FV '];
T['VPSRLVQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG W=1  ', 'VPSRLVx_ronomo', '         '];
T['VPSRLVQ VREG:w:u64=$XMMr PREG:r:Z=$Kk     VREG:r:u64=$XMMn  MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM W=1       ', 'VPSRLVx_ronomo', 'tuple=FV '];
T['VPSRLVD VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm                              ', 'MOD=REG VL=128 W=0', 'VPSRLVx_rxnxmx', '         '];
T['VPSRLVD VREG:w:s32=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm                              ', 'MOD=REG VL=256 W=0', 'VPSRLVx_rxnxmx', '         '];
T['VPSRLVD VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128                             ', 'MOD=MEM VL=128 W=0', 'VPSRLVx_rxnxmx', '         '];
T['VPSRLVD VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256                             ', 'MOD=MEM VL=256 W=0', 'VPSRLVx_rxnxmx', '         '];
T['VPSRLVQ VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm                              ', 'MOD=REG VL=128 W=1', 'VPSRLVx_rxnxmx', '         '];
T['VPSRLVQ VREG:w:s32=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm                              ', 'MOD=REG VL=256 W=1', 'VPSRLVx_rxnxmx', '         '];
T['VPSRLVQ VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128                             ', 'MOD=MEM VL=128 W=1', 'VPSRLVx_rxnxmx', '         '];
T['VPSRLVQ VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256                             ', 'MOD=MEM VL=256 W=1', 'VPSRLVx_rxnxmx', '         '];
T['VPSRLVD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG W=0  ', 'VPSRLVx_rynymy', '         '];
T['VPSRLVD VREG:w:u32=$YMMr PREG:r:Z=$Kk     VREG:r:u32=$YMMn  MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM W=0       ', 'VPSRLVx_rynymy', 'tuple=FV '];
T['VPSRLVQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG W=1  ', 'VPSRLVx_rynymy', '         '];
T['VPSRLVQ VREG:w:u64=$YMMr PREG:r:Z=$Kk     VREG:r:u64=$YMMn  MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM W=1       ', 'VPSRLVx_rynymy', 'tuple=FV '];
T['VPSRLVD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG W=0  ', 'VPSRLVx_rznzmz', '         '];
T['VPSRLVD VREG:w:u32=$ZMMr PREG:r:Z=$Kk     VREG:r:u32=$ZMMn  MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM W=0       ', 'VPSRLVx_rznzmz', 'tuple=FV '];
T['VPSRLVQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG W=1  ', 'VPSRLVx_rznzmz', '         '];
T['VPSRLVQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk     VREG:r:u64=$ZMMn  MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM W=1       ', 'VPSRLVx_rznzmz', 'tuple=FV '];

# VPTERNLOGD/VPTERNLOGQ-Bitwise Ternary Logic.
T['VPTERNLOGD VREG:rw:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn VREG:r:u32=$XMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG W=0', 'VPTERNLOGx_ronomoub', '        '];
T['VPTERNLOGD VREG:rw:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM W=0     ', 'VPTERNLOGx_ronomoub', 'tuple=FV'];
T['VPTERNLOGQ VREG:rw:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG W=1', 'VPTERNLOGx_ronomoub', '        '];
T['VPTERNLOGQ VREG:rw:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM W=1     ', 'VPTERNLOGx_ronomoub', 'tuple=FV'];
T['VPTERNLOGD VREG:rw:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG W=0', 'VPTERNLOGx_rynymyub', '        '];
T['VPTERNLOGD VREG:rw:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM W=0     ', 'VPTERNLOGx_rynymyub', 'tuple=FV'];
T['VPTERNLOGQ VREG:rw:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG W=1', 'VPTERNLOGx_rynymyub', '        '];
T['VPTERNLOGQ VREG:rw:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM W=1     ', 'VPTERNLOGx_rynymyub', 'tuple=FV'];
T['VPTERNLOGD VREG:rw:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG W=0', 'VPTERNLOGx_rznzmzub', '        '];
T['VPTERNLOGD VREG:rw:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM W=0     ', 'VPTERNLOGx_rznzmzub', 'tuple=FV'];
T['VPTERNLOGQ VREG:rw:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG W=1', 'VPTERNLOGx_rznzmzub', '        '];
T['VPTERNLOGQ VREG:rw:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM W=1     ', 'VPTERNLOGx_rznzmzub', 'tuple=FV'];

# VPTESTMB/VPTESTMW/VPTESTMD/VPTESTMQ-Logical AND and Set Mask.
T['VPTESTMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$XMMn  VREG:r:u8=$XMMm             ', 'MOD=REG W=0     ', 'VPTESTMx_nomo_1', '         '];
T['VPTESTMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$XMMn  MEM:r:u8=$mem128            ', 'MOD=MEM W=0     ', 'VPTESTMx_nomo_1', 'tuple=FVM'];
T['VPTESTMW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$XMMn VREG:r:u16=$XMMm            ', 'MOD=REG W=1     ', 'VPTESTMx_nomo_1', '         '];
T['VPTESTMW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$XMMn MEM:r:u16=$mem128           ', 'MOD=MEM W=1     ', 'VPTESTMx_nomo_1', 'tuple=FVM'];
T['VPTESTMD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$XMMn VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPTESTMx_nomo_2', '         '];
T['VPTESTMD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPTESTMx_nomo_2', 'tuple=FV '];
T['VPTESTMQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPTESTMx_nomo_2', '         '];
T['VPTESTMQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPTESTMx_nomo_2', 'tuple=FV '];
T['VPTESTMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$YMMn  VREG:r:u8=$YMMm             ', 'MOD=REG W=0     ', 'VPTESTMx_nymy_1', '         '];
T['VPTESTMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$YMMn  MEM:r:u8=$mem256            ', 'MOD=MEM W=0     ', 'VPTESTMx_nymy_1', 'tuple=FVM'];
T['VPTESTMW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$YMMn VREG:r:u16=$YMMm            ', 'MOD=REG W=1     ', 'VPTESTMx_nymy_1', '         '];
T['VPTESTMW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$YMMn MEM:r:u16=$mem256           ', 'MOD=MEM W=1     ', 'VPTESTMx_nymy_1', 'tuple=FVM'];
T['VPTESTMD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPTESTMx_nymy_2', '         '];
T['VPTESTMD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPTESTMx_nymy_2', 'tuple=FV '];
T['VPTESTMQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPTESTMx_nymy_2', '         '];
T['VPTESTMQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPTESTMx_nymy_2', 'tuple=FV '];
T['VPTESTMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$ZMMn  VREG:r:u8=$ZMMm             ', 'MOD=REG W=0     ', 'VPTESTMx_nzmz_1', '         '];
T['VPTESTMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$ZMMn  MEM:r:u8=$mem512            ', 'MOD=MEM W=0     ', 'VPTESTMx_nzmz_1', 'tuple=FVM'];
T['VPTESTMW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$ZMMn VREG:r:u16=$ZMMm            ', 'MOD=REG W=1     ', 'VPTESTMx_nzmz_1', '         '];
T['VPTESTMW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$ZMMn MEM:r:u16=$mem512           ', 'MOD=MEM W=1     ', 'VPTESTMx_nzmz_1', 'tuple=FVM'];
T['VPTESTMD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPTESTMx_nzmz_2', '         '];
T['VPTESTMD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPTESTMx_nzmz_2', 'tuple=FV '];
T['VPTESTMQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPTESTMx_nzmz_2', '         '];
T['VPTESTMQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPTESTMx_nzmz_2', 'tuple=FV '];

# VPTESTNMB/W/D/Q-Logical NAND and Set.
T['VPTESTNMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$XMMn  VREG:r:u8=$XMMm             ', 'MOD=REG W=0     ', 'VPTESTNMx_nomo_1', '         '];
T['VPTESTNMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$XMMn  MEM:r:u8=$mem128            ', 'MOD=MEM W=0     ', 'VPTESTNMx_nomo_1', 'tuple=FVM'];
T['VPTESTNMW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$XMMn VREG:r:u16=$XMMm            ', 'MOD=REG W=1     ', 'VPTESTNMx_nomo_1', '         '];
T['VPTESTNMW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$XMMn MEM:r:u16=$mem128           ', 'MOD=MEM W=1     ', 'VPTESTNMx_nomo_1', 'tuple=FVM'];
T['VPTESTNMD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$XMMn VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG W=0', 'VPTESTNMx_nomo_2', '         '];
T['VPTESTNMD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM W=0     ', 'VPTESTNMx_nomo_2', 'tuple=FV '];
T['VPTESTNMQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG W=1', 'VPTESTNMx_nomo_2', '         '];
T['VPTESTNMQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM W=1     ', 'VPTESTNMx_nomo_2', 'tuple=FV '];
T['VPTESTNMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$YMMn  VREG:r:u8=$YMMm             ', 'MOD=REG W=0     ', 'VPTESTNMx_nymy_1', '         '];
T['VPTESTNMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$YMMn  MEM:r:u8=$mem256            ', 'MOD=MEM W=0     ', 'VPTESTNMx_nymy_1', 'tuple=FVM'];
T['VPTESTNMW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$YMMn VREG:r:u16=$YMMm            ', 'MOD=REG W=1     ', 'VPTESTNMx_nymy_1', '         '];
T['VPTESTNMW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$YMMn MEM:r:u16=$mem256           ', 'MOD=MEM W=1     ', 'VPTESTNMx_nymy_1', 'tuple=FVM'];
T['VPTESTNMD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG W=0', 'VPTESTNMx_nymy_2', '         '];
T['VPTESTNMD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM W=0     ', 'VPTESTNMx_nymy_2', 'tuple=FV '];
T['VPTESTNMQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG W=1', 'VPTESTNMx_nymy_2', '         '];
T['VPTESTNMQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM W=1     ', 'VPTESTNMx_nymy_2', 'tuple=FV '];
T['VPTESTNMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$ZMMn  VREG:r:u8=$ZMMm             ', 'MOD=REG W=0     ', 'VPTESTNMx_nzmz_1', '         '];
T['VPTESTNMB REG:w=$Kr PREG:r:Z=$Kk VREG:r:u8=$ZMMn  MEM:r:u8=$mem512            ', 'MOD=MEM W=0     ', 'VPTESTNMx_nzmz_1', 'tuple=FVM'];
T['VPTESTNMW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$ZMMn VREG:r:u16=$ZMMm            ', 'MOD=REG W=1     ', 'VPTESTNMx_nzmz_1', '         '];
T['VPTESTNMW REG:w=$Kr PREG:r:Z=$Kk VREG:r:u16=$ZMMn MEM:r:u16=$mem512           ', 'MOD=MEM W=1     ', 'VPTESTNMx_nzmz_1', 'tuple=FVM'];
T['VPTESTNMD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG W=0', 'VPTESTNMx_nzmz_2', '         '];
T['VPTESTNMD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM W=0     ', 'VPTESTNMx_nzmz_2', 'tuple=FV '];
T['VPTESTNMQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG W=1', 'VPTESTNMx_nzmz_2', '         '];
T['VPTESTNMQ REG:w=$Kr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM W=1     ', 'VPTESTNMx_nzmz_2', 'tuple=FV '];

# VRANGEPD-Range Restriction Calculation For Packed Pairs of Float64 Values.
T['VRANGEPD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VRANGEPD_ronomoub', '        '];
T['VRANGEPD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn MEM:r:f64=$mem128 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM             ', 'VRANGEPD_ronomoub', 'tuple=FV'];
T['VRANGEPD VREG:w:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMn VREG:r:f64=$YMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VRANGEPD_rynymyub', '        '];
T['VRANGEPD VREG:w:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMn MEM:r:f64=$mem256 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM             ', 'VRANGEPD_rynymyub', 'tuple=FV'];
T['VRANGEPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SAE=$SAE    IMM:u8=$uimm8', 'BP=1 MOD=REG RVL=512', 'VRANGEPD_rznzmzub', '        '];
T['VRANGEPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG VL=512 ', 'VRANGEPD_rznzmzub', '        '];
T['VRANGEPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r:f64=$mem512 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM VL=512      ', 'VRANGEPD_rznzmzub', 'tuple=FV'];

# VRANGEPS-Range Restriction Calculation For Packed Pairs of Float32 Values.
T['VRANGEPS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VRANGEPS_ronomoub', '        '];
T['VRANGEPS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem128 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM             ', 'VRANGEPS_ronomoub', 'tuple=FV'];
T['VRANGEPS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMn VREG:r:f32=$YMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VRANGEPS_rynymyub', '        '];
T['VRANGEPS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMn MEM:r:f32=$mem256 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM             ', 'VRANGEPS_rynymyub', 'tuple=FV'];
T['VRANGEPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SAE=$SAE    IMM:u8=$uimm8', 'BP=1 MOD=REG RVL=512', 'VRANGEPS_rznzmzub', '        '];
T['VRANGEPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG VL=512 ', 'VRANGEPS_rznzmzub', '        '];
T['VRANGEPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r:f32=$mem512 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM VL=512      ', 'VRANGEPS_rznzmzub', 'tuple=FV'];

# VRANGESD-Range Restriction Calculation From a pair of Scalar Float64 Values.
T['VRANGESD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm          IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VRANGESD', '         '];
T['VRANGESD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm SAE=$SAE IMM:u8=$uimm8', 'BP=1 MOD=REG RVL=128', 'VRANGESD', '         '];
T['VRANGESD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn MEM:r:f64=$mem64          IMM:u8=$uimm8', 'BP=0 MOD=MEM        ', 'VRANGESD', 'tuple=T1S'];

# VRANGESS-Range Restriction Calculation From a Pair of Scalar Float32 Values.
T['VRANGESS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm          IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VRANGESS', '         '];
T['VRANGESS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm SAE=$SAE IMM:u8=$uimm8', 'BP=1 MOD=REG RVL=128', 'VRANGESS', '         '];
T['VRANGESS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem32          IMM:u8=$uimm8', 'BP=0 MOD=MEM        ', 'VRANGESS', 'tuple=T1S'];

# VRCP14PD-Compute Approximate Reciprocals of Packed Float64 Values.
T['VRCP14PD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG', 'VRCP14PD_romo', '        '];
T['VRCP14PD VREG:w:f64=$XMMr PREG:r:Z=$Kk MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VRCP14PD_romo', 'tuple=FV'];
T['VRCP14PD VREG:w:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG', 'VRCP14PD_rymy', '        '];
T['VRCP14PD VREG:w:f64=$YMMr PREG:r:Z=$Kk MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VRCP14PD_rymy', 'tuple=FV'];
T['VRCP14PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG', 'VRCP14PD_rzmz', '        '];
T['VRCP14PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VRCP14PD_rzmz', 'tuple=FV'];

# VRCP14PS-Compute Approximate Reciprocals of Packed Float32 Values.
T['VRCP14PS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG', 'VRCP14PS_romo', '        '];
T['VRCP14PS VREG:w:f32=$XMMr PREG:r:Z=$Kk MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VRCP14PS_romo', 'tuple=FV'];
T['VRCP14PS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG', 'VRCP14PS_rymy', '        '];
T['VRCP14PS VREG:w:f32=$YMMr PREG:r:Z=$Kk MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VRCP14PS_rymy', 'tuple=FV'];
T['VRCP14PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG', 'VRCP14PS_rzmz', '        '];
T['VRCP14PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VRCP14PS_rzmz', 'tuple=FV'];

# VRCP14SD-Compute Approximate Reciprocal of Scalar Float64 Value.
T['VRCP14SD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm', 'MOD=REG', 'VRCP14SD', '         '];
T['VRCP14SD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn MEM:r:f64=$mem64', 'MOD=MEM', 'VRCP14SD', 'tuple=T1S'];

# VRCP14SS-Compute Approximate Reciprocal of Scalar Float32 Value.
T['VRCP14SS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm', 'MOD=REG', 'VRCP14SS', '         '];
T['VRCP14SS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem32', 'MOD=MEM', 'VRCP14SS', 'tuple=T1S'];

# VREDUCEPD-Perform Reduction Transformation on Packed Float64 Values.
T['VREDUCEPD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VREDUCEPD_romoub', '        '];
T['VREDUCEPD VREG:w:f64=$XMMr PREG:r:Z=$Kk MEM:r:f64=$mem128 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM             ', 'VREDUCEPD_romoub', 'tuple=FV'];
T['VREDUCEPD VREG:w:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VREDUCEPD_rymyub', '        '];
T['VREDUCEPD VREG:w:f64=$YMMr PREG:r:Z=$Kk MEM:r:f64=$mem256 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM             ', 'VREDUCEPD_rymyub', 'tuple=FV'];
T['VREDUCEPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SAE=$SAE    IMM:u8=$uimm8', 'BP=1 MOD=REG RVL=512', 'VREDUCEPD_rzmzub', '        '];
T['VREDUCEPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG VL=512 ', 'VREDUCEPD_rzmzub', '        '];
T['VREDUCEPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk MEM:r:f64=$mem512 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM VL=512      ', 'VREDUCEPD_rzmzub', 'tuple=FV'];

# VREDUCEPS-Perform Reduction Transformation on Packed Float32 Values.
T['VREDUCEPS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VREDUCEPS_romoub', '        '];
T['VREDUCEPS VREG:w:f32=$XMMr PREG:r:Z=$Kk MEM:r:f32=$mem128 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM             ', 'VREDUCEPS_romoub', 'tuple=FV'];
T['VREDUCEPS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VREDUCEPS_rymyub', '        '];
T['VREDUCEPS VREG:w:f32=$YMMr PREG:r:Z=$Kk MEM:r:f32=$mem256 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM             ', 'VREDUCEPS_rymyub', 'tuple=FV'];
T['VREDUCEPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SAE=$SAE    IMM:u8=$uimm8', 'BP=1 MOD=REG RVL=512', 'VREDUCEPS_rzmzub', '        '];
T['VREDUCEPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG VL=512 ', 'VREDUCEPS_rzmzub', '        '];
T['VREDUCEPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk MEM:r:f32=$mem512 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM VL=512      ', 'VREDUCEPS_rzmzub', 'tuple=FV'];

# VREDUCESD-Perform a Reduction Transformation on a Scalar Float64 Value.
T['VREDUCESD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm          IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VREDUCESD', '         '];
T['VREDUCESD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm SAE=$SAE IMM:u8=$uimm8', 'BP=1 MOD=REG RVL=128', 'VREDUCESD', '         '];
T['VREDUCESD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn MEM:r:f64=$mem64          IMM:u8=$uimm8', 'BP=0 MOD=MEM        ', 'VREDUCESD', 'tuple=T1S'];

# VREDUCESS-Perform a Reduction Transformation on a Scalar Float32 Value.
T['VREDUCESS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm          IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VREDUCESS', '         '];
T['VREDUCESS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm SAE=$SAE IMM:u8=$uimm8', 'BP=1 MOD=REG RVL=128', 'VREDUCESS', '         '];
T['VREDUCESS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem32          IMM:u8=$uimm8', 'BP=0 MOD=MEM        ', 'VREDUCESS', 'tuple=T1S'];

# VRNDSCALEPD-Round Packed Float64 Values To Include A Given Number Of Fraction Bits.
T['VRNDSCALEPD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VRNDSCALEPD_romoub', '        '];
T['VRNDSCALEPD VREG:w:f64=$XMMr PREG:r:Z=$Kk MEM:r:f64=$mem128 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM             ', 'VRNDSCALEPD_romoub', 'tuple=FV'];
T['VRNDSCALEPD VREG:w:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VRNDSCALEPD_rymyub', '        '];
T['VRNDSCALEPD VREG:w:f64=$YMMr PREG:r:Z=$Kk MEM:r:f64=$mem256 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM             ', 'VRNDSCALEPD_rymyub', 'tuple=FV'];
T['VRNDSCALEPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SAE=$SAE    IMM:u8=$uimm8', 'BP=1 MOD=REG RVL=512', 'VRNDSCALEPD_rzmzub', '        '];
T['VRNDSCALEPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG VL=512 ', 'VRNDSCALEPD_rzmzub', '        '];
T['VRNDSCALEPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk MEM:r:f64=$mem512 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM VL=512      ', 'VRNDSCALEPD_rzmzub', 'tuple=FV'];

# VRNDSCALEPS-Round Packed Float32 Values To Include A Given Number Of Fraction Bits.
T['VRNDSCALEPS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VRNDSCALEPS_romoub', '        '];
T['VRNDSCALEPS VREG:w:f32=$XMMr PREG:r:Z=$Kk MEM:r:f32=$mem128 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM             ', 'VRNDSCALEPS_romoub', 'tuple=FV'];
T['VRNDSCALEPS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VRNDSCALEPS_rymyub', '        '];
T['VRNDSCALEPS VREG:w:f32=$YMMr PREG:r:Z=$Kk MEM:r:f32=$mem256 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM             ', 'VRNDSCALEPS_rymyub', 'tuple=FV'];
T['VRNDSCALEPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SAE=$SAE    IMM:u8=$uimm8', 'BP=1 MOD=REG RVL=512', 'VRNDSCALEPS_rzmzub', '        '];
T['VRNDSCALEPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm             IMM:u8=$uimm8', 'BP=0 MOD=REG VL=512 ', 'VRNDSCALEPS_rzmzub', '        '];
T['VRNDSCALEPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk MEM:r:f32=$mem512 BCST=$BCST IMM:u8=$uimm8', 'MOD=MEM VL=512      ', 'VRNDSCALEPS_rzmzub', 'tuple=FV'];

# VRNDSCALESD-Round Scalar Float64 Value To Include A Given Number Of Fraction Bits.
T['VRNDSCALESD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm          IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VRNDSCALESD', '         '];
T['VRNDSCALESD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm SAE=$SAE IMM:u8=$uimm8', 'BP=1 MOD=REG RVL=128', 'VRNDSCALESD', '         '];
T['VRNDSCALESD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn MEM:r:f64=$mem64          IMM:u8=$uimm8', 'BP=0 MOD=MEM        ', 'VRNDSCALESD', 'tuple=T1S'];

# VRNDSCALESS-Round Scalar Float32 Value To Include A Given Number Of Fraction Bits.
T['VRNDSCALESS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm          IMM:u8=$uimm8', 'BP=0 MOD=REG        ', 'VRNDSCALESS', '         '];
T['VRNDSCALESS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm SAE=$SAE IMM:u8=$uimm8', 'BP=1 MOD=REG RVL=128', 'VRNDSCALESS', '         '];
T['VRNDSCALESS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem32          IMM:u8=$uimm8', 'BP=0 MOD=MEM        ', 'VRNDSCALESS', 'tuple=T1S'];

# VRSQRT14PD-Compute Approximate Reciprocals of Square Roots of Packed Float64 Values.
T['VRSQRT14PD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG', 'VRSQRT14PD_romo', '        '];
T['VRSQRT14PD VREG:w:f64=$XMMr PREG:r:Z=$Kk MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VRSQRT14PD_romo', 'tuple=FV'];
T['VRSQRT14PD VREG:w:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG', 'VRSQRT14PD_rymy', '        '];
T['VRSQRT14PD VREG:w:f64=$YMMr PREG:r:Z=$Kk MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VRSQRT14PD_rymy', 'tuple=FV'];
T['VRSQRT14PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG', 'VRSQRT14PD_rzmz', '        '];
T['VRSQRT14PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VRSQRT14PD_rzmz', 'tuple=FV'];

# VRSQRT14PS-Compute Approximate Reciprocals of Square Roots of Packed Float32 Values.
T['VRSQRT14PS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG', 'VRSQRT14PS_romo', '        '];
T['VRSQRT14PS VREG:w:f32=$XMMr PREG:r:Z=$Kk MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VRSQRT14PS_romo', 'tuple=FV'];
T['VRSQRT14PS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG', 'VRSQRT14PS_rymy', '        '];
T['VRSQRT14PS VREG:w:f32=$YMMr PREG:r:Z=$Kk MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VRSQRT14PS_rymy', 'tuple=FV'];
T['VRSQRT14PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG', 'VRSQRT14PS_rzmz', '        '];
T['VRSQRT14PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VRSQRT14PS_rzmz', 'tuple=FV'];

# VRSQRT14SD-Compute Approximate Reciprocal of Square Root of Scalar Float64 Value.
T['VRSQRT14SD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm', 'MOD=REG', 'VRSQRT14SD', '         '];
T['VRSQRT14SD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn MEM:r:f64=$mem64', 'MOD=MEM', 'VRSQRT14SD', 'tuple=T1S'];

# VRSQRT14SS-Compute Approximate Reciprocal of Square Root of Scalar Float32 Value.
T['VRSQRT14SS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm', 'MOD=REG', 'VRSQRT14SS', '         '];
T['VRSQRT14SS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem32', 'MOD=MEM', 'VRSQRT14SS', 'tuple=T1S'];

# VSCALEFPD-Scale Packed Float64 Values With Float64 Values.
T['VSCALEFPD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG        ', 'VSCALEFPD_ronomo', '        '];
T['VSCALEFPD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VSCALEFPD_ronomo', 'tuple=FV'];
T['VSCALEFPD VREG:w:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMn VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG        ', 'VSCALEFPD_rynymy', '        '];
T['VSCALEFPD VREG:w:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMn MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VSCALEFPD_rynymy', 'tuple=FV'];
T['VSCALEFPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VSCALEFPD_rznzmz', '        '];
T['VSCALEFPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VSCALEFPD_rznzmz', '        '];
T['VSCALEFPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VSCALEFPD_rznzmz', 'tuple=FV'];

# VSCALEFPS-Scale Packed Float32 Values With Float32 Values.
T['VSCALEFPS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG        ', 'VSCALEFPS_ronomo', '        '];
T['VSCALEFPS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM             ', 'VSCALEFPS_ronomo', 'tuple=FV'];
T['VSCALEFPS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMn VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG        ', 'VSCALEFPS_rynymy', '        '];
T['VSCALEFPS VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMn MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM             ', 'VSCALEFPS_rynymy', 'tuple=FV'];
T['VSCALEFPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC     ', 'BP=1 MOD=REG RVL=512', 'VSCALEFPS_rznzmz', '        '];
T['VSCALEFPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VSCALEFPS_rznzmz', '        '];
T['VSCALEFPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VSCALEFPS_rznzmz', 'tuple=FV'];

# VSCALEFSD-Scale Scalar Float64 Values With Float64 Values.
T['VSCALEFSD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm       ', 'BP=0 MOD=REG        ', 'VSCALEFSD', '         '];
T['VSCALEFSD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VSCALEFSD', '         '];
T['VSCALEFSD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn MEM:r:f64=$mem64       ', 'BP=0 MOD=MEM        ', 'VSCALEFSD', 'tuple=T1S'];

# VSCALEFSS-Scale Scalar Float32 Value With Float32 Value.
T['VSCALEFSS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm       ', 'BP=0 MOD=REG        ', 'VSCALEFSS', '         '];
T['VSCALEFSS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm RC=$RC', 'BP=1 MOD=REG RVL=128', 'VSCALEFSS', '         '];
T['VSCALEFSS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem32       ', 'BP=0 MOD=MEM        ', 'VSCALEFSS', 'tuple=T1S'];

# VSCATTERDPS/VSCATTERDPD/VSCATTERQPS/VSCATTERQPD-Scatter Packed Single, Packed Double with Signed Dword and Qword Indices.
T['VSCATTERDPD MEM:w:f64=$vmx64 PREG:rw=$Kk VREG:r:f64=$XMMr', 'W=1', 'VSCATTERDPx_mxro  ', 'tuple=T1S'];
T['VSCATTERDPS MEM:w:f32=$vmx32 PREG:rw=$Kk VREG:r:f32=$XMMr', 'W=0', 'VSCATTERDPx_mxro  ', 'tuple=T1S'];
T['VSCATTERDPD MEM:w:f64=$vmx64 PREG:rw=$Kk VREG:r:f64=$YMMr', 'W=1', 'VSCATTERDPx_mxry  ', 'tuple=T1S'];
T['VSCATTERDPS MEM:w:f32=$vmy32 PREG:rw=$Kk VREG:r:f32=$YMMr', 'W=0', 'VSCATTERDPx_mxry  ', 'tuple=T1S'];
T['VSCATTERDPD MEM:w:f64=$vmy64 PREG:rw=$Kk VREG:r:f64=$ZMMr', 'W=1', 'VSCATTERDPx_mxrz  ', 'tuple=T1S'];
T['VSCATTERDPS MEM:w:f32=$vmz32 PREG:rw=$Kk VREG:r:f32=$ZMMr', 'W=0', 'VSCATTERDPx_mxrz  ', 'tuple=T1S'];
T['VSCATTERQPD MEM:w:f64=$vmx64 PREG:rw=$Kk VREG:r:f64=$XMMr', 'W=1', 'VSCATTERQPx_mxro  ', 'tuple=T1S'];
T['VSCATTERQPS MEM:w:f32=$vmx32 PREG:rw=$Kk VREG:r:f32=$XMMr', 'W=0', 'VSCATTERQPx_mxro  ', 'tuple=T1S'];
T['VSCATTERQPD MEM:w:f64=$vmy64 PREG:rw=$Kk VREG:r:f64=$YMMr', 'W=1', 'VSCATTERQPx_mxrx_1', 'tuple=T1S'];
T['VSCATTERQPS MEM:w:f32=$vmy32 PREG:rw=$Kk VREG:r:f32=$XMMr', 'W=0', 'VSCATTERQPx_mxrx_1', 'tuple=T1S'];
T['VSCATTERQPD MEM:w:f64=$vmz64 PREG:rw=$Kk VREG:r:f64=$ZMMr', 'W=1', 'VSCATTERQPx_mxrx_2', 'tuple=T1S'];
T['VSCATTERQPS MEM:w:f32=$vmz32 PREG:rw=$Kk VREG:r:f32=$YMMr', 'W=0', 'VSCATTERQPx_mxrx_2', 'tuple=T1S'];

# VSHUFF32x4/VSHUFF64x2/VSHUFI32x4/VSHUFI64x2-Shuffle Packed Values at 128-bit Granularity.
T['VSHUFF32X4 VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMn VREG:r:f32=$YMMm             ORDER:r:u8=$order', 'BP=0 MOD=REG W=0', 'VSHUFFxxXx_rynymyub', '        '];
T['VSHUFF32X4 VREG:w:f32=$YMMr PREG:r:Z=$Kk VREG:r:f32=$YMMn MEM:r:f32=$mem256 BCST=$BCST ORDER:r:u8=$order', 'MOD=MEM W=0     ', 'VSHUFFxxXx_rynymyub', 'tuple=FV'];
T['VSHUFF64X2 VREG:w:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMn VREG:r:f64=$YMMm             ORDER:r:u8=$order', 'BP=0 MOD=REG W=1', 'VSHUFFxxXx_rynymyub', '        '];
T['VSHUFF64X2 VREG:w:f64=$YMMr PREG:r:Z=$Kk VREG:r:f64=$YMMn MEM:r:f64=$mem256 BCST=$BCST ORDER:r:u8=$order', 'MOD=MEM W=1     ', 'VSHUFFxxXx_rynymyub', 'tuple=FV'];
T['VSHUFF32X4 VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm             ORDER:r:u8=$order', 'BP=0 MOD=REG W=0', 'VSHUFFxxXx_rznzmzub', '        '];
T['VSHUFF32X4 VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r:f32=$mem512 BCST=$BCST ORDER:r:u8=$order', 'MOD=MEM W=0     ', 'VSHUFFxxXx_rznzmzub', 'tuple=FV'];
T['VSHUFF64X2 VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm             ORDER:r:u8=$order', 'BP=0 MOD=REG W=1', 'VSHUFFxxXx_rznzmzub', '        '];
T['VSHUFF64X2 VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r:f64=$mem512 BCST=$BCST ORDER:r:u8=$order', 'MOD=MEM W=1     ', 'VSHUFFxxXx_rznzmzub', 'tuple=FV'];
T['VSHUFI32X4 VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm             ORDER:r:u8=$order', 'BP=0 MOD=REG W=0', 'VSHUFIxxXx_rynymyub', '        '];
T['VSHUFI32X4 VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST ORDER:r:u8=$order', 'MOD=MEM W=0     ', 'VSHUFIxxXx_rynymyub', 'tuple=FV'];
T['VSHUFI64X2 VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm             ORDER:r:u8=$order', 'BP=0 MOD=REG W=1', 'VSHUFIxxXx_rynymyub', '        '];
T['VSHUFI64X2 VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST ORDER:r:u8=$order', 'MOD=MEM W=1     ', 'VSHUFIxxXx_rynymyub', 'tuple=FV'];
T['VSHUFI32X4 VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm             ORDER:r:u8=$order', 'BP=0 MOD=REG W=0', 'VSHUFIxxXx_rznzmzub', '        '];
T['VSHUFI32X4 VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST ORDER:r:u8=$order', 'MOD=MEM W=0     ', 'VSHUFIxxXx_rznzmzub', 'tuple=FV'];
T['VSHUFI64X2 VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm             ORDER:r:u8=$order', 'BP=0 MOD=REG W=1', 'VSHUFIxxXx_rznzmzub', '        '];
T['VSHUFI64X2 VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST ORDER:r:u8=$order', 'MOD=MEM W=1     ', 'VSHUFIxxXx_rznzmzub', 'tuple=FV'];

# VTESTPD/VTESTPS-Packed Bit Test.
T['VTESTPD VREG:r:f64=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG VL=128', 'VTESTPD', ''];
T['VTESTPD VREG:r:f64=$YMMr VREG:r:f64=$YMMm ', 'MOD=REG VL=256', 'VTESTPD', ''];
T['VTESTPD VREG:r:f64=$XMMr MEM:r:f64=$mem128', 'MOD=MEM VL=128', 'VTESTPD', ''];
T['VTESTPD VREG:r:f64=$YMMr MEM:r:f64=$mem256', 'MOD=MEM VL=256', 'VTESTPD', ''];
T['VTESTPS VREG:r:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG VL=128', 'VTESTPS', ''];
T['VTESTPS VREG:r:f32=$YMMr VREG:r:f32=$YMMm ', 'MOD=REG VL=256', 'VTESTPS', ''];
T['VTESTPS VREG:r:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM VL=128', 'VTESTPS', ''];
T['VTESTPS VREG:r:f32=$YMMr MEM:r:f32=$mem256', 'MOD=MEM VL=256', 'VTESTPS', ''];

# VZEROALL-Zero XMM, YMM and ZMM Registers.
T['VZEROALL', 'NONE', 'VZEROALL', ''];

# VZEROUPPER-Zero Upper Bits of YMM and ZMM Registers.
T['VZEROUPPER', 'NONE', 'VZEROUPPER', ''];

# VPANDD-Bitwise AND Int32 Vectors.
T['VPANDD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VPANDD_ronomo', '        '];
T['VPANDD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPANDD_ronomo', 'tuple=FV'];
T['VPANDD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VPANDD_rynymy', '        '];
T['VPANDD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPANDD_rynymy', 'tuple=FV'];
T['VPANDD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VPANDD_rznzmz', '        '];
T['VPANDD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPANDD_rznzmz', 'tuple=FV'];

# VPANDND-Bitwise AND NOT Int32 Vectors.
T['VPANDND VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VPANDND_ronomo', '        '];
T['VPANDND VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPANDND_ronomo', 'tuple=FV'];
T['VPANDND VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VPANDND_rynymy', '        '];
T['VPANDND VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPANDND_rynymy', 'tuple=FV'];
T['VPANDND VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VPANDND_rznzmz', '        '];
T['VPANDND VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPANDND_rznzmz', 'tuple=FV'];

# VPANDNQ-Bitwise AND NOT Int64 Vectors.
T['VPANDNQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG', 'VPANDNQ_ronomo', '        '];
T['VPANDNQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPANDNQ_ronomo', 'tuple=FV'];
T['VPANDNQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG', 'VPANDNQ_rynymy', '        '];
T['VPANDNQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPANDNQ_rynymy', 'tuple=FV'];
T['VPANDNQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG', 'VPANDNQ_rznzmz', '        '];
T['VPANDNQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPANDNQ_rznzmz', 'tuple=FV'];

# VPANDQ-Bitwise AND Int64 Vectors.
T['VPANDQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG', 'VPANDQ_ronomo', '        '];
T['VPANDQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPANDQ_ronomo', 'tuple=FV'];
T['VPANDQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG', 'VPANDQ_rynymy', '        '];
T['VPANDQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPANDQ_rynymy', 'tuple=FV'];
T['VPANDQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG', 'VPANDQ_rznzmz', '        '];
T['VPANDQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPANDQ_rznzmz', 'tuple=FV'];

# VPORD-Bitwise OR Int32 Vectors.
T['VPORD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VPORD_ronomo', '        '];
T['VPORD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPORD_ronomo', 'tuple=FV'];
T['VPORD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VPORD_rynymy', '        '];
T['VPORD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPORD_rynymy', 'tuple=FV'];
T['VPORD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VPORD_rznzmz', '        '];
T['VPORD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPORD_rznzmz', 'tuple=FV'];

# VPORQ-Bitwise OR Int64 Vectors.
T['VPORQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG', 'VPORQ_ronomo', '        '];
T['VPORQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPORQ_ronomo', 'tuple=FV'];
T['VPORQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG', 'VPORQ_rynymy', '        '];
T['VPORQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPORQ_rynymy', 'tuple=FV'];
T['VPORQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG', 'VPORQ_rznzmz', '        '];
T['VPORQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPORQ_rznzmz', 'tuple=FV'];

# VPXORD-Bitwise XOR Int32 Vectors.
T['VPXORD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn VREG:r:u32=$XMMm            ', 'BP=0 MOD=REG', 'VPXORD_ronomo', '        '];
T['VPXORD VREG:w:u32=$XMMr PREG:r:Z=$Kk VREG:r:u32=$XMMn MEM:r:u32=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPXORD_ronomo', 'tuple=FV'];
T['VPXORD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn VREG:r:u32=$YMMm            ', 'BP=0 MOD=REG', 'VPXORD_rynymy', '        '];
T['VPXORD VREG:w:u32=$YMMr PREG:r:Z=$Kk VREG:r:u32=$YMMn MEM:r:u32=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPXORD_rynymy', 'tuple=FV'];
T['VPXORD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm            ', 'BP=0 MOD=REG', 'VPXORD_rznzmz', '        '];
T['VPXORD VREG:w:u32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r:u32=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPXORD_rznzmz', 'tuple=FV'];

# VPXORQ-Bitwise XOR Int64 Vectors.
T['VPXORQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn VREG:r:u64=$XMMm            ', 'BP=0 MOD=REG', 'VPXORQ_ronomo', '        '];
T['VPXORQ VREG:w:u64=$XMMr PREG:r:Z=$Kk VREG:r:u64=$XMMn MEM:r:u64=$mem128 BCST=$BCST', 'MOD=MEM     ', 'VPXORQ_ronomo', 'tuple=FV'];
T['VPXORQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn VREG:r:u64=$YMMm            ', 'BP=0 MOD=REG', 'VPXORQ_rynymy', '        '];
T['VPXORQ VREG:w:u64=$YMMr PREG:r:Z=$Kk VREG:r:u64=$YMMn MEM:r:u64=$mem256 BCST=$BCST', 'MOD=MEM     ', 'VPXORQ_rynymy', 'tuple=FV'];
T['VPXORQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn VREG:r:u64=$ZMMm            ', 'BP=0 MOD=REG', 'VPXORQ_rznzmz', '        '];
T['VPXORQ VREG:w:u64=$ZMMr PREG:r:Z=$Kk VREG:r:u64=$ZMMn MEM:r:u64=$mem512 BCST=$BCST', 'MOD=MEM     ', 'VPXORQ_rznzmz', 'tuple=FV'];

# XORPD-Bitwise Logical XOR of Packed Double Precision Floating-Point Values.
T['VXORPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  VREG:r:f64=$XMMm            ', 'BP=0 MOD=REG  ', 'VXORPD_ronomo', '        '];
T['VXORPD VREG:w:f64=$XMMr PREG:r:Z=$Kk     VREG:r:f64=$XMMn  MEM:r:f64=$mem128 BCST=$BCST', 'MOD=MEM       ', 'VXORPD_ronomo', 'tuple=FV'];
T['VXORPD VREG:w:u64=$XMMr VREG:r:u64=$XMMn VREG:r:u64=$XMMm                              ', 'MOD=REG VL=128', 'VXORPD_rxnxmx', '        '];
T['VXORPD VREG:w:u64=$YMMr VREG:r:u64=$YMMn VREG:r:u64=$YMMm                              ', 'MOD=REG VL=256', 'VXORPD_rxnxmx', '        '];
T['VXORPD VREG:w:u64=$XMMr VREG:r:u64=$XMMn MEM:r:u64=$mem128                             ', 'MOD=MEM VL=128', 'VXORPD_rxnxmx', '        '];
T['VXORPD VREG:w:u64=$YMMr VREG:r:u64=$YMMn MEM:r:u64=$mem256                             ', 'MOD=MEM VL=256', 'VXORPD_rxnxmx', '        '];
T['VXORPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  VREG:r:f64=$YMMm            ', 'BP=0 MOD=REG  ', 'VXORPD_rynymy', '        '];
T['VXORPD VREG:w:f64=$YMMr PREG:r:Z=$Kk     VREG:r:f64=$YMMn  MEM:r:f64=$mem256 BCST=$BCST', 'MOD=MEM       ', 'VXORPD_rynymy', 'tuple=FV'];
T['VXORPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG  ', 'VXORPD_rznzmz', '        '];
T['VXORPD VREG:w:f64=$ZMMr PREG:r:Z=$Kk     VREG:r:f64=$ZMMn  MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM       ', 'VXORPD_rznzmz', 'tuple=FV'];

# XORPS-Bitwise Logical XOR of Packed Single Precision Floating-Point Values.
T['VXORPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  VREG:r:f32=$XMMm            ', 'BP=0 MOD=REG  ', 'VXORPS_ronomo', '        '];
T['VXORPS VREG:w:f32=$XMMr PREG:r:Z=$Kk     VREG:r:f32=$XMMn  MEM:r:f32=$mem128 BCST=$BCST', 'MOD=MEM       ', 'VXORPS_ronomo', 'tuple=FV'];
T['VXORPS VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm                              ', 'MOD=REG VL=128', 'VXORPS_rxnxmx', '        '];
T['VXORPS VREG:w:s32=$YMMr VREG:r:s32=$YMMn VREG:r:s32=$YMMm                              ', 'MOD=REG VL=256', 'VXORPS_rxnxmx', '        '];
T['VXORPS VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128                             ', 'MOD=MEM VL=128', 'VXORPS_rxnxmx', '        '];
T['VXORPS VREG:w:s32=$YMMr VREG:r:s32=$YMMn MEM:r:s32=$mem256                             ', 'MOD=MEM VL=256', 'VXORPS_rxnxmx', '        '];
T['VXORPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  VREG:r:f32=$YMMm            ', 'BP=0 MOD=REG  ', 'VXORPS_rynymy', '        '];
T['VXORPS VREG:w:f32=$YMMr PREG:r:Z=$Kk     VREG:r:f32=$YMMn  MEM:r:f32=$mem256 BCST=$BCST', 'MOD=MEM       ', 'VXORPS_rynymy', 'tuple=FV'];
T['VXORPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG  ', 'VXORPS_rznzmz', '        '];
T['VXORPS VREG:w:f32=$ZMMr PREG:r:Z=$Kk     VREG:r:f32=$ZMMn  MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM       ', 'VXORPS_rznzmz', 'tuple=FV'];
