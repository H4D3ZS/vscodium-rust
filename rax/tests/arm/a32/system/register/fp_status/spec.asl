__instruction aarch32_VMSR_AS
    __encoding aarch32_VMSR_T1A1_AS
        __instruction_set A32
        __field cond 28 +: 4
        __field reg 16 +: 4
        __field Rt 12 +: 4
        __opcode 'xxxx1110 1110xxxx xxxx1010 xxx1xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);
            if reg != '000x' && reg != '1000' then UNPREDICTABLE;
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __encoding aarch32_VMSR_T1A1_AS
        __instruction_set T32
        __field reg 16 +: 4
        __field Rt 12 +: 4
        __opcode '11101110 1110xxxx xxxx1010 xxx1xxxx'
        __guard TRUE
        __decode
            t = UInt(Rt);
            if reg != '000x' && reg != '1000' then UNPREDICTABLE;
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        if reg == '0001' then                 // FPSCR
            CheckVFPEnabled(TRUE);
            FPSCR = R[t];
        elsif PSTATE.EL == EL0 then
            UNDEFINED;                        // Non-FPSCR registers accessible only at PL1 or above
        else
            CheckVFPEnabled(FALSE);           // Non-FPSCR registers are not affected by FPEXC.EN
            case reg of
                when '0000'                   // VMSR access to FPSID is ignored
                when '1000'  FPEXC = R[t];
                otherwise    Unreachable();   // Dealt with above or in encoding-specific pseudocode

__instruction aarch32_VMRS_AS
    __encoding aarch32_VMRS_T1A1_AS
        __instruction_set A32
        __field cond 28 +: 4
        __field reg 16 +: 4
        __field Rt 12 +: 4
        __opcode 'xxxx1110 1111xxxx xxxx1010 xxx1xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);
            if !(reg IN {'000x', '0101', '011x', '1000'}) then UNPREDICTABLE;
            if t == 15 && reg != '0001' then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __encoding aarch32_VMRS_T1A1_AS
        __instruction_set T32
        __field reg 16 +: 4
        __field Rt 12 +: 4
        __opcode '11101110 1111xxxx xxxx1010 xxx1xxxx'
        __guard TRUE
        __decode
            t = UInt(Rt);
            if !(reg IN {'000x', '0101', '011x', '1000'}) then UNPREDICTABLE;
            if t == 15 && reg != '0001' then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        if reg == '0001' then                 // FPSCR
            CheckVFPEnabled(TRUE);
            if t == 15 then
                PSTATE.<N,Z,C,V> = FPSR.<N,Z,C,V>;
            else
                R[t] = FPSCR;
        elsif PSTATE.EL == EL0 then
            UNDEFINED;                        // Non-FPSCR registers accessible only at PL1 or above
        else
            CheckVFPEnabled(FALSE);           // Non-FPSCR registers are not affected by FPEXC.EN
            AArch32.CheckAdvSIMDOrFPRegisterTraps(reg);
            case reg of
                when '0000'  R[t] = FPSID;
                when '0101'  R[t] = MVFR2;
                when '0110'  R[t] = MVFR1;
                when '0111'  R[t] = MVFR0;
                when '1000'  R[t] = FPEXC;
                otherwise    Unreachable();   // Dealt with above or in encoding-specific pseudocode
