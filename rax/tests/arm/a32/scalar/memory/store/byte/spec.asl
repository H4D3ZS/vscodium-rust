__instruction aarch32_STRB_i_A
    __encoding aarch32_STRB_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field P 24 +: 1
        __field U 23 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx010x x1x0xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            if P == '0' && W == '1' then SEE "STRBT";
            t = UInt(Rt);  n = UInt(Rn);  imm32 = ZeroExtend(imm12, 32);
            index = (P == '1');  add = (U == '1');  wback = (P == '0') || (W == '1');
            if t == 15 then UNPREDICTABLE;
            if wback && (n == 15 || n == t) then UNPREDICTABLE;

    __encoding aarch32_STRB_i_T1_A
        __instruction_set T16
        __field imm5 22 +: 5
        __field Rn 19 +: 3
        __field Rt 16 +: 3
        __opcode '01110xxx xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            t = UInt(Rt);  n = UInt(Rn);  imm32 = ZeroExtend(imm5, 32);
            index = TRUE;  add = TRUE;  wback = FALSE;

    __encoding aarch32_STRB_i_T2_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm12 0 +: 12
        __opcode '11111000 1000xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then UNDEFINED;
            t = UInt(Rt);  n = UInt(Rn);  imm32 = ZeroExtend(imm12, 32);
            index = TRUE;  add = TRUE;  wback = FALSE;
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __encoding aarch32_STRB_i_T3_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field P 10 +: 1
        __field U 9 +: 1
        __field W 8 +: 1
        __field imm8 0 +: 8
        __opcode '11111000 0000xxxx xxxx1xxx xxxxxxxx'
        __guard TRUE
        __decode
            if P == '1' && U == '1' && W == '0' then SEE "STRBT";
            if Rn == '1111' || (P == '0' && W == '0') then UNDEFINED;
            t = UInt(Rt);  n = UInt(Rn);  imm32 = ZeroExtend(imm8, 32);
            index = (P == '1');  add = (U == '1');  wback = (W == '1');
            if t == 15 || (wback && n == t) then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute
        if CurrentInstrSet() == InstrSet_A32 then
            if ConditionPassed() then
                EncodingSpecificOperations();
                offset_addr = if add then (R[n] + imm32) else (R[n] - imm32);
                address = if index then offset_addr else R[n];
                MemU[address,1] = R[t]<7:0>;
                if wback then R[n] = offset_addr;
        else
            if ConditionPassed() then
                EncodingSpecificOperations();
                offset_addr = if add then (R[n] + imm32) else (R[n] - imm32);
                address = if index then offset_addr else R[n];
                MemU[address,1] = R[t]<7:0>;
                if wback then R[n] = offset_addr;

__instruction aarch32_STRB_r_A
    __encoding aarch32_STRB_r_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field P 24 +: 1
        __field U 23 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm5 7 +: 5
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx011x x1x0xxxx xxxxxxxx xxx0xxxx'
        __guard cond != '1111'
        __decode
            if P == '0' && W == '1' then SEE "STRBT";
            t = UInt(Rt);  n = UInt(Rn);  m = UInt(Rm);
            index = (P == '1');  add = (U == '1');  wback = (P == '0') || (W == '1');
            (shift_t, shift_n) = DecodeImmShift(type1, imm5);
            if t == 15 || m == 15 then UNPREDICTABLE;
            if wback && (n == 15 || n == t) then UNPREDICTABLE;

    __encoding aarch32_STRB_r_T1_A
        __instruction_set T16
        __field Rm 22 +: 3
        __field Rn 19 +: 3
        __field Rt 16 +: 3
        __opcode '0101010x xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            t = UInt(Rt);  n = UInt(Rn);  m = UInt(Rm);
            index = TRUE;  add = TRUE;  wback = FALSE;
            (shift_t, shift_n) = (SRType_LSL, 0);

    __encoding aarch32_STRB_r_T2_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm2 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111000 0000xxxx xxxx0000 00xxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then UNDEFINED;
            t = UInt(Rt);  n = UInt(Rn);  m = UInt(Rm);
            index = TRUE;  add = TRUE;  wback = FALSE;
            (shift_t, shift_n) = (SRType_LSL, UInt(imm2));
            if t == 15 || m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        offset = Shift(R[m], shift_t, shift_n, PSTATE.C);
        offset_addr = if add then (R[n] + offset) else (R[n] - offset);
        address = if index then offset_addr else R[n];
        MemU[address,1] = R[t]<7:0>;
        if wback then R[n] = offset_addr;
