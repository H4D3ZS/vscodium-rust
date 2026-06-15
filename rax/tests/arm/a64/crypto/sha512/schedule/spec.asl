__instruction aarch64_vector_crypto_sha512_sha512su0
    __encoding aarch64_vector_crypto_sha512_sha512su0
        __instruction_set A64
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11001110 11000000 100000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSHA512Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(64) sig0;
        bits(128) Vtmp;
        bits(128) X = V[n];
        bits(128) W = V[d];
        sig0 = ROR(W<127:64>, 1) EOR ROR(W<127:64>, 8) EOR ('0000000':W<127:71>);
        Vtmp<63:0> = W<63:0> + sig0;
        sig0 = ROR(X<63:0>, 1) EOR ROR(X<63:0>, 8) EOR ('0000000':X<63:7>);
        Vtmp<127:64> = W<127:64> + sig0;
        V[d] = Vtmp;

__instruction aarch64_vector_crypto_sha512_sha512su1
    __encoding aarch64_vector_crypto_sha512_sha512su1
        __instruction_set A64
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11001110 011xxxxx 100010xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSHA512Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(64) sig1;
        bits(128) Vtmp;
        bits(128) X = V[n];
        bits(128) Y = V[m];
        bits(128) W = V[d];

        sig1 = ROR(X<127:64>, 19) EOR ROR(X<127:64>,61) EOR ('000000':X<127:70>);
        Vtmp<127:64> = W<127:64> + sig1 + Y<127:64>;
        sig1 = ROR(X<63:0>, 19) EOR ROR(X<63:0>,61) EOR ('000000':X<63:6>);
        Vtmp<63:0> = W<63:0> + sig1 + Y<63:0>;
        V[d] = Vtmp;
