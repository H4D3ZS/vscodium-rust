__instruction aarch64_float_arithmetic_mul_add_sub
    __encoding aarch64_float_arithmetic_mul_add_sub
        __instruction_set A64
        __field type1 22 +: 2
        __field o1 21 +: 1
        __field Rm 16 +: 5
        __field o0 15 +: 1
        __field Ra 10 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '00011111 xxxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer a = UInt(Ra);
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

            boolean opa_neg = (o1 == '1');
            boolean op1_neg = (o0 != o1);

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) result;
        bits(datasize) operanda = V[a];
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];

        if opa_neg then operanda = FPNeg(operanda);
        if op1_neg then operand1 = FPNeg(operand1);
        result = FPMulAdd(operanda, operand1, operand2, FPCR);

        V[d] = result;
