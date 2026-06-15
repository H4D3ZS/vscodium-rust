__instruction aarch32_VRECPE_A
    __encoding aarch32_VRECPE_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field F 8 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx11 xxxx010x 0xx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if (size == '01' && !HaveFP16Ext()) || size IN {'00', '11'} then UNDEFINED;
            floating_point = (F == '1');
            case size of
                when '01' esize = 16; elements = 4;
                when '10' esize = 32; elements = 2;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VRECPE_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field F 8 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx11 xxxx010x 0xx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if (size == '01' && !HaveFP16Ext()) || size IN {'00', '11'} then UNDEFINED;
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            floating_point = (F == '1');
            case size of
                when '01' esize = 16; elements = 4;
                when '10' esize = 32; elements = 2;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                if floating_point then
                    Elem[D[d+r],e,esize] = FPRecipEstimate(Elem[D[m+r],e,esize], StandardFPSCRValue());
                else
                    Elem[D[d+r],e,esize] = UnsignedRecipEstimate(Elem[D[m+r],e,esize]);

__instruction aarch32_VRSQRTE_A
    __encoding aarch32_VRSQRTE_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field F 8 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx11 xxxx010x 1xx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if (size == '01' && !HaveFP16Ext()) || size IN {'00', '11'} then UNDEFINED;
            floating_point = (F == '1');
            case size of
                when '01' esize = 16; elements = 4;
                when '10' esize = 32; elements = 2;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VRSQRTE_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field F 8 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx11 xxxx010x 1xx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if (size == '01' && !HaveFP16Ext()) || size IN {'00', '11'} then UNDEFINED;
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            floating_point = (F == '1');
            case size of
                when '01' esize = 16; elements = 4;
                when '10' esize = 32; elements = 2;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                if floating_point then
                    Elem[D[d+r],e,esize] = FPRSqrtEstimate(Elem[D[m+r],e,esize], StandardFPSCRValue());
                else
                    Elem[D[d+r],e,esize] = UnsignedRSqrtEstimate(Elem[D[m+r],e,esize]);
