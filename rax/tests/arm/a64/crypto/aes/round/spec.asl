__instruction aarch64_vector_crypto_aes_round
    __encoding aarch64_vector_crypto_aes_round
        __instruction_set A64
        __field D 12 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01001110 00101000 010x10xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            if !HaveAESExt() then UNDEFINED;
            boolean decrypt = (D == '1');

    __execute
        AArch64.CheckFPAdvSIMDEnabled();

        bits(128) operand1 = V[d];
        bits(128) operand2 = V[n];
        bits(128) result;
        result = operand1 EOR operand2;
        if decrypt then
            result = AESInvSubBytes(AESInvShiftRows(result));
        else
            result = AESSubBytes(AESShiftRows(result));

        V[d] = result;
