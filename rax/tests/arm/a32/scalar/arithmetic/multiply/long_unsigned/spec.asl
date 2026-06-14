__instruction aarch32_UMULL_A
    __encoding aarch32_UMULL_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field RdHi 16 +: 4
        __field RdLo 12 +: 4
        __field Rm 8 +: 4
        __field Rn 0 +: 4
        __opcode 'xxxx0000 100xxxxx xxxxxxxx 1001xxxx'
        __guard cond != '1111'
        __decode
            dLo = UInt(RdLo);  dHi = UInt(RdHi);  n = UInt(Rn);  m = UInt(Rm);  setflags = (S == '1');
            if dLo == 15 || dHi == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            if dHi == dLo then UNPREDICTABLE;

    __encoding aarch32_UMULL_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field RdLo 12 +: 4
        __field RdHi 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111011 1010xxxx xxxxxxxx 0000xxxx'
        __guard TRUE
        __decode
            dLo = UInt(RdLo);  dHi = UInt(RdHi);  n = UInt(Rn);  m = UInt(Rm);  setflags = FALSE;
            if dLo == 15 || dHi == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            // Armv8-A removes UNPREDICTABLE for R13
            if dHi == dLo then UNPREDICTABLE;

    __execute __conditional
        result = UInt(R[n]) * UInt(R[m]);
        R[dHi] = result<63:32>;
        R[dLo] = result<31:0>;
        if setflags then
            PSTATE.N = result<63>;
            PSTATE.Z = IsZeroBit(result<63:0>);
            // PSTATE.C, PSTATE.V unchanged

__instruction aarch32_UMLAL_A
    __encoding aarch32_UMLAL_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field RdHi 16 +: 4
        __field RdLo 12 +: 4
        __field Rm 8 +: 4
        __field Rn 0 +: 4
        __opcode 'xxxx0000 101xxxxx xxxxxxxx 1001xxxx'
        __guard cond != '1111'
        __decode
            dLo = UInt(RdLo);  dHi = UInt(RdHi);  n = UInt(Rn);  m = UInt(Rm);  setflags = (S == '1');
            if dLo == 15 || dHi == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            if dHi == dLo then UNPREDICTABLE;

    __encoding aarch32_UMLAL_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field RdLo 12 +: 4
        __field RdHi 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111011 1110xxxx xxxxxxxx 0000xxxx'
        __guard TRUE
        __decode
            dLo = UInt(RdLo);  dHi = UInt(RdHi);  n = UInt(Rn);  m = UInt(Rm);  setflags = FALSE;
            if dLo == 15 || dHi == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            // Armv8-A removes UNPREDICTABLE for R13
            if dHi == dLo then UNPREDICTABLE;

    __execute __conditional
        result = UInt(R[n]) * UInt(R[m]) + UInt(R[dHi]:R[dLo]);
        R[dHi] = result<63:32>;
        R[dLo] = result<31:0>;
        if setflags then
            PSTATE.N = result<63>;
            PSTATE.Z = IsZeroBit(result<63:0>);
            // PSTATE.C, PSTATE.V unchanged

__instruction aarch32_UMAAL_A
    __encoding aarch32_UMAAL_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field RdHi 16 +: 4
        __field RdLo 12 +: 4
        __field Rm 8 +: 4
        __field Rn 0 +: 4
        __opcode 'xxxx0000 0100xxxx xxxxxxxx 1001xxxx'
        __guard cond != '1111'
        __decode
            dLo = UInt(RdLo);  dHi = UInt(RdHi);  n = UInt(Rn);  m = UInt(Rm);
            if dLo == 15 || dHi == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            if dHi == dLo then UNPREDICTABLE;

    __encoding aarch32_UMAAL_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field RdLo 12 +: 4
        __field RdHi 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111011 1110xxxx xxxxxxxx 0110xxxx'
        __guard TRUE
        __decode
            dLo = UInt(RdLo);  dHi = UInt(RdHi);  n = UInt(Rn);  m = UInt(Rm);
            if dLo == 15 || dHi == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            // Armv8-A removes UNPREDICTABLE for R13
            if dHi == dLo then UNPREDICTABLE;

    __execute __conditional
        result = UInt(R[n]) * UInt(R[m]) + UInt(R[dHi]) + UInt(R[dLo]);
        R[dHi] = result<63:32>;
        R[dLo] = result<31:0>;
