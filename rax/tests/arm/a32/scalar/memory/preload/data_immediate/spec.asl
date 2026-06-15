__instruction aarch32_PLD_i_A
    __encoding aarch32_PLD_i_A1_A
        __instruction_set A32
        __field U 23 +: 1
        __field R 22 +: 1
        __field Rn 16 +: 4
        __field imm12 0 +: 12
        __opcode '11110101 xx01xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "PLD (literal)";
            n = UInt(Rn);  imm32 = ZeroExtend(imm12, 32);  add = (U == '1');  is_pldw = (R == '0');

    __encoding aarch32_PLD_i_T1_A
        __instruction_set T32
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field imm12 0 +: 12
        __opcode '11111000 10x1xxxx 1111xxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "PLD (literal)";
            n = UInt(Rn);  imm32 = ZeroExtend(imm12, 32);  add = TRUE;  is_pldw = (W == '1');

    __encoding aarch32_PLD_i_T2_A
        __instruction_set T32
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field imm8 0 +: 8
        __opcode '11111000 00x1xxxx 11111100 xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "PLD (literal)";
            n = UInt(Rn);  imm32 = ZeroExtend(imm8, 32);  add = FALSE;  is_pldw = (W == '1');

    __execute __conditional
        address = if add then (R[n] + imm32) else (R[n] - imm32);
        if is_pldw then
            Hint_PreloadDataForWrite(address);
        else
            Hint_PreloadData(address);

__instruction aarch32_PLD_l_A
    __encoding aarch32_PLD_l_A1_A
        __instruction_set A32
        __field U 23 +: 1
        __field imm12 0 +: 12
        __opcode '11110101 xx011111 xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            imm32 = ZeroExtend(imm12, 32);  add = (U == '1');

    __encoding aarch32_PLD_l_T1_A
        __instruction_set T32
        __field U 23 +: 1
        __field imm12 0 +: 12
        __opcode '11111000 x0x11111 1111xxxx xxxxxxxx'
        __guard TRUE
        __decode
            imm32 = ZeroExtend(imm12, 32);  add = (U == '1');

    __execute __conditional
        address = if add then (Align(PC,4) + imm32) else (Align(PC,4) - imm32);
        Hint_PreloadData(address);
