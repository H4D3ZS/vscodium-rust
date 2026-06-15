__instruction aarch64_vector_crypto_sha2op_sha256_sched0
    __encoding aarch64_vector_crypto_sha2op_sha256_sched0
        __instruction_set A64
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 00101000 001010xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            if !HaveSHA256Ext() then UNDEFINED;

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) operand1 = V[d];
        bits(128) operand2 = V[n];
        bits(128) result;
        bits(128) T = operand2<31:0> : operand1<127:32>;
        bits(32) elt;

        for e = 0 to 3
            elt = Elem[T, e, 32];
            elt = ROR(elt, 7) EOR ROR(elt, 18) EOR LSR(elt, 3);
            Elem[result, e, 32] = elt + Elem[operand1, e, 32];
        V[d] = result;

__instruction aarch64_vector_crypto_sha3op_sha256_sched1
    __encoding aarch64_vector_crypto_sha3op_sha256_sched1
        __instruction_set A64
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 000xxxxx 011000xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if !HaveSHA256Ext() then UNDEFINED;

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) operand1 = V[d];
        bits(128) operand2 = V[n];
        bits(128) operand3 = V[m];
        bits(128) result;
        bits(128) T0 = operand3<31:0> : operand2<127:32>;
        bits(64) T1;
        bits(32) elt;

        T1 = operand3<127:64>;
        for e = 0 to 1
            elt = Elem[T1, e, 32];
            elt = ROR(elt, 17) EOR ROR(elt, 19) EOR LSR(elt, 10);
            elt = elt + Elem[operand1, e, 32] + Elem[T0, e, 32];
            Elem[result, e, 32] = elt;

        T1 = result<63:0>;
        for e = 2 to 3
            elt = Elem[T1, e - 2, 32];
            elt = ROR(elt, 17) EOR ROR(elt, 19) EOR LSR(elt, 10);
            elt = elt + Elem[operand1, e, 32] + Elem[T0, e, 32];
            Elem[result, e, 32] = elt;

        V[d] = result;
