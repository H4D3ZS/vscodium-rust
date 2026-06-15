__instruction aarch32_QDADD_A
    __encoding aarch32_QDADD_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0001 0100xxxx xxxxxxxx 0101xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_QDADD_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1000xxxx 1111xxxx 1001xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        (doubled, sat1) = SignedSatQ(2 * SInt(R[n]), 32);
        (R[d], sat2)  = SignedSatQ(SInt(R[m]) + SInt(doubled), 32);
        if sat1 || sat2 then
            PSTATE.Q = '1';

__instruction aarch32_QDSUB_A
    __encoding aarch32_QDSUB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0001 0110xxxx xxxxxxxx 0101xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_QDSUB_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1000xxxx 1111xxxx 1011xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        (doubled, sat1) = SignedSatQ(2 * SInt(R[n]), 32);
        (R[d], sat2)  = SignedSatQ(SInt(R[m]) - SInt(doubled), 32);
        if sat1 || sat2 then
            PSTATE.Q = '1';
