__instruction aarch32_RSC_i_A
    __encoding aarch32_RSC_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0010 111xxxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');  imm32 = A32ExpandImm(imm12);

    __execute __conditional
        (result, nzcv) = AddWithCarry(NOT(R[n]), imm32, PSTATE.C);
        if d == 15 then
            if setflags then
                ALUExceptionReturn(result);
            else
                ALUWritePC(result);
        else
            R[d] = result;
            if setflags then
                PSTATE.<N,Z,C,V> = nzcv;

__instruction aarch32_SBC_i_A
    __encoding aarch32_SBC_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0010 110xxxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');  imm32 = A32ExpandImm(imm12);

    __encoding aarch32_SBC_i_T1_A
        __instruction_set T32
        __field i 26 +: 1
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm8 0 +: 8
        __opcode '11110x01 011xxxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');  imm32 = T32ExpandImm(i:imm3:imm8);
            if d == 15 || n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        (result, nzcv) = AddWithCarry(R[n], NOT(imm32), PSTATE.C);
        if d == 15 then          // Can only occur for A32 encoding
            if setflags then
                ALUExceptionReturn(result);
            else
                ALUWritePC(result);
        else
            R[d] = result;
            if setflags then
                PSTATE.<N,Z,C,V> = nzcv;
