__instruction aarch32_STC_A
    __encoding aarch32_STC_T1A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field P 24 +: 1
        __field U 23 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field imm8 0 +: 8
        __opcode 'xxxx110x x0x0xxxx 01011110 xxxxxxxx'
        __guard cond != '1111'
        __decode
            if P == '0' && U == '0' && W == '0' then UNDEFINED;
            n = UInt(Rn);  cp = 14;
            imm32 = ZeroExtend(imm8:'00', 32);  index = (P == '1');  add = (U == '1');  wback = (W == '1');
            if n == 15 && (wback || CurrentInstrSet() != InstrSet_A32) then UNPREDICTABLE;

    __encoding aarch32_STC_T1A1_A
        __instruction_set T32
        __field P 24 +: 1
        __field U 23 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field imm8 0 +: 8
        __opcode '1110110x x0x0xxxx 01011110 xxxxxxxx'
        __guard TRUE
        __decode
            if P == '0' && U == '0' && W == '0' then UNDEFINED;
            n = UInt(Rn);  cp = 14;
            imm32 = ZeroExtend(imm8:'00', 32);  index = (P == '1');  add = (U == '1');  wback = (W == '1');
            if n == 15 && (wback || CurrentInstrSet() != InstrSet_A32) then UNPREDICTABLE;

    __execute __conditional
        offset_addr = if add then (R[n] + imm32) else (R[n] - imm32);
        address = if index then offset_addr else R[n];

        // System register read from DBGDTRRXint.
        MemA[address,4] = DBGDTR_EL0[];

        if wback then R[n] = offset_addr;

__instruction aarch32_LDC_i_A
    __encoding aarch32_LDC_i_T1A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field P 24 +: 1
        __field U 23 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field imm8 0 +: 8
        __opcode 'xxxx110x x0x1xxxx 01011110 xxxxxxxx'
        __guard cond != '1111'
        __decode
            if Rn == '1111' then SEE "LDC (literal)";
            if P == '0' && U == '0' && W == '0' then UNDEFINED;
            n = UInt(Rn);  cp = 14;
            imm32 = ZeroExtend(imm8:'00', 32);  index = (P == '1');  add = (U == '1');  wback = (W == '1');

    __encoding aarch32_LDC_i_T1A1_A
        __instruction_set T32
        __field P 24 +: 1
        __field U 23 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field imm8 0 +: 8
        __opcode '1110110x x0x1xxxx 01011110 xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "LDC (literal)";
            if P == '0' && U == '0' && W == '0' then UNDEFINED;
            n = UInt(Rn);  cp = 14;
            imm32 = ZeroExtend(imm8:'00', 32);  index = (P == '1');  add = (U == '1');  wback = (W == '1');

    __execute __conditional
        offset_addr = if add then (R[n] + imm32) else (R[n] - imm32);
        address = if index then offset_addr else R[n];

        // System register write to DBGDTRTXint.
        DBGDTR_EL0[] = MemA[address,4];

        if wback then R[n] = offset_addr;

__instruction aarch32_LDC_l_A
    __encoding aarch32_LDC_l_T1A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field P 24 +: 1
        __field U 23 +: 1
        __field W 21 +: 1
        __field imm8 0 +: 8
        __opcode 'xxxx110x x0x11111 01011110 xxxxxxxx'
        __guard cond != '1111'
        __decode
            if P == '0' && U == '0' && W == '0' then UNDEFINED;
            index = (P == '1');  add = (U == '1');  cp = 14;  imm32 = ZeroExtend(imm8:'00', 32);
            if W == '1' || (P == '0' && CurrentInstrSet() != InstrSet_A32) then UNPREDICTABLE;

    __encoding aarch32_LDC_l_T1A1_A
        __instruction_set T32
        __field P 24 +: 1
        __field U 23 +: 1
        __field W 21 +: 1
        __field imm8 0 +: 8
        __opcode '1110110x x0x11111 01011110 xxxxxxxx'
        __guard TRUE
        __decode
            if P == '0' && U == '0' && W == '0' then UNDEFINED;
            index = (P == '1');  add = (U == '1');  cp = 14;  imm32 = ZeroExtend(imm8:'00', 32);
            if W == '1' || (P == '0' && CurrentInstrSet() != InstrSet_A32) then UNPREDICTABLE;

    __execute __conditional
        offset_addr = if add then (Align(PC,4) + imm32) else (Align(PC,4) - imm32);
        address = if index then offset_addr else Align(PC,4);

        // System register write to DBGDTRTXint.
        DBGDTR_EL0[] = MemA[address,4];
