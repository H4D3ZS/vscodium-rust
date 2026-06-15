__instruction aarch64_vector_crypto_sha2op_sha1_hash
    __encoding aarch64_vector_crypto_sha2op_sha1_hash
        __instruction_set A64
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 00101000 000010xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            if !HaveSHA1Ext() then UNDEFINED;

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(32) operand = V[n];        // read element [0] only,  [1-3] zeroed
        V[d] = ROL(operand, 30);

__instruction aarch64_vector_crypto_sha3op_sha1_hash_choose
    __encoding aarch64_vector_crypto_sha3op_sha1_hash_choose
        __instruction_set A64
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 000xxxxx 000000xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if !HaveSHA1Ext() then UNDEFINED;

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) X = V[d];
        bits(32)  Y = V[n];     // Note: 32 not 128 bits wide
        bits(128) W = V[m];
        bits(32)  t;

        for e = 0 to 3
            t = SHAchoose(X<63:32>, X<95:64>, X<127:96>);
            Y = Y + ROL(X<31:0>, 5) + t + Elem[W, e, 32];
            X<63:32> = ROL(X<63:32>, 30);
            <Y, X> = ROL(Y : X, 32);
        V[d] = X;

__instruction aarch64_vector_crypto_sha3op_sha1_hash_majority
    __encoding aarch64_vector_crypto_sha3op_sha1_hash_majority
        __instruction_set A64
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 000xxxxx 001000xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if !HaveSHA1Ext() then UNDEFINED;

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) X = V[d];
        bits(32)  Y = V[n];     // Note: 32 not 128 bits wide
        bits(128) W = V[m];
        bits(32)  t;

        for e = 0 to 3
            t = SHAmajority(X<63:32>, X<95:64>, X<127:96>);
            Y = Y + ROL(X<31:0>, 5) + t + Elem[W, e, 32];
            X<63:32> = ROL(X<63:32>, 30);
            <Y, X> = ROL(Y : X, 32);
        V[d] = X;

__instruction aarch64_vector_crypto_sha3op_sha1_hash_parity
    __encoding aarch64_vector_crypto_sha3op_sha1_hash_parity
        __instruction_set A64
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 000xxxxx 000100xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if !HaveSHA1Ext() then UNDEFINED;

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) X = V[d];
        bits(32)  Y = V[n];     // Note: 32 not 128 bits wide
        bits(128) W = V[m];
        bits(32)  t;

        for e = 0 to 3
            t = SHAparity(X<63:32>, X<95:64>, X<127:96>);
            Y = Y + ROL(X<31:0>, 5) + t + Elem[W, e, 32];
            X<63:32> = ROL(X<63:32>, 30);
            <Y, X> = ROL(Y : X, 32);
        V[d] = X;
