__instruction aarch32_BKPT_A
    __encoding aarch32_BKPT_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field imm12 8 +: 12
        __field imm4 0 +: 4
        __opcode 'xxxx0001 0010xxxx xxxxxxxx 0111xxxx'
        __guard cond != '1111'
        __decode
            imm16 = imm12:imm4;
            if cond != '1110' then UNPREDICTABLE;  // BKPT must be encoded with AL condition

    __encoding aarch32_BKPT_T1_A
        __instruction_set T16
        __field imm8 16 +: 8
        __opcode '10111110 xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            imm16 = ZeroExtend(imm8, 16);

    __execute
        AArch32.SoftwareBreakpoint(imm16);

__instruction aarch32_HLT_A
    __encoding aarch32_HLT_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field imm12 8 +: 12
        __field imm4 0 +: 4
        __opcode 'xxxx0001 0000xxxx xxxxxxxx 0111xxxx'
        __guard cond != '1111'
        __decode
            if EDSCR.HDE == '0' || !HaltingAllowed() then UNDEFINED;
            if cond != '1110' then UNPREDICTABLE; // HLT must be encoded with AL condition

    __encoding aarch32_HLT_T1_A
        __instruction_set T16
        __field imm6 16 +: 6
        __opcode '10111010 10xxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            if EDSCR.HDE == '0' || !HaltingAllowed() then UNDEFINED;

    __execute
        Halt(DebugHalt_HaltInstruction);

__instruction aarch32_DCPS_A
    __encoding aarch32_DCPS_T1_A
        __instruction_set T32
        __field opt 0 +: 2
        __opcode '11110111 10001111 10000000 000000xx'
        __guard TRUE
        __decode
            if !Halted() || opt == '00' then UNDEFINED;

    __execute
        DCPSInstruction(opt);
