__instruction aarch32_SMC_AS
    __encoding aarch32_SMC_A1_AS
        __instruction_set A32
        __field cond 28 +: 4
        __field imm4 0 +: 4
        __opcode 'xxxx0001 0110xxxx xxxxxxxx 0111xxxx'
        __guard cond != '1111'
        __decode
            // imm4 is for assembly/disassembly only and is ignored by hardware

    __encoding aarch32_SMC_T1_AS
        __instruction_set T32
        __field imm4 16 +: 4
        __opcode '11110111 1111xxxx 1000xxxx xxxxxxxx'
        __guard TRUE
        __decode
            // imm4 is for assembly/disassembly only and is ignored by hardware
            if InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __execute __conditional

        AArch32.CheckForSMCUndefOrTrap();

        if !ELUsingAArch32(EL3) then
            if SCR_EL3.SMD == '1' then
                // SMC disabled.
                UNDEFINED;
        else
            if SCR.SCD == '1' then
                // SMC disabled
                if IsSecure() then
                    // Executes either as a NOP or UNALLOCATED.
                    c = ConstrainUnpredictable(Unpredictable_SMD);
                    assert c IN {Constraint_NOP, Constraint_UNDEF};
                    if c == Constraint_NOP then EndOfInstruction();
                UNDEFINED;

        if !ELUsingAArch32(EL3) then
            AArch64.CallSecureMonitor(Zeros(16));
        else
            AArch32.TakeSMCException();
