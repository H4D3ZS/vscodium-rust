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
T['NOP REG:r=$GPRvm  ', 'MOD=REG', 'NOP_mv_11', ''];
T['NOP MEM:r:sx=$memv', 'MOD=MEM', 'NOP_mv_11', ''];

# CLEVICT0-Evict L1 line.
T['CLEVICT0 MEM:r:s64=$mem8                    ', 'NONE', 'CLEVICT0_mb_1', ''];
T['CLEVICT0 MEM:r:s64=$mem8 SWZ=$n_i32 EVH=$EVH', 'NONE', 'CLEVICT0_mb_2', ''];

# CLEVICT1-Evict L2 line.
T['CLEVICT1 MEM:r:s64=$mem8                    ', 'NONE', 'CLEVICT1_mb_1', ''];
T['CLEVICT1 MEM:r:s64=$mem8 SWZ=$n_i32 EVH=$EVH', 'NONE', 'CLEVICT1_mb_2', ''];

# DELAY-Stall Thread.
T['DELAY REG:r:s32=$GPR32m', 'W=0', 'DELAY', ''];
T['DELAY REG:r:s64=$GPR64m', 'W=1', 'DELAY', ''];

# JKNZD-Jump near if mask is not zero.
T['JKNZD REG:w=$Kn REL:s8=$rel8', 'NONE ', 'JKNZD_s8', ''];
T['JKNZD REG:w=$Kn REL:sx=$relz', 'F64=1', 'JKNZD_sz', ''];

# JKZD-Jump near if mask is zero.
T['JKZD REG:w=$Kn REL:s8=$rel8', 'NONE ', 'JKZD_s8', ''];
T['JKZD REG:w=$Kn REL:sx=$relz', 'F64=1', 'JKZD_sz', ''];

# KAND-AND Vector Mask.
T['KAND REG:w=$Kr REG:r=$Km', 'NONE', 'KAND', ''];

# KANDN-AND NOT Vector Mask.
T['KANDN REG:w=$Kr REG:r=$Km', 'NONE', 'KANDN', ''];

# KANDNR-Reverse AND NOT Vector Mask.
T['KANDNR REG:w=$Kr REG:r=$Km', 'NONE', 'KANDNR', ''];

# KCONCATH-Pack and Move High Vector Mask.
T['KCONCATH REG:w:s64=$GPR64r REG:r=$Kn REG:r=$Km', 'NONE', 'KCONCATH', ''];

# KCONCATL-Pack and Move Low Vector Mask.
T['KCONCATL REG:w:s64=$GPR64r REG:r=$Kn REG:r=$Km', 'NONE', 'KCONCATL', ''];

# KEXTRACT-Extract Vector Mask From Register.
T['KEXTRACT REG:w=$Kr REG:r:s64=$GPR64m IMM:u8=$index', 'NONE', 'KEXTRACT', ''];

# KMERGE2L1H-Swap and Merge High Element Portion and Low Portion of Vector Masks.
T['KMERGE2L1H REG:w=$Kr REG:r=$Km', 'NONE', 'KMERGE2L1H', ''];

# KMERGE2L1L-Move Low Element Portion into High Portion of Vector Mask.
T['KMERGE2L1L REG:w=$Kr REG:r=$Km', 'NONE', 'KMERGE2L1L', ''];

# KMOV-Move Vector Mask.
T['KMOV REG:w=$Kr     REG:r=$Km    ', 'NONE', 'KMOV_V ', ''];
T['KMOV REG:w=$Kr     REG:r=$GPR32m', 'NONE', 'KMOV_mq', ''];
T['KMOV REG:w=$GPR32r REG:r=$Km    ', 'NONE', 'KMOV_rq', ''];

# KNOT-Not Vector Mask.
T['KNOT REG:w=$Kr REG:r=$Km', 'NONE', 'KNOT', ''];

# KOR-OR Vector Masks.
T['KOR REG:w=$Kr REG:r=$Km', 'NONE', 'KOR', ''];

# KORTEST-OR Vector Mask And Set EFLAGS.
T['KORTEST REG:r=$Kr REG:r=$Km', 'NONE', 'KORTEST', ''];

# KXNOR-XNOR Vector Masks.
T['KXNOR REG:w=$Kr REG:r=$Km', 'NONE', 'KXNOR', ''];

# KXOR-XOR Vector Masks.
T['KXOR REG:w=$Kr REG:r=$Km', 'NONE', 'KXOR', ''];

# LZCNT-Leading Zero Count.
T['LZCNT REG:w:s32=$GPR32r REG:r:s32=$GPR32m', 'W=0', 'LZCNT_rxmx', ''];
T['LZCNT REG:w:s64=$GPR64r REG:r:s64=$GPR64m', 'W=1', 'LZCNT_rxmx', ''];

# POPCNT-Return the Count of Number of Bits Set to 1.
T['POPCNT REG:w:s32=$GPR32r REG:r:s32=$GPR32m', 'W=0', 'POPCNT_rxmx', ''];
T['POPCNT REG:w:s64=$GPR64r REG:r:s64=$GPR64m', 'W=1', 'POPCNT_rxmx', ''];

# PREFETCHWT1-Prefetch Vector Data Into Caches with Intent to Write and T1 Hint.
T['PREFETCHWT1 MEM:r:u8=$mem8', 'NONE', 'PREFETCHWT1', ''];

# SPFLT-Set performance monitor filtering mask.
T['SPFLT REG:r:s32=$GPR32m', 'W=0', 'SPFLT', ''];
T['SPFLT REG:r:s64=$GPR64m', 'W=1', 'SPFLT', ''];

# TZCNT-Trailing Zero Count.
T['TZCNT REG:w:s32=$GPR32r REG:r:s32=$GPR32m', 'W=0', 'TZCNT_rxmx', ''];
T['TZCNT REG:w:s64=$GPR64r REG:r:s64=$GPR64m', 'W=1', 'TZCNT_rxmx', ''];

# TZCNTI-Initialized Trailing Zero Count.
T['TZCNTI REG:rw:s32=$GPR32r REG:r:s32=$GPR32m', 'W=0', 'TZCNTI', ''];
T['TZCNTI REG:rw:s64=$GPR64r REG:r:s64=$GPR64m', 'W=1', 'TZCNTI', ''];

# V4FMADDPS-Packed Single-Precision Floating-Point Fused Multiply-Add (4-iterations).
T['V4FMADDPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r:f32=$mem128', 'NONE', 'V4FMADDPS', 'tuple=T4'];

# V4FMADDSS-Scalar Single-Precision Floating-Point Fused Multiply-Add (4-iterations).
T['V4FMADDSS VREG:rw:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem128', 'NONE', 'V4FMADDSS', 'tuple=T4'];

# V4FNMADDPS-Packed Single-Precision Floating-Point Fused Multiply-Add (4-iterations).
T['V4FNMADDPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r:f32=$mem128', 'NONE', 'V4FNMADDPS', 'tuple=T4'];

# V4FNMADDSS-Scalar Single-Precision Floating-Point Fused Multiply-Add (4-iterations).
T['V4FNMADDSS VREG:rw:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem128', 'NONE', 'V4FNMADDSS', 'tuple=T4'];

# VADDNPD-Add and Negate Float64 Vectors.
T['VADDNPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VADDNPD', ''];
T['VADDNPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VADDNPD', ''];
T['VADDNPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VADDNPD', ''];

# VADDNPS-Add and Negate Float32 Vectors.
T['VADDNPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VADDNPS', ''];
T['VADDNPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VADDNPS', ''];
T['VADDNPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VADDNPS', ''];

# VADDPD-Add Float64 Vectors.
T['VADDPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VADDPD_rznzmx', ''];
T['VADDPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VADDPD_rznzmx', ''];
T['VADDPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VADDPD_rznzmx', ''];

# VADDPS-Add Float32 Vectors.
T['VADDPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VADDPS_rznzmx', ''];
T['VADDPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VADDPS_rznzmx', ''];
T['VADDPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VADDPS_rznzmx', ''];

# VADDSETSPS-Add Float32 Vectors and Set Mask to Sign.
T['VADDSETSPS VREG:rw:f32=$ZMMr PREG:rw:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'NONE', 'VADDSETSPS_rznzm0', ''];
T['VADDSETSPS VREG:rw:f32=$ZMMr PREG:rw:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 ', 'VADDSETSPS_rznzmz', ''];
T['VADDSETSPS VREG:rw:f32=$ZMMr PREG:rw:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 ', 'VADDSETSPS_rznzmz', ''];

# VALIGND-Align Doubleword Vectors.
T['VALIGND VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm                      IMM:u8=$uimm8', 'E=0 MOD=REG', 'VALIGND', ''];
T['VALIGND VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r:s32=$mem512 SWZ=$n_i32 EVH=$EVH IMM:u8=$uimm8', 'MOD=MEM    ', 'VALIGND', ''];

# VBLENDMPD-Blend Float64 Vectors using the Instruction Mask.
T['VBLENDMPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VBLENDMPD', ''];
T['VBLENDMPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VBLENDMPD', ''];
T['VBLENDMPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VBLENDMPD', ''];

# VBLENDMPS-Blend Float32 Vectors using the Instruction Mask.
T['VBLENDMPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VBLENDMPS', ''];
T['VBLENDMPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VBLENDMPS', ''];
T['VBLENDMPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VBLENDMPS', ''];

# VBROADCASTF32X4-Broadcast 4xFloat32 Vector.
T['VBROADCASTF32X4 VREG:rw:f32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SBCST:SUPP={FROM=4 TO=16} SWZ=$u_f32 EVH=$EVH', 'NONE', 'VBROADCASTF32X4_rzm0', ''];

# VBROADCASTF64X4-Broadcast 4xFloat64 Vector.
T['VBROADCASTF64X4 VREG:rw:f64=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SBCST:SUPP={FROM=4 TO=8} SWZ=$u_f64 EVH=$EVH', 'NONE', 'VBROADCASTF64X4_rzm0', ''];

# VBROADCASTI32X4-Broadcast 4xInt32 Vector.
T['VBROADCASTI32X4 VREG:rw:s32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SBCST:SUPP={FROM=4 TO=16} SWZ=$u_i32 EVH=$EVH', 'NONE', 'VBROADCASTI32X4_rzm0', ''];

# VBROADCASTI64X4-Broadcast 4xInt64 Vector.
T['VBROADCASTI64X4 VREG:rw:s64=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SBCST:SUPP={FROM=4 TO=8} SWZ=$u_f64 EVH=$EVH', 'NONE', 'VBROADCASTI64X4_rzm0', ''];

# VBROADCASTSD-Broadcast Float64 Vector.
T['VBROADCASTSD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SBCST:SUPP={FROM=1 TO=8} SWZ=$u_f64 EVH=$EVH', 'NONE', 'VBROADCASTSD_rzm0', ''];

# VBROADCASTSS-Broadcast Float32 Vector.
T['VBROADCASTSS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SBCST:SUPP={FROM=1 TO=16} SWZ=$u_f32 EVH=$EVH', 'NONE', 'VBROADCASTSS_rzm0', ''];

# VCMPPD-Compare Float64 Vectors and Set Vector Mask.
T['VCMPPD REG:w=$Kr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64     CC:u8=$cc5', 'E=0 MOD=REG', 'VCMPPD_nzmxub', ''];
T['VCMPPD REG:w=$Kr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SAE=$SAE       CC:u8=$cc5', 'E=1 MOD=REG', 'VCMPPD_nzmxub', ''];
T['VCMPPD REG:w=$Kr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH CC:u8=$cc5', 'MOD=MEM    ', 'VCMPPD_nzmxub', ''];

# VCMPPS-Compare Float32 Vectors and Set Vector Mask.
T['VCMPPS REG:w=$Kr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i64     CC:u8=$cc5', 'E=0 MOD=REG', 'VCMPPS_nzmxub', ''];
T['VCMPPS REG:w=$Kr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SAE=$SAE       CC:u8=$cc5', 'E=1 MOD=REG', 'VCMPPS_nzmxub', ''];
T['VCMPPS REG:w=$Kr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH CC:u8=$cc5', 'MOD=MEM    ', 'VCMPPS_nzmxub', ''];

# VCVTDQ2PD-Convert Int32 Vector to Float64 Vector.
T['VCVTDQ2PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SWZ=$r_i32     ', 'E=0 MOD=REG', 'VCVTDQ2PD_rzmx', ''];
T['VCVTDQ2PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm                ', 'E=1 MOD=REG', 'VCVTDQ2PD_rzmx', ''];
T['VCVTDQ2PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_hi32 EVH=$EVH', 'MOD=MEM    ', 'VCVTDQ2PD_rzmx', ''];

# VCVTFXPNTDQ2PS-Convert Fixed Point Int32 Vector to Float32 Vector.
T['VCVTFXPNTDQ2PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMm SWZ=$r_i32     IMM:u8=$uimm8', 'E=0 MOD=REG', 'VCVTFXPNTDQ2PS', ''];
T['VCVTFXPNTDQ2PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMm SAE=$SAE       IMM:u8=$uimm8', 'E=1 MOD=REG', 'VCVTFXPNTDQ2PS', ''];
T['VCVTFXPNTDQ2PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_i32 EVH=$EVH IMM:u8=$uimm8', 'MOD=MEM    ', 'VCVTFXPNTDQ2PS', ''];

# VCVTFXPNTPD2DQ-Convert Float64 Vector to Fixed Point Int32 Vector.
T['VCVTFXPNTPD2DQ VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SWZ=$r_i64     IMM:u8=$uimm8', 'E=0 MOD=REG', 'VCVTFXPNTPD2DQ', ''];
T['VCVTFXPNTPD2DQ VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SAE=$SAE       IMM:u8=$uimm8', 'E=1 MOD=REG', 'VCVTFXPNTPD2DQ', ''];
T['VCVTFXPNTPD2DQ VREG:rw:s32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_f64 EVH=$EVH IMM:u8=$uimm8', 'MOD=MEM    ', 'VCVTFXPNTPD2DQ', ''];

# VCVTFXPNTPD2UDQ-Convert Float64 Vector to Fixed Point Uint32 Vector.
T['VCVTFXPNTPD2UDQ VREG:rw:u32=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SWZ=$r_i64     IMM:u8=$uimm8', 'E=0 MOD=REG', 'VCVTFXPNTPD2UDQ', ''];
T['VCVTFXPNTPD2UDQ VREG:rw:u32=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SAE=$SAE       IMM:u8=$uimm8', 'E=1 MOD=REG', 'VCVTFXPNTPD2UDQ', ''];
T['VCVTFXPNTPD2UDQ VREG:rw:u32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_f64 EVH=$EVH IMM:u8=$uimm8', 'MOD=MEM    ', 'VCVTFXPNTPD2UDQ', ''];

# VCVTFXPNTPS2DQ-Convert Float32 Vector to Fixed Point Int32 Vector.
T['VCVTFXPNTPS2DQ VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SWZ=$r_i32     IMM:u8=$uimm8', 'E=0 MOD=REG', 'VCVTFXPNTPS2DQ', ''];
T['VCVTFXPNTPS2DQ VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SAE=$SAE       IMM:u8=$uimm8', 'E=1 MOD=REG', 'VCVTFXPNTPS2DQ', ''];
T['VCVTFXPNTPS2DQ VREG:rw:s32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_f32 EVH=$EVH IMM:u8=$uimm8', 'MOD=MEM    ', 'VCVTFXPNTPS2DQ', ''];

# VCVTFXPNTPS2UDQ-Convert Float32 Vector to Fixed Point Uint32 Vector.
T['VCVTFXPNTPS2UDQ VREG:rw:u32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SWZ=$r_i32     IMM:u8=$uimm8', 'E=0 MOD=REG', 'VCVTFXPNTPS2UDQ', ''];
T['VCVTFXPNTPS2UDQ VREG:rw:u32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SAE=$SAE       IMM:u8=$uimm8', 'E=1 MOD=REG', 'VCVTFXPNTPS2UDQ', ''];
T['VCVTFXPNTPS2UDQ VREG:rw:u32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_f32 EVH=$EVH IMM:u8=$uimm8', 'MOD=MEM    ', 'VCVTFXPNTPS2UDQ', ''];

# VCVTFXPNTUDQ2PS-Convert Fixed Point Uint32 Vector to Float32 Vector.
T['VCVTFXPNTUDQ2PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMm SWZ=$r_i32     IMM:u8=$uimm8', 'E=0 MOD=REG', 'VCVTFXPNTUDQ2PS', ''];
T['VCVTFXPNTUDQ2PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:u32=$ZMMm SAE=$SAE       IMM:u8=$uimm8', 'E=1 MOD=REG', 'VCVTFXPNTUDQ2PS', ''];
T['VCVTFXPNTUDQ2PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_i32 EVH=$EVH IMM:u8=$uimm8', 'MOD=MEM    ', 'VCVTFXPNTUDQ2PS', ''];

# VCVTPD2PS-Convert Float64 Vector to Float32 Vector.
T['VCVTPD2PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VCVTPD2PS_rzmx', ''];
T['VCVTPD2PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VCVTPD2PS_rzmx', ''];
T['VCVTPD2PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VCVTPD2PS_rzmx', ''];

# VCVTPS2PD-Convert Float32 Vector to Float64 Vector.
T['VCVTPS2PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SWZ=$r_i32     ', 'E=0 MOD=REG', 'VCVTPS2PD_rzmx', ''];
T['VCVTPS2PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SAE=$SAE       ', 'E=1 MOD=REG', 'VCVTPS2PD_rzmx', ''];
T['VCVTPS2PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_hf32 EVH=$EVH', 'MOD=MEM    ', 'VCVTPS2PD_rzmx', ''];

# VCVTUDQ2PD-Convert Uint32 Vector to Float64 Vector.
T['VCVTUDQ2PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SWZ=$r_i32     ', 'E=0 MOD=REG', 'VCVTUDQ2PD_rzmx', ''];
T['VCVTUDQ2PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm                ', 'E=1 MOD=REG', 'VCVTUDQ2PD_rzmx', ''];
T['VCVTUDQ2PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_hi32 EVH=$EVH', 'MOD=MEM    ', 'VCVTUDQ2PD_rzmx', ''];

# VEXP223PS-Base-2 Exponential Calculation of Float32 Vector.
T['VEXP223PS VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMm                     ', 'E=0 MOD=REG', 'VEXP223PS', ''];
T['VEXP223PS VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMm SAE=$SAE            ', 'E=1 MOD=REG', 'VEXP223PS', ''];
T['VEXP223PS VREG:rw:s32=$ZMMr PREG:r:Z=$Kk MEM:r:s32=$mem512 SWZ=$n_f32 EVH=$EVH', 'MOD=MEM    ', 'VEXP223PS', ''];

# VEXP2PD-Approximation to the Exponential 2^x of Packed Double-Precision Floating-Point Values with Less Than 2^-23 Relative Error.
T['VEXP2PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VEXP2PD', '        '];
T['VEXP2PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VEXP2PD', '        '];
T['VEXP2PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VEXP2PD', 'tuple=FV'];

# VEXP2PS-Approximation to the Exponential 2^x of Packed Single-Precision Floating-Point Values with Less Than 2^-23 Relative Error.
T['VEXP2PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VEXP2PS', '        '];
T['VEXP2PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VEXP2PS', '        '];
T['VEXP2PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VEXP2PS', 'tuple=FV'];

# VFIXUPNANPD-Fix Up Special Float64 Vector Numbers With NaN Passthrough.
T['VFIXUPNANPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VFIXUPNANPD', ''];
T['VFIXUPNANPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SAE=$SAE      ', 'E=1 MOD=REG', 'VFIXUPNANPD', ''];
T['VFIXUPNANPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_i64 EVH=$EVH', 'MOD=MEM    ', 'VFIXUPNANPD', ''];

# VFIXUPNANPS-Fix Up Special Float32 Vector Numbers With NaN Passthrough.
T['VFIXUPNANPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VFIXUPNANPS', ''];
T['VFIXUPNANPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SAE=$SAE      ', 'E=1 MOD=REG', 'VFIXUPNANPS', ''];
T['VFIXUPNANPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VFIXUPNANPS', ''];

# VFMADD132PD-Multiply Destination By Second Source and Add To First Source Float64 Vectors 201.
T['VFMADD132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VFMADD132PD_rznzmx', ''];
T['VFMADD132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFMADD132PD_rznzmx', ''];
T['VFMADD132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VFMADD132PD_rznzmx', ''];

# VFMADD132PS-Multiply Destination By Second Source and Add To First Source Float32 Vectors 205.
T['VFMADD132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VFMADD132PS_rznzmx', ''];
T['VFMADD132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFMADD132PS_rznzmx', ''];
T['VFMADD132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VFMADD132PS_rznzmx', ''];

# VFMADD213PD-Multiply First Source By Destination and Add Second Source Float64 Vectors . 208.
T['VFMADD213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VFMADD213PD_rznzmx', ''];
T['VFMADD213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFMADD213PD_rznzmx', ''];
T['VFMADD213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VFMADD213PD_rznzmx', ''];

# VFMADD213PS-Multiply First Source By Destination and Add Second Source Float32 Vectors . . 212.
T['VFMADD213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VFMADD213PS_rznzmx', ''];
T['VFMADD213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFMADD213PS_rznzmx', ''];
T['VFMADD213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VFMADD213PS_rznzmx', ''];

# VFMADD231PD-Multiply First Source By Second Source and Add To Destination Float64 Vectors 216.
T['VFMADD231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VFMADD231PD_rznzmx', ''];
T['VFMADD231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFMADD231PD_rznzmx', ''];
T['VFMADD231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VFMADD231PD_rznzmx', ''];

# VFMADD231PS-Multiply First Source By Second Source and Add To Destination Float32 Vectors 220.
T['VFMADD231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VFMADD231PS_rznzmx', ''];
T['VFMADD231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFMADD231PS_rznzmx', ''];
T['VFMADD231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VFMADD231PS_rznzmx', ''];

# VFMADD233PS-Multiply First Source By Specially Swizzled Second Source and Add To Second Source Float32 Vectors.
T['VFMADD233PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32     ', 'E=0 MOD=REG', 'VFMADD233PS', ''];
T['VFMADD233PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC         ', 'E=1 MOD=REG', 'VFMADD233PS', ''];
T['VFMADD233PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_mf32 EVH=$EVH', 'MOD=MEM    ', 'VFMADD233PS', ''];

# VFMSUB132PD-Multiply Destination By Second Source and Subtract First Source Float64 Vectors228.
T['VFMSUB132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VFMSUB132PD_rznzmx', ''];
T['VFMSUB132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFMSUB132PD_rznzmx', ''];
T['VFMSUB132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VFMSUB132PD_rznzmx', ''];

# VFMSUB132PS-Multiply Destination By Second Source and Subtract First Source Float32 Vectors232.
T['VFMSUB132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VFMSUB132PS_rznzmx', ''];
T['VFMSUB132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFMSUB132PS_rznzmx', ''];
T['VFMSUB132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VFMSUB132PS_rznzmx', ''];

# VFMSUB213PD-Multiply First Source By Destination and Subtract Second Source Float64 Vectors235.
T['VFMSUB213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VFMSUB213PD_rznzmx', ''];
T['VFMSUB213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFMSUB213PD_rznzmx', ''];
T['VFMSUB213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VFMSUB213PD_rznzmx', ''];

# VFMSUB213PS-Multiply First Source By Destination and Subtract Second Source Float32 Vectors239.
T['VFMSUB213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VFMSUB213PS_rznzmx', ''];
T['VFMSUB213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFMSUB213PS_rznzmx', ''];
T['VFMSUB213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VFMSUB213PS_rznzmx', ''];

# VFMSUB231PD-Multiply First Source By Second Source and Subtract Destination Float64 Vectors242.
T['VFMSUB231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VFMSUB231PD_rznzmx', ''];
T['VFMSUB231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFMSUB231PD_rznzmx', ''];
T['VFMSUB231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VFMSUB231PD_rznzmx', ''];

# VFMSUB231PS-Multiply First Source By Second Source and Subtract Destination Float32 Vectors246.
T['VFMSUB231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VFMSUB231PS_rznzmx', ''];
T['VFMSUB231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFMSUB231PS_rznzmx', ''];
T['VFMSUB231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VFMSUB231PS_rznzmx', ''];

# VFNMADD132PD-Multiply Destination By Second Source and Subtract From First Source Float64 Vectors.
T['VFNMADD132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VFNMADD132PD_rznzmx', ''];
T['VFNMADD132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFNMADD132PD_rznzmx', ''];
T['VFNMADD132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VFNMADD132PD_rznzmx', ''];

# VFNMADD132PS-Multiply Destination By Second Source and Subtract From First Source Float32 Vectors.
T['VFNMADD132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VFNMADD132PS_rznzmx', ''];
T['VFNMADD132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFNMADD132PS_rznzmx', ''];
T['VFNMADD132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VFNMADD132PS_rznzmx', ''];

# VFNMADD213PD-Multiply First Source By Destination and Subtract From Second Source Float64 Vectors.
T['VFNMADD213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VFNMADD213PD_rznzmx', ''];
T['VFNMADD213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFNMADD213PD_rznzmx', ''];
T['VFNMADD213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VFNMADD213PD_rznzmx', ''];

# VFNMADD213PS-Multiply First Source By Destination and Subtract From Second Source Float32 Vectors.
T['VFNMADD213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VFNMADD213PS_rznzmx', ''];
T['VFNMADD213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFNMADD213PS_rznzmx', ''];
T['VFNMADD213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VFNMADD213PS_rznzmx', ''];

# VFNMADD231PD-Multiply First Source By Second Source and Subtract From Destination Float64 Vectors.
T['VFNMADD231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VFNMADD231PD_rznzmx', ''];
T['VFNMADD231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFNMADD231PD_rznzmx', ''];
T['VFNMADD231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VFNMADD231PD_rznzmx', ''];

# VFNMADD231PS-Multiply First Source By Second Source and Subtract From Destination Float32 Vectors.
T['VFNMADD231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VFNMADD231PS_rznzmx', ''];
T['VFNMADD231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFNMADD231PS_rznzmx', ''];
T['VFNMADD231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VFNMADD231PS_rznzmx', ''];

# VFNMSUB132PD-Multiply Destination By Second Source, Negate, and Subtract First Source Float64 Vectors.
T['VFNMSUB132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VFNMSUB132PD_rznzmx', ''];
T['VFNMSUB132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFNMSUB132PD_rznzmx', ''];
T['VFNMSUB132PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VFNMSUB132PD_rznzmx', ''];

# VFNMSUB132PS-Multiply Destination By Second Source, Negate, and Subtract First Source Float32 Vectors.
T['VFNMSUB132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VFNMSUB132PS_rznzmx', ''];
T['VFNMSUB132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFNMSUB132PS_rznzmx', ''];
T['VFNMSUB132PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VFNMSUB132PS_rznzmx', ''];

# VFNMSUB213PD-Multiply First Source By Destination, Negate, and Subtract Second Source Float64 Vectors.
T['VFNMSUB213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VFNMSUB213PD_rznzmx', ''];
T['VFNMSUB213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFNMSUB213PD_rznzmx', ''];
T['VFNMSUB213PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VFNMSUB213PD_rznzmx', ''];

# VFNMSUB213PS-Multiply First Source By Destination, Negate, and Subtract Second Source Float32 Vectors.
T['VFNMSUB213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VFNMSUB213PS_rznzmx', ''];
T['VFNMSUB213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFNMSUB213PS_rznzmx', ''];
T['VFNMSUB213PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VFNMSUB213PS_rznzmx', ''];

# VFNMSUB231PD-Multiply First Source By Second Source, Negate, and Subtract Destination Float64 Vectors.
T['VFNMSUB231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VFNMSUB231PD_rznzmx', ''];
T['VFNMSUB231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFNMSUB231PD_rznzmx', ''];
T['VFNMSUB231PD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VFNMSUB231PD_rznzmx', ''];

# VFNMSUB231PS-Multiply First Source By Second Source, Negate, and Subtract Destination Float32 Vectors.
T['VFNMSUB231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VFNMSUB231PS_rznzmx', ''];
T['VFNMSUB231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VFNMSUB231PS_rznzmx', ''];
T['VFNMSUB231PS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VFNMSUB231PS_rznzmx', ''];

# VGATHERDPD-Gather Float64 Vector With Signed Dword Indices.
T['VGATHERDPD VREG:rw:f64=$ZMMr PREG:rw:Z=$Kk MEM:r=$vmz SBCST:SUPP={FROM=1 TO=8} SWZ=$u_f64 EVH=$EVH', 'NONE', 'VGATHERDPD', 'egran=1'];

# VGATHERDPS-Gather Float32 Vector With Signed Dword Indices.
T['VGATHERDPS VREG:rw:f32=$ZMMr PREG:rw:Z=$Kk MEM:r=$vmz SBCST:SUPP={FROM=1 TO=16} SWZ=$u_f32 EVH=$EVH', 'NONE', 'VGATHERDPS', 'egran=1'];

# VGATHERPF0DPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint.
T['VGATHERPF0DPD MEM:r:f64=$vmy64 PREG:rw=$Kk', 'NONE', 'VGATHERPF0DPD', 'tuple=T1S'];

# VGATHERPF0DPS-Gather Prefetch Float32 Vector With Signed Dword Indices Into L1.
T['VGATHERPF0DPS MEM:r=$vmz SWZ=$u_f32 EVH=$EVH PREG:rw:Z=$Kk', 'NONE', 'VGATHERPF0DPS_m0', 'egran=1  '];
T['VGATHERPF0DPS MEM:r:f32=$vmz32               PREG:rw=$Kk  ', 'NONE', 'VGATHERPF0DPS_md', 'tuple=T1S'];

# VGATHERPF0HINTDPD-Gather Prefetch Float64 Vector Hint With Signed Dword Indices.
T['VGATHERPF0HINTDPD MEM:r=$vmz SWZ=$u_f32 EVH=$EVH PREG:rw:Z=$Kk', 'NONE', 'VGATHERPF0HINTDPD', 'egran=1'];

# VGATHERPF0HINTDPS-Gather Prefetch Float32 Vector Hint With Signed Dword Indices.
T['VGATHERPF0HINTDPS MEM:r=$vmz SWZ=$u_f32 EVH=$EVH PREG:rw:Z=$Kk', 'NONE', 'VGATHERPF0HINTDPS', 'egran=1'];

# VGATHERPF0QPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint.
T['VGATHERPF0QPD MEM:r:f64=$vmz64 PREG:rw=$Kk', 'NONE', 'VGATHERPF0QPD', 'tuple=T1S'];

# VGATHERPF0QPS-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint.
T['VGATHERPF0QPS MEM:r:f32=$vmz32 PREG:rw=$Kk', 'NONE', 'VGATHERPF0QPS', 'tuple=T1S'];

# VGATHERPF1DPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint.
T['VGATHERPF1DPD MEM:r:f64=$vmy64 PREG:rw=$Kk', 'NONE', 'VGATHERPF1DPD', 'tuple=T1S'];

# VGATHERPF1DPS-Gather Prefetch Float32 Vector With Signed Dword Indices Into L2.
T['VGATHERPF1DPS MEM:r=$vmz SWZ=$u_f32 EVH=$EVH PREG:rw:Z=$Kk', 'NONE', 'VGATHERPF1DPS_m0', 'egran=1  '];
T['VGATHERPF1DPS MEM:r:f32=$vmz32               PREG:rw=$Kk  ', 'NONE', 'VGATHERPF1DPS_md', 'tuple=T1S'];

# VGATHERPF1QPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint.
T['VGATHERPF1QPD MEM:r:f64=$vmz64 PREG:rw=$Kk', 'NONE', 'VGATHERPF1QPD', 'tuple=T1S'];

# VGATHERPF1QPS-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint.
T['VGATHERPF1QPS MEM:r:f32=$vmz32 PREG:rw=$Kk', 'NONE', 'VGATHERPF1QPS', 'tuple=T1S'];

# VGETEXPPD-Extract Float64 Vector of Exponents from Float64 Vector.
T['VGETEXPPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VGETEXPPD_rzmx', ''];
T['VGETEXPPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SAE=$SAE      ', 'E=1 MOD=REG', 'VGETEXPPD_rzmx', ''];
T['VGETEXPPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VGETEXPPD_rzmx', ''];

# VGETEXPPS-Extract Float32 Vector of Exponents from Float32 Vector.
T['VGETEXPPS VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VGETEXPPS_rzmx', ''];
T['VGETEXPPS VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SAE=$SAE      ', 'E=1 MOD=REG', 'VGETEXPPS_rzmx', ''];
T['VGETEXPPS VREG:rw:f64=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VGETEXPPS_rzmx', ''];

# VGETMANTPD-Extract Float64 Vector of Normalized Mantissas from Float64 Vector.
T['VGETMANTPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SWZ=$r_i64     SIGNC:r:u8=$signc', 'E=0 MOD=REG', 'VGETMANTPD_rzmxub', ''];
T['VGETMANTPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SAE=$SAE       SIGNC:r:u8=$signc', 'E=1 MOD=REG', 'VGETMANTPD_rzmxub', ''];
T['VGETMANTPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_f64 EVH=$EVH SIGNC:r:u8=$signc', 'MOD=MEM    ', 'VGETMANTPD_rzmxub', ''];

# VGETMANTPS-Extract Float32 Vector of Normalized Mantissas from Float32 Vector.
T['VGETMANTPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SWZ=$r_i32     SIGNC:r:u8=$signc', 'E=0 MOD=REG', 'VGETMANTPS_rzmxub', ''];
T['VGETMANTPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SAE=$SAE       SIGNC:r:u8=$signc', 'E=1 MOD=REG', 'VGETMANTPS_rzmxub', ''];
T['VGETMANTPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_f32 EVH=$EVH SIGNC:r:u8=$signc', 'MOD=MEM    ', 'VGETMANTPS_rzmxub', ''];

# VGMAXABSPS-Absolute Maximum of Float32 Vectors.
T['VGMAXABSPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VGMAXABSPS', ''];
T['VGMAXABSPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SAE=$SAE      ', 'E=1 MOD=REG', 'VGMAXABSPS', ''];
T['VGMAXABSPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VGMAXABSPS', ''];

# VGMAXPD-Maximum of Float64 Vectors.
T['VGMAXPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VGMAXPD', ''];
T['VGMAXPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SAE=$SAE      ', 'E=1 MOD=REG', 'VGMAXPD', ''];
T['VGMAXPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VGMAXPD', ''];

# VGMAXPS-Maximum of Float32 Vectors.
T['VGMAXPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VGMAXPS', ''];
T['VGMAXPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VGMAXPS', ''];
T['VGMAXPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VGMAXPS', ''];

# VGMINPD-Minimum of Float64 Vectors.
T['VGMINPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VGMINPD', ''];
T['VGMINPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SAE=$SAE      ', 'E=1 MOD=REG', 'VGMINPD', ''];
T['VGMINPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VGMINPD', ''];

# VGMINPS-Minimum of Float32 Vectors.
T['VGMINPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VGMINPS', ''];
T['VGMINPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VGMINPS', ''];
T['VGMINPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VGMINPS', ''];

# VLOADUNPACKHD-Load Unaligned High And Unpack To Doubleword Vector.
T['VLOADUNPACKHD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$u_i32 EVH=$EVH', 'NONE', 'VLOADUNPACKHD', 'egran=1'];

# VLOADUNPACKHPD-Load Unaligned High And Unpack To Float64 Vector.
T['VLOADUNPACKHPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$u_f64 EVH=$EVH', 'NONE', 'VLOADUNPACKHPD', 'egran=1'];

# VLOADUNPACKHPS-Load Unaligned High And Unpack To Float32 Vector.
T['VLOADUNPACKHPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$u_f32 EVH=$EVH', 'NONE', 'VLOADUNPACKHPS', 'egran=1'];

# VLOADUNPACKHQ-Load Unaligned High And Unpack To Int64 Vector.
T['VLOADUNPACKHQ VREG:rw:s64=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$u_i64 EVH=$EVH', 'NONE', 'VLOADUNPACKHQ', 'egran=1'];

# VLOADUNPACKLD-Load Unaligned Low And Unpack To Doubleword Vector.
T['VLOADUNPACKLD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$u_i32 EVH=$EVH', 'NONE', 'VLOADUNPACKLD', 'egran=1'];

# VLOADUNPACKLPD-Load Unaligned Low And Unpack To Float64 Vector.
T['VLOADUNPACKLPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$u_f64 EVH=$EVH', 'NONE', 'VLOADUNPACKLPD', 'egran=1'];

# VLOADUNPACKLPS-Load Unaligned Low And Unpack To Float32 Vector.
T['VLOADUNPACKLPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$u_f32 EVH=$EVH', 'NONE', 'VLOADUNPACKLPS', 'egran=1'];

# VLOADUNPACKLQ-Load Unaligned Low And Unpack To Int64 Vector.
T['VLOADUNPACKLQ VREG:rw:s64=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$u_i64 EVH=$EVH', 'NONE', 'VLOADUNPACKLQ', 'egran=1'];

# VLOG2PS-Vector Logarithm Base-2 of Float32 Vector.
T['VLOG2PS VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMm                     ', 'E=0 MOD=REG', 'VLOG2PS', ''];
T['VLOG2PS VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMm SAE=$SAE            ', 'E=1 MOD=REG', 'VLOG2PS', ''];
T['VLOG2PS VREG:rw:s32=$ZMMr PREG:r:Z=$Kk MEM:r:s32=$mem512 SWZ=$n_f32 EVH=$EVH', 'MOD=MEM    ', 'VLOG2PS', ''];

# VMOVAPD-Move Aligned Float64 Vector.
T['VMOVAPD MEM:w:f64=$mem512 PREG:r:Z=$Kk VREG:r:f64=$ZMMr SWZ=$d_f64 EVH=$EVH', 'NONE       ', 'VMOVAPD_mzrz_3', ''];
T['VMOVAPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SWZ=$r_i64         ', 'E=0 MOD=REG', 'VMOVAPD_rzmx  ', ''];
T['VMOVAPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm                    ', 'E=1 MOD=REG', 'VMOVAPD_rzmx  ', ''];
T['VMOVAPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$u_f64 EVH=$EVH      ', 'MOD=MEM    ', 'VMOVAPD_rzmx  ', ''];

# VMOVAPS-Move Aligned Float32 Vector.
T['VMOVAPS MEM:w=$mem        PREG:r:Z=$Kk VREG:r:f32=$ZMMr SWZ=$d_f32 EVH=$EVH', 'NONE       ', 'VMOVAPS_m0rz', ''];
T['VMOVAPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SWZ=$r_i32         ', 'E=0 MOD=REG', 'VMOVAPS_rzmx', ''];
T['VMOVAPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm                    ', 'E=1 MOD=REG', 'VMOVAPS_rzmx', ''];
T['VMOVAPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$u_f32 EVH=$EVH      ', 'MOD=MEM    ', 'VMOVAPS_rzmx', ''];

# VMOVDQA32-Move Aligned Int32 Vector.
T['VMOVDQA32 MEM:w=$mem        PREG:r:Z=$Kk VREG:r:s32=$ZMMr SWZ=$d_i32 EVH=$EVH', 'NONE       ', 'VMOVDQA32_m0rz', ''];
T['VMOVDQA32 VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMm SWZ=$r_i32         ', 'E=0 MOD=REG', 'VMOVDQA32_rzmx', ''];
T['VMOVDQA32 VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMm                    ', 'E=1 MOD=REG', 'VMOVDQA32_rzmx', ''];
T['VMOVDQA32 VREG:rw:s32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$u_i32 EVH=$EVH      ', 'MOD=MEM    ', 'VMOVDQA32_rzmx', ''];

# VMOVDQA64-Move Aligned Int64 Vector.
T['VMOVDQA64 MEM:w:s64=$mem512 PREG:r:Z=$Kk VREG:r:s64=$ZMMr SWZ=$d_i64 EVH=$EVH', 'NONE       ', 'VMOVDQA64_mzrz', ''];
T['VMOVDQA64 VREG:rw:s64=$ZMMr PREG:r:Z=$Kk VREG:r:s64=$ZMMm SWZ=$r_i64         ', 'E=0 MOD=REG', 'VMOVDQA64_rzmx', ''];
T['VMOVDQA64 VREG:rw:s64=$ZMMr PREG:r:Z=$Kk VREG:r:s64=$ZMMm                    ', 'E=1 MOD=REG', 'VMOVDQA64_rzmx', ''];
T['VMOVDQA64 VREG:rw:s64=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$u_i64 EVH=$EVH      ', 'MOD=MEM    ', 'VMOVDQA64_rzmx', ''];

# VMOVNRAPD-Store Aligned Float64 Vector With No-Read Hint.
T['VMOVNRAPD MEM:rw:f64=$mem512 PREG:r:Z=$Kk VREG:r:f64=$ZMMr SWZ=$d_f64 EVH=$EVH', 'NONE', 'VMOVNRAPD', ''];

# VMOVNRAPS-Store Aligned Float32 Vector With No-Read Hint.
T['VMOVNRAPS MEM:rw=$mem PREG:r:Z=$Kk VREG:r:f32=$ZMMr SWZ=$d_f32 EVH=$EVH', 'NONE', 'VMOVNRAPS', ''];

# VMOVNRNGOAPD-Non-globally Ordered Store Aligned Float64 Vector With No-Read Hint.
T['VMOVNRNGOAPD MEM:rw:f64=$mem512 PREG:r:Z=$Kk VREG:r:f64=$ZMMr SWZ=$d_f64 EVH=$EVH', 'NONE', 'VMOVNRNGOAPD', ''];

# VMOVNRNGOAPS-Non-globally Ordered Store Aligned Float32 Vector With No-Read Hint.
T['VMOVNRNGOAPS MEM:rw=$mem PREG:r:Z=$Kk VREG:r:f32=$ZMMr SWZ=$d_f32 EVH=$EVH', 'NONE', 'VMOVNRNGOAPS', ''];

# VMULPD-Multiply Float64 Vectors.
T['VMULPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VMULPD_rznzmx', ''];
T['VMULPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VMULPD_rznzmx', ''];
T['VMULPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VMULPD_rznzmx', ''];

# VMULPS-Multiply Float32 Vectors.
T['VMULPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VMULPS_rznzmx', ''];
T['VMULPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VMULPS_rznzmx', ''];
T['VMULPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VMULPS_rznzmx', ''];

# VP4DPWSSD-Dot Product of Signed Words with Dword Accumulation (4-iterations).
T['VP4DPWSSD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s16=$ZMMn MEM:r:u32=$mem128', 'NONE', 'VP4DPWSSD', 'tuple=T4'];

# VP4DPWSSDS-Dot Product of Signed Words with Dword Accumulation and Saturation (4-iterations).
T['VP4DPWSSDS VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s16=$ZMMn MEM:r:u32=$mem128', 'NONE', 'VP4DPWSSDS', 'tuple=T4'];

# VPACKSTOREHD-Pack And Store Unaligned High From Int32 Vector.
T['VPACKSTOREHD MEM:w=$mem PREG:r:Z=$Kk VREG:r:s32=$ZMMr SWZ=$d_i32 EVH=$EVH', 'NONE', 'VPACKSTOREHD', 'egran=1'];

# VPACKSTOREHPD-Pack And Store Unaligned High From Float64 Vector.
T['VPACKSTOREHPD MEM:w=$mem PREG:r:Z=$Kk VREG:r:f64=$ZMMr SWZ=$d_f64 EVH=$EVH', 'NONE', 'VPACKSTOREHPD', 'egran=1'];

# VPACKSTOREHPS-Pack And Store Unaligned High From Float32 Vector.
T['VPACKSTOREHPS MEM:w=$mem PREG:r:Z=$Kk VREG:r:f32=$ZMMr SWZ=$d_f32 EVH=$EVH', 'NONE', 'VPACKSTOREHPS', 'egran=1'];

# VPACKSTOREHQ-Pack And Store Unaligned High From Int64 Vector.
T['VPACKSTOREHQ MEM:w=$mem PREG:r:Z=$Kk VREG:r:s64=$ZMMr SWZ=$d_i64 EVH=$EVH', 'NONE', 'VPACKSTOREHQ', 'egran=1'];

# VPACKSTORELD-Pack and Store Unaligned Low From Int32 Vector.
T['VPACKSTORELD MEM:w=$mem PREG:r:Z=$Kk VREG:r:s32=$ZMMr SWZ=$d_i32 EVH=$EVH', 'NONE', 'VPACKSTORELD', 'egran=1'];

# VPACKSTORELPD-Pack and Store Unaligned Low From Float64 Vector.
T['VPACKSTORELPD MEM:w=$mem PREG:r:Z=$Kk VREG:r:f64=$ZMMr SWZ=$d_f64 EVH=$EVH', 'NONE', 'VPACKSTORELPD', 'egran=1'];

# VPACKSTORELPS-Pack and Store Unaligned Low From Float32 Vector.
T['VPACKSTORELPS MEM:w=$mem PREG:r:Z=$Kk VREG:r:f32=$ZMMr SWZ=$d_f32 EVH=$EVH', 'NONE', 'VPACKSTORELPS', 'egran=1'];

# VPACKSTORELQ-Pack and Store Unaligned Low From Int64 Vector.
T['VPACKSTORELQ MEM:w=$mem PREG:r:Z=$Kk VREG:r:s64=$ZMMr SWZ=$d_i64 EVH=$EVH', 'NONE', 'VPACKSTORELQ', 'egran=1'];

# VPADCD-Add Int32 Vectors with Carry.
T['VPADCD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn VREG:r:s32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPADCD', ''];
T['VPADCD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn VREG:r:s32=$ZMMm               ', 'E=1 MOD=REG', 'VPADCD', ''];
T['VPADCD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPADCD', ''];

# VPADDD-Add Int32 Vectors.
T['VPADDD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPADDD_rznzmx', ''];
T['VPADDD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VPADDD_rznzmx', ''];
T['VPADDD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPADDD_rznzmx', ''];

# VPADDSETCD-Add Int32 Vectors and Set Mask to Carry.
T['VPADDSETCD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn VREG:r:s32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPADDSETCD', ''];
T['VPADDSETCD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn VREG:r:s32=$ZMMm               ', 'E=1 MOD=REG', 'VPADDSETCD', ''];
T['VPADDSETCD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPADDSETCD', ''];

# VPADDSETSD-Add Int32 Vectors and Set Mask to Sign.
T['VPADDSETSD VREG:rw:f32=$ZMMr PREG:rw:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPADDSETSD', ''];
T['VPADDSETSD VREG:rw:f32=$ZMMr PREG:rw:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VPADDSETSD', ''];
T['VPADDSETSD VREG:rw:f32=$ZMMr PREG:rw:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPADDSETSD', ''];

# VPANDD-Bitwise AND Int32 Vectors.
T['VPANDD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPANDD_rznzmx', ''];
T['VPANDD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VPANDD_rznzmx', ''];
T['VPANDD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPANDD_rznzmx', ''];

# VPANDND-Bitwise AND NOT Int32 Vectors.
T['VPANDND VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPANDND_rznzmx', ''];
T['VPANDND VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VPANDND_rznzmx', ''];
T['VPANDND VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPANDND_rznzmx', ''];

# VPANDNQ-Bitwise AND NOT Int64 Vectors.
T['VPANDNQ VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VPANDNQ_rznzmx', ''];
T['VPANDNQ VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm               ', 'E=1 MOD=REG', 'VPANDNQ_rznzmx', ''];
T['VPANDNQ VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_i64 EVH=$EVH', 'MOD=MEM    ', 'VPANDNQ_rznzmx', ''];

# VPANDQ-Bitwise AND Int64 Vectors.
T['VPANDQ VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VPANDQ_rznzmx', ''];
T['VPANDQ VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm               ', 'E=1 MOD=REG', 'VPANDQ_rznzmx', ''];
T['VPANDQ VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_i64 EVH=$EVH', 'MOD=MEM    ', 'VPANDQ_rznzmx', ''];

# VPBLENDMD-Blend Int32 Vectors using the Instruction Mask.
T['VPBLENDMD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPBLENDMD', ''];
T['VPBLENDMD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VPBLENDMD', ''];
T['VPBLENDMD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPBLENDMD', ''];

# VPBLENDMQ-Blend Int64 Vectors using the Instruction Mask.
T['VPBLENDMQ VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VPBLENDMQ', ''];
T['VPBLENDMQ VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm               ', 'E=1 MOD=REG', 'VPBLENDMQ', ''];
T['VPBLENDMQ VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_i64 EVH=$EVH', 'MOD=MEM    ', 'VPBLENDMQ', ''];

# VPBROADCASTD-Broadcast Int32 Vector.
T['VPBROADCASTD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SBCST:SUPP={FROM=1 TO=16} SWZ=$u_i32 EVH=$EVH', 'NONE', 'VPBROADCASTD_rzm0', ''];

# VPBROADCASTQ-Broadcast Int64 Vector.
T['VPBROADCASTQ VREG:rw:s64=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SBCST:SUPP={FROM=1 TO=8} SWZ=$u_f64 EVH=$EVH', 'NONE', 'VPBROADCASTQ_rzm0', ''];

# VPCMPD-Compare Int32 Vectors and Set Vector Mask.
T['VPCMPD REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$ZMMn VREG:r:s32=$ZMMm SWZ=$r_i64     CC:u8=$cc3', 'E=0 MOD=REG', 'VPCMPD_nzmxub', ''];
T['VPCMPD REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$ZMMn VREG:r:s32=$ZMMm                CC:u8=$cc3', 'E=1 MOD=REG', 'VPCMPD_nzmxub', ''];
T['VPCMPD REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH CC:u8=$cc3', 'MOD=MEM    ', 'VPCMPD_nzmxub', ''];

# VPCMPEQD-Compare Equal Int32 Vectors and Set Vector Mask.
T['VPCMPEQD REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$ZMMn VREG:r:s32=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VPCMPEQD_nzmx', ''];
T['VPCMPEQD REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$ZMMn VREG:r:s32=$ZMMm               ', 'E=1 MOD=REG', 'VPCMPEQD_nzmx', ''];
T['VPCMPEQD REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPCMPEQD_nzmx', ''];

# VPCMPGTD-Compare Greater Than Int32 Vectors and Set Vector Mask.
T['VPCMPGTD REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$ZMMn VREG:r:s32=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VPCMPGTD_nzmx', ''];
T['VPCMPGTD REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$ZMMn VREG:r:s32=$ZMMm               ', 'E=1 MOD=REG', 'VPCMPGTD_nzmx', ''];
T['VPCMPGTD REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPCMPGTD_nzmx', ''];

# VPCMPLTD-Compare Less Than Int32 Vectors and Set Vector Mask.
T['VPCMPLTD REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$ZMMn VREG:r:s32=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VPCMPLTD', ''];
T['VPCMPLTD REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$ZMMn VREG:r:s32=$ZMMm               ', 'E=1 MOD=REG', 'VPCMPLTD', ''];
T['VPCMPLTD REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPCMPLTD', ''];

# VPCMPUD-Compare Uint32 Vectors and Set Vector Mask.
T['VPCMPUD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm SWZ=$r_i64     CC:u8=$cc3', 'E=0 MOD=REG', 'VPCMPUD_nzmxub', ''];
T['VPCMPUD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$ZMMn VREG:r:u32=$ZMMm                CC:u8=$cc3', 'E=1 MOD=REG', 'VPCMPUD_nzmxub', ''];
T['VPCMPUD REG:w=$Kr PREG:r:Z=$Kk VREG:r:u32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH CC:u8=$cc3', 'MOD=MEM    ', 'VPCMPUD_nzmxub', ''];

# VPERMD-Permutes Int32 Vectors.
T['VPERMD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm                     ', 'E=0 MOD=REG', 'VPERMD_rznzmz_2', ''];
T['VPERMD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r:s32=$mem512 SWZ=$n_i32 EVH=$EVH', 'MOD=MEM    ', 'VPERMD_rznzmz_2', ''];

# VPERMF32X4-Shuffle Vector Dqwords.
T['VPERMF32X4 VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMm                      ORDER:r:u8=$order', 'E=0 MOD=REG', 'VPERMF32X4', ''];
T['VPERMF32X4 VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMm                      ORDER:r:u8=$order', 'E=1 MOD=REG', 'VPERMF32X4', ''];
T['VPERMF32X4 VREG:rw:s32=$ZMMr PREG:r:Z=$Kk MEM:r:s32=$mem512 SWZ=$n_i32 EVH=$EVH ORDER:r:u8=$order', 'MOD=MEM    ', 'VPERMF32X4', ''];

# VPGATHERDD-Gather Int32 Vector With Signed Dword Indices.
T['VPGATHERDD VREG:rw:s32=$ZMMr PREG:rw:Z=$Kk MEM:r=$vmz SBCST:SUPP={FROM=1 TO=16} SWZ=$u_i32 EVH=$EVH', 'NONE', 'VPGATHERDD', ''];

# VPGATHERDQ-Gather Int64 Vector With Signed Dword Indices.
T['VPGATHERDQ VREG:rw:s64=$ZMMr PREG:rw:Z=$Kk MEM:r=$vmz SBCST:SUPP={FROM=1 TO=8} SWZ=$u_i64 EVH=$EVH', 'NONE', 'VPGATHERDQ', ''];

# VPMADD231D-Multiply First Source By Second Source and Add To Destination Int32 Vectors.
T['VPMADD231D VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPMADD231D', ''];
T['VPMADD231D VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VPMADD231D', ''];
T['VPMADD231D VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPMADD231D', ''];

# VPMADD233D-Multiply First Source By Specially Swizzled Second Source and Add To Second Source Int32 Vectors.
T['VPMADD233D VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32     ', 'E=0 MOD=REG', 'VPMADD233D', ''];
T['VPMADD233D VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm                ', 'E=1 MOD=REG', 'VPMADD233D', ''];
T['VPMADD233D VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_mi32 EVH=$EVH', 'MOD=MEM    ', 'VPMADD233D', ''];

# VPMAXSD-Maximum of Int32 Vectors.
T['VPMAXSD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPMAXSD_rznzmx', ''];
T['VPMAXSD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VPMAXSD_rznzmx', ''];
T['VPMAXSD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPMAXSD_rznzmx', ''];

# VPMAXUD-Maximum of Uint32 Vectors.
T['VPMAXUD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPMAXUD_rznzmx', ''];
T['VPMAXUD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VPMAXUD_rznzmx', ''];
T['VPMAXUD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPMAXUD_rznzmx', ''];

# VPMINSD-Minimum of Int32 Vectors.
T['VPMINSD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPMINSD_rznzmx', ''];
T['VPMINSD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VPMINSD_rznzmx', ''];
T['VPMINSD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPMINSD_rznzmx', ''];

# VPMINUD-Minimum of Uint32 Vectors.
T['VPMINUD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPMINUD_rznzmx', ''];
T['VPMINUD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VPMINUD_rznzmx', ''];
T['VPMINUD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPMINUD_rznzmx', ''];

# VPMULHD-Multiply Int32 Vectors And Store High Result.
T['VPMULHD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPMULHD', ''];
T['VPMULHD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VPMULHD', ''];
T['VPMULHD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPMULHD', ''];

# VPMULHUD-Multiply Uint32 Vectors And Store High Result.
T['VPMULHUD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPMULHUD', ''];
T['VPMULHUD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VPMULHUD', ''];
T['VPMULHUD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPMULHUD', ''];

# VPMULLD-Multiply Int32 Vectors And Store Low Result.
T['VPMULLD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPMULLD_rznzmx', ''];
T['VPMULLD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VPMULLD_rznzmx', ''];
T['VPMULLD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPMULLD_rznzmx', ''];

# VPORD-Bitwise OR Int32 Vectors.
T['VPORD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPORD_rznzmx', ''];
T['VPORD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VPORD_rznzmx', ''];
T['VPORD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPORD_rznzmx', ''];

# VPORQ-Bitwise OR Int64 Vectors.
T['VPORQ VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VPORQ_rznzmx', ''];
T['VPORQ VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm               ', 'E=1 MOD=REG', 'VPORQ_rznzmx', ''];
T['VPORQ VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_i64 EVH=$EVH', 'MOD=MEM    ', 'VPORQ_rznzmx', ''];

# VPREFETCH0-Prefetch memory line using T0 hint.
T['VPREFETCH0 MEM:r:s64=$mem8                    ', 'NONE', 'VPREFETCH0_mb_1', ''];
T['VPREFETCH0 MEM:r:s64=$mem8 SWZ=$n_i32 EVH=$EVH', 'NONE', 'VPREFETCH0_mb_2', ''];

# VPREFETCH1-Prefetch memory line using T1 hint.
T['VPREFETCH1 MEM:r:s64=$mem8                    ', 'NONE', 'VPREFETCH1_mb_1', ''];
T['VPREFETCH1 MEM:r:s64=$mem8 SWZ=$n_i32 EVH=$EVH', 'NONE', 'VPREFETCH1_mb_2', ''];

# VPREFETCH2-Prefetch memory line using T2 hint.
T['VPREFETCH2 MEM:r:s64=$mem8                    ', 'NONE', 'VPREFETCH2_mb_1', ''];
T['VPREFETCH2 MEM:r:s64=$mem8 SWZ=$n_i32 EVH=$EVH', 'NONE', 'VPREFETCH2_mb_2', ''];

# VPREFETCHE0-Prefetch memory line using T0 hint, with intent to write.
T['VPREFETCHE0 MEM:r:s64=$mem8                    ', 'NONE', 'VPREFETCHE0_mb_1', ''];
T['VPREFETCHE0 MEM:r:s64=$mem8 SWZ=$n_i32 EVH=$EVH', 'NONE', 'VPREFETCHE0_mb_2', ''];

# VPREFETCHE1-Prefetch memory line using T1 hint, with intent to write.
T['VPREFETCHE1 MEM:r:s64=$mem8                    ', 'NONE', 'VPREFETCHE1_mb_1', ''];
T['VPREFETCHE1 MEM:r:s64=$mem8 SWZ=$n_i32 EVH=$EVH', 'NONE', 'VPREFETCHE1_mb_2', ''];

# VPREFETCHE2-Prefetch memory line using T2 hint, with intent to write.
T['VPREFETCHE2 MEM:r:s64=$mem8                    ', 'NONE', 'VPREFETCHE2_mb_1', ''];
T['VPREFETCHE2 MEM:r:s64=$mem8 SWZ=$n_i32 EVH=$EVH', 'NONE', 'VPREFETCHE2_mb_2', ''];

# VPREFETCHENTA-Prefetch memory line using NTA hint, with intent to write.
T['VPREFETCHENTA MEM:r:s64=$mem8                    ', 'NONE', 'VPREFETCHENTA_mb_1', ''];
T['VPREFETCHENTA MEM:r:s64=$mem8 SWZ=$n_i32 EVH=$EVH', 'NONE', 'VPREFETCHENTA_mb_2', ''];

# VPREFETCHNTA-Prefetch memory line using NTA hint.
T['VPREFETCHNTA MEM:r:s64=$mem8                    ', 'NONE', 'VPREFETCHNTA_mb_1', ''];
T['VPREFETCHNTA MEM:r:s64=$mem8 SWZ=$n_i32 EVH=$EVH', 'NONE', 'VPREFETCHNTA_mb_2', ''];

# VPSBBD-Subtract Int32 Vectors with Borrow.
T['VPSBBD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn VREG:r:s32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPSBBD', ''];
T['VPSBBD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn VREG:r:s32=$ZMMm               ', 'E=1 MOD=REG', 'VPSBBD', ''];
T['VPSBBD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPSBBD', ''];

# VPSBBRD-Reverse Subtract Int32 Vectors with Borrow.
T['VPSBBRD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn VREG:r:s32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPSBBRD', ''];
T['VPSBBRD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn VREG:r:s32=$ZMMm               ', 'E=1 MOD=REG', 'VPSBBRD', ''];
T['VPSBBRD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPSBBRD', ''];

# VPSCATTERDD-Scatter Int32 Vector With Signed Dword Indices.
T['VPSCATTERDD MEM:w=$vmz SBCST:SUPP={FROM=1 TO=16} PREG:rw:Z=$Kk VREG:r:s32=$ZMMr SWZ=$d_i32 EVH=$EVH', 'NONE', 'VPSCATTERDD', ''];

# VPSCATTERDQ-Scatter Int64 Vector With Signed Dword Indices.
T['VPSCATTERDQ MEM:w=$vmz SBCST:SUPP={FROM=1 TO=8} PREG:rw:Z=$Kk VREG:r:s64=$ZMMr SWZ=$d_i64 EVH=$EVH', 'NONE', 'VPSCATTERDQ', ''];

# VPSHUFD-Shuffle Vector Doublewords.
T['VPSHUFD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMm                      ORDER:r:u8=$order', 'E=0 MOD=REG', 'VPSHUFD_rzmzub_2', ''];
T['VPSHUFD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMm                      ORDER:r:u8=$order', 'E=1 MOD=REG', 'VPSHUFD_rzmzub_2', ''];
T['VPSHUFD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk MEM:r:s32=$mem512 SWZ=$n_i32 EVH=$EVH ORDER:r:u8=$order', 'MOD=MEM    ', 'VPSHUFD_rzmzub_2', ''];

# VPSLLD-Shift Int32 Vector Immediate Left Logical.
T['VPSLLD VREG:rw:f32=$ZMMn PREG:r:Z=$Kk VREG:r:f32=$ZMMm SWZ=$r_i32     SHIFT:r:u8=$shl', 'E=0 MOD=REG', 'VPSLLD_nzmxub', ''];
T['VPSLLD VREG:rw:f32=$ZMMn PREG:r:Z=$Kk VREG:r:f32=$ZMMm                SHIFT:r:u8=$shl', 'E=1 MOD=REG', 'VPSLLD_nzmxub', ''];
T['VPSLLD VREG:rw:f32=$ZMMn PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_i32 EVH=$EVH SHIFT:r:u8=$shl', 'MOD=MEM    ', 'VPSLLD_nzmxub', ''];

# VPSLLVD-Shift Int32 Vector Left Logical.
T['VPSLLVD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMn VREG:r:s32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPSLLVD', ''];
T['VPSLLVD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMn VREG:r:s32=$ZMMm               ', 'E=1 MOD=REG', 'VPSLLVD', ''];
T['VPSLLVD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPSLLVD', ''];

# VPSRAD-Shift Int32 Vector Immediate Right Arithmetic.
T['VPSRAD VREG:rw:f32=$ZMMn PREG:r:Z=$Kk VREG:r:f32=$ZMMm SWZ=$r_i32     SHIFT:r:u8=$shr', 'E=0 MOD=REG', 'VPSRAD_nzmxub', ''];
T['VPSRAD VREG:rw:f32=$ZMMn PREG:r:Z=$Kk VREG:r:f32=$ZMMm                SHIFT:r:u8=$shr', 'E=1 MOD=REG', 'VPSRAD_nzmxub', ''];
T['VPSRAD VREG:rw:f32=$ZMMn PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_i32 EVH=$EVH SHIFT:r:u8=$shr', 'MOD=MEM    ', 'VPSRAD_nzmxub', ''];

# VPSRAVD-Shift Int32 Vector Right Arithmetic.
T['VPSRAVD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMn VREG:r:s32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPSRAVD_rznzmx', ''];
T['VPSRAVD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMn VREG:r:s32=$ZMMm               ', 'E=1 MOD=REG', 'VPSRAVD_rznzmx', ''];
T['VPSRAVD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPSRAVD_rznzmx', ''];

# VPSRLD-Shift Int32 Vector Immediate Right Logical.
T['VPSRLD VREG:rw:f32=$ZMMn PREG:r:Z=$Kk VREG:r:f32=$ZMMm SWZ=$r_i32     SHIFT:r:u8=$shr', 'E=0 MOD=REG', 'VPSRLD_nzmxub', ''];
T['VPSRLD VREG:rw:f32=$ZMMn PREG:r:Z=$Kk VREG:r:f32=$ZMMm                SHIFT:r:u8=$shr', 'E=1 MOD=REG', 'VPSRLD_nzmxub', ''];
T['VPSRLD VREG:rw:f32=$ZMMn PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_i32 EVH=$EVH SHIFT:r:u8=$shr', 'MOD=MEM    ', 'VPSRLD_nzmxub', ''];

# VPSRLVD-Shift Int32 Vector Right Logical.
T['VPSRLVD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMn VREG:r:s32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPSRLVD', ''];
T['VPSRLVD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMn VREG:r:s32=$ZMMm               ', 'E=1 MOD=REG', 'VPSRLVD', ''];
T['VPSRLVD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPSRLVD', ''];

# VPSUBD-Subtract Int32 Vectors.
T['VPSUBD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPSUBD_rznzmx', ''];
T['VPSUBD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VPSUBD_rznzmx', ''];
T['VPSUBD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPSUBD_rznzmx', ''];

# VPSUBRD-Reverse Subtract Int32 Vectors.
T['VPSUBRD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPSUBRD', ''];
T['VPSUBRD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VPSUBRD', ''];
T['VPSUBRD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPSUBRD', ''];

# VPSUBRSETBD-Reverse Subtract Int32 Vectors and Set Borrow.
T['VPSUBRSETBD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn VREG:r:s32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPSUBRSETBD', ''];
T['VPSUBRSETBD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn VREG:r:s32=$ZMMm               ', 'E=1 MOD=REG', 'VPSUBRSETBD', ''];
T['VPSUBRSETBD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPSUBRSETBD', ''];

# VPSUBSETBD-Subtract Int32 Vectors and Set Borrow.
T['VPSUBSETBD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn VREG:r:s32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPSUBSETBD', ''];
T['VPSUBSETBD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn VREG:r:s32=$ZMMm               ', 'E=1 MOD=REG', 'VPSUBSETBD', ''];
T['VPSUBSETBD VREG:rw:s32=$ZMMr PREG:r:Z=$Kk REG:w=$Kn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPSUBSETBD', ''];

# VPTESTMD-Logical AND Int32 Vectors and Set Vector Mask.
T['VPTESTMD REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$ZMMn VREG:r:s32=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VPTESTMD', ''];
T['VPTESTMD REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$ZMMn VREG:r:s32=$ZMMm               ', 'E=1 MOD=REG', 'VPTESTMD', ''];
T['VPTESTMD REG:w=$Kr PREG:r:Z=$Kk VREG:r:s32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPTESTMD', ''];

# VPXORD-Bitwise XOR Int32 Vectors.
T['VPXORD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VPXORD_rznzmx', ''];
T['VPXORD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VPXORD_rznzmx', ''];
T['VPXORD VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VPXORD_rznzmx', ''];

# VPXORQ-Bitwise XOR Int64 Vectors.
T['VPXORQ VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VPXORQ_rznzmx', ''];
T['VPXORQ VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm               ', 'E=1 MOD=REG', 'VPXORQ_rznzmx', ''];
T['VPXORQ VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_i64 EVH=$EVH', 'MOD=MEM    ', 'VPXORQ_rznzmx', ''];

# VRCP23PS-Reciprocal of Float32 Vector.
T['VRCP23PS VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMm                     ', 'E=0 MOD=REG', 'VRCP23PS', ''];
T['VRCP23PS VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMm SAE=$SAE            ', 'E=1 MOD=REG', 'VRCP23PS', ''];
T['VRCP23PS VREG:rw:s32=$ZMMr PREG:r:Z=$Kk MEM:r:s32=$mem512 SWZ=$n_f32 EVH=$EVH', 'MOD=MEM    ', 'VRCP23PS', ''];

# VRCP28PD-Approximation to the Reciprocal of Packed Double-Precision Floating-Point Values with Less Than 2^-28 Relative Error.
T['VRCP28PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VRCP28PD', '        '];
T['VRCP28PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VRCP28PD', '        '];
T['VRCP28PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VRCP28PD', 'tuple=FV'];

# VRCP28PS-Approximation to the Reciprocal of Packed Single-Precision Floating-Point Values with Less Than 2^-28 Relative Error.
T['VRCP28PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VRCP28PS', '        '];
T['VRCP28PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VRCP28PS', '        '];
T['VRCP28PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VRCP28PS', 'tuple=FV'];

# VRCP28SD-Approximation to the Reciprocal of Scalar Double-Precision Floating-Point Value with Less Than 2^-28 Relative Error.
T['VRCP28SD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm         ', 'BP=0 MOD=REG        ', 'VRCP28SD', '         '];
T['VRCP28SD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128', 'VRCP28SD', '         '];
T['VRCP28SD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn MEM:r:f64=$mem64         ', 'BP=0 MOD=MEM        ', 'VRCP28SD', 'tuple=T1S'];

# VRCP28SS-Approximation to the Reciprocal of Scalar Single-Precision Floating-Point Value with Less Than 2^-28 Relative Error.
T['VRCP28SS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm         ', 'BP=0 MOD=REG        ', 'VRCP28SS', '         '];
T['VRCP28SS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128', 'VRCP28SS', '         '];
T['VRCP28SS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem32         ', 'BP=0 MOD=MEM        ', 'VRCP28SS', 'tuple=T1S'];

# VRNDFXPNTPD-Round Float64 Vector.
T['VRNDFXPNTPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SWZ=$r_i64     IMM:u8=$uimm8', 'E=0 MOD=REG', 'VRNDFXPNTPD', ''];
T['VRNDFXPNTPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SAE=$SAE       IMM:u8=$uimm8', 'E=1 MOD=REG', 'VRNDFXPNTPD', ''];
T['VRNDFXPNTPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_f64 EVH=$EVH IMM:u8=$uimm8', 'MOD=MEM    ', 'VRNDFXPNTPD', ''];

# VRNDFXPNTPS-Round Float32 Vector.
T['VRNDFXPNTPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SWZ=$r_i32     IMM:u8=$uimm8', 'E=0 MOD=REG', 'VRNDFXPNTPS', ''];
T['VRNDFXPNTPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SAE=$SAE       IMM:u8=$uimm8', 'E=1 MOD=REG', 'VRNDFXPNTPS', ''];
T['VRNDFXPNTPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk MEM:r=$mem SWZ=$su_f32 EVH=$EVH IMM:u8=$uimm8', 'MOD=MEM    ', 'VRNDFXPNTPS', ''];

# VRSQRT23PS-Vector Reciprocal Square Root of Float32 Vector.
T['VRSQRT23PS VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMm                     ', 'E=0 MOD=REG', 'VRSQRT23PS', ''];
T['VRSQRT23PS VREG:rw:s32=$ZMMr PREG:r:Z=$Kk VREG:r:s32=$ZMMm SAE=$SAE            ', 'E=1 MOD=REG', 'VRSQRT23PS', ''];
T['VRSQRT23PS VREG:rw:s32=$ZMMr PREG:r:Z=$Kk MEM:r:s32=$mem512 SWZ=$n_f32 EVH=$EVH', 'MOD=MEM    ', 'VRSQRT23PS', ''];

# VRSQRT28PD-Approximation to the Reciprocal Square Root of Packed Double-Precision Floating-Point Values with Less Than 2^-28 Relative Error.
T['VRSQRT28PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VRSQRT28PD', '        '];
T['VRSQRT28PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VRSQRT28PD', '        '];
T['VRSQRT28PD VREG:w:f64=$ZMMr PREG:r:Z=$Kk MEM:r:f64=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VRSQRT28PD', 'tuple=FV'];

# VRSQRT28PS-Approximation to the Reciprocal Square Root of Packed Single-Precision Floating-Point Values with Less Than 2^-28 Relative Error.
T['VRSQRT28PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm SAE=$SAE   ', 'BP=1 MOD=REG RVL=512', 'VRSQRT28PS', '        '];
T['VRSQRT28PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMm            ', 'BP=0 MOD=REG VL=512 ', 'VRSQRT28PS', '        '];
T['VRSQRT28PS VREG:w:f32=$ZMMr PREG:r:Z=$Kk MEM:r:f32=$mem512 BCST=$BCST', 'MOD=MEM VL=512      ', 'VRSQRT28PS', 'tuple=FV'];

# VRSQRT28SD-Approximation to the Reciprocal Square Root of Scalar Double-Precision Floating-Point Value with Less Than 2^-28 Relative Error.
T['VRSQRT28SD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm         ', 'BP=0 MOD=REG        ', 'VRSQRT28SD', '         '];
T['VRSQRT28SD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn VREG:r:f64=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128', 'VRSQRT28SD', '         '];
T['VRSQRT28SD VREG:w:f64=$XMMr PREG:r:Z=$Kk VREG:r:f64=$XMMn MEM:r:f64=$mem64         ', 'BP=0 MOD=MEM        ', 'VRSQRT28SD', 'tuple=T1S'];

# VRSQRT28SS-Approximation to the Reciprocal Square Root of Scalar Single-Precision Floating- Point Value with Less Than 2^-28 Relative Error.
T['VRSQRT28SS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm         ', 'BP=0 MOD=REG        ', 'VRSQRT28SS', '         '];
T['VRSQRT28SS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn VREG:r:f32=$XMMm SAE=$SAE', 'BP=1 MOD=REG RVL=128', 'VRSQRT28SS', '         '];
T['VRSQRT28SS VREG:w:f32=$XMMr PREG:r:Z=$Kk VREG:r:f32=$XMMn MEM:r:f32=$mem32         ', 'BP=0 MOD=MEM        ', 'VRSQRT28SS', 'tuple=T1S'];

# VSCALEPS-Scale Float32 Vectors.
T['VSCALEPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VSCALEPS', ''];
T['VSCALEPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm               ', 'E=1 MOD=REG', 'VSCALEPS', ''];
T['VSCALEPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_i32 EVH=$EVH', 'MOD=MEM    ', 'VSCALEPS', ''];

# VSCATTERDPD-Scatter Float64 Vector With Signed Dword Indices.
T['VSCATTERDPD MEM:w=$vmz SBCST:SUPP={FROM=1 TO=8} PREG:rw:Z=$Kk VREG:r:f64=$ZMMr SWZ=$d_f64 EVH=$EVH', 'NONE', 'VSCATTERDPD', 'egran=1'];

# VSCATTERDPS-Scatter Float32 Vector With Signed Dword Indices.
T['VSCATTERDPS MEM:w=$vmz SBCST:SUPP={FROM=1 TO=16} PREG:rw:Z=$Kk VREG:r:f32=$ZMMr SWZ=$d_f32 EVH=$EVH', 'NONE', 'VSCATTERDPS', 'egran=1'];

# VSCATTERPF0DPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint with Intent to Write.
T['VSCATTERPF0DPD MEM:r:f64=$vmy64 PREG:rw=$Kk', 'NONE', 'VSCATTERPF0DPD', 'tuple=T1S'];

# VSCATTERPF0DPS-Scatter Prefetch Float32 Vector With Signed Dword Indices Into L1.
T['VSCATTERPF0DPS MEM:w=$vmz SWZ=$u_f32 EVH=$EVH PREG:rw:Z=$Kk', 'NONE', 'VSCATTERPF0DPS_m0', 'egran=1  '];
T['VSCATTERPF0DPS MEM:r:f32=$vmz32               PREG:rw=$Kk  ', 'NONE', 'VSCATTERPF0DPS_md', 'tuple=T1S'];

# VSCATTERPF0HINTDPD-Scatter Prefetch Float64 Vector Hint With Signed Dword Indices.
T['VSCATTERPF0HINTDPD MEM:w=$vmz SWZ=$u_f32 EVH=$EVH PREG:rw:Z=$Kk', 'NONE', 'VSCATTERPF0HINTDPD', 'egran=1'];

# VSCATTERPF0HINTDPS-Scatter Prefetch Float32 Vector Hint With Signed Dword Indices.
T['VSCATTERPF0HINTDPS MEM:w=$vmz SWZ=$u_f32 EVH=$EVH PREG:rw:Z=$Kk', 'NONE', 'VSCATTERPF0HINTDPS', 'egran=1'];

# VSCATTERPF0QPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint with Intent to Write.
T['VSCATTERPF0QPD MEM:r:f64=$vmz64 PREG:rw=$Kk', 'NONE', 'VSCATTERPF0QPD', 'tuple=T1S'];

# VSCATTERPF0QPS-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint with Intent to Write.
T['VSCATTERPF0QPS MEM:r:f32=$vmz32 PREG:rw=$Kk', 'NONE', 'VSCATTERPF0QPS', 'tuple=T1S'];

# VSCATTERPF1DPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint with Intent to Write.
T['VSCATTERPF1DPD MEM:r:f64=$vmy64 PREG:rw=$Kk', 'NONE', 'VSCATTERPF1DPD', 'tuple=T1S'];

# VSCATTERPF1DPS-Scatter Prefetch Float32 Vector With Signed Dword Indices Into L2.
T['VSCATTERPF1DPS MEM:w=$vmz SWZ=$u_f32 EVH=$EVH PREG:rw:Z=$Kk', 'NONE', 'VSCATTERPF1DPS_m0', 'egran=1  '];
T['VSCATTERPF1DPS MEM:r:f32=$vmz32               PREG:rw=$Kk  ', 'NONE', 'VSCATTERPF1DPS_md', 'tuple=T1S'];

# VSCATTERPF1QPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint with Intent to Write.
T['VSCATTERPF1QPD MEM:r:f64=$vmz64 PREG:rw=$Kk', 'NONE', 'VSCATTERPF1QPD', 'tuple=T1S'];

# VSCATTERPF1QPS-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint with Intent to Write.
T['VSCATTERPF1QPS MEM:r:f32=$vmz32 PREG:rw=$Kk', 'NONE', 'VSCATTERPF1QPS', 'tuple=T1S'];

# VSUBPD-Subtract Float64 Vectors.
T['VSUBPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VSUBPD_rznzmx', ''];
T['VSUBPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VSUBPD_rznzmx', ''];
T['VSUBPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VSUBPD_rznzmx', ''];

# VSUBPS-Subtract Float32 Vectors.
T['VSUBPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VSUBPS_rznzmx', ''];
T['VSUBPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VSUBPS_rznzmx', ''];
T['VSUBPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VSUBPS_rznzmx', ''];

# VSUBRPD-Reverse Subtract Float64 Vectors.
T['VSUBRPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm SWZ=$r_i64    ', 'E=0 MOD=REG', 'VSUBRPD', ''];
T['VSUBRPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn VREG:r:f64=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VSUBRPD', ''];
T['VSUBRPD VREG:rw:f64=$ZMMr PREG:r:Z=$Kk VREG:r:f64=$ZMMn MEM:r=$mem SWZ=$su_f64 EVH=$EVH', 'MOD=MEM    ', 'VSUBRPD', ''];

# VSUBRPS-Reverse Subtract Float32 Vectors.
T['VSUBRPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm SWZ=$r_i32    ', 'E=0 MOD=REG', 'VSUBRPS', ''];
T['VSUBRPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn VREG:r:f32=$ZMMm RC=$RC        ', 'E=1 MOD=REG', 'VSUBRPS', ''];
T['VSUBRPS VREG:rw:f32=$ZMMr PREG:r:Z=$Kk VREG:r:f32=$ZMMn MEM:r=$mem SWZ=$su_f32 EVH=$EVH', 'MOD=MEM    ', 'VSUBRPS', ''];
