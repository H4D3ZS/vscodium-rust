__instruction aarch32_VCVT_iv_A
    __encoding aarch32_VCVT_iv_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field opc2 16 +: 3
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field op 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode 'xxxx1110 1x11110x xxxx10xx 01x0xxxx'
        __guard cond != '1111'
        __decode
            if opc2 != '000' && opc2 != '10x' then SEE "Related encodings";
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && cond != '1110' then UNPREDICTABLE;
            to_integer = (opc2<2> == '1');
            if to_integer then
                unsigned = (opc2<0> == '0');
                rounding = if op == '1' then FPRounding_ZERO else FPRoundingMode(FPSCR);
                d = UInt(Vd:D);
                case size of
                    when '01' esize = 16; m = UInt(Vm:M);
                    when '10' esize = 32; m = UInt(Vm:M);
                    when '11' esize = 64; m = UInt(M:Vm);
            else
                unsigned = (op == '0');
                rounding = FPRoundingMode(FPSCR);
                m = UInt(Vm:M);
                case size of
                    when '01' esize = 16; d = UInt(Vd:D);
                    when '10' esize = 32; d = UInt(Vd:D);
                    when '11' esize = 64; d = UInt(D:Vd);

    __encoding aarch32_VCVT_iv_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field opc2 16 +: 3
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field op 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101110 1x11110x xxxx10xx 01x0xxxx'
        __guard TRUE
        __decode
            if opc2 != '000' && opc2 != '10x' then SEE "Related encodings";
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            to_integer = (opc2<2> == '1');
            if to_integer then
                unsigned = (opc2<0> == '0');
                rounding = if op == '1' then FPRounding_ZERO else FPRoundingMode(FPSCR);
                d = UInt(Vd:D);
                case size of
                    when '01' esize = 16; m = UInt(Vm:M);
                    when '10' esize = 32; m = UInt(Vm:M);
                    when '11' esize = 64; m = UInt(M:Vm);
            else
                unsigned = (op == '0');
                rounding = FPRoundingMode(FPSCR);
                m = UInt(Vm:M);
                case size of
                    when '01' esize = 16; d = UInt(Vd:D);
                    when '10' esize = 32; d = UInt(Vd:D);
                    when '11' esize = 64; d = UInt(D:Vd);

    __execute __conditional
        CheckVFPEnabled(TRUE);
        if to_integer then
            case esize of
                when 16
                    S[d] = FPToFixed(S[m]<15:0>, 0, unsigned, FPSCR, rounding);
                when 32
                    S[d] = FPToFixed(S[m], 0, unsigned, FPSCR, rounding);
                when 64
                    S[d] = FPToFixed(D[m], 0, unsigned, FPSCR, rounding);
        else
            case esize of
                when 16
                    bits(16) fp16 = FixedToFP(S[m], 0, unsigned, FPSCR, rounding);
                    S[d] = Zeros(16):fp16;
                when 32
                    S[d] = FixedToFP(S[m], 0, unsigned, FPSCR, rounding);
                when 64
                    D[d] = FixedToFP(S[m], 0, unsigned, FPSCR, rounding);
