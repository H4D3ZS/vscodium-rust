__instruction aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
    __encoding aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field eq 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11110 xx1xxxxx 0011x1xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if size != '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = esize;
            integer elements = 1;
            boolean unsigned = (U == '1');
            boolean cmp_eq = (eq == '1');

    __encoding aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field eq 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx1xxxxx 0011x1xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if size:Q == '110' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean unsigned = (U == '1');
            boolean cmp_eq = (eq == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) result;
        integer element1;
        integer element2;
        boolean test_passed;

        for e = 0 to elements-1
            element1 = Int(Elem[operand1, e, esize], unsigned);
            element2 = Int(Elem[operand2, e, esize], unsigned);
            test_passed = if cmp_eq then element1 >= element2 else element1 > element2;
            Elem[result, e, esize] = if test_passed then Ones() else Zeros();

        V[d] = result;
