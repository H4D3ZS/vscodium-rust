__instruction aarch64_vector_arithmetic_binary_uniform_mul_int_dotp
    __encoding aarch64_vector_arithmetic_binary_uniform_mul_int_dotp
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx0xxxxx 100101xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveDOTPExt() then UNDEFINED;
            if size!= '10' then UNDEFINED;
            boolean signed = (U=='0');
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) result;

        result = V[d];
        for e = 0 to elements-1
            integer res = 0;
            integer element1, element2;
            for i = 0 to 3
                if signed then
                    element1 = SInt(Elem[operand1, 4 * e + i, esize DIV 4]);
                    element2 = SInt(Elem[operand2, 4 * e + i, esize DIV 4]);
                else
                    element1 = UInt(Elem[operand1, 4 * e + i, esize DIV 4]);
                    element2 = UInt(Elem[operand2, 4 * e + i, esize DIV 4]);
                res = res + element1 * element2;
            Elem[result, e, esize] = Elem[result, e, esize] + res;
        V[d] = result;
