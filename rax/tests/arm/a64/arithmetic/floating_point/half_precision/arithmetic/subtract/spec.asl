__instruction aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
    __encoding aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 110xxxxx 000101xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer esize = 16;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean abs = (U == '1');

    __encoding aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field sz 22 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 1x1xxxxx 110101xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if sz:Q == '10' then UNDEFINED;
            integer esize = 32 << UInt(sz);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean abs = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) result;
        bits(esize) element1;
        bits(esize) element2;
        bits(esize) diff;

        for e = 0 to elements-1
            element1 = Elem[operand1, e, esize];
            element2 = Elem[operand2, e, esize];
            diff = FPSub(element1, element2, FPCR);
            Elem[result, e, esize] = if abs then FPAbs(diff) else diff;

        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
    __encoding aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
        __instruction_set A64
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01111110 110xxxxx 000101xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer esize = 16;
            integer datasize = esize;
            integer elements = 1;
            boolean abs = TRUE;

    __encoding aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
        __instruction_set A64
        __field sz 22 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01111110 1x1xxxxx 110101xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer esize = 32 << UInt(sz);
            integer datasize = esize;
            integer elements = 1;
            boolean abs = TRUE;

    __encoding aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 110xxxxx 000101xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer esize = 16;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean abs = (U == '1');

    __encoding aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field sz 22 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 1x1xxxxx 110101xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if sz:Q == '10' then UNDEFINED;
            integer esize = 32 << UInt(sz);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean abs = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) result;
        bits(esize) element1;
        bits(esize) element2;
        bits(esize) diff;

        for e = 0 to elements-1
            element1 = Elem[operand1, e, esize];
            element2 = Elem[operand2, e, esize];
            diff = FPSub(element1, element2, FPCR);
            Elem[result, e, esize] = if abs then FPAbs(diff) else diff;

        V[d] = result;
