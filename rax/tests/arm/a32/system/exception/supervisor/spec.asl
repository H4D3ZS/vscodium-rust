__instruction aarch32_SVC_A
    __encoding aarch32_SVC_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field imm24 0 +: 24
        __opcode 'xxxx1111 xxxxxxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            imm32 = ZeroExtend(imm24, 32);

    __encoding aarch32_SVC_T1_A
        __instruction_set T16
        __field imm8 16 +: 8
        __opcode '11011111 xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            imm32 = ZeroExtend(imm8, 32);

    __execute __conditional
        AArch32.CallSupervisor(imm32<15:0>);
