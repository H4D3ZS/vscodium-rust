__instruction aarch32_VCVT_xs_A
    __encoding aarch32_VCVT_xs_A1_A
        __instruction_set A32
        __field U 24 +: 1
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field op 8 +: 2
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 1xxxxxxx xxxx11xx 0xx1xxxx'
        __guard TRUE
        __decode
            if imm6 == '000xxx' then SEE "Related encodings";
            if op<1> == '0' && !HaveFP16Ext() then UNDEFINED;
            if op<1> == '0' && imm6 == '10xxxx' then UNDEFINED;
            if imm6 == '0xxxxx' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            to_fixed = (op<0> == '1');  frac_bits = 64 - UInt(imm6);
            unsigned = (U == '1');
            case op<1> of
                 when '0' esize = 16; elements = 4;
                 when '1' esize = 32; elements = 2;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VCVT_xs_T1_A
        __instruction_set T32
        __field U 28 +: 1
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field op 8 +: 2
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 1xxxxxxx xxxx11xx 0xx1xxxx'
        __guard TRUE
        __decode
            if imm6 == '000xxx' then SEE "Related encodings";
            if op<1> == '0' && !HaveFP16Ext() then UNDEFINED;
            if op<1> == '0' && imm6 == '10xxxx' then UNDEFINED;
            if imm6 == '0xxxxx' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            to_fixed = (op<0> == '1');  frac_bits = 64 - UInt(imm6);
            unsigned = (U == '1');
            case op<1> of
                 when '0' esize = 16; elements = 4;
                 when '1' esize = 32; elements = 2;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        bits(esize) result;
        for r = 0 to regs-1
            for e = 0 to elements-1
                op1 = Elem[D[m+r],e,esize];
                if to_fixed then
                    result = FPToFixed(op1, frac_bits, unsigned, StandardFPSCRValue(),
                                       FPRounding_ZERO);
                else
                    result = FixedToFP(op1, frac_bits, unsigned, StandardFPSCRValue(),
                                       FPRounding_TIEEVEN);
                Elem[D[d+r],e,esize] = result;
