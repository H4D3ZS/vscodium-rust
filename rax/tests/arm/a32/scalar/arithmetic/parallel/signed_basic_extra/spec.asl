__instruction aarch32_SASX_A
    __encoding aarch32_SASX_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 0001xxxx xxxxxxxx 0011xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_SASX_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1010xxxx 1111xxxx 0000xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        diff = SInt(R[n]<15:0>) - SInt(R[m]<31:16>);
        sum  = SInt(R[n]<31:16>) + SInt(R[m]<15:0>);
        R[d]<15:0>  = diff<15:0>;
        R[d]<31:16> = sum<15:0>;
        PSTATE.GE<1:0> = if diff >= 0 then '11' else '00';
        PSTATE.GE<3:2> = if sum  >= 0 then '11' else '00';
