__instruction aarch32_ISB_A
    __encoding aarch32_ISB_A1_A
        __instruction_set A32
        __field option 0 +: 4
        __opcode '11110101 0111xxxx xxxxxxxx 0110xxxx'
        __guard TRUE
        __decode
            // No additional decoding required

    __encoding aarch32_ISB_T1_A
        __instruction_set T32
        __field option 0 +: 4
        __opcode '11110011 1011xxxx 10x0xxxx 0110xxxx'
        __guard TRUE
        __decode
            // No additional decoding required

    __execute __conditional
        InstructionSynchronizationBarrier();
