__instruction aarch32_PKH_A
    __encoding aarch32_PKH_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field imm5 7 +: 5
        __field tb 6 +: 1
        __field Rm 0 +: 4
        __opcode 'xxxx0110 1000xxxx xxxxxxxx xx01xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  tbform = (tb == '1');
            (shift_t, shift_n) = DecodeImmShift(tb:'0', imm5);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_PKH_T1_A
        __instruction_set T32
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm2 6 +: 2
        __field tb 5 +: 1
        __field T 4 +: 1
        __field Rm 0 +: 4
        __opcode '11101010 1100xxxx xxxxxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if S == '1' || T == '1' then UNDEFINED;
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  tbform = (tb == '1');
            (shift_t, shift_n) = DecodeImmShift(tb:'0', imm3:imm2);
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        operand2 = Shift(R[m], shift_t, shift_n, PSTATE.C);  // PSTATE.C ignored
        R[d]<15:0>  = if tbform then operand2<15:0> else R[n]<15:0>;
        R[d]<31:16> = if tbform then R[n]<31:16>    else operand2<31:16>;
