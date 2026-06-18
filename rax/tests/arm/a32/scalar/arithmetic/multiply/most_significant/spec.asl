__instruction aarch32_SMMLA_A
    __encoding aarch32_SMMLA_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 16 +: 4
        __field Ra 12 +: 4
        __field Rm 8 +: 4
        __field R 5 +: 1
        __field Rn 0 +: 4
        __opcode 'xxxx0111 0101xxxx xxxxxxxx 00x1xxxx'
        __guard cond != '1111'
        __decode
            if Ra == '1111' then SEE "SMMUL";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);  round = (R == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;

    __encoding aarch32_SMMLA_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Ra 12 +: 4
        __field Rd 8 +: 4
        __field R 4 +: 1
        __field Rm 0 +: 4
        __opcode '11111011 0101xxxx xxxxxxxx 000xxxxx'
        __guard TRUE
        __decode
            if Ra == '1111' then SEE "SMMUL";
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);  round = (R == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        result = (SInt(R[a]) << 32) + SInt(R[n]) * SInt(R[m]);
        if round then result = result + 0x80000000;
        R[d] = result<63:32>;

__instruction aarch32_SMMLS_A
    __encoding aarch32_SMMLS_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 16 +: 4
        __field Ra 12 +: 4
        __field Rm 8 +: 4
        __field R 5 +: 1
        __field Rn 0 +: 4
        __opcode 'xxxx0111 0101xxxx xxxxxxxx 11x1xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);  round = (R == '1');
            if d == 15 || n == 15 || m == 15 || a == 15 then UNPREDICTABLE;

    __encoding aarch32_SMMLS_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Ra 12 +: 4
        __field Rd 8 +: 4
        __field R 4 +: 1
        __field Rm 0 +: 4
        __opcode '11111011 0110xxxx xxxxxxxx 000xxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  m = UInt(Rm);  a = UInt(Ra);  round = (R == '1');
            if d == 15 || n == 15 || m == 15 || a == 15 then UNPREDICTABLE;
            // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        result = (SInt(R[a]) << 32) - SInt(R[n]) * SInt(R[m]);
        if round then result = result + 0x80000000;
        R[d] = result<63:32>;
