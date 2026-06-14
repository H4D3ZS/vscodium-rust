__instruction aarch32_USADA8_A
    __encoding aarch32_USADA8_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 16 +: 4
        __field Ra 12 +: 4
        __field Rm 8 +: 4
        __field Rn 0 +: 4
        __opcode 'xxxx0111 1000xxxx xxxxxxxx 0001xxxx'
        __guard cond != '1111'
        __decode
            if Ra == '1111' then SEE "USAD8";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_USADA8_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Ra 12 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111011 0111xxxx xxxxxxxx 0000xxxx'
        __guard TRUE
        __decode
            if Ra == '1111' then SEE "USAD8";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        absdiff1 = Abs(UInt(R[n]<7:0>)   - UInt(R[m]<7:0>));
        absdiff2 = Abs(UInt(R[n]<15:8>)  - UInt(R[m]<15:8>));
        absdiff3 = Abs(UInt(R[n]<23:16>) - UInt(R[m]<23:16>));
        absdiff4 = Abs(UInt(R[n]<31:24>) - UInt(R[m]<31:24>));
        result = UInt(R[a]) + absdiff1 + absdiff2 + absdiff3 + absdiff4;
        R[d] = result<31:0>;
