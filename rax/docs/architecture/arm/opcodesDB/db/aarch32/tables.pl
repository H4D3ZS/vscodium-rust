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

TABLE tbl_align => [
	TV_ALIGN('OMITTED', '0b00'),
	TV_ALIGN('64'     , '0b01'),
];

TABLE tbl_align1 => [
	TV_ALIGN('OMITTED', '0b00'),
	TV_ALIGN('64'     , '0b01'),
	TV_ALIGN('128'    , '0b10'),
];

TABLE tbl_align2 => [
	TV_ALIGN('OMITTED', '0b00'),
	TV_ALIGN('64'     , '0b01'),
	TV_ALIGN('128'    , '0b10'),
	TV_ALIGN('256'    , '0b11'),
];

TABLE tbl_align3 => [
	TV_ALIGN('OMITTED'  , '0b0'),
	TV_ALIGN('UNDEFINED', '0b1'),
];

TABLE tbl_align4 => [
	TV_ALIGN('OMITTED', '0b00'),
	TV_ALIGN('16'     , '0b01'),
];

TABLE tbl_align5 => [
	TV_ALIGN('OMITTED', '0b000'),
	TV_ALIGN('32'     , '0b011'),
];

TABLE tbl_align6 => [
	TV_ALIGN('OMITTED', '0b0'),
	TV_ALIGN('16'     , '0b1'),
];

TABLE tbl_align7 => [
	TV_ALIGN('OMITTED', '0b0'),
	TV_ALIGN('32'     , '0b1'),
];

TABLE tbl_align8 => [
	TV_ALIGN('OMITTED', '0b0'),
	TV_ALIGN('64'     , '0b1'),
];

TABLE tbl_barrier => [
	TV_BARRIER('SY'   , '0b1111'),
	TV_BARRIER('ST'   , '0b1110'),
	TV_BARRIER('SYST' , '0b1110'),
	TV_BARRIER('LD'   , '0b1101'),
	TV_BARRIER('ISH'  , '0b1011'),
	TV_BARRIER('ISHST', '0b1010'),
	TV_BARRIER('ISHLD', '0b1001'),
	TV_BARRIER('NSH'  , '0b0111'),
	TV_BARRIER('NSHST', '0b0110'),
	TV_BARRIER('NSHLD', '0b0101'),
	TV_BARRIER('OSH'  , '0b0011'),
	TV_BARRIER('OSHST', '0b0010'),
	TV_BARRIER('OSHLD', '0b0001'),
];

TABLE tbl_barrier1 => [
	TV_BARRIER('SY', '0b1111'),
];

TABLE tbl_bnkdreg => [
	TV_BNKDREG('R8_usr'       , '0b000000'),
	TV_BNKDREG('R9_usr'       , '0b000001'),
	TV_BNKDREG('R10_usr'      , '0b000010'),
	TV_BNKDREG('R11_usr'      , '0b000011'),
	TV_BNKDREG('R12_usr'      , '0b000100'),
	TV_BNKDREG('SP_usr'       , '0b000101'),
	TV_BNKDREG('LR_usr'       , '0b000110'),
	TV_BNKDREG('UNPREDICTABLE', '0b000111'),
	TV_BNKDREG('R8_fiq'       , '0b001000'),
	TV_BNKDREG('R9_fiq'       , '0b001001'),
	TV_BNKDREG('R10_fiq'      , '0b001010'),
	TV_BNKDREG('R11_fiq'      , '0b001011'),
	TV_BNKDREG('R12_fiq'      , '0b001100'),
	TV_BNKDREG('SP_fiq'       , '0b001101'),
	TV_BNKDREG('LR_fiq'       , '0b001110'),
	TV_BNKDREG('UNPREDICTABLE', '0b001111'),
	TV_BNKDREG('LR_irq'       , '0b010000'),
	TV_BNKDREG('SP_irq'       , '0b010001'),
	TV_BNKDREG('LR_svc'       , '0b010010'),
	TV_BNKDREG('SP_svc'       , '0b010011'),
	TV_BNKDREG('LR_abt'       , '0b010100'),
	TV_BNKDREG('SP_abt'       , '0b010101'),
	TV_BNKDREG('LR_und'       , '0b010110'),
	TV_BNKDREG('SP_und'       , '0b010111'),
	TV_BNKDREG('UNPREDICTABLE', '0b0110xx'),
	TV_BNKDREG('LR_mon'       , '0b011100'),
	TV_BNKDREG('SP_mon'       , '0b011101'),
	TV_BNKDREG('ELR_hyp'      , '0b011110'),
	TV_BNKDREG('SP_hyp'       , '0b011111'),
	TV_BNKDREG('UNPREDICTABLE', '0b100xxx'),
	TV_BNKDREG('UNPREDICTABLE', '0b1010xx'),
	TV_BNKDREG('UNPREDICTABLE', '0b10110x'),
	TV_BNKDREG('SPSR_fiq'     , '0b101110'),
	TV_BNKDREG('UNPREDICTABLE', '0b101111'),
	TV_BNKDREG('SPSR_irq'     , '0b110000'),
	TV_BNKDREG('UNPREDICTABLE', '0b110001'),
	TV_BNKDREG('SPSR_svc'     , '0b110010'),
	TV_BNKDREG('UNPREDICTABLE', '0b110011'),
	TV_BNKDREG('SPSR_abt'     , '0b110100'),
	TV_BNKDREG('UNPREDICTABLE', '0b110101'),
	TV_BNKDREG('SPSR_und'     , '0b110110'),
	TV_BNKDREG('UNPREDICTABLE', '0b110111'),
	TV_BNKDREG('UNPREDICTABLE', '0b1110xx'),
	TV_BNKDREG('SPSR_mon'     , '0b111100'),
	TV_BNKDREG('UNPREDICTABLE', '0b111101'),
	TV_BNKDREG('SPSR_hyp'     , '0b111110'),
	TV_BNKDREG('UNPREDICTABLE', '0b111111'),
];

TABLE tbl_endian => [
	TV_ENDIAN('LE', '0b0'),
	TV_ENDIAN('BE', '0b1'),
];

TABLE tbl_imm0 => [
	TV_IMM('OMITTED'  , '0b00'),
	TV_IMM('8'        , '0b01'),
	TV_IMM('16'       , '0b10'),
	TV_IMM('24'       , '0b11'),
];

TABLE tbl_imm1 => [
	TV_IMM('0'  , '0b00'),
	TV_IMM('90' , '0b01'),
	TV_IMM('180', '0b10'),
	TV_IMM('270', '0b11'),
];

TABLE tbl_imm2 => [
	TV_IMM('90' , '0b0'),
	TV_IMM('270', '0b1'),
];

TABLE tbl_inc => [
	TV_INC('1', '0b0'),
	TV_INC('2', '0b1'),
];

TABLE tbl_inc1 => [
	TV_INC('1'        , '0b0'),
	TV_INC('UNDEFINED', '0b1'),
];

TABLE tbl_inc2 => [
	TV_INC('1', '0b00'),
	TV_INC('2', '0b10'),
];

TABLE tbl_inc3 => [
	TV_INC('1', '0b000'),
	TV_INC('2', '0b100'),
];

TABLE tbl_pspace => [
	TV_PSPACE('p14', '0b0'),
	TV_PSPACE('p15', '0b1'),
];

TABLE tbl_shift0 => [
	TV_SHIFT('LSL', '0b00'),
	TV_SHIFT('LSR', '0b01'),
	TV_SHIFT('ASR', '0b10'),
];

TABLE tbl_shift1 => [
	TV_SHIFT('LSL', '0b00'),
	TV_SHIFT('LSR', '0b01'),
	TV_SHIFT('ASR', '0b10'),
	TV_SHIFT('ROR', '0b11'),
];

TABLE tbl_specreg => [
	TV_SPECREG('CPSR', '0b0', 'HASALIAS'),
	TV_SPECREG('SPSR', '0b1'),
	TV_SPECREG('APSR', '0b0', 'ALIAS'),
];

TABLE tbl_specreg1 => [
	TV_SPECREG('FPSID'        , '0b0000'),
	TV_SPECREG('FPSCR'        , '0b0001'),
	TV_SPECREG('UNPREDICTABLE', '0b001x'),
	TV_SPECREG('UNPREDICTABLE', '0b0100'),
	TV_SPECREG('MVFR2'        , '0b0101'),
	TV_SPECREG('MVFR1'        , '0b0110'),
	TV_SPECREG('MVFR0'        , '0b0111'),
	TV_SPECREG('FPEXC'        , '0b1000'),
	TV_SPECREG('UNPREDICTABLE', '0b1001'),
	TV_SPECREG('UNPREDICTABLE', '0b101x'),
	TV_SPECREG('UNPREDICTABLE', '0b11xx'),
];

TABLE tbl_specreg2 => [
	TV_SPECREG('FPSID'        , '0b0000'),
	TV_SPECREG('FPSCR'        , '0b0001'),
	TV_SPECREG('UNPREDICTABLE', '0b001x'),
	TV_SPECREG('UNPREDICTABLE', '0b01xx'),
	TV_SPECREG('FPEXC'        , '0b1000'),
	TV_SPECREG('UNPREDICTABLE', '0b1001'),
	TV_SPECREG('UNPREDICTABLE', '0b101x'),
	TV_SPECREG('UNPREDICTABLE', '0b11xx'),
];


1;

