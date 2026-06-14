__instruction aarch64_system_sysops
    __encoding aarch64_system_sysops
        __instruction_set A64
        __field L 21 +: 1
        __field op1 16 +: 3
        __field CRn 12 +: 4
        __field CRm 8 +: 4
        __field op2 5 +: 3
        __field Rt 0 +: 5
        __opcode '11010101 00x01xxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            AArch64.CheckSystemAccess('01', op1, CRn, CRm, op2, Rt, L);

            integer t = UInt(Rt);

            integer sys_op0 = 1;
            integer sys_op1 = UInt(op1);
            integer sys_op2 = UInt(op2);
            integer sys_crn = UInt(CRn);
            integer sys_crm = UInt(CRm);
            boolean has_result = (L == '1');

    __execute
        if has_result then
            X[t] = AArch64.SysInstrWithResult(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2);
        else
            AArch64.SysInstr(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2, X[t]);
