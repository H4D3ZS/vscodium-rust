__instruction aarch32_ERET_AS
    __encoding aarch32_ERET_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __opcode 'xxxx0001 0110xxxx xxxxxxxx 0110xxxx'
        __guard cond != '1111'
        __decode
            // No additional decoding required

    __encoding aarch32_ERET_T1_A
        __instruction_set T32
        __opcode '11110011 11011110 10x0xxxx 00000000'
        __guard TRUE
        __decode
            if InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __execute __conditional
        if PSTATE.M IN {M32_User,M32_System} then
            UNPREDICTABLE;                        // UNDEFINED or NOP
        else
            new_pc_value = if PSTATE.EL == EL2 then ELR_hyp else R[14];
            AArch32.ExceptionReturn(new_pc_value, SPSR[]);
