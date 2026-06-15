__instruction aarch32_VCVT_xv_A
    __encoding aarch32_VCVT_xv_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field op 18 +: 1
        __field U 16 +: 1
        __field Vd 12 +: 4
        __field sf 8 +: 2
        __field sx 7 +: 1
        __field i 5 +: 1
        __field imm4 0 +: 4
        __opcode 'xxxx1110 1x111x1x xxxx10xx x1x0xxxx'
        __guard cond != '1111'
        __decode
            if sf == '00' || (sf == '01' && !HaveFP16Ext()) then UNDEFINED;
            if sf == '01' && cond != '1110' then UNPREDICTABLE;
            to_fixed = (op == '1');  unsigned = (U == '1');
            size = if sx == '0' then 16 else 32;
            frac_bits = size - UInt(imm4:i);
            case sf of
                when '01' fp_size = 16; d = UInt(Vd:D);
                when '10' fp_size = 32; d = UInt(Vd:D);
                when '11' fp_size = 64; d = UInt(D:Vd);

            if frac_bits < 0 then UNPREDICTABLE;

    __encoding aarch32_VCVT_xv_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field op 18 +: 1
        __field U 16 +: 1
        __field Vd 12 +: 4
        __field sf 8 +: 2
        __field sx 7 +: 1
        __field i 5 +: 1
        __field imm4 0 +: 4
        __opcode '11101110 1x111x1x xxxx10xx x1x0xxxx'
        __guard TRUE
        __decode
            if sf == '00' || (sf == '01' && !HaveFP16Ext()) then UNDEFINED;
            if sf == '01' && InITBlock() then UNPREDICTABLE;
            to_fixed = (op == '1');  unsigned = (U == '1');
            size = if sx == '0' then 16 else 32;
            frac_bits = size - UInt(imm4:i);
            case sf of
                when '01' fp_size = 16; d = UInt(Vd:D);
                when '10' fp_size = 32; d = UInt(Vd:D);
                when '11' fp_size = 64; d = UInt(D:Vd);

            if frac_bits < 0 then UNPREDICTABLE;

    __execute __conditional
        CheckVFPEnabled(TRUE);
        if to_fixed then
            bits(size) result;
            case fp_size of
                when 16
                    result = FPToFixed(S[d]<15:0>, frac_bits, unsigned, FPSCR, FPRounding_ZERO);
                    S[d] = Extend(result, 32, unsigned);
                when 32
                    result = FPToFixed(S[d], frac_bits, unsigned, FPSCR, FPRounding_ZERO);
                    S[d] = Extend(result, 32, unsigned);
                when 64
                    result = FPToFixed(D[d], frac_bits, unsigned, FPSCR, FPRounding_ZERO);
                    D[d] = Extend(result, 64, unsigned);
        else
            case fp_size of
                when 16
                    bits(16) fp16 = FixedToFP(S[d]<size-1:0>, frac_bits, unsigned, FPSCR, FPRounding_TIEEVEN);
                    S[d] = Zeros(16):fp16;
                when 32
                    S[d] = FixedToFP(S[d]<size-1:0>, frac_bits, unsigned, FPSCR, FPRounding_TIEEVEN);
                when 64
                    D[d] = FixedToFP(D[d]<size-1:0>, frac_bits, unsigned, FPSCR, FPRounding_TIEEVEN);
