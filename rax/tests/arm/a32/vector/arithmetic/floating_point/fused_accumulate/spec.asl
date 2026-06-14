__instruction aarch32_VFMA_A
    __encoding aarch32_VFMA_A1_A
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
        __opcode '11110010 0x0xxxxx xxxx1100 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if sz == '1' && !HaveFP16Ext() then UNDEFINED;
            advsimd = TRUE; op1_neg = (op == '1');
            case sz of
                when '0' esize = 32; elements = 2;
                when '1' esize = 16; elements = 4;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);
            regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VFMA_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field N 7 +: 1
        __field op 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode 'xxxx1110 1x10xxxx xxxx10xx x0x0xxxx'
        __guard cond != '1111'
        __decode
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && cond != '1110' then UNPREDICTABLE;
            advsimd = FALSE; op1_neg = (op == '1');
            case size of
                when '01' esize = 16; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __encoding aarch32_VFMA_T1_A
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
        __opcode '11101111 0x0xxxxx xxxx1100 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if sz == '1' && !HaveFP16Ext() then UNDEFINED;
            if sz == '1' && InITBlock() then UNPREDICTABLE;
            advsimd = TRUE; op1_neg = (op == '1');
            case sz of
                when '0' esize = 32; elements = 2;
                when '1' esize = 16; elements = 4;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);
            regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VFMA_T2_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field N 7 +: 1
        __field op 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101110 1x10xxxx xxxx10xx x0x0xxxx'
        __guard TRUE
        __decode
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            advsimd = FALSE; op1_neg = (op == '1');
            case size of
                when '01' esize = 16; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDOrVFPEnabled(TRUE, advsimd);
        if advsimd then // Advanced SIMD instruction
            for r = 0 to regs-1
                for e = 0 to elements-1
                    bits(esize) op1 = Elem[D[n+r],e,esize];
                    if op1_neg then op1 = FPNeg(op1);
                    Elem[D[d+r],e,esize] = FPMulAdd(Elem[D[d+r],e,esize],
                                           op1, Elem[D[m+r],e,esize], StandardFPSCRValue());

        else // VFP instruction
            case esize of
                when 16
                    op16 = if op1_neg then FPNeg(S[n]<15:0>) else S[n]<15:0>;
                    S[d] = Zeros(16) : FPMulAdd(S[d]<15:0>, op16, S[m]<15:0>, FPSCR);
                when 32
                    op32 = if op1_neg then FPNeg(S[n]) else S[n];
                    S[d] = FPMulAdd(S[d], op32, S[m], FPSCR);
                when 64
                    op64 = if op1_neg then FPNeg(D[n]) else D[n];
                    D[d] = FPMulAdd(D[d], op64, D[m], FPSCR);

__instruction aarch32_VFNMA_A
    __encoding aarch32_VFNMA_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field N 7 +: 1
        __field op 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode 'xxxx1110 1x01xxxx xxxx10xx x0x0xxxx'
        __guard cond != '1111'
        __decode
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && cond != '1110' then UNPREDICTABLE;
            op1_neg = (op == '1');
            case size of
                when '01' esize = 16; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __encoding aarch32_VFNMA_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field N 7 +: 1
        __field op 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101110 1x01xxxx xxxx10xx x0x0xxxx'
        __guard TRUE
        __decode
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            op1_neg = (op == '1');
            case size of
                when '01' esize = 16; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __execute __conditional
        CheckVFPEnabled(TRUE);
        case esize of
            when 16
                op16 = if op1_neg then FPNeg(S[n]<15:0>) else S[n]<15:0>;
                S[d] = Zeros(16) : FPMulAdd(FPNeg(S[d]<15:0>), op16, S[m]<15:0>, FPSCR);
            when 32
                op32 = if op1_neg then FPNeg(S[n]) else S[n];
                S[d] = FPMulAdd(FPNeg(S[d]), op32, S[m], FPSCR);
            when 64
                op64 = if op1_neg then FPNeg(D[n]) else D[n];
                D[d] = FPMulAdd(FPNeg(D[d]), op64, D[m], FPSCR);
