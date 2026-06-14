__instruction aarch32_SHA1C_A
    __encoding aarch32_SHA1C_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 0x00xxxx xxxx1100 xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSHA1Ext() then UNDEFINED;
            if Q != '1' then UNDEFINED;
            if Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __encoding aarch32_SHA1C_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 0x00xxxx xxxx1100 xxx0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveSHA1Ext() then UNDEFINED;
            if Q != '1' then UNDEFINED;
            if Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __execute __conditional
        CheckCryptoEnabled32();
        X = Q[d>>1];
        Y = Q[n>>1]<31:0>; // Note: 32 bits wide
        W = Q[m>>1];
        for e = 0 to 3
            t = SHAchoose(X<63:32>, X<95:64>, X<127:96>);
            Y = Y + ROL(X<31:0>, 5) + t + Elem[W, e, 32];
            X<63:32> = ROL(X<63:32>, 30);
            <Y, X> = ROL(Y:X, 32);
        Q[d>>1] = X;

__instruction aarch32_SHA1H_A
    __encoding aarch32_SHA1H_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx01 xxxx0010 11x0xxxx'
        __guard TRUE
        __decode
            if !HaveSHA1Ext() then UNDEFINED;
            if size != '10' then UNDEFINED;
            if Vd<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); m = UInt(M:Vm);

    __encoding aarch32_SHA1H_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx01 xxxx0010 11x0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveSHA1Ext() then UNDEFINED;
            if size != '10' then UNDEFINED;
            if Vd<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); m = UInt(M:Vm);

    __execute __conditional
        CheckCryptoEnabled32();
        Q[d>>1] = ZeroExtend(ROL(Q[m>>1]<31:0>, 30), 128);

__instruction aarch32_SHA1M_A
    __encoding aarch32_SHA1M_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 0x10xxxx xxxx1100 xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSHA1Ext() then UNDEFINED;
            if Q != '1' then UNDEFINED;
            if Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __encoding aarch32_SHA1M_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 0x10xxxx xxxx1100 xxx0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveSHA1Ext() then UNDEFINED;
            if Q != '1' then UNDEFINED;
            if Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __execute __conditional
        CheckCryptoEnabled32();
        X = Q[d>>1];
        Y = Q[n>>1]<31:0>; // Note: 32 bits wide
        W = Q[m>>1];
        for e = 0 to 3
            t = SHAmajority(X<63:32>, X<95:64>, X<127:96>);
            Y = Y + ROL(X<31:0>, 5) + t + Elem[W, e, 32];
            X<63:32> = ROL(X<63:32>, 30);
            <Y, X> = ROL(Y:X, 32);
        Q[d>>1] = X;

__instruction aarch32_SHA1P_A
    __encoding aarch32_SHA1P_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 0x01xxxx xxxx1100 xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSHA1Ext() then UNDEFINED;
            if Q != '1' then UNDEFINED;
            if Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __encoding aarch32_SHA1P_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 0x01xxxx xxxx1100 xxx0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveSHA1Ext() then UNDEFINED;
            if Q != '1' then UNDEFINED;
            if Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __execute __conditional
        CheckCryptoEnabled32();
        X = Q[d>>1];
        Y = Q[n>>1]<31:0>; // Note: 32 bits wide
        W = Q[m>>1];
        for e = 0 to 3
            t = SHAparity(X<63:32>, X<95:64>, X<127:96>);
            Y = Y + ROL(X<31:0>, 5) + t + Elem[W, e, 32];
            X<63:32> = ROL(X<63:32>, 30);
            <Y, X> = ROL(Y:X, 32);
        Q[d>>1] = X;
