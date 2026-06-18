__instruction aarch64_float_arithmetic_max_min
    __encoding aarch64_float_arithmetic_max_min
        __instruction_set A64
        __field type1 22 +: 2
        __field Rm 16 +: 5
        __field op 12 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '00011110 xx1xxxxx 01xx10xx xxxxxxxx'
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

            FPMaxMinOp operation;
            case op of
                when '00' operation = FPMaxMinOp_MAX;
                when '01' operation = FPMaxMinOp_MIN;
                when '10' operation = FPMaxMinOp_MAXNUM;
                when '11' operation = FPMaxMinOp_MINNUM;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) result;
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];

        case operation of
            when FPMaxMinOp_MAX    result = FPMax(operand1, operand2, FPCR);
            when FPMaxMinOp_MIN    result = FPMin(operand1, operand2, FPCR);
            when FPMaxMinOp_MAXNUM result = FPMaxNum(operand1, operand2, FPCR);
            when FPMaxMinOp_MINNUM result = FPMinNum(operand1, operand2, FPCR);

        V[d] = result;
