__instruction aarch32_UADD8_A
    __encoding aarch32_UADD8_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 0101xxxx xxxxxxxx 1001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_UADD8_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1000xxxx 1111xxxx 0100xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m  == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        sum1 = UInt(R[n]<7:0>) + UInt(R[m]<7:0>);
        sum2 = UInt(R[n]<15:8>) + UInt(R[m]<15:8>);
        sum3 = UInt(R[n]<23:16>) + UInt(R[m]<23:16>);
        sum4 = UInt(R[n]<31:24>) + UInt(R[m]<31:24>);
        R[d]<7:0>   = sum1<7:0>;
        R[d]<15:8>  = sum2<7:0>;
        R[d]<23:16> = sum3<7:0>;
        R[d]<31:24> = sum4<7:0>;
        PSTATE.GE<0>  = if sum1 >= 0x100 then '1' else '0';
        PSTATE.GE<1>  = if sum2 >= 0x100 then '1' else '0';
        PSTATE.GE<2>  = if sum3 >= 0x100 then '1' else '0';
        PSTATE.GE<3>  = if sum4 >= 0x100 then '1' else '0';

__instruction aarch32_UADD16_A
    __encoding aarch32_UADD16_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 0101xxxx xxxxxxxx 0001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_UADD16_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1001xxxx 1111xxxx 0100xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        sum1 = UInt(R[n]<15:0>) + UInt(R[m]<15:0>);
        sum2 = UInt(R[n]<31:16>) + UInt(R[m]<31:16>);
        R[d]<15:0>  = sum1<15:0>;
        R[d]<31:16> = sum2<15:0>;
        PSTATE.GE<1:0> = if sum1 >= 0x10000 then '11' else '00';
        PSTATE.GE<3:2> = if sum2 >= 0x10000 then '11' else '00';

__instruction aarch32_USAX_A
    __encoding aarch32_USAX_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 0101xxxx xxxxxxxx 0101xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_USAX_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1110xxxx 1111xxxx 0100xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        sum  = UInt(R[n]<15:0>) + UInt(R[m]<31:16>);
        diff = UInt(R[n]<31:16>) - UInt(R[m]<15:0>);
        R[d]<15:0>  = sum<15:0>;
        R[d]<31:16> = diff<15:0>;
        PSTATE.GE<1:0> = if sum  >= 0x10000 then '11' else '00';
        PSTATE.GE<3:2> = if diff >= 0 then '11' else '00';

__instruction aarch32_UASX_A
    __encoding aarch32_UASX_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 0101xxxx xxxxxxxx 0011xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_UASX_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1010xxxx 1111xxxx 0100xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        diff = UInt(R[n]<15:0>) - UInt(R[m]<31:16>);
        sum  = UInt(R[n]<31:16>) + UInt(R[m]<15:0>);
        R[d]<15:0>  = diff<15:0>;
        R[d]<31:16> = sum<15:0>;
        PSTATE.GE<1:0> = if diff >= 0 then '11' else '00';
        PSTATE.GE<3:2> = if sum  >= 0x10000 then '11' else '00';
