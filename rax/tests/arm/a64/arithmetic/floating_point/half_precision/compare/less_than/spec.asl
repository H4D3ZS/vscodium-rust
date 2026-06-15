__instruction aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
    __encoding aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
        __instruction_set A64
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 11111000 111010xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 16;
            integer datasize = esize;
            integer elements = 1;

            CompareOp comparison = CompareOp_LT;

    __encoding aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
        __instruction_set A64
        __field sz 22 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 1x100000 111010xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 32 << UInt(sz);
            integer datasize = esize;
            integer elements = 1;

            CompareOp comparison = CompareOp_LT;

    __encoding aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 11111000 111010xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 16;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            CompareOp comparison = CompareOp_LT;

    __encoding aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field sz 22 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 1x100000 111010xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if sz:Q == '10' then UNDEFINED;
            integer esize = 32 << UInt(sz);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            CompareOp comparison = CompareOp_LT;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        bits(datasize) result;
        bits(esize) zero = FPZero('0');
        bits(esize) element;
        boolean test_passed;

        for e = 0 to elements-1
            element = Elem[operand, e, esize];
            case comparison of
                when CompareOp_GT test_passed = FPCompareGT(element, zero, FPCR);
                when CompareOp_GE test_passed = FPCompareGE(element, zero, FPCR);
                when CompareOp_EQ test_passed = FPCompareEQ(element, zero, FPCR);
                when CompareOp_LE test_passed = FPCompareGE(zero, element, FPCR);
                when CompareOp_LT test_passed = FPCompareGT(zero, element, FPCR);
            Elem[result, e, esize] = if test_passed then Ones() else Zeros();

        V[d] = result;
