__instruction aarch64_vector_crypto_sm4_sm4enc
    __encoding aarch64_vector_crypto_sm4_sm4enc
        __instruction_set A64
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11001110 11000000 100001xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSM4Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) Vn = V[n];
        bits(32) intval;
        bits(8) sboxout;
        bits(128) roundresult;
        bits(32) roundkey;

        roundresult=V[d];
        for index = 0 to 3
            roundkey = Elem[Vn,index,32];

            intval = roundresult<127:96> EOR roundresult<95:64> EOR roundresult<63:32> EOR roundkey;

            for i = 0 to 3
                Elem[intval,i,8]  = Sbox(Elem[intval,i,8]);

            intval = intval EOR ROL(intval,2) EOR ROL(intval,10) EOR ROL(intval,18) EOR ROL(intval,24);
            intval = intval EOR roundresult<31:0>;

            roundresult<31:0> = roundresult<63:32>;
            roundresult<63:32> = roundresult<95:64>;
            roundresult<95:64> = roundresult<127:96>;
            roundresult<127:96> = intval;
        V[d] = roundresult;

__instruction aarch64_vector_crypto_sm4_sm4enckey
    __encoding aarch64_vector_crypto_sm4_sm4enckey
        __instruction_set A64
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11001110 011xxxxx 110010xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSM4Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) Vm = V[m];
        bits(32) intval;
        bits(8) sboxout;
        bits(128) result;
        bits(32) const;
        bits(128) roundresult;

        roundresult = V[n];
        for index = 0 to 3
            const = Elem[Vm,index,32];

            intval = roundresult<127:96> EOR roundresult<95:64> EOR roundresult<63:32> EOR const;

            for i = 0 to 3
                Elem[intval,i,8] = Sbox(Elem[intval,i,8]);

            intval = intval EOR ROL(intval,13) EOR ROL(intval,23);
            intval = intval EOR roundresult<31:0>;

            roundresult<31:0> = roundresult<63:32>;
            roundresult<63:32> = roundresult<95:64>;
            roundresult<95:64> = roundresult<127:96>;
            roundresult<127:96> = intval;
        V[d] = roundresult;
