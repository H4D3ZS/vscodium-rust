__instruction aarch64_vector_reduce_add_long
    __encoding aarch64_vector_reduce_add_long
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx110000 001110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if size:Q == '100' then UNDEFINED;
            if size == '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            boolean unsigned = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        integer sum;

        sum = Int(Elem[operand, 0, esize], unsigned);
        for e = 1 to elements-1
            sum = sum + Int(Elem[operand, e, esize], unsigned);

        V[d] = sum<2*esize-1:0>;
