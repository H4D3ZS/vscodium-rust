__instruction aarch32_STRT_A
    __encoding aarch32_STRT_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0100 x010xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);  postindex = TRUE;  add = (U == '1');
            register_form = FALSE;  imm32 = ZeroExtend(imm12, 32);
            if n == 15 || n == t then UNPREDICTABLE;

    __encoding aarch32_STRT_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm5 7 +: 5
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0110 x010xxxx xxxxxxxx xxx0xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);  m = UInt(Rm);  postindex = TRUE;  add = (U == '1');
            register_form = TRUE;  (shift_t, shift_n) = DecodeImmShift(type1, imm5);
            if n == 15 || n == t || m == 15 then UNPREDICTABLE;

    __encoding aarch32_STRT_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm8 0 +: 8
        __opcode '11111000 0100xxxx xxxx1110 xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then UNDEFINED;
            t = UInt(Rt);  n = UInt(Rn);  postindex = FALSE;  add = TRUE;
            register_form = FALSE;  imm32 = ZeroExtend(imm8, 32);
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        if PSTATE.EL == EL2 then UNPREDICTABLE;               // Hyp mode
        EncodingSpecificOperations();
        offset = if register_form then Shift(R[m], shift_t, shift_n, PSTATE.C) else imm32;
        offset_addr = if add then (R[n] + offset) else (R[n] - offset);
        address = if postindex then R[n] else offset_addr;
        if t == 15 then  // Only possible for encodings A1 and A2
            data = PCStoreValue();
        else
            data = R[t];
        MemU_unpriv[address,4] = data;
        if postindex then R[n] = offset_addr;

__instruction aarch32_STRBT_A
    __encoding aarch32_STRBT_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0100 x110xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);  postindex = TRUE;  add = (U == '1');
            register_form = FALSE;  imm32 = ZeroExtend(imm12, 32);
            if t == 15 || n == 15 || n == t then UNPREDICTABLE;

    __encoding aarch32_STRBT_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm5 7 +: 5
        __field type1 5 +: 2
        __field Rm 0 +: 4
        __opcode 'xxxx0110 x110xxxx xxxxxxxx xxx0xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);  m = UInt(Rm);  postindex = TRUE;  add = (U == '1');
            register_form = TRUE;  (shift_t, shift_n) = DecodeImmShift(type1, imm5);
            if t == 15 || n == 15 || n == t || m == 15 then UNPREDICTABLE;

    __encoding aarch32_STRBT_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm8 0 +: 8
        __opcode '11111000 0000xxxx xxxx1110 xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then UNDEFINED;
            t = UInt(Rt);  n = UInt(Rn);  postindex = FALSE;  add = TRUE;
            register_form = FALSE;  imm32 = ZeroExtend(imm8, 32);
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        if PSTATE.EL == EL2 then UNPREDICTABLE;               // Hyp mode
        EncodingSpecificOperations();
        offset = if register_form then Shift(R[m], shift_t, shift_n, PSTATE.C) else imm32;
        offset_addr = if add then (R[n] + offset) else (R[n] - offset);
        address = if postindex then R[n] else offset_addr;
        MemU_unpriv[address,1] = R[t]<7:0>;
        if postindex then R[n] = offset_addr;

__instruction aarch32_STRHT_A
    __encoding aarch32_STRHT_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm4H 8 +: 4
        __field imm4L 0 +: 4
        __opcode 'xxxx0000 x110xxxx xxxxxxxx 1011xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);  postindex = TRUE;  add = (U == '1');
            register_form = FALSE;  imm32 = ZeroExtend(imm4H:imm4L, 32);
            if t == 15 || n == 15 || n == t then UNPREDICTABLE;

    __encoding aarch32_STRHT_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0000 x010xxxx xxxxxxxx 1011xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);  m = UInt(Rm);  postindex = TRUE;  add = (U == '1');
            register_form = TRUE;
            if t == 15 || n == 15 || n == t || m == 15 then UNPREDICTABLE;

    __encoding aarch32_STRHT_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm8 0 +: 8
        __opcode '11111000 0010xxxx xxxx1110 xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then UNDEFINED;
            t = UInt(Rt);  n = UInt(Rn);  postindex = FALSE;  add = TRUE;
            register_form = FALSE;  imm32 = ZeroExtend(imm8, 32);
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        if PSTATE.EL == EL2 then UNPREDICTABLE;               // Hyp mode
        EncodingSpecificOperations();
        offset = if register_form then R[m] else imm32;
        offset_addr = if add then (R[n] + offset) else (R[n] - offset);
        address = if postindex then R[n] else offset_addr;
        MemU_unpriv[address,2] = R[t]<15:0>;
        if postindex then R[n] = offset_addr;
