__instruction aarch32_VMOV_s_A
    __encoding aarch32_VMOV_s_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field op 20 +: 1
        __field Vn 16 +: 4
        __field Rt 12 +: 4
        __field N 7 +: 1
        __opcode 'xxxx1110 000xxxxx xxxx1010 xxx1xxxx'
        __guard cond != '1111'
        __decode
            to_arm_register = (op == '1');  t = UInt(Rt);  n = UInt(Vn:N);
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __encoding aarch32_VMOV_s_T1_A
        __instruction_set T32
        __field op 20 +: 1
        __field Vn 16 +: 4
        __field Rt 12 +: 4
        __field N 7 +: 1
        __opcode '11101110 000xxxxx xxxx1010 xxx1xxxx'
        __guard TRUE
        __decode
            to_arm_register = (op == '1');  t = UInt(Rt);  n = UInt(Vn:N);
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        CheckVFPEnabled(TRUE);
        if to_arm_register then
            R[t] = S[n];
        else
            S[n] = R[t];
