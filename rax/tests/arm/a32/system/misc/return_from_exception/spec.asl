__instruction aarch32_RFE_AS
    __encoding aarch32_RFE_A1_AS
        __instruction_set A32
        __field P 24 +: 1
        __field U 23 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __opcode '1111100x x0x1xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            n = UInt(Rn);
            wback = (W == '1');  increment = (U == '1');  wordhigher = (P == U);
            if n == 15 then UNPREDICTABLE;

    __encoding aarch32_RFE_T1_AS
        __instruction_set T32
        __field W 21 +: 1
        __field Rn 16 +: 4
        __opcode '11101000 00x1xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            n = UInt(Rn);  wback = (W == '1');  increment = FALSE;  wordhigher = FALSE;
            if n == 15 then UNPREDICTABLE;
            if InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __encoding aarch32_RFE_T2_AS
        __instruction_set T32
        __field W 21 +: 1
        __field Rn 16 +: 4
        __opcode '11101001 10x1xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            n = UInt(Rn);  wback = (W == '1');  increment = TRUE;  wordhigher = FALSE;
            if n == 15 then UNPREDICTABLE;
            if InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __execute __conditional
        if PSTATE.EL == EL2 then
            UNDEFINED;
        elsif PSTATE.EL == EL0 then
            UNPREDICTABLE;                        // UNDEFINED or NOP
        else
            address = if increment then R[n] else R[n]-8;
            if wordhigher then address = address+4;
            new_pc_value = MemA[address,4];
            spsr = MemA[address+4,4];
            if wback then R[n] = if increment then R[n]+8 else R[n]-8;
            AArch32.ExceptionReturn(new_pc_value, spsr);
