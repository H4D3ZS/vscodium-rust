__instruction aarch64_vector_transfer_integer_move_signed
    __encoding aarch64_vector_transfer_integer_move_signed
        __instruction_set A64
        __field Q 30 +: 1
        __field imm5 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 000xxxxx 001011xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer size;
            case Q:imm5 of
                when 'xxxxx1' size = 0;     // SMOV [WX]d, Vn.B
                when 'xxxx10' size = 1;     // SMOV [WX]d, Vn.H
                when '1xx100' size = 2;     // SMOV Xd, Vn.S
                otherwise     UNDEFINED;

            integer idxdsize = if imm5<4> == '1' then 128 else 64;
            integer index = UInt(imm5<4:size+1>);
            integer esize = 8 << size;
            integer datasize = if Q == '1' then 64 else 32;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(idxdsize) operand = V[n];

        X[d] = SignExtend(Elem[operand, index, esize], datasize);
