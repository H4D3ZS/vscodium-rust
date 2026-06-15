__instruction aarch32_VCVT_ds_A
    __encoding aarch32_VCVT_ds_T1A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode 'xxxx1110 1x110111 xxxx101x 11x0xxxx'
        __guard cond != '1111'
        __decode
            double_to_single = (size == '11');
            d = if double_to_single then UInt(Vd:D) else UInt(D:Vd);
            m = if double_to_single then UInt(M:Vm) else UInt(Vm:M);

    __encoding aarch32_VCVT_ds_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101110 1x110111 xxxx101x 11x0xxxx'
        __guard TRUE
        __decode
            double_to_single = (size == '11');
            d = if double_to_single then UInt(Vd:D) else UInt(D:Vd);
            m = if double_to_single then UInt(M:Vm) else UInt(Vm:M);

    __execute __conditional
        CheckVFPEnabled(TRUE);
        if double_to_single then
            S[d] = FPConvert(D[m], FPSCR);
        else
            D[d] = FPConvert(S[m], FPSCR);

__instruction aarch32_VCVT_hs_A
    __encoding aarch32_VCVT_hs_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field op 8 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx10 xxxx011x 00x0xxxx'
        __guard TRUE
        __decode
            if size != '01' then UNDEFINED;
            half_to_single = (op == '1');
            if half_to_single && Vd<0> == '1' then UNDEFINED;
            if !half_to_single && Vm<0> == '1' then UNDEFINED;
            esize = 16;  elements = 4;
            m = UInt(M:Vm);  d = UInt(D:Vd);

    __encoding aarch32_VCVT_hs_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field op 8 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx10 xxxx011x 00x0xxxx'
        __guard TRUE
        __decode
            if size != '01' then UNDEFINED;
            half_to_single = (op == '1');
            if half_to_single && Vd<0> == '1' then UNDEFINED;
            if !half_to_single && Vm<0> == '1' then UNDEFINED;
            esize = 16;  elements = 4;
            m = UInt(M:Vm);  d = UInt(D:Vd);

    __execute __conditional
        CheckAdvSIMDEnabled();
        for e = 0 to elements-1
            if half_to_single then
                Elem[Q[d>>1],e,32] = FPConvert(Elem[Din[m],e,16], StandardFPSCRValue());
            else
                Elem[D[d],e,16] = FPConvert(Elem[Qin[m>>1],e,32], StandardFPSCRValue());
