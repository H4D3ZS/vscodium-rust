__instruction aarch64_integer_logical_immediate
    __encoding aarch64_integer_logical_immediate
        __instruction_set A64
        __field sf 31 +: 1
        __field opc 29 +: 2
        __field N 22 +: 1
        __field immr 16 +: 6
        __field imms 10 +: 6
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode 'xxx10010 0xxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer datasize = if sf == '1' then 64 else 32;
            boolean setflags;
            LogicalOp op;
            case opc of
                when '00' op = LogicalOp_AND; setflags = FALSE;
                when '01' op = LogicalOp_ORR; setflags = FALSE;
                when '10' op = LogicalOp_EOR; setflags = FALSE;
                when '11' op = LogicalOp_AND; setflags = TRUE;

            bits(datasize) imm;
            if sf == '0' && N != '0' then UNDEFINED;
            (imm, -) = DecodeBitMasks(N, imms, immr, TRUE);

    __execute
        bits(datasize) result;
        bits(datasize) operand1 = X[n];
        bits(datasize) operand2 = imm;

        case op of
            when LogicalOp_AND result = operand1 AND operand2;
            when LogicalOp_ORR result = operand1 OR  operand2;
            when LogicalOp_EOR result = operand1 EOR operand2;

        if setflags then
            PSTATE.<N,Z,C,V> = result<datasize-1>:IsZeroBit(result):'00';

        if d == 31 && !setflags then
            SP[] = result;
        else
            X[d] = result;
