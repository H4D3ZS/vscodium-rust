__instruction aarch64_vector_crypto_sm3_sm3partw1
    __encoding aarch64_vector_crypto_sm3_sm3partw1
        __instruction_set A64
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11001110 011xxxxx 110000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSM3Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) Vm = V[m];
        bits(128) Vn = V[n];
        bits(128) Vd = V[d];
        bits(128) result;

        result<95:0> = (Vd EOR Vn)<95:0> EOR (ROL(Vm<127:96>,15):ROL(Vm<95:64>,15):ROL(Vm<63:32>,15));

        for i = 0 to 3
            if i == 3 then
                result<127:96> = (Vd EOR Vn)<127:96> EOR (ROL(result<31:0>,15));
            result<(32*i)+31:(32*i)> = result<(32*i)+31:(32*i)> EOR ROL(result<(32*i)+31:(32*i)>,15) EOR ROL(result<(32*i)+31:(32*i)>,23);
        V[d] = result;

__instruction aarch64_vector_crypto_sm3_sm3partw2
    __encoding aarch64_vector_crypto_sm3_sm3partw2
        __instruction_set A64
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11001110 011xxxxx 110001xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSM3Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) Vm = V[m];
        bits(128) Vn = V[n];
        bits(128) Vd = V[d];
        bits(128) result;
        bits(128) tmp;
        bits(32) tmp2;
        tmp<127:0> = Vn EOR (ROL(Vm<127:96>,7):ROL(Vm<95:64>,7):ROL(Vm<63:32>,7):ROL(Vm<31:0>,7));
        result<127:0> = Vd<127:0> EOR tmp<127:0>;
        tmp2 = ROL(tmp<31:0>,15);
        tmp2 = tmp2 EOR ROL(tmp2,15) EOR ROL(tmp2,23);
        result<127:96> = result<127:96> EOR tmp2;
        V[d]= result;
