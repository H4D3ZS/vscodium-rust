__instruction aarch32_UHADD8_A
    __encoding aarch32_UHADD8_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 0111xxxx xxxxxxxx 1001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_UHADD8_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1000xxxx 1111xxxx 0110xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        sum1 = UInt(R[n]<7:0>) + UInt(R[m]<7:0>);
        sum2 = UInt(R[n]<15:8>) + UInt(R[m]<15:8>);
        sum3 = UInt(R[n]<23:16>) + UInt(R[m]<23:16>);
        sum4 = UInt(R[n]<31:24>) + UInt(R[m]<31:24>);
        R[d]<7:0>   = sum1<8:1>;
        R[d]<15:8>  = sum2<8:1>;
        R[d]<23:16> = sum3<8:1>;
        R[d]<31:24> = sum4<8:1>;

__instruction aarch32_UHADD16_A
    __encoding aarch32_UHADD16_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 0111xxxx xxxxxxxx 0001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_UHADD16_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1001xxxx 1111xxxx 0110xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        sum1 = UInt(R[n]<15:0>) + UInt(R[m]<15:0>);
        sum2 = UInt(R[n]<31:16>) + UInt(R[m]<31:16>);
        R[d]<15:0>  = sum1<16:1>;
        R[d]<31:16> = sum2<16:1>;

__instruction aarch32_UHSUB8_A
    __encoding aarch32_UHSUB8_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 0111xxxx xxxxxxxx 1111xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_UHSUB8_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1100xxxx 1111xxxx 0110xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        diff1 = UInt(R[n]<7:0>) - UInt(R[m]<7:0>);
        diff2 = UInt(R[n]<15:8>) - UInt(R[m]<15:8>);
        diff3 = UInt(R[n]<23:16>) - UInt(R[m]<23:16>);
        diff4 = UInt(R[n]<31:24>) - UInt(R[m]<31:24>);
        R[d]<7:0>   = diff1<8:1>;
        R[d]<15:8>  = diff2<8:1>;
        R[d]<23:16> = diff3<8:1>;
        R[d]<31:24> = diff4<8:1>;

__instruction aarch32_UHSUB16_A
    __encoding aarch32_UHSUB16_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0110 0111xxxx xxxxxxxx 0111xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_UHSUB16_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111010 1101xxxx 1111xxxx 0110xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        diff1 = UInt(R[n]<15:0>) - UInt(R[m]<15:0>);
        diff2 = UInt(R[n]<31:16>) - UInt(R[m]<31:16>);
        R[d]<15:0>  = diff1<16:1>;
        R[d]<31:16> = diff2<16:1>;
