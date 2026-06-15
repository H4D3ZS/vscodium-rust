__instruction aarch64_vector_arithmetic_binary_uniform_div_fp16
    __encoding aarch64_vector_arithmetic_binary_uniform_div_fp16
        __instruction_set A64
        __field Q 30 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101110 010xxxxx 001111xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer esize = 16;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

    __encoding aarch64_vector_arithmetic_binary_uniform_div
        __instruction_set A64
        __field Q 30 +: 1
        __field sz 22 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101110 0x1xxxxx 111111xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if sz:Q == '10' then UNDEFINED;
            integer esize = 32 << UInt(sz);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) result;
        bits(esize) element1;
        bits(esize) element2;

        for e = 0 to elements-1
            element1 = Elem[operand1, e, esize];
            element2 = Elem[operand2, e, esize];
            Elem[result, e, esize] = FPDiv(element1, element2, FPCR);

        V[d] = result;
