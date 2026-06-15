__instruction aarch32_QASX_A
    __encoding aarch32_QASX_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 0010xxxx xxxxxxxx 0011xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_QASX_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1010xxxx 1111xxxx 0001xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        diff = SInt(R[n]<15:0>) - SInt(R[m]<31:16>);
        sum  = SInt(R[n]<31:16>) + SInt(R[m]<15:0>);
        R[d]<15:0>  = SignedSat(diff, 16);
        R[d]<31:16> = SignedSat(sum, 16);

__instruction aarch32_QSAX_A
    __encoding aarch32_QSAX_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 0010xxxx xxxxxxxx 0101xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_QSAX_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1110xxxx 1111xxxx 0001xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        sum  = SInt(R[n]<15:0>) + SInt(R[m]<31:16>);
        diff = SInt(R[n]<31:16>) - SInt(R[m]<15:0>);
        R[d]<15:0>  = SignedSat(sum, 16);
        R[d]<31:16> = SignedSat(diff, 16);
