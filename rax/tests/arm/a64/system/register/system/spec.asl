__instruction aarch64_system_register_system
    __encoding aarch64_system_register_system
        __instruction_set A64
        __field L 21 +: 1
        __field o0 19 +: 1
        __field op1 16 +: 3
        __field CRn 12 +: 4
        __field CRm 8 +: 4
        __field op2 5 +: 3
        __field Rt 0 +: 5
        __opcode '11010101 00x1xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            AArch64.CheckSystemAccess('1':o0, op1, CRn, CRm, op2, Rt, L);

            integer t = UInt(Rt);

            integer sys_op0 = 2 + UInt(o0);
            integer sys_op1 = UInt(op1);
            integer sys_op2 = UInt(op2);
            integer sys_crn = UInt(CRn);
            integer sys_crm = UInt(CRm);
            boolean read = (L == '1');

    __execute
        if read then
            X[t] = AArch64.SysRegRead(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2);
        else
            AArch64.SysRegWrite(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2, X[t]);
