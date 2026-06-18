__instruction aarch32_NOP_A
    __encoding aarch32_NOP_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __opcode 'xxxx0011 00100000 xxxxxxxx 00000000'
        __guard cond != '1111'
        __decode
            // No additional decoding required

    __encoding aarch32_NOP_T1_A
        __instruction_set T16
        __opcode '10111111 00000000 00000000 00000000'
        __guard TRUE
        __decode
            // No additional decoding required

    __encoding aarch32_NOP_T2_A
        __instruction_set T32
        __opcode '11110011 1010xxxx 10x0x000 00000000'
        __guard TRUE
        __decode
            // No additional decoding required

    __execute __conditional
        // Do nothing
