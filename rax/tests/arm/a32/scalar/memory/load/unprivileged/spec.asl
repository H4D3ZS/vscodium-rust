__instruction aarch32_LDRT_A
    __encoding aarch32_LDRT_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0100 x011xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);  postindex = TRUE;  add = (U == '1');
            register_form = FALSE;  imm32 = ZeroExtend(imm12, 32);
            if t == 15 || n == 15 || n == t then UNPREDICTABLE;

    __encoding aarch32_LDRT_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm5 7 +: 5
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0110 x011xxxx xxxxxxxx xxx0xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);  m = UInt(Rm);  postindex = TRUE;  add = (U == '1');
            register_form = TRUE;  (shift_t, shift_n) = DecodeImmShift(type1, imm5);
            if t == 15 || n == 15 || n == t || m == 15 then UNPREDICTABLE;

    __encoding aarch32_LDRT_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm8 0 +: 8
        __opcode '11111000 0101xxxx xxxx1110 xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "LDR (literal)";
            t = UInt(Rt);  n = UInt(Rn);  postindex = FALSE;  add = TRUE;
            register_form = FALSE;  imm32 = ZeroExtend(imm8, 32);
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        if PSTATE.EL == EL2 then UNPREDICTABLE;               // Hyp mode
        EncodingSpecificOperations();
        offset = if register_form then Shift(R[m], shift_t, shift_n, PSTATE.C) else imm32;
        offset_addr = if add then (R[n] + offset) else (R[n] - offset);
        address = if postindex then R[n] else offset_addr;
        data = MemU_unpriv[address,4];
        if postindex then R[n] = offset_addr;
        R[t] = data;

__instruction aarch32_LDRBT_A
    __encoding aarch32_LDRBT_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0100 x111xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);  postindex = TRUE;  add = (U == '1');
            register_form = FALSE;  imm32 = ZeroExtend(imm12, 32);
            if t == 15 || n == 15 || n == t then UNPREDICTABLE;

    __encoding aarch32_LDRBT_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm5 7 +: 5
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0110 x111xxxx xxxxxxxx xxx0xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);  m = UInt(Rm);  postindex = TRUE;  add = (U == '1');
            register_form = TRUE;  (shift_t, shift_n) = DecodeImmShift(type1, imm5);
            if t == 15 || n == 15 || n == t || m == 15 then UNPREDICTABLE;

    __encoding aarch32_LDRBT_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm8 0 +: 8
        __opcode '11111000 0001xxxx xxxx1110 xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "LDRB (literal)";
            t = UInt(Rt);  n = UInt(Rn);  postindex = FALSE;  add = TRUE;
            register_form = FALSE;  imm32 = ZeroExtend(imm8, 32);
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        if PSTATE.EL == EL2 then UNPREDICTABLE;               // Hyp mode
        EncodingSpecificOperations();
        offset = if register_form then Shift(R[m], shift_t, shift_n, PSTATE.C) else imm32;
        offset_addr = if add then (R[n] + offset) else (R[n] - offset);
        address = if postindex then R[n] else offset_addr;
        R[t] = ZeroExtend(MemU_unpriv[address,1],32);
        if postindex then R[n] = offset_addr;

__instruction aarch32_LDRHT_A
    __encoding aarch32_LDRHT_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm4H 8 +: 4
        __field imm4L 0 +: 4
        __opcode 'xxxx0000 x111xxxx xxxxxxxx 1011xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);  postindex = TRUE;  add = (U == '1');
            register_form = FALSE;  imm32 = ZeroExtend(imm4H:imm4L, 32);
            if t == 15 || n == 15 || n == t then UNPREDICTABLE;

    __encoding aarch32_LDRHT_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0000 x011xxxx xxxxxxxx 1011xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);  m = UInt(Rm);  postindex = TRUE;  add = (U == '1');
            register_form = TRUE;
            if t == 15 || n == 15 || n == t || m == 15 then UNPREDICTABLE;

    __encoding aarch32_LDRHT_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm8 0 +: 8
        __opcode '11111000 0011xxxx xxxx1110 xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "LDRH (literal)";
            t = UInt(Rt);  n = UInt(Rn);  postindex = FALSE;  add = TRUE;
            register_form = FALSE;  imm32 = ZeroExtend(imm8, 32);
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        if PSTATE.EL == EL2 then UNPREDICTABLE;               // Hyp mode
        EncodingSpecificOperations();
        offset = if register_form then R[m] else imm32;
        offset_addr = if add then (R[n] + offset) else (R[n] - offset);
        address = if postindex then R[n] else offset_addr;
        data = MemU_unpriv[address,2];
        if postindex then R[n] = offset_addr;
        R[t] = ZeroExtend(data, 32);

__instruction aarch32_LDRSBT_A
    __encoding aarch32_LDRSBT_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm4H 8 +: 4
        __field imm4L 0 +: 4
        __opcode 'xxxx0000 x111xxxx xxxxxxxx 1101xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);  postindex = TRUE;  add = (U == '1');
            register_form = FALSE;  imm32 = ZeroExtend(imm4H:imm4L, 32);
            if t == 15 || n == 15 || n == t then UNPREDICTABLE;

    __encoding aarch32_LDRSBT_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0000 x011xxxx xxxxxxxx 1101xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);  m = UInt(Rm);  postindex = TRUE;  add = (U == '1');
            register_form = TRUE;
            if t == 15 || n == 15 || n == t || m == 15 then UNPREDICTABLE;

    __encoding aarch32_LDRSBT_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm8 0 +: 8
        __opcode '11111001 0001xxxx xxxx1110 xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "LDRSB (literal)";
            t = UInt(Rt);  n = UInt(Rn);  postindex = FALSE;  add = TRUE;
            register_form = FALSE;  imm32 = ZeroExtend(imm8, 32);
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        if PSTATE.EL == EL2 then UNPREDICTABLE;               // Hyp mode
        EncodingSpecificOperations();
        offset = if register_form then R[m] else imm32;
        offset_addr = if add then (R[n] + offset) else (R[n] - offset);
        address = if postindex then R[n] else offset_addr;
        R[t] = SignExtend(MemU_unpriv[address,1], 32);
        if postindex then R[n] = offset_addr;
