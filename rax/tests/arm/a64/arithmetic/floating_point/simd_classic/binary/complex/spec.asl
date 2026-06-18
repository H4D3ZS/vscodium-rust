__instruction aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower
    __encoding aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower
        __instruction_set A64
        __field Q 30 +: 1
        __field S 23 +: 1
        __field sz 22 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 xx1xxxxx 111011xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16MulNoRoundingToFP32Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if sz == '1' then UNDEFINED;
            integer esize = 32;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean sub_op = (S == '1');
            integer part = 0;

    __encoding aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper
        __instruction_set A64
        __field Q 30 +: 1
        __field S 23 +: 1
        __field sz 22 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101110 xx1xxxxx 110011xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16MulNoRoundingToFP32Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if sz == '1' then UNDEFINED;
            integer esize = 32;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean sub_op = (S == '1');
            integer part = 1;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize DIV 2) operand1 = Vpart[n,part];
        bits(datasize DIV 2) operand2 = Vpart[m,part];
        bits(datasize) operand3 = V[d];
        bits(datasize) result;
        bits(esize DIV 2) element1;
        bits(esize DIV 2) element2;

        for e = 0 to elements-1
            element1 = Elem[operand1, e, esize DIV 2];
            element2 = Elem[operand2, e, esize DIV 2];
            if sub_op then element1 = FPNeg(element1);
            Elem[result,e,esize] = FPMulAddH(Elem[operand3, e, esize], element1, element2, FPCR);
        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_uniform_mul_fp_complex
    __encoding aarch64_vector_arithmetic_binary_uniform_mul_fp_complex
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field rot 11 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101110 xx0xxxxx 110xx1xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFCADDExt() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if size == '00' then UNDEFINED;
            if Q == '0' && size == '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            if !HaveFP16Ext() && esize == 16 then UNDEFINED;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) operand3 = V[d];
        bits(datasize) result;
        bits(esize) element1;
        bits(esize) element2;
        bits(esize) element3;
        bits(esize) element4;

        for e = 0 to (elements DIV 2) -1
            case rot of
                when '00'
                    element1 = Elem[operand2, e*2, esize];
                    element2 = Elem[operand1, e*2, esize];
                    element3 = Elem[operand2, e*2+1, esize];
                    element4 = Elem[operand1, e*2, esize];
                when '01'
                    element1 = FPNeg(Elem[operand2, e*2+1, esize]);
                    element2 = Elem[operand1, e*2+1, esize];
                    element3 = Elem[operand2, e*2, esize];
                    element4 = Elem[operand1, e*2+1, esize];
                when '10'
                    element1 = FPNeg(Elem[operand2, e*2, esize]);
                    element2 = Elem[operand1, e*2, esize];
                    element3 = FPNeg(Elem[operand2, e*2+1, esize]);
                    element4 = Elem[operand1, e*2, esize];
                when '11'
                    element1 = Elem[operand2, e*2+1, esize];
                    element2 = Elem[operand1, e*2+1, esize];
                    element3 = FPNeg(Elem[operand2, e*2, esize]);
                    element4 = Elem[operand1, e*2+1, esize];

            Elem[result, e*2,   esize] = FPMulAdd(Elem[operand3, e*2,   esize], element2, element1, FPCR);
            Elem[result, e*2+1, esize] = FPMulAdd(Elem[operand3, e*2+1, esize], element4, element3, FPCR);

        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_uniform_add_fp_complex
    __encoding aarch64_vector_arithmetic_binary_uniform_add_fp_complex
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field rot 12 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101110 xx0xxxxx 111x01xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFCADDExt() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if size == '00' then UNDEFINED;
            if Q == '0' && size == '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            if !HaveFP16Ext() && esize == 16 then UNDEFINED;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) operand3 = V[d];
        bits(datasize) result;
        bits(esize) element1;
        bits(esize) element3;

        for e = 0 to (elements DIV 2) -1
            case rot of
                when '0'
                    element1 = FPNeg(Elem[operand2, e*2+1, esize]);
                    element3 = Elem[operand2, e*2, esize];
                when '1'
                    element1 = Elem[operand2, e*2+1, esize];
                    element3 = FPNeg(Elem[operand2, e*2, esize]);
            Elem[result, e*2,   esize] = FPAdd(Elem[operand1, e*2, esize], element1, FPCR);
            Elem[result, e*2+1, esize] = FPAdd(Elem[operand1, e*2+1, esize], element3, FPCR);

        V[d] = result;
