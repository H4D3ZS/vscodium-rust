__instruction aarch32_TST_i_A
    __encoding aarch32_TST_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0011 0001xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            n = UInt(Rn);
            (imm32, carry) = A32ExpandImm_C(imm12, PSTATE.C);

    __encoding aarch32_TST_i_T1_A
        __instruction_set T32
        __field i 26 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field imm8 0 +: 8
        __opcode '11110x00 0001xxxx 0xxx1111 xxxxxxxx'
        __guard TRUE
        __decode
            n = UInt(Rn);
            (imm32, carry) = T32ExpandImm_C(i:imm3:imm8, PSTATE.C);
            if n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        result = R[n] AND imm32;
        PSTATE.N = result<31>;
        PSTATE.Z = IsZeroBit(result);
        PSTATE.C = carry;
        // PSTATE.V unchanged

__instruction aarch32_TST_r_A
    __encoding aarch32_TST_r_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field imm5 7 +: 5
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0001 0001xxxx xxxxxxxx xxx0xxxx'
        __guard cond != '1111'
        __decode
            n = UInt(Rn);  m = UInt(Rm);
            (shift_t, shift_n) = DecodeImmShift(type1, imm5);

    __encoding aarch32_TST_r_T1_A
        __instruction_set T16
        __field Rm 19 +: 3
        __field Rn 16 +: 3
        __opcode '01000010 00xxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            n = UInt(Rn);  m = UInt(Rm);
            (shift_t, shift_n) = (SRType_LSL, 0);

    __encoding aarch32_TST_r_T2_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field imm2 6 +: 2
        __field type1 4 +: 2
        __field Rm 0 +: 4
        __opcode '11101010 0001xxxx xxxx1111 xxxxxxxx'
        __guard TRUE
        __decode
            n = UInt(Rn);  m = UInt(Rm);
            (shift_t, shift_n) = DecodeImmShift(type1, imm3:imm2);
            if n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        (shifted, carry) = Shift_C(R[m], shift_t, shift_n, PSTATE.C);
        result = R[n] AND shifted;
        PSTATE.N = result<31>;
        PSTATE.Z = IsZeroBit(result);
        PSTATE.C = carry;
        // PSTATE.V unchanged

__instruction aarch32_TST_rr_A
    __encoding aarch32_TST_rr_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rs 8 +: 4
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0001 0001xxxx xxxxxxxx 0xx1xxxx'
        __guard cond != '1111'
        __decode
            n = UInt(Rn);  m = UInt(Rm);  s = UInt(Rs);
            shift_t = DecodeRegShift(type1);
            if n == 15 || m == 15 || s == 15 then UNPREDICTABLE;

    __execute __conditional
        shift_n = UInt(R[s]<7:0>);
        (shifted, carry) = Shift_C(R[m], shift_t, shift_n, PSTATE.C);
        result = R[n] AND shifted;
        PSTATE.N = result<31>;
        PSTATE.Z = IsZeroBit(result);
        PSTATE.C = carry;
        // PSTATE.V unchanged
