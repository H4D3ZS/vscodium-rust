__instruction aarch32_VRINTA_asimd_A
    __encoding aarch32_VRINTA_asimd_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field op 7 +: 3
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx10 xxxx0111 1xx0xxxx'
        __guard TRUE
        __decode
            if op<2> != op<0> then SEE "Related encodings";
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if (size == '01' && !HaveFP16Ext()) || size IN {'00', '11'} then UNDEFINED;
            // Rounding encoded differently from other VCVT and VRINT instructions
            rounding = FPDecodeRM(op<2>:NOT(op<1>));  exact = FALSE;
            case size of
                when '01' esize = 16; elements = 4;
                when '10' esize = 32; elements = 2;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VRINTA_asimd_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field op 7 +: 3
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx10 xxxx0111 1xx0xxxx'
        __guard TRUE
        __decode
            if op<2> != op<0> then SEE "Related encodings";
            if InITBlock() then UNPREDICTABLE;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if (size == '01' && !HaveFP16Ext()) || size IN {'00', '11'} then UNDEFINED;
            // Rounding encoded differently from other VCVT and VRINT instructions
            rounding = FPDecodeRM(op<2>:NOT(op<1>));  exact = FALSE;
            case size of
                when '01' esize = 16; elements = 4;
                when '10' esize = 32; elements = 2;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                op1 = Elem[D[m+r],e,esize];
                result = FPRoundInt(op1, StandardFPSCRValue(), rounding, exact);
                Elem[D[d+r],e,esize] = result;

__instruction aarch32_VRINTA_vfp_A
    __encoding aarch32_VRINTA_vfp_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field RM 16 +: 2
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111110 1x111001 xxxx10xx 01x0xxxx'
        __guard TRUE
        __decode
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            rounding = FPDecodeRM(RM);  exact = FALSE;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); m = UInt(M:Vm);

    __encoding aarch32_VRINTA_vfp_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field RM 16 +: 2
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111110 1x111001 xxxx10xx 01x0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            rounding = FPDecodeRM(RM);  exact = FALSE;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); m = UInt(M:Vm);

    __execute
        CheckVFPEnabled(TRUE);
        case esize of
            when 16
                S[d] = Zeros(16) : FPRoundInt(S[m]<15:0>, FPSCR, rounding, exact);
            when 32
                S[d] = FPRoundInt(S[m], FPSCR, rounding, exact);
            when 64
                D[d] = FPRoundInt(D[m], FPSCR, rounding, exact);
