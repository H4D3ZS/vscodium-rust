__instruction aarch32_SETPAN_A
    __encoding aarch32_SETPAN_A1_A
        __instruction_set A32
        __field imm1 9 +: 1
        __opcode '11110001 0001xxxx xxxxxxxx 0000xxxx'
        __guard TRUE
        __decode
            if !HavePANExt() then UNDEFINED;
            value = imm1;

    __encoding aarch32_SETPAN_T1_A
        __instruction_set T16
        __field imm1 19 +: 1
        __opcode '10110110 000xxxxx 00000000 00000000'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HavePANExt() then UNDEFINED;
            value = imm1;

    __execute
        if PSTATE.EL != EL0 then
            PSTATE.PAN = value;
