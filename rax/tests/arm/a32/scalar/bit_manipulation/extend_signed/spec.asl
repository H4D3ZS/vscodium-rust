__instruction aarch32_SXTB_A
    __encoding aarch32_SXTB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 12 +: 4
        __field rotate 10 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0110 10101111 xxxxxxxx 0111xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  m = UInt(Rm);  rotation = UInt(rotate:'000');
            if d == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_SXTB_T1_A
        __instruction_set T16
        __field Rm 19 +: 3
        __field Rd 16 +: 3
        __opcode '10110010 01xxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rd);  m = UInt(Rm);  rotation = 0;

    __encoding aarch32_SXTB_T2_A
        __instruction_set T32
        __field Rd 8 +: 4
        __field rotate 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111010 01001111 1111xxxx 1xxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  m = UInt(Rm);  rotation = UInt(rotate:'000');
            if d == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        rotated = ROR(R[m], rotation);
        R[d] = SignExtend(rotated<7:0>, 32);

__instruction aarch32_SXTH_A
    __encoding aarch32_SXTH_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 12 +: 4
        __field rotate 10 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0110 10111111 xxxxxxxx 0111xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  m = UInt(Rm);  rotation = UInt(rotate:'000');
            if d == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_SXTH_T1_A
        __instruction_set T16
        __field Rm 19 +: 3
        __field Rd 16 +: 3
        __opcode '10110010 00xxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rd);  m = UInt(Rm);  rotation = 0;

    __encoding aarch32_SXTH_T2_A
        __instruction_set T32
        __field Rd 8 +: 4
        __field rotate 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111010 00001111 1111xxxx 1xxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  m = UInt(Rm);  rotation = UInt(rotate:'000');
            if d == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        rotated = ROR(R[m], rotation);
        R[d] = SignExtend(rotated<15:0>, 32);

__instruction aarch32_SXTB16_A
    __encoding aarch32_SXTB16_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 12 +: 4
        __field rotate 10 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0110 10001111 xxxxxxxx 0111xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  m = UInt(Rm);  rotation = UInt(rotate:'000');
            if d == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_SXTB16_T1_A
        __instruction_set T32
        __field Rd 8 +: 4
        __field rotate 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111010 00101111 1111xxxx 1xxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  m = UInt(Rm);  rotation = UInt(rotate:'000');
            if d == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        rotated = ROR(R[m], rotation);
        R[d]<15:0>  = SignExtend(rotated<7:0>, 16);
        R[d]<31:16> = SignExtend(rotated<23:16>, 16);
