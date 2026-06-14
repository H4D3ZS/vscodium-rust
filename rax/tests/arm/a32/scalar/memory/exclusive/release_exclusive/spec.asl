__instruction aarch32_STLEX_A
    __encoding aarch32_STLEX_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rt 0 +: 4
        __opcode 'xxxx0001 1000xxxx xxxxxx10 1001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  t = UInt(Rt);  n = UInt(Rn);
            if d == 15 || t == 15 || n == 15 then UNPREDICTABLE;
            if d == n || d == t then UNPREDICTABLE;

    __encoding aarch32_STLEX_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field Rd 0 +: 4
        __opcode '11101000 1100xxxx xxxxxxxx 1110xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  t = UInt(Rt);  n = UInt(Rn);
            if d == 15 || t == 15 || n == 15 then UNPREDICTABLE;
            if d == n || d == t then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        if AArch32.ExclusiveMonitorsPass(address,4) then
            MemO[address, 4] = R[t];
            R[d] = ZeroExtend('0');
        else
            R[d] = ZeroExtend('1');

__instruction aarch32_STLEXB_A
    __encoding aarch32_STLEXB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rt 0 +: 4
        __opcode 'xxxx0001 1100xxxx xxxxxx10 1001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  t = UInt(Rt);  n = UInt(Rn);
            if d == 15 || t == 15 || n == 15 then UNPREDICTABLE;
            if d == n || d == t then UNPREDICTABLE;

    __encoding aarch32_STLEXB_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field Rd 0 +: 4
        __opcode '11101000 1100xxxx xxxxxxxx 1100xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  t = UInt(Rt);  n = UInt(Rn);
            if d == 15 || t == 15 || n == 15 then UNPREDICTABLE;
            if d == n || d == t then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        if AArch32.ExclusiveMonitorsPass(address,1) then
            MemO[address, 1] = R[t]<7:0>;
            R[d] = ZeroExtend('0');
        else
            R[d] = ZeroExtend('1');

__instruction aarch32_STLEXH_A
    __encoding aarch32_STLEXH_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rt 0 +: 4
        __opcode 'xxxx0001 1110xxxx xxxxxx10 1001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  t = UInt(Rt);  n = UInt(Rn);
            if d == 15 || t == 15 || n == 15 then UNPREDICTABLE;
            if d == n || d == t then UNPREDICTABLE;

    __encoding aarch32_STLEXH_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field Rd 0 +: 4
        __opcode '11101000 1100xxxx xxxxxxxx 1101xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  t = UInt(Rt);  n = UInt(Rn);
            if d == 15 || t == 15 || n == 15 then UNPREDICTABLE;
            if d == n || d == t then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        if AArch32.ExclusiveMonitorsPass(address,2) then
            MemO[address, 2] = R[t]<15:0>;
            R[d] = ZeroExtend('0');
        else
            R[d] = ZeroExtend('1');

__instruction aarch32_STLEXD_A
    __encoding aarch32_STLEXD_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rt 0 +: 4
        __opcode 'xxxx0001 1010xxxx xxxxxx10 1001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  t = UInt(Rt);  t2 = t+1;  n = UInt(Rn);
            if d == 15 || Rt<0> == '1' || t2 == 15 || n == 15 then UNPREDICTABLE;
            if d == n || d == t || d == t2 then UNPREDICTABLE;

    __encoding aarch32_STLEXD_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field Rt2 8 +: 4
        __field Rd 0 +: 4
        __opcode '11101000 1100xxxx xxxxxxxx 1111xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  t = UInt(Rt);  t2 = UInt(Rt2);  n = UInt(Rn);
            if d == 15 || t == 15 || t2 == 15 || n == 15 then UNPREDICTABLE;
            if d == n || d == t || d == t2 then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        // Create doubleword to store such that R[t] will be stored at address and R[t2] at address+4.
        value = if BigEndian() then R[t]:R[t2] else R[t2]:R[t];
        if AArch32.ExclusiveMonitorsPass(address, 8) then
            MemO[address, 8] = value;
            R[d] = ZeroExtend('0');
        else
            R[d] = ZeroExtend('1');
