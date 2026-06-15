__instruction aarch64_integer_logical_shiftedreg
    __encoding aarch64_integer_logical_shiftedreg
        __instruction_set A64
        __field sf 31 +: 1
        __field opc 29 +: 2
        __field shift 22 +: 2
        __field N 21 +: 1
        __field Rm 16 +: 5
        __field imm6 10 +: 6
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode 'xxx01010 xxxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer datasize = if sf == '1' then 64 else 32;
            boolean setflags;
            LogicalOp op;
            case opc of
                when '00' op = LogicalOp_AND; setflags = FALSE;
                when '01' op = LogicalOp_ORR; setflags = FALSE;
                when '10' op = LogicalOp_EOR; setflags = FALSE;
                when '11' op = LogicalOp_AND; setflags = TRUE;

            if sf == '0' && imm6<5> == '1' then UNDEFINED;

            ShiftType shift_type = DecodeShift(shift);
            integer shift_amount = UInt(imm6);
            boolean invert = (N == '1');

    __execute
        bits(datasize) operand1 = X[n];
        bits(datasize) operand2 = ShiftReg(m, shift_type, shift_amount);

        if invert then operand2 = NOT(operand2);

        case op of
            when LogicalOp_AND result = operand1 AND operand2;
            when LogicalOp_ORR result = operand1 OR  operand2;
            when LogicalOp_EOR result = operand1 EOR operand2;

        if setflags then
            PSTATE.<N,Z,C,V> = result<datasize-1>:IsZeroBit(result):'00';

        X[d] = result;
