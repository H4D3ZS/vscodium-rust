__instruction aarch32_UDIV_A
    __encoding aarch32_UDIV_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 16 +: 4
        __field Ra 12 +: 4
        __field Rm 8 +: 4
        __field Rn 0 +: 4
        __opcode 'xxxx0111 0011xxxx xxxxxxxx 0001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);
            if d == 15 || n == 15 || m == 15 || a != 15 then UNPREDICTABLE;

    __encoding aarch32_UDIV_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Ra 12 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111011 1011xxxx xxxxxxxx 1111xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);
            if d == 15 || n == 15 || m == 15 || a != 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        if UInt(R[m]) == 0 then
            result = 0;
        else
            result = RoundTowardsZero(Real(UInt(R[n])) / Real(UInt(R[m])));
        R[d] = result<31:0>;

__instruction aarch32_SDIV_A
    __encoding aarch32_SDIV_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 16 +: 4
        __field Ra 12 +: 4
        __field Rm 8 +: 4
        __field Rn 0 +: 4
        __opcode 'xxxx0111 0001xxxx xxxxxxxx 0001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);
            if d == 15 || n == 15 || m == 15 || a != 15 then UNPREDICTABLE;

    __encoding aarch32_SDIV_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Ra 12 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111011 1001xxxx xxxxxxxx 1111xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);
            if d == 15 || n == 15 || m == 15 || a != 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        if SInt(R[m]) == 0 then
            result = 0;
        else
            result = RoundTowardsZero(Real(SInt(R[n])) / Real(SInt(R[m])));
        R[d] = result<31:0>;
