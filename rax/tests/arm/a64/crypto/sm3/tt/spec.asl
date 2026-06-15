__instruction aarch64_vector_crypto_sm3_sm3tt1a
    __encoding aarch64_vector_crypto_sm3_sm3tt1a
        __instruction_set A64
        __field Rm 16 +: 5
        __field imm2 12 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11001110 010xxxxx 10xx00xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSM3Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            integer i = UInt(imm2);

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) Vm = V[m];
        bits(128) Vn = V[n];
        bits(128) Vd = V[d];
        bits(32) WjPrime;
        bits(128) result;
        bits(32) TT1;
        bits(32) SS2;

        WjPrime = Elem[Vm,i,32];
        SS2 = Vn<127:96> EOR ROL(Vd<127:96>,12);
        TT1 = Vd<63:32> EOR (Vd<127:96> EOR Vd<95:64>);
        TT1 = (TT1 + Vd<31:0> + SS2 + WjPrime)<31:0>;
        result<31:0> = Vd<63:32>;
        result<63:32> = ROL(Vd<95:64>,9);
        result<95:64> = Vd<127:96>;
        result<127:96> = TT1;
        V[d] = result;

__instruction aarch64_vector_crypto_sm3_sm3tt1b
    __encoding aarch64_vector_crypto_sm3_sm3tt1b
        __instruction_set A64
        __field Rm 16 +: 5
        __field imm2 12 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11001110 010xxxxx 10xx01xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSM3Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            integer i = UInt(imm2);

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) Vm = V[m];
        bits(128) Vn = V[n];
        bits(128) Vd = V[d];
        bits(32) WjPrime;
        bits(128) result;
        bits(32) TT1;
        bits(32) SS2;

        WjPrime = Elem[Vm,i,32];
        SS2 = Vn<127:96> EOR ROL(Vd<127:96>,12);
        TT1 = (Vd<127:96> AND Vd<63:32>) OR (Vd<127:96> AND Vd<95:64>) OR (Vd<63:32> AND Vd<95:64>);
        TT1 = (TT1 + Vd<31:0> + SS2 + WjPrime)<31:0>;
        result<31:0> = Vd<63:32>;
        result<63:32> = ROL(Vd<95:64>,9);
        result<95:64> = Vd<127:96>;
        result<127:96> = TT1;
        V[d] = result;

__instruction aarch64_vector_crypto_sm3_sm3tt2a
    __encoding aarch64_vector_crypto_sm3_sm3tt2a
        __instruction_set A64
        __field Rm 16 +: 5
        __field imm2 12 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11001110 010xxxxx 10xx10xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSM3Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            integer i = UInt(imm2);

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) Vm = V[m];
        bits(128) Vn = V[n];
        bits(128) Vd = V[d];
        bits(32) Wj;
        bits(128) result;
        bits(32) TT1;

        Wj = Elem[Vm,i,32];
        TT2 = Vd<63:32> EOR (Vd<127:96> EOR Vd<95:64>);
        TT2 = (TT2 + Vd<31:0> + Vn<127:96> + Wj)<31:0>;

        result<31:0> = Vd<63:32>;
        result<63:32> = ROL(Vd<95:64>,19);
        result<95:64> = Vd<127:96>;
        result<127:96> = TT2 EOR ROL(TT2,9) EOR ROL(TT2,17);
        V[d] = result;

__instruction aarch64_vector_crypto_sm3_sm3tt2b
    __encoding aarch64_vector_crypto_sm3_sm3tt2b
        __instruction_set A64
        __field Rm 16 +: 5
        __field imm2 12 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11001110 010xxxxx 10xx11xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSM3Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            integer i = UInt(imm2);

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) Vm = V[m];
        bits(128) Vn = V[n];
        bits(128) Vd = V[d];
        bits(32) Wj;
        bits(128) result;
        bits(32) TT2;

        Wj = Elem[Vm,i,32];
        TT2 = (Vd<127:96> AND Vd<95:64>) OR (NOT(Vd<127:96>) AND Vd<63:32>);
        TT2 = (TT2 + Vd<31:0> + Vn<127:96> + Wj)<31:0>;

        result<31:0> = Vd<63:32>;
        result<63:32> = ROL(Vd<95:64>,19);
        result<95:64> = Vd<127:96>;
        result<127:96> = TT2 EOR ROL(TT2,9) EOR ROL(TT2,17);
        V[d] = result;
