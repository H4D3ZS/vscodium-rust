__instruction aarch32_WFE_A
    __encoding aarch32_WFE_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __opcode 'xxxx0011 00100000 xxxxxxxx 00000010'
        __guard cond != '1111'
        __decode
            // No additional decoding required

    __encoding aarch32_WFE_T1_A
        __instruction_set T16
        __opcode '10111111 00100000 00000000 00000000'
        __guard TRUE
        __decode
            // No additional decoding required

    __encoding aarch32_WFE_T2_A
        __instruction_set T32
        __opcode '11110011 1010xxxx 10x0x000 00000010'
        __guard TRUE
        __decode
            // No additional decoding required

    __execute __conditional
        if IsEventRegisterSet() then
            ClearEventRegister();
        else
            if PSTATE.EL == EL0 then
                // Check for traps described by the OS which may be EL1 or EL2.
                AArch32.CheckForWFxTrap(EL1, TRUE);
            if EL2Enabled() && PSTATE.EL IN {EL0,EL1} && !IsInHost() then
                // Check for traps described by the Hypervisor.
                AArch32.CheckForWFxTrap(EL2, TRUE);
            if HaveEL(EL3) && PSTATE.M != M32_Monitor then
                // Check for traps described by the Secure Monitor.
                AArch32.CheckForWFxTrap(EL3, TRUE);
            WaitForEvent();

__instruction aarch32_WFI_A
    __encoding aarch32_WFI_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __opcode 'xxxx0011 00100000 xxxxxxxx 00000011'
        __guard cond != '1111'
        __decode
            // No additional decoding required

    __encoding aarch32_WFI_T1_A
        __instruction_set T16
        __opcode '10111111 00110000 00000000 00000000'
        __guard TRUE
        __decode
            // No additional decoding required

    __encoding aarch32_WFI_T2_A
        __instruction_set T32
        __opcode '11110011 1010xxxx 10x0x000 00000011'
        __guard TRUE
        __decode
            // No additional decoding required

    __execute __conditional
        if !InterruptPending() then
            if PSTATE.EL == EL0 then
                // Check for traps described by the OS which may be EL1 or EL2.
                AArch32.CheckForWFxTrap(EL1, FALSE);
            if EL2Enabled() && PSTATE.EL IN {EL0,EL1} && !IsInHost() then
                // Check for traps described by the Hypervisor.
                AArch32.CheckForWFxTrap(EL2, FALSE);
            if HaveEL(EL3) && PSTATE.M != M32_Monitor then
                // Check for traps described by the Secure Monitor.
                AArch32.CheckForWFxTrap(EL3, FALSE);
            WaitForInterrupt();
