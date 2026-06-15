__instruction aarch64_vector_transfer_vector_extract
    __encoding aarch64_vector_transfer_vector_extract
        __instruction_set A64
        __field Q 30 +: 1
        __field Rm 16 +: 5
        __field imm4 11 +: 4
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101110 000xxxxx 0xxxx0xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            if Q == '0' && imm4<3> == '1' then UNDEFINED;

            integer datasize = if Q == '1' then 128 else 64;
            integer position = UInt(imm4) << 3;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) hi = V[m];
        bits(datasize) lo = V[n];
        bits(datasize*2) concat = hi : lo;

        V[d] = concat<position+datasize-1:position>;
