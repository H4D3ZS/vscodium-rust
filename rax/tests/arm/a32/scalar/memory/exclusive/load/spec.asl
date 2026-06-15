__instruction aarch32_LDREX_A
    __encoding aarch32_LDREX_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode 'xxxx0001 1001xxxx xxxxxx11 1001xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);  imm32 = Zeros(32); // Zero offset
            if t == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_LDREX_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field imm8 0 +: 8
        __opcode '11101000 0101xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            t = UInt(Rt);  n = UInt(Rn);  imm32 = ZeroExtend(imm8:'00', 32);
            if t == 15 || n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        address = R[n] + imm32;
        AArch32.SetExclusiveMonitors(address,4);
        R[t] = MemA[address,4];

__instruction aarch32_LDREXB_A
    __encoding aarch32_LDREXB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode 'xxxx0001 1101xxxx xxxxxx11 1001xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_LDREXB_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode '11101000 1101xxxx xxxxxxxx 0100xxxx'
        __guard TRUE
        __decode
            t = UInt(Rt);  n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        address = R[n];
        AArch32.SetExclusiveMonitors(address,1);
        R[t] = ZeroExtend(MemA[address,1], 32);

__instruction aarch32_LDREXH_A
    __encoding aarch32_LDREXH_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode 'xxxx0001 1111xxxx xxxxxx11 1001xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_LDREXH_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode '11101000 1101xxxx xxxxxxxx 0101xxxx'
        __guard TRUE
        __decode
            t = UInt(Rt);  n = UInt(Rn);
            if t == 15 || n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        address = R[n];
        AArch32.SetExclusiveMonitors(address,2);
        R[t] = ZeroExtend(MemA[address,2], 32);

__instruction aarch32_LDREXD_A
    __encoding aarch32_LDREXD_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __opcode 'xxxx0001 1011xxxx xxxxxx11 1001xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  t2 = t + 1;  n = UInt(Rn);
            if Rt<0> == '1' || t2 == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_LDREXD_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field Rt 12 +: 4
        __field Rt2 8 +: 4
        __opcode '11101000 1101xxxx xxxxxxxx 0111xxxx'
        __guard TRUE
        __decode
            t = UInt(Rt);  t2 = UInt(Rt2);  n = UInt(Rn);
            if t == 15 || t2 == 15 || t == t2 || n == 15 then UNPREDICTABLE;
            // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        address = R[n];
        AArch32.SetExclusiveMonitors(address,8);
        value = MemA[address,8];
        // Extract words from 64-bit loaded value such that R[t] is
        // loaded from address and R[t2] from address+4.
        R[t]  = if BigEndian() then value<63:32> else value<31:0>;
        R[t2] = if BigEndian() then value<31:0> else value<63:32>;
