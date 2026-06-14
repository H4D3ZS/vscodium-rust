__instruction aarch64_vector_reduce_fp16_max_sisd
    __encoding aarch64_vector_reduce_fp16_max_sisd
        __instruction_set A64
        __field o1 23 +: 1
        __field sz 22 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 xx110000 111110xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 16;
            if sz == '1' then UNDEFINED;
            integer datasize = esize * 2;
            integer elements = 2;

            ReduceOp op = if o1 == '1' then ReduceOp_FMIN else ReduceOp_FMAX;

    __encoding aarch64_vector_reduce_fp_max_sisd
        __instruction_set A64
        __field o1 23 +: 1
        __field sz 22 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01111110 xx110000 111110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 32 << UInt(sz);
            integer datasize = esize * 2;
            integer elements = 2;

            ReduceOp op = if o1 == '1' then ReduceOp_FMIN else ReduceOp_FMAX;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        V[d] = Reduce(op, operand, esize);

__instruction aarch64_vector_reduce_fp16_max_simd
    __encoding aarch64_vector_reduce_fp16_max_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field o1 23 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 x0110000 111110xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 16;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            ReduceOp op = if o1 == '1' then ReduceOp_FMIN else ReduceOp_FMAX;

    __encoding aarch64_vector_reduce_fp_max_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field o1 23 +: 1
        __field sz 22 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101110 xx110000 111110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if sz:Q != '01' then UNDEFINED;

            integer esize = 32 << UInt(sz);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            ReduceOp op = if o1 == '1' then ReduceOp_FMIN else ReduceOp_FMAX;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        V[d] = Reduce(op, operand, esize);
