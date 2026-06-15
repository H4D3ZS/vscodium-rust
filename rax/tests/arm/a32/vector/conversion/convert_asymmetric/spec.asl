__instruction aarch32_VCVTA_asimd_A
    __encoding aarch32_VCVTA_asimd_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field RM 8 +: 2
        __field op 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx11 xxxx0010 xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if (size == '01' && !HaveFP16Ext()) || size IN {'00', '11'} then UNDEFINED;
            rounding = FPDecodeRM(RM);  unsigned = (op == '1');
            case size of
                when '01' esize = 16; elements = 4;
                when '10' esize = 32; elements = 2;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VCVTA_asimd_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field RM 8 +: 2
        __field op 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx11 xxxx0010 xxx0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if (size == '01' && !HaveFP16Ext()) || size IN {'00', '11'} then UNDEFINED;
            rounding = FPDecodeRM(RM);  unsigned = (op == '1');
            case size of
                when '01' esize = 16; elements = 4;
                when '10' esize = 32; elements = 2;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute
        CheckAdvSIMDEnabled();
        bits(esize) result;
        for r = 0 to regs-1
            for e = 0 to elements-1
                Elem[D[d+r],e,esize] = FPToFixed(Elem[D[m+r],e,esize], 0, unsigned,
                                                 StandardFPSCRValue(), rounding);

__instruction aarch32_VCVTA_vfp_A
    __encoding aarch32_VCVTA_vfp_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field RM 16 +: 2
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field op 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111110 1x111100 xxxx10xx x1x0xxxx'
        __guard TRUE
        __decode
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            rounding = FPDecodeRM(RM);  unsigned = (op == '0');
            d = UInt(Vd:D);
            case size of
                when '01' esize = 16; m = UInt(Vm:M);
                when '10' esize = 32; m = UInt(Vm:M);
                when '11' esize = 64; m = UInt(M:Vm);

    __encoding aarch32_VCVTA_vfp_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field RM 16 +: 2
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field op 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111110 1x111100 xxxx10xx x1x0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            rounding = FPDecodeRM(RM);  unsigned = (op == '0');
            d = UInt(Vd:D);
            case size of
                when '01' esize = 16; m = UInt(Vm:M);
                when '10' esize = 32; m = UInt(Vm:M);
                when '11' esize = 64; m = UInt(M:Vm);

    __execute
        CheckVFPEnabled(TRUE);
        case esize of
            when 16
                S[d] = FPToFixed(S[m]<15:0>, 0, unsigned, FPSCR, rounding);
            when 32
                S[d] = FPToFixed(S[m], 0, unsigned, FPSCR, rounding);
            when 64
                S[d] = FPToFixed(D[m], 0, unsigned, FPSCR, rounding);
