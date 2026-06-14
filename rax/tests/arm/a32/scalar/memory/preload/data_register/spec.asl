__instruction aarch32_PLD_r_A
    __encoding aarch32_PLD_r_A1_A
        __instruction_set A32
        __field U 23 +: 1
        __field R 22 +: 1
        __field Rn 16 +: 4
        __field imm5 7 +: 5
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode '11110111 xx01xxxx xxxxxxxx xxx0xxxx'
        __guard TRUE
        __decode
            n = UInt(Rn);  m = UInt(Rm);  add = (U == '1');  is_pldw = (R == '0');
            (shift_t, shift_n) = DecodeImmShift(type1, imm5);
            if m == 15 || (n == 15 && is_pldw) then UNPREDICTABLE;

    __encoding aarch32_PLD_r_T1_A
        __instruction_set T32
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field imm2 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111000 00x1xxxx 11110000 00xxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "PLD (literal)";
            n = UInt(Rn);  m = UInt(Rm);  add = TRUE;  is_pldw = (W == '1');
            (shift_t, shift_n) = (SRType_LSL, UInt(imm2));
            if m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        offset = Shift(R[m], shift_t, shift_n, PSTATE.C);
        address = if add then (R[n] + offset) else (R[n] - offset);
        if is_pldw then
            Hint_PreloadDataForWrite(address);
        else
            Hint_PreloadData(address);
