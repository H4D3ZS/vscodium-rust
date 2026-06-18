__instruction aarch32_AND_r_A
    __encoding aarch32_AND_r_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field imm5 7 +: 5
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0000 000xxxxx xxxxxxxx xxx0xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  setflags = (S == '1');
            (shift_t, shift_n) = DecodeImmShift(type1, imm5);

    __encoding aarch32_AND_r_T1_A
        __instruction_set T16
        __field Rm 19 +: 3
        __field Rdn 16 +: 3
        __opcode '01000000 00xxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rdn);  n = UInt(Rdn);  m = UInt(Rm);  setflags = !InITBlock();
            (shift_t, shift_n) = (SRType_LSL, 0);

    __encoding aarch32_AND_r_T2_A
        __instruction_set T32
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm2 6 +: 2
        __field type1 4 +: 2
        __field Rm 0 +: 4
        __opcode '11101010 000xxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rd == '1111' && S == '1' then SEE "TST (register)";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  setflags = (S == '1');
            (shift_t, shift_n) = DecodeImmShift(type1, imm3:imm2);
            if (d == 15 && !setflags) || n == 15 || m == 15 then UNPREDICTABLE;
            // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        (shifted, carry) = Shift_C(R[m], shift_t, shift_n, PSTATE.C);
        result = R[n] AND shifted;
        if d == 15 then          // Can only occur for A32 encoding
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

__instruction aarch32_ORR_r_A
    __encoding aarch32_ORR_r_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field imm5 7 +: 5
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0001 100xxxxx xxxxxxxx xxx0xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  setflags = (S == '1');
            (shift_t, shift_n) = DecodeImmShift(type1, imm5);

    __encoding aarch32_ORR_r_T1_A
        __instruction_set T16
        __field Rm 19 +: 3
        __field Rdn 16 +: 3
        __opcode '01000011 00xxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rdn);  n = UInt(Rdn);  m = UInt(Rm);  setflags = !InITBlock();
            (shift_t, shift_n) = (SRType_LSL, 0);

    __encoding aarch32_ORR_r_T2_A
        __instruction_set T32
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm2 6 +: 2
        __field type1 4 +: 2
        __field Rm 0 +: 4
        __opcode '11101010 010xxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "Related encodings";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  setflags = (S == '1');
            (shift_t, shift_n) = DecodeImmShift(type1, imm3:imm2);
            if d == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        (shifted, carry) = Shift_C(R[m], shift_t, shift_n, PSTATE.C);
        result = R[n] OR shifted;
        if d == 15 then          // Can only occur for A32 encoding
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

__instruction aarch32_EOR_r_A
    __encoding aarch32_EOR_r_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field imm5 7 +: 5
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0000 001xxxxx xxxxxxxx xxx0xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  setflags = (S == '1');
            (shift_t, shift_n) = DecodeImmShift(type1, imm5);

    __encoding aarch32_EOR_r_T1_A
        __instruction_set T16
        __field Rm 19 +: 3
        __field Rdn 16 +: 3
        __opcode '01000000 01xxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rdn);  n = UInt(Rdn);  m = UInt(Rm);  setflags = !InITBlock();
            (shift_t, shift_n) = (SRType_LSL, 0);

    __encoding aarch32_EOR_r_T2_A
        __instruction_set T32
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm2 6 +: 2
        __field type1 4 +: 2
        __field Rm 0 +: 4
        __opcode '11101010 100xxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rd == '1111' && S == '1' then SEE "TEQ (register)";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  setflags = (S == '1');
            (shift_t, shift_n) = DecodeImmShift(type1, imm3:imm2);
            if (d == 15 && !setflags) || n == 15 || m == 15 then UNPREDICTABLE;
            // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        (shifted, carry) = Shift_C(R[m], shift_t, shift_n, PSTATE.C);
        result = R[n] EOR shifted;
        if d == 15 then          // Can only occur for A32 encoding
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

__instruction aarch32_BIC_r_A
    __encoding aarch32_BIC_r_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field imm5 7 +: 5
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0001 110xxxxx xxxxxxxx xxx0xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  setflags = (S == '1');
            (shift_t, shift_n) = DecodeImmShift(type1, imm5);

    __encoding aarch32_BIC_r_T1_A
        __instruction_set T16
        __field Rm 19 +: 3
        __field Rdn 16 +: 3
        __opcode '01000011 10xxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rdn);  n = UInt(Rdn);  m = UInt(Rm);  setflags = !InITBlock();
            (shift_t, shift_n) = (SRType_LSL, 0);

    __encoding aarch32_BIC_r_T2_A
        __instruction_set T32
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm2 6 +: 2
        __field type1 4 +: 2
        __field Rm 0 +: 4
        __opcode '11101010 001xxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  setflags = (S == '1');
            (shift_t, shift_n) = DecodeImmShift(type1, imm3:imm2);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;  // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        (shifted, carry) = Shift_C(R[m], shift_t, shift_n, PSTATE.C);
        result = R[n] AND NOT(shifted);
        if d == 15 then          // Can only occur for A32 encoding
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
