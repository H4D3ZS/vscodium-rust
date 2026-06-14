__instruction aarch32_VJCVT_A
    __encoding aarch32_VJCVT_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode 'xxxx1110 1x111001 xxxx1011 11x0xxxx'
        __guard cond != '1111'
        __decode
            if !HaveFJCVTZSExt() then UNDEFINED;
            if cond != '1110' then UNPREDICTABLE;
            d = UInt(Vd:D);  m = UInt(M:Vm);

    __encoding aarch32_VJCVT_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101110 1x111001 xxxx1011 11x0xxxx'
        __guard TRUE
        __decode
            if !HaveFJCVTZSExt() then UNDEFINED;
            if InITBlock() then UNPREDICTABLE;
            d = UInt(Vd:D);  m = UInt(M:Vm);

    __execute
        CheckVFPEnabled(TRUE);
        bits(64) fltval = D[m];
        bits(32) intval = FPToFixedJS(fltval, FPSCR, FALSE);
        S[d] = intval;
