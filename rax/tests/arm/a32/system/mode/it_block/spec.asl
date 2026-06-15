__instruction aarch32_IT_A
    __encoding aarch32_IT_T1_A
        __instruction_set T16
        __field firstcond 20 +: 4
        __field mask 16 +: 4
        __opcode '10111111 xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            if mask == '0000' then SEE "Related encodings";
            if firstcond == '1111' || (firstcond == '1110' && BitCount(mask) != 1) then UNPREDICTABLE;
            if InITBlock() then UNPREDICTABLE;

    __execute
        AArch32.CheckITEnabled(mask);
        PSTATE.IT<7:0> = firstcond:mask;
        ShouldAdvanceIT = FALSE;
