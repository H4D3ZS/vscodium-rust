__instruction aarch32_VCVT_is_A
    __encoding aarch32_VCVT_is_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field op 7 +: 2
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx11 xxxx011x xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if (size == '01' && !HaveFP16Ext()) || size IN {'00', '11'} then UNDEFINED;
            to_integer = (op<1> == '1');  unsigned = (op<0> == '1');
            case size of
                when '01' esize = 16; elements = 4;
                when '10' esize = 32; elements = 2;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VCVT_is_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field op 7 +: 2
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx11 xxxx011x xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if (size == '01' && !HaveFP16Ext()) || size IN {'00', '11'} then UNDEFINED;
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            to_integer = (op<1> == '1');  unsigned = (op<0> == '1');
            case size of
                when '01' esize = 16; elements = 4;
                when '10' esize = 32; elements = 2;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        bits(esize) result;
        for r = 0 to regs-1
            for e = 0 to elements-1
                op1 = Elem[D[m+r],e,esize];
                if to_integer then
                    result = FPToFixed(op1, 0, unsigned, StandardFPSCRValue(), FPRounding_ZERO);
                else
                    result = FixedToFP(op1, 0, unsigned, StandardFPSCRValue(), FPRounding_TIEEVEN);
                Elem[D[d+r],e,esize] = result;
