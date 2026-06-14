__instruction aarch64_vector_reduce_fp16_add_sisd
    __encoding aarch64_vector_reduce_fp16_add_sisd
        __instruction_set A64
        __field sz 22 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 0x110000 110110xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer esize = 16;
            if sz == '1' then UNDEFINED;
            integer datasize = esize * 2;
            integer elements = 2;

            ReduceOp op = ReduceOp_FADD;

    __encoding aarch64_vector_reduce_fp_add_sisd
        __instruction_set A64
        __field sz 22 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01111110 0x110000 110110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 32 << UInt(sz);
            integer datasize = esize * 2;
            integer elements = 2;

            ReduceOp op = ReduceOp_FADD;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        V[d] = Reduce(op, operand, esize);
