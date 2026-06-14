__instruction aarch32_ADC_i_A
    __encoding aarch32_ADC_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0010 101xxxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');  imm32 = A32ExpandImm(imm12);

    __encoding aarch32_ADC_i_T1_A
        __instruction_set T32
        __field i 26 +: 1
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm8 0 +: 8
        __opcode '11110x01 010xxxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');  imm32 = T32ExpandImm(i:imm3:imm8);
            if d == 15 || n == 15 then UNPREDICTABLE;  // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        (result, nzcv) = AddWithCarry(R[n], imm32, PSTATE.C);
        if d == 15 then          // Can only occur for A32 encoding
            if setflags then
                ALUExceptionReturn(result);
            else
                ALUWritePC(result);
        else
            R[d] = result;
            if setflags then
                PSTATE.<N,Z,C,V> = nzcv;

__instruction aarch32_ADD_i_A
    __encoding aarch32_ADD_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0010 100xxxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            if Rn == '1111' && S == '0' then SEE "ADR";
            if Rn == '1101' then SEE "ADD (SP plus immediate)";
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');  imm32 = A32ExpandImm(imm12);

    __encoding aarch32_ADD_i_T1_A
        __instruction_set T16
        __field imm3 22 +: 3
        __field Rn 19 +: 3
        __field Rd 16 +: 3
        __opcode '0001110x xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  setflags = !InITBlock();  imm32 = ZeroExtend(imm3, 32);

    __encoding aarch32_ADD_i_T2_A
        __instruction_set T16
        __field Rdn 24 +: 3
        __field imm8 16 +: 8
        __opcode '00110xxx xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rdn);  n = UInt(Rdn);  setflags = !InITBlock();  imm32 = ZeroExtend(imm8, 32);

    __encoding aarch32_ADD_i_T3_A
        __instruction_set T32
        __field i 26 +: 1
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm8 0 +: 8
        __opcode '11110x01 000xxxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rd == '1111' && S == '1' then SEE "CMN (immediate)";
            if Rn == '1101' then SEE "ADD (SP plus immediate)";
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');  imm32 = T32ExpandImm(i:imm3:imm8);
            if (d == 15 && !setflags) || n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __encoding aarch32_ADD_i_T4_A
        __instruction_set T32
        __field i 26 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm8 0 +: 8
        __opcode '11110x10 0000xxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "ADR";
            if Rn == '1101' then SEE "ADD (SP plus immediate)";
            d = UInt(Rd);  n = UInt(Rn);  setflags = FALSE;  imm32 = ZeroExtend(i:imm3:imm8, 32);
            if d == 15 then UNPREDICTABLE;   // Armv8-A removes UNPREDICTABLE for R13

    __execute
        if CurrentInstrSet() == InstrSet_A32 then
            if ConditionPassed() then
                EncodingSpecificOperations();
                (result, nzcv) = AddWithCarry(R[n], imm32, '0');
                if d == 15 then          // Can only occur for A32 encoding
                    if setflags then
                        ALUExceptionReturn(result);
                    else
                        ALUWritePC(result);
                else
                    R[d] = result;
                    if setflags then
                        PSTATE.<N,Z,C,V> = nzcv;
        else
            if ConditionPassed() then
                EncodingSpecificOperations();
                (result, nzcv) = AddWithCarry(R[n], imm32, '0');
                R[d] = result;
                if setflags then
                    PSTATE.<N,Z,C,V> = nzcv;

__instruction aarch32_SUB_i_A
    __encoding aarch32_SUB_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0010 010xxxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            if Rn == '1111' && S == '0' then SEE "ADR";
            if Rn == '1101' then SEE "SUB (SP minus immediate)";
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');  imm32 = A32ExpandImm(imm12);

    __encoding aarch32_SUB_i_T1_A
        __instruction_set T16
        __field imm3 22 +: 3
        __field Rn 19 +: 3
        __field Rd 16 +: 3
        __opcode '0001111x xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  setflags = !InITBlock();  imm32 = ZeroExtend(imm3, 32);

    __encoding aarch32_SUB_i_T2_A
        __instruction_set T16
        __field Rdn 24 +: 3
        __field imm8 16 +: 8
        __opcode '00111xxx xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rdn);  n = UInt(Rdn);  setflags = !InITBlock();  imm32 = ZeroExtend(imm8, 32);

    __encoding aarch32_SUB_i_T3_A
        __instruction_set T32
        __field i 26 +: 1
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm8 0 +: 8
        __opcode '11110x01 101xxxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rd == '1111' && S == '1' then SEE "CMP (immediate)";
            if Rn == '1101' then SEE "SUB (SP minus immediate)";
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');  imm32 = T32ExpandImm(i:imm3:imm8);
            if (d == 15 && !setflags) || n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __encoding aarch32_SUB_i_T4_A
        __instruction_set T32
        __field i 26 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm8 0 +: 8
        __opcode '11110x10 1010xxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "ADR";
            if Rn == '1101' then SEE "SUB (SP minus immediate)";
            d = UInt(Rd);  n = UInt(Rn);  setflags = FALSE;  imm32 = ZeroExtend(i:imm3:imm8, 32);
            if d == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __encoding aarch32_SUB_i_T5_AS
        __instruction_set T32
        __field Rn 16 +: 4
        __field imm8 0 +: 8
        __opcode '11110011 1101xxxx 10x0xxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1110' && IsZero(imm8) then SEE "ERET";
            d = 15;  n = UInt(Rn);  setflags = TRUE;  imm32 = ZeroExtend(imm8, 32);
            if n != 14 then UNPREDICTABLE;
            if InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __execute __conditional
        (result, nzcv) = AddWithCarry(R[n], NOT(imm32), '1');
        if d == 15 then
            if setflags then
                ALUExceptionReturn(result);
            else
                ALUWritePC(result);
        else
            R[d] = result;
            if setflags then
                PSTATE.<N,Z,C,V> = nzcv;

__instruction aarch32_RSB_i_A
    __encoding aarch32_RSB_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0010 011xxxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');  imm32 = A32ExpandImm(imm12);

    __encoding aarch32_RSB_i_T1_A
        __instruction_set T16
        __field Rn 19 +: 3
        __field Rd 16 +: 3
        __opcode '01000010 01xxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  setflags = !InITBlock();  imm32 = Zeros(32); // immediate = #0

    __encoding aarch32_RSB_i_T2_A
        __instruction_set T32
        __field i 26 +: 1
        __field S 20 +: 1
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm8 0 +: 8
        __opcode '11110x01 110xxxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);  setflags = (S == '1');  imm32 = T32ExpandImm(i:imm3:imm8);
            if d == 15 || n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        (result, nzcv) = AddWithCarry(NOT(R[n]), imm32, '1');
        if d == 15 then          // Can only occur for A32 encoding
            if setflags then
                ALUExceptionReturn(result);
            else
                ALUWritePC(result);
        else
            R[d] = result;
            if setflags then
                PSTATE.<N,Z,C,V> = nzcv;
