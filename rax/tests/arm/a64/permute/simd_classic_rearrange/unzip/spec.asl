__instruction aarch64_vector_transfer_vector_permute_unzip
    __encoding aarch64_vector_transfer_vector_permute_unzip
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field op 14 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 xx0xxxxx 0x0110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            if size:Q == '110' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            integer part = UInt(op);

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operandl = V[n];
        bits(datasize) operandh = V[m];
        bits(datasize) result;

        bits(datasize*2) zipped = operandh:operandl;
        for e = 0 to elements-1
            Elem[result, e, esize] = Elem[zipped, 2*e+part, esize];

        V[d] = result;
