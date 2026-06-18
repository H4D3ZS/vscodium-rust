__instruction aarch64_vector_crypto_sm3_sm3ss1
    __encoding aarch64_vector_crypto_sm3_sm3ss1
        __instruction_set A64
        __field Rm 16 +: 5
        __field Ra 10 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11001110 010xxxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSM3Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            integer a = UInt(Ra);

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) Vm = V[m];
        bits(128) Vn = V[n];
        bits(128) Vd = V[d];
        bits(128) Va = V[a];
        Vd<127:96> = ROL((ROL(Vn<127:96>,12) + Vm<127:96> + Va<127:96>) , 7);
        Vd<95:0> = Zeros();
        V[d] = Vd;
