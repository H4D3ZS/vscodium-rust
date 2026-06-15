__instruction aarch32_ESB_A
    __encoding aarch32_ESB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __opcode 'xxxx0011 00100000 xxxxxxxx 00010000'
        __guard cond != '1111'
        __decode
            if !HaveRASExt() then EndOfInstruction();  // Instruction executes as NOP
            if cond != '1110' then UNPREDICTABLE;      // ESB must be encoded with AL condition

    __encoding aarch32_ESB_T1_A
        __instruction_set T32
        __opcode '11110011 1010xxxx 10x0x000 00010000'
        __guard TRUE
        __decode
            if !HaveRASExt() then EndOfInstruction();  // Instruction executes as NOP
            if InITBlock() then UNPREDICTABLE;

    __execute __conditional

        SynchronizeErrors();
        AArch32.ESBOperation();
        if EL2Enabled() && PSTATE.EL IN {EL0,EL1} then AArch32.vESBOperation();
        TakeUnmaskedSErrorInterrupts();
