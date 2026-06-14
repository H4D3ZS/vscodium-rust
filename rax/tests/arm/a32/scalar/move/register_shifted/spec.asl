__instruction aarch32_MOV_rr_A
    __encoding aarch32_MOV_rr_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rd 12 +: 4
        __field Rs 8 +: 4
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0001 101xxxxx xxxxxxxx 0xx1xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  m = UInt(Rm);  s = UInt(Rs);
            setflags = (S == '1');  shift_t = DecodeRegShift(type1);
            if d == 15 || m == 15 || s == 15 then UNPREDICTABLE;

    __encoding aarch32_MOV_rr_T1_A
        __instruction_set T16
        __field op 22 +: 4
        __field Rs 19 +: 3
        __field Rdm 16 +: 3
        __opcode '0100000x xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            if !(op IN {'0010', '0011', '0100', '0111'}) then SEE "Related encodings";
            d = UInt(Rdm);  m = UInt(Rdm);  s = UInt(Rs);
            setflags = !InITBlock();  shift_t = DecodeRegShift(op<2>:op<0>);

    __encoding aarch32_MOV_rr_T2_A
        __instruction_set T32
        __field type1 21 +: 2
        __field S 20 +: 1
        __field Rm 16 +: 4
        __field Rd 8 +: 4
        __field Rs 0 +: 4
        __opcode '11111010 0xxxxxxx 1111xxxx 0000xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  m = UInt(Rm);  s = UInt(Rs);
            setflags = (S == '1');  shift_t = DecodeRegShift(type1);
            if d == 15 || m == 15 || s == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        shift_n = UInt(R[s]<7:0>);
        (result, carry) = Shift_C(R[m], shift_t, shift_n, PSTATE.C);
        R[d] = result;
        if setflags then
            PSTATE.N = result<31>;
            PSTATE.Z = IsZeroBit(result);
            PSTATE.C = carry;
            // PSTATE.V unchanged
