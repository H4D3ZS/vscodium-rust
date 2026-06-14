__instruction aarch32_VMOV_h_A
    __encoding aarch32_VMOV_h_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field op 20 +: 1
        __field Vn 16 +: 4
        __field Rt 12 +: 4
        __field N 7 +: 1
        __opcode 'xxxx1110 000xxxxx xxxx1001 xxx1xxxx'
        __guard cond != '1111'
        __decode
            if !HaveFP16Ext() then UNDEFINED;
            if cond != '1110' then UNPREDICTABLE;
            to_arm_register = (op == '1');  t = UInt(Rt);  n = UInt(Vn:N);
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __encoding aarch32_VMOV_h_T1_A
        __instruction_set T32
        __field op 20 +: 1
        __field Vn 16 +: 4
        __field Rt 12 +: 4
        __field N 7 +: 1
        __opcode '11101110 000xxxxx xxxx1001 xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;
            if InITBlock() then UNPREDICTABLE;
            to_arm_register = (op == '1');  t = UInt(Rt);  n = UInt(Vn:N);
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        CheckVFPEnabled(TRUE);
        if to_arm_register then
            R[t] = Zeros(16) : S[n]<15:0>;
        else
            S[n] = Zeros(16) : R[t]<15:0>;
