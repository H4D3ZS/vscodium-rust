__instruction aarch32_LDAEX_A
    __encoding aarch32_LDAEX_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode 'xxxx0001 1001xxxx xxxxxx10 1001xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_LDAEX_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode '11101000 1101xxxx xxxxxxxx 1110xxxx'
        __guard TRUE
        __decode
            t = UInt(Rt);  n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        AArch32.SetExclusiveMonitors(address, 4);
        R[t] = MemO[address, 4];

__instruction aarch32_LDAEXB_A
    __encoding aarch32_LDAEXB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode 'xxxx0001 1101xxxx xxxxxx10 1001xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_LDAEXB_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode '11101000 1101xxxx xxxxxxxx 1100xxxx'
        __guard TRUE
        __decode
            t = UInt(Rt);  n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        AArch32.SetExclusiveMonitors(address, 1);
        R[t] = ZeroExtend(MemO[address, 1], 32);

__instruction aarch32_LDAEXH_A
    __encoding aarch32_LDAEXH_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode 'xxxx0001 1111xxxx xxxxxx10 1001xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_LDAEXH_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode '11101000 1101xxxx xxxxxxxx 1101xxxx'
        __guard TRUE
        __decode
            t = UInt(Rt);  n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        AArch32.SetExclusiveMonitors(address, 2);
        R[t] = ZeroExtend(MemO[address, 2], 32);

__instruction aarch32_LDAEXD_A
    __encoding aarch32_LDAEXD_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode 'xxxx0001 1011xxxx xxxxxx10 1001xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  t2 = t + 1;  n = UInt(Rn);
            if Rt<0> == '1' || t2 == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_LDAEXD_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field Rt2 8 +: 4
        __opcode '11101000 1101xxxx xxxxxxxx 1111xxxx'
        __guard TRUE
        __decode
            t = UInt(Rt);  t2 = UInt(Rt2);  n = UInt(Rn);
            if t == 15 || t2 == 15 || t == t2 || n == 15 then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        AArch32.SetExclusiveMonitors(address, 8);
        value = MemO[address, 8];
        // Extract words from 64-bit loaded value such that R[t] is
        // loaded from address and R[t2] from address+4.
        R[t]  = if BigEndian() then value<63:32> else value<31:0>;
        R[t2] = if BigEndian() then value<31:0>  else value<63:32>;
