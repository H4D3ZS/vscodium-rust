__instruction aarch32_RSC_r_A
    __encoding aarch32_RSC_r_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field imm5 7 +: 5
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0000 111xxxxx xxxxxxxx xxx0xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  setflags = (S == '1');
            (shift_t, shift_n) = DecodeImmShift(type1, imm5);

    __execute __conditional
        shifted = Shift(R[m], shift_t, shift_n, PSTATE.C);
        (result, nzcv) = AddWithCarry(NOT(R[n]), shifted, PSTATE.C);
        if d == 15 then
            if setflags then
                ALUExceptionReturn(result);
            else
                ALUWritePC(result);
        else
            R[d] = result;
            if setflags then
                PSTATE.<N,Z,C,V> = nzcv;

__instruction aarch32_SBC_r_A
    __encoding aarch32_SBC_r_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field imm5 7 +: 5
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0000 110xxxxx xxxxxxxx xxx0xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  setflags = (S == '1');
            (shift_t, shift_n) = DecodeImmShift(type1, imm5);

    __encoding aarch32_SBC_r_T1_A
        __instruction_set T16
        __field Rm 19 +: 3
        __field Rdn 16 +: 3
        __opcode '01000001 10xxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rdn);  n = UInt(Rdn);  m = UInt(Rm);  setflags = !InITBlock();
            (shift_t, shift_n) = (SRType_LSL, 0);

    __encoding aarch32_SBC_r_T2_A
        __instruction_set T32
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm2 6 +: 2
        __field type1 4 +: 2
        __field Rm 0 +: 4
        __opcode '11101011 011xxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  setflags = (S == '1');
            (shift_t, shift_n) = DecodeImmShift(type1, imm3:imm2);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        shifted = Shift(R[m], shift_t, shift_n, PSTATE.C);
        (result, nzcv) = AddWithCarry(R[n], NOT(shifted), PSTATE.C);
        if d == 15 then          // Can only occur for A32 encoding
            if setflags then
                ALUExceptionReturn(result);
            else
                ALUWritePC(result);
        else
            R[d] = result;
            if setflags then
                PSTATE.<N,Z,C,V> = nzcv;
