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


# BNDCL-Check Lower Bound.
T['BNDCL REG:r=$BNDr AGEN:r=$agen ', 'MODE=64         ', 'BNDCL_ro  ', ''];
T['BNDCL REG:r=$BNDr AGEN:r=$agen ', 'ASZ=32 MODE=NO64', 'BNDCL_ro  ', ''];
T['BNDCL REG:r=$BNDr REG:r=$GPR64m', 'MODE=64         ', 'BNDCL_romx', ''];
T['BNDCL REG:r=$BNDr REG:r=$GPR32m', 'MODE=NO64       ', 'BNDCL_romx', ''];

# BNDCU/BNDCN-Check Upper Bound.
T['BNDCN REG:r=$BNDr AGEN:r=$agen ', 'MODE=64         ', 'BNDCN_ro  ', ''];
T['BNDCN REG:r=$BNDr AGEN:r=$agen ', 'ASZ=32 MODE=NO64', 'BNDCN_ro  ', ''];
T['BNDCN REG:r=$BNDr REG:r=$GPR64m', 'MODE=64         ', 'BNDCN_romx', ''];
T['BNDCN REG:r=$BNDr REG:r=$GPR32m', 'MODE=NO64       ', 'BNDCN_romx', ''];
T['BNDCU REG:r=$BNDr AGEN:r=$agen ', 'MODE=64         ', 'BNDCU_ro  ', ''];
T['BNDCU REG:r=$BNDr AGEN:r=$agen ', 'ASZ=32 MODE=NO64', 'BNDCU_ro  ', ''];
T['BNDCU REG:r=$BNDr REG:r=$GPR64m', 'MODE=64         ', 'BNDCU_romx', ''];
T['BNDCU REG:r=$BNDr REG:r=$GPR32m', 'MODE=NO64       ', 'BNDCU_romx', ''];

# BNDLDX-Load Extended Bounds Using Address Translation.
T['BNDLDX REG:w=$BNDr MIB:r:u64=$mib192', 'MODE=64         ', 'BNDLDX', ''];
T['BNDLDX REG:w=$BNDr MIB:r:u32=$mib96 ', 'ASZ=32 MODE=NO64', 'BNDLDX', ''];

# BNDMK-Make Bounds.
T['BNDMK REG:w=$BNDr AGEN:r=$agen', 'MODE=64         ', 'BNDMK', ''];
T['BNDMK REG:w=$BNDr AGEN:r=$agen', 'ASZ=32 MODE=NO64', 'BNDMK', ''];

# BNDMOV-Move Bounds.
T['BNDMOV REG:w=$BNDm       REG:r=$BNDr      ', 'MOD=REG                 ', 'BNDMOV_mxro', ''];
T['BNDMOV MEM:w:u64=$mem128 REG:r=$BNDr      ', 'MOD=MEM MODE=64         ', 'BNDMOV_mxro', ''];
T['BNDMOV MEM:w:u32=$mem64  REG:r=$BNDr      ', 'ASZ=32 MOD=MEM MODE=NO64', 'BNDMOV_mxro', ''];
T['BNDMOV REG:w=$BNDr       REG:r=$BNDm      ', 'MOD=REG                 ', 'BNDMOV_romx', ''];
T['BNDMOV REG:w=$BNDr       MEM:r:u64=$mem128', 'MOD=MEM MODE=64         ', 'BNDMOV_romx', ''];
T['BNDMOV REG:w=$BNDr       MEM:r:u32=$mem64 ', 'ASZ=32 MOD=MEM MODE=NO64', 'BNDMOV_romx', ''];

# BNDSTX-Store Extended Bounds Using Address Translation.
T['BNDSTX MIB:w:u64=$mib192 REG:r=$BNDr', 'MODE=64         ', 'BNDSTX', ''];
T['BNDSTX MIB:w:u32=$mib96  REG:r=$BNDr', 'ASZ=32 MODE=NO64', 'BNDSTX', ''];

# CLAC-Clear AC Flag in EFLAGS Register.
T['CLAC', 'NONE', 'CLAC', ''];

# CLDEMOTE-Cache Line Demote.
T['CLDEMOTE MEM:r:u8=$mem8', 'NONE', 'CLDEMOTE', ''];

# CLFLUSH-Flush Cache Line.
T['CLFLUSH MEM:r:s64=$mem512', 'NONE', 'CLFLUSH', ''];

# CLFLUSHOPT-Flush Cache Line Optimized.
T['CLFLUSHOPT MEM:r:s64=$mem512', 'NONE', 'CLFLUSHOPT', ''];

# CLRSSBSY-Clear Busy Flag in a Supervisor Shadow Stack Token.
T['CLRSSBSY MEM:rw:u64=$mem64', 'NONE', 'CLRSSBSY', ''];

# CLWB-Cache Line Write Back.
T['CLWB MEM:r:s64=$mem512', 'NONE', 'CLWB', ''];

# ENCLS.
T['ENCLS REG:SUPP:r=EAX REG:SUPP:crw=RBX REG:SUPP:crw=RCX REG:SUPP:crw=RDX', 'MODE=64  ', 'ENCLS', ''];
T['ENCLS REG:SUPP:r=EAX REG:SUPP:crw=EBX REG:SUPP:crw=ECX REG:SUPP:crw=EDX', 'MODE=NO64', 'ENCLS', ''];

# ENCLU.
T['ENCLU REG:SUPP:r=EAX REG:SUPP:crw=RBX REG:SUPP:crw=RCX REG:SUPP:crw=RDX', 'MODE=64  ', 'ENCLU', ''];
T['ENCLU REG:SUPP:r=EAX REG:SUPP:crw=EBX REG:SUPP:crw=ECX REG:SUPP:crw=EDX', 'MODE=NO64', 'ENCLU', ''];

# ENCLV.
T['ENCLV REG:SUPP:r:u32=EAX REG:SUPP:crw:u64=RBX REG:SUPP:crw:u64=RCX REG:SUPP:crw:u64=RDX', 'MODE=64  ', 'ENCLV', ''];
T['ENCLV REG:SUPP:r:u32=EAX REG:SUPP:crw:u64=EBX REG:SUPP:crw:u64=ECX REG:SUPP:crw:u64=EDX', 'MODE=NO64', 'ENCLV', ''];

# ENDBR32-Terminate an Indirect Branch in 32-bit and Compatibility Mode.
T['ENDBR32', 'NONE', 'ENDBR32', ''];

# ENDBR64-Terminate an Indirect Branch in 64-bit Mode.
T['ENDBR64', 'NONE', 'ENDBR64', ''];

# ENQCMD.
T['ENQCMD REG:r=$GPRar MEM:r:u32=$mem512', 'NONE', 'ENQCMD', ''];

# ENQCMDS.
T['ENQCMDS REG:r=$GPRar MEM:r:u32=$mem512', 'NONE', 'ENQCMDS', ''];

# GETSEC.
T['GETSEC REG:SUPP:rcw=EAX REG:SUPP:r=EBX', 'NONE', 'GETSEC', ''];

# INCSSPD/INCSSPQ-Increment Shadow Stack Pointer.
T['INCSSPD REG:r:u8=$GPR32m', 'W=0', 'INCSSPx', ''];
T['INCSSPQ REG:r:u8=$GPR64m', 'W=1', 'INCSSPx', ''];

# INVEPT-Invalidate Translations Derived from EPT.
T['INVEPT REG:r=$GPR64r MEM:r:s32=$mem128', 'MODE=64  ', 'INVEPT', ''];
T['INVEPT REG:r=$GPR32r MEM:r:s32=$mem128', 'MODE=NO64', 'INVEPT', ''];

# INVPCID-Invalidate Process-Context Identifier.
T['INVPCID REG:r=$GPR64r MEM:r:s32=$mem128', 'MODE=64  ', 'INVPCID', ''];
T['INVPCID REG:r=$GPR32r MEM:r:s32=$mem128', 'MODE=NO64', 'INVPCID', ''];

# INVVPID-Invalidate Translations Based on VPID.
T['INVVPID REG:r=$GPR64r MEM:r:s32=$mem128', 'MODE=64  ', 'INVVPID', ''];
T['INVVPID REG:r=$GPR32r MEM:r:s32=$mem128', 'MODE=NO64', 'INVVPID', ''];

# MONITOR-Set Up Monitor Address.
T['MONITOR REG:SUPP:r=$AXa REG:SUPP:r=RCX REG:SUPP:r=RDX', 'MODE=64  ', 'MONITOR', ''];
T['MONITOR REG:SUPP:r=$AXa REG:SUPP:r=ECX REG:SUPP:r=EDX', 'MODE=NO64', 'MONITOR', ''];

# MWAIT-Monitor Wait.
T['MWAIT REG:SUPP:r=RAX REG:SUPP:r=RCX', 'MODE=64  ', 'MWAIT', ''];
T['MWAIT REG:SUPP:r=EAX REG:SUPP:r=ECX', 'MODE=NO64', 'MWAIT', ''];

# PTWRITE-Write Data to a Processor Trace Packet.
T['PTWRITE REG:r=$GPRym  ', 'MOD=REG', 'PTWRITE', ''];
T['PTWRITE MEM:r:sx=$memy', 'MOD=MEM', 'PTWRITE', ''];

# RDPID-Read Processor ID.
T['RDPID REG:w:u64=$GPR64m', 'MODE=64  ', 'RDPID', ''];
T['RDPID REG:w:u32=$GPR32m', 'MODE=NO64', 'RDPID', ''];

# RDPKRU-Read Protection Key Rights for User Pages.
T['RDPKRU REG:SUPP:r=PKRU REG:SUPP:w=EDX REG:SUPP:w=EAX REG:SUPP:r=ECX', 'NONE', 'RDPKRU', ''];

# RDRAND-Read Random Number.
T['RDRAND REG:w=$GPRvm', 'NONE', 'RDRAND', ''];

# RDSEED-Read Random SEED.
T['RDSEED REG:w=$GPRvm', 'NONE', 'RDSEED', ''];

# RDSSPD/RDSSPQ-Read Shadow Stack Pointer.
T['RDSSPD REG:w:u32=$GPR32m', 'MODE=NO64 W=0', 'RDSSPx', ''];
T['RDSSPQ REG:w:u64=$GPR64m', 'MODE=64 W=1  ', 'RDSSPx', ''];

# RDTSCP-Read Time-Stamp Counter and Processor ID.
T['RDTSCP REG:SUPP:w=EAX REG:SUPP:w=EDX REG:SUPP:w=ECX', 'NONE', 'RDTSCP', ''];

# RDFSBASE/RDGSBASE-Read FS/GS Segment Base.
T['RDFSBASE REG:w=$GPRym', 'NONE', 'RDFSBASE', ''];
T['RDGSBASE REG:w=$GPRym', 'NONE', 'RDGSBASE', ''];

# RSTORSSP-Restore Saved Shadow Stack Pointer.
T['RSTORSSP MEM:rw:u64=$mem64', 'NONE', 'RSTORSSP', ''];

# SAVEPREVSSP-Save Previous Shadow Stack Pointer.
T['SAVEPREVSSP', 'NONE', 'SAVEPREVSSP', ''];

# SETSSBSY-Mark Shadow Stack Busy.
T['SETSSBSY', 'NONE', 'SETSSBSY', ''];

# STAC-Set AC Flag in EFLAGS Register.
T['STAC', 'NONE', 'STAC', ''];

# TPAUSE-Timed PAUSE.
T['TPAUSE REG:r:u32=$GPR32m REG:SUPP:r:u32=EDX REG:SUPP:r:u32=EAX', 'W=0', 'TPAUSE', ''];
T['TPAUSE REG:r:u64=$GPR64m REG:SUPP:r:u32=EDX REG:SUPP:r:u32=EAX', 'W=1', 'TPAUSE', ''];

# UMONITOR-User Level Set Up Monitor Address.
T['UMONITOR REG:r=$GPRam', 'NONE', 'UMONITOR', ''];

# UMWAIT-User Level Monitor Wait.
T['UMWAIT REG:r:u32=$GPR32m REG:SUPP:r:u32=EDX REG:SUPP:r:u32=EAX', 'W=0', 'UMWAIT', ''];
T['UMWAIT REG:r:u64=$GPR64m REG:SUPP:r:u32=EDX REG:SUPP:r:u32=EAX', 'W=1', 'UMWAIT', ''];

# VMCALL-Call to VM Monitor.
T['VMCALL', 'NONE', 'VMCALL', ''];

# VMCLEAR-Clear Virtual-Machine Control Structure.
T['VMCLEAR MEM:r:s64=$mem64', 'NONE', 'VMCLEAR', ''];

# VMFUNC-Invoke VM function.
T['VMFUNC REG:SUPP:r=EAX', 'NONE', 'VMFUNC', ''];

# VMLAUNCH/VMRESUME-Launch/Resume Virtual Machine.
T['VMLAUNCH', 'NONE', 'VMLAUNCH', ''];
T['VMRESUME', 'NONE', 'VMRESUME', ''];

# VMPTRLD-Load Pointer to Virtual-Machine Control Structure.
T['VMPTRLD MEM:r:s64=$mem64', 'NONE', 'VMPTRLD', ''];

# VMPTRST-Store Pointer to Virtual-Machine Control Structure.
T['VMPTRST MEM:w:s64=$mem64', 'NONE', 'VMPTRST', ''];

# VMREAD-Read Field from Virtual-Machine Control Structure.
T['VMREAD REG:w=$GPR64m    REG:r=$GPR64r', 'MOD=REG MODE=64  ', 'VMREAD', ''];
T['VMREAD MEM:w:s64=$mem64 REG:r=$GPR64r', 'MOD=MEM MODE=64  ', 'VMREAD', ''];
T['VMREAD REG:w=$GPR32m    REG:r=$GPR32r', 'MOD=REG MODE=NO64', 'VMREAD', ''];
T['VMREAD MEM:w:s32=$mem32 REG:r=$GPR32r', 'MOD=MEM MODE=NO64', 'VMREAD', ''];

# VMWRITE-Write Field to Virtual-Machine Control Structure.
T['VMWRITE REG:r=$GPR64r REG:r=$GPR64m   ', 'MOD=REG MODE=64  ', 'VMWRITE', ''];
T['VMWRITE REG:r=$GPR64r MEM:r:s64=$mem64', 'MOD=MEM MODE=64  ', 'VMWRITE', ''];
T['VMWRITE REG:r=$GPR32r REG:r=$GPR32m   ', 'MOD=REG MODE=NO64', 'VMWRITE', ''];
T['VMWRITE REG:r=$GPR32r MEM:r:s32=$mem32', 'MOD=MEM MODE=NO64', 'VMWRITE', ''];

# VMXOFF-Leave VMX Operation.
T['VMXOFF', 'NONE', 'VMXOFF', ''];

# VMXON-Enter VMX Operation.
T['VMXON MEM:r:s64=$mem64', 'NONE', 'VMXON', ''];

# WRPKRU-Write Data to User Page Key Register.
T['WRPKRU REG:SUPP:w=PKRU REG:SUPP:r=EDX REG:SUPP:r=EAX REG:SUPP:r=ECX', 'NONE', 'WRPKRU', ''];

# WRSSD/WRSSQ-Write to Shadow Stack.
T['WRSSD MEM:w:u32=$mem32 REG:r:u32=$GPR32r', 'W=0', 'WRSSx', ''];
T['WRSSQ MEM:w:u64=$mem64 REG:r:u64=$GPR64r', 'W=1', 'WRSSx', ''];

# WRUSSD/WRUSSQ-Write to User Shadow Stack.
T['WRUSSD MEM:w:u32=$mem32 REG:r:u32=$GPR32r', 'W=0', 'WRUSSx', ''];
T['WRUSSQ MEM:w:u64=$mem64 REG:r:u64=$GPR64r', 'W=1', 'WRUSSx', ''];

# WRFSBASE/WRGSBASE-Write FS/GS Segment Base.
T['WRFSBASE REG:r=$GPRym', 'NONE', 'WRFSBASE', ''];
T['WRGSBASE REG:r=$GPRym', 'NONE', 'WRGSBASE', ''];

# XABORT-Transactional Abort.
T['XABORT IMM:u8=$uimm8 REG:SUPP:rcw=EAX', 'NONE', 'XABORT', ''];

# XBEGIN-Transactional Begin.
T['XBEGIN REL:sx=$relz REG:SUPP:rw=$IPa REG:SUPP:cw=EAX', 'NONE', 'XBEGIN', ''];

# XEND-Transactional End.
T['XEND', 'NONE', 'XEND', ''];

# XGETBV-Get Value of Extended Control Register.
T['XGETBV REG:SUPP:r=ECX REG:SUPP:w=EDX REG:SUPP:w=EAX REG:SUPP:r=XCR0', 'NONE', 'XGETBV', ''];

# XRSTOR-Restore Processor Extended States.
T['XRSTOR   MEM:r=$mem4608 REG:SUPP:r=EDX REG:SUPP:r=EAX REG:SUPP:r=XCR0', 'W=0', 'XRSTOR', ''];
T['XRSTOR64 MEM:r=$mem4608 REG:SUPP:r=EDX REG:SUPP:r=EAX REG:SUPP:r=XCR0', 'W=1', 'XRSTOR', ''];

# XRSTORS-Restore Processor Extended States Supervisor.
T['XRSTORS   MEM:r=$mem4608 REG:SUPP:r=EDX REG:SUPP:r=EAX REG:SUPP:r=XCR0', 'W=0', 'XRSTORS', ''];
T['XRSTORS64 MEM:r=$mem4608 REG:SUPP:r=EDX REG:SUPP:r=EAX REG:SUPP:r=XCR0', 'W=1', 'XRSTORS', ''];

# XSAVE-Save Processor Extended States.
T['XSAVE   MEM:rw=$mem4608 REG:SUPP:r=EDX REG:SUPP:r=EAX REG:SUPP:r=XCR0', 'W=0', 'XSAVE', ''];
T['XSAVE64 MEM:rw=$mem4608 REG:SUPP:r=EDX REG:SUPP:r=EAX REG:SUPP:r=XCR0', 'W=1', 'XSAVE', ''];

# XSAVEC-Save Processor Extended States with Compaction.
T['XSAVEC   MEM:w=$mem4608 REG:SUPP:r=EDX REG:SUPP:r=EAX REG:SUPP:r=XCR0', 'W=0', 'XSAVEC', ''];
T['XSAVEC64 MEM:w=$mem4608 REG:SUPP:r=EDX REG:SUPP:r=EAX REG:SUPP:r=XCR0', 'W=1', 'XSAVEC', ''];

# XSAVEOPT-Save Processor Extended States Optimized.
T['XSAVEOPT   MEM:rw=$mem4608 REG:SUPP:r=EDX REG:SUPP:r=EAX REG:SUPP:r=XCR0', 'W=0', 'XSAVEOPT', ''];
T['XSAVEOPT64 MEM:rw=$mem4608 REG:SUPP:r=EDX REG:SUPP:r=EAX REG:SUPP:r=XCR0', 'W=1', 'XSAVEOPT', ''];

# XSAVES-Save Processor Extended States Supervisor.
T['XSAVES   MEM:w=$mem4608 REG:SUPP:r=EDX REG:SUPP:r=EAX REG:SUPP:r=XCR0', 'NONE', 'XSAVES  ', ''];
T['XSAVES64 MEM:w=$mem4608 REG:SUPP:r=EDX REG:SUPP:r=EAX REG:SUPP:r=XCR0', 'NONE', 'XSAVES64', ''];

# XSETBV-Set Extended Control Register.
T['XSETBV REG:SUPP:r=ECX REG:SUPP:r=EDX REG:SUPP:r=EAX REG:SUPP:w=XCR0', 'NONE', 'XSETBV', ''];

# XTEST-Test If In Transactional Execution.
T['XTEST', 'NONE', 'XTEST', ''];
