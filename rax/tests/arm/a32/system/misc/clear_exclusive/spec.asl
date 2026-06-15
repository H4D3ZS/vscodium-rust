__instruction aarch32_CLREX_A
    __encoding aarch32_CLREX_A1_A
        __instruction_set A32
        __opcode '11110101 0111xxxx xxxxxxxx 0001xxxx'
        __guard TRUE
        __decode
            // No additional decoding required

    __encoding aarch32_CLREX_T1_A
        __instruction_set T32
        __opcode '11110011 1011xxxx 10x0xxxx 0010xxxx'
        __guard TRUE
        __decode
            // No additional decoding required

    __execute __conditional
        ClearExclusiveLocal(ProcessorID());
