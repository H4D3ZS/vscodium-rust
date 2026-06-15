__instruction aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
    __encoding aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field E 23 +: 1
        __field Rm 16 +: 5
        __field ac 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11110 x10xxxxx 0010x1xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer esize = 16;
            integer datasize = esize;
            integer elements = 1;
            CompareOp cmp;
            boolean abs;

            case E:U:ac of
                when '000' cmp = CompareOp_EQ; abs = FALSE;
                when '010' cmp = CompareOp_GE; abs = FALSE;
                when '011' cmp = CompareOp_GE; abs = TRUE;
                when '110' cmp = CompareOp_GT; abs = FALSE;
                when '111' cmp = CompareOp_GT; abs = TRUE;
                otherwise  UNDEFINED;

    __encoding aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field E 23 +: 1
        __field sz 22 +: 1
        __field Rm 16 +: 5
        __field ac 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11110 xx1xxxxx 1110x1xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer esize = 32 << UInt(sz);
            integer datasize = esize;
            integer elements = 1;
            CompareOp cmp;
            boolean abs;

            case E:U:ac of
                when '000' cmp = CompareOp_EQ; abs = FALSE;
                when '010' cmp = CompareOp_GE; abs = FALSE;
                when '011' cmp = CompareOp_GE; abs = TRUE;
                when '110' cmp = CompareOp_GT; abs = FALSE;
                when '111' cmp = CompareOp_GT; abs = TRUE;
                otherwise  UNDEFINED;

    __encoding aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field E 23 +: 1
        __field Rm 16 +: 5
        __field ac 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 x10xxxxx 0010x1xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer esize = 16;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            CompareOp cmp;
            boolean abs;

            case E:U:ac of
                when '000' cmp = CompareOp_EQ; abs = FALSE;
                when '010' cmp = CompareOp_GE; abs = FALSE;
                when '011' cmp = CompareOp_GE; abs = TRUE;
                when '110' cmp = CompareOp_GT; abs = FALSE;
                when '111' cmp = CompareOp_GT; abs = TRUE;
                otherwise  UNDEFINED;

    __encoding aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field E 23 +: 1
        __field sz 22 +: 1
        __field Rm 16 +: 5
        __field ac 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx1xxxxx 1110x1xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if sz:Q == '10' then UNDEFINED;
            integer esize = 32 << UInt(sz);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            CompareOp cmp;
            boolean abs;

            case E:U:ac of
                when '000' cmp = CompareOp_EQ; abs = FALSE;
                when '010' cmp = CompareOp_GE; abs = FALSE;
                when '011' cmp = CompareOp_GE; abs = TRUE;
                when '110' cmp = CompareOp_GT; abs = FALSE;
                when '111' cmp = CompareOp_GT; abs = TRUE;
                otherwise  UNDEFINED;

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
            if abs then
                element1 = FPAbs(element1);
                element2 = FPAbs(element2);
            case cmp of
                when CompareOp_EQ test_passed = FPCompareEQ(element1, element2, FPCR);
                when CompareOp_GE test_passed = FPCompareGE(element1, element2, FPCR);
                when CompareOp_GT test_passed = FPCompareGT(element1, element2, FPCR);
            Elem[result, e, esize] = if test_passed then Ones() else Zeros();

        V[d] = result;
