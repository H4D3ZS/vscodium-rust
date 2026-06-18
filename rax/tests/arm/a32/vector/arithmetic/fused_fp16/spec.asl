__instruction aarch32_VFMAL_A
    __encoding aarch32_VFMAL_A1_A
        __instruction_set A32
        __field S 23 +: 1
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111100 0x10xxxx xxxx1000 xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveFP16MulNoRoundingToFP32Ext() then UNDEFINED;
            if Q == '1' && Vd<0> == '1' then UNDEFINED;

            integer d = UInt(D:Vd);
            integer n = if Q == '1' then UInt(N:Vn) else UInt(Vn:N);
            integer m = if Q == '1' then UInt(M:Vm) else UInt(Vm:M);
            integer esize = 32;
            integer regs = if Q=='1' then 2 else 1;
            integer datasize = if Q=='1' then 64 else 32;
            boolean sub_op = S=='1';

    __encoding aarch32_VFMAL_T1_A
        __instruction_set T32
        __field S 23 +: 1
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111100 0x10xxxx xxxx1000 xxx1xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveFP16MulNoRoundingToFP32Ext() then UNDEFINED;
            if Q == '1' && Vd<0> == '1' then UNDEFINED;

            integer d = UInt(D:Vd);
            integer n = if Q == '1' then UInt(N:Vn) else UInt(Vn:N);
            integer m = if Q == '1' then UInt(M:Vm) else UInt(Vm:M);
            integer esize = 32;
            integer regs = if Q=='1' then 2 else 1;
            integer datasize = if Q=='1' then 64 else 32;
            boolean sub_op = S=='1';

    __execute
        CheckAdvSIMDEnabled();
        bits(datasize) operand1 ;
        bits(datasize) operand2 ;
        bits(64) operand3;
        bits(64) result;
        bits(esize DIV 2) element1;
        bits(esize DIV 2) element2;

        if Q=='0' then
            operand1 = S[n]<datasize-1:0>;
            operand2 = S[m]<datasize-1:0>;
        else
            operand1 = D[n]<datasize-1:0>;
            operand2 = D[m]<datasize-1:0>;
        for r = 0 to regs-1
            operand3 = D[d+r];
            for e = 0 to 1
                element1 = Elem[operand1, 2*r+e, esize DIV 2];
                element2 = Elem[operand2, 2*r+e, esize DIV 2];
                if sub_op then element1 = FPNeg(element1);
                Elem[result, e, esize] = FPMulAddH(Elem[operand3, e, esize], element1, element2, StandardFPSCRValue());
            D[d+r] = result;

__instruction aarch32_VFMAL_i_A
    __encoding aarch32_VFMAL_i_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field S 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111110 0x01xxxx xxxx1000 xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveFP16MulNoRoundingToFP32Ext() then UNDEFINED;
            if Q == '1' && Vd<0> == '1' then UNDEFINED;

            integer d = UInt(D:Vd);
            integer n = if Q == '1' then UInt(N:Vn) else UInt(Vn:N);
            integer m = if Q == '1' then UInt(Vm<2:0>) else UInt(Vm<2:0>:M);

            integer index = if Q == '1' then UInt(M:Vm<3>) else UInt(Vm<3>);
            integer esize = 32;
            integer regs = if Q=='1' then 2 else 1;
            integer datasize = if Q=='1' then 64 else 32;
            boolean sub_op = S=='1';

    __encoding aarch32_VFMAL_i_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field S 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111110 0x01xxxx xxxx1000 xxx1xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveFP16MulNoRoundingToFP32Ext() then UNDEFINED;
            if Q == '1' && Vd<0> == '1' then UNDEFINED;

            integer d = UInt(D:Vd);
            integer n = if Q == '1' then UInt(N:Vn) else UInt(Vn:N);
            integer m = if Q == '1' then UInt(Vm<2:0>) else UInt(Vm<2:0>:M);

            integer index = if Q == '1' then UInt(M:Vm<3>) else UInt(Vm<3>);
            integer esize = 32;
            integer regs = if Q=='1' then 2 else 1;
            integer datasize = if Q=='1' then 64 else 32;
            boolean sub_op = S=='1';

    __execute
        CheckAdvSIMDEnabled();
        bits(datasize) operand1 ;
        bits(datasize) operand2 ;
        bits(64) operand3;
        bits(64) result;
        bits(esize DIV 2) element1;
        bits(esize DIV 2) element2;

        if Q=='0' then
            operand1 = S[n]<datasize-1:0>;
            operand2 = S[m]<datasize-1:0>;
        else
            operand1 = D[n]<datasize-1:0>;
            operand2 = D[m]<datasize-1:0>;
        element2 = Elem[operand2, index, esize DIV 2];
        for r = 0 to regs-1
            operand3 = D[d+r];
            for e = 0 to 1
                element1 = Elem[operand1, 2*r+e, esize DIV 2];
                if sub_op then element1 = FPNeg(element1);
                Elem[result, e, esize] = FPMulAddH(Elem[operand3, e, esize], element1, element2, StandardFPSCRValue());
            D[d+r] = result;
