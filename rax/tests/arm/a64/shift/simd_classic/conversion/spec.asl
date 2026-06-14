__instruction aarch64_vector_shift_conv_float_sisd
    __encoding aarch64_vector_shift_conv_float_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11111 0xxxxxxx 111111xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if immh == '000x' || (immh == '001x' && !HaveFP16Ext()) then UNDEFINED;
            integer esize = if immh == '1xxx' then 64 else if immh == '01xx' then 32 else 16;
            integer datasize = esize;
            integer elements = 1;

            integer fracbits = (esize * 2) - UInt(immh:immb);
            boolean unsigned = (U == '1');
            FPRounding rounding = FPRounding_ZERO;

    __encoding aarch64_vector_shift_conv_float_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01111 0xxxxxxx 111111xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if immh == '0000' then SEE(asimdimm);
            if immh == '000x' || (immh == '001x' && !HaveFP16Ext()) then UNDEFINED;
            if immh<3>:Q == '10' then UNDEFINED;
            integer esize = if immh == '1xxx' then 64 else if immh == '01xx' then 32 else 16;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            integer fracbits = (esize * 2) - UInt(immh:immb);
            boolean unsigned = (U == '1');
            FPRounding rounding = FPRounding_ZERO;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand  = V[n];
        bits(datasize) result;
        bits(esize) element;

        for e = 0 to elements-1
            element = Elem[operand, e, esize];
            Elem[result, e, esize] = FPToFixed(element, fracbits, unsigned, FPCR, rounding);

        V[d] = result;

__instruction aarch64_vector_shift_conv_int_sisd
    __encoding aarch64_vector_shift_conv_int_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11111 0xxxxxxx 111001xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if immh == '000x' || (immh == '001x' && !HaveFP16Ext()) then UNDEFINED;
            integer esize = if immh == '1xxx' then 64 else if immh == '01xx' then 32 else 16;
            integer datasize = esize;
            integer elements = 1;

            integer fracbits = (esize * 2) - UInt(immh:immb);
            boolean unsigned = (U == '1');
            FPRounding rounding = FPRoundingMode(FPCR);

    __encoding aarch64_vector_shift_conv_int_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01111 0xxxxxxx 111001xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if immh == '0000' then SEE(asimdimm);
            if immh == '000x' || (immh == '001x' && !HaveFP16Ext()) then UNDEFINED;
            if immh<3>:Q == '10' then UNDEFINED;
            integer esize = if immh == '1xxx' then 64 else if immh == '01xx' then 32 else 16;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            integer fracbits = (esize * 2) - UInt(immh:immb);
            boolean unsigned = (U == '1');
            FPRounding rounding = FPRoundingMode(FPCR);

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand  = V[n];
        bits(datasize) result;
        bits(esize) element;

        for e = 0 to elements-1
            element = Elem[operand, e, esize];
            Elem[result, e, esize] = FixedToFP(element, fracbits, unsigned, FPCR, rounding);

        V[d] = result;
