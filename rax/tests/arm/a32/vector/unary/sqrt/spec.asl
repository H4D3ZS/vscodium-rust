__instruction aarch32_VSQRT_A
    __encoding aarch32_VSQRT_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode 'xxxx1110 1x110001 xxxx10xx 11x0xxxx'
        __guard cond != '1111'
        __decode
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && cond != '1110' then UNPREDICTABLE;
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); m = UInt(M:Vm);

    __encoding aarch32_VSQRT_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101110 1x110001 xxxx10xx 11x0xxxx'
        __guard TRUE
        __decode
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); m = UInt(M:Vm);

    __execute __conditional
        CheckVFPEnabled(TRUE);
        case esize of
            when 16 S[d] = Zeros(16) : FPSqrt(S[m]<15:0>, FPSCR);
            when 32 S[d] = FPSqrt(S[m], FPSCR);
            when 64 D[d] = FPSqrt(D[m], FPSCR);
