__instruction aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
    __encoding aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field o1 13 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx1xxxxx 01x000xx xxxxxxxx'
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
            boolean round = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(2*datasize) operand1 = V[n];
        bits(2*datasize) operand2 = V[m];
        bits(datasize)   result;
        integer round_const = if round then 1 << (esize - 1) else 0;
        bits(2*esize) element1;
        bits(2*esize) element2;
        bits(2*esize) sum;

        for e = 0 to elements-1
            element1 = Elem[operand1, e, 2*esize];
            element2 = Elem[operand2, e, 2*esize];
            if sub_op then
                sum = element1 - element2;
            else
                sum = element1 + element2;
            sum = sum + round_const;
            Elem[result, e, esize] = sum<2*esize-1:esize>;

        Vpart[d, part] = result;
