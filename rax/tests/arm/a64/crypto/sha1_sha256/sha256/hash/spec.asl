__instruction aarch64_vector_crypto_sha3op_sha256_hash
    __encoding aarch64_vector_crypto_sha3op_sha256_hash
        __instruction_set A64
        __field Rm 16 +: 5
        __field P 12 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 000xxxxx 010x00xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if !HaveSHA256Ext() then UNDEFINED;
            boolean part1 = (P == '0');

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) result;
        if part1 then
            result = SHA256hash(V[d], V[n], V[m], TRUE);
        else
            result = SHA256hash(V[n], V[d], V[m], FALSE);
        V[d] = result;
