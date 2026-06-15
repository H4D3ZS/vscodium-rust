__instruction aarch32_SMULL_A
    __encoding aarch32_SMULL_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field RdHi 16 +: 4
        __field RdLo 12 +: 4
        __field Rm 8 +: 4
        __field Rn 0 +: 4
        __opcode 'xxxx0000 110xxxxx xxxxxxxx 1001xxxx'
        __guard cond != '1111'
        __decode
            dLo = UInt(RdLo);  dHi = UInt(RdHi);  n = UInt(Rn);  m = UInt(Rm);  setflags = (S == '1');
            if dLo == 15 || dHi == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            if dHi == dLo then UNPREDICTABLE;

    __encoding aarch32_SMULL_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field RdLo 12 +: 4
        __field RdHi 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111011 1000xxxx xxxxxxxx 0000xxxx'
        __guard TRUE
        __decode
            dLo = UInt(RdLo);  dHi = UInt(RdHi);  n = UInt(Rn);  m = UInt(Rm);  setflags = FALSE;
            if dLo == 15 || dHi == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            // Armv8-A removes UNPREDICTABLE for R13
            if dHi == dLo then UNPREDICTABLE;

    __execute __conditional
        result = SInt(R[n]) * SInt(R[m]);
        R[dHi] = result<63:32>;
        R[dLo] = result<31:0>;
        if setflags then
            PSTATE.N = result<63>;
            PSTATE.Z = IsZeroBit(result<63:0>);
            // PSTATE.C, PSTATE.V unchanged

__instruction aarch32_SMLAL_A
    __encoding aarch32_SMLAL_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field RdHi 16 +: 4
        __field RdLo 12 +: 4
        __field Rm 8 +: 4
        __field Rn 0 +: 4
        __opcode 'xxxx0000 111xxxxx xxxxxxxx 1001xxxx'
        __guard cond != '1111'
        __decode
            dLo = UInt(RdLo);  dHi = UInt(RdHi);  n = UInt(Rn);  m = UInt(Rm);  setflags = (S == '1');
            if dLo == 15 || dHi == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            if dHi == dLo then UNPREDICTABLE;

    __encoding aarch32_SMLAL_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field RdLo 12 +: 4
        __field RdHi 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111011 1100xxxx xxxxxxxx 0000xxxx'
        __guard TRUE
        __decode
            dLo = UInt(RdLo);  dHi = UInt(RdHi);  n = UInt(Rn);  m = UInt(Rm);  setflags = FALSE;
            if dLo == 15 || dHi == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            // Armv8-A removes UNPREDICTABLE for R13
            if dHi == dLo then UNPREDICTABLE;

    __execute __conditional
        result = SInt(R[n]) * SInt(R[m]) + SInt(R[dHi]:R[dLo]);
        R[dHi] = result<63:32>;
        R[dLo] = result<31:0>;
        if setflags then
            PSTATE.N = result<63>;
            PSTATE.Z = IsZeroBit(result<63:0>);
            // PSTATE.C, PSTATE.V unchanged

__instruction aarch32_SMLALD_A
    __encoding aarch32_SMLALD_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field RdHi 16 +: 4
        __field RdLo 12 +: 4
        __field Rm 8 +: 4
        __field M 5 +: 1
        __field Rn 0 +: 4
        __opcode 'xxxx0111 0100xxxx xxxxxxxx 00x1xxxx'
        __guard cond != '1111'
        __decode
            dLo = UInt(RdLo);  dHi = UInt(RdHi);  n = UInt(Rn);  m = UInt(Rm);  m_swap = (M == '1');
            if dLo == 15 || dHi == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            if dHi == dLo then UNPREDICTABLE;

    __encoding aarch32_SMLALD_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field RdLo 12 +: 4
        __field RdHi 8 +: 4
        __field M 4 +: 1
        __field Rm 0 +: 4
        __opcode '11111011 1100xxxx xxxxxxxx 110xxxxx'
        __guard TRUE
        __decode
            dLo = UInt(RdLo);  dHi = UInt(RdHi);  n = UInt(Rn);  m = UInt(Rm);  m_swap = (M == '1');
            if dLo == 15 || dHi == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            // Armv8-A removes UNPREDICTABLE for R13
            if dHi == dLo then UNPREDICTABLE;

    __execute __conditional
        operand2 = if m_swap then ROR(R[m],16) else R[m];
        product1 = SInt(R[n]<15:0>) * SInt(operand2<15:0>);
        product2 = SInt(R[n]<31:16>) * SInt(operand2<31:16>);
        result = product1 + product2 + SInt(R[dHi]:R[dLo]);
        R[dHi] = result<63:32>;
        R[dLo] = result<31:0>;

__instruction aarch32_SMLALBB_A
    __encoding aarch32_SMLALBB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field RdHi 16 +: 4
        __field RdLo 12 +: 4
        __field Rm 8 +: 4
        __field M 6 +: 1
        __field N 5 +: 1
        __field Rn 0 +: 4
        __opcode 'xxxx0001 0100xxxx xxxxxxxx 1xx0xxxx'
        __guard cond != '1111'
        __decode
            dLo = UInt(RdLo);  dHi = UInt(RdHi);  n = UInt(Rn);  m = UInt(Rm);
            n_high = (N == '1');  m_high = (M == '1');
            if dLo == 15 || dHi == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            if dHi == dLo then UNPREDICTABLE;

    __encoding aarch32_SMLALBB_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field RdLo 12 +: 4
        __field RdHi 8 +: 4
        __field N 5 +: 1
        __field M 4 +: 1
        __field Rm 0 +: 4
        __opcode '11111011 1100xxxx xxxxxxxx 10xxxxxx'
        __guard TRUE
        __decode
            dLo = UInt(RdLo);  dHi = UInt(RdHi);  n = UInt(Rn);  m = UInt(Rm);
            n_high = (N == '1');  m_high = (M == '1');
            if dLo == 15 || dHi == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            // Armv8-A removes UNPREDICTABLE for R13
            if dHi == dLo then UNPREDICTABLE;

    __execute __conditional
        operand1 = if n_high then R[n]<31:16> else R[n]<15:0>;
        operand2 = if m_high then R[m]<31:16> else R[m]<15:0>;
        result = SInt(operand1) * SInt(operand2) + SInt(R[dHi]:R[dLo]);
        R[dHi] = result<63:32>;
        R[dLo] = result<31:0>;
