__instruction aarch32_SEV_A
    __encoding aarch32_SEV_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __opcode 'xxxx0011 00100000 xxxxxxxx 00000100'
        __guard cond != '1111'
        __decode
            // No additional decoding required

    __encoding aarch32_SEV_T1_A
        __instruction_set T16
        __opcode '10111111 01000000 00000000 00000000'
        __guard TRUE
        __decode
            // No additional decoding required

    __encoding aarch32_SEV_T2_A
        __instruction_set T32
        __opcode '11110011 1010xxxx 10x0x000 00000100'
        __guard TRUE
        __decode
            // No additional decoding required

    __execute __conditional
        SendEvent();

__instruction aarch32_SEVL_A
    __encoding aarch32_SEVL_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __opcode 'xxxx0011 00100000 xxxxxxxx 00000101'
        __guard cond != '1111'
        __decode
            // No additional decoding required

    __encoding aarch32_SEVL_T1_A
        __instruction_set T16
        __opcode '10111111 01010000 00000000 00000000'
        __guard TRUE
        __decode
            // No additional decoding required

    __encoding aarch32_SEVL_T2_A
        __instruction_set T32
        __opcode '11110011 1010xxxx 10x0x000 00000101'
        __guard TRUE
        __decode
            // No additional decoding required

    __execute __conditional
        SendEventLocal();
