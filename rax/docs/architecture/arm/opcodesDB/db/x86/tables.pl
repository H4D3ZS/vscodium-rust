use strict;
use warnings;

# ============================================== WIDTHS ==============================================

# usually for GPR/IMM:
DEF SZ_V => { encodedin => 'OSZ', table => [16,32,64] };
DEF SZ_Y => { encodedin => 'OSZ', table => [32,32,64] };
DEF SZ_Z => { encodedin => 'OSZ', table => [16,32,32] };

# usually for FP:
DEF SZ_DPP => { encodedin => 'OSZ', table => [32,48,48]   };
DEF SZ_DPR => { encodedin => 'OSZ', table => [32,48,80]   };
DEF SZ_RST => { encodedin => 'OSZ', table => [80,160,320] };
DEF SZ_PPR => { encodedin => 'OSZ', table => [48,48,80]   };

# usually for SIMD:
DEF SZ_F => { encodedin => 'OSZ', table => [32,64,128]   };
DEF SZ_X => { encodedin => 'OSZ', table => [128,128,256] };

# scale with address size:
DEF SZ_A => { encodedin => 'ASZ', table => [16,32,64]    };



#  ============================================ REGISTERS ============================================

REG TABLE AXa_TBL => [
	{ reg => 'AX' }, # 0 asz=0 encoding=0b00
	{ reg => 'EAX' }, # 1 asz=1 encoding=0b01
	{ reg => 'RAX' }, # 2 asz=2 encoding=0b10
];

REG TABLE CXa_TBL => [
	{ reg => 'CX' },  # 0 asz=0 encoding=0b00
	{ reg => 'ECX' }, # 1 asz=1 encoding=0b01
	{ reg => 'RCX' }, # 2 asz=2 encoding=0b10
];

REG TABLE DXa_TBL => [
	{ reg => 'DX' },  # 0 asz=0 encoding=0b00
	{ reg => 'EDX' }, # 1 asz=1 encoding=0b01
	{ reg => 'RDX' }, # 2 asz=2 encoding=0b10
];

REG TABLE BXa_TBL => [
	{ reg => 'BX' },  # 0 asz=0 encoding=0b00
	{ reg => 'EBX' }, # 1 asz=1 encoding=0b01
	{ reg => 'RBX' }, # 2 asz=2 encoding=0b10
];

REG TABLE SPa_TBL => [
	{ reg => 'SP' },  # 0 asz=0 encoding=0b00
	{ reg => 'ESP' }, # 1 asz=1 encoding=0b01
	{ reg => 'RSP' }, # 2 asz=2 encoding=0b10
];

REG TABLE BPa_TBL => [
	{ reg => 'BP' },  # 0 asz=0 encoding=0b00
	{ reg => 'EBP' }, # 1 asz=1 encoding=0b01
	{ reg => 'RBP' }, # 2 asz=2 encoding=0b10
];

REG TABLE SIa_TBL => [
	{ reg => 'SI' },  # 0 asz=0 encoding=0b00
	{ reg => 'ESI' }, # 1 asz=1 encoding=0b01
	{ reg => 'RSI' }, # 2 asz=2 encoding=0b10
];

REG TABLE DIa_TBL => [
	{ reg => 'DI' },  # 0 asz=0 encoding=0b00
	{ reg => 'EDI' }, # 1 asz=1 encoding=0b01
	{ reg => 'RDI' }, # 2 asz=2 encoding=0b10
];

REG TABLE AXv_TBL => [
	{ reg => 'AX' },  # 0 osz=0 encoding=0b00
	{ reg => 'EAX' }, # 1 osz=1 encoding=0b01
	{ reg => 'RAX' }, # 2 osz=2 encoding=0b10
];

REG TABLE CXv_TBL => [
	{ reg => 'CX' },  # 0 osz=0 encoding=0b00
	{ reg => 'ECX' }, # 1 osz=1 encoding=0b01
	{ reg => 'RCX' }, # 2 osz=2 encoding=0b10
];

REG TABLE DXv_TBL => [
	{ reg => 'DX' },  # 0 osz=0 encoding=0b00
	{ reg => 'EDX' }, # 1 osz=1 encoding=0b01
	{ reg => 'RDX' }, # 2 osz=2 encoding=0b10
];

REG TABLE BXv_TBL => [
	{ reg => 'BX' },  # 0 osz=0 encoding=0b00
	{ reg => 'EBX' }, # 1 osz=1 encoding=0b01
	{ reg => 'RBX' }, # 2 osz=2 encoding=0b10
];

REG TABLE SPv_TBL => [
	{ reg => 'SP' },  # 0 osz=0 encoding=0b00
	{ reg => 'ESP' }, # 1 osz=1 encoding=0b01
	{ reg => 'RSP' }, # 2 osz=2 encoding=0b10
];

REG TABLE BPv_TBL => [
	{ reg => 'BP' },  # 0 osz=0 encoding=0b00
	{ reg => 'EBP' }, # 1 osz=1 encoding=0b01
	{ reg => 'RBP' }, # 2 osz=2 encoding=0b10
];

REG TABLE SIv_TBL => [
	{ reg => 'SI' },  # 0 osz=0 encoding=0b00
	{ reg => 'ESI' }, # 1 osz=1 encoding=0b01
	{ reg => 'RSI' }, # 2 osz=2 encoding=0b10
];

REG TABLE DIv_TBL => [
	{ reg => 'DI' },  # 0 osz=0 encoding=0b00
	{ reg => 'EDI' }, # 1 osz=1 encoding=0b01
	{ reg => 'RDI' }, # 2 osz=2 encoding=0b10
];

REG TABLE GPR8_TBL => [
	{ reg => 'AL' },   # 0 idx=0 r=0 rex=0 encoding=0b00000
	{ reg => 'CL' },   # 1 idx=1 r=0 rex=0 encoding=0b00001
	{ reg => 'DL' },   # 2 idx=2 r=0 rex=0 encoding=0b00010
	{ reg => 'BL' },   # 3 idx=3 r=0 rex=0 encoding=0b00011
	{ reg => 'AH' },   # 4 idx=4 r=0 rex=0 encoding=0b00100
	{ reg => 'CH' },   # 5 idx=5 r=0 rex=0 encoding=0b00101
	{ reg => 'DH' },   # 6 idx=6 r=0 rex=0 encoding=0b00110
	{ reg => 'BH' },   # 7 idx=7 r=0 rex=0 encoding=0b00111
	{ reg => 'NONE' }, # 8 idx=0 r=1 rex=0 encoding=0b01000
	{ reg => 'NONE' }, # 9 idx=1 r=1 rex=0 encoding=0b01001
	{ reg => 'NONE' }, # 10 idx=2 r=1 rex=0 encoding=0b01010
	{ reg => 'NONE' }, # 11 idx=3 r=1 rex=0 encoding=0b01011
	{ reg => 'NONE' }, # 12 idx=4 r=1 rex=0 encoding=0b01100
	{ reg => 'NONE' }, # 13 idx=5 r=1 rex=0 encoding=0b01101
	{ reg => 'NONE' }, # 14 idx=6 r=1 rex=0 encoding=0b01110
	{ reg => 'NONE' }, # 15 idx=7 r=1 rex=0 encoding=0b01111
	{ reg => 'AL' },   # 16 idx=0 r=0 rex=1 encoding=0b10000
	{ reg => 'CL' },   # 17 idx=1 r=0 rex=1 encoding=0b10001
	{ reg => 'DL' },   # 18 idx=2 r=0 rex=1 encoding=0b10010
	{ reg => 'BL' },   # 19 idx=3 r=0 rex=1 encoding=0b10011
	{ reg => 'SPL' },  # 20 idx=4 r=0 rex=1 encoding=0b10100
	{ reg => 'BPL' },  # 21 idx=5 r=0 rex=1 encoding=0b10101
	{ reg => 'SIL' },  # 22 idx=6 r=0 rex=1 encoding=0b10110
	{ reg => 'DIL' },  # 23 idx=7 r=0 rex=1 encoding=0b10111
	{ reg => 'R8B' },  # 24 idx=0 r=1 rex=1 encoding=0b11000
	{ reg => 'R9B' },  # 25 idx=1 r=1 rex=1 encoding=0b11001
	{ reg => 'R10B' }, # 26 idx=2 r=1 rex=1 encoding=0b11010
	{ reg => 'R11B' }, # 27 idx=3 r=1 rex=1 encoding=0b11011
	{ reg => 'R12B' }, # 28 idx=4 r=1 rex=1 encoding=0b11100
	{ reg => 'R13B' }, # 29 idx=5 r=1 rex=1 encoding=0b11101
	{ reg => 'R14B' }, # 30 idx=6 r=1 rex=1 encoding=0b11110
	{ reg => 'R15B' }, # 31 idx=7 r=1 rex=1 encoding=0b11111
];

REG TABLE GPR16_TBL => [
	{ reg => 'AX' },   # 0 idx=0 r=0 encoding=0b0000
	{ reg => 'CX' },   # 1 idx=1 r=0 encoding=0b0001
	{ reg => 'DX' },   # 2 idx=2 r=0 encoding=0b0010
	{ reg => 'BX' },   # 3 idx=3 r=0 encoding=0b0011
	{ reg => 'SP' },   # 4 idx=4 r=0 encoding=0b0100
	{ reg => 'BP' },   # 5 idx=5 r=0 encoding=0b0101
	{ reg => 'SI' },   # 6 idx=6 r=0 encoding=0b0110
	{ reg => 'DI' },   # 7 idx=7 r=0 encoding=0b0111
	{ reg => 'R8W' },  # 8 idx=0 r=1 encoding=0b1000
	{ reg => 'R9W' },  # 9 idx=1 r=1 encoding=0b1001
	{ reg => 'R10W' }, # 10 idx=2 r=1 encoding=0b1010
	{ reg => 'R11W' }, # 11 idx=3 r=1 encoding=0b1011
	{ reg => 'R12W' }, # 12 idx=4 r=1 encoding=0b1100
	{ reg => 'R13W' }, # 13 idx=5 r=1 encoding=0b1101
	{ reg => 'R14W' }, # 14 idx=6 r=1 encoding=0b1110
	{ reg => 'R15W' }, # 15 idx=7 r=1 encoding=0b1111
];

REG TABLE GPR32_TBL => [
	{ reg => 'EAX' },  # 0 idx=0 r=0 encoding=0b0000
	{ reg => 'ECX' },  # 1 idx=1 r=0 encoding=0b0001
	{ reg => 'EDX' },  # 2 idx=2 r=0 encoding=0b0010
	{ reg => 'EBX' },  # 3 idx=3 r=0 encoding=0b0011
	{ reg => 'ESP' },  # 4 idx=4 r=0 encoding=0b0100
	{ reg => 'EBP' },  # 5 idx=5 r=0 encoding=0b0101
	{ reg => 'ESI' },  # 6 idx=6 r=0 encoding=0b0110
	{ reg => 'EDI' },  # 7 idx=7 r=0 encoding=0b0111
	{ reg => 'R8D' },  # 8 idx=0 r=1 encoding=0b1000
	{ reg => 'R9D' },  # 9 idx=1 r=1 encoding=0b1001
	{ reg => 'R10D' }, # 10 idx=2 r=1 encoding=0b1010
	{ reg => 'R11D' }, # 11 idx=3 r=1 encoding=0b1011
	{ reg => 'R12D' }, # 12 idx=4 r=1 encoding=0b1100
	{ reg => 'R13D' }, # 13 idx=5 r=1 encoding=0b1101
	{ reg => 'R14D' }, # 14 idx=6 r=1 encoding=0b1110
	{ reg => 'R15D' }, # 15 idx=7 r=1 encoding=0b1111
];

REG TABLE GPR64_TBL => [
	{ reg => 'RAX' }, # 0 idx=0 r=0 encoding=0b0000
	{ reg => 'RCX' }, # 1 idx=1 r=0 encoding=0b0001
	{ reg => 'RDX' }, # 2 idx=2 r=0 encoding=0b0010
	{ reg => 'RBX' }, # 3 idx=3 r=0 encoding=0b0011
	{ reg => 'RSP' }, # 4 idx=4 r=0 encoding=0b0100
	{ reg => 'RBP' }, # 5 idx=5 r=0 encoding=0b0101
	{ reg => 'RSI' }, # 6 idx=6 r=0 encoding=0b0110
	{ reg => 'RDI' }, # 7 idx=7 r=0 encoding=0b0111
	{ reg => 'R8' },  # 8 idx=0 r=1 encoding=0b1000
	{ reg => 'R9' },  # 9 idx=1 r=1 encoding=0b1001
	{ reg => 'R10' }, # 10 idx=2 r=1 encoding=0b1010
	{ reg => 'R11' }, # 11 idx=3 r=1 encoding=0b1011
	{ reg => 'R12' }, # 12 idx=4 r=1 encoding=0b1100
	{ reg => 'R13' }, # 13 idx=5 r=1 encoding=0b1101
	{ reg => 'R14' }, # 14 idx=6 r=1 encoding=0b1110
	{ reg => 'R15' }, # 15 idx=7 r=1 encoding=0b1111
];

REG TABLE GPRv_TBL => [
	{ reg => 'AX' },   # 0 idx=0 r=0 osz=0 encoding=0b000000
	{ reg => 'CX' },   # 1 idx=1 r=0 osz=0 encoding=0b000001
	{ reg => 'DX' },   # 2 idx=2 r=0 osz=0 encoding=0b000010
	{ reg => 'BX' },   # 3 idx=3 r=0 osz=0 encoding=0b000011
	{ reg => 'SP' },   # 4 idx=4 r=0 osz=0 encoding=0b000100
	{ reg => 'BP' },   # 5 idx=5 r=0 osz=0 encoding=0b000101
	{ reg => 'SI' },   # 6 idx=6 r=0 osz=0 encoding=0b000110
	{ reg => 'DI' },   # 7 idx=7 r=0 osz=0 encoding=0b000111
	{ reg => 'R8W' },  # 8 idx=0 r=1 osz=0 encoding=0b001000
	{ reg => 'R9W' },  # 9 idx=1 r=1 osz=0 encoding=0b001001
	{ reg => 'R10W' }, # 10 idx=2 r=1 osz=0 encoding=0b001010
	{ reg => 'R11W' }, # 11 idx=3 r=1 osz=0 encoding=0b001011
	{ reg => 'R12W' }, # 12 idx=4 r=1 osz=0 encoding=0b001100
	{ reg => 'R13W' }, # 13 idx=5 r=1 osz=0 encoding=0b001101
	{ reg => 'R14W' }, # 14 idx=6 r=1 osz=0 encoding=0b001110
	{ reg => 'R15W' }, # 15 idx=7 r=1 osz=0 encoding=0b001111
	{ reg => 'EAX' },  # 16 idx=0 r=0 osz=1 encoding=0b010000
	{ reg => 'ECX' },  # 17 idx=1 r=0 osz=1 encoding=0b010001
	{ reg => 'EDX' },  # 18 idx=2 r=0 osz=1 encoding=0b010010
	{ reg => 'EBX' },  # 19 idx=3 r=0 osz=1 encoding=0b010011
	{ reg => 'ESP' },  # 20 idx=4 r=0 osz=1 encoding=0b010100
	{ reg => 'EBP' },  # 21 idx=5 r=0 osz=1 encoding=0b010101
	{ reg => 'ESI' },  # 22 idx=6 r=0 osz=1 encoding=0b010110
	{ reg => 'EDI' },  # 23 idx=7 r=0 osz=1 encoding=0b010111
	{ reg => 'R8D' },  # 24 idx=0 r=1 osz=1 encoding=0b011000
	{ reg => 'R9D' },  # 25 idx=1 r=1 osz=1 encoding=0b011001
	{ reg => 'R10D' }, # 26 idx=2 r=1 osz=1 encoding=0b011010
	{ reg => 'R11D' }, # 27 idx=3 r=1 osz=1 encoding=0b011011
	{ reg => 'R12D' }, # 28 idx=4 r=1 osz=1 encoding=0b011100
	{ reg => 'R13D' }, # 29 idx=5 r=1 osz=1 encoding=0b011101
	{ reg => 'R14D' }, # 30 idx=6 r=1 osz=1 encoding=0b011110
	{ reg => 'R15D' }, # 31 idx=7 r=1 osz=1 encoding=0b011111
	{ reg => 'RAX' }, # 32 idx=0 r=0 osz=2 encoding=0b100000
	{ reg => 'RCX' }, # 33 idx=1 r=0 osz=2 encoding=0b100001
	{ reg => 'RDX' }, # 34 idx=2 r=0 osz=2 encoding=0b100010
	{ reg => 'RBX' }, # 35 idx=3 r=0 osz=2 encoding=0b100011
	{ reg => 'RSP' }, # 36 idx=4 r=0 osz=2 encoding=0b100100
	{ reg => 'RBP' }, # 37 idx=5 r=0 osz=2 encoding=0b100101
	{ reg => 'RSI' }, # 38 idx=6 r=0 osz=2 encoding=0b100110
	{ reg => 'RDI' }, # 39 idx=7 r=0 osz=2 encoding=0b100111
	{ reg => 'R8' }, # 40 idx=0 r=1 osz=2 encoding=0b101000
	{ reg => 'R9' }, # 41 idx=1 r=1 osz=2 encoding=0b101001
	{ reg => 'R10' }, # 42 idx=2 r=1 osz=2 encoding=0b101010
	{ reg => 'R11' }, # 43 idx=3 r=1 osz=2 encoding=0b101011
	{ reg => 'R12' }, # 44 idx=4 r=1 osz=2 encoding=0b101100
	{ reg => 'R13' }, # 45 idx=5 r=1 osz=2 encoding=0b101101
	{ reg => 'R14' }, # 46 idx=6 r=1 osz=2 encoding=0b101110
	{ reg => 'R15' }, # 47 idx=7 r=1 osz=2 encoding=0b101111
];

REG TABLE GPRz_TBL => [
	{ reg => 'AX' }, # 0 idx=0 r=0 osz=0 encoding=0b000000
	{ reg => 'CX' }, # 1 idx=1 r=0 osz=0 encoding=0b000001
	{ reg => 'DX' }, # 2 idx=2 r=0 osz=0 encoding=0b000010
	{ reg => 'BX' }, # 3 idx=3 r=0 osz=0 encoding=0b000011
	{ reg => 'SP' }, # 4 idx=4 r=0 osz=0 encoding=0b000100
	{ reg => 'BP' }, # 5 idx=5 r=0 osz=0 encoding=0b000101
	{ reg => 'SI' }, # 6 idx=6 r=0 osz=0 encoding=0b000110
	{ reg => 'DI' }, # 7 idx=7 r=0 osz=0 encoding=0b000111
	{ reg => 'R8W' }, # 8 idx=0 r=1 osz=0 encoding=0b001000
	{ reg => 'R9W' }, # 9 idx=1 r=1 osz=0 encoding=0b001001
	{ reg => 'R10W' }, # 10 idx=2 r=1 osz=0 encoding=0b001010
	{ reg => 'R11W' }, # 11 idx=3 r=1 osz=0 encoding=0b001011
	{ reg => 'R12W' }, # 12 idx=4 r=1 osz=0 encoding=0b001100
	{ reg => 'R13W' }, # 13 idx=5 r=1 osz=0 encoding=0b001101
	{ reg => 'R14W' }, # 14 idx=6 r=1 osz=0 encoding=0b001110
	{ reg => 'R15W' }, # 15 idx=7 r=1 osz=0 encoding=0b001111
	{ reg => 'EAX' }, # 16 idx=0 r=0 osz=1 encoding=0b010000
	{ reg => 'ECX' }, # 17 idx=1 r=0 osz=1 encoding=0b010001
	{ reg => 'EDX' }, # 18 idx=2 r=0 osz=1 encoding=0b010010
	{ reg => 'EBX' }, # 19 idx=3 r=0 osz=1 encoding=0b010011
	{ reg => 'ESP' }, # 20 idx=4 r=0 osz=1 encoding=0b010100
	{ reg => 'EBP' }, # 21 idx=5 r=0 osz=1 encoding=0b010101
	{ reg => 'ESI' }, # 22 idx=6 r=0 osz=1 encoding=0b010110
	{ reg => 'EDI' }, # 23 idx=7 r=0 osz=1 encoding=0b010111
	{ reg => 'R8D' }, # 24 idx=0 r=1 osz=1 encoding=0b011000
	{ reg => 'R9D' }, # 25 idx=1 r=1 osz=1 encoding=0b011001
	{ reg => 'R10D' }, # 26 idx=2 r=1 osz=1 encoding=0b011010
	{ reg => 'R11D' }, # 27 idx=3 r=1 osz=1 encoding=0b011011
	{ reg => 'R12D' }, # 28 idx=4 r=1 osz=1 encoding=0b011100
	{ reg => 'R13D' }, # 29 idx=5 r=1 osz=1 encoding=0b011101
	{ reg => 'R14D' }, # 30 idx=6 r=1 osz=1 encoding=0b011110
	{ reg => 'R15D' }, # 31 idx=7 r=1 osz=1 encoding=0b011111
	{ reg => 'EAX' }, # 32 idx=0 r=0 osz=2 encoding=0b100000
	{ reg => 'ECX' }, # 33 idx=1 r=0 osz=2 encoding=0b100001
	{ reg => 'EDX' }, # 34 idx=2 r=0 osz=2 encoding=0b100010
	{ reg => 'EBX' }, # 35 idx=3 r=0 osz=2 encoding=0b100011
	{ reg => 'ESP' }, # 36 idx=4 r=0 osz=2 encoding=0b100100
	{ reg => 'EBP' }, # 37 idx=5 r=0 osz=2 encoding=0b100101
	{ reg => 'ESI' }, # 38 idx=6 r=0 osz=2 encoding=0b100110
	{ reg => 'EDI' }, # 39 idx=7 r=0 osz=2 encoding=0b100111
	{ reg => 'R8D' }, # 40 idx=0 r=1 osz=2 encoding=0b101000
	{ reg => 'R9D' }, # 41 idx=1 r=1 osz=2 encoding=0b101001
	{ reg => 'R10D' }, # 42 idx=2 r=1 osz=2 encoding=0b101010
	{ reg => 'R11D' }, # 43 idx=3 r=1 osz=2 encoding=0b101011
	{ reg => 'R12D' }, # 44 idx=4 r=1 osz=2 encoding=0b101100
	{ reg => 'R13D' }, # 45 idx=5 r=1 osz=2 encoding=0b101101
	{ reg => 'R14D' }, # 46 idx=6 r=1 osz=2 encoding=0b101110
	{ reg => 'R15D' }, # 47 idx=7 r=1 osz=2 encoding=0b101111
];

REG TABLE GPRy_TBL => [
	{ reg => 'EAX' }, # 0 idx=0 r=0 osz=0 encoding=0b000000
	{ reg => 'ECX' }, # 1 idx=1 r=0 osz=0 encoding=0b000001
	{ reg => 'EDX' }, # 2 idx=2 r=0 osz=0 encoding=0b000010
	{ reg => 'EBX' }, # 3 idx=3 r=0 osz=0 encoding=0b000011
	{ reg => 'ESP' }, # 4 idx=4 r=0 osz=0 encoding=0b000100
	{ reg => 'EBP' }, # 5 idx=5 r=0 osz=0 encoding=0b000101
	{ reg => 'ESI' }, # 6 idx=6 r=0 osz=0 encoding=0b000110
	{ reg => 'EDI' }, # 7 idx=7 r=0 osz=0 encoding=0b000111
	{ reg => 'R8D' }, # 8 idx=0 r=1 osz=0 encoding=0b001000
	{ reg => 'R9D' }, # 9 idx=1 r=1 osz=0 encoding=0b001001
	{ reg => 'R10D' }, # 10 idx=2 r=1 osz=0 encoding=0b001010
	{ reg => 'R11D' }, # 11 idx=3 r=1 osz=0 encoding=0b001011
	{ reg => 'R12D' }, # 12 idx=4 r=1 osz=0 encoding=0b001100
	{ reg => 'R13D' }, # 13 idx=5 r=1 osz=0 encoding=0b001101
	{ reg => 'R14D' }, # 14 idx=6 r=1 osz=0 encoding=0b001110
	{ reg => 'R15D' }, # 15 idx=7 r=1 osz=0 encoding=0b001111
	{ reg => 'EAX' }, # 16 idx=0 r=0 osz=1 encoding=0b010000
	{ reg => 'ECX' }, # 17 idx=1 r=0 osz=1 encoding=0b010001
	{ reg => 'EDX' }, # 18 idx=2 r=0 osz=1 encoding=0b010010
	{ reg => 'EBX' }, # 19 idx=3 r=0 osz=1 encoding=0b010011
	{ reg => 'ESP' }, # 20 idx=4 r=0 osz=1 encoding=0b010100
	{ reg => 'EBP' }, # 21 idx=5 r=0 osz=1 encoding=0b010101
	{ reg => 'ESI' }, # 22 idx=6 r=0 osz=1 encoding=0b010110
	{ reg => 'EDI' }, # 23 idx=7 r=0 osz=1 encoding=0b010111
	{ reg => 'R8D' }, # 24 idx=0 r=1 osz=1 encoding=0b011000
	{ reg => 'R9D' }, # 25 idx=1 r=1 osz=1 encoding=0b011001
	{ reg => 'R10D' }, # 26 idx=2 r=1 osz=1 encoding=0b011010
	{ reg => 'R11D' }, # 27 idx=3 r=1 osz=1 encoding=0b011011
	{ reg => 'R12D' }, # 28 idx=4 r=1 osz=1 encoding=0b011100
	{ reg => 'R13D' }, # 29 idx=5 r=1 osz=1 encoding=0b011101
	{ reg => 'R14D' }, # 30 idx=6 r=1 osz=1 encoding=0b011110
	{ reg => 'R15D' }, # 31 idx=7 r=1 osz=1 encoding=0b011111
	{ reg => 'RAX' }, # 32 idx=0 r=0 osz=2 encoding=0b100000
	{ reg => 'RCX' }, # 33 idx=1 r=0 osz=2 encoding=0b100001
	{ reg => 'RDX' }, # 34 idx=2 r=0 osz=2 encoding=0b100010
	{ reg => 'RBX' }, # 35 idx=3 r=0 osz=2 encoding=0b100011
	{ reg => 'RSP' }, # 36 idx=4 r=0 osz=2 encoding=0b100100
	{ reg => 'RBP' }, # 37 idx=5 r=0 osz=2 encoding=0b100101
	{ reg => 'RSI' }, # 38 idx=6 r=0 osz=2 encoding=0b100110
	{ reg => 'RDI' }, # 39 idx=7 r=0 osz=2 encoding=0b100111
	{ reg => 'R8' }, # 40 idx=0 r=1 osz=2 encoding=0b101000
	{ reg => 'R9' }, # 41 idx=1 r=1 osz=2 encoding=0b101001
	{ reg => 'R10' }, # 42 idx=2 r=1 osz=2 encoding=0b101010
	{ reg => 'R11' }, # 43 idx=3 r=1 osz=2 encoding=0b101011
	{ reg => 'R12' }, # 44 idx=4 r=1 osz=2 encoding=0b101100
	{ reg => 'R13' }, # 45 idx=5 r=1 osz=2 encoding=0b101101
	{ reg => 'R14' }, # 46 idx=6 r=1 osz=2 encoding=0b101110
	{ reg => 'R15' }, # 47 idx=7 r=1 osz=2 encoding=0b101111
];

REG TABLE SEGREG_TBL => [
	{ reg => 'ES' }, # 0 idx=0 encoding=0b000
	{ reg => 'CS' }, # 1 idx=1 encoding=0b001
	{ reg => 'SS' }, # 2 idx=2 encoding=0b010
	{ reg => 'DS' }, # 3 idx=3 encoding=0b011
	{ reg => 'FS' }, # 4 idx=4 encoding=0b100
	{ reg => 'GS' }, # 5 idx=5 encoding=0b101
	{ reg => 'NONE' }, # 6 idx=6 encoding=0b110
	{ reg => 'NONE' }, # 7 idx=7 encoding=0b111
];

REG TABLE FPREG_TBL => [
	{ reg => 'ST0' }, # 0 idx=0 encoding=0b000
	{ reg => 'ST1' }, # 1 idx=1 encoding=0b001
	{ reg => 'ST2' }, # 2 idx=2 encoding=0b010
	{ reg => 'ST3' }, # 3 idx=3 encoding=0b011
	{ reg => 'ST4' }, # 4 idx=4 encoding=0b100
	{ reg => 'ST5' }, # 5 idx=5 encoding=0b101
	{ reg => 'ST6' }, # 6 idx=6 encoding=0b110
	{ reg => 'ST7' }, # 7 idx=7 encoding=0b111
];

REG TABLE MMXREG_TBL => [
	{ reg => 'MM0' }, # 0 idx=0 encoding=0b000
	{ reg => 'MM1' }, # 1 idx=1 encoding=0b001
	{ reg => 'MM2' }, # 2 idx=2 encoding=0b010
	{ reg => 'MM3' }, # 3 idx=3 encoding=0b011
	{ reg => 'MM4' }, # 4 idx=4 encoding=0b100
	{ reg => 'MM5' }, # 5 idx=5 encoding=0b101
	{ reg => 'MM6' }, # 6 idx=6 encoding=0b110
	{ reg => 'MM7' }, # 7 idx=7 encoding=0b111
];

REG TABLE CNTRLREG_TBL => [
	{ reg => 'CR0' }, # 0 idx=0 r=0 encoding=0b0000
	{ reg => 'CR1' }, # 1 idx=1 r=0 encoding=0b0001
	{ reg => 'CR2' }, # 2 idx=2 r=0 encoding=0b0010
	{ reg => 'CR3' }, # 3 idx=3 r=0 encoding=0b0011
	{ reg => 'CR4' }, # 4 idx=4 r=0 encoding=0b0100
	{ reg => 'CR5' }, # 5 idx=5 r=0 encoding=0b0101
	{ reg => 'CR6' }, # 6 idx=6 r=0 encoding=0b0110
	{ reg => 'CR7' }, # 7 idx=7 r=0 encoding=0b0111
	{ reg => 'CR8' }, # 8 idx=0 r=1 encoding=0b1000
	{ reg => 'CR9' }, # 9 idx=1 r=1 encoding=0b1001
	{ reg => 'CR10' }, # 10 idx=2 r=1 encoding=0b1010
	{ reg => 'CR11' }, # 11 idx=3 r=1 encoding=0b1011
	{ reg => 'CR12' }, # 12 idx=4 r=1 encoding=0b1100
	{ reg => 'CR13' }, # 13 idx=5 r=1 encoding=0b1101
	{ reg => 'CR14' }, # 14 idx=6 r=1 encoding=0b1110
	{ reg => 'CR15' }, # 15 idx=7 r=1 encoding=0b1111
];

REG TABLE DBGREG_TBL => [
	{ reg => 'DR0' }, # 0 idx=0 r=0 encoding=0b0000
	{ reg => 'DR1' }, # 1 idx=1 r=0 encoding=0b0001
	{ reg => 'DR2' }, # 2 idx=2 r=0 encoding=0b0010
	{ reg => 'DR3' }, # 3 idx=3 r=0 encoding=0b0011
	{ reg => 'DR4' }, # 4 idx=4 r=0 encoding=0b0100
	{ reg => 'DR5' }, # 5 idx=5 r=0 encoding=0b0101
	{ reg => 'DR6' }, # 6 idx=6 r=0 encoding=0b0110
	{ reg => 'DR7' }, # 7 idx=7 r=0 encoding=0b0111
	{ reg => 'DR8' }, # 8 idx=0 r=1 encoding=0b1000
	{ reg => 'DR9' }, # 9 idx=1 r=1 encoding=0b1001
	{ reg => 'DR10' }, # 10 idx=2 r=1 encoding=0b1010
	{ reg => 'DR11' }, # 11 idx=3 r=1 encoding=0b1011
	{ reg => 'DR12' }, # 12 idx=4 r=1 encoding=0b1100
	{ reg => 'DR13' }, # 13 idx=5 r=1 encoding=0b1101
	{ reg => 'DR14' }, # 14 idx=6 r=1 encoding=0b1110
	{ reg => 'DR15' }, # 15 idx=7 r=1 encoding=0b1111
];

REG TABLE KREG_TBL => [
	{ reg => 'K0' }, # 0 idx=0 encoding=0b000
	{ reg => 'K1' }, # 1 idx=1 encoding=0b001
	{ reg => 'K2' }, # 2 idx=2 encoding=0b010
	{ reg => 'K3' }, # 3 idx=3 encoding=0b011
	{ reg => 'K4' }, # 4 idx=4 encoding=0b100
	{ reg => 'K5' }, # 5 idx=5 encoding=0b101
	{ reg => 'K6' }, # 6 idx=6 encoding=0b110
	{ reg => 'K7' }, # 7 idx=7 encoding=0b111
];

REG TABLE BNDREG_TBL => [
	{ reg => 'BND0' }, # 0 idx=0 encoding=0b000
	{ reg => 'BND1' }, # 1 idx=1 encoding=0b001
	{ reg => 'BND2' }, # 2 idx=2 encoding=0b010
	{ reg => 'BND3' }, # 3 idx=3 encoding=0b011
	{ reg => 'NONE' }, # 4 idx=4 encoding=0b100
	{ reg => 'NONE' }, # 5 idx=5 encoding=0b101
	{ reg => 'NONE' }, # 6 idx=6 encoding=0b110
	{ reg => 'NONE' }, # 7 idx=7 encoding=0b111
];

REG TABLE XMMREG_TBL => [
	{ reg => 'XMM0' }, # 0 idx=0 r=0 rp=0 encoding=0b00000
	{ reg => 'XMM1' }, # 1 idx=1 r=0 rp=0 encoding=0b00001
	{ reg => 'XMM2' }, # 2 idx=2 r=0 rp=0 encoding=0b00010
	{ reg => 'XMM3' }, # 3 idx=3 r=0 rp=0 encoding=0b00011
	{ reg => 'XMM4' }, # 4 idx=4 r=0 rp=0 encoding=0b00100
	{ reg => 'XMM5' }, # 5 idx=5 r=0 rp=0 encoding=0b00101
	{ reg => 'XMM6' }, # 6 idx=6 r=0 rp=0 encoding=0b00110
	{ reg => 'XMM7' }, # 7 idx=7 r=0 rp=0 encoding=0b00111
	{ reg => 'XMM8' }, # 8 idx=0 r=1 rp=0 encoding=0b01000
	{ reg => 'XMM9' }, # 9 idx=1 r=1 rp=0 encoding=0b01001
	{ reg => 'XMM10' }, # 10 idx=2 r=1 rp=0 encoding=0b01010
	{ reg => 'XMM11' }, # 11 idx=3 r=1 rp=0 encoding=0b01011
	{ reg => 'XMM12' }, # 12 idx=4 r=1 rp=0 encoding=0b01100
	{ reg => 'XMM13' }, # 13 idx=5 r=1 rp=0 encoding=0b01101
	{ reg => 'XMM14' }, # 14 idx=6 r=1 rp=0 encoding=0b01110
	{ reg => 'XMM15' }, # 15 idx=7 r=1 rp=0 encoding=0b01111
	{ reg => 'XMM16' }, # 16 idx=0 r=0 rp=1 encoding=0b10000
	{ reg => 'XMM17' }, # 17 idx=1 r=0 rp=1 encoding=0b10001
	{ reg => 'XMM18' }, # 18 idx=2 r=0 rp=1 encoding=0b10010
	{ reg => 'XMM19' }, # 19 idx=3 r=0 rp=1 encoding=0b10011
	{ reg => 'XMM20' }, # 20 idx=4 r=0 rp=1 encoding=0b10100
	{ reg => 'XMM21' }, # 21 idx=5 r=0 rp=1 encoding=0b10101
	{ reg => 'XMM22' }, # 22 idx=6 r=0 rp=1 encoding=0b10110
	{ reg => 'XMM23' }, # 23 idx=7 r=0 rp=1 encoding=0b10111
	{ reg => 'XMM24' }, # 24 idx=0 r=1 rp=1 encoding=0b11000
	{ reg => 'XMM25' }, # 25 idx=1 r=1 rp=1 encoding=0b11001
	{ reg => 'XMM26' }, # 26 idx=2 r=1 rp=1 encoding=0b11010
	{ reg => 'XMM27' }, # 27 idx=3 r=1 rp=1 encoding=0b11011
	{ reg => 'XMM28' }, # 28 idx=4 r=1 rp=1 encoding=0b11100
	{ reg => 'XMM29' }, # 29 idx=5 r=1 rp=1 encoding=0b11101
	{ reg => 'XMM30' }, # 30 idx=6 r=1 rp=1 encoding=0b11110
	{ reg => 'XMM31' }, # 31 idx=7 r=1 rp=1 encoding=0b11111
];

REG TABLE YMMREG_TBL => [
	{ reg => 'YMM0' }, # 0 idx=0 r=0 rp=0 encoding=0b00000
	{ reg => 'YMM1' }, # 1 idx=1 r=0 rp=0 encoding=0b00001
	{ reg => 'YMM2' }, # 2 idx=2 r=0 rp=0 encoding=0b00010
	{ reg => 'YMM3' }, # 3 idx=3 r=0 rp=0 encoding=0b00011
	{ reg => 'YMM4' }, # 4 idx=4 r=0 rp=0 encoding=0b00100
	{ reg => 'YMM5' }, # 5 idx=5 r=0 rp=0 encoding=0b00101
	{ reg => 'YMM6' }, # 6 idx=6 r=0 rp=0 encoding=0b00110
	{ reg => 'YMM7' }, # 7 idx=7 r=0 rp=0 encoding=0b00111
	{ reg => 'YMM8' }, # 8 idx=0 r=1 rp=0 encoding=0b01000
	{ reg => 'YMM9' }, # 9 idx=1 r=1 rp=0 encoding=0b01001
	{ reg => 'YMM10' }, # 10 idx=2 r=1 rp=0 encoding=0b01010
	{ reg => 'YMM11' }, # 11 idx=3 r=1 rp=0 encoding=0b01011
	{ reg => 'YMM12' }, # 12 idx=4 r=1 rp=0 encoding=0b01100
	{ reg => 'YMM13' }, # 13 idx=5 r=1 rp=0 encoding=0b01101
	{ reg => 'YMM14' }, # 14 idx=6 r=1 rp=0 encoding=0b01110
	{ reg => 'YMM15' }, # 15 idx=7 r=1 rp=0 encoding=0b01111
	{ reg => 'YMM16' }, # 16 idx=0 r=0 rp=1 encoding=0b10000
	{ reg => 'YMM17' }, # 17 idx=1 r=0 rp=1 encoding=0b10001
	{ reg => 'YMM18' }, # 18 idx=2 r=0 rp=1 encoding=0b10010
	{ reg => 'YMM19' }, # 19 idx=3 r=0 rp=1 encoding=0b10011
	{ reg => 'YMM20' }, # 20 idx=4 r=0 rp=1 encoding=0b10100
	{ reg => 'YMM21' }, # 21 idx=5 r=0 rp=1 encoding=0b10101
	{ reg => 'YMM22' }, # 22 idx=6 r=0 rp=1 encoding=0b10110
	{ reg => 'YMM23' }, # 23 idx=7 r=0 rp=1 encoding=0b10111
	{ reg => 'YMM24' }, # 24 idx=0 r=1 rp=1 encoding=0b11000
	{ reg => 'YMM25' }, # 25 idx=1 r=1 rp=1 encoding=0b11001
	{ reg => 'YMM26' }, # 26 idx=2 r=1 rp=1 encoding=0b11010
	{ reg => 'YMM27' }, # 27 idx=3 r=1 rp=1 encoding=0b11011
	{ reg => 'YMM28' }, # 28 idx=4 r=1 rp=1 encoding=0b11100
	{ reg => 'YMM29' }, # 29 idx=5 r=1 rp=1 encoding=0b11101
	{ reg => 'YMM30' }, # 30 idx=6 r=1 rp=1 encoding=0b11110
	{ reg => 'YMM31' }, # 31 idx=7 r=1 rp=1 encoding=0b11111
];

REG TABLE ZMMREG_TBL => [
	{ reg => 'ZMM0' }, # 0 idx=0 r=0 rp=0 encoding=0b00000
	{ reg => 'ZMM1' }, # 1 idx=1 r=0 rp=0 encoding=0b00001
	{ reg => 'ZMM2' }, # 2 idx=2 r=0 rp=0 encoding=0b00010
	{ reg => 'ZMM3' }, # 3 idx=3 r=0 rp=0 encoding=0b00011
	{ reg => 'ZMM4' }, # 4 idx=4 r=0 rp=0 encoding=0b00100
	{ reg => 'ZMM5' }, # 5 idx=5 r=0 rp=0 encoding=0b00101
	{ reg => 'ZMM6' }, # 6 idx=6 r=0 rp=0 encoding=0b00110
	{ reg => 'ZMM7' }, # 7 idx=7 r=0 rp=0 encoding=0b00111
	{ reg => 'ZMM8' }, # 8 idx=0 r=1 rp=0 encoding=0b01000
	{ reg => 'ZMM9' }, # 9 idx=1 r=1 rp=0 encoding=0b01001
	{ reg => 'ZMM10' }, # 10 idx=2 r=1 rp=0 encoding=0b01010
	{ reg => 'ZMM11' }, # 11 idx=3 r=1 rp=0 encoding=0b01011
	{ reg => 'ZMM12' }, # 12 idx=4 r=1 rp=0 encoding=0b01100
	{ reg => 'ZMM13' }, # 13 idx=5 r=1 rp=0 encoding=0b01101
	{ reg => 'ZMM14' }, # 14 idx=6 r=1 rp=0 encoding=0b01110
	{ reg => 'ZMM15' }, # 15 idx=7 r=1 rp=0 encoding=0b01111
	{ reg => 'ZMM16' }, # 16 idx=0 r=0 rp=1 encoding=0b10000
	{ reg => 'ZMM17' }, # 17 idx=1 r=0 rp=1 encoding=0b10001
	{ reg => 'ZMM18' }, # 18 idx=2 r=0 rp=1 encoding=0b10010
	{ reg => 'ZMM19' }, # 19 idx=3 r=0 rp=1 encoding=0b10011
	{ reg => 'ZMM20' }, # 20 idx=4 r=0 rp=1 encoding=0b10100
	{ reg => 'ZMM21' }, # 21 idx=5 r=0 rp=1 encoding=0b10101
	{ reg => 'ZMM22' }, # 22 idx=6 r=0 rp=1 encoding=0b10110
	{ reg => 'ZMM23' }, # 23 idx=7 r=0 rp=1 encoding=0b10111
	{ reg => 'ZMM24' }, # 24 idx=0 r=1 rp=1 encoding=0b11000
	{ reg => 'ZMM25' }, # 25 idx=1 r=1 rp=1 encoding=0b11001
	{ reg => 'ZMM26' }, # 26 idx=2 r=1 rp=1 encoding=0b11010
	{ reg => 'ZMM27' }, # 27 idx=3 r=1 rp=1 encoding=0b11011
	{ reg => 'ZMM28' }, # 28 idx=4 r=1 rp=1 encoding=0b11100
	{ reg => 'ZMM29' }, # 29 idx=5 r=1 rp=1 encoding=0b11101
	{ reg => 'ZMM30' }, # 30 idx=6 r=1 rp=1 encoding=0b11110
	{ reg => 'ZMM31' }, # 31 idx=7 r=1 rp=1 encoding=0b11111
];

