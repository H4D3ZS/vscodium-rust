__instruction aarch32_TBB_A
    __encoding aarch32_TBB_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field H 4 +: 1
        __field Rm 0 +: 4
        __opcode '11101000 1101xxxx xxxxxxxx 000xxxxx'
        __guard TRUE
        __decode
            n = UInt(Rn);  m = UInt(Rm);  is_tbh = (H == '1');
            if m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13
            if InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __execute __conditional
        if is_tbh then
            halfwords = UInt(MemU[R[n]+LSL(R[m],1), 2]);
        else
            halfwords = UInt(MemU[R[n]+R[m], 1]);
        BranchWritePC(PC + 2*halfwords, BranchType_INDIR);
