__instruction aarch32_AND_i_A
    __encoding aarch32_AND_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0010 000xxxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');
            (imm32, carry) = A32ExpandImm_C(imm12, PSTATE.C);

    __encoding aarch32_AND_i_T1_A
        __instruction_set T32
        __field i 26 +: 1
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm8 0 +: 8
        __opcode '11110x00 000xxxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rd == '1111' && S == '1' then SEE "TST (immediate)";
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');
            (imm32, carry) = T32ExpandImm_C(i:imm3:imm8, PSTATE.C);
            if (d == 15 && !setflags) || n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        result = R[n] AND imm32;
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

__instruction aarch32_ORR_i_A
    __encoding aarch32_ORR_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0011 100xxxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');
            (imm32, carry) = A32ExpandImm_C(imm12, PSTATE.C);

    __encoding aarch32_ORR_i_T1_A
        __instruction_set T32
        __field i 26 +: 1
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm8 0 +: 8
        __opcode '11110x00 010xxxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "MOV (immediate)";
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');
            (imm32, carry) = T32ExpandImm_C(i:imm3:imm8, PSTATE.C);
            if d == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        result = R[n] OR imm32;
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

__instruction aarch32_EOR_i_A
    __encoding aarch32_EOR_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0010 001xxxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');
            (imm32, carry) = A32ExpandImm_C(imm12, PSTATE.C);

    __encoding aarch32_EOR_i_T1_A
        __instruction_set T32
        __field i 26 +: 1
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm8 0 +: 8
        __opcode '11110x00 100xxxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rd == '1111' && S == '1' then SEE "TEQ (immediate)";
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');
            (imm32, carry) = T32ExpandImm_C(i:imm3:imm8, PSTATE.C);
            if (d == 15 && !setflags) || n == 15 then UNPREDICTABLE;
            // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        result = R[n] EOR imm32;
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

__instruction aarch32_BIC_i_A
    __encoding aarch32_BIC_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0011 110xxxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');
            (imm32, carry) = A32ExpandImm_C(imm12, PSTATE.C);

    __encoding aarch32_BIC_i_T1_A
        __instruction_set T32
        __field i 26 +: 1
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm8 0 +: 8
        __opcode '11110x00 001xxxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');
            (imm32, carry) = T32ExpandImm_C(i:imm3:imm8, PSTATE.C);
            if d == 15 || n == 15 then UNPREDICTABLE;  // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        result = R[n] AND NOT(imm32);
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
