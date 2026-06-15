__instruction aarch32_BX_A
    __encoding aarch32_BX_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0001 0010xxxx xxxxxxxx 0001xxxx'
        __guard cond != '1111'
        __decode
            m = UInt(Rm);

    __encoding aarch32_BX_T1_A
        __instruction_set T16
        __field Rm 19 +: 4
        __opcode '01000111 0xxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            m = UInt(Rm);
            if InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __execute __conditional
        BXWritePC(R[m], BranchType_INDIR);

__instruction aarch32_BLX_r_A
    __encoding aarch32_BLX_r_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0001 0010xxxx xxxxxxxx 0011xxxx'
        __guard cond != '1111'
        __decode
            m = UInt(Rm);
            if m == 15 then UNPREDICTABLE;

    __encoding aarch32_BLX_r_T1_A
        __instruction_set T16
        __field Rm 19 +: 4
        __opcode '01000111 1xxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            m = UInt(Rm);
            if m == 15 then UNPREDICTABLE;
            if InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __execute __conditional
        target = R[m];
        if CurrentInstrSet() == InstrSet_A32 then
            next_instr_addr = PC - 4;
            LR = next_instr_addr;
        else
            next_instr_addr = PC - 2;
            LR = next_instr_addr<31:1> : '1';
        BXWritePC(target, BranchType_INDCALL);
