use strict;
use warnings;

TABLE GPR32    => [
	TV_REG(R0  =>  0b0000),
	TV_REG(R1  =>  0b0001),
	TV_REG(R2  =>  0b0010),
	TV_REG(R3  =>  0b0011),
	TV_REG(R4  =>  0b0100),
	TV_REG(R5  =>  0b0101),
	TV_REG(R6  =>  0b0110),
	TV_REG(R7  =>  0b0111),
	TV_REG(R8  =>  0b1000),
	TV_REG(R9  =>  0b1001),
	TV_REG(R10 =>  0b1010),
	TV_REG(R11 =>  0b1011),
	TV_REG(R12 =>  0b1100),
	TV_REG(SP  =>  0b1101),
	TV_REG(LR  =>  0b1110),
	TV_REG(PC  =>  0b1111),
];

# thumb16 : R0-R7.
TABLE GPR32LO  => [
	TV_REG(R0  =>  0b000),
	TV_REG(R1  =>  0b001),
	TV_REG(R2  =>  0b010),
	TV_REG(R3  =>  0b011),
	TV_REG(R4  =>  0b100),
	TV_REG(R5  =>  0b101),
	TV_REG(R6  =>  0b110),
	TV_REG(R7  =>  0b111),
];

# thumb16 : R8-R15.
TABLE GPR32HI  => [
	TV_REG(R8  =>  0b000),
	TV_REG(R9  =>  0b001),
	TV_REG(R10 =>  0b010),
	TV_REG(R11 =>  0b011),
	TV_REG(R12 =>  0b100),
	TV_REG(SP  =>  0b101),
	TV_REG(LR  =>  0b110),
	TV_REG(PC  =>  0b111),
];


# instruction doesnt support PC.
TABLE GPR32NOPC => [
	CONSTRAINT   TV_REG(PC  => 0b1111),
	FALL THROUGH 'GPR32',
];

# instruction is unpredictable if PC is used.
TABLE GPR32UPC => [
	UNPREDICTABLE  TV_REG(PC => 0b1111),
	FALL THROUGH   'GPR32',
];

# The PC can be used, but this is deprecated.
TABLE GPR32DPC => [
	DEPRECATED     TV_REG(PC => 0b1111),
	FALL THROUGH   'GPR32',
];

# 32-bit SIMD&FP register.
TABLE SIMD32    => [
    TV_REG(S0   =>  0b00000),
    TV_REG(S1   =>  0b00001),
    TV_REG(S2   =>  0b00010),
    TV_REG(S3   =>  0b00011),
    TV_REG(S4   =>  0b00100),
    TV_REG(S5   =>  0b00101),
    TV_REG(S6   =>  0b00110),
    TV_REG(S7   =>  0b00111),
    TV_REG(S8   =>  0b01000),
    TV_REG(S9   =>  0b01001),
    TV_REG(S10  =>  0b01010),
    TV_REG(S11  =>  0b01011),
    TV_REG(S12  =>  0b01100),
    TV_REG(S13  =>  0b01101),
    TV_REG(S14  =>  0b01110),
    TV_REG(S15  =>  0b01111),
    TV_REG(S16  =>  0b10000),
    TV_REG(S17  =>  0b10001),
    TV_REG(S18  =>  0b10010),
    TV_REG(S19  =>  0b10011),
    TV_REG(S20  =>  0b10100),
    TV_REG(S21  =>  0b10101),
    TV_REG(S22  =>  0b10110),
    TV_REG(S23  =>  0b10111),
    TV_REG(S24  =>  0b11000),
    TV_REG(S25  =>  0b11001),
    TV_REG(S26  =>  0b11010),
    TV_REG(S27  =>  0b11011),
    TV_REG(S28  =>  0b11100),
    TV_REG(S29  =>  0b11101),
    TV_REG(S30  =>  0b11110),
    TV_REG(S31  =>  0b11111),
];

# 64-bit SIMD&FP register.
TABLE SIMD64    => [
    TV_REG(D0   =>  0b00000),
    TV_REG(D1   =>  0b00001),
    TV_REG(D2   =>  0b00010),
    TV_REG(D3   =>  0b00011),
    TV_REG(D4   =>  0b00100),
    TV_REG(D5   =>  0b00101),
    TV_REG(D6   =>  0b00110),
    TV_REG(D7   =>  0b00111),
    TV_REG(D8   =>  0b01000),
    TV_REG(D9   =>  0b01001),
    TV_REG(D10  =>  0b01010),
    TV_REG(D11  =>  0b01011),
    TV_REG(D12  =>  0b01100),
    TV_REG(D13  =>  0b01101),
    TV_REG(D14  =>  0b01110),
    TV_REG(D15  =>  0b01111),
    TV_REG(D16  =>  0b10000),
    TV_REG(D17  =>  0b10001),
    TV_REG(D18  =>  0b10010),
    TV_REG(D19  =>  0b10011),
    TV_REG(D20  =>  0b10100),
    TV_REG(D21  =>  0b10101),
    TV_REG(D22  =>  0b10110),
    TV_REG(D23  =>  0b10111),
    TV_REG(D24  =>  0b11000),
    TV_REG(D25  =>  0b11001),
    TV_REG(D26  =>  0b11010),
    TV_REG(D27  =>  0b11011),
    TV_REG(D28  =>  0b11100),
    TV_REG(D29  =>  0b11101),
    TV_REG(D30  =>  0b11110),
    TV_REG(D31  =>  0b11111),
];

# 128-bit SIMD&FP register.
TABLE SIMD128   => [
    TV_REG(Q0   =>  0b00000), # 0
    TV_REG(Q1   =>  0b00010), # 2
    TV_REG(Q2   =>  0b00100), # 4
    TV_REG(Q3   =>  0b00110), # 6
    TV_REG(Q4   =>  0b01000), # 8
    TV_REG(Q5   =>  0b01010), # 10
    TV_REG(Q6   =>  0b01100), # 12
    TV_REG(Q7   =>  0b01110), # 14
    TV_REG(Q8   =>  0b10000), # 16
    TV_REG(Q9   =>  0b10010), # 18
    TV_REG(Q10  =>  0b10100), # 20
    TV_REG(Q11  =>  0b10110), # 22
    TV_REG(Q12  =>  0b11000), # 24
    TV_REG(Q13  =>  0b11010), # 26
    TV_REG(Q14  =>  0b11100), # 28
    TV_REG(Q15  =>  0b11110), # 30
];
