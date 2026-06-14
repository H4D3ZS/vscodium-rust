__instruction aarch32_SB_A
    __encoding aarch32_SB_A1_A
        __instruction_set A32
        __opcode '11110101 0111xxxx xxxxxxxx 0111xxxx'
        __guard TRUE
        __decode
            // No additional decoding required

    __encoding aarch32_SB_T1_A
        __instruction_set T32
        __opcode '11110011 1011xxxx 10x0xxxx 0111xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;

    __execute __conditional
        SpeculationBarrier();

__instruction aarch32_CSDB_A
    __encoding aarch32_CSDB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __opcode 'xxxx0011 00100000 xxxxxxxx 00010100'
        __guard cond != '1111'
        __decode
            if cond != '1110' then UNPREDICTABLE;      // CSDB must be encoded with AL condition

    __encoding aarch32_CSDB_T1_A
        __instruction_set T32
        __opcode '11110011 1010xxxx 10x0x000 00010100'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;

    __execute __conditional

        ConsumptionOfSpeculativeDataBarrier();

__instruction aarch32_SSBB_A
    __encoding aarch32_SSBB_A1_A
        __instruction_set A32
        __opcode '11110101 0111xxxx xxxxxxxx 01000000'
        __guard TRUE
        __decode
            // No additional decoding required

    __encoding aarch32_SSBB_T1_A
        __instruction_set T32
        __opcode '11110011 1011xxxx 10x0xxxx 01000000'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;

    __execute __conditional
        SpeculativeStoreBypassBarrierToVA();

__instruction aarch32_PSSBB_A
    __encoding aarch32_PSSBB_A1_A
        __instruction_set A32
        __opcode '11110101 0111xxxx xxxxxxxx 01000100'
        __guard TRUE
        __decode
            // No additional decoding required

    __encoding aarch32_PSSBB_T1_A
        __instruction_set T32
        __opcode '11110011 1011xxxx 10x0xxxx 01000100'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;

    __execute __conditional
        SpeculativeStoreBypassBarrierToPA();
