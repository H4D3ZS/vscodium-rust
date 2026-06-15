__instruction aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985
    __encoding aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field o1 23 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 x10xxxxx 001101xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer esize = 16;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            boolean pair = (U == '1');
            boolean minimum = (o1 == '1');

    __encoding aarch64_vector_arithmetic_binary_uniform_max_min_fp_1985
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field o1 23 +: 1
        __field sz 22 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx1xxxxx 111101xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if sz:Q == '10' then UNDEFINED;
            integer esize = 32 << UInt(sz);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            boolean pair = (U == '1');
            boolean minimum = (o1 == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) result;
        bits(2*datasize) concat = operand2:operand1;
        bits(esize) element1;
        bits(esize) element2;

        for e = 0 to elements-1
            if pair then
                element1 = Elem[concat, 2*e, esize];
                element2 = Elem[concat, (2*e)+1, esize];
            else
                element1 = Elem[operand1, e, esize];
                element2 = Elem[operand2, e, esize];

            if minimum then
                Elem[result, e, esize] = FPMin(element1, element2, FPCR);
            else
                Elem[result, e, esize] = FPMax(element1, element2, FPCR);

        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008
    __encoding aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field a 23 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 x10xxxxx 000001xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer esize = 16;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            boolean pair = (U == '1');
            boolean minimum = (a == '1');

    __encoding aarch64_vector_arithmetic_binary_uniform_max_min_fp_2008
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field o1 23 +: 1
        __field sz 22 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx1xxxxx 110001xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if sz:Q == '10' then UNDEFINED;
            integer esize = 32 << UInt(sz);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            boolean pair = (U == '1');
            boolean minimum = (o1 == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) result;
        bits(2*datasize) concat = operand2:operand1;
        bits(esize) element1;
        bits(esize) element2;

        for e = 0 to elements-1
            if pair then
                element1 = Elem[concat, 2*e, esize];
                element2 = Elem[concat, (2*e)+1, esize];
            else
                element1 = Elem[operand1, e, esize];
                element2 = Elem[operand2, e, esize];

            if minimum then
                Elem[result, e, esize] = FPMinNum(element1, element2, FPCR);
            else
                Elem[result, e, esize] = FPMaxNum(element1, element2, FPCR);

        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_uniform_max_min_single
    __encoding aarch64_vector_arithmetic_binary_uniform_max_min_single
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field o1 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx1xxxxx 0110x1xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if size == '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            boolean unsigned = (U == '1');
            boolean minimum = (o1 == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) result;
        integer element1;
        integer element2;
        integer maxmin;

        for e = 0 to elements-1
            element1 = Int(Elem[operand1, e, esize], unsigned);
            element2 = Int(Elem[operand2, e, esize], unsigned);
            maxmin = if minimum then Min(element1, element2) else Max(element1, element2);
            Elem[result, e, esize] = maxmin<esize-1:0>;

        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_uniform_max_min_pair
    __encoding aarch64_vector_arithmetic_binary_uniform_max_min_pair
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field o1 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx1xxxxx 1010x1xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if size == '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            boolean unsigned = (U == '1');
            boolean minimum = (o1 == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) result;
        bits(2*datasize) concat = operand2:operand1;
        integer element1;
        integer element2;
        integer maxmin;

        for e = 0 to elements-1
            element1 = Int(Elem[concat, 2*e, esize], unsigned);
            element2 = Int(Elem[concat, (2*e)+1, esize], unsigned);
            maxmin = if minimum then Min(element1, element2) else Max(element1, element2);
            Elem[result, e, esize] = maxmin<esize-1:0>;

        V[d] = result;
