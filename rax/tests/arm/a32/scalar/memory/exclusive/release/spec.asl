__instruction aarch32_STL_A
    __encoding aarch32_STL_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rt 0 +: 4
        __opcode 'xxxx0001 1000xxxx xxxxxx00 1001xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt); n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_STL_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode '11101000 1100xxxx xxxxxxxx 1010xxxx'
        __guard TRUE
        __decode
            t = UInt(Rt); n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        MemO[address, 4] = R[t];

__instruction aarch32_STLB_A
    __encoding aarch32_STLB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rt 0 +: 4
        __opcode 'xxxx0001 1100xxxx xxxxxx00 1001xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt); n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_STLB_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode '11101000 1100xxxx xxxxxxxx 1000xxxx'
        __guard TRUE
        __decode
            t = UInt(Rt); n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        MemO[address, 1] = R[t]<7:0>;

__instruction aarch32_STLH_A
    __encoding aarch32_STLH_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rt 0 +: 4
        __opcode 'xxxx0001 1110xxxx xxxxxx00 1001xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt); n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_STLH_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode '11101000 1100xxxx xxxxxxxx 1001xxxx'
        __guard TRUE
        __decode
            t = UInt(Rt); n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        MemO[address, 2] = R[t]<15:0>;
