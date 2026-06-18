__instruction aarch64_vector_arithmetic_binary_element_mul_acc_fp16_sisd
    __encoding aarch64_vector_arithmetic_binary_element_mul_acc_fp16_sisd
        __instruction_set A64
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field o2 14 +: 1
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011111 00xxxxxx 0x01x0xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer idxdsize = if H == '1' then 128 else 64;
            integer index;
            index = UInt(H:L:M);

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            integer esize = 16;
            integer datasize = esize;
            integer elements = 1;
            boolean sub_op = (o2 == '1');

    __encoding aarch64_vector_arithmetic_binary_element_mul_acc_fp_sisd
        __instruction_set A64
        __field sz 22 +: 1
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field o2 14 +: 1
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011111 1xxxxxxx 0x01x0xx xxxxxxxx'
        __guard TRUE
        __decode
            integer idxdsize = if H == '1' then 128 else 64;
            integer index;
            bit Rmhi = M;
            case sz:L of
                when '0x' index = UInt(H:L);
                when '10' index = UInt(H);
                when '11' UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rmhi:Rm);

            integer esize = 32 << UInt(sz);
            integer datasize = esize;
            integer elements = 1;
            boolean sub_op = (o2 == '1');

    __encoding aarch64_vector_arithmetic_binary_element_mul_acc_fp16_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field o2 14 +: 1
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001111 00xxxxxx 0x01x0xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer idxdsize = if H == '1' then 128 else 64;
            integer index;
            index = UInt(H:L:M);

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            integer esize = 16;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean sub_op = (o2 == '1');

    __encoding aarch64_vector_arithmetic_binary_element_mul_acc_fp_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field sz 22 +: 1
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field o2 14 +: 1
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001111 1xxxxxxx 0x01x0xx xxxxxxxx'
        __guard TRUE
        __decode
            integer idxdsize = if H == '1' then 128 else 64;
            integer index;
            bit Rmhi = M;
            case sz:L of
                when '0x' index = UInt(H:L);
                when '10' index = UInt(H);
                when '11' UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rmhi:Rm);

            if sz:Q == '10' then UNDEFINED;
            integer esize = 32 << UInt(sz);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean sub_op = (o2 == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(idxdsize) operand2 = V[m];
        bits(datasize) operand3 = V[d];
        bits(datasize) result;
        bits(esize) element1;
        bits(esize) element2 = Elem[operand2, index, esize];

        for e = 0 to elements-1
            element1 = Elem[operand1, e, esize];
            if sub_op then element1 = FPNeg(element1);
            Elem[result, e, esize] = FPMulAdd(Elem[operand3, e, esize], element1, element2, FPCR);
        V[d] = result;
