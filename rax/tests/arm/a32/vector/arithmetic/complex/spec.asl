__instruction aarch32_VCMLA_A
    __encoding aarch32_VCMLA_A1_A
        __instruction_set A32
        __field rot 23 +: 2
        __field D 22 +: 1
        __field S 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111110x xx1xxxxx xxxx1000 xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveFCADDExt() then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);
            esize = 16 << UInt(S);
            if !HaveFP16Ext() && esize == 16 then UNDEFINED;
            elements = 64 DIV esize;
            regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VCMLA_T1_A
        __instruction_set T32
        __field rot 23 +: 2
        __field D 22 +: 1
        __field S 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111110x xx1xxxxx xxxx1000 xxx0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveFCADDExt() then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);
            esize = 16 << UInt(S);
            if !HaveFP16Ext() && esize == 16 then UNDEFINED;
            elements = 64 DIV esize;
            regs = if Q == '0' then 1 else 2;

    __execute
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            operand1 = D[n+r];
            operand2 = D[m+r];
            operand3 = D[d+r];
            for e = 0 to (elements DIV 2)-1
                case rot of
                    when '00'
                       element1 = Elem[operand2,e*2,esize];
                       element2 = Elem[operand1,e*2,esize];
                       element3 = Elem[operand2,e*2+1,esize];
                       element4 = Elem[operand1,e*2,esize];
                    when '01'
                       element1 = FPNeg(Elem[operand2,e*2+1,esize]);
                       element2 = Elem[operand1,e*2+1,esize];
                       element3 = Elem[operand2,e*2,esize];
                       element4 = Elem[operand1,e*2+1,esize];
                    when '10'
                       element1 = FPNeg(Elem[operand2,e*2,esize]);
                       element2 = Elem[operand1,e*2,esize];
                       element3 = FPNeg(Elem[operand2,e*2+1,esize]);
                       element4 = Elem[operand1,e*2,esize];
                    when '11'
                       element1 = Elem[operand2,e*2+1,esize];
                       element2 = Elem[operand1,e*2+1,esize];
                       element3 = FPNeg(Elem[operand2,e*2,esize]);
                       element4 = Elem[operand1,e*2+1,esize];
                result1 = FPMulAdd(Elem[operand3,e*2,esize],element2,element1, StandardFPSCRValue());
                result2 = FPMulAdd(Elem[operand3,e*2+1,esize],element4,element3, StandardFPSCRValue());
                Elem[D[d+r],e*2,esize] = result1;
                Elem[D[d+r],e*2+1,esize] = result2;

__instruction aarch32_VCMLA_idx_A
    __encoding aarch32_VCMLA_idx_A1_A
        __instruction_set A32
        __field S 23 +: 1
        __field D 22 +: 1
        __field rot 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111110 xxxxxxxx xxxx1000 xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveFCADDExt() then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1') then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn);
            m = if S=='1' then UInt(M:Vm) else UInt(Vm);
            esize = 16 << UInt(S);
            if !HaveFP16Ext() && esize == 16 then UNDEFINED;
            elements = 64 DIV esize;
            regs = if Q == '0' then 1 else 2;
            index = if S=='1' then 0 else UInt(M);

    __encoding aarch32_VCMLA_idx_T1_A
        __instruction_set T32
        __field S 23 +: 1
        __field D 22 +: 1
        __field rot 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111110 xxxxxxxx xxxx1000 xxx0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveFCADDExt() then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1') then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn);
            m = if S=='1' then UInt(M:Vm) else UInt(Vm);
            esize = 16 << UInt(S);
            if !HaveFP16Ext() && esize == 16 then UNDEFINED;
            elements = 64 DIV esize;
            regs = if Q == '0' then 1 else 2;
            index = if S=='1' then 0 else UInt(M);

    __execute
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            operand1 = D[n+r];
            operand2 = Din[m];
            operand3 = D[d+r];
            for e = 0 to (elements DIV 2)-1
                case rot of
                    when '00'
                        element1 = Elem[operand2,index*2,esize];
                        element2 = Elem[operand1,e*2,esize];
                        element3 = Elem[operand2,index*2+1,esize];
                        element4 = Elem[operand1,e*2,esize];
                    when '01'
                        element1 = FPNeg(Elem[operand2,index*2+1,esize]);
                        element2 = Elem[operand1,e*2+1,esize];
                        element3 = Elem[operand2,index*2,esize];
                        element4 = Elem[operand1,e*2+1,esize];
                    when '10'
                        element1 = FPNeg(Elem[operand2,index*2,esize]);
                        element2 = Elem[operand1,e*2,esize];
                        element3 = FPNeg(Elem[operand2,index*2+1,esize]);
                        element4 = Elem[operand1,e*2,esize];
                    when '11'
                        element1 = Elem[operand2,index*2+1,esize];
                        element2 = Elem[operand1,e*2+1,esize];
                        element3 = FPNeg(Elem[operand2,index*2,esize]);
                        element4 = Elem[operand1,e*2+1,esize];
                result1 = FPMulAdd(Elem[operand3,e*2,esize],element2,element1, StandardFPSCRValue());
                result2 = FPMulAdd(Elem[operand3,e*2+1,esize],element4,element3,StandardFPSCRValue());
                Elem[D[d+r],e*2,esize] = result1;
                Elem[D[d+r],e*2+1,esize] = result2;

__instruction aarch32_VCADD_A
    __encoding aarch32_VCADD_A1_A
        __instruction_set A32
        __field rot 24 +: 1
        __field D 22 +: 1
        __field S 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111110x 1x0xxxxx xxxx1000 xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveFCADDExt() then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);
            esize = 16 << UInt(S);
            if !HaveFP16Ext() && esize == 16 then UNDEFINED;
            elements = 64 DIV esize;
            regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VCADD_T1_A
        __instruction_set T32
        __field rot 24 +: 1
        __field D 22 +: 1
        __field S 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111110x 1x0xxxxx xxxx1000 xxx0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveFCADDExt() then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);
            esize = 16 << UInt(S);
            if !HaveFP16Ext() && esize == 16 then UNDEFINED;
            elements = 64 DIV esize;
            regs = if Q == '0' then 1 else 2;

    __execute
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            operand1 = D[n+r];
            operand2 = D[m+r];
            operand3 = D[d+r];
            for e = 0 to (elements DIV 2)-1
                case rot of
                    when '0'
                        element1 = FPNeg(Elem[operand2,e*2+1,esize]);
                        element3 = Elem[operand2,e*2,esize];
                    when '1'
                        element1 = Elem[operand2,e*2+1,esize];
                        element3 = FPNeg(Elem[operand2,e*2,esize]);
                result1 = FPAdd(Elem[operand1,e*2,esize],element1,StandardFPSCRValue());
                result2 = FPAdd(Elem[operand1,e*2+1,esize],element3,StandardFPSCRValue());
                Elem[D[d+r],e*2,esize] = result1;
                Elem[D[d+r],e*2+1,esize] = result2;
