__instruction aarch32_UQASX_A
    __encoding aarch32_UQASX_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 0110xxxx xxxxxxxx 0011xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_UQASX_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1010xxxx 1111xxxx 0101xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        diff = UInt(R[n]<15:0>) - UInt(R[m]<31:16>);
        sum  = UInt(R[n]<31:16>) + UInt(R[m]<15:0>);
        R[d]<15:0>  = UnsignedSat(diff, 16);
        R[d]<31:16> = UnsignedSat(sum, 16);

__instruction aarch32_UQSAX_A
    __encoding aarch32_UQSAX_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 0110xxxx xxxxxxxx 0101xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_UQSAX_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1110xxxx 1111xxxx 0101xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        sum  = UInt(R[n]<15:0>) + UInt(R[m]<31:16>);
        diff = UInt(R[n]<31:16>) - UInt(R[m]<15:0>);
        R[d]<15:0>  = UnsignedSat(sum, 16);
        R[d]<31:16> = UnsignedSat(diff, 16);
