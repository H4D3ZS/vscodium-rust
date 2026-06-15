__instruction aarch32_CMP_i_A
    __encoding aarch32_CMP_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0011 0101xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            n = UInt(Rn);  imm32 = A32ExpandImm(imm12);

    __encoding aarch32_CMP_i_T1_A
        __instruction_set T16
        __field Rn 24 +: 3
        __field imm8 16 +: 8
        __opcode '00101xxx xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            n = UInt(Rn);  imm32 = ZeroExtend(imm8, 32);

    __encoding aarch32_CMP_i_T2_A
        __instruction_set T32
        __field i 26 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field imm8 0 +: 8
        __opcode '11110x01 1011xxxx 0xxx1111 xxxxxxxx'
        __guard TRUE
        __decode
            n = UInt(Rn);  imm32 = T32ExpandImm(i:imm3:imm8);
            if n == 15 then UNPREDICTABLE;

    __execute __conditional
        (result, nzcv) = AddWithCarry(R[n], NOT(imm32), '1');
        PSTATE.<N,Z,C,V> = nzcv;

__instruction aarch32_CMP_r_A
    __encoding aarch32_CMP_r_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field imm5 7 +: 5
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0001 0101xxxx xxxxxxxx xxx0xxxx'
        __guard cond != '1111'
        __decode
            n = UInt(Rn);  m = UInt(Rm);
            (shift_t, shift_n) = DecodeImmShift(type1, imm5);

    __encoding aarch32_CMP_r_T1_A
        __instruction_set T16
        __field Rm 19 +: 3
        __field Rn 16 +: 3
        __opcode '01000010 10xxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            n = UInt(Rn);  m = UInt(Rm);
            (shift_t, shift_n) = (SRType_LSL, 0);

    __encoding aarch32_CMP_r_T2_A
        __instruction_set T16
        __field N 23 +: 1
        __field Rm 19 +: 4
        __field Rn 16 +: 3
        __opcode '01000101 xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            n = UInt(N:Rn);  m = UInt(Rm);
            (shift_t, shift_n) = (SRType_LSL, 0);
            if n < 8 && m < 8 then UNPREDICTABLE;
            if n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_CMP_r_T3_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field imm2 6 +: 2
        __field type1 4 +: 2
        __field Rm 0 +: 4
        __opcode '11101011 1011xxxx xxxx1111 xxxxxxxx'
        __guard TRUE
        __decode
            n = UInt(Rn);  m = UInt(Rm);
            (shift_t, shift_n) = DecodeImmShift(type1, imm3:imm2);
            if n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        shifted = Shift(R[m], shift_t, shift_n, PSTATE.C);
        (result, nzcv) = AddWithCarry(R[n], NOT(shifted), '1');
        PSTATE.<N,Z,C,V> = nzcv;

__instruction aarch32_CMP_rr_A
    __encoding aarch32_CMP_rr_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rs 8 +: 4
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0001 0101xxxx xxxxxxxx 0xx1xxxx'
        __guard cond != '1111'
        __decode
            n = UInt(Rn);  m = UInt(Rm);  s = UInt(Rs);
            shift_t = DecodeRegShift(type1);
            if n == 15 || m == 15 || s == 15 then UNPREDICTABLE;

    __execute __conditional
        shift_n = UInt(R[s]<7:0>);
        shifted = Shift(R[m], shift_t, shift_n, PSTATE.C);
        (result, nzcv) = AddWithCarry(R[n], NOT(shifted), '1');
        PSTATE.<N,Z,C,V> = nzcv;
