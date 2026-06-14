__instruction aarch64_vector_transfer_vector_permute_transpose
    __encoding aarch64_vector_transfer_vector_permute_transpose
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field op 14 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 xx0xxxxx 0x1010xx xxxxxxxx'
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
            integer pairs = elements DIV 2;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) result;

        for p = 0 to pairs-1
            Elem[result, 2*p+0, esize] = Elem[operand1, 2*p+part, esize];
            Elem[result, 2*p+1, esize] = Elem[operand2, 2*p+part, esize];

        V[d] = result;
