__instruction aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
    __encoding aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11110 xx1xxxxx 100011xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if size != '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = esize;
            integer elements = 1;
            boolean and_test = (U == '0');

    __encoding aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx1xxxxx 100011xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if size:Q == '110' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean and_test = (U == '0');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) result;
        bits(esize) element1;
        bits(esize) element2;
        boolean test_passed;

        for e = 0 to elements-1
            element1 = Elem[operand1, e, esize];
            element2 = Elem[operand2, e, esize];
            if and_test then
                test_passed = !IsZero(element1 AND element2);
            else
                test_passed = (element1 == element2);
            Elem[result, e, esize] = if test_passed then Ones() else Zeros();

        V[d] = result;
