__instruction aarch32_B_A
    __encoding aarch32_B_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field imm24 0 +: 24
        __opcode 'xxxx1010 xxxxxxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            imm32 = SignExtend(imm24:'00', 32);

    __encoding aarch32_B_T1_A
        __instruction_set T16
        __field cond 24 +: 4
        __field imm8 16 +: 8
        __opcode '1101xxxx xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            if cond == '1110' then SEE "UDF";
            if cond == '1111' then SEE "SVC";
            imm32 = SignExtend(imm8:'0', 32);
            if InITBlock() then UNPREDICTABLE;

    __encoding aarch32_B_T2_A
        __instruction_set T16
        __field imm11 16 +: 11
        __opcode '11100xxx xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            imm32 = SignExtend(imm11:'0', 32);
            if InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __encoding aarch32_B_T3_A
        __instruction_set T32
        __field S 26 +: 1
        __field cond 22 +: 4
        __field imm6 16 +: 6
        __field J1 13 +: 1
        __field J2 11 +: 1
        __field imm11 0 +: 11
        __opcode '11110xxx xxxxxxxx 10x0xxxx xxxxxxxx'
        __guard TRUE
        __decode
            if cond<3:1> == '111' then SEE "Related encodings";
            imm32 = SignExtend(S:J2:J1:imm6:imm11:'0', 32);
            if InITBlock() then UNPREDICTABLE;

    __encoding aarch32_B_T4_A
        __instruction_set T32
        __field S 26 +: 1
        __field imm10 16 +: 10
        __field J1 13 +: 1
        __field J2 11 +: 1
        __field imm11 0 +: 11
        __opcode '11110xxx xxxxxxxx 10x1xxxx xxxxxxxx'
        __guard TRUE
        __decode
            I1 = NOT(J1 EOR S);  I2 = NOT(J2 EOR S);  imm32 = SignExtend(S:I1:I2:imm10:imm11:'0', 32);
            if InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __execute __conditional
        BranchWritePC(PC + imm32, BranchType_DIR);

__instruction aarch32_BL_i_A
    __encoding aarch32_BL_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field imm24 0 +: 24
        __opcode 'xxxx1011 xxxxxxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            imm32 = SignExtend(imm24:'00', 32);  targetInstrSet = InstrSet_A32;

    __encoding aarch32_BL_i_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field H 24 +: 1
        __field imm24 0 +: 24
        __opcode '1111101x xxxxxxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            imm32 = SignExtend(imm24:H:'0', 32);  targetInstrSet = InstrSet_T32;

    __encoding aarch32_BL_i_T1_A
        __instruction_set T32
        __field S 26 +: 1
        __field imm10 16 +: 10
        __field J1 13 +: 1
        __field J2 11 +: 1
        __field imm11 0 +: 11
        __opcode '11110xxx xxxxxxxx 11x1xxxx xxxxxxxx'
        __guard TRUE
        __decode
            I1 = NOT(J1 EOR S);  I2 = NOT(J2 EOR S);  imm32 = SignExtend(S:I1:I2:imm10:imm11:'0', 32);
            targetInstrSet = InstrSet_T32;
            if InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __encoding aarch32_BL_i_T2_A
        __instruction_set T32
        __field S 26 +: 1
        __field imm10H 16 +: 10
        __field J1 13 +: 1
        __field J2 11 +: 1
        __field imm10L 1 +: 10
        __field H 0 +: 1
        __opcode '11110xxx xxxxxxxx 11x0xxxx xxxxxxxx'
        __guard TRUE
        __decode
            if H == '1' then UNDEFINED;
            I1 = NOT(J1 EOR S);  I2 = NOT(J2 EOR S);  imm32 = SignExtend(S:I1:I2:imm10H:imm10L:'00', 32);
            targetInstrSet = InstrSet_A32;
            if InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __execute __conditional
        if CurrentInstrSet() == InstrSet_A32 then
            LR = PC - 4;
        else
            LR = PC<31:1> : '1';
        if targetInstrSet == InstrSet_A32 then
            targetAddress = Align(PC,4) + imm32;
        else
            targetAddress = PC + imm32;
        SelectInstrSet(targetInstrSet);
        BranchWritePC(targetAddress, BranchType_DIRCALL);
