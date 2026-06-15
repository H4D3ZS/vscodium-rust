__instruction aarch64_float_arithmetic_unary
    __encoding aarch64_float_arithmetic_unary
        __instruction_set A64
        __field type1 22 +: 2
        __field opc 15 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '00011110 xx10000x x10000xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

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

            FPUnaryOp fpop;
            case opc of
                when '00' fpop = FPUnaryOp_MOV;
                when '01' fpop = FPUnaryOp_ABS;
                when '10' fpop = FPUnaryOp_NEG;
                when '11' fpop = FPUnaryOp_SQRT;

    __execute
        CheckFPAdvSIMDEnabled64();

        bits(datasize) result;
        bits(datasize) operand = V[n];

        case fpop of
            when FPUnaryOp_MOV  result = operand;
            when FPUnaryOp_ABS  result = FPAbs(operand);
            when FPUnaryOp_NEG  result = FPNeg(operand);
            when FPUnaryOp_SQRT result = FPSqrt(operand, FPCR);

        V[d] = result;
