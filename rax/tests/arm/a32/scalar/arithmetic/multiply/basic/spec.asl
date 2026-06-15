__instruction aarch32_MUL_A
    __encoding aarch32_MUL_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rd 16 +: 4
        __field Rm 8 +: 4
        __field Rn 0 +: 4
        __opcode 'xxxx0000 000xxxxx xxxxxxxx 1001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  setflags = (S == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_MUL_T1_A
        __instruction_set T16
        __field Rn 19 +: 3
        __field Rdm 16 +: 3
        __opcode '01000011 01xxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rdm);  n = UInt(Rn);  m = UInt(Rdm);  setflags = !InITBlock();

    __encoding aarch32_MUL_T2_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111011 0000xxxx 1111xxxx 0000xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  setflags = FALSE;
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        operand1 = SInt(R[n]);  // operand1 = UInt(R[n]) produces the same final results
        operand2 = SInt(R[m]);  // operand2 = UInt(R[m]) produces the same final results
        result = operand1 * operand2;
        R[d] = result<31:0>;
        if setflags then
            PSTATE.N = result<31>;
            PSTATE.Z = IsZeroBit(result<31:0>);
            // PSTATE.C, PSTATE.V unchanged

__instruction aarch32_MLA_A
    __encoding aarch32_MLA_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rd 16 +: 4
        __field Ra 12 +: 4
        __field Rm 8 +: 4
        __field Rn 0 +: 4
        __opcode 'xxxx0000 001xxxxx xxxxxxxx 1001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);  setflags = (S == '1');
            if d == 15 || n == 15 || m == 15 || a == 15 then UNPREDICTABLE;

    __encoding aarch32_MLA_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Ra 12 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111011 0000xxxx xxxxxxxx 0000xxxx'
        __guard TRUE
        __decode
            if Ra == '1111' then SEE "MUL";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);  setflags = FALSE;
            if d == 15 || n == 15 || m  == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        operand1 = SInt(R[n]);  // operand1 = UInt(R[n]) produces the same final results
        operand2 = SInt(R[m]);  // operand2 = UInt(R[m]) produces the same final results
        addend   = SInt(R[a]);  // addend   = UInt(R[a]) produces the same final results
        result = operand1 * operand2 + addend;
        R[d] = result<31:0>;
        if setflags then
            PSTATE.N = result<31>;
            PSTATE.Z = IsZeroBit(result<31:0>);
            // PSTATE.C, PSTATE.V unchanged

__instruction aarch32_MLS_A
    __encoding aarch32_MLS_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 16 +: 4
        __field Ra 12 +: 4
        __field Rm 8 +: 4
        __field Rn 0 +: 4
        __opcode 'xxxx0000 0110xxxx xxxxxxxx 1001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);
            if d == 15 || n == 15 || m == 15 || a == 15 then UNPREDICTABLE;

    __encoding aarch32_MLS_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Ra 12 +: 4
        __field Rd 8 +: 4
        __field Rm 0 +: 4
        __opcode '11111011 0000xxxx xxxxxxxx 0001xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);
            if d == 15 || n == 15 || m == 15 || a == 15 then UNPREDICTABLE;
            // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        operand1 = SInt(R[n]);  // operand1 = UInt(R[n]) produces the same final results
        operand2 = SInt(R[m]);  // operand2 = UInt(R[m]) produces the same final results
        addend   = SInt(R[a]);  // addend   = UInt(R[a]) produces the same final results
        result = addend - operand1 * operand2;
        R[d] = result<31:0>;
