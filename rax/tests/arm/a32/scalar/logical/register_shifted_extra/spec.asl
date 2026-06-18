__instruction aarch32_MVN_rr_A
    __encoding aarch32_MVN_rr_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rd 12 +: 4
        __field Rs 8 +: 4
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0001 111xxxxx xxxxxxxx 0xx1xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  m = UInt(Rm);  s = UInt(Rs);
            setflags = (S == '1');  shift_t = DecodeRegShift(type1);
            if d == 15 || m == 15 || s == 15 then UNPREDICTABLE;

    __execute __conditional
        shift_n = UInt(R[s]<7:0>);
        (shifted, carry) = Shift_C(R[m], shift_t, shift_n, PSTATE.C);
        result = NOT(shifted);
        R[d] = result;
        if setflags then
            PSTATE.N = result<31>;
            PSTATE.Z = IsZeroBit(result);
            PSTATE.C = carry;
            // PSTATE.V unchanged
