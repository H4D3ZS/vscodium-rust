__instruction aarch32_VABS_A
    __encoding aarch32_VABS_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field F 10 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx01 xxxx0x11 0xx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if F == '1' && ((size == '01' && !HaveFP16Ext()) || size == '00') then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            advsimd = TRUE;  floating_point = (F == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VABS_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode 'xxxx1110 1x110000 xxxx10xx 11x0xxxx'
        __guard cond != '1111'
        __decode
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && cond != '1110' then UNPREDICTABLE;
            advsimd = FALSE;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); m = UInt(M:Vm);

    __encoding aarch32_VABS_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field F 10 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx01 xxxx0x11 0xx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if F == '1' && ((size == '01' && !HaveFP16Ext()) || size == '00') then UNDEFINED;
            if F == '1' && size == '01' && InITBlock() then UNPREDICTABLE;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            advsimd = TRUE;  floating_point = (F == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VABS_T2_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101110 1x110000 xxxx10xx 11x0xxxx'
        __guard TRUE
        __decode
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            advsimd = FALSE;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDOrVFPEnabled(TRUE, advsimd);
        if advsimd then  // Advanced SIMD instruction
            for r = 0 to regs-1
                for e = 0 to elements-1
                    if floating_point then
                        Elem[D[d+r],e,esize] = FPAbs(Elem[D[m+r],e,esize]);
                    else
                        result = Abs(SInt(Elem[D[m+r],e,esize]));
                        Elem[D[d+r],e,esize] = result<esize-1:0>;
        else             // VFP instruction
            case esize of
                when 16 S[d] = Zeros(16) : FPAbs(S[m]<15:0>);
                when 32 S[d] = FPAbs(S[m]);
                when 64 D[d] = FPAbs(D[m]);

__instruction aarch32_VNEG_A
    __encoding aarch32_VNEG_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field F 10 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx01 xxxx0x11 1xx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if F == '1' && ((size == '01' && !HaveFP16Ext()) || size == '00') then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            advsimd = TRUE;  floating_point = (F == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VNEG_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode 'xxxx1110 1x110001 xxxx10xx 01x0xxxx'
        __guard cond != '1111'
        __decode
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && cond != '1110' then UNPREDICTABLE;
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            advsimd = FALSE;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); m = UInt(M:Vm);

    __encoding aarch32_VNEG_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field F 10 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx01 xxxx0x11 1xx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if F == '1' && ((size == '01' && !HaveFP16Ext()) || size == '00') then UNDEFINED;
            if F == '1' && size == '01' && InITBlock() then UNPREDICTABLE;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            advsimd = TRUE;  floating_point = (F == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VNEG_T2_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101110 1x110001 xxxx10xx 01x0xxxx'
        __guard TRUE
        __decode
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            advsimd = FALSE;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDOrVFPEnabled(TRUE, advsimd);
        if advsimd then  // Advanced SIMD instruction
            for r = 0 to regs-1
                for e = 0 to elements-1
                    if floating_point then
                        Elem[D[d+r],e,esize] = FPNeg(Elem[D[m+r],e,esize]);
                    else
                        result = -SInt(Elem[D[m+r],e,esize]);
                        Elem[D[d+r],e,esize] = result<esize-1:0>;
        else             // VFP instruction
            case esize of
                when 16 S[d] = Zeros(16) : FPNeg(S[m]<15:0>);
                when 32 S[d] = FPNeg(S[m]);
                when 64 D[d] = FPNeg(D[m]);
