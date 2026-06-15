__instruction aarch64_vector_crypto_sha3_rax1
    __encoding aarch64_vector_crypto_sha3_rax1
        __instruction_set A64
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11001110 011xxxxx 100011xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSHA3Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) Vm = V[m];
        bits(128) Vn = V[n];
        V[d] = Vn EOR (ROL(Vm<127:64>,1):ROL(Vm<63:0>, 1));

__instruction aarch64_vector_crypto_sha3_eor3
    __encoding aarch64_vector_crypto_sha3_eor3
        __instruction_set A64
        __field Rm 16 +: 5
        __field Ra 10 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11001110 000xxxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSHA3Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            integer a = UInt(Ra);

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) Vm = V[m];
        bits(128) Vn = V[n];
        bits(128) Va = V[a];
        V[d] = Vn EOR Vm EOR Va;

__instruction aarch64_vector_crypto_sha3_bcax
    __encoding aarch64_vector_crypto_sha3_bcax
        __instruction_set A64
        __field Rm 16 +: 5
        __field Ra 10 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11001110 001xxxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSHA3Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            integer a = UInt(Ra);

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) Vm = V[m];
        bits(128) Vn = V[n];
        bits(128) Va = V[a];
        V[d] = Vn EOR (Vm AND NOT(Va));

__instruction aarch64_vector_crypto_sha3_xar
    __encoding aarch64_vector_crypto_sha3_xar
        __instruction_set A64
        __field Rm 16 +: 5
        __field imm6 10 +: 6
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11001110 100xxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSHA3Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) Vm = V[m];
        bits(128) Vn = V[n];
        bits(128) tmp;
        tmp = Vn EOR Vm;
        V[d] = ROR(tmp<127:64>, UInt(imm6)):ROR(tmp<63:0>, UInt(imm6));
