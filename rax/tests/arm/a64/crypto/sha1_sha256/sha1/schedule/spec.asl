__instruction aarch64_vector_crypto_sha2op_sha1_sched1
    __encoding aarch64_vector_crypto_sha2op_sha1_sched1
        __instruction_set A64
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 00101000 000110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            if !HaveSHA1Ext() then UNDEFINED;

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) operand1 = V[d];
        bits(128) operand2 = V[n];
        bits(128) result;
        bits(128) T = operand1 EOR LSR(operand2, 32);
        result<31:0>   = ROL(T<31:0>,   1);
        result<63:32>  = ROL(T<63:32>,  1);
        result<95:64>  = ROL(T<95:64>,  1);
        result<127:96> = ROL(T<127:96>, 1) EOR ROL(T<31:0>, 2);
        V[d] = result;

__instruction aarch64_vector_crypto_sha3op_sha1_sched0
    __encoding aarch64_vector_crypto_sha3op_sha1_sched0
        __instruction_set A64
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 000xxxxx 001100xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if !HaveSHA1Ext() then UNDEFINED;

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) operand1 = V[d];
        bits(128) operand2 = V[n];
        bits(128) operand3 = V[m];
        bits(128) result;

        result = operand2<63:0> : operand1<127:64>;
        result = result EOR operand1 EOR operand3;
        V[d] = result;
