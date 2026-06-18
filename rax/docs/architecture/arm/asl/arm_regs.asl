////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
// Proprietary Notice
// This document is protected by copyright and other related rights
// and the practice or implementation of the information contained in
// this document may be protected by one or more patents or pending
// patent applications. No part of this document may be reproduced in any
// form by any means without the express prior written permission of
// Arm.No license, express or implied, by estoppel or otherwise to
// any intellectual property rights is granted by this document unless
// specifically stated.
// Your access to the information in this document is conditional upon
// your acceptance that you will not use or permit others to use the
// information for the purposes of determining whether implementations
// infringe any third party patents.
// THIS DOCUMENT IS PROVIDED “AS IS”. ARM PROVIDES NO REPRESENTATIONS
// AND NO WARRANTIES, EXPRESS, IMPLIED OR STATUTORY, INCLUDING, WITHOUT
// LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTABILITY, SATISFACTORY
// QUALITY, NON-INFRINGEMENT OR FITNESS FOR A PARTICULAR PURPOSE WITH
// RESPECT TO THE DOCUMENT. For the avoidance of doubt, Arm makes no
// representation with respect to, and has undertaken no analysis to
// identify or understand the scope and content of, patents, copyrights,
// trade secrets, or other rights.
// This document may include technical inaccuracies or typographical
// errors.
// TO THE EXTENT NOT PROHIBITED BY LAW, IN NO EVENT WILL ARM BE
// LIABLE FOR ANY DAMAGES, INCLUDING WITHOUT LIMITATION ANY DIRECT,
// INDIRECT, SPECIAL, INCIDENTAL, PUNITIVE, OR CONSEQUENTIAL DAMAGES,
// HOWEVER CAUSED AND REGARDLESS OF THE THEORY OF LIABILITY, ARISING OUT
// OF ANY USE OF THIS DOCUMENT, EVEN IF ARM HAS BEEN ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGES.
// This document consists solely of commercial items. You shall be
// responsible for ensuring that any use, duplication or disclosure of
// this document complies fully with any relevant export laws and
// regulations to assure that this document or any portion thereof is not
// exported, directly or indirectly, in violation of such export
// laws. Use of the word “partner” in reference to Arm’s customers is not
// intended to create or refer to any partnership relationship with any
// other company. Arm may make changes to this document at any time and
// without notice.
// If any of the provisions contained in these terms conflict with
// any of the provisions of any click through or signed written agreement
// covering this document with Arm, then the click through or signed
// written agreement prevails over and supersedes the conflicting
// provisions of these terms. This document may be translated into other
// languages for convenience, and you agree that if there is any conflict
// between the English version of this document and any translation, the
// terms of the English version of the Agreement shall prevail.
// The Arm corporate logo and words marked with ® or ™™
// are registered trademarks or trademarks of Arm Limited (or its
// subsidiaries) in the US and/or elsewhere. All rights reserved.  Other
// brands and names mentioned in this document may be the trademarks of
// their respective owners. Please follow Arm’s trademark usage
// guidelines athttp://www.arm.com/company/policies/trademarks.
// Copyright © 2018 Arm Limited (or its affiliates). All rights reserved.
// Arm Limited. Company 02557590 registered in England.
// 110 Fulbourn Road, Cambridge, England CB1 9NJ.
// LES-PRE-20349
////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////

// Interrupt Controller Virtual Active Priorities Group 1 Registers
array [0..3] of __register 32 {  } ICV_AP1R;

// Virtual Machine Deactivate Interrupt Register
__register 32 { 24:0 INTID } GICV_DIR;

// Performance Monitors Lock Status Register
__register 32 { 2:2 nTT, 1:1 SLK, 0:0 SLI } PMLSR;

// Set Non-secure SPI Pending Register
__register 32 { 12:0 INTID } GICD_SETSPI_NSR;

// Debug Breakpoint Value Registers
array [0..15] of __register 32 { 31:2 VA, 31:0 ContextID } DBGBVR;

// Multiprocessor Affinity Register
__register 32 { 31:31 M, 24:24 MT, 15:8 Aff1, 23:16 Aff2, 7:0 Aff0, 30:30 U } MPIDR;

// Auxiliary Control Register
__register 32 {  } ACTLR;

// Performance Monitors Machine Identification Register
__register 32 { 7:0 SLOTS } PMMIR;

// Virtual Nested Control Register
__register 64 { 63:53 RESS, 52:12 BADDR } VNCR_EL2;

// EL1 Software Thread ID Register
__register 64 {  } TPIDR_EL1;

// Jazelle ID Register
__register 32 {  } JIDR;

// Counter-timer Virtual Timer CompareValue
__register 64 { 63:0 CompareValue } CNTV_CVAL;

// Pointer Authentication Key B for Data (bits[63:0])
__register 64 {  } APDBKeyLo_EL1;

// Error Recovery Interrupt Configuration Register 2
__register 32 { 5:4 SH, 3:0 MemAttr, 6:6 NSMSI, 7:7 IRQEN } ERRERICR2;

// SVE Feature ID register 0
__register 64 { 3:0 SVEver } ID_AA64ZFR0_EL1;

// Activity Monitors Event Type Registers 0
array [0..15] of __register 64 { 15:0 evtCount } AMEVTYPER0_EL0;

// Counter-timer Secure Physical Timer TimerValue register (EL2)
__register 32 { 31:0 TimerValue } CNTHPS_TVAL_EL2;

// Saved Program Status Register (EL3)
__register 32 { 7:7 I, 11:10 BTYPE, 23:23 UAO, 9:9 E, 15:10, 26:25 IT, 29:29 C, 31:31 N, 19:16 GE, 25:25 TCO, 9:9 D, 12:12 SSBS, 27:27 Q, 8:8 A, 28:28 V, 6:6 F, 21:21 SS, 22:22 PAN, 20:20 IL, 24:24 DIT, 5:5 T, 30:30 Z, 4:4, 4:4, 3:0, 3:0 M } SPSR_EL3;

// Architectural Feature Trap Register (EL3)
__register 32 { 8:8 EZ, 31:31 TCPAC, 30:30 TAM, 10:10 TFP, 20:20 TTA } CPTR_EL3;

// CPU Interface Deactivate Interrupt Register
__register 32 { 23:0 INTID } GICC_DIR;

// CTI Component Identification Register 2
__register 32 { 7:0 PRMBL_2 } CTICIDR2;

// Domain Access Control Register
__register 32 {  } DACR32_EL2;

// External Debug Processor Feature Register
__register 64 { 15:12 EL3, 19:16 FP, 7:4 EL1, 3:0 EL0, 47:44 AMU, 35:32 SVE, 27:24 GIC, 39:36 SEL2, 23:20 AdvSIMD, 11:8 EL2 } EDPFR;

// System Control Register (EL1)
__register 64 { 14:14 DZE, 9:9 UMA, 26:26 UCI, 28:28 nTLSMD, 12:12 I, 36:36 BT1, 35:35 BT0, 0:0 M, 41:40 TCF, 4:4 SA0, 30:30 EnIB, 37:37 ITFSB, 25:25 EE, 13:13 EnDB, 15:15 UCT, 24:24 E0E, 31:31 EnIA, 7:7 ITD, 6:6 nAA, 29:29 LSMAOE, 27:27 EnDA, 43:43 ATA, 19:19 WXN, 44:44 DSSBS, 39:38 TCF0, 21:21 IESB, 18:18 nTWE, 2:2 C, 23:23 SPAN, 22:22 EIS, 11:11 EOS, 20:20 TSCXT, 8:8 SED, 10:10 EnRCTX, 3:3 SA, 42:42 ATA0, 5:5 CP15BEN, 16:16 nTWI, 1:1 A } SCTLR_EL1;

// CONTEXTIDR_EL2 Sample Register
__register 32 { 31:0 CONTEXTIDR_EL2 } PMCID2SR;

// MPAM Features Secure Identification Register
__register 32 { 15:0 S_PARTID_MAX, 23:16 S_PMG_MAX } MPAMF_SIDR;

// Selected Error Record Feature Register 2
__register 32 {  } ERXFR2;

// Performance Monitors Event Type Registers
array [0..30] of __register 32 { 28:28 NSU, 27:27 NSH, 15:10, 9:0 evtCount, 25:25 MT, 30:30 U, 29:29 NSK, 31:31 P } PMEVTYPER;

// Interrupt Controller Virtual Highest Priority Pending Interrupt Register 1
__register 32 { 23:0 INTID } ICV_HPPIR1_EL1;

// Interrupt Controller Virtual Interrupt Group 0 Enable register
__register 32 { 0:0 Enable } ICV_IGRPEN0_EL1;

// Debug OS Lock Access Register
__register 32 {  } DBGOSLAR;

// Software Generated Interrupt Register
__register 32 { 23:16 CPUTargetList, 15:15 NSATT, 3:0 INTID, 25:24 TargetListFilter } GICD_SGIR;

// External Debug Event Status Register
__register 32 { 0:0 OSUC, 1:1 RC, 2:2 SS } EDESR;

// Virtual Machine Error Reporting Status Register
__register 32 { 1:1 WRD, 0:0 RRD, 2:2 RWOD, 3:3 WROD } GICV_STATUSR;

// Hyp Auxiliary Data Fault Status Register
__register 32 {  } HADFSR;

// Interrupt Controller Hyp Control Register
__register 32 { 5:5 VGrp0DIE, 14:14 TDIR, 4:4 VGrp0EIE, 3:3 NPIE, 1:1 UIE, 10:10 TC, 0:0 En, 13:13 TSEI, 31:27 EOIcount, 12:12 TALL1, 2:2 LRENPIE, 7:7 VGrp1DIE, 11:11 TALL0, 6:6 VGrp1EIE } ICH_HCR;

// Exception Syndrome Register (EL3)
__register 32 { 19:12 imm8, 3:1 AM, 10:10 FnV, 19:17 Opc2, 4:4 Offset, 23:22 SAS, 8:8 CM, 2:2 OFF, 6:6 WnR, 15:0 Comment, 20:16 SRT, 24:24 CV, 6:6 EX, 19:16 Opc1, 0:0 ERETA, 12:11 SET, 1:0 BTYPE, 1:1 ERET, 15:0 imm16, 19:17 Op2, 7:7 S1PTW, 4:4 IXF, 13:13 IESB, 14:10 Rt2, 0:0 TI, 4:1 CRm, 24:24 ISV, 9:5 Rt, 21:20 Op0, 16:14 Op1, 24:24 IDS, 14:14 AR, 5:0 DFSC, 21:21 SSE, 7:7 IDF, 23:20 COND, 25:25 IL, 13:13 VNCR, 10:8 VECITR, 9:9 EA, 23:23 TFV, 15:15 SF, 3:3 UFF, 5:0 IFSC, 0:0 IOF, 13:10 CRn, 19:19 CCKNOWNPASS, 9:5 Rn, 31:26 EC, 12:10 AET, 0:0 Direction, 1:1 DZF } ESR_EL3;

// Profiling Buffer Limit Address Register
__register 64 { 63:12 LIMIT, 2:1 FM, 0:0 E } PMBLIMITR_EL1;

// Interrupt Controller Virtual Machine Control Register
__register 32 { 20:18 VBPR1, 9:9 VEOIM, 23:21 VBPR0, 31:24 VPMR, 1:1 VENG1, 4:4 VCBPR, 3:3 VFIQEn, 2:2 VAckCtl, 0:0 VENG0 } ICH_VMCR_EL2;

// Interrupt Set-Enable Registers
array [1..2] of __register 32 {  } GICR_ISENABLERE;

// Performance Monitors Event Counter Selection Register
__register 32 { 4:0 SEL } PMSELR_EL0;

// Interrupt Controller Hyp Active Priorities Group 0 Registers
array [0..3] of __register 32 {  } ICH_AP0R;

// Selected Error Record Feature Register
__register 32 {  } ERXFR;

// Hyp Data Fault Address Register
__register 32 {  } HDFAR;

// Interrupt Controller Maintenance Interrupt State Register
__register 32 { 2:2 LRENP, 3:3 NP, 1:1 U, 0:0 EOI, 6:6 VGrp1E, 5:5 VGrp0D, 4:4 VGrp0E, 7:7 VGrp1D } ICH_MISR_EL2;

// Activity Monitors Count Enable Set Register 0
__register 32 {  } AMCNTENSET0;

// Virtualization Multiprocessor ID Register
__register 64 { 24:24 MT, 23:16 Aff2, 7:0 Aff0, 15:8 Aff1, 39:32 Aff3, 30:30 U } VMPIDR_EL2;

// Counter-timer Kernel Control register
__register 32 { 1:1 PL0VCTEN, 9:9 PL0PTEN, 8:8 PL0VTEN, 7:4 EVNTI, 2:2 EVNTEN, 0:0 PL0PCTEN, 3:3 EVNTDIR } CNTKCTL;

// Counter-timer Physical Timer TimerValue register
__register 32 { 31:0 TimerValue } CNTP_TVAL_EL0;

// Activity Monitors Count Enable Set Register 1
__register 32 {  } AMCNTENSET1;

// Selected Error Record Miscellaneous Register 0
__register 32 {  } ERXMISC0;

// Interrupt Controller System Register Enable register (EL1)
__register 32 { 1:1 DFB, 0:0 SRE, 2:2 DIB } ICC_SRE_EL1;

// Counter-timer Secure Virtual Timer Control Register (EL2)
__register 32 { 0:0 ENABLE, 2:2 ISTATUS, 1:1 IMASK } CNTHVS_CTL;

// Pointer Authentication Key A for Instruction (bits[63:0])
__register 64 {  } APIAKeyLo_EL1;

// Performance Monitors Common Event Identification register 1
__register 64 {  } PMCEID1_EL0;

// Counter-timer Virtual Timer Control register
__register 32 { 2:2 ISTATUS, 1:1 IMASK, 0:0 ENABLE } CNTV_CTL_EL0;

// Interrupt Controller VGIC Type Register
__register 32 { 28:26 PREbits, 21:21 A3V, 31:29 PRIbits, 22:22 SEIS, 19:19 TDS, 20:20 nV4, 25:23 IDbits, 4:0 ListRegs } ICH_VTR_EL2;

// Selected Error Record Address Register
__register 64 {  } ERXADDR_EL1;

// Translation Table Base Register 0
__register 64 { 31:7 TTB0, 4:3 RGN, 55:48 ASID, 0:0, 6:6 IRGN, 5:5 NOS, 2:2 IMP, 47:1 BADDR, 1:1 S, 0:0 CnP } TTBR0;

// Hyp Translation Table Base Register
__register 64 { 47:1 BADDR, 0:0 CnP } HTTBR;

// CPU Interface Status Register
__register 32 { 4:4 ASV, 0:0 RRD, 1:1 WRD, 3:3 WROD, 2:2 RWOD } GICC_STATUSR;

// EL0 Read/Write Software Thread ID Register
__register 64 {  } TPIDR_EL0;

// ITS Command Queue Descriptor
__register 64 { 7:0 Size, 63:63 Valid, 55:53 OuterCache, 61:59 InnerCache, 51:12 Physical_Address, 11:10 Shareability } GITS_CBASER;

// Interrupt Controller Interrupt Acknowledge Register 0
__register 32 { 23:0 INTID } ICC_IAR0;

// MPAM Cache Maximum Capacity Partition Configuration Register
__register 32 { 15:0 MIN } MPAMCFG_MBW_MIN;

// Counter-timer Virtual Timer TimerValue register
__register 32 { 31:0 TimerValue } CNTV_TVAL_EL0;

// AArch32 Instruction Set Attribute Register 3
__register 32 { 19:16 TabBranch, 7:4 SIMD, 31:28 T32EE, 15:12 SynchPrim, 11:8 SVC, 27:24 TrueNOP, 3:0 Saturate, 23:20 T32Copy } ID_ISAR3_EL1;

// Performance Monitors Event Type Registers
array [0..30] of __register 64 { 29:29 NSK, 25:25 MT, 28:28 NSU, 26:26 M, 31:31 P, 24:24 SH, 15:10, 9:0 evtCount, 27:27 NSH, 30:30 U } PMEVTYPER_EL0;

// Activity Monitors Count Enable Set Register 0
__register 64 {  } AMCNTENSET0_EL0;

// External Debug Component Identification Register 1
__register 32 { 3:0 PRMBL_1, 7:4 CLASS } EDCIDR1;

// Interrupt Controller VGIC Type Register
__register 32 { 28:26 PREbits, 31:29 PRIbits, 4:0 ListRegs, 19:19 TDS, 25:23 IDbits, 22:22 SEIS, 21:21 A3V, 20:20 nV4 } ICH_VTR;

// Performance Monitors Software Increment register
__register 32 {  } PMSWINC_EL0;

// Redistributor Synchronize Register
__register 32 { 0:0 Busy } GICR_SYNCR;

// Fault Address Register (EL3)
__register 64 {  } FAR_EL3;

// Hyp Configuration Register 2
__register 32 { 18:18 TICAB, 5:5 TEA, 22:22 TTLBIS, 1:1 ID, 20:20 TOCU, 17:17 TID4, 0:0 CD, 4:4 TERR, 6:6 MIOCNCE } HCR2;

// Main ID Register
__register 32 { 19:16 Architecture, 3:0 Revision, 31:24 Implementer, 23:20 Variant, 15:4 PartNum } MIDR_EL1;

// External Debug Device Affinity register 0
__register 32 {  } EDDEVAFF0;

// EL0 Read-Only Software Thread ID Register
__register 64 {  } TPIDRRO_EL0;

// Activity Monitors User Enable Register
__register 32 { 0:0 EN } AMUSERENR;

// MPAM Memory Bandwidth Maximum Partition Configuration Register
__register 32 { 15:0 MAX, 31:31 HARDLIM } MPAMCFG_MBW_MAX;

// Translation Table Base Control Register 2
__register 32 { 16:16 HWU160, 17:17 HWU161, 14:14 HWU062, 15:15 HWU159, 12:12 HWU060, 18:18 HWU162, 10:10 HPD1, 13:13 HWU061, 9:9 HPD0, 11:11 HWU059 } TTBCR2;

// AArch64 Instruction Set Attribute Register 0
__register 64 { 47:44 DP, 35:32 SHA3, 31:28 RDM, 59:56 TLB, 39:36 SM3, 55:52 TS, 23:20 Atomic, 19:16 CRC32, 63:60 RNDR, 51:48 FHM, 15:12 SHA2, 43:40 SM4, 7:4 AES, 11:8 SHA1 } ID_AA64ISAR0_EL1;

// Activity Monitors Control Register
__register 64 { 10:10 HDBG } AMCR_EL0;

// Secure Debug Control Register
__register 32 { 18:18 STE, 15:14 SPD, 19:19 TTRF, 17:17 SPME, 23:23 SCCD, 21:21 EPMAD, 20:20 EDAD } SDCR;

// Redistributor LPI Pending Table Base Address Register
__register 64 { 58:56 OuterCache, 11:10 Shareability, 51:16 Physical_Address, 62:62 PTZ, 9:7 InnerCache } GICR_PENDBASER;

// Performance Monitors Selected Event Count Register
__register 32 {  } PMXEVCNTR_EL0;

// Debug Link Register
__register 32 {  } DLR;

// CTI Component Identification Register 1
__register 32 { 7:4 CLASS, 3:0 PRMBL_1 } CTICIDR1;

// MPAM Memory Bandwidth Usage Monitor Capture Register
__register 32 { 31:31 NRDY, 30:0 VALUE } MSMON_MBWU_CAPTURE;

// Interrupt Configuration Registers
array [0..63] of __register 32 {  } GICD_ICFGR;

// Critical Error Interrupt Configuration Register 2
__register 32 { 3:0 MemAttr, 6:6 NSMSI, 5:4 SH, 7:7 IRQEN } ERRCRICR2;

// Reset Vector Base Address Register
__register 32 {  } RVBAR;

// Profiling Buffer Status/syndrome Register
__register 64 { 5:0 BSC, 17:17 S, 31:26 EC, 19:19 DL, 16:16 COLL, 18:18 EA, 5:0 FSC } PMBSR_EL1;

// External Debug Component Identification Register 3
__register 32 { 7:0 PRMBL_3 } EDCIDR3;

// External Debug Processor Status Register
__register 32 { 8:8 SDAD, 0:0 PU, 2:2 R, 9:9 EPMAD, 7:7 EDAD, 4:4 HALTED, 10:10 SPMAD, 3:3 SR, 5:5 OSLK, 11:11 SDR, 1:1 SPD, 6:6 DLK } EDPRSR;

// EL2 Read/Write Software Context Number
__register 64 {  } SCXTNUM_EL2;

// Interrupt Group Modifier Registers (extended SPI range)
array [1..2] of __register 32 {  } GICD_IGRPMODRE;

// Performance Monitors Device ID register
__register 32 { 3:0 PCSample } PMDEVID;

// Interrupt Controller Virtual Running Priority Register
__register 32 { 7:0 Priority } ICV_RPR;

// Activity Monitors Count Enable Clear Register 1
__register 32 {  } AMCNTENCLR1;

// Device Affinity Register
__register 64 { 39:32 Aff3, 15:8 Aff1, 7:0 Aff0, 31:31 F0V, 30:30 U, 24:24 MT, 23:16 Aff2 } ERRDEVAFF;

// Auxiliary Data Fault Status Register
__register 32 {  } ADFSR;

// Exception Link Register (EL2)
__register 64 {  } ELR_EL2;

// Sampling Interval Counter Register
__register 64 { 31:0 COUNT, 63:56 ECOUNT } PMSICR_EL1;

// Reset Management Register (EL3)
__register 32 { 0:0 AA64, 1:1 RR } RMR_EL3;

// Activity Monitors Count Enable Set Register 1
__register 64 {  } AMCNTENSET1_EL0;

// AArch32 Instruction Set Attribute Register 2
__register 32 { 31:28 Reversal, 27:24 PSR_AR, 3:0 LoadStore, 23:20 MultU, 15:12 Mult, 11:8 MultiAccessInt, 7:4 MemHint, 19:16 MultS } ID_ISAR2_EL1;

// ITS Write Register
__register 64 { 0:0 Retry, 19:5 Offset } GITS_CWRITER;

// Interrupt Controller Hyp Active Priorities Group 1 Registers
array [0..3] of __register 32 {  } ICH_AP1R;

// Error Record Miscellaneous Register 0
__register 64 { 46:40 CECO, 39:39 OF, 39:39 OFR, 38:32 CECR, 47:47 OFO, 38:32 CEC } ERRMISC0;

// Performance Monitors Peripheral Identification Register 3
__register 32 { 3:0 CMOD, 7:4 REVAND } PMPIDR3;

// Statistical Profiling Control Register (EL2)
__register 64 { 6:6 PCT, 4:4 PA, 1:1 E2SPE, 0:0 E0HSPE, 3:3 CX, 5:5 TS } PMSCR_EL2;

// CPU Interface Aliased Binary Point Register
__register 32 { 2:0 Binary_Point } GICC_ABPR;

// CTI Trigger In Status register
__register 32 {  } CTITRIGINSTATUS;

// Virtual Machine Aliased Interrupt Acknowledge Register
__register 32 { 24:0 INTID } GICV_AIAR;

// Device Configuration Register
__register 32 { 15:0 NUM } ERRDEVID;

// PL0 Read-Only Software Thread ID Register
__register 32 {  } TPIDRURO;

// System Control Register (EL2)
__register 64 { 19:19 WXN, 6:6 nAA, 20:20 TSCXT, 22:22 EIS, 29:29 LSMAOE, 28:28 nTLSMD, 23:23 SPAN, 10:10 EnRCTX, 25:25 EE, 39:38 TCF0, 36:36 BT1, 7:7 ITD, 11:11 EOS, 4:4 SA0, 27:27 EnDA, 12:12 I, 8:8 SED, 43:43 ATA, 3:3 SA, 16:16 nTWI, 14:14 DZE, 1:1 A, 30:30 EnIB, 26:26 UCI, 24:24 E0E, 41:40 TCF, 35:35 BT0, 42:42 ATA0, 36:36 BT, 44:44 DSSBS, 31:31 EnIA, 2:2 C, 18:18 nTWE, 13:13 EnDB, 21:21 IESB, 15:15 UCT, 37:37 ITFSB, 0:0 M, 5:5 CP15BEN } SCTLR_EL2;

// Data Fault Address Register
__register 32 {  } DFAR;

// Counter-timer Virtual Count register
__register 64 {  } CNTVCT_EL0;

// Interrupt Clear-Pending Registers
array [1..2] of __register 32 {  } GICR_ICPENDRE;

// Floating-Point System ID register
__register 32 { 31:24 Implementer, 23:23 SW, 15:8 PartNum, 22:16 Subarchitecture, 3:0 Revision, 7:4 Variant } FPSID;

// ITS Read Register
__register 64 { 0:0 Stalled, 19:5 Offset } GITS_CREADR;

// CTI Channel In Status register
__register 32 {  } CTICHINSTATUS;

// Activity Monitors Device Type Register
__register 32 { 3:0 MAJOR, 7:4 SUB } AMDEVTYPE;

// Interrupt Controller Highest Priority Pending Interrupt Register 0
__register 32 { 23:0 INTID } ICC_HPPIR0;

// MPAM0 Register (EL1)
__register 64 { 47:40 PMG_D, 39:32 PMG_I, 15:0 PARTID_I, 31:16 PARTID_D } MPAM0_EL1;

// CTI Application Pulse register
__register 32 {  } CTIAPPPULSE;

// Performance Monitors Peripheral Identification Register 1
__register 32 { 3:0 PART_1, 7:4 DES_0 } PMPIDR1;

// Tag Fail Status Register (EL0).
__register 64 { 0:0 TF0, 1:1 TF1 } TFSRE0_EL1;

// AArch64 Memory Model Feature Register 2
__register 64 { 39:36 IDS, 55:52 BBM, 27:24 NV, 63:60 E0PD, 23:20 CCIDX, 35:32 AT, 43:40 FWB, 15:12 IESB, 11:8 LSM, 31:28 ST, 51:48 TTL, 3:0 CnP, 7:4 UAO, 59:56 EVT, 19:16 VARange } ID_AA64MMFR2_EL1;

// Performance Monitors Interrupt Enable Clear register
__register 32 { 31:31 C } PMINTENCLR;

// AArch64 Memory Model Feature Register 1
__register 64 { 19:16 LO, 11:8 VH, 7:4 VMIDBits, 31:28 XNX, 27:24 SpecSEI, 3:0 HAFDBS, 15:12 HPDS, 23:20 PAN } ID_AA64MMFR1_EL1;

// Auxiliary ID Register
__register 32 {  } AIDR;

// LORegionID (EL1)
__register 64 { 23:16 LD, 7:0 LR } LORID_EL1;

// Counter Frequency ID
__register 32 { 31:0 Frequency } CNTFID0;

// Auxiliary Memory Attribute Indirection Register 0
__register 32 {  } AMAIR0;

// AArch64 Memory Model Feature Register 0
__register 64 { 35:32 TGran16_2, 47:44 ExS, 43:40 TGran4_2, 7:4 ASIDBits, 3:0 PARange, 39:36 TGran64_2, 27:24 TGran64, 23:20 TGran16, 19:16 BigEndEL0, 15:12 SNSMem, 11:8 BigEnd, 31:28 TGran4 } ID_AA64MMFR0_EL1;

// Translation Table Base Register 1 (EL1)
__register 64 { 47:1 BADDR, 63:48 ASID, 0:0 CnP } TTBR1_EL1;

// Monitor Debug System Control Register
__register 32 { 30:30 RXfull, 27:27 RXO, 26:26 TXU, 14:14 HDE, 19:19 SC2, 31:31 TFO, 6:6 ERR, 0:0 SS, 23:22 INTdis, 29:29 TXfull, 12:12 TDCC, 15:15 MDE, 13:13 KDE, 21:21 TDA } MDSCR_EL1;

// MPAM Features Cache Capacity Partitioning ID register
__register 32 { 5:0 CMAX_WD } MPAMF_CCAP_IDR;

// Application Program Status Register
__register 32 { 31:31 N, 30:30 Z, 27:27 Q, 19:16 GE, 28:28 V, 29:29 C } APSR;

// Activity Monitors Peripheral Identification Register 2
__register 32 { 7:4 REVISION, 3:3 JEDEC, 2:0 DES_1 } AMPIDR2;

// Interrupt Controller Alias Software Generated Interrupt Group 1 Register
__register 64 { 39:32 Aff2, 23:16 Aff1, 47:44 RS, 27:24 INTID, 40:40 IRM, 15:0 TargetList, 55:48 Aff3 } ICC_ASGI1R_EL1;

// Fault Address Register (EL2)
__register 64 {  } FAR_EL2;

// Performance Monitors Component Identification Register 3
__register 32 { 7:0 PRMBL_3 } PMCIDR3;

// Activity Monitors Counter Group Configuration Register
__register 64 { 15:8 CG1NC, 7:0 CG0NC } AMCGCR_EL0;

// Activity Monitors Implementation Identification Register
__register 32 { 31:20 ProductID, 15:12 Revision, 11:0 Implementer, 19:16 Variant } AMIIDR;

// Selected Error Record Miscellaneous Register 6
__register 32 {  } ERXMISC6;

// Holds the priority of the corresponding interrupt for each extended SPI supported by the GIC.
array [0..254] of __register 32 { 31:24 Priority_offset_3B, 7:0 Priority_offset_0B, 15:8 Priority_offset_1B, 23:16 Priority_offset_2B } GICD_IPRIORITYRE;

// Interrupt Clear-Enable Register 0
__register 32 {  } GICR_ICENABLER0;

// Performance Monitors Peripheral Identification Register 0
__register 32 { 7:0 PART_0 } PMPIDR0;

// Interrupt Set-Enable Registers
array [0..31] of __register 32 {  } GICD_ISENABLERE;

// Interrupt Controller Monitor Interrupt Group 1 Enable register
__register 32 { 0:0 EnableGrp1NS, 1:1 EnableGrp1S } ICC_MGRPEN1;

// Memory Attribute Indirection Register (EL2)
__register 64 {  } MAIR_EL2;

// Interrupt configuration registers
array [2..5] of __register 32 {  } GICR_ICFGRE;

// Virtual Machine Aliased End Of Interrupt Register
__register 32 { 24:0 INTID } GICV_AEOIR;

// CTI Device ID register 0
__register 32 { 21:16 NUMCHAN, 4:0 EXTMUXNUM, 13:8 NUMTRIG, 25:24 INOUT } CTIDEVID;

// Report maximum PARTID and PMG Register
__register 32 { 15:0 PARTIDmax, 23:16 PMGmax } GITS_MPAMIDR;

// Interrupt Clear-Active Registers (extended SPI range)
array [0..31] of __register 32 {  } GICD_ICACTIVERE;

// Pointer Authentication Key B for Instruction (bits[127:64])
__register 64 {  } APIBKeyHi_EL1;

// MPAM Memory Bandwidth Proportional Stride Partition Configuration Register
__register 32 { 15:0 STRIDEM1, 31:31 EN } MPAMCFG_MBW_PROP;

// AArch32 Auxiliary Feature Register 0
__register 32 {  } ID_AFR0_EL1;

// Debug Claim Tag Set register
__register 32 { 7:0 CLAIM } DBGCLAIMSET_EL1;

// Counter-timer Physical Timer CompareValue register
__register 64 { 63:0 CompareValue } CNTP_CVAL_EL0;

// Interrupt Set-Active Registers (extended SPI range)
array [0..31] of __register 32 {  } GICD_ISACTIVERE;

// Performance Monitors Device Architecture register
__register 32 { 31:21 ARCHITECT, 19:16 REVISION, 20:20 PRESENT, 15:0 ARCHID } PMDEVARCH;

// Auxiliary Fault Status Register 1 (EL3)
__register 32 {  } AFSR1_EL3;

// Counter-timer Hypervisor Physical Timer Control register
__register 32 { 2:2 ISTATUS, 0:0 ENABLE, 1:1 IMASK } CNTHP_CTL_EL2;

// AArch32 Memory Model Feature Register 3
__register 32 { 15:12 MaintBcst, 7:4 CMaintSW, 11:8 BPMaint, 31:28 Supersec, 19:16 PAN, 23:20 CohWalk, 3:0 CMaintVA, 27:24 CMemSz } ID_MMFR3_EL1;

// Counter-timer Secure Virtual Timer CompareValue Register (EL2)
__register 64 { 63:0 CompareValue } CNTHVS_CVAL;

// Performance Monitors Event Counter Selection Register
__register 32 { 4:0 SEL } PMSELR;

// MPAM Memory Bandwdith Usage Monitor Register
__register 32 { 31:31 NRDY, 30:0 VALUE } MSMON_MBWU;

// Memory Model Feature Register 3
__register 32 { 19:16 PAN, 27:24 CMemSz, 15:12 MaintBcst, 31:28 Supersec, 11:8 BPMaint, 3:0 CMaintVA, 7:4 CMaintSW, 23:20 CohWalk } ID_MMFR3;

// MPAM Virtual PARTID Mapping Register 7
__register 64 { 47:32 PhyPARTID30, 31:16 PhyPARTID29, 15:0 PhyPARTID28, 63:48 PhyPARTID31 } MPAMVPM7_EL2;

// Reset Vector Base Address Register (if EL2 and EL3 not implemented)
__register 64 {  } RVBAR_EL1;

// Activity Monitors Device Affinity Register 1
__register 32 {  } AMDEVAFF1;

// Interrupt Controller Software Generated Interrupt Group 1 Register
__register 64 { 27:24 INTID, 40:40 IRM, 55:48 Aff3, 39:32 Aff2, 15:0 TargetList, 47:44 RS, 23:16 Aff1 } ICC_SGI1R_EL1;

// MPAM Cache Storage Usage Monitor Capture Register
__register 32 { 31:31 NRDY, 30:0 VALUE } MSMON_CSU_CAPTURE;

// AArch32 Memory Model Feature Register 4
__register 32 { 7:4 AC2, 3:0 SpecSEI, 27:24 CCIDX, 19:16 HPDS, 11:8 XNX, 31:28 EVT, 23:20 LSM, 15:12 CnP } ID_MMFR4_EL1;

// Counter ID registers
array [0..11] of __register 32 {  } CounterID;

// Interrupt Controller Binary Point Register 1
__register 32 { 2:0 BinaryPoint } ICC_BPR1_EL1;

// Error Record Select Register
__register 32 { 15:0 SEL } ERRSELR;

// Pseudo-fault Generation Feature Register
array [0..65534] of __register 64 { 4:4 UEO, 10:10 PN, 5:5 DE, 3:3 UER, 29:29 SYN, 11:11 AV, 0:0 OF, 8:8 CI, 30:30 R, 9:9 ER, 7:6 CE, 2:2 UEU, 1:1 UC, 12:12 MV } ERRPFGF;

// CTI Application Trigger Clear register
__register 32 {  } CTIAPPCLEAR;

// Current Cache Size ID Register 2
__register 32 { 23:0 NumSets } CCSIDR2;

// Selected Error Record Miscellaneous Register 0
__register 64 {  } ERXMISC0_EL1;

// Pointer Authentication Key B for Instruction (bits[63:0])
__register 64 {  } APIBKeyLo_EL1;

// External Debug Auxiliary Control Register
__register 32 {  } EDACR;

// Distributor Control Register
__register 32 { 6:6 DS, 0:0 EnableGrp0, 1:1 EnableGrp1, 31:31 RWP, 1:1 EnableGrp1NS, 4:4 ARE_NS, 1:1 EnableGrp1A, 7:7 E1NWF, 2:2 EnableGrp1S, 4:4 ARE, 4:4 ARE_S } GICD_CTLR;

// Interrupt Controller Control Register
__register 32 { 10:8 PRIbits, 19:19 ExtRange, 15:15 A3V, 1:1 EOImode, 14:14 SEIS, 13:11 IDbits, 0:0 CBPR, 18:18 RSS, 6:6 PMHE } ICC_CTLR;

// CTI Peripheral Identification Register 2
__register 32 { 7:4 REVISION, 3:3 JEDEC, 2:0 DES_1 } CTIPIDR2;

// CTI Claim Tag Clear register
__register 32 {  } CTICLAIMCLR;

// Counter-timer Physical Timer CompareValue
__register 64 { 63:0 CompareValue } CNTP_CVAL;

// Pointer Authentication Key B for Data (bits[127:64])
__register 64 {  } APDBKeyHi_EL1;

// Interrupt Group Registers
array [0..31] of __register 32 {  } GICD_IGROUPR;

// MPAM Memory Bandwidth Partitioning Window Width Configuration Register
__register 32 { 23:8 US_INT, 7:0 US_FRAC } MPAMCFG_MBW_WINWD;

// Interrupt Controller Binary Point Register 0
__register 32 { 2:0 BinaryPoint } ICC_BPR0_EL1;

// Memory Attribute Indirection Register 1
__register 32 {  } MAIR1;

// Current Cache Size ID Register
__register 32 { 2:0 LineSize, 27:13 NumSets, 12:3 Associativity } CCSIDR;

// Context ID Register (EL1)
__register 32 { 31:0 PROCID } CONTEXTIDR_EL1;

// Interrupt Controller Empty List Register Status Register
__register 32 {  } ICH_ELRSR_EL2;

// Stack Pointer Select
__register 64 { 0:0 SP } SPSel;

// Error Record Miscellaneous Register 1
array [0..65534] of __register 64 {  } ERRMISC1;

// Debug Data Transfer Register, Transmit
__register 32 {  } DBGDTRTXint;

// Interrupt Controller Virtual Active Priorities Group 0 Registers
array [0..3] of __register 64 {  } ICV_AP0R_EL1;

// Debug Claim Tag Clear register
__register 32 { 7:0 CLAIM } DBGCLAIMCLR_EL1;

// Auxiliary ID Register
__register 32 {  } AIDR_EL1;

// Profiling Buffer ID Register
__register 64 { 4:4 P, 3:0 Align, 5:5 F } PMBIDR_EL1;

// Debug ROM Address Register
__register 64 { 1:0 Valid, 47:12 ROMADDR } DBGDRAR;

// AArch32 Processor Feature Register 2
__register 32 { 11:8 RAS_frac, 7:4 SSBS, 3:0 CSV3 } ID_PFR2_EL1;

// Data Cache Zero ID register
__register 32 { 4:4 DZP, 3:0 BS } DCZID_EL0;

// Error Record Miscellaneous Register 3
array [0..65534] of __register 64 { 63:0 TS } ERRMISC3;

// Critical Error Interrupt Configuration Register 0
__register 64 { 55:2 ADDR } ERRCRICR0;

// Monitor Vector Base Address Register
__register 32 { 4:0 Reserved } MVBAR;

// Performance Monitors Device Affinity register 1
__register 32 {  } PMDEVAFF1;

// External Debug Component Identification Register 2
__register 32 { 7:0 PRMBL_2 } EDCIDR2;

// CTI Peripheral Identification Register 3
__register 32 { 3:0 CMOD, 7:4 REVAND } CTIPIDR3;

// Distributor Implementer Identification Register
__register 32 { 31:24 ProductID, 15:12 Revision, 19:16 Variant, 11:0 Implementer } GICD_IIDR;

// Performance Monitors Count Enable Clear register
__register 32 { 31:31 C } PMCNTENCLR_EL0;

// Debug Power Control Register
__register 32 { 0:0 CORENPDRQ } DBGPRCR_EL1;

// Interrupt Controller Interrupt Priority Mask Register
__register 32 { 7:0 Priority } ICC_PMR;

// Media and VFP Feature Register 1
__register 32 { 19:16 SIMDSP, 7:4 FPDNaN, 31:28 SIMDFMAC, 27:24 FPHP, 23:20 SIMDHP, 3:0 FPFtZ, 11:8 SIMDLS, 15:12 SIMDInt } MVFR1;

// LORegion End Address (EL1)
__register 64 { 51:48, 47:16 EA } LOREA_EL1;

// Vector Base Address Register (EL2)
__register 64 {  } VBAR_EL2;

// Component Identification Register 0
__register 32 { 7:0 PRMBL_0 } ERRCIDR0;

// Primary Region Remap Register
__register 32 { 18:18 NS0, 17:17 DS1, 19:19 NS1, 16:16 DS0 } PRRR;

// Error Group Status Register
__register 64 {  } ERRGSR;

// External Debug Program Counter Sample Register
__register 64 { 63:63 NS, 62:61 EL } EDPCSR;

// EL2 Software Thread ID Register
__register 64 {  } TPIDR_EL2;

// AArch64 Processor Feature Register 1
__register 64 { 7:4 SSBS, 11:8 MTE, 15:12 RAS_frac, 3:0 BT } ID_AA64PFR1_EL1;

// Interrupt Controller Virtual Interrupt Acknowledge Register 1
__register 32 { 23:0 INTID } ICV_IAR1;

// External Debug Device Affinity register 1
__register 32 {  } EDDEVAFF1;

// CTI Authentication Status register
__register 32 { 3:2 NSNID, 1:0 NSID } CTIAUTHSTATUS;

// Counter-timer Timer ID Register
__register 32 {  } CNTTIDR;

// External Debug AArch32 Processor Feature Register
__register 64 { 7:4 PMSA, 15:12 EL3, 11:8 EL2, 3:0 VMSA } EDAA32PFR;

// Debug Data Transfer Register, half-duplex
__register 64 { 63:32 HighWord, 31:0 LowWord } DBGDTR_EL0;

// Debug Claim Tag Set register
__register 32 { 7:0 CLAIM } DBGCLAIMSET;

// Hypervisor IPA Fault Address Register
__register 64 { 43:40, 39:4 FIPA, 63:63 NS } HPFAR_EL2;

// Instruction Set Attribute Register 5
__register 32 { 27:24 RDM, 19:16 CRC32, 15:12 SHA2, 11:8 SHA1, 31:28 VCMA, 7:4 AES, 3:0 SEVL } ID_ISAR5;

// User Access Override
__register 32 { 23:23 UAO } UAO;

// Fault-Handling Interrupt Configuration Register 1
__register 32 { 31:0 DATA } ERRFHICR1;

// External Debug Context ID Sample Register
__register 32 { 31:0 CONTEXTIDR } EDCIDSR;

// MPAM Resource Monitoring Identification Register
__register 32 { 31:31 HAS_LOCAL_CAPT_EVNT, 17:17 MSMON_MBWU, 16:16 MSMON_CSU } MPAMF_MSMON_IDR;

// MPAM Features Memory Bandwidth Usage Monitoring ID register
__register 32 { 15:0 NUM_MON, 31:31 HAS_CAPTURE } MPAMF_MBWUMON_IDR;

// Translation Control Register (EL2)
__register 64 { 38:38 TBI1, 25:24 IRGN1, 58:58 TCMA1, 46:46 HWU062, 39:39 HA, 18:16 PS, 37:37 TBI0, 29:28 SH1, 27:26 ORGN1, 54:54 NFD1, 24:24 HPD, 49:49 HWU161, 29:29 TBID, 20:20 TBI, 56:56 E0PD1, 30:30 TCMA, 57:57 TCMA0, 45:45 HWU061, 41:41 HPD0, 48:48 HWU160, 44:44 HWU060, 9:8 IRGN0, 51:51 TBID0, 40:40 HD, 50:50 HWU162, 36:36 AS, 31:30 TG1, 11:10 ORGN0, 23:23 EPD1, 55:55 E0PD0, 43:43 HWU059, 42:42 HPD1, 28:28 HWU62, 34:32 IPS, 25:25 HWU59, 47:47 HWU159, 5:0 T0SZ, 52:52 TBID1, 27:27 HWU61, 15:14 TG0, 13:12 SH0, 26:26 HWU60, 7:7 EPD0, 22:22 A1, 21:16 T1SZ, 53:53 NFD0 } TCR_EL2;

// Deferred Interrupt Status Register
__register 64 { 24:24 IDS, 12:10 AET, 5:0 DFSC, 23:0 ISS, 9:9 EA, 31:31 A } DISR_EL1;

// Virtual Deferred Interrupt Status Register
__register 32 { 12:12 ExT, 31:31 A, 9:9 LPAE, 5:0 STATUS, 10:10, 3:0 FS, 15:14 AET } VDISR;

// Counter-timer Secure Virtual Timer CompareValue register (EL2)
__register 64 { 63:0 CompareValue } CNTHVS_CVAL_EL2;

// Selected Pseudo-fault Generation Control Register
__register 64 {  } ERXPFGCTL_EL1;

// OS Lock Data Transfer Register, Receive
__register 32 {  } OSDTRRX_EL1;

// Activity Monitors Peripheral Identification Register 4
__register 32 { 7:4 SIZE, 3:0 DES_2 } AMPIDR4;

// Debug OS Lock Data Transfer Register, Transmit
__register 32 {  } DBGDTRTXext;

// Counter Identification Register
__register 32 { 3:0 CNTSC } CNTID;

// Performance Monitors Component Identification Register 1
__register 32 { 3:0 PRMBL_1, 7:4 CLASS } PMCIDR1;

// External Debug Watchpoint Address Register
__register 64 {  } EDWAR;

// Fault-Handling Interrupt Configuration Register 2
__register 32 { 5:4 SH, 6:6 NSMSI, 3:0 MemAttr, 7:7 IRQEN } ERRFHICR2;

// Selected Error Record Control Register
__register 64 {  } ERXCTLR_EL1;

// MPAM Implemenation Identification Register
__register 32 { 19:16 Variant, 15:12 Revision, 11:0 Implementer, 31:20 ProductID } MPAMF_IIDR;

// Instruction Set Attribute Register 1
__register 32 { 19:16 IfThen, 3:0 Endian, 15:12 Extend, 27:24 Interwork, 11:8 Except_AR, 31:28 Jazelle, 23:20 Immediate, 7:4 Except } ID_ISAR1;

// External Debug Virtual Context Sample Register
__register 32 { 31:0 CONTEXTIDR_EL2, 28:28 HV, 30:30 E2, 29:29 E3, 31:31 NS, 15:0 VMID } EDVIDSR;

// Interrupt Controller Interrupt Acknowledge Register 1
__register 32 { 23:0 INTID } ICC_IAR1_EL1;

// Trace Filter Control Register
__register 32 { 6:5 TS, 0:0 E0TRE, 1:1 E1TRE } TRFCR;

// Interrupt Group Registers (extended SPI range)
array [0..31] of __register 32 {  } GICD_IGROUPRE;

// Interrupt Controller End Of Interrupt Register 1
__register 32 { 23:0 INTID } ICC_EOIR1;

// Selected Error Record Primary Status Register
__register 32 {  } ERXSTATUS;

// MPAM Internal PARTID Narrowing Configuration Register
__register 32 { 16:16 INTERNAL, 15:0 INTPARTID } MPAMCFG_INTPARTID;

// Debug Device ID register 0
__register 32 { 31:28 CIDMask, 23:20 DoubleLock, 27:24 AuxRegs, 11:8 BPAddrMask, 3:0 PCSample, 19:16 VirtExtns, 15:12 VectorCatch, 7:4 WPAddrMask } DBGDEVID;

// Selected Pseudo-fault Generation Countdown Register
__register 64 {  } ERXPFGCDN_EL1;

// Context ID Register
__register 32 { 31:0 PROCID, 7:0 ASID } CONTEXTIDR;

// Counter-timer Virtual Timer TimerValue register (EL2)
__register 32 { 31:0 TimerValue } CNTHV_TVAL;

// Counter-timer Virtual Timer CompareValue register (EL2)
__register 64 { 63:0 CompareValue } CNTHV_CVAL;

// Tag Fail Status Register (EL3).
__register 64 { 0:0 TF0 } TFSR_EL3;

// CTI Component Identification Register 0
__register 32 { 7:0 PRMBL_0 } CTICIDR0;

// AArch32 Instruction Set Attribute Register 6
__register 32 { 15:12 SB, 11:8 FHM, 7:4 DP, 19:16 SPECRES, 3:0 JSCVT } ID_ISAR6_EL1;

// Media and VFP Feature Register 2
__register 32 { 7:4 FPMisc, 3:0 SIMDMisc } MVFR2;

// TLB Type Register
__register 32 { 0:0 nU } TLBTR;

// Interrupt Controller List Registers
array [0..15] of __register 64 { 55:48 Priority, 44:32 pINTID, 60:60 Group, 63:62 State, 61:61 HW, 31:0 vINTID } ICH_LR_EL2;

// Performance Monitors User Enable Register
__register 32 { 1:1 SW, 3:3 ER, 2:2 CR, 0:0 EN } PMUSERENR_EL0;

// Exception Syndrome Register (EL2)
__register 32 { 4:4 Offset, 4:4 IXF, 3:3 UFF, 19:17 Op2, 2:2 OFF, 23:23 TFV, 13:13 IESB, 9:5 Rn, 14:10 Rt2, 15:0 Comment, 6:6 WnR, 7:7 IDF, 0:0 IOF, 14:14 AR, 10:10 FnV, 0:0 TI, 23:22 SAS, 19:16 Opc1, 12:11 SET, 24:24 CV, 21:20 Op0, 19:19 CCKNOWNPASS, 21:21 SSE, 5:0 DFSC, 6:6 EX, 24:24 IDS, 1:0 BTYPE, 13:10 CRn, 19:12 imm8, 15:0 imm16, 24:24 ISV, 7:7 S1PTW, 1:1 DZF, 3:1 AM, 10:8 VECITR, 1:1 ERET, 0:0 Direction, 9:9 EA, 20:16 SRT, 12:10 AET, 15:15 SF, 25:25 IL, 0:0 ERETA, 5:0 IFSC, 8:8 CM, 23:20 COND, 9:5 Rt, 16:14 Op1, 13:13 VNCR, 19:17 Opc2, 31:26 EC, 4:1 CRm } ESR_EL2;

// Counter-timer Physical Timer TimerValue register
__register 32 { 31:0 TimerValue } CNTP_TVAL;

// Stack Pointer (EL2)
__register 64 {  } SP_EL2;

// Hyp IPA Fault Address Register
__register 32 { 31:4 FIPA } HPFAR;

// Auxiliary Control Register (EL3)
__register 64 {  } ACTLR_EL3;

// Interrupt Controller Virtual Control Register
__register 32 { 0:0 CBPR, 14:14 SEIS, 10:8 PRIbits, 18:18 RSS, 1:1 EOImode, 15:15 A3V, 19:19 ExtRange, 13:11 IDbits } ICV_CTLR;

// Processor Feature Register 2
__register 32 { 7:4 SSBS, 3:0 CSV3, 11:8 RAS_frac } ID_PFR2;

// Counter Count Value register
__register 64 { 63:0 CountValue } CNTCV;

// Instruction Set Attribute Register 4
__register 32 { 27:24 PSR_M, 31:28 SWP_frac, 7:4 WithShifts, 23:20 SynchPrim_frac, 19:16 Barrier, 15:12 SMC, 3:0 Unpriv, 11:8 Writeback } ID_ISAR4;

// Random Number
__register 64 { 63:0 RNDR } RNDR;

// Virtual Machine Running Priority Register
__register 32 { 7:0 Priority } GICV_RPR;

// MPAM2 Register (EL2)
__register 64 { 47:40 PMG_D, 31:16 PARTID_D, 15:0 PARTID_I, 49:49 TRAPMPAM0EL1, 48:48 TRAPMPAM1EL1, 63:63 MPAMEN, 39:32 PMG_I } MPAM2_EL2;

// Performance Monitors Overflow Flag Status Set register
__register 32 { 31:31 C } PMOVSSET;

// Interrupt Controller End of Interrupt Status Register
__register 32 {  } ICH_EISR;

// Counter-timer Virtual Offsets
array [0..7] of __register 64 {  } CNTVOFF;

// Floating-point Status Register
__register 32 { 29:29 C, 31:31 N, 28:28 V, 4:4 IXC, 1:1 DZC, 30:30 Z, 27:27 QC, 3:3 UFC, 2:2 OFC, 0:0 IOC, 7:7 IDC } FPSR;

// Virtualization Multiprocessor ID Register
__register 32 { 7:0 Aff0, 15:8 Aff1, 24:24 MT, 31:31 M, 30:30 U, 23:16 Aff2 } VMPIDR;

// Activity Monitors Peripheral Identification Register 1
__register 32 { 3:0 PART_1, 7:4 DES_0 } AMPIDR1;

// AArch32 Secure Debug Enable Register
__register 32 { 1:1 SUNIDEN, 0:0 SUIDEN } SDER32_EL2;

// Performance Monitors Device Type register
__register 32 { 3:0 MAJOR, 7:4 SUB } PMDEVTYPE;

// Interrupt Clear-Pending Registers
array [0..31] of __register 32 {  } GICD_ICPENDR;

// Interrupt Controller Running Priority Register
__register 32 { 7:0 Priority } ICC_RPR_EL1;

// Error Reporting Status Register
__register 32 { 0:0 RRD, 3:3 WROD, 1:1 WRD, 2:2 RWOD } GICD_STATUSR;

// Saved Program Status Register (Abort mode)
__register 32 { 4:0 M, 20:20 IL, 24:24 J, 15:10, 26:25 IT, 8:8 A, 28:28 V, 31:31 N, 19:16 GE, 21:21 DIT, 6:6 F, 30:30 Z, 27:27 Q, 7:7 I, 9:9 E, 22:22 PAN, 23:23 SSBS, 29:29 C, 5:5 T } SPSR_abt;

// Virtualization Secure Translation Control Register
__register 32 { 29:29 SW, 7:6 SL0, 5:0 T0SZ, 15:14 TG0, 30:30 SA } VSTCR_EL2;

// Interrupt Priority Registers
array [0..254] of __register 32 { 7:0 Priority_offset_0B, 31:24 Priority_offset_3B, 23:16 Priority_offset_2B, 15:8 Priority_offset_1B } GICD_IPRIORITYR;

// Interrupt Controller Highest Priority Pending Interrupt Register 0
__register 32 { 23:0 INTID } ICC_HPPIR0_EL1;

// External Debug Peripheral Identification Register 2
__register 32 { 3:3 JEDEC, 7:4 REVISION, 2:0 DES_1 } EDPIDR2;

// Debug Vector Catch Register
__register 32 { 28:28 NSD, 1:1 U, 6:6 SI, 3:3 SP, 27:27 NSP, 25:25 NSU, 31:31 NSF, 4:4 D, 26:26 NSS, 30:30 NSI, 2:2 SS, 6:6 I, 4:4 SD, 7:7 F, 7:7 SF, 1:1 SU, 3:3 P, 2:2 S } DBGVCR32_EL2;

// Interrupt Controller Monitor Control Register
__register 32 { 1:1 CBPR_EL1NS, 0:0 CBPR_EL1S, 19:19 ExtRange, 14:14 SEIS, 18:18 RSS, 17:17 nDS, 15:15 A3V, 6:6 PMHE, 10:8 PRIbits, 5:5 RM, 3:3 EOImode_EL1S, 2:2 EOImode_EL3, 13:11 IDbits, 4:4 EOImode_EL1NS } ICC_MCTLR;

// Debug Claim Tag Clear register
__register 32 { 7:0 CLAIM } DBGCLAIMCLR;

// Trace Filter Control Register (EL1)
__register 64 { 0:0 E0TRE, 6:5 TS, 1:1 E1TRE } TRFCR_EL1;

// AArch32 Media and VFP Feature Register 0
__register 32 { 3:0 SIMDReg, 7:4 FPSP, 31:28 FPRound, 23:20 FPSqrt, 27:24 FPShVec, 15:12 FPTrap, 11:8 FPDP, 19:16 FPDivide } MVFR0_EL1;

// Random Allocation Tag Seed Register.
__register 32 { 3:0 TAG, 23:8 SEED } RGSR_EL1;

// Counter-timer Physical Timer Control register
__register 32 { 2:2 ISTATUS, 0:0 ENABLE, 1:1 IMASK } CNTP_CTL_EL0;

// Active Priorities Registers
array [0..3] of __register 32 {  } GICH_APR;

// MPAM Virtual PARTID Mapping Register 6
__register 64 { 47:32 PhyPARTID26, 31:16 PhyPARTID25, 15:0 PhyPARTID24, 63:48 PhyPARTID27 } MPAMVPM6_EL2;

// DCC Interrupt Enable Register
__register 32 { 30:30 RX, 29:29 TX } DBGDCCINT;

// Virtual Machine Priority Mask Register
__register 32 { 7:0 Priority } GICV_PMR;

// CTI Integration mode Control register
__register 32 { 0:0 IME } CTIITCTRL;

// Processor Feature Register 1
__register 32 { 31:28 GIC, 19:16 GenTimer, 15:12 Virtualization, 7:4 Security, 3:0 ProgMod, 23:20 Sec_frac, 11:8 MProgMod, 27:24 Virt_frac } ID_PFR1;

// Virtual Machine Aliased Binary Point Register
__register 32 { 2:0 Binary_Point } GICV_ABPR;

// Redistributor Invalidate All Register
__register 64 {  } GICR_INVALLR;

// Data Independent Timing
__register 32 { 24:24 DIT } DIT;

// Counter-timer Secure Virtual Timer TimerValue Register (EL2)
__register 32 { 31:0 TimerValue } CNTHVS_TVAL;

// Debug Watchpoint Fault Address Register
__register 32 {  } DBGWFAR;

// Counter-timer Secure Physical Timer Control Register (EL2)
__register 32 { 0:0 ENABLE, 1:1 IMASK, 2:2 ISTATUS } CNTHPS_CTL;

// Debug Data Transfer Register, Receive
__register 32 {  } DBGDTRRXint;

// Error Record ID Register
__register 64 { 15:0 NUM } ERRIDR_EL1;

// Interrupt Processor Targets Registers
array [0..254] of __register 32 { 7:0 CPU_targets_offset_0B, 15:8 CPU_targets_offset_1B, 31:24 CPU_targets_offset_3B, 23:16 CPU_targets_offset_2B } GICD_ITARGETSR;

// Generic Error Interrupt Configuration Register
array [0..15] of __register 64 {  } ERRIRQCR;

// Virtual Type Register
__register 32 { 31:29 PRIbits, 28:26 PREbits, 25:23 IDbits, 21:21 A3V, 4:0 ListRegs, 22:22 SEIS } GICH_VTR;

// CPU Interface Aliased Highest Priority Pending Interrupt Register
__register 32 { 23:0 INTID } GICC_AHPPIR;

// AArch32 Secure Debug Enable Register
__register 32 { 0:0 SUIDEN, 1:1 SUNIDEN } SDER32_EL3;

// Component Identification Register 2
__register 32 { 7:0 PRMBL_2 } ERRCIDR2;

// Debug Watchpoint Value Registers
array [0..15] of __register 32 { 31:2 VA } DBGWVR;

// Debug Data Transfer Register, Receive
__register 32 {  } DBGDTRRX_EL0;

// Performance Monitors Overflow Flag Status Clear register
__register 32 { 31:31 C } PMOVSCLR_EL0;

// Interrupt Mask Bits
__register 32 { 8:8 A, 9:9 D, 7:7 I, 6:6 F } DAIF;

// Auxiliary Memory Attribute Indirection Register (EL3)
__register 64 {  } AMAIR_EL3;

// Interrupt Controller Monitor System Register Enable register
__register 32 { 3:3 Enable, 0:0 SRE, 2:2 DIB, 1:1 DFB } ICC_MSRE;

// Interrupt Controller Hyp Control Register
__register 32 { 31:27 EOIcount, 10:10 TC, 7:7 VGrp1DIE, 5:5 VGrp0DIE, 3:3 NPIE, 13:13 TSEI, 2:2 LRENPIE, 1:1 UIE, 0:0 En, 12:12 TALL1, 4:4 VGrp0EIE, 11:11 TALL0, 6:6 VGrp1EIE, 14:14 TDIR } ICH_HCR_EL2;

// Performance Monitors Cycle Count Filter Register
__register 32 { 24:24 SH, 29:29 NSK, 28:28 NSU, 27:27 NSH, 31:31 P, 30:30 U, 26:26 M } PMCCFILTR_EL0;

// Interrupt Controller Virtual Running Priority Register
__register 32 { 7:0 Priority } ICV_RPR_EL1;

// Activity Monitors Component Identification Register 2
__register 32 { 7:0 PRMBL_2 } AMCIDR2;

// Exception Link Register (EL1)
__register 64 {  } ELR_EL1;

// MPAM Cache Storage Usage Monitor Register
__register 32 { 30:0 VALUE, 31:31 NRDY } MSMON_CSU;

// Interrupt Controller Software Generated Interrupt Group 0 Register
__register 64 { 55:48 Aff3, 40:40 IRM, 23:16 Aff1, 47:44 RS, 39:32 Aff2, 15:0 TargetList, 27:24 INTID } ICC_SGI0R;

// Interrupt Set-Pending Registers (extended SPI range)
array [0..31] of __register 32 {  } GICD_ISPENDRE;

// Counter Control Register
__register 32 { 17:8 FCREQ, 0:0 EN, 2:2 SCEN, 1:1 HDBG } CNTCR;

// Stack Pointer (EL0)
__register 64 {  } SP_EL0;

// External Debug Device ID register 0
__register 32 { 27:24 AuxRegs, 3:0 PCSample, 7:4 DebugPower } EDDEVID;

// MPAM Virtual PARTID Mapping Register 1
__register 64 { 31:16 PhyPARTID5, 63:48 PhyPARTID7, 15:0 PhyPARTID4, 47:32 PhyPARTID6 } MPAMVPM1_EL2;

// Interrupt Controller Active Priorities Group 0 Registers
array [0..3] of __register 64 {  } ICC_AP0R_EL1;

// CPU Interface Identification Register
__register 32 { 15:12 Revision, 11:0 Implementer, 19:16 Architecture_version, 31:20 ProductID } GICC_IIDR;

// Saved Program Status Register (Undefined mode)
__register 32 { 27:27 Q, 20:20 IL, 29:29 C, 6:6 F, 19:16 GE, 7:7 I, 5:5 T, 15:10, 26:25 IT, 24:24 J, 21:21 DIT, 31:31 N, 8:8 A, 30:30 Z, 9:9 E, 22:22 PAN, 4:0 M, 23:23 SSBS, 28:28 V } SPSR_und;

// CTI Device Type register
__register 32 { 7:4 SUB, 3:0 MAJOR } CTIDEVTYPE;

// Translation Control Register (EL3)
__register 32 { 11:10 ORGN0, 15:14 TG0, 13:12 SH0, 24:24 HPD, 25:25 HWU59, 20:20 TBI, 22:22 HD, 9:8 IRGN0, 21:21 HA, 26:26 HWU60, 30:30 TCMA, 5:0 T0SZ, 18:16 PS, 28:28 HWU62, 29:29 TBID, 27:27 HWU61 } TCR_EL3;

// Current Cache Size ID Register
__register 32 { 2:0 LineSize, 12:3 Associativity, 27:13 NumSets } CCSIDR_EL1;

// Interrupt Clear-Pending Registers (extended SPI range)
array [0..31] of __register 32 {  } GICD_ICPENDRE;

// OS Double Lock Register
__register 32 { 0:0 DLK } OSDLR_EL1;

// Performance Monitors Authentication Status register
__register 32 { 7:6 SNID, 1:0 NSID, 3:2 NSNID, 5:4 SID } PMAUTHSTATUS;

// Sampling Interval Reload Register
__register 64 { 31:8 INTERVAL, 0:0 RND } PMSIRR_EL1;

// AArch32 Memory Model Feature Register 0
__register 32 { 19:16 TCM, 27:24 FCSE, 11:8 OuterShr, 31:28 InnerShr, 15:12 ShareLvl, 23:20 AuxReg, 3:0 VMSA, 7:4 PMSA } ID_MMFR0_EL1;

// Activity Monitors Event Type Registers 1
array [0..15] of __register 32 { 15:0 evtCount } AMEVTYPER1;

// Interrupt Controller Deactivate Virtual Interrupt Register
__register 32 { 23:0 INTID } ICV_DIR;

// External Debug Peripheral Identification Register 3
__register 32 { 7:4 REVAND, 3:0 CMOD } EDPIDR3;

// AArch64 Debug Feature Register 0
__register 64 { 35:32 PMSVer, 23:20 WRPs, 43:40 TraceFilt, 11:8 PMUVer, 3:0 DebugVer, 15:12 BRPs, 7:4 TraceVer, 39:36 DoubleLock, 31:28 CTX_CMPs } ID_AA64DFR0_EL1;

// Performance Monitors Count Enable Set register
__register 32 { 31:31 C } PMCNTENSET;

// Cache Type Register
__register 32 { 37:32 TminLine, 27:24 CWG, 29:29 DIC, 19:16 DminLine, 3:0 IminLine, 28:28 IDC, 23:20 ERG, 15:14 L1Ip } CTR_EL0;

// Interrupt Set-Active Register 0
__register 32 {  } GICR_ISACTIVER0;

// Virtual Machine Control Register
__register 32 { 2:2 AckCtl, 0:0 EnableGrp0, 4:4 CBPR, 9:9 EOImode, 3:3 FIQEn, 1:1 EnableGrp1 } GICV_CTLR;

// Set PARTID and PMG Register
__register 32 { 15:0 PARTID, 23:16 PMG } GICR_PARTIDR;

// Interrupt Controller Hyp Active Priorities Group 1 Registers
array [0..3] of __register 64 {  } ICH_AP1R_EL2;

// External Debug Device Type register
__register 32 { 7:4 SUB, 3:0 MAJOR } EDDEVTYPE;

// Counter-timer Physical Timer TimerValue register (EL2)
__register 32 { 31:0 TimerValue } CNTHP_TVAL_EL2;

// Hyp Memory Attribute Indirection Register 1
__register 32 {  } HMAIR1;

// Debug Watchpoint Value Registers
array [0..15] of __register 64 { 52:49, 48:2 VA, 63:53, 52:49 RESS } DBGWVR_EL1;

// Counter-timer Physical Secure Timer TimerValue register
__register 32 { 31:0 TimerValue } CNTPS_TVAL_EL1;

// Activity Monitors Event Counter Registers 0
array [0..15] of __register 64 { 63:0 ACNT } AMEVCNTR0;

// PL1 Software Thread ID Register
__register 32 {  } TPIDRPRW;

// AArch32 Media and VFP Feature Register 1
__register 32 { 27:24 FPHP, 7:4 FPDNaN, 15:12 SIMDInt, 31:28 SIMDFMAC, 11:8 SIMDLS, 3:0 FPFtZ, 23:20 SIMDHP, 19:16 SIMDSP } MVFR1_EL1;

// Selected Error Record Primary Status Register
__register 64 {  } ERXSTATUS_EL1;

// Interrupt Controller Virtual Interrupt Acknowledge Register 1
__register 32 { 23:0 INTID } ICV_IAR1_EL1;

// Counter-timer Access Control Registers
array [0..7] of __register 32 { 1:1 RVCT, 0:0 RPCT, 2:2 RFRQ, 4:4 RWVT, 5:5 RWPT, 3:3 RVOFF } CNTACR;

// Activity Monitors Event Counter Registers 1
array [0..15] of __register 64 { 63:0 ACNT } AMEVCNTR1;

// Error Record Select Register
__register 64 { 15:0 SEL } ERRSELR_EL1;

// Interrupt Controller Virtual Highest Priority Pending Interrupt Register 1
__register 32 { 23:0 INTID } ICV_HPPIR1;

// MPAM Bandwidth Portion Bitmap Parition Configuration Register
__register 4096 {  } MPAMCFG_MBW_PBM;

// EL0 Read/Write Software Context Number
__register 64 {  } SCXTNUM_EL0;

// Virtualization Processor ID Register
__register 32 { 23:20 Variant, 19:16 Architecture, 15:4 PartNum, 31:24 Implementer, 3:0 Revision } VPIDR_EL2;

// CTI Claim Tag Set register
__register 32 {  } CTICLAIMSET;

// Counter Scale Register
__register 32 { 31:0 ScaleVal } CNTSCR;

// Error Recovery Interrupt Configuration Register 0
__register 64 { 55:2 ADDR } ERRERICR0;

// Virtual Redistributor Properties Base Address Register
__register 64 { 51:12 Physical_Address, 58:56 OuterCache, 9:7 InnerCache, 11:10 Shareability, 4:0 IDbits } GICR_VPROPBASER;

// Error Record Control Register
array [0..65534] of __register 64 { 2:2 RUI, 3:3 RFI, 2:2 UI, 11:11 WDUI, 7:7 WUE, 10:10 DUI, 8:8 RCFI, 13:13 CI, 10:10 RDUI, 5:5 WUI, 4:4 RUE, 8:8 CFI, 3:3 FI, 4:4 UE, 0:0 ED, 6:6 WFI, 9:9 WCFI } ERRCTLR;

// Hyp System Trap Register
__register 32 {  } HSTR;

// CTI External Multiplexer Control register
__register 32 {  } ASICCTL;

// Interrupt Controller Interrupt Group 0 Enable register
__register 32 { 0:0 Enable } ICC_IGRPEN0_EL1;

// Interrupt Controller Interrupt Group 0 Enable register
__register 32 { 0:0 Enable } ICC_IGRPEN0;

// Interrupt Controller Interrupt Acknowledge Register 0
__register 32 { 23:0 INTID } ICC_IAR0_EL1;

// Activity Monitors Event Counter Registers 0
array [0..15] of __register 64 { 63:0 ACNT } AMEVCNTR0_EL0;

// PL0 Read/Write Software Thread ID Register
__register 32 {  } TPIDRURW;

// Domain Access Control Register
__register 32 {  } DACR;

// CTI Output Trigger Acknowledge register
__register 32 {  } CTIINTACK;

// Device Architecture Register
__register 32 { 20:20 PRESENT, 19:16 REVISION, 31:21 ARCHITECT, 15:12 ARCHVER, 11:0 ARCHPART } ERRDEVARCH;

// Architectural Feature Access Control Register
__register 32 { 21:20 FPEN, 17:16 ZEN, 28:28 TTA } CPACR_EL1;

// Interrupt Set-Pending Registers
array [1..2] of __register 32 {  } GICR_ISPENDRE;

// Current Cache Size ID Register 2
__register 32 { 23:0 NumSets } CCSIDR2_EL1;

// Normal Memory Remap Register
__register 32 {  } NMRR;

// Redistributor Invalidate LPI Register
__register 64 { 31:0 pINTID } GICR_INVLPIR;

// OS Lock Status Register
__register 32 { 3:3, 0:0 OSLM, 1:1 OSLK, 2:2 nTT } OSLSR_EL1;

// Interrupt Controller Alias Software Generated Interrupt Group 1 Register
__register 64 { 39:32 Aff2, 15:0 TargetList, 47:44 RS, 55:48 Aff3, 27:24 INTID, 23:16 Aff1, 40:40 IRM } ICC_ASGI1R;

// ITS Control Register
__register 32 { 1:1 ImDe, 7:4 ITS_Number, 31:31 Quiescent, 0:0 Enabled } GITS_CTLR;

// Pseudo-fault Generation Countdown Register
array [0..65534] of __register 64 { 31:0 CDN } ERRPFGCDN;

// Counter-timer Virtual Offset register
__register 64 {  } CNTVOFF_EL2;

// Cache Level ID Register
__register 32 { 29:27 LoUU, 26:24 LoC, 31:30 ICB, 23:21 LoUIS } CLIDR;

// Architectural Feature Access Control Register
__register 32 { 21:20 cp10, 31:31 ASEDIS, 23:22 cp11, 28:28 TRCDIS } CPACR;

// CTI Lock Access Register
__register 32 { 31:0 KEY } CTILAR;

// Speculative Store Bypass Safe
__register 32 { 12:12 SSBS } SSBS;

// Performance Monitors Lock Access Register
__register 32 { 31:0 KEY } PMLAR;

// Sampling Latency Filter Register
__register 64 { 11:0 MINLAT } PMSLATFR_EL1;

// Interrupt Group Modifier Register 0
__register 32 {  } GICR_IGRPMODR0;

// MPAM1 Register (EL1)
__register 64 { 39:32 PMG_I, 63:63 MPAMEN, 31:16 PARTID_D, 47:40 PMG_D, 15:0 PARTID_I } MPAM1_EL1;

// Memory Model Feature Register 2
__register 32 { 15:12 HvdTLB, 3:0 L1HvdFG, 11:8 L1HvdRng, 31:28 HWAccFlg, 19:16 UniTLB, 27:24 WFIStall, 7:4 L1HvdBG, 23:20 MemBarr } ID_MMFR2;

// Error Reporting Status Register
__register 32 { 0:0 RRD, 2:2 RWOD, 1:1 WRD, 3:3 WROD } GICR_STATUSR;

// System Control Register (EL3)
__register 64 { 3:3 SA, 36:36 BT, 11:11 EOS, 1:1 A, 44:44 DSSBS, 37:37 ITFSB, 22:22 EIS, 43:43 ATA, 27:27 EnDA, 6:6 nAA, 31:31 EnIA, 21:21 IESB, 13:13 EnDB, 2:2 C, 41:40 TCF, 0:0 M, 30:30 EnIB, 25:25 EE, 19:19 WXN, 12:12 I } SCTLR_EL3;

// Virtualization Translation Control Register
__register 32 { 26:26 HWU60, 4:4 S, 13:12 SH0, 25:25 HWU59, 7:6 SL0, 3:0 T0SZ, 27:27 HWU61, 9:8 IRGN0, 28:28 HWU62, 11:10 ORGN0 } VTCR;

// Performance Monitors Peripheral Identification Register 4
__register 32 { 7:4 SIZE, 3:0 DES_2 } PMPIDR4;

// Debug Vector Catch Register
__register 32 { 1:1 SU, 7:7 F, 6:6 I, 31:31 NSF, 7:7 SF, 11:11 MP, 6:6 SI, 4:4 D, 2:2 S, 14:14 MI, 10:10 MS, 15:15 MF, 30:30 NSI, 1:1 U, 26:26 NSS, 3:3 P, 27:27 NSP, 3:3 SP, 28:28 NSD, 4:4 SD, 25:25 NSU, 2:2 SS, 12:12 MD } DBGVCR;

// Activity Monitors Control Register
__register 32 { 10:10 HDBG } AMCR;

// Counter-timer Physical Secure Timer CompareValue register
__register 64 { 63:0 CompareValue } CNTPS_CVAL_EL1;

// Interrupt Controller Virtual End Of Interrupt Register 0
__register 32 { 23:0 INTID } ICV_EOIR0_EL1;

// Interrupt Controller Virtual Active Priorities Group 1 Registers
array [0..3] of __register 64 {  } ICV_AP1R_EL1;

// Interrupt Controller Virtual Binary Point Register 1
__register 32 { 2:0 BinaryPoint } ICV_BPR1;

// Interrupt Set-Enable Registers
array [0..31] of __register 32 {  } GICD_ISENABLER;

// Cache Type Register
__register 32 { 15:14 L1Ip, 28:28 IDC, 3:0 IminLine, 19:16 DminLine, 29:29 DIC, 23:20 ERG, 27:24 CWG } CTR;

// MPAM Memory System Monitor Configure Cache Storage Usage Monitor Control Register
__register 32 { 24:24 OFLOW_FRZ, 25:25 OFLOW_INTR, 27:27 CAPT_RESET, 17:17 MATCH_PMG, 16:16 MATCH_PARTID, 23:20 SUBTYPE, 30:28 CAPT_EVNT, 7:0 TYPE, 26:26 OFLOW_STATUS, 31:31 EN } MSMON_CFG_CSU_CTL;

// MPAM Capture Event Generation Register
__register 32 { 1:1 ALL, 0:0 NOW } MSMON_CAPT_EVNT;

// External Debug Power/Reset Control Register
__register 32 { 1:1 CWRR, 0:0 CORENPDRQ, 3:3 COREPURQ } EDPRCR;

// AArch32 Memory Model Feature Register 2
__register 32 { 19:16 UniTLB, 7:4 L1HvdBG, 31:28 HWAccFlg, 3:0 L1HvdFG, 23:20 MemBarr, 27:24 WFIStall, 11:8 L1HvdRng, 15:12 HvdTLB } ID_MMFR2_EL1;

// Instruction Fault Address Register
__register 32 {  } IFAR;

// Redistributor Implementer Identification Register
__register 32 { 19:16 Variant, 11:0 Implementer, 15:12 Revision, 31:24 ProductID } GICR_IIDR;

// MPAM Cache Portion Bitmap Partition Configuration Register
__register 32768 {  } MPAMCFG_CPBM;

// Interrupt Controller Virtual Interrupt Acknowledge Register 0
__register 32 { 23:0 INTID } ICV_IAR0;

// Monitor DCC Status Register
__register 32 { 30:30 RXfull, 29:29 TXfull } MDCCSR_EL0;

// Memory Attribute Indirection Register (EL1)
__register 64 {  } MAIR_EL1;

// External Debug Device Architecture register
__register 32 { 19:16 REVISION, 15:12 ARCHVER, 31:21 ARCHITECT, 20:20 PRESENT, 11:0 ARCHPART } EDDEVARCH;

// Selected Error Record Miscellaneous Register 2
__register 64 {  } ERXMISC2_EL1;

// Counter-timer Hyp Control register
__register 32 { 2:2 EVNTEN, 3:3 EVNTDIR, 1:1 PL1PCEN, 7:4 EVNTI, 0:0 PL1PCTEN } CNTHCTL;

// Counter-timer Physical Count
__register 64 {  } CNTPCT;

// Virtual Machine Binary Point Register
__register 32 { 2:0 Binary_Point } GICV_BPR;

// Activity Monitors Event Counter Registers 1
array [0..15] of __register 64 { 63:0 ACNT } AMEVCNTR1_EL0;

// SVE Control Register for EL2
__register 64 { 3:0 LEN } ZCR_EL2;

// Saved Program Status Register (EL1)
__register 32 { 22:22 PAN, 15:10, 26:25 IT, 4:4, 4:4, 3:0, 3:0 M, 11:10 BTYPE, 24:24 DIT, 9:9 E, 8:8 A, 21:21 SS, 6:6 F, 23:23 UAO, 25:25 TCO, 31:31 N, 30:30 Z, 27:27 Q, 28:28 V, 29:29 C, 20:20 IL, 7:7 I, 9:9 D, 19:16 GE, 12:12 SSBS, 5:5 T } SPSR_EL1;

// Counter-timer Virtual Timer Control register (EL2)
__register 32 { 1:1 IMASK, 0:0 ENABLE, 2:2 ISTATUS } CNTHV_CTL_EL2;

// External Debug Reserve Control Register
__register 32 { 4:4 CBRRQ, 2:2 CSE, 3:3 CSPA } EDRCR;

// Counter-timer Physical Secure Timer Control register
__register 32 { 1:1 IMASK, 0:0 ENABLE, 2:2 ISTATUS } CNTPS_CTL_EL1;

// Floating-Point Exception Control register
__register 32 { 3:3 UFF, 10:8 VECITR, 28:28 FP2V, 4:4 IXF, 0:0 IOF, 30:30 EN, 27:27 VV, 1:1 DZF, 26:26 TFV, 7:7 IDF, 29:29 DEX, 2:2 OFF, 31:31 EX } FPEXC;

// MPAM PARTID Narrowing ID register
__register 32 { 15:0 INTPARTID_MAX } MPAMF_PARTID_NRW_IDR;

// CTI Trigger Out Status register
__register 32 {  } CTITRIGOUTSTATUS;

// Hyp Vector Base Address Register
__register 32 {  } HVBAR;

// Instruction Set Attribute Register 2
__register 32 { 19:16 MultS, 23:20 MultU, 27:24 PSR_AR, 3:0 LoadStore, 11:8 MultiAccessInt, 15:12 Mult, 31:28 Reversal, 7:4 MemHint } ID_ISAR2;

// Debug Feature Register 0
__register 32 { 27:24 PerfMon, 15:12 CopTrc, 7:4 CopSDbg, 31:28 TraceFilt, 19:16 MMapTrc, 23:20 MProfDbg, 3:0 CopDbg, 11:8 MMapDbg } ID_DFR0;

// Secure Debug Enable Register
__register 32 { 1:1 SUNIDEN, 0:0 SUIDEN } SDER;

// Interrupt Controller End of Interrupt Status Register
__register 32 {  } ICH_EISR_EL2;

// External Debug Lock Status Register
__register 32 { 2:2 nTT, 0:0 SLI, 1:1 SLK } EDLSR;

// Activity Monitors Configuration Register
__register 32 { 31:28 NCG, 13:8 SIZE, 24:24 HDBG, 7:0 N } AMCFGR;

// CTI Component Identification Register 3
__register 32 { 7:0 PRMBL_3 } CTICIDR3;

// Statistical Profiling Control Register (EL1)
__register 64 { 3:3 CX, 1:1 E1SPE, 5:5 TS, 6:6 PCT, 0:0 E0SPE, 4:4 PA } PMSCR_EL1;

// Interrupt Controller Deactivate Interrupt Register
__register 32 { 23:0 INTID } ICC_DIR_EL1;

// Interrupt Controller List Registers
array [0..15] of __register 32 { 28:28 Group, 31:30 State, 23:16 Priority, 12:0 pINTID, 29:29 HW } ICH_LRC;

// Counter-timer Secure Physical Timer Control register (EL2)
__register 32 { 0:0 ENABLE, 1:1 IMASK, 2:2 ISTATUS } CNTHPS_CTL_EL2;

// Activity Monitors Component Identification Register 0
__register 32 { 7:0 PRMBL_0 } AMCIDR0;

// Interrupt Set-Pending Registers
array [0..31] of __register 32 {  } GICD_ISPENDR;

// Error Interrupt Status Register
__register 64 { 2:2 ERI, 3:3 ERIERR, 0:0 FHI, 5:5 CRIERR, 1:1 FHIERR, 4:4 CRI } ERRIRQSR;

// Interrupt Controller Deactivate Interrupt Register
__register 32 { 23:0 INTID } ICC_DIR;

// EL1 Read/Write Software Context Number
__register 64 {  } SCXTNUM_EL1;

// Interrupt Group Modifier Registers
array [0..31] of __register 32 {  } GICD_IGRPMODR;

// Performance Monitors Integration mode Control register
__register 32 { 0:0 IME } PMITCTRL;

// CPU Interface Control Register
__register 32 { 5:5 FIQBypDisGrp0, 8:8 IRQBypDisGrp1, 9:9 EOImodeS, 6:6 IRQBypDisGrp0, 4:4 CBPR, 3:3 FIQEn, 0:0 EnableGrp0, 9:9 EOImode, 10:10 EOImodeNS, 1:1 EnableGrp1, 7:7 FIQBypDisGrp1 } GICC_CTLR;

// Virtual Machine Interrupt Acknowledge Register
__register 32 { 24:0 INTID } GICV_IAR;

// Auxiliary Fault Status Register 0 (EL2)
__register 32 {  } AFSR0_EL2;

// AArch64 Auxiliary Feature Register 0
__register 64 {  } ID_AA64AFR0_EL1;

// Interrupt Clear-Pending Register 0
__register 32 {  } GICR_ICPENDR0;

// Monitor DCC Interrupt Enable Register
__register 32 { 29:29 TX, 30:30 RX } MDCCINT_EL1;

// OS Lock Access Register
__register 32 { 0:0 OSLK } OSLAR_EL1;

// MPAM Error Control Register
__register 32 { 0:0 INTEN } MPAMF_ECR;

// CPU Interface Highest Priority Pending Interrupt Register
__register 32 { 23:0 INTID } GICC_HPPIR;

// CTI Application Trigger Set register
__register 32 {  } CTIAPPSET;

// Interrupt Set-Active Registers
array [1..2] of __register 32 {  } GICR_ISACTIVERE;

// MPAM Priority Partitioning Identification Register
__register 32 { 17:17 DSPRI_0_IS_LOW, 9:4 INTPRI_WD, 1:1 INTPRI_0_IS_LOW, 0:0 HAS_INTPRI, 16:16 HAS_DSPRI, 25:20 DSPRI_WD } MPAMF_PRI_IDR;

// CTI Control register
__register 32 { 0:0 GLBEN } CTICONTROL;

// Exception Link Register (EL3)
__register 64 {  } ELR_EL3;

// CTI Input Trigger to Output Channel Enable registers
array [0..31] of __register 32 {  } CTIINEN;

// Monitor Debug Configuration Register (EL3)
__register 32 { 20:20 EDAD, 9:9 TDA, 17:17 SPME, 13:12 NSPB, 10:10 TDOSA, 19:19 TTRF, 23:23 SCCD, 21:21 EPMAD, 18:18 STE, 6:6 TPM, 15:14 SPD32, 16:16 SDD } MDCR_EL3;

// CPU Interface Running Priority Register
__register 32 { 7:0 Priority } GICC_RPR;

// Debug Saved Program Status Register
__register 32 { 15:10, 26:25 IT, 22:22 PAN, 6:6 F, 8:8 A, 9:9 E, 20:20 IL, 29:29 C, 30:30 Z, 31:31 N, 19:16 GE, 4:0 M, 23:23 SSBS, 5:5 T, 24:24 DIT, 7:7 I, 28:28 V, 27:27 Q, 21:21 SS } DSPSR;

// Interrupt Controller End Of Interrupt Register 1
__register 32 { 23:0 INTID } ICC_EOIR1_EL1;

// Counter-timer Secure Physical Timer TimerValue Register (EL2)
__register 32 { 31:0 TimerValue } CNTHPS_TVAL;

// MPAM Virtual Partition Mapping Valid Register
__register 64 {  } MPAMVPMV_EL2;

// Error Record ID Register
__register 32 { 15:0 NUM } ERRIDR;

// Interrupt Group Registers
array [1..2] of __register 32 {  } GICR_IGROUPRE;

// Interrupt Controller Type Register
__register 32 { 24:24 A3V, 23:19 IDbits, 15:11 num_LPIs, 7:5 CPUNumber, 16:16 MBIS, 25:25 No1N, 26:26 RSS, 18:18 DVIS, 10:10 SecurityExtn, 8:8 ESPI, 4:0 ITLinesNumber, 17:17 LPIS } GICD_TYPER;

// Counter-timer Physical Timer CompareValue register (EL2)
__register 64 { 63:0 CompareValue } CNTHP_CVAL_EL2;

// Floating-Point Status and Control Register
__register 32 { 28:28 V, 29:29 C, 25:25 DN, 26:26 AHP, 23:22 RMode, 21:20 Stride, 15:15 IDE, 24:24 FZ, 11:11 UFE, 18:16 Len, 31:31 N, 9:9 DZE, 30:30 Z, 4:4 IXC, 2:2 OFC, 1:1 DZC, 10:10 OFE, 27:27 QC, 0:0 IOC, 8:8 IOE, 19:19 FZ16, 3:3 UFC, 12:12 IXE, 7:7 IDC } FPSCR;

// Activity Monitors Event Type Registers 1
array [0..15] of __register 64 { 15:0 evtCount } AMEVTYPER1_EL0;

// Pointer Authentication Key A for Data (bits[127:64])
__register 64 {  } APDAKeyHi_EL1;

// MPAM3 Register (EL3)
__register 64 { 47:40 PMG_D, 63:63 MPAMEN, 39:32 PMG_I, 31:16 PARTID_D, 15:0 PARTID_I, 62:62 TRAPLOWER } MPAM3_EL3;

// Interrupt Controller System Register Enable register (EL2)
__register 32 { 3:3 Enable, 1:1 DFB, 0:0 SRE, 2:2 DIB } ICC_SRE_EL2;

// Counter-timer Secure Physical Timer CompareValue register (EL2)
__register 64 { 63:0 CompareValue } CNTHPS_CVAL_EL2;

// Current Exception Level
__register 64 { 3:2 EL } CurrentEL;

// Interrupt Controller Software Generated Interrupt Group 0 Register
__register 64 { 27:24 INTID, 15:0 TargetList, 55:48 Aff3, 47:44 RS, 40:40 IRM, 39:32 Aff2, 23:16 Aff1 } ICC_SGI0R_EL1;

// Activity Monitors Count Enable Clear Register 0
__register 32 {  } AMCNTENCLR0;

// Activity Monitors Peripheral Identification Register 0
__register 32 { 7:0 PART_0 } AMPIDR0;

// Performance Monitors Interrupt Enable Set register
__register 32 { 31:31 C } PMINTENSET_EL1;

// Debug Power Control Register
__register 32 { 0:0 CORENPDRQ } DBGPRCR;

// Selected Error Record Miscellaneous Register 3
__register 64 {  } ERXMISC3_EL1;

// Interrupt Set-Pending Register 0
__register 32 {  } GICR_ISPENDR0;

// Context ID Register (EL2)
__register 32 { 31:0 PROCID } CONTEXTIDR_EL2;

// Selected Error Record Miscellaneous Register 1
__register 64 {  } ERXMISC1_EL1;

// CTI Device Architecture register
__register 32 { 19:16 REVISION, 31:21 ARCHITECT, 20:20 PRESENT, 15:0 ARCHID } CTIDEVARCH;

// Interrupt Controller Interrupt Acknowledge Register 1
__register 32 { 23:0 INTID } ICC_IAR1;

// Report maximum PARTID and PMG Register
__register 32 { 15:0 PARTIDmax, 23:16 PMGmax } GICR_MPAMIDR;

// Saved Program Status Register (IRQ mode)
__register 32 { 6:6 F, 30:30 Z, 5:5 T, 7:7 I, 19:16 GE, 9:9 E, 27:27 Q, 4:0 M, 28:28 V, 31:31 N, 22:22 PAN, 8:8 A, 15:10, 26:25 IT, 24:24 J, 29:29 C, 21:21 DIT, 23:23 SSBS, 20:20 IL } SPSR_irq;

// Performance Monitors Count Enable Set register
__register 32 { 31:31 C } PMCNTENSET_EL0;

// Performance Monitors Interrupt Enable Clear register
__register 32 { 31:31 C } PMINTENCLR_EL1;

// Counter-timer EL0 Access Control Register
__register 32 { 1:1 EL0VCTEN, 9:9 EL0PTEN, 0:0 EL0PCTEN, 8:8 EL0VTEN } CNTEL0ACR;

// Selected Error Record Miscellaneous Register 4
__register 32 {  } ERXMISC4;

// Reset Management Register
__register 32 { 0:0 AA64, 1:1 RR } RMR;

// ITS Translation Table Descriptors
array [0..7] of __register 64 { 9:8 Page_Size, 62:62 Indirect, 58:56 Type, 47:12 Physical_Address, 7:0 Size, 52:48 Entry_Size, 63:63 Valid, 61:59 InnerCache, 11:10 Shareability, 55:53 OuterCache } GITS_BASER;

// Performance Monitors Selected Event Type Register
__register 32 {  } PMXEVTYPER;

// Interrupt Controller Virtual Highest Priority Pending Interrupt Register 0
__register 32 { 23:0 INTID } ICV_HPPIR0;

// Interrupt Set-Enable Register 0
__register 32 {  } GICR_ISENABLER0;

// Peripheral Identification Register 4
__register 32 { 3:0 DES_2, 7:4 SIZE } ERRPIDR4;

// Performance Monitors Machine Identification Register
__register 64 { 7:0 SLOTS } PMMIR_EL1;

// Auxiliary Instruction Fault Status Register
__register 32 {  } AIFSR;

// Media and VFP Feature Register 0
__register 32 { 27:24 FPShVec, 11:8 FPDP, 3:0 SIMDReg, 19:16 FPDivide, 31:28 FPRound, 7:4 FPSP, 23:20 FPSqrt, 15:12 FPTrap } MVFR0;

// Debug Breakpoint Extended Value Registers
array [0..15] of __register 32 { 15:8, 7:0 VMID, 31:0 ContextID2 } DBGBXVR;

// Non-Secure Access Control Register
__register 32 { 20:20 NSTRCDIS, 15:15 NSASEDIS, 10:10 cp10, 11:11 cp11 } NSACR;

// Auxiliary Memory Attribute Indirection Register (EL2)
__register 64 {  } AMAIR_EL2;

// Virtualization Translation Table Base Register
__register 64 { 55:48 VMID, 47:1 BADDR, 0:0 CnP } VTTBR;

// Counter-timer Hyp Physical Timer TimerValue register
__register 32 { 31:0 TimerValue } CNTHP_TVAL;

// Peripheral Identification Register 1
__register 32 { 3:0 PART_1, 7:4 DES_0 } ERRPIDR1;

// CPU Interface Non-secure Active Priorities Registers
array [0..3] of __register 32 {  } GICC_NSAPR;

// Interrupt Controller List Registers
array [0..15] of __register 32 { 31:0 vINTID } ICH_LR;

// Interrupt Controller Control Register (EL1)
__register 32 { 13:11 IDbits, 1:1 EOImode, 0:0 CBPR, 14:14 SEIS, 6:6 PMHE, 15:15 A3V, 10:8 PRIbits, 18:18 RSS, 19:19 ExtRange } ICC_CTLR_EL1;

// Activity Monitors Component Identification Register 1
__register 32 { 3:0 PRMBL_1, 7:4 CLASS } AMCIDR1;

// AArch32 Debug Feature Register 0
__register 32 { 27:24 PerfMon, 23:20 MProfDbg, 31:28 TraceFilt, 15:12 CopTrc, 19:16 MMapTrc, 7:4 CopSDbg, 11:8 MMapDbg, 3:0 CopDbg } ID_DFR0_EL1;

// Vector Base Address Register (EL3)
__register 64 {  } VBAR_EL3;

// CPU Interface Interrupt Acknowledge Register
__register 32 { 23:0 INTID } GICC_IAR;

// Pointer Authentication Key A for Data (bits[63:0])
__register 64 {  } APDAKeyLo_EL1;

// Selected Error Record Miscellaneous Register 3
__register 32 {  } ERXMISC3;

// Auxiliary Memory Attribute Indirection Register (EL1)
__register 64 {  } AMAIR_EL1;

// Counter-timer Virtual Timer Control
__register 32 { 2:2 ISTATUS, 0:0 ENABLE, 1:1 IMASK } CNTV_CTL;

// Performance Monitors Common Event Identification register 3
__register 32 {  } PMCEID3;

// Virtualization Translation Control Register
__register 32 { 9:8 IRGN0, 13:12 SH0, 15:14 TG0, 21:21 HA, 11:10 ORGN0, 27:27 HWU61, 28:28 HWU62, 26:26 HWU60, 25:25 HWU59, 7:6 SL0, 5:0 T0SZ, 29:29 NSW, 30:30 NSA, 18:16 PS, 22:22 HD, 19:19 VS } VTCR_EL2;

// Reset Management Register (EL1)
__register 32 { 0:0 AA64, 1:1 RR } RMR_EL1;

// Debug Device ID register 1
__register 32 { 3:0 PCSROffset } DBGDEVID1;

// Redistributor Type Register
__register 64 { 0:0 PLPIS, 3:3 DirectLPI, 1:1 VLPIS, 6:6 MPAM, 23:8 Processor_Number, 31:27 PPInum, 4:4 Last, 63:32 Affinity_Value, 5:5 DPGS, 25:24 CommonLPIAff, 2:2 Dirty } GICR_TYPER;

// Interrupt Status Register
__register 32 { 8:8 A, 7:7 I, 6:6 F } ISR;

// MPAM Hypervisor Control Register (EL2)
__register 64 { 1:1 EL1_VPMEN, 0:0 EL0_VPMEN, 31:31 TRAP_MPAMIDR_EL1, 8:8 GSTAPP_PLK } MPAMHCR_EL2;

// Performance Monitors Software Increment register
__register 32 {  } PMSWINC;

// Hyp Auxiliary Instruction Fault Status Register
__register 32 {  } HAIFSR;

// Secure Configuration Register
__register 32 { 9:9 SIF, 7:7 SCD, 6:6 nET, 13:13 TWE, 1:1 IRQ, 12:12 TWI, 8:8 HCE, 4:4 FW, 3:3 EA, 5:5 AW, 2:2 FIQ, 0:0 NS, 15:15 TERR } SCR;

// Counter-timer Secure Virtual Timer Control register (EL2)
__register 32 { 2:2 ISTATUS, 0:0 ENABLE, 1:1 IMASK } CNTHVS_CTL_EL2;

// Clear Non-secure SPI Pending Register
__register 32 { 12:0 INTID } GICD_CLRSPI_NSR;

// CPU Interface Priority Mask Register
__register 32 { 7:0 Priority } GICC_PMR;

// Auxiliary Control Register 2
__register 32 {  } ACTLR2;

// Interrupt Clear-Active Registers
array [0..31] of __register 32 {  } GICD_ICACTIVER;

// Profiling Buffer Write Pointer Register
__register 64 { 63:0 PTR } PMBPTR_EL1;

// AArch32 Instruction Set Attribute Register 4
__register 32 { 23:20 SynchPrim_frac, 7:4 WithShifts, 31:28 SWP_frac, 11:8 Writeback, 27:24 PSR_M, 19:16 Barrier, 3:0 Unpriv, 15:12 SMC } ID_ISAR4_EL1;

// Interrupt Clear-Enable Registers
array [0..31] of __register 32 {  } GICD_ICENABLER;

// Performance Monitors Component Identification Register 0
__register 32 { 7:0 PRMBL_0 } PMCIDR0;

// Interrupt Configuration Registers (Extended SPI Range)
array [0..63] of __register 32 {  } GICD_ICFGRE;

// Activity Monitors Count Enable Clear Register 0
__register 64 {  } AMCNTENCLR0_EL0;

// Performance Monitors Device Affinity register 0
__register 32 {  } PMDEVAFF0;

// MPAM Virtual PARTID Mapping Register 5
__register 64 { 31:16 PhyPARTID21, 15:0 PhyPARTID20, 63:48 PhyPARTID23, 47:32 PhyPARTID22 } MPAMVPM5_EL2;

// Performance Monitors Control Register
__register 32 { 6:6 LC, 1:1 P, 3:3 D, 2:2 C, 7:7 LP, 4:4 X, 0:0 E, 5:5 DP, 23:16 IDCODE, 31:24 IMP, 15:11 N } PMCR_EL0;

// Selected Error Record Address Register 2
__register 32 {  } ERXADDR2;

// Hyp Reset Management Register
__register 32 { 0:0 AA64, 1:1 RR } HRMR;

// Debug OS Double Lock Register
__register 32 { 0:0 DLK } DBGOSDLR;

// Interrupt Controller Interrupt Group 1 Enable register
__register 32 { 0:0 Enable } ICC_IGRPEN1_EL1;

// Interrupt Controller Virtual End Of Interrupt Register 1
__register 32 { 23:0 INTID } ICV_EOIR1;

// CPU Interface Binary Point Register
__register 32 { 2:0 Binary_Point } GICC_BPR;

// Secure Configuration Register
__register 32 { 26:26 ATA, 2:2 FIQ, 11:11 ST, 7:7 SMD, 0:0 NS, 20:20 NMEA, 12:12 TWI, 10:10 RW, 16:16 APK, 25:25 EnSCXT, 15:15 TERR, 13:13 TWE, 8:8 HCE, 1:1 IRQ, 21:21 FIEN, 14:14 TLOR, 9:9 SIF, 3:3 EA, 17:17 API, 19:19 EASE, 18:18 EEL2 } SCR_EL3;

// Counter-timer Hypervisor Control register
__register 32 { 3:3 EVNTDIR, 9:9 EL0PTEN, 11:11 EL1PTEN, 0:0 EL0PCTEN, 2:2 EVNTEN, 7:4 EVNTI, 8:8 EL0VTEN, 1:1 EL0VCTEN, 10:10 EL1PCTEN, 1:1 EL1PCEN } CNTHCTL_EL2;

// Interrupt Controller Virtual Highest Priority Pending Interrupt Register 0
__register 32 { 23:0 INTID } ICV_HPPIR0_EL1;

// Translation Table Base Register 0 (EL3)
__register 64 { 47:1 BADDR, 0:0 CnP } TTBR0_EL3;

// Cache Size Selection Register
__register 32 { 0:0 InD, 3:1 Level } CSSELR;

// Peripheral Identification Register 0
__register 32 { 7:0 PART_0 } ERRPIDR0;

// External Debug Peripheral Identification Register 4
__register 32 { 3:0 DES_2, 7:4 SIZE } EDPIDR4;

// Interrupt Controller Maintenance Interrupt State Register
__register 32 { 4:4 VGrp0E, 5:5 VGrp0D, 3:3 NP, 0:0 EOI, 7:7 VGrp1D, 1:1 U, 6:6 VGrp1E, 2:2 LRENP } ICH_MISR;

// CPU Interface Aliased End Of Interrupt Register
__register 32 { 23:0 INTID } GICC_AEOIR;

// Debug Watchpoint Control Registers
array [0..15] of __register 32 { 20:20 WT, 28:24 MASK, 19:16 LBN, 2:1 PAC, 12:5 BAS, 15:14 SSC, 0:0 E, 13:13 HMC, 4:3 LSC } DBGWCR;

// SVE Control Register for EL1
__register 64 { 3:0 LEN } ZCR_EL1;

// Hypervisor Control Register
__register 32 { 1:1 UIE, 4:4 VGrp0EIE, 0:0 En, 31:27 EOICount, 6:6 VGrp1EIE, 7:7 VGrp1DIE, 5:5 VGrp0DIE, 2:2 LRENPIE, 3:3 NPIE } GICH_HCR;

// AArch64 Processor Feature Register 0
__register 64 { 39:36 SEL2, 19:16 FP, 63:60 CSV3, 51:48 DIT, 47:44 AMU, 23:20 AdvSIMD, 3:0 EL0, 7:4 EL1, 59:56 CSV2, 27:24 GIC, 43:40 MPAM, 31:28 RAS, 11:8 EL2, 35:32 SVE, 15:12 EL3 } ID_AA64PFR0_EL1;

// Vector Base Address Register
__register 32 {  } VBAR;

// CPU Interface Aliased Interrupt Acknowledge Register
__register 32 { 23:0 INTID } GICC_AIAR;

// Instruction Fault Status Register
__register 32 { 5:0 STATUS, 9:9 LPAE, 12:12 ExT, 10:10, 3:0 FS, 16:16 FnV } IFSR;

// External Debug Feature Register
__register 64 { 7:4 TraceVer, 11:8 PMUVer, 15:12 BRPs, 31:28 CTX_CMPs, 23:20 WRPs, 43:40 TraceFilt } EDDFR;

// External Debug Lock Access Register
__register 32 { 31:0 KEY } EDLAR;

// List Registers
array [0..15] of __register 32 { 19:10 pINTID, 9:0 vINTID, 31:31 HW, 29:28 State, 30:30 Group, 27:23 Priority } GICH_LR;

// Saved Program Status Register (Hyp mode)
__register 32 { 28:28 V, 20:20 IL, 27:27 Q, 7:7 I, 4:0 M, 30:30 Z, 23:23 SSBS, 5:5 T, 29:29 C, 8:8 A, 15:10, 26:25 IT, 21:21 DIT, 24:24 J, 22:22 PAN, 19:16 GE, 6:6 F, 31:31 N, 9:9 E } SPSR_hyp;

// Monitor Debug ROM Address Register
__register 64 { 1:0 Valid, 51:48, 47:12 ROMADDR } MDRAR_EL1;

// Vector Base Address Register (EL1)
__register 64 {  } VBAR_EL1;

// External Debug Peripheral Identification Register 0
__register 32 { 7:0 PART_0 } EDPIDR0;

// Error Record Primary Status Register
array [0..65534] of __register 64 { 31:31 AV, 25:24 CE, 30:30 V, 27:27 OF, 21:20 UET, 15:8 IERR, 29:29 UE, 22:22 PN, 7:0 SERR, 26:26 MV, 23:23 DE, 19:19 CI, 28:28 ER } ERRSTATUS;

// Counter-timer Frequency
__register 32 {  } CNTFRQ;

// Selected Error Record Miscellaneous Register 2
__register 32 {  } ERXMISC2;

// Program Counter Sample Register
__register 64 { 63:63 NS, 62:61 EL } PMPCSR;

// Set LPI Pending Register
__register 64 { 31:0 pINTID } GICR_SETLPIR;

// Interrupt Clear-Active Registers
array [1..2] of __register 32 {  } GICR_ICACTIVERE;

// Empty List Register Status Register
__register 32 {  } GICH_ELRSR;

// Debug OS Lock Data Transfer Register, Receive, External View
__register 32 {  } DBGDTRRXext;

// MPAM Virtual PARTID Mapping Register 2
__register 64 { 15:0 PhyPARTID8, 63:48 PhyPARTID11, 47:32 PhyPARTID10, 31:16 PhyPARTID9 } MPAMVPM2_EL2;

// Auxiliary Fault Status Register 1 (EL2)
__register 32 {  } AFSR1_EL2;

// CTI Peripheral Identification Register 0
__register 32 { 7:0 PART_0 } CTIPIDR0;

// Error Record Address Register
array [0..65534] of __register 64 { 62:62 SI, 60:60 VA, 63:63 NS, 55:0 PADDR, 61:61 AI } ERRADDR;

// LORegion Number (EL1)
__register 64 { 7:0 Num } LORN_EL1;

// Sampling Event Filter Register
__register 64 { 18:18, 17:17, 11:11, 7:7, 5:5, 3:3, 1:1 E } PMSEVFR_EL1;

// Activity Monitors Device Affinity Register 0
__register 32 {  } AMDEVAFF0;

// Performance Monitors Selected Event Count Register
__register 32 {  } PMXEVCNTR;

// Hyp Auxiliary Memory Attribute Indirection Register 1
__register 32 {  } HAMAIR1;

// Interrupt Priority Registers
array [0..7] of __register 32 { 15:8 Priority_offset_1B, 31:24 Priority_offset_3B, 7:0 Priority_offset_0B, 23:16 Priority_offset_2B } GICR_IPRIORITYR;

// Pointer Authentication Key A for Instruction (bits[127:64])
__register 64 {  } APIAKeyHi_EL1;

// Virtual SError Exception Syndrome Register
__register 32 { 15:14 AET, 12:12 ExT } VDFSR;

// MPAM Memory System Monitor Configure Memory Bandwidth Usage Monitor Control Register
__register 32 { 31:31 EN, 27:27 CAPT_RESET, 17:17 MATCH_PMG, 16:16 MATCH_PARTID, 7:0 TYPE, 23:20 SUBTYPE, 25:25 OFLOW_INTR, 24:24 OFLOW_FRZ, 30:28 CAPT_EVNT, 26:26 OFLOW_STATUS } MSMON_CFG_MBWU_CTL;

// Performance Monitors Event Count Registers
array [0..30] of __register 32 {  } PMEVCNTR;

// Virtual Deferred Interrupt Status Register
__register 64 { 24:24 IDS, 23:0 ISS, 12:12 ExT, 15:14 AET } VSESR_EL2;

// MPAM Virtual PARTID Mapping Register 4
__register 64 { 31:16 PhyPARTID17, 15:0 PhyPARTID16, 63:48 PhyPARTID19, 47:32 PhyPARTID18 } MPAMVPM4_EL2;

// Hyp Instruction Fault Address Register
__register 32 {  } HIFAR;

// System Control Register
__register 32 { 20:20 UWXN, 2:2 C, 0:0 M, 6:6 UNK, 12:12 I, 8:8 SED, 29:29 AFE, 19:19 WXN, 16:16 nTWI, 13:13 V, 31:31 DSSBS, 7:7 ITD, 4:4 LSMAOE, 3:3 nTLSMD, 1:1 A, 10:10 EnRCTX, 30:30 TE, 25:25 EE, 23:23 SPAN, 5:5 CP15BEN, 28:28 TRE, 18:18 nTWE } SCTLR;

// AArch32 Processor Feature Register 1
__register 32 { 19:16 GenTimer, 7:4 Security, 27:24 Virt_frac, 23:20 Sec_frac, 15:12 Virtualization, 11:8 MProgMod, 3:0 ProgMod, 31:28 GIC } ID_PFR1_EL1;

// Counter-timer Physical Timer Control
__register 32 { 2:2 ISTATUS, 1:1 IMASK, 0:0 ENABLE } CNTP_CTL;

// Saved Program Status Register (Supervisor mode)
__register 32 { 5:5 T, 31:31 N, 8:8 A, 27:27 Q, 9:9 E, 19:16 GE, 21:21 DIT, 4:0 M, 22:22 PAN, 28:28 V, 23:23 SSBS, 15:10, 26:25 IT, 6:6 F, 30:30 Z, 20:20 IL, 29:29 C, 24:24 J, 7:7 I } SPSR_svc;

// Interrupt Controller Active Priorities Group 0 Registers
array [0..3] of __register 32 {  } ICC_AP0R;

// Revision ID Register
__register 32 {  } REVIDR;

// Counter-timer Virtual Timer CompareValue register
__register 64 { 63:0 CompareValue } CNTV_CVAL_EL0;

// Hyp Auxiliary Memory Attribute Indirection Register 0
__register 32 {  } HAMAIR0;

// MPAM Partion Configuration Selection Register
__register 32 { 16:16 INTERNAL, 15:0 PARTID_SEL } MPAMCFG_PART_SEL;

// Exception Link Register (Hyp mode)
__register 32 {  } ELR_hyp;

// Reset Vector Base Address Register (if EL3 implemented)
__register 64 {  } RVBAR_EL3;

// Activity Monitors Event Type Registers 0
array [0..15] of __register 32 { 15:0 evtCount } AMEVTYPER0;

// CPU Interface Active Priorities Registers
array [0..3] of __register 32 {  } GICC_APR;

// Interrupt Controller Virtual End Of Interrupt Register 0
__register 32 { 23:0 INTID } ICV_EOIR0;

// Virtual Machine Active Priorities Registers
array [0..3] of __register 32 {  } GICV_APR;

// Interrupt Controller Binary Point Register 1
__register 32 { 2:0 BinaryPoint } ICC_BPR1;

// Hyp Translation Control Register
__register 32 { 2:0 T0SZ, 27:27 HWU61, 25:25 HWU59, 28:28 HWU62, 26:26 HWU60, 24:24 HPD, 9:8 IRGN0, 11:10 ORGN0, 13:12 SH0 } HTCR;

// AArch32 Memory Model Feature Register 1
__register 32 { 3:0 L1HvdVA, 15:12 L1UniSW, 23:20 L1Uni, 31:28 BPred, 7:4 L1UniVA, 27:24 L1TstCln, 19:16 L1Hvd, 11:8 L1HvdSW } ID_MMFR1_EL1;

// Physical Address Register
__register 64 { 6:1 FS, 0:0 F, 6:1 FST, 6:4 Inner, 63:56 ATTR, 8:7 SH, 3:2 Outer, 9:9 NS, 9:9 FSTAGE, 8:8 S2WLK, 11:11 LPAE, 39:12 PA, 10:10 NOS, 1:1 SS } PAR;

// Error Record Miscellaneous Register 2
array [0..65534] of __register 64 {  } ERRMISC2;

// VMID Sample Register
__register 32 { 15:0 VMID } PMVIDSR;

// Performance Monitors Common Event Identification register 1
__register 32 {  } PMCEID1;

// Hyp Auxiliary Control Register
__register 32 {  } HACTLR;

// Interrupt Controller Virtual Interrupt Acknowledge Register 0
__register 32 { 23:0 INTID } ICV_IAR0_EL1;

// Interrupt Controller Virtual Binary Point Register 1
__register 32 { 2:0 BinaryPoint } ICV_BPR1_EL1;

// Performance Monitors Overflow Flag Status Register
__register 32 { 31:31 C } PMOVSR;

// Memory Model Feature Register 4
__register 32 { 19:16 HPDS, 11:8 XNX, 27:24 CCIDX, 23:20 LSM, 7:4 AC2, 15:12 CnP, 3:0 SpecSEI } ID_MMFR4;

// LORegion Control (EL1)
__register 64 { 0:0 EN, 9:2 DS } LORC_EL1;

// CTI Device Affinity register 1
__register 32 {  } CTIDEVAFF1;

// Interrupt Configuration Register 0
__register 32 {  } GICR_ICFGR0;

// Maintenance Interrupt Status Register
__register 32 { 7:7 VGrp1D, 3:3 NP, 4:4 VGrp0E, 6:6 VGrp1E, 1:1 U, 5:5 VGrp0D, 2:2 LRENP, 0:0 EOI } GICH_MISR;

// Debug Data Transfer Register, Transmit
__register 32 {  } DBGDTRTX_EL0;

// Counter-timer Virtual Timer CompareValue register (EL2)
__register 64 { 63:0 CompareValue } CNTHV_CVAL_EL2;

// Interrupt Controller End Of Interrupt Register 0
__register 32 { 23:0 INTID } ICC_EOIR0_EL1;

// Translation Table Base Control Register
__register 32 { 18:16 T1SZ, 13:12 SH0, 6:6 T2E, 5:5 PD1, 2:0 N, 29:28 SH1, 22:22 A1, 7:7 EPD0, 27:26 ORGN1, 2:0 T0SZ, 4:4 PD0, 25:24 IRGN1, 31:31 EAE, 23:23 EPD1, 9:8 IRGN0, 11:10 ORGN0 } TTBCR;

// AArch64 Auxiliary Feature Register 1
__register 64 {  } ID_AA64AFR1_EL1;

// MPAM Features Cache Portion Partitioning ID register
__register 32 { 15:0 CPBM_WD } MPAMF_CPOR_IDR;

// Peripheral Identification Register 2
__register 32 { 7:4 PART_2, 7:4 REVISION, 2:0 DES_1, 3:3 JEDEC } ERRPIDR2;

// FCSE Process ID register
__register 32 {  } FCSEIDR;

// Performance Monitors Count Enable Clear register
__register 32 { 31:31 C } PMCNTENCLR;

// Interrupt Routing Registers (Extended SPI Range)
array [0..1023] of __register 64 { 7:0 Aff0, 15:8 Aff1, 39:32 Aff3, 31:31 Interrupt_Routing_Mode, 23:16 Aff2 } GICD_IROUTERE;

// External Debug Component Identification Register 0
__register 32 { 7:0 PRMBL_0 } EDCIDR0;

// Interrupt Controller Active Priorities Group 1 Registers
array [0..3] of __register 64 {  } ICC_AP1R_EL1;

// Virtual Redistributor LPI Pending Table Base Address Register
__register 64 { 60:60 Dirty, 61:61 PendingLast, 63:63 Valid, 58:56 OuterCache, 51:16 Physical_Address, 9:7 InnerCache, 62:62 IDAI, 11:10 Shareability } GICR_VPENDBASER;

// Counter Status Register
__register 32 { 31:8 FCACK, 1:1 DBGH } CNTSR;

// SGI Set-Pending Registers
array [0..3] of __register 32 {  } GICD_SPENDSGIR;

// Auxiliary Control Register (EL1)
__register 64 {  } ACTLR_EL1;

// Counter-timer Virtual Timer TimerValue register
__register 32 { 31:0 TimerValue } CNTV_TVAL;

// AArch32 Instruction Set Attribute Register 5
__register 32 { 11:8 SHA1, 7:4 AES, 31:28 VCMA, 15:12 SHA2, 3:0 SEVL, 27:24 RDM, 19:16 CRC32 } ID_ISAR5_EL1;

// Selected Error Record Control Register 2
__register 32 {  } ERXCTLR2;

// MPAM Implementation-Specific Partitioning Feature Identification Register
__register 32 {  } MPAMF_IMPL_IDR;

// Fault-Handling Interrupt Configuration Register 0
__register 64 { 55:2 ADDR } ERRFHICR0;

// Floating-point Control Register
__register 32 { 15:15 IDE, 21:20 Stride, 12:12 IXE, 24:24 FZ, 11:11 UFE, 18:16 Len, 9:9 DZE, 8:8 IOE, 25:25 DN, 10:10 OFE, 26:26 AHP, 19:19 FZ16, 23:22 RMode } FPCR;

// Selected Error Record Address Register
__register 32 {  } ERXADDR;

// AArch32 Instruction Set Attribute Register 1
__register 32 { 3:0 Endian, 31:28 Jazelle, 15:12 Extend, 7:4 Except, 11:8 Except_AR, 19:16 IfThen, 27:24 Interwork, 23:20 Immediate } ID_ISAR1_EL1;

// Virtual Machine Control Register
__register 32 { 0:0 VENG0, 31:24 VPMR, 20:18 VBPR1, 9:9 VEOIM, 3:3 VFIQEn, 23:21 VBPR0, 4:4 VCBPR, 2:2 VAckCtl, 1:1 VENG1 } GICH_VMCR;

// Performance Monitors Peripheral Identification Register 2
__register 32 { 2:0 DES_1, 3:3 JEDEC, 7:4 REVISION } PMPIDR2;

// Counter-timer Virtual Count
__register 64 {  } CNTVCT;

// Interrupt Configuration Register 1
__register 32 {  } GICR_ICFGR1;

// Cache Size Selection Register
__register 32 { 4:4 TnD, 3:1 Level, 0:0 InD } CSSELR_EL1;

// Interrupt Controller Hyp System Register Enable register
__register 32 { 1:1 DFB, 2:2 DIB, 0:0 SRE, 3:3 Enable } ICC_HSRE;

// Hyp Syndrome Register
__register 32 { 15:0 imm16, 5:0 DFSC, 13:10 Rt2, 11:10 FnV, 24:24 CV, 31:26 EC, 3:0 coproc, 4:1 CRm, 24:24 ISV, 23:22 SAS, 8:5 Rt, 19:17 Opc2, 19:16 SRT, 19:16 Opc1, 5:5 TA, 19:12 imm8, 25:25 IL, 0:0 Direction, 5:0 IFSC, 4:4 Offset, 8:8 CM, 21:21 SSE, 13:10 CRn, 11:10 AET, 19:19 CCKNOWNPASS, 3:1 AM, 14:14 AR, 0:0 TI, 6:6 WnR, 23:20 COND, 9:9 EA, 8:5 Rn, 7:7 S1PTW } HSR;

// Interrupt Controller Virtual Control Register
__register 32 { 15:15 A3V, 1:1 EOImode, 0:0 CBPR, 18:18 RSS, 14:14 SEIS, 13:11 IDbits, 10:8 PRIbits, 19:19 ExtRange } ICV_CTLR_EL1;

// Virtualization Secure Translation Table Base Register
__register 64 { 0:0 CnP, 47:1 BADDR } VSTTBR_EL2;

// Interrupt Controller System Register Enable register
__register 32 { 0:0 SRE, 1:1 DFB, 2:2 DIB } ICC_SRE;

// Activity Monitors Counter Group Configuration Register
__register 32 { 7:0 CG0NC, 15:8 CG1NC } AMCGCR;

// Debug Watchpoint Control Registers
array [0..15] of __register 64 { 20:20 WT, 13:13 HMC, 2:1 PAC, 28:24 MASK, 12:5 BAS, 0:0 E, 19:16 LBN, 15:14 SSC, 4:3 LSC } DBGWCR_EL1;

// End Interrupt Status Register
__register 32 {  } GICH_EISR;

// Performance Monitors Cycle Counter
__register 64 { 63:0 CCNT } PMCCNTR_EL0;

// Performance Monitors Common Event Identification register 2
__register 32 {  } PMCEID2;

// MPAM Virtual PARTID Mapping Register 3
__register 64 { 31:16 PhyPARTID13, 15:0 PhyPARTID12, 47:32 PhyPARTID14, 63:48 PhyPARTID15 } MPAMVPM3_EL2;

// Interrupt Controller System Register Enable register (EL3)
__register 32 { 3:3 Enable, 1:1 DFB, 2:2 DIB, 0:0 SRE } ICC_SRE_EL3;

// Counter-timer Frequency register
__register 32 {  } CNTFRQ_EL0;

// Non-secure Access Control Registers
array [0..63] of __register 32 {  } GICD_NSACR;

// Interrupt Set-Active Registers
array [0..31] of __register 32 {  } GICD_ISACTIVER;

// Interrupt Controller Virtual Interrupt Priority Mask Register
__register 32 { 7:0 Priority } ICV_PMR_EL1;

// Component Identification Register 1
__register 32 { 7:4 CLASS, 3:0 PRMBL_1 } ERRCIDR1;

// Interrupt Controller Virtual Interrupt Group 0 Enable register
__register 32 { 0:0 Enable } ICV_IGRPEN0;

// Redistributor Wake Register
__register 32 { 1:1 ProcessorSleep, 2:2 ChildrenAsleep } GICR_WAKER;

// Interrupt Clear-Enable Registers
array [0..31] of __register 32 {  } GICD_ICENABLERE;

// Hyp Software Thread ID Register
__register 32 {  } HTPIDR;

// Architectural Feature Trap Register (EL2)
__register 32 { 21:20 FPEN, 8:8 TZ, 31:31 TCPAC, 17:16 ZEN, 30:30 TAM, 10:10 TFP, 28:28 TTA } CPTR_EL2;

// Debug OS Lock Status Register
__register 32 { 2:2 nTT, 1:1 OSLK, 3:3, 0:0 OSLM } DBGOSLSR;

// Interrupt Controller Virtual Interrupt Group 1 Enable register
__register 32 { 0:0 Enable } ICV_IGRPEN1_EL1;

// Counter-timer Hyp Physical Timer Control register
__register 32 { 2:2 ISTATUS, 0:0 ENABLE, 1:1 IMASK } CNTHP_CTL;

// CTI Input Channel to Output Trigger Enable registers
array [0..31] of __register 32 {  } CTIOUTEN;

// Hypervisor Auxiliary Control Register
__register 32 {  } HACR_EL2;

// MPAM Features Identification Register
__register 32 { 29:29 HAS_IMPL_IDR, 23:16 PMG_MAX, 25:25 HAS_CPOR_PART, 30:30 HAS_MSMON, 27:27 HAS_PRI_PART, 31:31 HAS_PARTID_NRW, 15:0 PARTID_MAX, 26:26 HAS_MBW_PART, 24:24 HAS_CCAP_PART } MPAMF_IDR;

// Interrupt Controller Deactivate Virtual Interrupt Register
__register 32 { 23:0 INTID } ICV_DIR_EL1;

// Critical Error Interrupt Configuration Register 1
__register 32 { 31:0 DATA } ERRCRICR1;

// Saved Program Status Register
__register 32 { 27:27 Q, 20:20 IL, 5:5 T, 19:16 GE, 31:31 N, 6:6 F, 22:22 PAN, 21:21 DIT, 9:9 E, 29:29 C, 28:28 V, 30:30 Z, 24:24 J, 4:4, 3:0 M, 23:23 SSBS, 7:7 I, 8:8 A, 15:10, 26:25 IT } SPSR;

// External Debug Status and Control Register
__register 32 { 6:6 ERR, 29:29 TXfull, 23:22 INTdis, 31:31 TFO, 18:18 NS, 30:30 RXfull, 27:27 RXO, 13:10 RW, 21:21 TDA, 19:19 SC2, 14:14 HDE, 7:7 A, 28:28 ITO, 5:0 STATUS, 25:25 PipeAdv, 20:20 MA, 16:16 SDD, 26:26 TXU, 24:24 ITE, 9:8 EL } EDSCR;

// Reset Vector Base Address Register (if EL3 not implemented)
__register 64 {  } RVBAR_EL2;

// Interrupt Controller Interrupt Group 1 Enable register (EL3)
__register 32 { 0:0 EnableGrp1NS, 1:1 EnableGrp1S } ICC_IGRPEN1_EL3;

// Multiprocessor Affinity Register
__register 64 { 23:16 Aff2, 30:30 U, 15:8 Aff1, 39:32 Aff3, 7:0 Aff0, 24:24 MT } MPIDR_EL1;

// Redistributor Properties Base Address Register
__register 64 { 4:0 IDbits, 58:56 OuterCache, 9:7 InnerCache, 51:12 Physical_Address, 11:10 Shareability } GICR_PROPBASER;

// Translation Table Base Register 1 (EL2)
__register 64 { 0:0 CnP, 47:1 BADDR, 63:48 ASID } TTBR1_EL2;

// Auxiliary Fault Status Register 0 (EL1)
__register 32 {  } AFSR0_EL1;

// Counter-timer Hyp Physical CompareValue register
__register 64 { 63:0 CompareValue } CNTHP_CVAL;

// Auxiliary Fault Status Register 1 (EL1)
__register 32 {  } AFSR1_EL1;

// Memory Model Feature Register 0
__register 32 { 7:4 PMSA, 15:12 ShareLvl, 11:8 OuterShr, 3:0 VMSA, 27:24 FCSE, 23:20 AuxReg, 19:16 TCM, 31:28 InnerShr } ID_MMFR0;

// Interrupt Controller Control Register (EL3)
__register 32 { 4:4 EOImode_EL1NS, 17:17 nDS, 15:15 A3V, 3:3 EOImode_EL1S, 13:11 IDbits, 1:1 CBPR_EL1NS, 0:0 CBPR_EL1S, 18:18 RSS, 2:2 EOImode_EL3, 14:14 SEIS, 19:19 ExtRange, 6:6 PMHE, 10:8 PRIbits, 5:5 RM } ICC_CTLR_EL3;

// Deferred Interrupt Status Register
__register 32 { 9:9 LPAE, 10:10, 3:0 FS, 5:0 STATUS, 9:9 EA, 31:31 A, 5:0 DFSC, 15:14 AET, 12:12 ExT } DISR;

// Selected Error Record Miscellaneous Register 1
__register 32 {  } ERXMISC1;

// Activity Monitors Component Identification Register 3
__register 32 { 7:0 PRMBL_3 } AMCIDR3;

// Interrupt Controller Virtual Binary Point Register 0
__register 32 { 2:0 BinaryPoint } ICV_BPR0;

// Revision ID Register
__register 32 {  } REVIDR_EL1;

// Interrupt Controller Virtual Machine Control Register
__register 32 { 23:21 VBPR0, 20:18 VBPR1, 2:2 VAckCtl, 31:24 VPMR, 0:0 VENG0, 3:3 VFIQEn, 4:4 VCBPR, 1:1 VENG1, 9:9 VEOIM } ICH_VMCR;

// Virtual Machine CPU Interface Identification Register
__register 32 { 31:20 ProductID, 15:12 Revision, 19:16 Architecture_version, 11:0 Implementer } GICV_IIDR;

// Performance Monitors Overflow Flag Status Set register
__register 32 { 31:31 C } PMOVSSET_EL0;

// Interrupt Controller Empty List Register Status Register
__register 32 {  } ICH_ELRSR;

// Floating-Point Exception Control register
__register 32 { 10:8 VECITR, 3:3 UFF, 27:27 VV, 31:31 EX, 2:2 OFF, 30:30 EN, 29:29 DEX, 1:1 DZF, 28:28 FP2V, 4:4 IXF, 0:0 IOF, 26:26 TFV, 7:7 IDF } FPEXC32_EL2;

// CTI Peripheral Identification Register 4
__register 32 { 3:0 DES_2, 7:4 SIZE } CTIPIDR4;

// CTI Device ID register 2
__register 32 {  } CTIDEVID2;

// External Debug Exception Catch Control Register
__register 32 {  } EDECCR;

// MPAM Partion Configuration Selection Register
__register 32 { 7:0 MON_SEL } MSMON_CFG_MON_SEL;

// CTI Device Control register
__register 32 { 0:0 OSUCE, 1:1 RCE } CTIDEVCTL;

// Selected Error Record Miscellaneous Register 7
__register 32 {  } ERXMISC7;

// EL3 Read/Write Software Context Number
__register 64 {  } SCXTNUM_EL3;

// Performance Monitors Common Event Identification register 0
__register 64 {  } PMCEID0_EL0;

// Interrupt Controller End Of Interrupt Register 0
__register 32 { 23:0 INTID } ICC_EOIR0;

// Performance Monitors User Enable Register
__register 32 { 3:3 ER, 1:1 SW, 2:2 CR, 0:0 EN } PMUSERENR;

// Peripheral Identification Register 3
__register 32 { 7:4 REVAND, 7:4 REVISION, 3:0 CMOD } ERRPIDR3;

// ITS Translation Register
__register 32 { 31:0 EventID } GITS_TRANSLATER;

// CTI Device Affinity register 0
__register 32 {  } CTIDEVAFF0;

// Memory Attribute Indirection Register (EL3)
__register 64 {  } MAIR_EL3;

// Fault Address Register (EL1)
__register 64 {  } FAR_EL1;

// Hyp Debug Control Register
__register 32 { 9:9 TDA, 23:23 HCCD, 8:8 TDE, 26:26 HLP, 11:11 TDRA, 7:7 HPME, 4:0 HPMN, 19:19 TTRF, 10:10 TDOSA, 17:17 HPMD, 6:6 TPM, 5:5 TPMCR } HDCR;

// Saved Program Status Register (EL2)
__register 32 { 19:16 GE, 5:5 T, 8:8 A, 7:7 I, 12:12 SSBS, 22:22 PAN, 4:4, 4:4, 3:0, 3:0 M, 31:31 N, 30:30 Z, 27:27 Q, 11:10 BTYPE, 25:25 TCO, 9:9 E, 20:20 IL, 21:21 SS, 29:29 C, 6:6 F, 9:9 D, 15:10, 26:25 IT, 24:24 DIT, 28:28 V, 23:23 UAO } SPSR_EL2;

// Clear Secure SPI Pending Register
__register 32 { 12:0 INTID } GICD_CLRSPI_SR;

// Debug Breakpoint Control Registers
array [0..15] of __register 32 { 13:13 HMC, 8:5 BAS, 2:1 PMC, 0:0 E, 19:16 LBN, 15:14 SSC, 23:20 BT } DBGBCR;

// Counter-timer Virtual Timer Control register (EL2)
__register 32 { 1:1 IMASK, 2:2 ISTATUS, 0:0 ENABLE } CNTHV_CTL;

// Debug Saved Program Status Register
__register 32 { 30:30 Z, 20:20 IL, 11:10 BTYPE, 15:10, 26:25 IT, 29:29 C, 28:28 V, 22:22 PAN, 21:21 SS, 7:7 I, 27:27 Q, 8:8 A, 9:9 E, 12:12 SSBS, 24:24 DIT, 25:25 TCO, 9:9 D, 19:16 GE, 5:5 T, 31:31 N, 6:6 F, 23:23 UAO, 4:4, 4:4, 3:0, 3:0 M } DSPSR_EL0;

// Hyp Auxiliary Configuration Register
__register 32 {  } HACR;

// CTI Lock Status Register
__register 32 { 2:2 nTT, 0:0 SLI, 1:1 SLK } CTILSR;

// Performance Monitors Control Register
__register 32 { 0:0 E, 2:2 C, 1:1 P, 7:7 LP, 6:6 LC, 4:4 X, 15:11 N, 23:16 IDCODE, 31:24 IMP, 5:5 DP, 3:3 D } PMCR;

// Auxiliary Memory Attribute Indirection Register 1
__register 32 {  } AMAIR1;

// Processor Feature Register 0
__register 32 { 27:24 DIT, 11:8 State2, 19:16 CSV2, 7:4 State1, 15:12 State3, 3:0 State0, 31:28 RAS, 23:20 AMU } ID_PFR0;

// Condition Flags
__register 32 { 28:28 V, 31:31 N, 29:29 C, 30:30 Z } NZCV;

// Tag Fail Status Register (EL1).
__register 64 { 1:1 TF1, 0:0 TF0 } TFSR_EL1;

// Activity Monitors Peripheral Identification Register 3
__register 32 { 3:0 CMOD, 7:4 REVAND } AMPIDR3;

// Error Record Feature Register
array [0..65534] of __register 64 { 9:8 UE, 7:6 FI, 5:4 UI, 1:0 ED, 19:18 CEO, 14:12 CEC, 11:10 CFI, 21:20 INJ, 23:22 CI, 25:24 TS, 17:16 DUI, 15:15 RP } ERRFR;

// Set Secure SPI Pending Register
__register 32 { 12:0 INTID } GICD_SETSPI_SR;

// Performance Monitors Selected Event Type Register
__register 32 {  } PMXEVTYPER_EL0;

// Interrupt Controller Hyp Active Priorities Group 0 Registers
array [0..3] of __register 64 {  } ICH_AP0R_EL2;

// Activity Monitors User Enable Register
__register 64 { 0:0 EN } AMUSERENR_EL0;

// Interrupt Controller Interrupt Priority Mask Register
__register 32 { 7:0 Priority } ICC_PMR_EL1;

// Cache Level ID Register
__register 64 { 32:30 ICB, 29:27 LoUU, 26:24 LoC, 23:21 LoUIS } CLIDR_EL1;

// Sampling Filter Control Register
__register 64 { 0:0 FE, 18:18 ST, 17:17 LD, 2:2 FL, 1:1 FT, 16:16 B } PMSFCR_EL1;

// Debug Self Address Register
__register 64 { 63:0 Offset } DBGDSAR;

// Hyp Configuration Register
__register 32 { 21:21 TAC, 4:4 IMO, 1:1 SWIO, 9:9 FB, 13:13 TWI, 25:25 TTLB, 12:12 DC, 7:7 VI, 24:24 TPU, 27:27 TGE, 23:23 TPC, 11:10 BSU, 6:6 VF, 17:17 TID2, 8:8 VA, 3:3 FMO, 2:2 PTW, 30:30 TRVM, 19:19 TSC, 18:18 TID3, 29:29 HCD, 20:20 TIDCP, 15:15 TID0, 26:26 TVM, 16:16 TID1, 5:5 AMO, 0:0 VM, 14:14 TWE, 22:22 TSW } HCR;

// Selected Pseudo-fault Generation Feature Register
__register 64 {  } ERXPFGF_EL1;

// Interrupt Clear-Active Register 0
__register 32 {  } GICR_ICACTIVER0;

// MPAM Priority Partition Configuration Register
__register 32 { 15:0 INTPRI, 31:16 DSPRI } MPAMCFG_PRI;

// Interrupt Controller Running Priority Register
__register 32 { 7:0 Priority } ICC_RPR;

// Interrupt Controller Virtual End Of Interrupt Register 1
__register 32 { 23:0 INTID } ICV_EOIR1_EL1;

// Performance Monitors Event Count Registers
array [0..30] of __register 64 {  } PMEVCNTR_EL0;

// Hyp Memory Attribute Indirection Register 0
__register 32 {  } HMAIR0;

// Performance Monitors Cycle Count Filter Register
__register 32 { 28:28 NSU, 29:29 NSK, 27:27 NSH, 31:31 P, 30:30 U } PMCCFILTR;

// Interrupt Controller Binary Point Register 0
__register 32 { 2:0 BinaryPoint } ICC_BPR0;

// MPAM Memory Bandwidth Partitioning Identification Register
__register 32 { 14:14 WINDWR, 28:16 BWPBM_WD, 11:11 HAS_MAX, 10:10 HAS_MIN, 13:13 HAS_PROP, 5:0 BWA_WD, 12:12 HAS_PBM } MPAMF_MBW_IDR;

// Implementation Identification Register
__register 32 { 19:16 Variant, 31:20 ProductID, 11:0 Implementer, 15:12 Revision } ERRIIDR;

// Privileged Access Never
__register 32 { 22:22 PAN } PAN;

// Virtualization Translation Table Base Register
__register 64 { 63:56, 55:48 VMID, 47:1 BADDR, 0:0 CnP } VTTBR_EL2;

// MPAM Cache Maximum Capacity Partition Configuration Register
__register 32 { 15:0 CMAX } MPAMCFG_CMAX;

// EL3 Software Thread ID Register
__register 64 {  } TPIDR_EL3;

// Activity Monitors Configuration Register
__register 64 { 31:28 NCG, 24:24 HDBG, 7:0 N, 13:8 SIZE } AMCFGR_EL0;

// Debug Breakpoint Control Registers
array [0..15] of __register 32 { 23:20 BT, 19:16 LBN, 13:13 HMC, 15:14 SSC, 2:1 PMC, 0:0 E, 8:5 BAS } DBGBCR_EL1;

// Reset Management Register (EL2)
__register 32 { 1:1 RR, 0:0 AA64 } RMR_EL2;

// Counter-timer Secure Physical Timer CompareValue Register (EL2)
__register 64 { 63:0 CompareValue } CNTHPS_CVAL;

// CTI Channel Out Status register
__register 32 {  } CTICHOUTSTATUS;

// Virtual Machine Highest Priority Pending Interrupt Register
__register 32 { 24:0 INTID } GICV_HPPIR;

// MPAM Architecture Identification Register
__register 32 { 3:0 ArchMinorRev, 7:4 ArchMajorRev } MPAMF_AIDR;

// Selected Error Record Control Register
__register 32 {  } ERXCTLR;

// Exception Syndrome Register (EL1)
__register 32 { 24:24 IDS, 5:0 IFSC, 21:20 Op0, 3:1 AM, 16:14 Op1, 6:6 WnR, 23:23 TFV, 4:4 IXF, 12:10 AET, 6:6 EX, 0:0 IOF, 19:16 Opc1, 31:26 EC, 15:0 imm16, 19:19 CCKNOWNPASS, 25:25 IL, 23:20 COND, 15:0 Comment, 14:10 Rt2, 9:9 EA, 13:13 VNCR, 1:1 ERET, 12:11 SET, 2:2 OFF, 0:0 ERETA, 24:24 CV, 4:4 Offset, 14:14 AR, 4:1 CRm, 0:0 Direction, 23:22 SAS, 20:16 SRT, 7:7 S1PTW, 24:24 ISV, 10:10 FnV, 0:0 TI, 19:17 Opc2, 10:8 VECITR, 19:17 Op2, 7:7 IDF, 21:21 SSE, 9:5 Rt, 5:0 DFSC, 8:8 CM, 1:1 DZF, 13:13 IESB, 19:12 imm8, 9:5 Rn, 13:10 CRn, 15:15 SF, 3:3 UFF, 1:0 BTYPE } ESR_EL1;

// Data Fault Status Register
__register 32 { 13:13 CM, 7:4 Domain, 15:14 AET, 12:12 ExT, 11:11 WnR, 9:9 LPAE, 16:16 FnV, 10:10, 3:0 FS, 5:0 STATUS } DFSR;

// Counter-timer Kernel Control register
__register 32 { 0:0 EL0PCTEN, 9:9 EL0PTEN, 8:8 EL0VTEN, 2:2 EVNTEN, 7:4 EVNTI, 3:3 EVNTDIR, 1:1 EL0VCTEN } CNTKCTL_EL1;

// Jazelle Main Configuration Register
__register 32 {  } JMCR;

// Debug Authentication Status register
__register 32 { 1:0 NSID, 5:4 SID, 7:6 SNID, 3:2 NSNID } DBGAUTHSTATUS_EL1;

// TCM Type Register
__register 32 {  } TCMTR;

// Instruction Set Attribute Register 6
__register 32 { 15:12 SB, 19:16 SPECRES, 7:4 DP, 3:0 JSCVT, 11:8 FHM } ID_ISAR6;

// Interrupt Controller Active Priorities Group 1 Registers
array [0..3] of __register 32 {  } ICC_AP1R;

// Debug Authentication Status register
__register 32 { 5:4 SID, 7:6 SNID, 1:0 NSID, 3:2 NSNID } DBGAUTHSTATUS;

// Set PARTID and PMG Register
__register 32 { 23:16 PMG, 15:0 PARTID } GITS_PARTIDR;

// Auxiliary Control Register (EL2)
__register 64 {  } ACTLR_EL2;

// Debug Breakpoint Value Registers
array [0..15] of __register 64 { 52:49, 48:2 VA, 31:0 ContextID, 63:53, 52:49 RESS, 63:32 ContextID2, 47:40, 47:40, 39:32, 39:32 VMID } DBGBVR_EL1;

// Interrupt Priority Registers (extended PPI range)
array [8..23] of __register 32 { 31:24 Priority_offset_3B, 15:8 Priority_offset_1B, 7:0 Priority_offset_0B, 23:16 Priority_offset_2B } GICR_IPRIORITYRE;

// Debug Link Register
__register 64 {  } DLR_EL0;

// SVE Control Register for EL3
__register 64 { 3:0 LEN } ZCR_EL3;

// Translation Control Register (EL1)
__register 64 { 50:50 HWU162, 44:44 HWU060, 52:52 TBID1, 58:58 TCMA1, 48:48 HWU160, 39:39 HA, 15:14 TG0, 5:0 T0SZ, 42:42 HPD1, 11:10 ORGN0, 31:30 TG1, 27:26 ORGN1, 34:32 IPS, 29:28 SH1, 25:24 IRGN1, 47:47 HWU159, 49:49 HWU161, 36:36 AS, 56:56 E0PD1, 40:40 HD, 55:55 E0PD0, 37:37 TBI0, 53:53 NFD0, 46:46 HWU062, 13:12 SH0, 7:7 EPD0, 54:54 NFD1, 45:45 HWU061, 9:8 IRGN0, 21:16 T1SZ, 57:57 TCMA0, 41:41 HPD0, 23:23 EPD1, 38:38 TBI1, 22:22 A1, 43:43 HWU059, 51:51 TBID0 } TCR_EL1;

// Debug Device ID register 2
__register 32 {  } DBGDEVID2;

// Pseudo-fault Generation Control Register
array [0..65534] of __register 64 { 8:8 CI, 10:10 PN, 31:31 CDNEN, 5:5 DE, 30:30 R, 9:9 ER, 3:3 UER, 1:1 UC, 11:11 AV, 0:0 OF, 2:2 UEU, 12:12 MV, 7:6 CE, 4:4 UEO } ERRPFGCTL;

// Counter-timer Physical Count register
__register 64 {  } CNTPCT_EL0;

// Selected Error Record Feature Register
__register 64 {  } ERXFR_EL1;

// Instruction Fault Status Register (EL2)
__register 32 { 12:12 ExT, 9:9 LPAE, 16:16 FnV, 10:10, 3:0 FS, 5:0 STATUS } IFSR32_EL2;

// Virtualization Processor ID Register
__register 32 { 19:16 Architecture, 31:24 Implementer, 15:4 PartNum, 23:20 Variant, 3:0 Revision } VPIDR;

// Counter-timer Secure Virtual Timer TimerValue register (EL2)
__register 32 { 31:0 TimerValue } CNTHVS_TVAL_EL2;

// MPAM Memory System Monitor Configure Memory Bandwidth Usage Monitor Filter Register
__register 32 { 23:16 PMG, 15:0 PARTID } MSMON_CFG_MBWU_FLT;

// Performance Monitors Configuration Register
__register 32 { 18:18 WT, 14:14 CC, 19:19 UEN, 17:17 NA, 13:8 SIZE, 31:28 NCG, 15:15 CCD, 7:0 N, 16:16 EX } PMCFGR;

// Memory Attribute Indirection Register 0
__register 32 {  } MAIR0;

// Hyp Architectural Feature Trap Register
__register 32 { 20:20 TTA, 31:31 TCPAC, 11:11 TCP11, 10:10 TCP10, 15:15 TASE, 30:30 TAM } HCPTR;

// Clear LPI Pending Register
__register 64 { 31:0 pINTID } GICR_CLRLPIR;

// CONTEXTIDR_EL1 Sample Register
__register 32 { 31:0 CONTEXTIDR_EL1 } PMCID1SR;

// Error Recovery Interrupt Configuration Register 1
__register 32 { 31:0 DATA } ERRERICR1;

// OS Lock Data Transfer Register, Transmit
__register 32 {  } OSDTRTX_EL1;

// Counter Frequency IDs, n > 0
__register 32 { 31:0 Frequency } CNTFID;

// Stack Pointer (EL1)
__register 64 {  } SP_EL1;

// Interrupt Controller Highest Priority Pending Interrupt Register 1
__register 32 { 23:0 INTID } ICC_HPPIR1;

// Counter-timer Non-secure Access Register
__register 32 {  } CNTNSAR;

// Translation Table Base Register 0 (EL2)
__register 64 { 63:48 ASID, 47:1 BADDR, 0:0 CnP } TTBR0_EL2;

// LORegion Start Address (EL1)
__register 64 { 0:0 Valid, 51:48, 47:16 SA } LORSA_EL1;

// Activity Monitors Count Enable Clear Register 1
__register 64 {  } AMCNTENCLR1_EL0;

// Interrupt Controller Virtual Active Priorities Group 0 Registers
array [0..3] of __register 32 {  } ICV_AP0R;

// MPAM Virtual PARTID Mapping Register 0
__register 64 { 31:16 PhyPARTID1, 15:0 PhyPARTID0, 47:32 PhyPARTID2, 63:48 PhyPARTID3 } MPAMVPM0_EL2;

// Interrupt Routing Registers
array [32..1019] of __register 64 { 39:32 Aff3, 23:16 Aff2, 7:0 Aff0, 31:31 Interrupt_Routing_Mode, 15:8 Aff1 } GICD_IROUTER;

// AArch32 Media and VFP Feature Register 2
__register 32 { 7:4 FPMisc, 3:0 SIMDMisc } MVFR2_EL1;

// Performance Monitors Common Event Identification register 0
__register 32 {  } PMCEID0;

// Trace Filter Control Register (EL2)
__register 64 { 6:5 TS, 3:3 CX, 0:0 E0HTRE, 1:1 E2TRE } TRFCR_EL2;

// Tag Control Register.
__register 64 { 15:0 Exclude, 16:16 RRND } GCR_EL1;

// Interrupt Status Register
__register 32 { 7:7 I, 6:6 F, 8:8 A } ISR_EL1;

// External Debug Integration mode Control register
__register 32 { 0:0 IME } EDITCTRL;

// AArch32 Instruction Set Attribute Register 0
__register 32 { 19:16 Coproc, 7:4 BitCount, 11:8 BitField, 3:0 Swap, 15:12 CmpBranch, 27:24 Divide, 23:20 Debug } ID_ISAR0_EL1;

// Interrupt Controller Interrupt Group 1 Enable register
__register 32 { 0:0 Enable } ICC_IGRPEN1;

// OS Lock Exception Catch Control Register
__register 32 { 31:0 EDECCR } OSECCR_EL1;

// Non-secure Access Control Register
__register 32 {  } GICR_NSACR;

// Jazelle OS Control Register
__register 32 {  } JOSCR;

// Hyp Trace Filter Control Register
__register 32 { 0:0 E0HTRE, 6:5 TS, 1:1 E2TRE, 3:3 CX } HTRFCR;

// Pointer Authentication Key A for Code (bits[63:0])
__register 64 {  } APGAKeyLo_EL1;

// AArch64 Debug Feature Register 1
__register 64 {  } ID_AA64DFR1_EL1;

// CTI Channel Gate Enable register
__register 32 {  } CTIGATE;

// Stack Pointer (EL3)
__register 64 {  } SP_EL3;

// Interrupt Controller Virtual Binary Point Register 0
__register 32 { 2:0 BinaryPoint } ICV_BPR0_EL1;

// Interrupt Clear-Enable Registers
array [1..2] of __register 32 {  } GICR_ICENABLERE;

// Hyp Auxiliary Control Register 2
__register 32 {  } HACTLR2;

// Instruction Set Attribute Register 0
__register 32 { 19:16 Coproc, 7:4 BitCount, 23:20 Debug, 15:12 CmpBranch, 3:0 Swap, 27:24 Divide, 11:8 BitField } ID_ISAR0;

// Virtual Machine Aliased Highest Priority Pending Interrupt Register
__register 32 { 24:0 INTID } GICV_AHPPIR;

// Saved Program Status Register (FIQ mode)
__register 32 { 31:31 N, 22:22 PAN, 29:29 C, 23:23 SSBS, 28:28 V, 27:27 Q, 8:8 A, 15:10, 26:25 IT, 24:24 J, 6:6 F, 30:30 Z, 21:21 DIT, 20:20 IL, 9:9 E, 5:5 T, 4:0 M, 7:7 I, 19:16 GE } SPSR_fiq;

// Component Identification Register 3
__register 32 { 7:0 PRMBL_3 } ERRCIDR3;

// Physical Address Register
__register 64 { 9:9 NS, 8:8 PTW, 6:1 FST, 8:7 SH, 51:48, 47:12 PA, 0:0 F, 9:9 S, 63:56 ATTR } PAR_EL1;

// Tag Fail Status Register (EL2).
__register 64 { 0:0 TF0, 1:1 TF1 } TFSR_EL2;

// Memory Model Feature Register 1
__register 32 { 7:4 L1UniVA, 11:8 L1HvdSW, 31:28 BPred, 3:0 L1HvdVA, 27:24 L1TstCln, 15:12 L1UniSW, 23:20 L1Uni, 19:16 L1Hvd } ID_MMFR1;

// Interrupt Controller Software Generated Interrupt Group 1 Register
__register 64 { 47:44 RS, 27:24 INTID, 15:0 TargetList, 55:48 Aff3, 40:40 IRM, 39:32 Aff2, 23:16 Aff1 } ICC_SGI1R;

// External Debug Instruction Transfer Register
__register 32 { 15:0 T32First, 31:16 T32Second } EDITR;

// Monitor Debug Configuration Register (EL2)
__register 32 { 26:26 HLP, 5:5 TPMCR, 17:17 HPMD, 14:14 TPMS, 8:8 TDE, 4:0 HPMN, 13:12 E2PB, 19:19 TTRF, 6:6 TPM, 23:23 HCCD, 10:10 TDOSA, 9:9 TDA, 11:11 TDRA, 7:7 HPME } MDCR_EL2;

// Activity Monitors Device Architecture Register
__register 32 { 20:20 PRESENT, 31:21 ARCHITECT, 19:16 REVISION, 15:0 ARCHID } AMDEVARCH;

// Hypervisor Configuration Register
__register 64 { 2:2 PTW, 46:46 FWB, 43:43 NV1, 4:4 IMO, 0:0 VM, 44:44 AT, 42:42 NV, 23:23 TPC, 38:38 MIOCNCE, 47:47 FIEN, 7:7 VI, 41:41 API, 30:30 TRVM, 35:35 TLOR, 57:57 DCT, 29:29 HCD, 28:28 TDZ, 21:21 TACR, 16:16 TID1, 40:40 APK, 23:23 TPCP, 22:22 TSW, 26:26 TVM, 54:54 TTLBIS, 34:34 E2H, 11:10 BSU, 45:45 NV2, 8:8 VSE, 32:32 CD, 6:6 VF, 19:19 TSC, 53:53 EnSCXT, 20:20 TIDCP, 18:18 TID3, 13:13 TWI, 12:12 DC, 9:9 FB, 37:37 TEA, 15:15 TID0, 14:14 TWE, 5:5 AMO, 24:24 TPU, 49:49 TID4, 3:3 FMO, 27:27 TGE, 56:56 ATA, 31:31 RW, 1:1 SWIO, 36:36 TERR, 52:52 TOCU, 25:25 TTLB, 17:17 TID2, 50:50 TICAB, 55:55 TTLBOS, 33:33 ID } HCR_EL2;

// CPU Interface End Of Interrupt Register
__register 32 { 23:0 INTID } GICC_EOIR;

// External Debug Execution Control Register
__register 32 { 1:1 RCE, 2:2 SS, 0:0 OSUCE } EDECR;

// External Debug Device ID register 2
__register 32 {  } EDDEVID2;

// Virtual Machine End Of Interrupt Register
__register 32 { 24:0 INTID } GICV_EOIR;

// CTI Peripheral Identification Register 1
__register 32 { 7:4 DES_0, 3:0 PART_1 } CTIPIDR1;

// Translation Table Base Register 0 (EL1)
__register 64 { 63:48 ASID, 0:0 CnP, 47:1 BADDR } TTBR0_EL1;

// Saved Program Status Register (Monitor mode)
__register 32 { 31:31 N, 24:24 J, 27:27 Q, 19:16 GE, 9:9 E, 22:22 PAN, 23:23 SSBS, 29:29 C, 5:5 T, 6:6 F, 4:0 M, 7:7 I, 30:30 Z, 28:28 V, 8:8 A, 21:21 DIT, 20:20 IL, 15:10, 26:25 IT } SPSR_mon;

// Debug ID Register
__register 32 { 14:14 nSUHD_imp, 19:16 Version, 12:12 SE_imp, 27:24 BRPs, 23:20 CTX_CMPs, 31:28 WRPs } DBGDIDR;

// Translation Table Base Register 1
__register 64 { 6:6, 0:0 IRGN, 2:2 IMP, 0:0 CnP, 5:5 NOS, 4:3 RGN, 1:1 S, 47:1 BADDR, 31:7 TTB1, 55:48 ASID } TTBR1;

// Redistributor Control Register
__register 32 { 0:0 EnableLPIs, 1:1 CES, 26:26 DPG1S, 3:3 RWP, 31:31 UWP, 25:25 DPG1NS, 24:24 DPG0 } GICR_CTLR;

// External Debug Peripheral Identification Register 1
__register 32 { 7:4 DES_0, 3:0 PART_1 } EDPIDR1;

// Interrupt Group Modifier Registers
array [1..2] of __register 32 {  } GICR_IGRPMODRE;

// Tag Check Override
__register 64 { 25:25 TCO } TCO;

// Debug OS Lock Exception Catch Control Register
__register 32 { 31:0 EDECCR } DBGOSECCR;

// Performance Monitors Cycle Count Register
__register 64 { 63:0 CCNT } PMCCNTR;

// Interrupt Controller Virtual Interrupt Group 1 Enable register
__register 32 { 0:0 Enable } ICV_IGRPEN1;

// Debug Status and Control Register, Internal View
__register 32 { 5:2 MOE, 15:15 MDBGen, 30:30 RXfull, 17:17 SPNIDdis, 18:18 NS, 16:16 SPIDdis, 29:29 TXfull, 12:12 UDCCdis } DBGDSCRint;

// Counter-timer Virtual Timer TimerValue Register (EL2)
__register 32 { 31:0 TimerValue } CNTHV_TVAL_EL2;

// Current Program Status Register
__register 32 { 31:31 N, 19:16 GE, 23:23 SSBS, 7:7 I, 30:30 Z, 6:6 F, 22:22 PAN, 28:28 V, 27:27 Q, 29:29 C, 8:8 A, 9:9 E, 3:0 M, 21:21 DIT } CPSR;

// ITS Type Register
__register 64 { 2:2 CCT, 31:24 HCC, 36:36 CIL, 18:18 SEIS, 19:19 PTA, 12:8 ID_bits, 38:38 MPAM, 17:13 Devbits, 7:4 ITT_entry_size, 0:0 Physical, 1:1 Virtual, 35:32 CIDbits, 37:37 VMOVP } GITS_TYPER;

// Non-secure Access Control Registers
array [0..63] of __register 32 {  } GICD_NSACRE;

// MPAM ID Register (EL1)
__register 64 { 15:0 PARTID_MAX, 39:32 PMG_MAX, 17:17 HAS_HCR, 20:18 VPMR_MAX } MPAMIDR_EL1;

// Interrupt Controller Highest Priority Pending Interrupt Register 1
__register 32 { 23:0 INTID } ICC_HPPIR1_EL1;

// Auxiliary Feature Register 0
__register 32 {  } ID_AFR0;

// Virtual Deferred Interrupt Status Register
__register 64 { 5:0 STATUS, 23:0 ISS, 10:10, 3:0 FS, 31:31 A, 24:24 IDS, 12:12 ExT, 9:9 LPAE, 15:14 AET } VDISR_EL2;

// Performance Monitors Interrupt Enable Set register
__register 32 { 31:31 C } PMINTENSET;

// MPAM Error Status Register
__register 32 { 15:0 PARTID_MON, 31:31 OVRWR, 27:24 ERRCODE, 23:16 PMG } MPAMF_ESR;

// ITS Identification Register
__register 32 { 31:24 ProductID, 15:12 Revision, 19:16 Variant, 11:0 Implementer } GITS_IIDR;

// Selected Error Record Miscellaneous Register 5
__register 32 {  } ERXMISC5;

// Sampling Profiling ID Register
__register 64 { 4:4 LDS, 1:1 FT, 3:3 ArchInst, 2:2 FL, 19:16 CountSize, 15:12 MaxSize, 11:8 Interval, 0:0 FE, 5:5 ERnd } PMSIDR_EL1;

// Debug Status and Control Register, External View
__register 32 { 15:15 MDBGen, 5:2 MOE, 29:29 TXfull, 17:17 SPNIDdis, 6:6 ERR, 31:31 TFO, 27:27 RXO, 14:14 HDE, 18:18 NS, 12:12 UDCCdis, 26:26 TXU, 21:21 TDA, 19:19 SC2, 16:16 SPIDdis, 23:22 INTdis, 30:30 RXfull } DBGDSCRext;

// MPAM Features Cache Storage Usage Monitoring ID register
__register 32 { 31:31 HAS_CAPTURE, 15:0 NUM_MON } MPAMF_CSUMON_IDR;

// CTI Device ID register 1
__register 32 {  } CTIDEVID1;

// External Debug Device ID register 1
__register 32 { 3:0 PCSROffset } EDDEVID1;

// MPAM Memory System Monitor Configure Cache Storage Usage Monitor Filter Register
__register 32 { 15:0 PARTID, 23:16 PMG } MSMON_CFG_CSU_FLT;

// SGI Clear-Pending Registers
array [0..3] of __register 32 {  } GICD_CPENDSGIR;

// AArch64 Instruction Set Attribute Register 1
__register 64 { 31:28 GPI, 27:24 GPA, 23:20 LRCPC, 15:12 JSCVT, 11:8 API, 43:40 SPECRES, 3:0 DPB, 7:4 APA, 35:32 FRINTTS, 19:16 FCMA, 39:36 SB } ID_AA64ISAR1_EL1;

// Interrupt Group Register 0
__register 32 {  } GICR_IGROUPR0;

// Pointer Authentication Key A for Code (bits[127:64])
__register 64 {  } APGAKeyHi_EL1;

// Hyp System Control Register
__register 32 { 7:7 ITD, 25:25 EE, 5:5 CP15BEN, 2:2 C, 1:1 A, 0:0 M, 30:30 TE, 4:4 LSMAOE, 31:31 DSSBS, 12:12 I, 19:19 WXN, 3:3 nTLSMD, 8:8 SED } HSCTLR;

// Main ID Register
__register 32 { 23:20 Variant, 19:16 Architecture, 15:4 PartNum, 3:0 Revision, 31:24 Implementer } MIDR;

// Auxiliary Fault Status Register 0 (EL3)
__register 32 {  } AFSR0_EL3;

// AArch32 Processor Feature Register 0
__register 32 { 3:0 State0, 31:28 RAS, 19:16 CSV2, 11:8 State2, 23:20 AMU, 27:24 DIT, 7:4 State1, 15:12 State3 } ID_PFR0_EL1;

// Reseeded Random Number
__register 64 { 63:0 RNDRRS } RNDRRS;

// Hypervisor System Trap Register
__register 32 {  } HSTR_EL2;

// Performance Monitors Component Identification Register 2
__register 32 { 7:0 PRMBL_2 } PMCIDR2;

// Interrupt Controller Virtual Interrupt Priority Mask Register
__register 32 { 7:0 Priority } ICV_PMR;

// Instruction Set Attribute Register 3
__register 32 { 27:24 TrueNOP, 23:20 T32Copy, 31:28 T32EE, 19:16 TabBranch, 15:12 SynchPrim, 3:0 Saturate, 11:8 SVC, 7:4 SIMD } ID_ISAR3;

