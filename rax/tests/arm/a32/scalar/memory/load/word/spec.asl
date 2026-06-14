__instruction aarch32_LDR_i_A
    __encoding aarch32_LDR_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field P 24 +: 1
        __field U 23 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx010x x0x1xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            if Rn == '1111' then SEE "LDR (literal)";
            if P == '0' && W == '1' then SEE "LDRT";
            t = UInt(Rt);  n = UInt(Rn);  imm32 = ZeroExtend(imm12, 32);
            index = (P == '1');  add = (U == '1');  wback = (P == '0') || (W == '1');
            if wback && n == t then UNPREDICTABLE;

    __encoding aarch32_LDR_i_T1_A
        __instruction_set T16
        __field imm5 22 +: 5
        __field Rn 19 +: 3
        __field Rt 16 +: 3
        __opcode '01101xxx xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            t = UInt(Rt);  n = UInt(Rn);  imm32 = ZeroExtend(imm5:'00', 32);
            index = TRUE;  add = TRUE;  wback = FALSE;

    __encoding aarch32_LDR_i_T2_A
        __instruction_set T16
        __field Rt 24 +: 3
        __field imm8 16 +: 8
        __opcode '10011xxx xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            t = UInt(Rt);  n = 13;  imm32 = ZeroExtend(imm8:'00', 32);
            index = TRUE;  add = TRUE;  wback = FALSE;

    __encoding aarch32_LDR_i_T3_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm12 0 +: 12
        __opcode '11111000 1101xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "LDR (literal)";
            t = UInt(Rt);  n = UInt(Rn);  imm32 = ZeroExtend(imm12, 32); index = TRUE;  add = TRUE;
            wback = FALSE; if t == 15 && InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __encoding aarch32_LDR_i_T4_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field P 10 +: 1
        __field U 9 +: 1
        __field W 8 +: 1
        __field imm8 0 +: 8
        __opcode '11111000 0101xxxx xxxx1xxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "LDR (literal)";
            if P == '1' && U == '1' && W == '0' then SEE "LDRT";
            if P == '0' && W == '0' then UNDEFINED;
            t = UInt(Rt);  n = UInt(Rn);
            imm32 = ZeroExtend(imm8, 32); index = (P == '1');  add = (U == '1');  wback = (W == '1');
            if (wback && n == t) || (t == 15 && InITBlock() && !LastInITBlock()) then UNPREDICTABLE;

    __execute
        if CurrentInstrSet() == InstrSet_A32 then
            if ConditionPassed() then
                EncodingSpecificOperations();
                offset_addr = if add then (R[n] + imm32) else (R[n] - imm32);
                address = if index then offset_addr else R[n];
                data = MemU[address,4];
                if wback then R[n] = offset_addr;
                if t == 15 then
                    if address<1:0> == '00' then
                        LoadWritePC(data);
                    else
                        UNPREDICTABLE;
                else
                    R[t] = data;
        else
            if ConditionPassed() then
                EncodingSpecificOperations();
                offset_addr = if add then (R[n] + imm32) else (R[n] - imm32);
                address = if index then offset_addr else R[n];
                data = MemU[address,4];
                if wback then R[n] = offset_addr;
                if t == 15 then
                    if address<1:0> == '00' then
                        LoadWritePC(data);
                    else
                        UNPREDICTABLE;
                else
                    R[t] = data;

__instruction aarch32_LDR_l_A
    __encoding aarch32_LDR_l_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field P 24 +: 1
        __field U 23 +: 1
        __field W 21 +: 1
        __field Rt 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx010x x0x11111 xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            if P == '0' && W == '1' then SEE "LDRT";
            t = UInt(Rt);  imm32 = ZeroExtend(imm12, 32);
            add = (U == '1');  wback = (P == '0') || (W == '1');
            if wback then UNPREDICTABLE;

    __encoding aarch32_LDR_l_T1_A
        __instruction_set T16
        __field Rt 24 +: 3
        __field imm8 16 +: 8
        __opcode '01001xxx xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            t = UInt(Rt);  imm32 = ZeroExtend(imm8:'00', 32);  add = TRUE;

    __encoding aarch32_LDR_l_T2_A
        __instruction_set T32
        __field U 23 +: 1
        __field Rt 12 +: 4
        __field imm12 0 +: 12
        __opcode '11111000 x1011111 xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            t = UInt(Rt);  imm32 = ZeroExtend(imm12, 32);  add = (U == '1');
            if t == 15 && InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __execute __conditional
        base = Align(PC,4);
        address = if add then (base + imm32) else (base - imm32);
        data = MemU[address,4];
        if t == 15 then
            if address<1:0> == '00' then
                LoadWritePC(data);
            else
                UNPREDICTABLE;
        else
            R[t] = data;

__instruction aarch32_LDR_r_A
    __encoding aarch32_LDR_r_A1_A
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
        __opcode 'xxxx011x x0x1xxxx xxxxxxxx xxx0xxxx'
        __guard cond != '1111'
        __decode
            if P == '0' && W == '1' then SEE "LDRT";
            t = UInt(Rt);  n = UInt(Rn);  m = UInt(Rm);
            index = (P == '1');  add = (U == '1');  wback = (P == '0') || (W == '1');
            (shift_t, shift_n) = DecodeImmShift(type1, imm5);
            if m == 15 then UNPREDICTABLE;
            if wback && (n == 15 || n == t) then UNPREDICTABLE;

    __encoding aarch32_LDR_r_T1_A
        __instruction_set T16
        __field Rm 22 +: 3
        __field Rn 19 +: 3
        __field Rt 16 +: 3
        __opcode '0101100x xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            t = UInt(Rt);  n = UInt(Rn);  m = UInt(Rm);
            (shift_t, shift_n) = (SRType_LSL, 0);

    __encoding aarch32_LDR_r_T2_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm2 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111000 0101xxxx xxxx0000 00xxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "LDR (literal)";
            t = UInt(Rt);  n = UInt(Rn);  m = UInt(Rm);
            (shift_t, shift_n) = (SRType_LSL, UInt(imm2));
            if m == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13
            if t == 15 && InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __execute
        if CurrentInstrSet() == InstrSet_A32 then
            if ConditionPassed() then
                EncodingSpecificOperations();
                offset = Shift(R[m], shift_t, shift_n, PSTATE.C);
                offset_addr = if add then (R[n] + offset) else (R[n] - offset);
                address = if index then offset_addr else R[n];
                data = MemU[address,4];
                if wback then R[n] = offset_addr;
                if t == 15 then
                    if address<1:0> == '00' then
                        LoadWritePC(data);
                    else
                        UNPREDICTABLE;
                else
                    R[t] = data;
        else
            if ConditionPassed() then
                EncodingSpecificOperations();
                offset = Shift(R[m], shift_t, shift_n, PSTATE.C);
                offset_addr = (R[n] + offset);
                address = offset_addr;
                data = MemU[address,4];
                if t == 15 then
                    if address<1:0> == '00' then
                        LoadWritePC(data);
                    else
                        UNPREDICTABLE;
                else
                    R[t] = data;
