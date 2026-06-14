__instruction aarch64_vector_arithmetic_unary_float_xtn_sisd
    __encoding aarch64_vector_arithmetic_unary_float_xtn_sisd
        __instruction_set A64
        __field sz 22 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01111110 0x100001 011010xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if sz == '0' then UNDEFINED;
            integer esize = 32;
            integer datasize = esize;
            integer elements = 1;
            integer part = 0;

    __encoding aarch64_vector_arithmetic_unary_float_xtn_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field sz 22 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101110 0x100001 011010xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if sz == '0' then UNDEFINED;
            integer esize = 32;
            integer datasize = 64;
            integer elements = 2;
            integer part = UInt(Q);

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(2*datasize) operand = V[n];
        bits(datasize) result;

        for e = 0 to elements-1
            Elem[result, e, esize] = FPConvert(Elem[operand, e, 2*esize], FPCR, FPRounding_ODD);

        Vpart[d, part] = result;

__instruction aarch64_vector_arithmetic_unary_float_widen
    __encoding aarch64_vector_arithmetic_unary_float_widen
        __instruction_set A64
        __field Q 30 +: 1
        __field sz 22 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 0x100001 011110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 16 << UInt(sz);
            integer datasize = 64;
            integer part = UInt(Q);
            integer elements = datasize DIV esize;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = Vpart[n, part];
        bits(2*datasize) result;

        for e = 0 to elements-1
            Elem[result, e, 2*esize] = FPConvert(Elem[operand, e, esize], FPCR);

        V[d] = result;

__instruction aarch64_vector_arithmetic_unary_float_narrow
    __encoding aarch64_vector_arithmetic_unary_float_narrow
        __instruction_set A64
        __field Q 30 +: 1
        __field sz 22 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 0x100001 011010xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 16 << UInt(sz);
            integer datasize = 64;
            integer part = UInt(Q);
            integer elements = datasize DIV esize;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(2*datasize) operand = V[n];
        bits(datasize) result;

        for e = 0 to elements-1
            Elem[result, e, esize] = FPConvert(Elem[operand, e, 2*esize], FPCR);

        Vpart[d, part] = result;
