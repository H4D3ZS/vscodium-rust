__instruction aarch64_float_arithmetic_round_frint
    __encoding aarch64_float_arithmetic_round_frint
        __instruction_set A64
        __field type1 22 +: 2
        __field rmode 15 +: 3
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '00011110 xx1001xx x10000xx xxxxxxxx'
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

            boolean exact = FALSE;
            FPRounding rounding;
            case rmode of
                when '0xx' rounding = FPDecodeRounding(rmode<1:0>);
                when '100' rounding = FPRounding_TIEAWAY;
                when '101' UNDEFINED;
                when '110' rounding = FPRoundingMode(FPCR); exact = TRUE;
                when '111' rounding = FPRoundingMode(FPCR);

    __execute
        CheckFPAdvSIMDEnabled64();

        bits(datasize) result;
        bits(datasize) operand = V[n];

        result = FPRoundInt(operand, FPCR, rounding, exact);

        V[d] = result;

__instruction aarch64_float_arithmetic_round_frint_32_64
    __encoding aarch64_float_arithmetic_round_frint_32_64
        __instruction_set A64
        __field type1 22 +: 2
        __field op 15 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '00011110 xx10100x x10000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFrintExt() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer datasize;
            case type1 of
                when '00' datasize = 32;
                when '01' datasize = 64;
                when '1x' UNDEFINED;

            integer intsize = if op<1> == '0' then 32 else 64;

            FPRounding rounding = if op<0> == '0' then FPRounding_ZERO else FPRoundingMode(FPCR);

    __execute
        CheckFPAdvSIMDEnabled64();

        bits(datasize) result;
        bits(datasize) operand = V[n];

        result = FPRoundIntN(operand, FPCR, rounding, intsize);

        V[d] = result;
