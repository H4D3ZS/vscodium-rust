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


# addsub_carry: Add/subtract (with carry).
# A64: sf:u=0bx op:u=0bx S:u=0bx ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'addsub_carry', 'A64');

DEF Wd => GPR32('Wd', 'Rd', 'write') ;
DEF Wm => GPR32('Wm', 'Rm', 'read')  ;
DEF Wn => GPR32('Wn', 'Rn', 'read')  ;
DEF Xd => GPR64('Xd', 'Rd', 'write') ;
DEF Xm => GPR64('Xm', 'Rm', 'read')  ;
DEF Xn => GPR64('Xn', 'Rn', 'read')  ;

T[' ADC  REG=$Wd REG=$Wn REG=$Wm ', 'NONE ', 'ADC_32_addsub_carry       ', ' '];
T[' ADC  REG=$Xd REG=$Xn REG=$Xm ', 'NONE ', 'ADC_64_addsub_carry       ', ' '];
T[' ADCS REG=$Wd REG=$Wn REG=$Wm ', 'NONE ', 'ADCS_32_addsub_carry      ', ' '];
T[' ADCS REG=$Xd REG=$Xn REG=$Xm ', 'NONE ', 'ADCS_64_addsub_carry      ', ' '];
T[' SBC  REG=$Wd REG=$Wn REG=$Wm ', 'NONE ', 'SBC_32_addsub_carry       ', ' '];
T[' SBC  REG=$Xd REG=$Xn REG=$Xm ', 'NONE ', 'SBC_64_addsub_carry       ', ' '];
T[' SBCS REG=$Wd REG=$Wn REG=$Wm ', 'NONE ', 'SBCS_32_addsub_carry      ', ' '];
T[' SBCS REG=$Xd REG=$Xn REG=$Xm ', 'NONE ', 'SBCS_64_addsub_carry      ', ' '];
T[' NGC  REG=$Wd REG=$Wm         ', 'NONE ', 'NGC_SBC_32_addsub_carry   ', ' '];
T[' NGC  REG=$Xd REG=$Xm         ', 'NONE ', 'NGC_SBC_64_addsub_carry   ', ' '];
T[' NGCS REG=$Wd REG=$Wm         ', 'NONE ', 'NGCS_SBCS_32_addsub_carry ', ' '];
T[' NGCS REG=$Xd REG=$Xm         ', 'NONE ', 'NGCS_SBCS_64_addsub_carry ', ' '];



# addsub_ext: Add/subtract (extended register).
# A64: sf:u=0bx op:u=0bx S:u=0bx ig0=0b01011 opt:u=0bxx ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'addsub_ext', 'A64');

DEF Wd      => GPR32('Wd', 'Rd', 'write')                           ;
DEF WdSP    => GPR32('Wd', 'Rd', 'write|SP')                        ;
DEF Wm      => GPR32('Wm', 'Rm', 'read')                            ;
DEF WnSP    => GPR32('Wn', 'Rn', 'read|SP')                         ;
DEF Xd      => GPR64('Xd', 'Rd', 'write')                           ;
DEF XdSP    => GPR64('Xd', 'Rd', 'write|SP')                        ;
DEF Xm      => GPR64('Xm', 'Rm', 'read')                            ;
DEF XnSP    => GPR64('Xn', 'Rn', 'read|SP')                         ;
DEF amount0 => IMM('amount', 'imm3', range => (0, 4), default => 0) ;
DEF extend0 => EXTEND('extend', 'option', table => 'tbl_extend0')   ;
DEF extend2 => EXTEND('extend', 'option', table => 'tbl_extend1')   ;

T[' ADD  REG=$WdSP REG=$WnSP REGSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend0 AMOUNT:u8=$amount0 }} ', 'NONE         ', 'ADD_32_addsub_ext       ', ' '];
T[' ADD  REG=$XdSP REG=$XnSP REGSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:u8=$amount0 }} ', 'option=0bx11 ', 'ADD_64_addsub_ext       ', ' '];
T[' ADD  REG=$XdSP REG=$XnSP REGSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:u8=$amount0 }} ', 'option=0bxxx ', 'ADD_64_addsub_ext       ', ' '];
T[' ADDS REG=$Wd REG=$WnSP REGSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend0 AMOUNT:u8=$amount0 }}   ', 'NONE         ', 'ADDS_32S_addsub_ext     ', ' '];
T[' ADDS REG=$Xd REG=$XnSP REGSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:u8=$amount0 }}   ', 'option=0bx11 ', 'ADDS_64S_addsub_ext     ', ' '];
T[' ADDS REG=$Xd REG=$XnSP REGSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:u8=$amount0 }}   ', 'option=0bxxx ', 'ADDS_64S_addsub_ext     ', ' '];
T[' SUB  REG=$WdSP REG=$WnSP REGSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend0 AMOUNT:u8=$amount0 }} ', 'NONE         ', 'SUB_32_addsub_ext       ', ' '];
T[' SUB  REG=$XdSP REG=$XnSP REGSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:u8=$amount0 }} ', 'option=0bx11 ', 'SUB_64_addsub_ext       ', ' '];
T[' SUB  REG=$XdSP REG=$XnSP REGSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:u8=$amount0 }} ', 'option=0bxxx ', 'SUB_64_addsub_ext       ', ' '];
T[' SUBS REG=$Wd REG=$WnSP REGSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend0 AMOUNT:u8=$amount0 }}   ', 'NONE         ', 'SUBS_32S_addsub_ext     ', ' '];
T[' SUBS REG=$Xd REG=$XnSP REGSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:u8=$amount0 }}   ', 'option=0bx11 ', 'SUBS_64S_addsub_ext     ', ' '];
T[' SUBS REG=$Xd REG=$XnSP REGSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:u8=$amount0 }}   ', 'option=0bxxx ', 'SUBS_64S_addsub_ext     ', ' '];
T[' CMN  REG=$WnSP REGSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend0 AMOUNT:u8=$amount0 }}           ', 'NONE         ', 'CMN_ADDS_32S_addsub_ext ', ' '];
T[' CMN  REG=$XnSP REGSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:u8=$amount0 }}           ', 'option=0bx11 ', 'CMN_ADDS_64S_addsub_ext ', ' '];
T[' CMN  REG=$XnSP REGSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:u8=$amount0 }}           ', 'option=0bxxx ', 'CMN_ADDS_64S_addsub_ext ', ' '];
T[' CMP  REG=$WnSP REGSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend0 AMOUNT:u8=$amount0 }}           ', 'NONE         ', 'CMP_SUBS_32S_addsub_ext ', ' '];
T[' CMP  REG=$XnSP REGSH={REG=$Xm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:u8=$amount0 }}           ', 'option=0bx11 ', 'CMP_SUBS_64S_addsub_ext ', ' '];
T[' CMP  REG=$XnSP REGSH={REG=$Wm SHIFTER:OPT={EXTEND=$extend2 AMOUNT:u8=$amount0 }}           ', 'option=0bxxx ', 'CMP_SUBS_64S_addsub_ext ', ' '];



# addsub_imm: Add/subtract (immediate).
# A64: sf:u=0bx op:u=0bx S:u=0bx ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'addsub_imm', 'A64');

DEF Wd      => GPR32('Wd', 'Rd', 'write')                                            ;
DEF WdSP    => GPR32('Wd', 'Rd', 'write|SP')                                         ;
DEF WnSP    => GPR32('Wn', 'Rn', 'read|SP')                                          ;
DEF Xd      => GPR64('Xd', 'Rd', 'write')                                            ;
DEF XdSP    => GPR64('Xd', 'Rd', 'write|SP')                                         ;
DEF XnSP    => GPR64('Xn', 'Rn', 'read|SP')                                          ;
DEF imm     => IMM('imm', 'imm12', range => (0, 4095))                               ;
DEF shifter => SHIFTER('shifter', 'sh', table => 'tbl_shifter', default => 'LSL #0') ;

T[' ADD  REG=$WdSP REG=$WnSP IMMSH={IMM:u16=$imm SHIFTER:OPT=$shifter} ', 'NONE ', 'ADD_32_addsub_imm       ', ' '];
T[' ADD  REG=$XdSP REG=$XnSP IMMSH={IMM:u16=$imm SHIFTER:OPT=$shifter} ', 'NONE ', 'ADD_64_addsub_imm       ', ' '];
T[' ADDS REG=$Wd REG=$WnSP IMMSH={IMM:u16=$imm SHIFTER:OPT=$shifter}   ', 'NONE ', 'ADDS_32S_addsub_imm     ', ' '];
T[' ADDS REG=$Xd REG=$XnSP IMMSH={IMM:u16=$imm SHIFTER:OPT=$shifter}   ', 'NONE ', 'ADDS_64S_addsub_imm     ', ' '];
T[' SUB  REG=$WdSP REG=$WnSP IMMSH={IMM:u16=$imm SHIFTER:OPT=$shifter} ', 'NONE ', 'SUB_32_addsub_imm       ', ' '];
T[' SUB  REG=$XdSP REG=$XnSP IMMSH={IMM:u16=$imm SHIFTER:OPT=$shifter} ', 'NONE ', 'SUB_64_addsub_imm       ', ' '];
T[' SUBS REG=$Wd REG=$WnSP IMMSH={IMM:u16=$imm SHIFTER:OPT=$shifter}   ', 'NONE ', 'SUBS_32S_addsub_imm     ', ' '];
T[' SUBS REG=$Xd REG=$XnSP IMMSH={IMM:u16=$imm SHIFTER:OPT=$shifter}   ', 'NONE ', 'SUBS_64S_addsub_imm     ', ' '];
T[' CMN  REG=$WnSP IMMSH={IMM:u16=$imm SHIFTER:OPT=$shifter}           ', 'NONE ', 'CMN_ADDS_32S_addsub_imm ', ' '];
T[' CMN  REG=$XnSP IMMSH={IMM:u16=$imm SHIFTER:OPT=$shifter}           ', 'NONE ', 'CMN_ADDS_64S_addsub_imm ', ' '];
T[' CMP  REG=$WnSP IMMSH={IMM:u16=$imm SHIFTER:OPT=$shifter}           ', 'NONE ', 'CMP_SUBS_32S_addsub_imm ', ' '];
T[' CMP  REG=$XnSP IMMSH={IMM:u16=$imm SHIFTER:OPT=$shifter}           ', 'NONE ', 'CMP_SUBS_64S_addsub_imm ', ' '];
T[' MOV  REG=$WdSP REG=$WnSP                                           ', 'NONE ', 'MOV_ADD_32_addsub_imm   ', ' '];
T[' MOV  REG=$XdSP REG=$XnSP                                           ', 'NONE ', 'MOV_ADD_64_addsub_imm   ', ' '];



# addsub_immtags: Add/subtract (immediate, with tags).
# A64: sf:u=0bx op:u=0bx S:u=0bx ig0=0b100011 o2:u=0bx uimm6:u=0bxxxxxx op3:u=0bxx uimm4:u=0bxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'addsub_immtags', 'A64');

DEF XdSP    => GPR64('Xd', 'Xd', 'write|SP')                          ;
DEF XnSP    => GPR64('Xn', 'Xn', 'read|SP')                           ;
DEF uimm    => IMM('uimm4', 'uimm4', range => (0, 15))                ;
DEF uimmx16 => IMM('uimm6', 'uimm6', scale => 16, range => (0, 1008)) ;

T[' ADDG REG=$XdSP REG=$XnSP IMM:u16=$uimmx16 IMM:u8=$uimm ', 'NONE ', 'ADDG_64_addsub_immtags ', 'arch_variant=ARMv8v5 '];
T[' SUBG REG=$XdSP REG=$XnSP IMM:u16=$uimmx16 IMM:u8=$uimm ', 'NONE ', 'SUBG_64_addsub_immtags ', 'arch_variant=ARMv8v5 '];



# addsub_shift: Add/subtract (shifted register).
# A64: sf:u=0bx op:u=0bx S:u=0bx ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'addsub_shift', 'A64');

DEF Wd      => GPR32('Wd', 'Rd', 'write')                                       ;
DEF Wm      => GPR32('Wm', 'Rm', 'read')                                        ;
DEF Wn      => GPR32('Wn', 'Rn', 'read')                                        ;
DEF Xd      => GPR64('Xd', 'Rd', 'write')                                       ;
DEF Xm      => GPR64('Xm', 'Rm', 'read')                                        ;
DEF Xn      => GPR64('Xn', 'Rn', 'read')                                        ;
DEF amount0 => IMM('amount', 'imm6', range => (0, 31), default => 0)            ;
DEF amount2 => IMM('amount', 'imm6', range => (0, 63), default => 0)            ;
DEF shift0  => SHIFT('shift', 'shift', table => 'tbl_shift0', default => 'LSL') ;

T[' ADD  REG=$Wd REG=$Wn REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'ADD_32_addsub_shift       ', ' '];
T[' ADD  REG=$Xd REG=$Xn REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }} ', 'NONE ', 'ADD_64_addsub_shift       ', ' '];
T[' ADDS REG=$Wd REG=$Wn REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'ADDS_32_addsub_shift      ', ' '];
T[' ADDS REG=$Xd REG=$Xn REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }} ', 'NONE ', 'ADDS_64_addsub_shift      ', ' '];
T[' SUB  REG=$Wd REG=$Wn REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'SUB_32_addsub_shift       ', ' '];
T[' SUB  REG=$Xd REG=$Xn REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }} ', 'NONE ', 'SUB_64_addsub_shift       ', ' '];
T[' SUBS REG=$Wd REG=$Wn REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'SUBS_32_addsub_shift      ', ' '];
T[' SUBS REG=$Xd REG=$Xn REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }} ', 'NONE ', 'SUBS_64_addsub_shift      ', ' '];
T[' CMN  REG=$Wn REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}         ', 'NONE ', 'CMN_ADDS_32_addsub_shift  ', ' '];
T[' CMN  REG=$Xn REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }}         ', 'NONE ', 'CMN_ADDS_64_addsub_shift  ', ' '];
T[' CMP  REG=$Wn REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}         ', 'NONE ', 'CMP_SUBS_32_addsub_shift  ', ' '];
T[' CMP  REG=$Xn REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }}         ', 'NONE ', 'CMP_SUBS_64_addsub_shift  ', ' '];
T[' NEG  REG=$Wd REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}         ', 'NONE ', 'NEG_SUB_32_addsub_shift   ', ' '];
T[' NEG  REG=$Xd REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }}         ', 'NONE ', 'NEG_SUB_64_addsub_shift   ', ' '];
T[' NEGS REG=$Wd REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}         ', 'NONE ', 'NEGS_SUBS_32_addsub_shift ', ' '];
T[' NEGS REG=$Xd REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }}         ', 'NONE ', 'NEGS_SUBS_64_addsub_shift ', ' '];



# barriers: Barriers.
# A64: ig0=0b11010101000000110011 CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx
HINT('ICLASS', 'barriers', 'A64');

DEF imm    => IMM('imm', 'CRm', range => (0, 15), default => 15) ;
DEF option => BARRIER('option', 'CRm')                           ;

T[' CLREX   IMM:OPT:u8=$imm     ', 'NONE ', 'CLREX_BN_barriers     ', '                                   '];
T[' DMB     BARRIER=$option     ', 'NONE ', 'DMB_BO_barriers       ', '                                   '];
T[' DSB     BARRIER=$option     ', 'NONE ', 'DSB_BO_barriers       ', '                                   '];
T[' ISB     BARRIER:OPT=$option ', 'NONE ', 'ISB_BI_barriers       ', '                                   '];
T[' PSSBB                       ', 'NONE ', 'PSSBB_only_barriers   ', 'may_store=1                        '];
T[' SB                          ', 'NONE ', 'SB_only_barriers      ', 'arch_variant=ARMv8v5 extensions=SB '];
T[' SSBB                        ', 'NONE ', 'SSBB_only_barriers    ', 'may_store=1                        '];
T[' TCOMMIT                     ', 'NONE ', 'TCOMMIT_only_barriers ', 'arch_variant=TME                   '];
T[' DFB                         ', 'NONE ', 'DFB_DSB_BO_barriers   ', 'arch_variant=vv                    '];



# bitfield: Bitfield.
# A64: sf:u=0bx opc:u=0bxx ig0=0b100110 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'bitfield', 'A64');

DEF Wd     => GPR32('Wd', 'Rd', 'write')             ;
DEF Wn     => GPR32('Wn', 'Rn', 'read')              ;
DEF Wnnwzr => GPR32('Wn', 'Rn', 'read|nowzr')        ;
DEF Xd     => GPR64('Xd', 'Rd', 'write')             ;
DEF Xn     => GPR64('Xn', 'Rn', 'read')              ;
DEF Xnnxzr => GPR64('Xn', 'Rn', 'read|noxzr')        ;
DEF immr   => IMM('immr', 'immr', range => (0, 31))  ;
DEF immr2  => IMM('immr', 'immr', range => (0, 63))  ;
DEF imms   => IMM('imms', 'imms', range => (0, 31))  ;
DEF imms2  => IMM('imms', 'imms', range => (0, 63))  ;
DEF lsb    => IMM('lsb', 'immr', range => (0, 31))   ;
DEF lsb2   => IMM('lsb', 'immr', range => (0, 63))   ;
DEF shift0 => IMM('shift', 'immr', range => (0, 31)) ;
DEF shift2 => IMM('shift', 'immr', range => (0, 63)) ;
DEF width  => IMM('width', 'imms', range => (1, 32)) ;
DEF width2 => IMM('width', 'imms', range => (1, 64)) ;

T[' BFM   REG=$Wd REG=$Wn IMM:u8=$immr IMM:u8=$imms           ', 'NONE ', 'BFM_32M_bitfield        ', '                     '];
T[' BFM   REG=$Xd REG=$Xn IMM:u8=$immr2 IMM:u8=$imms2         ', 'NONE ', 'BFM_64M_bitfield        ', '                     '];
T[' SBFM  REG:s32=$Wd REG:s32=$Wn IMM:u8=$immr IMM:u8=$imms   ', 'NONE ', 'SBFM_32M_bitfield       ', '                     '];
T[' SBFM  REG:s64=$Xd REG:s64=$Xn IMM:u8=$immr2 IMM:u8=$imms2 ', 'NONE ', 'SBFM_64M_bitfield       ', '                     '];
T[' UBFM  REG:u32=$Wd REG:u32=$Wn IMM:u8=$immr IMM:u8=$imms   ', 'NONE ', 'UBFM_32M_bitfield       ', '                     '];
T[' UBFM  REG:u64=$Xd REG:u64=$Xn IMM:u8=$immr2 IMM:u8=$imms2 ', 'NONE ', 'UBFM_64M_bitfield       ', '                     '];
T[' ASR   REG=$Wd REG=$Wn IMM:u8=$shift0                      ', 'NONE ', 'ASR_SBFM_32M_bitfield   ', '                     '];
T[' ASR   REG=$Xd REG=$Xn IMM:u8=$shift2                      ', 'NONE ', 'ASR_SBFM_64M_bitfield   ', '                     '];
T[' BFC   REG=$Wd IMM:u8=$lsb IMM:u8=$width                   ', 'NONE ', 'BFC_BFM_32M_bitfield    ', 'arch_variant=ARMv8v2 '];
T[' BFC   REG=$Xd IMM:u8=$lsb2 IMM:u8=$width2                 ', 'NONE ', 'BFC_BFM_64M_bitfield    ', 'arch_variant=ARMv8v2 '];
T[' BFI   REG=$Wd REG=$Wnnwzr IMM:u8=$lsb IMM:u8=$width       ', 'NONE ', 'BFI_BFM_32M_bitfield    ', '                     '];
T[' BFI   REG=$Xd REG=$Xnnxzr IMM:u8=$lsb2 IMM:u8=$width2     ', 'NONE ', 'BFI_BFM_64M_bitfield    ', '                     '];
T[' BFXIL REG=$Wd REG=$Wn IMM:u8=$lsb IMM:u8=$width           ', 'NONE ', 'BFXIL_BFM_32M_bitfield  ', '                     '];
T[' BFXIL REG=$Xd REG=$Xn IMM:u8=$lsb2 IMM:u8=$width2         ', 'NONE ', 'BFXIL_BFM_64M_bitfield  ', '                     '];
T[' LSL   REG=$Wd REG=$Wn IMM:u8=$shift0                      ', 'NONE ', 'LSL_UBFM_32M_bitfield   ', '                     '];
T[' LSL   REG=$Xd REG=$Xn IMM:u8=$shift2                      ', 'NONE ', 'LSL_UBFM_64M_bitfield   ', '                     '];
T[' LSR   REG=$Wd REG=$Wn IMM:u8=$shift0                      ', 'NONE ', 'LSR_UBFM_32M_bitfield   ', '                     '];
T[' LSR   REG=$Xd REG=$Xn IMM:u8=$shift2                      ', 'NONE ', 'LSR_UBFM_64M_bitfield   ', '                     '];
T[' SBFIZ REG:s32=$Wd REG:s32=$Wn IMM:u8=$lsb IMM:u8=$width   ', 'NONE ', 'SBFIZ_SBFM_32M_bitfield ', '                     '];
T[' SBFIZ REG:s64=$Xd REG:s64=$Xn IMM:u8=$lsb2 IMM:u8=$width2 ', 'NONE ', 'SBFIZ_SBFM_64M_bitfield ', '                     '];
T[' SBFX  REG:s32=$Wd REG:s32=$Wn IMM:u8=$lsb IMM:u8=$width   ', 'NONE ', 'SBFX_SBFM_32M_bitfield  ', '                     '];
T[' SBFX  REG:s64=$Xd REG:s64=$Xn IMM:u8=$lsb2 IMM:u8=$width2 ', 'NONE ', 'SBFX_SBFM_64M_bitfield  ', '                     '];
T[' SXTB  REG:s32=$Wd REG:s32=$Wn                             ', 'NONE ', 'SXTB_SBFM_32M_bitfield  ', '                     '];
T[' SXTB  REG:s64=$Xd REG:s32=$Wn                             ', 'NONE ', 'SXTB_SBFM_64M_bitfield  ', '                     '];
T[' SXTH  REG=$Wd REG=$Wn                                     ', 'NONE ', 'SXTH_SBFM_32M_bitfield  ', '                     '];
T[' SXTH  REG=$Xd REG=$Wn                                     ', 'NONE ', 'SXTH_SBFM_64M_bitfield  ', '                     '];
T[' SXTW  REG=$Xd REG=$Wn                                     ', 'NONE ', 'SXTW_SBFM_64M_bitfield  ', '                     '];
T[' UBFIZ REG:u32=$Wd REG:u32=$Wn IMM:u8=$lsb IMM:u8=$width   ', 'NONE ', 'UBFIZ_UBFM_32M_bitfield ', '                     '];
T[' UBFIZ REG:u64=$Xd REG:u64=$Xn IMM:u8=$lsb2 IMM:u8=$width2 ', 'NONE ', 'UBFIZ_UBFM_64M_bitfield ', '                     '];
T[' UBFX  REG:u32=$Wd REG:u32=$Wn IMM:u8=$lsb IMM:u8=$width   ', 'NONE ', 'UBFX_UBFM_32M_bitfield  ', '                     '];
T[' UBFX  REG:u64=$Xd REG:u64=$Xn IMM:u8=$lsb2 IMM:u8=$width2 ', 'NONE ', 'UBFX_UBFM_64M_bitfield  ', '                     '];
T[' UXTB  REG:u32=$Wd REG:u32=$Wn                             ', 'NONE ', 'UXTB_UBFM_32M_bitfield  ', '                     '];
T[' UXTH  REG:u32=$Wd REG:u32=$Wn                             ', 'NONE ', 'UXTH_UBFM_32M_bitfield  ', '                     '];



# branch_imm: Unconditional branch (immediate).
# A64: op:u=0bx ig0=0b00101 imm26:u=0bxxxxxxxxxxxxxxxxxxxxxxxxxx
HINT('ICLASS', 'branch_imm', 'A64');

DEF labelx4 => LABEL('label', 'imm26', scale => 4, signpos => 'imm26<25>', range => (-128000000, 128000000)) ;

T[' B  LABEL:s64=$labelx4 ', 'NONE ', 'B_only_branch_imm  ', ' '];
T[' BL LABEL:s64=$labelx4 ', 'NONE ', 'BL_only_branch_imm ', ' '];



# branch_reg: Unconditional branch (register).
# A64: ig0=0b1101011 opc:u=0bxxxx op2:u=0bxxxxx op3:u=0bxxxxxx Rn:u=0bxxxxx op4:u=0bxxxxx
HINT('ICLASS', 'branch_reg', 'A64');

DEF XmSP => GPR64('Xm', 'Rm', 'read|SP')                ;
DEF Xn   => GPR64('Xn', 'Rn', 'read')                   ;
DEF Xn2  => GPR64('Xn', 'Rn', default => 'X30', 'read') ;

T[' BLR    REG=$Xn           ', 'NONE ', 'BLR_64_branch_reg     ', '                                    '];
T[' BLRAAZ REG=$Xn           ', 'NONE ', 'BLRAAZ_64_branch_reg  ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' BLRAA  REG=$Xn REG=$XmSP ', 'NONE ', 'BLRAA_64P_branch_reg  ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' BLRABZ REG=$Xn           ', 'NONE ', 'BLRABZ_64_branch_reg  ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' BLRAB  REG=$Xn REG=$XmSP ', 'NONE ', 'BLRAB_64P_branch_reg  ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' BR     REG=$Xn           ', 'NONE ', 'BR_64_branch_reg      ', '                                    '];
T[' BRAAZ  REG=$Xn           ', 'NONE ', 'BRAAZ_64_branch_reg   ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' BRAA   REG=$Xn REG=$XmSP ', 'NONE ', 'BRAA_64P_branch_reg   ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' BRABZ  REG=$Xn           ', 'NONE ', 'BRABZ_64_branch_reg   ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' BRAB   REG=$Xn REG=$XmSP ', 'NONE ', 'BRAB_64P_branch_reg   ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' DRPS                     ', 'NONE ', 'DRPS_64E_branch_reg   ', '                                    '];
T[' ERET                     ', 'NONE ', 'ERET_64E_branch_reg   ', '                                    '];
T[' ERETAA                   ', 'NONE ', 'ERETAA_64E_branch_reg ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' ERETAB                   ', 'NONE ', 'ERETAB_64E_branch_reg ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' RET    REG:OPT=$Xn2      ', 'NONE ', 'RET_64R_branch_reg    ', '                                    '];
T[' RETAA                    ', 'NONE ', 'RETAA_64E_branch_reg  ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' RETAB                    ', 'NONE ', 'RETAB_64E_branch_reg  ', 'arch_variant=ARMv8v3 extensions=PAC '];



# compbranch: Compare and branch (immediate).
# A64: sf:u=0bx ig0=0b011010 op:u=0bx imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'compbranch', 'A64');

DEF Wt      => GPR32('Wt', 'Rt')                                                                         ;
DEF Xt      => GPR64('Xt', 'Rt')                                                                         ;
DEF labelx4 => LABEL('label', 'imm19', scale => 4, signpos => 'imm19<18>', range => (-1000000, 1000000)) ;

T[' CBNZ REG:r=$Wt LABEL:s64=$labelx4 ', 'NONE ', 'CBNZ_32_compbranch ', ' '];
T[' CBNZ REG:r=$Xt LABEL:s64=$labelx4 ', 'NONE ', 'CBNZ_64_compbranch ', ' '];
T[' CBZ  REG:r=$Wt LABEL:s64=$labelx4 ', 'NONE ', 'CBZ_32_compbranch  ', ' '];
T[' CBZ  REG:r=$Xt LABEL:s64=$labelx4 ', 'NONE ', 'CBZ_64_compbranch  ', ' '];



# condbranch: Conditional branch (immediate).
# A64: ig0=0b0101010 o1:u=0bx imm19:u=0bxxxxxxxxxxxxxxxxxxx o0:u=0bx cond:u=0bxxxx
HINT('ICLASS', 'condbranch', 'A64');

DEF labelx4 => LABEL('label', 'imm19', scale => 4, signpos => 'imm19<18>', range => (-1000000, 1000000)) ;

T[' B LABEL:s64=$labelx4 ', 'NONE ', 'B_only_condbranch ', 'cond=COND '];



# condcmp_imm: Conditional compare (immediate).
# A64: sf:u=0bx op:u=0bx S:u=0bx ig0=0b11010010 imm5:u=0bxxxxx cond:u=0bxxxx ig1=0b1 o2:u=0bx Rn:u=0bxxxxx o3:u=0bx nzcv:u=0bxxxx
HINT('ICLASS', 'condcmp_imm', 'A64');

DEF Wn   => GPR32('Wn', 'Rn', 'read')             ;
DEF Xn   => GPR64('Xn', 'Rn', 'read')             ;
DEF cond => COND('cond', 'cond')                  ;
DEF imm  => IMM('imm', 'imm5')                    ;
DEF nzcv => IMM('nzcv', 'nzcv', range => (0, 15)) ;

T[' CCMN REG=$Wn IMM:i32=$imm IMM:u8=$nzcv COND=$cond ', 'NONE ', 'CCMN_32_condcmp_imm ', ' '];
T[' CCMN REG=$Xn IMM:i64=$imm IMM:u8=$nzcv COND=$cond ', 'NONE ', 'CCMN_64_condcmp_imm ', ' '];
T[' CCMP REG=$Wn IMM:i32=$imm IMM:u8=$nzcv COND=$cond ', 'NONE ', 'CCMP_32_condcmp_imm ', ' '];
T[' CCMP REG=$Xn IMM:i64=$imm IMM:u8=$nzcv COND=$cond ', 'NONE ', 'CCMP_64_condcmp_imm ', ' '];



# condcmp_reg: Conditional compare (register).
# A64: sf:u=0bx op:u=0bx S:u=0bx ig0=0b11010010 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0bx Rn:u=0bxxxxx o3:u=0bx nzcv:u=0bxxxx
HINT('ICLASS', 'condcmp_reg', 'A64');

DEF Wm   => GPR32('Wm', 'Rm', 'read')             ;
DEF Wn   => GPR32('Wn', 'Rn', 'read')             ;
DEF Xm   => GPR64('Xm', 'Rm', 'read')             ;
DEF Xn   => GPR64('Xn', 'Rn', 'read')             ;
DEF cond => COND('cond', 'cond')                  ;
DEF nzcv => IMM('nzcv', 'nzcv', range => (0, 15)) ;

T[' CCMN REG=$Wn REG=$Wm IMM:u8=$nzcv COND=$cond ', 'NONE ', 'CCMN_32_condcmp_reg ', ' '];
T[' CCMN REG=$Xn REG=$Xm IMM:u8=$nzcv COND=$cond ', 'NONE ', 'CCMN_64_condcmp_reg ', ' '];
T[' CCMP REG=$Wn REG=$Wm IMM:u8=$nzcv COND=$cond ', 'NONE ', 'CCMP_32_condcmp_reg ', ' '];
T[' CCMP REG=$Xn REG=$Xm IMM:u8=$nzcv COND=$cond ', 'NONE ', 'CCMP_64_condcmp_reg ', ' '];



# condsel: Conditional select.
# A64: sf:u=0bx op:u=0bx S:u=0bx ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx op2:u=0bxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'condsel', 'A64');

DEF Wd     => GPR32('Wd', 'Rd', 'write')         ;
DEF Wm     => GPR32('Wm', 'Rm', 'read')          ;
DEF Wn     => GPR32('Wn', 'Rn', 'read')          ;
DEF Wn2    => GPR32('Wn', 'Rm:Rn', 'read')       ;
DEF Wnnwzr => GPR32('Wn', 'Rm:Rn', 'read|nowzr') ;
DEF Xd     => GPR64('Xd', 'Rd', 'write')         ;
DEF Xm     => GPR64('Xm', 'Rm', 'read')          ;
DEF Xn     => GPR64('Xn', 'Rn', 'read')          ;
DEF Xn2    => GPR64('Xn', 'Rm:Rn', 'read')       ;
DEF Xnnxzr => GPR64('Xn', 'Rm:Rn', 'read|noxzr') ;
DEF cond   => COND('cond', 'cond')               ;

T[' CSEL  REG=$Wd REG=$Wn REG=$Wm COND=$cond ', 'NONE ', 'CSEL_32_condsel        ', ' '];
T[' CSEL  REG=$Xd REG=$Xn REG=$Xm COND=$cond ', 'NONE ', 'CSEL_64_condsel        ', ' '];
T[' CSINC REG=$Wd REG=$Wn REG=$Wm COND=$cond ', 'NONE ', 'CSINC_32_condsel       ', ' '];
T[' CSINC REG=$Xd REG=$Xn REG=$Xm COND=$cond ', 'NONE ', 'CSINC_64_condsel       ', ' '];
T[' CSINV REG=$Wd REG=$Wn REG=$Wm COND=$cond ', 'NONE ', 'CSINV_32_condsel       ', ' '];
T[' CSINV REG=$Xd REG=$Xn REG=$Xm COND=$cond ', 'NONE ', 'CSINV_64_condsel       ', ' '];
T[' CSNEG REG=$Wd REG=$Wn REG=$Wm COND=$cond ', 'NONE ', 'CSNEG_32_condsel       ', ' '];
T[' CSNEG REG=$Xd REG=$Xn REG=$Xm COND=$cond ', 'NONE ', 'CSNEG_64_condsel       ', ' '];
T[' CINC  REG=$Wd REG=$Wnnwzr INVCOND=$cond  ', 'NONE ', 'CINC_CSINC_32_condsel  ', ' '];
T[' CINC  REG=$Xd REG=$Xnnxzr INVCOND=$cond  ', 'NONE ', 'CINC_CSINC_64_condsel  ', ' '];
T[' CINV  REG=$Wd REG=$Wnnwzr INVCOND=$cond  ', 'NONE ', 'CINV_CSINV_32_condsel  ', ' '];
T[' CINV  REG=$Xd REG=$Xnnxzr INVCOND=$cond  ', 'NONE ', 'CINV_CSINV_64_condsel  ', ' '];
T[' CNEG  REG=$Wd REG=$Wn2 INVCOND=$cond     ', 'NONE ', 'CNEG_CSNEG_32_condsel  ', ' '];
T[' CNEG  REG=$Xd REG=$Xn2 INVCOND=$cond     ', 'NONE ', 'CNEG_CSNEG_64_condsel  ', ' '];
T[' CSET  REG=$Wd INVCOND=$cond              ', 'NONE ', 'CSET_CSINC_32_condsel  ', ' '];
T[' CSET  REG=$Xd INVCOND=$cond              ', 'NONE ', 'CSET_CSINC_64_condsel  ', ' '];
T[' CSETM REG=$Wd INVCOND=$cond              ', 'NONE ', 'CSETM_CSINV_32_condsel ', ' '];
T[' CSETM REG=$Xd INVCOND=$cond              ', 'NONE ', 'CSETM_CSINV_64_condsel ', ' '];



# dp_1src: Data-processing (1 source).
# A64: sf:u=0bx ig0=0b1 S:u=0bx ig1=0b11010110 opcode2:u=0bxxxxx opcode:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'dp_1src', 'A64');

DEF Wd   => GPR32('Wd', 'Rd', 'write')   ;
DEF Wn   => GPR32('Wn', 'Rn', 'read')    ;
DEF Xd   => GPR64('Xd', 'Rd', 'write')   ;
DEF Xn   => GPR64('Xn', 'Rn', 'read')    ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP') ;

T[' AUTDA  REG=$Xd REG=$XnSP ', 'NONE ', 'AUTDA_64P_dp_1src    ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' AUTDZA REG=$Xd           ', 'NONE ', 'AUTDZA_64Z_dp_1src   ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' AUTDB  REG=$Xd REG=$XnSP ', 'NONE ', 'AUTDB_64P_dp_1src    ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' AUTDZB REG=$Xd           ', 'NONE ', 'AUTDZB_64Z_dp_1src   ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' AUTIA  REG=$Xd REG=$XnSP ', 'NONE ', 'AUTIA_64P_dp_1src    ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' AUTIZA REG=$Xd           ', 'NONE ', 'AUTIZA_64Z_dp_1src   ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' AUTIB  REG=$Xd REG=$XnSP ', 'NONE ', 'AUTIB_64P_dp_1src    ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' AUTIZB REG=$Xd           ', 'NONE ', 'AUTIZB_64Z_dp_1src   ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' CLS    REG=$Wd REG=$Wn   ', 'NONE ', 'CLS_32_dp_1src       ', '                                    '];
T[' CLS    REG=$Xd REG=$Xn   ', 'NONE ', 'CLS_64_dp_1src       ', '                                    '];
T[' CLZ    REG=$Wd REG=$Wn   ', 'NONE ', 'CLZ_32_dp_1src       ', '                                    '];
T[' CLZ    REG=$Xd REG=$Xn   ', 'NONE ', 'CLZ_64_dp_1src       ', '                                    '];
T[' PACDA  REG=$Xd REG=$XnSP ', 'NONE ', 'PACDA_64P_dp_1src    ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' PACDZA REG=$Xd           ', 'NONE ', 'PACDZA_64Z_dp_1src   ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' PACDB  REG=$Xd REG=$XnSP ', 'NONE ', 'PACDB_64P_dp_1src    ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' PACDZB REG=$Xd           ', 'NONE ', 'PACDZB_64Z_dp_1src   ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' PACIA  REG=$Xd REG=$XnSP ', 'NONE ', 'PACIA_64P_dp_1src    ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' PACIZA REG=$Xd           ', 'NONE ', 'PACIZA_64Z_dp_1src   ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' PACIB  REG=$Xd REG=$XnSP ', 'NONE ', 'PACIB_64P_dp_1src    ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' PACIZB REG=$Xd           ', 'NONE ', 'PACIZB_64Z_dp_1src   ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' RBIT   REG=$Wd REG=$Wn   ', 'NONE ', 'RBIT_32_dp_1src      ', '                                    '];
T[' RBIT   REG=$Xd REG=$Xn   ', 'NONE ', 'RBIT_64_dp_1src      ', '                                    '];
T[' REV    REG=$Wd REG=$Wn   ', 'NONE ', 'REV_32_dp_1src       ', '                                    '];
T[' REV    REG=$Xd REG=$Xn   ', 'NONE ', 'REV_64_dp_1src       ', '                                    '];
T[' REV16  REG=$Wd REG=$Wn   ', 'NONE ', 'REV16_32_dp_1src     ', '                                    '];
T[' REV16  REG=$Xd REG=$Xn   ', 'NONE ', 'REV16_64_dp_1src     ', '                                    '];
T[' REV32  REG=$Xd REG=$Xn   ', 'NONE ', 'REV32_64_dp_1src     ', '                                    '];
T[' XPACD  REG=$Xd           ', 'NONE ', 'XPACD_64Z_dp_1src    ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' XPACI  REG=$Xd           ', 'NONE ', 'XPACI_64Z_dp_1src    ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' REV64  REG=$Xd REG=$Xn   ', 'NONE ', 'REV64_REV_64_dp_1src ', '                                    '];



# dp_2src: Data-processing (2 source).
# A64: sf:u=0bx ig0=0b0 S:u=0bx ig1=0b11010110 Rm:u=0bxxxxx opcode:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'dp_2src', 'A64');

DEF Wd    => GPR32('Wd', 'Rd', 'write')                  ;
DEF Wm    => GPR32('Wm', 'Rm', 'read')                   ;
DEF Wn    => GPR32('Wn', 'Rn', 'read')                   ;
DEF Xd    => GPR64('Xd', 'Rd', 'write')                  ;
DEF Xd2   => GPR64('Xd', 'Xd', 'write')                  ;
DEF XdSP  => GPR64('Xd', 'Xd', 'write|SP')               ;
DEF Xm    => GPR64('Xm', 'Rm', 'read')                   ;
DEF Xm2   => GPR64('Xm', 'Xm', 'read')                   ;
DEF Xm3   => GPR64('Xm', 'Xm', default => 'XZR', 'read') ;
DEF XmSP  => GPR64('Xm', 'Rm', 'read|SP')                ;
DEF XmSP2 => GPR64('Xm', 'Xm', 'read|SP')                ;
DEF Xn    => GPR64('Xn', 'Rn', 'read')                   ;
DEF XnSP  => GPR64('Xn', 'Xn', 'read|SP')                ;

T[' ASRV    REG=$Wd REG=$Wn REG=$Wm             ', 'NONE ', 'ASRV_32_dp_2src        ', '                                    '];
T[' ASRV    REG=$Xd REG=$Xn REG=$Xm             ', 'NONE ', 'ASRV_64_dp_2src        ', '                                    '];
T[' CRC32B  REG=$Wd REG=$Wn REG=$Wm             ', 'NONE ', 'CRC32B_32C_dp_2src     ', 'arch_variant=ARMv8v1 extensions=CRC '];
T[' CRC32H  REG=$Wd REG=$Wn REG=$Wm             ', 'NONE ', 'CRC32H_32C_dp_2src     ', 'arch_variant=ARMv8v1 extensions=CRC '];
T[' CRC32W  REG=$Wd REG=$Wn REG=$Wm             ', 'NONE ', 'CRC32W_32C_dp_2src     ', 'arch_variant=ARMv8v1 extensions=CRC '];
T[' CRC32X  REG=$Wd REG=$Wn REG=$Xm             ', 'NONE ', 'CRC32X_64C_dp_2src     ', 'arch_variant=ARMv8v1 extensions=CRC '];
T[' CRC32CB REG=$Wd REG=$Wn REG=$Wm             ', 'NONE ', 'CRC32CB_32C_dp_2src    ', 'arch_variant=ARMv8v1 extensions=CRC '];
T[' CRC32CH REG=$Wd REG=$Wn REG=$Wm             ', 'NONE ', 'CRC32CH_32C_dp_2src    ', 'arch_variant=ARMv8v1 extensions=CRC '];
T[' CRC32CW REG=$Wd REG=$Wn REG=$Wm             ', 'NONE ', 'CRC32CW_32C_dp_2src    ', 'arch_variant=ARMv8v1 extensions=CRC '];
T[' CRC32CX REG=$Wd REG=$Wn REG=$Xm             ', 'NONE ', 'CRC32CX_64C_dp_2src    ', 'arch_variant=ARMv8v1 extensions=CRC '];
T[' GMI     REG=$Xd2 REG=$XnSP REG=$Xm2         ', 'NONE ', 'GMI_64G_dp_2src        ', 'arch_variant=ARMv8v5                '];
T[' IRG     REG=$XdSP REG=$XnSP REG:OPT=$Xm3    ', 'NONE ', 'IRG_64I_dp_2src        ', 'arch_variant=ARMv8v5                '];
T[' LSLV    REG=$Wd REG=$Wn REG=$Wm             ', 'NONE ', 'LSLV_32_dp_2src        ', '                                    '];
T[' LSLV    REG=$Xd REG=$Xn REG=$Xm             ', 'NONE ', 'LSLV_64_dp_2src        ', '                                    '];
T[' LSRV    REG=$Wd REG=$Wn REG=$Wm             ', 'NONE ', 'LSRV_32_dp_2src        ', '                                    '];
T[' LSRV    REG=$Xd REG=$Xn REG=$Xm             ', 'NONE ', 'LSRV_64_dp_2src        ', '                                    '];
T[' PACGA   REG=$Xd REG=$Xn REG=$XmSP           ', 'NONE ', 'PACGA_64P_dp_2src      ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' RORV    REG=$Wd REG=$Wn REG=$Wm             ', 'NONE ', 'RORV_32_dp_2src        ', '                                    '];
T[' RORV    REG=$Xd REG=$Xn REG=$Xm             ', 'NONE ', 'RORV_64_dp_2src        ', '                                    '];
T[' SDIV    REG:s32=$Wd REG:s32=$Wn REG:s32=$Wm ', 'NONE ', 'SDIV_32_dp_2src        ', '                                    '];
T[' SDIV    REG:s64=$Xd REG:s64=$Xn REG:s64=$Xm ', 'NONE ', 'SDIV_64_dp_2src        ', '                                    '];
T[' SUBP    REG=$Xd2 REG=$XnSP REG=$XmSP2       ', 'NONE ', 'SUBP_64S_dp_2src       ', 'arch_variant=ARMv8v5                '];
T[' SUBPS   REG=$Xd2 REG=$XnSP REG=$XmSP2       ', 'NONE ', 'SUBPS_64S_dp_2src      ', 'arch_variant=ARMv8v5                '];
T[' UDIV    REG:u32=$Wd REG:u32=$Wn REG:u32=$Wm ', 'NONE ', 'UDIV_32_dp_2src        ', '                                    '];
T[' UDIV    REG:u64=$Xd REG:u64=$Xn REG:u64=$Xm ', 'NONE ', 'UDIV_64_dp_2src        ', '                                    '];
T[' ASR     REG=$Wd REG=$Wn REG=$Wm             ', 'NONE ', 'ASR_ASRV_32_dp_2src    ', '                                    '];
T[' ASR     REG=$Xd REG=$Xn REG=$Xm             ', 'NONE ', 'ASR_ASRV_64_dp_2src    ', '                                    '];
T[' CMPP    REG=$XnSP REG=$XmSP2                ', 'NONE ', 'CMPP_SUBPS_64S_dp_2src ', 'arch_variant=ARMv8v5                '];
T[' LSL     REG=$Wd REG=$Wn REG=$Wm             ', 'NONE ', 'LSL_LSLV_32_dp_2src    ', '                                    '];
T[' LSL     REG=$Xd REG=$Xn REG=$Xm             ', 'NONE ', 'LSL_LSLV_64_dp_2src    ', '                                    '];
T[' LSR     REG=$Wd REG=$Wn REG=$Wm             ', 'NONE ', 'LSR_LSRV_32_dp_2src    ', '                                    '];
T[' LSR     REG=$Xd REG=$Xn REG=$Xm             ', 'NONE ', 'LSR_LSRV_64_dp_2src    ', '                                    '];
T[' ROR     REG=$Wd REG=$Wn REG=$Wm             ', 'NONE ', 'ROR_RORV_32_dp_2src    ', '                                    '];
T[' ROR     REG=$Xd REG=$Xn REG=$Xm             ', 'NONE ', 'ROR_RORV_64_dp_2src    ', '                                    '];



# dp_3src: Data-processing (3 source).
# A64: sf:u=0bx op54:u=0bxx ig0=0b11011 op31:u=0bxxx Rm:u=0bxxxxx o0:u=0bx Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'dp_3src', 'A64');

DEF Wa => GPR32('Wa', 'Ra', 'read')  ;
DEF Wd => GPR32('Wd', 'Rd', 'write') ;
DEF Wm => GPR32('Wm', 'Rm', 'read')  ;
DEF Wn => GPR32('Wn', 'Rn', 'read')  ;
DEF Xa => GPR64('Xa', 'Ra', 'read')  ;
DEF Xd => GPR64('Xd', 'Rd', 'write') ;
DEF Xm => GPR64('Xm', 'Rm', 'read')  ;
DEF Xn => GPR64('Xn', 'Rn', 'read')  ;

T[' MADD   REG=$Wd REG=$Wn REG=$Wm REG=$Wa                 ', 'NONE ', 'MADD_32A_dp_3src           ', ' '];
T[' MADD   REG=$Xd REG=$Xn REG=$Xm REG=$Xa                 ', 'NONE ', 'MADD_64A_dp_3src           ', ' '];
T[' MSUB   REG=$Wd REG=$Wn REG=$Wm REG=$Wa                 ', 'NONE ', 'MSUB_32A_dp_3src           ', ' '];
T[' MSUB   REG=$Xd REG=$Xn REG=$Xm REG=$Xa                 ', 'NONE ', 'MSUB_64A_dp_3src           ', ' '];
T[' SMADDL REG:s64=$Xd REG:s32=$Wn REG:s32=$Wm REG:s64=$Xa ', 'NONE ', 'SMADDL_64WA_dp_3src        ', ' '];
T[' SMSUBL REG:s64=$Xd REG:s32=$Wn REG:s32=$Wm REG:s64=$Xa ', 'NONE ', 'SMSUBL_64WA_dp_3src        ', ' '];
T[' SMULH  REG:s64=$Xd REG:s64=$Xn REG:s64=$Xm             ', 'NONE ', 'SMULH_64_dp_3src           ', ' '];
T[' UMADDL REG:u64=$Xd REG:u32=$Wn REG:u32=$Wm REG:u64=$Xa ', 'NONE ', 'UMADDL_64WA_dp_3src        ', ' '];
T[' UMSUBL REG:u64=$Xd REG:u32=$Wn REG:u32=$Wm REG:u64=$Xa ', 'NONE ', 'UMSUBL_64WA_dp_3src        ', ' '];
T[' UMULH  REG:u64=$Xd REG:u64=$Xn REG:u64=$Xm             ', 'NONE ', 'UMULH_64_dp_3src           ', ' '];
T[' MNEG   REG=$Wd REG=$Wn REG=$Wm                         ', 'NONE ', 'MNEG_MSUB_32A_dp_3src      ', ' '];
T[' MNEG   REG=$Xd REG=$Xn REG=$Xm                         ', 'NONE ', 'MNEG_MSUB_64A_dp_3src      ', ' '];
T[' MUL    REG=$Wd REG=$Wn REG=$Wm                         ', 'NONE ', 'MUL_MADD_32A_dp_3src       ', ' '];
T[' MUL    REG=$Xd REG=$Xn REG=$Xm                         ', 'NONE ', 'MUL_MADD_64A_dp_3src       ', ' '];
T[' SMNEGL REG:s64=$Xd REG:s32=$Wn REG:s32=$Wm             ', 'NONE ', 'SMNEGL_SMSUBL_64WA_dp_3src ', ' '];
T[' SMULL  REG:s64=$Xd REG:s32=$Wn REG:s32=$Wm             ', 'NONE ', 'SMULL_SMADDL_64WA_dp_3src  ', ' '];
T[' UMNEGL REG:u64=$Xd REG:u32=$Wn REG:u32=$Wm             ', 'NONE ', 'UMNEGL_UMSUBL_64WA_dp_3src ', ' '];
T[' UMULL  REG:u64=$Xd REG:u32=$Wn REG:u32=$Wm             ', 'NONE ', 'UMULL_UMADDL_64WA_dp_3src  ', ' '];



# exception: Exception generation.
# A64: ig0=0b11010100 opc:u=0bxxx imm16:u=0bxxxxxxxxxxxxxxxx op2:u=0bxxx LL:u=0bxx
HINT('ICLASS', 'exception', 'A64');

DEF imm  => IMM('imm', 'imm16', range => (0, 65535))               ;
DEF imm2 => IMM('imm', 'imm16', range => (0, 65535), default => 0) ;

T[' BRK     IMM:u16=$imm      ', 'NONE ', 'BRK_EX_exception     ', 'arch_variant=ARMv8v5 extensions=BTI '];
T[' DCPS1   IMM:OPT:u16=$imm2 ', 'NONE ', 'DCPS1_DC_exception   ', '                                    '];
T[' DCPS2   IMM:OPT:u16=$imm2 ', 'NONE ', 'DCPS2_DC_exception   ', '                                    '];
T[' DCPS3   IMM:OPT:u16=$imm2 ', 'NONE ', 'DCPS3_DC_exception   ', 'arch_variant=ARMv8v0-A              '];
T[' HLT     IMM:u16=$imm      ', 'NONE ', 'HLT_EX_exception     ', 'arch_variant=ARMv8v5 extensions=BTI '];
T[' HVC     IMM:u16=$imm      ', 'NONE ', 'HVC_EX_exception     ', '                                    '];
T[' SMC     IMM:u16=$imm      ', 'NONE ', 'SMC_EX_exception     ', 'arch_variant=ARMv8v0-A              '];
T[' SVC     IMM:u16=$imm      ', 'NONE ', 'SVC_EX_exception     ', '                                    '];
T[' TCANCEL IMM:u16=$imm      ', 'NONE ', 'TCANCEL_EX_exception ', 'arch_variant=TME                    '];



# extract: Extract.
# A64: sf:u=0bx op21:u=0bxx ig0=0b100111 N:u=0bx o0:u=0bx Rm:u=0bxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'extract', 'A64');

DEF Wd     => GPR32('Wd', 'Rd', 'write')             ;
DEF Wm     => GPR32('Wm', 'Rm', 'read')              ;
DEF Wn     => GPR32('Wn', 'Rn', 'read')              ;
DEF Ws     => GPR32('Ws', 'Rm:Rn')                   ;
DEF Xd     => GPR64('Xd', 'Rd', 'write')             ;
DEF Xm     => GPR64('Xm', 'Rm', 'read')              ;
DEF Xn     => GPR64('Xn', 'Rn', 'read')              ;
DEF Xs     => GPR64('Xs', 'Rm:Rn')                   ;
DEF lsb    => IMM('lsb', 'imms', range => (0, 31))   ;
DEF lsb2   => IMM('lsb', 'imms', range => (0, 63))   ;
DEF shift0 => IMM('shift', 'imms', range => (0, 31)) ;
DEF shift2 => IMM('shift', 'imms', range => (0, 63)) ;

T[' EXTR REG=$Wd REG=$Wn REG=$Wm IMM:u8=$lsb  ', 'NONE ', 'EXTR_32_extract     ', ' '];
T[' EXTR REG=$Xd REG=$Xn REG=$Xm IMM:u8=$lsb2 ', 'NONE ', 'EXTR_64_extract     ', ' '];
T[' ROR  REG=$Wd REG=$Ws IMM:u8=$shift0       ', 'NONE ', 'ROR_EXTR_32_extract ', ' '];
T[' ROR  REG=$Xd REG=$Xs IMM:u8=$shift2       ', 'NONE ', 'ROR_EXTR_64_extract ', ' '];



# hints: Hints.
# A64: ig0=0b11010101000000110010 CRm:u=0bxxxx op2:u=0bxxx ig1=0b11111
HINT('ICLASS', 'hints', 'A64');

DEF imm     => IMM('imm', 'CRm:op2', range => (0, 127))             ;
DEF targets => BTI_OP('targets', 'op2<2:1>', table => 'tbl_bti_op') ;

T[' AUTIA1716                     ', 'NONE ', 'AUTIA1716_HI_hints ', 'arch_variant=ARMv8v3                    '];
T[' AUTIASP                       ', 'NONE ', 'AUTIASP_HI_hints   ', 'arch_variant=ARMv8v3                    '];
T[' AUTIAZ                        ', 'NONE ', 'AUTIAZ_HI_hints    ', 'arch_variant=ARMv8v3                    '];
T[' AUTIB1716                     ', 'NONE ', 'AUTIB1716_HI_hints ', 'arch_variant=ARMv8v3                    '];
T[' AUTIBSP                       ', 'NONE ', 'AUTIBSP_HI_hints   ', 'arch_variant=ARMv8v3                    '];
T[' AUTIBZ                        ', 'NONE ', 'AUTIBZ_HI_hints    ', 'arch_variant=ARMv8v3                    '];
T[' BTI       BTI_OP:OPT=$targets ', 'NONE ', 'BTI_HB_hints       ', 'arch_variant=ARMv8v5                    '];
T[' CSDB                          ', 'NONE ', 'CSDB_HI_hints      ', '                                        '];
T[' DGH                           ', 'NONE ', 'DGH_HI_hints       ', 'arch_variant=ARMv8v6 extensions=DGH     '];
T[' ESB                           ', 'NONE ', 'ESB_HI_hints       ', 'arch_variant=ARMv8v2 extensions=RAS     '];
T[' HINT      IMM:u8=$imm         ', 'NONE ', 'HINT_HM_hints      ', 'arch_variant=ARMv8v2 extensions=DGH|RAS '];
T[' NOP                           ', 'NONE ', 'NOP_HI_hints       ', '                                        '];
T[' PACIA1716                     ', 'NONE ', 'PACIA1716_HI_hints ', 'arch_variant=ARMv8v3 extensions=BTI     '];
T[' PACIASP                       ', 'NONE ', 'PACIASP_HI_hints   ', 'arch_variant=ARMv8v3 extensions=BTI     '];
T[' PACIAZ                        ', 'NONE ', 'PACIAZ_HI_hints    ', 'arch_variant=ARMv8v3 extensions=BTI     '];
T[' PACIB1716                     ', 'NONE ', 'PACIB1716_HI_hints ', 'arch_variant=ARMv8v3 extensions=BTI     '];
T[' PACIBSP                       ', 'NONE ', 'PACIBSP_HI_hints   ', 'arch_variant=ARMv8v3 extensions=BTI     '];
T[' PACIBZ                        ', 'NONE ', 'PACIBZ_HI_hints    ', 'arch_variant=ARMv8v3 extensions=BTI     '];
T[' PSB       SYNC_OP=CSYNC       ', 'NONE ', 'PSB_HC_hints       ', 'arch_variant=ARMv8v2                    '];
T[' SEV                           ', 'NONE ', 'SEV_HI_hints       ', '                                        '];
T[' SEVL                          ', 'NONE ', 'SEVL_HI_hints      ', '                                        '];
T[' TSB       SYNC_OP=CSYNC       ', 'NONE ', 'TSB_HC_hints       ', 'arch_variant=ARMv8v4                    '];
T[' WFE                           ', 'NONE ', 'WFE_HI_hints       ', '                                        '];
T[' WFI                           ', 'NONE ', 'WFI_HI_hints       ', '                                        '];
T[' XPACLRI                       ', 'NONE ', 'XPACLRI_HI_hints   ', 'arch_variant=ARMv8v3                    '];
T[' YIELD                         ', 'NONE ', 'YIELD_HI_hints     ', '                                        '];



# ldapstl_unscaled: LDAPR/STLR (unscaled immediate).
# A64: size:u=0bxx ig0=0b011001 opc:u=0bxx ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'ldapstl_unscaled', 'A64');

DEF Wt   => GPR32('Wt', 'Rt')                                                             ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                                                  ;
DEF Xt   => GPR64('Xt', 'Rt')                                                             ;
DEF simm => IMM('simm', 'imm9', signpos => 'imm9<8>', range => (-256, 255), default => 0) ;

T[' LDAPUR   REG:w=$Wt MEM:OFF:r:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:s16=$simm LDACC=ACQUIRE}     ', 'NONE ', 'LDAPUR_32_ldapstl_unscaled   ', 'arch_variant=ARMv8v4 '];
T[' LDAPUR   REG:w=$Xt MEM:OFF:r:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:s16=$simm LDACC=ACQUIRE}     ', 'NONE ', 'LDAPUR_64_ldapstl_unscaled   ', 'arch_variant=ARMv8v4 '];
T[' LDAPURB  REG:w=$Wt MEM:OFF:r:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:s16=$simm LDACC=ACQUIRE}     ', 'NONE ', 'LDAPURB_32_ldapstl_unscaled  ', 'arch_variant=ARMv8v4 '];
T[' LDAPURH  REG:w=$Wt MEM:OFF:r:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:s16=$simm LDACC=ACQUIRE}     ', 'NONE ', 'LDAPURH_32_ldapstl_unscaled  ', 'arch_variant=ARMv8v4 '];
T[' LDAPURSB REG:w:s32=$Wt MEM:OFF:r:s8={ BASE=$XnSP SZ=8  IMMOFF:OPT:s16=$simm LDACC=ACQUIRE} ', 'NONE ', 'LDAPURSB_32_ldapstl_unscaled ', 'arch_variant=ARMv8v4 '];
T[' LDAPURSB REG:w:s64=$Xt MEM:OFF:r:s8={ BASE=$XnSP SZ=8  IMMOFF:OPT:s16=$simm LDACC=ACQUIRE} ', 'NONE ', 'LDAPURSB_64_ldapstl_unscaled ', 'arch_variant=ARMv8v4 '];
T[' LDAPURSH REG:w:s32=$Wt MEM:OFF:r:s16={BASE=$XnSP SZ=16 IMMOFF:OPT:s16=$simm LDACC=ACQUIRE} ', 'NONE ', 'LDAPURSH_32_ldapstl_unscaled ', 'arch_variant=ARMv8v4 '];
T[' LDAPURSH REG:w:s64=$Xt MEM:OFF:r:s16={BASE=$XnSP SZ=16 IMMOFF:OPT:s16=$simm LDACC=ACQUIRE} ', 'NONE ', 'LDAPURSH_64_ldapstl_unscaled ', 'arch_variant=ARMv8v4 '];
T[' LDAPURSW REG:w:s64=$Xt MEM:OFF:r:s32={BASE=$XnSP SZ=32 IMMOFF:OPT:s16=$simm LDACC=ACQUIRE} ', 'NONE ', 'LDAPURSW_64_ldapstl_unscaled ', 'arch_variant=ARMv8v4 '];
T[' STLUR    REG:r=$Wt MEM:OFF:w:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:s16=$simm STACC=RELEASE}     ', 'NONE ', 'STLUR_32_ldapstl_unscaled    ', 'arch_variant=ARMv8v4 '];
T[' STLUR    REG:r=$Xt MEM:OFF:w:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:s16=$simm STACC=RELEASE}     ', 'NONE ', 'STLUR_64_ldapstl_unscaled    ', 'arch_variant=ARMv8v4 '];
T[' STLURB   REG:r=$Wt MEM:OFF:w:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:s16=$simm STACC=RELEASE}     ', 'NONE ', 'STLURB_32_ldapstl_unscaled   ', 'arch_variant=ARMv8v4 '];
T[' STLURH   REG:r=$Wt MEM:OFF:w:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:s16=$simm STACC=RELEASE}     ', 'NONE ', 'STLURH_32_ldapstl_unscaled   ', 'arch_variant=ARMv8v4 '];



# ldst_pac: Load/store register (pac).
# A64: size:u=0bxx ig0=0b111 V:u=0bx ig1=0b00 M:u=0bx S:u=0bx ig2=0b1 imm9:u=0bxxxxxxxxx W:u=0bx ig3=0b1 Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'ldst_pac', 'A64');

DEF XnSP   => GPR64('Xn', 'Rn', 'read|SP')                                                            ;
DEF Xt     => GPR64('Xt', 'Rt')                                                                       ;
DEF simmx8 => IMM('simm', 'S:imm9', scale => 8, signpos => 'S', range => (-4096, 4088), default => 0) ;

T[' LDRAA REG:w=$Xt MEM:OFF:r:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:s16=$simmx8} ', 'NONE ', 'LDRAA_64_ldst_pac  ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' LDRAA REG:w=$Xt MEM:PRE:r:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:s16=$simmx8} ', 'NONE ', 'LDRAA_64W_ldst_pac ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' LDRAB REG:w=$Xt MEM:OFF:r:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:s16=$simmx8} ', 'NONE ', 'LDRAB_64_ldst_pac  ', 'arch_variant=ARMv8v3 extensions=PAC '];
T[' LDRAB REG:w=$Xt MEM:PRE:r:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:s16=$simmx8} ', 'NONE ', 'LDRAB_64W_ldst_pac ', 'arch_variant=ARMv8v3 extensions=PAC '];



# ldst_unpriv: Load/store register (unprivileged).
# A64: size:u=0bxx ig0=0b111 V:u=0bx ig1=0b00 opc:u=0bxx ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'ldst_unpriv', 'A64');

DEF Wt   => GPR32('Wt', 'Rt')                                                             ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP')                                                  ;
DEF Xt   => GPR64('Xt', 'Rt')                                                             ;
DEF simm => IMM('simm', 'imm9', signpos => 'imm9<8>', range => (-256, 255), default => 0) ;

T[' LDTR   REG:w=$Wt MEM:OFF:r:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:s16=$simm LDACC=UNPRIVILEGED}     ', 'NONE ', 'LDTR_32_ldst_unpriv   ', ' '];
T[' LDTR   REG:w=$Xt MEM:OFF:r:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:s16=$simm LDACC=UNPRIVILEGED}     ', 'NONE ', 'LDTR_64_ldst_unpriv   ', ' '];
T[' LDTRB  REG:w=$Wt MEM:OFF:r:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:s16=$simm LDACC=UNPRIVILEGED}     ', 'NONE ', 'LDTRB_32_ldst_unpriv  ', ' '];
T[' LDTRH  REG:w=$Wt MEM:OFF:r:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:s16=$simm LDACC=UNPRIVILEGED}     ', 'NONE ', 'LDTRH_32_ldst_unpriv  ', ' '];
T[' LDTRSB REG:w:s32=$Wt MEM:OFF:r:s8={ BASE=$XnSP SZ=8  IMMOFF:OPT:s16=$simm LDACC=UNPRIVILEGED} ', 'NONE ', 'LDTRSB_32_ldst_unpriv ', ' '];
T[' LDTRSB REG:w:s64=$Xt MEM:OFF:r:s8={ BASE=$XnSP SZ=8  IMMOFF:OPT:s16=$simm LDACC=UNPRIVILEGED} ', 'NONE ', 'LDTRSB_64_ldst_unpriv ', ' '];
T[' LDTRSH REG:w:s32=$Wt MEM:OFF:r:s16={BASE=$XnSP SZ=16 IMMOFF:OPT:s16=$simm LDACC=UNPRIVILEGED} ', 'NONE ', 'LDTRSH_32_ldst_unpriv ', ' '];
T[' LDTRSH REG:w:s64=$Xt MEM:OFF:r:s16={BASE=$XnSP SZ=16 IMMOFF:OPT:s16=$simm LDACC=UNPRIVILEGED} ', 'NONE ', 'LDTRSH_64_ldst_unpriv ', ' '];
T[' LDTRSW REG:w:s64=$Xt MEM:OFF:r:s32={BASE=$XnSP SZ=32 IMMOFF:OPT:s16=$simm LDACC=UNPRIVILEGED} ', 'NONE ', 'LDTRSW_64_ldst_unpriv ', ' '];
T[' STTR   REG:r=$Wt MEM:OFF:w:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:s16=$simm STACC=UNPRIVILEGED}     ', 'NONE ', 'STTR_32_ldst_unpriv   ', ' '];
T[' STTR   REG:r=$Xt MEM:OFF:w:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:s16=$simm STACC=UNPRIVILEGED}     ', 'NONE ', 'STTR_64_ldst_unpriv   ', ' '];
T[' STTRB  REG:r=$Wt MEM:OFF:w:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:s16=$simm STACC=UNPRIVILEGED}     ', 'NONE ', 'STTRB_32_ldst_unpriv  ', ' '];
T[' STTRH  REG:r=$Wt MEM:OFF:w:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:s16=$simm STACC=UNPRIVILEGED}     ', 'NONE ', 'STTRH_32_ldst_unpriv  ', ' '];



# ldstexcl: Load/store exclusive.
# A64: size:u=0bxx ig0=0b001000 o2:u=0bx L:u=0bx o1:u=0bx Rs:u=0bxxxxx o0:u=0bx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'ldstexcl', 'A64');

DEF Ws     => GPR32('Ws', 'Rs')             ;
DEF Ws2    => GPR32('Ws', 'Rs', 'even')     ;
DEF WsPair => GPR32('WsPair', 'Rs', 'pair') ;
DEF Wt     => GPR32('Wt', 'Rt')             ;
DEF Wt2    => GPR32('Wt', 'Rt', 'even')     ;
DEF Wt3    => GPR32('Wt1', 'Rt')            ;
DEF Wt4    => GPR32('Wt2', 'Rt2')           ;
DEF WtPair => GPR32('WtPair', 'Rt', 'pair') ;
DEF XnSP   => GPR64('Xn', 'Rn', 'read|SP')  ;
DEF Xs     => GPR64('Xs', 'Rs')             ;
DEF Xs2    => GPR64('Xs', 'Rs', 'even')     ;
DEF XsPair => GPR64('XsPair', 'Rs', 'pair') ;
DEF Xt     => GPR64('Xt', 'Rt')             ;
DEF Xt2    => GPR64('Xt', 'Rt', 'even')     ;
DEF Xt3    => GPR64('Xt1', 'Rt')            ;
DEF Xt4    => GPR64('Xt2', 'Rt2')           ;
DEF XtPair => GPR64('XtPair', 'Rt', 'pair') ;

T[' CAS    REG:rw=$Ws REG:r=$Wt MEM:OFF:rw:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:u8=0 LDACC=ATOMIC STACC=ATOMIC}                                                  ', 'NONE ', 'CAS_C32_ldstexcl     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASA   REG:rw=$Ws REG:r=$Wt MEM:OFF:rw:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                                          ', 'NONE ', 'CASA_C32_ldstexcl    ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASAL  REG:rw=$Ws REG:r=$Wt MEM:OFF:rw:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}                                  ', 'NONE ', 'CASAL_C32_ldstexcl   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASL   REG:rw=$Ws REG:r=$Wt MEM:OFF:rw:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:u8=0 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                                          ', 'NONE ', 'CASL_C32_ldstexcl    ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CAS    REG:rw=$Xs REG:r=$Xt MEM:OFF:rw:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:u8=0 LDACC=ATOMIC STACC=ATOMIC}                                                  ', 'NONE ', 'CAS_C64_ldstexcl     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASA   REG:rw=$Xs REG:r=$Xt MEM:OFF:rw:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                                          ', 'NONE ', 'CASA_C64_ldstexcl    ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASAL  REG:rw=$Xs REG:r=$Xt MEM:OFF:rw:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}                                  ', 'NONE ', 'CASAL_C64_ldstexcl   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASL   REG:rw=$Xs REG:r=$Xt MEM:OFF:rw:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:u8=0 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                                          ', 'NONE ', 'CASL_C64_ldstexcl    ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASAB  REG:rw=$Ws REG:r=$Wt MEM:OFF:rw:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                                          ', 'NONE ', 'CASAB_C32_ldstexcl   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASALB REG:rw=$Ws REG:r=$Wt MEM:OFF:rw:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}                                  ', 'NONE ', 'CASALB_C32_ldstexcl  ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASB   REG:rw=$Ws REG:r=$Wt MEM:OFF:rw:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=0 LDACC=ATOMIC STACC=ATOMIC}                                                  ', 'NONE ', 'CASB_C32_ldstexcl    ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASLB  REG:rw=$Ws REG:r=$Wt MEM:OFF:rw:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=0 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                                          ', 'NONE ', 'CASLB_C32_ldstexcl   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASAH  REG:rw=$Ws REG:r=$Wt MEM:OFF:rw:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                                          ', 'NONE ', 'CASAH_C32_ldstexcl   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASALH REG:rw=$Ws REG:r=$Wt MEM:OFF:rw:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}                                  ', 'NONE ', 'CASALH_C32_ldstexcl  ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASH   REG:rw=$Ws REG:r=$Wt MEM:OFF:rw:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=0 LDACC=ATOMIC STACC=ATOMIC}                                                  ', 'NONE ', 'CASH_C32_ldstexcl    ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASLH  REG:rw=$Ws REG:r=$Wt MEM:OFF:rw:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=0 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                                          ', 'NONE ', 'CASLH_C32_ldstexcl   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASP   REG:rw=$Ws2 REG:rw=$WsPair REG:r=$Wt2 REG:r=$WtPair MEM:OFF:rw:i32={BASE=$XnSP SZ=2x32 IMMOFF:OPT:u8=0 LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'CASP_CP32_ldstexcl   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASPA  REG:rw=$Ws2 REG:rw=$WsPair REG:r=$Wt2 REG:r=$WtPair MEM:OFF:rw:i32={BASE=$XnSP SZ=2x32 IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'CASPA_CP32_ldstexcl  ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASPAL REG:rw=$Ws2 REG:rw=$WsPair REG:r=$Wt2 REG:r=$WtPair MEM:OFF:rw:i32={BASE=$XnSP SZ=2x32 IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'CASPAL_CP32_ldstexcl ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASPL  REG:rw=$Ws2 REG:rw=$WsPair REG:r=$Wt2 REG:r=$WtPair MEM:OFF:rw:i32={BASE=$XnSP SZ=2x32 IMMOFF:OPT:u8=0 LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'CASPL_CP32_ldstexcl  ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASP   REG:rw=$Xs2 REG:rw=$XsPair REG:r=$Xt2 REG:r=$XtPair MEM:OFF:rw:i64={BASE=$XnSP SZ=2x64 IMMOFF:OPT:u8=0 LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'CASP_CP64_ldstexcl   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASPA  REG:rw=$Xs2 REG:rw=$XsPair REG:r=$Xt2 REG:r=$XtPair MEM:OFF:rw:i64={BASE=$XnSP SZ=2x64 IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'CASPA_CP64_ldstexcl  ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASPAL REG:rw=$Xs2 REG:rw=$XsPair REG:r=$Xt2 REG:r=$XtPair MEM:OFF:rw:i64={BASE=$XnSP SZ=2x64 IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'CASPAL_CP64_ldstexcl ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' CASPL  REG:rw=$Xs2 REG:rw=$XsPair REG:r=$Xt2 REG:r=$XtPair MEM:OFF:rw:i64={BASE=$XnSP SZ=2x64 IMMOFF:OPT:u8=0 LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'CASPL_CP64_ldstexcl  ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDAR   REG:w=$Wt MEM:OFF:r:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:u8=0 LDACC=ACQUIRE}                                                                          ', 'NONE ', 'LDAR_LR32_ldstexcl   ', '                                       '];
T[' LDAR   REG:w=$Xt MEM:OFF:r:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:u8=0 LDACC=ACQUIRE}                                                                          ', 'NONE ', 'LDAR_LR64_ldstexcl   ', '                                       '];
T[' LDARB  REG:w=$Wt MEM:OFF:r:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=0 LDACC=ACQUIRE}                                                                          ', 'NONE ', 'LDARB_LR32_ldstexcl  ', '                                       '];
T[' LDARH  REG:w=$Wt MEM:OFF:r:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=0 LDACC=ACQUIRE}                                                                          ', 'NONE ', 'LDARH_LR32_ldstexcl  ', '                                       '];
T[' LDAXP  REG:w=$Wt3 REG:w=$Wt4 MEM:OFF:r:i32={BASE=$XnSP SZ=2x32 IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE}                                                     ', 'NONE ', 'LDAXP_LP32_ldstexcl  ', '                                       '];
T[' LDAXP  REG:w=$Xt3 REG:w=$Xt4 MEM:OFF:r:i64={BASE=$XnSP SZ=2x64 IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE}                                                     ', 'NONE ', 'LDAXP_LP64_ldstexcl  ', '                                       '];
T[' LDAXR  REG:w=$Wt MEM:OFF:r:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE}                                                                   ', 'NONE ', 'LDAXR_LR32_ldstexcl  ', '                                       '];
T[' LDAXR  REG:w=$Xt MEM:OFF:r:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE}                                                                   ', 'NONE ', 'LDAXR_LR64_ldstexcl  ', '                                       '];
T[' LDAXRB REG:w=$Wt MEM:OFF:r:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE}                                                                   ', 'NONE ', 'LDAXRB_LR32_ldstexcl ', '                                       '];
T[' LDAXRH REG:w=$Wt MEM:OFF:r:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=0 LDACC=ATOMIC_ACQUIRE}                                                                   ', 'NONE ', 'LDAXRH_LR32_ldstexcl ', '                                       '];
T[' LDLAR  REG:w=$Wt MEM:OFF:r:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:u8=0 LDACC=LOACQUIRE}                                                                        ', 'NONE ', 'LDLAR_LR32_ldstexcl  ', 'arch_variant=ARMv8v1                   '];
T[' LDLAR  REG:w=$Xt MEM:OFF:r:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:u8=0 LDACC=LOACQUIRE}                                                                        ', 'NONE ', 'LDLAR_LR64_ldstexcl  ', 'arch_variant=ARMv8v1                   '];
T[' LDLARB REG:w=$Wt MEM:OFF:r:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=0 LDACC=LOACQUIRE}                                                                        ', 'NONE ', 'LDLARB_LR32_ldstexcl ', 'arch_variant=ARMv8v1                   '];
T[' LDLARH REG:w=$Wt MEM:OFF:r:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=0 LDACC=LOACQUIRE}                                                                        ', 'NONE ', 'LDLARH_LR32_ldstexcl ', 'arch_variant=ARMv8v1                   '];
T[' LDXP   REG:w=$Wt3 REG:w=$Wt4 MEM:OFF:r:i32={BASE=$XnSP SZ=2x32 IMMOFF:OPT:u8=0 LDACC=ATOMIC}                                                             ', 'NONE ', 'LDXP_LP32_ldstexcl   ', '                                       '];
T[' LDXP   REG:w=$Xt3 REG:w=$Xt4 MEM:OFF:r:i64={BASE=$XnSP SZ=2x64 IMMOFF:OPT:u8=0 LDACC=ATOMIC}                                                             ', 'NONE ', 'LDXP_LP64_ldstexcl   ', '                                       '];
T[' LDXR   REG:w=$Wt MEM:OFF:r:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:u8=0 LDACC=ATOMIC}                                                                           ', 'NONE ', 'LDXR_LR32_ldstexcl   ', '                                       '];
T[' LDXR   REG:w=$Xt MEM:OFF:r:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:u8=0 LDACC=ATOMIC}                                                                           ', 'NONE ', 'LDXR_LR64_ldstexcl   ', '                                       '];
T[' LDXRB  REG:w=$Wt MEM:OFF:r:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=0 LDACC=ATOMIC}                                                                           ', 'NONE ', 'LDXRB_LR32_ldstexcl  ', '                                       '];
T[' LDXRH  REG:w=$Wt MEM:OFF:r:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=0 LDACC=ATOMIC}                                                                           ', 'NONE ', 'LDXRH_LR32_ldstexcl  ', '                                       '];
T[' STLLR  REG:r=$Wt MEM:OFF:w:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:u8=0 STACC=LORELEASE}                                                                        ', 'NONE ', 'STLLR_SL32_ldstexcl  ', 'arch_variant=ARMv8v1                   '];
T[' STLLR  REG:r=$Xt MEM:OFF:w:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:u8=0 STACC=LORELEASE}                                                                        ', 'NONE ', 'STLLR_SL64_ldstexcl  ', 'arch_variant=ARMv8v1                   '];
T[' STLLRB REG:r=$Wt MEM:OFF:w:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=0 STACC=LORELEASE}                                                                        ', 'NONE ', 'STLLRB_SL32_ldstexcl ', 'arch_variant=ARMv8v1                   '];
T[' STLLRH REG:r=$Wt MEM:OFF:w:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=0 STACC=LORELEASE}                                                                        ', 'NONE ', 'STLLRH_SL32_ldstexcl ', 'arch_variant=ARMv8v1                   '];
T[' STLR   REG:r=$Wt MEM:OFF:w:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:u8=0 STACC=RELEASE}                                                                          ', 'NONE ', 'STLR_SL32_ldstexcl   ', '                                       '];
T[' STLR   REG:r=$Xt MEM:OFF:w:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:u8=0 STACC=RELEASE}                                                                          ', 'NONE ', 'STLR_SL64_ldstexcl   ', '                                       '];
T[' STLRB  REG:r=$Wt MEM:OFF:w:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=0 STACC=RELEASE}                                                                          ', 'NONE ', 'STLRB_SL32_ldstexcl  ', '                                       '];
T[' STLRH  REG:r=$Wt MEM:OFF:w:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=0 STACC=RELEASE}                                                                          ', 'NONE ', 'STLRH_SL32_ldstexcl  ', '                                       '];
T[' STLXP  REG:w=$Ws REG:r=$Wt3 REG:r=$Wt4 MEM:OFF:w:i32={BASE=$XnSP SZ=2x32 IMMOFF:OPT:u8=0 STACC=ATOMIC_RELEASE}                                           ', 'NONE ', 'STLXP_SP32_ldstexcl  ', '                                       '];
T[' STLXP  REG:w=$Ws REG:r=$Xt3 REG:r=$Xt4 MEM:OFF:w:i64={BASE=$XnSP SZ=2x64 IMMOFF:OPT:u8=0 STACC=ATOMIC_RELEASE}                                           ', 'NONE ', 'STLXP_SP64_ldstexcl  ', '                                       '];
T[' STLXR  REG:w=$Ws REG:r=$Wt MEM:OFF:w:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:u8=0 STACC=ATOMIC_RELEASE}                                                         ', 'NONE ', 'STLXR_SR32_ldstexcl  ', '                                       '];
T[' STLXR  REG:w=$Ws REG:r=$Xt MEM:OFF:w:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:u8=0 STACC=ATOMIC_RELEASE}                                                         ', 'NONE ', 'STLXR_SR64_ldstexcl  ', '                                       '];
T[' STLXRB REG:w=$Ws REG:r=$Wt MEM:OFF:w:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=0 STACC=ATOMIC_RELEASE}                                                         ', 'NONE ', 'STLXRB_SR32_ldstexcl ', '                                       '];
T[' STLXRH REG:w=$Ws REG:r=$Wt MEM:OFF:w:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=0 STACC=ATOMIC_RELEASE}                                                         ', 'NONE ', 'STLXRH_SR32_ldstexcl ', '                                       '];
T[' STXP   REG:w=$Ws REG:r=$Wt3 REG:r=$Wt4 MEM:OFF:w:i32={BASE=$XnSP SZ=2x32 IMMOFF:OPT:u8=0 STACC=ATOMIC}                                                   ', 'NONE ', 'STXP_SP32_ldstexcl   ', '                                       '];
T[' STXP   REG:w=$Ws REG:r=$Xt3 REG:r=$Xt4 MEM:OFF:w:i64={BASE=$XnSP SZ=2x64 IMMOFF:OPT:u8=0 STACC=ATOMIC}                                                   ', 'NONE ', 'STXP_SP64_ldstexcl   ', '                                       '];
T[' STXR   REG:w=$Ws REG:r=$Wt MEM:OFF:w:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:u8=0 STACC=ATOMIC}                                                                 ', 'NONE ', 'STXR_SR32_ldstexcl   ', '                                       '];
T[' STXR   REG:w=$Ws REG:r=$Xt MEM:OFF:w:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:u8=0 STACC=ATOMIC}                                                                 ', 'NONE ', 'STXR_SR64_ldstexcl   ', '                                       '];
T[' STXRB  REG:w=$Ws REG:r=$Wt MEM:OFF:w:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=0 STACC=ATOMIC}                                                                 ', 'NONE ', 'STXRB_SR32_ldstexcl  ', '                                       '];
T[' STXRH  REG:w=$Ws REG:r=$Wt MEM:OFF:w:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=0 STACC=ATOMIC}                                                                 ', 'NONE ', 'STXRH_SR32_ldstexcl  ', '                                       '];



# ldsttags: Load/store memory tags.
# A64: ig0=0b11011001 opc:u=0bxx ig1=0b1 imm9:u=0bxxxxxxxxx op2:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'ldsttags', 'A64');

DEF XnSP    => GPR64('Xn', 'Xn', 'read|SP')                                                                 ;
DEF Xt      => GPR64('Xt', 'Xt')                                                                            ;
DEF XtSP    => GPR64('Xt', 'Xt', 'SP')                                                                      ;
DEF simmx16 => IMM('simm', 'imm9', scale => 16, signpos => 'imm9<8>', range => (-4096, 4080), default => 0) ;

T[' LDG   REG:rw=$Xt MEM:OFF:r:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:s16=$simmx16}  ', 'NONE ', 'LDG_64Loffset_ldsttags   ', 'arch_variant=ARMv8v5 '];
T[' LDGM  REG:w=$Xt MEM:r:i64={BASE=$XnSP SZ=64}                               ', 'NONE ', 'LDGM_64bulk_ldsttags     ', 'arch_variant=ARMv8v5 '];
T[' ST2G  REG:r=$XtSP MEM:PST:w:i64={BASE=$XnSP SZ=64 IMMOFF:s16=$simmx16}     ', 'NONE ', 'ST2G_64Spost_ldsttags    ', 'arch_variant=ARMv8v5 '];
T[' ST2G  REG:r=$XtSP MEM:PRE:w:i64={BASE=$XnSP SZ=64 IMMOFF:s16=$simmx16}     ', 'NONE ', 'ST2G_64Spre_ldsttags     ', 'arch_variant=ARMv8v5 '];
T[' ST2G  REG:r=$XtSP MEM:OFF:w:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:s16=$simmx16} ', 'NONE ', 'ST2G_64Soffset_ldsttags  ', 'arch_variant=ARMv8v5 '];
T[' STG   REG:r=$XtSP MEM:PST:w:i64={BASE=$XnSP SZ=64 IMMOFF:s16=$simmx16}     ', 'NONE ', 'STG_64Spost_ldsttags     ', 'arch_variant=ARMv8v5 '];
T[' STG   REG:r=$XtSP MEM:PRE:w:i64={BASE=$XnSP SZ=64 IMMOFF:s16=$simmx16}     ', 'NONE ', 'STG_64Spre_ldsttags      ', 'arch_variant=ARMv8v5 '];
T[' STG   REG:r=$XtSP MEM:OFF:w:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:s16=$simmx16} ', 'NONE ', 'STG_64Soffset_ldsttags   ', 'arch_variant=ARMv8v5 '];
T[' STGM  REG:r=$Xt MEM:w:i64={BASE=$XnSP SZ=64}                               ', 'NONE ', 'STGM_64bulk_ldsttags     ', 'arch_variant=ARMv8v5 '];
T[' STZ2G REG:r=$XtSP MEM:PST:w:i64={BASE=$XnSP SZ=64 IMMOFF:s16=$simmx16}     ', 'NONE ', 'STZ2G_64Spost_ldsttags   ', 'arch_variant=ARMv8v5 '];
T[' STZ2G REG:r=$XtSP MEM:PRE:w:i64={BASE=$XnSP SZ=64 IMMOFF:s16=$simmx16}     ', 'NONE ', 'STZ2G_64Spre_ldsttags    ', 'arch_variant=ARMv8v5 '];
T[' STZ2G REG:r=$XtSP MEM:OFF:w:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:s16=$simmx16} ', 'NONE ', 'STZ2G_64Soffset_ldsttags ', 'arch_variant=ARMv8v5 '];
T[' STZG  REG:r=$XtSP MEM:PST:w:i64={BASE=$XnSP SZ=64 IMMOFF:s16=$simmx16}     ', 'NONE ', 'STZG_64Spost_ldsttags    ', 'arch_variant=ARMv8v5 '];
T[' STZG  REG:r=$XtSP MEM:PRE:w:i64={BASE=$XnSP SZ=64 IMMOFF:s16=$simmx16}     ', 'NONE ', 'STZG_64Spre_ldsttags     ', 'arch_variant=ARMv8v5 '];
T[' STZG  REG:r=$XtSP MEM:OFF:w:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:s16=$simmx16} ', 'NONE ', 'STZG_64Soffset_ldsttags  ', 'arch_variant=ARMv8v5 '];
T[' STZGM REG:r=$Xt MEM:w:i64={BASE=$XnSP SZ=64}                               ', 'NONE ', 'STZGM_64bulk_ldsttags    ', 'arch_variant=ARMv8v5 '];



# log_imm: Logical (immediate).
# A64: sf:u=0bx opc:u=0bxx ig0=0b100100 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'log_imm', 'A64');

DEF Wd   => GPR32('Wd', 'Rd', 'write')    ;
DEF WdSP => GPR32('Wd', 'Rd', 'write|SP') ;
DEF Wn   => GPR32('Wn', 'Rn', 'read')     ;
DEF Xd   => GPR64('Xd', 'Rd', 'write')    ;
DEF XdSP => GPR64('Xd', 'Rd', 'write|SP') ;
DEF Xn   => GPR64('Xn', 'Rn', 'read')     ;
DEF aimm => IMM('aimm', 'N:imms:immr')    ;

T[' AND  REG=$WdSP REG=$Wn IMM:i32=$aimm ', 'NONE ', 'AND_32_log_imm       ', ' '];
T[' AND  REG=$XdSP REG=$Xn IMM:i64=$aimm ', 'NONE ', 'AND_64_log_imm       ', ' '];
T[' ANDS REG=$Wd REG=$Wn IMM:i32=$aimm   ', 'NONE ', 'ANDS_32S_log_imm     ', ' '];
T[' ANDS REG=$Xd REG=$Xn IMM:i64=$aimm   ', 'NONE ', 'ANDS_64S_log_imm     ', ' '];
T[' EOR  REG=$WdSP REG=$Wn IMM:i32=$aimm ', 'NONE ', 'EOR_32_log_imm       ', ' '];
T[' EOR  REG=$XdSP REG=$Xn IMM:i64=$aimm ', 'NONE ', 'EOR_64_log_imm       ', ' '];
T[' ORR  REG=$WdSP REG=$Wn IMM:i32=$aimm ', 'NONE ', 'ORR_32_log_imm       ', ' '];
T[' ORR  REG=$XdSP REG=$Xn IMM:i64=$aimm ', 'NONE ', 'ORR_64_log_imm       ', ' '];
T[' MOV  REG=$WdSP IMM:i32=$aimm         ', 'NONE ', 'MOV_ORR_32_log_imm   ', ' '];
T[' MOV  REG=$XdSP IMM:i64=$aimm         ', 'NONE ', 'MOV_ORR_64_log_imm   ', ' '];
T[' TST  REG=$Wn IMM:i32=$aimm           ', 'NONE ', 'TST_ANDS_32S_log_imm ', ' '];
T[' TST  REG=$Xn IMM:i64=$aimm           ', 'NONE ', 'TST_ANDS_64S_log_imm ', ' '];



# log_shift: Logical (shifted register).
# A64: sf:u=0bx opc:u=0bxx ig0=0b01010 shift:u=0bxx N:u=0bx Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'log_shift', 'A64');

DEF Wd      => GPR32('Wd', 'Rd', 'write')                                       ;
DEF Wm      => GPR32('Wm', 'Rm', 'read')                                        ;
DEF Wn      => GPR32('Wn', 'Rn', 'read')                                        ;
DEF Xd      => GPR64('Xd', 'Rd', 'write')                                       ;
DEF Xm      => GPR64('Xm', 'Rm', 'read')                                        ;
DEF Xn      => GPR64('Xn', 'Rn', 'read')                                        ;
DEF amount0 => IMM('amount', 'imm6', range => (0, 31), default => 0)            ;
DEF amount2 => IMM('amount', 'imm6', range => (0, 63), default => 0)            ;
DEF shift0  => SHIFT('shift', 'shift', table => 'tbl_shift1', default => 'LSL') ;

T[' AND  REG=$Wd REG=$Wn REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'AND_32_log_shift      ', ' '];
T[' AND  REG=$Xd REG=$Xn REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }} ', 'NONE ', 'AND_64_log_shift      ', ' '];
T[' ANDS REG=$Wd REG=$Wn REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'ANDS_32_log_shift     ', ' '];
T[' ANDS REG=$Xd REG=$Xn REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }} ', 'NONE ', 'ANDS_64_log_shift     ', ' '];
T[' BIC  REG=$Wd REG=$Wn REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'BIC_32_log_shift      ', ' '];
T[' BIC  REG=$Xd REG=$Xn REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }} ', 'NONE ', 'BIC_64_log_shift      ', ' '];
T[' BICS REG=$Wd REG=$Wn REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'BICS_32_log_shift     ', ' '];
T[' BICS REG=$Xd REG=$Xn REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }} ', 'NONE ', 'BICS_64_log_shift     ', ' '];
T[' EON  REG=$Wd REG=$Wn REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'EON_32_log_shift      ', ' '];
T[' EON  REG=$Xd REG=$Xn REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }} ', 'NONE ', 'EON_64_log_shift      ', ' '];
T[' EOR  REG=$Wd REG=$Wn REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'EOR_32_log_shift      ', ' '];
T[' EOR  REG=$Xd REG=$Xn REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }} ', 'NONE ', 'EOR_64_log_shift      ', ' '];
T[' ORN  REG=$Wd REG=$Wn REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'ORN_32_log_shift      ', ' '];
T[' ORN  REG=$Xd REG=$Xn REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }} ', 'NONE ', 'ORN_64_log_shift      ', ' '];
T[' ORR  REG=$Wd REG=$Wn REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }} ', 'NONE ', 'ORR_32_log_shift      ', ' '];
T[' ORR  REG=$Xd REG=$Xn REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }} ', 'NONE ', 'ORR_64_log_shift      ', ' '];
T[' MOV  REG=$Wd REG=$Wm                                                                 ', 'NONE ', 'MOV_ORR_32_log_shift  ', ' '];
T[' MOV  REG=$Xd REG=$Xm                                                                 ', 'NONE ', 'MOV_ORR_64_log_shift  ', ' '];
T[' MVN  REG=$Wd REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}         ', 'NONE ', 'MVN_ORN_32_log_shift  ', ' '];
T[' MVN  REG=$Xd REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }}         ', 'NONE ', 'MVN_ORN_64_log_shift  ', ' '];
T[' TST  REG=$Wn REGSH={REG=$Wm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount0 }}         ', 'NONE ', 'TST_ANDS_32_log_shift ', ' '];
T[' TST  REG=$Xn REGSH={REG=$Xm SHIFTER:OPT={SHIFT=$shift0 AMOUNT:u8=$amount2 }}         ', 'NONE ', 'TST_ANDS_64_log_shift ', ' '];



# memop: Atomic memory operations.
# A64: size:u=0bxx ig0=0b111 V:u=0bx ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3:u=0bx opc:u=0bxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'memop', 'A64');

DEF Ws   => GPR32('Ws', 'Rs')            ;
DEF Wt   => GPR32('Wt', 'Rt')            ;
DEF XnSP => GPR64('Xn', 'Rn', 'read|SP') ;
DEF Xs   => GPR64('Xs', 'Rs')            ;
DEF Xt   => GPR64('Xt', 'Rt')            ;

T[' LDADD     REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC}                         ', 'NONE ', 'LDADD_32_memop             ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDADDA    REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                 ', 'NONE ', 'LDADDA_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDADDAL   REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDADDAL_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDADDL    REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'LDADDL_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDADD     REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC}                         ', 'NONE ', 'LDADD_64_memop             ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDADDA    REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                 ', 'NONE ', 'LDADDA_64_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDADDAL   REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDADDAL_64_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDADDL    REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'LDADDL_64_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDADDAB   REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                 ', 'NONE ', 'LDADDAB_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDADDALB  REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDADDALB_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDADDB    REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC}                         ', 'NONE ', 'LDADDB_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDADDLB   REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'LDADDLB_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDADDAH   REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                 ', 'NONE ', 'LDADDAH_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDADDALH  REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDADDALH_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDADDH    REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC}                         ', 'NONE ', 'LDADDH_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDADDLH   REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'LDADDLH_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDAPR     REG:w=$Wt MEM:OFF:r:i32={BASE=$XnSP SZ=32 IMMOFF:OPT:u8=0 LDACC=ACQUIRE}                            ', 'NONE ', 'LDAPR_32L_memop            ', 'arch_variant=ARMv8v3                   '];
T[' LDAPR     REG:w=$Xt MEM:OFF:r:i64={BASE=$XnSP SZ=64 IMMOFF:OPT:u8=0 LDACC=ACQUIRE}                            ', 'NONE ', 'LDAPR_64L_memop            ', 'arch_variant=ARMv8v3                   '];
T[' LDAPRB    REG:w=$Wt MEM:OFF:r:i8={ BASE=$XnSP SZ=8  IMMOFF:OPT:u8=0 LDACC=ACQUIRE}                            ', 'NONE ', 'LDAPRB_32L_memop           ', 'arch_variant=ARMv8v3                   '];
T[' LDAPRH    REG:w=$Wt MEM:OFF:r:i16={BASE=$XnSP SZ=16 IMMOFF:OPT:u8=0 LDACC=ACQUIRE}                            ', 'NONE ', 'LDAPRH_32L_memop           ', 'arch_variant=ARMv8v3                   '];
T[' LDCLR     REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC}                         ', 'NONE ', 'LDCLR_32_memop             ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDCLRA    REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                 ', 'NONE ', 'LDCLRA_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDCLRAL   REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDCLRAL_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDCLRL    REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'LDCLRL_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDCLR     REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC}                         ', 'NONE ', 'LDCLR_64_memop             ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDCLRA    REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                 ', 'NONE ', 'LDCLRA_64_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDCLRAL   REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDCLRAL_64_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDCLRL    REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'LDCLRL_64_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDCLRAB   REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                 ', 'NONE ', 'LDCLRAB_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDCLRALB  REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDCLRALB_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDCLRB    REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC}                         ', 'NONE ', 'LDCLRB_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDCLRLB   REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'LDCLRLB_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDCLRAH   REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                 ', 'NONE ', 'LDCLRAH_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDCLRALH  REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDCLRALH_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDCLRH    REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC}                         ', 'NONE ', 'LDCLRH_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDCLRLH   REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'LDCLRLH_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDEOR     REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC}                         ', 'NONE ', 'LDEOR_32_memop             ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDEORA    REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                 ', 'NONE ', 'LDEORA_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDEORAL   REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDEORAL_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDEORL    REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'LDEORL_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDEOR     REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC}                         ', 'NONE ', 'LDEOR_64_memop             ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDEORA    REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                 ', 'NONE ', 'LDEORA_64_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDEORAL   REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDEORAL_64_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDEORL    REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'LDEORL_64_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDEORAB   REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                 ', 'NONE ', 'LDEORAB_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDEORALB  REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDEORALB_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDEORB    REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC}                         ', 'NONE ', 'LDEORB_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDEORLB   REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'LDEORLB_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDEORAH   REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                 ', 'NONE ', 'LDEORAH_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDEORALH  REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDEORALH_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDEORH    REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC}                         ', 'NONE ', 'LDEORH_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDEORLH   REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'LDEORLH_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSET     REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC}                         ', 'NONE ', 'LDSET_32_memop             ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSETA    REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                 ', 'NONE ', 'LDSETA_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSETAL   REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDSETAL_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSETL    REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'LDSETL_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSET     REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC}                         ', 'NONE ', 'LDSET_64_memop             ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSETA    REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                 ', 'NONE ', 'LDSETA_64_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSETAL   REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDSETAL_64_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSETL    REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'LDSETL_64_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSETAB   REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                 ', 'NONE ', 'LDSETAB_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSETALB  REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDSETALB_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSETB    REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC}                         ', 'NONE ', 'LDSETB_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSETLB   REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'LDSETLB_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSETAH   REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}                 ', 'NONE ', 'LDSETAH_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSETALH  REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDSETALH_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSETH    REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC}                         ', 'NONE ', 'LDSETH_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSETLH   REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                 ', 'NONE ', 'LDSETLH_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMAX    REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'LDSMAX_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMAXA   REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s32={BASE=$XnSP SZ=32 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'LDSMAXA_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMAXAL  REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s32={BASE=$XnSP SZ=32 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'LDSMAXAL_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMAXL   REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDSMAXL_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMAX    REG:r:s64=$Xs REG:w:s64=$Xt MEM:rw:s64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'LDSMAX_64_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMAXA   REG:r:s64=$Xs REG:w:s64=$Xt MEM:rw:s64={BASE=$XnSP SZ=64 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'LDSMAXA_64_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMAXAL  REG:r:s64=$Xs REG:w:s64=$Xt MEM:rw:s64={BASE=$XnSP SZ=64 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'LDSMAXAL_64_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMAXL   REG:r:s64=$Xs REG:w:s64=$Xt MEM:rw:s64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDSMAXL_64_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMAXAB  REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s8={ BASE=$XnSP SZ=8  LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'LDSMAXAB_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMAXALB REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s8={ BASE=$XnSP SZ=8  LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'LDSMAXALB_32_memop         ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMAXB   REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'LDSMAXB_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMAXLB  REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDSMAXLB_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMAXAH  REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s16={BASE=$XnSP SZ=16 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'LDSMAXAH_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMAXALH REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s16={BASE=$XnSP SZ=16 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'LDSMAXALH_32_memop         ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMAXH   REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'LDSMAXH_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMAXLH  REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDSMAXLH_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMIN    REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'LDSMIN_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMINA   REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s32={BASE=$XnSP SZ=32 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'LDSMINA_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMINAL  REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s32={BASE=$XnSP SZ=32 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'LDSMINAL_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMINL   REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDSMINL_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMIN    REG:r:s64=$Xs REG:w:s64=$Xt MEM:rw:s64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'LDSMIN_64_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMINA   REG:r:s64=$Xs REG:w:s64=$Xt MEM:rw:s64={BASE=$XnSP SZ=64 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'LDSMINA_64_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMINAL  REG:r:s64=$Xs REG:w:s64=$Xt MEM:rw:s64={BASE=$XnSP SZ=64 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'LDSMINAL_64_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMINL   REG:r:s64=$Xs REG:w:s64=$Xt MEM:rw:s64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDSMINL_64_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMINAB  REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s8={ BASE=$XnSP SZ=8  LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'LDSMINAB_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMINALB REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s8={ BASE=$XnSP SZ=8  LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'LDSMINALB_32_memop         ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMINB   REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'LDSMINB_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMINLB  REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDSMINLB_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMINAH  REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s16={BASE=$XnSP SZ=16 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'LDSMINAH_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMINALH REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s16={BASE=$XnSP SZ=16 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'LDSMINALH_32_memop         ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMINH   REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'LDSMINH_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDSMINLH  REG:r:s32=$Ws REG:w:s32=$Wt MEM:rw:s16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDSMINLH_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMAX    REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'LDUMAX_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMAXA   REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u32={BASE=$XnSP SZ=32 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'LDUMAXA_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMAXAL  REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u32={BASE=$XnSP SZ=32 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'LDUMAXAL_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMAXL   REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDUMAXL_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMAX    REG:r:u64=$Xs REG:w:u64=$Xt MEM:rw:u64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'LDUMAX_64_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMAXA   REG:r:u64=$Xs REG:w:u64=$Xt MEM:rw:u64={BASE=$XnSP SZ=64 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'LDUMAXA_64_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMAXAL  REG:r:u64=$Xs REG:w:u64=$Xt MEM:rw:u64={BASE=$XnSP SZ=64 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'LDUMAXAL_64_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMAXL   REG:r:u64=$Xs REG:w:u64=$Xt MEM:rw:u64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDUMAXL_64_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMAXAB  REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u8={ BASE=$XnSP SZ=8  LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'LDUMAXAB_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMAXALB REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u8={ BASE=$XnSP SZ=8  LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'LDUMAXALB_32_memop         ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMAXB   REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'LDUMAXB_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMAXLB  REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDUMAXLB_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMAXAH  REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u16={BASE=$XnSP SZ=16 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'LDUMAXAH_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMAXALH REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u16={BASE=$XnSP SZ=16 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'LDUMAXALH_32_memop         ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMAXH   REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'LDUMAXH_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMAXLH  REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDUMAXLH_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMIN    REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'LDUMIN_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMINA   REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u32={BASE=$XnSP SZ=32 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'LDUMINA_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMINAL  REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u32={BASE=$XnSP SZ=32 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'LDUMINAL_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMINL   REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDUMINL_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMIN    REG:r:u64=$Xs REG:w:u64=$Xt MEM:rw:u64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'LDUMIN_64_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMINA   REG:r:u64=$Xs REG:w:u64=$Xt MEM:rw:u64={BASE=$XnSP SZ=64 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'LDUMINA_64_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMINAL  REG:r:u64=$Xs REG:w:u64=$Xt MEM:rw:u64={BASE=$XnSP SZ=64 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'LDUMINAL_64_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMINL   REG:r:u64=$Xs REG:w:u64=$Xt MEM:rw:u64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDUMINL_64_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMINAB  REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u8={ BASE=$XnSP SZ=8  LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'LDUMINAB_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMINALB REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u8={ BASE=$XnSP SZ=8  LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'LDUMINALB_32_memop         ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMINB   REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'LDUMINB_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMINLB  REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDUMINLB_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMINAH  REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u16={BASE=$XnSP SZ=16 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC}         ', 'NONE ', 'LDUMINAH_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMINALH REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u16={BASE=$XnSP SZ=16 LDACC=ATOMIC_ACQUIRE STACC=ATOMIC_RELEASE} ', 'NONE ', 'LDUMINALH_32_memop         ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMINH   REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC}                 ', 'NONE ', 'LDUMINH_32_memop           ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' LDUMINLH  REG:r:u32=$Ws REG:w:u32=$Wt MEM:rw:u16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC_RELEASE}         ', 'NONE ', 'LDUMINLH_32_memop          ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' SWP       REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32}                                                   ', 'NONE ', 'SWP_32_memop               ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' SWPA      REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32}                                                   ', 'NONE ', 'SWPA_32_memop              ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' SWPAL     REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32}                                                   ', 'NONE ', 'SWPAL_32_memop             ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' SWPL      REG:r=$Ws REG:w=$Wt MEM:rw:i32={BASE=$XnSP SZ=32}                                                   ', 'NONE ', 'SWPL_32_memop              ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' SWP       REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64}                                                   ', 'NONE ', 'SWP_64_memop               ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' SWPA      REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64}                                                   ', 'NONE ', 'SWPA_64_memop              ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' SWPAL     REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64}                                                   ', 'NONE ', 'SWPAL_64_memop             ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' SWPL      REG:r=$Xs REG:w=$Xt MEM:rw:i64={BASE=$XnSP SZ=64}                                                   ', 'NONE ', 'SWPL_64_memop              ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' SWPAB     REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8 }                                                   ', 'NONE ', 'SWPAB_32_memop             ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' SWPALB    REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8 }                                                   ', 'NONE ', 'SWPALB_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' SWPB      REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8 }                                                   ', 'NONE ', 'SWPB_32_memop              ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' SWPLB     REG:r=$Ws REG:w=$Wt MEM:rw:i8={ BASE=$XnSP SZ=8 }                                                   ', 'NONE ', 'SWPLB_32_memop             ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' SWPAH     REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16}                                                   ', 'NONE ', 'SWPAH_32_memop             ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' SWPALH    REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16}                                                   ', 'NONE ', 'SWPALH_32_memop            ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' SWPH      REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16}                                                   ', 'NONE ', 'SWPH_32_memop              ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' SWPLH     REG:r=$Ws REG:w=$Wt MEM:rw:i16={BASE=$XnSP SZ=16}                                                   ', 'NONE ', 'SWPLH_32_memop             ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STADD     REG:r=$Ws MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC}                                   ', 'NONE ', 'STADD_LDADD_32_memop       ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STADDL    REG:r=$Ws MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                           ', 'NONE ', 'STADDL_LDADDL_32_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STADD     REG:r=$Xs MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC}                                   ', 'NONE ', 'STADD_LDADD_64_memop       ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STADDL    REG:r=$Xs MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                           ', 'NONE ', 'STADDL_LDADDL_64_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STADDB    REG:r=$Ws MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC}                                   ', 'NONE ', 'STADDB_LDADDB_32_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STADDLB   REG:r=$Ws MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC_RELEASE}                           ', 'NONE ', 'STADDLB_LDADDLB_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STADDH    REG:r=$Ws MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC}                                   ', 'NONE ', 'STADDH_LDADDH_32_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STADDLH   REG:r=$Ws MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                           ', 'NONE ', 'STADDLH_LDADDLH_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STCLR     REG:r=$Ws MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC}                                   ', 'NONE ', 'STCLR_LDCLR_32_memop       ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STCLRL    REG:r=$Ws MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                           ', 'NONE ', 'STCLRL_LDCLRL_32_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STCLR     REG:r=$Xs MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC}                                   ', 'NONE ', 'STCLR_LDCLR_64_memop       ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STCLRL    REG:r=$Xs MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                           ', 'NONE ', 'STCLRL_LDCLRL_64_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STCLRB    REG:r=$Ws MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC}                                   ', 'NONE ', 'STCLRB_LDCLRB_32_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STCLRLB   REG:r=$Ws MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC_RELEASE}                           ', 'NONE ', 'STCLRLB_LDCLRLB_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STCLRH    REG:r=$Ws MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC}                                   ', 'NONE ', 'STCLRH_LDCLRH_32_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STCLRLH   REG:r=$Ws MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                           ', 'NONE ', 'STCLRLH_LDCLRLH_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STEOR     REG:r=$Ws MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC}                                   ', 'NONE ', 'STEOR_LDEOR_32_memop       ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STEORL    REG:r=$Ws MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                           ', 'NONE ', 'STEORL_LDEORL_32_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STEOR     REG:r=$Xs MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC}                                   ', 'NONE ', 'STEOR_LDEOR_64_memop       ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STEORL    REG:r=$Xs MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                           ', 'NONE ', 'STEORL_LDEORL_64_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STEORB    REG:r=$Ws MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC}                                   ', 'NONE ', 'STEORB_LDEORB_32_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STEORLB   REG:r=$Ws MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC_RELEASE}                           ', 'NONE ', 'STEORLB_LDEORLB_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STEORH    REG:r=$Ws MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC}                                   ', 'NONE ', 'STEORH_LDEORH_32_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STEORLH   REG:r=$Ws MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                           ', 'NONE ', 'STEORLH_LDEORLH_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSET     REG:r=$Ws MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC}                                   ', 'NONE ', 'STSET_LDSET_32_memop       ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSETL    REG:r=$Ws MEM:rw:i32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                           ', 'NONE ', 'STSETL_LDSETL_32_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSET     REG:r=$Xs MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC}                                   ', 'NONE ', 'STSET_LDSET_64_memop       ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSETL    REG:r=$Xs MEM:rw:i64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                           ', 'NONE ', 'STSETL_LDSETL_64_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSETB    REG:r=$Ws MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC}                                   ', 'NONE ', 'STSETB_LDSETB_32_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSETLB   REG:r=$Ws MEM:rw:i8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC_RELEASE}                           ', 'NONE ', 'STSETLB_LDSETLB_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSETH    REG:r=$Ws MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC}                                   ', 'NONE ', 'STSETH_LDSETH_32_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSETLH   REG:r=$Ws MEM:rw:i16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                           ', 'NONE ', 'STSETLH_LDSETLH_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSMAX    REG:r:s32=$Ws MEM:rw:s32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC}                               ', 'NONE ', 'STSMAX_LDSMAX_32_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSMAXL   REG:r:s32=$Ws MEM:rw:s32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                       ', 'NONE ', 'STSMAXL_LDSMAXL_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSMAX    REG:r:s64=$Xs MEM:rw:s64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC}                               ', 'NONE ', 'STSMAX_LDSMAX_64_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSMAXL   REG:r:s64=$Xs MEM:rw:s64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                       ', 'NONE ', 'STSMAXL_LDSMAXL_64_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSMAXB   REG:r:s32=$Ws MEM:rw:s8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC}                               ', 'NONE ', 'STSMAXB_LDSMAXB_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSMAXLB  REG:r:s32=$Ws MEM:rw:s8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC_RELEASE}                       ', 'NONE ', 'STSMAXLB_LDSMAXLB_32_memop ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSMAXH   REG:r:s32=$Ws MEM:rw:s16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC}                               ', 'NONE ', 'STSMAXH_LDSMAXH_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSMAXLH  REG:r:s32=$Ws MEM:rw:s16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                       ', 'NONE ', 'STSMAXLH_LDSMAXLH_32_memop ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSMIN    REG:r:s32=$Ws MEM:rw:s32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC}                               ', 'NONE ', 'STSMIN_LDSMIN_32_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSMINL   REG:r:s32=$Ws MEM:rw:s32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                       ', 'NONE ', 'STSMINL_LDSMINL_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSMIN    REG:r:s64=$Xs MEM:rw:s64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC}                               ', 'NONE ', 'STSMIN_LDSMIN_64_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSMINL   REG:r:s64=$Xs MEM:rw:s64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                       ', 'NONE ', 'STSMINL_LDSMINL_64_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSMINB   REG:r:s32=$Ws MEM:rw:s8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC}                               ', 'NONE ', 'STSMINB_LDSMINB_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSMINLB  REG:r:s32=$Ws MEM:rw:s8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC_RELEASE}                       ', 'NONE ', 'STSMINLB_LDSMINLB_32_memop ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSMINH   REG:r:s32=$Ws MEM:rw:s16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC}                               ', 'NONE ', 'STSMINH_LDSMINH_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STSMINLH  REG:r:s32=$Ws MEM:rw:s16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                       ', 'NONE ', 'STSMINLH_LDSMINLH_32_memop ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STUMAX    REG:r:u32=$Ws MEM:rw:u32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC}                               ', 'NONE ', 'STUMAX_LDUMAX_32_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STUMAXL   REG:r:u32=$Ws MEM:rw:u32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                       ', 'NONE ', 'STUMAXL_LDUMAXL_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STUMAX    REG:r:u64=$Xs MEM:rw:u64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC}                               ', 'NONE ', 'STUMAX_LDUMAX_64_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STUMAXL   REG:r:u64=$Xs MEM:rw:u64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                       ', 'NONE ', 'STUMAXL_LDUMAXL_64_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STUMAXB   REG:r:u32=$Ws MEM:rw:u8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC}                               ', 'NONE ', 'STUMAXB_LDUMAXB_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STUMAXLB  REG:r:u32=$Ws MEM:rw:u8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC_RELEASE}                       ', 'NONE ', 'STUMAXLB_LDUMAXLB_32_memop ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STUMAXH   REG:r:u32=$Ws MEM:rw:u16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC}                               ', 'NONE ', 'STUMAXH_LDUMAXH_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STUMAXLH  REG:r:u32=$Ws MEM:rw:u16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                       ', 'NONE ', 'STUMAXLH_LDUMAXLH_32_memop ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STUMIN    REG:r:u32=$Ws MEM:rw:u32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC}                               ', 'NONE ', 'STUMIN_LDUMIN_32_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STUMINL   REG:r:u32=$Ws MEM:rw:u32={BASE=$XnSP SZ=32 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                       ', 'NONE ', 'STUMINL_LDUMINL_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STUMIN    REG:r:u64=$Xs MEM:rw:u64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC}                               ', 'NONE ', 'STUMIN_LDUMIN_64_memop     ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STUMINL   REG:r:u64=$Xs MEM:rw:u64={BASE=$XnSP SZ=64 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                       ', 'NONE ', 'STUMINL_LDUMINL_64_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STUMINB   REG:r:u32=$Ws MEM:rw:u8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC}                               ', 'NONE ', 'STUMINB_LDUMINB_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STUMINLB  REG:r:u32=$Ws MEM:rw:u8={ BASE=$XnSP SZ=8  LDACC=ATOMIC STACC=ATOMIC_RELEASE}                       ', 'NONE ', 'STUMINLB_LDUMINLB_32_memop ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STUMINH   REG:r:u32=$Ws MEM:rw:u16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC}                               ', 'NONE ', 'STUMINH_LDUMINH_32_memop   ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];
T[' STUMINLH  REG:r:u32=$Ws MEM:rw:u16={BASE=$XnSP SZ=16 LDACC=ATOMIC STACC=ATOMIC_RELEASE}                       ', 'NONE ', 'STUMINLH_LDUMINLH_32_memop ', 'arch_variant=ARMv8v1 extensions=ATOMIC '];



# movewide: Move wide (immediate).
# A64: sf:u=0bx opc:u=0bxx ig0=0b100101 hw:u=0bxx imm16:u=0bxxxxxxxxxxxxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'movewide', 'A64');

DEF Wd              => GPR32('Wd', 'Rd', 'write')               ;
DEF WideImmAlias    => IMM('WideImmAlias', 'hw:imm16')          ;
DEF WideImmInverted => IMM('WideImmInverted', 'hw:imm16')       ;
DEF Xd              => GPR64('Xd', 'Rd', 'write')               ;
DEF amountx16       => IMM('amount', 'hw', scale => 16)         ;
DEF imm             => IMM('imm', 'imm16', range => (0, 65535)) ;

T[' MOVK REG=$Wd IMMSH={IMM:u16=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amountx16}} ', 'NONE ', 'MOVK_32_movewide     ', ' '];
T[' MOVK REG=$Xd IMMSH={IMM:u16=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i64=$amountx16}} ', 'NONE ', 'MOVK_64_movewide     ', ' '];
T[' MOVN REG=$Wd IMMSH={IMM:u16=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amountx16}} ', 'NONE ', 'MOVN_32_movewide     ', ' '];
T[' MOVN REG=$Xd IMMSH={IMM:u16=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i64=$amountx16}} ', 'NONE ', 'MOVN_64_movewide     ', ' '];
T[' MOVZ REG=$Wd IMMSH={IMM:u16=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i32=$amountx16}} ', 'NONE ', 'MOVZ_32_movewide     ', ' '];
T[' MOVZ REG=$Xd IMMSH={IMM:u16=$imm SHIFTER:OPT={SHIFT=LSL AMOUNT:i64=$amountx16}} ', 'NONE ', 'MOVZ_64_movewide     ', ' '];
T[' MOV  REG=$Wd IMM:i32=$WideImmInverted                                           ', 'NONE ', 'MOV_MOVN_32_movewide ', ' '];
T[' MOV  REG=$Xd IMM:i64=$WideImmInverted                                           ', 'NONE ', 'MOV_MOVN_64_movewide ', ' '];
T[' MOV  REG=$Wd IMM:i32=$WideImmAlias                                              ', 'NONE ', 'MOV_MOVZ_32_movewide ', ' '];
T[' MOV  REG=$Xd IMM:i64=$WideImmAlias                                              ', 'NONE ', 'MOV_MOVZ_64_movewide ', ' '];



# pcreladdr: PC-rel. addressing.
# A64: op:u=0bx immlo:u=0bxx ig0=0b10000 immhi:u=0bxxxxxxxxxxxxxxxxxxx Rd:u=0bxxxxx
HINT('ICLASS', 'pcreladdr', 'A64');

DEF Xd         => GPR64('Xd', 'Rd', 'write')                                                                               ;
DEF label      => LABEL('label', 'immhi:immlo', signpos => 'immhi<18>', range => (-1000000, 1000000))                      ;
DEF labelx4096 => LABEL('label', 'immhi:immlo', scale => 4096, signpos => 'immhi<18>', range => (-4000000000, 4000000000)) ;

T[' ADR  REG=$Xd ADDR:s64=$label       ', 'NONE ', 'ADR_only_pcreladdr  ', ' '];
T[' ADRP REG=$Xd LABEL:s64=$labelx4096 ', 'NONE ', 'ADRP_only_pcreladdr ', ' '];



# perm_undef: Reserved.
# A64: ig0=0b0000000000000000 imm16:u=0bxxxxxxxxxxxxxxxx
HINT('ICLASS', 'perm_undef', 'A64');

DEF imm => IMM('imm', 'imm16', range => (0, 65535)) ;

T[' UDF IMM:u16=$imm ', 'NONE ', 'UDF_only_perm_undef ', ' '];



# pstate: PSTATE.
# A64: ig0=0b1101010100000 op1:u=0bxxx ig1=0b0100 CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx
HINT('ICLASS', 'pstate', 'A64');

DEF imm         => IMM('imm', 'CRm', range => (0, 15))                               ;
DEF pstatefield => PSTATEFIELD('pstatefield', 'op1:op2', table => 'tbl_pstatefield') ;

T[' AXFLAG                                      ', 'NONE ', 'AXFLAG_M_pstate ', 'arch_variant=ARMv8v5 extensions=FLAGFORMAT           '];
T[' CFINV                                       ', 'NONE ', 'CFINV_M_pstate  ', 'arch_variant=ARMv8v4 extensions=FLAGMANIPULATE       '];
T[' MSR    PSTATEFIELD=$pstatefield IMM:u8=$imm ', 'NONE ', 'MSR_SI_pstate   ', 'arch_variant=ARMv8v1 extensions=DIT|MTE|PAN|SSBS|UAO '];
T[' XAFLAG                                      ', 'NONE ', 'XAFLAG_M_pstate ', 'arch_variant=ARMv8v5 extensions=FLAGFORMAT           '];



# rmif: Rotate right into flags.
# A64: sf:u=0bx op:u=0bx S:u=0bx ig0=0b11010000 imm6:u=0bxxxxxx ig1=0b00001 Rn:u=0bxxxxx o2:u=0bx mask:u=0bxxxx
HINT('ICLASS', 'rmif', 'A64');

DEF Xn     => GPR64('Xn', 'Rn', 'read')                            ;
DEF mask   => IMM('mask', 'mask', range => (0, 15))                ;
DEF shift0 => IMM('shift', 'imm6', range => (0, 63), default => 0) ;

T[' RMIF REG=$Xn IMM:u8=$shift0 IMM:u8=$mask ', 'NONE ', 'RMIF_only_rmif ', 'arch_variant=ARMv8v4 extensions=FLAGMANIPULATE '];



# setf: Evaluate into flags.
# A64: sf:u=0bx op:u=0bx S:u=0bx ig0=0b11010000 opcode2:u=0bxxxxxx sz:u=0bx ig1=0b0010 Rn:u=0bxxxxx o3:u=0bx mask:u=0bxxxx
HINT('ICLASS', 'setf', 'A64');

DEF Wn => GPR32('Wn', 'Rn', 'read') ;

T[' SETF8  REG=$Wn ', 'NONE ', 'SETF8_only_setf  ', 'arch_variant=ARMv8v4 extensions=FLAGMANIPULATE '];
T[' SETF16 REG=$Wn ', 'NONE ', 'SETF16_only_setf ', 'arch_variant=ARMv8v4 extensions=FLAGMANIPULATE '];



# systeminstrs: System instructions.
# A64: ig0=0b1101010100 L:u=0bx ig1=0b01 op1:u=0bxxx CRn:u=0bxxxx CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx
HINT('ICLASS', 'systeminstrs', 'A64');

DEF Cm      => CSPACE('Cm', 'CRm')                                       ;
DEF Cn      => CSPACE('Cn', 'CRn')                                       ;
DEF Xt      => GPR64('Xt', 'Rt', default => '0b11111')                   ;
DEF Xt2     => GPR64('Xt', 'Rt')                                         ;
DEF at_op   => AT_OP('at_op', 'op1:CRm<0>:op2', table => 'tbl_at_op')    ;
DEF dc_op   => DC_OP('dc_op', 'op1:CRm:op2', table => 'tbl_dc_op')       ;
DEF ic_op   => IC_OP('ic_op', 'op1:CRm:op2', table => 'tbl_ic_op')       ;
DEF op      => IMM('op1', 'op1', range => (0, 7))                        ;
DEF op2     => IMM('op2', 'op2', range => (0, 7))                        ;
DEF tlbi_op => TLBI_OP('tlbi_op', 'op1:CRm:op2', table => 'tbl_tlbi_op') ;

T[' SYS  IMM:u8=$op CSPACE=$Cn CSPACE=$Cm IMM:u8=$op2 REG:OPT:r=$Xt ', 'NONE ', 'SYS_CR_systeminstrs      ', '                     '];
T[' SYSL REG:w=$Xt2 IMM:u8=$op CSPACE=$Cn CSPACE=$Cm IMM:u8=$op2    ', 'NONE ', 'SYSL_RC_systeminstrs     ', '                     '];
T[' AT   AT_OP=$at_op REG:r=$Xt2                                    ', 'NONE ', 'AT_SYS_CR_systeminstrs   ', '                     '];
T[' CFP  CTX_OP=RCTX REG:r=$Xt2                                     ', 'NONE ', 'CFP_SYS_CR_systeminstrs  ', 'arch_variant=ARMv8v5 '];
T[' CPP  CTX_OP=RCTX REG:r=$Xt2                                     ', 'NONE ', 'CPP_SYS_CR_systeminstrs  ', 'arch_variant=ARMv8v5 '];
T[' DC   DC_OP=$dc_op REG:r=$Xt2                                    ', 'NONE ', 'DC_SYS_CR_systeminstrs   ', '                     '];
T[' DVP  CTX_OP=RCTX REG:r=$Xt2                                     ', 'NONE ', 'DVP_SYS_CR_systeminstrs  ', 'arch_variant=ARMv8v5 '];
T[' IC   IC_OP=$ic_op REG:OPT:r=$Xt                                 ', 'NONE ', 'IC_SYS_CR_systeminstrs   ', '                     '];
T[' TLBI TLBI_OP=$tlbi_op REG:OPT:r=$Xt                             ', 'NONE ', 'TLBI_SYS_CR_systeminstrs ', '                     '];



# systemmove: System register move.
# A64: ig0=0b1101010100 L:u=0bx ig1=0b1 o0:u=0bx op1:u=0bxxx CRn:u=0bxxxx CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx
HINT('ICLASS', 'systemmove', 'A64');

DEF Xt        => GPR64('Xt', 'Rt')                         ;
DEF systemreg => SYSREG('systemreg', 'CRm:CRn:o0:op1:op2') ;

T[' MRS REG:w=$Xt SYSREG=$systemreg ', 'NONE ', 'MRS_RS_systemmove ', ' '];
T[' MSR SYSREG=$systemreg REG:r=$Xt ', 'NONE ', 'MSR_SR_systemmove ', ' '];



# systemresult: System with result.
# A64: ig0=0b1101010100100 op1:u=0bxxx CRn:u=0bxxxx CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx
HINT('ICLASS', 'systemresult', 'A64');

DEF Xt => GPR64('Xt', 'Rt') ;

T[' TSTART REG:w=$Xt ', 'NONE ', 'TSTART_BR_systemresult ', 'arch_variant=TME '];
T[' TTEST  REG:w=$Xt ', 'NONE ', 'TTEST_BR_systemresult  ', 'arch_variant=TME '];



# testbranch: Test and branch (immediate).
# A64: b5:u=0bx ig0=0b011011 op:u=0bx b40:u=0bxxxxx imm14:u=0bxxxxxxxxxxxxxx Rt:u=0bxxxxx
HINT('ICLASS', 'testbranch', 'A64');

DEF Wt      => GPR32('Wt', 'Rt')                                                                     ;
DEF Xt      => GPR64('Xt', 'Rt')                                                                     ;
DEF imm     => IMM('imm', 'b5:b40', range => (0, 63))                                                ;
DEF labelx4 => LABEL('label', 'imm14', scale => 4, signpos => 'imm14<13>', range => (-32000, 32000)) ;

T[' TBNZ REG:r=$Wt IMM:u8=$imm LABEL:s64=$labelx4 ', 'b5=0b0 ', 'TBNZ_only_testbranch ', ' '];
T[' TBNZ REG:r=$Xt IMM:u8=$imm LABEL:s64=$labelx4 ', 'b5=0b1 ', 'TBNZ_only_testbranch ', ' '];
T[' TBZ  REG:r=$Wt IMM:u8=$imm LABEL:s64=$labelx4 ', 'b5=0b0 ', 'TBZ_only_testbranch  ', ' '];
T[' TBZ  REG:r=$Xt IMM:u8=$imm LABEL:s64=$labelx4 ', 'b5=0b1 ', 'TBZ_only_testbranch  ', ' '];



1;

