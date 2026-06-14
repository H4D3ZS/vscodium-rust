__instruction aarch64_float_arithmetic_add_sub
    __encoding aarch64_float_arithmetic_add_sub
        __instruction_set A64
        __field type1 22 +: 2
        __field Rm 16 +: 5
        __field op 12 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '00011110 xx1xxxxx 001x10xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            integer datasize;
            case type1 of
                when '00' datasize = 32;
                when '01' datasize = 64;
                when '10' UNDEFINED;
                when '11'
                    if HaveFP16Ext() then
                        datasize = 16;
                    else
                        UNDEFINED;

            boolean sub_op = (op == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) result;
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];

        if sub_op then
            result = FPSub(operand1, operand2, FPCR);
        else
            result = FPAdd(operand1, operand2, FPCR);

        V[d] = result;

__instruction aarch64_float_arithmetic_div
    __encoding aarch64_float_arithmetic_div
        __instruction_set A64
        __field type1 22 +: 2
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '00011110 xx1xxxxx 000110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            integer datasize;
            case type1 of
                when '00' datasize = 32;
                when '01' datasize = 64;
                when '10' UNDEFINED;
                when '11'
                    if HaveFP16Ext() then
                        datasize = 16;
                    else
                        UNDEFINED;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) result;
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];

        result = FPDiv(operand1, operand2, FPCR);

        V[d] = result;

__instruction aarch64_float_arithmetic_mul_product
    __encoding aarch64_float_arithmetic_mul_product
        __instruction_set A64
        __field type1 22 +: 2
        __field Rm 16 +: 5
        __field op 15 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '00011110 xx1xxxxx x00010xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            integer datasize;
            case type1 of
                when '00' datasize = 32;
                when '01' datasize = 64;
                when '10' UNDEFINED;
                when '11'
                    if HaveFP16Ext() then
                        datasize = 16;
                    else
                        UNDEFINED;

            boolean negated = (op == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) result;
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];

        result = FPMul(operand1, operand2, FPCR);

        if negated then result = FPNeg(result);

        V[d] = result;
