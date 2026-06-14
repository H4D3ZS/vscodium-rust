__instruction aarch32_UDF_A
    __encoding aarch32_UDF_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field imm12 8 +: 12
        __field imm4 0 +: 4
        __opcode '11100111 1111xxxx xxxxxxxx 1111xxxx'
        __guard cond != '1111'
        __decode
            imm32 = ZeroExtend(imm12:imm4, 32);
            // imm32 is for assembly and disassembly only, and is ignored by hardware.

    __encoding aarch32_UDF_T1_A
        __instruction_set T16
        __field imm8 16 +: 8
        __opcode '11011110 xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            imm32 = ZeroExtend(imm8, 32);
            // imm32 is for assembly and disassembly only, and is ignored by hardware.

    __encoding aarch32_UDF_T2_A
        __instruction_set T32
        __field imm4 16 +: 4
        __field imm12 0 +: 12
        __opcode '11110111 1111xxxx 1010xxxx xxxxxxxx'
        __guard TRUE
        __decode
            imm32 = ZeroExtend(imm4:imm12, 32);
            // imm32 is for assembly and disassembly only, and is ignored by hardware.

    __execute __conditional
        UNDEFINED;
