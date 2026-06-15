__instruction aarch64_vector_arithmetic_unary_special_recip_int
    __encoding aarch64_vector_arithmetic_unary_special_recip_int
        __instruction_set A64
        __field Q 30 +: 1
        __field sz 22 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 1x100001 110010xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if sz == '1' then UNDEFINED;
            integer esize = 32;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        bits(datasize) result;
        bits(32) element;

        for e = 0 to elements-1
            element = Elem[operand, e, 32];
            Elem[result, e, 32] = UnsignedRecipEstimate(element);

        V[d] = result;

__instruction aarch64_vector_arithmetic_unary_special_sqrt_est_int
    __encoding aarch64_vector_arithmetic_unary_special_sqrt_est_int
        __instruction_set A64
        __field Q 30 +: 1
        __field sz 22 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101110 1x100001 110010xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if sz == '1' then UNDEFINED;
            integer esize = 32;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        bits(datasize) result;
        bits(32) element;

        for e = 0 to elements-1
            element = Elem[operand, e, 32];
            Elem[result, e, 32] = UnsignedRSqrtEstimate(element);

        V[d] = result;
