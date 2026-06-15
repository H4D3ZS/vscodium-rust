__instruction aarch64_vector_transfer_integer_move_unsigned
    __encoding aarch64_vector_transfer_integer_move_unsigned
        __instruction_set A64
        __field Q 30 +: 1
        __field imm5 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 000xxxxx 001111xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer size;
            case Q:imm5 of
                when '0xxxx1' size = 0;     // UMOV Wd, Vn.B
                when '0xxx10' size = 1;     // UMOV Wd, Vn.H
                when '0xx100' size = 2;     // UMOV Wd, Vn.S
                when '1x1000' size = 3;     // UMOV Xd, Vn.D
                otherwise     UNDEFINED;

            integer idxdsize = if imm5<4> == '1' then 128 else 64;
            integer index = UInt(imm5<4:size+1>);
            integer esize = 8 << size;
            integer datasize = if Q == '1' then 64 else 32;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(idxdsize) operand = V[n];

        X[d] = ZeroExtend(Elem[operand, index, esize], datasize);
