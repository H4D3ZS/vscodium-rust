__instruction aarch32_MVN_i_A
    __encoding aarch32_MVN_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rd 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0011 111xxxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  setflags = (S == '1');
            (imm32, carry) = A32ExpandImm_C(imm12, PSTATE.C);

    __encoding aarch32_MVN_i_T1_A
        __instruction_set T32
        __field i 26 +: 1
        __field S 20 +: 1
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm8 0 +: 8
        __opcode '11110x00 011x1111 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  setflags = (S == '1');
            (imm32, carry) = T32ExpandImm_C(i:imm3:imm8, PSTATE.C);
            if d == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        result = NOT(imm32);
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

__instruction aarch32_ORN_i_A
    __encoding aarch32_ORN_i_T1_A
        __instruction_set T32
        __field i 26 +: 1
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm8 0 +: 8
        __opcode '11110x00 011xxxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "MVN (immediate)";
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');
            (imm32, carry) = T32ExpandImm_C(i:imm3:imm8, PSTATE.C);
            if d == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        result = R[n] OR NOT(imm32);
        R[d] = result;
        if setflags then
            PSTATE.N = result<31>;
            PSTATE.Z = IsZeroBit(result);
            PSTATE.C = carry;
            // PSTATE.V unchanged
