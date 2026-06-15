__instruction aarch32_SMUAD_A
    __encoding aarch32_SMUAD_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 16 +: 4
        __field Rm 8 +: 4
        __field M 5 +: 1
        __field Rn 0 +: 4
        __opcode 'xxxx0111 0000xxxx 1111xxxx 00x1xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  m_swap = (M == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_SMUAD_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field M 4 +: 1
        __field Rm 0 +: 4
        __opcode '11111011 0010xxxx 1111xxxx 000xxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  m_swap = (M == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        operand2 = if m_swap then ROR(R[m],16) else R[m];
        product1 = SInt(R[n]<15:0>) * SInt(operand2<15:0>);
        product2 = SInt(R[n]<31:16>) * SInt(operand2<31:16>);
        result = product1 + product2;
        R[d] = result<31:0>;
        if result != SInt(result<31:0>) then  // Signed overflow
            PSTATE.Q = '1';

__instruction aarch32_SMUSD_A
    __encoding aarch32_SMUSD_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 16 +: 4
        __field Rm 8 +: 4
        __field M 5 +: 1
        __field Rn 0 +: 4
        __opcode 'xxxx0111 0000xxxx 1111xxxx 01x1xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  m_swap = (M == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_SMUSD_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field M 4 +: 1
        __field Rm 0 +: 4
        __opcode '11111011 0100xxxx 1111xxxx 000xxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  m_swap = (M == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        operand2 = if m_swap then ROR(R[m],16) else R[m];
        product1 = SInt(R[n]<15:0>) * SInt(operand2<15:0>);
        product2 = SInt(R[n]<31:16>) * SInt(operand2<31:16>);
        result = product1 - product2;
        R[d] = result<31:0>;
        // Signed overflow cannot occur

__instruction aarch32_SMLAD_A
    __encoding aarch32_SMLAD_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 16 +: 4
        __field Ra 12 +: 4
        __field Rm 8 +: 4
        __field M 5 +: 1
        __field Rn 0 +: 4
        __opcode 'xxxx0111 0000xxxx xxxxxxxx 00x1xxxx'
        __guard cond != '1111'
        __decode
            if Ra == '1111' then SEE "SMUAD";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);
            m_swap = (M == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_SMLAD_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Ra 12 +: 4
        __field Rd 8 +: 4
        __field M 4 +: 1
        __field Rm 0 +: 4
        __opcode '11111011 0010xxxx xxxxxxxx 000xxxxx'
        __guard TRUE
        __decode
            if Ra == '1111' then SEE "SMUAD";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);
            m_swap = (M == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        operand2 = if m_swap then ROR(R[m],16) else R[m];
        product1 = SInt(R[n]<15:0>) * SInt(operand2<15:0>);
        product2 = SInt(R[n]<31:16>) * SInt(operand2<31:16>);
        result = product1 + product2 + SInt(R[a]);
        R[d] = result<31:0>;
        if result != SInt(result<31:0>) then  // Signed overflow
            PSTATE.Q = '1';

__instruction aarch32_SMLSD_A
    __encoding aarch32_SMLSD_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 16 +: 4
        __field Ra 12 +: 4
        __field Rm 8 +: 4
        __field M 5 +: 1
        __field Rn 0 +: 4
        __opcode 'xxxx0111 0000xxxx xxxxxxxx 01x1xxxx'
        __guard cond != '1111'
        __decode
            if Ra == '1111' then SEE "SMUSD";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);  m_swap = (M == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_SMLSD_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Ra 12 +: 4
        __field Rd 8 +: 4
        __field M 4 +: 1
        __field Rm 0 +: 4
        __opcode '11111011 0100xxxx xxxxxxxx 000xxxxx'
        __guard TRUE
        __decode
            if Ra == '1111' then SEE "SMUSD";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);  m_swap = (M == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        operand2 = if m_swap then ROR(R[m],16) else R[m];
        product1 = SInt(R[n]<15:0>) * SInt(operand2<15:0>);
        product2 = SInt(R[n]<31:16>) * SInt(operand2<31:16>);
        result = product1 - product2 + SInt(R[a]);
        R[d] = result<31:0>;
        if result != SInt(result<31:0>) then  // Signed overflow
            PSTATE.Q = '1';
