__instruction aarch64_vector_arithmetic_unary_fp16_round
    __encoding aarch64_vector_arithmetic_unary_fp16_round
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field o2 23 +: 1
        __field o1 12 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 x1111001 100x10xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 16;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            boolean exact = FALSE;
            FPRounding rounding;
            case U:o1:o2 of
                when '0xx' rounding = FPDecodeRounding(o1:o2);
                when '100' rounding = FPRounding_TIEAWAY;
                when '101' UNDEFINED;
                when '110' rounding = FPRoundingMode(FPCR); exact = TRUE;
                when '111' rounding = FPRoundingMode(FPCR);

    __encoding aarch64_vector_arithmetic_unary_float_round
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field o2 23 +: 1
        __field sz 22 +: 1
        __field o1 12 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx100001 100x10xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if sz:Q == '10' then UNDEFINED;
            integer esize = 32 << UInt(sz);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            boolean exact = FALSE;
            FPRounding rounding;
            case U:o1:o2 of
                when '0xx' rounding = FPDecodeRounding(o1:o2);
                when '100' rounding = FPRounding_TIEAWAY;
                when '101' UNDEFINED;
                when '110' rounding = FPRoundingMode(FPCR); exact = TRUE;
                when '111' rounding = FPRoundingMode(FPCR);

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        bits(datasize) result;
        bits(esize) element;

        for e = 0 to elements-1
            element = Elem[operand, e, esize];
            Elem[result, e, esize] = FPRoundInt(element, FPCR, rounding, exact);

        V[d] = result;
