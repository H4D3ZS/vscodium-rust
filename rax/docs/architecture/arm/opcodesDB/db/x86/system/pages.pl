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

PAGE BNDCL => {
  title => 'BNDCL-Check Lower Bound',
};

PAGE BNDCx => {
  title => 'BNDCU/BNDCN-Check Upper Bound',
};

PAGE BNDLDX => {
  title => 'BNDLDX-Load Extended Bounds Using Address Translation',
};

PAGE BNDMK => {
  title => 'BNDMK-Make Bounds',
};

PAGE BNDMOV => {
  title => 'BNDMOV-Move Bounds',
};

PAGE BNDSTX => {
  title => 'BNDSTX-Store Extended Bounds Using Address Translation',
};

PAGE CLAC => {
  title => 'CLAC-Clear AC Flag in EFLAGS Register',
};

PAGE CLDEMOTE => {
  title => 'CLDEMOTE-Cache Line Demote',
};

PAGE CLFLUSH => {
  title => 'CLFLUSH-Flush Cache Line',
};

PAGE CLFLUSHOPT => {
  title => 'CLFLUSHOPT-Flush Cache Line Optimized',
};

PAGE CLRSSBSY => {
  title => 'CLRSSBSY-Clear Busy Flag in a Supervisor Shadow Stack Token',
};

PAGE CLWB => {
  title => 'CLWB-Cache Line Write Back',
};

PAGE ENCLS => {
  title => 'ENCLS',
};

PAGE ENCLU => {
  title => 'ENCLU',
};

PAGE ENCLV => {
  title => 'ENCLV',
};

PAGE ENDBR32 => {
  title => 'ENDBR32-Terminate an Indirect Branch in 32-bit and Compatibility Mode',
};

PAGE ENDBR64 => {
  title => 'ENDBR64-Terminate an Indirect Branch in 64-bit Mode',
};

PAGE ENQCMD => {
  title => 'ENQCMD',
};

PAGE ENQCMDS => {
  title => 'ENQCMDS',
};

PAGE GETSEC => {
  title => 'GETSEC',
};

PAGE INCSSPx => {
  title => 'INCSSPD/INCSSPQ-Increment Shadow Stack Pointer',
};

PAGE INVEPT => {
  title => 'INVEPT-Invalidate Translations Derived from EPT',
};

PAGE INVPCID => {
  title => 'INVPCID-Invalidate Process-Context Identifier',
};

PAGE INVVPID => {
  title => 'INVVPID-Invalidate Translations Based on VPID',
};

PAGE MONITOR => {
  title => 'MONITOR-Set Up Monitor Address',
};

PAGE MWAIT => {
  title => 'MWAIT-Monitor Wait',
};

PAGE PTWRITE => {
  title => 'PTWRITE-Write Data to a Processor Trace Packet',
};

PAGE RDPID => {
  title => 'RDPID-Read Processor ID',
};

PAGE RDPKRU => {
  title => 'RDPKRU-Read Protection Key Rights for User Pages',
};

PAGE RDRAND => {
  title => 'RDRAND-Read Random Number',
};

PAGE RDSEED => {
  title => 'RDSEED-Read Random SEED',
};

PAGE RDSSPx => {
  title => 'RDSSPD/RDSSPQ-Read Shadow Stack Pointer',
};

PAGE RDTSCP => {
  title => 'RDTSCP-Read Time-Stamp Counter and Processor ID',
};

PAGE RDxSBASE => {
  title => 'RDFSBASE/RDGSBASE-Read FS/GS Segment Base',
};

PAGE RSTORSSP => {
  title => 'RSTORSSP-Restore Saved Shadow Stack Pointer',
};

PAGE SAVEPREVSSP => {
  title => 'SAVEPREVSSP-Save Previous Shadow Stack Pointer',
};

PAGE SETSSBSY => {
  title => 'SETSSBSY-Mark Shadow Stack Busy',
};

PAGE STAC => {
  title => 'STAC-Set AC Flag in EFLAGS Register',
};

PAGE TPAUSE => {
  title => 'TPAUSE-Timed PAUSE',
};

PAGE UMONITOR => {
  title => 'UMONITOR-User Level Set Up Monitor Address',
};

PAGE UMWAIT => {
  title => 'UMWAIT-User Level Monitor Wait',
};

PAGE VMCALL => {
  title => 'VMCALL-Call to VM Monitor',
};

PAGE VMCLEAR => {
  title => 'VMCLEAR-Clear Virtual-Machine Control Structure',
};

PAGE VMFUNC => {
  title => 'VMFUNC-Invoke VM function',
};

PAGE VMLAUNCH_RESUME => {
  title => 'VMLAUNCH/VMRESUME-Launch/Resume Virtual Machine',
};

PAGE VMPTRLD => {
  title => 'VMPTRLD-Load Pointer to Virtual-Machine Control Structure',
};

PAGE VMPTRST => {
  title => 'VMPTRST-Store Pointer to Virtual-Machine Control Structure',
};

PAGE VMREAD => {
  title => 'VMREAD-Read Field from Virtual-Machine Control Structure',
};

PAGE VMWRITE => {
  title => 'VMWRITE-Write Field to Virtual-Machine Control Structure',
};

PAGE VMXOFF => {
  title => 'VMXOFF-Leave VMX Operation',
};

PAGE VMXON => {
  title => 'VMXON-Enter VMX Operation',
};

PAGE WRPKRU => {
  title => 'WRPKRU-Write Data to User Page Key Register',
};

PAGE WRSSx => {
  title => 'WRSSD/WRSSQ-Write to Shadow Stack',
};

PAGE WRUSSx => {
  title => 'WRUSSD/WRUSSQ-Write to User Shadow Stack',
};

PAGE WRxSBASE => {
  title => 'WRFSBASE/WRGSBASE-Write FS/GS Segment Base',
};

PAGE XABORT => {
  title => 'XABORT-Transactional Abort',
};

PAGE XBEGIN => {
  title => 'XBEGIN-Transactional Begin',
};

PAGE XEND => {
  title => 'XEND-Transactional End',
};

PAGE XGETBV => {
  title => 'XGETBV-Get Value of Extended Control Register',
};

PAGE XRSTOR => {
  title => 'XRSTOR-Restore Processor Extended States',
};

PAGE XRSTORS => {
  title => 'XRSTORS-Restore Processor Extended States Supervisor',
};

PAGE XSAVE => {
  title => 'XSAVE-Save Processor Extended States',
};

PAGE XSAVEC => {
  title => 'XSAVEC-Save Processor Extended States with Compaction',
};

PAGE XSAVEOPT => {
  title => 'XSAVEOPT-Save Processor Extended States Optimized',
};

PAGE XSAVES => {
  title => 'XSAVES-Save Processor Extended States Supervisor',
};

PAGE XSETBV => {
  title => 'XSETBV-Set Extended Control Register',
};

PAGE XTEST => {
  title => 'XTEST-Test If In Transactional Execution',
};

