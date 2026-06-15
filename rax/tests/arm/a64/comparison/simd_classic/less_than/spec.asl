__instruction aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
    __encoding aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
        __instruction_set A64
        __field size 22 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 xx100000 101010xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if size != '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = esize;
            integer elements = 1;

            CompareOp comparison = CompareOp_LT;

    __encoding aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 xx100000 101010xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if size:Q == '110' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            CompareOp comparison = CompareOp_LT;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        bits(datasize) result;
        integer element;
        boolean test_passed;

        for e = 0 to elements-1
            element = SInt(Elem[operand, e, esize]);
            case comparison of
                when CompareOp_GT test_passed = element > 0;
                when CompareOp_GE test_passed = element >= 0;
                when CompareOp_EQ test_passed = element == 0;
                when CompareOp_LE test_passed = element <= 0;
                when CompareOp_LT test_passed = element < 0;
            Elem[result, e, esize] = if test_passed then Ones() else Zeros();

        V[d] = result;
