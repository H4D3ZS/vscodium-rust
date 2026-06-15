__instruction aarch32_MRC_A
    __encoding aarch32_MRC_T1A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field opc1 21 +: 3
        __field CRn 16 +: 4
        __field Rt 12 +: 4
        __field coproc 9 +: 0
        __field opc2 5 +: 3
        __field CRm 0 +: 4
        __opcode 'xxxx1110 xxx1xxxx xxxx111x xxx1xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  cp = if coproc<0> == '0' then 14 else 15;
            // Armv8-A removes UNPREDICTABLE for R13

    __encoding aarch32_MRC_T1A1_A
        __instruction_set T32
        __field opc1 21 +: 3
        __field CRn 16 +: 4
        __field Rt 12 +: 4
        __field coproc 9 +: 0
        __field opc2 5 +: 3
        __field CRm 0 +: 4
        __opcode '11101110 xxx1xxxx xxxx111x xxx1xxxx'
        __guard TRUE
        __decode
            t = UInt(Rt);  cp = if coproc<0> == '0' then 14 else 15;
            // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        bits(32) value = AArch32.SysRegRead(cp, ThisInstr());
        if t != 15 then
            R[t] = value;
        elsif AArch32.SysRegReadCanWriteAPSR(cp, ThisInstr()) then
            PSTATE.<N,Z,C,V> = value<31:28>;
            // value<27:0> are not used.
        else
            PSTATE.<N,Z,C,V> = bits(4) UNKNOWN;

__instruction aarch32_MCR_A
    __encoding aarch32_MCR_T1A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field opc1 21 +: 3
        __field CRn 16 +: 4
        __field Rt 12 +: 4
        __field coproc 9 +: 0
        __field opc2 5 +: 3
        __field CRm 0 +: 4
        __opcode 'xxxx1110 xxx0xxxx xxxx111x xxx1xxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  cp = if coproc<0> == '0' then 14 else 15;
            if t == 15  then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __encoding aarch32_MCR_T1A1_A
        __instruction_set T32
        __field opc1 21 +: 3
        __field CRn 16 +: 4
        __field Rt 12 +: 4
        __field coproc 9 +: 0
        __field opc2 5 +: 3
        __field CRm 0 +: 4
        __opcode '11101110 xxx0xxxx xxxx111x xxx1xxxx'
        __guard TRUE
        __decode
            t = UInt(Rt);  cp = if coproc<0> == '0' then 14 else 15;
            if t == 15  then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        AArch32.SysRegWrite(cp, ThisInstr(), R[t]);

__instruction aarch32_MRRC_A
    __encoding aarch32_MRRC_T1A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rt2 16 +: 4
        __field Rt 12 +: 4
        __field coproc 9 +: 0
        __field opc1 4 +: 4
        __field CRm 0 +: 4
        __opcode 'xxxx1100 0101xxxx xxxx111x xxxxxxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  t2 = UInt(Rt2);  cp = if coproc<0> == '0' then 14 else 15;
            if t == 15 || t2 == 15 || t == t2 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __encoding aarch32_MRRC_T1A1_A
        __instruction_set T32
        __field Rt2 16 +: 4
        __field Rt 12 +: 4
        __field coproc 9 +: 0
        __field opc1 4 +: 4
        __field CRm 0 +: 4
        __opcode '11101100 0101xxxx xxxx111x xxxxxxxx'
        __guard TRUE
        __decode
            t = UInt(Rt);  t2 = UInt(Rt2);  cp = if coproc<0> == '0' then 14 else 15;
            if t == 15 || t2 == 15 || t == t2 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        value = AArch32.SysRegRead64(cp, ThisInstr());
        R[t] = value<31:0>;
        R[t2] = value<63:32>;

__instruction aarch32_MCRR_A
    __encoding aarch32_MCRR_T1A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field Rt2 16 +: 4
        __field Rt 12 +: 4
        __field coproc 9 +: 0
        __field opc1 4 +: 4
        __field CRm 0 +: 4
        __opcode 'xxxx1100 0100xxxx xxxx111x xxxxxxxx'
        __guard cond != '1111'
        __decode
            t = UInt(Rt);  t2 = UInt(Rt2);  cp = if coproc<0> == '0' then 14 else 15;
            if t == 15 || t2 == 15 then UNPREDICTABLE;
            // Armv8-A removes UNPREDICTABLE for R13

    __encoding aarch32_MCRR_T1A1_A
        __instruction_set T32
        __field Rt2 16 +: 4
        __field Rt 12 +: 4
        __field coproc 9 +: 0
        __field opc1 4 +: 4
        __field CRm 0 +: 4
        __opcode '11101100 0100xxxx xxxx111x xxxxxxxx'
        __guard TRUE
        __decode
            t = UInt(Rt);  t2 = UInt(Rt2);  cp = if coproc<0> == '0' then 14 else 15;
            if t == 15 || t2 == 15 then UNPREDICTABLE;
            // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        value = R[t2]:R[t];
        AArch32.SysRegWrite64(cp, ThisInstr(), value);
