__instruction aarch32_STRH_i_A
    __encoding aarch32_STRH_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field P 24 +: 1
        __field U 23 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm4H 8 +: 4
        __field imm4L 0 +: 4
        __opcode 'xxxx000x x1x0xxxx xxxxxxxx 1011xxxx'
        __guard cond != '1111'
        __decode
            if P == '0' && W == '1' then SEE "STRHT";
            t = UInt(Rt);  n = UInt(Rn);  imm32 = ZeroExtend(imm4H:imm4L, 32);
            index = (P == '1');  add = (U == '1');  wback = (P == '0') || (W == '1');
            if t == 15 then UNPREDICTABLE;
            if wback && (n == 15 || n == t) then UNPREDICTABLE;

    __encoding aarch32_STRH_i_T1_A
        __instruction_set T16
        __field imm5 22 +: 5
        __field Rn 19 +: 3
        __field Rt 16 +: 3
        __opcode '10000xxx xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            t = UInt(Rt);  n = UInt(Rn);  imm32 = ZeroExtend(imm5:'0', 32);
            index = TRUE;  add = TRUE;  wback = FALSE;

    __encoding aarch32_STRH_i_T2_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm12 0 +: 12
        __opcode '11111000 1010xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then UNDEFINED;
            t = UInt(Rt);  n = UInt(Rn);  imm32 = ZeroExtend(imm12, 32);
            index = TRUE;  add = TRUE;  wback = FALSE;
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __encoding aarch32_STRH_i_T3_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field P 10 +: 1
        __field U 9 +: 1
        __field W 8 +: 1
        __field imm8 0 +: 8
        __opcode '11111000 0010xxxx xxxx1xxx xxxxxxxx'
        __guard TRUE
        __decode
            if P == '1' && U == '1' && W == '0' then SEE "STRHT";
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
                MemU[address,2] = R[t]<15:0>;
                if wback then R[n] = offset_addr;
        else
            if ConditionPassed() then
                EncodingSpecificOperations();
                offset_addr = if add then (R[n] + imm32) else (R[n] - imm32);
                address = if index then offset_addr else R[n];
                MemU[address,2] = R[t]<15:0>;
                if wback then R[n] = offset_addr;

__instruction aarch32_STRH_r_A
    __encoding aarch32_STRH_r_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field P 24 +: 1
        __field U 23 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx000x x0x0xxxx xxxxxxxx 1011xxxx'
        __guard cond != '1111'
        __decode
            if P == '0' && W == '1' then SEE "STRHT";
            t = UInt(Rt);  n = UInt(Rn);  m = UInt(Rm);
            index = (P == '1');  add = (U == '1');  wback = (P == '0') || (W == '1');
            (shift_t, shift_n) = (SRType_LSL, 0);
            if t == 15 || m == 15 then UNPREDICTABLE;
            if wback && (n == 15 || n == t) then UNPREDICTABLE;

    __encoding aarch32_STRH_r_T1_A
        __instruction_set T16
        __field Rm 22 +: 3
        __field Rn 19 +: 3
        __field Rt 16 +: 3
        __opcode '0101001x xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            t = UInt(Rt);  n = UInt(Rn);  m = UInt(Rm);
            index = TRUE;  add = TRUE;  wback = FALSE;
            (shift_t, shift_n) = (SRType_LSL, 0);

    __encoding aarch32_STRH_r_T2_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm2 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111000 0010xxxx xxxx0000 00xxxxxx'
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
        MemU[address,2] = R[t]<15:0>;
        if wback then R[n] = offset_addr;
