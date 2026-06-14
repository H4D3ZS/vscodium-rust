__instruction aarch64_vector_shift_left_sisd
    __encoding aarch64_vector_shift_left_sisd
        __instruction_set A64
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011111 0xxxxxxx 010101xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if immh<3> != '1' then UNDEFINED;
            integer esize = 8 << 3;
            integer datasize = esize;
            integer elements = 1;

            integer shift = UInt(immh:immb) - esize;

    __encoding aarch64_vector_shift_left_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001111 0xxxxxxx 010101xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if immh == '0000' then SEE(asimdimm);
            if immh<3>:Q == '10' then UNDEFINED;
            integer esize = 8 << HighestSetBit(immh);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            integer shift = UInt(immh:immb) - esize;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand  = V[n];
        bits(datasize) result;

        for e = 0 to elements-1
            Elem[result, e, esize] = LSL(Elem[operand, e, esize], shift);

        V[d] = result;

__instruction aarch64_vector_shift_left_insert_sisd
    __encoding aarch64_vector_shift_left_insert_sisd
        __instruction_set A64
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01111111 0xxxxxxx 010101xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if immh<3> != '1' then UNDEFINED;
            integer esize = 8 << 3;
            integer datasize = esize;
            integer elements = 1;

            integer shift = UInt(immh:immb) - esize;

    __encoding aarch64_vector_shift_left_insert_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101111 0xxxxxxx 010101xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if immh == '0000' then SEE(asimdimm);
            if immh<3>:Q == '10' then UNDEFINED;
            integer esize = 8 << HighestSetBit(immh);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            integer shift = UInt(immh:immb) - esize;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand  = V[n];
        bits(datasize) operand2 = V[d];
        bits(datasize) result;
        bits(esize) mask = LSL(Ones(esize), shift);
        bits(esize) shifted;

        for e = 0 to elements-1
            shifted = LSL(Elem[operand, e, esize], shift);
            Elem[result, e, esize] = (Elem[operand2, e, esize] AND NOT(mask)) OR shifted;
        V[d] = result;

__instruction aarch64_vector_shift_left_long
    __encoding aarch64_vector_shift_left_long
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01111 0xxxxxxx 101001xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if immh == '0000' then SEE(asimdimm);
            if immh<3> == '1' then UNDEFINED;
            integer esize = 8 << HighestSetBit(immh);
            integer datasize = 64;
            integer part = UInt(Q);
            integer elements = datasize DIV esize;

            integer shift = UInt(immh:immb) - esize;
            boolean unsigned = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = Vpart[n, part];
        bits(datasize*2) result;
        integer element;

        for e = 0 to elements-1
            element = Int(Elem[operand, e, esize], unsigned) << shift;
            Elem[result, e, 2*esize] = element<2*esize-1:0>;

        V[d] = result;
