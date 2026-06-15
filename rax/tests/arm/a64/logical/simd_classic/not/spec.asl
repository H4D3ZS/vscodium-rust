__instruction aarch64_vector_arithmetic_unary_not
    __encoding aarch64_vector_arithmetic_unary_not
        __instruction_set A64
        __field Q 30 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101110 00100000 010110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 8;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV 8;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        bits(datasize) result;
        bits(esize) element;

        for e = 0 to elements-1
            element = Elem[operand, e, esize];
            Elem[result, e, esize] = NOT(element);

        V[d] = result;
