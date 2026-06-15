__instruction aarch64_vector_arithmetic_binary_uniform_logical_and_orr
    __encoding aarch64_vector_arithmetic_binary_uniform_logical_and_orr
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 xx1xxxxx 000111xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer esize = 8;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            boolean invert = (size<0> == '1');
            LogicalOp op = if size<1> == '1' then LogicalOp_ORR else LogicalOp_AND;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) result;

        if invert then operand2 = NOT(operand2);

        case op of
            when LogicalOp_AND
                result = operand1 AND operand2;
            when LogicalOp_ORR
                result = operand1 OR operand2;

        V[d] = result;
