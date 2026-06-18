__instruction aarch64_vector_arithmetic_binary_disparate_diff
    __encoding aarch64_vector_arithmetic_binary_disparate_diff
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field op 13 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx1xxxxx 01x100xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            if size == '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = 64;
            integer part = UInt(Q);
            integer elements = datasize DIV esize;

            boolean accumulate = (op == '0');
            boolean unsigned = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize)   operand1 = Vpart[n, part];
        bits(datasize)   operand2 = Vpart[m, part];
        bits(2*datasize) result;
        integer element1;
        integer element2;
        bits(2*esize) absdiff;

        result = if accumulate then V[d] else Zeros();
        for e = 0 to elements-1
            element1 = Int(Elem[operand1, e, esize], unsigned);
            element2 = Int(Elem[operand2, e, esize], unsigned);
            absdiff = Abs(element1 - element2)<2*esize-1:0>;
            Elem[result, e, 2*esize] = Elem[result, e, 2*esize] + absdiff;
        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_uniform_diff
    __encoding aarch64_vector_arithmetic_binary_uniform_diff
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field ac 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx1xxxxx 0111x1xx xxxxxxxx'
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
            boolean accumulate = (ac == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) result;
        integer element1;
        integer element2;
        bits(esize) absdiff;

        result = if accumulate then V[d] else Zeros();
        for e = 0 to elements-1
            element1 = Int(Elem[operand1, e, esize], unsigned);
            element2 = Int(Elem[operand2, e, esize], unsigned);
            absdiff = Abs(element1 - element2)<esize-1:0>;
            Elem[result, e, esize] = Elem[result, e, esize] + absdiff;
        V[d] = result;
