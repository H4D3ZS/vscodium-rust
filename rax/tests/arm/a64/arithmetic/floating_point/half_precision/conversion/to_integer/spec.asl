__instruction aarch64_vector_arithmetic_unary_fp16_conv_int_sisd
    __encoding aarch64_vector_arithmetic_unary_fp16_conv_int_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11110 01111001 110110xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 16;
            integer datasize = esize;
            integer elements = 1;
            boolean unsigned = (U == '1');

    __encoding aarch64_vector_arithmetic_unary_float_conv_int_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field sz 22 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11110 0x100001 110110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 32 << UInt(sz);
            integer datasize = esize;
            integer elements = 1;
            boolean unsigned = (U == '1');

    __encoding aarch64_vector_arithmetic_unary_fp16_conv_int_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 01111001 110110xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 16;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean unsigned = (U == '1');

    __encoding aarch64_vector_arithmetic_unary_float_conv_int_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field sz 22 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 0x100001 110110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if sz:Q == '10' then UNDEFINED;
            integer esize = 32 << UInt(sz);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean unsigned = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        bits(datasize) result;
        FPRounding rounding = FPRoundingMode(FPCR);
        bits(esize) element;
        for e = 0 to elements-1
            element = Elem[operand, e, esize];
            Elem[result, e, esize] = FixedToFP(element, 0, unsigned, FPCR, rounding);

        V[d] = result;
