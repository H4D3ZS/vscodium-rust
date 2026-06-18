__instruction aarch64_vector_transfer_vector_cpy_dup_sisd
    __encoding aarch64_vector_transfer_vector_cpy_dup_sisd
        __instruction_set A64
        __field imm5 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 000xxxxx 000001xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer size = LowestSetBit(imm5);
            if size > 3 then UNDEFINED;

            integer index = UInt(imm5<4:size+1>);
            integer idxdsize = if imm5<4> == '1' then 128 else 64;

            integer esize = 8 << size;
            integer datasize = esize;
            integer elements = 1;

    __encoding aarch64_vector_transfer_vector_cpy_dup_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field imm5 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 000xxxxx 000001xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer size = LowestSetBit(imm5);
            if size > 3 then UNDEFINED;

            integer index = UInt(imm5<4:size+1>);
            integer idxdsize = if imm5<4> == '1' then 128 else 64;

            if size == 3 && Q == '0' then UNDEFINED;
            integer esize = 8 << size;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(idxdsize) operand = V[n];
        bits(datasize) result;
        bits(esize) element;

        element = Elem[operand, index, esize];
        for e = 0 to elements-1
            Elem[result, e, esize] = element;
        V[d] = result;

__instruction aarch64_vector_transfer_integer_dup
    __encoding aarch64_vector_transfer_integer_dup
        __instruction_set A64
        __field Q 30 +: 1
        __field imm5 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 000xxxxx 000011xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer size = LowestSetBit(imm5);
            if size > 3 then UNDEFINED;

            // imm5<4:size+1> is IGNORED

            if size == 3 && Q == '0' then UNDEFINED;
            integer esize = 8 << size;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(esize) element = X[n];
        bits(datasize) result;

        for e = 0 to elements-1
            Elem[result, e, esize] = element;
        V[d] = result;
