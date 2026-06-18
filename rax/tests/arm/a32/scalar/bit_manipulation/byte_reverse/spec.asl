__instruction aarch32_REV_A
    __encoding aarch32_REV_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 1011xxxx xxxxxxxx 0011xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  m = UInt(Rm);
            if d == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_REV_T1_A
        __instruction_set T16
        __field Rm 19 +: 3
        __field Rd 16 +: 3
        __opcode '10111010 00xxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rd);  m = UInt(Rm);

    __encoding aarch32_REV_T2_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1001xxxx 1111xxxx 1000xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  m = UInt(Rm);  n = UInt(Rn);
            if m != n || d == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        bits(32) result;
        result<31:24> = R[m]<7:0>;
        result<23:16> = R[m]<15:8>;
        result<15:8>  = R[m]<23:16>;
        result<7:0>   = R[m]<31:24>;
        R[d] = result;

__instruction aarch32_REV16_A
    __encoding aarch32_REV16_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 1011xxxx xxxxxxxx 1011xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  m = UInt(Rm);
            if d == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_REV16_T1_A
        __instruction_set T16
        __field Rm 19 +: 3
        __field Rd 16 +: 3
        __opcode '10111010 01xxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rd);  m = UInt(Rm);

    __encoding aarch32_REV16_T2_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1001xxxx 1111xxxx 1001xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  m = UInt(Rm);  n = UInt(Rn);
            if m != n || d == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        bits(32) result;
        result<31:24> = R[m]<23:16>;
        result<23:16> = R[m]<31:24>;
        result<15:8>  = R[m]<7:0>;
        result<7:0>   = R[m]<15:8>;
        R[d] = result;

__instruction aarch32_REVSH_A
    __encoding aarch32_REVSH_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 1111xxxx xxxxxxxx 1011xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  m = UInt(Rm);
            if d == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_REVSH_T1_A
        __instruction_set T16
        __field Rm 19 +: 3
        __field Rd 16 +: 3
        __opcode '10111010 11xxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rd);  m = UInt(Rm);

    __encoding aarch32_REVSH_T2_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1001xxxx 1111xxxx 1011xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  m = UInt(Rm);  n = UInt(Rn);
            if m != n || d == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        bits(32) result;
        result<31:8>  = SignExtend(R[m]<7:0>, 24);
        result<7:0>   = R[m]<15:8>;
        R[d] = result;
