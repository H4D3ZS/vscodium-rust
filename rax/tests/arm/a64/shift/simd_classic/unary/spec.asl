__instruction aarch64_vector_arithmetic_unary_shift
    __encoding aarch64_vector_arithmetic_unary_shift
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101110 xx100001 001110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if size == '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = 64;
            integer part = UInt(Q);
            integer elements = datasize DIV esize;

            integer shift = esize;
            boolean unsigned = FALSE; // Or TRUE without change of functionality

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = Vpart[n, part];
        bits(2*datasize) result;
        integer element;

        for e = 0 to elements-1
            element = Int(Elem[operand, e, esize], unsigned) << shift;
            Elem[result, e, 2*esize] = element<2*esize-1:0>;

        V[d] = result;
