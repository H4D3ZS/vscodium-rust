__instruction aarch32_LDA_A
    __encoding aarch32_LDA_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode 'xxxx0001 1001xxxx xxxxxx00 1001xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt); n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_LDA_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode '11101000 1101xxxx xxxxxxxx 1010xxxx'
        __guard TRUE
        __decode
            t = UInt(Rt); n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        R[t] = MemO[address, 4];

__instruction aarch32_LDAB_A
    __encoding aarch32_LDAB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode 'xxxx0001 1101xxxx xxxxxx00 1001xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt); n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_LDAB_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode '11101000 1101xxxx xxxxxxxx 1000xxxx'
        __guard TRUE
        __decode
            t = UInt(Rt); n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        R[t] = ZeroExtend(MemO[address, 1], 32);

__instruction aarch32_LDAH_A
    __encoding aarch32_LDAH_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode 'xxxx0001 1111xxxx xxxxxx00 1001xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_LDAH_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode '11101000 1101xxxx xxxxxxxx 1001xxxx'
        __guard TRUE
        __decode
            t = UInt(Rt);  n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        R[t] = ZeroExtend(MemO[address, 2], 32);
