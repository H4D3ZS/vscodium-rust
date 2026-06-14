__instruction aarch32_VMOV_ss_A
    __encoding aarch32_VMOV_ss_T1A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field op 20 +: 1
        __field Rt2 16 +: 4
        __field Rt 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode 'xxxx1100 010xxxxx xxxx1010 00x1xxxx'
        __guard cond != '1111'
        __decode
            to_arm_registers = (op == '1');  t = UInt(Rt);  t2 = UInt(Rt2);  m = UInt(Vm:M);
            if t == 15 || t2 == 15 || m == 31 then UNPREDICTABLE;
            if to_arm_registers && t == t2 then UNPREDICTABLE;

    __encoding aarch32_VMOV_ss_T1A1_A
        __instruction_set T32
        __field op 20 +: 1
        __field Rt2 16 +: 4
        __field Rt 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101100 010xxxxx xxxx1010 00x1xxxx'
        __guard TRUE
        __decode
            to_arm_registers = (op == '1');  t = UInt(Rt);  t2 = UInt(Rt2);  m = UInt(Vm:M);
            if t == 15 || t2 == 15 || m == 31 then UNPREDICTABLE;
            if to_arm_registers && t == t2 then UNPREDICTABLE;

    __execute __conditional
        CheckVFPEnabled(TRUE);
        if to_arm_registers then
            R[t] = S[m];
            R[t2] = S[m+1];
        else
            S[m] = R[t];
            S[m+1] = R[t2];
