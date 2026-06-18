__instruction aarch32_VMUL_s_A
    __encoding aarch32_VMUL_s_A1_A
        __instruction_set A32
        __field Q 24 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field F 8 +: 1
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 1xxxxxxx xxxx100x x1x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if size == '00' || (F == '1' && size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1') then UNDEFINED;
            unsigned = FALSE;  // "Don't care" value: TRUE produces same functionality
            floating_point = (F == '1');  long_destination = FALSE;
            d = UInt(D:Vd);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;
            if size == '01' then esize = 16;  elements = 4;  m = UInt(Vm<2:0>);  index = UInt(M:Vm<3>);
            if size == '10' then esize = 32;  elements = 2;  m = UInt(Vm);  index = UInt(M);

    __encoding aarch32_VMUL_s_T1_A
        __instruction_set T32
        __field Q 28 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field F 8 +: 1
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 1xxxxxxx xxxx100x x1x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if F == '1' && size == '01' && InITBlock() then UNPREDICTABLE;
            if size == '00' || (F == '1' && size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1') then UNDEFINED;
            unsigned = FALSE;  // "Don't care" value: TRUE produces same functionality
            floating_point = (F == '1');  long_destination = FALSE;
            d = UInt(D:Vd);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;
            if size == '01' then esize = 16;  elements = 4;  m = UInt(Vm<2:0>);  index = UInt(M:Vm<3>);
            if size == '10' then esize = 32;  elements = 2;  m = UInt(Vm);  index = UInt(M);

    __execute __conditional
        CheckAdvSIMDEnabled();
        op2 = Elem[Din[m],index,esize];  op2val = Int(op2, unsigned);
        for r = 0 to regs-1
            for e = 0 to elements-1
                op1 = Elem[Din[n+r],e,esize];  op1val = Int(op1, unsigned);
                if floating_point then
                    Elem[D[d+r],e,esize] = FPMul(op1, op2, StandardFPSCRValue());
                else
                    if long_destination then
                        Elem[Q[d>>1],e,2*esize] = (op1val*op2val)<2*esize-1:0>;
                    else
                        Elem[D[d+r],e,esize] = (op1val*op2val)<esize-1:0>;

__instruction aarch32_VMLA_s_A
    __encoding aarch32_VMLA_s_A1_A
        __instruction_set A32
        __field Q 24 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field op 10 +: 1
        __field F 8 +: 1
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 1xxxxxxx xxxx000x x1x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if size == '00' || (F == '1' && size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1') then UNDEFINED;
            unsigned = FALSE;  // "Don't care" value: TRUE produces same functionality
            add = (op == '0');  floating_point = (F == '1');  long_destination = FALSE;
            d = UInt(D:Vd);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;
            if size == '01' then esize = 16;  elements = 4;  m = UInt(Vm<2:0>);  index = UInt(M:Vm<3>);
            if size == '10' then esize = 32;  elements = 2;  m = UInt(Vm);  index = UInt(M);

    __encoding aarch32_VMLA_s_T1_A
        __instruction_set T32
        __field Q 28 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field op 10 +: 1
        __field F 8 +: 1
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 1xxxxxxx xxxx000x x1x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if size == '00' || (F == '1' && size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if F == '1' && size == '01' && InITBlock() then UNPREDICTABLE;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1') then UNDEFINED;
            unsigned = FALSE;  // "Don't care" value: TRUE produces same functionality
            add = (op == '0');  floating_point = (F == '1');  long_destination = FALSE;
            d = UInt(D:Vd);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;
            if size == '01' then esize = 16;  elements = 4;  m = UInt(Vm<2:0>);  index = UInt(M:Vm<3>);
            if size == '10' then esize = 32;  elements = 2;  m = UInt(Vm);  index = UInt(M);

    __execute __conditional
        CheckAdvSIMDEnabled();
        op2 = Elem[Din[m],index,esize];  op2val = Int(op2, unsigned);
        for r = 0 to regs-1
            for e = 0 to elements-1
                op1 = Elem[Din[n+r],e,esize];  op1val = Int(op1, unsigned);
                if floating_point then
                    fp_addend = if add then FPMul(op1,op2,StandardFPSCRValue()) else FPNeg(FPMul(op1,op2,StandardFPSCRValue()));
                    Elem[D[d+r],e,esize] = FPAdd(Elem[Din[d+r],e,esize], fp_addend, StandardFPSCRValue());
                else
                    addend = if add then op1val*op2val else -op1val*op2val;
                    if long_destination then
                        Elem[Q[d>>1],e,2*esize] = Elem[Qin[d>>1],e,2*esize] + addend;
                    else
                        Elem[D[d+r],e,esize] = Elem[Din[d+r],e,esize] + addend;
