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
ENCODING BNDCL_ro => {
  diagram    => 'MAP=0f MOD=MEM MPX=1 MR=1 OP=0x1a P66=0 PF2=0 PF3=1',
  extensions => 'MPX',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=MPX',
  tags       => 'page=BNDCL',
};

ENCODING BNDCL_romx => {
  diagram    => 'MAP=0f MOD=REG MPX=1 MR=1 OP=0x1a P66=0 PF2=0 PF3=1',
  extensions => 'MPX',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=MPX',
  tags       => 'page=BNDCL',
};


# BNDCU/BNDCN-Check Upper Bound.
ENCODING BNDCN_ro => {
  diagram    => 'MAP=0f MOD=MEM MPX=1 MR=1 OP=0x1b P66=0 PF2=1 PF3=0',
  extensions => 'MPX',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=MPX',
  tags       => 'page=BNDCx',
};

ENCODING BNDCN_romx => {
  diagram    => 'MAP=0f MOD=REG MPX=1 MR=1 OP=0x1b P66=0 PF2=1 PF3=0',
  extensions => 'MPX',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=MPX',
  tags       => 'page=BNDCx',
};

ENCODING BNDCU_ro => {
  diagram    => 'MAP=0f MOD=MEM MPX=1 MR=1 OP=0x1a P66=0 PF2=1 PF3=0',
  extensions => 'MPX',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=MPX',
  tags       => 'page=BNDCx',
};

ENCODING BNDCU_romx => {
  diagram    => 'MAP=0f MOD=REG MPX=1 MR=1 OP=0x1a P66=0 PF2=1 PF3=0',
  extensions => 'MPX',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=MPX',
  tags       => 'page=BNDCx',
};


# BNDLDX-Load Extended Bounds Using Address Translation.
ENCODING BNDLDX => {
  diagram    => 'MAP=0f MOD=MEM MPX=1 MR=1 OP=0x1a P66=0 PF2=0 PF3=0',
  extensions => 'MPX',
  categories => 'SYSTEM',
  metadata   => 'isa=MPX',
  tags       => 'page=BNDLDX',
};


# BNDMK-Make Bounds.
ENCODING BNDMK => {
  diagram    => 'MAP=0f MOD=MEM MPX=1 MR=1 OP=0x1b P66=0 PF2=0 PF3=1',
  extensions => 'MPX',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=MPX',
  tags       => 'page=BNDMK',
};


# BNDMOV-Move Bounds.
ENCODING BNDMOV_mxro => {
  diagram    => 'MAP=0f MPX=1 MR=1 OP=0x1b P66=1 PF2=0 PF3=0',
  extensions => 'MPX',
  categories => 'SYSTEM',
  metadata   => 'isa=MPX',
  tags       => 'page=BNDMOV',
};

ENCODING BNDMOV_romx => {
  diagram    => 'MAP=0f MPX=1 MR=1 OP=0x1a P66=1 PF2=0 PF3=0',
  extensions => 'MPX',
  categories => 'SYSTEM',
  metadata   => 'isa=MPX',
  tags       => 'page=BNDMOV',
};


# BNDSTX-Store Extended Bounds Using Address Translation.
ENCODING BNDSTX => {
  diagram    => 'MAP=0f MOD=MEM MPX=1 MR=1 OP=0x1b P66=0 PF2=0 PF3=0',
  extensions => 'MPX',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=MPX',
  tags       => 'page=BNDSTX',
};


# CLAC-Clear AC Flag in EFLAGS Register.
ENCODING CLAC => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=1 RM=2',
  iflags     => 'ac=Z',
  extensions => 'SMAP',
  categories => 'SYSTEM',
  metadata   => 'isa=SMAP cpl=RING0',
  tags       => 'page=CLAC',
};


# CLDEMOTE-Cache Line Demote.
ENCODING CLDEMOTE => {
  diagram    => 'CLDEMOTE=1 MAP=0f MOD=MEM MR=1 OP=0x1c P66=0 PF2=0 PF3=0 REG=0',
  extensions => 'CLDEMOTE',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=CLDEMOTE',
  tags       => 'page=CLDEMOTE',
};


# CLFLUSH-Flush Cache Line.
ENCODING CLFLUSH => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xae P66=0 PF2=0 PF3=0 REG=7',
  extensions => 'CLFSH',
  categories => 'SYSTEM',
  metadata   => 'isa=CLFSH',
  tags       => 'page=CLFLUSH',
};


# CLFLUSHOPT-Flush Cache Line Optimized.
ENCODING CLFLUSHOPT => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xae P66=1 PF2=0 PF3=0 REG=7',
  extensions => 'CLFLUSHOPT',
  categories => 'SYSTEM',
  metadata   => 'isa=CLFLUSHOPT',
  tags       => 'page=CLFLUSHOPT',
};


# CLRSSBSY-Clear Busy Flag in a Supervisor Shadow Stack Token.
ENCODING CLRSSBSY => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xae P66=0 PF2=0 PF3=1 REG=6',
  extensions => 'CET',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=CET',
  tags       => 'page=CLRSSBSY',
};


# CLWB-Cache Line Write Back.
ENCODING CLWB => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xae P66=1 PF2=0 PF3=0 REG=6',
  extensions => 'CLWB',
  categories => 'SYSTEM',
  metadata   => 'isa=CLWB',
  tags       => 'page=CLWB',
};


# ENCLS.
ENCODING ENCLS => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=1 RM=7',
  extensions => 'SGX',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=SGX cpl=RING0',
  tags       => 'page=ENCLS',
};


# ENCLU.
ENCODING ENCLU => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=2 RM=7',
  extensions => 'SGX',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=SGX',
  tags       => 'page=ENCLU',
};


# ENCLV.
ENCODING ENCLV => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=0 RM=0',
  extensions => 'SGX_ENCLV',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=SGX_ENCLV',
  tags       => 'page=ENCLV',
};


# ENDBR32-Terminate an Indirect Branch in 32-bit and Compatibility Mode.
ENCODING ENDBR32 => {
  diagram    => 'CET=1 MAP=0f MOD=REG MR=1 OP=0x1e PF2=0 PF3=1 REG=7 RM=3',
  extensions => 'CET',
  categories => 'SYSTEM',
  metadata   => 'isa=CET',
  tags       => 'page=ENDBR32',
};


# ENDBR64-Terminate an Indirect Branch in 64-bit Mode.
ENCODING ENDBR64 => {
  diagram    => 'CET=1 MAP=0f MOD=REG MR=1 OP=0x1e PF2=0 PF3=1 REG=7 RM=2',
  extensions => 'CET',
  categories => 'SYSTEM',
  metadata   => 'isa=CET',
  tags       => 'page=ENDBR64',
};


# ENQCMD.
ENCODING ENQCMD => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0xf8 P66=0 PF2=1 PF3=0',
  iflags     => 'af=Z cf=Z of=Z pf=Z sf=Z zf=W',
  extensions => 'ENQCMD',
  categories => 'SYSTEM',
  metadata   => 'isa=ENQCMD',
  tags       => 'page=ENQCMD',
};


# ENQCMDS.
ENCODING ENQCMDS => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0xf8 P66=0 PF2=0 PF3=1',
  iflags     => 'af=Z cf=Z of=Z pf=Z sf=Z zf=W',
  extensions => 'ENQCMD',
  categories => 'SYSTEM',
  metadata   => 'isa=ENQCMD',
  tags       => 'page=ENQCMDS',
};


# GETSEC.
ENCODING GETSEC => {
  diagram    => 'MAP=0f OP=0x37 P66=0 PF2=0 PF3=0',
  extensions => 'SMX',
  categories => 'SYSTEM',
  metadata   => 'isa=SMX',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=GETSEC',
};


# INCSSPD/INCSSPQ-Increment Shadow Stack Pointer.
ENCODING INCSSPx => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xae P66=0 PF2=0 PF3=1 REG=5',
  extensions => 'CET',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=CET',
  tags       => 'page=INCSSPx',
};


# INVEPT-Invalidate Translations Derived from EPT.
ENCODING INVEPT => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0x80 P66=1 PF2=0 PF3=0',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'VTX',
  categories => 'SYSTEM',
  metadata   => 'isa=VTX cpl=RING0',
  tags       => 'page=INVEPT',
};


# INVPCID-Invalidate Process-Context Identifier.
ENCODING INVPCID => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0x82 P66=1 PF2=0 PF3=0',
  extensions => 'INVPCID',
  categories => 'SYSTEM|OS',
  metadata   => 'isa=INVPCID cpl=RING0',
  tags       => 'page=INVPCID',
};


# INVVPID-Invalidate Translations Based on VPID.
ENCODING INVVPID => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0x81 P66=1 PF2=0 PF3=0',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'VTX',
  categories => 'SYSTEM',
  metadata   => 'isa=VTX cpl=RING0',
  tags       => 'page=INVVPID',
};


# MONITOR-Set Up Monitor Address.
ENCODING MONITOR => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=1 RM=0',
  extensions => 'MONITOR',
  categories => 'SYSTEM',
  metadata   => 'isa=MONITOR cpl=RING0',
  tags       => 'page=MONITOR',
};


# MWAIT-Monitor Wait.
ENCODING MWAIT => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=1 RM=1',
  extensions => 'MONITOR',
  categories => 'SYSTEM',
  metadata   => 'isa=MONITOR cpl=RING0',
  tags       => 'page=MWAIT',
};


# PTWRITE-Write Data to a Processor Trace Packet.
ENCODING PTWRITE => {
  diagram    => 'MAP=0f MR=1 OP=0xae P66=0 PF2=0 PF3=1 REG=4',
  extensions => 'PT',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=PT',
  tags       => 'page=PTWRITE',
};


# RDPID-Read Processor ID.
ENCODING RDPID => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xc7 P66=0 PF2=0 PF3=1 REG=7',
  extensions => 'RDPID',
  categories => 'SYSTEM',
  metadata   => 'isa=RDPID',
  tags       => 'page=RDPID',
};


# RDPKRU-Read Protection Key Rights for User Pages.
ENCODING RDPKRU => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=5 RM=6',
  extensions => 'PKU',
  categories => 'SYSTEM',
  metadata   => 'isa=PKU',
  tags       => 'page=RDPKRU',
};


# RDRAND-Read Random Number.
ENCODING RDRAND => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xc7 REG=6',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=Z',
  extensions => 'RDRAND',
  categories => 'SYSTEM|RANDOM',
  metadata   => 'isa=RDRAND',
  tags       => 'page=RDRAND',
};


# RDSEED-Read Random SEED.
ENCODING RDSEED => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xc7 PF2=0 PF3=0 REG=7',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=Z',
  extensions => 'RDSEED',
  categories => 'SYSTEM|RANDOM',
  metadata   => 'isa=RDSEED',
  tags       => 'page=RDSEED',
};


# RDSSPD/RDSSPQ-Read Shadow Stack Pointer.
ENCODING RDSSPx => {
  diagram    => 'CET=1 MAP=0f MOD=REG MR=1 OP=0x1e PF2=0 PF3=1 REG=1',
  extensions => 'CET',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=CET',
  tags       => 'page=RDSSPx',
};


# RDTSCP-Read Time-Stamp Counter and Processor ID.
ENCODING RDTSCP => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 REG=7 RM=1',
  extensions => 'RDTSCP',
  categories => 'SYSTEM',
  metadata   => 'isa=RDTSCP',
  tags       => 'page=RDTSCP',
};


# RDFSBASE/RDGSBASE-Read FS/GS Segment Base.
ENCODING RDFSBASE => {
  diagram    => 'MAP=0f MOD=REG MODE=64 MR=1 OP=0xae P66=0 PF2=0 PF3=1 REG=0',
  extensions => 'RDWRFSGS',
  categories => 'SYSTEM',
  metadata   => 'isa=RDWRFSGS',
  tags       => 'page=RDxSBASE',
};

ENCODING RDGSBASE => {
  diagram    => 'MAP=0f MOD=REG MODE=64 MR=1 OP=0xae P66=0 PF2=0 PF3=1 REG=1',
  extensions => 'RDWRFSGS',
  categories => 'SYSTEM',
  metadata   => 'isa=RDWRFSGS',
  tags       => 'page=RDxSBASE',
};


# RSTORSSP-Restore Saved Shadow Stack Pointer.
ENCODING RSTORSSP => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0x01 P66=0 PF2=0 PF3=1 REG=5',
  extensions => 'CET',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=CET',
  tags       => 'page=RSTORSSP',
};


# SAVEPREVSSP-Save Previous Shadow Stack Pointer.
ENCODING SAVEPREVSSP => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=1 REG=5 RM=2',
  extensions => 'CET',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=CET',
  tags       => 'page=SAVEPREVSSP',
};


# SETSSBSY-Mark Shadow Stack Busy.
ENCODING SETSSBSY => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=1 REG=5 RM=0',
  extensions => 'CET',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=CET',
  tags       => 'page=SETSSBSY',
};


# STAC-Set AC Flag in EFLAGS Register.
ENCODING STAC => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=1 RM=3',
  iflags     => 'ac=S',
  extensions => 'SMAP',
  categories => 'SYSTEM',
  metadata   => 'isa=SMAP cpl=RING0',
  tags       => 'page=STAC',
};


# TPAUSE-Timed PAUSE.
ENCODING TPAUSE => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xae P66=1 PF2=0 PF3=0 REG=6',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=Z',
  extensions => 'WAITPKG',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=WAITPKG',
  tags       => 'page=TPAUSE',
};


# UMONITOR-User Level Set Up Monitor Address.
ENCODING UMONITOR => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xae P66=0 PF2=0 PF3=1 REG=6',
  extensions => 'WAITPKG',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=WAITPKG',
  tags       => 'page=UMONITOR',
};


# UMWAIT-User Level Monitor Wait.
ENCODING UMWAIT => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xae P66=0 PF2=1 PF3=0 REG=6',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=Z',
  extensions => 'WAITPKG',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=WAITPKG',
  tags       => 'page=UMWAIT',
};


# VMCALL-Call to VM Monitor.
ENCODING VMCALL => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=0 RM=1',
  iflags     => 'ac=CW af=CW cf=CW df=CW id=CW _if=CW iopl=RCW nt=CW of=CW pf=CW rf=CW sf=CW tf=CW vif=CW vip=CW vm=R zf=CW',
  extensions => 'VTX',
  categories => 'SYSTEM',
  metadata   => 'isa=VTX',
  tags       => 'page=VMCALL',
};


# VMCLEAR-Clear Virtual-Machine Control Structure.
ENCODING VMCLEAR => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xc7 P66=1 PF2=0 PF3=0 REG=6',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'VTX',
  categories => 'SYSTEM',
  metadata   => 'isa=VTX cpl=RING0',
  tags       => 'page=VMCLEAR',
};


# VMFUNC-Invoke VM function.
ENCODING VMFUNC => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=2 RM=4',
  extensions => 'VMFUNC',
  categories => 'SYSTEM',
  metadata   => 'isa=VMFUNC',
  tags       => 'page=VMFUNC',
};


# VMLAUNCH/VMRESUME-Launch/Resume Virtual Machine.
ENCODING VMLAUNCH => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=0 RM=2',
  iflags     => 'ac=CW af=CW cf=CW df=CW id=CW _if=CW iopl=RCW nt=CW of=CW pf=CW rf=CW sf=CW tf=CW vif=CW vip=CW vm=R zf=CW',
  extensions => 'VTX',
  categories => 'SYSTEM',
  metadata   => 'isa=VTX cpl=RING0',
  tags       => 'page=VMLAUNCH_RESUME',
};

ENCODING VMRESUME => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=0 RM=3',
  iflags     => 'ac=CW af=CW cf=CW df=CW id=CW _if=CW iopl=RCW nt=CW of=CW pf=CW rf=CW sf=CW tf=CW vif=CW vip=CW vm=R zf=CW',
  extensions => 'VTX',
  categories => 'SYSTEM',
  metadata   => 'isa=VTX cpl=RING0',
  tags       => 'page=VMLAUNCH_RESUME',
};


# VMPTRLD-Load Pointer to Virtual-Machine Control Structure.
ENCODING VMPTRLD => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xc7 P66=0 PF2=0 PF3=0 REG=6',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'VTX',
  categories => 'SYSTEM',
  metadata   => 'isa=VTX cpl=RING0',
  tags       => 'page=VMPTRLD',
};


# VMPTRST-Store Pointer to Virtual-Machine Control Structure.
ENCODING VMPTRST => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xc7 P66=0 PF2=0 PF3=0 REG=7',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'VTX',
  categories => 'SYSTEM',
  metadata   => 'isa=VTX cpl=RING0',
  tags       => 'page=VMPTRST',
};


# VMREAD-Read Field from Virtual-Machine Control Structure.
ENCODING VMREAD => {
  diagram    => 'MAP=0f MR=1 OP=0x78 P66=0 PF2=0 PF3=0',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'VTX',
  categories => 'SYSTEM',
  metadata   => 'isa=VTX cpl=RING0',
  tags       => 'page=VMREAD',
};


# VMWRITE-Write Field to Virtual-Machine Control Structure.
ENCODING VMWRITE => {
  diagram    => 'MAP=0f MR=1 OP=0x79 P66=0 PF2=0 PF3=0',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'VTX',
  categories => 'SYSTEM',
  metadata   => 'isa=VTX cpl=RING0',
  tags       => 'page=VMWRITE',
};


# VMXOFF-Leave VMX Operation.
ENCODING VMXOFF => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=0 RM=4',
  iflags     => 'af=W cf=W of=W pf=W sf=W zf=W',
  extensions => 'VTX',
  categories => 'SYSTEM',
  metadata   => 'isa=VTX cpl=RING0',
  tags       => 'page=VMXOFF',
};


# VMXON-Enter VMX Operation.
ENCODING VMXON => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xc7 P66=0 PF2=0 PF3=1 REG=6',
  iflags     => 'af=Z cf=W of=Z pf=Z sf=Z zf=W',
  extensions => 'VTX',
  categories => 'SYSTEM',
  metadata   => 'isa=VTX cpl=RING0',
  docvars    => 'PROTECTED_MODE=1',
  tags       => 'page=VMXON',
};


# WRPKRU-Write Data to User Page Key Register.
ENCODING WRPKRU => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=5 RM=7',
  extensions => 'PKU',
  categories => 'SYSTEM',
  metadata   => 'isa=PKU',
  tags       => 'page=WRPKRU',
};


# WRSSD/WRSSQ-Write to Shadow Stack.
ENCODING WRSSx => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0xf6 P66=0 PF2=0 PF3=0',
  extensions => 'CET',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=CET',
  tags       => 'page=WRSSx',
};


# WRUSSD/WRUSSQ-Write to User Shadow Stack.
ENCODING WRUSSx => {
  diagram    => 'MAP=0f38 MOD=MEM MR=1 OP=0xf5 P66=1 PF2=0 PF3=0',
  extensions => 'CET',
  categories => 'SYSTEM|MISC',
  metadata   => 'isa=CET',
  tags       => 'page=WRUSSx',
};


# WRFSBASE/WRGSBASE-Write FS/GS Segment Base.
ENCODING WRFSBASE => {
  diagram    => 'MAP=0f MOD=REG MODE=64 MR=1 OP=0xae P66=0 PF2=0 PF3=1 REG=2',
  extensions => 'RDWRFSGS',
  categories => 'SYSTEM',
  metadata   => 'isa=RDWRFSGS',
  tags       => 'page=WRxSBASE',
};

ENCODING WRGSBASE => {
  diagram    => 'MAP=0f MOD=REG MODE=64 MR=1 OP=0xae P66=0 PF2=0 PF3=1 REG=3',
  extensions => 'RDWRFSGS',
  categories => 'SYSTEM',
  metadata   => 'isa=RDWRFSGS',
  tags       => 'page=WRxSBASE',
};


# XABORT-Transactional Abort.
ENCODING XABORT => {
  diagram    => 'MOD=REG MR=1 OP=0xc6 REG=7 RM=0',
  extensions => 'RTM',
  categories => 'SYSTEM|BRANCH|UNCONDITIONALLY',
  metadata   => 'isa=RTM',
  tags       => 'page=XABORT',
};


# XBEGIN-Transactional Begin.
ENCODING XBEGIN => {
  diagram    => 'MOD=REG MR=1 OP=0xc7 REG=7 RM=0',
  extensions => 'RTM',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY',
  metadata   => 'isa=RTM',
  tags       => 'page=XBEGIN',
};


# XEND-Transactional End.
ENCODING XEND => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=2 RM=5',
  extensions => 'RTM',
  categories => 'SYSTEM|BRANCH|CONDITIONALLY',
  metadata   => 'isa=RTM',
  tags       => 'page=XEND',
};


# XGETBV-Get Value of Extended Control Register.
ENCODING XGETBV => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=2 RM=0',
  extensions => 'XSAVE',
  categories => 'SYSTEM|OS',
  metadata   => 'isa=XSAVE',
  tags       => 'page=XGETBV',
};


# XRSTOR-Restore Processor Extended States.
ENCODING XRSTOR => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xae P66=0 PF2=0 PF3=0 REG=5',
  extensions => 'XSAVE',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|OS',
  metadata   => 'isa=XSAVE requires_alignment=1',
  docvars    => 'XMM_STATE_CW=1',
  tags       => 'page=XRSTOR',
};


# XRSTORS-Restore Processor Extended States Supervisor.
ENCODING XRSTORS => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xc7 P66=0 PF2=0 PF3=0 REG=3',
  extensions => 'XSAVES',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|OS',
  metadata   => 'isa=XSAVES cpl=RING0 requires_alignment=1',
  docvars    => 'XMM_STATE_W=1',
  tags       => 'page=XRSTORS',
};


# XSAVE-Save Processor Extended States.
ENCODING XSAVE => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xae P66=0 PF2=0 PF3=0 REG=4',
  extensions => 'XSAVE',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|OS',
  metadata   => 'isa=XSAVE requires_alignment=1',
  docvars    => 'XMM_STATE_R=1',
  tags       => 'page=XSAVE',
};


# XSAVEC-Save Processor Extended States with Compaction.
ENCODING XSAVEC => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xc7 P66=0 PF2=0 PF3=0 REG=4',
  extensions => 'XSAVEC',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|OS',
  metadata   => 'isa=XSAVEC requires_alignment=1',
  docvars    => 'XMM_STATE_R=1',
  tags       => 'page=XSAVEC',
};


# XSAVEOPT-Save Processor Extended States Optimized.
ENCODING XSAVEOPT => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xae P66=0 PF2=0 PF3=0 REG=6',
  extensions => 'XSAVEOPT',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|OS',
  metadata   => 'isa=XSAVEOPT requires_alignment=1',
  docvars    => 'XMM_STATE_R=1',
  tags       => 'page=XSAVEOPT',
};


# XSAVES-Save Processor Extended States Supervisor.
ENCODING XSAVES => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xc7 P66=0 PF2=0 PF3=0 REG=5 W=0',
  extensions => 'XSAVES',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM|OS',
  metadata   => 'isa=XSAVES cpl=RING0 requires_alignment=1',
  docvars    => 'XMM_STATE_R=1',
  tags       => 'page=XSAVES',
};

ENCODING XSAVES64 => {
  diagram    => 'MAP=0f MOD=MEM MR=1 OP=0xc7 P66=0 PF2=0 PF3=0 REG=5 W=1',
  extensions => 'XSAVES',
  exceptions => 'ALIGNMENT',
  categories => 'SYSTEM',
  metadata   => 'isa=XSAVES cpl=RING0 requires_alignment=1',
  docvars    => 'XMM_STATE_R=1',
  tags       => 'page=XSAVES',
};


# XSETBV-Set Extended Control Register.
ENCODING XSETBV => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=2 RM=1',
  extensions => 'XSAVE',
  categories => 'SYSTEM|OS',
  metadata   => 'isa=XSAVE cpl=RING0',
  tags       => 'page=XSETBV',
};


# XTEST-Test If In Transactional Execution.
ENCODING XTEST => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0x01 P66=0 PF2=0 PF3=0 REG=2 RM=6',
  iflags     => 'af=Z cf=Z of=Z pf=Z sf=Z zf=W',
  extensions => 'RTM',
  categories => 'SYSTEM|BITWISE|LOGICAL',
  metadata   => 'isa=RTM',
  tags       => 'page=XTEST',
};

