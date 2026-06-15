__instruction aarch64_float_convert_int
    __encoding aarch64_float_convert_int
        __instruction_set A64
        __field sf 31 +: 1
        __field type1 22 +: 2
        __field rmode 19 +: 2
        __field opcode 16 +: 3
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode 'x0011110 xx1xxxxx 000000xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer intsize = if sf == '1' then 64 else 32;
            integer fltsize;
            FPConvOp op;
            FPRounding rounding;
            boolean unsigned;
            integer part;

            case type1 of
                when '00'
                    fltsize = 32;
                when '01'
                    fltsize = 64;
                when '10'
                    if opcode<2:1>:rmode != '11 01' then UNDEFINED;
                    fltsize = 128;
                when '11'
                    if HaveFP16Ext() then
                        fltsize = 16;
                    else
                        UNDEFINED;

            case opcode<2:1>:rmode of
                when '00 xx'        // FCVT[NPMZ][US]
                    rounding = FPDecodeRounding(rmode);
                    unsigned = (opcode<0> == '1');
                    op = FPConvOp_CVT_FtoI;
                when '01 00'        // [US]CVTF
                    rounding = FPRoundingMode(FPCR);
                    unsigned = (opcode<0> == '1');
                    op = FPConvOp_CVT_ItoF;
                when '10 00'        // FCVTA[US]
                    rounding = FPRounding_TIEAWAY;
                    unsigned = (opcode<0> == '1');
                    op = FPConvOp_CVT_FtoI;
                when '11 00'        // FMOV
                    if fltsize != 16 && fltsize != intsize then UNDEFINED;
                    op = if opcode<0> == '1' then FPConvOp_MOV_ItoF else FPConvOp_MOV_FtoI;
                    part = 0;
                when '11 01'        // FMOV D[1]
                    if intsize != 64 || fltsize != 128 then UNDEFINED;
                    op = if opcode<0> == '1' then FPConvOp_MOV_ItoF else FPConvOp_MOV_FtoI;
                    part = 1;
                    fltsize = 64;  // size of D[1] is 64
                when '11 11'       // FJCVTZS
                    if !HaveFJCVTZSExt() then UNDEFINED;
                    rounding = FPRounding_ZERO;
                    unsigned = (opcode<0> == '1');
                    op = FPConvOp_CVT_FtoI_JS;
                otherwise
                    UNDEFINED;

    __execute
        CheckFPAdvSIMDEnabled64();

        bits(fltsize) fltval;
        bits(intsize) intval;

        case op of
            when FPConvOp_CVT_FtoI
                fltval = V[n];
                intval = FPToFixed(fltval, 0, unsigned, FPCR, rounding);
                X[d] = intval;
            when FPConvOp_CVT_ItoF
                intval = X[n];
                fltval = FixedToFP(intval, 0, unsigned, FPCR, rounding);
                V[d] = fltval;
            when FPConvOp_MOV_FtoI
                fltval = Vpart[n,part];
                intval = ZeroExtend(fltval, intsize);
                X[d] = intval;
            when FPConvOp_MOV_ItoF
                intval = X[n];
                fltval = intval<fltsize-1:0>;
                Vpart[d,part] = fltval;
            when FPConvOp_CVT_FtoI_JS
                fltval = V[n];
                intval = FPToFixedJS(fltval, FPCR, TRUE);
                X[d] = ZeroExtend(intval<31:0>, 64);

__instruction aarch64_float_convert_fix
    __encoding aarch64_float_convert_fix
        __instruction_set A64
        __field sf 31 +: 1
        __field type1 22 +: 2
        __field rmode 19 +: 2
        __field opcode 16 +: 3
        __field scale 10 +: 6
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode 'x0011110 xx0xxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer intsize = if sf == '1' then 64 else 32;
            integer fltsize;
            FPConvOp op;
            FPRounding rounding;
            boolean unsigned;

            case type1 of
                when '00' fltsize = 32;
                when '01' fltsize = 64;
                when '10' UNDEFINED;
                when '11'
                    if HaveFP16Ext() then
                        fltsize = 16;
                    else
                        UNDEFINED;

            if sf == '0' && scale<5> == '0' then UNDEFINED;
            integer fracbits = 64 - UInt(scale);

            case opcode<2:1>:rmode of
                when '00 11'        // FCVTZ
                    rounding = FPRounding_ZERO;
                    unsigned = (opcode<0> == '1');
                    op = FPConvOp_CVT_FtoI;
                when '01 00'        // [US]CVTF
                    rounding = FPRoundingMode(FPCR);
                    unsigned = (opcode<0> == '1');
                    op = FPConvOp_CVT_ItoF;
                otherwise
                    UNDEFINED;

    __execute
        CheckFPAdvSIMDEnabled64();

        bits(fltsize) fltval;
        bits(intsize) intval;

        case op of
            when FPConvOp_CVT_FtoI
                fltval = V[n];
                intval = FPToFixed(fltval, fracbits, unsigned, FPCR, rounding);
                X[d] = intval;
            when FPConvOp_CVT_ItoF
                intval = X[n];
                fltval = FixedToFP(intval, fracbits, unsigned, FPCR, rounding);
                V[d] = fltval;

__instruction aarch64_float_convert_fp
    __encoding aarch64_float_convert_fp
        __instruction_set A64
        __field type1 22 +: 2
        __field opc 15 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '00011110 xx10001x x10000xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if type1 == opc then UNDEFINED;

            integer srcsize;
            case type1 of
                when '00' srcsize = 32;
                when '01' srcsize = 64;
                when '10' UNDEFINED;
                when '11' srcsize = 16;
            integer dstsize;
            case opc of
                when '00' dstsize = 32;
                when '01' dstsize = 64;
                when '10' UNDEFINED;
                when '11' dstsize = 16;

    __execute
        CheckFPAdvSIMDEnabled64();

        bits(dstsize) result;
        bits(srcsize) operand = V[n];

        result = FPConvert(operand, FPCR);
        V[d] = result;
