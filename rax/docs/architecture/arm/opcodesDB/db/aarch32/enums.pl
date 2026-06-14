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

# FIXME: this should be declared as set !

# OPTIONAL STRINGIFY ENUM EXTENSION => qw/NONE AESExt BF16Ext Bit128PMULLExt CRCExt DOTPExt 
#                                        FCADDExt FJCVTZSExt FP16Ext FP16MulNoRoundingToFP32Ext Int8MatMulExt 
#					                     PANExt QRDMLAHExt RASExt SHA1Ext SHA256Ext/;

# OPTIONAL STRINGIFY ENUM EXCEPTION => qw/SOFTWAREBREAKPOINT HYPERVISORCALL ALIGNMENT DATAABORT 
#										  UNCATEGORIZED MONITORCALL SUPERVISORCALL WFXTRAP ADVSIMDFPACCESSTRAP FP
#										  CP14DTTRAP CP14RRTTRAP CP14RTTRAP CP15RRTTRAP CP15RTTRAP FPIDTRAP/;



OPTIONAL STRINGIFY ENUM EXTENSION => qw/NONE AES BF16 BIT128PMULL CRC DOTP 
                                        FCADD FJCVTZS FP16 FP16MULNOROUNDINGTOFP32 INT8MATMUL 
										PAN QRDMLAH RAS SHA1 SHA256/;

OPTIONAL STRINGIFY ENUM EXCEPTION => qw/SOFTWAREBREAKPOINT HYPERVISORCALL ALIGNMENT DATAABORT 
										  UNCATEGORIZED MONITORCALL SUPERVISORCALL WFXTRAP ADVSIMDFPACCESSTRAP FP
										  CP14DTTRAP CP14RRTTRAP CP14RTTRAP CP15RRTTRAP CP15RTTRAP FPIDTRAP/;

OPTIONAL STRINGIFY ENUM NODE_TYPE => qw/NONE  ADD ALIGN SZ COUNT INC IDX WBACK   
					                    SHIFTER EXTEND SHIFT
										XYZ COND ENDIAN SYNC_OP BARRIER IFLAGS
										BASE VBASE EBASE REG WREG VREG EREG RS REGOFF REGOFFSH REGSH 
										GLIST SLIST FSLIST FDLIST VLIST ELIST
					                    AMOUNT FPIMM  IMM IMMOFF IMMOFFSH  LABEL ADDR 
										LM MEM  
					                    CSPACE PSPACE BNKDREG SPECREG
										/;

STRINGIFY ENUM SYNC_OP => qw/NONE CSYNC/;

OPTIONAL STRINGIFY ENUM SYMBOL => qw/NONE ADD CRm CRn INC 
									 aimm timm vfpimm advimm
                                     align amount banked_reg cond const coproc count endian_specifier iflags 
									 shift_left shift_right fbits_right
				                     imm imm12 imm16 imm3 imm4 imm7 imm8 
									 label labelj nlabel plabel plabela nlabela
									 lsb mode opc1 opc2 option 
				                     registers registers_with_pc registers_without_pc rotate shift shifter 
									 single_register_list spec_reg type wback wback16 width xyz
									 GPR32 SIMD32 SIMD64 SIMD128 SYSREG
									 Dd Ddm Dm Dn  Qd Qm Qn 
                                     Ra Rd RdHi RdLo Rdm Rdn Rm Rn Rs Rt Rt2 
									 Sd Sdm Sm Sm1 Sn 
									 /;

STRINGIFY ENUM IT_OP   => qw/ NONE TTT TT  TTE T   TET TE  TEE ETT ET  ETE E   EET EE  EEE/;


1;