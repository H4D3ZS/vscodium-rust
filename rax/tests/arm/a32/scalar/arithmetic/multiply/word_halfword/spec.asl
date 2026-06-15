__instruction aarch32_SMULBB_A
    __encoding aarch32_SMULBB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 16 +: 4
        __field Rm 8 +: 4
        __field M 6 +: 1
        __field N 5 +: 1
        __field Rn 0 +: 4
        __opcode 'xxxx0001 0110xxxx xxxxxxxx 1xx0xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            n_high = (N == '1');  m_high = (M == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_SMULBB_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field N 5 +: 1
        __field M 4 +: 1
        __field Rm 0 +: 4
        __opcode '11111011 0001xxxx 1111xxxx 00xxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            n_high = (N == '1');  m_high = (M == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        operand1 = if n_high then R[n]<31:16> else R[n]<15:0>;
        operand2 = if m_high then R[m]<31:16> else R[m]<15:0>;
        result = SInt(operand1) * SInt(operand2);
        R[d] = result<31:0>;
        // Signed overflow cannot occur

__instruction aarch32_SMLABB_A
    __encoding aarch32_SMLABB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 16 +: 4
        __field Ra 12 +: 4
        __field Rm 8 +: 4
        __field M 6 +: 1
        __field N 5 +: 1
        __field Rn 0 +: 4
        __opcode 'xxxx0001 0000xxxx xxxxxxxx 1xx0xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);
            n_high = (N == '1');  m_high = (M == '1');
            if d == 15 || n == 15 || m == 15 || a == 15 then UNPREDICTABLE;

    __encoding aarch32_SMLABB_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Ra 12 +: 4
        __field Rd 8 +: 4
        __field N 5 +: 1
        __field M 4 +: 1
        __field Rm 0 +: 4
        __opcode '11111011 0001xxxx xxxxxxxx 00xxxxxx'
        __guard TRUE
        __decode
            if Ra == '1111' then SEE "SMULBB, SMULBT, SMULTB, SMULTT";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);
            n_high = (N == '1');  m_high = (M == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        operand1 = if n_high then R[n]<31:16> else R[n]<15:0>;
        operand2 = if m_high then R[m]<31:16> else R[m]<15:0>;
        result = SInt(operand1) * SInt(operand2) + SInt(R[a]);
        R[d] = result<31:0>;
        if result != SInt(result<31:0>) then  // Signed overflow
            PSTATE.Q = '1';

__instruction aarch32_SMULWB_A
    __encoding aarch32_SMULWB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 16 +: 4
        __field Rm 8 +: 4
        __field M 6 +: 1
        __field Rn 0 +: 4
        __opcode 'xxxx0001 0010xxxx xxxxxxxx 1x10xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  m_high = (M == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_SMULWB_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field M 4 +: 1
        __field Rm 0 +: 4
        __opcode '11111011 0011xxxx 1111xxxx 000xxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  m_high = (M == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        operand2 = if m_high then R[m]<31:16> else R[m]<15:0>;
        product = SInt(R[n]) * SInt(operand2);
        R[d] = product<47:16>;
        // Signed overflow cannot occur

__instruction aarch32_SMLAWB_A
    __encoding aarch32_SMLAWB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 16 +: 4
        __field Ra 12 +: 4
        __field Rm 8 +: 4
        __field M 6 +: 1
        __field Rn 0 +: 4
        __opcode 'xxxx0001 0010xxxx xxxxxxxx 1x00xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);  m_high = (M == '1');
            if d == 15 || n == 15 || m == 15 || a == 15 then UNPREDICTABLE;

    __encoding aarch32_SMLAWB_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Ra 12 +: 4
        __field Rd 8 +: 4
        __field M 4 +: 1
        __field Rm 0 +: 4
        __opcode '11111011 0011xxxx xxxxxxxx 000xxxxx'
        __guard TRUE
        __decode
            if Ra == '1111' then SEE "SMULWB, SMULWT";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);  m_high = (M == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        operand2 = if m_high then R[m]<31:16> else R[m]<15:0>;
        result = SInt(R[n]) * SInt(operand2) + (SInt(R[a]) << 16);
        R[d] = result<47:16>;
        if (result >> 16) != SInt(R[d]) then  // Signed overflow
            PSTATE.Q = '1';
