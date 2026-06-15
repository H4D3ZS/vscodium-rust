__instruction aarch64_vector_arithmetic_unary_diff_neg_fp16
    __encoding aarch64_vector_arithmetic_unary_diff_neg_fp16
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 11111000 111110xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 16;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean neg = (U == '1');

    __encoding aarch64_vector_arithmetic_unary_diff_neg_float
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field sz 22 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 1x100000 111110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if sz:Q == '10' then UNDEFINED;
            integer esize = 32 << UInt(sz);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean neg = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        bits(datasize) result;
        bits(esize) element;

        for e = 0 to elements-1
            element = Elem[operand, e, esize];
            if neg then
                element = FPNeg(element);
            else
                element = FPAbs(element);
            Elem[result, e, esize] = element;

        V[d] = result;
