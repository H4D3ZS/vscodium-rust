__instruction aarch32_VCMP_A
    __encoding aarch32_VCMP_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field E 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode 'xxxx1110 1x110100 xxxx10xx 11x0xxxx'
        __guard cond != '1111'
        __decode
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && cond != '1110' then UNPREDICTABLE;
            quiet_nan_exc = (E == '1');  with_zero = FALSE;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); m = UInt(M:Vm);

    __encoding aarch32_VCMP_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field E 7 +: 1
        __opcode 'xxxx1110 1x110101 xxxx10xx 11x0xxxx'
        __guard cond != '1111'
        __decode
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && cond != '1110' then UNPREDICTABLE;
            quiet_nan_exc = (E == '1');  with_zero = TRUE;
            case size of
                when '01' esize = 16; d = UInt(Vd:D);
                when '10' esize = 32; d = UInt(Vd:D);
                when '11' esize = 64; d = UInt(D:Vd);

    __encoding aarch32_VCMP_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field E 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101110 1x110100 xxxx10xx 11x0xxxx'
        __guard TRUE
        __decode
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            quiet_nan_exc = (E == '1');  with_zero = FALSE;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); m = UInt(M:Vm);

    __encoding aarch32_VCMP_T2_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field E 7 +: 1
        __opcode '11101110 1x110101 xxxx10xx 11x0xxxx'
        __guard TRUE
        __decode
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            quiet_nan_exc = (E == '1');  with_zero = TRUE;
            case size of
                when '01' esize = 16; d = UInt(Vd:D);
                when '10' esize = 32; d = UInt(Vd:D);
                when '11' esize = 64; d = UInt(D:Vd);

    __execute __conditional
        CheckVFPEnabled(TRUE);
        bits(4) nzcv;
        case esize of
            when 16
                bits(16) op16 = if with_zero then FPZero('0') else S[m]<15:0>;
                nzcv = FPCompare(S[d]<15:0>, op16, quiet_nan_exc, FPSCR);
            when 32
                bits(32) op32 = if with_zero then FPZero('0') else S[m];
                nzcv = FPCompare(S[d], op32, quiet_nan_exc, FPSCR);
            when 64
                bits(64) op64 = if with_zero then FPZero('0') else D[m];
                nzcv = FPCompare(D[d], op64, quiet_nan_exc, FPSCR);

        FPSCR.<N,Z,C,V> = nzcv;
