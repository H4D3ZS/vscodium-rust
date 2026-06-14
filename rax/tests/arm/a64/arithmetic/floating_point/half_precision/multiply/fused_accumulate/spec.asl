__instruction aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused
    __encoding aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused
        __instruction_set A64
        __field Q 30 +: 1
        __field a 23 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 x10xxxxx 000011xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer esize = 16;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            boolean sub_op = (a == '1');

    __encoding aarch64_vector_arithmetic_binary_uniform_mul_fp_fused
        __instruction_set A64
        __field Q 30 +: 1
        __field op 23 +: 1
        __field sz 22 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 xx1xxxxx 110011xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if sz:Q == '10' then UNDEFINED;
            integer esize = 32 << UInt(sz);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            boolean sub_op = (op == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) operand3 = V[d];
        bits(datasize) result;
        bits(esize) element1;
        bits(esize) element2;

        for e = 0 to elements-1
            element1 = Elem[operand1, e, esize];
            element2 = Elem[operand2, e, esize];
            if sub_op then element1 = FPNeg(element1);
            Elem[result, e, esize] = FPMulAdd(Elem[operand3, e, esize], element1, element2, FPCR);

        V[d] = result;
