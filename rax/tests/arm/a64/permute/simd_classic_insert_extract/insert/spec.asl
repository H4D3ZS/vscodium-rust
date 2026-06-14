__instruction aarch64_vector_transfer_integer_insert
    __encoding aarch64_vector_transfer_integer_insert
        __instruction_set A64
        __field imm5 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01001110 000xxxxx 000111xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer size = LowestSetBit(imm5);

            if size > 3 then UNDEFINED;
            integer index = UInt(imm5<4:size+1>);

            integer esize = 8 << size;
            integer datasize = 128;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(esize) element = X[n];
        bits(datasize) result;

        result = V[d];
        Elem[result, index, esize] = element;
        V[d] = result;

__instruction aarch64_vector_transfer_vector_insert
    __encoding aarch64_vector_transfer_vector_insert
        __instruction_set A64
        __field imm5 16 +: 5
        __field imm4 11 +: 4
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01101110 000xxxxx 0xxxx1xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer size = LowestSetBit(imm5);
            if size > 3 then UNDEFINED;

            integer dst_index = UInt(imm5<4:size+1>);
            integer src_index = UInt(imm4<3:size>);
            integer idxdsize = if imm4<3> == '1' then 128 else 64;
            // imm4<size-1:0> is IGNORED

            integer esize = 8 << size;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(idxdsize) operand = V[n];
        bits(128) result;

        result = V[d];
        Elem[result, dst_index, esize] = Elem[operand, src_index, esize];
        V[d] = result;
