__instruction aarch32_STREX_A
    __encoding aarch32_STREX_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rt 0 +: 4
        __opcode 'xxxx0001 1000xxxx xxxxxx11 1001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  t = UInt(Rt);  n = UInt(Rn);  imm32 = Zeros(32); // Zero offset
            if d == 15 || t == 15 || n == 15 then UNPREDICTABLE;
            if d == n || d == t then UNPREDICTABLE;

    __encoding aarch32_STREX_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field Rd 8 +: 4
        __field imm8 0 +: 8
        __opcode '11101000 0100xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  t = UInt(Rt);  n = UInt(Rn);  imm32 = ZeroExtend(imm8:'00', 32);
            if d == 15 || t == 15 || n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13
            if d == n || d == t then UNPREDICTABLE;

    __execute __conditional
        address = R[n] + imm32;
        if AArch32.ExclusiveMonitorsPass(address,4) then
            MemA[address,4] = R[t];
            R[d] = ZeroExtend('0');
        else
            R[d] = ZeroExtend('1');

__instruction aarch32_STREXB_A
    __encoding aarch32_STREXB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rt 0 +: 4
        __opcode 'xxxx0001 1100xxxx xxxxxx11 1001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  t = UInt(Rt);  n = UInt(Rn);
            if d == 15 || t == 15 || n == 15 then UNPREDICTABLE;
            if d == n || d == t then UNPREDICTABLE;

    __encoding aarch32_STREXB_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field Rd 0 +: 4
        __opcode '11101000 1100xxxx xxxxxxxx 0100xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  t = UInt(Rt);  n = UInt(Rn);
            if d == 15 || t == 15 || n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13
            if d == n || d == t then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        if AArch32.ExclusiveMonitorsPass(address,1) then
            MemA[address,1] = R[t]<7:0>;
            R[d] = ZeroExtend('0');
        else
            R[d] = ZeroExtend('1');

__instruction aarch32_STREXH_A
    __encoding aarch32_STREXH_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rt 0 +: 4
        __opcode 'xxxx0001 1110xxxx xxxxxx11 1001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  t = UInt(Rt);  n = UInt(Rn);
            if d == 15 || t == 15 || n == 15 then UNPREDICTABLE;
            if d == n || d == t then UNPREDICTABLE;

    __encoding aarch32_STREXH_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field Rd 0 +: 4
        __opcode '11101000 1100xxxx xxxxxxxx 0101xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  t = UInt(Rt);  n = UInt(Rn);
            if d == 15 || t == 15 || n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13
            if d == n || d == t then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        if AArch32.ExclusiveMonitorsPass(address,2) then
            MemA[address,2] = R[t]<15:0>;
            R[d] = ZeroExtend('0');
        else
            R[d] = ZeroExtend('1');

__instruction aarch32_STREXD_A
    __encoding aarch32_STREXD_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field Rt 0 +: 4
        __opcode 'xxxx0001 1010xxxx xxxxxx11 1001xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  t = UInt(Rt);  t2 = t+1;  n = UInt(Rn);
            if d == 15 || Rt<0> == '1' || t2 == 15 || n == 15 then UNPREDICTABLE;
            if d == n || d == t || d == t2 then UNPREDICTABLE;

    __encoding aarch32_STREXD_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field Rt2 8 +: 4
        __field Rd 0 +: 4
        __opcode '11101000 1100xxxx xxxxxxxx 0111xxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  t = UInt(Rt);  t2 = UInt(Rt2);  n = UInt(Rn);
            if d == 15 || t == 15 || t2 == 15 || n == 15 then UNPREDICTABLE;
            // Armv8-A removes UNPREDICTABLE for R13
            if d == n || d == t || d == t2 then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        // Create doubleword to store such that R[t] will be stored at address and R[t2] at address+4.
        value = if BigEndian() then R[t]:R[t2] else R[t2]:R[t];
        if AArch32.ExclusiveMonitorsPass(address,8) then
            MemA[address,8] = value;  R[d] = ZeroExtend('0');
        else
            R[d] = ZeroExtend('1');
