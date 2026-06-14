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
T['BEXTR REG:w:sx=$GPRyr   REG:r:sx=$GPRym   IMM:s32=$simm32', 'MOD=REG MODE=64  ', 'BEXTR_rxmxud', ''];
T['BEXTR REG:w:sx=$GPRyr   MEM:r:sx=$memy    IMM:s32=$simm32', 'MOD=MEM MODE=64  ', 'BEXTR_rxmxud', ''];
T['BEXTR REG:w:s32=$GPR32r REG:r:s32=$GPR32m IMM:s32=$simm32', 'MOD=REG MODE=NO64', 'BEXTR_rxmxud', ''];
T['BEXTR REG:w:s32=$GPR32r MEM:r:s32=$mem32  IMM:s32=$simm32', 'MOD=MEM MODE=NO64', 'BEXTR_rxmxud', ''];

# BLCFILL.
T['BLCFILL REG:w:sx=$GPRyn   REG:r:sx=$GPRvm  ', 'MOD=REG MODE=64  ', 'BLCFILL', ''];
T['BLCFILL REG:w:sx=$GPRyn   MEM:r:sx=$memy   ', 'MOD=MEM MODE=64  ', 'BLCFILL', ''];
T['BLCFILL REG:w:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=NO64', 'BLCFILL', ''];
T['BLCFILL REG:w:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=NO64', 'BLCFILL', ''];

# BLCI.
T['BLCI REG:w:sx=$GPRyn   REG:r:sx=$GPRvm ', 'MOD=REG MODE=64  ', 'BLCI', ''];
T['BLCI REG:w:sx=$GPRyn   MEM:r:sx=$memy  ', 'MOD=MEM MODE=64  ', 'BLCI', ''];
T['BLCI REG:w:s32=$GPR32n REG:r:s32=$GPRvm', 'MOD=REG MODE=NO64', 'BLCI', ''];
T['BLCI REG:w:s32=$GPR32n MEM:r:s32=$mem32', 'MOD=MEM MODE=NO64', 'BLCI', ''];

# BLCIC.
T['BLCIC REG:w:sx=$GPRyn   REG:r:sx=$GPRvm  ', 'MOD=REG MODE=64  ', 'BLCIC', ''];
T['BLCIC REG:w:sx=$GPRyn   MEM:r:sx=$memy   ', 'MOD=MEM MODE=64  ', 'BLCIC', ''];
T['BLCIC REG:w:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=NO64', 'BLCIC', ''];
T['BLCIC REG:w:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=NO64', 'BLCIC', ''];

# BLCMSK.
T['BLCMSK REG:w:sx=$GPRyn   REG:r:sx=$GPRvm ', 'MOD=REG MODE=64  ', 'BLCMSK', ''];
T['BLCMSK REG:w:sx=$GPRyn   MEM:r:sx=$memy  ', 'MOD=MEM MODE=64  ', 'BLCMSK', ''];
T['BLCMSK REG:w:s32=$GPR32n REG:r:s32=$GPRvm', 'MOD=REG MODE=NO64', 'BLCMSK', ''];
T['BLCMSK REG:w:s32=$GPR32n MEM:r:s32=$mem32', 'MOD=MEM MODE=NO64', 'BLCMSK', ''];

# BLCS.
T['BLCS REG:w:sx=$GPRyn   REG:r:sx=$GPRvm  ', 'MOD=REG MODE=64  ', 'BLCS', ''];
T['BLCS REG:w:sx=$GPRyn   MEM:r:sx=$memy   ', 'MOD=MEM MODE=64  ', 'BLCS', ''];
T['BLCS REG:w:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=NO64', 'BLCS', ''];
T['BLCS REG:w:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=NO64', 'BLCS', ''];

# BLSFILL.
T['BLSFILL REG:w:sx=$GPRyn   REG:r:sx=$GPRvm  ', 'MOD=REG MODE=64  ', 'BLSFILL', ''];
T['BLSFILL REG:w:sx=$GPRyn   MEM:r:sx=$memy   ', 'MOD=MEM MODE=64  ', 'BLSFILL', ''];
T['BLSFILL REG:w:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=NO64', 'BLSFILL', ''];
T['BLSFILL REG:w:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=NO64', 'BLSFILL', ''];

# BLSIC.
T['BLSIC REG:w:sx=$GPRyn   REG:r:sx=$GPRvm  ', 'MOD=REG MODE=64  ', 'BLSIC', ''];
T['BLSIC REG:w:sx=$GPRyn   MEM:r:sx=$memy   ', 'MOD=MEM MODE=64  ', 'BLSIC', ''];
T['BLSIC REG:w:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=NO64', 'BLSIC', ''];
T['BLSIC REG:w:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=NO64', 'BLSIC', ''];

# CLZERO.
T['CLZERO REG:r=$AXv', 'NONE', 'CLZERO', ''];

# EXTRQ.
T['EXTRQ REG:w:s64=$XMMm IMM:u8=$uimm8    IMM:u8=$uimm8', 'NONE', 'EXTRQ_mqub', ''];
T['EXTRQ REG:w:s64=$XMMr VREG:r:s32=$XMMm              ', 'NONE', 'EXTRQ_rqmo', ''];

# INSERTQ.
T['INSERTQ REG:w:s64=$XMMr VREG:r:s32=$XMMm                            ', 'NONE', 'INSERTQ_rqmo  ', ''];
T['INSERTQ REG:w:s64=$XMMr REG:r:s64=$XMMm  IMM:u8=$index IMM:u8=$index', 'NONE', 'INSERTQ_rqmqub', ''];

# LLWPCB-Load LWPCB Address.
T['LLWPCB REG:w:sx=$GPRym', 'NONE', 'LLWPCB', ''];

# LWPINS-Insert User Event Record in LWP Ring Buffer.
T['LWPINS REG:w:sx=$GPRyn REG:r:s32=$GPR32m IMM:s32=$simm32', 'MOD=REG', 'LWPINS', ''];
T['LWPINS REG:w:sx=$GPRyn MEM:r:s32=$mem32  IMM:s32=$simm32', 'MOD=MEM', 'LWPINS', ''];

# LWPVAL-Insert Value Sample in LWP Ring Buffer.
T['LWPVAL REG:w:sx=$GPRyn REG:r:s32=$GPR32m IMM:s32=$simm32', 'MOD=REG', 'LWPVAL', ''];
T['LWPVAL REG:w:sx=$GPRyn MEM:r:s32=$mem32  IMM:s32=$simm32', 'MOD=MEM', 'LWPVAL', ''];

# SLWPCB-Store LWPCB Address.
T['SLWPCB REG:w:sx=$GPRym', 'NONE', 'SLWPCB', ''];

# MONITORX.
T['MONITORX REG:SUPP:r=$AXa REG:SUPP:r=RCX REG:SUPP:r=RDX', 'MODE=64  ', 'MONITORX', ''];
T['MONITORX REG:SUPP:r=$AXa REG:SUPP:r=ECX REG:SUPP:r=EDX', 'MODE=NO64', 'MONITORX', ''];

# MOVNTSD.
T['MOVNTSD MEM:w:s64=$mem64 REG:r:s64=$XMMr', 'NONE', 'MOVNTSD', ''];

# MOVNTSS.
T['MOVNTSS MEM:w:s32=$mem32 REG:r:s32=$XMMr', 'NONE', 'MOVNTSS', ''];

# MWAITX.
T['MWAITX REG:SUPP:r=RAX REG:SUPP:r=RCX', 'MODE=64  ', 'MWAITX', ''];
T['MWAITX REG:SUPP:r=EAX REG:SUPP:r=ECX', 'MODE=NO64', 'MWAITX', ''];

# RDPRU.
T['RDPRU REG:SUPP:w:s32=EDX REG:SUPP:w:s32=EAX REG:SUPP:r:s32=ECX', 'NONE', 'RDPRU', ''];

# CLGI-Clear Global Interrupt Flag.
T['CLGI', 'NONE', 'CLGI', ''];

# INVLPGA-Invalidate TLB Entry in a Specified ASID.
T['INVLPGA REG:r=$AXv REG:r=ECX', 'NONE', 'INVLPGA', ''];

# SKINIT-Secure Init and Jump with Attestation.
T['SKINIT REG:r=EAX', 'NONE', 'SKINIT', ''];

# STGI-Set Global Interrupt Flag.
T['STGI', 'NONE', 'STGI', ''];

# VMLOAD-Load State from VMCB.
T['VMLOAD REG:r=$AXv', 'NONE', 'VMLOAD', ''];

# VMMCALL-Call VMM.
T['VMMCALL', 'NONE', 'VMMCALL', ''];

# VMRUN-Run Virtual Machine.
T['VMRUN REG:r=$AXv', 'NONE', 'VMRUN', ''];

# VMSAVE-Save State to VMCB.
T['VMSAVE', 'NONE', 'VMSAVE', ''];

# SYSCALL-Fast System Call.
T['SYSCALL REG:SUPP:w=$IPa REG:SUPP:w=RCX REG:SUPP:w=R11 REG:SUPP:w=CS REG:SUPP:w=SS', 'NONE', 'SYSCALL_2', ''];

# SYSRET-Return From Fast System Call.
T['SYSRET REG:SUPP:w=EIP REG:SUPP:r=RCX REG:SUPP:r=R11 REG:SUPP:w=CS REG:SUPP:w=SS', 'NONE', 'SYSRET_2', ''];

# T1MSKC.
T['T1MSKC REG:w:sx=$GPRyn   REG:r:sx=$GPRvm ', 'MOD=REG MODE=64  ', 'T1MSKC', ''];
T['T1MSKC REG:w:sx=$GPRyn   MEM:r:sx=$memy  ', 'MOD=MEM MODE=64  ', 'T1MSKC', ''];
T['T1MSKC REG:w:s32=$GPR32n REG:r:s32=$GPRvm', 'MOD=REG MODE=NO64', 'T1MSKC', ''];
T['T1MSKC REG:w:s32=$GPR32n MEM:r:s32=$mem32', 'MOD=MEM MODE=NO64', 'T1MSKC', ''];

# TZMSK.
T['TZMSK REG:w:sx=$GPRyn   REG:r:sx=$GPRvm  ', 'MOD=REG MODE=64  ', 'TZMSK', ''];
T['TZMSK REG:w:sx=$GPRyn   MEM:r:sx=$memy   ', 'MOD=MEM MODE=64  ', 'TZMSK', ''];
T['TZMSK REG:w:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=NO64', 'TZMSK', ''];
T['TZMSK REG:w:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=NO64', 'TZMSK', ''];

# VFMADDPD-Multiply and Add Packed Double-Precision Floating-Point.
T['VFMADDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 VREG:r:f64=$XMMm  ', 'MOD=REG VL=128', 'VFMADDPD_rxnxis4xmx', ''];
T['VFMADDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMis4 VREG:r:f64=$YMMm  ', 'MOD=REG VL=256', 'VFMADDPD_rxnxis4xmx', ''];
T['VFMADDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 MEM:r:f64=$mem128 ', 'MOD=MEM VL=128', 'VFMADDPD_rxnxis4xmx', ''];
T['VFMADDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMis4 MEM:r:f64=$mem256 ', 'MOD=MEM VL=256', 'VFMADDPD_rxnxis4xmx', ''];
T['VFMADDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm   VREG:r:f64=$XMMis4', 'MOD=REG VL=128', 'VFMADDPD_rxnxmxis4x', ''];
T['VFMADDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm   VREG:r:f64=$YMMis4', 'MOD=REG VL=256', 'VFMADDPD_rxnxmxis4x', ''];
T['VFMADDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128  VREG:r:f64=$XMMis4', 'MOD=MEM VL=128', 'VFMADDPD_rxnxmxis4x', ''];
T['VFMADDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256  VREG:r:f64=$YMMis4', 'MOD=MEM VL=256', 'VFMADDPD_rxnxmxis4x', ''];

# VFMADDPS-Multiply and Add Packed Single-Precision Floating-Point.
T['VFMADDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 VREG:r:f32=$XMMm  ', 'MOD=REG VL=128', 'VFMADDPS_rxnxis4xmx', ''];
T['VFMADDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMis4 VREG:r:f32=$YMMm  ', 'MOD=REG VL=256', 'VFMADDPS_rxnxis4xmx', ''];
T['VFMADDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 MEM:r:f32=$mem128 ', 'MOD=MEM VL=128', 'VFMADDPS_rxnxis4xmx', ''];
T['VFMADDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMis4 MEM:r:f32=$mem256 ', 'MOD=MEM VL=256', 'VFMADDPS_rxnxis4xmx', ''];
T['VFMADDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm   VREG:r:f32=$XMMis4', 'MOD=REG VL=128', 'VFMADDPS_rxnxmxis4x', ''];
T['VFMADDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm   VREG:r:f32=$YMMis4', 'MOD=REG VL=256', 'VFMADDPS_rxnxmxis4x', ''];
T['VFMADDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128  VREG:r:f32=$XMMis4', 'MOD=MEM VL=128', 'VFMADDPS_rxnxmxis4x', ''];
T['VFMADDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256  VREG:r:f32=$YMMis4', 'MOD=MEM VL=256', 'VFMADDPS_rxnxmxis4x', ''];

# VFMADDSD-Multiply and Add Scalar Double-Precision Floating-Point.
T['VFMADDSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 REG:r:f64=$XMMm   ', 'MOD=REG', 'VFMADDSD_ronois4omq', ''];
T['VFMADDSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 MEM:r:f64=$mem64  ', 'MOD=MEM', 'VFMADDSD_ronois4omq', ''];
T['VFMADDSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn REG:r:f64=$XMMm    VREG:r:f64=$XMMis4', 'MOD=REG', 'VFMADDSD_ronomqis4o', ''];
T['VFMADDSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem64   VREG:r:f64=$XMMis4', 'MOD=MEM', 'VFMADDSD_ronomqis4o', ''];

# VFMADDSS-Multiply and Add Scalar Single-Precision Floating-Point.
T['VFMADDSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 REG:r:f32=$XMMm   ', 'MOD=REG', 'VFMADDSS_ronois4omd', ''];
T['VFMADDSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 MEM:r:f32=$mem32  ', 'MOD=MEM', 'VFMADDSS_ronois4omd', ''];
T['VFMADDSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn REG:r:f32=$XMMm    VREG:r:f32=$XMMis4', 'MOD=REG', 'VFMADDSS_ronomdis4o', ''];
T['VFMADDSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem32   VREG:r:f32=$XMMis4', 'MOD=MEM', 'VFMADDSS_ronomdis4o', ''];

# VFMADDSUBPD-Multiply with Alternating Add/Subtract of Packed Double-Precision Floating-Point.
T['VFMADDSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 VREG:r:f64=$XMMm  ', 'MOD=REG VL=128', 'VFMADDSUBPD_rxnxis4xmx', ''];
T['VFMADDSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMis4 VREG:r:f64=$YMMm  ', 'MOD=REG VL=256', 'VFMADDSUBPD_rxnxis4xmx', ''];
T['VFMADDSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 MEM:r:f64=$mem128 ', 'MOD=MEM VL=128', 'VFMADDSUBPD_rxnxis4xmx', ''];
T['VFMADDSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMis4 MEM:r:f64=$mem256 ', 'MOD=MEM VL=256', 'VFMADDSUBPD_rxnxis4xmx', ''];
T['VFMADDSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm   VREG:r:f64=$XMMis4', 'MOD=REG VL=128', 'VFMADDSUBPD_rxnxmxis4x', ''];
T['VFMADDSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm   VREG:r:f64=$YMMis4', 'MOD=REG VL=256', 'VFMADDSUBPD_rxnxmxis4x', ''];
T['VFMADDSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128  VREG:r:f64=$XMMis4', 'MOD=MEM VL=128', 'VFMADDSUBPD_rxnxmxis4x', ''];
T['VFMADDSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256  VREG:r:f64=$YMMis4', 'MOD=MEM VL=256', 'VFMADDSUBPD_rxnxmxis4x', ''];

# VFMADDSUBPS-Multiply with Alternating Add/Subtract of Packed Single-Precision Floating-Point.
T['VFMADDSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 VREG:r:f32=$XMMm  ', 'MOD=REG VL=128', 'VFMADDSUBPS_rxnxis4xmx', ''];
T['VFMADDSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMis4 VREG:r:f32=$YMMm  ', 'MOD=REG VL=256', 'VFMADDSUBPS_rxnxis4xmx', ''];
T['VFMADDSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 MEM:r:f32=$mem128 ', 'MOD=MEM VL=128', 'VFMADDSUBPS_rxnxis4xmx', ''];
T['VFMADDSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMis4 MEM:r:f32=$mem256 ', 'MOD=MEM VL=256', 'VFMADDSUBPS_rxnxis4xmx', ''];
T['VFMADDSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm   VREG:r:f32=$XMMis4', 'MOD=REG VL=128', 'VFMADDSUBPS_rxnxmxis4x', ''];
T['VFMADDSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm   VREG:r:f32=$YMMis4', 'MOD=REG VL=256', 'VFMADDSUBPS_rxnxmxis4x', ''];
T['VFMADDSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128  VREG:r:f32=$XMMis4', 'MOD=MEM VL=128', 'VFMADDSUBPS_rxnxmxis4x', ''];
T['VFMADDSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256  VREG:r:f32=$YMMis4', 'MOD=MEM VL=256', 'VFMADDSUBPS_rxnxmxis4x', ''];

# VFMSUBADDPD-Multiply with Alternating Subtract/Add of Packed Double-Precision Floating-Point.
T['VFMSUBADDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 VREG:r:f64=$XMMm  ', 'MOD=REG VL=128', 'VFMSUBADDPD_rxnxis4xmx', ''];
T['VFMSUBADDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMis4 VREG:r:f64=$YMMm  ', 'MOD=REG VL=256', 'VFMSUBADDPD_rxnxis4xmx', ''];
T['VFMSUBADDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 MEM:r:f64=$mem128 ', 'MOD=MEM VL=128', 'VFMSUBADDPD_rxnxis4xmx', ''];
T['VFMSUBADDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMis4 MEM:r:f64=$mem256 ', 'MOD=MEM VL=256', 'VFMSUBADDPD_rxnxis4xmx', ''];
T['VFMSUBADDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm   VREG:r:f64=$XMMis4', 'MOD=REG VL=128', 'VFMSUBADDPD_rxnxmxis4x', ''];
T['VFMSUBADDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm   VREG:r:f64=$YMMis4', 'MOD=REG VL=256', 'VFMSUBADDPD_rxnxmxis4x', ''];
T['VFMSUBADDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128  VREG:r:f64=$XMMis4', 'MOD=MEM VL=128', 'VFMSUBADDPD_rxnxmxis4x', ''];
T['VFMSUBADDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256  VREG:r:f64=$YMMis4', 'MOD=MEM VL=256', 'VFMSUBADDPD_rxnxmxis4x', ''];

# VFMSUBADDPS-Multiply with Alternating Subtract/Add of Packed Single-Precision Floating-Point.
T['VFMSUBADDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 VREG:r:f32=$XMMm  ', 'MOD=REG VL=128', 'VFMSUBADDPS_rxnxis4xmx', ''];
T['VFMSUBADDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMis4 VREG:r:f32=$YMMm  ', 'MOD=REG VL=256', 'VFMSUBADDPS_rxnxis4xmx', ''];
T['VFMSUBADDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 MEM:r:f32=$mem128 ', 'MOD=MEM VL=128', 'VFMSUBADDPS_rxnxis4xmx', ''];
T['VFMSUBADDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMis4 MEM:r:f32=$mem256 ', 'MOD=MEM VL=256', 'VFMSUBADDPS_rxnxis4xmx', ''];
T['VFMSUBADDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm   VREG:r:f32=$XMMis4', 'MOD=REG VL=128', 'VFMSUBADDPS_rxnxmxis4x', ''];
T['VFMSUBADDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm   VREG:r:f32=$YMMis4', 'MOD=REG VL=256', 'VFMSUBADDPS_rxnxmxis4x', ''];
T['VFMSUBADDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128  VREG:r:f32=$XMMis4', 'MOD=MEM VL=128', 'VFMSUBADDPS_rxnxmxis4x', ''];
T['VFMSUBADDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256  VREG:r:f32=$YMMis4', 'MOD=MEM VL=256', 'VFMSUBADDPS_rxnxmxis4x', ''];

# VFMSUBPD-Multiply and Subtract Packed Double-Precision Floating-Point.
T['VFMSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 VREG:r:f64=$XMMm  ', 'MOD=REG VL=128', 'VFMSUBPD_rxnxis4xmx', ''];
T['VFMSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMis4 VREG:r:f64=$YMMm  ', 'MOD=REG VL=256', 'VFMSUBPD_rxnxis4xmx', ''];
T['VFMSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 MEM:r:f64=$mem128 ', 'MOD=MEM VL=128', 'VFMSUBPD_rxnxis4xmx', ''];
T['VFMSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMis4 MEM:r:f64=$mem256 ', 'MOD=MEM VL=256', 'VFMSUBPD_rxnxis4xmx', ''];
T['VFMSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm   VREG:r:f64=$XMMis4', 'MOD=REG VL=128', 'VFMSUBPD_rxnxmxis4x', ''];
T['VFMSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm   VREG:r:f64=$YMMis4', 'MOD=REG VL=256', 'VFMSUBPD_rxnxmxis4x', ''];
T['VFMSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128  VREG:r:f64=$XMMis4', 'MOD=MEM VL=128', 'VFMSUBPD_rxnxmxis4x', ''];
T['VFMSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256  VREG:r:f64=$YMMis4', 'MOD=MEM VL=256', 'VFMSUBPD_rxnxmxis4x', ''];

# VFMSUBPS-Multiply and Subtract Packed Single-Precision Floating-Point.
T['VFMSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 VREG:r:f32=$XMMm  ', 'MOD=REG VL=128', 'VFMSUBPS_rxnxis4xmx', ''];
T['VFMSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMis4 VREG:r:f32=$YMMm  ', 'MOD=REG VL=256', 'VFMSUBPS_rxnxis4xmx', ''];
T['VFMSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 MEM:r:f32=$mem128 ', 'MOD=MEM VL=128', 'VFMSUBPS_rxnxis4xmx', ''];
T['VFMSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMis4 MEM:r:f32=$mem256 ', 'MOD=MEM VL=256', 'VFMSUBPS_rxnxis4xmx', ''];
T['VFMSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm   VREG:r:f32=$XMMis4', 'MOD=REG VL=128', 'VFMSUBPS_rxnxmxis4x', ''];
T['VFMSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm   VREG:r:f32=$YMMis4', 'MOD=REG VL=256', 'VFMSUBPS_rxnxmxis4x', ''];
T['VFMSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128  VREG:r:f32=$XMMis4', 'MOD=MEM VL=128', 'VFMSUBPS_rxnxmxis4x', ''];
T['VFMSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256  VREG:r:f32=$YMMis4', 'MOD=MEM VL=256', 'VFMSUBPS_rxnxmxis4x', ''];

# VFMSUBSD-Multiply and Subtract Scalar Double-Precision Floating-Point.
T['VFMSUBSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 REG:r:f64=$XMMm   ', 'MOD=REG', 'VFMSUBSD_ronois4omq', ''];
T['VFMSUBSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 MEM:r:f64=$mem64  ', 'MOD=MEM', 'VFMSUBSD_ronois4omq', ''];
T['VFMSUBSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn REG:r:f64=$XMMm    VREG:r:f64=$XMMis4', 'MOD=REG', 'VFMSUBSD_ronomqis4o', ''];
T['VFMSUBSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem64   VREG:r:f64=$XMMis4', 'MOD=MEM', 'VFMSUBSD_ronomqis4o', ''];

# VFMSUBSS-Multiply and Subtract Scalar Single-Precision Floating-Point.
T['VFMSUBSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 REG:r:f32=$XMMm   ', 'MOD=REG', 'VFMSUBSS_ronois4omd', ''];
T['VFMSUBSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 MEM:r:f32=$mem32  ', 'MOD=MEM', 'VFMSUBSS_ronois4omd', ''];
T['VFMSUBSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn REG:r:f32=$XMMm    VREG:r:f32=$XMMis4', 'MOD=REG', 'VFMSUBSS_ronomdis4o', ''];
T['VFMSUBSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem32   VREG:r:f32=$XMMis4', 'MOD=MEM', 'VFMSUBSS_ronomdis4o', ''];

# VFNMADDPD-Negative Multiply and Add Packed Double-Precision Floating-Point.
T['VFNMADDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 VREG:r:f64=$XMMm  ', 'MOD=REG VL=128', 'VFNMADDPD_rxnxis4xmx', ''];
T['VFNMADDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMis4 VREG:r:f64=$YMMm  ', 'MOD=REG VL=256', 'VFNMADDPD_rxnxis4xmx', ''];
T['VFNMADDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 MEM:r:f64=$mem128 ', 'MOD=MEM VL=128', 'VFNMADDPD_rxnxis4xmx', ''];
T['VFNMADDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMis4 MEM:r:f64=$mem256 ', 'MOD=MEM VL=256', 'VFNMADDPD_rxnxis4xmx', ''];
T['VFNMADDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm   VREG:r:f64=$XMMis4', 'MOD=REG VL=128', 'VFNMADDPD_rxnxmxis4x', ''];
T['VFNMADDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm   VREG:r:f64=$YMMis4', 'MOD=REG VL=256', 'VFNMADDPD_rxnxmxis4x', ''];
T['VFNMADDPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128  VREG:r:f64=$XMMis4', 'MOD=MEM VL=128', 'VFNMADDPD_rxnxmxis4x', ''];
T['VFNMADDPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256  VREG:r:f64=$YMMis4', 'MOD=MEM VL=256', 'VFNMADDPD_rxnxmxis4x', ''];

# VFNMADDPS-Negative Multiply and Add Packed Single-Precision Floating-Point.
T['VFNMADDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 VREG:r:f32=$XMMm  ', 'MOD=REG VL=128', 'VFNMADDPS_rxnxis4xmx', ''];
T['VFNMADDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMis4 VREG:r:f32=$YMMm  ', 'MOD=REG VL=256', 'VFNMADDPS_rxnxis4xmx', ''];
T['VFNMADDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 MEM:r:f32=$mem128 ', 'MOD=MEM VL=128', 'VFNMADDPS_rxnxis4xmx', ''];
T['VFNMADDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMis4 MEM:r:f32=$mem256 ', 'MOD=MEM VL=256', 'VFNMADDPS_rxnxis4xmx', ''];
T['VFNMADDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm   VREG:r:f32=$XMMis4', 'MOD=REG VL=128', 'VFNMADDPS_rxnxmxis4x', ''];
T['VFNMADDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm   VREG:r:f32=$YMMis4', 'MOD=REG VL=256', 'VFNMADDPS_rxnxmxis4x', ''];
T['VFNMADDPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128  VREG:r:f32=$XMMis4', 'MOD=MEM VL=128', 'VFNMADDPS_rxnxmxis4x', ''];
T['VFNMADDPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256  VREG:r:f32=$YMMis4', 'MOD=MEM VL=256', 'VFNMADDPS_rxnxmxis4x', ''];

# VFNMADDSD-Negative Multiply and Add Scalar Double-Precision Floating-Point.
T['VFNMADDSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 REG:r:f64=$XMMm   ', 'MOD=REG', 'VFNMADDSD_ronois4omq', ''];
T['VFNMADDSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 MEM:r:f64=$mem64  ', 'MOD=MEM', 'VFNMADDSD_ronois4omq', ''];
T['VFNMADDSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn REG:r:f64=$XMMm    VREG:r:f64=$XMMis4', 'MOD=REG', 'VFNMADDSD_ronomqis4o', ''];
T['VFNMADDSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem64   VREG:r:f64=$XMMis4', 'MOD=MEM', 'VFNMADDSD_ronomqis4o', ''];

# VFNMADDSS-Negative Multiply and Add Scalar Single-Precision Floating-Point.
T['VFNMADDSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 REG:r:f32=$XMMm   ', 'MOD=REG', 'VFNMADDSS_ronois4omd', ''];
T['VFNMADDSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 MEM:r:f32=$mem32  ', 'MOD=MEM', 'VFNMADDSS_ronois4omd', ''];
T['VFNMADDSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn REG:r:f32=$XMMm    VREG:r:f32=$XMMis4', 'MOD=REG', 'VFNMADDSS_ronomdis4o', ''];
T['VFNMADDSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem32   VREG:r:f32=$XMMis4', 'MOD=MEM', 'VFNMADDSS_ronomdis4o', ''];

# VFNMSUBPD-Negative Multiply and Subtract Packed Double-Precision Floating-Point.
T['VFNMSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 VREG:r:f64=$XMMm  ', 'MOD=REG VL=128', 'VFNMSUBPD_rxnxis4xmx', ''];
T['VFNMSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMis4 VREG:r:f64=$YMMm  ', 'MOD=REG VL=256', 'VFNMSUBPD_rxnxis4xmx', ''];
T['VFNMSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 MEM:r:f64=$mem128 ', 'MOD=MEM VL=128', 'VFNMSUBPD_rxnxis4xmx', ''];
T['VFNMSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMis4 MEM:r:f64=$mem256 ', 'MOD=MEM VL=256', 'VFNMSUBPD_rxnxis4xmx', ''];
T['VFNMSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm   VREG:r:f64=$XMMis4', 'MOD=REG VL=128', 'VFNMSUBPD_rxnxmxis4x', ''];
T['VFNMSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm   VREG:r:f64=$YMMis4', 'MOD=REG VL=256', 'VFNMSUBPD_rxnxmxis4x', ''];
T['VFNMSUBPD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128  VREG:r:f64=$XMMis4', 'MOD=MEM VL=128', 'VFNMSUBPD_rxnxmxis4x', ''];
T['VFNMSUBPD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256  VREG:r:f64=$YMMis4', 'MOD=MEM VL=256', 'VFNMSUBPD_rxnxmxis4x', ''];

# VFNMSUBPS-Negative Multiply and Subtract Packed Single-Precision Floating-Point.
T['VFNMSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 VREG:r:f32=$XMMm  ', 'MOD=REG VL=128', 'VFNMSUBPS_rxnxis4xmx', ''];
T['VFNMSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMis4 VREG:r:f32=$YMMm  ', 'MOD=REG VL=256', 'VFNMSUBPS_rxnxis4xmx', ''];
T['VFNMSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 MEM:r:f32=$mem128 ', 'MOD=MEM VL=128', 'VFNMSUBPS_rxnxis4xmx', ''];
T['VFNMSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMis4 MEM:r:f32=$mem256 ', 'MOD=MEM VL=256', 'VFNMSUBPS_rxnxis4xmx', ''];
T['VFNMSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm   VREG:r:f32=$XMMis4', 'MOD=REG VL=128', 'VFNMSUBPS_rxnxmxis4x', ''];
T['VFNMSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm   VREG:r:f32=$YMMis4', 'MOD=REG VL=256', 'VFNMSUBPS_rxnxmxis4x', ''];
T['VFNMSUBPS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128  VREG:r:f32=$XMMis4', 'MOD=MEM VL=128', 'VFNMSUBPS_rxnxmxis4x', ''];
T['VFNMSUBPS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256  VREG:r:f32=$YMMis4', 'MOD=MEM VL=256', 'VFNMSUBPS_rxnxmxis4x', ''];

# VFNMSUBSD-Negative Multiply and Subtract Scalar Double-Precision Floating-Point.
T['VFNMSUBSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 REG:r:f64=$XMMm   ', 'MOD=REG', 'VFNMSUBSD_ronois4omq', ''];
T['VFNMSUBSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 MEM:r:f64=$mem64  ', 'MOD=MEM', 'VFNMSUBSD_ronois4omq', ''];
T['VFNMSUBSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn REG:r:f64=$XMMm    VREG:r:f64=$XMMis4', 'MOD=REG', 'VFNMSUBSD_ronomqis4o', ''];
T['VFNMSUBSD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem64   VREG:r:f64=$XMMis4', 'MOD=MEM', 'VFNMSUBSD_ronomqis4o', ''];

# VFNMSUBSS-Negative Multiply and Subtract Scalar Single-Precision Floating-Point.
T['VFNMSUBSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 REG:r:f32=$XMMm   ', 'MOD=REG', 'VFNMSUBSS_ronois4omd', ''];
T['VFNMSUBSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 MEM:r:f32=$mem32  ', 'MOD=MEM', 'VFNMSUBSS_ronois4omd', ''];
T['VFNMSUBSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn REG:r:f32=$XMMm    VREG:r:f32=$XMMis4', 'MOD=REG', 'VFNMSUBSS_ronomdis4o', ''];
T['VFNMSUBSS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem32   VREG:r:f32=$XMMis4', 'MOD=MEM', 'VFNMSUBSS_ronomdis4o', ''];

# VPERMIL2PD-Permute Two-Source Double-Precision Floating-Point Values.
T['VPERMIL2PD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 VREG:r:f64=$XMMm   CTL:r:u8=$ctl', 'MOD=REG VL=128', 'VPERMIL2PD_rxnxis4xmxis4', ''];
T['VPERMIL2PD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMis4 VREG:r:f64=$YMMm   CTL:r:u8=$ctl', 'MOD=REG VL=256', 'VPERMIL2PD_rxnxis4xmxis4', ''];
T['VPERMIL2PD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMis4 MEM:r:f64=$mem128  CTL:r:u8=$ctl', 'MOD=MEM VL=128', 'VPERMIL2PD_rxnxis4xmxis4', ''];
T['VPERMIL2PD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMis4 MEM:r:f64=$mem256  CTL:r:u8=$ctl', 'MOD=MEM VL=256', 'VPERMIL2PD_rxnxis4xmxis4', ''];
T['VPERMIL2PD VREG:w:f64=$XMMr VREG:r:f64=$XMMn VREG:r:f64=$XMMm   VREG:r:f64=$XMMis4 CTL:r:u8=$ctl', 'MOD=REG VL=128', 'VPERMIL2PD_rxnxmxis4xis4', ''];
T['VPERMIL2PD VREG:w:f64=$YMMr VREG:r:f64=$YMMn VREG:r:f64=$YMMm   VREG:r:f64=$YMMis4 CTL:r:u8=$ctl', 'MOD=REG VL=256', 'VPERMIL2PD_rxnxmxis4xis4', ''];
T['VPERMIL2PD VREG:w:f64=$XMMr VREG:r:f64=$XMMn MEM:r:f64=$mem128  VREG:r:f64=$XMMis4 CTL:r:u8=$ctl', 'MOD=MEM VL=128', 'VPERMIL2PD_rxnxmxis4xis4', ''];
T['VPERMIL2PD VREG:w:f64=$YMMr VREG:r:f64=$YMMn MEM:r:f64=$mem256  VREG:r:f64=$YMMis4 CTL:r:u8=$ctl', 'MOD=MEM VL=256', 'VPERMIL2PD_rxnxmxis4xis4', ''];

# VPHADDUWD.
T['VPHADDUWD VREG:w:u32=$XMMr VREG:r:u16=$XMMm ', 'MOD=REG', 'VPHADDUWD', ''];
T['VPHADDUWD VREG:w:u32=$XMMr MEM:r:u16=$mem128', 'MOD=MEM', 'VPHADDUWD', ''];

# VFRCZPD-Extract Fraction Packed Double-Precision Floating-Point.
T['VFRCZPD VREG:w:f64=$XMMr VREG:r:f64=$XMMm ', 'MOD=REG VL=128', 'VFRCZPD', ''];
T['VFRCZPD VREG:w:f64=$YMMr VREG:r:f64=$YMMm ', 'MOD=REG VL=256', 'VFRCZPD', ''];
T['VFRCZPD VREG:w:f64=$XMMr MEM:r:f64=$mem128', 'MOD=MEM VL=128', 'VFRCZPD', ''];
T['VFRCZPD VREG:w:f64=$YMMr MEM:r:f64=$mem256', 'MOD=MEM VL=256', 'VFRCZPD', ''];

# VFRCZPS-Extract Fraction Packed Single-Precision Floating-Point.
T['VFRCZPS VREG:w:f32=$XMMr VREG:r:f32=$XMMm ', 'MOD=REG VL=128', 'VFRCZPS', ''];
T['VFRCZPS VREG:w:f32=$YMMr VREG:r:f32=$YMMm ', 'MOD=REG VL=256', 'VFRCZPS', ''];
T['VFRCZPS VREG:w:f32=$XMMr MEM:r:f32=$mem128', 'MOD=MEM VL=128', 'VFRCZPS', ''];
T['VFRCZPS VREG:w:f32=$YMMr MEM:r:f32=$mem256', 'MOD=MEM VL=256', 'VFRCZPS', ''];

# VFRCZSD-Extract Fraction Scalar Double-Precision Floating-Point.
T['VFRCZSD VREG:w:f64=$XMMr REG:r:f64=$XMMm ', 'MOD=REG', 'VFRCZSD', ''];
T['VFRCZSD VREG:w:f64=$XMMr MEM:r:f64=$mem64', 'MOD=MEM', 'VFRCZSD', ''];

# VFRCZSS-Extract Fraction Scalar Single-Precision Floating Point.
T['VFRCZSS VREG:w:f32=$XMMr REG:r:f32=$XMMm ', 'MOD=REG', 'VFRCZSS', ''];
T['VFRCZSS VREG:w:f32=$XMMr MEM:r:f32=$mem32', 'MOD=MEM', 'VFRCZSS', ''];

# VPCMOV-Vector Conditional Moves.
T['VPCMOV VREG:w=$XMMr VREG:r=$XMMn VREG:r=$XMMis4 VREG:r=$XMMm  ', 'MOD=REG VL=128', 'VPCMOV_rxnxis4xmx', ''];
T['VPCMOV VREG:w=$YMMr VREG:r=$YMMn VREG:r=$YMMis4 VREG:r=$YMMm  ', 'MOD=REG VL=256', 'VPCMOV_rxnxis4xmx', ''];
T['VPCMOV VREG:w=$XMMr VREG:r=$XMMn VREG:r=$XMMis4 MEM:r=$mem128 ', 'MOD=MEM VL=128', 'VPCMOV_rxnxis4xmx', ''];
T['VPCMOV VREG:w=$YMMr VREG:r=$YMMn VREG:r=$YMMis4 MEM:r=$mem256 ', 'MOD=MEM VL=256', 'VPCMOV_rxnxis4xmx', ''];
T['VPCMOV VREG:w=$XMMr VREG:r=$XMMn VREG:r=$XMMm   VREG:r=$XMMis4', 'MOD=REG VL=128', 'VPCMOV_rxnxmxis4x', ''];
T['VPCMOV VREG:w=$YMMr VREG:r=$YMMn VREG:r=$YMMm   VREG:r=$YMMis4', 'MOD=REG VL=256', 'VPCMOV_rxnxmxis4x', ''];
T['VPCMOV VREG:w=$XMMr VREG:r=$XMMn MEM:r=$mem128  VREG:r=$XMMis4', 'MOD=MEM VL=128', 'VPCMOV_rxnxmxis4x', ''];
T['VPCMOV VREG:w=$YMMr VREG:r=$YMMn MEM:r=$mem256  VREG:r=$YMMis4', 'MOD=MEM VL=256', 'VPCMOV_rxnxmxis4x', ''];

# VPCOMB-Compare Vector Signed Bytes.
T['VPCOMB VREG:w:s8=$XMMr VREG:r:s8=$XMMn VREG:r:s8=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPCOMB', ''];
T['VPCOMB VREG:w:s8=$XMMr VREG:r:s8=$XMMn MEM:r:s8=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'VPCOMB', ''];

# VPCOMD-Compare Vector Signed Doublewords.
T['VPCOMD VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPCOMD', ''];
T['VPCOMD VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'VPCOMD', ''];

# VPCOMQ-Compare Vector Signed Quadwords.
T['VPCOMQ VREG:w:s64=$XMMr VREG:r:s64=$XMMn VREG:r:s64=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPCOMQ', ''];
T['VPCOMQ VREG:w:s64=$XMMr VREG:r:s64=$XMMn MEM:r:s64=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'VPCOMQ', ''];

# VPCOMUB-Compare Vector Unsigned Bytes.
T['VPCOMUB VREG:w:u8=$XMMr VREG:r:u8=$XMMn VREG:r:u8=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPCOMUB', ''];
T['VPCOMUB VREG:w:u8=$XMMr VREG:r:u8=$XMMn MEM:r:u8=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'VPCOMUB', ''];

# VPCOMUD-Compare Vector Unsigned Doublewords.
T['VPCOMUD VREG:w:u32=$XMMr VREG:r:u32=$XMMn VREG:r:u32=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPCOMUD', ''];
T['VPCOMUD VREG:w:u32=$XMMr VREG:r:u32=$XMMn MEM:r:u32=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'VPCOMUD', ''];

# VPCOMUQ-Compare Vector Unsigned Quadwords.
T['VPCOMUQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn VREG:r:u64=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPCOMUQ', ''];
T['VPCOMUQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn MEM:r:u64=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'VPCOMUQ', ''];

# VPCOMUW-Compare Vector Unsigned Words.
T['VPCOMUW VREG:w:u16=$XMMr VREG:r:u16=$XMMn VREG:r:u16=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPCOMUW', ''];
T['VPCOMUW VREG:w:u16=$XMMr VREG:r:u16=$XMMn MEM:r:u16=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'VPCOMUW', ''];

# VPCOMW-Compare Vector Signed Words.
T['VPCOMW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPCOMW', ''];
T['VPCOMW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'VPCOMW', ''];

# VPERMIL2PS-Permute Two-Source Single-Precision Floating-Point Values.
T['VPERMIL2PS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 VREG:r:f32=$XMMm   CTL:r:u8=$ctl', 'MOD=REG VL=128', 'VPERMIL2PS_rxnxis4xmxis4', ''];
T['VPERMIL2PS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMis4 VREG:r:f32=$YMMm   CTL:r:u8=$ctl', 'MOD=REG VL=256', 'VPERMIL2PS_rxnxis4xmxis4', ''];
T['VPERMIL2PS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMis4 MEM:r:f32=$mem128  CTL:r:u8=$ctl', 'MOD=MEM VL=128', 'VPERMIL2PS_rxnxis4xmxis4', ''];
T['VPERMIL2PS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMis4 MEM:r:f32=$mem256  CTL:r:u8=$ctl', 'MOD=MEM VL=256', 'VPERMIL2PS_rxnxis4xmxis4', ''];
T['VPERMIL2PS VREG:w:f32=$XMMr VREG:r:f32=$XMMn VREG:r:f32=$XMMm   VREG:r:f32=$XMMis4 CTL:r:u8=$ctl', 'MOD=REG VL=128', 'VPERMIL2PS_rxnxmxis4xis4', ''];
T['VPERMIL2PS VREG:w:f32=$YMMr VREG:r:f32=$YMMn VREG:r:f32=$YMMm   VREG:r:f32=$YMMis4 CTL:r:u8=$ctl', 'MOD=REG VL=256', 'VPERMIL2PS_rxnxmxis4xis4', ''];
T['VPERMIL2PS VREG:w:f32=$XMMr VREG:r:f32=$XMMn MEM:r:f32=$mem128  VREG:r:f32=$XMMis4 CTL:r:u8=$ctl', 'MOD=MEM VL=128', 'VPERMIL2PS_rxnxmxis4xis4', ''];
T['VPERMIL2PS VREG:w:f32=$YMMr VREG:r:f32=$YMMn MEM:r:f32=$mem256  VREG:r:f32=$YMMis4 CTL:r:u8=$ctl', 'MOD=MEM VL=256', 'VPERMIL2PS_rxnxmxis4xis4', ''];

# VPHADDBD-Packed Horizontal Add Signed Byte to Signed Doubleword.
T['VPHADDBD VREG:w:s32=$XMMr VREG:r:s8=$XMMm ', 'MOD=REG', 'VPHADDBD', ''];
T['VPHADDBD VREG:w:s32=$XMMr MEM:r:s8=$mem128', 'MOD=MEM', 'VPHADDBD', ''];

# VPHADDBQ-Packed Horizontal Add Signed Byte to Signed Quadword.
T['VPHADDBQ VREG:w:s64=$XMMr VREG:r:s8=$XMMm ', 'MOD=REG', 'VPHADDBQ', ''];
T['VPHADDBQ VREG:w:s64=$XMMr MEM:r:s8=$mem128', 'MOD=MEM', 'VPHADDBQ', ''];

# VPHADDBW-Packed Horizontal Add Signed Byte to Signed Word.
T['VPHADDBW VREG:w:s8=$XMMr VREG:r:s8=$XMMm ', 'MOD=REG', 'VPHADDBW', ''];
T['VPHADDBW VREG:w:s8=$XMMr MEM:r:s8=$mem128', 'MOD=MEM', 'VPHADDBW', ''];

# VPHADDDQ-Packed Horizontal Add Signed Doubleword to Signed Quadword.
T['VPHADDDQ VREG:w:s64=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'VPHADDDQ', ''];
T['VPHADDDQ VREG:w:s64=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'VPHADDDQ', ''];

# VPHADDUBD-Packed Horizontal Add Unsigned Byte to Doubleword.
T['VPHADDUBD VREG:w:u32=$XMMr VREG:r:u8=$XMMm ', 'MOD=REG', 'VPHADDUBD', ''];
T['VPHADDUBD VREG:w:u32=$XMMr MEM:r:u8=$mem128', 'MOD=MEM', 'VPHADDUBD', ''];

# VPHADDUBQ-Packed Horizontal Add Unsigned Byte to Quadword.
T['VPHADDUBQ VREG:w:u64=$XMMr VREG:r:u8=$XMMm ', 'MOD=REG', 'VPHADDUBQ', ''];
T['VPHADDUBQ VREG:w:u64=$XMMr MEM:r:u8=$mem128', 'MOD=MEM', 'VPHADDUBQ', ''];

# VPHADDUBW-Packed Horizontal Add Unsigned Byte to Word.
T['VPHADDUBW VREG:w:u16=$XMMr VREG:r:u8=$XMMm ', 'MOD=REG', 'VPHADDUBW', ''];
T['VPHADDUBW VREG:w:u16=$XMMr MEM:r:u8=$mem128', 'MOD=MEM', 'VPHADDUBW', ''];

# VPHADDUDQ-Packed Horizontal Add Unsigned Doubleword to Quadword.
T['VPHADDUDQ VREG:w:u64=$XMMr VREG:r:u32=$XMMm ', 'MOD=REG', 'VPHADDUDQ', ''];
T['VPHADDUDQ VREG:w:u64=$XMMr MEM:r:u32=$mem128', 'MOD=MEM', 'VPHADDUDQ', ''];

# VPHADDUWQ-Packed Horizontal Add Unsigned Word to Quadword.
T['VPHADDUWQ VREG:w:u64=$XMMr VREG:r:u16=$XMMm ', 'MOD=REG', 'VPHADDUWQ', ''];
T['VPHADDUWQ VREG:w:u64=$XMMr MEM:r:u16=$mem128', 'MOD=MEM', 'VPHADDUWQ', ''];

# VPHADDWD-Packed Horizontal Add Signed Word to Signed Doubleword.
T['VPHADDWD VREG:w:s32=$XMMr VREG:r:s16=$XMMm ', 'MOD=REG', 'VPHADDWD', ''];
T['VPHADDWD VREG:w:s32=$XMMr MEM:r:s16=$mem128', 'MOD=MEM', 'VPHADDWD', ''];

# VPHADDWQ-Packed Horizontal Add Signed Word to Signed Quadword.
T['VPHADDWQ VREG:w:s64=$XMMr VREG:r:s16=$XMMm ', 'MOD=REG', 'VPHADDWQ', ''];
T['VPHADDWQ VREG:w:s64=$XMMr MEM:r:s16=$mem128', 'MOD=MEM', 'VPHADDWQ', ''];

# VPHSUBBW-Packed Horizontal Subtract Signed Byte to Signed Word.
T['VPHSUBBW VREG:w:s16=$XMMr VREG:r:s8=$XMMm ', 'MOD=REG', 'VPHSUBBW', ''];
T['VPHSUBBW VREG:w:s16=$XMMr MEM:r:s8=$mem128', 'MOD=MEM', 'VPHSUBBW', ''];

# VPHSUBDQ-Packed Horizontal Subtract Signed Doubleword to Signed Quadword.
T['VPHSUBDQ VREG:w:s64=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'VPHSUBDQ', ''];
T['VPHSUBDQ VREG:w:s64=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'VPHSUBDQ', ''];

# VPHSUBWD-Packed Horizontal Subtract Signed Word to Signed Doubleword.
T['VPHSUBWD VREG:w:s32=$XMMr VREG:r:s16=$XMMm ', 'MOD=REG', 'VPHSUBWD', ''];
T['VPHSUBWD VREG:w:s32=$XMMr MEM:r:s16=$mem128', 'MOD=MEM', 'VPHSUBWD', ''];

# VPMACSDD-Packed Multiply Accumulate Signed Doubleword to Signed Doubleword.
T['VPMACSDD VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm  VREG:r:s32=$XMMis4', 'MOD=REG', 'VPMACSDD', ''];
T['VPMACSDD VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128 VREG:r:s32=$XMMis4', 'MOD=MEM', 'VPMACSDD', ''];

# VPMACSDQH-Packed Multiply Accumulate Signed High Doubleword to Signed Quadword.
T['VPMACSDQH VREG:w:s64=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm  VREG:r:s64=$XMMis4', 'MOD=REG', 'VPMACSDQH', ''];
T['VPMACSDQH VREG:w:s64=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128 VREG:r:s64=$XMMis4', 'MOD=MEM', 'VPMACSDQH', ''];

# VPMACSDQL-Packed Multiply Accumulate Signed Low Doubleword to Signed Quadword.
T['VPMACSDQL VREG:w:s64=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm  VREG:r:s64=$XMMis4', 'MOD=REG', 'VPMACSDQL', ''];
T['VPMACSDQL VREG:w:s64=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128 VREG:r:s64=$XMMis4', 'MOD=MEM', 'VPMACSDQL', ''];

# VPMACSSDD-Packed Multiply Accumulate Signed Doubleword to Signed Doubleword with Saturation.
T['VPMACSSDD VREG:w:s32=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm  VREG:r:s32=$XMMis4', 'MOD=REG', 'VPMACSSDD', ''];
T['VPMACSSDD VREG:w:s32=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128 VREG:r:s32=$XMMis4', 'MOD=MEM', 'VPMACSSDD', ''];

# VPMACSSDQH-Packed Multiply Accumulate Signed High Doubleword to Signed Quadword with.
T['VPMACSSDQH VREG:w:s64=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm  VREG:r:s64=$XMMis4', 'MOD=REG', 'VPMACSSDQH', ''];
T['VPMACSSDQH VREG:w:s64=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128 VREG:r:s64=$XMMis4', 'MOD=MEM', 'VPMACSSDQH', ''];

# VPMACSSDQL-Packed Multiply Accumulate Signed Low Doubleword to Signed Quadword with.
T['VPMACSSDQL VREG:w:s64=$XMMr VREG:r:s32=$XMMn VREG:r:s32=$XMMm  VREG:r:s64=$XMMis4', 'MOD=REG', 'VPMACSSDQL', ''];
T['VPMACSSDQL VREG:w:s64=$XMMr VREG:r:s32=$XMMn MEM:r:s32=$mem128 VREG:r:s64=$XMMis4', 'MOD=MEM', 'VPMACSSDQL', ''];

# VPMACSSWD-Packed Multiply Accumulate Signed Word to Signed Doubleword with Saturation.
T['VPMACSSWD VREG:w:s32=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm  VREG:r:s32=$XMMis4', 'MOD=REG', 'VPMACSSWD', ''];
T['VPMACSSWD VREG:w:s32=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128 VREG:r:s32=$XMMis4', 'MOD=MEM', 'VPMACSSWD', ''];

# VPMACSSWW-Packed Multiply Accumulate Signed Word to Signed Word with Saturation.
T['VPMACSSWW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm  VREG:r:s16=$XMMis4', 'MOD=REG', 'VPMACSSWW', ''];
T['VPMACSSWW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128 VREG:r:s16=$XMMis4', 'MOD=MEM', 'VPMACSSWW', ''];

# VPMACSWD-Packed Multiply Accumulate Signed Word to Signed Doubleword.
T['VPMACSWD VREG:w:s32=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm  VREG:r:s32=$XMMis4', 'MOD=REG', 'VPMACSWD', ''];
T['VPMACSWD VREG:w:s32=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128 VREG:r:s32=$XMMis4', 'MOD=MEM', 'VPMACSWD', ''];

# VPMACSWW-Packed Multiply Accumulate Signed Word to Signed Word.
T['VPMACSWW VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm  VREG:r:s16=$XMMis4', 'MOD=REG', 'VPMACSWW', ''];
T['VPMACSWW VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128 VREG:r:s16=$XMMis4', 'MOD=MEM', 'VPMACSWW', ''];

# VPMADCSSWD-Packed Multiply, Add and Accumulate Signed Word to Signed Doubleword with Saturation.
T['VPMADCSSWD VREG:w:s32=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm  VREG:r:s32=$XMMis4', 'MOD=REG', 'VPMADCSSWD', ''];
T['VPMADCSSWD VREG:w:s32=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128 VREG:r:s32=$XMMis4', 'MOD=MEM', 'VPMADCSSWD', ''];

# VPMADCSWD-Packed Multiply Add and Accumulate Signed Word to Signed Doubleword.
T['VPMADCSWD VREG:w:s32=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm  VREG:r:s32=$XMMis4', 'MOD=REG', 'VPMADCSWD', ''];
T['VPMADCSWD VREG:w:s32=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128 VREG:r:s32=$XMMis4', 'MOD=MEM', 'VPMADCSWD', ''];

# VPPERM-Packed Permute Bytes.
T['VPPERM VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMis4 VREG:r:s16=$XMMm  ', 'MOD=REG', 'VPPERM_ronois4omo', ''];
T['VPPERM VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMis4 MEM:r:s16=$mem128 ', 'MOD=MEM', 'VPPERM_ronois4omo', ''];
T['VPPERM VREG:w:s16=$XMMr VREG:r:s16=$XMMn VREG:r:s16=$XMMm   VREG:r:s16=$XMMis4', 'MOD=REG', 'VPPERM_ronomois4o', ''];
T['VPPERM VREG:w:s16=$XMMr VREG:r:s16=$XMMn MEM:r:s16=$mem128  VREG:r:s16=$XMMis4', 'MOD=MEM', 'VPPERM_ronomois4o', ''];

# VPROTB-Packed Rotate Bytes.
T['VPROTB VREG:w:u8=$XMMr VREG:r:u8=$XMMm  VREG:r:u8=$XMMn ', 'MOD=REG', 'VPROTB_romono', ''];
T['VPROTB VREG:w:u8=$XMMr MEM:r:u8=$mem128 VREG:r:u8=$XMMn ', 'MOD=MEM', 'VPROTB_romono', ''];
T['VPROTB VREG:w:u8=$XMMr VREG:r:u8=$XMMm  IMM:u8=$uimm8   ', 'MOD=REG', 'VPROTB_romoub', ''];
T['VPROTB VREG:w:u8=$XMMr MEM:r:u8=$mem128 IMM:u8=$uimm8   ', 'MOD=MEM', 'VPROTB_romoub', ''];
T['VPROTB VREG:w:u8=$XMMr VREG:r:u8=$XMMn  VREG:r:u8=$XMMm ', 'MOD=REG', 'VPROTB_ronomo', ''];
T['VPROTB VREG:w:u8=$XMMr VREG:r:u8=$XMMn  MEM:r:u8=$mem128', 'MOD=MEM', 'VPROTB_ronomo', ''];

# VPROTD-Packed Rotate Doublewords.
T['VPROTD VREG:w:u32=$XMMr VREG:r:u32=$XMMm  VREG:r:u32=$XMMn ', 'MOD=REG', 'VPROTD_romono', ''];
T['VPROTD VREG:w:u32=$XMMr MEM:r:u32=$mem128 VREG:r:u32=$XMMn ', 'MOD=MEM', 'VPROTD_romono', ''];
T['VPROTD VREG:w:u32=$XMMr VREG:r:u32=$XMMm  IMM:u8=$uimm8    ', 'MOD=REG', 'VPROTD_romoub', ''];
T['VPROTD VREG:w:u32=$XMMr MEM:r:u32=$mem128 IMM:u8=$uimm8    ', 'MOD=MEM', 'VPROTD_romoub', ''];
T['VPROTD VREG:w:u32=$XMMr VREG:r:u32=$XMMn  VREG:r:u32=$XMMm ', 'MOD=REG', 'VPROTD_ronomo', ''];
T['VPROTD VREG:w:u32=$XMMr VREG:r:u32=$XMMn  MEM:r:u32=$mem128', 'MOD=MEM', 'VPROTD_ronomo', ''];

# VPROTQ-Packed Rotate Quadwords.
T['VPROTQ VREG:w:u64=$XMMr VREG:r:u64=$XMMm  VREG:r:u64=$XMMn ', 'MOD=REG', 'VPROTQ_romono', ''];
T['VPROTQ VREG:w:u64=$XMMr MEM:r:u64=$mem128 VREG:r:u64=$XMMn ', 'MOD=MEM', 'VPROTQ_romono', ''];
T['VPROTQ VREG:w:u64=$XMMr VREG:r:u64=$XMMm  IMM:u8=$uimm8    ', 'MOD=REG', 'VPROTQ_romoub', ''];
T['VPROTQ VREG:w:u64=$XMMr MEM:r:u64=$mem128 IMM:u8=$uimm8    ', 'MOD=MEM', 'VPROTQ_romoub', ''];
T['VPROTQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn  VREG:r:u64=$XMMm ', 'MOD=REG', 'VPROTQ_ronomo', ''];
T['VPROTQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn  MEM:r:u64=$mem128', 'MOD=MEM', 'VPROTQ_ronomo', ''];

# VPROTW-Packed Rotate Words.
T['VPROTW VREG:w:u16=$XMMr VREG:r:u16=$XMMm  VREG:r:u16=$XMMn ', 'MOD=REG', 'VPROTW_romono', ''];
T['VPROTW VREG:w:u16=$XMMr MEM:r:u16=$mem128 VREG:r:u16=$XMMn ', 'MOD=MEM', 'VPROTW_romono', ''];
T['VPROTW VREG:w:u16=$XMMr VREG:r:u16=$XMMm  IMM:u8=$uimm8    ', 'MOD=REG', 'VPROTW_romoub', ''];
T['VPROTW VREG:w:u16=$XMMr MEM:r:u16=$mem128 IMM:u8=$uimm8    ', 'MOD=MEM', 'VPROTW_romoub', ''];
T['VPROTW VREG:w:u16=$XMMr VREG:r:u16=$XMMn  VREG:r:u16=$XMMm ', 'MOD=REG', 'VPROTW_ronomo', ''];
T['VPROTW VREG:w:u16=$XMMr VREG:r:u16=$XMMn  MEM:r:u16=$mem128', 'MOD=MEM', 'VPROTW_ronomo', ''];

# VPSHAB-Packed Shift Arithmetic Bytes.
T['VPSHAB VREG:w:s8=$XMMr VREG:r:s8=$XMMm  VREG:r:s8=$XMMn ', 'MOD=REG', 'VPSHAB_romono', ''];
T['VPSHAB VREG:w:s8=$XMMr MEM:r:s8=$mem128 VREG:r:s8=$XMMn ', 'MOD=MEM', 'VPSHAB_romono', ''];
T['VPSHAB VREG:w:s8=$XMMr VREG:r:s8=$XMMn  VREG:r:s8=$XMMm ', 'MOD=REG', 'VPSHAB_ronomo', ''];
T['VPSHAB VREG:w:s8=$XMMr VREG:r:s8=$XMMn  MEM:r:s8=$mem128', 'MOD=MEM', 'VPSHAB_ronomo', ''];

# VPSHAD-Packed Shift Arithmetic Doublewords.
T['VPSHAD VREG:w:s32=$XMMr VREG:r:s32=$XMMm  VREG:r:s32=$XMMn ', 'MOD=REG', 'VPSHAD_romono', ''];
T['VPSHAD VREG:w:s32=$XMMr MEM:r:s32=$mem128 VREG:r:s32=$XMMn ', 'MOD=MEM', 'VPSHAD_romono', ''];
T['VPSHAD VREG:w:s32=$XMMr VREG:r:s32=$XMMn  VREG:r:s32=$XMMm ', 'MOD=REG', 'VPSHAD_ronomo', ''];
T['VPSHAD VREG:w:s32=$XMMr VREG:r:s32=$XMMn  MEM:r:s32=$mem128', 'MOD=MEM', 'VPSHAD_ronomo', ''];

# VPSHAQ-Packed Shift Arithmetic Quadwords.
T['VPSHAQ VREG:w:s64=$XMMr VREG:r:s64=$XMMm  VREG:r:s64=$XMMn ', 'MOD=REG', 'VPSHAQ_romono', ''];
T['VPSHAQ VREG:w:s64=$XMMr MEM:r:s64=$mem128 VREG:r:s64=$XMMn ', 'MOD=MEM', 'VPSHAQ_romono', ''];
T['VPSHAQ VREG:w:s64=$XMMr VREG:r:s64=$XMMn  VREG:r:s64=$XMMm ', 'MOD=REG', 'VPSHAQ_ronomo', ''];
T['VPSHAQ VREG:w:s64=$XMMr VREG:r:s64=$XMMn  MEM:r:s64=$mem128', 'MOD=MEM', 'VPSHAQ_ronomo', ''];

# VPSHAW-Packed Shift Arithmetic Words.
T['VPSHAW VREG:w:s16=$XMMr VREG:r:s16=$XMMm  VREG:r:s16=$XMMn ', 'MOD=REG', 'VPSHAW_romono', ''];
T['VPSHAW VREG:w:s16=$XMMr MEM:r:s16=$mem128 VREG:r:s16=$XMMn ', 'MOD=MEM', 'VPSHAW_romono', ''];
T['VPSHAW VREG:w:s16=$XMMr VREG:r:s16=$XMMn  VREG:r:s16=$XMMm ', 'MOD=REG', 'VPSHAW_ronomo', ''];
T['VPSHAW VREG:w:s16=$XMMr VREG:r:s16=$XMMn  MEM:r:s16=$mem128', 'MOD=MEM', 'VPSHAW_ronomo', ''];

# VPSHLB-Packed Shift Logical Bytes.
T['VPSHLB VREG:w:u8=$XMMr VREG:r:u8=$XMMm  VREG:r:u8=$XMMn ', 'MOD=REG', 'VPSHLB_romono', ''];
T['VPSHLB VREG:w:u8=$XMMr MEM:r:u8=$mem128 VREG:r:u8=$XMMn ', 'MOD=MEM', 'VPSHLB_romono', ''];
T['VPSHLB VREG:w:u8=$XMMr VREG:r:u8=$XMMn  VREG:r:u8=$XMMm ', 'MOD=REG', 'VPSHLB_ronomo', ''];
T['VPSHLB VREG:w:u8=$XMMr VREG:r:u8=$XMMn  MEM:r:u8=$mem128', 'MOD=MEM', 'VPSHLB_ronomo', ''];

# VPSHLD-Packed Shift Logical Doublewords.
T['VPSHLD VREG:w:u32=$XMMr VREG:r:u32=$XMMm  VREG:r:u32=$XMMn ', 'MOD=REG', 'VPSHLD_romono', ''];
T['VPSHLD VREG:w:u32=$XMMr MEM:r:u32=$mem128 VREG:r:u32=$XMMn ', 'MOD=MEM', 'VPSHLD_romono', ''];
T['VPSHLD VREG:w:u32=$XMMr VREG:r:u32=$XMMn  VREG:r:u32=$XMMm ', 'MOD=REG', 'VPSHLD_ronomo', ''];
T['VPSHLD VREG:w:u32=$XMMr VREG:r:u32=$XMMn  MEM:r:u32=$mem128', 'MOD=MEM', 'VPSHLD_ronomo', ''];

# VPSHLQ-Packed Shift Logical Quadwords.
T['VPSHLQ VREG:w:u64=$XMMr VREG:r:u64=$XMMm  VREG:r:u64=$XMMn ', 'MOD=REG', 'VPSHLQ_romono', ''];
T['VPSHLQ VREG:w:u64=$XMMr MEM:r:u64=$mem128 VREG:r:u64=$XMMn ', 'MOD=MEM', 'VPSHLQ_romono', ''];
T['VPSHLQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn  VREG:r:u64=$XMMm ', 'MOD=REG', 'VPSHLQ_ronomo', ''];
T['VPSHLQ VREG:w:u64=$XMMr VREG:r:u64=$XMMn  MEM:r:u64=$mem128', 'MOD=MEM', 'VPSHLQ_ronomo', ''];

# VPSHLW-Packed Shift Logical Words.
T['VPSHLW VREG:w:u16=$XMMr VREG:r:u16=$XMMm  VREG:r:u16=$XMMn ', 'MOD=REG', 'VPSHLW_romono', ''];
T['VPSHLW VREG:w:u16=$XMMr MEM:r:u16=$mem128 VREG:r:u16=$XMMn ', 'MOD=MEM', 'VPSHLW_romono', ''];
T['VPSHLW VREG:w:u16=$XMMr VREG:r:u16=$XMMn  VREG:r:u16=$XMMm ', 'MOD=REG', 'VPSHLW_ronomo', ''];
T['VPSHLW VREG:w:u16=$XMMr VREG:r:u16=$XMMn  MEM:r:u16=$mem128', 'MOD=MEM', 'VPSHLW_ronomo', ''];
