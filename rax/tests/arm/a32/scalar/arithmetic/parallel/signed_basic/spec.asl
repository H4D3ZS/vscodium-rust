__instruction aarch32_SADD8_A
    __encoding aarch32_SADD8_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 0001xxxx xxxxxxxx 1001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_SADD8_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1000xxxx 1111xxxx 0000xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        sum1 = SInt(R[n]<7:0>) + SInt(R[m]<7:0>);
        sum2 = SInt(R[n]<15:8>) + SInt(R[m]<15:8>);
        sum3 = SInt(R[n]<23:16>) + SInt(R[m]<23:16>);
        sum4 = SInt(R[n]<31:24>) + SInt(R[m]<31:24>);
        R[d]<7:0>   = sum1<7:0>;
        R[d]<15:8>  = sum2<7:0>;
        R[d]<23:16> = sum3<7:0>;
        R[d]<31:24> = sum4<7:0>;
        PSTATE.GE<0>  = if sum1 >= 0 then '1' else '0';
        PSTATE.GE<1>  = if sum2 >= 0 then '1' else '0';
        PSTATE.GE<2>  = if sum3 >= 0 then '1' else '0';
        PSTATE.GE<3>  = if sum4 >= 0 then '1' else '0';

__instruction aarch32_SADD16_A
    __encoding aarch32_SADD16_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 0001xxxx xxxxxxxx 0001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_SADD16_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1001xxxx 1111xxxx 0000xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        sum1 = SInt(R[n]<15:0>) + SInt(R[m]<15:0>);
        sum2 = SInt(R[n]<31:16>) + SInt(R[m]<31:16>);
        R[d]<15:0>  = sum1<15:0>;
        R[d]<31:16> = sum2<15:0>;
        PSTATE.GE<1:0> = if sum1 >= 0 then '11' else '00';
        PSTATE.GE<3:2> = if sum2 >= 0 then '11' else '00';

__instruction aarch32_SSUB8_A
    __encoding aarch32_SSUB8_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 0001xxxx xxxxxxxx 1111xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_SSUB8_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1100xxxx 1111xxxx 0000xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        diff1 = SInt(R[n]<7:0>) - SInt(R[m]<7:0>);
        diff2 = SInt(R[n]<15:8>) - SInt(R[m]<15:8>);
        diff3 = SInt(R[n]<23:16>) - SInt(R[m]<23:16>);
        diff4 = SInt(R[n]<31:24>) - SInt(R[m]<31:24>);
        R[d]<7:0>   = diff1<7:0>;
        R[d]<15:8>  = diff2<7:0>;
        R[d]<23:16> = diff3<7:0>;
        R[d]<31:24> = diff4<7:0>;
        PSTATE.GE<0>  = if diff1 >= 0 then '1' else '0';
        PSTATE.GE<1>  = if diff2 >= 0 then '1' else '0';
        PSTATE.GE<2>  = if diff3 >= 0 then '1' else '0';
        PSTATE.GE<3>  = if diff4 >= 0 then '1' else '0';

__instruction aarch32_SSUB16_A
    __encoding aarch32_SSUB16_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 0001xxxx xxxxxxxx 0111xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_SSUB16_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1101xxxx 1111xxxx 0000xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        diff1 = SInt(R[n]<15:0>) - SInt(R[m]<15:0>);
        diff2 = SInt(R[n]<31:16>) - SInt(R[m]<31:16>);
        R[d]<15:0>  = diff1<15:0>;
        R[d]<31:16> = diff2<15:0>;
        PSTATE.GE<1:0> = if diff1 >= 0 then '11' else '00';
        PSTATE.GE<3:2> = if diff2 >= 0 then '11' else '00';
