__instruction aarch32_CLZ_A
    __encoding aarch32_CLZ_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0001 0110xxxx xxxxxxxx 0001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  m = UInt(Rm);
            if d == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_CLZ_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1011xxxx 1111xxxx 1000xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  m = UInt(Rm);  n = UInt(Rn);
            if m != n || d == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        result = CountLeadingZeroBits(R[m]);
        R[d] = result<31:0>;

__instruction aarch32_RBIT_A
    __encoding aarch32_RBIT_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 1111xxxx xxxxxxxx 0011xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  m = UInt(Rm);
            if d == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_RBIT_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1001xxxx 1111xxxx 1010xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  m = UInt(Rm);  n = UInt(Rn);
            if m != n || d == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        bits(32) result;
        for i = 0 to 31
            result<31-i> = R[m]<i>;
        R[d] = result;
