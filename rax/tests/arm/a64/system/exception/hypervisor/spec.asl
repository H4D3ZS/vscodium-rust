__instruction aarch64_system_exceptions_runtime_hvc
    __encoding aarch64_system_exceptions_runtime_hvc
        __instruction_set A64
        __field imm16 5 +: 16
        __opcode '11010100 000xxxxx xxxxxxxx xxx00010'
        __guard TRUE
        __decode
            bits(16) imm = imm16;

    __execute
        if !HaveEL(EL2) || PSTATE.EL == EL0 || (PSTATE.EL == EL1 && (!IsSecureEL2Enabled() && IsSecure())) then
            UNDEFINED;

        hvc_enable = if HaveEL(EL3) then SCR_EL3.HCE else NOT(HCR_EL2.HCD);
        if hvc_enable == '0' then
            AArch64.UndefinedFault();
        else
            AArch64.CallHypervisor(imm);
