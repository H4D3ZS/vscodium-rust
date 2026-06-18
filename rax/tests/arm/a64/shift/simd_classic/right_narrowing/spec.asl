__instruction aarch64_vector_shift_right_narrow_nonuniform_sisd
    __encoding aarch64_vector_shift_right_narrow_nonuniform_sisd
        __instruction_set A64
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field op 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01111111 0xxxxxxx 1000x1xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if immh == '0000' then UNDEFINED;
            if immh<3> == '1' then UNDEFINED;
            integer esize = 8 << HighestSetBit(immh);
            integer datasize = esize;
            integer elements = 1;
            integer part = 0;

            integer shift = (2 * esize) - UInt(immh:immb);
            boolean round = (op == '1');

    __encoding aarch64_vector_shift_right_narrow_nonuniform_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field op 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101111 0xxxxxxx 1000x1xx xxxxxxxx'
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

            integer shift = (2 * esize) - UInt(immh:immb);
            boolean round = (op == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize*2) operand = V[n];
        bits(datasize) result;
        integer round_const = if round then (1 << (shift - 1)) else 0;
        integer element;
        boolean sat;

        for e = 0 to elements-1
            element = (SInt(Elem[operand, e, 2*esize]) + round_const) >> shift;
            (Elem[result, e, esize], sat) = UnsignedSatQ(element, esize);
            if sat then FPSR.QC = '1';

        Vpart[d, part] = result;

__instruction aarch64_vector_shift_right_narrow_uniform_sisd
    __encoding aarch64_vector_shift_right_narrow_uniform_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field op 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11111 0xxxxxxx 1001x1xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if immh == '0000' then UNDEFINED;
            if immh<3> == '1' then UNDEFINED;
            integer esize = 8 << HighestSetBit(immh);
            integer datasize = esize;
            integer elements = 1;
            integer part = 0;

            integer shift = (2 * esize) - UInt(immh:immb);
            boolean round = (op == '1');
            boolean unsigned = (U == '1');

    __encoding aarch64_vector_shift_right_narrow_uniform_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field op 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01111 0xxxxxxx 1001x1xx xxxxxxxx'
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

            integer shift = (2 * esize) - UInt(immh:immb);
            boolean round = (op == '1');
            boolean unsigned = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize*2) operand = V[n];
        bits(datasize) result;
        integer round_const = if round then (1 << (shift - 1)) else 0;
        integer element;
        boolean sat;

        for e = 0 to elements-1
            element = (Int(Elem[operand, e, 2*esize], unsigned) + round_const) >> shift;
            (Elem[result, e, esize], sat) = SatQ(element, esize, unsigned);
            if sat then FPSR.QC = '1';

        Vpart[d, part] = result;

__instruction aarch64_vector_shift_right_narrow_logical
    __encoding aarch64_vector_shift_right_narrow_logical
        __instruction_set A64
        __field Q 30 +: 1
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field op 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001111 0xxxxxxx 1000x1xx xxxxxxxx'
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

            integer shift = (2 * esize) - UInt(immh:immb);
            boolean round = (op == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize*2) operand = V[n];
        bits(datasize) result;
        integer round_const = if round then (1 << (shift - 1)) else 0;
        integer element;

        for e = 0 to elements-1
            element = (UInt(Elem[operand, e, 2*esize]) + round_const) >> shift;
            Elem[result, e, esize] = element<esize-1:0>;

        Vpart[d, part] = result;
