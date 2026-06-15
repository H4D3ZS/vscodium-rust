__instruction aarch32_PLI_i_A
    __encoding aarch32_PLI_i_A1_A
        __instruction_set A32
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field imm12 0 +: 12
        __opcode '11110100 x101xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            n = UInt(Rn);  imm32 = ZeroExtend(imm12, 32);  add = (U == '1');

    __encoding aarch32_PLI_i_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field imm12 0 +: 12
        __opcode '11111001 1001xxxx 1111xxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "encoding T3";
            n = UInt(Rn);  imm32 = ZeroExtend(imm12, 32);  add = TRUE;

    __encoding aarch32_PLI_i_T2_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field imm8 0 +: 8
        __opcode '11111001 0001xxxx 11111100 xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "encoding T3";
            n = UInt(Rn);  imm32 = ZeroExtend(imm8, 32);  add = FALSE;

    __encoding aarch32_PLI_i_T3_A
        __instruction_set T32
        __field U 23 +: 1
        __field imm12 0 +: 12
        __opcode '11111001 x0011111 1111xxxx xxxxxxxx'
        __guard TRUE
        __decode
            n = 15;  imm32 = ZeroExtend(imm12, 32);  add = (U == '1');

    __execute __conditional
        base = if n == 15 then Align(PC,4) else R[n];
        address = if add then (base + imm32) else (base - imm32);
        Hint_PreloadInstr(address);

__instruction aarch32_PLI_r_A
    __encoding aarch32_PLI_r_A1_A
        __instruction_set A32
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field imm5 7 +: 5
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode '11110110 x101xxxx xxxxxxxx xxx0xxxx'
        __guard TRUE
        __decode
            n = UInt(Rn);  m = UInt(Rm);  add = (U == '1');
            (shift_t, shift_n) = DecodeImmShift(type1, imm5);
            if m == 15 then UNPREDICTABLE;

    __encoding aarch32_PLI_r_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field imm2 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111001 0001xxxx 11110000 00xxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "PLI (immediate, literal)";
            n = UInt(Rn);  m = UInt(Rm);  add = TRUE;
            (shift_t, shift_n) = (SRType_LSL, UInt(imm2));
            if m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        offset = Shift(R[m], shift_t, shift_n, PSTATE.C);
        address = if add then (R[n] + offset) else (R[n] - offset);
        Hint_PreloadInstr(address);
