__instruction aarch32_SSAT_A
    __encoding aarch32_SSAT_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field sat_imm 16 +: 5
        __field Rd 12 +: 4
        __field imm5 7 +: 5
        __field sh 6 +: 1
        __field Rn 0 +: 4
        __opcode 'xxxx0110 101xxxxx xxxxxxxx xx01xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  saturate_to = UInt(sat_imm)+1;
            (shift_t, shift_n) = DecodeImmShift(sh:'0', imm5);
            if d == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_SSAT_T1_A
        __instruction_set T32
        __field sh 21 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm2 6 +: 2
        __field sat_imm 0 +: 5
        __opcode '11110x11 00x0xxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if sh == '1' && (imm3:imm2) == '00000' then SEE "SSAT16";
            d = UInt(Rd);  n = UInt(Rn);  saturate_to = UInt(sat_imm)+1;
            (shift_t, shift_n) = DecodeImmShift(sh:'0', imm3:imm2);
            if d == 15 || n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        operand = Shift(R[n], shift_t, shift_n, PSTATE.C);  // PSTATE.C ignored
        (result, sat) = SignedSatQ(SInt(operand), saturate_to);
        R[d] = SignExtend(result, 32);
        if sat then
            PSTATE.Q = '1';

__instruction aarch32_SSAT16_A
    __encoding aarch32_SSAT16_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field sat_imm 16 +: 4
        __field Rd 12 +: 4
        __field Rn 0 +: 4
        __opcode 'xxxx0110 1010xxxx xxxxxxxx 0011xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  saturate_to = UInt(sat_imm)+1;
            if d == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_SSAT16_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field sat_imm 0 +: 4
        __opcode '11110x11 0010xxxx 0000xxxx 00xxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  saturate_to = UInt(sat_imm)+1;
            if d == 15 || n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        (result1, sat1) = SignedSatQ(SInt(R[n]<15:0>), saturate_to);
        (result2, sat2) = SignedSatQ(SInt(R[n]<31:16>), saturate_to);
        R[d]<15:0> = SignExtend(result1, 16);
        R[d]<31:16> = SignExtend(result2, 16);
        if sat1 || sat2 then
            PSTATE.Q = '1';

__instruction aarch32_USAT_A
    __encoding aarch32_USAT_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field sat_imm 16 +: 5
        __field Rd 12 +: 4
        __field imm5 7 +: 5
        __field sh 6 +: 1
        __field Rn 0 +: 4
        __opcode 'xxxx0110 111xxxxx xxxxxxxx xx01xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  saturate_to = UInt(sat_imm);
            (shift_t, shift_n) = DecodeImmShift(sh:'0', imm5);
            if d == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_USAT_T1_A
        __instruction_set T32
        __field sh 21 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm2 6 +: 2
        __field sat_imm 0 +: 5
        __opcode '11110x11 10x0xxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if sh == '1' && (imm3:imm2) == '00000' then SEE "USAT16";
            d = UInt(Rd);  n = UInt(Rn);  saturate_to = UInt(sat_imm);
            (shift_t, shift_n) = DecodeImmShift(sh:'0', imm3:imm2);
            if d == 15 || n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        operand = Shift(R[n], shift_t, shift_n, PSTATE.C);  // PSTATE.C ignored
        (result, sat) = UnsignedSatQ(SInt(operand), saturate_to);
        R[d] = ZeroExtend(result, 32);
        if sat then
            PSTATE.Q = '1';

__instruction aarch32_USAT16_A
    __encoding aarch32_USAT16_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field sat_imm 16 +: 4
        __field Rd 12 +: 4
        __field Rn 0 +: 4
        __opcode 'xxxx0110 1110xxxx xxxxxxxx 0011xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  saturate_to = UInt(sat_imm);
            if d == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_USAT16_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field sat_imm 0 +: 4
        __opcode '11110x11 1010xxxx 0000xxxx 00xxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  saturate_to = UInt(sat_imm);
            if d == 15 || n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        (result1, sat1) = UnsignedSatQ(SInt(R[n]<15:0>), saturate_to);
        (result2, sat2) = UnsignedSatQ(SInt(R[n]<31:16>), saturate_to);
        R[d]<15:0> = ZeroExtend(result1, 16);
        R[d]<31:16> = ZeroExtend(result2, 16);
        if sat1 || sat2 then
            PSTATE.Q = '1';
