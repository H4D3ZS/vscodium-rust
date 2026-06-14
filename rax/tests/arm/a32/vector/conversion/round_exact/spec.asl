__instruction aarch32_VRINTX_asimd_A
    __encoding aarch32_VRINTX_asimd_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx10 xxxx0100 1xx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if (size == '01' && !HaveFP16Ext()) || size IN {'00', '11'} then UNDEFINED;
            rounding = FPRounding_TIEEVEN;  exact = TRUE;
            case size of
                when '01' esize = 16; elements = 4;
                when '10' esize = 32; elements = 2;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VRINTX_asimd_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx10 xxxx0100 1xx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if (size == '01' && !HaveFP16Ext()) || size IN {'00', '11'} then UNDEFINED;
            rounding = FPRounding_TIEEVEN;  exact = TRUE;
            case size of
                when '01' esize = 16; elements = 4;
                when '10' esize = 32; elements = 2;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;
            if InITBlock() then UNPREDICTABLE;

    __execute
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                op1 = Elem[D[m+r],e,esize];
                result = FPRoundInt(op1, StandardFPSCRValue(), rounding, exact);
                Elem[D[d+r],e,esize] = result;

__instruction aarch32_VRINTX_vfp_A
    __encoding aarch32_VRINTX_vfp_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode 'xxxx1110 1x110111 xxxx10xx 01x0xxxx'
        __guard cond != '1111'
        __decode
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && cond != '1110' then UNPREDICTABLE;
            exact = TRUE;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); m = UInt(M:Vm);

    __encoding aarch32_VRINTX_vfp_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101110 1x110111 xxxx10xx 01x0xxxx'
        __guard TRUE
        __decode
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            exact = TRUE;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); m = UInt(M:Vm);

    __execute __conditional
        CheckVFPEnabled(TRUE);
        rounding = FPRoundingMode(FPSCR);
        case esize of
            when 16
                S[d] = Zeros(16) : FPRoundInt(S[m]<15:0>, FPSCR, rounding, exact);
            when 32
                S[d] = FPRoundInt(S[m], FPSCR, rounding, exact);
            when 64
                D[d] = FPRoundInt(D[m], FPSCR, rounding, exact);
