__instruction aarch32_CPS_AS
    __encoding aarch32_CPS_A1_AS
        __instruction_set A32
        __field imod 18 +: 2
        __field M 17 +: 1
        __field A 8 +: 1
        __field I 7 +: 1
        __field F 6 +: 1
        __field mode 0 +: 5
        __opcode '11110001 0000xxx0 xxxxxxxx xx0xxxxx'
        __guard TRUE
        __decode
            if mode != '00000' && M == '0' then UNPREDICTABLE;
            if (imod<1> == '1' && A:I:F == '000') || (imod<1> == '0' && A:I:F != '000') then UNPREDICTABLE;
            enable = (imod == '10');  disable = (imod == '11');  changemode = (M == '1');
            affectA = (A == '1');  affectI = (I == '1');  affectF = (F == '1');
            if (imod == '00' && M == '0') || imod == '01' then UNPREDICTABLE;

    __encoding aarch32_CPS_T1_AS
        __instruction_set T16
        __field im 20 +: 1
        __field A 18 +: 1
        __field I 17 +: 1
        __field F 16 +: 1
        __opcode '10110110 011xxxxx 00000000 00000000'
        __guard TRUE
        __decode
            if A:I:F == '000' then UNPREDICTABLE;
            enable = (im == '0');  disable = (im == '1');  changemode = FALSE;
            affectA = (A == '1');  affectI = (I == '1');  affectF = (F == '1');
            if InITBlock() then UNPREDICTABLE;

    __encoding aarch32_CPS_T2_AS
        __instruction_set T32
        __field imod 9 +: 2
        __field M 8 +: 1
        __field A 7 +: 1
        __field I 6 +: 1
        __field F 5 +: 1
        __field mode 0 +: 5
        __opcode '11110011 1010xxxx 10x0xxxx xxxxxxxx'
        __guard TRUE
        __decode
            if imod == '00' && M == '0' then SEE "Hint instructions";
            if mode != '00000' && M == '0' then UNPREDICTABLE;
            if (imod<1> == '1' && A:I:F == '000') || (imod<1> == '0' && A:I:F != '000') then UNPREDICTABLE;
            enable = (imod == '10');  disable = (imod == '11');  changemode = (M == '1');
            affectA = (A == '1');  affectI = (I == '1');  affectF = (F == '1');
            if imod == '01' || InITBlock() then UNPREDICTABLE;

    __execute
        if CurrentInstrSet() == InstrSet_A32 then
            EncodingSpecificOperations();
            if PSTATE.EL != EL0 then
                if enable then
                    if affectA then PSTATE.A = '0';
                    if affectI then PSTATE.I = '0';
                    if affectF then PSTATE.F = '0';
                if disable then
                    if affectA then PSTATE.A = '1';
                    if affectI then PSTATE.I = '1';
                    if affectF then PSTATE.F = '1';
                if changemode then
                    // AArch32.WriteModeByInstr() sets PSTATE.IL to 1 if this is an illegal mode change.
                    AArch32.WriteModeByInstr(mode);
        else
            EncodingSpecificOperations();
            if PSTATE.EL != EL0 then
                if enable then
                    if affectA then PSTATE.A = '0';
                    if affectI then PSTATE.I = '0';
                    if affectF then PSTATE.F = '0';
                if disable then
                    if affectA then PSTATE.A = '1';
                    if affectI then PSTATE.I = '1';
                    if affectF then PSTATE.F = '1';
                if changemode then
                    // AArch32.WriteModeByInstr() sets PSTATE.IL to 1 if this is an illegal mode change.
                    AArch32.WriteModeByInstr(mode);
