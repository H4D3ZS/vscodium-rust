__instruction aarch32_SMLSLD_A
    __encoding aarch32_SMLSLD_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field RdHi 16 +: 4
        __field RdLo 12 +: 4
        __field Rm 8 +: 4
        __field M 5 +: 1
        __field Rn 0 +: 4
        __opcode 'xxxx0111 0100xxxx xxxxxxxx 01x1xxxx'
        __guard cond != '1111'
        __decode
            dLo = UInt(RdLo);  dHi = UInt(RdHi);  n = UInt(Rn);  m = UInt(Rm);  m_swap = (M == '1');
            if dLo == 15 || dHi == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            if dHi == dLo then UNPREDICTABLE;

    __encoding aarch32_SMLSLD_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field RdLo 12 +: 4
        __field RdHi 8 +: 4
        __field M 4 +: 1
        __field Rm 0 +: 4
        __opcode '11111011 1101xxxx xxxxxxxx 110xxxxx'
        __guard TRUE
        __decode
            dLo = UInt(RdLo);  dHi = UInt(RdHi);  n = UInt(Rn);  m = UInt(Rm);  m_swap = (M == '1');
            if dLo == 15 || dHi == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            // Armv8-A removes UPREDICTABLE for R13
            if dHi == dLo then UNPREDICTABLE;

    __execute __conditional
        operand2 = if m_swap then ROR(R[m],16) else R[m];
        product1 = SInt(R[n]<15:0>) * SInt(operand2<15:0>);
        product2 = SInt(R[n]<31:16>) * SInt(operand2<31:16>);
        result = product1 - product2 + SInt(R[dHi]:R[dLo]);
        R[dHi] = result<63:32>;
        R[dLo] = result<31:0>;
