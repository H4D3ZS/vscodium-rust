__instruction aarch64_vector_arithmetic_unary_diff_neg_int_sisd
    __encoding aarch64_vector_arithmetic_unary_diff_neg_int_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11110 xx100000 101110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if size != '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = esize;
            integer elements = 1;
            boolean neg = (U == '1');

    __encoding aarch64_vector_arithmetic_unary_diff_neg_int_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx100000 101110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if size:Q == '110' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean neg = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        bits(datasize) result;
        integer element;

        for e = 0 to elements-1
            element = SInt(Elem[operand, e, esize]);
            if neg then
                element = -element;
            else
                element = Abs(element);
            Elem[result, e, esize] = element<esize-1:0>;

        V[d] = result;
