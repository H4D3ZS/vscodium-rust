__instruction aarch32_VACGE_A
    __encoding aarch32_VACGE_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field op 21 +: 1
        __field sz 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 0x1xxxxx xxxx1110 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if sz == '1' && !HaveFP16Ext() then UNDEFINED;
            or_equal = (op == '0');
            case sz of
                when '0' esize = 32; elements = 2;
                when '1' esize = 16; elements = 4;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VACGE_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field op 21 +: 1
        __field sz 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 0x1xxxxx xxxx1110 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if sz == '1' && !HaveFP16Ext() then UNDEFINED;
            if sz == '1' && InITBlock() then UNPREDICTABLE;
            or_equal = (op == '0');
            case sz of
                when '0' esize = 32; elements = 2;
                when '1' esize = 16; elements = 4;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                op1 = FPAbs(Elem[D[n+r],e,esize]);  op2 = FPAbs(Elem[D[m+r],e,esize]);
                if or_equal then
                    test_passed = FPCompareGE(op1, op2, StandardFPSCRValue());
                else
                    test_passed = FPCompareGT(op1, op2, StandardFPSCRValue());
                Elem[D[d+r],e,esize] = if test_passed then Ones(esize) else Zeros(esize);
