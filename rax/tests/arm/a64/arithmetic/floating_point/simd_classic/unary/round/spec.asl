__instruction aarch64_vector_arithmetic_unary_float_round_frint_32_64
    __encoding aarch64_vector_arithmetic_unary_float_round_frint_32_64
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field sz 22 +: 1
        __field op 12 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 0x100001 111x10xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFrintExt() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if sz:Q == '10' then UNDEFINED;
            integer esize = 32 << UInt(sz);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            integer intsize = if op == '0' then 32 else 64;
            FPRounding rounding = if U == '0' then FPRounding_ZERO else FPRoundingMode(FPCR);

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        bits(datasize) result;
        bits(esize) element;

        for e = 0 to elements-1
            element = Elem[operand, e, esize];
            Elem[result, e, esize] = FPRoundIntN(element, FPCR, rounding, intsize);

        V[d] = result;
