__instruction aarch64_vector_arithmetic_binary_uniform_add_fp16
    __encoding aarch64_vector_arithmetic_binary_uniform_add_fp16
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 010xxxxx 000101xx xxxxxxxx'
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

    __encoding aarch64_vector_arithmetic_binary_uniform_add_fp
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field sz 22 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 0x1xxxxx 110101xx xxxxxxxx'
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
            Elem[result, e, esize] = FPAdd(element1, element2, FPCR);

        V[d] = result;
