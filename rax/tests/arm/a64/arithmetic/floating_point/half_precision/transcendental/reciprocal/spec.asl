__instruction aarch64_vector_arithmetic_unary_special_frecpx_fp16
    __encoding aarch64_vector_arithmetic_unary_special_frecpx_fp16
        __instruction_set A64
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 11111001 111110xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 16;
            integer datasize = esize;
            integer elements = 1;

    __encoding aarch64_vector_arithmetic_unary_special_frecpx
        __instruction_set A64
        __field sz 22 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 1x100001 111110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 32 << UInt(sz);
            integer datasize = esize;
            integer elements = 1;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        bits(datasize) result;
        bits(esize) element;

        for e = 0 to elements-1
            element = Elem[operand, e, esize];
            Elem[result, e, esize] = FPRecpX(element, FPCR);

        V[d] = result;
