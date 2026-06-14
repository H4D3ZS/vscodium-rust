__instruction aarch64_system_exceptions_runtime_smc
    __encoding aarch64_system_exceptions_runtime_smc
        __instruction_set A64
        __field imm16 5 +: 16
        __opcode '11010100 000xxxxx xxxxxxxx xxx00011'
        __guard TRUE
        __decode
            bits(16) imm = imm16;

    __execute
        AArch64.CheckForSMCUndefOrTrap(imm);

        if SCR_EL3.SMD == '1' then
            // SMC disabled
            AArch64.UndefinedFault();
        else
            AArch64.CallSecureMonitor(imm);
