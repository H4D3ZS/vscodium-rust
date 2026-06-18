__instruction aarch32_VMOVX_A
    __encoding aarch32_VMOVX_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111110 1x110000 xxxx1010 01x0xxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            d = UInt(Vd:D); m = UInt(Vm:M);

    __encoding aarch32_VMOVX_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111110 1x110000 xxxx1010 01x0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveFP16Ext() then UNDEFINED;
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            d = UInt(Vd:D); m = UInt(Vm:M);

    __execute __conditional
        CheckVFPEnabled(TRUE);
        S[d] = Zeros(16) : S[m]<31:16>;

__instruction aarch32_VINS_A
    __encoding aarch32_VINS_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111110 1x110000 xxxx1010 11x0xxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            d = UInt(Vd:D); m = UInt(Vm:M);

    __encoding aarch32_VINS_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111110 1x110000 xxxx1010 11x0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveFP16Ext() then UNDEFINED;
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            d = UInt(Vd:D); m = UInt(Vm:M);

    __execute __conditional
        CheckVFPEnabled(TRUE);
        S[d] = S[m]<15:0> : S[d]<15:0>;
