__instruction aarch64_vector_crypto_sha512_sha512h
    __encoding aarch64_vector_crypto_sha512_sha512h
        __instruction_set A64
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11001110 011xxxxx 100000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSHA512Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) Vtmp;
        bits(64)  MSigma1;
        bits(64)  tmp;
        bits(128) X = V[n];
        bits(128) Y = V[m];
        bits(128) W = V[d];

        MSigma1 =  ROR(Y<127:64>, 14) EOR ROR(Y<127:64>,18) EOR ROR(Y<127:64>,41);
        Vtmp<127:64> =  (Y<127:64> AND X<63:0>) EOR (NOT(Y<127:64>) AND X<127:64>);
        Vtmp<127:64> = (Vtmp<127:64> + MSigma1 +  W<127:64>);
        tmp = Vtmp<127:64> + Y<63:0>;
        MSigma1 = ROR(tmp, 14) EOR ROR(tmp,18) EOR ROR(tmp,41);
        Vtmp<63:0> = (tmp AND Y<127:64>) EOR (NOT(tmp) AND X<63:0>);
        Vtmp<63:0> = (Vtmp<63:0> + MSigma1 + W<63:0>);
        V[d] =  Vtmp;

__instruction aarch64_vector_crypto_sha512_sha512h2
    __encoding aarch64_vector_crypto_sha512_sha512h2
        __instruction_set A64
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11001110 011xxxxx 100001xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSHA512Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) Vtmp;
        bits(64) NSigma0;
        bits(64) tmp;
        bits(128) X = V[n];
        bits(128) Y = V[m];
        bits(128) W = V[d];

        NSigma0 =  ROR(Y<63:0>, 28) EOR ROR(Y<63:0>,34) EOR ROR(Y<63:0>,39);
        Vtmp<127:64> = (X<63:0> AND Y<127:64>) EOR (X<63:0> AND Y<63:0>) EOR (Y<127:64> AND Y<63:0>);
        Vtmp<127:64> = (Vtmp<127:64> + NSigma0 +  W<127:64>);
        NSigma0 =  ROR(Vtmp<127:64>, 28) EOR ROR(Vtmp<127:64>,34) EOR ROR(Vtmp<127:64>,39);
        Vtmp<63:0> =   (Vtmp<127:64> AND Y<63:0>) EOR (Vtmp<127:64> AND Y<127:64>) EOR (Y<127:64> AND Y<63:0>);
        Vtmp<63:0> =   (Vtmp<63:0> + NSigma0 + W<63:0>);

        V[d] = Vtmp;
