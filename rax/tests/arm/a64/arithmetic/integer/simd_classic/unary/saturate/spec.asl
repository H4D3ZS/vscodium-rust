__instruction aarch64_vector_arithmetic_unary_add_saturating_sisd
    __encoding aarch64_vector_arithmetic_unary_add_saturating_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11110 xx100000 001110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 8 << UInt(size);
            integer datasize = esize;
            integer elements = 1;

            boolean unsigned = (U == '1');

    __encoding aarch64_vector_arithmetic_unary_add_saturating_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx100000 001110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if size:Q == '110' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            boolean unsigned = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        bits(datasize) result;

        bits(datasize) operand2 = V[d];
        integer op1;
        integer op2;
        boolean sat;

        for e = 0 to elements-1
            op1 = Int(Elem[operand, e, esize], !unsigned);
            op2 = Int(Elem[operand2, e, esize], unsigned);
            (Elem[result, e, esize], sat) = SatQ(op1 + op2, esize, unsigned);
            if sat then FPSR.QC = '1';
        V[d] = result;

__instruction aarch64_vector_arithmetic_unary_diff_neg_sat_sisd
    __encoding aarch64_vector_arithmetic_unary_diff_neg_sat_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11110 xx100000 011110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer esize = 8 << UInt(size);
            integer datasize = esize;
            integer elements = 1;
            boolean neg = (U == '1');

    __encoding aarch64_vector_arithmetic_unary_diff_neg_sat_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx100000 011110xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if size:Q == '110' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean neg = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        bits(datasize) result;
        integer element;
        boolean sat;

        for e = 0 to elements-1
            element = SInt(Elem[operand, e, esize]);
            if neg then
                element = -element;
            else
                element = Abs(element);
            (Elem[result, e, esize], sat) = SignedSatQ(element, esize);
            if sat then FPSR.QC = '1';

        V[d] = result;
