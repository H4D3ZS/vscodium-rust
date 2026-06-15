__instruction aarch32_HVC_AS
    __encoding aarch32_HVC_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field imm12 8 +: 12
        __field imm4 0 +: 4
        __opcode 'xxxx0001 0100xxxx xxxxxxxx 0111xxxx'
        __guard cond != '1111'
        __decode
            if cond != '1110' then UNPREDICTABLE;
            imm16 = imm12:imm4;

    __encoding aarch32_HVC_T1_A
        __instruction_set T32
        __field imm4 16 +: 4
        __field imm12 0 +: 12
        __opcode '11110111 1110xxxx 1000xxxx xxxxxxxx'
        __guard TRUE
        __decode
            imm16 = imm4:imm12;
            if InITBlock() then UNPREDICTABLE;

    __execute
        if !HaveEL(EL2) || PSTATE.EL == EL0 || (IsSecure() && !IsSecureEL2Enabled()) then
            UNDEFINED;

        if HaveEL(EL3) then
            if ELUsingAArch32(EL3) && SCR.HCE == '0' && PSTATE.EL == EL2 then
                UNPREDICTABLE;
            else
                hvc_enable = SCR_GEN[].HCE;
        else
            hvc_enable = if ELUsingAArch32(EL2) then NOT(HCR.HCD) else NOT(HCR_EL2.HCD);

        if hvc_enable == '0' then
            UNDEFINED;
        else
            AArch32.CallHypervisor(imm16);
