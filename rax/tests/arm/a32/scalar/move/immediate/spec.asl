__instruction aarch32_MOV_i_A
    __encoding aarch32_MOV_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rd 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0011 101xxxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  setflags = (S == '1');  (imm32, carry) = A32ExpandImm_C(imm12, PSTATE.C);

    __encoding aarch32_MOV_i_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field imm4 16 +: 4
        __field Rd 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0011 0000xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  setflags = FALSE;  imm32 = ZeroExtend(imm4:imm12, 32);
            if d == 15 then UNPREDICTABLE;

    __encoding aarch32_MOV_i_T1_A
        __instruction_set T16
        __field Rd 24 +: 3
        __field imm8 16 +: 8
        __opcode '00100xxx xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rd);  setflags = !InITBlock();  imm32 = ZeroExtend(imm8, 32);  carry = PSTATE.C;

    __encoding aarch32_MOV_i_T2_A
        __instruction_set T32
        __field i 26 +: 1
        __field S 20 +: 1
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm8 0 +: 8
        __opcode '11110x00 010x1111 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  setflags = (S == '1');  (imm32, carry) = T32ExpandImm_C(i:imm3:imm8, PSTATE.C);
            if d == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __encoding aarch32_MOV_i_T3_A
        __instruction_set T32
        __field i 26 +: 1
        __field imm4 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm8 0 +: 8
        __opcode '11110x10 0100xxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  setflags = FALSE;  imm32 = ZeroExtend(imm4:i:imm3:imm8, 32);
            if d == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        result = imm32;
        if d == 15 then          // Can only occur for encoding A1
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
