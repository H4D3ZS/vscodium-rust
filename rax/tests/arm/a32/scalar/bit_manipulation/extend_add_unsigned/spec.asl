__instruction aarch32_UXTAB_A
    __encoding aarch32_UXTAB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field rotate 10 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0110 1110xxxx xxxxxxxx 0111xxxx'
        __guard cond != '1111'
        __decode
            if Rn == '1111' then SEE "UXTB";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  rotation = UInt(rotate:'000');
            if d == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_UXTAB_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field rotate 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111010 0101xxxx 1111xxxx 1xxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "UXTB";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  rotation = UInt(rotate:'000');
            if d == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        rotated = ROR(R[m], rotation);
        R[d] = R[n] + ZeroExtend(rotated<7:0>, 32);

__instruction aarch32_UXTAH_A
    __encoding aarch32_UXTAH_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field rotate 10 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0110 1111xxxx xxxxxxxx 0111xxxx'
        __guard cond != '1111'
        __decode
            if Rn == '1111' then SEE "UXTH";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  rotation = UInt(rotate:'000');
            if d == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_UXTAH_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field rotate 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111010 0001xxxx 1111xxxx 1xxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "UXTH";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  rotation = UInt(rotate:'000');
            if d == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        rotated = ROR(R[m], rotation);
        R[d] = R[n] + ZeroExtend(rotated<15:0>, 32);

__instruction aarch32_UXTAB16_A
    __encoding aarch32_UXTAB16_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field rotate 10 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0110 1100xxxx xxxxxxxx 0111xxxx'
        __guard cond != '1111'
        __decode
            if Rn == '1111' then SEE "UXTB16";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  rotation = UInt(rotate:'000');
            if d == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_UXTAB16_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field rotate 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111010 0011xxxx 1111xxxx 1xxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "UXTB16";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  rotation = UInt(rotate:'000');
            if d == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        rotated = ROR(R[m], rotation);
        R[d]<15:0>  = R[n]<15:0> + ZeroExtend(rotated<7:0>, 16);
        R[d]<31:16> = R[n]<31:16> + ZeroExtend(rotated<23:16>, 16);
