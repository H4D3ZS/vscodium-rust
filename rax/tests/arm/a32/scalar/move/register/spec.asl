__instruction aarch32_MOV_r_A
    __encoding aarch32_MOV_r_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rd 12 +: 4
        __field imm5 7 +: 5
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0001 101xxxxx xxxxxxxx xxx0xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  m = UInt(Rm);  setflags = (S == '1');
            (shift_t, shift_n) = DecodeImmShift(type1, imm5);

    __encoding aarch32_MOV_r_T1_A
        __instruction_set T16
        __field D 23 +: 1
        __field Rm 19 +: 4
        __field Rd 16 +: 3
        __opcode '01000110 xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(D:Rd);  m = UInt(Rm);  setflags = FALSE;
            (shift_t, shift_n) = (SRType_LSL, 0);
            if d == 15 && InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __encoding aarch32_MOV_r_T2_A
        __instruction_set T16
        __field op 27 +: 2
        __field imm5 22 +: 5
        __field Rm 19 +: 3
        __field Rd 16 +: 3
        __opcode '000xxxxx xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rd);  m = UInt(Rm);  setflags = !InITBlock();
            (shift_t, shift_n) = DecodeImmShift(op, imm5);
            if op == '00' && imm5 == '00000' && InITBlock() then UNPREDICTABLE;

    __encoding aarch32_MOV_r_T3_A
        __instruction_set T32
        __field S 20 +: 1
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm2 6 +: 2
        __field type1 4 +: 2
        __field Rm 0 +: 4
        __opcode '11101010 010x1111 xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  m = UInt(Rm);  setflags = (S == '1');
            (shift_t, shift_n) = DecodeImmShift(type1, imm3:imm2);
            if d == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        (shifted, carry) = Shift_C(R[m], shift_t, shift_n, PSTATE.C);
        result = shifted;
        if d == 15 then
            if setflags then
                ALUExceptionReturn(result);
            else
                ALUWritePC(result);
        else
            R[d] = result;
            if setflags then
                PSTATE.N = result<31>;
                PSTATE.Z = IsZeroBit(result);
                PSTATE.C = carry;
                // PSTATE.V unchanged
