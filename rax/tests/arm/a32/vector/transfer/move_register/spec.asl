__instruction aarch32_VMOV_r_A
    __encoding aarch32_VMOV_r_T2A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode 'xxxx1110 1x110000 xxxx101x 01x0xxxx'
        __guard cond != '1111'
        __decode
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            single_register = (size == '10');  advsimd = FALSE;
            if single_register then
                d = UInt(Vd:D);  m = UInt(Vm:M);
            else
                d = UInt(D:Vd);  m = UInt(M:Vm);  regs = 1;

    __encoding aarch32_VMOV_r_T2A2_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101110 1x110000 xxxx101x 01x0xxxx'
        __guard TRUE
        __decode
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            single_register = (size == '10');  advsimd = FALSE;
            if single_register then
                d = UInt(Vd:D);  m = UInt(Vm:M);
            else
                d = UInt(D:Vd);  m = UInt(M:Vm);  regs = 1;

    __execute __conditional
        CheckAdvSIMDOrVFPEnabled(TRUE, advsimd);
        if single_register then
            S[d] = S[m];
        else
            for r = 0 to regs-1
                D[d+r] = D[m+r];
