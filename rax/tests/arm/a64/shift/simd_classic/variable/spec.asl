__instruction aarch64_vector_arithmetic_binary_uniform_shift_sisd
    __encoding aarch64_vector_arithmetic_binary_uniform_shift_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field R 12 +: 1
        __field S 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11110 xx1xxxxx 010xx1xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer esize = 8 << UInt(size);
            integer datasize = esize;
            integer elements = 1;
            boolean unsigned = (U == '1');
            boolean rounding = (R == '1');
            boolean saturating = (S == '1');
            if S == '0' && size != '11' then UNDEFINED;

    __encoding aarch64_vector_arithmetic_binary_uniform_shift_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field R 12 +: 1
        __field S 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx1xxxxx 010xx1xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if size:Q == '110' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean unsigned = (U == '1');
            boolean rounding = (R == '1');
            boolean saturating = (S == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) result;

        integer round_const = 0;
        integer shift;
        integer element;
        boolean sat;

        for e = 0 to elements-1
            shift = SInt(Elem[operand2, e, esize]<7:0>);
            if rounding then
                round_const = 1 << (-shift - 1); // 0 for left shift, 2^(n-1) for right shift
            element = (Int(Elem[operand1, e, esize], unsigned) + round_const) << shift;
            if saturating then
                (Elem[result, e, esize], sat) = SatQ(element, esize, unsigned);
                if sat then FPSR.QC = '1';
            else
                Elem[result, e, esize] = element<esize-1:0>;

        V[d] = result;
