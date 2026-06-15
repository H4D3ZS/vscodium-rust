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


# addpcsp16: Add PC/SP (immediate).
# T16: ig0=0b1010 SP:u=0bx Rd:u=0bxxx imm8:u=0bxxxxxxxx
HINT('ICLASS', 'addpcsp16', 'T16');

DEF Rd       => GPR32('Rd', 'Rd', 'write')                              ;
DEF immx4    => IMM('imm8', 'imm8', scale => 4, range => (0, 1020))     ;
DEF plabelx4 => LABEL('plabel', 'imm8', scale => 4, range => (0, 1020)) ;

T[' ADD REG=$Rd REG=SP IMM:u16=$immx4 ', 'NONE ', 'ADD_SP_i_T1 ', 'cond=COND '];
T[' ADR REG=$Rd ADDR:u32=$plabelx4    ', 'NONE ', 'ADR_T1      ', 'cond=COND '];
T[' ADD REG=$Rd REG=PC IMM:u16=$immx4 ', 'NONE ', 'ADD_ADR_T1  ', 'cond=COND '];



# addsub16_1l_imm: Add, subtract, compare, move (one low register and immediate).
# T16: ig0=0b001 op:u=0bxx Rd:u=0bxxx imm8:u=0bxxxxxxxx
HINT('ICLASS', 'addsub16_1l_imm', 'T16');

DEF Rd  => GPR32('Rd', 'Rd', 'write')             ;
DEF Rdn => GPR32('Rdn', 'Rdn', 'read|write')      ;
DEF Rn  => GPR32('Rn', 'Rn', 'read')              ;
DEF imm => IMM('imm8', 'imm8', range => (0, 255)) ;

T[' ADD  REG:OPT=$Rdn REG=$Rdn IMM:u8=$imm ', 'NONE ', 'ADD_i_T2 ', 'init =1 fit=1 cond=MUST_COND showqlf=1 '];
T[' ADDS REG:OPT=$Rdn REG=$Rdn IMM:u8=$imm ', 'NONE ', 'ADD_i_T2 ', 'outit=1 fit=1 cond=MUST_AL showqlf=1   '];
T[' CMP  REG=$Rn IMM:u8=$imm               ', 'NONE ', 'CMP_i_T1 ', 'cond=COND                              '];
T[' MOV  REG=$Rd IMM:u8=$imm               ', 'NONE ', 'MOV_i_T1 ', 'init =1 fit=1 cond=MUST_COND           '];
T[' MOVS REG=$Rd IMM:u8=$imm               ', 'NONE ', 'MOV_i_T1 ', 'outit=1 fit=1 cond=MUST_AL             '];
T[' SUB  REG:OPT=$Rdn REG=$Rdn IMM:u8=$imm ', 'NONE ', 'SUB_i_T2 ', 'init =1 fit=1 cond=MUST_COND showqlf=1 '];
T[' SUBS REG:OPT=$Rdn REG=$Rdn IMM:u8=$imm ', 'NONE ', 'SUB_i_T2 ', 'outit=1 fit=1 cond=MUST_AL showqlf=1   '];



# addsub16_2h: Add, subtract, compare, move (two high registers).
# T16: ig0=0b010001 op:u!=0b11 D:u=0bx Rs:u=0bxxxx Rd:u=0bxxx
HINT('ICLASS', 'addsub16_2h', 'T16');

DEF Rd     => GPR32('Rd', 'D:Rd', 'write')              ;
DEF Rdmdpc => GPR32('Rdm', 'Rdm', 'read|write|dpc')     ;
DEF Rdnnsp => GPR32('Rdn', 'DN:Rdn', 'read|write|nosp') ;
DEF Rm     => GPR32('Rm', 'Rm', 'read')                 ;
DEF Rmnsp  => GPR32('Rm', 'Rm', 'read|nosp')            ;
DEF Rmnsp2 => GPR32('Rm', 'Rm', 'read|nosp|dpc')        ;
DEF Rmupc  => GPR32('Rm', 'Rm', 'read|upc')             ;
DEF Rnupc  => GPR32('Rn', 'N:Rn', 'read|upc')           ;

T[' ADD REG:OPT:iwb=$Rdnnsp REG:iwb=$Rdnnsp REG=$Rmnsp ', 'NONE ', 'ADD_r_T2    ', 'cond=COND '];
T[' ADD REG:OPT:iwb=$Rdmdpc REG=SP REG:iwb=$Rdmdpc     ', 'NONE ', 'ADD_SP_r_T1 ', 'cond=COND '];
T[' ADD REG:OPT=SP REG=SP REG=$Rmnsp2                  ', 'NONE ', 'ADD_SP_r_T2 ', 'cond=COND '];
T[' CMP REG=$Rnupc REG=$Rmupc                          ', 'NONE ', 'CMP_r_T2    ', 'cond=COND '];
T[' MOV REG=$Rd REG=$Rm                                ', 'NONE ', 'MOV_r_T1    ', 'cond=COND '];



# addsub16_2l_imm: Add, subtract (two low registers and immediate).
# T16: ig0=0b000111 S:u=0bx imm3:u=0bxxx Rn:u=0bxxx Rd:u=0bxxx
HINT('ICLASS', 'addsub16_2l_imm', 'T16');

DEF Rd  => GPR32('Rd', 'Rd', 'write')           ;
DEF Rn  => GPR32('Rn', 'Rn', 'read')            ;
DEF imm => IMM('imm3', 'imm3', range => (0, 7)) ;

T[' ADD  REG=$Rd REG=$Rn IMM:u8=$imm ', 'NONE ', 'ADD_i_T1 ', 'init =1 fit=1 cond=MUST_COND '];
T[' ADDS REG=$Rd REG=$Rn IMM:u8=$imm ', 'NONE ', 'ADD_i_T1 ', 'outit=1 fit=1 cond=MUST_AL   '];
T[' SUB  REG=$Rd REG=$Rn IMM:u8=$imm ', 'NONE ', 'SUB_i_T1 ', 'init =1 fit=1 cond=MUST_COND '];
T[' SUBS REG=$Rd REG=$Rn IMM:u8=$imm ', 'NONE ', 'SUB_i_T1 ', 'outit=1 fit=1 cond=MUST_AL   '];



# addsub16_3l: Add, subtract (three low registers).
# T16: ig0=0b000110 S:u=0bx Rm:u=0bxxx Rn:u=0bxxx Rd:u=0bxxx
HINT('ICLASS', 'addsub16_3l', 'T16');

DEF Rd => GPR32('Rd', 'Rd', 'write') ;
DEF Rm => GPR32('Rm', 'Rm', 'read')  ;
DEF Rn => GPR32('Rn', 'Rn', 'read')  ;

T[' ADD  REG=$Rd REG=$Rn REG=$Rm     ', 'NONE ', 'ADD_r_T1 ', 'init =1 fit=1 cond=MUST_COND '];
T[' ADDS REG:OPT=$Rd REG=$Rn REG=$Rm ', 'NONE ', 'ADD_r_T1 ', 'outit=1 fit=1 cond=MUST_AL   '];
T[' SUB  REG=$Rd REG=$Rn REG=$Rm     ', 'NONE ', 'SUB_r_T1 ', 'init =1 fit=1 cond=MUST_COND '];
T[' SUBS REG:OPT=$Rd REG=$Rn REG=$Rm ', 'NONE ', 'SUB_r_T1 ', 'outit=1 fit=1 cond=MUST_AL   '];



# adjsp16: Adjust SP (immediate).
# T16: ig0=0b10110000 S:u=0bx imm7:u=0bxxxxxxx
HINT('ICLASS', 'adjsp16', 'T16');

DEF immx4 => IMM('imm7', 'imm7', scale => 4, range => (0, 508)) ;

T[' ADD REG:OPT=SP REG=SP IMM:u16=$immx4 ', 'NONE ', 'ADD_SP_i_T2 ', 'cond=COND '];
T[' SUB REG:OPT=SP REG=SP IMM:u16=$immx4 ', 'NONE ', 'SUB_SP_i_T1 ', 'cond=COND '];



# b16: Unconditional branch.
# T16: ig0=0b11100 imm11:u=0bxxxxxxxxxxx
HINT('ICLASS', 'b16', 'T16');

DEF labelx2 => LABEL('label', 'imm11', scale => 2, signpos => 'imm11<10>') ;

T[' B LABEL:i32=$labelx2 ', 'NONE ', 'B_T2 ', 'cond=COND '];



# bcond16: Conditional branch.
# T16: ig0=0b1101 cond:u=0bxxxx imm8:u=0bxxxxxxxx
HINT('ICLASS', 'bcond16', 'T16');

DEF labelx2 => LABEL('label', 'imm8', scale => 2, signpos => 'imm8<7>') ;

T[' B LABEL:i32=$labelx2 ', 'NONE ', 'B_T1 ', 'cond=UNP_COND '];



# bkpt16: Software breakpoint.
# T16: ig0=0b10111110 imm8:u=0bxxxxxxxx
HINT('ICLASS', 'bkpt16', 'T16');

DEF imm => IMM('imm', 'imm8', range => (0, 255)) ;

T[' BKPT IMM:u8=$imm ', 'NONE ', 'BKPT_T1 ', 'cond=UNP_COND '];



# bx16: Branch and exchange.
# T16: ig0=0b01000111 L:u=0bx Rm:u=0bxxxx ig1=0bZ ig2=0bZ ig3=0bZ
HINT('ICLASS', 'bx16', 'T16');

DEF Rm    => GPR32('Rm', 'Rm', 'read')     ;
DEF Rmupc => GPR32('Rm', 'Rm', 'read|upc') ;

T[' BLX REG=$Rmupc ', 'NONE ', 'BLX_r_T1 ', 'cond=COND '];
T[' BX  REG=$Rm    ', 'NONE ', 'BX_T1    ', 'cond=COND '];



# cbznz16: Compare and branch zero/non-zero.
# T16: ig0=0b1011 op:u=0bx ig1=0b0 i:u=0bx ig2=0b1 imm5:u=0bxxxxx Rn:u=0bxxx
HINT('ICLASS', 'cbznz16', 'T16');

DEF Rn      => GPR32('Rn', 'Rn', 'read')                               ;
DEF labelx2 => LABEL('label', 'i:imm5', scale => 2, range => (0, 126)) ;

T[' CBNZ REG=$Rn LABEL:u32=$labelx2 ', 'NONE ', 'CBNZ_T1 ', 'cond=UNP_COND '];
T[' CBZ  REG=$Rn LABEL:u32=$labelx2 ', 'NONE ', 'CBZ_T1  ', 'cond=UNP_COND '];



# cps16: Change Processor State.
# T16: ig0=0b1011011001 op:u=0bx flags:u=0bxxxxx
HINT('ICLASS', 'cps16', 'T16');

DEF endian_specifier => ENDIAN('endian_specifier', 'E', table => 'tbl_endian') ;
DEF iflags           => IFLAGS('iflags', 'A:I:F')                              ;

T[' CPSID  IFLAGS=$iflags           ', 'NONE ', 'CPSID_T1_AS ', 'cond=UNP_COND '];
T[' CPSIE  IFLAGS=$iflags           ', 'NONE ', 'CPSIE_T1_AS ', 'cond=UNP_COND '];
T[' SETEND ENDIAN=$endian_specifier ', 'NONE ', 'SETEND_T1   ', 'cond=UNP_COND '];



# dpint16_2l: Data-processing (two low registers).
# T16: ig0=0b010000 op:u=0bxxxx Rs:u=0bxxx Rd:u=0bxxx
HINT('ICLASS', 'dpint16_2l', 'T16');

DEF Rd  => GPR32('Rd', 'Rd', 'write')        ;
DEF Rdm => GPR32('Rdm', 'Rdm', 'read|write') ;
DEF Rdn => GPR32('Rdn', 'Rdn', 'read|write') ;
DEF Rm  => GPR32('Rm', 'Rm', 'read')         ;
DEF Rn  => GPR32('Rn', 'Rn', 'read')         ;
DEF Rs  => GPR32('Rs', 'Rs')                 ;

T[' ADC  REG:OPT=$Rdn REG=$Rdn REG=$Rm                          ', 'NONE ', 'ADC_r_T1           ', 'init =1 fit=1 cond=MUST_COND '];
T[' ADCS REG:OPT=$Rdn REG=$Rdn REG=$Rm                          ', 'NONE ', 'ADC_r_T1           ', 'outit=1 fit=1 cond=MUST_AL   '];
T[' AND  REG:OPT=$Rdn REG=$Rdn REG=$Rm                          ', 'NONE ', 'AND_r_T1           ', 'init =1 fit=1 cond=MUST_COND '];
T[' ANDS REG:OPT=$Rdn REG=$Rdn REG=$Rm                          ', 'NONE ', 'AND_r_T1           ', 'outit=1 fit=1 cond=MUST_AL   '];
T[' BIC  REG:OPT=$Rdn REG=$Rdn REG=$Rm                          ', 'NONE ', 'BIC_r_T1           ', 'init =1 fit=1 cond=MUST_COND '];
T[' BICS REG:OPT=$Rdn REG=$Rdn REG=$Rm                          ', 'NONE ', 'BIC_r_T1           ', 'outit=1 fit=1 cond=MUST_AL   '];
T[' CMN  REG=$Rn REG=$Rm                                        ', 'NONE ', 'CMN_r_T1           ', 'cond=COND                    '];
T[' CMP  REG=$Rn REG=$Rm                                        ', 'NONE ', 'CMP_r_T1           ', 'cond=COND                    '];
T[' EOR  REG:OPT=$Rdn REG=$Rdn REG=$Rm                          ', 'NONE ', 'EOR_r_T1           ', 'init =1 fit=1 cond=MUST_COND '];
T[' EORS REG:OPT=$Rdn REG=$Rdn REG=$Rm                          ', 'NONE ', 'EOR_r_T1           ', 'outit=1 fit=1 cond=MUST_AL   '];
T[' MOV  REG=$Rdm REGSH={REG=$Rdm SHIFTER={SHIFT=ASR RS:r=$Rs}} ', 'NONE ', 'MOV_rr_T1_ASR      ', 'init =1 fit=1 cond=MUST_COND '];
T[' MOVS REG=$Rdm REGSH={REG=$Rdm SHIFTER={SHIFT=ASR RS:r=$Rs}} ', 'NONE ', 'MOV_rr_T1_ASR      ', 'outit=1 fit=1 cond=MUST_AL   '];
T[' MOV  REG=$Rdm REGSH={REG=$Rdm SHIFTER={SHIFT=LSL RS:r=$Rs}} ', 'NONE ', 'MOV_rr_T1_LSL      ', 'init =1 fit=1 cond=MUST_COND '];
T[' MOVS REG=$Rdm REGSH={REG=$Rdm SHIFTER={SHIFT=LSL RS:r=$Rs}} ', 'NONE ', 'MOV_rr_T1_LSL      ', 'outit=1 fit=1 cond=MUST_AL   '];
T[' MOV  REG=$Rdm REGSH={REG=$Rdm SHIFTER={SHIFT=LSR RS:r=$Rs}} ', 'NONE ', 'MOV_rr_T1_LSR      ', 'init =1 fit=1 cond=MUST_COND '];
T[' MOVS REG=$Rdm REGSH={REG=$Rdm SHIFTER={SHIFT=LSR RS:r=$Rs}} ', 'NONE ', 'MOV_rr_T1_LSR      ', 'outit=1 fit=1 cond=MUST_AL   '];
T[' MOV  REG=$Rdm REGSH={REG=$Rdm SHIFTER={SHIFT=ROR RS:r=$Rs}} ', 'NONE ', 'MOV_rr_T1_ROR      ', 'init =1 fit=1 cond=MUST_COND '];
T[' MOVS REG=$Rdm REGSH={REG=$Rdm SHIFTER={SHIFT=ROR RS:r=$Rs}} ', 'NONE ', 'MOV_rr_T1_ROR      ', 'outit=1 fit=1 cond=MUST_AL   '];
T[' MUL  REG=$Rdm REG=$Rn REG:OPT=$Rdm                          ', 'NONE ', 'MUL_T1             ', 'init =1 fit=1 cond=MUST_COND '];
T[' MULS REG=$Rdm REG=$Rn REG:OPT=$Rdm                          ', 'NONE ', 'MUL_T1             ', 'outit=1 fit=1 cond=MUST_AL   '];
T[' MVN  REG=$Rd REG=$Rm                                        ', 'NONE ', 'MVN_r_T1           ', 'init =1 fit=1 cond=MUST_COND '];
T[' MVNS REG=$Rd REG=$Rm                                        ', 'NONE ', 'MVN_r_T1           ', 'outit=1 fit=1 cond=MUST_AL   '];
T[' ORR  REG:OPT=$Rdn REG=$Rdn REG=$Rm                          ', 'NONE ', 'ORR_r_T1           ', 'init =1 fit=1 cond=MUST_COND '];
T[' ORRS REG:OPT=$Rdn REG=$Rdn REG=$Rm                          ', 'NONE ', 'ORR_r_T1           ', 'outit=1 fit=1 cond=MUST_AL   '];
T[' RSB  REG:OPT=$Rd REG=$Rn IMM:u8=0                           ', 'NONE ', 'RSB_i_T1           ', 'init =1 fit=1 cond=MUST_COND '];
T[' RSBS REG:OPT=$Rd REG=$Rn IMM:u8=0                           ', 'NONE ', 'RSB_i_T1           ', 'outit=1 fit=1 cond=MUST_AL   '];
T[' SBC  REG:OPT=$Rdn REG=$Rdn REG=$Rm                          ', 'NONE ', 'SBC_r_T1           ', 'init =1 fit=1 cond=MUST_COND '];
T[' SBCS REG:OPT=$Rdn REG=$Rdn REG=$Rm                          ', 'NONE ', 'SBC_r_T1           ', 'outit=1 fit=1 cond=MUST_AL   '];
T[' TST  REG=$Rn REG=$Rm                                        ', 'NONE ', 'TST_r_T1           ', 'cond=COND                    '];
T[' ASR  REG:OPT=$Rdm REG=$Rdm REG:r=$Rs                        ', 'NONE ', 'ASR_MOV_rr_T1_ASR  ', 'cond=COND                    '];
T[' ASRS REG:OPT=$Rdm REG=$Rdm REG:r=$Rs                        ', 'NONE ', 'ASRS_MOV_rr_T1_ASR ', 'cond=UNP_COND                '];
T[' LSL  REG:OPT=$Rdm REG=$Rdm REG:r=$Rs                        ', 'NONE ', 'LSL_MOV_rr_T1_LSL  ', 'cond=COND                    '];
T[' LSLS REG:OPT=$Rdm REG=$Rdm REG:r=$Rs                        ', 'NONE ', 'LSLS_MOV_rr_T1_LSL ', 'cond=UNP_COND                '];
T[' LSR  REG:OPT=$Rdm REG=$Rdm REG:r=$Rs                        ', 'NONE ', 'LSR_MOV_rr_T1_LSR  ', 'cond=COND                    '];
T[' LSRS REG:OPT=$Rdm REG=$Rdm REG:r=$Rs                        ', 'NONE ', 'LSRS_MOV_rr_T1_LSR ', 'cond=UNP_COND                '];
T[' ROR  REG:OPT=$Rdm REG=$Rdm REG:r=$Rs                        ', 'NONE ', 'ROR_MOV_rr_T1_ROR  ', 'cond=COND                    '];
T[' RORS REG:OPT=$Rdm REG=$Rdm REG:r=$Rs                        ', 'NONE ', 'RORS_MOV_rr_T1_ROR ', 'cond=UNP_COND                '];



# except16: Exception generation.
# T16: ig0=0b1101111 S:u=0bx imm8:u=0bxxxxxxxx
HINT('ICLASS', 'except16', 'T16');

DEF imm => IMM('imm', 'imm8', range => (0, 255)) ;

T[' SVC IMM:u8=$imm ', 'NONE ', 'SVC_T1 ', 'cond=COND '];
T[' UDF IMM:u8=$imm ', 'NONE ', 'UDF_T1 ', 'cond=COND '];



# ext16: Extend.
# T16: ig0=0b10110010 U:u=0bx B:u=0bx Rm:u=0bxxx Rd:u=0bxxx
HINT('ICLASS', 'ext16', 'T16');

DEF Rd => GPR32('Rd', 'Rd', 'write') ;
DEF Rm => GPR32('Rm', 'Rm', 'read')  ;

T[' SXTB REG:OPT=$Rd REG=$Rm ', 'NONE ', 'SXTB_T1 ', 'cond=COND '];
T[' SXTH REG:OPT=$Rd REG=$Rm ', 'NONE ', 'SXTH_T1 ', 'cond=COND '];
T[' UXTB REG:OPT=$Rd REG=$Rm ', 'NONE ', 'UXTB_T1 ', 'cond=COND '];
T[' UXTH REG:OPT=$Rd REG=$Rm ', 'NONE ', 'UXTH_T1 ', 'cond=COND '];



# hints16: Hints.
# T16: ig0=0b10111111 hint:u=0bxxxx ig1=0b0000
HINT('ICLASS', 'hints16', 'T16');


T[' NOP    ', 'NONE ', 'NOP_T1   ', 'cond=COND '];
T[' SEV    ', 'NONE ', 'SEV_T1   ', 'cond=COND '];
T[' SEVL   ', 'NONE ', 'SEVL_T1  ', 'cond=COND '];
T[' WFE    ', 'NONE ', 'WFE_T1   ', 'cond=COND '];
T[' WFI    ', 'NONE ', 'WFI_T1   ', 'cond=COND '];
T[' YIELD  ', 'NONE ', 'YIELD_T1 ', 'cond=COND '];



# hlt16: Halting breakpoint.
# T16: ig0=0b1011101010 imm6:u=0bxxxxxx
HINT('ICLASS', 'hlt16', 'T16');

DEF imm => IMM('imm', 'imm6', range => (0, 63)) ;

T[' HLT IMM:u8=$imm ', 'NONE ', 'HLT_T1 ', 'cond=UNP_COND '];



# it16: If-Then.
# T16: ig0=0b10111111 firstcond:u=0bxxxx mask:u!=0b0000
HINT('ICLASS', 'it16', 'T16');

DEF cond => COND('cond', 'firstcond') ;
DEF xyz  => XYZ('xyz', 'mask')        ;

T[' IT XYZ=$xyz COND=$cond ', 'NONE ', 'IT_T1 ', 'cond=UNP_COND '];



# ldlit16: Load literal.
# T16: ig0=0b01001 Rt:u=0bxxx imm8:u=0bxxxxxxxx
HINT('ICLASS', 'ldlit16', 'T16');

DEF Rt       => GPR32('Rt', 'Rt')                                       ;
DEF plabelx4 => LABEL('plabel', 'imm8', scale => 4, range => (0, 1020)) ;

T[' LDR REG:w=$Rt LABEL:u32=$plabelx4 ', 'NONE ', 'LDR_l_T1 ', 'cond=COND may_load=1 '];



# ldst16_imm: Load/store word/byte (immediate offset).
# T16: ig0=0b011 B:u=0bx L:u=0bx imm5:u=0bxxxxx Rn:u=0bxxx Rt:u=0bxxx
HINT('ICLASS', 'ldst16_imm', 'T16');

DEF Rn    => GPR32('Rn', 'Rn', 'read')                                       ;
DEF Rt    => GPR32('Rt', 'Rt')                                               ;
DEF imm   => IMM('imm', 'imm5', range => (0, 31), default => 0)              ;
DEF immx4 => IMM('imm', 'imm5', scale => 4, range => (0, 124), default => 0) ;

T[' LDR  REG:w=$Rt MEM:OFF:r:i32={BASE=$Rn SZ=32 ADD=1 IMMOFF:OPT:u8=$immx4} ', 'NONE ', 'LDR_i_T1  ', 'cond=COND '];
T[' LDRB REG:w=$Rt MEM:OFF:r:i8={ BASE=$Rn SZ=8  ADD=1 IMMOFF:OPT:u8=$imm}   ', 'NONE ', 'LDRB_i_T1 ', 'cond=COND '];
T[' STR  REG:r=$Rt MEM:OFF:w:i32={BASE=$Rn SZ=32 ADD=1 IMMOFF:OPT:u8=$immx4} ', 'NONE ', 'STR_i_T1  ', 'cond=COND '];
T[' STRB REG:r=$Rt MEM:OFF:w:i8={ BASE=$Rn SZ=8  ADD=1 IMMOFF:OPT:u8=$imm}   ', 'NONE ', 'STRB_i_T1 ', 'cond=COND '];



# ldst16_reg: Load/store (register offset).
# T16: ig0=0b0101 L:u=0bx B:u=0bx H:u=0bx Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx
HINT('ICLASS', 'ldst16_reg', 'T16');

DEF Rm => GPR32('Rm', 'Rm', 'read') ;
DEF Rn => GPR32('Rn', 'Rn', 'read') ;
DEF Rt => GPR32('Rt', 'Rt')         ;

T[' LDR   REG:w=$Rt MEM:OFF:r:i32={BASE=$Rn SZ=32 ADD=1 REGOFF=$Rm} ', 'NONE ', 'LDR_r_T1   ', 'cond=COND '];
T[' LDRB  REG:w=$Rt MEM:OFF:r:i8={ BASE=$Rn SZ=8  ADD=1 REGOFF=$Rm} ', 'NONE ', 'LDRB_r_T1  ', 'cond=COND '];
T[' LDRH  REG:w=$Rt MEM:OFF:r:i16={BASE=$Rn SZ=16 ADD=1 REGOFF=$Rm} ', 'NONE ', 'LDRH_r_T1  ', 'cond=COND '];
T[' LDRSB REG:w=$Rt MEM:OFF:r:s8={ BASE=$Rn SZ=8  ADD=1 REGOFF=$Rm} ', 'NONE ', 'LDRSB_r_T1 ', 'cond=COND '];
T[' LDRSH REG:w=$Rt MEM:OFF:r:s16={BASE=$Rn SZ=16 ADD=1 REGOFF=$Rm} ', 'NONE ', 'LDRSH_r_T1 ', 'cond=COND '];
T[' STR   REG:r=$Rt MEM:OFF:w:i32={BASE=$Rn SZ=32 ADD=1 REGOFF=$Rm} ', 'NONE ', 'STR_r_T1   ', 'cond=COND '];
T[' STRB  REG:r=$Rt MEM:OFF:w:i8={ BASE=$Rn SZ=8  ADD=1 REGOFF=$Rm} ', 'NONE ', 'STRB_r_T1  ', 'cond=COND '];
T[' STRH  REG:r=$Rt MEM:OFF:w:i16={BASE=$Rn SZ=16 ADD=1 REGOFF=$Rm} ', 'NONE ', 'STRH_r_T1  ', 'cond=COND '];



# ldst16_sp: Load/store (SP-relative).
# T16: ig0=0b1001 L:u=0bx Rt:u=0bxxx imm8:u=0bxxxxxxxx
HINT('ICLASS', 'ldst16_sp', 'T16');

DEF Rt    => GPR32('Rt', 'Rt')                                                ;
DEF immx4 => IMM('imm', 'imm8', scale => 4, range => (0, 1020), default => 0) ;

T[' LDR REG:w=$Rt MEM:OFF:r:i32={BASE=SP SZ=32 ADD=1 IMMOFF:OPT:u16=$immx4} ', 'NONE ', 'LDR_i_T2 ', 'cond=COND '];
T[' STR REG:r=$Rt MEM:OFF:w:i32={BASE=SP SZ=32 ADD=1 IMMOFF:OPT:u16=$immx4} ', 'NONE ', 'STR_i_T2 ', 'cond=COND '];



# ldsth16_imm: Load/store halfword (immediate offset).
# T16: ig0=0b1000 L:u=0bx imm5:u=0bxxxxx Rn:u=0bxxx Rt:u=0bxxx
HINT('ICLASS', 'ldsth16_imm', 'T16');

DEF Rn    => GPR32('Rn', 'Rn', 'read')                                      ;
DEF Rt    => GPR32('Rt', 'Rt')                                              ;
DEF immx2 => IMM('imm', 'imm5', scale => 2, range => (0, 62), default => 0) ;

T[' LDRH REG:w=$Rt MEM:OFF:r:i16={BASE=$Rn SZ=16 ADD=1 IMMOFF:OPT:u8=$immx2} ', 'NONE ', 'LDRH_i_T1 ', 'cond=COND '];
T[' STRH REG:r=$Rt MEM:OFF:w:i16={BASE=$Rn SZ=16 ADD=1 IMMOFF:OPT:u8=$immx2} ', 'NONE ', 'STRH_i_T1 ', 'cond=COND '];



# ldstm16: Load/store multiple.
# T16: ig0=0b1100 L:u=0bx Rn:u=0bxxx register_list:u=0bxxxxxxxx
HINT('ICLASS', 'ldstm16', 'T16');

DEF Rn        => GPR32('Rn', 'Rn', 'read')           ;
DEF registers => GLIST('registers', 'register_list') ;
DEF wback16   => WBACK('wback16', 'register_list')   ;

T[' LDM   WREG={REG=$Rn WBACK:OPT=$wback16} GLIST=$registers ', 'NONE ', 'LDM_T1 ', 'cond=COND may_load=1            '];
T[' LDMFD WREG={REG=$Rn WBACK:OPT=$wback16} GLIST=$registers ', 'NONE ', 'LDM_T1 ', 'cond=COND asmonly=1 may_load=1  '];
T[' LDMIA WREG={REG=$Rn WBACK:OPT=$wback16} GLIST=$registers ', 'NONE ', 'LDM_T1 ', 'cond=COND asmonly=1 may_load=1  '];
T[' STM   WREG={REG=$Rn WBACK=1} GLIST=$registers            ', 'NONE ', 'STM_T1 ', 'cond=COND may_store=1           '];
T[' STMEA WREG={REG=$Rn WBACK=1} GLIST=$registers            ', 'NONE ', 'STM_T1 ', 'cond=COND asmonly=1 may_store=1 '];
T[' STMIA WREG={REG=$Rn WBACK=1} GLIST=$registers            ', 'NONE ', 'STM_T1 ', 'cond=COND asmonly=1 may_store=1 '];



# pushpop16: Push and Pop.
# T16: ig0=0b1011 L:u=0bx ig1=0b10 P:u=0bx register_list:u=0bxxxxxxxx
HINT('ICLASS', 'pushpop16', 'T16');

DEF registers => GLIST('registers', 'register_list', default => '0') ;

T[' POP   GLIST=$registers                       ', 'NONE ', 'POP_T1  ', 'cond=COND           '];
T[' LDM   WREG={REG=SP WBACK=1} GLIST=$registers ', 'NONE ', 'POP_T1  ', 'cond=COND asmonly=1 '];
T[' PUSH  GLIST=$registers                       ', 'NONE ', 'PUSH_T1 ', 'cond=COND           '];
T[' STMDB WREG={REG=SP WBACK=1} GLIST=$registers ', 'NONE ', 'PUSH_T1 ', 'cond=COND asmonly=1 '];



# rev16: Reverse bytes.
# T16: ig0=0b10111010 op:u!=0b10 Rm:u=0bxxx Rd:u=0bxxx
HINT('ICLASS', 'rev16', 'T16');

DEF Rd => GPR32('Rd', 'Rd', 'write') ;
DEF Rm => GPR32('Rm', 'Rm', 'read')  ;

T[' REV   REG=$Rd REG=$Rm ', 'NONE ', 'REV_T1   ', 'cond=COND '];
T[' REV16 REG=$Rd REG=$Rm ', 'NONE ', 'REV16_T1 ', 'cond=COND '];
T[' REVSH REG=$Rd REG=$Rm ', 'NONE ', 'REVSH_T1 ', 'cond=COND '];



# setpan16: SETPAN.
# T16: ig0=0b10110110000 ig1=0bS imm1:u=0bx ig2=0bZ ig3=0bZ ig4=0bZ
HINT('ICLASS', 'setpan16', 'T16');

DEF imm => IMM('imm', 'imm1') ;

T[' SETPAN IMM:i32=$imm ', 'NONE ', 'SETPAN_T1 ', 'arch_variant=ARMv8v1 extensions=PAN cond=UNP_COND '];



# shift16_imm: Shift (immediate).
# T16: ig0=0b000 op:u!=0b11 imm5:u=0bxxxxx Rm:u=0bxxx Rd:u=0bxxx
HINT('ICLASS', 'shift16_imm', 'T16');

DEF Rd      => GPR32('Rd', 'Rd', 'write')                  ;
DEF Rm      => GPR32('Rm', 'Rm', 'read')                   ;
DEF amount0 => IMM('amount', 'imm5', range => (1, 31))     ;
DEF imm     => IMM('imm', 'imm5', range => (1, 32))        ;
DEF imm2    => IMM('imm', 'imm5', range => (1, 31))        ;
DEF shift0  => SHIFT('shift', 'op', table => 'tbl_shift0') ;

T[' MOV  REG=$Rd REGSH={REG=$Rm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'MOV_r_T2      ', 'init =1 fit=1 cond=MUST_COND '];
T[' MOVS REG=$Rd REGSH={REG=$Rm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'MOV_r_T2      ', 'outit=1 fit=1 cond=MUST_AL   '];
T[' ASR  REG:OPT=$Rd REG=$Rm IMM:u8=$imm                                         ', 'NONE ', 'ASR_MOV_r_T2  ', 'cond=COND                    '];
T[' ASRS REG:OPT=$Rd REG=$Rm IMM:u8=$imm                                         ', 'NONE ', 'ASRS_MOV_r_T2 ', 'cond=UNP_COND                '];
T[' LSL  REG:OPT=$Rd REG=$Rm IMM:u8=$imm2                                        ', 'NONE ', 'LSL_MOV_r_T2  ', 'cond=COND                    '];
T[' LSLS REG:OPT=$Rd REG=$Rm IMM:u8=$imm2                                        ', 'NONE ', 'LSLS_MOV_r_T2 ', 'cond=UNP_COND                '];
T[' LSR  REG:OPT=$Rd REG=$Rm IMM:u8=$imm                                         ', 'NONE ', 'LSR_MOV_r_T2  ', 'cond=COND                    '];
T[' LSRS REG:OPT=$Rd REG=$Rm IMM:u8=$imm                                         ', 'NONE ', 'LSRS_MOV_r_T2 ', 'cond=UNP_COND                '];



# addsub_par: Parallel add-subtract.
# T32: ig0=0b111110101 op1:u=0bxxx Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U:u=0bx H:u=0bx S:u=0bx Rm:u=0bxxxx
HINT('ICLASS', 'addsub_par', 'T32');

DEF Rdupc => GPR32('Rd', 'Rd', 'write|upc') ;
DEF Rmupc => GPR32('Rm', 'Rm', 'read|upc')  ;
DEF Rnupc => GPR32('Rn', 'Rn', 'read|upc')  ;

T[' QADD16  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'QADD16_T1  ', 'cond=COND '];
T[' QADD8   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'QADD8_T1   ', 'cond=COND '];
T[' QASX    REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'QASX_T1    ', 'cond=COND '];
T[' QSAX    REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'QSAX_T1    ', 'cond=COND '];
T[' QSUB16  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'QSUB16_T1  ', 'cond=COND '];
T[' QSUB8   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'QSUB8_T1   ', 'cond=COND '];
T[' SADD16  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SADD16_T1  ', 'cond=COND '];
T[' SADD8   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SADD8_T1   ', 'cond=COND '];
T[' SASX    REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SASX_T1    ', 'cond=COND '];
T[' SHADD16 REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SHADD16_T1 ', 'cond=COND '];
T[' SHADD8  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SHADD8_T1  ', 'cond=COND '];
T[' SHASX   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SHASX_T1   ', 'cond=COND '];
T[' SHSAX   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SHSAX_T1   ', 'cond=COND '];
T[' SHSUB16 REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SHSUB16_T1 ', 'cond=COND '];
T[' SHSUB8  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SHSUB8_T1  ', 'cond=COND '];
T[' SSAX    REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SSAX_T1    ', 'cond=COND '];
T[' SSUB16  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SSUB16_T1  ', 'cond=COND '];
T[' SSUB8   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SSUB8_T1   ', 'cond=COND '];
T[' UADD16  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UADD16_T1  ', 'cond=COND '];
T[' UADD8   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UADD8_T1   ', 'cond=COND '];
T[' UASX    REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UASX_T1    ', 'cond=COND '];
T[' UHADD16 REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UHADD16_T1 ', 'cond=COND '];
T[' UHADD8  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UHADD8_T1  ', 'cond=COND '];
T[' UHASX   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UHASX_T1   ', 'cond=COND '];
T[' UHSAX   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UHSAX_T1   ', 'cond=COND '];
T[' UHSUB16 REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UHSUB16_T1 ', 'cond=COND '];
T[' UHSUB8  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UHSUB8_T1  ', 'cond=COND '];
T[' UQADD16 REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UQADD16_T1 ', 'cond=COND '];
T[' UQADD8  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UQADD8_T1  ', 'cond=COND '];
T[' UQASX   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UQASX_T1   ', 'cond=COND '];
T[' UQSAX   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UQSAX_T1   ', 'cond=COND '];
T[' UQSUB16 REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UQSUB16_T1 ', 'cond=COND '];
T[' UQSUB8  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UQSUB8_T1  ', 'cond=COND '];
T[' USAX    REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'USAX_T1    ', 'cond=COND '];
T[' USUB16  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'USUB16_T1  ', 'cond=COND '];
T[' USUB8   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'USUB8_T1   ', 'cond=COND '];



# b: Unconditional branch.
# T32: ig0=0b11110 S:u=0bx imm10:u=0bxxxxxxxxxx ig1=0b10 J1:u=0bx ig2=0b1 J2:u=0bx imm11:u=0bxxxxxxxxxxx
HINT('ICLASS', 'b', 'T32');

DEF labeljx2 => LABEL('labelj', 'S:J1:J2:imm10:imm11', scale => 2, signpos => 'S') ;

T[' B LABEL:i32=$labeljx2 ', 'NONE ', 'B_T4 ', 'cond=COND showqlf=1 '];



# b_imm: Branch (immediate).
# A32: cond:u=0bxxxx ig0=0b101 H:u=0bx imm24:u=0bxxxxxxxxxxxxxxxxxxxxxxxx
HINT('ICLASS', 'b_imm', 'A32');

DEF labelx2 => LABEL('label', 'imm24:H', scale => 2, signpos => 'imm24<23>') ;
DEF labelx4 => LABEL('label', 'imm24', scale => 4, signpos => 'imm24<23>')   ;

T[' B   LABEL:i32=$labelx4 ', 'NONE ', 'B_A1    ', 'cond=NO_NV   '];
T[' BL  LABEL:i32=$labelx4 ', 'NONE ', 'BL_i_A1 ', 'cond=NO_NV   '];
T[' BLX LABEL:i32=$labelx2 ', 'NONE ', 'BL_i_A2 ', 'cond=MUST_NV '];



# barriers: Barriers.
# A32: ig0=0b111101010111 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ opcode:u=0bxxxx option:u=0bxxxx
HINT('ICLASS', 'barriers', 'A32');

DEF option  => BARRIER('option', 'option', table => 'tbl_barrier')  ;
DEF option2 => BARRIER('option', 'option', table => 'tbl_barrier1') ;

T[' CLREX                      ', 'NONE ', 'CLREX_A1 ', 'cond=UNCOND             '];
T[' DMB   BARRIER:OPT=$option  ', 'NONE ', 'DMB_A1   ', 'cond=UNCOND             '];
T[' DSB   BARRIER:OPT=$option  ', 'NONE ', 'DSB_A1   ', 'cond=UNCOND             '];
T[' ISB   BARRIER:OPT=$option2 ', 'NONE ', 'ISB_A1   ', 'cond=UNCOND             '];
T[' PSSBB                      ', 'NONE ', 'PSSBB_A1 ', 'cond=UNCOND may_store=1 '];
T[' SB                         ', 'NONE ', 'SB_A1    ', 'cond=UNCOND             '];
T[' SSBB                       ', 'NONE ', 'SSBB_A1  ', 'cond=UNCOND may_store=1 '];



# bcond: Conditional branch.
# T32: ig0=0b11110 S:u=0bx cond:u=0bxxxx imm6:u=0bxxxxxx ig1=0b10 J1:u=0bx ig2=0b0 J2:u=0bx imm11:u=0bxxxxxxxxxxx
HINT('ICLASS', 'bcond', 'T32');

DEF labelx2 => LABEL('label', 'S:J2:J1:imm6:imm11', scale => 2, signpos => 'S') ;

T[' B LABEL:i32=$labelx2 ', 'NONE ', 'B_T3 ', 'cond=UNP_COND showqlf=1 '];



# bfi: Bitfield Insert.
# A32: cond:u!=0b1111 ig0=0b0111110 msb:u=0bxxxxx Rd:u=0bxxxx lsb:u=0bxxxxx ig1=0b001 Rn:u=0bxxxx
HINT('ICLASS', 'bfi', 'A32');

DEF Rdupc => GPR32('Rd', 'Rd', 'write|upc')        ;
DEF Rnnpc => GPR32('Rn', 'Rn', 'read|nopc')        ;
DEF lsb   => IMM('lsb', 'lsb', range => (0, 31))   ;
DEF width => IMM('width', 'msb', range => (1, 32)) ;

T[' BFC REG=$Rdupc IMM:u8=$lsb IMM:u8=$width            ', 'NONE ', 'BFC_A1 ', 'cond=NO_NV '];
T[' BFI REG=$Rdupc REG=$Rnnpc IMM:u8=$lsb IMM:u8=$width ', 'NONE ', 'BFI_A1 ', 'cond=NO_NV '];



# bfx: Bitfield Extract.
# A32: cond:u!=0b1111 ig0=0b01111 U:u=0bx ig1=0b1 widthm1:u=0bxxxxx Rd:u=0bxxxx lsb:u=0bxxxxx ig2=0b101 Rn:u=0bxxxx
HINT('ICLASS', 'bfx', 'A32');

DEF Rdupc => GPR32('Rd', 'Rd', 'write|upc')            ;
DEF Rnupc => GPR32('Rn', 'Rn', 'read|upc')             ;
DEF lsb   => IMM('lsb', 'lsb', range => (0, 31))       ;
DEF width => IMM('width', 'widthm1', range => (1, 32)) ;

T[' SBFX REG=$Rdupc REG=$Rnupc IMM:u8=$lsb IMM:u8=$width ', 'NONE ', 'SBFX_A1 ', 'cond=NO_NV '];
T[' UBFX REG=$Rdupc REG=$Rnupc IMM:u8=$lsb IMM:u8=$width ', 'NONE ', 'UBFX_A1 ', 'cond=NO_NV '];



# bl: Unconditional branch and link.
# T32: ig0=0b11110 S:u=0bx imm10:u=0bxxxxxxxxxx ig1=0b11 J1:u=0bx ig2=0b1 J2:u=0bx imm11:u=0bxxxxxxxxxxx
HINT('ICLASS', 'bl', 'T32');

DEF labeljx2 => LABEL('labelj', 'S:J1:J2:imm10:imm11', scale => 2, signpos => 'S') ;

T[' BL LABEL:i32=$labeljx2 ', 'NONE ', 'BL_i_T1 ', 'cond=COND '];



# blx: Unconditional branch and link exchange.
# T32: ig0=0b11110 S:u=0bx imm10:u=0bxxxxxxxxxx ig1=0b11 J1:u=0bx ig2=0b0 J2:u=0bx imm11:u=0bxxxxxxxxxxx
HINT('ICLASS', 'blx', 'T32');

DEF labeljx4 => LABEL('labelj', 'S:J1:J2:imm10H:imm10L', scale => 4, signpos => 'S') ;

T[' BLX LABEL:i32=$labeljx4 ', 'NONE ', 'BL_i_T2 ', 'cond=COND '];



# blx_reg: Branch with Link and Exchange (register).
# A32: cond:u!=0b1111 ig0=0b00010010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bS ig10=0bS ig11=0bS ig12=0bS ig13=0b0011 Rm:u=0bxxxx
HINT('ICLASS', 'blx_reg', 'A32');

DEF Rmupc => GPR32('Rm', 'Rm', 'read|upc') ;

T[' BLX REG=$Rmupc ', 'NONE ', 'BLX_r_A1 ', 'cond=NO_NV '];



# bx_jaz: Branch and Exchange Jazelle.
# T32: ig0=0b111100111100 Rm:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bZ ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ ig13=0bZ ig14=0bZ ig15=0bZ
HINT('ICLASS', 'bx_jaz', 'T32');

DEF Rmupc => GPR32('Rm', 'Rm', 'read|upc') ;

T[' BXJ REG=$Rmupc ', 'NONE ', 'BXJ_T1 ', 'cond=COND '];



# bx_reg: Branch and Exchange (register).
# A32: cond:u!=0b1111 ig0=0b00010010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bS ig10=0bS ig11=0bS ig12=0bS ig13=0b0001 Rm:u=0bxxxx
HINT('ICLASS', 'bx_reg', 'A32');

DEF Rm => GPR32('Rm', 'Rm', 'read') ;

T[' BX REG=$Rm ', 'NONE ', 'BX_A1 ', 'cond=NO_NV '];



# bxj_reg: Branch and Exchange to Jazelle (register).
# A32: cond:u!=0b1111 ig0=0b00010010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bS ig10=0bS ig11=0bS ig12=0bS ig13=0b0010 Rm:u=0bxxxx
HINT('ICLASS', 'bxj_reg', 'A32');

DEF Rmupc => GPR32('Rm', 'Rm', 'read|upc') ;

T[' BXJ REG=$Rmupc ', 'NONE ', 'BXJ_A1 ', 'cond=NO_NV '];



# clz: Count Leading Zeros.
# A32: cond:u!=0b1111 ig0=0b00010110 ig1=0bS ig2=0bS ig3=0bS ig4=0bS Rd:u=0bxxxx ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0b0001 Rm:u=0bxxxx
HINT('ICLASS', 'clz', 'A32');

DEF Rdupc => GPR32('Rd', 'Rd', 'write|upc') ;
DEF Rmupc => GPR32('Rm', 'Rm', 'read|upc')  ;

T[' CLZ REG=$Rdupc REG=$Rmupc ', 'NONE ', 'CLZ_A1 ', 'cond=NO_NV '];



# cp_ldst: System register Load/Store.
# T32: ig0=0b111 o0:u=0bx ig1=0b110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L:u=0bx Rn:u=0bxxxx CRd:u=0bxxxx ig2=0b111 cp15:u=0bx imm8:u=0bxxxxxxxx
HINT('ICLASS', 'cp_ldst', 'T32');

DEF ADD    => ADD('ADD', 'U')                              ;
DEF Rn     => GPR32('Rn', 'Rn', 'read')                    ;
DEF Rnnpc  => GPR32('Rn', 'Rn', 'read|nopc')               ;
DEF immx4  => IMM('imm', 'imm8', scale => 4, default => 0) ;
DEF option => IMM('option', 'imm8', range => (0, 255))     ;

T[' LDC PSPACE=P14 CSPACE=C5 MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=$ADD IMMOFF:OPT:i32=$immx4} ', 'NONE ', 'LDC_i_T1_off   ', 'cond=COND '];
T[' LDC PSPACE=P14 CSPACE=C5 MEM:PST:r:i32={BASE=$Rnnpc SZ=32 ADD=$ADD IMMOFF:i32=$immx4}     ', 'NONE ', 'LDC_i_T1_post  ', 'cond=COND '];
T[' LDC PSPACE=P14 CSPACE=C5 MEM:PRE:r:i32={BASE=$Rnnpc SZ=32 ADD=$ADD IMMOFF:i32=$immx4}     ', 'NONE ', 'LDC_i_T1_pre   ', 'cond=COND '];
T[' LDC PSPACE=P14 CSPACE=C5 MEM:PST:r:i32={BASE=$Rnnpc SZ=32 IMMOFF:u8=$option}              ', 'NONE ', 'LDC_i_T1_unind ', 'cond=COND '];
T[' LDC PSPACE=P14 CSPACE=C5 LM:OFF:r:i32={BASE=PC SZ=32 ADD=$ADD IMMOFF:i32=$immx4}          ', 'NONE ', 'LDC_l_T1       ', 'cond=COND '];
T[' STC PSPACE=P14 CSPACE=C5 MEM:OFF:w:i32={BASE=$Rn SZ=32 ADD=$ADD IMMOFF:OPT:i32=$immx4}    ', 'NONE ', 'STC_T1_off     ', 'cond=COND '];
T[' STC PSPACE=P14 CSPACE=C5 MEM:PST:w:i32={BASE=$Rn SZ=32 ADD=$ADD IMMOFF:i32=$immx4}        ', 'NONE ', 'STC_T1_post    ', 'cond=COND '];
T[' STC PSPACE=P14 CSPACE=C5 MEM:PRE:w:i32={BASE=$Rn SZ=32 ADD=$ADD IMMOFF:i32=$immx4}        ', 'NONE ', 'STC_T1_pre     ', 'cond=COND '];
T[' STC PSPACE=P14 CSPACE=C5 MEM:PST:w:i32={BASE=$Rn SZ=32 IMMOFF:u8=$option}                 ', 'NONE ', 'STC_T1_unind   ', 'cond=COND '];



# cp_mov32: System register 32-bit move.
# T32: ig0=0b111 o0:u=0bx ig1=0b1110 opc1:u=0bxxx L:u=0bx CRn:u=0bxxxx Rt:u=0bxxxx ig2=0b111 cp15:u=0bx opc2:u=0bxxx ig3=0b1 CRm:u=0bxxxx
HINT('ICLASS', 'cp_mov32', 'T32');

DEF CRm    => CSPACE('CRm', 'CRm')                                 ;
DEF CRn    => CSPACE('CRn', 'CRn')                                 ;
DEF Rt     => GPR32('Rt', 'Rt')                                    ;
DEF Rtupc  => GPR32('Rt', 'Rt', 'upc')                             ;
DEF coproc => PSPACE('coproc', 'coproc<0>', table => 'tbl_pspace') ;
DEF opc    => IMM('opc1', 'opc1', range => (0, 7))                 ;
DEF opc2   => IMM('opc2', 'opc2', range => (0, 7))                 ;

T[' MCR PSPACE=$coproc IMM:u8=$opc REG:r=$Rtupc CSPACE=$CRn CSPACE=$CRm IMM:OPT:u8=$opc2 ', 'NONE ', 'MCR_T1 ', 'cond=COND '];
T[' MRC PSPACE=$coproc IMM:u8=$opc REG:w=$Rt CSPACE=$CRn CSPACE=$CRm IMM:OPT:u8=$opc2    ', 'NONE ', 'MRC_T1 ', 'cond=COND '];



# cp_mov64: System register 64-bit move.
# T32: ig0=0b111 o0:u=0bx ig1=0b11000 D:u=0bx ig2=0b0 L:u=0bx Rt2:u=0bxxxx Rt:u=0bxxxx ig3=0b111 cp15:u=0bx opc1:u=0bxxxx CRm:u=0bxxxx
HINT('ICLASS', 'cp_mov64', 'T32');

DEF CRm    => CSPACE('CRm', 'CRm')                                 ;
DEF Rtupc  => GPR32('Rt', 'Rt', 'upc')                             ;
DEF Rtupc2 => GPR32('Rt2', 'Rt2', 'upc')                           ;
DEF coproc => PSPACE('coproc', 'coproc<0>', table => 'tbl_pspace') ;
DEF opc    => IMM('opc1', 'opc1', range => (0, 15))                ;

T[' MCRR PSPACE=$coproc IMM:u8=$opc REG:r=$Rtupc REG:r=$Rtupc2 CSPACE=$CRm ', 'NONE ', 'MCRR_T1 ', 'cond=COND '];
T[' MRRC PSPACE=$coproc IMM:u8=$opc REG:w=$Rtupc REG:w=$Rtupc2 CSPACE=$CRm ', 'NONE ', 'MRRC_T1 ', 'cond=COND '];



# cps: Change Process State.
# A32: ig0=0b111100010000 imod:u=0bxx M:u=0bx op:u=0bx ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ E:u=0bx A:u=0bx I:u=0bx F:u=0bx ig7=0b0 mode:u=0bxxxxx
HINT('ICLASS', 'cps', 'A32');

DEF endian_specifier => ENDIAN('endian_specifier', 'E', table => 'tbl_endian') ;
DEF iflags           => IFLAGS('iflags', 'A:I:F')                              ;
DEF mode             => IMM('mode', 'mode', range => (0, 31))                  ;

T[' CPS    IMM:u8=$mode                ', 'NONE ', 'CPS_A1_AS    ', 'cond=UNCOND '];
T[' CPSID  IFLAGS=$iflags              ', 'NONE ', 'CPSID_A1_AS  ', 'cond=UNCOND '];
T[' CPSID  IFLAGS=$iflags IMM:u8=$mode ', 'NONE ', 'CPSID_A1_ASM ', 'cond=UNCOND '];
T[' CPSIE  IFLAGS=$iflags              ', 'NONE ', 'CPSIE_A1_AS  ', 'cond=UNCOND '];
T[' CPSIE  IFLAGS=$iflags IMM:u8=$mode ', 'NONE ', 'CPSIE_A1_ASM ', 'cond=UNCOND '];
T[' SETEND ENDIAN=$endian_specifier    ', 'NONE ', 'SETEND_A1    ', 'cond=UNCOND '];



# cps: Change processor state.
# T32: ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ imod:u=0bxx M:u=0bx A:u=0bx I:u=0bx F:u=0bx mode:u=0bxxxxx
HINT('ICLASS', 'cps', 'T32');

DEF iflags => IFLAGS('iflags', 'A:I:F')             ;
DEF mode   => IMM('mode', 'mode', range => (0, 31)) ;

T[' CPS   IMM:u8=$mode                ', 'NONE ', 'CPS_T2_AS    ', 'cond=UNP_COND           '];
T[' CPSID IFLAGS=$iflags              ', 'NONE ', 'CPSID_T2_AS  ', 'cond=UNP_COND showqlf=1 '];
T[' CPSID IFLAGS=$iflags IMM:u8=$mode ', 'NONE ', 'CPSID_T2_ASM ', 'cond=UNP_COND           '];
T[' CPSIE IFLAGS=$iflags              ', 'NONE ', 'CPSIE_T2_AS  ', 'cond=UNP_COND showqlf=1 '];
T[' CPSIE IFLAGS=$iflags IMM:u8=$mode ', 'NONE ', 'CPSIE_T2_ASM ', 'cond=UNP_COND           '];



# crc32: Cyclic Redundancy Check.
# A32: cond:u!=0b1111 ig0=0b00010 sz:u=0bxx ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ C:u=0bx ig4=0bZ ig5=0b0100 Rm:u=0bxxxx
HINT('ICLASS', 'crc32', 'A32');

DEF Rdupc => GPR32('Rd', 'Rd', 'write|upc') ;
DEF Rmupc => GPR32('Rm', 'Rm', 'read|upc')  ;
DEF Rnupc => GPR32('Rn', 'Rn', 'read|upc')  ;

T[' CRC32B  REG=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'CRC32B_A1  ', 'arch_variant=ARMv8v1 extensions=CRC cond=NO_NV_UNP_COND '];
T[' CRC32H  REG=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'CRC32H_A1  ', 'arch_variant=ARMv8v1 extensions=CRC cond=NO_NV_UNP_COND '];
T[' CRC32W  REG=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'CRC32W_A1  ', 'arch_variant=ARMv8v1 extensions=CRC cond=NO_NV_UNP_COND '];
T[' CRC32CB REG=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'CRC32CB_A1 ', 'arch_variant=ARMv8v1 extensions=CRC cond=NO_NV_UNP_COND '];
T[' CRC32CH REG=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'CRC32CH_A1 ', 'arch_variant=ARMv8v1 extensions=CRC cond=NO_NV_UNP_COND '];
T[' CRC32CW REG=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'CRC32CW_A1 ', 'arch_variant=ARMv8v1 extensions=CRC cond=NO_NV_UNP_COND '];



# dcps: DCPS.
# T32: ig0=0b111101111000 imm4:u=0bxxxx ig1=0b1000 imm10:u=0bxxxxxxxxxx opt:u=0bxx
HINT('ICLASS', 'dcps', 'T32');


T[' DCPS1  ', 'NONE ', 'DCPS1_T1 ', 'cond=UNP_COND '];
T[' DCPS2  ', 'NONE ', 'DCPS2_T1 ', 'cond=UNP_COND '];
T[' DCPS3  ', 'NONE ', 'DCPS3_T1 ', 'cond=UNP_COND '];



# dpint_2r: Data-processing (two source registers).
# T32: ig0=0b111110101 op1:u=0bxxx Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2:u=0bxx Rm:u=0bxxxx
HINT('ICLASS', 'dpint_2r', 'T32');

DEF Rdupc => GPR32('Rd', 'Rd', 'write|upc') ;
DEF Rmupc => GPR32('Rm', 'Rm', 'read|upc')  ;
DEF Rnupc => GPR32('Rn', 'Rn', 'read|upc')  ;

T[' CLZ     REG=$Rdupc REG=$Rmupc                ', 'NONE ', 'CLZ_T1     ', 'cond=COND                                         '];
T[' CRC32B  REG=$Rdupc REG=$Rnupc REG=$Rmupc     ', 'NONE ', 'CRC32B_T1  ', 'arch_variant=ARMv8v1 extensions=CRC cond=UNP_COND '];
T[' CRC32H  REG=$Rdupc REG=$Rnupc REG=$Rmupc     ', 'NONE ', 'CRC32H_T1  ', 'arch_variant=ARMv8v1 extensions=CRC cond=UNP_COND '];
T[' CRC32W  REG=$Rdupc REG=$Rnupc REG=$Rmupc     ', 'NONE ', 'CRC32W_T1  ', 'arch_variant=ARMv8v1 extensions=CRC cond=UNP_COND '];
T[' CRC32CB REG=$Rdupc REG=$Rnupc REG=$Rmupc     ', 'NONE ', 'CRC32CB_T1 ', 'arch_variant=ARMv8v1 extensions=CRC cond=UNP_COND '];
T[' CRC32CH REG=$Rdupc REG=$Rnupc REG=$Rmupc     ', 'NONE ', 'CRC32CH_T1 ', 'arch_variant=ARMv8v1 extensions=CRC cond=UNP_COND '];
T[' CRC32CW REG=$Rdupc REG=$Rnupc REG=$Rmupc     ', 'NONE ', 'CRC32CW_T1 ', 'arch_variant=ARMv8v1 extensions=CRC cond=UNP_COND '];
T[' QADD    REG:OPT=$Rdupc REG=$Rmupc REG=$Rnupc ', 'NONE ', 'QADD_T1    ', 'cond=COND                                         '];
T[' QDADD   REG:OPT=$Rdupc REG=$Rmupc REG=$Rnupc ', 'NONE ', 'QDADD_T1   ', 'cond=COND                                         '];
T[' QDSUB   REG:OPT=$Rdupc REG=$Rmupc REG=$Rnupc ', 'NONE ', 'QDSUB_T1   ', 'cond=COND                                         '];
T[' QSUB    REG:OPT=$Rdupc REG=$Rmupc REG=$Rnupc ', 'NONE ', 'QSUB_T1    ', 'cond=COND                                         '];
T[' RBIT    REG=$Rdupc REG=$Rmupc                ', 'NONE ', 'RBIT_T1    ', 'cond=COND                                         '];
T[' REV     REG=$Rdupc REG=$Rmupc                ', 'NONE ', 'REV_T2     ', 'cond=COND showqlf=1                               '];
T[' REV16   REG=$Rdupc REG=$Rmupc                ', 'NONE ', 'REV16_T2   ', 'cond=COND showqlf=1                               '];
T[' REVSH   REG=$Rdupc REG=$Rmupc                ', 'NONE ', 'REVSH_T2   ', 'cond=COND showqlf=1                               '];
T[' SEL     REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SEL_T1     ', 'cond=COND                                         '];



# dpint_immm: Data-processing (modified immediate).
# T32: ig0=0b11110 i:u=0bx ig1=0b0 op1:u=0bxxxx S:u=0bx Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx
HINT('ICLASS', 'dpint_immm', 'T32');

DEF Rd    => GPR32('Rd', 'Rd', 'write')         ;
DEF Rdnpc => GPR32('Rd', 'Rd', 'write|nopc')    ;
DEF Rdupc => GPR32('Rd', 'Rd', 'write|upc')     ;
DEF Rnnpc => GPR32('Rn', 'Rn', 'read|nopc')     ;
DEF Rnnsp => GPR32('Rn', 'Rn', 'read|nosp|upc') ;
DEF Rnupc => GPR32('Rn', 'Rn', 'read|upc')      ;
DEF timm  => IMM('timm', 'i:imm3:imm8')         ;

T[' ADC  REG:OPT=$Rdupc REG=$Rnupc IMM:i32=$timm ', 'NONE ', 'ADC_i_T1     ', 'cond=COND           '];
T[' ADCS REG:OPT=$Rdupc REG=$Rnupc IMM:i32=$timm ', 'NONE ', 'ADCS_i_T1    ', 'cond=COND           '];
T[' ADD  REG:OPT=$Rd REG=$Rnnsp IMM:i32=$timm    ', 'NONE ', 'ADD_i_T3     ', 'cond=COND showqlf=1 '];
T[' ADDS REG:OPT=$Rdnpc REG=$Rnnsp IMM:i32=$timm ', 'NONE ', 'ADDS_i_T3    ', 'cond=COND showqlf=1 '];
T[' ADD  REG:OPT=$Rd REG=SP IMM:i32=$timm        ', 'NONE ', 'ADD_SP_i_T3  ', 'cond=COND showqlf=1 '];
T[' ADDS REG:OPT=$Rdnpc REG=SP IMM:i32=$timm     ', 'NONE ', 'ADDS_SP_i_T3 ', 'cond=COND           '];
T[' AND  REG:OPT=$Rd REG=$Rnupc IMM:i32=$timm    ', 'NONE ', 'AND_i_T1     ', 'cond=COND           '];
T[' ANDS REG:OPT=$Rdnpc REG=$Rnupc IMM:i32=$timm ', 'NONE ', 'ANDS_i_T1    ', 'cond=COND           '];
T[' BIC  REG:OPT=$Rdupc REG=$Rnupc IMM:i32=$timm ', 'NONE ', 'BIC_i_T1     ', 'cond=COND           '];
T[' BICS REG:OPT=$Rdupc REG=$Rnupc IMM:i32=$timm ', 'NONE ', 'BICS_i_T1    ', 'cond=COND           '];
T[' CMN  REG=$Rnupc IMM:i32=$timm                ', 'NONE ', 'CMN_i_T1     ', 'cond=COND           '];
T[' CMP  REG=$Rnupc IMM:i32=$timm                ', 'NONE ', 'CMP_i_T2     ', 'cond=COND showqlf=1 '];
T[' EOR  REG:OPT=$Rd REG=$Rnupc IMM:i32=$timm    ', 'NONE ', 'EOR_i_T1     ', 'cond=COND           '];
T[' EORS REG:OPT=$Rdnpc REG=$Rnupc IMM:i32=$timm ', 'NONE ', 'EORS_i_T1    ', 'cond=COND           '];
T[' MOV  REG=$Rdupc IMM:i32=$timm                ', 'NONE ', 'MOV_i_T2     ', 'cond=COND showqlf=1 '];
T[' MOVS REG=$Rdupc IMM:i32=$timm                ', 'NONE ', 'MOVS_i_T2    ', 'cond=COND showqlf=1 '];
T[' MVN  REG=$Rdupc IMM:i32=$timm                ', 'NONE ', 'MVN_i_T1     ', 'cond=COND           '];
T[' MVNS REG=$Rdupc IMM:i32=$timm                ', 'NONE ', 'MVNS_i_T1    ', 'cond=COND           '];
T[' ORNS REG:OPT=$Rdupc REG=$Rnnpc IMM:i32=$timm ', 'NONE ', 'ORNS_i_T1    ', 'cond=COND           '];
T[' ORN  REG:OPT=$Rdupc REG=$Rnnpc IMM:i32=$timm ', 'NONE ', 'ORN_i_T1     ', 'cond=COND           '];
T[' ORR  REG:OPT=$Rdupc REG=$Rnnpc IMM:i32=$timm ', 'NONE ', 'ORR_i_T1     ', 'cond=COND           '];
T[' ORRS REG:OPT=$Rdupc REG=$Rnnpc IMM:i32=$timm ', 'NONE ', 'ORRS_i_T1    ', 'cond=COND           '];
T[' RSB  REG:OPT=$Rdupc REG=$Rnupc IMM:i32=$timm ', 'NONE ', 'RSB_i_T2     ', 'cond=COND showqlf=1 '];
T[' RSBS REG:OPT=$Rdupc REG=$Rnupc IMM:i32=$timm ', 'NONE ', 'RSBS_i_T2    ', 'cond=COND showqlf=1 '];
T[' SBC  REG:OPT=$Rdupc REG=$Rnupc IMM:i32=$timm ', 'NONE ', 'SBC_i_T1     ', 'cond=COND           '];
T[' SBCS REG:OPT=$Rdupc REG=$Rnupc IMM:i32=$timm ', 'NONE ', 'SBCS_i_T1    ', 'cond=COND           '];
T[' SUB  REG:OPT=$Rd REG=$Rnnsp IMM:i32=$timm    ', 'NONE ', 'SUB_i_T3     ', 'cond=COND showqlf=1 '];
T[' SUBS REG:OPT=$Rdnpc REG=$Rnnsp IMM:i32=$timm ', 'NONE ', 'SUBS_i_T3    ', 'cond=COND showqlf=1 '];
T[' SUB  REG:OPT=$Rd REG=SP IMM:i32=$timm        ', 'NONE ', 'SUB_SP_i_T2  ', 'cond=COND showqlf=1 '];
T[' SUBS REG:OPT=$Rdnpc REG=SP IMM:i32=$timm     ', 'NONE ', 'SUBS_SP_i_T2 ', 'cond=COND           '];
T[' TEQ  REG=$Rnupc IMM:i32=$timm                ', 'NONE ', 'TEQ_i_T1     ', 'cond=COND           '];
T[' TST  REG=$Rnupc IMM:i32=$timm                ', 'NONE ', 'TST_i_T1     ', 'cond=COND           '];



# dpint_imms: Data-processing (simple immediate).
# T32: ig0=0b11110 i:u=0bx ig1=0b10 o1:u=0bx ig2=0b0 o2:u=0bx ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx
HINT('ICLASS', 'dpint_imms', 'T32');

DEF Rdupc   => GPR32('Rd', 'Rd', 'write|upc')                  ;
DEF Rnnsppc => GPR32('Rn', 'Rn', 'read|nosp|nopc')             ;
DEF imm     => IMM('imm12', 'i:imm3:imm8', range => (0, 4095)) ;
DEF nlabel  => LABEL('nlabel', 'i:imm3:imm8')                  ;
DEF plabel  => LABEL('plabel', 'i:imm3:imm8')                  ;

T[' ADD  REG:OPT=$Rdupc REG=$Rnnsppc IMM:u16=$imm ', 'NONE ', 'ADD_i_T4    ', 'cond=COND asmonly=1 showqlf=1 '];
T[' ADDW REG:OPT=$Rdupc REG=$Rnnsppc IMM:u16=$imm ', 'NONE ', 'ADD_i_T4    ', 'cond=COND                     '];
T[' ADD  REG:OPT=$Rdupc REG=SP IMM:u16=$imm       ', 'NONE ', 'ADD_SP_i_T4 ', 'cond=COND asmonly=1 showqlf=1 '];
T[' ADDW REG:OPT=$Rdupc REG=SP IMM:u16=$imm       ', 'NONE ', 'ADD_SP_i_T4 ', 'cond=COND                     '];
T[' ADR  REG=$Rdupc ADDR:i32=$nlabel              ', 'NONE ', 'ADR_T2      ', 'cond=COND                     '];
T[' ADR  REG=$Rdupc ADDR:i32=$plabel              ', 'NONE ', 'ADR_T3      ', 'cond=COND showqlf=1           '];
T[' SUB  REG:OPT=$Rdupc REG=$Rnnsppc IMM:u16=$imm ', 'NONE ', 'SUB_i_T4    ', 'cond=COND asmonly=1 showqlf=1 '];
T[' SUBW REG:OPT=$Rdupc REG=$Rnnsppc IMM:u16=$imm ', 'NONE ', 'SUB_i_T4    ', 'cond=COND                     '];
T[' SUB  REG:OPT=$Rdupc REG=SP IMM:u16=$imm       ', 'NONE ', 'SUB_SP_i_T3 ', 'cond=COND asmonly=1 showqlf=1 '];
T[' SUBW REG:OPT=$Rdupc REG=SP IMM:u16=$imm       ', 'NONE ', 'SUB_SP_i_T3 ', 'cond=COND                     '];
T[' ADDW REG=$Rdupc REG=PC IMM:u16=$imm           ', 'NONE ', 'ADD_ADR_T3  ', 'cond=COND                     '];
T[' ADD  REG=$Rdupc REG=PC IMM:u16=$imm           ', 'NONE ', 'ADD_ADR_T3  ', 'cond=COND asmonly=1 showqlf=1 '];
T[' SUB  REG=$Rdupc REG=PC IMM:u16=$imm           ', 'NONE ', 'SUB_ADR_T2  ', 'cond=COND                     '];



# dpint_shiftr: Data-processing (shifted register).
# T32: ig0=0b1110101 op1:u=0bxxxx S:u=0bx Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx
HINT('ICLASS', 'dpint_shiftr', 'T32');

DEF Rd      => GPR32('Rd', 'Rd', 'write')                     ;
DEF Rdnpc   => GPR32('Rd', 'Rd', 'write|nopc')                ;
DEF Rdupc   => GPR32('Rd', 'Rd', 'write|upc')                 ;
DEF Rmupc   => GPR32('Rm', 'Rm', 'read|upc')                  ;
DEF Rnnpc   => GPR32('Rn', 'Rn', 'read|nopc')                 ;
DEF Rnnsp   => GPR32('Rn', 'Rn', 'read|nosp|upc')             ;
DEF Rnupc   => GPR32('Rn', 'Rn', 'read|upc')                  ;
DEF amount0 => IMM('amount', 'imm3:imm2', range => (1, 31))   ;
DEF amount2 => IMM('amount', 'imm3:imm2', range => (0, 31))   ;
DEF amount3 => IMM('amount', 'imm3:imm2')                     ;
DEF imm     => IMM('imm', 'imm3:imm2', range => (1, 32))      ;
DEF imm2    => IMM('imm', 'imm3:imm2', range => (0, 31))      ;
DEF imm3    => IMM('imm', 'imm3:imm2', range => (1, 31))      ;
DEF shift0  => SHIFT('shift', 'stype', table => 'tbl_shift1') ;

T[' ADC   REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'ADC_r_T2_RRX       ', 'cond=COND           '];
T[' ADC   REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'ADC_r_T2           ', 'cond=COND showqlf=1 '];
T[' ADCS  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'ADCS_r_T2_RRX      ', 'cond=COND           '];
T[' ADCS  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'ADCS_r_T2          ', 'cond=COND showqlf=1 '];
T[' ADD   REG:OPT=$Rd REG=$Rnnsp REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                               ', 'NONE ', 'ADD_r_T3_RRX       ', 'cond=COND           '];
T[' ADD   REG:OPT=$Rd REG=$Rnnsp REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}    ', 'NONE ', 'ADD_r_T3           ', 'cond=COND showqlf=1 '];
T[' ADDS  REG:OPT=$Rdnpc REG=$Rnnsp REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'ADDS_r_T3_RRX      ', 'cond=COND           '];
T[' ADDS  REG:OPT=$Rdnpc REG=$Rnnsp REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'ADDS_r_T3          ', 'cond=COND showqlf=1 '];
T[' ADD   REG:OPT=$Rd REG=SP REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                                   ', 'NONE ', 'ADD_SP_r_T3_RRX    ', 'cond=COND           '];
T[' ADD   REG:OPT=$Rd REG=SP REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}        ', 'NONE ', 'ADD_SP_r_T3        ', 'cond=COND showqlf=1 '];
T[' ADDS  REG:OPT=$Rdnpc REG=SP REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                                ', 'NONE ', 'ADDS_SP_r_T3_RRX   ', 'cond=COND           '];
T[' ADDS  REG:OPT=$Rdnpc REG=SP REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}     ', 'NONE ', 'ADDS_SP_r_T3       ', 'cond=COND           '];
T[' AND   REG:OPT=$Rd REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                               ', 'NONE ', 'AND_r_T2_RRX       ', 'cond=COND           '];
T[' AND   REG:OPT=$Rd REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}    ', 'NONE ', 'AND_r_T2           ', 'cond=COND showqlf=1 '];
T[' ANDS  REG:OPT=$Rdnpc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'ANDS_r_T2_RRX      ', 'cond=COND           '];
T[' ANDS  REG:OPT=$Rdnpc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'ANDS_r_T2          ', 'cond=COND showqlf=1 '];
T[' BIC   REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'BIC_r_T2_RRX       ', 'cond=COND           '];
T[' BIC   REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'BIC_r_T2           ', 'cond=COND showqlf=1 '];
T[' BICS  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'BICS_r_T2_RRX      ', 'cond=COND           '];
T[' BICS  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'BICS_r_T2          ', 'cond=COND showqlf=1 '];
T[' CMN   REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                                           ', 'NONE ', 'CMN_r_T2_RRX       ', 'cond=COND           '];
T[' CMN   REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}                ', 'NONE ', 'CMN_r_T2           ', 'cond=COND showqlf=1 '];
T[' CMP   REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                                           ', 'NONE ', 'CMP_r_T3_RRX       ', 'cond=COND           '];
T[' CMP   REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 AMOUNT:u8=$amount0 }}                    ', 'NONE ', 'CMP_r_T3           ', 'cond=COND showqlf=1 '];
T[' EOR   REG:OPT=$Rd REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                               ', 'NONE ', 'EOR_r_T2_RRX       ', 'cond=COND           '];
T[' EOR   REG:OPT=$Rd REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}    ', 'NONE ', 'EOR_r_T2           ', 'cond=COND showqlf=1 '];
T[' EORS  REG:OPT=$Rdnpc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'EORS_r_T2_RRX      ', 'cond=COND           '];
T[' EORS  REG:OPT=$Rdnpc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'EORS_r_T2          ', 'cond=COND showqlf=1 '];
T[' MOV   REG=$Rdupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                                           ', 'NONE ', 'MOV_r_T3_RRX       ', 'cond=COND           '];
T[' MOV   REG=$Rdupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }}                ', 'NONE ', 'MOV_r_T3           ', 'cond=COND showqlf=1 '];
T[' MOVS  REG=$Rdupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                                           ', 'NONE ', 'MOVS_r_T3_RRX      ', 'cond=COND           '];
T[' MOVS  REG=$Rdupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }}                ', 'NONE ', 'MOVS_r_T3          ', 'cond=COND showqlf=1 '];
T[' MVN   REG=$Rdupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                                           ', 'NONE ', 'MVN_r_T2_RRX       ', 'cond=COND           '];
T[' MVN   REG=$Rdupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}                ', 'NONE ', 'MVN_r_T2           ', 'cond=COND showqlf=1 '];
T[' MVNS  REG=$Rdupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                                           ', 'NONE ', 'MVNS_r_T2_RRX      ', 'cond=COND           '];
T[' MVNS  REG=$Rdupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}                ', 'NONE ', 'MVNS_r_T2          ', 'cond=COND showqlf=1 '];
T[' ORN   REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'ORN_r_T1_RRX       ', 'cond=COND           '];
T[' ORN   REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'ORN_r_T1           ', 'cond=COND           '];
T[' ORNS  REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'ORNS_r_T1_RRX      ', 'cond=COND           '];
T[' ORNS  REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'ORNS_r_T1          ', 'cond=COND           '];
T[' ORR   REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'ORR_r_T2_RRX       ', 'cond=COND           '];
T[' ORR   REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'ORR_r_T2           ', 'cond=COND showqlf=1 '];
T[' ORRS  REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'ORRS_r_T2_RRX      ', 'cond=COND           '];
T[' ORRS  REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'ORRS_r_T2          ', 'cond=COND showqlf=1 '];
T[' PKHBT REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount3}}     ', 'NONE ', 'PKHBT_T1           ', 'cond=COND           '];
T[' PKHTB REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ASR AMOUNT:i32=$amount3}}     ', 'NONE ', 'PKHTB_T1           ', 'cond=COND           '];
T[' RSB   REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'RSB_r_T1_RRX       ', 'cond=COND           '];
T[' RSB   REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'RSB_r_T1           ', 'cond=COND           '];
T[' RSBS  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'RSBS_r_T1_RRX      ', 'cond=COND           '];
T[' RSBS  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'RSBS_r_T1          ', 'cond=COND           '];
T[' SBC   REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'SBC_r_T2_RRX       ', 'cond=COND           '];
T[' SBC   REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'SBC_r_T2           ', 'cond=COND showqlf=1 '];
T[' SBCS  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'SBCS_r_T2_RRX      ', 'cond=COND           '];
T[' SBCS  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'SBCS_r_T2          ', 'cond=COND showqlf=1 '];
T[' SUB   REG:OPT=$Rd REG=$Rnnsp REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                               ', 'NONE ', 'SUB_r_T2_RRX       ', 'cond=COND           '];
T[' SUB   REG:OPT=$Rd REG=$Rnnsp REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}    ', 'NONE ', 'SUB_r_T2           ', 'cond=COND showqlf=1 '];
T[' SUBS  REG:OPT=$Rdnpc REG=$Rnnsp REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'SUBS_r_T2_RRX      ', 'cond=COND           '];
T[' SUBS  REG:OPT=$Rdnpc REG=$Rnnsp REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'SUBS_r_T2          ', 'cond=COND showqlf=1 '];
T[' SUB   REG:OPT=$Rd REG=SP REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                                   ', 'NONE ', 'SUB_SP_r_T1_RRX    ', 'cond=COND           '];
T[' SUB   REG:OPT=$Rd REG=SP REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}        ', 'NONE ', 'SUB_SP_r_T1        ', 'cond=COND showqlf=1 '];
T[' SUBS  REG:OPT=$Rdnpc REG=SP REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                                ', 'NONE ', 'SUBS_SP_r_T1_RRX   ', 'cond=COND           '];
T[' SUBS  REG:OPT=$Rdnpc REG=SP REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}     ', 'NONE ', 'SUBS_SP_r_T1       ', 'cond=COND           '];
T[' TEQ   REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                                           ', 'NONE ', 'TEQ_r_T1_RRX       ', 'cond=COND           '];
T[' TEQ   REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}                ', 'NONE ', 'TEQ_r_T1           ', 'cond=COND           '];
T[' TST   REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}                                           ', 'NONE ', 'TST_r_T2_RRX       ', 'cond=COND           '];
T[' TST   REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}                ', 'NONE ', 'TST_r_T2           ', 'cond=COND showqlf=1 '];
T[' ASR   REG:OPT=$Rdupc REG=$Rmupc IMM:u8=$imm                                                        ', 'NONE ', 'ASR_MOV_r_T3       ', 'cond=COND showqlf=1 '];
T[' ASRS  REG:OPT=$Rdupc REG=$Rmupc IMM:u8=$imm                                                        ', 'NONE ', 'ASRS_MOVS_r_T3     ', 'cond=COND showqlf=1 '];
T[' LSL   REG:OPT=$Rdupc REG=$Rmupc IMM:u8=$imm2                                                       ', 'NONE ', 'LSL_MOV_r_T3       ', 'cond=COND showqlf=1 '];
T[' LSLS  REG:OPT=$Rdupc REG=$Rmupc IMM:u8=$imm2                                                       ', 'NONE ', 'LSLS_MOVS_r_T3     ', 'cond=COND showqlf=1 '];
T[' LSR   REG:OPT=$Rdupc REG=$Rmupc IMM:u8=$imm                                                        ', 'NONE ', 'LSR_MOV_r_T3       ', 'cond=COND showqlf=1 '];
T[' LSRS  REG:OPT=$Rdupc REG=$Rmupc IMM:u8=$imm                                                        ', 'NONE ', 'LSRS_MOVS_r_T3     ', 'cond=COND showqlf=1 '];
T[' ROR   REG:OPT=$Rdupc REG=$Rmupc IMM:u8=$imm3                                                       ', 'NONE ', 'ROR_MOV_r_T3       ', 'cond=COND           '];
T[' RORS  REG:OPT=$Rdupc REG=$Rmupc IMM:u8=$imm3                                                       ', 'NONE ', 'RORS_MOVS_r_T3     ', 'cond=COND           '];
T[' RRX   REG:OPT=$Rdupc REG=$Rmupc                                                                    ', 'NONE ', 'RRX_MOV_r_T3_RRX   ', 'cond=COND           '];
T[' RRXS  REG:OPT=$Rdupc REG=$Rmupc                                                                    ', 'NONE ', 'RRXS_MOVS_r_T3_RRX ', 'cond=COND           '];



# eret: Exception Return.
# A32: cond:u!=0b1111 ig0=0b00010110 ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ ig13=0b0110 ig14=0bS ig15=0bS ig16=0bS ig17=0bZ
HINT('ICLASS', 'eret', 'A32');


T[' ERET  ', 'NONE ', 'ERET_A1 ', 'cond=NO_NV '];



# eret: Exception return.
# T32: ig0=0b111100111101 Rn:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 ig4=0bS ig5=0bS ig6=0bS ig7=0bS imm8:u=0bxxxxxxxx
HINT('ICLASS', 'eret', 'T32');

DEF imm => IMM('imm8', 'imm8', range => (0, 255)) ;

T[' ERET                           ', 'NONE ', 'ERET_T1       ', 'cond=COND '];
T[' SUBS REG=PC REG=LR IMM:u8=$imm ', 'NONE ', 'SUBS_PC_T5_AS ', 'cond=COND '];



# except: Exception Generation.
# A32: cond:u!=0b1111 ig0=0b00010 opc:u=0bxx ig1=0b0 imm12:u=0bxxxxxxxxxxxx ig2=0b0111 imm4:u=0bxxxx
HINT('ICLASS', 'except', 'A32');

DEF imm  => IMM('imm', 'imm12:imm4', range => (0, 65535))   ;
DEF imm2 => IMM('imm16', 'imm12:imm4', range => (0, 65535)) ;
DEF imm3 => IMM('imm4', 'imm4', range => (0, 15))           ;

T[' BKPT IMM:u16=$imm  ', 'NONE ', 'BKPT_A1   ', 'cond=NO_NV_UNP_COND '];
T[' HLT  IMM:u16=$imm  ', 'NONE ', 'HLT_A1    ', 'cond=NO_NV_UNP_COND '];
T[' HVC  IMM:u16=$imm2 ', 'NONE ', 'HVC_A1    ', 'cond=NO_NV_UNP_COND '];
T[' SMC  IMM:u8=$imm3  ', 'NONE ', 'SMC_A1_AS ', 'cond=NO_NV          '];



# except: Exception generation.
# T32: ig0=0b11110111111 o1:u=0bx imm4:u=0bxxxx ig1=0b10 o2:u=0bx ig2=0b0 imm12:u=0bxxxxxxxxxxxx
HINT('ICLASS', 'except', 'T32');

DEF imm  => IMM('imm16', 'imm4:imm12', range => (0, 65535)) ;
DEF imm2 => IMM('imm4', 'imm4', range => (0, 15))           ;
DEF imm3 => IMM('imm', 'imm4:imm12', range => (0, 65535))   ;

T[' HVC IMM:u16=$imm  ', 'NONE ', 'HVC_T1    ', 'cond=UNP_COND       '];
T[' SMC IMM:u8=$imm2  ', 'NONE ', 'SMC_T1_AS ', 'cond=COND           '];
T[' UDF IMM:u16=$imm3 ', 'NONE ', 'UDF_T2    ', 'cond=COND showqlf=1 '];



# extend: Extend and Add.
# A32: cond:u!=0b1111 ig0=0b01101 U:u=0bx op:u=0bxx Rn:u=0bxxxx Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx
HINT('ICLASS', 'extend', 'A32');

DEF Rdupc   => GPR32('Rd', 'Rd', 'write|upc')               ;
DEF Rmupc   => GPR32('Rm', 'Rm', 'read|upc')                ;
DEF Rnnpc   => GPR32('Rn', 'Rn', 'read|nopc')               ;
DEF amount0 => IMM('amount', 'rotate', table => 'tbl_imm0') ;

T[' SXTAB   REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}} ', 'NONE ', 'SXTAB_A1   ', 'cond=NO_NV '];
T[' SXTAB16 REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}} ', 'NONE ', 'SXTAB16_A1 ', 'cond=NO_NV '];
T[' SXTAH   REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}} ', 'NONE ', 'SXTAH_A1   ', 'cond=NO_NV '];
T[' SXTB    REG:OPT=$Rdupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}}            ', 'NONE ', 'SXTB_A1    ', 'cond=NO_NV '];
T[' SXTB16  REG:OPT=$Rdupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}}            ', 'NONE ', 'SXTB16_A1  ', 'cond=NO_NV '];
T[' SXTH    REG:OPT=$Rdupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}}            ', 'NONE ', 'SXTH_A1    ', 'cond=NO_NV '];
T[' UXTAB   REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}} ', 'NONE ', 'UXTAB_A1   ', 'cond=NO_NV '];
T[' UXTAB16 REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}} ', 'NONE ', 'UXTAB16_A1 ', 'cond=NO_NV '];
T[' UXTAH   REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}} ', 'NONE ', 'UXTAH_A1   ', 'cond=NO_NV '];
T[' UXTB    REG:OPT=$Rdupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}}            ', 'NONE ', 'UXTB_A1    ', 'cond=NO_NV '];
T[' UXTB16  REG:OPT=$Rdupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}}            ', 'NONE ', 'UXTB16_A1  ', 'cond=NO_NV '];
T[' UXTH    REG:OPT=$Rdupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}}            ', 'NONE ', 'UXTH_A1    ', 'cond=NO_NV '];



# extendr: Register extends.
# T32: ig0=0b111110100 op1:u=0bxx U:u=0bx Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx
HINT('ICLASS', 'extendr', 'T32');

DEF Rdupc   => GPR32('Rd', 'Rd', 'write|upc')               ;
DEF Rmupc   => GPR32('Rm', 'Rm', 'read|upc')                ;
DEF Rnnpc   => GPR32('Rn', 'Rn', 'read|nopc')               ;
DEF amount0 => IMM('amount', 'rotate', table => 'tbl_imm0') ;

T[' SXTAB   REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}} ', 'NONE ', 'SXTAB_T1   ', 'cond=COND           '];
T[' SXTAB16 REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}} ', 'NONE ', 'SXTAB16_T1 ', 'cond=COND           '];
T[' SXTAH   REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}} ', 'NONE ', 'SXTAH_T1   ', 'cond=COND           '];
T[' SXTB    REG:OPT=$Rdupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}}            ', 'NONE ', 'SXTB_T2    ', 'cond=COND showqlf=1 '];
T[' SXTB16  REG:OPT=$Rdupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}}            ', 'NONE ', 'SXTB16_T1  ', 'cond=COND           '];
T[' SXTH    REG:OPT=$Rdupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}}            ', 'NONE ', 'SXTH_T2    ', 'cond=COND showqlf=1 '];
T[' UXTAB   REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}} ', 'NONE ', 'UXTAB_T1   ', 'cond=COND           '];
T[' UXTAB16 REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}} ', 'NONE ', 'UXTAB16_T1 ', 'cond=COND           '];
T[' UXTAH   REG:OPT=$Rdupc REG=$Rnnpc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}} ', 'NONE ', 'UXTAH_T1   ', 'cond=COND           '];
T[' UXTB    REG:OPT=$Rdupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}}            ', 'NONE ', 'UXTB_T2    ', 'cond=COND showqlf=1 '];
T[' UXTB16  REG:OPT=$Rdupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}}            ', 'NONE ', 'UXTB16_T1  ', 'cond=COND           '];
T[' UXTH    REG:OPT=$Rdupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ROR AMOUNT:i32=$amount0}}            ', 'NONE ', 'UXTH_T2    ', 'cond=COND showqlf=1 '];



# hints: Hints.
# T32: ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint:u=0bxxxx option:u=0bxxxx
HINT('ICLASS', 'hints', 'T32');

DEF option => IMM('option', 'option', range => (0, 15)) ;

T[' CSDB                 ', 'NONE ', 'CSDB_T1  ', 'cond=UNP_COND showqlf=1                                     '];
T[' DBG   IMM:u8=$option ', 'NONE ', 'DBG_T1   ', 'cond=COND                                                   '];
T[' ESB                  ', 'NONE ', 'ESB_T1   ', 'arch_variant=ARMv8v2 extensions=RAS cond=UNP_COND showqlf=1 '];
T[' NOP                  ', 'NONE ', 'NOP_T2   ', 'cond=COND showqlf=1                                         '];
T[' SEV                  ', 'NONE ', 'SEV_T2   ', 'cond=COND showqlf=1                                         '];
T[' SEVL                 ', 'NONE ', 'SEVL_T2  ', 'cond=COND showqlf=1                                         '];
T[' TSB   SYNC_OP=CSYNC  ', 'NONE ', 'TSB_T1   ', 'arch_variant=ARMv8v4 cond=UNP_COND                          '];
T[' WFE                  ', 'NONE ', 'WFE_T2   ', 'cond=COND showqlf=1                                         '];
T[' WFI                  ', 'NONE ', 'WFI_T2   ', 'cond=COND showqlf=1                                         '];
T[' YIELD                ', 'NONE ', 'YIELD_T2 ', 'cond=COND showqlf=1                                         '];



# intdp1reg_imm: Integer Test and Compare (one register and immediate).
# A32: cond:u!=0b1111 ig0=0b00110 opc:u=0bxx ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm12:u=0bxxxxxxxxxxxx
HINT('ICLASS', 'intdp1reg_imm', 'A32');

DEF Rndpc => GPR32('Rn', 'Rn', 'read|dpc') ;
DEF aimm  => IMM('aimm', 'imm12')          ;

T[' CMN REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'CMN_i_A1 ', 'cond=NO_NV '];
T[' CMP REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'CMP_i_A1 ', 'cond=NO_NV '];
T[' TEQ REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'TEQ_i_A1 ', 'cond=NO_NV '];
T[' TST REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'TST_i_A1 ', 'cond=NO_NV '];



# intdp2reg_imm: Integer Data Processing (two register and immediate).
# A32: cond:u!=0b1111 ig0=0b0010 opc:u=0bxxx S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx
HINT('ICLASS', 'intdp2reg_imm', 'A32');

DEF Rd      => GPR32('Rd', 'Rd', 'write')          ;
DEF Rddpc   => GPR32('Rd', 'Rd', 'write|dpc')      ;
DEF Rndpc   => GPR32('Rn', 'Rn', 'read|dpc')       ;
DEF Rnnsp   => GPR32('Rn', 'Rn', 'read|nosp')      ;
DEF Rnnsppc => GPR32('Rn', 'Rn', 'read|nosp|nopc') ;
DEF aimm    => IMM('aimm', 'imm12')                ;
DEF nlabela => LABEL('nlabela', 'imm12')           ;
DEF plabela => LABEL('plabela', 'imm12')           ;

T[' ADC  REG:OPT:iwb=$Rddpc REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'ADC_i_A1     ', 'cond=NO_NV '];
T[' ADCS REG:OPT:erb=$Rddpc REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'ADCS_i_A1    ', 'cond=NO_NV '];
T[' ADD  REG:OPT:iwb=$Rd REG=$Rnnsppc IMM:i32=$aimm  ', 'NONE ', 'ADD_i_A1     ', 'cond=NO_NV '];
T[' ADDS REG:OPT:erb=$Rddpc REG=$Rnnsp IMM:i32=$aimm ', 'NONE ', 'ADDS_i_A1    ', 'cond=NO_NV '];
T[' ADD  REG:OPT:iwb=$Rddpc REG=SP IMM:i32=$aimm     ', 'NONE ', 'ADD_SP_i_A1  ', 'cond=NO_NV '];
T[' ADDS REG:OPT:erb=$Rddpc REG=SP IMM:i32=$aimm     ', 'NONE ', 'ADDS_SP_i_A1 ', 'cond=NO_NV '];
T[' ADR  REG:iwb=$Rd ADDR:i32=$plabela               ', 'NONE ', 'ADR_A1       ', 'cond=NO_NV '];
T[' ADR  REG:iwb=$Rd ADDR:i32=$nlabela               ', 'NONE ', 'ADR_A2       ', 'cond=NO_NV '];
T[' AND  REG:OPT:iwb=$Rddpc REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'AND_i_A1     ', 'cond=NO_NV '];
T[' ANDS REG:OPT:erb=$Rddpc REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'ANDS_i_A1    ', 'cond=NO_NV '];
T[' EOR  REG:OPT:iwb=$Rddpc REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'EOR_i_A1     ', 'cond=NO_NV '];
T[' EORS REG:OPT:erb=$Rddpc REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'EORS_i_A1    ', 'cond=NO_NV '];
T[' RSB  REG:OPT:iwb=$Rddpc REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'RSB_i_A1     ', 'cond=NO_NV '];
T[' RSBS REG:OPT:erb=$Rddpc REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'RSBS_i_A1    ', 'cond=NO_NV '];
T[' RSC  REG:OPT:iwb=$Rddpc REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'RSC_i_A1     ', 'cond=NO_NV '];
T[' RSCS REG:OPT:erb=$Rddpc REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'RSCS_i_A1    ', 'cond=NO_NV '];
T[' SBC  REG:OPT:iwb=$Rddpc REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'SBC_i_A1     ', 'cond=NO_NV '];
T[' SBCS REG:OPT:erb=$Rddpc REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'SBCS_i_A1    ', 'cond=NO_NV '];
T[' SUB  REG:OPT:iwb=$Rd REG=$Rnnsppc IMM:i32=$aimm  ', 'NONE ', 'SUB_i_A1     ', 'cond=NO_NV '];
T[' SUBS REG:OPT:erb=$Rddpc REG=$Rnnsp IMM:i32=$aimm ', 'NONE ', 'SUBS_i_A1    ', 'cond=NO_NV '];
T[' SUB  REG:OPT:iwb=$Rd REG=SP IMM:i32=$aimm        ', 'NONE ', 'SUB_SP_i_A1  ', 'cond=NO_NV '];
T[' SUBS REG:OPT:erb=$Rddpc REG=SP IMM:i32=$aimm     ', 'NONE ', 'SUBS_SP_i_A1 ', 'cond=NO_NV '];
T[' ADD  REG:iwb=$Rd REG=PC IMM:i32=$aimm            ', 'NONE ', 'ADD_ADR_A1   ', 'cond=NO_NV '];
T[' SUB  REG:iwb=$Rd REG=PC IMM:i32=$aimm            ', 'NONE ', 'SUB_ADR_A2   ', 'cond=NO_NV '];



# intdp2reg_immsh: Integer Test and Compare (two register, immediate shift).
# A32: cond:u!=0b1111 ig0=0b00010 opc:u=0bxx ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm5:u=0bxxxxx stype:u=0bxx ig6=0b0 Rm:u=0bxxxx
HINT('ICLASS', 'intdp2reg_immsh', 'A32');

DEF Rmdpc   => GPR32('Rm', 'Rm', 'read|dpc')                  ;
DEF Rndpc   => GPR32('Rn', 'Rn', 'read|dpc')                  ;
DEF amount0 => IMM('amount', 'imm5', range => (1, 31))        ;
DEF shift0  => SHIFT('shift', 'stype', table => 'tbl_shift1') ;

T[' CMN REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'CMN_r_A1_RRX ', 'cond=NO_NV '];
T[' CMN REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'CMN_r_A1     ', 'cond=NO_NV '];
T[' CMP REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'CMP_r_A1_RRX ', 'cond=NO_NV '];
T[' CMP REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'CMP_r_A1     ', 'cond=NO_NV '];
T[' TEQ REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'TEQ_r_A1_RRX ', 'cond=NO_NV '];
T[' TEQ REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'TEQ_r_A1     ', 'cond=NO_NV '];
T[' TST REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'TST_r_A1_RRX ', 'cond=NO_NV '];
T[' TST REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'TST_r_A1     ', 'cond=NO_NV '];



# intdp2reg_regsh: Integer Test and Compare (two register, register shift).
# A32: cond:u!=0b1111 ig0=0b00010 opc:u=0bxx ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ Rs:u=0bxxxx ig6=0b0 stype:u=0bxx ig7=0b1 Rm:u=0bxxxx
HINT('ICLASS', 'intdp2reg_regsh', 'A32');

DEF Rmupc => GPR32('Rm', 'Rm', 'read|upc')                 ;
DEF Rnupc => GPR32('Rn', 'Rn', 'read|upc')                 ;
DEF Rsupc => GPR32('Rs', 'Rs', 'upc')                      ;
DEF type  => SHIFT('type', 'stype', table => 'tbl_shift1') ;

T[' CMN REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$type RS:r=$Rsupc}} ', 'NONE ', 'CMN_rr_A1 ', 'cond=NO_NV '];
T[' CMP REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$type RS:r=$Rsupc}} ', 'NONE ', 'CMP_rr_A1 ', 'cond=NO_NV '];
T[' TEQ REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$type RS:r=$Rsupc}} ', 'NONE ', 'TEQ_rr_A1 ', 'cond=NO_NV '];
T[' TST REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$type RS:r=$Rsupc}} ', 'NONE ', 'TST_rr_A1 ', 'cond=NO_NV '];



# intdp3reg_immsh: Integer Data Processing (three register, immediate shift).
# A32: cond:u!=0b1111 ig0=0b0000 opc:u=0bxxx S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx
HINT('ICLASS', 'intdp3reg_immsh', 'A32');

DEF Rd      => GPR32('Rd', 'Rd', 'write')                     ;
DEF Rddpc   => GPR32('Rd', 'Rd', 'write|dpc')                 ;
DEF Rmdpc   => GPR32('Rm', 'Rm', 'read|dpc')                  ;
DEF Rndpc   => GPR32('Rn', 'Rn', 'read|dpc')                  ;
DEF Rnnsp   => GPR32('Rn', 'Rn', 'read|nosp')                 ;
DEF Rnnsp2  => GPR32('Rn', 'Rn', 'read|nosp|dpc')             ;
DEF amount0 => IMM('amount', 'imm5', range => (1, 31))        ;
DEF shift0  => SHIFT('shift', 'stype', table => 'tbl_shift1') ;

T[' ADC  REG:OPT:iwb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                             ', 'NONE ', 'ADC_r_A1_RRX     ', 'cond=NO_NV '];
T[' ADC  REG:OPT:iwb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}  ', 'NONE ', 'ADC_r_A1         ', 'cond=NO_NV '];
T[' ADCS REG:OPT:erb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                             ', 'NONE ', 'ADCS_r_A1_RRX    ', 'cond=NO_NV '];
T[' ADCS REG:OPT:erb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}  ', 'NONE ', 'ADCS_r_A1        ', 'cond=NO_NV '];
T[' ADD  REG:OPT:iwb=$Rd REG=$Rnnsp REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                                ', 'NONE ', 'ADD_r_A1_RRX     ', 'cond=NO_NV '];
T[' ADD  REG:OPT:iwb=$Rd REG=$Rnnsp REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}     ', 'NONE ', 'ADD_r_A1         ', 'cond=NO_NV '];
T[' ADDS REG:OPT:erb=$Rddpc REG=$Rnnsp REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                             ', 'NONE ', 'ADDS_r_A1_RRX    ', 'cond=NO_NV '];
T[' ADDS REG:OPT:erb=$Rddpc REG=$Rnnsp REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}  ', 'NONE ', 'ADDS_r_A1        ', 'cond=NO_NV '];
T[' ADD  REG:OPT:iwb=$Rddpc REG=SP REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                                 ', 'NONE ', 'ADD_SP_r_A1_RRX  ', 'cond=NO_NV '];
T[' ADD  REG:OPT:iwb=$Rddpc REG=SP REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}      ', 'NONE ', 'ADD_SP_r_A1      ', 'cond=NO_NV '];
T[' ADDS REG:OPT:erb=$Rddpc REG=SP REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                                 ', 'NONE ', 'ADDS_SP_r_A1_RRX ', 'cond=NO_NV '];
T[' ADDS REG:OPT:erb=$Rddpc REG=SP REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}      ', 'NONE ', 'ADDS_SP_r_A1     ', 'cond=NO_NV '];
T[' AND  REG:OPT:iwb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                             ', 'NONE ', 'AND_r_A1_RRX     ', 'cond=NO_NV '];
T[' AND  REG:OPT:iwb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}  ', 'NONE ', 'AND_r_A1         ', 'cond=NO_NV '];
T[' ANDS REG:OPT:erb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                             ', 'NONE ', 'ANDS_r_A1_RRX    ', 'cond=NO_NV '];
T[' ANDS REG:OPT:erb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}  ', 'NONE ', 'ANDS_r_A1        ', 'cond=NO_NV '];
T[' EOR  REG:OPT:iwb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                             ', 'NONE ', 'EOR_r_A1_RRX     ', 'cond=NO_NV '];
T[' EOR  REG:OPT:iwb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}  ', 'NONE ', 'EOR_r_A1         ', 'cond=NO_NV '];
T[' EORS REG:OPT:erb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                             ', 'NONE ', 'EORS_r_A1_RRX    ', 'cond=NO_NV '];
T[' EORS REG:OPT:erb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}  ', 'NONE ', 'EORS_r_A1        ', 'cond=NO_NV '];
T[' RSB  REG:OPT:iwb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                             ', 'NONE ', 'RSB_r_A1_RRX     ', 'cond=NO_NV '];
T[' RSB  REG:OPT:iwb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}  ', 'NONE ', 'RSB_r_A1         ', 'cond=NO_NV '];
T[' RSBS REG:OPT:erb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                             ', 'NONE ', 'RSBS_r_A1_RRX    ', 'cond=NO_NV '];
T[' RSBS REG:OPT:erb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}  ', 'NONE ', 'RSBS_r_A1        ', 'cond=NO_NV '];
T[' RSC  REG:OPT:iwb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                             ', 'NONE ', 'RSC_r_A1_RRX     ', 'cond=NO_NV '];
T[' RSC  REG:OPT:iwb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}  ', 'NONE ', 'RSC_r_A1         ', 'cond=NO_NV '];
T[' RSCS REG:OPT:erb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                             ', 'NONE ', 'RSCS_r_A1_RRX    ', 'cond=NO_NV '];
T[' RSCS REG:OPT:erb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}  ', 'NONE ', 'RSCS_r_A1        ', 'cond=NO_NV '];
T[' SBC  REG:OPT:iwb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                             ', 'NONE ', 'SBC_r_A1_RRX     ', 'cond=NO_NV '];
T[' SBC  REG:OPT:iwb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}  ', 'NONE ', 'SBC_r_A1         ', 'cond=NO_NV '];
T[' SBCS REG:OPT:erb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                             ', 'NONE ', 'SBCS_r_A1_RRX    ', 'cond=NO_NV '];
T[' SBCS REG:OPT:erb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}  ', 'NONE ', 'SBCS_r_A1        ', 'cond=NO_NV '];
T[' SUB  REG:OPT:iwb=$Rddpc REG=$Rnnsp2 REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'SUB_r_A1_RRX     ', 'cond=NO_NV '];
T[' SUB  REG:OPT:iwb=$Rddpc REG=$Rnnsp2 REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'SUB_r_A1         ', 'cond=NO_NV '];
T[' SUBS REG:OPT:erb=$Rddpc REG=$Rnnsp2 REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'SUBS_r_A1_RRX    ', 'cond=NO_NV '];
T[' SUBS REG:OPT:erb=$Rddpc REG=$Rnnsp2 REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'SUBS_r_A1        ', 'cond=NO_NV '];
T[' SUB  REG:OPT:iwb=$Rddpc REG=SP REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                                 ', 'NONE ', 'SUB_SP_r_A1_RRX  ', 'cond=NO_NV '];
T[' SUB  REG:OPT:iwb=$Rddpc REG=SP REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}      ', 'NONE ', 'SUB_SP_r_A1      ', 'cond=NO_NV '];
T[' SUBS REG:OPT:erb=$Rddpc REG=SP REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                                 ', 'NONE ', 'SUBS_SP_r_A1_RRX ', 'cond=NO_NV '];
T[' SUBS REG:OPT:erb=$Rddpc REG=SP REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}      ', 'NONE ', 'SUBS_SP_r_A1     ', 'cond=NO_NV '];



# intdp3reg_regsh: Integer Data Processing (three register, register shift).
# A32: cond:u!=0b1111 ig0=0b0000 opc:u=0bxxx S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx
HINT('ICLASS', 'intdp3reg_regsh', 'A32');

DEF Rdupc  => GPR32('Rd', 'Rd', 'write|upc')                 ;
DEF Rmupc  => GPR32('Rm', 'Rm', 'read|upc')                  ;
DEF Rnupc  => GPR32('Rn', 'Rn', 'read|upc')                  ;
DEF Rsupc  => GPR32('Rs', 'Rs', 'upc')                       ;
DEF shift0 => SHIFT('shift', 'stype', table => 'tbl_shift1') ;

T[' ADCS REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'ADCS_rr_A1 ', 'cond=NO_NV '];
T[' ADC  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'ADC_rr_A1  ', 'cond=NO_NV '];
T[' ADDS REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'ADDS_rr_A1 ', 'cond=NO_NV '];
T[' ADD  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'ADD_rr_A1  ', 'cond=NO_NV '];
T[' ANDS REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'ANDS_rr_A1 ', 'cond=NO_NV '];
T[' AND  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'AND_rr_A1  ', 'cond=NO_NV '];
T[' EORS REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'EORS_rr_A1 ', 'cond=NO_NV '];
T[' EOR  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'EOR_rr_A1  ', 'cond=NO_NV '];
T[' RSBS REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'RSBS_rr_A1 ', 'cond=NO_NV '];
T[' RSB  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'RSB_rr_A1  ', 'cond=NO_NV '];
T[' RSCS REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'RSCS_rr_A1 ', 'cond=NO_NV '];
T[' RSC  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'RSC_rr_A1  ', 'cond=NO_NV '];
T[' SBCS REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'SBCS_rr_A1 ', 'cond=NO_NV '];
T[' SBC  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'SBC_rr_A1  ', 'cond=NO_NV '];
T[' SUBS REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'SUBS_rr_A1 ', 'cond=NO_NV '];
T[' SUB  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'SUB_rr_A1  ', 'cond=NO_NV '];



# intsat: Integer Saturating Arithmetic.
# A32: cond:u!=0b1111 ig0=0b00010 opc:u=0bxx ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b0101 Rm:u=0bxxxx
HINT('ICLASS', 'intsat', 'A32');

DEF Rdupc => GPR32('Rd', 'Rd', 'write|upc') ;
DEF Rmupc => GPR32('Rm', 'Rm', 'read|upc')  ;
DEF Rnupc => GPR32('Rn', 'Rn', 'read|upc')  ;

T[' QADD  REG:OPT=$Rdupc REG=$Rmupc REG=$Rnupc ', 'NONE ', 'QADD_A1  ', 'cond=NO_NV '];
T[' QDADD REG:OPT=$Rdupc REG=$Rmupc REG=$Rnupc ', 'NONE ', 'QDADD_A1 ', 'cond=NO_NV '];
T[' QDSUB REG:OPT=$Rdupc REG=$Rmupc REG=$Rnupc ', 'NONE ', 'QDSUB_A1 ', 'cond=NO_NV '];
T[' QSUB  REG:OPT=$Rdupc REG=$Rmupc REG=$Rnupc ', 'NONE ', 'QSUB_A1  ', 'cond=NO_NV '];



# ldastl: Load-acquire / Store-release.
# T32: ig0=0b11101000110 L:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx ig1=0b1 op:u=0bx sz:u=0bxx Rd:u=0bxxxx
HINT('ICLASS', 'ldastl', 'T32');

DEF Rdupc  => GPR32('Rd', 'Rd', 'write|upc') ;
DEF Rnupc  => GPR32('Rn', 'Rn', 'read|upc')  ;
DEF Rtupc  => GPR32('Rt', 'Rt', 'upc')       ;
DEF Rtupc2 => GPR32('Rt2', 'Rt2', 'upc')     ;

T[' LDA    REG:w=$Rtupc MEM:r:i32={BASE=$Rnupc SZ=32 LDACC=ACQUIRE}                                 ', 'NONE ', 'LDA_T1    ', 'cond=COND '];
T[' LDAB   REG:w=$Rtupc MEM:r:i8={ BASE=$Rnupc SZ=8  LDACC=ACQUIRE}                                 ', 'NONE ', 'LDAB_T1   ', 'cond=COND '];
T[' LDAEX  REG:w=$Rtupc MEM:r:i32={BASE=$Rnupc SZ=32 LDACC=ATOMIC_ACQUIRE}                          ', 'NONE ', 'LDAEX_T1  ', 'cond=COND '];
T[' LDAEXB REG:w=$Rtupc MEM:r:i8={ BASE=$Rnupc SZ=8  LDACC=ATOMIC_ACQUIRE}                          ', 'NONE ', 'LDAEXB_T1 ', 'cond=COND '];
T[' LDAEXD REG:w=$Rtupc REG:w=$Rtupc2 MEM:r:i64={BASE=$Rnupc SZ=64 LDACC=ATOMIC_ACQUIRE}            ', 'NONE ', 'LDAEXD_T1 ', 'cond=COND '];
T[' LDAEXH REG:w=$Rtupc MEM:r:i16={BASE=$Rnupc SZ=16 LDACC=ATOMIC_ACQUIRE}                          ', 'NONE ', 'LDAEXH_T1 ', 'cond=COND '];
T[' LDAH   REG:w=$Rtupc MEM:r:i16={BASE=$Rnupc SZ=16 LDACC=ACQUIRE}                                 ', 'NONE ', 'LDAH_T1   ', 'cond=COND '];
T[' STL    REG:r=$Rtupc MEM:w:i32={BASE=$Rnupc SZ=32 STACC=RELEASE}                                 ', 'NONE ', 'STL_T1    ', 'cond=COND '];
T[' STLB   REG:r=$Rtupc MEM:w:i8={ BASE=$Rnupc SZ=8  STACC=RELEASE}                                 ', 'NONE ', 'STLB_T1   ', 'cond=COND '];
T[' STLEX  REG=$Rdupc REG:r=$Rtupc MEM:w:i32={BASE=$Rnupc SZ=32 STACC=ATOMIC_RELEASE}               ', 'NONE ', 'STLEX_T1  ', 'cond=COND '];
T[' STLEXB REG=$Rdupc REG:r=$Rtupc MEM:w:i8={ BASE=$Rnupc SZ=8  STACC=ATOMIC_RELEASE}               ', 'NONE ', 'STLEXB_T1 ', 'cond=COND '];
T[' STLEXD REG=$Rdupc REG:r=$Rtupc REG:r=$Rtupc2 MEM:w:i64={BASE=$Rnupc SZ=64 STACC=ATOMIC_RELEASE} ', 'NONE ', 'STLEXD_T1 ', 'cond=COND '];
T[' STLEXH REG=$Rdupc REG:r=$Rtupc MEM:w:i16={BASE=$Rnupc SZ=16 STACC=ATOMIC_RELEASE}               ', 'NONE ', 'STLEXH_T1 ', 'cond=COND '];
T[' STLH   REG:r=$Rtupc MEM:w:i16={BASE=$Rnupc SZ=16 STACC=RELEASE}                                 ', 'NONE ', 'STLH_T1   ', 'cond=COND '];



# lddlit: Load dual (literal).
# T32: ig0=0b1110100 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx L:u=0bx ig2=0b1111 Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx
HINT('ICLASS', 'lddlit', 'T32');

DEF ADD    => ADD('ADD', 'U')                                     ;
DEF Rtupc  => GPR32('Rt', 'Rt', 'upc')                            ;
DEF Rtupc2 => GPR32('Rt2', 'Rt2', 'upc')                          ;
DEF imm    => IMM('imm', 'imm8', range => (0, 255), default => 0) ;

T[' LDRD REG:w=$Rtupc REG:w=$Rtupc2 LM:OFF:r:i32={BASE=PC SZ=2x32 ADD=$ADD IMMOFF:u8=$imm} ', 'NONE ', 'LDRD_l_T1 ', 'cond=COND '];



# ldlit_signed: Load, signed (literal).
# T32: ig0=0b11111001 U:u=0bx size:u=0bxx ig1=0b11111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx
HINT('ICLASS', 'ldlit_signed', 'T32');

DEF ADD   => ADD('ADD', 'U')                         ;
DEF Rtnpc => GPR32('Rt', 'Rt', 'nopc')               ;
DEF imm   => IMM('imm', 'imm12', range => (0, 4095)) ;

T[' LDRSB REG:w=$Rtnpc LM:OFF:r:s8={ BASE=PC SZ=8  ADD=$ADD IMMOFF:u16=$imm} ', 'NONE ', 'LDRSB_l_T1 ', 'cond=COND '];
T[' LDRSH REG:w=$Rtnpc LM:OFF:r:s16={BASE=PC SZ=16 ADD=$ADD IMMOFF:u16=$imm} ', 'NONE ', 'LDRSH_l_T1 ', 'cond=COND '];
T[' PLI   LM:OFF:r:i32={BASE=PC SZ=32 ADD=$ADD IMMOFF:u16=$imm}              ', 'NONE ', 'PLI_i_T3   ', 'cond=COND '];



# ldlit_unsigned: Load, unsigned (literal).
# T32: ig0=0b11111000 U:u=0bx size:u=0bxx L:u=0bx ig1=0b1111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx
HINT('ICLASS', 'ldlit_unsigned', 'T32');

DEF ADD   => ADD('ADD', 'U')                         ;
DEF Rt    => GPR32('Rt', 'Rt')                       ;
DEF Rtnpc => GPR32('Rt', 'Rt', 'nopc')               ;
DEF imm   => IMM('imm', 'imm12', range => (0, 4095)) ;

T[' LDR  REG:w:iwb=$Rt LM:OFF:r:i32={BASE=PC SZ=32 ADD=$ADD IMMOFF:u16=$imm} ', 'NONE ', 'LDR_l_T2  ', 'cond=COND showqlf=1 '];
T[' LDRB REG:w=$Rtnpc LM:OFF:r:i8={ BASE=PC SZ=8  ADD=$ADD IMMOFF:u16=$imm}  ', 'NONE ', 'LDRB_l_T1 ', 'cond=COND           '];
T[' LDRH REG:w=$Rtnpc LM:OFF:r:i16={BASE=PC SZ=16 ADD=$ADD IMMOFF:u16=$imm}  ', 'NONE ', 'LDRH_l_T1 ', 'cond=COND           '];
T[' PLD  LM:OFF:r:i32={BASE=PC SZ=32 ADD=$ADD IMMOFF:u16=$imm}               ', 'NONE ', 'PLD_l_T1  ', 'cond=COND           '];



# ldst_excl: Load/Store Exclusive and Load-Acquire/Store-Release.
# A32: cond:u!=0b1111 ig0=0b00011 size:u=0bxx L:u=0bx Rn:u=0bxxxx xRd:u=0bxxxx ig1=0bS ig2=0bS ex:u=0bx ord:u=0bx ig3=0b1001 xRt:u=0bxxxx
HINT('ICLASS', 'ldst_excl', 'A32');

DEF Rdupc  => GPR32('Rd', 'Rd', 'write|upc') ;
DEF Rnupc  => GPR32('Rn', 'Rn', 'read|upc')  ;
DEF Rt     => GPR32('Rt', 'Rt', 'even')      ;
DEF Rtupc  => GPR32('Rt', 'Rt', 'upc')       ;
DEF Rtupc2 => GPR32('Rt2', 'Rt', 'pair|upc') ;
DEF imm    => IMM('imm', undef)              ;

T[' LDA    REG:w=$Rtupc MEM:r:i32={BASE=$Rnupc SZ=32 LDACC=ACQUIRE}                                   ', 'NONE ', 'LDA_A1    ', 'cond=NO_NV '];
T[' LDAB   REG:w=$Rtupc MEM:r:i8={ BASE=$Rnupc SZ=8  LDACC=ACQUIRE}                                   ', 'NONE ', 'LDAB_A1   ', 'cond=NO_NV '];
T[' LDAEX  REG:w=$Rtupc MEM:r:i32={BASE=$Rnupc SZ=32 LDACC=ATOMIC_ACQUIRE}                            ', 'NONE ', 'LDAEX_A1  ', 'cond=NO_NV '];
T[' LDAEXB REG:w=$Rtupc MEM:r:i8={ BASE=$Rnupc SZ=8  LDACC=ATOMIC_ACQUIRE}                            ', 'NONE ', 'LDAEXB_A1 ', 'cond=NO_NV '];
T[' LDAEXD REG:w=$Rt REG:w=$Rtupc2 MEM:r:i64={BASE=$Rnupc SZ=64 LDACC=ATOMIC_ACQUIRE}                 ', 'NONE ', 'LDAEXD_A1 ', 'cond=NO_NV '];
T[' LDAEXH REG:w=$Rtupc MEM:r:i16={BASE=$Rnupc SZ=16 LDACC=ATOMIC_ACQUIRE}                            ', 'NONE ', 'LDAEXH_A1 ', 'cond=NO_NV '];
T[' LDAH   REG:w=$Rtupc MEM:r:i16={BASE=$Rnupc SZ=16 LDACC=ACQUIRE}                                   ', 'NONE ', 'LDAH_A1   ', 'cond=NO_NV '];
T[' LDREX  REG:w=$Rtupc MEM:OFF:r:i32={BASE=$Rnupc SZ=32 IMMOFF:OPT:i32=$imm LDACC=ATOMIC}            ', 'NONE ', 'LDREX_A1  ', 'cond=NO_NV '];
T[' LDREXB REG:w=$Rtupc MEM:r:i8={ BASE=$Rnupc SZ=8  LDACC=ATOMIC}                                    ', 'NONE ', 'LDREXB_A1 ', 'cond=NO_NV '];
T[' LDREXD REG:w=$Rt REG:w=$Rtupc2 MEM:r:i64={BASE=$Rnupc SZ=64 LDACC=ATOMIC}                         ', 'NONE ', 'LDREXD_A1 ', 'cond=NO_NV '];
T[' LDREXH REG:w=$Rtupc MEM:r:i16={BASE=$Rnupc SZ=16 LDACC=ATOMIC}                                    ', 'NONE ', 'LDREXH_A1 ', 'cond=NO_NV '];
T[' STL    REG:r=$Rtupc MEM:w:i32={BASE=$Rnupc SZ=32 STACC=RELEASE}                                   ', 'NONE ', 'STL_A1    ', 'cond=NO_NV '];
T[' STLB   REG:r=$Rtupc MEM:w:i8={ BASE=$Rnupc SZ=8  STACC=RELEASE}                                   ', 'NONE ', 'STLB_A1   ', 'cond=NO_NV '];
T[' STLEX  REG=$Rdupc REG:r=$Rtupc MEM:w:i32={BASE=$Rnupc SZ=32 STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'STLEX_A1  ', 'cond=NO_NV '];
T[' STLEXB REG=$Rdupc REG:r=$Rtupc MEM:w:i8={ BASE=$Rnupc SZ=8  STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'STLEXB_A1 ', 'cond=NO_NV '];
T[' STLEXD REG=$Rdupc REG:r=$Rt REG:r=$Rtupc2 MEM:w:i64={BASE=$Rnupc SZ=64 STACC=ATOMIC_RELEASE}      ', 'NONE ', 'STLEXD_A1 ', 'cond=NO_NV '];
T[' STLEXH REG=$Rdupc REG:r=$Rtupc MEM:w:i16={BASE=$Rnupc SZ=16 STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'STLEXH_A1 ', 'cond=NO_NV '];
T[' STLH   REG:r=$Rtupc MEM:w:i16={BASE=$Rnupc SZ=16 STACC=RELEASE}                                   ', 'NONE ', 'STLH_A1   ', 'cond=NO_NV '];
T[' STREX  REG=$Rdupc REG:r=$Rtupc MEM:OFF:w:i32={BASE=$Rnupc SZ=32 IMMOFF:OPT:i32=$imm STACC=ATOMIC} ', 'NONE ', 'STREX_A1  ', 'cond=NO_NV '];
T[' STREXB REG=$Rdupc REG:r=$Rtupc MEM:w:i8={ BASE=$Rnupc SZ=8  STACC=ATOMIC}                         ', 'NONE ', 'STREXB_A1 ', 'cond=NO_NV '];
T[' STREXD REG=$Rdupc REG:r=$Rt REG:r=$Rtupc2 MEM:w:i64={BASE=$Rnupc SZ=64 STACC=ATOMIC}              ', 'NONE ', 'STREXD_A1 ', 'cond=NO_NV '];
T[' STREXH REG=$Rdupc REG:r=$Rtupc MEM:w:i16={BASE=$Rnupc SZ=16 STACC=ATOMIC}                         ', 'NONE ', 'STREXH_A1 ', 'cond=NO_NV '];



# ldst_signed_nimm: Load/store, signed (negative immediate).
# T32: ig0=0b111110010 size:u=0bxx ig1=0b1 Rn:u!=0b1111 Rt:u=0bxxxx ig2=0b1100 imm8:u=0bxxxxxxxx
HINT('ICLASS', 'ldst_signed_nimm', 'T32');

DEF Rnnpc => GPR32('Rn', 'Rn', 'read|nopc')                      ;
DEF Rtnpc => GPR32('Rt', 'Rt', 'nopc')                           ;
DEF imm   => IMM('imm', 'imm8', range => (0, 255), default => 0) ;

T[' LDRSB REG:w=$Rtnpc MEM:OFF:r:s8={ BASE=$Rnnpc SZ=8  ADD=0 IMMOFF:OPT:u8=$imm} ', 'NONE ', 'LDRSB_i_T2_off ', 'cond=COND '];
T[' LDRSH REG:w=$Rtnpc MEM:OFF:r:s16={BASE=$Rnnpc SZ=16 ADD=0 IMMOFF:OPT:u8=$imm} ', 'NONE ', 'LDRSH_i_T2_off ', 'cond=COND '];
T[' PLI   MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=0 IMMOFF:OPT:u8=$imm}              ', 'NONE ', 'PLI_i_T2       ', 'cond=COND '];



# ldst_signed_pimm: Load/store, signed (positive immediate).
# T32: ig0=0b111110011 size:u=0bxx ig1=0b1 Rn:u!=0b1111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx
HINT('ICLASS', 'ldst_signed_pimm', 'T32');

DEF Rnnpc => GPR32('Rn', 'Rn', 'read|nopc')                        ;
DEF Rtnpc => GPR32('Rt', 'Rt', 'nopc')                             ;
DEF imm   => IMM('imm', 'imm12', range => (0, 4095), default => 0) ;

T[' LDRSB REG:w=$Rtnpc MEM:OFF:r:s8={ BASE=$Rnnpc SZ=8  ADD=1 IMMOFF:OPT:u16=$imm} ', 'NONE ', 'LDRSB_i_T1 ', 'cond=COND '];
T[' LDRSH REG:w=$Rtnpc MEM:OFF:r:s16={BASE=$Rnnpc SZ=16 ADD=1 IMMOFF:OPT:u16=$imm} ', 'NONE ', 'LDRSH_i_T1 ', 'cond=COND '];
T[' PLI   MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=1 IMMOFF:OPT:u16=$imm}              ', 'NONE ', 'PLI_i_T1   ', 'cond=COND '];



# ldst_signed_post: Load/store, signed (immediate, post-indexed).
# T32: ig0=0b111110010 size:u=0bxx ig1=0b1 Rn:u!=0b1111 Rt:u=0bxxxx ig2=0b10 U:u=0bx ig3=0b1 imm8:u=0bxxxxxxxx
HINT('ICLASS', 'ldst_signed_post', 'T32');

DEF ADD   => ADD('ADD', 'U')                                     ;
DEF Rnnpc => GPR32('Rn', 'Rn', 'read|nopc')                      ;
DEF Rt    => GPR32('Rt', 'Rt')                                   ;
DEF imm   => IMM('imm', 'imm8', range => (0, 255), default => 0) ;

T[' LDRSB REG:w=$Rt MEM:PST:r:s8={ BASE=$Rnnpc SZ=8  ADD=$ADD IMMOFF:u8=$imm} ', 'NONE ', 'LDRSB_i_T2_post ', 'cond=COND '];
T[' LDRSH REG:w=$Rt MEM:PST:r:s16={BASE=$Rnnpc SZ=16 ADD=$ADD IMMOFF:u8=$imm} ', 'NONE ', 'LDRSH_i_T2_post ', 'cond=COND '];



# ldst_signed_pre: Load/store, signed (immediate, pre-indexed).
# T32: ig0=0b111110010 size:u=0bxx ig1=0b1 Rn:u!=0b1111 Rt:u=0bxxxx ig2=0b11 U:u=0bx ig3=0b1 imm8:u=0bxxxxxxxx
HINT('ICLASS', 'ldst_signed_pre', 'T32');

DEF ADD   => ADD('ADD', 'U')                                     ;
DEF Rnnpc => GPR32('Rn', 'Rn', 'read|nopc')                      ;
DEF Rt    => GPR32('Rt', 'Rt')                                   ;
DEF imm   => IMM('imm', 'imm8', range => (0, 255), default => 0) ;

T[' LDRSB REG:w=$Rt MEM:PRE:r:s8={ BASE=$Rnnpc SZ=8  ADD=$ADD IMMOFF:u8=$imm} ', 'NONE ', 'LDRSB_i_T2_pre ', 'cond=COND '];
T[' LDRSH REG:w=$Rt MEM:PRE:r:s16={BASE=$Rnnpc SZ=16 ADD=$ADD IMMOFF:u8=$imm} ', 'NONE ', 'LDRSH_i_T2_pre ', 'cond=COND '];



# ldst_signed_reg: Load/store, signed (register offset).
# T32: ig0=0b111110010 size:u=0bxx ig1=0b1 Rn:u!=0b1111 Rt:u=0bxxxx ig2=0b000000 imm2:u=0bxx Rm:u=0bxxxx
HINT('ICLASS', 'ldst_signed_reg', 'T32');

DEF Rmupc   => GPR32('Rm', 'Rm', 'read|upc')                        ;
DEF Rnnpc   => GPR32('Rn', 'Rn', 'read|nopc')                       ;
DEF Rtnpc   => GPR32('Rt', 'Rt', 'nopc')                            ;
DEF amount0 => IMM('amount', 'imm2')                                ;
DEF amount2 => IMM('amount', 'imm2', range => (0, 3), default => 0) ;

T[' LDRSB REG:w=$Rtnpc MEM:OFF:r:s8={ BASE=$Rnnpc SZ=8  ADD=1 REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount0}}} ', 'NONE ', 'LDRSB_r_T2 ', 'cond=COND showqlf=1 '];
T[' LDRSH REG:w=$Rtnpc MEM:OFF:r:s16={BASE=$Rnnpc SZ=16 ADD=1 REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount0}}} ', 'NONE ', 'LDRSH_r_T2 ', 'cond=COND showqlf=1 '];
T[' PLI   MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=1 REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=LSL AMOUNT:u8=$amount2 }}}              ', 'NONE ', 'PLI_r_T1   ', 'cond=COND           '];



# ldst_signed_unpriv: Load/store, signed (unprivileged).
# T32: ig0=0b111110010 size:u=0bxx ig1=0b1 Rn:u!=0b1111 Rt:u=0bxxxx ig2=0b1110 imm8:u=0bxxxxxxxx
HINT('ICLASS', 'ldst_signed_unpriv', 'T32');

DEF Rnnpc => GPR32('Rn', 'Rn', 'read|nopc')                      ;
DEF Rtupc => GPR32('Rt', 'Rt', 'upc')                            ;
DEF imm   => IMM('imm', 'imm8', range => (0, 255), default => 0) ;

T[' LDRSBT REG:w=$Rtupc MEM:OFF:r:s8={ BASE=$Rnnpc SZ=8  ADD=1 IMMOFF:OPT:u8=$imm LDACC=UNPRIVILEGED} ', 'NONE ', 'LDRSBT_T1 ', 'cond=COND '];
T[' LDRSHT REG:w=$Rtupc MEM:OFF:r:s16={BASE=$Rnnpc SZ=16 ADD=1 IMMOFF:OPT:u8=$imm LDACC=UNPRIVILEGED} ', 'NONE ', 'LDRSHT_T1 ', 'cond=COND '];



# ldst_unsigned_nimm: Load/store, unsigned (negative immediate).
# T32: ig0=0b111110000 size:u=0bxx L:u=0bx Rn:u!=0b1111 Rt:u=0bxxxx ig1=0b1100 imm8:u=0bxxxxxxxx
HINT('ICLASS', 'ldst_unsigned_nimm', 'T32');

DEF Rnnpc => GPR32('Rn', 'Rn', 'read|nopc')                      ;
DEF Rt    => GPR32('Rt', 'Rt')                                   ;
DEF Rtnpc => GPR32('Rt', 'Rt', 'nopc')                           ;
DEF Rtupc => GPR32('Rt', 'Rt', 'upc')                            ;
DEF imm   => IMM('imm', 'imm8', range => (0, 255), default => 0) ;

T[' LDR  REG:w:iwb=$Rt MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=0 IMMOFF:OPT:u8=$imm} ', 'NONE ', 'LDR_i_T4_off   ', 'cond=COND '];
T[' LDRB REG:w=$Rtnpc MEM:OFF:r:i8={ BASE=$Rnnpc SZ=8  ADD=0 IMMOFF:OPT:u8=$imm}  ', 'NONE ', 'LDRB_i_T3_off  ', 'cond=COND '];
T[' LDRH REG:w=$Rtnpc MEM:OFF:r:i16={BASE=$Rnnpc SZ=16 ADD=0 IMMOFF:OPT:u8=$imm}  ', 'NONE ', 'LDRH_i_T3_off  ', 'cond=COND '];
T[' PLD  MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=0 IMMOFF:OPT:u8=$imm}               ', 'NONE ', 'PLD_i_T2       ', 'cond=COND '];
T[' PLDW MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=0 IMMOFF:OPT:u8=$imm}               ', 'NONE ', 'PLDW_i_T2      ', 'cond=COND '];
T[' STR  REG:r=$Rtupc MEM:OFF:w:i32={BASE=$Rnnpc SZ=32 ADD=0 IMMOFF:OPT:u8=$imm}  ', 'NONE ', 'STR_i_T4_off   ', 'cond=COND '];
T[' STRB REG:r=$Rtupc MEM:OFF:w:i8={ BASE=$Rnnpc SZ=8  ADD=0 IMMOFF:OPT:u8=$imm}  ', 'NONE ', 'STRB_i_T3_offn ', 'cond=COND '];
T[' STRH REG:r=$Rtupc MEM:OFF:w:i16={BASE=$Rnnpc SZ=16 ADD=0 IMMOFF:OPT:u8=$imm}  ', 'NONE ', 'STRH_i_T3_offn ', 'cond=COND '];



# ldst_unsigned_pimm: Load/store, unsigned (positive immediate).
# T32: ig0=0b111110001 size:u=0bxx L:u=0bx Rn:u!=0b1111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx
HINT('ICLASS', 'ldst_unsigned_pimm', 'T32');

DEF Rnnpc => GPR32('Rn', 'Rn', 'read|nopc')                        ;
DEF Rt    => GPR32('Rt', 'Rt')                                     ;
DEF Rtnpc => GPR32('Rt', 'Rt', 'nopc')                             ;
DEF Rtupc => GPR32('Rt', 'Rt', 'upc')                              ;
DEF imm   => IMM('imm', 'imm12', range => (0, 4095), default => 0) ;

T[' LDR  REG:w:iwb=$Rt MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=1 IMMOFF:OPT:u16=$imm} ', 'NONE ', 'LDR_i_T3  ', 'cond=COND showqlf=1 '];
T[' LDRB REG:w=$Rtnpc MEM:OFF:r:i8={ BASE=$Rnnpc SZ=8  ADD=1 IMMOFF:OPT:u16=$imm}  ', 'NONE ', 'LDRB_i_T2 ', 'cond=COND showqlf=1 '];
T[' LDRH REG:w=$Rtnpc MEM:OFF:r:i16={BASE=$Rnnpc SZ=16 ADD=1 IMMOFF:OPT:u16=$imm}  ', 'NONE ', 'LDRH_i_T2 ', 'cond=COND showqlf=1 '];
T[' PLD  MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=1 IMMOFF:OPT:u16=$imm}               ', 'NONE ', 'PLD_i_T1  ', 'cond=COND           '];
T[' PLDW MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=1 IMMOFF:OPT:u16=$imm}               ', 'NONE ', 'PLDW_i_T1 ', 'cond=COND           '];
T[' STR  REG:r=$Rtupc MEM:OFF:w:i32={BASE=$Rnnpc SZ=32 ADD=1 IMMOFF:OPT:u16=$imm}  ', 'NONE ', 'STR_i_T3  ', 'cond=COND showqlf=1 '];
T[' STRB REG:r=$Rtupc MEM:OFF:w:i8={ BASE=$Rnnpc SZ=8  ADD=1 IMMOFF:OPT:u16=$imm}  ', 'NONE ', 'STRB_i_T2 ', 'cond=COND showqlf=1 '];
T[' STRH REG:r=$Rtupc MEM:OFF:w:i16={BASE=$Rnnpc SZ=16 ADD=1 IMMOFF:OPT:u16=$imm}  ', 'NONE ', 'STRH_i_T2 ', 'cond=COND showqlf=1 '];



# ldst_unsigned_post: Load/store, unsigned (immediate, post-indexed).
# T32: ig0=0b111110000 size:u=0bxx L:u=0bx Rn:u!=0b1111 Rt:u=0bxxxx ig1=0b10 U:u=0bx ig2=0b1 imm8:u=0bxxxxxxxx
HINT('ICLASS', 'ldst_unsigned_post', 'T32');

DEF ADD                  => ADD('ADD', 'U')                                     ;
DEF Rnnpc                => GPR32('Rn', 'Rn', 'read|nopc')                      ;
DEF Rt                   => GPR32('Rt', 'Rt')                                   ;
DEF Rtupc                => GPR32('Rt', 'Rt', 'upc')                            ;
DEF imm                  => IMM('imm', 'imm8', range => (0, 255), default => 0) ;
DEF single_register_list => SLIST('single_register_list', 'Rt')                 ;

T[' LDR  REG:w:iwb=$Rt MEM:PST:r:i32={BASE=$Rnnpc SZ=32 ADD=$ADD IMMOFF:u8=$imm} ', 'NONE ', 'LDR_i_T4_post     ', 'cond=COND '];
T[' LDRB REG:w=$Rt MEM:PST:r:i8={ BASE=$Rnnpc SZ=8  ADD=$ADD IMMOFF:u8=$imm}     ', 'NONE ', 'LDRB_i_T3_post    ', 'cond=COND '];
T[' LDRH REG:w=$Rt MEM:PST:r:i16={BASE=$Rnnpc SZ=16 ADD=$ADD IMMOFF:u8=$imm}     ', 'NONE ', 'LDRH_i_T3_post    ', 'cond=COND '];
T[' STR  REG:r=$Rtupc MEM:PST:w:i32={BASE=$Rnnpc SZ=32 ADD=$ADD IMMOFF:u8=$imm}  ', 'NONE ', 'STR_i_T4_post     ', 'cond=COND '];
T[' STRB REG:r=$Rtupc MEM:PST:w:i8={ BASE=$Rnnpc SZ=8  ADD=$ADD IMMOFF:u8=$imm}  ', 'NONE ', 'STRB_i_T3_post    ', 'cond=COND '];
T[' STRH REG:r=$Rtupc MEM:PST:w:i16={BASE=$Rnnpc SZ=16 ADD=$ADD IMMOFF:u8=$imm}  ', 'NONE ', 'STRH_i_T3_post    ', 'cond=COND '];
T[' POP  SLIST=$single_register_list                                             ', 'NONE ', 'POP_LDR_i_T4_post ', 'cond=COND '];



# ldst_unsigned_pre: Load/store, unsigned (immediate, pre-indexed).
# T32: ig0=0b111110000 size:u=0bxx L:u=0bx Rn:u!=0b1111 Rt:u=0bxxxx ig1=0b11 U:u=0bx ig2=0b1 imm8:u=0bxxxxxxxx
HINT('ICLASS', 'ldst_unsigned_pre', 'T32');

DEF ADD                  => ADD('ADD', 'U')                                     ;
DEF Rnnpc                => GPR32('Rn', 'Rn', 'read|nopc')                      ;
DEF Rt                   => GPR32('Rt', 'Rt')                                   ;
DEF Rtupc                => GPR32('Rt', 'Rt', 'upc')                            ;
DEF imm                  => IMM('imm', 'imm8', range => (0, 255), default => 0) ;
DEF single_register_list => SLIST('single_register_list', 'Rt')                 ;

T[' LDR  REG:w:iwb=$Rt MEM:PRE:r:i32={BASE=$Rnnpc SZ=32 ADD=$ADD IMMOFF:u8=$imm} ', 'NONE ', 'LDR_i_T4_pre      ', 'cond=COND '];
T[' LDRB REG:w=$Rt MEM:PRE:r:i8={ BASE=$Rnnpc SZ=8  ADD=$ADD IMMOFF:u8=$imm}     ', 'NONE ', 'LDRB_i_T3_pre     ', 'cond=COND '];
T[' LDRH REG:w=$Rt MEM:PRE:r:i16={BASE=$Rnnpc SZ=16 ADD=$ADD IMMOFF:u8=$imm}     ', 'NONE ', 'LDRH_i_T3_pre     ', 'cond=COND '];
T[' STR  REG:r=$Rtupc MEM:PRE:w:i32={BASE=$Rnnpc SZ=32 ADD=$ADD IMMOFF:u8=$imm}  ', 'NONE ', 'STR_i_T4_pre      ', 'cond=COND '];
T[' STRB REG:r=$Rtupc MEM:PRE:w:i8={ BASE=$Rnnpc SZ=8  ADD=$ADD IMMOFF:u8=$imm}  ', 'NONE ', 'STRB_i_T3_pre     ', 'cond=COND '];
T[' STRH REG:r=$Rtupc MEM:PRE:w:i16={BASE=$Rnnpc SZ=16 ADD=$ADD IMMOFF:u8=$imm}  ', 'NONE ', 'STRH_i_T3_pre     ', 'cond=COND '];
T[' PUSH SLIST=$single_register_list                                             ', 'NONE ', 'PUSH_STR_i_T4_pre ', 'cond=COND '];



# ldst_unsigned_reg: Load/store, unsigned (register offset).
# T32: ig0=0b111110000 size:u=0bxx L:u=0bx Rn:u!=0b1111 Rt:u=0bxxxx ig1=0b000000 imm2:u=0bxx Rm:u=0bxxxx
HINT('ICLASS', 'ldst_unsigned_reg', 'T32');

DEF Rmupc   => GPR32('Rm', 'Rm', 'read|upc')                        ;
DEF Rnnpc   => GPR32('Rn', 'Rn', 'read|nopc')                       ;
DEF Rt      => GPR32('Rt', 'Rt')                                    ;
DEF Rtnpc   => GPR32('Rt', 'Rt', 'nopc')                            ;
DEF Rtupc   => GPR32('Rt', 'Rt', 'upc')                             ;
DEF amount0 => IMM('amount', 'imm2')                                ;
DEF amount2 => IMM('amount', 'imm2', range => (0, 3), default => 0) ;

T[' LDR  REG:w:iwb=$Rt MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=1 REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount0}}} ', 'NONE ', 'LDR_r_T2  ', 'cond=COND showqlf=1 '];
T[' LDRB REG:w=$Rtnpc MEM:OFF:r:i8={ BASE=$Rnnpc SZ=8  ADD=1 REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount0}}}  ', 'NONE ', 'LDRB_r_T2 ', 'cond=COND showqlf=1 '];
T[' LDRH REG:w=$Rtnpc MEM:OFF:r:i16={BASE=$Rnnpc SZ=16 ADD=1 REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount0}}}  ', 'NONE ', 'LDRH_r_T2 ', 'cond=COND showqlf=1 '];
T[' PLD  MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=1 REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=LSL AMOUNT:u8=$amount2 }}}               ', 'NONE ', 'PLD_r_T1  ', 'cond=COND           '];
T[' PLDW MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=1 REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=LSL AMOUNT:u8=$amount2 }}}               ', 'NONE ', 'PLDW_r_T1 ', 'cond=COND           '];
T[' STR  REG:r=$Rtupc MEM:OFF:w:i32={BASE=$Rnnpc SZ=32 ADD=1 REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount0}}}  ', 'NONE ', 'STR_r_T2  ', 'cond=COND showqlf=1 '];
T[' STRB REG:r=$Rtupc MEM:OFF:w:i8={ BASE=$Rnnpc SZ=8  ADD=1 REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount0}}}  ', 'NONE ', 'STRB_r_T2 ', 'cond=COND showqlf=1 '];
T[' STRH REG:r=$Rtupc MEM:OFF:w:i16={BASE=$Rnnpc SZ=16 ADD=1 REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount0}}}  ', 'NONE ', 'STRH_r_T2 ', 'cond=COND showqlf=1 '];



# ldst_unsigned_unpriv: Load/store, unsigned (unprivileged).
# T32: ig0=0b111110000 size:u=0bxx L:u=0bx Rn:u!=0b1111 Rt:u=0bxxxx ig1=0b1110 imm8:u=0bxxxxxxxx
HINT('ICLASS', 'ldst_unsigned_unpriv', 'T32');

DEF Rnnpc => GPR32('Rn', 'Rn', 'read|nopc')                      ;
DEF Rtupc => GPR32('Rt', 'Rt', 'upc')                            ;
DEF imm   => IMM('imm', 'imm8', range => (0, 255), default => 0) ;

T[' LDRBT REG:w=$Rtupc MEM:OFF:r:i8={ BASE=$Rnnpc SZ=8  ADD=1 IMMOFF:OPT:u8=$imm LDACC=UNPRIVILEGED} ', 'NONE ', 'LDRBT_T1 ', 'cond=COND '];
T[' LDRHT REG:w=$Rtupc MEM:OFF:r:i16={BASE=$Rnnpc SZ=16 ADD=1 IMMOFF:OPT:u8=$imm LDACC=UNPRIVILEGED} ', 'NONE ', 'LDRHT_T1 ', 'cond=COND '];
T[' LDRT  REG:w=$Rtupc MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=1 IMMOFF:OPT:u8=$imm LDACC=UNPRIVILEGED} ', 'NONE ', 'LDRT_T1  ', 'cond=COND '];
T[' STRBT REG:r=$Rtupc MEM:OFF:w:i8={ BASE=$Rnnpc SZ=8  ADD=1 IMMOFF:OPT:u8=$imm STACC=UNPRIVILEGED} ', 'NONE ', 'STRBT_T1 ', 'cond=COND '];
T[' STRHT REG:r=$Rtupc MEM:OFF:w:i16={BASE=$Rnnpc SZ=16 ADD=1 IMMOFF:OPT:u8=$imm STACC=UNPRIVILEGED} ', 'NONE ', 'STRHT_T1 ', 'cond=COND '];
T[' STRT  REG:r=$Rtupc MEM:OFF:w:i32={BASE=$Rnnpc SZ=32 ADD=1 IMMOFF:OPT:u8=$imm STACC=UNPRIVILEGED} ', 'NONE ', 'STRT_T1  ', 'cond=COND '];



# ldstcp: System register load/store.
# A32: cond:u=0bxxxx ig0=0b110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L:u=0bx Rn:u=0bxxxx CRd:u=0bxxxx ig1=0b111 cp15:u=0bx imm8:u=0bxxxxxxxx
HINT('ICLASS', 'ldstcp', 'A32');

DEF ADD    => ADD('ADD', 'U')                              ;
DEF Rn     => GPR32('Rn', 'Rn', 'read')                    ;
DEF Rndpc  => GPR32('Rn', 'Rn', 'read|dpc')                ;
DEF Rnnpc  => GPR32('Rn', 'Rn', 'read|nopc')               ;
DEF immx4  => IMM('imm', 'imm8', scale => 4, default => 0) ;
DEF option => IMM('option', 'imm8', range => (0, 255))     ;

T[' LDC PSPACE=P14 CSPACE=C5 MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=$ADD IMMOFF:OPT:i32=$immx4} ', 'NONE ', 'LDC_i_A1_off   ', 'cond=NO_NV '];
T[' LDC PSPACE=P14 CSPACE=C5 MEM:PST:r:i32={BASE=$Rnnpc SZ=32 ADD=$ADD IMMOFF:i32=$immx4}     ', 'NONE ', 'LDC_i_A1_post  ', 'cond=NO_NV '];
T[' LDC PSPACE=P14 CSPACE=C5 MEM:PRE:r:i32={BASE=$Rnnpc SZ=32 ADD=$ADD IMMOFF:i32=$immx4}     ', 'NONE ', 'LDC_i_A1_pre   ', 'cond=NO_NV '];
T[' LDC PSPACE=P14 CSPACE=C5 MEM:PST:r:i32={BASE=$Rnnpc SZ=32 IMMOFF:u8=$option}              ', 'NONE ', 'LDC_i_A1_unind ', 'cond=NO_NV '];
T[' LDC PSPACE=P14 CSPACE=C5 LM:OFF:r:i32={BASE=PC SZ=32 ADD=$ADD IMMOFF:i32=$immx4}          ', 'NONE ', 'LDC_l_A1       ', 'cond=NO_NV '];
T[' LDC PSPACE=P14 CSPACE=C5 MEM:PST:r:i32={BASE=PC SZ=32 IMMOFF:u8=$option}                  ', 'NONE ', 'LDC_l_A1       ', 'cond=NO_NV '];
T[' STC PSPACE=P14 CSPACE=C5 MEM:OFF:w:i32={BASE=$Rndpc SZ=32 ADD=$ADD IMMOFF:OPT:i32=$immx4} ', 'NONE ', 'STC_A1_off     ', 'cond=NO_NV '];
T[' STC PSPACE=P14 CSPACE=C5 MEM:PST:w:i32={BASE=$Rn SZ=32 ADD=$ADD IMMOFF:i32=$immx4}        ', 'NONE ', 'STC_A1_post    ', 'cond=NO_NV '];
T[' STC PSPACE=P14 CSPACE=C5 MEM:PRE:w:i32={BASE=$Rn SZ=32 ADD=$ADD IMMOFF:i32=$immx4}        ', 'NONE ', 'STC_A1_pre     ', 'cond=NO_NV '];
T[' STC PSPACE=P14 CSPACE=C5 MEM:PST:w:i32={BASE=$Rndpc SZ=32 IMMOFF:u8=$option}              ', 'NONE ', 'STC_A1_unind   ', 'cond=NO_NV '];



# ldstd_imm: Load/store dual (immediate).
# T32: ig0=0b11101001 U:u=0bx ig1=0b10 L:u=0bx Rn:u!=0b1111 Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx
HINT('ICLASS', 'ldstd_imm', 'T32');

DEF ADD    => ADD('ADD', 'U')                                                  ;
DEF Rnnpc  => GPR32('Rn', 'Rn', 'read|nopc')                                   ;
DEF Rnnpc2 => GPR32('Rn', 'Rn', 'read|nopc|upc')                               ;
DEF Rtupc  => GPR32('Rt', 'Rt', 'upc')                                         ;
DEF Rtupc2 => GPR32('Rt2', 'Rt2', 'upc')                                       ;
DEF immx4  => IMM('imm', 'imm8', scale => 4, range => (0, 1020), default => 0) ;

T[' LDRD REG:w=$Rtupc REG:w=$Rtupc2 MEM:OFF:r:i32={BASE=$Rnnpc SZ=2x32 ADD=$ADD IMMOFF:OPT:u16=$immx4}  ', 'NONE ', 'LDRD_i_T1_off ', 'cond=COND '];
T[' STRD REG:r=$Rtupc REG:r=$Rtupc2 MEM:OFF:w:i32={BASE=$Rnnpc2 SZ=2x32 ADD=$ADD IMMOFF:OPT:u16=$immx4} ', 'NONE ', 'STRD_i_T1_off ', 'cond=COND '];



# ldstd_post: Load/store dual (immediate, post-indexed).
# T32: ig0=0b11101000 U:u=0bx ig1=0b11 L:u=0bx Rn:u!=0b1111 Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx
HINT('ICLASS', 'ldstd_post', 'T32');

DEF ADD    => ADD('ADD', 'U')                                                  ;
DEF Rnnpc  => GPR32('Rn', 'Rn', 'read|nopc')                                   ;
DEF Rnnpc2 => GPR32('Rn', 'Rn', 'read|nopc|upc')                               ;
DEF Rtupc  => GPR32('Rt', 'Rt', 'upc')                                         ;
DEF Rtupc2 => GPR32('Rt2', 'Rt2', 'upc')                                       ;
DEF immx4  => IMM('imm', 'imm8', scale => 4, range => (0, 1020), default => 0) ;

T[' LDRD REG:w=$Rtupc REG:w=$Rtupc2 MEM:PST:r:i32={BASE=$Rnnpc SZ=2x32 ADD=$ADD IMMOFF:u16=$immx4}  ', 'NONE ', 'LDRD_i_T1_post ', 'cond=COND '];
T[' STRD REG:r=$Rtupc REG:r=$Rtupc2 MEM:PST:w:i32={BASE=$Rnnpc2 SZ=2x32 ADD=$ADD IMMOFF:u16=$immx4} ', 'NONE ', 'STRD_i_T1_post ', 'cond=COND '];



# ldstd_pre: Load/store dual (immediate, pre-indexed).
# T32: ig0=0b11101001 U:u=0bx ig1=0b11 L:u=0bx Rn:u!=0b1111 Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx
HINT('ICLASS', 'ldstd_pre', 'T32');

DEF ADD    => ADD('ADD', 'U')                                                  ;
DEF Rnnpc  => GPR32('Rn', 'Rn', 'read|nopc')                                   ;
DEF Rnnpc2 => GPR32('Rn', 'Rn', 'read|nopc|upc')                               ;
DEF Rtupc  => GPR32('Rt', 'Rt', 'upc')                                         ;
DEF Rtupc2 => GPR32('Rt2', 'Rt2', 'upc')                                       ;
DEF immx4  => IMM('imm', 'imm8', scale => 4, range => (0, 1020), default => 0) ;

T[' LDRD REG:w=$Rtupc REG:w=$Rtupc2 MEM:PRE:r:i32={BASE=$Rnnpc SZ=2x32 ADD=$ADD IMMOFF:u16=$immx4}  ', 'NONE ', 'LDRD_i_T1_pre ', 'cond=COND '];
T[' STRD REG:r=$Rtupc REG:r=$Rtupc2 MEM:PRE:w:i32={BASE=$Rnnpc2 SZ=2x32 ADD=$ADD IMMOFF:u16=$immx4} ', 'NONE ', 'STRD_i_T1_pre ', 'cond=COND '];



# ldstex: Load/store exclusive.
# T32: ig0=0b11101000010 L:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx
HINT('ICLASS', 'ldstex', 'T32');

DEF Rdupc => GPR32('Rd', 'Rd', 'write|upc') ;
DEF Rnupc => GPR32('Rn', 'Rn', 'read|upc')  ;
DEF Rtupc => GPR32('Rt', 'Rt', 'upc')       ;
DEF immx4 => IMM('imm', undef, scale => 4)  ;

T[' LDREX REG:w=$Rtupc MEM:OFF:r:i32={BASE=$Rnupc SZ=32 IMMOFF:OPT:i32=$immx4 LDACC=ATOMIC}            ', 'NONE ', 'LDREX_T1 ', 'cond=COND '];
T[' STREX REG=$Rdupc REG:r=$Rtupc MEM:OFF:w:i32={BASE=$Rnupc SZ=32 IMMOFF:OPT:i32=$immx4 STACC=ATOMIC} ', 'NONE ', 'STREX_T1 ', 'cond=COND '];



# ldstex_bhd: Load/store exclusive byte/half/dual.
# T32: ig0=0b11101000110 L:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx ig1=0b01 sz:u=0bxx Rd:u=0bxxxx
HINT('ICLASS', 'ldstex_bhd', 'T32');

DEF Rdupc  => GPR32('Rd', 'Rd', 'write|upc') ;
DEF Rnupc  => GPR32('Rn', 'Rn', 'read|upc')  ;
DEF Rtupc  => GPR32('Rt', 'Rt', 'upc')       ;
DEF Rtupc2 => GPR32('Rt2', 'Rt2', 'upc')     ;

T[' LDREXB REG:w=$Rtupc MEM:r:i8={ BASE=$Rnupc SZ=8  LDACC=ATOMIC}                          ', 'NONE ', 'LDREXB_T1 ', 'cond=COND '];
T[' LDREXD REG:w=$Rtupc REG:w=$Rtupc2 MEM:r:i64={BASE=$Rnupc SZ=64 LDACC=ATOMIC}            ', 'NONE ', 'LDREXD_T1 ', 'cond=COND '];
T[' LDREXH REG:w=$Rtupc MEM:r:i16={BASE=$Rnupc SZ=16 LDACC=ATOMIC}                          ', 'NONE ', 'LDREXH_T1 ', 'cond=COND '];
T[' STREXB REG=$Rdupc REG:r=$Rtupc MEM:w:i8={ BASE=$Rnupc SZ=8  STACC=ATOMIC}               ', 'NONE ', 'STREXB_T1 ', 'cond=COND '];
T[' STREXD REG=$Rdupc REG:r=$Rtupc REG:r=$Rtupc2 MEM:w:i64={BASE=$Rnupc SZ=64 STACC=ATOMIC} ', 'NONE ', 'STREXD_T1 ', 'cond=COND '];
T[' STREXH REG=$Rdupc REG:r=$Rtupc MEM:w:i16={BASE=$Rnupc SZ=16 STACC=ATOMIC}               ', 'NONE ', 'STREXH_T1 ', 'cond=COND '];



# ldstexcept: Exception Save/Restore.
# A32: ig0=0b1111100 P:u=0bx U:u=0bx S:u=0bx W:u=0bx L:u=0bx Rn:u=0bxxxx op:u=0bxxxxxxxxxxx mode:u=0bxxxxx
HINT('ICLASS', 'ldstexcept', 'A32');

DEF Rnupc => GPR32('Rn', 'Rn', 'read|upc')       ;
DEF mode  => IMM('mode', 'mode')                 ;
DEF wback => WBACK('wback', 'W', default => '0') ;

T[' RFEDA WREG={REG=$Rnupc WBACK:OPT=$wback}           ', 'NONE ', 'RFEDA_A1_AS ', 'cond=UNCOND             '];
T[' RFEDB WREG={REG=$Rnupc WBACK:OPT=$wback}           ', 'NONE ', 'RFEDB_A1_AS ', 'cond=UNCOND             '];
T[' RFEIA WREG={REG=$Rnupc WBACK:OPT=$wback}           ', 'NONE ', 'RFEIA_A1_AS ', 'cond=UNCOND             '];
T[' RFEIB WREG={REG=$Rnupc WBACK:OPT=$wback}           ', 'NONE ', 'RFEIB_A1_AS ', 'cond=UNCOND             '];
T[' SRSDA WREG={REG=SP WBACK:OPT=$wback} IMM:i32=$mode ', 'NONE ', 'SRSDA_A1_AS ', 'cond=UNCOND may_store=1 '];
T[' SRSDB WREG={REG=SP WBACK:OPT=$wback} IMM:i32=$mode ', 'NONE ', 'SRSDB_A1_AS ', 'cond=UNCOND may_store=1 '];
T[' SRSIA WREG={REG=SP WBACK:OPT=$wback} IMM:i32=$mode ', 'NONE ', 'SRSIA_A1_AS ', 'cond=UNCOND may_store=1 '];
T[' SRSIB WREG={REG=SP WBACK:OPT=$wback} IMM:i32=$mode ', 'NONE ', 'SRSIB_A1_AS ', 'cond=UNCOND may_store=1 '];



# ldstimm: Load/Store Word, Unsigned Byte (immediate, literal).
# A32: cond:u!=0b1111 ig0=0b010 P:u=0bx U:u=0bx o2:u=0bx W:u=0bx o1:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx
HINT('ICLASS', 'ldstimm', 'A32');

DEF ADD                  => ADD('ADD', 'U')                                       ;
DEF Rndpc                => GPR32('Rn', 'Rn', 'read|dpc')                         ;
DEF Rnnpc                => GPR32('Rn', 'Rn', 'read|nopc')                        ;
DEF Rnupc                => GPR32('Rn', 'Rn', 'read|upc')                         ;
DEF Rt                   => GPR32('Rt', 'Rt')                                     ;
DEF Rtdpc                => GPR32('Rt', 'Rt', 'dpc')                              ;
DEF Rtupc                => GPR32('Rt', 'Rt', 'upc')                              ;
DEF Rtupc2               => GPR32('Rt', 'Rt', 'upc|dpc')                          ;
DEF imm                  => IMM('imm', 'imm12', range => (0, 4095), default => 0) ;
DEF single_register_list => SLIST('single_register_list', 'Rt')                   ;

T[' LDR   REG:w:iwb=$Rt MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=$ADD IMMOFF:OPT:u16=$imm}                    ', 'NONE ', 'LDR_i_A1_off      ', 'cond=NO_NV '];
T[' LDR   REG:w:iwb=$Rt MEM:PST:r:i32={BASE=$Rnnpc SZ=32 ADD=$ADD IMMOFF:u16=$imm}                        ', 'NONE ', 'LDR_i_A1_post     ', 'cond=NO_NV '];
T[' LDR   REG:w:iwb=$Rt MEM:PRE:r:i32={BASE=$Rnnpc SZ=32 ADD=$ADD IMMOFF:u16=$imm}                        ', 'NONE ', 'LDR_i_A1_pre      ', 'cond=NO_NV '];
T[' LDR   REG:w:iwb=$Rt LM:OFF:r:i32={BASE=PC SZ=32 ADD=$ADD IMMOFF:u16=$imm}                             ', 'NONE ', 'LDR_l_A1          ', 'cond=NO_NV '];
T[' LDRB  REG:w=$Rtupc MEM:OFF:r:i8={ BASE=$Rnnpc SZ=8  ADD=$ADD IMMOFF:OPT:u16=$imm}                     ', 'NONE ', 'LDRB_i_A1_off     ', 'cond=NO_NV '];
T[' LDRB  REG:w=$Rtupc MEM:PST:r:i8={ BASE=$Rnnpc SZ=8  ADD=$ADD IMMOFF:u16=$imm}                         ', 'NONE ', 'LDRB_i_A1_post    ', 'cond=NO_NV '];
T[' LDRB  REG:w=$Rtupc MEM:PRE:r:i8={ BASE=$Rnnpc SZ=8  ADD=$ADD IMMOFF:u16=$imm}                         ', 'NONE ', 'LDRB_i_A1_pre     ', 'cond=NO_NV '];
T[' LDRB  REG:w=$Rtupc LM:OFF:r:i8={ BASE=PC SZ=8  ADD=$ADD IMMOFF:u16=$imm}                              ', 'NONE ', 'LDRB_l_A1         ', 'cond=NO_NV '];
T[' LDRBT REG:w=$Rtupc2 MEM:PST:r:i8={ BASE=$Rnupc SZ=8  ADD=$ADD IMMOFF:OPT:u16=$imm LDACC=UNPRIVILEGED} ', 'NONE ', 'LDRBT_A1          ', 'cond=NO_NV '];
T[' LDRT  REG:w=$Rtupc2 MEM:PST:r:i32={BASE=$Rnupc SZ=32 ADD=$ADD IMMOFF:OPT:u16=$imm LDACC=UNPRIVILEGED} ', 'NONE ', 'LDRT_A1           ', 'cond=NO_NV '];
T[' STR   REG:r=$Rtdpc MEM:OFF:w:i32={BASE=$Rndpc SZ=32 ADD=$ADD IMMOFF:OPT:u16=$imm}                     ', 'NONE ', 'STR_i_A1_off      ', 'cond=NO_NV '];
T[' STR   REG:r=$Rtdpc MEM:PST:w:i32={BASE=$Rndpc SZ=32 ADD=$ADD IMMOFF:u16=$imm}                         ', 'NONE ', 'STR_i_A1_post     ', 'cond=NO_NV '];
T[' STR   REG:r=$Rtdpc MEM:PRE:w:i32={BASE=$Rndpc SZ=32 ADD=$ADD IMMOFF:u16=$imm}                         ', 'NONE ', 'STR_i_A1_pre      ', 'cond=NO_NV '];
T[' STRB  REG:r=$Rtupc MEM:OFF:w:i8={ BASE=$Rndpc SZ=8  ADD=$ADD IMMOFF:OPT:u16=$imm}                     ', 'NONE ', 'STRB_i_A1_off     ', 'cond=NO_NV '];
T[' STRB  REG:r=$Rtupc MEM:PST:w:i8={ BASE=$Rndpc SZ=8  ADD=$ADD IMMOFF:u16=$imm}                         ', 'NONE ', 'STRB_i_A1_post    ', 'cond=NO_NV '];
T[' STRB  REG:r=$Rtupc MEM:PRE:w:i8={ BASE=$Rndpc SZ=8  ADD=$ADD IMMOFF:u16=$imm}                         ', 'NONE ', 'STRB_i_A1_pre     ', 'cond=NO_NV '];
T[' STRBT REG:r=$Rtupc2 MEM:PST:w:i8={ BASE=$Rnupc SZ=8  ADD=$ADD IMMOFF:OPT:u16=$imm STACC=UNPRIVILEGED} ', 'NONE ', 'STRBT_A1          ', 'cond=NO_NV '];
T[' STRT  REG:r=$Rtdpc MEM:PST:w:i32={BASE=$Rnupc SZ=32 ADD=$ADD IMMOFF:OPT:u16=$imm STACC=UNPRIVILEGED}  ', 'NONE ', 'STRT_A1           ', 'cond=NO_NV '];
T[' POP   SLIST=$single_register_list                                                                     ', 'NONE ', 'POP_LDR_i_A1_post ', 'cond=NO_NV '];
T[' PUSH  SLIST=$single_register_list                                                                     ', 'NONE ', 'PUSH_STR_i_A1_pre ', 'cond=NO_NV '];



# ldstm: Load/Store Multiple.
# A32: cond:u!=0b1111 ig0=0b100 P:u=0bx U:u=0bx op:u=0bx W:u=0bx L:u=0bx Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx
HINT('ICLASS', 'ldstm', 'A32');

DEF Rnupc                => GPR32('Rn', 'Rn', 'read|upc')                  ;
DEF registers            => GLIST('registers', 'register_list')            ;
DEF registers_with_pc    => GLIST('registers_with_pc', 'register_list')    ;
DEF registers_without_pc => GLIST('registers_without_pc', 'register_list') ;
DEF wback                => WBACK('wback', 'W', default => '0')            ;

T[' LDM   WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'LDM_A1        ', 'cond=NO_NV may_load=1            '];
T[' LDMFD WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'LDM_A1        ', 'cond=NO_NV asmonly=1 may_load=1  '];
T[' LDMIA WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'LDM_A1        ', 'cond=NO_NV asmonly=1 may_load=1  '];
T[' LDMDA WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers_with_pc ', 'P=0b0 U=0b0 ', 'LDM_e_A1_AS   ', 'cond=NO_NV may_load=1            '];
T[' LDMFA WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers_with_pc ', 'P=0b0 U=0b0 ', 'LDM_e_A1_AS   ', 'cond=NO_NV may_load=1            '];
T[' LDMDB WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers_with_pc ', 'P=0b1 U=0b0 ', 'LDM_e_A1_AS   ', 'cond=NO_NV may_load=1            '];
T[' LDMEA WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers_with_pc ', 'P=0b1 U=0b0 ', 'LDM_e_A1_AS   ', 'cond=NO_NV may_load=1            '];
T[' LDMIA WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers_with_pc ', 'P=0b0 U=0b1 ', 'LDM_e_A1_AS   ', 'cond=NO_NV may_load=1            '];
T[' LDMFD WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers_with_pc ', 'P=0b0 U=0b1 ', 'LDM_e_A1_AS   ', 'cond=NO_NV may_load=1            '];
T[' LDMIB WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers_with_pc ', 'P=0b1 U=0b1 ', 'LDM_e_A1_AS   ', 'cond=NO_NV may_load=1            '];
T[' LDMED WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers_with_pc ', 'P=0b1 U=0b1 ', 'LDM_e_A1_AS   ', 'cond=NO_NV may_load=1            '];
T[' LDMDA REG=$Rnupc GLIST=$registers_without_pc                      ', 'P=0b0 U=0b0 ', 'LDM_u_A1_AS   ', 'cond=NO_NV may_load=1            '];
T[' LDMFA REG=$Rnupc GLIST=$registers_without_pc                      ', 'P=0b0 U=0b0 ', 'LDM_u_A1_AS   ', 'cond=NO_NV may_load=1            '];
T[' LDMDB REG=$Rnupc GLIST=$registers_without_pc                      ', 'P=0b1 U=0b0 ', 'LDM_u_A1_AS   ', 'cond=NO_NV may_load=1            '];
T[' LDMEA REG=$Rnupc GLIST=$registers_without_pc                      ', 'P=0b1 U=0b0 ', 'LDM_u_A1_AS   ', 'cond=NO_NV may_load=1            '];
T[' LDMIA REG=$Rnupc GLIST=$registers_without_pc                      ', 'P=0b0 U=0b1 ', 'LDM_u_A1_AS   ', 'cond=NO_NV may_load=1            '];
T[' LDMFD REG=$Rnupc GLIST=$registers_without_pc                      ', 'P=0b0 U=0b1 ', 'LDM_u_A1_AS   ', 'cond=NO_NV may_load=1            '];
T[' LDMIB REG=$Rnupc GLIST=$registers_without_pc                      ', 'P=0b1 U=0b1 ', 'LDM_u_A1_AS   ', 'cond=NO_NV may_load=1            '];
T[' LDMED REG=$Rnupc GLIST=$registers_without_pc                      ', 'P=0b1 U=0b1 ', 'LDM_u_A1_AS   ', 'cond=NO_NV may_load=1            '];
T[' LDMDA WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'LDMDA_A1      ', 'cond=NO_NV may_load=1            '];
T[' LDMFA WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'LDMDA_A1      ', 'cond=NO_NV asmonly=1 may_load=1  '];
T[' LDMDB WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'LDMDB_A1      ', 'cond=NO_NV may_load=1            '];
T[' LDMEA WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'LDMDB_A1      ', 'cond=NO_NV asmonly=1 may_load=1  '];
T[' LDMIB WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'LDMIB_A1      ', 'cond=NO_NV may_load=1            '];
T[' LDMED WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'LDMIB_A1      ', 'cond=NO_NV asmonly=1 may_load=1  '];
T[' STM   WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'STM_A1        ', 'cond=NO_NV may_store=1           '];
T[' STMEA WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'STM_A1        ', 'cond=NO_NV asmonly=1 may_store=1 '];
T[' STMIA WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'STM_A1        ', 'cond=NO_NV asmonly=1 may_store=1 '];
T[' STMDA REG=$Rnupc GLIST=$registers                                 ', 'P=0b0 U=0b0 ', 'STM_u_A1_AS   ', 'cond=NO_NV may_store=1           '];
T[' STMED REG=$Rnupc GLIST=$registers                                 ', 'P=0b0 U=0b0 ', 'STM_u_A1_AS   ', 'cond=NO_NV may_store=1           '];
T[' STMDB REG=$Rnupc GLIST=$registers                                 ', 'P=0b1 U=0b0 ', 'STM_u_A1_AS   ', 'cond=NO_NV may_store=1           '];
T[' STMFD REG=$Rnupc GLIST=$registers                                 ', 'P=0b1 U=0b0 ', 'STM_u_A1_AS   ', 'cond=NO_NV may_store=1           '];
T[' STMIA REG=$Rnupc GLIST=$registers                                 ', 'P=0b0 U=0b1 ', 'STM_u_A1_AS   ', 'cond=NO_NV may_store=1           '];
T[' STMEA REG=$Rnupc GLIST=$registers                                 ', 'P=0b0 U=0b1 ', 'STM_u_A1_AS   ', 'cond=NO_NV may_store=1           '];
T[' STMIB REG=$Rnupc GLIST=$registers                                 ', 'P=0b1 U=0b1 ', 'STM_u_A1_AS   ', 'cond=NO_NV may_store=1           '];
T[' STMFA REG=$Rnupc GLIST=$registers                                 ', 'P=0b1 U=0b1 ', 'STM_u_A1_AS   ', 'cond=NO_NV may_store=1           '];
T[' STMDA WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'STMDA_A1      ', 'cond=NO_NV may_store=1           '];
T[' STMED WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'STMDA_A1      ', 'cond=NO_NV asmonly=1 may_store=1 '];
T[' STMDB WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'STMDB_A1      ', 'cond=NO_NV may_store=1           '];
T[' STMFD WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'STMDB_A1      ', 'cond=NO_NV asmonly=1 may_store=1 '];
T[' STMIB WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'STMIB_A1      ', 'cond=NO_NV may_store=1           '];
T[' STMFA WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers         ', 'NONE        ', 'STMIB_A1      ', 'cond=NO_NV asmonly=1 may_store=1 '];
T[' POP   GLIST=$registers                                            ', 'NONE        ', 'POP_LDM_A1    ', 'cond=NO_NV                       '];
T[' PUSH  GLIST=$registers                                            ', 'NONE        ', 'PUSH_STMDB_A1 ', 'cond=NO_NV                       '];



# ldstm: Load/store multiple.
# T32: ig0=0b1110100 opc:u=0bxx ig1=0b0 W:u=0bx L:u=0bx Rn:u=0bxxxx P:u=0bx M:u=0bx register_list:u=0bxxxxxxxxxxxxxx
HINT('ICLASS', 'ldstm', 'T32');

DEF Rnupc     => GPR32('Rn', 'Rn', 'read|upc')                           ;
DEF mode      => IMM('mode', 'mode')                                     ;
DEF registers => GLIST('registers', 'P:M:register_list', default => '0') ;
DEF wback     => WBACK('wback', 'W', default => '0')                     ;

T[' LDM   WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers ', 'NONE ', 'LDM_T2        ', 'cond=COND showqlf=1 may_load=1            '];
T[' LDMFD WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers ', 'NONE ', 'LDM_T2        ', 'cond=COND asmonly=1 showqlf=1 may_load=1  '];
T[' LDMIA WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers ', 'NONE ', 'LDM_T2        ', 'cond=COND asmonly=1 showqlf=1 may_load=1  '];
T[' LDMDB WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers ', 'NONE ', 'LDMDB_T1      ', 'cond=COND may_load=1                      '];
T[' LDMEA WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers ', 'NONE ', 'LDMDB_T1      ', 'cond=COND asmonly=1 may_load=1            '];
T[' RFEDB WREG={REG=$Rnupc WBACK:OPT=$wback}                  ', 'NONE ', 'RFE_T1_AS     ', 'cond=COND                                 '];
T[' RFEIA WREG={REG=$Rnupc WBACK:OPT=$wback}                  ', 'NONE ', 'RFE_T2_AS     ', 'cond=COND                                 '];
T[' SRSDB WREG={REG=SP WBACK:OPT=$wback} IMM:i32=$mode        ', 'NONE ', 'SRS_T1_AS     ', 'cond=COND may_store=1                     '];
T[' SRSIA WREG={REG=SP WBACK:OPT=$wback} IMM:i32=$mode        ', 'NONE ', 'SRS_T2_AS     ', 'cond=COND may_store=1                     '];
T[' STM   WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers ', 'NONE ', 'STM_T2        ', 'cond=COND showqlf=1 may_store=1           '];
T[' STMEA WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers ', 'NONE ', 'STM_T2        ', 'cond=COND asmonly=1 showqlf=1 may_store=1 '];
T[' STMIA WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers ', 'NONE ', 'STM_T2        ', 'cond=COND asmonly=1 showqlf=1 may_store=1 '];
T[' STMDB WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers ', 'NONE ', 'STMDB_T1      ', 'cond=COND may_store=1                     '];
T[' STMFD WREG={REG=$Rnupc WBACK:OPT=$wback} GLIST=$registers ', 'NONE ', 'STMDB_T1      ', 'cond=COND asmonly=1 may_store=1           '];
T[' POP   GLIST=$registers                                    ', 'NONE ', 'POP_LDM_T2    ', 'cond=COND showqlf=1                       '];
T[' PUSH  GLIST=$registers                                    ', 'NONE ', 'PUSH_STMDB_T1 ', 'cond=COND showqlf=1                       '];



# ldstreg: Load/Store Word, Unsigned Byte (register).
# A32: cond:u!=0b1111 ig0=0b011 P:u=0bx U:u=0bx o2:u=0bx W:u=0bx o1:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx
HINT('ICLASS', 'ldstreg', 'A32');

DEF ADD     => ADD('ADD', 'U')               ;
DEF Rmupc   => GPR32('Rm', 'Rm', 'read|upc') ;
DEF Rn      => GPR32('Rn', 'Rn', 'read')     ;
DEF Rndpc   => GPR32('Rn', 'Rn', 'read|dpc') ;
DEF Rnupc   => GPR32('Rn', 'Rn', 'read|upc') ;
DEF Rt      => GPR32('Rt', 'Rt')             ;
DEF Rtdpc   => GPR32('Rt', 'Rt', 'dpc')      ;
DEF Rtupc   => GPR32('Rt', 'Rt', 'upc')      ;
DEF amount0 => IMM('amount', 'imm5')         ;
DEF shift0  => SHIFT('shift', 'stype')       ;

T[' LDR   REG:w:iwb=$Rt MEM:OFF:r:i32={BASE=$Rn SZ=32 ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:i32=$amount0}}}                      ', 'NONE ', 'LDR_r_A1_off   ', 'cond=NO_NV '];
T[' LDR   REG:w:iwb=$Rt MEM:PST:r:i32={BASE=$Rn SZ=32 ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:i32=$amount0}}}                      ', 'NONE ', 'LDR_r_A1_post  ', 'cond=NO_NV '];
T[' LDR   REG:w:iwb=$Rt MEM:PRE:r:i32={BASE=$Rn SZ=32 ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:i32=$amount0}}}                      ', 'NONE ', 'LDR_r_A1_pre   ', 'cond=NO_NV '];
T[' LDRB  REG:w=$Rtupc MEM:OFF:r:i8={ BASE=$Rn SZ=8  ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:i32=$amount0}}}                       ', 'NONE ', 'LDRB_r_A1_off  ', 'cond=NO_NV '];
T[' LDRB  REG:w=$Rtupc MEM:PST:r:i8={ BASE=$Rn SZ=8  ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:i32=$amount0}}}                       ', 'NONE ', 'LDRB_r_A1_post ', 'cond=NO_NV '];
T[' LDRB  REG:w=$Rtupc MEM:PRE:r:i8={ BASE=$Rn SZ=8  ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:i32=$amount0}}}                       ', 'NONE ', 'LDRB_r_A1_pre  ', 'cond=NO_NV '];
T[' LDRBT REG:w=$Rtupc MEM:PST:r:i8={ BASE=$Rnupc SZ=8  ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:i32=$amount0}} LDACC=UNPRIVILEGED} ', 'NONE ', 'LDRBT_A2       ', 'cond=NO_NV '];
T[' LDRT  REG:w=$Rtupc MEM:PST:r:i32={BASE=$Rnupc SZ=32 ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:i32=$amount0}} LDACC=UNPRIVILEGED} ', 'NONE ', 'LDRT_A2        ', 'cond=NO_NV '];
T[' STR   REG:r=$Rtdpc MEM:OFF:w:i32={BASE=$Rndpc SZ=32 ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:i32=$amount0}}}                    ', 'NONE ', 'STR_r_A1_off   ', 'cond=NO_NV '];
T[' STR   REG:r=$Rtdpc MEM:PST:w:i32={BASE=$Rndpc SZ=32 ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:i32=$amount0}}}                    ', 'NONE ', 'STR_r_A1_post  ', 'cond=NO_NV '];
T[' STR   REG:r=$Rtdpc MEM:PRE:w:i32={BASE=$Rndpc SZ=32 ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:i32=$amount0}}}                    ', 'NONE ', 'STR_r_A1_pre   ', 'cond=NO_NV '];
T[' STRB  REG:r=$Rtupc MEM:OFF:w:i8={ BASE=$Rndpc SZ=8  ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:i32=$amount0}}}                    ', 'NONE ', 'STRB_r_A1_off  ', 'cond=NO_NV '];
T[' STRB  REG:r=$Rtupc MEM:PST:w:i8={ BASE=$Rndpc SZ=8  ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:i32=$amount0}}}                    ', 'NONE ', 'STRB_r_A1_post ', 'cond=NO_NV '];
T[' STRB  REG:r=$Rtupc MEM:PRE:w:i8={ BASE=$Rndpc SZ=8  ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:i32=$amount0}}}                    ', 'NONE ', 'STRB_r_A1_pre  ', 'cond=NO_NV '];
T[' STRBT REG:r=$Rtupc MEM:PST:w:i8={ BASE=$Rnupc SZ=8  ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:i32=$amount0}} STACC=UNPRIVILEGED} ', 'NONE ', 'STRBT_A2       ', 'cond=NO_NV '];
T[' STRT  REG:r=$Rtdpc MEM:PST:w:i32={BASE=$Rnupc SZ=32 ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:i32=$amount0}} STACC=UNPRIVILEGED} ', 'NONE ', 'STRT_A2        ', 'cond=NO_NV '];



# ldstximm: Load/Store Dual, Half, Signed Byte (immediate, literal).
# A32: cond:u!=0b1111 ig0=0b000 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx o1:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2:u!=0b00 ig3=0b1 imm4L:u=0bxxxx
HINT('ICLASS', 'ldstximm', 'A32');

DEF ADD    => ADD('ADD', 'U')                                            ;
DEF Rn     => GPR32('Rn', 'Rn', 'read')                                  ;
DEF Rndpc  => GPR32('Rn', 'Rn', 'read|dpc')                              ;
DEF Rnnpc  => GPR32('Rn', 'Rn', 'read|nopc')                             ;
DEF Rnupc  => GPR32('Rn', 'Rn', 'read|upc')                              ;
DEF Rt     => GPR32('Rt', 'Rt', 'even')                                  ;
DEF Rtupc  => GPR32('Rt2', 'Rt', 'pair|upc')                             ;
DEF Rtupc2 => GPR32('Rt', 'Rt', 'upc')                                   ;
DEF imm    => IMM('imm', 'imm4H:imm4L', range => (0, 255), default => 0) ;

T[' LDRD   REG:w=$Rt REG:w=$Rtupc MEM:OFF:r:i32={BASE=$Rnnpc SZ=2x32 ADD=$ADD IMMOFF:OPT:u8=$imm}         ', 'NONE ', 'LDRD_i_A1_off   ', 'cond=NO_NV '];
T[' LDRD   REG:w=$Rt REG:w=$Rtupc MEM:PST:r:i32={BASE=$Rnnpc SZ=2x32 ADD=$ADD IMMOFF:u8=$imm}             ', 'NONE ', 'LDRD_i_A1_post  ', 'cond=NO_NV '];
T[' LDRD   REG:w=$Rt REG:w=$Rtupc MEM:PRE:r:i32={BASE=$Rnnpc SZ=2x32 ADD=$ADD IMMOFF:u8=$imm}             ', 'NONE ', 'LDRD_i_A1_pre   ', 'cond=NO_NV '];
T[' LDRD   REG:w=$Rt REG:w=$Rtupc LM:OFF:r:i32={BASE=PC SZ=2x32 ADD=$ADD IMMOFF:u8=$imm}                  ', 'NONE ', 'LDRD_l_A1       ', 'cond=NO_NV '];
T[' LDRH   REG:w=$Rtupc2 MEM:OFF:r:i16={BASE=$Rnnpc SZ=16 ADD=$ADD IMMOFF:OPT:u8=$imm}                    ', 'NONE ', 'LDRH_i_A1_off   ', 'cond=NO_NV '];
T[' LDRH   REG:w=$Rtupc2 MEM:PST:r:i16={BASE=$Rnnpc SZ=16 ADD=$ADD IMMOFF:u8=$imm}                        ', 'NONE ', 'LDRH_i_A1_post  ', 'cond=NO_NV '];
T[' LDRH   REG:w=$Rtupc2 MEM:PRE:r:i16={BASE=$Rnnpc SZ=16 ADD=$ADD IMMOFF:u8=$imm}                        ', 'NONE ', 'LDRH_i_A1_pre   ', 'cond=NO_NV '];
T[' LDRH   REG:w=$Rtupc2 LM:OFF:r:i16={BASE=PC SZ=16 ADD=$ADD IMMOFF:u8=$imm}                             ', 'NONE ', 'LDRH_l_A1       ', 'cond=NO_NV '];
T[' LDRHT  REG:w=$Rtupc2 MEM:PST:r:i16={BASE=$Rnupc SZ=16 ADD=$ADD IMMOFF:OPT:u8=$imm LDACC=UNPRIVILEGED} ', 'NONE ', 'LDRHT_A1        ', 'cond=NO_NV '];
T[' LDRSB  REG:w=$Rtupc2 MEM:OFF:r:s8={ BASE=$Rnnpc SZ=8  ADD=$ADD IMMOFF:OPT:u8=$imm}                    ', 'NONE ', 'LDRSB_i_A1_off  ', 'cond=NO_NV '];
T[' LDRSB  REG:w=$Rtupc2 MEM:PST:r:s8={ BASE=$Rnnpc SZ=8  ADD=$ADD IMMOFF:u8=$imm}                        ', 'NONE ', 'LDRSB_i_A1_post ', 'cond=NO_NV '];
T[' LDRSB  REG:w=$Rtupc2 MEM:PRE:r:s8={ BASE=$Rnnpc SZ=8  ADD=$ADD IMMOFF:u8=$imm}                        ', 'NONE ', 'LDRSB_i_A1_pre  ', 'cond=NO_NV '];
T[' LDRSB  REG:w=$Rtupc2 LM:OFF:r:s8={ BASE=PC SZ=8  ADD=$ADD IMMOFF:u8=$imm}                             ', 'NONE ', 'LDRSB_l_A1      ', 'cond=NO_NV '];
T[' LDRSBT REG:w=$Rtupc2 MEM:PST:r:s8={ BASE=$Rnupc SZ=8  ADD=$ADD IMMOFF:OPT:u8=$imm LDACC=UNPRIVILEGED} ', 'NONE ', 'LDRSBT_A1       ', 'cond=NO_NV '];
T[' LDRSH  REG:w=$Rtupc2 MEM:OFF:r:s16={BASE=$Rnnpc SZ=16 ADD=$ADD IMMOFF:OPT:u8=$imm}                    ', 'NONE ', 'LDRSH_i_A1_off  ', 'cond=NO_NV '];
T[' LDRSH  REG:w=$Rtupc2 MEM:PST:r:s16={BASE=$Rnnpc SZ=16 ADD=$ADD IMMOFF:u8=$imm}                        ', 'NONE ', 'LDRSH_i_A1_post ', 'cond=NO_NV '];
T[' LDRSH  REG:w=$Rtupc2 MEM:PRE:r:s16={BASE=$Rnnpc SZ=16 ADD=$ADD IMMOFF:u8=$imm}                        ', 'NONE ', 'LDRSH_i_A1_pre  ', 'cond=NO_NV '];
T[' LDRSH  REG:w=$Rtupc2 LM:OFF:r:s16={BASE=PC SZ=16 ADD=$ADD IMMOFF:u8=$imm}                             ', 'NONE ', 'LDRSH_l_A1      ', 'cond=NO_NV '];
T[' LDRSHT REG:w=$Rtupc2 MEM:PST:r:s16={BASE=$Rnupc SZ=16 ADD=$ADD IMMOFF:OPT:u8=$imm LDACC=UNPRIVILEGED} ', 'NONE ', 'LDRSHT_A1       ', 'cond=NO_NV '];
T[' STRD   REG:r=$Rt REG:r=$Rtupc MEM:OFF:w:i32={BASE=$Rndpc SZ=2x32 ADD=$ADD IMMOFF:OPT:u8=$imm}         ', 'NONE ', 'STRD_i_A1_off   ', 'cond=NO_NV '];
T[' STRD   REG:r=$Rt REG:r=$Rtupc MEM:PST:w:i32={BASE=$Rndpc SZ=2x32 ADD=$ADD IMMOFF:u8=$imm}             ', 'NONE ', 'STRD_i_A1_post  ', 'cond=NO_NV '];
T[' STRD   REG:r=$Rt REG:r=$Rtupc MEM:PRE:w:i32={BASE=$Rndpc SZ=2x32 ADD=$ADD IMMOFF:u8=$imm}             ', 'NONE ', 'STRD_i_A1_pre   ', 'cond=NO_NV '];
T[' STRH   REG:r=$Rtupc2 MEM:OFF:w:i16={BASE=$Rndpc SZ=16 ADD=$ADD IMMOFF:OPT:u8=$imm}                    ', 'NONE ', 'STRH_i_A1_off   ', 'cond=NO_NV '];
T[' STRH   REG:r=$Rtupc2 MEM:PST:w:i16={BASE=$Rn SZ=16 ADD=$ADD IMMOFF:u8=$imm}                           ', 'NONE ', 'STRH_i_A1_post  ', 'cond=NO_NV '];
T[' STRH   REG:r=$Rtupc2 MEM:PRE:w:i16={BASE=$Rn SZ=16 ADD=$ADD IMMOFF:u8=$imm}                           ', 'NONE ', 'STRH_i_A1_pre   ', 'cond=NO_NV '];
T[' STRHT  REG:r=$Rtupc2 MEM:PST:w:i16={BASE=$Rnupc SZ=16 ADD=$ADD IMMOFF:OPT:u8=$imm STACC=UNPRIVILEGED} ', 'NONE ', 'STRHT_A1        ', 'cond=NO_NV '];



# ldstxreg: Load/Store Dual, Half, Signed Byte (register).
# A32: cond:u!=0b1111 ig0=0b000 P:u=0bx U:u=0bx ig1=0b0 W:u=0bx o1:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2:u!=0b00 ig7=0b1 Rm:u=0bxxxx
HINT('ICLASS', 'ldstxreg', 'A32');

DEF ADD    => ADD('ADD', 'U')                ;
DEF Rmupc  => GPR32('Rm', 'Rm', 'read|upc')  ;
DEF Rn     => GPR32('Rn', 'Rn', 'read')      ;
DEF Rndpc  => GPR32('Rn', 'Rn', 'read|dpc')  ;
DEF Rnupc  => GPR32('Rn', 'Rn', 'read|upc')  ;
DEF Rt     => GPR32('Rt', 'Rt', 'even')      ;
DEF Rtupc  => GPR32('Rt2', 'Rt', 'pair|upc') ;
DEF Rtupc2 => GPR32('Rt', 'Rt', 'upc')       ;

T[' LDRD   REG:w=$Rt REG:w=$Rtupc MEM:OFF:r:i32={BASE=$Rn SZ=2x32 ADD=$ADD REGOFF=$Rmupc}            ', 'NONE ', 'LDRD_r_A1_off   ', 'cond=NO_NV '];
T[' LDRD   REG:w=$Rt REG:w=$Rtupc MEM:PST:r:i32={BASE=$Rn SZ=2x32 ADD=$ADD REGOFF=$Rmupc}            ', 'NONE ', 'LDRD_r_A1_post  ', 'cond=NO_NV '];
T[' LDRD   REG:w=$Rt REG:w=$Rtupc MEM:PRE:r:i32={BASE=$Rn SZ=2x32 ADD=$ADD REGOFF=$Rmupc}            ', 'NONE ', 'LDRD_r_A1_pre   ', 'cond=NO_NV '];
T[' LDRH   REG:w=$Rtupc2 MEM:OFF:r:i16={BASE=$Rn SZ=16 ADD=$ADD REGOFF=$Rmupc}                       ', 'NONE ', 'LDRH_r_A1_off   ', 'cond=NO_NV '];
T[' LDRH   REG:w=$Rtupc2 MEM:PST:r:i16={BASE=$Rn SZ=16 ADD=$ADD REGOFF=$Rmupc}                       ', 'NONE ', 'LDRH_r_A1_post  ', 'cond=NO_NV '];
T[' LDRH   REG:w=$Rtupc2 MEM:PRE:r:i16={BASE=$Rn SZ=16 ADD=$ADD REGOFF=$Rmupc}                       ', 'NONE ', 'LDRH_r_A1_pre   ', 'cond=NO_NV '];
T[' LDRHT  REG:w=$Rtupc2 MEM:PST:r:i16={BASE=$Rnupc SZ=16 ADD=$ADD REGOFF=$Rmupc LDACC=UNPRIVILEGED} ', 'NONE ', 'LDRHT_A2        ', 'cond=NO_NV '];
T[' LDRSB  REG:w=$Rtupc2 MEM:OFF:r:s8={ BASE=$Rn SZ=8  ADD=$ADD REGOFF=$Rmupc}                       ', 'NONE ', 'LDRSB_r_A1_off  ', 'cond=NO_NV '];
T[' LDRSB  REG:w=$Rtupc2 MEM:PST:r:s8={ BASE=$Rn SZ=8  ADD=$ADD REGOFF=$Rmupc}                       ', 'NONE ', 'LDRSB_r_A1_post ', 'cond=NO_NV '];
T[' LDRSB  REG:w=$Rtupc2 MEM:PRE:r:s8={ BASE=$Rn SZ=8  ADD=$ADD REGOFF=$Rmupc}                       ', 'NONE ', 'LDRSB_r_A1_pre  ', 'cond=NO_NV '];
T[' LDRSBT REG:w=$Rtupc2 MEM:PST:r:s8={ BASE=$Rnupc SZ=8  ADD=$ADD REGOFF=$Rmupc LDACC=UNPRIVILEGED} ', 'NONE ', 'LDRSBT_A2       ', 'cond=NO_NV '];
T[' LDRSH  REG:w=$Rtupc2 MEM:OFF:r:s16={BASE=$Rn SZ=16 ADD=$ADD REGOFF=$Rmupc}                       ', 'NONE ', 'LDRSH_r_A1_off  ', 'cond=NO_NV '];
T[' LDRSH  REG:w=$Rtupc2 MEM:PST:r:s16={BASE=$Rn SZ=16 ADD=$ADD REGOFF=$Rmupc}                       ', 'NONE ', 'LDRSH_r_A1_post ', 'cond=NO_NV '];
T[' LDRSH  REG:w=$Rtupc2 MEM:PRE:r:s16={BASE=$Rn SZ=16 ADD=$ADD REGOFF=$Rmupc}                       ', 'NONE ', 'LDRSH_r_A1_pre  ', 'cond=NO_NV '];
T[' LDRSHT REG:w=$Rtupc2 MEM:PST:r:s16={BASE=$Rnupc SZ=16 ADD=$ADD REGOFF=$Rmupc LDACC=UNPRIVILEGED} ', 'NONE ', 'LDRSHT_A2       ', 'cond=NO_NV '];
T[' STRD   REG:r=$Rt REG:r=$Rtupc MEM:OFF:w:i32={BASE=$Rndpc SZ=2x32 ADD=$ADD REGOFF=$Rmupc}         ', 'NONE ', 'STRD_r_A1_off   ', 'cond=NO_NV '];
T[' STRD   REG:r=$Rt REG:r=$Rtupc MEM:PST:w:i32={BASE=$Rndpc SZ=2x32 ADD=$ADD REGOFF=$Rmupc}         ', 'NONE ', 'STRD_r_A1_post  ', 'cond=NO_NV '];
T[' STRD   REG:r=$Rt REG:r=$Rtupc MEM:PRE:w:i32={BASE=$Rndpc SZ=2x32 ADD=$ADD REGOFF=$Rmupc}         ', 'NONE ', 'STRD_r_A1_pre   ', 'cond=NO_NV '];
T[' STRH   REG:r=$Rtupc2 MEM:OFF:w:i16={BASE=$Rndpc SZ=16 ADD=$ADD REGOFF=$Rmupc}                    ', 'NONE ', 'STRH_r_A1_off   ', 'cond=NO_NV '];
T[' STRH   REG:r=$Rtupc2 MEM:PST:w:i16={BASE=$Rndpc SZ=16 ADD=$ADD REGOFF=$Rmupc}                    ', 'NONE ', 'STRH_r_A1_post  ', 'cond=NO_NV '];
T[' STRH   REG:r=$Rtupc2 MEM:PRE:w:i16={BASE=$Rndpc SZ=16 ADD=$ADD REGOFF=$Rmupc}                    ', 'NONE ', 'STRH_r_A1_pre   ', 'cond=NO_NV '];
T[' STRHT  REG:r=$Rtupc2 MEM:PST:w:i16={BASE=$Rnupc SZ=16 ADD=$ADD REGOFF=$Rmupc STACC=UNPRIVILEGED} ', 'NONE ', 'STRHT_A2        ', 'cond=NO_NV '];



# lmul_div: Long multiply and divide.
# T32: ig0=0b111110111 op1:u=0bxxx Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx op2:u=0bxxxx Rm:u=0bxxxx
HINT('ICLASS', 'lmul_div', 'T32');

DEF RdHiupc => GPR32('RdHi', 'RdHi', 'write|upc') ;
DEF RdLoupc => GPR32('RdLo', 'RdLo', 'write|upc') ;
DEF Rdupc   => GPR32('Rd', 'Rd', 'write|upc')     ;
DEF Rmupc   => GPR32('Rm', 'Rm', 'read|upc')      ;
DEF Rnupc   => GPR32('Rn', 'Rn', 'read|upc')      ;

T[' SDIV    REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc            ', 'NONE ', 'SDIV_T1    ', 'cond=COND '];
T[' SMLAL   REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLAL_T1   ', 'cond=COND '];
T[' SMLALBB REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLALBB_T1 ', 'cond=COND '];
T[' SMLALBT REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLALBT_T1 ', 'cond=COND '];
T[' SMLALTB REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLALTB_T1 ', 'cond=COND '];
T[' SMLALTT REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLALTT_T1 ', 'cond=COND '];
T[' SMLALD  REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLALD_T1  ', 'cond=COND '];
T[' SMLALDX REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLALDX_T1 ', 'cond=COND '];
T[' SMLSLD  REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLSLD_T1  ', 'cond=COND '];
T[' SMLSLDX REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLSLDX_T1 ', 'cond=COND '];
T[' SMULL   REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMULL_T1   ', 'cond=COND '];
T[' UDIV    REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc            ', 'NONE ', 'UDIV_T1    ', 'cond=COND '];
T[' UMAAL   REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UMAAL_T1   ', 'cond=COND '];
T[' UMLAL   REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UMLAL_T1   ', 'cond=COND '];
T[' UMULL   REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UMULL_T1   ', 'cond=COND '];



# log2reg_imm: Logical Arithmetic  (two register and immediate).
# A32: cond:u!=0b1111 ig0=0b00111 opc:u=0bxx S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx
HINT('ICLASS', 'log2reg_imm', 'A32');

DEF Rddpc => GPR32('Rd', 'Rd', 'write|dpc') ;
DEF Rndpc => GPR32('Rn', 'Rn', 'read|dpc')  ;
DEF aimm  => IMM('aimm', 'imm12')           ;

T[' BIC  REG:OPT:iwb=$Rddpc REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'BIC_i_A1  ', 'cond=NO_NV '];
T[' BICS REG:OPT:erb=$Rddpc REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'BICS_i_A1 ', 'cond=NO_NV '];
T[' MOV  REG:iwb=$Rddpc IMM:i32=$aimm                ', 'NONE ', 'MOV_i_A1  ', 'cond=NO_NV '];
T[' MOVS REG:erb=$Rddpc IMM:i32=$aimm                ', 'NONE ', 'MOVS_i_A1 ', 'cond=NO_NV '];
T[' MVN  REG:iwb=$Rddpc IMM:i32=$aimm                ', 'NONE ', 'MVN_i_A1  ', 'cond=NO_NV '];
T[' MVNS REG:erb=$Rddpc IMM:i32=$aimm                ', 'NONE ', 'MVNS_i_A1 ', 'cond=NO_NV '];
T[' ORR  REG:OPT:iwb=$Rddpc REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'ORR_i_A1  ', 'cond=NO_NV '];
T[' ORRS REG:OPT:erb=$Rddpc REG=$Rndpc IMM:i32=$aimm ', 'NONE ', 'ORRS_i_A1 ', 'cond=NO_NV '];



# logic3reg_immsh: Logical Arithmetic (three register, immediate shift).
# A32: cond:u!=0b1111 ig0=0b00011 opc:u=0bxx S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx
HINT('ICLASS', 'logic3reg_immsh', 'A32');

DEF Rddpc   => GPR32('Rd', 'Rd', 'write|dpc')                 ;
DEF Rm      => GPR32('Rm', 'Rm', 'read')                      ;
DEF Rmdpc   => GPR32('Rm', 'Rm', 'read|dpc')                  ;
DEF Rndpc   => GPR32('Rn', 'Rn', 'read|dpc')                  ;
DEF amount0 => IMM('amount', 'imm5', range => (1, 31))        ;
DEF amount2 => IMM('amount', 'imm5', range => (0, 31))        ;
DEF imm     => IMM('imm', 'imm5', range => (1, 32))           ;
DEF imm2    => IMM('imm', 'imm5', range => (0, 31))           ;
DEF imm3    => IMM('imm', 'imm5', range => (1, 31))           ;
DEF shift0  => SHIFT('shift', 'stype', table => 'tbl_shift1') ;

T[' BIC  REG:OPT:iwb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'BIC_r_A1_RRX       ', 'cond=NO_NV '];
T[' BIC  REG:OPT:iwb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'BIC_r_A1           ', 'cond=NO_NV '];
T[' BICS REG:OPT:erb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'BICS_r_A1_RRX      ', 'cond=NO_NV '];
T[' BICS REG:OPT:erb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'BICS_r_A1          ', 'cond=NO_NV '];
T[' MOV  REG:iwb=$Rddpc REGSH={REG=$Rm SHIFTER={EXTEND=RRX}}                                              ', 'NONE ', 'MOV_r_A1_RRX       ', 'cond=NO_NV '];
T[' MOV  REG:iwb=$Rddpc REGSH={REG=$Rm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }}                   ', 'NONE ', 'MOV_r_A1           ', 'cond=NO_NV '];
T[' MOVS REG:erb=$Rddpc REGSH={REG=$Rm SHIFTER={EXTEND=RRX}}                                              ', 'NONE ', 'MOVS_r_A1_RRX      ', 'cond=NO_NV '];
T[' MOVS REG:erb=$Rddpc REGSH={REG=$Rm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }}                   ', 'NONE ', 'MOVS_r_A1          ', 'cond=NO_NV '];
T[' MVN  REG:iwb=$Rddpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                                           ', 'NONE ', 'MVN_r_A1_RRX       ', 'cond=NO_NV '];
T[' MVN  REG:iwb=$Rddpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}                ', 'NONE ', 'MVN_r_A1           ', 'cond=NO_NV '];
T[' MVNS REG:erb=$Rddpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                                           ', 'NONE ', 'MVNS_r_A1_RRX      ', 'cond=NO_NV '];
T[' MVNS REG:erb=$Rddpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}                ', 'NONE ', 'MVNS_r_A1          ', 'cond=NO_NV '];
T[' ORR  REG:OPT:iwb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'ORR_r_A1_RRX       ', 'cond=NO_NV '];
T[' ORR  REG:OPT:iwb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'ORR_r_A1           ', 'cond=NO_NV '];
T[' ORRS REG:OPT:erb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER={EXTEND=RRX}}                            ', 'NONE ', 'ORRS_r_A1_RRX      ', 'cond=NO_NV '];
T[' ORRS REG:OPT:erb=$Rddpc REG=$Rndpc REGSH={REG=$Rmdpc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'ORRS_r_A1          ', 'cond=NO_NV '];
T[' ASR  REG:OPT:iwb=$Rddpc REG=$Rmdpc IMM:u8=$imm                                                        ', 'NONE ', 'ASR_MOV_r_A1       ', 'cond=NO_NV '];
T[' ASRS REG:OPT=$Rddpc REG=$Rmdpc IMM:u8=$imm                                                            ', 'NONE ', 'ASRS_MOVS_r_A1     ', 'cond=NO_NV '];
T[' LSL  REG:OPT:iwb=$Rddpc REG=$Rmdpc IMM:u8=$imm2                                                       ', 'NONE ', 'LSL_MOV_r_A1       ', 'cond=NO_NV '];
T[' LSLS REG:OPT=$Rddpc REG=$Rmdpc IMM:u8=$imm2                                                           ', 'NONE ', 'LSLS_MOVS_r_A1     ', 'cond=NO_NV '];
T[' LSR  REG:OPT:iwb=$Rddpc REG=$Rmdpc IMM:u8=$imm                                                        ', 'NONE ', 'LSR_MOV_r_A1       ', 'cond=NO_NV '];
T[' LSRS REG:OPT=$Rddpc REG=$Rmdpc IMM:u8=$imm                                                            ', 'NONE ', 'LSRS_MOVS_r_A1     ', 'cond=NO_NV '];
T[' ROR  REG:OPT:iwb=$Rddpc REG=$Rmdpc IMM:u8=$imm3                                                       ', 'NONE ', 'ROR_MOV_r_A1       ', 'cond=NO_NV '];
T[' RORS REG:OPT=$Rddpc REG=$Rmdpc IMM:u8=$imm3                                                           ', 'NONE ', 'RORS_MOVS_r_A1     ', 'cond=NO_NV '];
T[' RRX  REG:OPT:iwb=$Rddpc REG=$Rmdpc                                                                    ', 'NONE ', 'RRX_MOV_r_A1_RRX   ', 'cond=NO_NV '];
T[' RRXS REG:OPT=$Rddpc REG=$Rmdpc                                                                        ', 'NONE ', 'RRXS_MOVS_r_A1_RRX ', 'cond=NO_NV '];



# logic3reg_regsh: Logical Arithmetic (three register, register shift).
# A32: cond:u!=0b1111 ig0=0b00011 opc:u=0bxx S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx
HINT('ICLASS', 'logic3reg_regsh', 'A32');

DEF Rdupc  => GPR32('Rd', 'Rd', 'write|upc')                 ;
DEF Rmupc  => GPR32('Rm', 'Rm', 'read|upc')                  ;
DEF Rnupc  => GPR32('Rn', 'Rn', 'read|upc')                  ;
DEF Rsupc  => GPR32('Rs', 'Rs', 'upc')                       ;
DEF shift0 => SHIFT('shift', 'stype', table => 'tbl_shift1') ;

T[' BICS REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'BICS_rr_A1      ', 'cond=NO_NV '];
T[' BIC  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'BIC_rr_A1       ', 'cond=NO_NV '];
T[' MOVS REG=$Rdupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}}                ', 'NONE ', 'MOVS_rr_A1      ', 'cond=NO_NV '];
T[' MOV  REG=$Rdupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}}                ', 'NONE ', 'MOV_rr_A1       ', 'cond=NO_NV '];
T[' MVNS REG=$Rdupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}}                ', 'NONE ', 'MVNS_rr_A1      ', 'cond=NO_NV '];
T[' MVN  REG=$Rdupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}}                ', 'NONE ', 'MVN_rr_A1       ', 'cond=NO_NV '];
T[' ORRS REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'ORRS_rr_A1      ', 'cond=NO_NV '];
T[' ORR  REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'ORR_rr_A1       ', 'cond=NO_NV '];
T[' ASR  REG:OPT=$Rdupc REG=$Rmupc REG:r=$Rsupc                                           ', 'NONE ', 'ASR_MOV_rr_A1   ', 'cond=NO_NV '];
T[' ASRS REG:OPT=$Rdupc REG=$Rmupc REG:r=$Rsupc                                           ', 'NONE ', 'ASRS_MOVS_rr_A1 ', 'cond=NO_NV '];
T[' LSL  REG:OPT=$Rdupc REG=$Rmupc REG:r=$Rsupc                                           ', 'NONE ', 'LSL_MOV_rr_A1   ', 'cond=NO_NV '];
T[' LSLS REG:OPT=$Rdupc REG=$Rmupc REG:r=$Rsupc                                           ', 'NONE ', 'LSLS_MOVS_rr_A1 ', 'cond=NO_NV '];
T[' LSR  REG:OPT=$Rdupc REG=$Rmupc REG:r=$Rsupc                                           ', 'NONE ', 'LSR_MOV_rr_A1   ', 'cond=NO_NV '];
T[' LSRS REG:OPT=$Rdupc REG=$Rmupc REG:r=$Rsupc                                           ', 'NONE ', 'LSRS_MOVS_rr_A1 ', 'cond=NO_NV '];
T[' ROR  REG:OPT=$Rdupc REG=$Rmupc REG:r=$Rsupc                                           ', 'NONE ', 'ROR_MOV_rr_A1   ', 'cond=NO_NV '];
T[' RORS REG:OPT=$Rdupc REG=$Rmupc REG:r=$Rsupc                                           ', 'NONE ', 'RORS_MOVS_rr_A1 ', 'cond=NO_NV '];



# movcpgp32: System register 32-bit move.
# A32: cond:u=0bxxxx ig0=0b1110 opc1:u=0bxxx L:u=0bx CRn:u=0bxxxx Rt:u=0bxxxx ig1=0b111 cp15:u=0bx opc2:u=0bxxx ig2=0b1 CRm:u=0bxxxx
HINT('ICLASS', 'movcpgp32', 'A32');

DEF CRm    => CSPACE('CRm', 'CRm')                                 ;
DEF CRn    => CSPACE('CRn', 'CRn')                                 ;
DEF Rt     => GPR32('Rt', 'Rt')                                    ;
DEF Rtupc  => GPR32('Rt', 'Rt', 'upc')                             ;
DEF coproc => PSPACE('coproc', 'coproc<0>', table => 'tbl_pspace') ;
DEF opc    => IMM('opc1', 'opc1', range => (0, 7))                 ;
DEF opc2   => IMM('opc2', 'opc2', range => (0, 7))                 ;

T[' MCR PSPACE=$coproc IMM:u8=$opc REG:r=$Rtupc CSPACE=$CRn CSPACE=$CRm IMM:OPT:u8=$opc2 ', 'NONE ', 'MCR_A1 ', 'cond=NO_NV '];
T[' MRC PSPACE=$coproc IMM:u8=$opc REG:w=$Rt CSPACE=$CRn CSPACE=$CRm IMM:OPT:u8=$opc2    ', 'NONE ', 'MRC_A1 ', 'cond=NO_NV '];



# movcpgp64: System register 64-bit move.
# A32: cond:u=0bxxxx ig0=0b11000 D:u=0bx ig1=0b0 L:u=0bx Rt2:u=0bxxxx Rt:u=0bxxxx ig2=0b111 cp15:u=0bx opc1:u=0bxxxx CRm:u=0bxxxx
HINT('ICLASS', 'movcpgp64', 'A32');

DEF CRm    => CSPACE('CRm', 'CRm')                                 ;
DEF Rtupc  => GPR32('Rt', 'Rt', 'upc')                             ;
DEF Rtupc2 => GPR32('Rt2', 'Rt2', 'upc')                           ;
DEF coproc => PSPACE('coproc', 'coproc<0>', table => 'tbl_pspace') ;
DEF opc    => IMM('opc1', 'opc1', range => (0, 15))                ;

T[' MCRR PSPACE=$coproc IMM:u8=$opc REG:r=$Rtupc REG:r=$Rtupc2 CSPACE=$CRm ', 'NONE ', 'MCRR_A1 ', 'cond=NO_NV '];
T[' MRRC PSPACE=$coproc IMM:u8=$opc REG:w=$Rtupc REG:w=$Rtupc2 CSPACE=$CRm ', 'NONE ', 'MRRC_A1 ', 'cond=NO_NV '];



# movsr_hint_imm: Move Special Register and Hints (immediate).
# A32: cond:u!=0b1111 ig0=0b00110 R:u=0bx ig1=0b10 imm4:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12:u=0bxxxxxxxxxxxx
HINT('ICLASS', 'movsr_hint_imm', 'A32');

DEF aimm     => IMM('aimm', undef)                        ;
DEF option   => IMM('option', 'option', range => (0, 15)) ;
DEF spec_reg => SPECREG('spec_reg', 'mask')               ;

T[' CSDB                                  ', 'NONE ', 'CSDB_A1     ', 'cond=NO_NV                                     '];
T[' DBG   IMM:u8=$option                  ', 'NONE ', 'DBG_A1      ', 'cond=NO_NV                                     '];
T[' ESB                                   ', 'NONE ', 'ESB_A1      ', 'arch_variant=ARMv8v2 extensions=RAS cond=NO_NV '];
T[' MSR   SPECREG=$spec_reg IMM:i32=$aimm ', 'NONE ', 'MSR_i_A1_AS ', 'cond=NO_NV                                     '];
T[' NOP                                   ', 'NONE ', 'NOP_A1      ', 'cond=NO_NV                                     '];
T[' SEV                                   ', 'NONE ', 'SEV_A1      ', 'cond=NO_NV                                     '];
T[' SEVL                                  ', 'NONE ', 'SEVL_A1     ', 'cond=NO_NV                                     '];
T[' TSB   SYNC_OP=CSYNC                   ', 'NONE ', 'TSB_A1      ', 'arch_variant=ARMv8v4 cond=NO_NV                '];
T[' WFE                                   ', 'NONE ', 'WFE_A1      ', 'cond=NO_NV                                     '];
T[' WFI                                   ', 'NONE ', 'WFI_A1      ', 'cond=NO_NV                                     '];
T[' YIELD                                 ', 'NONE ', 'YIELD_A1    ', 'cond=NO_NV                                     '];



# movsr_reg: Move special register (register).
# A32: cond:u!=0b1111 ig0=0b00010 opc:u=0bxx ig1=0b0 mask:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ B:u=0bx m:u=0bx ig4=0b0000 Rn:u=0bxxxx
HINT('ICLASS', 'movsr_reg', 'A32');

DEF Rdupc      => GPR32('Rd', 'Rd', 'write|upc')                          ;
DEF Rnupc      => GPR32('Rn', 'Rn', 'read|upc')                           ;
DEF banked_reg => BNKDREG('banked_reg', 'R:M:M1', table => 'tbl_bnkdreg') ;
DEF spec_reg   => SPECREG('spec_reg', 'R', table => 'tbl_specreg')        ;
DEF spec_reg2  => SPECREG('spec_reg', 'mask')                             ;

T[' MRS REG=$Rdupc SPECREG=$spec_reg   ', 'NONE ', 'MRS_A1_AS    ', 'cond=NO_NV '];
T[' MRS REG=$Rdupc BNKDREG=$banked_reg ', 'NONE ', 'MRS_br_A1_AS ', 'cond=NO_NV '];
T[' MSR BNKDREG=$banked_reg REG=$Rnupc ', 'NONE ', 'MSR_br_A1_AS ', 'cond=NO_NV '];
T[' MSR SPECREG=$spec_reg2 REG=$Rnupc  ', 'NONE ', 'MSR_r_A1_AS  ', 'cond=NO_NV '];



# movw: Move Halfword (immediate).
# A32: cond:u!=0b1111 ig0=0b00110 H:u=0bx ig1=0b00 imm4:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx
HINT('ICLASS', 'movw', 'A32');

DEF Rdupc => GPR32('Rd', 'Rd', 'write|upc')                  ;
DEF imm   => IMM('imm16', 'imm4:imm12', range => (0, 65535)) ;

T[' MOV  REG=$Rdupc IMM:u16=$imm ', 'NONE ', 'MOV_i_A2 ', 'cond=NO_NV asmonly=1 showqlf=1 '];
T[' MOVW REG=$Rdupc IMM:u16=$imm ', 'NONE ', 'MOV_i_A2 ', 'cond=NO_NV                     '];
T[' MOVT REG=$Rdupc IMM:u16=$imm ', 'NONE ', 'MOVT_A1  ', 'cond=NO_NV                     '];



# movw: Move Wide (16-bit immediate).
# T32: ig0=0b11110 i:u=0bx ig1=0b10 o1:u=0bx ig2=0b100 imm4:u=0bxxxx ig3=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx
HINT('ICLASS', 'movw', 'T32');

DEF Rdupc => GPR32('Rd', 'Rd', 'write|upc')                        ;
DEF imm   => IMM('imm16', 'imm4:i:imm3:imm8', range => (0, 65535)) ;

T[' MOV  REG=$Rdupc IMM:u16=$imm ', 'NONE ', 'MOV_i_T3 ', 'cond=COND asmonly=1 showqlf=1 '];
T[' MOVW REG=$Rdupc IMM:u16=$imm ', 'NONE ', 'MOV_i_T3 ', 'cond=COND                     '];
T[' MOVT REG=$Rdupc IMM:u16=$imm ', 'NONE ', 'MOVT_T1  ', 'cond=COND                     '];



# mrs_bank: MRS (banked).
# T32: ig0=0b11110011111 R:u=0bx m1:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 Rd:u=0bxxxx ig4=0bZ ig5=0bZ ig6=0b1 m:u=0bx ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ
HINT('ICLASS', 'mrs_bank', 'T32');

DEF Rdupc      => GPR32('Rd', 'Rd', 'write|upc')                          ;
DEF banked_reg => BNKDREG('banked_reg', 'R:M:M1', table => 'tbl_bnkdreg') ;

T[' MRS REG=$Rdupc BNKDREG=$banked_reg ', 'NONE ', 'MRS_br_T1_AS ', 'cond=COND '];



# mrs_spec: MRS (special).
# T32: ig0=0b11110011111 R:u=0bx ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 Rd:u=0bxxxx ig8=0bZ ig9=0bZ ig10=0b0 ig11=0bZ ig12=0bZ ig13=0bZ ig14=0bZ ig15=0bZ
HINT('ICLASS', 'mrs_spec', 'T32');

DEF Rdupc    => GPR32('Rd', 'Rd', 'write|upc')                   ;
DEF spec_reg => SPECREG('spec_reg', 'R', table => 'tbl_specreg') ;

T[' MRS REG=$Rdupc SPECREG=$spec_reg ', 'NONE ', 'MRS_T1_AS ', 'cond=COND '];



# msr_bank: MSR (banked).
# T32: ig0=0b11110011100 R:u=0bx Rn:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 m1:u=0bxxxx ig4=0bZ ig5=0bZ ig6=0b1 m:u=0bx ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ
HINT('ICLASS', 'msr_bank', 'T32');

DEF Rnupc      => GPR32('Rn', 'Rn', 'read|upc')                           ;
DEF banked_reg => BNKDREG('banked_reg', 'R:M:M1', table => 'tbl_bnkdreg') ;

T[' MSR BNKDREG=$banked_reg REG=$Rnupc ', 'NONE ', 'MSR_br_T1_AS ', 'cond=COND '];



# msr_spec: MSR (special).
# T32: ig0=0b11110011100 R:u=0bx Rn:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 mask:u=0bxxxx ig4=0bZ ig5=0bZ ig6=0b0 ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ ig11=0bZ
HINT('ICLASS', 'msr_spec', 'T32');

DEF Rnupc    => GPR32('Rn', 'Rn', 'read|upc') ;
DEF spec_reg => SPECREG('spec_reg', 'mask')   ;

T[' MSR SPECREG=$spec_reg REG=$Rnupc ', 'NONE ', 'MSR_r_T1_AS ', 'cond=COND '];



# mul_abd: Multiply and absolute difference.
# T32: ig0=0b111110110 op1:u=0bxxx Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 op2:u=0bxx Rm:u=0bxxxx
HINT('ICLASS', 'mul_abd', 'T32');

DEF Ranpc => GPR32('Ra', 'Ra', 'read|nopc') ;
DEF Raupc => GPR32('Ra', 'Ra', 'read|upc')  ;
DEF Rdupc => GPR32('Rd', 'Rd', 'write|upc') ;
DEF Rmupc => GPR32('Rm', 'Rm', 'read|upc')  ;
DEF Rnupc => GPR32('Rn', 'Rn', 'read|upc')  ;

T[' MLA    REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc ', 'NONE ', 'MLA_T1    ', 'cond=COND           '];
T[' MLS    REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Raupc ', 'NONE ', 'MLS_T1    ', 'cond=COND           '];
T[' MUL    REG=$Rdupc REG=$Rnupc REG:OPT=$Rmupc        ', 'NONE ', 'MUL_T2    ', 'cond=COND showqlf=1 '];
T[' SMLABB REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc ', 'NONE ', 'SMLABB_T1 ', 'cond=COND           '];
T[' SMLABT REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc ', 'NONE ', 'SMLABT_T1 ', 'cond=COND           '];
T[' SMLATB REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc ', 'NONE ', 'SMLATB_T1 ', 'cond=COND           '];
T[' SMLATT REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc ', 'NONE ', 'SMLATT_T1 ', 'cond=COND           '];
T[' SMLAD  REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc ', 'NONE ', 'SMLAD_T1  ', 'cond=COND           '];
T[' SMLADX REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc ', 'NONE ', 'SMLADX_T1 ', 'cond=COND           '];
T[' SMLAWB REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc ', 'NONE ', 'SMLAWB_T1 ', 'cond=COND           '];
T[' SMLAWT REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc ', 'NONE ', 'SMLAWT_T1 ', 'cond=COND           '];
T[' SMLSD  REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc ', 'NONE ', 'SMLSD_T1  ', 'cond=COND           '];
T[' SMLSDX REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc ', 'NONE ', 'SMLSDX_T1 ', 'cond=COND           '];
T[' SMMLA  REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc ', 'NONE ', 'SMMLA_T1  ', 'cond=COND           '];
T[' SMMLAR REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc ', 'NONE ', 'SMMLAR_T1 ', 'cond=COND           '];
T[' SMMLS  REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Raupc ', 'NONE ', 'SMMLS_T1  ', 'cond=COND           '];
T[' SMMLSR REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Raupc ', 'NONE ', 'SMMLSR_T1 ', 'cond=COND           '];
T[' SMMUL  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc        ', 'NONE ', 'SMMUL_T1  ', 'cond=COND           '];
T[' SMMULR REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc        ', 'NONE ', 'SMMULR_T1 ', 'cond=COND           '];
T[' SMUAD  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc        ', 'NONE ', 'SMUAD_T1  ', 'cond=COND           '];
T[' SMUADX REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc        ', 'NONE ', 'SMUADX_T1 ', 'cond=COND           '];
T[' SMULBB REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc        ', 'NONE ', 'SMULBB_T1 ', 'cond=COND           '];
T[' SMULBT REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc        ', 'NONE ', 'SMULBT_T1 ', 'cond=COND           '];
T[' SMULTB REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc        ', 'NONE ', 'SMULTB_T1 ', 'cond=COND           '];
T[' SMULTT REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc        ', 'NONE ', 'SMULTT_T1 ', 'cond=COND           '];
T[' SMULWB REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc        ', 'NONE ', 'SMULWB_T1 ', 'cond=COND           '];
T[' SMULWT REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc        ', 'NONE ', 'SMULWT_T1 ', 'cond=COND           '];
T[' SMUSD  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc        ', 'NONE ', 'SMUSD_T1  ', 'cond=COND           '];
T[' SMUSDX REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc        ', 'NONE ', 'SMUSDX_T1 ', 'cond=COND           '];
T[' USAD8  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc        ', 'NONE ', 'USAD8_T1  ', 'cond=COND           '];
T[' USADA8 REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc ', 'NONE ', 'USADA8_T1 ', 'cond=COND           '];



# mul_half: Halfword Multiply and Accumulate.
# A32: cond:u!=0b1111 ig0=0b00010 opc:u=0bxx ig1=0b0 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig2=0b1 M:u=0bx N:u=0bx ig3=0b0 Rn:u=0bxxxx
HINT('ICLASS', 'mul_half', 'A32');

DEF Raupc   => GPR32('Ra', 'Ra', 'read|upc')      ;
DEF RdHiupc => GPR32('RdHi', 'RdHi', 'write|upc') ;
DEF RdLoupc => GPR32('RdLo', 'RdLo', 'write|upc') ;
DEF Rdupc   => GPR32('Rd', 'Rd', 'write|upc')     ;
DEF Rmupc   => GPR32('Rm', 'Rm', 'read|upc')      ;
DEF Rnupc   => GPR32('Rn', 'Rn', 'read|upc')      ;

T[' SMLABB  REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Raupc     ', 'NONE ', 'SMLABB_A1  ', 'cond=NO_NV '];
T[' SMLABT  REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Raupc     ', 'NONE ', 'SMLABT_A1  ', 'cond=NO_NV '];
T[' SMLATB  REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Raupc     ', 'NONE ', 'SMLATB_A1  ', 'cond=NO_NV '];
T[' SMLATT  REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Raupc     ', 'NONE ', 'SMLATT_A1  ', 'cond=NO_NV '];
T[' SMLALBB REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLALBB_A1 ', 'cond=NO_NV '];
T[' SMLALBT REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLALBT_A1 ', 'cond=NO_NV '];
T[' SMLALTB REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLALTB_A1 ', 'cond=NO_NV '];
T[' SMLALTT REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLALTT_A1 ', 'cond=NO_NV '];
T[' SMLAWB  REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Raupc     ', 'NONE ', 'SMLAWB_A1  ', 'cond=NO_NV '];
T[' SMLAWT  REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Raupc     ', 'NONE ', 'SMLAWT_A1  ', 'cond=NO_NV '];
T[' SMULBB  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc            ', 'NONE ', 'SMULBB_A1  ', 'cond=NO_NV '];
T[' SMULBT  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc            ', 'NONE ', 'SMULBT_A1  ', 'cond=NO_NV '];
T[' SMULTB  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc            ', 'NONE ', 'SMULTB_A1  ', 'cond=NO_NV '];
T[' SMULTT  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc            ', 'NONE ', 'SMULTT_A1  ', 'cond=NO_NV '];
T[' SMULWB  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc            ', 'NONE ', 'SMULWB_A1  ', 'cond=NO_NV '];
T[' SMULWT  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc            ', 'NONE ', 'SMULWT_A1  ', 'cond=NO_NV '];



# mul_word: Multiply and Accumulate.
# A32: cond:u!=0b1111 ig0=0b0000 opc:u=0bxxx S:u=0bx RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx
HINT('ICLASS', 'mul_word', 'A32');

DEF Raupc   => GPR32('Ra', 'Ra', 'read|upc')      ;
DEF RdHiupc => GPR32('RdHi', 'RdHi', 'write|upc') ;
DEF RdLoupc => GPR32('RdLo', 'RdLo', 'write|upc') ;
DEF Rdupc   => GPR32('Rd', 'Rd', 'write|upc')     ;
DEF Rmupc   => GPR32('Rm', 'Rm', 'read|upc')      ;
DEF Rnupc   => GPR32('Rn', 'Rn', 'read|upc')      ;

T[' MLAS   REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Raupc     ', 'NONE ', 'MLAS_A1   ', 'cond=NO_NV '];
T[' MLA    REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Raupc     ', 'NONE ', 'MLA_A1    ', 'cond=NO_NV '];
T[' MLS    REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Raupc     ', 'NONE ', 'MLS_A1    ', 'cond=NO_NV '];
T[' MULS   REG=$Rdupc REG=$Rnupc REG:OPT=$Rmupc            ', 'NONE ', 'MULS_A1   ', 'cond=NO_NV '];
T[' MUL    REG=$Rdupc REG=$Rnupc REG:OPT=$Rmupc            ', 'NONE ', 'MUL_A1    ', 'cond=NO_NV '];
T[' SMLALS REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLALS_A1 ', 'cond=NO_NV '];
T[' SMLAL  REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLAL_A1  ', 'cond=NO_NV '];
T[' SMULLS REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMULLS_A1 ', 'cond=NO_NV '];
T[' SMULL  REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMULL_A1  ', 'cond=NO_NV '];
T[' UMAAL  REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UMAAL_A1  ', 'cond=NO_NV '];
T[' UMLALS REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UMLALS_A1 ', 'cond=NO_NV '];
T[' UMLAL  REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UMLAL_A1  ', 'cond=NO_NV '];
T[' UMULLS REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UMULLS_A1 ', 'cond=NO_NV '];
T[' UMULL  REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UMULL_A1  ', 'cond=NO_NV '];



# pack: Pack Halfword.
# A32: cond:u!=0b1111 ig0=0b01101000 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx tb:u=0bx ig1=0b01 Rm:u=0bxxxx
HINT('ICLASS', 'pack', 'A32');

DEF Rdupc   => GPR32('Rd', 'Rd', 'write|upc') ;
DEF Rmupc   => GPR32('Rm', 'Rm', 'read|upc')  ;
DEF Rnupc   => GPR32('Rn', 'Rn', 'read|upc')  ;
DEF amount0 => IMM('amount', 'imm5')          ;

T[' PKHBT REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amount0}} ', 'NONE ', 'PKHBT_A1 ', 'cond=NO_NV '];
T[' PKHTB REG:OPT=$Rdupc REG=$Rnupc REGSH={REG=$Rmupc SHIFTER:OPT={SHIFT=ASR AMOUNT:i32=$amount0}} ', 'NONE ', 'PKHTB_A1 ', 'cond=NO_NV '];



# parallel: Parallel Arithmetic.
# A32: cond:u!=0b1111 ig0=0b01100 op1:u=0bxxx Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B:u=0bx op2:u=0bxx ig5=0b1 Rm:u=0bxxxx
HINT('ICLASS', 'parallel', 'A32');

DEF Rdupc => GPR32('Rd', 'Rd', 'write|upc') ;
DEF Rmupc => GPR32('Rm', 'Rm', 'read|upc')  ;
DEF Rnupc => GPR32('Rn', 'Rn', 'read|upc')  ;

T[' QADD16  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'QADD16_A1  ', 'cond=NO_NV '];
T[' QADD8   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'QADD8_A1   ', 'cond=NO_NV '];
T[' QASX    REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'QASX_A1    ', 'cond=NO_NV '];
T[' QSAX    REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'QSAX_A1    ', 'cond=NO_NV '];
T[' QSUB16  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'QSUB16_A1  ', 'cond=NO_NV '];
T[' QSUB8   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'QSUB8_A1   ', 'cond=NO_NV '];
T[' SADD16  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SADD16_A1  ', 'cond=NO_NV '];
T[' SADD8   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SADD8_A1   ', 'cond=NO_NV '];
T[' SASX    REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SASX_A1    ', 'cond=NO_NV '];
T[' SHADD16 REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SHADD16_A1 ', 'cond=NO_NV '];
T[' SHADD8  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SHADD8_A1  ', 'cond=NO_NV '];
T[' SHASX   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SHASX_A1   ', 'cond=NO_NV '];
T[' SHSAX   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SHSAX_A1   ', 'cond=NO_NV '];
T[' SHSUB16 REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SHSUB16_A1 ', 'cond=NO_NV '];
T[' SHSUB8  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SHSUB8_A1  ', 'cond=NO_NV '];
T[' SSAX    REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SSAX_A1    ', 'cond=NO_NV '];
T[' SSUB16  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SSUB16_A1  ', 'cond=NO_NV '];
T[' SSUB8   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SSUB8_A1   ', 'cond=NO_NV '];
T[' UADD16  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UADD16_A1  ', 'cond=NO_NV '];
T[' UADD8   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UADD8_A1   ', 'cond=NO_NV '];
T[' UASX    REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UASX_A1    ', 'cond=NO_NV '];
T[' UHADD16 REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UHADD16_A1 ', 'cond=NO_NV '];
T[' UHADD8  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UHADD8_A1  ', 'cond=NO_NV '];
T[' UHASX   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UHASX_A1   ', 'cond=NO_NV '];
T[' UHSAX   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UHSAX_A1   ', 'cond=NO_NV '];
T[' UHSUB16 REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UHSUB16_A1 ', 'cond=NO_NV '];
T[' UHSUB8  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UHSUB8_A1  ', 'cond=NO_NV '];
T[' UQADD16 REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UQADD16_A1 ', 'cond=NO_NV '];
T[' UQADD8  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UQADD8_A1  ', 'cond=NO_NV '];
T[' UQASX   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UQASX_A1   ', 'cond=NO_NV '];
T[' UQSAX   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UQSAX_A1   ', 'cond=NO_NV '];
T[' UQSUB16 REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UQSUB16_A1 ', 'cond=NO_NV '];
T[' UQSUB8  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'UQSUB8_A1  ', 'cond=NO_NV '];
T[' USAX    REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'USAX_A1    ', 'cond=NO_NV '];
T[' USUB16  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'USUB16_A1  ', 'cond=NO_NV '];
T[' USUB8   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'USUB8_A1   ', 'cond=NO_NV '];



# preload_imm: Preload (immediate).
# A32: ig0=0b1111010 D:u=0bx U:u=0bx R:u=0bx ig1=0b01 Rn:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12:u=0bxxxxxxxxxxxx
HINT('ICLASS', 'preload_imm', 'A32');

DEF ADD   => ADD('ADD', 'U')                                       ;
DEF Rn    => GPR32('Rn', 'Rn', 'read')                             ;
DEF Rnnpc => GPR32('Rn', 'Rn', 'read|nopc')                        ;
DEF imm   => IMM('imm', 'imm12', range => (0, 4095), default => 0) ;
DEF imm2  => IMM('imm', 'imm12', range => (0, 4095))               ;

T[' PLD  MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=$ADD IMMOFF:OPT:u16=$imm} ', 'NONE ', 'PLD_i_A1  ', 'cond=UNCOND '];
T[' PLDW MEM:OFF:r:i32={BASE=$Rnnpc SZ=32 ADD=$ADD IMMOFF:OPT:u16=$imm} ', 'NONE ', 'PLDW_i_A1 ', 'cond=UNCOND '];
T[' PLD  LM:OFF:r:i32={BASE=PC SZ=32 ADD=$ADD IMMOFF:u16=$imm2}         ', 'NONE ', 'PLD_l_A1  ', 'cond=UNCOND '];
T[' PLI  LM:OFF:r:i32={BASE=$Rn SZ=32 ADD=$ADD IMMOFF:OPT:u16=$imm}     ', 'NONE ', 'PLI_i_A1  ', 'cond=UNCOND '];



# preload_reg: Preload (register).
# A32: ig0=0b1111011 D:u=0bx U:u=0bx o2:u=0bx ig1=0b01 Rn:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm5:u=0bxxxxx stype:u=0bxx ig6=0b0 Rm:u=0bxxxx
HINT('ICLASS', 'preload_reg', 'A32');

DEF ADD     => ADD('ADD', 'U')                                ;
DEF Rmupc   => GPR32('Rm', 'Rm', 'read|upc')                  ;
DEF Rn      => GPR32('Rn', 'Rn', 'read')                      ;
DEF amount0 => IMM('amount', 'imm5', range => (1, 31))        ;
DEF shift0  => SHIFT('shift', 'stype', table => 'tbl_shift1') ;

T[' PLD  MEM:OFF:r:i32={BASE=$Rn SZ=32 ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}} ', 'NONE ', 'PLD_r_A1      ', 'cond=UNCOND '];
T[' PLD  MEM:OFF:r:i32={BASE=$Rn SZ=32 ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}}                            ', 'NONE ', 'PLD_r_A1_RRX  ', 'cond=UNCOND '];
T[' PLDW MEM:OFF:r:i32={BASE=$Rn SZ=32 ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}} ', 'NONE ', 'PLDW_r_A1     ', 'cond=UNCOND '];
T[' PLDW MEM:OFF:r:i32={BASE=$Rn SZ=32 ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}}                            ', 'NONE ', 'PLDW_r_A1_RRX ', 'cond=UNCOND '];
T[' PLI  MEM:OFF:r:i32={BASE=$Rn SZ=32 ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER={EXTEND=RRX}}}                            ', 'NONE ', 'PLI_r_A1_RRX  ', 'cond=UNCOND '];
T[' PLI  MEM:OFF:r:i32={BASE=$Rn SZ=32 ADD=$ADD REGOFFSH={REG=$Rmupc SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}} ', 'NONE ', 'PLI_r_A1      ', 'cond=UNCOND '];



# reverse: Reverse Bit/Byte.
# A32: cond:u!=0b1111 ig0=0b01101 o1:u=0bx ig1=0b11 ig2=0bS ig3=0bS ig4=0bS ig5=0bS Rd:u=0bxxxx ig6=0bS ig7=0bS ig8=0bS ig9=0bS o2:u=0bx ig10=0b011 Rm:u=0bxxxx
HINT('ICLASS', 'reverse', 'A32');

DEF Rdupc => GPR32('Rd', 'Rd', 'write|upc') ;
DEF Rmupc => GPR32('Rm', 'Rm', 'read|upc')  ;

T[' RBIT  REG=$Rdupc REG=$Rmupc ', 'NONE ', 'RBIT_A1  ', 'cond=NO_NV '];
T[' REV   REG=$Rdupc REG=$Rmupc ', 'NONE ', 'REV_A1   ', 'cond=NO_NV '];
T[' REV16 REG=$Rdupc REG=$Rmupc ', 'NONE ', 'REV16_A1 ', 'cond=NO_NV '];
T[' REVSH REG=$Rdupc REG=$Rmupc ', 'NONE ', 'REVSH_A1 ', 'cond=NO_NV '];



# sat16: Saturate 16-bit.
# A32: cond:u!=0b1111 ig0=0b01101 U:u=0bx ig1=0b10 sat_imm:u=0bxxxx Rd:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0b0011 Rn:u=0bxxxx
HINT('ICLASS', 'sat16', 'A32');

DEF Rdupc => GPR32('Rd', 'Rd', 'write|upc')          ;
DEF Rnupc => GPR32('Rn', 'Rn', 'read|upc')           ;
DEF imm   => IMM('imm', 'sat_imm', range => (1, 16)) ;
DEF imm2  => IMM('imm', 'sat_imm', range => (0, 15)) ;

T[' SSAT16 REG=$Rdupc IMM:u8=$imm REG=$Rnupc  ', 'NONE ', 'SSAT16_A1 ', 'cond=NO_NV '];
T[' USAT16 REG=$Rdupc IMM:u8=$imm2 REG=$Rnupc ', 'NONE ', 'USAT16_A1 ', 'cond=NO_NV '];



# sat32: Saturate 32-bit.
# A32: cond:u!=0b1111 ig0=0b01101 U:u=0bx ig1=0b1 sat_imm:u=0bxxxxx Rd:u=0bxxxx imm5:u=0bxxxxx sh:u=0bx ig2=0b01 Rn:u=0bxxxx
HINT('ICLASS', 'sat32', 'A32');

DEF Rdupc   => GPR32('Rd', 'Rd', 'write|upc')                        ;
DEF Rnupc   => GPR32('Rn', 'Rn', 'read|upc')                         ;
DEF amount0 => IMM('amount', 'imm5', range => (1, 32))               ;
DEF amount2 => IMM('amount', 'imm5', range => (0, 31), default => 0) ;
DEF imm     => IMM('imm', 'sat_imm', range => (1, 32))               ;
DEF imm2    => IMM('imm', 'sat_imm', range => (0, 31))               ;

T[' SSAT REG=$Rdupc IMM:u8=$imm REGSH={REG=$Rnupc SHIFTER={SHIFT=ASR AMOUNT:u8=$amount0 }}      ', 'NONE ', 'SSAT_A1_ASR ', 'cond=NO_NV '];
T[' SSAT REG=$Rdupc IMM:u8=$imm REGSH={REG=$Rnupc SHIFTER:OPT={SHIFT=LSL AMOUNT:u8=$amount2 }}  ', 'NONE ', 'SSAT_A1_LSL ', 'cond=NO_NV '];
T[' USAT REG=$Rdupc IMM:u8=$imm2 REGSH={REG=$Rnupc SHIFTER={SHIFT=ASR AMOUNT:u8=$amount0 }}     ', 'NONE ', 'USAT_A1_ASR ', 'cond=NO_NV '];
T[' USAT REG=$Rdupc IMM:u8=$imm2 REGSH={REG=$Rnupc SHIFTER:OPT={SHIFT=LSL AMOUNT:u8=$amount2 }} ', 'NONE ', 'USAT_A1_LSL ', 'cond=NO_NV '];



# sat_bit: Saturate, Bitfield.
# T32: ig0=0b11110 ig1=0bZ ig2=0b11 op1:u=0bxxx ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx ig5=0bZ widthm1:u=0bxxxxx
HINT('ICLASS', 'sat_bit', 'T32');

DEF Rdupc   => GPR32('Rd', 'Rd', 'write|upc')                             ;
DEF Rnnpc   => GPR32('Rn', 'Rn', 'read|nopc')                             ;
DEF Rnupc   => GPR32('Rn', 'Rn', 'read|upc')                              ;
DEF amount0 => IMM('amount', 'imm3:imm2', range => (1, 31))               ;
DEF amount2 => IMM('amount', 'imm3:imm2', range => (0, 31), default => 0) ;
DEF imm     => IMM('imm', 'sat_imm', range => (1, 32))                    ;
DEF imm2    => IMM('imm', 'sat_imm', range => (1, 16))                    ;
DEF imm3    => IMM('imm', 'sat_imm', range => (0, 31))                    ;
DEF imm4    => IMM('imm', 'sat_imm', range => (0, 15))                    ;
DEF lsb     => IMM('lsb', 'imm3:imm2', range => (0, 31))                  ;
DEF width   => IMM('width', 'msb', range => (1, 32))                      ;
DEF width2  => IMM('width', 'widthm1', range => (1, 32))                  ;

T[' BFC    REG=$Rdupc IMM:u8=$lsb IMM:u8=$width                                                   ', 'NONE ', 'BFC_T1      ', 'cond=COND '];
T[' BFI    REG=$Rdupc REG=$Rnnpc IMM:u8=$lsb IMM:u8=$width                                        ', 'NONE ', 'BFI_T1      ', 'cond=COND '];
T[' SBFX   REG=$Rdupc REG=$Rnupc IMM:u8=$lsb IMM:u8=$width2                                       ', 'NONE ', 'SBFX_T1     ', 'cond=COND '];
T[' SSAT   REG=$Rdupc IMM:u8=$imm REGSH={REG=$Rnupc SHIFTER={SHIFT=ASR AMOUNT:u8=$amount0 }}      ', 'NONE ', 'SSAT_T1_ASR ', 'cond=COND '];
T[' SSAT   REG=$Rdupc IMM:u8=$imm REGSH={REG=$Rnupc SHIFTER:OPT={SHIFT=LSL AMOUNT:u8=$amount2 }}  ', 'NONE ', 'SSAT_T1_LSL ', 'cond=COND '];
T[' SSAT16 REG=$Rdupc IMM:u8=$imm2 REG=$Rnupc                                                     ', 'NONE ', 'SSAT16_T1   ', 'cond=COND '];
T[' UBFX   REG=$Rdupc REG=$Rnupc IMM:u8=$lsb IMM:u8=$width2                                       ', 'NONE ', 'UBFX_T1     ', 'cond=COND '];
T[' USAT   REG=$Rdupc IMM:u8=$imm3 REGSH={REG=$Rnupc SHIFTER={SHIFT=ASR AMOUNT:u8=$amount0 }}     ', 'NONE ', 'USAT_T1_ASR ', 'cond=COND '];
T[' USAT   REG=$Rdupc IMM:u8=$imm3 REGSH={REG=$Rnupc SHIFTER:OPT={SHIFT=LSL AMOUNT:u8=$amount2 }} ', 'NONE ', 'USAT_T1_LSL ', 'cond=COND '];
T[' USAT16 REG=$Rdupc IMM:u8=$imm4 REG=$Rnupc                                                     ', 'NONE ', 'USAT16_T1   ', 'cond=COND '];



# selbytes: Select Bytes.
# A32: cond:u!=0b1111 ig0=0b01101000 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b1011 Rm:u=0bxxxx
HINT('ICLASS', 'selbytes', 'A32');

DEF Rdupc => GPR32('Rd', 'Rd', 'write|upc') ;
DEF Rmupc => GPR32('Rm', 'Rm', 'read|upc')  ;
DEF Rnupc => GPR32('Rn', 'Rn', 'read|upc')  ;

T[' SEL REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SEL_A1 ', 'cond=NO_NV '];



# setpan: SETPAN.
# A32: ig0=0b111100010001 ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ imm1:u=0bx ig11=0bZ ig12=0b0000 ig13=0bZ ig14=0bZ ig15=0bZ ig16=0bZ
HINT('ICLASS', 'setpan', 'A32');

DEF imm => IMM('imm', 'imm1') ;

T[' SETPAN IMM:i32=$imm ', 'NONE ', 'SETPAN_A1 ', 'arch_variant=ARMv8v1 extensions=PAN cond=UNCOND '];



# shiftr: Register shifts.
# T32: ig0=0b111110100 stype:u=0bxx S:u=0bx Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx
HINT('ICLASS', 'shiftr', 'T32');

DEF Rdupc  => GPR32('Rd', 'Rd', 'write|upc')                 ;
DEF Rmupc  => GPR32('Rm', 'Rm', 'read|upc')                  ;
DEF Rsupc  => GPR32('Rs', 'Rs', 'upc')                       ;
DEF shift0 => SHIFT('shift', 'stype', table => 'tbl_shift1') ;

T[' MOVS REG=$Rdupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'MOVS_rr_T2      ', 'cond=COND showqlf=1 '];
T[' MOV  REG=$Rdupc REGSH={REG=$Rmupc SHIFTER={SHIFT=$shift0 RS:r=$Rsupc}} ', 'NONE ', 'MOV_rr_T2       ', 'cond=COND showqlf=1 '];
T[' ASR  REG:OPT=$Rdupc REG=$Rmupc REG:r=$Rsupc                            ', 'NONE ', 'ASR_MOV_rr_T2   ', 'cond=COND showqlf=1 '];
T[' ASRS REG:OPT=$Rdupc REG=$Rmupc REG:r=$Rsupc                            ', 'NONE ', 'ASRS_MOVS_rr_T2 ', 'cond=COND showqlf=1 '];
T[' LSL  REG:OPT=$Rdupc REG=$Rmupc REG:r=$Rsupc                            ', 'NONE ', 'LSL_MOV_rr_T2   ', 'cond=COND showqlf=1 '];
T[' LSLS REG:OPT=$Rdupc REG=$Rmupc REG:r=$Rsupc                            ', 'NONE ', 'LSLS_MOVS_rr_T2 ', 'cond=COND showqlf=1 '];
T[' LSR  REG:OPT=$Rdupc REG=$Rmupc REG:r=$Rsupc                            ', 'NONE ', 'LSR_MOV_rr_T2   ', 'cond=COND showqlf=1 '];
T[' LSRS REG:OPT=$Rdupc REG=$Rmupc REG:r=$Rsupc                            ', 'NONE ', 'LSRS_MOVS_rr_T2 ', 'cond=COND showqlf=1 '];
T[' ROR  REG:OPT=$Rdupc REG=$Rmupc REG:r=$Rsupc                            ', 'NONE ', 'ROR_MOV_rr_T2   ', 'cond=COND showqlf=1 '];
T[' RORS REG:OPT=$Rdupc REG=$Rmupc REG:r=$Rsupc                            ', 'NONE ', 'RORS_MOVS_rr_T2 ', 'cond=COND showqlf=1 '];



# smul_div: Signed multiply, Divide.
# A32: cond:u!=0b1111 ig0=0b01110 op1:u=0bxxx Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx op2:u=0bxxx ig1=0b1 Rn:u=0bxxxx
HINT('ICLASS', 'smul_div', 'A32');

DEF Ranpc   => GPR32('Ra', 'Ra', 'read|nopc')     ;
DEF Raupc   => GPR32('Ra', 'Ra', 'read|upc')      ;
DEF RdHiupc => GPR32('RdHi', 'RdHi', 'write|upc') ;
DEF RdLoupc => GPR32('RdLo', 'RdLo', 'write|upc') ;
DEF Rdupc   => GPR32('Rd', 'Rd', 'write|upc')     ;
DEF Rmupc   => GPR32('Rm', 'Rm', 'read|upc')      ;
DEF Rnupc   => GPR32('Rn', 'Rn', 'read|upc')      ;

T[' SDIV    REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc            ', 'NONE ', 'SDIV_A1    ', 'cond=NO_NV '];
T[' SMLAD   REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc     ', 'NONE ', 'SMLAD_A1   ', 'cond=NO_NV '];
T[' SMLADX  REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc     ', 'NONE ', 'SMLADX_A1  ', 'cond=NO_NV '];
T[' SMLALD  REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLALD_A1  ', 'cond=NO_NV '];
T[' SMLALDX REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLALDX_A1 ', 'cond=NO_NV '];
T[' SMLSD   REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc     ', 'NONE ', 'SMLSD_A1   ', 'cond=NO_NV '];
T[' SMLSDX  REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc     ', 'NONE ', 'SMLSDX_A1  ', 'cond=NO_NV '];
T[' SMLSLD  REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLSLD_A1  ', 'cond=NO_NV '];
T[' SMLSLDX REG=$RdLoupc REG=$RdHiupc REG=$Rnupc REG=$Rmupc ', 'NONE ', 'SMLSLDX_A1 ', 'cond=NO_NV '];
T[' SMMLA   REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc     ', 'NONE ', 'SMMLA_A1   ', 'cond=NO_NV '];
T[' SMMLAR  REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc     ', 'NONE ', 'SMMLAR_A1  ', 'cond=NO_NV '];
T[' SMMLS   REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Raupc     ', 'NONE ', 'SMMLS_A1   ', 'cond=NO_NV '];
T[' SMMLSR  REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Raupc     ', 'NONE ', 'SMMLSR_A1  ', 'cond=NO_NV '];
T[' SMMUL   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc            ', 'NONE ', 'SMMUL_A1   ', 'cond=NO_NV '];
T[' SMMULR  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc            ', 'NONE ', 'SMMULR_A1  ', 'cond=NO_NV '];
T[' SMUAD   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc            ', 'NONE ', 'SMUAD_A1   ', 'cond=NO_NV '];
T[' SMUADX  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc            ', 'NONE ', 'SMUADX_A1  ', 'cond=NO_NV '];
T[' SMUSD   REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc            ', 'NONE ', 'SMUSD_A1   ', 'cond=NO_NV '];
T[' SMUSDX  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc            ', 'NONE ', 'SMUSDX_A1  ', 'cond=NO_NV '];
T[' UDIV    REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc            ', 'NONE ', 'UDIV_A1    ', 'cond=NO_NV '];



# svc: Supervisor Call.
# A32: cond:u!=0b1111 ig0=0b1111 imm24:u=0bxxxxxxxxxxxxxxxxxxxxxxxx
HINT('ICLASS', 'svc', 'A32');

DEF imm => IMM('imm', 'imm24', range => (0, 16777215)) ;

T[' SVC IMM:u32=$imm ', 'NONE ', 'SVC_A1 ', 'cond=NO_NV '];



# system: Miscellaneous system.
# T32: ig0=0b111100111011 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bS ig9=0bS ig10=0bS ig11=0bS opc:u=0bxxxx option:u=0bxxxx
HINT('ICLASS', 'system', 'T32');

DEF option  => BARRIER('option', 'option', table => 'tbl_barrier')  ;
DEF option2 => BARRIER('option', 'option', table => 'tbl_barrier1') ;

T[' CLREX                      ', 'NONE ', 'CLREX_T1 ', 'cond=COND                 '];
T[' DMB   BARRIER:OPT=$option  ', 'NONE ', 'DMB_T1   ', 'cond=COND                 '];
T[' DSB   BARRIER:OPT=$option  ', 'NONE ', 'DSB_T1   ', 'cond=COND                 '];
T[' ISB   BARRIER:OPT=$option2 ', 'NONE ', 'ISB_T1   ', 'cond=COND                 '];
T[' PSSBB                      ', 'NONE ', 'PSSBB_T1 ', 'cond=UNP_COND may_store=1 '];
T[' SB                         ', 'NONE ', 'SB_T1    ', 'cond=UNP_COND             '];
T[' SSBB                       ', 'NONE ', 'SSBB_T1  ', 'cond=UNP_COND may_store=1 '];



# tblbr: Table branch.
# T32: ig0=0b111010001101 Rn:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ ig9=0b000 H:u=0bx Rm:u=0bxxxx
HINT('ICLASS', 'tblbr', 'T32');

DEF Rmupc => GPR32('Rm', 'Rm', 'read|upc') ;
DEF Rn    => GPR32('Rn', 'Rn', 'read')     ;

T[' TBB MEM:OFF:r:i8={ BASE=$Rn SZ=8  REGOFF=$Rmupc}                                         ', 'NONE ', 'TBB_T1 ', 'cond=COND '];
T[' TBH MEM:OFF:r:i16={BASE=$Rn SZ=16 REGOFFSH={REG=$Rmupc SHIFTER={SHIFT=LSL AMOUNT:u8=1}}} ', 'NONE ', 'TBH_T1 ', 'cond=COND '];



# udf: Permanently UNDEFINED.
# A32: cond:u!=0b1111 ig0=0b01111111 imm12:u=0bxxxxxxxxxxxx ig1=0b1111 imm4:u=0bxxxx
HINT('ICLASS', 'udf', 'A32');

DEF imm => IMM('imm', 'imm12:imm4', range => (0, 65535)) ;

T[' UDF IMM:u16=$imm ', 'NONE ', 'UDF_A1 ', 'cond=MUST_AL '];



# usad: Unsigned Sum of Absolute Differences.
# A32: cond:u!=0b1111 ig0=0b01111000 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b0001 Rn:u=0bxxxx
HINT('ICLASS', 'usad', 'A32');

DEF Ranpc => GPR32('Ra', 'Ra', 'read|nopc') ;
DEF Rdupc => GPR32('Rd', 'Rd', 'write|upc') ;
DEF Rmupc => GPR32('Rm', 'Rm', 'read|upc')  ;
DEF Rnupc => GPR32('Rn', 'Rn', 'read|upc')  ;

T[' USAD8  REG:OPT=$Rdupc REG=$Rnupc REG=$Rmupc        ', 'NONE ', 'USAD8_A1  ', 'cond=NO_NV '];
T[' USADA8 REG=$Rdupc REG=$Rnupc REG=$Rmupc REG=$Ranpc ', 'NONE ', 'USADA8_A1 ', 'cond=NO_NV '];



1;

