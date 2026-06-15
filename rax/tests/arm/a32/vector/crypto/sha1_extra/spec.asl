__instruction aarch32_SHA1SU0_A
    __encoding aarch32_SHA1SU0_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 0x11xxxx xxxx1100 xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSHA1Ext() then UNDEFINED;
            if Q != '1' then UNDEFINED;
            if Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __encoding aarch32_SHA1SU0_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 0x11xxxx xxxx1100 xxx0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveSHA1Ext() then UNDEFINED;
            if Q != '1' then UNDEFINED;
            if Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __execute __conditional
        CheckCryptoEnabled32();
        op1 = Q[d>>1]; op2 = Q[n>>1]; op3 = Q[m>>1];
        op2 = op2<63:0> : op1<127:64>;
        Q[d>>1] = op1 EOR op2 EOR op3;

__instruction aarch32_SHA1SU1_A
    __encoding aarch32_SHA1SU1_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx10 xxxx0011 10x0xxxx'
        __guard TRUE
        __decode
            if !HaveSHA1Ext() then UNDEFINED;
            if size != '10' then UNDEFINED;
            if Vd<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); m = UInt(M:Vm);

    __encoding aarch32_SHA1SU1_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx10 xxxx0011 10x0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveSHA1Ext() then UNDEFINED;
            if size != '10' then UNDEFINED;
            if Vd<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); m = UInt(M:Vm);

    __execute __conditional
        CheckCryptoEnabled32();
        X = Q[d>>1]; Y = Q[m>>1];
        T = X EOR LSR(Y, 32);
        W0 = ROL(T<31:0>, 1);
        W1 = ROL(T<63:32>, 1);
        W2 = ROL(T<95:64>, 1);
        W3 = ROL(T<127:96>, 1) EOR ROL(T<31:0>, 2);
        Q[d>>1] = W3:W2:W1:W0;
