__instruction aarch32_SETEND_A
    __encoding aarch32_SETEND_A1_A
        __instruction_set A32
        __field E 9 +: 1
        __opcode '11110001 0000xxx1 xxxxxxxx 0000xxxx'
        __guard TRUE
        __decode
            set_bigend = (E == '1');

    __encoding aarch32_SETEND_T1_A
        __instruction_set T16
        __field E 19 +: 1
        __opcode '10110110 010xxxxx 00000000 00000000'
        __guard TRUE
        __decode
            set_bigend = (E == '1');
            if InITBlock() then UNPREDICTABLE;

    __execute
        AArch32.CheckSETENDEnabled();
        PSTATE.E = if set_bigend then '1' else '0';
