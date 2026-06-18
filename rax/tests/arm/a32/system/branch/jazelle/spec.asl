__instruction aarch32_BXJ_A
    __encoding aarch32_BXJ_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0001 0010xxxx xxxxxxxx 0010xxxx'
        __guard cond != '1111'
        __decode
            m = UInt(Rm);
            if m == 15 then UNPREDICTABLE;

    __encoding aarch32_BXJ_T1_A
        __instruction_set T32
        __field Rm 16 +: 4
        __opcode '11110011 1100xxxx 10x0xxxx xxxxxxxx'
        __guard TRUE
        __decode
            m = UInt(Rm);
            if m == 15 then UNPREDICTABLE;  // Armv8-A removes UNPREDICTABLE for R13
            if InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __execute __conditional
        BXWritePC(R[m], BranchType_INDIR);
