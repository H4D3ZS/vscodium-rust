__instruction aarch32_VDIV_A
    __encoding aarch32_VDIV_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode 'xxxx1110 1x00xxxx xxxx10xx x0x0xxxx'
        __guard cond != '1111'
        __decode
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && cond != '1110' then UNPREDICTABLE;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __encoding aarch32_VDIV_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101110 1x00xxxx xxxx10xx x0x0xxxx'
        __guard TRUE
        __decode
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __execute __conditional
        CheckVFPEnabled(TRUE);
        case esize of
            when 16
                S[d] = Zeros(16) : FPDiv(S[n]<15:0>, S[m]<15:0>, FPSCR);
            when 32
                S[d] = FPDiv(S[n], S[m], FPSCR);
            when 64
                D[d] = FPDiv(D[n], D[m], FPSCR);
