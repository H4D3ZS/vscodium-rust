__instruction aarch32_RSC_rr_A
    __encoding aarch32_RSC_rr_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rs 8 +: 4
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0000 111xxxxx xxxxxxxx 0xx1xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  s = UInt(Rs);
            setflags = (S == '1');  shift_t = DecodeRegShift(type1);
            if d == 15 || n == 15 || m == 15 || s == 15 then UNPREDICTABLE;

    __execute __conditional
        shift_n = UInt(R[s]<7:0>);
        shifted = Shift(R[m], shift_t, shift_n, PSTATE.C);
        (result, nzcv) = AddWithCarry(NOT(R[n]), shifted, PSTATE.C);
        R[d] = result;
        if setflags then
            PSTATE.<N,Z,C,V> = nzcv;

__instruction aarch32_SBC_rr_A
    __encoding aarch32_SBC_rr_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rs 8 +: 4
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0000 110xxxxx xxxxxxxx 0xx1xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  s = UInt(Rs);
            setflags = (S == '1');  shift_t = DecodeRegShift(type1);
            if d == 15 || n == 15 || m == 15 || s == 15 then UNPREDICTABLE;

    __execute __conditional
        shift_n = UInt(R[s]<7:0>);
        shifted = Shift(R[m], shift_t, shift_n, PSTATE.C);
        (result, nzcv) = AddWithCarry(R[n], NOT(shifted), PSTATE.C);
        R[d] = result;
        if setflags then
            PSTATE.<N,Z,C,V> = nzcv;
