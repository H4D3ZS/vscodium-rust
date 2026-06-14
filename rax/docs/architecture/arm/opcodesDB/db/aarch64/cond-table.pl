# Conditional execution for ARM instructions.
TABLE COND => [
#                               Meaning (integer)              Meaning (floating-point)              Condition flags 
#                               -----------------              ------------------------              ---------------
	TV_COND(EQ => 0b0000), #    Equal                          Equal                                 Z == 1 
	TV_COND(NE => 0b0001), #    Not equal                      Not equal, or unordered               Z == 0 
	TV_COND(CS => 0b0010), #    Carry set                      Greater than, equal, or unordered     C == 1 
	TV_COND(CC => 0b0011), #    Carry clear                    Less than                             C == 0 
	TV_COND(MI => 0b0100), #    Minus, negative                Less than                             N == 1 
	TV_COND(PL => 0b0101), #    Plus, positive or zero         Greater than, equal, or unordered     N == 0 
	TV_COND(VS => 0b0110), #    Overflow                       Unordered                             V == 1 
	TV_COND(VC => 0b0111), #    No overflow                    Not unordered                         V == 0 
	TV_COND(HI => 0b1000), #    Unsigned higher                Greater than, or unordered            C == 1 and Z == 0 
	TV_COND(LS => 0b1001), #    Unsigned lower or same         Less than or equal                    C == 0 or Z == 1 
	TV_COND(GE => 0b1010), #    Signed greater than or equal   Greater than or equal                 N == V 
	TV_COND(LT => 0b1011), #    Signed less than               Less than, or unordered               N != V 
	TV_COND(GT => 0b1100), #    Signed greater than            Greater than                          Z == 0 and N == V 
	TV_COND(LE => 0b1101), #    Signed less than or equal      Less than, equal, or unordered        Z == 1 or N != V 
	TV_COND(AL => 0b1110), #    Always (unconditional)         Always (unconditional)                Any 
	# ALIASES:
	TV_COND(HS => 0b0010), # HS (unsigned higher or same) is a synonym for CS. 
    TV_COND(LO => 0b0011), # LO (unsigned lower) is a synonym for CC. 
];

# conditional unpredictable if not AL.
TABLE AUAL => [
	TV_COND(AL => 0b1110),
	UNPREDICTABLE FALL THROUGH 'COND'
];