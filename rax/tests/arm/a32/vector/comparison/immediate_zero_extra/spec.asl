__instruction aarch32_VCLT_i_A
    __encoding aarch32_VCLT_i_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field F 10 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx01 xxxx0x10 0xx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if F == '1' && ((size == '01' && !HaveFP16Ext()) || size == '00') then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            floating_point = (F == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VCLT_i_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field F 10 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx01 xxxx0x10 0xx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if F == '1' && ((size == '01' && !HaveFP16Ext()) || size == '00') then UNDEFINED;
            if F == '1' && size == '01' && InITBlock() then UNPREDICTABLE;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            floating_point = (F == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                if floating_point then
                    bits(esize) zero = FPZero('0');
                    test_passed = FPCompareGT(zero, Elem[D[m+r],e,esize], StandardFPSCRValue());
                else
                    test_passed = (SInt(Elem[D[m+r],e,esize]) < 0);
                Elem[D[d+r],e,esize] = if test_passed then Ones(esize) else Zeros(esize);
