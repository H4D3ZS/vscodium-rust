__instruction aarch64_vector_arithmetic_binary_disparate_add_sub_long
    __encoding aarch64_vector_arithmetic_binary_disparate_add_sub_long
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field o1 13 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx1xxxxx 00x000xx xxxxxxxx'
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

            boolean sub_op = (o1 == '1');
            boolean unsigned = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize)   operand1 = Vpart[n, part];
        bits(datasize)   operand2 = Vpart[m, part];
        bits(2*datasize) result;
        integer element1;
        integer element2;
        integer sum;

        for e = 0 to elements-1
            element1 = Int(Elem[operand1, e, esize], unsigned);
            element2 = Int(Elem[operand2, e, esize], unsigned);
            if sub_op then
                sum = element1 - element2;
            else
                sum = element1 + element2;
            Elem[result, e, 2*esize] = sum<2*esize-1:0>;

        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_disparate_add_sub_wide
    __encoding aarch64_vector_arithmetic_binary_disparate_add_sub_wide
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field o1 13 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx1xxxxx 00x100xx xxxxxxxx'
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

            boolean sub_op = (o1 == '1');
            boolean unsigned = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(2*datasize) operand1 = V[n];
        bits(datasize)   operand2 = Vpart[m, part];
        bits(2*datasize) result;
        integer element1;
        integer element2;
        integer sum;

        for e = 0 to elements-1
            element1 = Int(Elem[operand1, e, 2*esize], unsigned);
            element2 = Int(Elem[operand2, e, esize], unsigned);
            if sub_op then
                sum = element1 - element2;
            else
                sum = element1 + element2;
            Elem[result, e, 2*esize] = sum<2*esize-1:0>;

        V[d] = result;
