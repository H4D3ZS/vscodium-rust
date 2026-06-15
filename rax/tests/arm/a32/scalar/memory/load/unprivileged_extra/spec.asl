__instruction aarch32_LDRSHT_A
    __encoding aarch32_LDRSHT_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm4H 8 +: 4
        __field imm4L 0 +: 4
        __opcode 'xxxx0000 x111xxxx xxxxxxxx 1111xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);  postindex = TRUE;  add = (U == '1');
            register_form = FALSE;  imm32 = ZeroExtend(imm4H:imm4L, 32);
            if t == 15 || n == 15 || n == t then UNPREDICTABLE;

    __encoding aarch32_LDRSHT_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx0000 x011xxxx xxxxxxxx 1111xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);  m = UInt(Rm);  postindex = TRUE;  add = (U == '1');
            register_form = TRUE;
            if t == 15 || n == 15 || n == t || m == 15 then UNPREDICTABLE;

    __encoding aarch32_LDRSHT_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm8 0 +: 8
        __opcode '11111001 0011xxxx xxxx1110 xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "LDRSH (literal)";
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
        R[t] = SignExtend(data, 32);
