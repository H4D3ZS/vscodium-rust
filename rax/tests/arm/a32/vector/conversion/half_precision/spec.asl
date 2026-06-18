__instruction aarch32_VCVTB_A
    __encoding aarch32_VCVTB_T1A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field op 16 +: 1
        __field Vd 12 +: 4
        __field sz 8 +: 1
        __field T 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode 'xxxx1110 1x11001x xxxx101x 11x0xxxx'
        __guard cond != '1111'
        __decode
            uses_double = (sz == '1'); convert_from_half = (op == '0');
            lowbit = (if T == '1' then 16 else 0);
            if uses_double then
                if convert_from_half then
                    d = UInt(D:Vd); m = UInt(Vm:M);
                else
                    d = UInt(Vd:D); m = UInt(M:Vm);
            else
                d = UInt(Vd:D); m = UInt(Vm:M);

    __encoding aarch32_VCVTB_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field op 16 +: 1
        __field Vd 12 +: 4
        __field sz 8 +: 1
        __field T 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101110 1x11001x xxxx101x 11x0xxxx'
        __guard TRUE
        __decode
            uses_double = (sz == '1'); convert_from_half = (op == '0');
            lowbit = (if T == '1' then 16 else 0);
            if uses_double then
                if convert_from_half then
                    d = UInt(D:Vd); m = UInt(Vm:M);
                else
                    d = UInt(Vd:D); m = UInt(M:Vm);
            else
                d = UInt(Vd:D); m = UInt(Vm:M);

    __execute __conditional
        CheckVFPEnabled(TRUE);
        bits(16) hp;
        if convert_from_half then
            hp = S[m]<lowbit+15:lowbit>;
            if uses_double then
                D[d] = FPConvert(hp, FPSCR);
            else
                S[d] = FPConvert(hp, FPSCR);
        else
            if uses_double then
                hp = FPConvert(D[m], FPSCR);
            else
                hp = FPConvert(S[m], FPSCR);
            S[d]<lowbit+15:lowbit> = hp;
