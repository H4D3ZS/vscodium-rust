__instruction aarch32_VADD_f_A
    __encoding aarch32_VADD_f_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field sz 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 0x0xxxxx xxxx1101 xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if sz == '1' && !HaveFP16Ext() then UNDEFINED;
            advsimd = TRUE;
            case sz of
                when '0' esize = 32; elements = 2;
                when '1' esize = 16; elements = 4;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VADD_f_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode 'xxxx1110 0x11xxxx xxxx10xx x0x0xxxx'
        __guard cond != '1111'
        __decode
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && cond != '1110' then UNPREDICTABLE;
            advsimd = FALSE;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __encoding aarch32_VADD_f_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field sz 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 0x0xxxxx xxxx1101 xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if sz == '1' && !HaveFP16Ext() then UNDEFINED;
            if sz == '1' && InITBlock() then UNPREDICTABLE;
            advsimd = TRUE;
            case sz of
                when '0' esize = 32; elements = 2;
                when '1' esize = 16; elements = 4;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VADD_f_T2_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101110 0x11xxxx xxxx10xx x0x0xxxx'
        __guard TRUE
        __decode
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            advsimd = FALSE;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDOrVFPEnabled(TRUE, advsimd);
        if advsimd then  // Advanced SIMD instruction
            for r = 0 to regs-1
                for e = 0 to elements-1
                    Elem[D[d+r],e,esize] = FPAdd(Elem[D[n+r],e,esize], Elem[D[m+r],e,esize],
                                                 StandardFPSCRValue());
        else             // VFP instruction
            case esize of
                when 16
                    S[d] = Zeros(16) : FPAdd(S[n]<15:0>, S[m]<15:0>, FPSCR);
                when 32
                    S[d] = FPAdd(S[n], S[m], FPSCR);
                when 64
                    D[d] = FPAdd(D[n], D[m], FPSCR);

__instruction aarch32_VSUB_f_A
    __encoding aarch32_VSUB_f_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field sz 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 0x1xxxxx xxxx1101 xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if sz == '1' && !HaveFP16Ext() then UNDEFINED;
            advsimd = TRUE;
            case sz of
                when '0' esize = 32; elements = 2;
                when '1' esize = 16; elements = 4;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VSUB_f_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode 'xxxx1110 0x11xxxx xxxx10xx x1x0xxxx'
        __guard cond != '1111'
        __decode
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && cond != '1110' then UNPREDICTABLE;
            advsimd = FALSE;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __encoding aarch32_VSUB_f_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field sz 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 0x1xxxxx xxxx1101 xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if sz == '1' && !HaveFP16Ext() then UNDEFINED;
            if sz == '1' && InITBlock() then UNPREDICTABLE;
            advsimd = TRUE;
            case sz of
                when '0' esize = 32; elements = 2;
                when '1' esize = 16; elements = 4;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VSUB_f_T2_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101110 0x11xxxx xxxx10xx x1x0xxxx'
        __guard TRUE
        __decode
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            advsimd = FALSE;
            case size of
                when '01' esize = 16; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDOrVFPEnabled(TRUE, advsimd);
        if advsimd then  // Advanced SIMD instruction
            for r = 0 to regs-1
                for e = 0 to elements-1
                    Elem[D[d+r],e,esize] = FPSub(Elem[D[n+r],e,esize], Elem[D[m+r],e,esize], StandardFPSCRValue());
        else             // VFP instruction
            case esize of
                when 16
                    S[d] = Zeros(16) : FPSub(S[n]<15:0>, S[m]<15:0>, FPSCR);
                when 32
                    S[d] = FPSub(S[n], S[m], FPSCR);
                when 64
                    D[d] = FPSub(D[n], D[m], FPSCR);
