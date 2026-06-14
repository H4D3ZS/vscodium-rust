__instruction aarch32_VSEL_A
    __encoding aarch32_VSEL_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field cc 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111110 0xxxxxxx xxxx10xx x0x0xxxx'
        __guard TRUE
        __decode
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);
            cond = cc:(cc<1> EOR cc<0>):'0';

    __encoding aarch32_VSEL_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field cc 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111110 0xxxxxxx xxxx10xx x0x0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);
            cond = cc:(cc<1> EOR cc<0>):'0';

    __execute
        CheckVFPEnabled(TRUE);
        case esize of
            when 16
                S[d] = Zeros(16) : (if ConditionHolds(cond) then S[n] else S[m])<15:0>;
            when 32
                S[d] = if ConditionHolds(cond) then S[n] else S[m];
            when 64
                D[d] = if ConditionHolds(cond) then D[n] else D[m];
