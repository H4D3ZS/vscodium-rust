__instruction aarch32_VMUL_f_A
    __encoding aarch32_VMUL_f_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field sz 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 0x0xxxxx xxxx1101 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if sz == '1' && !HaveFP16Ext() then UNDEFINED;
            advsimd = TRUE;
            case sz of
                when '0' esize = 32; elements = 2;
                when '1' esize = 16; elements = 4;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VMUL_f_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode 'xxxx1110 0x10xxxx xxxx10xx x0x0xxxx'
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

    __encoding aarch32_VMUL_f_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field sz 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 0x0xxxxx xxxx1101 xxx1xxxx'
        __guard TRUE
        __decode
            if sz == '1' && InITBlock() then UNPREDICTABLE;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if sz == '1' && !HaveFP16Ext() then UNDEFINED;
            advsimd = TRUE;
            case sz of
                when '0' esize = 32; elements = 2;
                when '1' esize = 16; elements = 4;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VMUL_f_T2_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101110 0x10xxxx xxxx10xx x0x0xxxx'
        __guard TRUE
        __decode
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
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
                    Elem[D[d+r],e,esize] = FPMul(Elem[D[n+r],e,esize], Elem[D[m+r],e,esize], StandardFPSCRValue());
        else             // VFP instruction
            case esize of
                when 16
                    S[d] = Zeros(16) : FPMul(S[n]<15:0>, S[m]<15:0>, FPSCR);
                when 32
                    S[d] = FPMul(S[n], S[m], FPSCR);
                when 64
                    D[d] = FPMul(D[n], D[m], FPSCR);

__instruction aarch32_VMLA_f_A
    __encoding aarch32_VMLA_f_A1_A
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
        __opcode '11110010 0x0xxxxx xxxx1101 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if sz == '1' && !HaveFP16Ext() then UNDEFINED;
            advsimd = TRUE;  add = (op == '0');
            case sz of
                when '0' esize = 32; elements = 2;
                when '1' esize = 16; elements = 4;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VMLA_f_A2_A
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
        __opcode 'xxxx1110 0x00xxxx xxxx10xx x0x0xxxx'
        __guard cond != '1111'
        __decode
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && cond != '1110' then UNPREDICTABLE;
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            advsimd = FALSE; add = (op == '0');
            case size of
                when '01' esize = 16; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __encoding aarch32_VMLA_f_T1_A
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
        __opcode '11101111 0x0xxxxx xxxx1101 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if sz == '1' && !HaveFP16Ext() then UNDEFINED;
            if sz == '1' && InITBlock() then UNPREDICTABLE;
            advsimd = TRUE;  add = (op == '0');
            case sz of
                when '0' esize = 32; elements = 2;
                when '1' esize = 16; elements = 4;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VMLA_f_T2_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field N 7 +: 1
        __field op 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101110 0x00xxxx xxxx10xx x0x0xxxx'
        __guard TRUE
        __decode
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            advsimd = FALSE; add = (op == '0');
            case size of
                when '01' esize = 16; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '10' esize = 32; d = UInt(Vd:D); n = UInt(Vn:N); m = UInt(Vm:M);
                when '11' esize = 64; d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDOrVFPEnabled(TRUE, advsimd);
        if advsimd then  // Advanced SIMD instruction
            for r = 0 to regs-1
                for e = 0 to elements-1
                    product = FPMul(Elem[D[n+r],e,esize], Elem[D[m+r],e,esize], StandardFPSCRValue());
                    addend = if add then product else FPNeg(product);
                    Elem[D[d+r],e,esize] = FPAdd(Elem[D[d+r],e,esize], addend, StandardFPSCRValue());
        else             // VFP instruction
            case esize of
                when 16
                    addend16 = if add then FPMul(S[n]<15:0>, S[m]<15:0>, FPSCR) else FPNeg(FPMul(S[n]<15:0>, S[m]<15:0>, FPSCR));
                    S[d] = Zeros(16) : FPAdd(S[d]<15:0>, addend16, FPSCR);
                when 32
                    addend32 = if add then FPMul(S[n], S[m], FPSCR) else FPNeg(FPMul(S[n], S[m], FPSCR));
                    S[d] = FPAdd(S[d], addend32, FPSCR);
                when 64
                    addend64 = if add then FPMul(D[n], D[m], FPSCR) else FPNeg(FPMul(D[n], D[m], FPSCR));
                    D[d] = FPAdd(D[d], addend64, FPSCR);
