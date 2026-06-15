__instruction aarch32_DBG_A
    __encoding aarch32_DBG_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field option 0 +: 4
        __opcode 'xxxx0011 00100000 xxxxxxxx 1111xxxx'
        __guard cond != '1111'
        __decode
            // DBG executes as a NOP. The 'option' field is ignored

    __encoding aarch32_DBG_T1_A
        __instruction_set T32
        __field option 0 +: 4
        __opcode '11110011 1010xxxx 10x0x000 1111xxxx'
        __guard TRUE
        __decode
            // DBG executes as a NOP. The 'option' field is ignored

    __execute __conditional
