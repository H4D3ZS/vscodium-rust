__instruction aarch32_SRS_AS
    __encoding aarch32_SRS_A1_AS
        __instruction_set A32
        __field P 24 +: 1
        __field U 23 +: 1
        __field W 21 +: 1
        __field mode 0 +: 5
        __opcode '1111100x x1x0xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            wback = (W == '1');  increment = (U == '1');  wordhigher = (P == U);

    __encoding aarch32_SRS_T1_AS
        __instruction_set T32
        __field W 21 +: 1
        __field mode 0 +: 5
        __opcode '11101000 00x0xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            wback = (W == '1');  increment = FALSE;  wordhigher = FALSE;

    __encoding aarch32_SRS_T2_AS
        __instruction_set T32
        __field W 21 +: 1
        __field mode 0 +: 5
        __opcode '11101001 10x0xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            wback = (W == '1');  increment = TRUE;  wordhigher = FALSE;

    __execute
        if CurrentInstrSet() == InstrSet_A32 then
            if ConditionPassed() then
                EncodingSpecificOperations();
                if PSTATE.EL == EL2 then          // UNDEFINED at EL2
                    UNDEFINED;

                // Check for UNPREDICTABLE cases. The definition of UNPREDICTABLE does not permit these
                // to be security holes
                if PSTATE.M IN {M32_User,M32_System} then
                    UNPREDICTABLE;
                elsif mode == M32_Hyp then        // Check for attempt to access Hyp mode SP
                    UNPREDICTABLE;
                elsif mode == M32_Monitor then    // Check for attempt to access Monitor mode SP
                    if !HaveEL(EL3) || !IsSecure()  then
                        UNPREDICTABLE;
                    elsif !ELUsingAArch32(EL3) then
                        AArch64.MonitorModeTrap();
                elsif BadMode(mode) then
                    UNPREDICTABLE;

                base = Rmode[13,mode];
                address = if increment then base else base-8;
                if wordhigher then address = address+4;
                MemA[address,4]   = LR;
                MemA[address+4,4] = SPSR[];
                if wback then Rmode[13,mode] = if increment then base+8 else base-8;
        else
            if ConditionPassed() then
                EncodingSpecificOperations();
                if PSTATE.EL == EL2 then          // UNDEFINED at EL2
                    UNDEFINED;

                // Check for UNPREDICTABLE cases. The definition of UNPREDICTABLE does not permit these
                // to be security holes
                if PSTATE.M IN {M32_User,M32_System} then
                    UNPREDICTABLE;
                elsif mode == M32_Hyp then        // Check for attempt to access Hyp mode SP
                    UNPREDICTABLE;
                elsif mode == M32_Monitor then    // Check for attempt to access Monitor mode SP
                    if !HaveEL(EL3) || !IsSecure()  then
                        UNPREDICTABLE;
                    elsif !ELUsingAArch32(EL3) then
                        AArch64.MonitorModeTrap();
                elsif BadMode(mode) then
                    UNPREDICTABLE;

                base = Rmode[13,mode];
                address = if increment then base else base-8;
                if wordhigher then address = address+4;
                MemA[address,4]   = LR;
                MemA[address+4,4] = SPSR[];
                if wback then Rmode[13,mode] = if increment then base+8 else base-8;
