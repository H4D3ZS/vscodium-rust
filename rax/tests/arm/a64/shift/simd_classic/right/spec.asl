__instruction aarch64_vector_shift_right_sisd
    __encoding aarch64_vector_shift_right_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field o1 13 +: 1
        __field o0 12 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11111 0xxxxxxx 00xx01xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if immh<3> != '1' then UNDEFINED;
            integer esize = 8 << 3;
            integer datasize = esize;
            integer elements = 1;

            integer shift = (esize * 2) - UInt(immh:immb);
            boolean unsigned = (U == '1');
            boolean round = (o1 == '1');
            boolean accumulate = (o0 == '1');

    __encoding aarch64_vector_shift_right_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field o1 13 +: 1
        __field o0 12 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01111 0xxxxxxx 00xx01xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if immh == '0000' then SEE(asimdimm);
            if immh<3>:Q == '10' then UNDEFINED;
            integer esize = 8 << HighestSetBit(immh);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            integer shift = (esize * 2) - UInt(immh:immb);
            boolean unsigned = (U == '1');
            boolean round = (o1 == '1');
            boolean accumulate = (o0 == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand  = V[n];
        bits(datasize) operand2;
        bits(datasize) result;
        integer round_const = if round then (1 << (shift - 1)) else 0;
        integer element;

        operand2 = if accumulate then V[d] else Zeros();
        for e = 0 to elements-1
            element = (Int(Elem[operand, e, esize], unsigned) + round_const) >> shift;
            Elem[result, e, esize] = Elem[operand2, e, esize] + element<esize-1:0>;

        V[d] = result;

__instruction aarch64_vector_shift_right_insert_sisd
    __encoding aarch64_vector_shift_right_insert_sisd
        __instruction_set A64
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01111111 0xxxxxxx 010001xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if immh<3> != '1' then UNDEFINED;
            integer esize = 8 << 3;
            integer datasize = esize;
            integer elements = 1;

            integer shift = (esize * 2) - UInt(immh:immb);

    __encoding aarch64_vector_shift_right_insert_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101111 0xxxxxxx 010001xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if immh == '0000' then SEE(asimdimm);
            if immh<3>:Q == '10' then UNDEFINED;
            integer esize = 8 << HighestSetBit(immh);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            integer shift = (esize * 2) - UInt(immh:immb);

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand  = V[n];
        bits(datasize) operand2 = V[d];
        bits(datasize) result;
        bits(esize) mask = LSR(Ones(esize), shift);
        bits(esize) shifted;

        for e = 0 to elements-1
            shifted = LSR(Elem[operand, e, esize], shift);
            Elem[result, e, esize] = (Elem[operand2, e, esize] AND NOT(mask)) OR shifted;
        V[d] = result;
