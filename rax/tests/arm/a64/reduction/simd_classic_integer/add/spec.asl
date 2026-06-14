__instruction aarch64_vector_reduce_add_sisd
    __encoding aarch64_vector_reduce_add_sisd
        __instruction_set A64
        __field size 22 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 xx110001 101110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if size != '11' then UNDEFINED;

            integer esize = 8 << UInt(size);
            integer datasize = esize * 2;
            integer elements = 2;

            ReduceOp op = ReduceOp_ADD;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        V[d] = Reduce(op, operand, esize);

__instruction aarch64_vector_reduce_add_simd
    __encoding aarch64_vector_reduce_add_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 xx110001 101110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if size:Q == '100' then UNDEFINED;
            if size == '11' then UNDEFINED;

            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            ReduceOp op = ReduceOp_ADD;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        V[d] = Reduce(op, operand, esize);
