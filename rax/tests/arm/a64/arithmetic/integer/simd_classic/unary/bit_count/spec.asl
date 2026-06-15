__instruction aarch64_vector_arithmetic_unary_clsz
    __encoding aarch64_vector_arithmetic_unary_clsz
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx100000 010010xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if size == '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            CountOp countop = if U == '1' then CountOp_CLZ else CountOp_CLS;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        bits(datasize) result;

        integer count;
        for e = 0 to elements-1
            if countop == CountOp_CLS then
                count = CountLeadingSignBits(Elem[operand, e, esize]);
            else
                count = CountLeadingZeroBits(Elem[operand, e, esize]);
            Elem[result, e, esize] = count<esize-1:0>;
        V[d] = result;

__instruction aarch64_vector_arithmetic_unary_cnt
    __encoding aarch64_vector_arithmetic_unary_cnt
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 xx100000 010110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if size != '00' then UNDEFINED;
            integer esize = 8;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV 8;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        bits(datasize) result;

        integer count;
        for e = 0 to elements-1
            count = BitCount(Elem[operand, e, esize]);
            Elem[result, e, esize] = count<esize-1:0>;
        V[d] = result;
