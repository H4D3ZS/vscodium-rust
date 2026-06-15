__instruction aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
    __encoding aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field size 22 +: 2
        __field op 12 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11110 xx100000 100x10xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if size != '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = esize;
            integer elements = 1;

            CompareOp comparison;
            case op:U of
                when '00' comparison = CompareOp_GT;
                when '01' comparison = CompareOp_GE;
                when '10' comparison = CompareOp_EQ;
                when '11' comparison = CompareOp_LE;

    __encoding aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field op 12 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx100000 100x10xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if size:Q == '110' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            CompareOp comparison;
            case op:U of
                when '00' comparison = CompareOp_GT;
                when '01' comparison = CompareOp_GE;
                when '10' comparison = CompareOp_EQ;
                when '11' comparison = CompareOp_LE;

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
