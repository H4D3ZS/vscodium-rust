__instruction aarch32_SHA256H_A
    __encoding aarch32_SHA256H_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 0x00xxxx xxxx1100 xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSHA256Ext() then UNDEFINED;
            if Q != '1' then UNDEFINED;
            if Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __encoding aarch32_SHA256H_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 0x00xxxx xxxx1100 xxx0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveSHA256Ext() then UNDEFINED;
            if Q != '1' then UNDEFINED;
            if Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __execute __conditional
        CheckCryptoEnabled32();
        X = Q[d>>1]; Y = Q[n>>1]; W = Q[m>>1]; part1 = TRUE;
        Q[d>>1] = SHA256hash(X, Y, W, part1);

__instruction aarch32_SHA256H2_A
    __encoding aarch32_SHA256H2_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 0x01xxxx xxxx1100 xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSHA256Ext() then UNDEFINED;
            if Q != '1' then UNDEFINED;
            if Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __encoding aarch32_SHA256H2_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 0x01xxxx xxxx1100 xxx0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveSHA256Ext() then UNDEFINED;
            if Q != '1' then UNDEFINED;
            if Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __execute __conditional
        CheckCryptoEnabled32();
        X = Q[n>>1]; Y = Q[d>>1]; W = Q[m>>1]; part1 = FALSE;
        Q[d>>1] = SHA256hash(X, Y, W, part1);

__instruction aarch32_SHA256SU0_A
    __encoding aarch32_SHA256SU0_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx10 xxxx0011 11x0xxxx'
        __guard TRUE
        __decode
            if !HaveSHA256Ext() then UNDEFINED;
            if size != '10' then UNDEFINED;
            if Vd<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); m = UInt(M:Vm);

    __encoding aarch32_SHA256SU0_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx10 xxxx0011 11x0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveSHA256Ext() then UNDEFINED;
            if size != '10' then UNDEFINED;
            if Vd<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); m = UInt(M:Vm);

    __execute __conditional
        bits(128) result;
        EncodingSpecificOperations(); CheckCryptoEnabled32();
        X = Q[d>>1]; Y = Q[m>>1];
        T = Y<31:0> : X<127:32>;
        for e = 0 to 3
            elt = Elem[T, e, 32];
            elt = ROR(elt, 7) EOR ROR(elt, 18) EOR LSR(elt, 3);
            Elem[result, e, 32] = elt + Elem[X, e, 32];
        Q[d>>1] = result;

__instruction aarch32_SHA256SU1_A
    __encoding aarch32_SHA256SU1_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 0x10xxxx xxxx1100 xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSHA256Ext() then UNDEFINED;
            if Q != '1' then UNDEFINED;
            if Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __encoding aarch32_SHA256SU1_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 0x10xxxx xxxx1100 xxx0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveSHA256Ext() then UNDEFINED;
            if Q != '1' then UNDEFINED;
            if Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm);

    __execute __conditional
        bits(128) result;
        EncodingSpecificOperations(); CheckCryptoEnabled32();
        X = Q[d>>1]; Y = Q[n>>1]; Z = Q[m>>1];
        T0 = Z<31:0> : Y<127:32>;

        T1 = Z<127:64>;
        for e = 0 to 1
            elt = Elem[T1, e, 32];
            elt = ROR(elt, 17) EOR ROR(elt, 19) EOR LSR(elt, 10);
            elt = elt + Elem[X, e, 32] + Elem[T0, e, 32];
            Elem[result, e, 32] = elt;

        T1 = result<63:0>;
        for e = 2 to 3
            elt = Elem[T1, e - 2, 32];
            elt = ROR(elt, 17) EOR ROR(elt, 19) EOR LSR(elt, 10);
            elt = elt + Elem[X, e, 32] + Elem[T0, e, 32];
            Elem[result, e, 32] = elt;

        Q[d>>1] = result;
