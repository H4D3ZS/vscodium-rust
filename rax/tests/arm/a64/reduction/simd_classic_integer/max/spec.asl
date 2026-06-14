__instruction aarch64_vector_reduce_int_max
    __encoding aarch64_vector_reduce_int_max
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field op 16 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx11000x 101010xx xxxxxxxx'
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
            boolean min = (op == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        integer maxmin;
        integer element;

        maxmin = Int(Elem[operand, 0, esize], unsigned);
        for e = 1 to elements-1
            element = Int(Elem[operand, e, esize], unsigned);
            maxmin = if min then Min(maxmin, element) else Max(maxmin, element);

        V[d] = maxmin<esize-1:0>;
