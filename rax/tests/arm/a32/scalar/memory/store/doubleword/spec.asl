__instruction aarch32_STRD_i_A
    __encoding aarch32_STRD_i_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field P 24 +: 1
        __field U 23 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm4H 8 +: 4
        __field imm4L 0 +: 4
        __opcode 'xxxx000x x1x0xxxx xxxxxxxx 1111xxxx'
        __guard cond != '1111'
        __decode
            if Rt<0> == '1' then UNPREDICTABLE;
            t = UInt(Rt);  t2 = t+1;  n = UInt(Rn);  imm32 = ZeroExtend(imm4H:imm4L, 32);
            index = (P == '1');  add = (U == '1');  wback = (P == '0') || (W == '1');
            if P == '0' && W == '1' then UNPREDICTABLE;
            if wback && (n == 15 || n == t || n == t2) then UNPREDICTABLE;
            if t2 == 15 then UNPREDICTABLE;

    __encoding aarch32_STRD_i_T1_A
        __instruction_set T32
        __field P 24 +: 1
        __field U 23 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field Rt2 8 +: 4
        __field imm8 0 +: 8
        __opcode '1110100x x1x0xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if P == '0' && W == '0' then SEE "Related encodings";
            t = UInt(Rt);  t2 = UInt(Rt2);  n = UInt(Rn);  imm32 = ZeroExtend(imm8:'00', 32);
            index = (P == '1');  add = (U == '1');  wback = (W == '1');
            if wback && (n == t || n == t2) then UNPREDICTABLE;
            if n == 15 || t == 15 || t2 == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        offset_addr = if add then (R[n] + imm32) else (R[n] - imm32);
        address = if index then offset_addr else R[n];
        if address == Align(address, 8) then
            bits(64) data;
            if BigEndian() then
                data<63:32> = R[t];
                data<31:0> = R[t2];
            else
                data<31:0> = R[t];
                data<63:32> = R[t2];
            MemA[address,8] = data;
        else
            MemA[address,4] = R[t];
            MemA[address+4,4] = R[t2];
        if wback then R[n] = offset_addr;

__instruction aarch32_STRD_r_A
    __encoding aarch32_STRD_r_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field P 24 +: 1
        __field U 23 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field Rm 0 +: 4
        __opcode 'xxxx000x x0x0xxxx xxxxxxxx 1111xxxx'
        __guard cond != '1111'
        __decode
            if Rt<0> == '1' then UNPREDICTABLE;
            t = UInt(Rt);  t2 = t+1;  n = UInt(Rn);  m = UInt(Rm);
            index = (P == '1');  add = (U == '1');  wback = (P == '0') || (W == '1');
            if P == '0' && W == '1' then UNPREDICTABLE;
            if t2 == 15 || m == 15 then UNPREDICTABLE;
            if wback && (n == 15 || n == t || n == t2) then UNPREDICTABLE;

    __execute __conditional
        offset_addr = if add then (R[n] + R[m]) else (R[n] - R[m]);
        address = if index then offset_addr else R[n];
        if address == Align(address, 8) then
            bits(64) data;
            if BigEndian() then
                data<63:32> = R[t];
                data<31:0> = R[t2];
            else
                data<31:0> = R[t];
                data<63:32> = R[t2];
            MemA[address,8] = data;
        else
            MemA[address,4] = R[t];
            MemA[address+4,4] = R[t2];
        if wback then R[n] = offset_addr;
